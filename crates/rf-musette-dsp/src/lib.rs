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

mod bellows;
pub mod cassotto;
pub mod compass;
mod decimator;
pub mod math;
pub mod motion;
pub mod pallet;
pub mod parameters;
pub mod programs;
pub mod reed;
pub mod stage;
pub mod tongue;
pub mod tuning;
pub mod wind;

pub use bellows::{Bellows, BellowsSource};
pub use decimator::Decimator;
use pallet::{Pallet, PalletDesign};
pub use parameters::{COUNT as PARAMETER_COUNT, Parameters, SPECS as PARAMETER_SPECS, Side};
use reed::{ReedModel, ReedState};
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
/// recording's headroom: -22 dB, full scale 116 dB SPL at the 1 m reference
/// (measured again with milestone 9d's room; the room's part of each
/// layout's balance moves the peaks a decibel or two).
pub const RECORDING_LEVEL: f32 = 0.079_433;

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

/// How the bass box follows the wheel's place as the bellows, s: the steps
/// of a 7-bit wheel are 1.6 mm of the box's travel at 12 L, smoothed so its
/// paths to the microphones do not jump.
const OPENING_SMOOTHING_SECONDS: f64 = 0.03;

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
    /// The intent as the arm follows it: velocity's strikes smoothed over
    /// "Bellows Smoothing", a controller's as it comes.
    intent: f64,
    /// Where each note on a treble channel went when it was played, so it
    /// is let go there whatever the split has become since.
    played: [Side; KEYS],
    /// The notes held on the bass and chord channels.
    bass_held: [bool; KEYS],
    chord_held: [bool; KEYS],
    bellows: Bellows,
    /// The modulation wheel as where the bellows is (milestone 8i): its
    /// motion, the samples rendered to time it by, and the air the arm moves,
    /// m³/s, above zero pushing, followed as the moving half's mass allows.
    motion: motion::Motion,
    clock: u64,
    flow: f64,
    /// The wheel's last high and low halves.
    wheel_msb: u8,
    wheel_lsb: u8,
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
            supply: 0.0,
            turn: parameters.direction(),
            flipped: false,
            spent: 0.0,
            decimator: Decimator::new(parameters.oversampling()),
            dirty: false,
            held: [false; KEYS],
            intent: 0.0,
            played: [Side::Treble; KEYS],
            bass_held: [false; KEYS],
            chord_held: [false; KEYS],
            bellows: Bellows::new(),
            motion: motion::Motion::default(),
            clock: 0,
            flow: 0.0,
            wheel_msb: 0,
            wheel_lsb: 0,
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
        // What the wheel is changed: it starts afresh, and gives the bellows
        // back to velocity until a controller moves it.
        if index == parameters::MOD_WHEEL {
            self.motion.forget();
            self.bellows.release_motion();
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

    /// A key goes down at `velocity` (0..=1). On a real accordion the key
    /// only opens its pallet; velocity reaches the sound through the bellows
    /// alone, and only while no Expression controller owns it.
    pub fn note_on(&mut self, key: u8, velocity: f32) {
        if usize::from(key) >= KEYS {
            return;
        }
        self.press(key, 1.0);
        self.bellows.strike(velocity);
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
    /// octave (a V-Accordion sends C3-B3). Velocity reaches the sound
    /// through the bellows only, as on the treble.
    pub fn bass_on(&mut self, key: u8, velocity: f32) {
        if self.hold(key, true, true) {
            self.bellows.strike(velocity);
        }
    }

    pub fn bass_off(&mut self, key: u8) {
        self.hold(key, true, false);
    }

    /// A chord note: `key`'s pitch class sounds on the chord ranks. A
    /// V-Accordion's chord button sends its three; a keyboard player's left
    /// hand sends what it holds.
    pub fn chord_on(&mut self, key: u8, velocity: f32) {
        if self.hold(key, false, true) {
            self.bellows.strike(velocity);
        }
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

    /// The modulation wheel's high seven bits (CC 1). With Mod Wheel on
    /// Pressure it is the push, as Expression is (milestone 8f); on Bellows,
    /// where the bellows is (milestone 8i).
    pub fn wheel_msb(&mut self, value: u8) {
        if !self.parameters.wheel_is_bellows() {
            self.bellows.expression_msb(value);
            return;
        }
        self.wheel_msb = value.min(127);
        self.wheel_lsb = 0;
        self.wheel_moved(false);
    }

    /// The wheel's low seven bits (CC 33). A low half with no high half
    /// before it has nothing to refine and is ignored.
    pub fn wheel_lsb(&mut self, value: u8) {
        if !self.parameters.wheel_is_bellows() {
            self.bellows.expression_lsb(value);
            return;
        }
        if !self.motion.is_known() {
            return;
        }
        self.wheel_lsb = value.min(127);
        self.wheel_moved(true);
    }

    /// The wheel at MIDI 2.0 width, already a fraction of its range.
    pub fn wheel_wide(&mut self, value: f32) {
        if !self.parameters.wheel_is_bellows() {
            self.bellows.expression_wide(value);
            return;
        }
        if value.is_finite() {
            self.bellows.take_by_motion();
            self.motion.moved(
                f64::from(value),
                self.clock,
                f64::from(self.sample_rate),
                false,
            );
        }
    }

    /// The wheel's two halves as one 14-bit position, on one scale whether
    /// or not the low half ever comes: a wheel that starts sending it does
    /// not seem to jump.
    fn wheel_moved(&mut self, refine: bool) {
        let position =
            f64::from(u16::from(self.wheel_msb) << 7 | u16::from(self.wheel_lsb)) / 16383.0;
        self.bellows.take_by_motion();
        self.motion
            .moved(position, self.clock, f64::from(self.sample_rate), refine);
    }

    /// Whether the wheel, as where the bellows is, moves the air now.
    fn driven(&self) -> bool {
        self.bellows.source() == BellowsSource::Motion && self.parameters.wheel_is_bellows()
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
                rank.started = [0.0; 2];
            }
        }
        self.ask = 0.0;
        self.wind = wind::Wind::default();
        self.draw = 0.0;
        for cassotto in self.cassotto.iter_mut() {
            cassotto.reset();
        }
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
        // The arm: velocity's strikes reached over the smoothing time, a
        // controller's at once.
        let asked = f64::from(self.bellows.intent());
        let following = match self.bellows.source() {
            BellowsSource::Velocity => {
                let seconds = self
                    .parameters
                    .get(parameters::BELLOWS_SMOOTHING)
                    .unwrap_or(0.0)
                    * 1e-3;
                if seconds > 0.0 {
                    1.0 - math::exp(-1.0 / (f64::from(self.sample_rate) * seconds))
                } else {
                    1.0
                }
            }
            BellowsSource::Expression | BellowsSource::Motion => 1.0,
        };
        // Driven by the wheel (milestone 8i), the arm moves air rather than
        // pushing: the bellows' air and the most the arm can push.
        let driven = self.driven();
        let air = self.parameters.bellows_air();
        let ceiling = self.parameters.bellows_pressure(1.0);
        let wheel_travel = self.parameters.travel();
        let rate = f64::from(self.sample_rate);
        if driven {
            // The bellows moves one way or the other: never half turned.
            self.turn = if self.turn < 0.0 { -1.0 } else { 1.0 };
        } else {
            self.flow = 0.0;
        }
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
        let travel_air = self.parameters.travel();
        let following_bellows = 1.0 - math::exp(-1.0 / (rate * OPENING_SMOOTHING_SECONDS));
        for n in 0..frames {
            // Where the bass box is: the wheel's place, as the bellows (8i),
            // or the air let through -- pulling opens the bellows, pushing
            // shuts it.
            if driven {
                if let Some(position) = self.motion.position() {
                    self.opened += (position * travel_air - self.opened) * following_bellows;
                }
            } else {
                self.opened -= self.turn * self.draw / rate;
            }
            self.opened = self.opened.clamp(0.0, travel_air);
            let extension = self.opened / area;
            // The wheel up opens the bellows -- pulls -- and down closes it.
            let arm = if driven {
                -self.motion.speed(self.clock, rate) * wheel_travel
            } else {
                0.0
            };
            self.clock = self.clock.wrapping_add(1);
            if let Some(travel) = travel.filter(|_| !driven) {
                self.turn_if_spent(travel);
            }
            let direction = self.direction();
            self.intent += (asked - self.intent) * following;
            // The bellows holds its pressure with or without a key down.
            let target = self.parameters.bellows_pressure(self.intent as f32);
            if self.keys.iter().all(Key::is_idle) && self.at_rest() {
                // Every pallet is shut, nothing moves and nothing is left in
                // the filter. The bellows keeps moving as asked, and only
                // its leaks and the air button spend its air.
                for cassotto in self.cassotto.iter_mut() {
                    cassotto.reset();
                }
                self.draw = 0.0;
                if driven {
                    self.drive(&air, ceiling, arm, h * factor as f64);
                } else {
                    self.ask = target;
                    self.supply = match &wind {
                        Some(design) => self.wind.step(design, target, 0.0, h * factor as f64),
                        None => target,
                    };
                    self.turn = toward(self.turn, direction, turning * factor as f64);
                }
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
                if driven {
                    self.drive(&air, ceiling, arm, h);
                } else {
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
                }
                let signed = self.turn * self.supply;
                // Each reed is blown only from its own side, and only while
                // its rank's register is open; the other's valve is shut,
                // and its reed sees nothing of the bellows. The flow through
                // the hole is inward on pull and outward on push; what
                // radiates is the outward flow's rate.
                // Each source's: the treble's quarters, then the bass box.
                let mut outward = [0.0f64; stage::SOURCES];
                // What L and M send into the cassotto, when there is one.
                let mut boxed = [0.0f64; stage::TREBLE_SOURCES];
                // The air every reed's hole passes, drawn from the bellows.
                let mut drawn = 0.0;
                for (number, key) in self.keys.iter_mut().enumerate() {
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
                        let chorded = bass && Parameters::is_chord_rank(index);
                        let mut area = key.pallet.area(&self.pallet_design, hole);
                        let mut down = key.pallet.target > 0.0;
                        if chorded {
                            area = area.max(key.chord.area(&self.pallet_design, hole));
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
                        for ((which, state), given) in
                            rank.states.iter_mut().enumerate().zip(&mut rank.started)
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
                            *into += sign * reed::step(model, state, blow, area, h);
                            drawn += state.hole_flow;
                            // An unblown reed rings down; once it is
                            // negligible it stops exactly, and is no longer
                            // computed.
                            if blow == 0.0 && state.energy(model) < 1.0e-12 {
                                *state = ReedState::default();
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

    /// One step of the bellows the wheel drives: the arm's air, `arm` m³/s
    /// (above zero pushing), followed over the moving half's time, into the
    /// air the reeds draw from; turning when it drains through zero.
    fn drive(&mut self, air: &wind::WindDesign, ceiling: f64, arm: f64, h: f64) {
        self.flow += (arm - self.flow) * (h / wind::MEAN_TIME).min(1.0);
        if self
            .wind
            .driven(air, ceiling, self.turn * self.flow, self.draw, h)
        {
            self.turn = -self.turn;
        }
        self.supply = self.wind.pressure;
        // Kept in step, so the other mode never starts from elsewhere.
        self.ask = self.supply;
    }

    /// True once every reed has stopped and the filter has emptied. A
    /// reed's state is zeroed exactly once its energy is negligible, so the
    /// silence that follows is exact too.
    fn at_rest(&mut self) -> bool {
        if self.decimator.is_quiet()
            && self.zone_decimators.iter().all(Decimator::is_quiet)
            && self.keys.iter().all(Key::is_still)
        {
            return true;
        }
        // A sounding F4 reed stores a few millijoules; 1e-12 J is about
        // 98 dB below it.
        for key in self.keys.iter_mut() {
            for rank in key.ranks.iter_mut() {
                let Some(model) = &rank.model else {
                    continue;
                };
                for state in rank.states.iter_mut() {
                    if state.energy(model) < 1.0e-12 {
                        *state = ReedState::default();
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
    /// is built: it must stay inside an eighth of it.
    #[test]
    fn a_whole_instrument_fits_the_stack() {
        let bytes = core::mem::size_of::<Engine>();
        std::println!("Engine: {} KiB", bytes / 1024);
        assert!(bytes < 1024 * 1024, "{bytes} bytes");
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
