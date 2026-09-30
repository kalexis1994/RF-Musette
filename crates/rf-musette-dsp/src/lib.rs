//! The RF-Musette engine: a physically modelled accordion, treble side first.
//!
//! One reed so far: the accordion F4 tongue the IfM Zwota measured, in its
//! cell, on key 65, behind the pallet that key lifts. The bellows holds its
//! pressure whether or not a key is down; the pallet is what lets air
//! through. Every other key is tracked and silent. The model and its sources
//! are in [`reed`] and [`pallet`]; every mechanism and constant is entered
//! in the ledger, `docs/MODEL.md`.
//!
//! The crate is `no_std` and never allocates: it runs inside the plugin's
//! WebAssembly component on every RackForge host, the Raspberry Pi included.

#![no_std]

mod bellows;
mod decimator;
pub mod math;
pub mod pallet;
pub mod parameters;
pub mod reed;
pub mod tongue;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineError {
    /// The sample rate is not a finite number inside [`SAMPLE_RATES`].
    SampleRate,
}

pub struct Engine {
    sample_rate: f32,
    parameters: Parameters,
    /// The tongue's profile, solved from its mode ratio; kept, because
    /// solving it takes milliseconds and the ratio rarely moves.
    mode: TongueMode,
    model: ReedModel,
    state: ReedState,
    pallet: Pallet,
    pallet_design: PalletDesign,
    supply: f64,
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
        let model = ReedModel::with_mode(parameters.reed_design(), &mode);
        Ok(Self {
            sample_rate,
            parameters,
            mode,
            model,
            state: ReedState::default(),
            pallet: Pallet::default(),
            pallet_design: parameters.pallet_design(),
            supply: 0.0,
            decimator: Decimator::new(parameters.oversampling()),
            dirty: false,
            held: [false; KEYS],
            bellows: Bellows::new(),
        })
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
        if index != parameters::GAIN {
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
        if key == REED_KEY {
            self.pallet.press(depth);
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

    /// The reed as it stands, for measurement.
    pub fn reed(&self) -> (&ReedModel, &ReedState) {
        (&self.model, &self.state)
    }

    /// The pressure the reed is being blown with, Pa.
    pub fn supply(&self) -> f64 {
        self.supply
    }

    /// Lets every key go and silences the reed. What the bellows was asked
    /// stays asked: resetting the audio does not move the player's arm.
    pub fn reset(&mut self) {
        self.held = [false; KEYS];
        self.state = ReedState::default();
        self.pallet = Pallet::default();
        self.supply = 0.0;
        self.decimator.reset();
    }

    fn rebuild(&mut self) {
        let design = self.parameters.reed_design();
        if design.mode_ratio != self.mode.ratio_asked {
            self.mode = TongueMode::with_ratio(design.mode_ratio);
        }
        self.model = ReedModel::with_mode(design, &self.mode);
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
        let factor = self.decimator.factor();
        let h = 1.0 / (f64::from(self.sample_rate) * factor as f64);
        // The bellows holds its pressure with or without a key down.
        let target = self.parameters.bellows_pressure(self.bellows.intent());
        let smoothing = 1.0 - math::exp(-h / SUPPLY_SMOOTHING_SECONDS);
        let gain = self.parameters.get(parameters::GAIN).unwrap_or(1.0) as f32;
        let hole_area = self.model.design.tone_hole_area;
        let mut chunk = [0.0f32; decimator::MAX_FACTOR];
        for sample in output.iter_mut() {
            if self.pallet.is_closed() && self.at_rest() {
                // The pallet is shut, nothing moves and nothing is left in
                // the filter. The supply keeps tracking the bellows.
                self.supply = target;
                *sample = 0.0;
                continue;
            }
            for slot in chunk.iter_mut().take(factor) {
                self.supply += (target - self.supply) * smoothing;
                self.pallet.advance(&self.pallet_design, h);
                let area = self.pallet.area(&self.pallet_design, hole_area);
                let flow_rate = reed::step(&self.model, &mut self.state, self.supply, area, h);
                *slot = (RADIATION * flow_rate) as f32;
            }
            *sample = self.decimator.decimate(&chunk[..factor]) * gain;
        }
    }

    /// True once the reed has stopped and the filter has emptied. The state
    /// is then zeroed exactly, so the silence that follows is exact too.
    fn at_rest(&mut self) -> bool {
        if self.decimator.is_quiet() && self.state == ReedState::default() {
            return true;
        }
        // A sounding F4 reed stores a few millijoules; 1e-12 J is about
        // 98 dB below it.
        if self.state.energy(&self.model) < 1.0e-12 {
            self.state = ReedState::default();
        }
        false
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

    #[test]
    fn only_the_reed_key_sounds_and_silence_is_exact() {
        let mut engine = Engine::new(48_000.0).unwrap();
        let mut block = [1.0f32; 256];
        engine.note_on(69, 1.0);
        engine.render(&mut block);
        assert!(
            block.iter().all(|sample| *sample == 0.0),
            "A4 has no reed yet"
        );
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
