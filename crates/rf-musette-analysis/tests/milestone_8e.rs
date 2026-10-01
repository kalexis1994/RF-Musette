//! Milestone 8e: the bass's inlet ducts, and a bellows that at audio
//! frequencies is its air. Predictions as written in docs/ROADMAP.md before
//! the code; each test says which.

use rf_musette_analysis::{LOW_REED, speaks_on_bellows};
use rf_musette_dsp::compass::{
    BASS_KEYS, FIRST_KEY, KEYS, bass_design, bass_target, design, target,
};
use rf_musette_dsp::parameters::{self, BASS_16, BASS_RANKS, Parameters, RANKS};
use rf_musette_dsp::{Engine, PULL_REED};

const RATE: f32 = 48_000.0;

/// Prediction 3: under the arm at CC 11 = 80, the 16′ C2 grows, its swing
/// passing 3 mm within 2 s.
#[test]
fn the_lowest_bass_speaks_under_the_arm() {
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(parameters::BASS_REGISTER, 6.0));
    engine.bellows_mut().expression_msb(80);
    let mut one = [0.0f32; 1];
    for _ in 0..(0.5 * RATE) as usize {
        engine.render(&mut one);
    }
    engine.bass_on(48, 1.0);
    let mut peak = 0.0f64;
    for _ in 0..(2.0 * RATE) as usize {
        engine.render(&mut one);
        peak = peak.max(
            engine
                .bass_reed(0, BASS_16, PULL_REED)
                .unwrap()
                .1
                .zeta
                .abs(),
        );
    }
    println!("16′ C2 under the arm: {:.2} mm", peak * 1e3);
    assert!(peak > 3.0e-3, "{:.2} mm", peak * 1e3);
}

/// Prediction 4: every reed under 300 Hz speaks, sustained, at 300 Pa and
/// 1 kPa on the engine's bellows.
#[test]
fn every_low_reed_speaks_on_the_bellows() {
    let p = Parameters::default();
    let pressures = [300.0, 1000.0];
    let mut silent = Vec::new();
    for rank in 0..RANKS {
        for index in 0..KEYS {
            let key = FIRST_KEY + index as u8;
            let aim = target(&p, key, rank);
            if aim < LOW_REED
                && !speaks_on_bellows(&p, design(&p, key, rank).unwrap(), aim, &pressures)
            {
                silent.push(format!("treble rank {rank} key {key}"));
            }
        }
    }
    for rank in 0..BASS_RANKS {
        for pitch_class in 0..BASS_KEYS {
            let aim = bass_target(&p, pitch_class, rank).unwrap();
            let reed = bass_design(&p, pitch_class, rank).unwrap();
            if aim < LOW_REED && !speaks_on_bellows(&p, reed, aim, &pressures) {
                silent.push(format!("bass rank {rank} pitch class {pitch_class}"));
            }
        }
    }
    println!("silent: {silent:?}");
    assert!(silent.is_empty(), "{silent:?}");
}
