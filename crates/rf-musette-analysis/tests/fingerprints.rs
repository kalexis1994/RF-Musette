//! Milestone 10: the sound the engine has always made. Every scene of
//! `scenes` renders to the same bits as when its fingerprint was taken; a
//! change meant to be exact must leave them all, and a fingerprint that
//! moves is a question, not a verdict -- measured against the reference
//! renders (`the_renders_against_the_reference`), heard, and only then
//! taken again (`print_fingerprints`).

use rf_musette_analysis::scenes::{compare, fingerprint, level_difference, nudged, render, scenes};

/// Taken again 2026-10-02 after milestone 8m (the cell as a tube, the
/// hole's and the slot's radiation, the set's shape, the curtain's slit and
/// mass, the pad's seating), 9f (the instruments), the wheel as the bellows
/// withdrawn and velocity no longer moving it -- each heard by the player.
/// Before: 2026-10-01, after 10c and 8k; first taken at dd7e761.
const FINGERPRINTS: &[(&str, u64)] = &[
    ("program research", 0xba569fdcade7d065),
    ("program musette-paris", 0x79f8c0da4a2e18a2),
    ("program scottish", 0x96ef06426a1fff30),
    ("program italian", 0xc1f545a4196f7a82),
    ("program alpine", 0x5dcf96362a1cb499),
    ("program oberkrainer", 0x17a51dc78aae5ff5),
    ("program cleveland", 0x7b2ff341cc1012bd),
    ("program american", 0xe1db72b884ec5474),
    ("program irish", 0x8366f477033c9b9a),
    ("program jazz", 0xfd36a410b11a7cd4),
    ("program tango", 0x1fa006b38bdfa414),
    ("program concert", 0xeaae24afaee01a9b),
    ("program keyboard-61", 0x97c2c0d64bdaceb1),
    ("program auto-bellows", 0x70567b35ccd162fb),
    ("program digital-accordion", 0xfcb7ea1b654b4fee),
    ("program student-48", 0xb4e3f0375346b9d3),
    ("program student-72", 0x5a1e92521ea52215),
    ("program italian-80", 0x6a1c44d68fd1f6ae),
    ("program full-120", 0xef1a72c5eb43892a),
    ("program cassotto-pro", 0x7a4ec039aee52eb5),
    ("master chord, mono", 0x7b34070c4b00e5e9),
    ("master chord, mono, 1x", 0x07b1affbd899b591),
    ("master chord, mono, 4x", 0x46470a9b706500c5),
    ("phrase at 44.1 kHz, ORTF", 0x776739fd3ec04c83),
    ("phrase at 96 kHz, dry", 0x483066a8c61521a5),
    ("reversal and air button", 0xfb50af7fe1703b4f),
    ("wheel as pressure", 0x835f0e927fa05446),
    ("silence, a note, the hall's tail", 0xfdaf598e7e216740),
];

#[test]
fn every_scene_renders_what_it_has_always_rendered() {
    let mut moved = Vec::new();
    for scene in scenes() {
        let expected = FINGERPRINTS
            .iter()
            .find(|(name, _)| *name == scene.name)
            .unwrap_or_else(|| panic!("{} has no fingerprint", scene.name))
            .1;
        let actual = fingerprint(&render(&scene));
        if actual != expected {
            moved.push(format!("{}: {actual:#018x}", scene.name));
        }
    }
    assert!(moved.is_empty(), "moved:\n{}", moved.join("\n"));
}

#[test]
#[ignore = "prints the fingerprints to take them again"]
fn print_fingerprints() {
    for scene in scenes() {
        let samples = render(&scene);
        let peak = samples.iter().fold(0.0f32, |m, x| m.max(x.abs()));
        assert!(peak > 1.0e-3, "{} is silent ({peak})", scene.name);
        println!("    (\"{}\", {:#018x}),", scene.name, fingerprint(&samples));
    }
}

/// Writes every scene's samples (little-endian f32, interleaved) to the
/// directory `RF_MUSETTE_REFERENCE` names: the reference a change that is
/// not exact is measured against.
#[test]
#[ignore = "writes the reference renders to $RF_MUSETTE_REFERENCE"]
fn write_the_reference() {
    let directory = std::env::var("RF_MUSETTE_REFERENCE").expect("RF_MUSETTE_REFERENCE");
    std::fs::create_dir_all(&directory).unwrap();
    for scene in scenes() {
        let bytes: Vec<u8> = render(&scene)
            .iter()
            .flat_map(|x| x.to_le_bytes())
            .collect();
        std::fs::write(
            std::path::Path::new(&directory).join(file_name(&scene.name)),
            bytes,
        )
        .unwrap();
    }
}

/// How far each scene now lies from the reference renders.
#[test]
#[ignore = "measurement: compares against $RF_MUSETTE_REFERENCE"]
fn the_renders_against_the_reference() {
    let directory = std::env::var("RF_MUSETTE_REFERENCE").expect("RF_MUSETTE_REFERENCE");
    for scene in scenes() {
        let bytes = std::fs::read(std::path::Path::new(&directory).join(file_name(&scene.name)))
            .unwrap_or_else(|error| panic!("{}: {error}", scene.name));
        let reference: Vec<f32> = bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|b| f32::from_le_bytes(*b))
            .collect();
        let now = render(&scene);
        let difference = compare(&reference, &now);
        // The yardstick: how far a one-ulp nudge moves the scene as the
        // engine now renders it.
        let nudge = level_difference(&now, &render(&nudged(&scene)), scene.rate);
        let change = level_difference(&reference, &now, scene.rate);
        let holds = change <= 1.5 * nudge + 0.2;
        println!(
            "{:34} {:6} differ  error {:7.1} dB  levels p99: change {:5.2} dB, nudge {:5.2} dB  {}",
            scene.name,
            difference.differing,
            difference.error_db,
            change,
            nudge,
            if holds { "same" } else { "MOVED" }
        );
    }
}

fn file_name(scene: &str) -> String {
    let safe: String = scene
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    format!("{safe}.f32")
}

/// The auto-bellows scene does what it is there for: the bellows runs out
/// and turns by itself while the chord is held.
#[test]
fn the_auto_bellows_scene_turns() {
    use rf_musette_dsp::Engine;
    use rf_musette_dsp::parameters;
    use rf_musette_dsp::programs::program;
    let mut engine = Engine::new(48_000.0).unwrap();
    let settings = program("auto-bellows").unwrap().parameters();
    for index in 0..parameters::COUNT {
        assert!(engine.set_parameter(index, settings.get(index).unwrap()));
    }
    assert!(engine.set_parameter(parameters::BELLOWS_TRAVEL, 2.0));
    for key in [53, 60, 64, 67, 72] {
        engine.note_on(key, 1.0);
    }
    let mut block = [0.0f32; 480];
    let mut opening = Vec::new();
    for _ in 0..220 {
        engine.render(&mut block);
        opening.push(engine.bellows_opening());
    }
    let turns = opening
        .windows(3)
        .filter(|w| (w[1] - w[0]) * (w[2] - w[1]) < 0.0)
        .count();
    assert!(turns >= 1, "the bellows never turned: {:?}", &opening[..10]);
}

/// Where in time a scene first leaves the reference, and how the
/// difference runs after: the error against the signal, window by window
/// of 10 ms. A rounding that drifts grows; a decision moved by a step
/// (a reed's start a substep early) jumps once and stays.
#[test]
#[ignore = "measurement: RF_MUSETTE_SCENE names the scene; compares against $RF_MUSETTE_REFERENCE"]
fn how_a_scene_leaves_the_reference() {
    let directory = std::env::var("RF_MUSETTE_REFERENCE").expect("RF_MUSETTE_REFERENCE");
    let wanted = std::env::var("RF_MUSETTE_SCENE").expect("RF_MUSETTE_SCENE");
    let scene = scenes()
        .into_iter()
        .find(|s| s.name == wanted)
        .expect("no such scene");
    let bytes =
        std::fs::read(std::path::Path::new(&directory).join(file_name(&scene.name))).unwrap();
    let reference: Vec<f32> = bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|b| f32::from_le_bytes(*b))
        .collect();
    let now = render(&scene);
    let first = reference
        .iter()
        .zip(&now)
        .position(|(a, b)| a.to_bits() != b.to_bits());
    let rate = f64::from(scene.rate);
    println!(
        "first difference at {:?} s",
        first.map(|n| (n / 2) as f64 / rate)
    );
    if let Some(first) = first {
        for n in first..(first + 40).min(now.len()) {
            println!(
                "  sample {:6} ch {}: {:+.9e} against {:+.9e} (diff {:+.3e})",
                n / 2,
                n % 2,
                now[n],
                reference[n],
                now[n] - reference[n]
            );
        }
    }
    let window = 2 * (rate / 100.0) as usize;
    for (n, (a, b)) in reference.chunks(window).zip(now.chunks(window)).enumerate() {
        let signal: f64 = a.iter().map(|x| f64::from(*x).powi(2)).sum();
        let error: f64 = a
            .iter()
            .zip(b)
            .map(|(x, y)| (f64::from(*x) - f64::from(*y)).powi(2))
            .sum();
        if signal > 0.0 && n % 3 == 0 {
            println!(
                "{:5.2} s  error {:7.1} dB",
                n as f64 * 0.01,
                10.0 * (error.max(1e-300) / signal).log10()
            );
        }
    }
}

/// How sensitive the dynamics are, apart from any change to the code: the
/// same scene with A4 one ulp from 440 Hz -- every reed's frequency moved
/// by one part in 10^16 -- against the scene as it is, window by window.
#[test]
#[ignore = "measurement: prints how a one-ulp difference grows, RF_MUSETTE_SCENE names the scene"]
fn how_a_one_ulp_difference_grows() {
    let wanted = std::env::var("RF_MUSETTE_SCENE").expect("RF_MUSETTE_SCENE");
    let scene = scenes()
        .into_iter()
        .find(|s| s.name == wanted)
        .expect("no such scene");
    let reference = render(&scene);
    let mut moved = scenes().into_iter().find(|s| s.name == wanted).unwrap();
    let a4 = moved
        .parameters
        .get(rf_musette_dsp::parameters::PITCH_A4)
        .unwrap();
    assert!(moved.parameters.set(
        rf_musette_dsp::parameters::PITCH_A4,
        f64::from_bits(a4.to_bits() + 1)
    ));
    let now = render(&moved);
    let rate = f64::from(scene.rate);
    let window = 2 * (rate / 100.0) as usize;
    for (n, (a, b)) in reference.chunks(window).zip(now.chunks(window)).enumerate() {
        let signal: f64 = a.iter().map(|x| f64::from(*x).powi(2)).sum();
        let error: f64 = a
            .iter()
            .zip(b)
            .map(|(x, y)| (f64::from(*x) - f64::from(*y)).powi(2))
            .sum();
        if signal > 0.0 && n % 5 == 0 {
            println!(
                "{:5.2} s  error {:7.1} dB",
                n as f64 * 0.01,
                10.0 * (error.max(1e-300) / signal).log10()
            );
        }
    }
}

/// The yardstick sees a real change: the output 1 dB louder, and the
/// tremolo a tenth faster, each move the scenes they touch past it.
#[test]
#[ignore = "measurement: checks the yardstick against changes it must see"]
fn the_yardstick_sees_a_real_change() {
    use rf_musette_dsp::parameters::{GAIN, TREMOLO};
    for wanted in [
        "program research",
        "program musette-paris",
        "reversal and air button",
    ] {
        let scene = scenes().into_iter().find(|s| s.name == wanted).unwrap();
        let now = render(&scene);
        let nudge = level_difference(&now, &render(&nudged(&scene)), scene.rate);
        let changed = |index: usize, by: &dyn Fn(f64) -> f64| {
            let mut other = nudged(&scene);
            other.parameters = scene.parameters;
            let value = scene.parameters.get(index).unwrap();
            assert!(other.parameters.set(index, by(value)));
            level_difference(&now, &render(&other), scene.rate)
        };
        let louder = changed(GAIN, &|g| g * 10f64.powf(1.0 / 20.0));
        let faster = changed(TREMOLO, &|t| t * 1.1);
        println!(
            "{wanted}: nudge {nudge:.2} dB, 1 dB louder {louder:.2} dB, tremolo +10 % {faster:.2} dB"
        );
        assert!(louder > 1.5 * nudge + 0.2, "{wanted}: louder");
    }
}

#[test]
#[ignore = "diagnosis: prints a scene's level every 100 ms, RF_MUSETTE_SCENE names it"]
fn a_scenes_level() {
    let wanted = std::env::var("RF_MUSETTE_SCENE").expect("RF_MUSETTE_SCENE");
    let scene = scenes()
        .into_iter()
        .find(|s| s.name == wanted)
        .expect("no such scene");
    let samples = render(&scene);
    let window = 2 * (f64::from(scene.rate) / 10.0) as usize;
    for (n, chunk) in samples.chunks(window).enumerate() {
        let rms =
            (chunk.iter().map(|x| f64::from(*x).powi(2)).sum::<f64>() / chunk.len() as f64).sqrt();
        let peak = chunk.iter().fold(0.0f32, |m, x| m.max(x.abs()));
        println!(
            "{:4.1} s  rms {:9.2e}  peak {:9.2e}",
            n as f64 / 10.0,
            rms,
            peak
        );
    }
}

/// The held tone alone against the reference: band levels over a stretch
/// where nothing starts or ends -- 8k's prediction 3.
#[test]
#[ignore = "measurement: compares held stretches against $RF_MUSETTE_REFERENCE"]
fn the_held_tone_against_the_reference() {
    let directory = std::env::var("RF_MUSETTE_REFERENCE").expect("RF_MUSETTE_REFERENCE");
    // (scene, from s, to s): held, after the last start, before the first end.
    let stretches = [
        ("master chord, mono", 0.25, 0.78),
        ("master chord, mono, 4x", 0.25, 0.78),
        ("program research", 0.30, 0.44),
        ("program musette-paris", 0.30, 0.44),
        ("program concert", 0.30, 0.44),
        ("phrase at 96 kHz, dry", 0.30, 0.44),
    ];
    for (name, from, to) in stretches {
        let scene = scenes().into_iter().find(|s| s.name == name).unwrap();
        let bytes =
            std::fs::read(std::path::Path::new(&directory).join(file_name(&scene.name))).unwrap();
        let reference: Vec<f32> = bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|b| f32::from_le_bytes(*b))
            .collect();
        let now = render(&scene);
        let rate = f64::from(scene.rate);
        let (a, b) = (2 * (from * rate) as usize, 2 * (to * rate) as usize);
        let change = level_difference(&reference[a..b], &now[a..b], scene.rate);
        let nudge = level_difference(&now[a..b], &render(&nudged(&scene))[a..b], scene.rate);
        println!("{name:28} held {from}-{to} s: change {change:.2} dB, nudge {nudge:.2} dB");
    }
}
