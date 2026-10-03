//! Why Musette notes do not come out in "Frère Jacques", measured before
//! anything is changed: `cargo test --release -p rf-musette-analysis --test
//! musette_diagnosis -- --ignored --nocapture`. The engine as the lab drives
//! it (the arm at CC 11 = 80, pulling), C4 in Musette; each rank's tongue
//! traced on its own, so the sum's envelope can be told from each reed's.
//! Prints; asserts nothing.

use rf_musette_dsp::parameters::{self, RANK_FLAT, RANK_MIDDLE, RANK_SHARP};
use rf_musette_dsp::{Engine, PULL_REED};

const RATE: f32 = 48_000.0;
const MUSETTE: f64 = 8.0;
const CLARINET: f64 = 11.0;

/// Amplitude and phase of `x` at `frequency` over a window `centre` ± 20 ms.
fn phasor(x: &[f64], rate: f64, frequency: f64, centre: f64) -> (f64, f64) {
    let half = (0.02 * rate) as usize;
    let mid = (centre * rate) as usize;
    let (a, b) = (mid.saturating_sub(half), (mid + half).min(x.len()));
    let (mut re, mut im, mut weight) = (0.0, 0.0, 0.0);
    for (i, value) in x[a..b].iter().enumerate() {
        let n = (a + i) as f64;
        let w = 0.5 - 0.5 * (2.0 * std::f64::consts::PI * i as f64 / (b - a) as f64).cos();
        let angle = 2.0 * std::f64::consts::PI * frequency * n / rate;
        re += w * value * angle.cos();
        im -= w * value * angle.sin();
        weight += w;
    }
    (2.0 * (re * re + im * im).sqrt() / weight, im.atan2(re))
}

#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn a_musette_note_rank_by_rank() {
    trace(MUSETTE);
    println!("Clarinet, the same key:");
    trace(CLARINET);
}

fn trace(register: f64) {
    let key = 60u8;
    let mut engine = Engine::new(RATE).unwrap();
    // The bellows played: Key Touch off, or it rests (9h again).
    assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
    assert!(engine.set_parameter(parameters::REGISTER, register));
    engine.bellows_mut().expression_msb(80);
    let mut block = [0.0f32; 64];
    for _ in 0..(1.5 * RATE / 64.0) as usize {
        engine.render(&mut block);
    }
    engine.note_on(key, 100.0 / 127.0);
    let ranks = [RANK_FLAT, RANK_MIDDLE, RANK_SHARP];
    let mut zeta = vec![Vec::new(); 3];
    let mut out = Vec::new();
    let mut supply = Vec::new();
    for _ in 0..(0.6 * RATE) as usize {
        let mut one = [0.0f32; 1];
        engine.render(&mut one);
        out.push(f64::from(one[0]));
        supply.push(engine.supply());
        for (trace, rank) in zeta.iter_mut().zip(ranks) {
            trace.push(engine.reed(key, rank, PULL_REED).map_or(0.0, |r| r.1.zeta));
        }
    }
    let frequency: Vec<f64> = ranks
        .iter()
        .map(|&rank| {
            engine
                .reed(key, rank, PULL_REED)
                .map_or(263.0, |r| r.0.design.frequency)
        })
        .collect();
    println!(
        "supply {:.0} Pa; ranks at {frequency:.2?} Hz",
        engine.supply()
    );
    println!("  t ms |  M− amp  phase |  M amp  phase |  M+ amp  phase | sum at M");
    for step in 1..29 {
        let t = 0.02 * f64::from(step);
        let mut line = format!("{:6.0} |", t * 1e3);
        for (trace, f) in zeta.iter().zip(&frequency) {
            // Each rank's phase read against M's frequency, so the drift
            // between them is the beat.
            let (amp, phase) = phasor(trace, f64::from(RATE), frequency[1], t);
            let _ = f;
            line += &format!(" {:7.2} µm {:5.0}° |", amp * 1e6, phase.to_degrees());
        }
        let (sum, _) = phasor(&out, f64::from(RATE), frequency[1], t);
        line += &format!(
            " {:6.1} dB | {:5.0} Pa",
            20.0 * (sum + 1e-12).log10(),
            supply[(t * f64::from(RATE)) as usize]
        );
        println!("{line}");
    }
}

/// The tune's first note: the arm and the key move together.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_first_note_with_the_bellows() {
    let key = 60u8;
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
    assert!(engine.set_parameter(parameters::REGISTER, CLARINET));
    engine.bellows_mut().expression_msb(80);
    engine.note_on(key, 100.0 / 127.0);
    let mut block = [0.0f32; 64];
    for n in 0..(0.4 * RATE / 64.0) as usize {
        engine.render(&mut block);
        if n % 30 == 0 {
            let zeta = engine
                .reed(key, RANK_MIDDLE, PULL_REED)
                .map_or(0.0, |r| r.1.zeta);
            println!(
                "{:5.0} ms  supply {:6.0} Pa  zeta {:8.2} µm",
                (n + 1) as f32 * 64.0 / RATE * 1e3,
                engine.supply(),
                zeta * 1e6
            );
        }
    }
}

/// A key held through a reversal: how soon the push reed takes over.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn a_held_key_through_a_reversal() {
    use rf_musette_dsp::PUSH_REED;
    let key = 65u8;
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
    assert!(engine.set_parameter(parameters::REGISTER, CLARINET));
    engine.bellows_mut().expression_wide(0.4f32.sqrt());
    engine.note_on(key, 1.0);
    let mut block = [0.0f32; 64];
    for _ in 0..(1.0 * RATE / 64.0) as usize {
        engine.render(&mut block);
    }
    if std::env::var("NO_KICK").is_ok() {
        assert!(engine.set_parameter(parameters::ATTACK_KICK, 0.0));
    }
    assert!(engine.set_parameter(parameters::BELLOWS_DIRECTION, parameters::PUSH));
    let mut peak: f64 = 0.0;
    let mut trace = Vec::new();
    for n in 0..(1.0 * RATE / 64.0) as usize {
        engine.render(&mut block);
        let zeta = engine
            .reed(key, RANK_MIDDLE, PUSH_REED)
            .map_or(0.0, |r| r.1.zeta);
        peak = peak.max(zeta.abs());
        trace.push(((n + 1) as f32 * 64.0 / RATE, peak));
    }
    for (t, p) in trace.iter().step_by(75) {
        println!(
            "{:5.0} ms after the turn began: push reed peak {:7.1} µm",
            t * 1e3,
            p * 1e6
        );
    }
}
