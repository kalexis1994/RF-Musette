//! Milestone 8f: the keyboard player's bellows -- the modulation wheel; and
//! since 2026-10-02, no velocity. Predictions as written in docs/ROADMAP.md
//! before the code; each test says which.

use rf_musette_dsp::Engine;
use rf_musette_dsp::parameters::{self, STIFF};

const RATE: f32 = 48_000.0;

/// The pressure the bellows holds every `step` seconds for `seconds`.
fn supply(engine: &mut Engine, seconds: f32, step: f32) -> Vec<f64> {
    let mut block = vec![0.0f32; (step * RATE) as usize];
    (0..(seconds / step) as usize)
        .map(|_| {
            engine.render(&mut block);
            engine.supply().abs()
        })
        .collect()
}

/// Prediction 2 was velocity's push, smoothed as an arm moves (150 ms). Since
/// 2026-10-02 velocity moves nothing -- an accordion's keys have none, and a
/// soft note took the air from the whole instrument -- so the test is the
/// opposite: with no controller the bellows rests at 300 Pa, a soft note and
/// a hard one alike.
#[test]
fn velocity_never_moves_the_bellows() {
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
    engine.note_on(60, 0.1);
    let soft = *supply(&mut engine, 0.5, 0.01).last().unwrap();
    engine.note_off(60);
    engine.note_on(60, 1.0);
    let hard = *supply(&mut engine, 0.5, 0.01).last().unwrap();
    println!("soft note {soft:.1} Pa, hard note {hard:.1} Pa");
    assert!((soft - 300.0).abs() < 1.0, "{soft:.1} Pa");
    assert_eq!(soft, hard);
}

/// Prediction 1: the wheel asks what Expression asks -- through the plugin,
/// in the plugin's contract tests. Here: with a controller the bellows
/// follows at once (prediction 3).
#[test]
fn a_controller_moves_the_bellows_at_once() {
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
    engine.bellows_mut().expression_wide(0.3);
    supply(&mut engine, 0.2, 0.01);
    engine.bellows_mut().expression_wide(1.0);
    let trace = supply(&mut engine, 0.02, 0.001);
    println!("5 ms after the move: {:.0} Pa", trace[4]);
    assert!(trace[9] > 990.0, "{:.0} Pa at 10 ms", trace[9]);
}
