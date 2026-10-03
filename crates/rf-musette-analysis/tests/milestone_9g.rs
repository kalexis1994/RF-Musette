//! Milestone 9g: the keys' own dynamics -- how much a key held part-way
//! down, its pallet part-open, changes a note. Predictions as written in
//! docs/ROADMAP.md before the measurement. Through the engine as the player
//! plays it: Musette Paris, Pallet Opening 35 ms, the bellows resting at
//! 300 Pa or at the wheel's 79 % (CC 1 = 100).

use rf_musette_analysis::{attack_time, cents, component_envelope};
use rf_musette_dsp::compass::target;
use rf_musette_dsp::parameters::{self, Parameters, RANK_MIDDLE};
use rf_musette_dsp::{Engine, PULL_REED, PUSH_REED, programs};

const RATE: f32 = 48_000.0;
const DEPTHS: [f64; 17] = [
    1.0, 0.8, 0.67, 0.5, 0.4, 0.33, 0.25, 0.2, 0.17, 0.15, 0.13, 0.12, 0.11, 0.1, 0.09, 0.08, 0.06,
];
/// F3, D4, F4, A5.
const NOTES: [u8; 4] = [53, 62, 65, 81];

#[derive(Clone, Copy)]
struct Reading {
    depth: f64,
    /// The steady level, dB; `None` when the note does not sound.
    level: Option<f64>,
    /// The M reed's frequency, Hz.
    frequency: Option<f64>,
    /// Llanos's attack time, s.
    attack: Option<f64>,
}

fn engine(wheel: Option<u8>) -> Engine {
    let mut engine = Engine::new(RATE).unwrap();
    let values = programs::program("musette-paris").unwrap().parameters();
    for (index, value) in values.values().iter().enumerate() {
        assert!(engine.set_parameter(index, *value));
    }
    // The bellows played: Key Touch off, or it rests (9h again).
    assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
    assert!(engine.set_parameter(parameters::PALLET_OPENING, 35.0));
    if let Some(value) = wheel {
        engine.wheel_msb(value);
    }
    engine
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

fn read(note: u8, wheel: Option<u8>, depth: f64) -> (Reading, f64) {
    let mut engine = engine(wheel);
    let mut block = [0.0f32; 256];
    for _ in 0..(0.3 * RATE / 256.0) as usize {
        engine.render(&mut block);
    }
    let supply = engine.supply();
    engine.press(note, depth);
    let mut out = Vec::new();
    for _ in 0..(1.8 * RATE / 256.0) as usize {
        engine.render(&mut block);
        out.extend(block.iter().map(|x| f64::from(*x)));
    }
    // The last 0.7 s a sample at a time, for the M reed's swing: whichever
    // of the plate's two reeds the bellows' direction sounds.
    let mut zeta = [Vec::new(), Vec::new()];
    let mut one = [0.0f32; 1];
    for _ in 0..(0.7 * RATE) as usize {
        engine.render(&mut one);
        out.push(f64::from(one[0]));
        for which in [PULL_REED, PUSH_REED] {
            let (_, state) = engine.reed(note, RANK_MIDDLE, which).unwrap();
            zeta[which].push(state.zeta);
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
    let decibels = |span: &[f64]| {
        let rms = (span.iter().map(|x| x * x).sum::<f64>() / span.len() as f64).sqrt();
        20.0 * (rms + 1e-300).log10()
    };
    let split = (1.8 * RATE) as usize;
    let (early, late) = (
        decibels(&out[(1.1 * RATE) as usize..split]),
        decibels(&out[split..]),
    );
    // A tone that sounds holds its level: one still dying at the edge,
    // where the key's depth no longer lets the reed keep going, does not.
    // (The first run counted those, and its ranges ran to 75 dB.)
    let level = ((early - late).abs() < 1.0 && late > -140.0).then_some(late);
    let frequency = level.and(frequency(sounding));
    let p = Parameters::default();
    let attack = level.and_then(|_| {
        attack_time(&component_envelope(
            &out,
            f64::from(RATE),
            target(&p, note, RANK_MIDDLE),
            4.0,
            0.001,
        ))
    });
    (
        Reading {
            depth,
            level,
            frequency,
            attack,
        },
        supply,
    )
}

/// One note at one bellows: every depth, the deepest first.
fn sweep(note: u8, wheel: Option<u8>) -> Vec<Reading> {
    let mut readings = Vec::new();
    let mut silent = 0;
    for depth in DEPTHS {
        let (reading, supply) = read(note, wheel, depth);
        if depth == 1.0 {
            println!("note {note}, bellows {supply:.0} Pa");
        }
        let full = readings.first().copied().unwrap_or(reading);
        // Within 40 dB of the full tone, or it is not a note to play.
        let reading = Reading {
            level: reading
                .level
                .filter(|level| full.level.is_none_or(|full| level - full > -40.0)),
            ..reading
        };
        let fall = reading
            .level
            .zip(full.level)
            .map(|(level, full)| level - full);
        let reading = Reading {
            frequency: reading.level.and(reading.frequency),
            attack: reading.level.and(reading.attack),
            ..reading
        };
        let bend = reading
            .frequency
            .zip(full.frequency)
            .map(|(f, full)| cents(full, f));
        println!(
            "  depth {depth:>4}: level {:>7} dB  pitch {:>7} cents  attack {:>6} ms",
            fall.map_or("silent".to_owned(), |d| format!("{d:+.1}")),
            bend.map_or("-".to_owned(), |c| format!("{c:+.1}")),
            reading
                .attack
                .map_or("-".to_owned(), |a| format!("{:.0}", a * 1e3)),
        );
        readings.push(reading);
        // Two silent depths in a row: shallower ones are silent too.
        silent = if reading.level.is_none() {
            silent + 1
        } else {
            0
        };
        if silent == 2 {
            break;
        }
    }
    readings
}

/// The range a key's depth spans: full depth's level less the shallowest
/// sounding depth's, and that depth's reading.
fn range(readings: &[Reading]) -> (f64, Reading) {
    let full = readings[0].level.unwrap();
    let last = readings
        .iter()
        .rfind(|reading| reading.level.is_some())
        .copied()
        .unwrap();
    (full - last.level.unwrap(), last)
}

fn at(readings: &[Reading], depth: f64) -> Reading {
    *readings
        .iter()
        .find(|reading| reading.depth == depth)
        .unwrap()
}

/// The measurement, every prediction checked and each result printed
/// before any is asserted.
#[test]
#[ignore = "a measurement; prediction 6 and the F3 at 300 Pa in 5 not met (docs/ROADMAP.md, 9g): run with --ignored --nocapture"]
fn the_keys_own_dynamics() {
    let mut failures = Vec::new();
    let mut ranges = Vec::new();
    for wheel in [None, Some(100)] {
        for note in NOTES {
            let readings = sweep(note, wheel);
            let full = readings[0];
            let (range, last) = range(&readings);
            println!(
                "  range {range:.1} dB, shallowest sounding depth {}",
                last.depth
            );
            ranges.push((wheel, note, range));
            // 1: the curtain is the hole from two thirds down.
            let two_thirds = at(&readings, 0.67);
            let moved = two_thirds.level.unwrap() - full.level.unwrap();
            let bent = cents(full.frequency.unwrap(), two_thirds.frequency.unwrap());
            if moved.abs() >= 0.5 || bent.abs() >= 1.0 {
                failures.push(format!(
                    "1: note {note} {wheel:?}: {moved:+.2} dB, {bent:+.2} cents at 2/3"
                ));
            }
            // 5: most of the fall below a third.
            let third = at(&readings, 0.33);
            if let Some(level) = third.level {
                let upper = two_thirds.level.unwrap() - level;
                if upper > range / 3.0 {
                    failures.push(format!(
                        "5: note {note} {wheel:?}: {upper:.1} of {range:.1} dB above a third"
                    ));
                }
            }
            // 6: flat and slow at the shallowest.
            let bend = cents(full.frequency.unwrap(), last.frequency.unwrap_or(f64::NAN));
            if !(-30.0..=-10.0).contains(&bend) {
                failures.push(format!(
                    "6: note {note} {wheel:?}: {bend:+.1} cents at depth {}",
                    last.depth
                ));
            }
            if let (Some(slow), Some(fast)) = (last.attack, full.attack)
                && slow <= fast
            {
                failures.push(format!(
                    "6: note {note} {wheel:?}: attack {:.0} ms against {:.0}",
                    slow * 1e3,
                    fast * 1e3
                ));
            }
        }
    }
    let of = |wheel: Option<u8>, note: u8| {
        ranges
            .iter()
            .find(|(w, n, _)| *w == wheel && *n == note)
            .unwrap()
            .2
    };
    // 2: the F4 at 300 Pa.
    let f4 = of(None, 65);
    if !(8.0..=15.0).contains(&f4) {
        failures.push(format!("2: F4 at 300 Pa spans {f4:.1} dB"));
    }
    // 3: the range shrinks with pitch.
    if of(None, 81) >= 6.0 || of(None, 53) <= f4 {
        failures.push(format!(
            "3: F3 {:.1}, F4 {f4:.1}, A5 {:.1} dB at 300 Pa",
            of(None, 53),
            of(None, 81)
        ));
    }
    // 4: wider with the wheel.
    for note in NOTES {
        if of(Some(100), note) <= of(None, note) {
            failures.push(format!(
                "4: note {note}: {:.1} dB at the wheel's 79 %, {:.1} at 300 Pa",
                of(Some(100), note),
                of(None, note)
            ));
        }
    }
    for failure in &failures {
        println!("NOT MET {failure}");
    }
    assert!(
        failures.is_empty(),
        "{} predictions not met",
        failures.len()
    );
}
