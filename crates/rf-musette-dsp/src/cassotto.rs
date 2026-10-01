//! The cassotto: a box the 16′ and true 8′ reed blocks sound into, with a
//! narrow way out.
//!
//! Below the box's own standing waves it is a Helmholtz resonator, and the
//! flow that leaves follows the flow the blocks send in through
//!
//! ```text
//! H(s) = ω₀² / (s² + (ω₀/Q) s + ω₀²)
//! ```
//!
//! -- a lift around ω₀ and −12 dB per octave above, unity at low frequency.
//! ω₀ is Richter's measured shaft resonance, 800 Hz-1 kHz (IfM Zwota,
//! Demusa report 1989); Q is assumed. It acts on the sound only, never on
//! the reeds (Llanos-Vázquez, thesis 2015, p174). Discretised as a
//! topology-preserving state-variable filter (Zavalishin, *The Art of VA
//! Filter Design*), stable at any frequency and Q.

/// The filter's two integrators.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Cassotto {
    first: f64,
    second: f64,
}

/// Coefficients for one resonance, Q and step.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CassottoTuning {
    g: f64,
    k: f64,
}

impl CassottoTuning {
    /// `resonance` Hz and `q`, stepped every `h` seconds.
    pub fn new(resonance: f64, q: f64, h: f64) -> Self {
        // Prewarped: the bilinear map keeps the resonance where it is asked.
        let angle = (core::f64::consts::PI * resonance * h).min(1.5);
        let (sin, cos) = crate::math::sin_cos(angle);
        Self {
            g: sin / cos,
            k: 1.0 / q,
        }
    }
}

impl Cassotto {
    /// The flow that leaves for `input` sent in, one step.
    pub fn process(&mut self, tuning: &CassottoTuning, input: f64) -> f64 {
        let CassottoTuning { g, k } = *tuning;
        let high = (input - (k + g) * self.first - self.second) / (1.0 + g * (k + g));
        let band = g * high + self.first;
        self.first = g * high + band;
        let low = g * band + self.second;
        self.second = g * band + low;
        low
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    /// The steady gain at `frequency`, by driving a sine through it.
    fn gain(resonance: f64, q: f64, frequency: f64) -> f64 {
        let rate = 96_000.0;
        let tuning = CassottoTuning::new(resonance, q, 1.0 / rate);
        let mut filter = Cassotto::default();
        let (mut peak_in, mut peak_out) = (0.0f64, 0.0f64);
        for n in 0..(rate as usize) {
            let x = (2.0 * core::f64::consts::PI * frequency * n as f64 / rate).sin();
            let y = filter.process(&tuning, x);
            if n > rate as usize / 2 {
                peak_in = peak_in.max(x.abs());
                peak_out = peak_out.max(y.abs());
            }
        }
        peak_out / peak_in
    }

    #[test]
    fn it_passes_the_low_lifts_the_resonance_and_cuts_the_high() {
        assert!((gain(900.0, 2.0, 50.0) - 1.0).abs() < 0.01);
        assert!(
            (gain(900.0, 2.0, 900.0) - 2.0).abs() < 0.02,
            "Q at resonance"
        );
        // Two octaves above, −12 dB per octave from the resonance: ~1/15.
        let high = gain(900.0, 2.0, 3600.0);
        assert!((0.05..0.08).contains(&high), "{high}");
    }
}
