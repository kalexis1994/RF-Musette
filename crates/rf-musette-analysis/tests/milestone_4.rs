//! Milestone 4: the key's five ranks, the tremolo between its 8′ reeds and
//! the registers that open them. Predictions as written in docs/ROADMAP.md
//! before the code; each test says which.

use rf_musette_analysis::{cents, steady};
use rf_musette_dsp::parameters::{self, Parameters};
use rf_musette_dsp::{Engine, REED_KEY};

const RATE: f64 = 192_000.0;
const PRESSURE: f64 = 300.0;

/// A rank's reed blown alone at 300 Pa: its sounding frequency, Hz.
fn sounding(rank: usize) -> f64 {
    let design = Parameters::default().rank_design(rank).unwrap();
    let (tone, _) = steady(design, RATE, PRESSURE, 1.5, 0.5).expect("no tone");
    tone.frequency
}

/// Predictions 2 and 3: the 8′ reeds beat as the tremolo asks -- M and M+
/// at 3.67 Hz, M and M− at 3.14 Hz, M− and M+ at their sum, each within
/// 5 %.
#[test]
fn the_eight_foot_ranks_beat_as_the_tremolo_asks() {
    let (sharp, flat) = Parameters::default().tremolo_beats();
    println!("asked of the modes: M+ {sharp:+.3} Hz, M− {flat:+.3} Hz");
    let middle = sounding(parameters::RANK_MIDDLE);
    let up = sounding(parameters::RANK_SHARP) - middle;
    let down = sounding(parameters::RANK_FLAT) - middle;
    println!(
        "sounding: M {middle:.3} Hz, M+ {up:+.3} Hz, M− {down:+.3} Hz, M−/M+ {:.3} Hz",
        up - down
    );
    // What the tremolo asks of the modes: 3.67, 3.14 and 6.81 Hz here.
    for (got, want) in [(up, sharp), (-down, -flat), (up - down, sharp - flat)] {
        assert!(
            (got / want - 1.0).abs() < 0.05,
            "beat {got:.3} Hz, asked {want}"
        );
    }
}

/// Prediction 5: L and H sound an octave below and above M, within 15
/// cents, each about as far below its own mode as M is.
#[test]
fn the_sixteen_and_four_foot_ranks_sound_their_octaves() {
    let p = Parameters::default();
    let middle = sounding(parameters::RANK_MIDDLE);
    for (rank, ratio, name) in [
        (parameters::RANK_LOW, 0.5, "L"),
        (parameters::RANK_HIGH, 2.0, "H"),
    ] {
        let design = p.rank_design(rank).unwrap();
        let got = sounding(rank);
        let octave = cents(middle * ratio, got);
        let below_mode = cents(design.frequency, got);
        println!(
            "{name}: {got:.2} Hz, {octave:+.1} cents from M's octave, {below_mode:+.1} cents from its mode ({:.1} mm long)",
            design.length * 1e3
        );
        assert!(octave.abs() < 15.0, "{name} is {octave:+.1} cents off");
    }
    let m_below = cents(p.reed_design().frequency, middle);
    println!("M: {m_below:+.1} cents from its mode");
}

/// The long-term level of key 65 through the engine at 300 Pa, with a
/// register open, dB re 1 Pa at 1 m. The bellows stiff, as when these
/// predictions were written (milestone 5's prediction 1).
fn level(register: f64) -> f64 {
    let rate = 48_000.0f32;
    let mut engine = Engine::new(rate).unwrap();
    assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, parameters::STIFF));
    assert!(engine.set_parameter(parameters::REGISTER, register));
    engine.bellows_mut().expression_wide(0.547_722_6);
    engine.note_on(REED_KEY, 1.0);
    let mut out = vec![0.0f32; (4.0 * rate) as usize];
    for block in out.chunks_mut(256) {
        engine.render(block);
    }
    let tail = &out[(1.5 * rate) as usize..];
    let rms = (tail.iter().map(|x| f64::from(*x).powi(2)).sum::<f64>() / tail.len() as f64).sqrt();
    20.0 * rms.log10()
}

/// Prediction 4: two 8′ ranks sound ~3 dB above one and three ~4.8 dB,
/// within 1 dB: incoherent reeds add their powers.
#[test]
fn the_ranks_add_their_powers() {
    let clarinet = level(11.0);
    let celeste = level(12.0) - clarinet;
    let musette = level(8.0) - clarinet;
    let master = level(6.0) - clarinet;
    println!(
        "Clarinet {clarinet:.1} dB; Celeste {celeste:+.2}, Musette {musette:+.2}, Master {master:+.2} dB"
    );
    assert!((celeste - 3.01).abs() < 1.0, "Celeste {celeste:+.2} dB");
    assert!((musette - 4.77).abs() < 1.0, "Musette {musette:+.2} dB");
}

/// Prediction 1, by construction and by every earlier test: Clarinet, the
/// default register, opens M alone, and a closed register keeps its reeds
/// at rest.
#[test]
fn clarinet_opens_the_measured_reed_alone() {
    let p = Parameters::default();
    assert_eq!(
        p.get(parameters::REGISTER),
        Some(11.0),
        "Clarinet is the default"
    );
    assert_eq!(p.open_ranks(), [false, false, true, false, false]);
    let mut engine = Engine::new(48_000.0).unwrap();
    engine.note_on(REED_KEY, 0.8);
    let mut out = vec![0.0f32; 24_000];
    engine.render(&mut out);
    for rank in [
        parameters::RANK_LOW,
        parameters::RANK_FLAT,
        parameters::RANK_SHARP,
        parameters::RANK_HIGH,
    ] {
        for which in [rf_musette_dsp::PULL_REED, rf_musette_dsp::PUSH_REED] {
            let (_, state) = engine.reed(rank, which).unwrap();
            assert_eq!(*state, Default::default(), "rank {rank} moved");
        }
    }
}
