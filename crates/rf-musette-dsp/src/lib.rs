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
pub mod pallet;
pub mod parameters;
pub mod reed;
pub mod tongue;
pub mod tuning;
pub mod wind;

pub use bellows::{Bellows, BellowsSource};
pub use decimator::Decimator;
use pallet::{Pallet, PalletDesign};
pub use parameters::{COUNT as PARAMETER_COUNT, Parameters, SPECS as PARAMETER_SPECS};
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
}

impl Rank {
    fn is_still(&self) -> bool {
        self.states
            .iter()
            .all(|state| *state == ReedState::default())
    }
}

/// A key of the compass: its pallet and its ranks' plates behind it.
struct Key {
    pallet: Pallet,
    ranks: [Rank; parameters::RANKS],
    /// Its reeds' models are out of date: built again before it next sounds,
    /// or a few at a time while nothing asks.
    stale: bool,
}

impl Key {
    fn is_still(&self) -> bool {
        self.ranks.iter().all(Rank::is_still)
    }

    /// Nothing to compute: the pallet shut and every reed at rest.
    fn is_idle(&self) -> bool {
        self.pallet.is_closed() && self.is_still()
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
    /// [`parameters::RANK_LOW`] .. [`parameters::RANK_HIGH`].
    keys: [Key; compass::KEYS],
    pallet_design: PalletDesign,
    /// What the intent asks, Pa: the pressure the push would make in a
    /// still bellows, guarded against steps.
    ask: f64,
    /// The bellows' air, when the intent is the arm's push.
    wind: wind::Wind,
    /// The air the reeds drew in the last step, m³/s.
    draw: f64,
    /// The cassotto L and M sound into, when the instrument has one.
    cassotto: cassotto::Cassotto,
    /// The bellows' pressure, Pa, without its sign.
    supply: f64,
    /// Which way the bellows moves: -1 pulling, +1 pushing, and in between
    /// while it turns. The signed pressure is `turn × supply`.
    turn: f64,
    decimator: Decimator,
    /// A reed parameter moved: rebuild before the next sample.
    dirty: bool,
    held: [bool; KEYS],
    bellows: Bellows,
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
            ranks: core::array::from_fn(|_| Rank {
                model: None,
                states: [ReedState::default(); 2],
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
            cassotto: cassotto::Cassotto::default(),
            supply: 0.0,
            turn: parameters.direction(),
            decimator: Decimator::new(parameters.oversampling()),
            dirty: false,
            held: [false; KEYS],
            bellows: Bellows::new(),
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
        if index == parameters::BELLOWS_DIRECTION
            && self.held_count() == 0
            && self.keys.iter().all(Key::is_still)
        {
            self.turn = self.parameters.direction();
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

    /// One reed as it stands: of `key`'s `rank` ([`parameters::RANK_LOW`] ..
    /// [`parameters::RANK_HIGH`]), the plate's [`PULL_REED`] or
    /// [`PUSH_REED`], for measurement. `None` outside the compass, or before
    /// the key is first built.
    pub fn reed(&self, key: u8, rank: usize, which: usize) -> Option<(&ReedModel, &ReedState)> {
        let rank = self.keys.get(compass_index(key)?)?.ranks.get(rank)?;
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
        for key in self.keys.iter_mut() {
            key.pallet = Pallet::default();
            for rank in key.ranks.iter_mut() {
                rank.states = [ReedState::default(); 2];
            }
        }
        self.ask = 0.0;
        self.wind = wind::Wind::default();
        self.draw = 0.0;
        self.cassotto.reset();
        self.supply = 0.0;
        self.turn = self.parameters.direction();
        self.decimator.reset();
    }

    /// Builds one key's reeds from the parameters, tuned as the compass says.
    fn build_key(&mut self, index: usize) {
        let note = compass::FIRST_KEY + index as u8;
        for (rank, slot) in self.keys[index].ranks.iter_mut().enumerate() {
            slot.model = compass::design(&self.parameters, note, rank)
                .map(|design| ReedModel::with_mode(design, &self.mode));
        }
        self.keys[index].stale = false;
    }

    /// Brings stale keys up to date: every key in use now, and a few others.
    fn build_stale(&mut self) {
        let mut spare = BUILDS_PER_BLOCK;
        for index in 0..compass::KEYS {
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
        }
        self.dirty = false;
    }

    /// Renders one block of mono output, in units of 1 Pa at 1 m times the
    /// gain.
    pub fn render(&mut self, output: &mut [f32]) {
        if self.dirty {
            self.rebuild();
        }
        self.build_stale();
        let factor = self.decimator.factor();
        let h = 1.0 / (f64::from(self.sample_rate) * factor as f64);
        // The bellows holds its pressure with or without a key down.
        let target = self.parameters.bellows_pressure(self.bellows.intent());
        let smoothing = 1.0 - math::exp(-h / SUPPLY_SMOOTHING_SECONDS);
        // Turning, the bellows takes its pressure through zero: the turn goes
        // from -1 to +1, or back, at a steady rate over the reversal time.
        let direction = self.parameters.direction();
        let turning = 2.0 * h / self.parameters.reversal_time();
        let gain = self.parameters.get(parameters::GAIN).unwrap_or(1.0) as f32;
        // The ranks the register lets the bellows reach.
        let open = self.parameters.open_ranks();
        // The bellows' air, when the intent is the arm's push; otherwise the
        // intent is the pressure.
        let wind = self.parameters.wind_design();
        // The cassotto L and M sound into, when there is one.
        let cassotto = self
            .parameters
            .cassotto()
            .map(|(resonance, q)| cassotto::CassottoTuning::new(resonance, q, h));
        let mut chunk = [0.0f32; decimator::MAX_FACTOR];
        for sample in output.iter_mut() {
            if self.keys.iter().all(Key::is_idle) && self.at_rest() {
                // Every pallet is shut, nothing moves and nothing is left in
                // the filter. The bellows keeps moving as asked, and only
                // its leaks and the air button spend its air.
                self.cassotto.reset();
                self.ask = target;
                self.draw = 0.0;
                self.supply = match &wind {
                    Some(design) => self.wind.step(design, target, 0.0, h * factor as f64),
                    None => target,
                };
                self.turn = toward(self.turn, direction, turning * factor as f64);
                *sample = 0.0;
                continue;
            }
            for slot in chunk.iter_mut().take(factor) {
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
                let mut outward = 0.0;
                // What L and M send into the cassotto, when there is one.
                let mut boxed = 0.0;
                // The air every reed's hole passes, drawn from the bellows.
                let mut drawn = 0.0;
                for key in self.keys.iter_mut() {
                    if key.is_idle() {
                        continue;
                    }
                    key.pallet.advance(&self.pallet_design, h);
                    for (index, rank) in key.ranks.iter_mut().enumerate() {
                        let Some(model) = &rank.model else {
                            continue;
                        };
                        // Each rank's cell has its own hole under the key's
                        // pallet, whose curtain grows with the holes it
                        // covers: each rank sees about its own (MODEL.md).
                        let area = key
                            .pallet
                            .area(&self.pallet_design, model.design.tone_hole_area);
                        let into = if cassotto.is_some() && Parameters::in_cassotto(index) {
                            &mut boxed
                        } else {
                            &mut outward
                        };
                        for (which, state) in rank.states.iter_mut().enumerate() {
                            let (side, sign) = if which == PULL_REED {
                                ((-signed).max(0.0), -1.0)
                            } else {
                                (signed.max(0.0), 1.0)
                            };
                            let blow = if open[index] { side } else { 0.0 };
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
                if let Some(tuning) = &cassotto {
                    outward += self.cassotto.process(tuning, boxed);
                }
                *slot = (RADIATION * outward) as f32;
            }
            *sample = self.decimator.decimate(&chunk[..factor]) * gain;
        }
    }

    /// True once every reed has stopped and the filter has emptied. A
    /// reed's state is zeroed exactly once its energy is negligible, so the
    /// silence that follows is exact too.
    fn at_rest(&mut self) -> bool {
        if self.decimator.is_quiet() && self.keys.iter().all(Key::is_still) {
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

    /// The whole treble lives inside the engine, which never allocates: it
    /// must fit, with room, the 1 MiB stack a WebAssembly module starts with.
    #[test]
    fn a_whole_treble_fits_the_stack() {
        let bytes = core::mem::size_of::<Engine>();
        std::println!("Engine: {} KiB", bytes / 1024);
        assert!(bytes < 384 * 1024, "{bytes} bytes");
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
