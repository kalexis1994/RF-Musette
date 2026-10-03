//! Milestone 9f: instruments by their mechanics. Predictions as written in
//! docs/ROADMAP.md before the code; each test says which.

use rf_musette_dsp::Engine;
use rf_musette_dsp::parameters;
use rf_musette_dsp::programs::program;

/// The bellows' pressure, as a share of the program's ceiling, under the
/// full Master chord of both hands pushed as hard as the player can.
fn share_of_ceiling(id: &str) -> f64 {
    let settings = program(id).unwrap().parameters();
    let mut engine = Engine::new(48_000.0).unwrap();
    for index in 0..parameters::COUNT {
        assert!(engine.set_parameter(index, settings.get(index).unwrap()));
    }
    // The bellows played: Key Touch off, or it rests (9h again).
    assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
    assert!(engine.set_parameter(parameters::REGISTER, 6.0));
    assert!(engine.set_parameter(parameters::MIC_LAYOUT, 6.0));
    engine.bellows_mut().expression_wide(1.0);
    for key in [60, 64, 67, 72, 76] {
        engine.note_on(key, 1.0);
    }
    engine.bass_on(36, 1.0);
    engine.chord_on(48, 1.0);
    let mut block = [0.0f32; 480];
    let mut pressures = Vec::new();
    for n in 0..150 {
        engine.render(&mut block);
        if n >= 50 {
            pressures.push(engine.supply().abs());
        }
    }
    let mean = pressures.iter().sum::<f64>() / pressures.len() as f64;
    mean / settings.get(parameters::BELLOWS_CEILING).unwrap()
}

/// Prediction 2: the arm on the smaller bellows sags further, as a share of
/// its ceiling, than on the largest.
#[test]
fn the_smaller_bellows_sags_further() {
    let small = share_of_ceiling("student-48");
    let large = share_of_ceiling("cassotto-pro");
    println!(
        "full chord pushed hard: 48-bass {:.0} %, cassotto professional {:.0} % of the ceiling",
        small * 100.0,
        large * 100.0
    );
    assert!(small < large, "{small} {large}");
}
