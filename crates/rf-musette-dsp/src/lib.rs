//! The RF-Musette engine: a physically modelled accordion, treble side first.
//!
//! Nothing here sounds yet. This crate holds what the instrument's contract
//! already fixes -- which keys are down, what the player asks of the bellows,
//! the output gain -- so the reed model has a place to arrive. The reed is
//! milestone 1 in `docs/ROADMAP.md`, and every mechanism it brings is entered
//! in the ledger, `docs/MODEL.md`, with the source that measured it.
//!
//! The crate is `no_std` and never allocates: it runs inside the plugin's
//! WebAssembly component on every RackForge host, the Raspberry Pi included.

#![no_std]

mod bellows;

pub use bellows::{Bellows, BellowsSource};

/// MIDI key numbers the engine tracks.
pub const KEYS: usize = 128;

/// The output gain's ceiling, as a linear factor.
pub const GAIN_MAX: f32 = 2.0;

/// The sample rates the engine accepts, in hertz.
pub const SAMPLE_RATES: core::ops::RangeInclusive<f32> = 8_000.0..=384_000.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineError {
    /// The sample rate is not a finite number inside [`SAMPLE_RATES`].
    SampleRate,
}

pub struct Engine {
    sample_rate: f32,
    gain: f32,
    held: [bool; KEYS],
    bellows: Bellows,
}

impl Engine {
    pub fn new(sample_rate: f32) -> Result<Self, EngineError> {
        if !sample_rate.is_finite() || !SAMPLE_RATES.contains(&sample_rate) {
            return Err(EngineError::SampleRate);
        }
        Ok(Self {
            sample_rate,
            gain: 1.0,
            held: [false; KEYS],
            bellows: Bellows::new(),
        })
    }

    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    pub fn gain(&self) -> f32 {
        self.gain
    }

    /// Sets the output gain. A value outside `0..=GAIN_MAX`, or not a
    /// number, is refused and changes nothing.
    pub fn set_gain(&mut self, gain: f32) -> bool {
        if !gain.is_finite() || !(0.0..=GAIN_MAX).contains(&gain) {
            return false;
        }
        self.gain = gain;
        true
    }

    /// A key goes down at `velocity` (0..=1). On a real accordion the key
    /// only opens its pallet; velocity reaches the sound through the bellows
    /// alone, and only while no Expression controller owns it.
    pub fn note_on(&mut self, key: u8, velocity: f32) {
        let Some(held) = self.held.get_mut(usize::from(key)) else {
            return;
        };
        *held = true;
        self.bellows.strike(velocity);
    }

    pub fn note_off(&mut self, key: u8) {
        if let Some(held) = self.held.get_mut(usize::from(key)) {
            *held = false;
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

    /// Lets every key go. What the bellows was asked stays asked: resetting
    /// the audio does not move the player's arm.
    pub fn reset(&mut self) {
        self.held = [false; KEYS];
    }

    /// Renders one block of mono output. Silent until the reed exists.
    pub fn render(&mut self, output: &mut [f32]) {
        output.fill(0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn the_gain_refuses_what_it_cannot_be() {
        let mut engine = Engine::new(48_000.0).unwrap();
        assert!(engine.set_gain(GAIN_MAX));
        assert!(!engine.set_gain(-0.1));
        assert!(!engine.set_gain(f32::INFINITY));
        assert_eq!(engine.gain(), GAIN_MAX);
    }

    #[test]
    fn it_is_silent_until_the_reed_exists() {
        let mut engine = Engine::new(48_000.0).unwrap();
        engine.note_on(69, 1.0);
        let mut block = [1.0f32; 64];
        engine.render(&mut block);
        assert!(block.iter().all(|sample| *sample == 0.0));
    }
}
