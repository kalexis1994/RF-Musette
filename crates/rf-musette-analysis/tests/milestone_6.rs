//! Milestone 6: the cassotto L and M sound into. Predictions as written in
//! docs/ROADMAP.md before the code; each test says which. Through the
//! engine, M alone (Clarinet), the arm pushing for ~400 Pa (mf).

use rf_musette_analysis::{Trace, attack_time, component_envelope};
use rf_musette_dsp::parameters;
use rf_musette_dsp::{Engine, REED_KEY};

const RATE: f32 = 48_000.0;
/// The intent that asks 400 Pa of the default bellows (1 kPa × intent²).
const INTENT_400_PA: f32 = 0.632_455_5;

fn engine(cassotto: bool) -> Engine {
    let mut engine = Engine::new(RATE).unwrap();
    // The bellows played: Key Touch off, or it rests (9h again).
    assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
    assert!(engine.set_parameter(parameters::CASSOTTO, if cassotto { 1.0 } else { 0.0 }));
    engine.bellows_mut().expression_wide(INTENT_400_PA);
    engine
}

fn render(engine: &mut Engine, seconds: f32) -> Vec<f64> {
    let mut out = vec![0.0f32; (seconds * RATE) as usize];
    for block in out.chunks_mut(256) {
        engine.render(block);
    }
    out.into_iter().map(f64::from).collect()
}

/// The magnitude of `signal` at `frequency`, Hann-windowed.
fn magnitude(signal: &[f64], frequency: f64) -> f64 {
    let n = signal.len() as f64;
    let omega = 2.0 * std::f64::consts::PI * frequency / f64::from(RATE);
    let (mut re, mut im) = (0.0, 0.0);
    for (i, x) in signal.iter().enumerate() {
        let w = 0.5 - 0.5 * (2.0 * std::f64::consts::PI * i as f64 / n).cos();
        re += w * x * (omega * i as f64).cos();
        im -= w * x * (omega * i as f64).sin();
    }
    (re * re + im * im).sqrt()
}

/// The sound's fundamental, Hz: the spectrum's peak within 3 % below the M
/// reed's mode, where it sounds. Zero crossings will not do: the sound is a
/// pulse train, crossing its mean several times a period.
fn fundamental(signal: &[f64]) -> f64 {
    let mode = parameters::Parameters::default().reed_design().frequency;
    (0..600)
        .map(|i| mode * (0.97 + 0.03 * f64::from(i) / 600.0))
        .max_by(|a, b| magnitude(signal, *a).total_cmp(&magnitude(signal, *b)))
        .unwrap()
}

/// The spectral centroid, Hz: the harmonics' amplitudes weighting their
/// frequencies, up to 20 kHz.
fn centroid(signal: &[f64]) -> f64 {
    let f0 = fundamental(signal);
    println!("fundamental {f0:.2} Hz");
    let count = (20_000.0 / f0) as usize;
    let levels = Trace::harmonics(signal, f64::from(RATE), f0, count);
    let (mut weighted, mut total) = (0.0, 0.0);
    for (k, db) in levels.iter().enumerate() {
        let amplitude = 10f64.powf(db / 20.0);
        weighted += (k + 1) as f64 * f0 * amplitude;
        total += amplitude;
    }
    weighted / total
}

/// Prediction 2: with the cassotto, M's spectral centroid at mf falls to
/// 0.4-0.8 of its centroid outside (Llanos-Vázquez measured 0.69 on E4:
/// 1389 Hz inside, 2013 outside).
#[test]
fn the_cassotto_darkens_the_true_eight_foot() {
    let mut results = [0.0; 2];
    for (slot, on) in results.iter_mut().zip([false, true]) {
        let mut engine = engine(on);
        engine.note_on(REED_KEY, 1.0);
        let out = render(&mut engine, 3.0);
        *slot = centroid(&out[(1.5 * RATE) as usize..]);
    }
    let ratio = results[1] / results[0];
    println!(
        "centroid outside {:.0} Hz, inside {:.0} Hz, ratio {ratio:.2} (Llanos E4: 2013, 1389, 0.69)",
        results[0], results[1]
    );
    assert!((0.4..0.8).contains(&ratio), "ratio {ratio:.2}");
}

/// The finger attack, s: the key goes down with the bellows already
/// pushing, and the first harmonic of the sound rises from −50 to −5 dB.
fn finger_attack(cassotto: bool) -> f64 {
    let mut engine = engine(cassotto);
    render(&mut engine, 0.5);
    engine.note_on(REED_KEY, 1.0);
    let out = render(&mut engine, 2.0);
    let f0 = fundamental(&out[RATE as usize..]);
    attack_time(&component_envelope(&out, f64::from(RATE), f0, 4.0, 0.001)).unwrap()
}

/// Prediction 3: the cassotto moves the finger attack by less than 10 %:
/// it acts on the sound, not on the reed (Llanos-Vázquez, p174).
#[test]
fn the_cassotto_leaves_the_attack_alone() {
    let (outside, inside) = (finger_attack(false), finger_attack(true));
    let change = inside / outside - 1.0;
    println!(
        "attack outside {:.0} ms, inside {:.0} ms ({:+.1} %)",
        outside * 1e3,
        inside * 1e3,
        100.0 * change
    );
    assert!(change.abs() < 0.10, "attack moved {:+.1} %", 100.0 * change);
}
