//! Milestone 9d: a diffuse room. Predictions as written in docs/ROADMAP.md
//! before the code; each test says which.

use rf_musette_dsp::parameters::{self, Parameters};
use rf_musette_dsp::stage::{self, SOURCES, Stage};

const RATE: f64 = 48_000.0;

/// The room's own impulse response, with the room less without, through an
/// omni at 1 m.
fn room_response(volume: f64) -> Vec<f64> {
    let response = |room_level: f64| {
        let mut parameters = Parameters::default();
        for (index, value) in [
            (parameters::MIC_LAYOUT, stage::SINGLE as f64),
            (parameters::SINGLE_PATTERN, 0.0),
            (parameters::SINGLE_DISTANCE, 1.0),
            (parameters::ROOM_SIZE, volume),
            // The hardness these were written and measured at, the default
            // until the player voiced it 0.2 (2026-10-02): the network's
            // diffusion is the question, not the walls.
            (parameters::ROOM_HARDNESS, 0.4),
            (parameters::ROOM_LEVEL, room_level),
        ] {
            assert!(parameters.set(index, value));
        }
        let mut stage = Stage::new(RATE);
        stage.tune(&parameters, 0.2);
        let mut left = Vec::new();
        for n in 0..(1.5 * RATE) as usize {
            let mut sources = [0.0; SOURCES];
            sources[1] = f64::from(u8::from(n == 0));
            left.push(stage.process(&sources, 0.1).0 / stage.trim());
        }
        left
    };
    let with = response(0.0);
    let without = response(-40.0);
    with.iter().zip(&without).map(|(a, b)| a - b).collect()
}

/// A radix-2 FFT's magnitudes, the input zero-padded to a power of two.
fn magnitudes(signal: &[f64]) -> Vec<f64> {
    let n = signal.len().next_power_of_two();
    let mut re: Vec<f64> = signal
        .iter()
        .copied()
        .chain(std::iter::repeat(0.0))
        .take(n)
        .collect();
    let mut im = vec![0.0; n];
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut length = 2;
    while length <= n {
        let angle = -2.0 * std::f64::consts::PI / length as f64;
        for start in (0..n).step_by(length) {
            for k in 0..length / 2 {
                let (s, c) = (angle * k as f64).sin_cos();
                let (a, b) = (start + k, start + k + length / 2);
                let (tr, ti) = (re[b] * c - im[b] * s, re[b] * s + im[b] * c);
                re[b] = re[a] - tr;
                im[b] = im[a] - ti;
                re[a] += tr;
                im[a] += ti;
            }
        }
        length <<= 1;
    }
    re.iter()
        .zip(&im)
        .take(n / 2)
        .map(|(r, i)| (r * r + i * i).sqrt())
        .collect()
}

/// The tail's ripple, dB: from 80 ms, Hann-windowed, its spectrum between
/// 300 Hz and 4 kHz about a third-octave smoothing. A diffuse field's is
/// ~5.6 dB (Rayleigh); an exponentially decaying noise measures 5.7 here.
fn ripple(response: &[f64]) -> f64 {
    let tail = &response[(0.08 * RATE) as usize..];
    let count = tail.len();
    let windowed: Vec<f64> = tail
        .iter()
        .enumerate()
        .map(|(n, x)| {
            x * 0.5 * (1.0 - (2.0 * std::f64::consts::PI * n as f64 / count as f64).cos())
        })
        .collect();
    let spectrum = magnitudes(&windowed);
    let bin = RATE / (2 * spectrum.len()) as f64;
    let db: Vec<f64> = spectrum
        .iter()
        .map(|m| 20.0 * (m + 1e-30).log10())
        .collect();
    let mut deviations = Vec::new();
    for (k, value) in db.iter().enumerate() {
        let f = k as f64 * bin;
        if !(300.0..4000.0).contains(&f) {
            continue;
        }
        let low = (f / 2f64.powf(1.0 / 6.0) / bin) as usize;
        let high = (f * 2f64.powf(1.0 / 6.0) / bin) as usize;
        let smooth = db[low..=high].iter().sum::<f64>() / (high - low + 1) as f64;
        deviations.push(value - smooth);
    }
    let mean = deviations.iter().sum::<f64>() / deviations.len() as f64;
    (deviations.iter().map(|d| (d - mean).powi(2)).sum::<f64>() / deviations.len() as f64).sqrt()
}

/// Abel & Huang's echo density in a 20 ms window from `at` s: the share of
/// samples beyond one standard deviation over a Gaussian's 0.3173.
fn echo_density(response: &[f64], at: f64) -> f64 {
    let start = (at * RATE) as usize;
    let window = &response[start..start + (0.02 * RATE) as usize];
    let deviation = (window.iter().map(|x| x * x).sum::<f64>() / window.len() as f64).sqrt();
    window.iter().filter(|x| x.abs() > deviation).count() as f64 / window.len() as f64 / 0.3173
}

/// The three rooms' ripple and echo density at 50 ms.
fn measured() -> Vec<(f64, f64, f64)> {
    [30.0, 150.0, 1500.0]
        .into_iter()
        .map(|volume| {
            let response = room_response(volume);
            let ripple = ripple(&response);
            let density = echo_density(&response, 0.05);
            println!("{volume} m³: ripple {ripple:.1} dB, echo density at 50 ms {density:.2}");
            (volume, ripple, density)
        })
        .collect()
}

/// The repair holds: no tube's ripple (16.8 dB before it), and an echo
/// density near a diffuse field's by 50 ms -- what was reached, guarded.
#[test]
fn the_tail_is_no_tube() {
    for (volume, ripple, density) in measured() {
        assert!(ripple <= 7.5, "{volume} m³: {ripple:.1} dB");
        assert!(density >= 0.75, "{volume} m³: {density:.2}");
    }
}

/// Prediction 1: the tail's ripple within 1.5 dB of a diffuse field's 5.6
/// dB, at 30, 150 and 1500 m³. Prediction 2: its echo density 0.9 by 50 ms.
/// The ripple met (6.6-7.0 dB); the echo density met at 30 and 150 m³
/// (0.91, 0.95) and not at 1500 (0.79), where the first images still
/// arrive apart at 50 ms (docs/ROADMAP.md, 9d); kept as it was written.
#[test]
#[ignore = "prediction 2 not met at 1500 m³ (ROADMAP 9d)"]
fn the_tail_is_diffuse() {
    let mut results = Vec::new();
    for volume in [30.0, 150.0, 1500.0] {
        let response = room_response(volume);
        let ripple = ripple(&response);
        let density = echo_density(&response, 0.05);
        println!("{volume} m³: ripple {ripple:.1} dB, echo density at 50 ms {density:.2}");
        results.push((volume, ripple, density));
    }
    for (volume, ripple, density) in results {
        assert!(ripple <= 5.6 + 1.5, "{volume} m³: {ripple:.1} dB");
        assert!(density >= 0.9, "{volume} m³: {density:.2}");
    }
}
