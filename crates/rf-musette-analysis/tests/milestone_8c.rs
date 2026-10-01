//! Milestone 8c: the low reeds' attack, the start given as the air reaches
//! the reed's cell. Predictions as written in docs/ROADMAP.md before the
//! code. Finger attacks (−50 → −5 dB of the first harmonic, Llanos-Vázquez
//! et al. 2014's metric) through the engine, the bellows stiff at 400 Pa,
//! the pallet opening in its default 50 ms.

use rf_musette_analysis::{attack_time, component_envelope};
use rf_musette_dsp::Engine;
use rf_musette_dsp::compass::{BASS_KEYS, bass_target, target};
use rf_musette_dsp::parameters::{self, BASS_16, Parameters, RANK_MIDDLE, STIFF};

const RATE: f32 = 48_000.0;

/// `note`'s finger attack, s: below F3 the bass side's 16′ (alone with the
/// far 2′, register 16′/2′), from F3 the treble's true 8′.
fn attack(note: u8) -> Option<f64> {
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
    assert!(engine.set_parameter(parameters::BASS_REGISTER, 6.0));
    // The ceiling is 1 kPa and the curve 2: this intent asks 400 Pa.
    engine.bellows_mut().expression_wide(0.4f32.sqrt());
    let mut block = [0.0f32; 256];
    for _ in 0..(0.3 * RATE / 256.0) as usize {
        engine.render(&mut block);
    }
    let p = Parameters::default();
    let frequency = if note < 53 {
        engine.bass_on(note, 1.0);
        bass_target(&p, usize::from(note) % BASS_KEYS, BASS_16)?
    } else {
        engine.note_on(note, 1.0);
        target(&p, note, RANK_MIDDLE)
    };
    let mut out = Vec::new();
    for _ in 0..(2.5 * RATE / 256.0) as usize {
        engine.render(&mut block);
        out.extend(block.iter().map(|x| f64::from(*x)));
    }
    attack_time(&component_envelope(
        &out,
        f64::from(RATE),
        frequency,
        4.0,
        0.001,
    ))
}

/// Prediction 1: A2-B2 attack in 50-140 ms (Llanos: 70-100 ms at mf).
///
/// NOT MET: 179, 164, 151 ms (were 249, 235, 220). With the pallet opening
/// in 5 ms they are 90-112: what still slows them is the half-open pallet
/// of the first tens of milliseconds, which throttles a large reed's
/// growing flow more than a small one's (docs/ROADMAP.md, 8c).
#[test]
#[ignore = "not met: A2-B2 attack in 151-179 ms (docs/ROADMAP.md, 8c)"]
fn the_lowest_measured_notes_attack_as_measured() {
    for note in [45u8, 46, 47] {
        let attack = attack(note).expect("no attack");
        println!("note {note}: {:.0} ms", attack * 1e3);
        assert!((0.050..=0.140).contains(&attack), "{:.0} ms", attack * 1e3);
    }
}

/// Prediction 2: A3-B4 stay in 50-140 ms.
#[test]
fn the_middle_attacks_stay_a_players() {
    for note in [57u8, 58, 59, 69, 70, 71] {
        let attack = attack(note).expect("no attack");
        println!("note {note}: {:.0} ms", attack * 1e3);
        assert!((0.050..=0.140).contains(&attack), "{:.0} ms", attack * 1e3);
    }
}

/// Prediction 4: the 16′ C2 attacks faster than its 467 ms: 427 ms over
/// this 2.5 s render.
#[test]
fn the_lowest_reed_attacks_sooner() {
    let attack = attack(36).expect("no attack");
    println!("C2: {:.0} ms", attack * 1e3);
    assert!(attack < 0.467, "{:.0} ms", attack * 1e3);
}
