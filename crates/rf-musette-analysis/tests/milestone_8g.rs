//! Milestone 8g: the bellows' travel -- Auto Reverse. Predictions as
//! written in docs/ROADMAP.md before the code; each test says which.

use rf_musette_dsp::parameters::{self, RANK_MIDDLE, STIFF};
use rf_musette_dsp::{Engine, PUSH_REED};

const RATE: f32 = 48_000.0;
const MASTER: f64 = 6.0;

fn engine(auto: bool, travel_litres: f64) -> Engine {
    engine_in(auto, travel_litres, MASTER)
}

fn engine_in(auto: bool, travel_litres: f64, register: f64) -> Engine {
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
    assert!(engine.set_parameter(parameters::REGISTER, register));
    assert!(engine.set_parameter(parameters::AUTO_REVERSE, if auto { 1.0 } else { 0.0 }));
    assert!(engine.set_parameter(parameters::BELLOWS_TRAVEL, travel_litres));
    // The ceiling is 1 kPa and the curve 2: 300 Pa.
    engine.bellows_mut().expression_wide(0.3f32.sqrt());
    engine
}

/// Renders one sample at a time for `seconds`, calling `each` with the
/// signed pressure and the air drawn after every sample.
fn run(engine: &mut Engine, seconds: f32, mut each: impl FnMut(&Engine, usize)) {
    let mut one = [0.0f32; 1];
    for n in 0..(seconds * RATE) as usize {
        engine.render(&mut one);
        each(engine, n);
    }
}

/// Prediction 1: off, a chord held for 10 s never turns the bellows, even
/// with the shortest travel.
#[test]
fn off_the_bellows_never_runs_out() {
    let mut engine = engine(false, 2.0);
    for key in [60, 64, 67, 72] {
        engine.note_on(key, 1.0);
    }
    let mut turned = false;
    run(&mut engine, 10.0, |engine, _| {
        turned |= engine.supply() > 0.0
    });
    assert!(!turned, "the bellows turned with Auto Reverse off");
}

/// Prediction 2: on, a chord held, the bellows turns once the air drawn
/// reaches the travel, within 5 %.
#[test]
fn a_held_chord_turns_the_bellows_when_its_air_is_spent() {
    let travel = 2.0e-3;
    let mut engine = engine(true, travel * 1e3);
    for key in [60, 64, 67, 72] {
        engine.note_on(key, 1.0);
    }
    let mut spent = 0.0;
    let mut at_turn = None;
    run(&mut engine, 20.0, |engine, _| {
        if at_turn.is_none() {
            // Pulling, the pressure is below zero; the turn has begun when
            // it climbs back past most of it.
            if spent > 1.0e-4 && engine.supply() > -250.0 {
                at_turn = Some(spent);
            }
            spent += engine.draw().abs() / f64::from(RATE);
        }
    });
    let at_turn = at_turn.expect("the bellows never turned");
    println!("turned after {:.2} L of {:.2}", at_turn * 1e3, travel * 1e3);
    assert!(
        (at_turn / travel - 1.0).abs() < 0.05,
        "{:.2} L",
        at_turn * 1e3
    );
}

/// Prediction 3: on, notes with gaps between them, every turn falls in a
/// gap and none before 70 % of the travel; prediction 4, after it the push
/// reed sounds. One true 8′ draws ~0.4 L/s: 2 L run out over many notes,
/// and a gap comes between the 70 % and the whole.
#[test]
fn the_bellows_turns_in_a_gap() {
    let travel = 2.0e-3;
    let mut engine = engine_in(true, travel * 1e3, 11.0);
    let rate = RATE as usize;
    // Notes of 400 ms, gaps of 100 ms.
    let note_on = |n: usize| n % (rate / 2) < rate * 2 / 5;
    let mut one = [0.0f32; 1];
    let mut spent = 0.0;
    let mut turns = Vec::new();
    let mut previous = -1.0f64;
    let mut held = false;
    let mut push_peak = 0.0f64;
    for n in 0..(20 * rate) {
        let down = note_on(n);
        if down && !held {
            engine.note_on(60, 1.0);
        } else if !down && held {
            engine.note_off(60);
        }
        held = down;
        engine.render(&mut one);
        let direction = engine.supply().signum();
        if direction != 0.0 && previous != 0.0 && direction != previous {
            turns.push((n, held, spent));
            spent = 0.0;
        }
        if direction != 0.0 {
            previous = direction;
        }
        spent += engine.draw().abs() / f64::from(RATE);
        if !turns.is_empty() && held {
            let (_, push) = engine.reed(60, RANK_MIDDLE, PUSH_REED).unwrap();
            push_peak = push_peak.max(push.zeta.abs());
        }
    }
    println!(
        "turns: {:?}",
        turns
            .iter()
            .map(|(n, held, spent)| format!(
                "{:.2} s held {held} after {:.2} L",
                *n as f32 / RATE,
                spent * 1e3
            ))
            .collect::<Vec<_>>()
    );
    assert!(!turns.is_empty(), "the bellows never turned");
    // The pressure crosses zero half a reversal time after the turn began,
    // so the turn itself began in the gap if the crossing is within 50 ms of
    // a gap's start.
    for (n, _, spent) in &turns {
        let into_cycle = n % (rate / 2);
        let gap_start = rate * 2 / 5;
        assert!(
            into_cycle >= gap_start && into_cycle <= gap_start + rate / 20 + rate / 10,
            "a turn at {:.3} s is not in a gap",
            *n as f32 / RATE
        );
        assert!(
            *spent >= 0.7 * travel * 0.95,
            "turned after {:.2} L",
            spent * 1e3
        );
    }
    // Prediction 4: after the first turn the push reed sounds.
    println!("push reed's swing after the turn {:.2} mm", push_peak * 1e3);
    assert!(push_peak > 2.0e-3, "{:.2} mm", push_peak * 1e3);
}
