//! The RF-Musette engine: a physically modelled accordion, treble side first.
//!
//! One note so far: the accordion F4 the IfM Zwota measured, on key 65 --
//! its plate's two reeds, the one inside the cell sounding when the bellows
//! pulls and the one on the bellows side when it pushes, behind the pallet
//! that key lifts. The bellows holds its pressure whether or not a key is
//! down; the pallet is what lets air through. Every other key is tracked and
//! silent. The model and its sources are in [`reed`] and [`pallet`]; every
//! mechanism and constant is entered in the ledger, `docs/MODEL.md`.
//!
//! The crate is `no_std` and never allocates: it runs inside the plugin's
//! WebAssembly component on every RackForge host, the Raspberry Pi included.

#![no_std]

// Natively -- the tests, the laboratory -- the standard library lends the
// engine its correctly rounded square root, the one WebAssembly's
// `f64.sqrt` gives the plugin ([`math::sqrt`]). Nothing here allocates.
#[cfg(not(target_arch = "wasm32"))]
extern crate std;

mod bellows;
pub mod cassotto;
pub mod compass;
mod decimator;
pub mod math;
pub mod pallet;
pub mod parameters;
pub mod programs;
pub mod reed;
pub mod stage;
pub mod tongue;
pub mod tuning;
pub mod wind;

pub use bellows::{Bellows, BellowsSource, RESTING_PUSH};
pub use decimator::Decimator;
use pallet::{Pallet, PalletDesign};
pub use parameters::{COUNT as PARAMETER_COUNT, Parameters, SPECS as PARAMETER_SPECS, Side};
use reed::{ReedModel, ReedState, Tube};
use tongue::TongueMode;

/// MIDI key numbers the engine tracks.
pub const KEYS: usize = 128;

/// The one key that sounds in milestone 1: F4, the reed Ziegenhals measured.
pub const REED_KEY: u8 = 65;

/// The sample rates the engine accepts, in hertz.
pub const SAMPLE_RATES: core::ops::RangeInclusive<f32> = 8_000.0..=384_000.0;

/// ρ / (4π r) at r = 1 m: sound pressure per unit rate of change of the
/// volume flow, for a monopole.
const RADIATION: f64 = reed::AIR_DENSITY / (4.0 * core::f64::consts::PI);

/// Time constant of the guard that keeps a stepped controller from reaching
/// the reed as a step. Numerical, not physical: the bellows' own compliance
/// is milestone 5.
const SUPPLY_SMOOTHING_SECONDS: f64 = 0.001;

/// The recording's level (milestone 9c): pascals at 1 m to full scale, set
/// so the loudest the instrument plays -- both hands' full chords in Master
/// at the bellows' ceiling, through the ORTF pair -- peaks at -6 dBFS, a
/// recording's headroom: -17.7 dB, full scale 111.7 dB SPL at the 1 m
/// reference (measured again with milestone 9d's room, whose part of each
/// layout's balance moves the peaks a decibel or two, and again with 8m's
/// tube, with which the loudest case peaked 3.3 dB lower, and its set's
/// shape, 1.0 dB lower again: -22 dB before).
pub const RECORDING_LEVEL: f32 = 0.130_3;

/// Where the soft ceiling starts, of full scale: -6 dBFS; and the level it
/// rounds toward, -0.2 dBFS, short of full scale even where a float's tanh
/// has rounded to one.
const CEILING_KNEE: f32 = 0.5;
const CEILING_TOP: f32 = 0.98;

/// The output's ceiling: nothing under the knee is touched, and what rises
/// above it rounds toward full scale, never reaching it, instead of being
/// cut there (milestone 9c).
pub fn ceiling(sample: f32) -> f32 {
    let size = sample.abs();
    if size <= CEILING_KNEE || !size.is_finite() {
        return if size.is_finite() { sample } else { 0.0 };
    }
    let over = f64::from((size - CEILING_KNEE) / (CEILING_TOP - CEILING_KNEE));
    // tanh, its slope 1 at the knee, so the ceiling bends in without a step.
    let e = math::exp(-2.0 * over);
    let bent = CEILING_KNEE + (CEILING_TOP - CEILING_KNEE) * ((1.0 - e) / (1.0 + e)) as f32;
    bent.copysign(sample)
}

/// The MIDI channels of the bass side, counted from 0, as Roland's
/// V-Accordions send them (FR-3x Owner's Manual, p. 57): the bass buttons
/// on channel 2, the chord buttons on channel 3. Every other channel plays
/// the treble.
pub const BASS_CHANNEL: u8 = 1;
pub const CHORD_CHANNEL: u8 = 2;

/// Where the bass side's keys sit among the engine's: after the treble's.
const BASS_START: usize = compass::KEYS;
/// The treble's keys and the bass side's.
const ALL_KEYS: usize = compass::KEYS + compass::BASS_KEYS;

/// The tongues heard through the bellows (8k). A tongue swinging in its slot
/// moves air into the bellows with its own face, S_r ζ', as well as setting
/// the jet that sounds through the hole; that reaches the listener through
/// the bellows' walls. Taken as the tongue's own monopole, S_r ζ'', through
/// a limp wall of mass m per area: the mass law, 1/(1 + jωm/2ρc), a single
/// pole at ρc/(πm). Only the tongue's own motion: the jet already sounds
/// through the hole, and its slow swings would come through as a rumble.
/// It is what carries the tongue's ringing once the pallet has shut.
///
/// The walls are 0.8-1 mm manila card (makers' accounts, "How to make
/// Bellows"), solid board ~615 g/m² a millimetre, lined with cloth: about
/// 0.75 kg/m² -- derived. Left out: the bellows' air between tongue and
/// wall, whose stiffness and the walls' mass make a resonance of their
/// own, and the walls' area.
const BELLOWS_WALL_MASS: f64 = 0.75;
/// ρc of air, Pa·s/m.
const AIR_IMPEDANCE: f64 = reed::AIR_DENSITY * 343.2;

/// The plate's reed that sounds when the bellows pulls: inside the cell.
pub const PULL_REED: usize = 0;
/// The one that sounds when it pushes: on the bellows side.
pub const PUSH_REED: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineError {
    /// The sample rate is not a finite number inside [`SAMPLE_RATES`].
    SampleRate,
}

/// One rank's plate for a key: its reed's model, and its two reeds,
/// [`PULL_REED`] and [`PUSH_REED`] -- the same design, each with its own
/// state. On an instrument a plate's reeds share one cell; here each keeps
/// its own copy of it, which differs only while both still move, through a
/// reversal (docs/MODEL.md).
struct Rank {
    /// `None` until the key is first built, or where the rank has no reed.
    model: Option<ReedModel>,
    states: [ReedState; 2],
    /// Each reed's cell, a tube (milestone 8l).
    tubes: [Tube; 2],
    /// How much of its start each reed has been given since air could last
    /// reach it, as the share P/(P + P₀) of the full start.
    started: [f64; 2],
}

impl Rank {
    fn is_still(&self) -> bool {
        self.states
            .iter()
            .all(|state| *state == ReedState::default())
    }
}

/// A key of the compass, or a pitch class of the bass side: its pallets and
/// its ranks' plates behind them.
struct Key {
    /// The treble key's pallet, or the bass button's: on the bass side it
    /// opens the pitch class on every rank.
    pallet: Pallet,
    /// On the bass side, what the chord buttons holding the pitch class
    /// open: its reeds on the chord ranks. Shut on the treble.
    chord: Pallet,
    ranks: [Rank; parameters::RANKS],
    /// Its reeds' models are out of date: built again before it next sounds,
    /// or a few at a time while nothing asks.
    stale: bool,
}

impl Key {
    fn is_still(&self) -> bool {
        self.ranks.iter().all(Rank::is_still)
    }

    /// Nothing to compute: the pallets shut and every reed at rest.
    fn is_idle(&self) -> bool {
        self.pallet.is_closed() && self.chord.is_closed() && self.is_still()
    }
}

/// Stale keys built per render while nothing asks for them: enough to catch
/// up within a few blocks, few enough not to load one.
const BUILDS_PER_BLOCK: usize = 2;

pub struct Engine {
    sample_rate: f32,
    parameters: Parameters,
    /// The tongue's profile, solved from its mode ratio; kept, because
    /// solving it takes milliseconds and the ratio rarely moves. Every reed
    /// shares it.
    mode: TongueMode,
    /// The compass, F3-A6: each key's pallet and its ranks,
    /// [`parameters::RANK_LOW`] .. [`parameters::RANK_HIGH`]; then the bass
    /// side's pitch classes, C-B, with their ranks [`parameters::BASS_16`]
    /// .. [`parameters::BASS_2`].
    keys: [Key; ALL_KEYS],
    pallet_design: PalletDesign,
    /// What the intent asks, Pa: the pressure the push would make in a
    /// still bellows, guarded against steps.
    ask: f64,
    /// The bellows' air, when the intent is the arm's push.
    wind: wind::Wind,
    /// The air the reeds drew in the last step, m³/s.
    draw: f64,
    /// The cassotto L and M sound into, when the instrument has one: one
    /// filter for each treble quarter the microphones hear apart.
    cassotto: [cassotto::Cassotto; stage::TREBLE_SOURCES],
    /// Each source's tongues heard through the bellows' walls: the mass
    /// law's one pole, its state (8k).
    inside: [f64; stage::SOURCES],
    /// The bellows' pressure, Pa, without its sign.
    supply: f64,
    /// Which way the bellows moves: -1 pulling, +1 pushing, and in between
    /// while it turns. The signed pressure is `turn × supply`.
    turn: f64,
    /// Auto Reverse has turned the bellows from the player's direction.
    flipped: bool,
    /// The air drawn in this direction since the bellows last turned, m³.
    spent: f64,
    decimator: Decimator,
    /// A reed parameter moved: rebuild before the next sample.
    dirty: bool,
    held: [bool; KEYS],
    /// Where each note on a treble channel went when it was played, so it
    /// is let go there whatever the split has become since.
    played: [Side; KEYS],
    /// The notes held on the bass and chord channels.
    bass_held: [bool; KEYS],
    chord_held: [bool; KEYS],
    bellows: Bellows,
    /// The microphones and the room (milestone 9b): each source brought to
    /// the host's rate apart, the stage, and whether it needs tuning.
    zone_decimators: [Decimator; stage::SOURCES],
    stage: stage::Stage,
    stage_dirty: bool,
    /// The air in the bellows, m³ from shut: where the bass box is.
    opened: f64,
}

impl Engine {
    pub fn new(sample_rate: f32) -> Result<Self, EngineError> {
        if !sample_rate.is_finite() || !SAMPLE_RATES.contains(&sample_rate) {
            return Err(EngineError::SampleRate);
        }
        let parameters = Parameters::default();
        let mode = TongueMode::with_ratio(parameters.reed_design().mode_ratio);
        let keys = core::array::from_fn(|_| Key {
            pallet: Pallet::default(),
            chord: Pallet::default(),
            ranks: core::array::from_fn(|_| Rank {
                model: None,
                states: [ReedState::default(); 2],
                tubes: [Tube::default(); 2],
                started: [0.0; 2],
            }),
            stale: true,
        });
        let mut engine = Self {
            sample_rate,
            parameters,
            mode,
            keys,
            pallet_design: parameters.pallet_design(),
            ask: 0.0,
            wind: wind::Wind::default(),
            draw: 0.0,
            cassotto: core::array::from_fn(|_| cassotto::Cassotto::default()),
            inside: [0.0; stage::SOURCES],
            supply: 0.0,
            turn: parameters.direction(),
            flipped: false,
            spent: 0.0,
            decimator: Decimator::new(parameters.oversampling()),
            dirty: false,
            held: [false; KEYS],
            played: [Side::Treble; KEYS],
            bass_held: [false; KEYS],
            chord_held: [false; KEYS],
            bellows: Bellows::new(),
            zone_decimators: core::array::from_fn(|_| Decimator::new(parameters.oversampling())),
            stage: stage::Stage::new(f64::from(sample_rate)),
            stage_dirty: true,
            opened: parameters.travel() / 2.0,
        };
        engine.rebuild();
        Ok(engine)
    }

    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    pub fn parameters(&self) -> &Parameters {
        &self.parameters
    }

    pub fn parameter(&self, index: usize) -> Option<f64> {
        self.parameters.get(index)
    }

    /// Sets one parameter in its own units. A value outside its range, or
    /// not a number, is refused and changes nothing. The reed is rebuilt
    /// from the new value before the next sample it renders.
    pub fn set_parameter(&mut self, index: usize, value: f64) -> bool {
        if !self.parameters.set(index, value) {
            return false;
        }
        // With no key down and nothing sounding, how long the bellows takes
        // to turn cannot be heard: a loaded program or a fresh engine starts
        // already turned.
        // Setting the direction takes the bellows back from Auto Reverse,
        // its travel counted afresh.
        if index == parameters::BELLOWS_DIRECTION {
            self.flipped = false;
            self.spent = 0.0;
        }
        // The microphones and the room are tuned before the next stereo
        // block; nothing of the reeds changes.
        if index >= parameters::MIC_LAYOUT {
            self.stage_dirty = true;
            return true;
        }
        if matches!(index, parameters::BELLOWS_TRAVEL | parameters::BELLOWS_AREA) {
            self.stage_dirty = true;
        }
        if index == parameters::BELLOWS_DIRECTION
            && !self.anything_held()
            && self.keys.iter().all(Key::is_still)
        {
            self.turn = self.direction();
        }
        // The gain, the register, the bellows' direction, turning time and
        // air, and the cassotto are read as the samples are made; everything
        // else rebuilds the reeds.
        if !matches!(
            index,
            parameters::GAIN
                | parameters::REGISTER
                | parameters::BELLOWS_DIRECTION
                | parameters::REVERSAL_TIME
                | parameters::BELLOWS_RESPONSE
                | parameters::BELLOWS_AREA
                | parameters::BELLOWS_VOLUME
                | parameters::ARM_SPEED
                | parameters::BELLOWS_LEAK
                | parameters::AIR_VALVE
                | parameters::CASSOTTO
                | parameters::CASSOTTO_RESONANCE
                | parameters::CASSOTTO_Q
                | parameters::ATTACK_KICK
                | parameters::BASS_REGISTER
                | parameters::LEFT_HAND
                | parameters::SPLIT_POINT
                | parameters::BELLOWS_SMOOTHING
                | parameters::AUTO_REVERSE
                | parameters::BELLOWS_TRAVEL
                | parameters::MOD_WHEEL
        ) {
            self.dirty = true;
        }
        true
    }

    /// A key goes down. On a real accordion the key only opens its pallet,
    /// and has no velocity: the sound's strength is the bellows', which the
    /// wheel or Expression sets (`bellows.rs`). The velocity is ignored.
    pub fn note_on(&mut self, key: u8, _velocity: f32) {
        if usize::from(key) >= KEYS {
            return;
        }
        self.press(key, 1.0);
    }

    pub fn note_off(&mut self, key: u8) {
        self.press(key, 0.0);
    }

    /// Takes `key` to `depth`, from 0 (up) to 1 (fully down). A key held
    /// partly down holds its pallet partly open -- how an accordionist
    /// bends a note.
    pub fn press(&mut self, key: u8, depth: f64) {
        let Some(held) = self.held.get_mut(usize::from(key)) else {
            return;
        };
        *held = depth > 0.0;
        if let Some(index) = compass_index(key) {
            self.keys[index].pallet.press(depth);
        }
    }

    /// A note on MIDI `channel` (0-15): the bass buttons on
    /// [`BASS_CHANNEL`], the chords on [`CHORD_CHANNEL`], the treble on every
    /// other -- or, with Left Hand on, under the split, the chords and the
    /// bass buttons (milestone 8b).
    pub fn channel_note_on(&mut self, channel: u8, key: u8, velocity: f32) {
        match channel {
            BASS_CHANNEL => self.bass_on(key, velocity),
            CHORD_CHANNEL => self.chord_on(key, velocity),
            _ => {
                let side = self.parameters.left_hand_side(key);
                if let Some(played) = self.played.get_mut(usize::from(key)) {
                    *played = side;
                }
                match side {
                    Side::Treble => self.note_on(key, velocity),
                    Side::Bass => self.bass_on(key, velocity),
                    Side::Chord => self.chord_on(key, velocity),
                }
            }
        }
    }

    pub fn channel_note_off(&mut self, channel: u8, key: u8) {
        match channel {
            BASS_CHANNEL => self.bass_off(key),
            CHORD_CHANNEL => self.chord_off(key),
            _ => match self.played.get(usize::from(key)).copied() {
                Some(Side::Bass) => self.bass_off(key),
                Some(Side::Chord) => self.chord_off(key),
                _ => self.note_off(key),
            },
        }
    }

    /// A bass button goes down: the one of `key`'s pitch class, whatever its
    /// octave (a V-Accordion sends C3-B3). The velocity is ignored, as on
    /// the treble.
    pub fn bass_on(&mut self, key: u8, _velocity: f32) {
        self.hold(key, true, true);
    }

    pub fn bass_off(&mut self, key: u8) {
        self.hold(key, true, false);
    }

    /// A chord note: `key`'s pitch class sounds on the chord ranks. A
    /// V-Accordion's chord button sends its three; a keyboard player's left
    /// hand sends what it holds. The velocity is ignored.
    pub fn chord_on(&mut self, key: u8, _velocity: f32) {
        self.hold(key, false, true);
    }

    pub fn chord_off(&mut self, key: u8) {
        self.hold(key, false, false);
    }

    /// Holds or lets go of a note on the bass or chord channel, and opens or
    /// shuts its pitch class's pallet while any octave of it is held. False
    /// for a key outside MIDI.
    fn hold(&mut self, key: u8, bass: bool, down: bool) -> bool {
        let held = if bass {
            &mut self.bass_held
        } else {
            &mut self.chord_held
        };
        let Some(slot) = held.get_mut(usize::from(key)) else {
            return false;
        };
        *slot = down;
        let pitch_class = usize::from(key) % compass::BASS_KEYS;
        let any = held
            .iter()
            .skip(pitch_class)
            .step_by(compass::BASS_KEYS)
            .any(|held| *held);
        let depth = if any { 1.0 } else { 0.0 };
        let key = &mut self.keys[BASS_START + pitch_class];
        if bass {
            key.pallet.press(depth);
        } else {
            key.chord.press(depth);
        }
        true
    }

    /// Whether any key, bass button or chord note is held.
    fn anything_held(&self) -> bool {
        self.held_count() > 0
            || self.bass_held.iter().any(|held| *held)
            || self.chord_held.iter().any(|held| *held)
    }

    pub fn is_held(&self, key: u8) -> bool {
        self.held.get(usize::from(key)).copied().unwrap_or(false)
    }

    pub fn held_count(&self) -> usize {
        self.held.iter().filter(|held| **held).count()
    }

    pub fn bellows(&self) -> &Bellows {
        &self.bellows
    }

    pub fn bellows_mut(&mut self) -> &mut Bellows {
        &mut self.bellows
    }

    /// The modulation wheel's high seven bits (CC 1): the push, as
    /// Expression is (milestone 8f). It was also, with Mod Wheel on Bellows,
    /// where the bellows is (8i) -- withdrawn, ROADMAP 8i.
    pub fn wheel_msb(&mut self, value: u8) {
        self.bellows.expression_msb(value);
    }

    /// The wheel's low seven bits (CC 33).
    pub fn wheel_lsb(&mut self, value: u8) {
        self.bellows.expression_lsb(value);
    }

    /// The wheel at MIDI 2.0 width, already a fraction of its range.
    pub fn wheel_wide(&mut self, value: f32) {
        self.bellows.expression_wide(value);
    }

    /// One reed as it stands: of `key`'s `rank` ([`parameters::RANK_LOW`] ..
    /// [`parameters::RANK_HIGH`]), the plate's [`PULL_REED`] or
    /// [`PUSH_REED`], for measurement. `None` outside the compass, or before
    /// the key is first built.
    pub fn reed(&self, key: u8, rank: usize, which: usize) -> Option<(&ReedModel, &ReedState)> {
        let rank = self.keys.get(compass_index(key)?)?.ranks.get(rank)?;
        Some((rank.model.as_ref()?, rank.states.get(which)?))
    }

    /// One bass-side reed as it stands: `pitch_class`'s (0 = C) on `rank`
    /// ([`parameters::BASS_16`] .. [`parameters::BASS_2`]), for measurement.
    pub fn bass_reed(
        &self,
        pitch_class: usize,
        rank: usize,
        which: usize,
    ) -> Option<(&ReedModel, &ReedState)> {
        if pitch_class >= compass::BASS_KEYS {
            return None;
        }
        let rank = self.keys[BASS_START + pitch_class].ranks.get(rank)?;
        Some((rank.model.as_ref()?, rank.states.get(which)?))
    }

    /// The bellows' pressure the reeds see, Pa: below zero pulling, above
    /// pushing.
    pub fn supply(&self) -> f64 {
        self.turn * self.supply
    }

    /// Lets every key go and silences the reeds. What the bellows was asked
    /// stays asked: resetting the audio does not move the player's arm, nor
    /// turn it.
    pub fn reset(&mut self) {
        self.held = [false; KEYS];
        self.bass_held = [false; KEYS];
        self.chord_held = [false; KEYS];
        for key in self.keys.iter_mut() {
            key.pallet = Pallet::default();
            key.chord = Pallet::default();
            for rank in key.ranks.iter_mut() {
                rank.states = [ReedState::default(); 2];
                rank.tubes = [Tube::default(); 2];
                rank.started = [0.0; 2];
            }
        }
        self.ask = 0.0;
        self.wind = wind::Wind::default();
        self.draw = 0.0;
        for cassotto in self.cassotto.iter_mut() {
            cassotto.reset();
        }
        self.inside = [0.0; stage::SOURCES];
        for decimator in self.zone_decimators.iter_mut() {
            decimator.reset();
        }
        self.stage.reset();
        self.supply = 0.0;
        self.turn = self.direction();
        self.decimator.reset();
    }

    /// Builds one key's reeds from the parameters, tuned as the compass says.
    fn build_key(&mut self, index: usize) {
        for (rank, slot) in self.keys[index].ranks.iter_mut().enumerate() {
            let design = match index.checked_sub(BASS_START) {
                Some(pitch_class) => compass::bass_design(&self.parameters, pitch_class, rank),
                None => compass::design(&self.parameters, compass::FIRST_KEY + index as u8, rank),
            };
            slot.model = design.map(|design| ReedModel::with_mode(design, &self.mode));
        }
        self.keys[index].stale = false;
    }

    /// Brings stale keys up to date: every key in use now, and a few others.
    fn build_stale(&mut self) {
        let mut spare = BUILDS_PER_BLOCK;
        for index in 0..ALL_KEYS {
            if !self.keys[index].stale {
                continue;
            }
            if !self.keys[index].is_idle() {
                self.build_key(index);
            } else if spare > 0 {
                self.build_key(index);
                spare -= 1;
            }
        }
    }

    /// Starts the reeds the air has just reached: each reed is set moving
    /// into its frame as if released from κ · set · P/(P + P₀) there, P its
    /// side's pressure. Voiced (parameter "Attack Kick"), standing on
    /// Cottingham's observation that a free reed's motion "begins with an
    /// initial displacement of the reed tongue into the reed frame" (ICA
    /// 2019).
    ///
    /// The start belongs to the air's arrival at the reed (docs/ROADMAP.md,
    /// 7c and 8c). A reed is owed it while its key is down, its register
    /// open and its side's pressure above P₀. It is given whole once the air
    /// is in its own cell, above P₀ -- so the start and the push of the air
    /// arriving through the opening pallet come together -- and the share's
    /// rise as the bellows' pressure rises after. The air gone --
    /// the key up, the register shut, the bellows' pressure at or below P₀
    /// as it stops or turns -- it is owed again. A key pressed before the
    /// bellows moves, or held through a reversal, starts as one pressed into
    /// a moving bellows does.
    ///
    /// Checked at every step, so it does not hang on how the host cuts its
    /// blocks. `given` is the share the reed has had; `blow` its side's
    /// pressure, zero when its register is shut.
    fn start_into_frame(
        model: &ReedModel,
        state: &mut ReedState,
        given: &mut f64,
        down: bool,
        blow: f64,
        kick: f64,
    ) {
        if !down || blow <= parameters::KICK_PRESSURE {
            *given = 0.0;
            return;
        }
        // Nothing until the air is in the reed's cell: then the start the
        // bellows' pressure gives, at once, and later rises of that pressure
        // as they come.
        if *given == 0.0 && state.cell_pressure <= parameters::KICK_PRESSURE {
            return;
        }
        // Nothing at the threshold itself, so a bellows that starts to move
        // gives the start as its pressure rises rather than half of it the
        // moment it passes P₀ (8j); a key pressed into a blowing bellows
        // gets all but P₀/P of it at once, as before.
        let share = blow / (blow + parameters::KICK_PRESSURE);
        if share > *given {
            state.velocity += model.omega * kick * model.design.set * (share - *given);
            *given = share;
        }
    }

    /// Which way the bellows is to move: the player's direction, or the
    /// other way once Auto Reverse has turned it.
    fn direction(&self) -> f64 {
        let direction = self.parameters.direction();
        if self.flipped { -direction } else { direction }
    }

    /// Auto Reverse (milestone 8g): with the bellows settled in its
    /// direction, it turns at a gap -- nothing held -- once 70 % of its
    /// travel is spent, or when all of it is. Checked every sample, so it
    /// does not hang on how the host cuts its blocks; what is held is looked
    /// at only once the 70 % is reached.
    fn turn_if_spent(&mut self, travel: f64) {
        if self.spent < 0.7 * travel || self.turn != self.direction() {
            return;
        }
        if self.spent >= travel || !self.anything_held() {
            self.flipped = !self.flipped;
            self.spent = 0.0;
        }
    }

    /// The air the reeds drew in the last step, m³/s: for measuring.
    pub fn draw(&self) -> f64 {
        self.draw
    }

    /// A reed parameter moved: every key's reeds are out of date. They are
    /// built again as they are needed, or a few per block meanwhile.
    fn rebuild(&mut self) {
        let design = self.parameters.reed_design();
        if design.mode_ratio != self.mode.ratio_asked {
            self.mode = TongueMode::with_ratio(design.mode_ratio);
        }
        for key in self.keys.iter_mut() {
            key.stale = true;
        }
        self.pallet_design = self.parameters.pallet_design();
        if self.decimator.factor() != self.parameters.oversampling() {
            self.decimator = Decimator::new(self.parameters.oversampling());
            self.zone_decimators =
                core::array::from_fn(|_| Decimator::new(self.parameters.oversampling()));
        }
        self.dirty = false;
    }

    /// Renders one block of mono output, in units of 1 Pa at 1 m times the
    /// gain: the instrument alone, as every measurement takes it.
    pub fn render(&mut self, output: &mut [f32]) {
        self.render_with(output.len(), false, &mut |n, mono, _| output[n] = mono);
    }

    /// Renders one block in stereo as the chosen microphones hear it in the
    /// room (milestone 9b), each layout brought to the dry instrument's
    /// loudness at 1 m, with Dry the mono render in both channels; then
    /// recorded as an engineer sets the preamp (milestone 9c): at
    /// [`RECORDING_LEVEL`], under a soft [`ceiling`].
    pub fn render_stereo(&mut self, left: &mut [f32], right: &mut [f32]) {
        let frames = left.len().min(right.len());
        self.render_with(frames, true, &mut |n, l, r| {
            left[n] = ceiling(l * RECORDING_LEVEL);
            right[n] = ceiling(r * RECORDING_LEVEL);
        });
    }

    /// Where the bass box is: the bellows' opening, m.
    pub fn bellows_opening(&self) -> f64 {
        let area = self.parameters.bellows_air().area;
        if area > 0.0 { self.opened / area } else { 0.0 }
    }

    fn render_with(&mut self, frames: usize, stereo: bool, sink: &mut dyn FnMut(usize, f32, f32)) {
        if self.dirty {
            self.rebuild();
        }
        let area = self.parameters.bellows_air().area.max(1.0e-6);
        if stereo && self.stage_dirty {
            self.stage
                .tune(&self.parameters, self.parameters.travel() / area);
            self.stage_dirty = false;
        }
        let staged = stereo && self.stage.is_active();
        self.build_stale();
        let factor = self.decimator.factor();
        let h = 1.0 / (f64::from(self.sample_rate) * factor as f64);
        // The arm: the wheel or the pedal is the player's hand already, and
        // the bellows follows it as it comes.
        let asked = self.bellows.intent();
        let rate = f64::from(self.sample_rate);
        let smoothing = 1.0 - math::exp(-h / SUPPLY_SMOOTHING_SECONDS);
        // Turning, the bellows takes its pressure through zero: the turn goes
        // from -1 to +1, or back, at a steady rate over the reversal time.
        let turning = 2.0 * h / self.parameters.reversal_time();
        // The bellows' travel, when it runs out (Auto Reverse).
        let travel = (self.parameters.get(parameters::AUTO_REVERSE) == Some(1.0)).then(|| {
            self.parameters
                .get(parameters::BELLOWS_TRAVEL)
                .unwrap_or(12.0)
                * 1.0e-3
        });
        let gain = self.parameters.get(parameters::GAIN).unwrap_or(1.0) as f32;
        let kick = self.parameters.get(parameters::ATTACK_KICK).unwrap_or(0.0);
        // The ranks the registers let the bellows reach, on either side.
        let open = self.parameters.open_ranks();
        let open_bass = self.parameters.open_bass_ranks();
        // The bellows' air, when the intent is the arm's push; otherwise the
        // intent is the pressure.
        let wind = self.parameters.wind_design();
        // The cassotto L and M sound into, when there is one.
        let cassotto = self
            .parameters
            .cassotto()
            .map(|(resonance, q)| cassotto::CassottoTuning::new(resonance, q, h));
        let mut chunk = [0.0f32; decimator::MAX_FACTOR];
        // The keys with anything to compute, in order. Between calls is the
        // only time a key is pressed, so none joins them during this one;
        // one that falls idle stays listed, and computing an idle key
        // changes nothing (milestone 10).
        let mut sounding = [0u8; ALL_KEYS];
        let mut count = 0;
        for (number, key) in self.keys.iter().enumerate() {
            if !key.is_idle() {
                sounding[count] = number as u8;
                count += 1;
            }
        }
        let sounding = &sounding[..count];
        let travel_air = self.parameters.travel();
        // The bellows' walls: the mass law's pole, 2ρc/m rad/s, at the
        // oversampled rate.
        let walls = 1.0 - math::exp(-2.0 * AIR_IMPEDANCE / BELLOWS_WALL_MASS * h);
        for n in 0..frames {
            // Where the bass box is: the air let through -- pulling opens
            // the bellows, pushing shuts it.
            self.opened -= self.turn * self.draw / rate;
            self.opened = self.opened.clamp(0.0, travel_air);
            let extension = self.opened / area;
            if let Some(travel) = travel {
                self.turn_if_spent(travel);
            }
            let direction = self.direction();
            // The bellows holds its pressure with or without a key down.
            let target = self.parameters.bellows_pressure(asked);
            if sounding
                .iter()
                .all(|&number| self.keys[usize::from(number)].is_idle())
                && self.at_rest(sounding)
            {
                // Every pallet is shut, nothing moves and nothing is left in
                // the filter. The bellows keeps moving as asked, and only
                // its leaks and the air button spend its air.
                for cassotto in self.cassotto.iter_mut() {
                    cassotto.reset();
                }
                self.inside = [0.0; stage::SOURCES];
                self.draw = 0.0;
                self.ask = target;
                self.supply = match &wind {
                    Some(design) => self.wind.step(design, target, 0.0, h * factor as f64),
                    None => target,
                };
                self.turn = toward(self.turn, direction, turning * factor as f64);
                // The room rings on after the instrument has stopped.
                let (left, right) = if staged {
                    let (left, right) = self.stage.process(&[0.0; stage::SOURCES], extension);
                    (left as f32 * gain, right as f32 * gain)
                } else {
                    (0.0, 0.0)
                };
                sink(n, left, right);
                continue;
            }
            let mut zone_chunks = [[0.0f32; decimator::MAX_FACTOR]; stage::SOURCES];
            for (slot, mono) in chunk.iter_mut().enumerate().take(factor) {
                self.ask += (target - self.ask) * smoothing;
                self.supply = match &wind {
                    Some(design) => self.wind.step(design, self.ask, self.draw, h),
                    None => {
                        // Kept in step, so turning the arm on never starts
                        // from an empty bellows.
                        self.wind.pressure = self.ask;
                        self.ask
                    }
                };
                self.turn = toward(self.turn, direction, turning);
                let signed = self.turn * self.supply;
                // Each reed is blown only from its own side, and only while
                // its rank's register is open; the other's valve is shut,
                // and its reed sees nothing of the bellows. The flow through
                // the hole is inward on pull and outward on push; what
                // radiates is the outward flow's rate.
                // Each source's: the treble's quarters, then the bass box.
                let mut outward = [0.0f64; stage::SOURCES];
                // Each source's tongues' volume acceleration, S_r ζ''.
                let mut inside_flow = [0.0f64; stage::SOURCES];
                // What L and M send into the cassotto, when there is one.
                let mut boxed = [0.0f64; stage::TREBLE_SOURCES];
                // The air every reed's hole passes, drawn from the bellows.
                let mut drawn = 0.0;
                for &number in sounding {
                    let number = usize::from(number);
                    let key = &mut self.keys[number];
                    if key.is_idle() {
                        continue;
                    }
                    let bass = number >= BASS_START;
                    let source = if bass {
                        stage::BASS_SOURCE
                    } else {
                        number * stage::TREBLE_SOURCES / compass::KEYS
                    };
                    key.pallet.advance(&self.pallet_design, h);
                    key.chord.advance(&self.pallet_design, h);
                    for (index, rank) in key.ranks.iter_mut().enumerate() {
                        let Some(model) = &rank.model else {
                            continue;
                        };
                        // Each rank's cell has its own hole under the key's
                        // pallet, whose curtain grows with the holes it
                        // covers: each rank sees about its own (MODEL.md).
                        // On the bass side a chord rank's hole is under the
                        // chord pallet too, and passes what the wider of the
                        // two lets through.
                        let hole = model.design.tone_hole_area;
                        let rim = model.hole_rim;
                        let chorded = bass && Parameters::is_chord_rank(index);
                        let mut area = key.pallet.area_by_rim(&self.pallet_design, hole, rim);
                        let mut down = key.pallet.target > 0.0;
                        if chorded {
                            area = area.max(key.chord.area_by_rim(&self.pallet_design, hole, rim));
                            down |= key.chord.target > 0.0;
                        }
                        let opened = if bass { open_bass[index] } else { open[index] };
                        // The bass side has no cassotto (Llanos-Vázquez,
                        // thesis 2015).
                        let boxed_here = !bass && Parameters::in_cassotto(index);
                        let into = if cassotto.is_some() && boxed_here {
                            &mut boxed[source]
                        } else {
                            &mut outward[source]
                        };
                        for (((which, state), tube), given) in rank
                            .states
                            .iter_mut()
                            .enumerate()
                            .zip(rank.tubes.iter_mut())
                            .zip(&mut rank.started)
                        {
                            let (side, sign) = if which == PULL_REED {
                                ((-signed).max(0.0), -1.0)
                            } else {
                                (signed.max(0.0), 1.0)
                            };
                            let blow = if opened { side } else { 0.0 };
                            Self::start_into_frame(model, state, given, down, blow, kick);
                            if blow == 0.0 && *state == ReedState::default() {
                                continue;
                            }
                            let swing = state.velocity;
                            *into += sign * reed::step(model, state, tube, blow, area, h);
                            inside_flow[source] +=
                                sign * model.effective_area * (state.velocity - swing) / h;
                            drawn += state.hole_flow;
                            // An unblown reed rings down; once it is
                            // negligible it stops exactly, and is no longer
                            // computed. A shut pallet seals the reed's cell
                            // as surely as a shut register or a still
                            // bellows: the bellows' pressure no longer
                            // reaches it (`reed::step`), and it rings down
                            // the same way (milestone 10c; it was computed
                            // until it underflowed, tens of seconds).
                            if (blow == 0.0 || area == 0.0) && state.energy(model) < 1.0e-12 {
                                *state = ReedState::default();
                                *tube = Tube::default();
                            }
                        }
                    }
                }
                self.draw = drawn;
                if travel.is_some() {
                    self.spent += drawn.abs() * h;
                }
                if let Some(tuning) = &cassotto {
                    for (quarter, cassotto) in self.cassotto.iter_mut().enumerate() {
                        outward[quarter] += cassotto.process(tuning, boxed[quarter]);
                    }
                }
                for ((source, flow), heard) in outward
                    .iter_mut()
                    .zip(inside_flow)
                    .zip(self.inside.iter_mut())
                {
                    *heard += walls * (flow - *heard);
                    *source += *heard;
                }
                if staged {
                    for (chunk, source) in zone_chunks.iter_mut().zip(outward) {
                        chunk[slot] = (RADIATION * source) as f32;
                    }
                } else {
                    *mono = (RADIATION * outward.iter().sum::<f64>()) as f32;
                }
            }
            if staged {
                let mut sources = [0.0f64; stage::SOURCES];
                for ((source, decimator), chunk) in sources
                    .iter_mut()
                    .zip(self.zone_decimators.iter_mut())
                    .zip(&zone_chunks)
                {
                    *source = f64::from(decimator.decimate(&chunk[..factor]));
                }
                let (left, right) = self.stage.process(&sources, extension);
                sink(n, left as f32 * gain, right as f32 * gain);
            } else {
                let mono = self.decimator.decimate(&chunk[..factor]) * gain;
                sink(n, mono, mono);
            }
        }
    }

    /// True once every reed has stopped and the filter has emptied. A
    /// reed's state is zeroed exactly once its energy is negligible, so the
    /// silence that follows is exact too. Only the `sounding` keys can hold
    /// anything: the others were idle, so still, when the block began.
    fn at_rest(&mut self, sounding: &[u8]) -> bool {
        if self.decimator.is_quiet()
            && self.zone_decimators.iter().all(Decimator::is_quiet)
            && sounding
                .iter()
                .all(|&number| self.keys[usize::from(number)].is_still())
        {
            return true;
        }
        // A sounding F4 reed stores a few millijoules; 1e-12 J is about
        // 98 dB below it.
        for &number in sounding {
            let key = &mut self.keys[usize::from(number)];
            for rank in key.ranks.iter_mut() {
                let Some(model) = &rank.model else {
                    continue;
                };
                for (state, tube) in rank.states.iter_mut().zip(rank.tubes.iter_mut()) {
                    if state.energy(model) < 1.0e-12 {
                        *state = ReedState::default();
                        *tube = Tube::default();
                    }
                }
            }
        }
        false
    }
}

/// Where `key` sits in the compass, if it has reeds.
fn compass_index(key: u8) -> Option<usize> {
    let index = usize::from(key.checked_sub(compass::FIRST_KEY)?);
    (index < compass::KEYS).then_some(index)
}

/// `from` moved toward `to` by at most `by`.
fn toward(from: f64, to: f64, by: f64) -> f64 {
    if from < to {
        (from + by).min(to)
    } else {
        (from - by).max(to)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use std::vec;

    #[test]
    fn only_a_usable_sample_rate_makes_an_engine() {
        assert!(Engine::new(48_000.0).is_ok());
        assert!(Engine::new(44_100.0).is_ok());
        assert_eq!(Engine::new(0.0).err(), Some(EngineError::SampleRate));
        assert_eq!(Engine::new(f32::NAN).err(), Some(EngineError::SampleRate));
        assert_eq!(Engine::new(1.0e6).err(), Some(EngineError::SampleRate));
    }

    #[test]
    fn keys_are_tracked_and_a_reset_lets_them_all_go() {
        let mut engine = Engine::new(48_000.0).unwrap();
        engine.note_on(69, 0.5);
        engine.note_on(72, 0.5);
        assert!(engine.is_held(69) && engine.is_held(72));
        assert_eq!(engine.held_count(), 2);
        engine.note_off(69);
        assert!(!engine.is_held(69));
        engine.note_on(200, 1.0);
        assert_eq!(engine.held_count(), 1, "a key outside MIDI is ignored");
        engine.bellows_mut().expression_wide(0.5);
        engine.reset();
        assert_eq!(engine.held_count(), 0);
        assert_eq!(
            engine.bellows().intent(),
            0.5,
            "a reset leaves the bellows alone"
        );
    }

    #[test]
    fn parameters_refuse_what_they_cannot_be() {
        let mut engine = Engine::new(48_000.0).unwrap();
        assert!(engine.set_parameter(parameters::GAIN, 2.0));
        assert!(!engine.set_parameter(parameters::GAIN, -0.1));
        assert!(!engine.set_parameter(parameters::GAIN, f64::INFINITY));
        assert_eq!(engine.parameter(parameters::GAIN), Some(2.0));
    }

    /// The whole instrument, its microphones and its room live inside the
    /// engine, which never allocates. The component is linked with an 8 MiB
    /// stack (.cargo/config.toml) and the engine is moved a few times as it
    /// is built: it must stay inside a sixth of it. An eighth until the
    /// cells became tubes (milestone 8m), whose waves take 269 KiB.
    #[test]
    fn a_whole_instrument_fits_the_stack() {
        let bytes = core::mem::size_of::<Engine>();
        std::println!("Engine: {} KiB", bytes / 1024);
        assert!(bytes < 8 * 1024 * 1024 / 6, "{bytes} bytes");
    }

    #[test]
    fn only_the_compass_sounds_and_silence_is_exact() {
        let mut engine = Engine::new(48_000.0).unwrap();
        let mut block = [1.0f32; 256];
        engine.note_on(40, 1.0);
        engine.render(&mut block);
        assert!(
            block.iter().all(|sample| *sample == 0.0),
            "E2 is below the treble's compass"
        );
        engine.note_off(40);
        engine.note_on(REED_KEY, 0.8);
        let mut sounding = [0.0f32; 48_000];
        engine.render(&mut sounding);
        assert!(sounding.iter().any(|sample| sample.abs() > 1.0e-4));
        engine.note_off(REED_KEY);
        // At Q 250 the tongue rings down with a 0.22 s time constant.
        let mut tail = vec![1.0f32; 4 * 48_000];
        engine.render(&mut tail);
        assert!(tail[3 * 48_000..].iter().all(|sample| *sample == 0.0));
    }
}
