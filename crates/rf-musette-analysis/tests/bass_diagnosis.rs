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
        // The bellows played: Key Touch off, or it rests (9h again).
        assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
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

/// What sets the low reeds' attack, at 400 Pa, from C2 up to F4: the growth
/// rate of a small disturbance, the start 7b gives (κ·set·P/(P + 20 Pa)) as
/// a share of the swing the reed settles to, and the time those two predict
/// to reach the swing, ln(swing/start)/σ.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn what_sets_the_low_attack() {
    use rf_musette_analysis::growth_rate;
    use rf_musette_dsp::compass::{bass_design, design};
    use rf_musette_dsp::parameters::RANK_MIDDLE;
    let p = Parameters::default();
    let pressure = 400.0;
    let reeds = [
        ("16′ C2", bass_design(&p, 0, BASS_16).unwrap()),
        ("16′ E2", bass_design(&p, 4, BASS_16).unwrap()),
        ("16′ A2", bass_design(&p, 9, BASS_16).unwrap()),
        ("16′ B2", bass_design(&p, 11, BASS_16).unwrap()),
        ("8′ C3", bass_design(&p, 0, 1).unwrap()),
        ("M F3", design(&p, 53, RANK_MIDDLE).unwrap()),
        ("M F4", design(&p, 65, RANK_MIDDLE).unwrap()),
        ("M F5", design(&p, 77, RANK_MIDDLE).unwrap()),
    ];
    for (name, reed) in reeds {
        let sigma = growth_rate(reed, 192_000.0, pressure);
        let swing = sounding(reed, pressure).map_or(f64::NAN, |t| t.amplitude);
        let start = reed.set * pressure / (pressure + 20.0);
        let model = ReedModel::new(reed);
        println!(
            "{name:7}: f {:6.1} Hz, Q {:5.1}, load {:.2}, σ {:6.1}/s ({:.3}/cycle), swing {:5.2} mm, start {:4.2} mm ({:5.1} dB), ln(swing/start)/σ {:5.0} ms, static {:4.2} mm",
            reed.frequency,
            reed.q,
            reed.tip_load,
            sigma,
            sigma / reed.frequency,
            swing * 1e3,
            start * 1e3,
            20.0 * (start / swing).log10(),
            (swing / start).ln() / sigma * 1e3,
            model.mu * pressure / (model.omega * model.omega) * 1e3,
        );
    }
}

/// The 16′ C2's growth at 400 Pa, one thing moved at a time: σ, and whether
/// it still holds a tone at 50 Pa and 1 kPa.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_c2_growth_levers() {
    use rf_musette_analysis::growth_rate;
    use rf_musette_dsp::compass::bass_design;
    let p = Parameters::default();
    let base = bass_design(&p, 0, BASS_16).unwrap();
    let report = |name: &str, reed: ReedDesign| {
        let sigma = growth_rate(reed, 192_000.0, 400.0);
        let low = sounding(reed, 50.0).is_some();
        let high = sounding(reed, 1000.0).is_some();
        let swing = sounding(reed, 400.0).map_or(f64::NAN, |t| t.amplitude);
        println!(
            "{name:34}: σ {sigma:5.1}/s, swing {:4.2} mm, 50 Pa {}, 1 kPa {}",
            swing * 1e3,
            if low { "yes" } else { "no " },
            if high { "yes" } else { "no " },
        );
    };
    report("as built (64.6 mm, load 3.67)", base);
    for load in [0.0, 1.0, 2.0, 6.0] {
        report(
            &format!("load {load}"),
            ReedDesign {
                tip_load: load,
                ..base
            },
        );
    }
    for (length, load) in [
        (0.052, 0.0),
        (0.052, 3.67),
        (0.052, 8.0),
        (0.045, 8.0),
        (0.045, 15.0),
    ] {
        // Shorter, the rest of the geometry kept.
        report(
            &format!("length {:.0} mm, load {load}", length * 1e3),
            ReedDesign {
                length,
                tip_load: load,
                ..base
            },
        );
    }
    for q in [2.0, 4.0] {
        report(
            &format!("Q ×{q}"),
            ReedDesign {
                q: base.q * q,
                ..base
            },
        );
    }
    for x in [0.5, 2.0] {
        report(
            &format!("cell ×{x}"),
            ReedDesign {
                cell_volume: base.cell_volume * x,
                ..base
            },
        );
        report(
            &format!("hole ×{x}"),
            ReedDesign {
                tone_hole_area: base.tone_hole_area * x,
                ..base
            },
        );
        report(
            &format!("set ×{x}"),
            ReedDesign {
                set: base.set * x,
                ..base
            },
        );
        report(
            &format!("width ×{x}"),
            ReedDesign {
                width: base.width * x,
                ..base
            },
        );
        report(
            &format!("near field ×{x}"),
            ReedDesign {
                inertance_scale: base.inertance_scale * x,
                ..base
            },
        );
    }
}

/// The growth against the upstream inertia, from C2 to F5 at 400 Pa: σ at
/// the near field ×1, ×2, ×4 and with the hole's inertance ×2 (hole area
/// ×0.5), and the inertances themselves.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn growth_against_the_upstream_inertia() {
    use rf_musette_analysis::growth_rate;
    use rf_musette_dsp::compass::{bass_design, design};
    use rf_musette_dsp::parameters::RANK_MIDDLE;
    let p = Parameters::default();
    let reeds = [
        ("16′ C2", bass_design(&p, 0, BASS_16).unwrap()),
        ("16′ B2", bass_design(&p, 11, BASS_16).unwrap()),
        ("M F3", design(&p, 53, RANK_MIDDLE).unwrap()),
        ("M F4", design(&p, 65, RANK_MIDDLE).unwrap()),
        ("M F5", design(&p, 77, RANK_MIDDLE).unwrap()),
    ];
    for (name, reed) in reeds {
        let model = ReedModel::new(reed);
        let sigma = |r: ReedDesign| growth_rate(r, 192_000.0, 400.0);
        println!(
            "{name:7}: M_n {:6.0}, M_h {:6.0} kg/m⁴, ωM_n·S_r/ρ... σ ×1 {:5.1}, near ×2 {:5.1}, near ×4 {:5.1}, hole-inertia ×2 {:5.1}, both ×2 {:5.1}",
            model.inertance,
            model.hole_inertance,
            sigma(reed),
            sigma(ReedDesign {
                inertance_scale: reed.inertance_scale * 2.0,
                ..reed
            }),
            sigma(ReedDesign {
                inertance_scale: reed.inertance_scale * 4.0,
                ..reed
            }),
            sigma(ReedDesign {
                tone_hole_area: reed.tone_hole_area * 0.5,
                ..reed
            }),
            sigma(ReedDesign {
                inertance_scale: reed.inertance_scale * 2.0,
                tone_hole_area: reed.tone_hole_area * 0.5,
                ..reed
            }),
        );
    }
}

/// Llanos-Vázquez et al. 2014, Table I, against the model: the finger attack
/// of the first harmonic (−50 → −5 dB) of the 8′ notes they measured, the
/// model at 400 Pa (their mf) and 100 Pa (p), the bellows stiff. A2-B2 are
/// the bass side's 16′ (the treble's 8′ starts at F3); the rest the treble's
/// true 8′.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn llanos_table_one_against_the_model() {
    use rf_musette_analysis::{attack_time, component_envelope};
    use rf_musette_dsp::Engine;
    use rf_musette_dsp::compass::target;
    use rf_musette_dsp::parameters::{self, RANK_MIDDLE, STIFF};
    let rate = 48_000.0f32;
    let measured: [(&str, i32, Option<u32>, Option<u32>); 13] = [
        ("A2", 45, Some(80), Some(110)),
        ("A#2", 46, Some(70), None),
        ("B2", 47, Some(100), None),
        ("A3", 57, Some(110), Some(90)),
        ("A#3", 58, Some(70), Some(100)),
        ("B3", 59, Some(60), Some(70)),
        ("A4", 69, Some(60), Some(90)),
        ("A#4", 70, Some(60), Some(140)),
        ("B4", 71, Some(70), Some(120)),
        ("A5", 81, Some(50), Some(60)),
        ("A#5", 82, Some(50), Some(80)),
        ("B5", 83, Some(50), Some(80)),
        ("A6", 93, Some(100), Some(130)),
    ];
    let attack = |note: i32, pressure: f32| -> Option<f64> {
        let mut engine = Engine::new(rate).unwrap();
        assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
        assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
        // The bass side in 16′ alone is register 16′/2′; the 2′ is far up.
        assert!(engine.set_parameter(parameters::BASS_REGISTER, 6.0));
        if let Ok(ms) = std::env::var("PALLET_MS") {
            assert!(engine.set_parameter(parameters::PALLET_OPENING, ms.parse().unwrap()));
        }
        if let Ok(kick) = std::env::var("KICK") {
            assert!(engine.set_parameter(parameters::ATTACK_KICK, kick.parse().unwrap()));
        }
        engine
            .bellows_mut()
            .expression_wide((pressure / 1000.0).sqrt());
        let mut block = [0.0f32; 256];
        for _ in 0..(0.3 * rate / 256.0) as usize {
            engine.render(&mut block);
        }
        let frequency = if note < 53 {
            engine.bass_on(note as u8, 1.0);
            440.0 * 2f64.powf((f64::from(note) - 69.0) / 12.0)
        } else {
            engine.note_on(note as u8, 1.0);
            target(&Parameters::default(), note as u8, RANK_MIDDLE)
        };
        let mut out = Vec::new();
        for _ in 0..(2.5 * rate / 256.0) as usize {
            engine.render(&mut block);
            out.extend(block.iter().map(|x| f64::from(*x)));
        }
        attack_time(&component_envelope(
            &out,
            f64::from(rate),
            frequency,
            4.0,
            0.001,
        ))
    };
    let ms = |a: Option<f64>| a.map_or("  --".to_owned(), |a| format!("{:4.0}", a * 1e3));
    let mm = |a: Option<u32>| a.map_or("  --".to_owned(), |a| format!("{a:4}"));
    println!("note | mf measured  model | p measured  model");
    for (name, note, mf, p) in measured {
        println!(
            "{name:4} |        {}   {} |       {}   {}",
            mm(mf),
            ms(attack(note, 400.0)),
            mm(p),
            ms(attack(note, 100.0)),
        );
    }
}

/// The 16′ A2 through the engine at 400 Pa, its pallet opening in 50 ms or
/// 5 ms, the start on or off: every 10 ms, the cell's pressure, the tongue's
/// mean and its swing over the last period.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_a2_opening_traced() {
    use rf_musette_dsp::parameters::{self, STIFF};
    use rf_musette_dsp::{Engine, PULL_REED};
    let rate = 48_000.0f32;
    for (opening, kick) in [(50.0, 1.0), (50.0, 0.0), (5.0, 1.0)] {
        let mut engine = Engine::new(rate).unwrap();
        assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
        assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
        assert!(engine.set_parameter(parameters::BASS_REGISTER, 6.0));
        assert!(engine.set_parameter(parameters::PALLET_OPENING, opening));
        assert!(engine.set_parameter(parameters::ATTACK_KICK, kick));
        engine.bellows_mut().expression_wide(0.4f32.sqrt());
        let mut one = [0.0f32; 1];
        for _ in 0..(0.3 * rate) as usize {
            engine.render(&mut one);
        }
        engine.bass_on(45, 1.0);
        println!("opening {opening} ms, kick {kick}:");
        let period = (rate / 110.0) as usize;
        let mut zeta = Vec::new();
        let mut cell = Vec::new();
        for _ in 0..(0.3 * rate) as usize {
            engine.render(&mut one);
            let (_, state) = engine.bass_reed(9, BASS_16, PULL_REED).unwrap();
            zeta.push(state.zeta);
            cell.push(state.cell_pressure);
        }
        for step in (1..30).map(|k| k * (rate as usize / 100)) {
            let window = &zeta[step.saturating_sub(period)..step];
            let (lo, hi) = window
                .iter()
                .fold((f64::MAX, f64::MIN), |(l, h), z| (l.min(*z), h.max(*z)));
            println!(
                "  {:4.0} ms: cell {:5.0} Pa, mean {:5.2} mm, swing {:5.2} mm",
                step as f32 / rate * 1e3,
                cell[step - 1],
                0.5 * (hi + lo) * 1e3,
                0.5 * (hi - lo) * 1e3
            );
        }
    }
}

/// The C bass held 2 s as the tune plays it -- the arm at CC 11 = 80, all
/// five ranks -- against the 16′ alone: the 16′ C2's swing every 100 ms, and
/// the bellows' pressure.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_c2_in_the_tune() {
    use rf_musette_dsp::parameters;
    use rf_musette_dsp::{Engine, PULL_REED};
    let rate = 48_000.0f32;
    for register in [3.0, 6.0] {
        let mut engine = Engine::new(rate).unwrap();
        assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
        assert!(engine.set_parameter(parameters::BASS_REGISTER, register));
        engine.bellows_mut().expression_msb(80);
        let mut one = [0.0f32; 1];
        for _ in 0..(0.5 * rate) as usize {
            engine.render(&mut one);
        }
        engine.bass_on(48, 100.0 / 127.0);
        println!("bass register {register}:");
        let mut peak = 0.0f64;
        for n in 0..(2.0 * rate) as usize {
            engine.render(&mut one);
            let zeta = engine.bass_reed(0, BASS_16, PULL_REED).unwrap().1.zeta;
            peak = peak.max(zeta.abs());
            if (n + 1) % (rate as usize / 10) == 0 {
                println!(
                    "  {:4.0} ms: C2 peak {:5.2} mm, supply {:5.0} Pa",
                    (n + 1) as f32 / rate * 1e3,
                    peak * 1e3,
                    engine.supply()
                );
                peak = 0.0;
            }
        }
    }
}

/// The 16′ C2 made shorter and loaded more, as Llanos-Vázquez's bass reeds
/// are (thesis Tables 3.1, 4.3): for each length and load, σ at 300 Pa and
/// whether it holds a tone at 50 Pa, 300 Pa and 1 kPa. The set and the hole
/// go with the length, the cell with its cube, as along the compass.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_c2_shorter_and_loaded() {
    use rf_musette_analysis::growth_rate;
    use rf_musette_dsp::compass::bass_bare;
    let p = Parameters::default();
    let base = bass_bare(&p, 0, BASS_16).unwrap();
    for length in [0.040, 0.045, 0.052, 0.058, 0.0646] {
        let k = length / base.length;
        let shaped = ReedDesign {
            length,
            set: base.set * k,
            tone_hole_area: base.tone_hole_area * k,
            cell_volume: base.cell_volume * k * k * k,
            ..base
        };
        let mut line = format!("{:4.1} mm:", length * 1e3);
        for load in [0.0, 1.0, 2.0, 3.0, 4.0, 6.0, 8.0, 12.0, 16.0] {
            let reed = ReedDesign {
                tip_load: load,
                ..shaped
            };
            let sigma = growth_rate(reed, 192_000.0, 300.0);
            let holds = [50.0, 300.0, 1000.0]
                .iter()
                .all(|pressure| sounding(reed, *pressure).is_some());
            line += &format!(" {load:>2}:{sigma:5.1}{}", if holds { "*" } else { " " });
        }
        println!("{line}");
    }
    println!("(load over the unloaded modal mass: σ at 300 Pa, * = holds at 50, 300 and 1000 Pa)");
}

/// The 16′ C2's growth against pressure, as built and at two other loads,
/// beside Cottingham, Reed & Busha's measured C3 (4.5/s at 0.3 kPa, 8.3 at
/// 0.5, 11.5 near 1 kPa).
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_c2_growth_against_pressure() {
    use rf_musette_analysis::growth_rate;
    use rf_musette_dsp::compass::{bass_bare, bass_design};
    let p = Parameters::default();
    let built = bass_design(&p, 0, BASS_16).unwrap();
    let bare = bass_bare(&p, 0, BASS_16).unwrap();
    let pressures = [100.0, 200.0, 300.0, 350.0, 400.0, 500.0, 700.0, 1000.0];
    println!("{:>14}: {pressures:?} Pa", "");
    for (name, reed) in [
        ("as built", built),
        (
            "load 2",
            ReedDesign {
                tip_load: 2.0,
                frequency: built.frequency,
                ..bare
            },
        ),
        (
            "load 6",
            ReedDesign {
                tip_load: 6.0,
                frequency: built.frequency,
                ..bare
            },
        ),
        (
            "8′ C3",
            rf_musette_dsp::compass::bass_design(&p, 0, 1).unwrap(),
        ),
    ] {
        let line: String = pressures
            .iter()
            .map(|pressure| format!(" {:5.1}", growth_rate(reed, 192_000.0, *pressure)))
            .collect();
        println!("{name:>14}:{line}");
    }
}

/// The 16′ C2 alone (register 16′/2′) at ~365 Pa, the bellows stiff or the
/// arm's, pulling or pushing: its swing every 200 ms over 2 s.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_c2_stiff_against_the_arm() {
    use rf_musette_dsp::parameters::{self, ARM, PUSH, STIFF};
    use rf_musette_dsp::{Engine, PULL_REED, PUSH_REED};
    let rate = 48_000.0f32;
    for (name, response, push) in [
        ("stiff, pull", STIFF, false),
        ("arm, pull", ARM, false),
        ("stiff, push", STIFF, true),
        ("arm ×5 fast", ARM, false),
    ] {
        let mut engine = Engine::new(rate).unwrap();
        assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
        assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, response));
        assert!(engine.set_parameter(parameters::BASS_REGISTER, 6.0));
        if push {
            assert!(engine.set_parameter(parameters::BELLOWS_DIRECTION, PUSH));
        }
        if name == "arm ×5 fast" {
            assert!(engine.set_parameter(parameters::ARM_SPEED, 5.0));
        }
        engine.bellows_mut().expression_wide(if response == STIFF {
            0.365f32.sqrt()
        } else {
            80.0 / 127.0
        });
        let mut one = [0.0f32; 1];
        for _ in 0..(0.5 * rate) as usize {
            engine.render(&mut one);
        }
        engine.bass_on(48, 1.0);
        let which = if push { PUSH_REED } else { PULL_REED };
        let mut line = format!("{name:12}:");
        let mut peak = 0.0f64;
        for n in 0..(2.0 * rate) as usize {
            engine.render(&mut one);
            peak = peak.max(engine.bass_reed(0, BASS_16, which).unwrap().1.zeta.abs());
            if (n + 1) % (rate as usize / 5) == 0 {
                line += &format!(" {:4.2}", peak * 1e3);
                peak = 0.0;
            }
        }
        println!("{line} mm, supply {:.0} Pa", engine.supply());
    }
}

/// Low and middle notes alone, the bellows stiff or the arm's, ~370 Pa: each
/// reed's swing after 0.5 and 2 s, and the Helmholtz frequency its tone hole
/// makes with the bellows' air.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_low_notes_under_the_arm() {
    use rf_musette_dsp::parameters::{self, ARM, RANK_MIDDLE, STIFF};
    use rf_musette_dsp::{Engine, PULL_REED};
    let rate = 48_000.0f32;
    let p = Parameters::default();
    let wind = {
        let mut q = p;
        q.set(parameters::BELLOWS_RESPONSE, ARM);
        q.wind_design().unwrap()
    };
    for note in [36u8, 40, 45, 48, 53, 65] {
        let mut line = format!("note {note}:");
        for response in [STIFF, ARM] {
            let mut engine = Engine::new(rate).unwrap();
            assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
            assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, response));
            assert!(engine.set_parameter(parameters::BASS_REGISTER, 6.0));
            assert!(engine.set_parameter(parameters::REGISTER, 11.0));
            engine.bellows_mut().expression_wide(if response == STIFF {
                0.37f32.sqrt()
            } else {
                80.0 / 127.0
            });
            let mut one = [0.0f32; 1];
            for _ in 0..(0.5 * rate) as usize {
                engine.render(&mut one);
            }
            let bass = note < 53;
            if bass {
                engine.bass_on(note, 1.0)
            } else {
                engine.note_on(note, 1.0)
            }
            // (frequency, the hole's inertance, the tip's displacement)
            let reed = |engine: &Engine| -> (f64, f64, f64) {
                let (model, state) = if bass {
                    engine
                        .bass_reed(usize::from(note) % 12, BASS_16, PULL_REED)
                        .unwrap()
                } else {
                    engine.reed(note, RANK_MIDDLE, PULL_REED).unwrap()
                };
                (model.design.frequency, model.hole_inertance, state.zeta)
            };
            let mut peaks = Vec::new();
            let mut peak = 0.0f64;
            for n in 0..(2.0 * rate) as usize {
                engine.render(&mut one);
                peak = peak.max(reed(&engine).2.abs());
                if (n + 1) % (rate as usize / 2) == 0 {
                    peaks.push(peak);
                    peak = 0.0;
                }
            }
            let (frequency, hole_inertance, _) = reed(&engine);
            let helmholtz =
                1.0 / (2.0 * std::f64::consts::PI * (hole_inertance * wind.compliance).sqrt());
            line += &format!(
                " {}: {:.2} → {:.2} mm (f {:.0} Hz, hole+bellows {:.0} Hz) |",
                if response == STIFF { "stiff" } else { "arm" },
                peaks[0] * 1e3,
                peaks[3] * 1e3,
                frequency,
                helmholtz
            );
        }
        println!("{line}");
    }
}

/// Milestone 8e, measured first: the lowest 16′ fed by the bellows' air
/// (12 L, the arm holding the mean over 0.2 s), one lever at a time from
/// the sources -- a higher set (Llanos's luthiers, p153), a deeper tone hole
/// (an inlet duct "favours the onset", Fletcher, Llanos p236), a thicker
/// plate (low reeds have thicker plates, p263), a smaller hole. For each:
/// whether it speaks, sustained, at 100, 300 and 1000 Pa, and its swing
/// after 3 s at 300 Pa.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_bass_coupling_levers() {
    use rf_musette_analysis::{simulate_fed, speaks};
    use rf_musette_dsp::compass::{bass_design, bass_target};
    let p = Parameters::default();
    let pallet = p.pallet_design();
    let seconds = 3.0;
    for pitch_class in [0usize, 2, 4] {
        let base = bass_design(&p, pitch_class, BASS_16).unwrap();
        let aim = bass_target(&p, pitch_class, BASS_16).unwrap();
        println!("16′ pitch class {pitch_class} ({aim:.1} Hz):");
        let levers: [(&str, ReedDesign); 9] = [
            ("as built", base),
            (
                "set ×1.5",
                ReedDesign {
                    set: base.set * 1.5,
                    ..base
                },
            ),
            (
                "set ×2",
                ReedDesign {
                    set: base.set * 2.0,
                    ..base
                },
            ),
            (
                "hole depth ×3",
                ReedDesign {
                    tone_hole_depth: base.tone_hole_depth * 3.0,
                    ..base
                },
            ),
            (
                "hole depth ×6",
                ReedDesign {
                    tone_hole_depth: base.tone_hole_depth * 6.0,
                    ..base
                },
            ),
            (
                "hole area ×0.5",
                ReedDesign {
                    tone_hole_area: base.tone_hole_area * 0.5,
                    ..base
                },
            ),
            (
                "plate ×1.5",
                ReedDesign {
                    plate_thickness: base.plate_thickness * 1.5,
                    ..base
                },
            ),
            (
                "set ×1.5, depth ×3",
                ReedDesign {
                    set: base.set * 1.5,
                    tone_hole_depth: base.tone_hole_depth * 3.0,
                    ..base
                },
            ),
            ("ideal source", base),
        ];
        for (name, reed) in levers {
            let mut line = format!("  {name:20}:");
            for pressure in [100.0, 300.0, 1000.0] {
                let trace = if name == "ideal source" {
                    rf_musette_analysis::simulate(reed, 96_000.0, seconds, |_| pressure)
                } else {
                    simulate_fed(reed, pallet, 96_000.0, seconds, pressure, 0.012, 0.2, 0.05)
                };
                let ok = speaks(&trace, seconds, reed.set);
                let swing = trace
                    .tone(seconds - 0.5, seconds)
                    .map_or(0.0, |t| t.amplitude);
                line += &format!(
                    " {pressure:4} Pa {} {:4.2} mm |",
                    if ok { "■" } else { "·" },
                    swing * 1e3
                );
            }
            println!("{line}");
        }
    }
}

/// The bellows-fed simulation on reeds that speak in the engine: A2 (16′),
/// the treble's F3 and F4 (8′), at 300 Pa, against an ideal source -- to
/// know the tool before trusting it.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_fed_simulation_on_speaking_reeds() {
    use rf_musette_analysis::{simulate_fed, speaks};
    use rf_musette_dsp::compass::{bass_design, design};
    use rf_musette_dsp::parameters::RANK_MIDDLE;
    let p = Parameters::default();
    let pallet = p.pallet_design();
    let seconds = 3.0;
    for (name, reed) in [
        ("16′ A2", bass_design(&p, 9, BASS_16).unwrap()),
        ("M F3", design(&p, 53, RANK_MIDDLE).unwrap()),
        ("M F4", design(&p, 65, RANK_MIDDLE).unwrap()),
    ] {
        let mut line = format!("{name:7}:");
        for (label, regulation, volume) in [
            ("fed 12 L", 0.2, 0.012),
            ("fed 12 L τ 20 ms", 0.02, 0.012),
            ("fed 40 L", 0.2, 0.040),
        ] {
            let trace = simulate_fed(
                reed, pallet, 96_000.0, seconds, 300.0, volume, regulation, 0.05,
            );
            let swing = trace
                .tone(seconds - 0.5, seconds)
                .map_or(0.0, |t| t.amplitude);
            line += &format!(
                " {label}: {} {:4.2} mm |",
                if speaks(&trace, seconds, reed.set) {
                    "■"
                } else {
                    "·"
                },
                swing * 1e3
            );
        }
        let ideal = rf_musette_analysis::simulate(reed, 96_000.0, seconds, |_| 300.0);
        line += &format!(
            " ideal: {:4.2} mm",
            ideal
                .tone(seconds - 0.5, seconds)
                .map_or(0.0, |t| t.amplitude)
                * 1e3
        );
        println!("{line}");
    }
}

/// The lowest 16′ fed by the bellows' air: hole depth against set, whether
/// it speaks (sustained) at 100, 300 and 1000 Pa.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn depth_against_set_for_the_lowest() {
    use rf_musette_analysis::{simulate_fed, speaks};
    use rf_musette_dsp::compass::bass_design;
    let p = Parameters::default();
    let pallet = p.pallet_design();
    let seconds = 3.0;
    println!(
        "hole depth default {:.1} mm",
        p.reed_design().tone_hole_depth * 1e3
    );
    for pitch_class in [0usize, 1, 3] {
        let base = bass_design(&p, pitch_class, BASS_16).unwrap();
        println!("pc {pitch_class}:");
        for set in [1.0, 1.5, 2.0] {
            let mut line = format!("  set ×{set}:");
            for depth in [6.0, 10.0, 15.0, 25.0] {
                let reed = ReedDesign {
                    set: base.set * set,
                    tone_hole_depth: base.tone_hole_depth * depth,
                    ..base
                };
                let marks: String = [100.0, 300.0, 1000.0]
                    .iter()
                    .map(|pressure| {
                        let trace = simulate_fed(
                            reed, pallet, 96_000.0, seconds, *pressure, 0.012, 0.2, 0.05,
                        );
                        if speaks(&trace, seconds, reed.set) {
                            '■'
                        } else {
                            '·'
                        }
                    })
                    .collect();
                line += &format!("  depth ×{depth}: {marks}");
            }
            println!("{line}");
        }
    }
    println!("(100, 300, 1000 Pa)");
}
