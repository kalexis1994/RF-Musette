//! Milestone 10m: a tremolo that does not loop. Predictions as written in
//! docs/ROADMAP.md before the code; each test says which.

use rf_musette_dsp::Engine;
use rf_musette_dsp::parameters;
use rf_musette_dsp::programs;

const RATE: f32 = 48_000.0;
/// The envelope's window, s.
const HOP: f64 = 0.002;

/// `seconds` of `key` held in `program`, mono, with Reed Drift at `drift`;
/// what comes after the first two and a half seconds, the start left out.
fn held(program: &str, key: u8, drift: f64, seconds: f64) -> Vec<f32> {
    let mut engine = Engine::new(RATE).unwrap();
    let values = programs::program(program).unwrap().parameters();
    for (index, value) in values.values().iter().enumerate() {
        assert!(engine.set_parameter(index, *value));
    }
    assert!(engine.set_parameter(parameters::REED_DRIFT, drift));
    engine.note_on(key, 100.0 / 127.0);
    let mut out = vec![0.0f32; (seconds * f64::from(RATE)) as usize];
    for block in out.chunks_mut(256) {
        engine.render(block);
    }
    out.split_off((2.5 * f64::from(RATE)) as usize)
}

/// The RMS envelope in windows of [`HOP`], its mean taken out.
fn envelope(signal: &[f32]) -> Vec<f64> {
    let hop = (HOP * f64::from(RATE)) as usize;
    let rms: Vec<f64> = signal
        .chunks_exact(hop)
        .map(|w| (w.iter().map(|x| f64::from(*x).powi(2)).sum::<f64>() / hop as f64).sqrt())
        .collect();
    let mean = rms.iter().sum::<f64>() / rms.len() as f64;
    rms.iter().map(|x| x - mean).collect()
}

fn correlation(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len());
    let (a, b) = (&a[..n], &b[..n]);
    let (ma, mb) = (
        a.iter().sum::<f64>() / n as f64,
        b.iter().sum::<f64>() / n as f64,
    );
    let (mut ab, mut aa, mut bb) = (0.0, 0.0, 0.0);
    for (x, y) in a.iter().zip(b) {
        ab += (x - ma) * (y - mb);
        aa += (x - ma) * (x - ma);
        bb += (y - mb) * (y - mb);
    }
    ab / (aa * bb).sqrt()
}

/// The tremolo's pattern: the lag, in windows, from 0.4 to 5 s at which the
/// clocks' envelope is most like itself.
fn pattern(clocks: &[f64]) -> usize {
    let lo = (0.4 / HOP) as usize;
    let hi = (5.0 / HOP) as usize;
    (lo..hi)
        .max_by(|&a, &b| {
            correlation(clocks, &clocks[a..]).total_cmp(&correlation(clocks, &clocks[b..]))
        })
        .unwrap()
}

/// How alike the envelope is with itself three patterns later.
fn alike(envelope: &[f64], period: usize) -> f64 {
    correlation(envelope, &envelope[3 * period..])
}

fn rms_db(signal: &[f32]) -> f64 {
    let mean = signal.iter().map(|x| f64::from(*x).powi(2)).sum::<f64>() / signal.len() as f64;
    10.0 * mean.log10()
}

/// The mean frequency over the signal, Hz, from its first to its last
/// rising zero crossing, each found between two samples.
fn mean_frequency(signal: &[f32]) -> f64 {
    let mut crossings = Vec::new();
    for (n, pair) in signal.windows(2).enumerate() {
        let (a, b) = (f64::from(pair[0]), f64::from(pair[1]));
        if a < 0.0 && b >= 0.0 {
            crossings.push(n as f64 + a / (a - b));
        }
    }
    let (first, last) = (crossings[0], crossings[crossings.len() - 1]);
    (crossings.len() - 1) as f64 * f64::from(RATE) / (last - first)
}

/// Prediction 1: Reed Drift at 0 keeps the reeds as clocks -- Musette
/// Paris's A5 envelope is its own three patterns later (0.996 measured
/// before the drift). Prediction 2: at the default 1.5 cents it is not, at
/// A4 or at A5: under 0.3.
#[test]
fn the_tremolo_no_longer_loops() {
    for key in [69, 81] {
        let clocks = envelope(&held("musette-paris", key, 0.0, 16.0));
        let drifting = envelope(&held("musette-paris", key, 1.5, 16.0));
        let period = pattern(&clocks);
        let (still, moving) = (alike(&clocks, period), alike(&drifting, period));
        println!(
            "key {key}: pattern {:.2} s, three later: clocks {still:.3}, drifting {moving:.3}",
            period as f64 * HOP
        );
        if key == 81 {
            assert!(still > 0.99, "key {key}: the clocks' pattern {still:.3}");
        }
        assert!(moving < 0.3, "key {key}: still looping, {moving:.3}");
    }
}

/// Prediction 3: the drift changes no level -- a held Musette A4 within
/// 0.1 dB of the clocks'. Prediction 4: nor the tuning -- a held Clarinet
/// A4's mean pitch within 0.3 cents of the clocks'.
#[test]
fn the_drift_keeps_the_level_and_the_tuning() {
    let (clocks, drifting) = (
        held("musette-paris", 69, 0.0, 12.0),
        held("musette-paris", 69, 1.5, 12.0),
    );
    let level = rms_db(&drifting) - rms_db(&clocks);
    let (clocks, drifting) = (
        held("research", 69, 0.0, 12.0),
        held("research", 69, 1.5, 12.0),
    );
    let cents = 1200.0 * (mean_frequency(&drifting) / mean_frequency(&clocks)).log2();
    println!("Musette A4 level {level:+.3} dB; Clarinet A4 mean pitch {cents:+.3} cents");
    assert!(level.abs() <= 0.1, "{level:+.3} dB");
    assert!(cents.abs() <= 0.3, "{cents:+.3} cents");
}
