//! Milestone 8f: the keyboard player's bellows -- the modulation wheel, and
//! velocity's push smoothed. Predictions as written in docs/ROADMAP.md
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

/// Prediction 2: velocity alone, from a soft strike to a hard one, the push
/// moves 63 % of the way in the smoothing time (150 ± 15 ms), and does not
/// jump. Read on the push, the intent: the pressure is the intent squared
/// (the curve's 2), so it moves its own share.
#[test]
fn velocity_moves_the_bellows_as_an_arm_would() {
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
    engine.note_on(60, 0.3);
    engine.note_off(60);
    supply(&mut engine, 2.0, 0.01);
    engine.note_on(60, 1.0);
    let trace = supply(&mut engine, 0.6, 0.001);
    // The ceiling is 1 kPa and the curve 2: intent = √(P / 1000).
    let intent: Vec<f64> = trace.iter().map(|p| (p / 1000.0).sqrt()).collect();
    let share = |i: f64| (i - 0.3) / 0.7;
    let reached = intent
        .iter()
        .position(|i| share(*i) >= 0.632)
        .expect("never reached") as f64
        * 1e-3;
    println!("63 % of the push at {:.0} ms", reached * 1e3);
    assert!(
        (0.135..=0.165).contains(&reached),
        "{:.0} ms",
        reached * 1e3
    );
    assert!(
        share(intent[0]) < 0.05,
        "it jumped: {:.2}",
        share(intent[0])
    );
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
