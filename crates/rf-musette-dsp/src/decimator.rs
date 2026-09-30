//! Brings the oversampled reed back to the host's rate.
//!
//! The jet through a closing slot is a train of sharp pulses, rich far above
//! the host's Nyquist frequency; computed at the host rate those partials
//! fold back as inharmonic tones. The reed therefore runs at a multiple of
//! the host rate and this low-pass decimates the instrument's sum -- one
//! filter for the whole instrument, not one per reed.
//!
//! A windowed-sinc FIR, Blackman window, 96 taps per unit of the factor: flat
//! within 0.1 dB to 0.40 of the host rate, and past its Nyquist frequency
//! the window's 74 dB.

use crate::math;

pub const MAX_FACTOR: usize = 4;
const TAPS_PER_FACTOR: usize = 96;
const MAX_TAPS: usize = TAPS_PER_FACTOR * MAX_FACTOR;
/// Where the response is half, as a fraction of the host rate.
const CUTOFF: f64 = 0.46;

pub struct Decimator {
    factor: usize,
    taps: [f32; MAX_TAPS],
    length: usize,
    history: [f32; MAX_TAPS],
    position: usize,
    /// Consecutive zero inputs: once the whole history is zero, so is the
    /// output, without computing it.
    quiet: usize,
}

impl Decimator {
    /// A decimator by `factor`, which must be 1, 2 or 4.
    pub fn new(factor: usize) -> Self {
        let factor = factor.clamp(1, MAX_FACTOR);
        let length = if factor == 1 {
            1
        } else {
            TAPS_PER_FACTOR * factor
        };
        let mut taps = [0.0f32; MAX_TAPS];
        if factor == 1 {
            taps[0] = 1.0;
        } else {
            let pi = core::f64::consts::PI;
            let cutoff = CUTOFF / factor as f64;
            let centre = (length - 1) as f64 / 2.0;
            let mut coefficients = [0.0f64; MAX_TAPS];
            let mut sum = 0.0;
            for (n, coefficient) in coefficients.iter_mut().take(length).enumerate() {
                let t = n as f64 - centre;
                let sinc = if t == 0.0 {
                    2.0 * cutoff
                } else {
                    math::sin_cos(2.0 * pi * cutoff * t).0 / (pi * t)
                };
                let phase = 2.0 * pi * n as f64 / (length - 1) as f64;
                let window =
                    0.42 - 0.5 * math::sin_cos(phase).1 + 0.08 * math::sin_cos(2.0 * phase).1;
                *coefficient = sinc * window;
                sum += *coefficient;
            }
            // Unity gain at DC.
            for (tap, coefficient) in taps.iter_mut().zip(coefficients.iter()).take(length) {
                *tap = (*coefficient / sum) as f32;
            }
        }
        Self {
            factor,
            taps,
            length,
            history: [0.0; MAX_TAPS],
            position: 0,
            quiet: MAX_TAPS,
        }
    }

    pub fn factor(&self) -> usize {
        self.factor
    }

    /// Takes `factor` input samples and returns one output sample.
    #[inline]
    pub fn decimate(&mut self, input: &[f32]) -> f32 {
        for &sample in input.iter().take(self.factor) {
            self.history[self.position] = sample;
            self.position = if self.position + 1 == self.length {
                0
            } else {
                self.position + 1
            };
            if sample == 0.0 {
                self.quiet = self.quiet.saturating_add(1);
            } else {
                self.quiet = 0;
            }
        }
        if self.quiet >= self.length {
            return 0.0;
        }
        // The newest sample is just behind `position`.
        let mut sum = 0.0f32;
        let mut index = self.position;
        for tap in self.taps.iter().take(self.length) {
            index = if index == 0 {
                self.length - 1
            } else {
                index - 1
            };
            sum += tap * self.history[index];
        }
        sum
    }

    /// True while the filter holds nothing but zeros.
    pub fn is_quiet(&self) -> bool {
        self.quiet >= self.length
    }

    pub fn reset(&mut self) {
        self.history = [0.0; MAX_TAPS];
        self.position = 0;
        self.quiet = MAX_TAPS;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gain_at(factor: usize, frequency_over_host_rate: f64) -> f64 {
        let mut decimator = Decimator::new(factor);
        let pi = core::f64::consts::PI;
        // RMS, not peak: the samples of a tone near Nyquist miss its crest.
        let mut energy = 0.0f64;
        let mut chunk = [0.0f32; MAX_FACTOR];
        for n in 0..5000 {
            for (k, sample) in chunk.iter_mut().take(factor).enumerate() {
                let t = (n * factor + k) as f64 / factor as f64;
                *sample = math::sin_cos(2.0 * pi * frequency_over_host_rate * t).0 as f32;
            }
            let out = f64::from(decimator.decimate(&chunk[..factor]));
            if n >= 1000 {
                energy += out * out;
            }
        }
        math::sqrt(2.0 * energy / 4000.0)
    }

    #[test]
    fn the_passband_passes_and_what_would_fold_back_is_stopped() {
        for factor in [2, 4] {
            let passband = gain_at(factor, 0.40);
            assert!(
                (passband - 1.0).abs() < 0.012,
                "factor {factor}: {passband}"
            );
            // Just above the host's Nyquist frequency, and far above it.
            for stop in [0.52, 0.7, 0.9] {
                let leaked = gain_at(factor, stop);
                assert!(
                    leaked < 10f64.powf(-70.0 / 20.0),
                    "factor {factor} at {stop}: {leaked}"
                );
            }
        }
    }

    #[test]
    fn a_factor_of_one_is_the_identity() {
        let mut decimator = Decimator::new(1);
        assert_eq!(decimator.decimate(&[0.25]), 0.25);
        assert_eq!(decimator.decimate(&[-1.0]), -1.0);
    }
}
