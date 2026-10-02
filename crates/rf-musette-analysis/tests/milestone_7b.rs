//! Milestone 7b: the finger attack. Predictions as written in
//! docs/ROADMAP.md before the code; each test says which. Through the engine
//! as a player drives it: the bellows already pushing (stiff, so the pressure
//! is the one asked), then the key goes down, and the first harmonic of the
//! sound rises from −50 to −5 dB (Llanos-Vázquez et al. 2014's metric).

use rf_musette_analysis::{attack_time, component_envelope};
use rf_musette_dsp::compass::{design, target};
use rf_musette_dsp::parameters::{self, Parameters, RANK_MIDDLE, STIFF};
use rf_musette_dsp::{Engine, REED_KEY};

const RATE: f32 = 48_000.0;

fn render(engine: &mut Engine, seconds: f32) -> Vec<f64> {
    let mut out = vec![0.0f32; (seconds * RATE) as usize];
    for block in out.chunks_mut(256) {
        engine.render(block);
    }
    out.into_iter().map(f64::from).collect()
}

/// `key`'s finger attack at `pressure`, s, Clarinet (the true 8′ alone).
fn finger_attack(key: u8, pressure: f64) -> Option<f64> {
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
    // The ceiling is 1 kPa and the curve 2: this intent asks `pressure`.
    engine
        .bellows_mut()
        .expression_wide((pressure / 1000.0).sqrt() as f32);
    render(&mut engine, 0.3);
    engine.note_on(key, 1.0);
    let out = render(&mut engine, 2.0);
    let p = Parameters::default();
    // Where the reed sounds: its target, which the tuning put it on.
    let frequency = target(&p, key, RANK_MIDDLE);
    let _ = design(&p, key, RANK_MIDDLE)?;
    attack_time(&component_envelope(
        &out,
        f64::from(RATE),
        frequency,
        4.0,
        0.001,
    ))
}

/// Prediction 3: F4's finger attack is 50-140 ms at 100 Pa and at 400 Pa
/// (Llanos: p 60-140 ms, mf 50-110 ms).
///
/// NOT MET since 8m by a few milliseconds: 58 ms at 100 Pa, 46 at 400.
/// Attack Kick 0.5 is between the F4, which wants less, and the 16′ C2,
/// which wants more (docs/ROADMAP.md, 8m).
#[test]
#[ignore = "known defect: 46 ms at 400 Pa, the kick between treble and bass (docs/ROADMAP.md, 8m)"]
fn the_finger_attack_is_a_players() {
    for pressure in [100.0, 400.0] {
        let attack = finger_attack(REED_KEY, pressure).expect("no attack");
        println!("F4 at {pressure} Pa: {:.0} ms", attack * 1e3);
        assert!((0.050..=0.140).contains(&attack), "{:.0} ms", attack * 1e3);
    }
}

/// Prediction 4: at 400 Pa the true 8′ attacks in 50-140 ms from F3 to A6,
/// with no trend in pitch.
///
/// NOT MET above A5: 106, 103, 99, 83, 73, 53 ms from F3 to A5, then 22 ms at
/// D♯6. The start is a share of the set, and at 400 Pa the high reeds, nearer
/// their thresholds, swing little more than that: they begin almost at their
/// full swing. (Llanos's ~80 ms windows read an instant onset as ~40 ms, so
/// the measured 50 ms floor is partly the window's; but the prediction stood
/// as written.)
#[test]
#[ignore = "known defect: the top attacks too fast (docs/ROADMAP.md, 7b)"]
fn the_finger_attack_holds_across_the_compass() {
    let mut attacks = Vec::new();
    for key in [53u8, 59, 65, 71, 77, 81, 87, 93] {
        let attack = finger_attack(key, 400.0).expect("no attack");
        println!("key {key} at 400 Pa: {:.0} ms", attack * 1e3);
        attacks.push((key, attack));
    }
    for (key, attack) in attacks {
        assert!(
            (0.050..=0.140).contains(&attack),
            "key {key}: {:.0} ms",
            attack * 1e3
        );
    }
}

/// Prediction 6: with the bellows still, pressing a key is silent.
#[test]
fn no_air_no_start() {
    let mut engine = Engine::new(RATE).unwrap();
    engine.bellows_mut().expression_wide(0.0);
    render(&mut engine, 0.1);
    engine.note_on(REED_KEY, 1.0);
    let out = render(&mut engine, 0.5);
    assert!(out.iter().all(|x| *x == 0.0), "a sound with no air");
}
