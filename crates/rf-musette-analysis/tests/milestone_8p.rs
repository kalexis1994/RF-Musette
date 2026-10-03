//! Milestone 8p: the free bass. Predictions as written in docs/ROADMAP.md
//! before the code; each test says which.

use rf_musette_analysis::{attack_time, cents, component_envelope, open_curtain, sounding_through};
use rf_musette_dsp::compass::{FREE_FIRST, FREE_NOTES, free_design, free_target};
use rf_musette_dsp::parameters::{self, BASS_SYSTEM, FREE_BASS, Parameters, STIFF};
use rf_musette_dsp::{Engine, PULL_REED, PUSH_REED};

const RATE: f32 = 48_000.0;

fn engine() -> Box<Engine> {
    let mut engine = Box::new(Engine::new(RATE).unwrap());
    // The bellows played: Key Touch off, or it rests (9h again).
    assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
    assert!(engine.set_parameter(BASS_SYSTEM, FREE_BASS));
    engine
}

/// Prediction 2: every free-bass note's 8′ reed speaks at 300 Pa, and once
/// tuned sounds within 5 cents of its pitch; the 4′ likewise wherever it
/// speaks at 300 Pa. Through its pallet's open curtain, as it is tuned and
/// played since the tuning as played.
#[test]
fn every_free_bass_reed_speaks_in_tune() {
    let p = Parameters::default();
    let mut failures = Vec::new();
    for voice in 0..2 {
        for index in 0..FREE_NOTES {
            let note = FREE_FIRST + index as u8;
            let design = free_design(&p, note, voice).unwrap();
            let aim = free_target(&p, note, voice).unwrap();
            match sounding_through(design, 300.0, open_curtain(&p, &design)) {
                Some(tone) => {
                    let off = cents(aim, tone.frequency);
                    println!("voice {voice} note {note}: {off:+.1} cents");
                    if off.abs() > 5.0 {
                        failures.push(format!("voice {voice} note {note}: {off:+.1} cents"));
                    }
                }
                None if voice == 0 => failures.push(format!("8′ note {note} silent at 300 Pa")),
                None => println!("voice {voice} note {note}: silent at 300 Pa"),
            }
        }
    }
    assert!(failures.is_empty(), "{failures:?}");
}

/// The frequency of a reed's swing from its upward crossings of its mean.
fn frequency(zeta: &[f64]) -> Option<f64> {
    let mean = zeta.iter().sum::<f64>() / zeta.len() as f64;
    let crossings: Vec<f64> = (1..zeta.len())
        .filter(|&i| zeta[i - 1] < mean && zeta[i] >= mean)
        .map(|i| {
            let (a, b) = (zeta[i - 1] - mean, zeta[i] - mean);
            i as f64 - 1.0 + a / (a - b)
        })
        .collect();
    (crossings.len() >= 10).then(|| {
        (crossings.len() - 1) as f64 * f64::from(RATE)
            / (crossings[crossings.len() - 1] - crossings[0])
    })
}

/// Prediction 3: a note played under the split with Free Bass sounds at its
/// own pitch, within 5 cents -- and with Stradella it does not.
///
/// Not met at first: the E1 at −11.9 cents, the tuner tuning a reed with
/// its pallet away while the engine plays it through the curtain. Met since
/// the tuner tunes as played: E1 −4.8 (the arm's sag), E2 +2.8, B2 +2.4
/// (docs/ROADMAP.md, 8p).
#[test]
fn a_key_under_the_split_plays_its_own_note() {
    let p = Parameters::default();
    for note in [28u8, 40, 47] {
        let mut engine = engine();
        engine.channel_note_on(0, note, 1.0);
        assert!(engine.is_free_held(note), "note {note} is the free bass's");
        let mut block = [0.0f32; 256];
        for _ in 0..(1.5 * RATE / 256.0) as usize {
            engine.render(&mut block);
        }
        let mut zeta = [Vec::new(), Vec::new()];
        let mut one = [0.0f32; 1];
        for _ in 0..(1.0 * RATE) as usize {
            engine.render(&mut one);
            for which in [PULL_REED, PUSH_REED] {
                zeta[which].push(engine.free_reed(note, 0, which).unwrap().1.zeta);
            }
        }
        let spread = |z: &[f64]| {
            let mean = z.iter().sum::<f64>() / z.len() as f64;
            z.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
        };
        let sounding = if spread(&zeta[PULL_REED]) > spread(&zeta[PUSH_REED]) {
            &zeta[PULL_REED]
        } else {
            &zeta[PUSH_REED]
        };
        let f = frequency(sounding).expect("a tone");
        let off = cents(free_target(&p, note, 0).unwrap(), f);
        println!("note {note}: {f:.2} Hz, {off:+.1} cents");
        assert!(off.abs() <= 5.0, "note {note}: {off:+.1} cents");
        engine.channel_note_off(0, note);
        assert!(!engine.is_free_held(note));
    }
    // With Stradella the same key is a bass button, not a free note.
    let mut stradella = Box::new(Engine::new(RATE).unwrap());
    assert!(stradella.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
    stradella.channel_note_on(0, 40, 1.0);
    assert!(!stradella.is_free_held(40));
}

/// Prediction 4: the free bass's lowest note attacks slower than the bass
/// side's C2 (143 ms): E1 over 143 ms at 400 Pa, the bellows stiff.
#[test]
fn the_lowest_note_attacks_slower() {
    let p = Parameters::default();
    let mut engine = engine();
    assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
    engine.bellows_mut().expression_wide(0.4f32.sqrt());
    let mut block = [0.0f32; 256];
    for _ in 0..(0.3 * RATE / 256.0) as usize {
        engine.render(&mut block);
    }
    engine.free_on(FREE_FIRST, 1.0);
    let mut out = Vec::new();
    for _ in 0..(3.0 * RATE / 256.0) as usize {
        engine.render(&mut block);
        out.extend(block.iter().map(|x| f64::from(*x)));
    }
    let aim = free_target(&p, FREE_FIRST, 0).unwrap();
    let attack = attack_time(&component_envelope(&out, f64::from(RATE), aim, 4.0, 0.001))
        .expect("an attack");
    println!("E1 at 400 Pa: {:.0} ms", attack * 1e3);
    assert!(attack > 0.143, "{:.0} ms", attack * 1e3);
}

/// Prediction 5: four free-bass notes struck at once, at 48 kHz in blocks
/// of 128, take under the block's 2.67 ms -- the slowest of the first
/// second's blocks.
#[test]
#[ignore = "measurement: run in release, prints the slowest block"]
fn four_free_notes_fit_a_block() {
    let mut engine = engine();
    engine.wheel_msb(100);
    let mut block = [0.0f32; 128];
    for _ in 0..100 {
        engine.render(&mut block);
    }
    let mut slowest = 0.0f64;
    for note in [28u8, 35, 40, 44] {
        engine.channel_note_on(0, note, 1.0);
    }
    for _ in 0..(RATE / 128.0) as usize {
        let start = std::time::Instant::now();
        engine.render(&mut block);
        slowest = slowest.max(start.elapsed().as_secs_f64());
    }
    println!("slowest block {:.2} ms of 2.67", slowest * 1e3);
    assert!(slowest < 128.0 / f64::from(RATE), "{:.2} ms", slowest * 1e3);
}

/// Where the E1 in the engine sounds, and the bellows' pressure under it:
/// with the arm (the default) and stiff, at two times.
#[test]
#[ignore = "diagnosis: prints the E1's pitch and the pressure under it"]
fn the_e1_in_the_engine() {
    let p = Parameters::default();
    for stiff in [false, true] {
        let mut engine = engine();
        if stiff {
            assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
        }
        engine.channel_note_on(0, 28, 1.0);
        let mut block = [0.0f32; 256];
        let mut seconds = 0.0;
        for until in [1.5, 3.0, 5.0] {
            while seconds < until {
                engine.render(&mut block);
                seconds += 256.0 / f64::from(RATE);
            }
            let mut zeta = Vec::new();
            let mut one = [0.0f32; 1];
            for _ in 0..(0.5 * RATE) as usize {
                engine.render(&mut one);
                zeta.push(engine.free_reed(28, 0, PULL_REED).unwrap().1.zeta);
            }
            seconds += 0.5;
            let f = frequency(&zeta).unwrap_or(f64::NAN);
            println!(
                "stiff {stiff}, {until:.1} s: {:.1} cents, bellows {:.0} Pa",
                cents(free_target(&p, 28, 0).unwrap(), f),
                engine.supply()
            );
        }
    }
}

/// How far the engine's low reeds sound from their targets with the bellows
/// stiff at 300 Pa, where they are tuned: the free bass's and the Stradella
/// bass's 16′ -- the tuner tunes a reed with its pallet away, the engine
/// plays it through its pallet's curtain.
#[test]
#[ignore = "diagnosis: prints the low reeds' pitch in the engine"]
fn the_low_reeds_in_the_engine() {
    use rf_musette_dsp::compass::bass_target;
    let p = Parameters::default();
    let measure = |engine: &mut Engine, reed: &dyn Fn(&Engine) -> f64| {
        let mut block = [0.0f32; 256];
        for _ in 0..(2.0 * RATE / 256.0) as usize {
            engine.render(&mut block);
        }
        let mut zeta = Vec::new();
        let mut one = [0.0f32; 1];
        for _ in 0..(1.0 * RATE) as usize {
            engine.render(&mut one);
            zeta.push(reed(engine));
        }
        frequency(&zeta).unwrap_or(f64::NAN)
    };
    for note in [28u8, 33, 40, 47, 52, 64, 76] {
        let mut engine = engine();
        assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
        engine.free_on(note, 1.0);
        let f = measure(&mut engine, &|e: &Engine| {
            e.free_reed(note, 0, PULL_REED).unwrap().1.zeta
        });
        println!(
            "free bass 8′ note {note}: {:+.1} cents",
            cents(free_target(&p, note, 0).unwrap(), f)
        );
    }
    for pitch_class in [0usize, 4, 9] {
        let mut engine = Box::new(Engine::new(RATE).unwrap());
        assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
        assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
        assert!(engine.set_parameter(parameters::BASS_REGISTER, 0.0 + 3.0));
        engine.bass_on(36 + pitch_class as u8, 1.0);
        let f = measure(&mut engine, &|e: &Engine| {
            e.bass_reed(pitch_class, parameters::BASS_16, PULL_REED)
                .unwrap()
                .1
                .zeta
        });
        println!(
            "Stradella 16′ pitch class {pitch_class}: {:+.1} cents",
            cents(
                bass_target(&p, pitch_class, parameters::BASS_16).unwrap(),
                f
            )
        );
    }
}

/// The treble in the engine after tuning as played: a few keys' ranks, the
/// bellows stiff at 300 Pa, each register opening the rank alone.
#[test]
#[ignore = "diagnosis: prints the treble's pitch in the engine"]
fn the_treble_in_the_engine() {
    use rf_musette_dsp::compass::target;
    let p = Parameters::default();
    // Registers opening one rank: Bassoon (L), Clarinet (M), Piccolo (H).
    for (rank, register, keys) in [
        (parameters::RANK_LOW, 0.0, [53u8, 65, 81]),
        (parameters::RANK_MIDDLE, 11.0, [53, 65, 81]),
        (parameters::RANK_HIGH, 13.0, [53, 65, 81]),
    ] {
        for key in keys {
            let mut engine = Box::new(Engine::new(RATE).unwrap());
            assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
            assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
            if !engine.set_parameter(parameters::REGISTER, register) {
                println!("register {register} refused");
                continue;
            }
            engine.note_on(key, 1.0);
            let mut block = [0.0f32; 256];
            for _ in 0..(1.5 * RATE / 256.0) as usize {
                engine.render(&mut block);
            }
            let mut zeta = Vec::new();
            let mut one = [0.0f32; 1];
            for _ in 0..(1.0 * RATE) as usize {
                engine.render(&mut one);
                zeta.push(engine.reed(key, rank, PULL_REED).unwrap().1.zeta);
            }
            let f = frequency(&zeta).unwrap_or(f64::NAN);
            println!(
                "rank {rank} key {key}: {:+.1} cents",
                cents(target(&p, key, rank), f)
            );
        }
    }
}
