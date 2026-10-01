//! The bass side's reeds, measured before anything is changed: `cargo test
//! --release -p rf-musette-analysis --test bass_diagnosis -- --ignored
//! --nocapture`. For C on each bass-side rank, and every pitch class of the
//! 16′: the geometry the slots carried down give, the linear threshold, and
//! whether and where the reed sounds at 300 Pa. Prints; asserts nothing.

use rf_musette_analysis::{linear_threshold, sounding};
use rf_musette_dsp::compass::{bass_note, bass_target, bass_untuned, design};
use rf_musette_dsp::parameters::{BASS_16, Parameters, RANK_MIDDLE};
use rf_musette_dsp::reed::{ReedDesign, ReedModel};
use rf_musette_dsp::tongue::TongueMode;

fn line(name: &str, mut reed: ReedDesign, aim: f64) {
    reed.frequency = aim;
    let threshold = linear_threshold(reed, 192_000.0, 1.0, 3000.0);
    let mode = TongueMode::with_ratio(reed.mode_ratio);
    let model = ReedModel::with_mode(reed, &mode);
    let tongue = reed.density * reed.width * model.root_thickness * reed.length;
    let load = (model.modal_mass - tongue * mode.mass_integral) / tongue;
    let yield_300 = model.mu * 300.0 / (model.omega * model.omega) / reed.set;
    let tone = sounding(reed, 300.0);
    println!(
        "{name}: {aim:6.1} Hz, load {load:4.2} (h0 {:4.2} mm), yield {yield_300:4.2} set, L {:4.1} mm, W {:4.2} mm, set {:4.2} mm, Q {:5.1}, cell {:6.2} cm³, hole {:5.1} mm², threshold {:>6.0?} Pa, at 300 Pa {}",
        model.root_thickness * 1e3,
        reed.length * 1e3,
        reed.width * 1e3,
        reed.set * 1e3,
        reed.q,
        reed.cell_volume * 1e6,
        reed.tone_hole_area * 1e6,
        threshold,
        tone.map_or("silent".to_owned(), |t| format!("{:.1} Hz", t.frequency)),
    );
}

#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_bass_reeds_as_carried_down() {
    let p = Parameters::default();
    line(
        "treble F3 8′ (built)",
        design(&p, 53, RANK_MIDDLE).unwrap(),
        174.6,
    );
    for rank in 0..5 {
        let aim = bass_target(&p, 0, rank).unwrap();
        line(
            &format!("rank {rank} C (note {})", bass_note(0, rank).unwrap()),
            bass_untuned(&p, 0, rank).unwrap(),
            aim,
        );
    }
    for pitch_class in 0..12 {
        let aim = bass_target(&p, pitch_class, BASS_16).unwrap();
        line(
            &format!("16′ pc {pitch_class:2}"),
            bass_untuned(&p, pitch_class, BASS_16).unwrap(),
            aim,
        );
    }
}

/// The 16′ C2 at 300 Pa, traced: what the tone detector sees.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_silent_c2() {
    let p = Parameters::default();
    for pitch_class in [0usize, 1] {
        let mut reed = bass_untuned(&p, pitch_class, BASS_16).unwrap();
        reed.frequency = bass_target(&p, pitch_class, BASS_16).unwrap();
        let rate = 192_000.0;
        let trace = rf_musette_analysis::simulate(reed, rate, 6.0, |_| 300.0);
        println!("pitch class {pitch_class}:");
        for k in 0..12 {
            let (a, b) = (
                (k as f64 * 0.5 * rate) as usize,
                ((k + 1) as f64 * 0.5 * rate) as usize,
            );
            let w = &trace.zeta[a..b];
            let (lo, hi) = w
                .iter()
                .fold((f64::MAX, f64::MIN), |(l, h), z| (l.min(*z), h.max(*z)));
            let mean = w.iter().sum::<f64>() / w.len() as f64;
            println!(
                "  {:4.1}-{:4.1} s: zeta {:8.3} .. {:8.3} mm, mean {:7.3} mm, tone {:?}",
                k as f64 * 0.5,
                (k + 1) as f64 * 0.5,
                lo * 1e3,
                hi * 1e3,
                mean * 1e3,
                trace
                    .tone(k as f64 * 0.5, (k + 1) as f64 * 0.5)
                    .map(|t| t.frequency)
            );
        }
    }
}

/// The 16′ C2's attack through the engine, as milestone 8's test makes it:
/// its first harmonic's envelope, and the tongue's swing.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_c2_attack() {
    use rf_musette_analysis::component_envelope;
    use rf_musette_dsp::parameters::{self, STIFF};
    use rf_musette_dsp::{Engine, PULL_REED};
    let rate = 48_000.0f32;
    for kick in [1.0, 0.0] {
        let mut engine = Engine::new(rate).unwrap();
        assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
        assert!(engine.set_parameter(parameters::BASS_REGISTER, 6.0));
        assert!(engine.set_parameter(parameters::ATTACK_KICK, kick));
        engine.bellows_mut().expression_wide(0.4f32.sqrt());
        let mut block = [0.0f32; 256];
        for _ in 0..(0.3 * rate / 256.0) as usize {
            engine.render(&mut block);
        }
        engine.bass_on(36, 1.0);
        let mut out = Vec::new();
        let mut peak = Vec::new();
        for _ in 0..(2.0 * rate / 256.0) as usize {
            engine.render(&mut block);
            out.extend(block.iter().map(|x| f64::from(*x)));
            peak.push(engine.bass_reed(0, BASS_16, PULL_REED).unwrap().1.zeta);
        }
        let envelope = component_envelope(&out, f64::from(rate), 65.41, 4.0, 0.001);
        let steady = envelope[envelope.len() * 4 / 5..]
            .iter()
            .map(|e| e.1)
            .sum::<f64>()
            / (envelope.len() - envelope.len() * 4 / 5) as f64;
        println!("kick {kick}: steady {steady:.1} dB");
        for (t, level) in envelope.iter().step_by(20).take(20) {
            println!("  {:5.0} ms: {:6.1} dB", t * 1e3, level - steady);
        }
    }
}

/// Where each low reed sounds: the pressures, of a player's range, at which
/// it holds a tone from rest.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_pressures_the_low_reeds_sound_at() {
    use rf_musette_dsp::compass::{bass_design, design};
    use rf_musette_dsp::parameters::RANK_LOW;
    let p = Parameters::default();
    let pressures = [50.0, 100.0, 200.0, 300.0, 400.0, 600.0, 800.0, 1000.0];
    let row = |name: String, reed: ReedDesign| {
        let line: String = pressures
            .iter()
            .map(|pressure| {
                if sounding(reed, *pressure).is_some() {
                    " ■"
                } else {
                    " ·"
                }
            })
            .collect();
        println!("{name:>22}:{line}");
    };
    println!("{:>22}: {pressures:?} Pa", "");
    for key in [53u8, 56, 59] {
        row(
            format!("treble L key {key}"),
            design(&p, key, RANK_LOW).unwrap(),
        );
    }
    for pitch_class in 0..12 {
        row(
            format!("16′ pc {pitch_class}"),
            bass_design(&p, pitch_class, BASS_16).unwrap(),
        );
    }
    for pitch_class in [0usize, 6] {
        row(
            format!("8′ pc {pitch_class}"),
            bass_design(&p, pitch_class, 1).unwrap(),
        );
    }
}

/// Which pressure each reed of both sides fails to hold a tone at, of 50,
/// 300 and 1000 Pa.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn where_each_reed_falls_silent() {
    use rf_musette_dsp::compass::{FIRST_KEY, KEYS, bass_design, design};
    let p = Parameters::default();
    let mut failing = std::collections::BTreeMap::<String, Vec<String>>::new();
    let mut check = |name: String, reed: ReedDesign| {
        for pressure in [50.0, 300.0, 1000.0] {
            if sounding(reed, pressure).is_none() {
                failing
                    .entry(format!("{pressure} Pa"))
                    .or_default()
                    .push(name.clone());
            }
        }
    };
    for rank in 0..5 {
        for index in 0..KEYS {
            let key = FIRST_KEY + index as u8;
            check(format!("t{rank}/{key}"), design(&p, key, rank).unwrap());
        }
        for pitch_class in 0..12 {
            check(
                format!("b{rank}/{pitch_class}"),
                bass_design(&p, pitch_class, rank).unwrap(),
            );
        }
    }
    for (pressure, names) in failing {
        println!("silent at {pressure}: {}", names.join(" "));
    }
}
