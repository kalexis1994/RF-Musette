//! Milestone 7b, continued: the start belongs to the air's arrival.
//! Predictions as written in docs/ROADMAP.md before the code. The first
//! harmonic's rise from −50 to −5 dB (Llanos-Vázquez et al. 2014's metric),
//! through the engine with the arm.

use rf_musette_analysis::{attack_time, component_envelope};
use rf_musette_dsp::Engine;
use rf_musette_dsp::compass::target;
use rf_musette_dsp::parameters::{self, Parameters, RANK_MIDDLE};

const RATE: f32 = 48_000.0;
const CLARINET: f64 = 11.0;
const KEY: u8 = 60;

fn render(engine: &mut Engine, seconds: f32) -> Vec<f64> {
    let mut out = vec![0.0f32; (seconds * RATE) as usize];
    for block in out.chunks_mut(64) {
        engine.render(block);
    }
    out.into_iter().map(f64::from).collect()
}

fn attack(out: &[f64]) -> Option<f64> {
    let frequency = target(&Parameters::default(), KEY, RANK_MIDDLE);
    attack_time(&component_envelope(
        out,
        f64::from(RATE),
        frequency,
        4.0,
        0.001,
    ))
}

fn clarinet() -> Engine {
    let mut engine = Engine::new(RATE).unwrap();
    // The bellows played: Key Touch off, or it rests (9h again).
    assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
    assert!(engine.set_parameter(parameters::REGISTER, CLARINET));
    engine
}

/// Prediction 7: the arm and the key move together, as the tune's first
/// note does.
#[test]
fn a_note_begun_with_the_bellows_starts() {
    let mut engine = clarinet();
    engine.bellows_mut().expression_msb(80);
    engine.note_on(KEY, 100.0 / 127.0);
    let out = render(&mut engine, 2.0);
    let attack = attack(&out).expect("no attack");
    println!("with the bellows: {:.0} ms", attack * 1e3);
    assert!((0.050..=0.140).contains(&attack), "{:.0} ms", attack * 1e3);
}

/// Prediction 8: the key held with the bellows still, then the arm pushes
/// 400 Pa's worth.
#[test]
fn a_held_key_starts_when_the_air_comes() {
    let mut engine = clarinet();
    engine.bellows_mut().expression_wide(0.0);
    engine.note_on(KEY, 1.0);
    render(&mut engine, 0.3);
    // The ceiling is 1 kPa and the curve 2: this intent asks 400 Pa.
    engine.bellows_mut().expression_wide(0.4f32.sqrt());
    let out = render(&mut engine, 2.0);
    let attack = attack(&out).expect("no attack");
    println!("held, then the air: {:.0} ms", attack * 1e3);
    assert!((0.050..=0.140).contains(&attack), "{:.0} ms", attack * 1e3);
}
