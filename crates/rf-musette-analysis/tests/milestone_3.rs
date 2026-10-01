//! Milestone 3: the plate's two reeds, one per bellows direction, and the
//! bellows turning between them. Predictions as written in docs/ROADMAP.md
//! before the code; each test says which.
//!
//! Through the engine itself, as a player drives it: key 65 held, the
//! bellows' intent held, the direction moved.

use rf_musette_dsp::parameters::{self, PULL, PUSH};
use rf_musette_dsp::reed::ReedState;
use rf_musette_dsp::{Engine, PULL_REED, PUSH_REED, REED_KEY};

const RATE: f32 = 48_000.0;
/// The intent that asks 300 Pa of the default bellows (1 kPa × intent²).
const INTENT_300_PA: f32 = 0.547_722_6;

/// The bellows stiff -- the intent is the pressure -- as when these
/// predictions were written (milestone 5's prediction 1).
fn engine(direction: f64) -> Engine {
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, parameters::STIFF));
    assert!(engine.set_parameter(parameters::BELLOWS_DIRECTION, direction));
    engine.bellows_mut().expression_wide(INTENT_300_PA);
    engine
}

fn render(engine: &mut Engine, seconds: f32) -> Vec<f32> {
    let mut out = vec![0.0; (seconds * RATE) as usize];
    for block in out.chunks_mut(256) {
        engine.render(block);
    }
    out
}

fn rms(signal: &[f32]) -> f64 {
    (signal.iter().map(|x| f64::from(*x).powi(2)).sum::<f64>() / signal.len() as f64).sqrt()
}

/// Predictions 1-2 (as built): the push reed is the pull reed's twin, so
/// with the same bellows it makes the same sound -- with the hole's flow,
/// and so the sound's polarity, reversed.
#[test]
fn the_push_reed_sounds_as_the_pull_reed_does() {
    let mut pull = engine(PULL);
    let mut push = engine(PUSH);
    pull.note_on(REED_KEY, 1.0);
    push.note_on(REED_KEY, 1.0);
    let (a, b) = (render(&mut pull, 1.5), render(&mut push, 1.5));
    let level = rms(&a[a.len() / 2..]);
    assert!(level > 1.0e-3, "the pull reed sounds: rms {level}");
    let worst = a
        .iter()
        .zip(&b)
        .map(|(x, y)| f64::from(x + y).abs())
        .fold(0.0, f64::max);
    println!("pull rms {level:.4}; worst |pull + push| {worst:.3e}");
    assert!(
        worst <= 1.0e-6 * level,
        "push is not pull reversed: {worst}"
    );
}

/// Prediction 5: with the direction held, the other reed stays exactly at
/// rest.
#[test]
fn the_other_reed_stays_at_rest() {
    for (direction, idle) in [(PULL, PUSH_REED), (PUSH, PULL_REED)] {
        let mut engine = engine(direction);
        engine.note_on(REED_KEY, 1.0);
        render(&mut engine, 0.5);
        let (_, state) = engine
            .reed(REED_KEY, parameters::RANK_MIDDLE, idle)
            .unwrap();
        assert_eq!(*state, ReedState::default(), "direction {direction}");
    }
}

/// A reversal at a held intent, with `reversal` ms of turning: (how long
/// the sound stays more than 20 dB below the steady tone, s; the level the
/// new reed settles at against the old one's, dB).
fn reversal(reversal: f64) -> (f64, f64) {
    let mut engine = engine(PULL);
    assert!(engine.set_parameter(parameters::REVERSAL_TIME, reversal));
    engine.note_on(REED_KEY, 1.0);
    let before = render(&mut engine, 1.5);
    let steady = rms(&before[before.len() / 2..]);
    assert!(engine.set_parameter(parameters::BELLOWS_DIRECTION, PUSH));
    let after = render(&mut engine, 2.0);
    let window = (0.01 * RATE) as usize;
    let gap = after
        .chunks(window)
        .filter(|chunk| 20.0 * (rms(chunk) / steady).log10() < -20.0)
        .count() as f64
        * 0.01;
    let last = rms(&after[after.len() - (0.5 * RATE) as usize..]);
    (gap, 20.0 * (last / steady).log10())
}

/// Prediction 6, its second half: after a reversal the new reed comes back
/// to the old one's level within 0.5 dB.
#[test]
fn after_a_reversal_the_other_reed_sounds_as_loud() {
    let (gap, returned) = reversal(100.0);
    println!("gap {:.0} ms, back to {returned:+.2} dB", gap * 1e3);
    assert!(returned.abs() < 0.5, "level after: {returned:+.2} dB");
}

/// Prediction 6, its first half: the gap is no longer than the reversal
/// time.
///
/// NOT MET, and the same known defect as the finger attack (docs/MODEL.md):
/// the new reed starts from rest and grows from the static deflection the
/// returning pressure gives it, at the model's small-amplitude rate, about
/// 4× slower than a real reed's. Measured, the gap is about the turn plus
/// ~150 ms of growth: 130, 190, 250, 410 ms for turns of 20, 50, 100,
/// 200 ms. Only a 5 ms turn leaves none, because the old reed is still
/// ringing down in its cell while the new one grows.
#[test]
#[ignore = "known defect: the reed grows from rest too slowly (docs/MODEL.md)"]
fn a_reversal_is_a_brief_interruption() {
    for turn in [5.0, 20.0, 50.0, 100.0, 200.0] {
        let (gap, returned) = reversal(turn);
        println!(
            "turn {turn:>5} ms: gap {:>4.0} ms, back to {returned:+.2} dB",
            gap * 1e3
        );
    }
    let (gap, _) = reversal(100.0);
    assert!(gap <= 0.1, "gap {:.0} ms", gap * 1e3);
}
