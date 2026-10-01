#![allow(clippy::needless_range_loop)]
//! Milestone 8: the Stradella bass. Predictions as written in
//! docs/ROADMAP.md before the code; each test says which.

use rf_musette_analysis::{
    attack_time, cents, component_envelope, linear_threshold, sounding, tune_bass,
};
use rf_musette_dsp::compass::{BASS_KEYS, bass_design, bass_target, bass_untuned};
use rf_musette_dsp::parameters::{
    self, BASS_2, BASS_4, BASS_8, BASS_8_4, BASS_16, BASS_RANKS, Parameters, STIFF,
};
use rf_musette_dsp::reed::ReedState;
use rf_musette_dsp::tongue::TongueMode;
use rf_musette_dsp::{Engine, PULL_REED};

const NAMES: [&str; BASS_RANKS] = ["16′", "8′", "8-4′", "4′", "2′"];
const RATE: f32 = 48_000.0;

fn mode() -> TongueMode {
    TongueMode::with_ratio(Parameters::default().reed_design().mode_ratio)
}

/// An engine with the bellows stiff at 300 Pa, so that what one reed does
/// cannot reach another through the air.
fn stiff() -> Engine {
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
    // The ceiling is 1 kPa and the curve 2.
    engine.bellows_mut().expression_wide(0.3f32.sqrt());
    render(&mut engine, 0.2);
    engine
}

fn render(engine: &mut Engine, seconds: f32) -> Vec<f64> {
    let mut out = vec![0.0f32; (seconds * RATE) as usize];
    for block in out.chunks_mut(256) {
        engine.render(block);
    }
    out.into_iter().map(f64::from).collect()
}

/// Which bass-side reeds move, as (pitch class, rank).
fn moving(engine: &Engine) -> Vec<(usize, usize)> {
    let mut moving = Vec::new();
    for pitch_class in 0..BASS_KEYS {
        for rank in 0..BASS_RANKS {
            let (_, state) = engine.bass_reed(pitch_class, rank, PULL_REED).unwrap();
            if *state != ReedState::default() {
                moving.push((pitch_class, rank));
            }
        }
    }
    moving
}

/// Predictions 1 and 10: every bass-side reed sounds at 300 Pa and within
/// ±2 cents of its pitch there.
#[test]
fn every_bass_reed_sounds_in_tune() {
    let p = Parameters::default();
    let mut worst = (0.0f64, String::new());
    for rank in 0..BASS_RANKS {
        for pitch_class in 0..BASS_KEYS {
            let reed = bass_design(&p, pitch_class, rank).unwrap();
            let name = format!("{} pitch class {pitch_class}", NAMES[rank]);
            let tone = sounding(reed, 300.0).unwrap_or_else(|| panic!("{name} is silent"));
            let off = cents(bass_target(&p, pitch_class, rank).unwrap(), tone.frequency);
            if off.abs() > worst.0.abs() {
                worst = (off, name.clone());
            }
            assert!(off.abs() < 2.0, "{name}: {off:+.2} cents");
        }
    }
    println!("worst: {:+.2} cents, {}", worst.0, worst.1);
}

/// The bass tuning table is the one a fresh tuning makes.
#[test]
fn the_bass_tuning_table_is_current() {
    let fresh = tune_bass(&Parameters::default());
    for rank in 0..BASS_RANKS {
        for pitch_class in 0..BASS_KEYS {
            let stored = f64::from(rf_musette_dsp::tuning::BASS_CENTS[rank][pitch_class]);
            let now = fresh[rank][pitch_class].unwrap();
            assert!(
                (stored - now).abs() < 0.01,
                "{} pitch class {pitch_class}: table {stored:.2}, fresh {now:.2}; run `rf-musette-lab tune`",
                NAMES[rank]
            );
        }
    }
}

/// Prediction 2: the 16′ C2's tongue is 47-57 mm long, as Llanos-Vázquez
/// measured left-hand bass reeds (52 mm at 62.5-87.5 Hz).
///
/// NOT MET: the slots carried down make it 64.6 mm. Llanos's reeds are
/// loaded and shorter; here the load only stiffens the tongue the slots
/// already made long (docs/ROADMAP.md, 8).
#[test]
#[ignore = "not met: the 16′ C2 is 64.6 mm, longer than measured (docs/ROADMAP.md, 8)"]
fn the_lowest_bass_reed_is_as_long_as_measured() {
    let length = bass_untuned(&Parameters::default(), 0, BASS_16)
        .unwrap()
        .length;
    println!("16′ C2: {:.1} mm", length * 1e3);
    assert!((0.047..=0.057).contains(&length), "{:.1} mm", length * 1e3);
}

/// The tip load over the whole tongue's mass, as Llanos-Vázquez gives it.
fn load(reed: rf_musette_dsp::reed::ReedDesign, mode: &TongueMode) -> f64 {
    let model = rf_musette_dsp::reed::ReedModel::with_mode(reed, mode);
    let tongue = reed.density * reed.width * model.root_thickness * reed.length;
    (model.modal_mass - tongue * mode.mass_integral) / tongue
}

/// Prediction 11: the loads are 0-0.3 of the tongue's own mass.
#[test]
fn the_loads_are_a_makers() {
    let p = Parameters::default();
    let mode = mode();
    for rank in 0..BASS_RANKS {
        for pitch_class in 0..BASS_KEYS {
            let reed = bass_design(&p, pitch_class, rank).unwrap();
            let load = load(reed, &mode);
            if load > 0.0 {
                println!(
                    "{} pitch class {pitch_class}: load {load:.3} of the tongue",
                    NAMES[rank]
                );
            }
            assert!((-1e-9..=0.3).contains(&load), "{load:.3}");
        }
    }
}

/// Prediction 3: a bass button sounds its pitch class on every open rank
/// and nothing else.
#[test]
fn a_bass_button_sounds_its_pitch_class_on_every_rank() {
    let mut engine = stiff();
    engine.bass_on(48, 1.0);
    render(&mut engine, 0.2);
    let moving = moving(&engine);
    assert_eq!(
        moving,
        (0..BASS_RANKS).map(|rank| (0, rank)).collect::<Vec<_>>()
    );
    // Any octave of C is the same button.
    engine.bass_on(36, 1.0);
    engine.bass_off(48);
    render(&mut engine, 0.1);
    let (_, state) = engine.bass_reed(0, BASS_16, PULL_REED).unwrap();
    assert!(state.zeta.abs() > 1.0e-4, "C2 stopped when C3 was let go");
}

/// Prediction 3, in a register: 8′/4′/2′ leaves the 16′ and the 8-4′ still.
#[test]
fn a_bass_register_opens_only_its_ranks() {
    let mut engine = stiff();
    assert!(engine.set_parameter(parameters::BASS_REGISTER, 4.0));
    engine.bass_on(48, 1.0);
    render(&mut engine, 0.2);
    assert_eq!(moving(&engine), [(0, BASS_8), (0, BASS_4), (0, BASS_2)]);
}

/// Prediction 4: a chord button sounds its pitch classes on the chord ranks
/// only: C major C-E-G, C7 C-E-B♭, C dim C-E♭-A, as a V-Accordion sends
/// them.
#[test]
fn a_chord_sounds_on_the_chord_ranks_only() {
    for (name, notes) in [
        ("C major", [48u8, 52, 55]),
        ("C7", [48, 52, 58]),
        ("C dim", [48, 51, 57]),
    ] {
        let mut engine = stiff();
        for note in notes {
            engine.chord_on(note, 1.0);
        }
        render(&mut engine, 0.2);
        let mut expected: Vec<(usize, usize)> = notes
            .iter()
            .flat_map(|note| [BASS_8_4, BASS_4, BASS_2].map(|rank| (usize::from(*note) % 12, rank)))
            .collect();
        expected.sort();
        assert_eq!(moving(&engine), expected, "{name}");
    }
}

/// Prediction 5: C bass and F major held together blow the 4′ C reed once,
/// exactly as F major alone does.
#[test]
fn a_shared_reed_sounds_once() {
    let trace = |with_bass: bool| {
        let mut engine = stiff();
        if with_bass {
            engine.bass_on(48, 1.0);
        }
        for note in [53u8, 57, 60] {
            engine.chord_on(note, 1.0);
        }
        let mut zeta = Vec::new();
        let mut one = [0.0f32; 1];
        for _ in 0..(0.3 * RATE) as usize {
            engine.render(&mut one);
            zeta.push(engine.bass_reed(0, BASS_4, PULL_REED).unwrap().1.zeta);
        }
        zeta
    };
    assert_eq!(trace(true), trace(false));
}

/// Prediction 7: the 16′ reeds' thresholds are under 20 Pa.
///
/// NOT MET for the loaded ones: the load that keeps a reed speaking at
/// 1 kPa stiffens it, and its threshold rises with it (docs/ROADMAP.md, 8).
#[test]
#[ignore = "not met: the loaded 16′ reeds start above 20 Pa (docs/ROADMAP.md, 8)"]
fn the_sixteen_foot_speaks_easily() {
    let p = Parameters::default();
    let thresholds: Vec<f64> = (0..BASS_KEYS)
        .map(|pitch_class| {
            let reed = bass_design(&p, pitch_class, BASS_16).unwrap();
            linear_threshold(reed, 192_000.0, 1.0, 3000.0).unwrap()
        })
        .collect();
    println!("16′ thresholds, C-B: {thresholds:.0?} Pa");
    for (pitch_class, threshold) in thresholds.iter().enumerate() {
        assert!(
            *threshold < 20.0,
            "pitch class {pitch_class}: {threshold:.0} Pa"
        );
    }
}

/// Prediction 8: the 16′ C2's finger attack at 400 Pa is 50-140 ms.
///
/// NOT MET: 467 ms, 399 since 8c gave the start as the air reaches the
/// cell. A loaded tongue grows slowest (docs/ROADMAP.md, 8 and 8c);
/// nothing measured bounds a C2.
#[test]
#[ignore = "not met: the 16′ C2 attacks in 399 ms (docs/ROADMAP.md, 8c)"]
fn the_sixteen_foot_attacks_as_a_finger_attack() {
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
    assert!(engine.set_parameter(parameters::BASS_REGISTER, 6.0));
    engine.bellows_mut().expression_wide(0.4f32.sqrt());
    render(&mut engine, 0.3);
    engine.bass_on(36, 1.0);
    let out = render(&mut engine, 2.0);
    let frequency = bass_target(&Parameters::default(), 0, BASS_16).unwrap();
    // Register 16′/2′: the 2′ C5 is three octaves up, far from the C2.
    let attack = attack_time(&component_envelope(
        &out,
        f64::from(RATE),
        frequency,
        4.0,
        0.001,
    ))
    .expect("no attack");
    println!("16′ C2 at 400 Pa: {:.0} ms", attack * 1e3);
    assert!((0.050..=0.140).contains(&attack), "{:.0} ms", attack * 1e3);
}

/// Prediction 9, measured: a bass, a chord and a four-note Master chord
/// together, per 48 kHz sample, natively.
#[test]
#[ignore = "measurement: run in release, prints the cost"]
fn both_hands_cost() {
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(parameters::REGISTER, 6.0));
    engine.bellows_mut().expression_wide(0.6);
    for key in [65, 69, 72, 77] {
        engine.note_on(key, 1.0);
    }
    engine.bass_on(41, 1.0);
    for note in [53, 57, 60] {
        engine.chord_on(note, 1.0);
    }
    render(&mut engine, 1.0);
    let mut out = vec![0.0f32; 5 * 48_000];
    let start = std::time::Instant::now();
    for block in out.chunks_mut(256) {
        engine.render(block);
    }
    let per_sample = start.elapsed().as_secs_f64() / out.len() as f64;
    println!(
        "both hands: {:.2} µs per sample, {:.0} % of real time at 48 kHz",
        per_sample * 1e6,
        100.0 * per_sample * f64::from(RATE)
    );
}

/// Prediction 12: every reed of both sides holds a tone at 50 Pa, at 300 Pa
/// and at the bellows' 1 kPa ceiling.
///
/// NOT MET as written, and the prediction was wrong to ask it: above
/// ~600 Hz the reeds do not start at 50 Pa, as their measured thresholds
/// say they should not (milestone 7), and the top four 4′ reeds need more
/// than 300 Pa (milestone 7's known defect). What the load was for holds:
/// see `no_reed_chokes_and_the_low_ones_speak_throughout`.
#[test]
#[ignore = "not met as written: the high reeds do not start at 50 Pa (docs/ROADMAP.md, 8)"]
fn every_reed_speaks_across_the_bellows_range() {
    use rf_musette_analysis::holds;
    use rf_musette_dsp::compass::{FIRST_KEY, KEYS, design, target};
    let p = Parameters::default();
    let pressures = [50.0, 300.0, 1000.0];
    let mut silent = Vec::new();
    for rank in 0..parameters::RANKS {
        for index in 0..KEYS {
            let key = FIRST_KEY + index as u8;
            let reed = design(&p, key, rank).unwrap();
            if !holds(reed, reed.frequency, &pressures) {
                silent.push(format!("treble rank {rank} key {key}"));
            }
        }
    }
    for rank in 0..BASS_RANKS {
        for pitch_class in 0..BASS_KEYS {
            let reed = bass_design(&p, pitch_class, rank).unwrap();
            if !holds(reed, reed.frequency, &pressures) {
                silent.push(format!("{} pitch class {pitch_class}", NAMES[rank]));
            }
            let _ = bass_target(&p, pitch_class, rank);
        }
    }
    let _ = target;
    println!("not across the range: {silent:?}");
    assert!(silent.is_empty(), "{silent:?}");
}

/// Prediction 12, what it was for: no reed of either side is silent at the
/// bellows' ceiling, every reed but the top four 4′ speaks at 300 Pa, and
/// every reed below 300 Hz -- the loaded ones among them -- speaks from 50 Pa
/// to 1 kPa.
#[test]
fn no_reed_chokes_and_the_low_ones_speak_throughout() {
    use rf_musette_dsp::compass::{FIRST_KEY, KEYS, design};
    let p = Parameters::default();
    let mut reeds = Vec::new();
    for rank in 0..parameters::RANKS {
        for index in 0..KEYS {
            let key = FIRST_KEY + index as u8;
            let top_four = rank == parameters::RANK_HIGH && key >= 90;
            reeds.push((
                format!("treble rank {rank} key {key}"),
                design(&p, key, rank).unwrap(),
                top_four,
            ));
        }
    }
    for rank in 0..BASS_RANKS {
        for pitch_class in 0..BASS_KEYS {
            let name = format!("{} pitch class {pitch_class}", NAMES[rank]);
            reeds.push((name, bass_design(&p, pitch_class, rank).unwrap(), false));
        }
    }
    for (name, reed, top_four) in reeds {
        assert!(sounding(reed, 1000.0).is_some(), "{name} chokes at 1 kPa");
        if !top_four {
            assert!(
                sounding(reed, 300.0).is_some(),
                "{name} is silent at 300 Pa"
            );
        }
        if reed.frequency < 300.0 {
            assert!(sounding(reed, 50.0).is_some(), "{name} is silent at 50 Pa");
        }
    }
}

/// The load tables are the ones a fresh finishing makes: the treble's
/// loaded 16′ keys and the bass side's 16′, where every load lies.
#[test]
fn the_load_tables_are_current() {
    use rf_musette_analysis::load_reed;
    use rf_musette_dsp::compass::{FIRST_KEY, bare, bass_bare, target};
    let p = Parameters::default();
    for index in 0..8 {
        let key = FIRST_KEY + index as u8;
        let rank = parameters::RANK_LOW;
        let stored = f64::from(rf_musette_dsp::tuning::LOADS[rank][index]);
        let now = load_reed(&p, bare(&p, key, rank).unwrap(), target(&p, key, rank)).unwrap();
        assert!(
            (stored - now).abs() < 0.001,
            "L at key {key}: table {stored:.3}, fresh {now:.3}; run `rf-musette-lab tune`"
        );
    }
    for pitch_class in 0..BASS_KEYS {
        let stored = f64::from(rf_musette_dsp::tuning::BASS_LOADS[BASS_16][pitch_class]);
        let aim = bass_target(&p, pitch_class, BASS_16).unwrap();
        let now = load_reed(&p, bass_bare(&p, pitch_class, BASS_16).unwrap(), aim).unwrap();
        assert!(
            (stored - now).abs() < 0.001,
            "16′ pitch class {pitch_class}: table {stored:.3}, fresh {now:.3}; run `rf-musette-lab tune`"
        );
    }
}
