//! Milestone 9b: the microphones and the room. Predictions as written in
//! docs/ROADMAP.md before the code; each test says which.

use rf_musette_dsp::parameters::{self, Parameters};
use rf_musette_dsp::stage::{self, Room, SOURCES, Stage};
use rf_musette_dsp::{BASS_CHANNEL, Engine};

const RATE: f64 = 48_000.0;
/// The bellows' whole opening at the defaults: 12 L over 600 cm².
const TRAVEL: f64 = 0.2;

fn settings(pairs: &[(usize, f64)]) -> Parameters {
    let mut parameters = Parameters::default();
    for (index, value) in pairs {
        assert!(parameters.set(*index, *value), "{index} = {value}");
    }
    parameters
}

fn stage(pairs: &[(usize, f64)]) -> Stage {
    let mut stage = Stage::new(RATE);
    stage.tune(&settings(pairs), TRAVEL);
    stage
}

/// The room all but off, every capsule to its own side, the player's
/// left the left.
const DRY_ROOM: [(usize, f64); 3] = [
    (parameters::ROOM_LEVEL, -40.0),
    (parameters::STEREO_WIDTH, 1.0),
    (parameters::PERSPECTIVE, parameters::PLAYER),
];

/// Feeds `signal(n)` to one source for `samples`, the bellows opened as
/// `extension(n)` says; the two channels.
fn feed(
    stage: &mut Stage,
    source: usize,
    samples: usize,
    signal: impl Fn(usize) -> f64,
    extension: impl Fn(usize) -> f64,
) -> (Vec<f64>, Vec<f64>) {
    let mut left = Vec::with_capacity(samples);
    let mut right = Vec::with_capacity(samples);
    for n in 0..samples {
        let mut sources = [0.0; SOURCES];
        sources[source] = signal(n);
        let (l, r) = stage.process(&sources, extension(n));
        left.push(l);
        right.push(r);
    }
    (left, right)
}

fn sine(frequency: f64) -> impl Fn(usize) -> f64 {
    move |n| (2.0 * std::f64::consts::PI * frequency * n as f64 / RATE).sin()
}

fn rms(signal: &[f64]) -> f64 {
    (signal.iter().map(|x| x * x).sum::<f64>() / signal.len() as f64).sqrt()
}

fn db(ratio: f64) -> f64 {
    20.0 * ratio.log10()
}

fn distance(a: stage::Point, b: stage::Point) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

/// The bass stand of Two spots: off the middle of the bass box's travel,
/// out along its grille's axis (stage.rs's layout).
fn bass_stand(distance: f64) -> stage::Point {
    let middle = stage::bass_position(TRAVEL / 2.0);
    let axis = std::f64::consts::FRAC_1_SQRT_2;
    [
        middle[0] - axis * distance,
        middle[1] + axis * distance,
        middle[2],
    ]
}

/// Prediction 2: a stand mic, the room off -- opening the bellows moves a
/// held bass note's level by 20 log₁₀ of the distance ratio within 0.5 dB,
/// and its arrival by the path's change over c within one sample.
#[test]
fn a_stand_mic_hears_the_bass_box_come_and_go() {
    let pairs = [
        (parameters::MIC_LAYOUT, stage::SPOTS as f64),
        (parameters::SPOTS_PATTERN, 0.0),
    ];
    let all: Vec<_> = pairs.iter().chain(DRY_ROOM.iter()).copied().collect();
    let mic = bass_stand(0.30);
    let mut levels = Vec::new();
    let mut arrivals = Vec::new();
    for extension in [0.0, TRAVEL] {
        let mut s = stage(&all);
        let (left, _) = feed(&mut s, stage::BASS_SOURCE, 12_000, sine(500.0), |_| {
            extension
        });
        levels.push(rms(&left[4000..]));
        let mut s = stage(&all);
        let (left, _) = feed(
            &mut s,
            stage::BASS_SOURCE,
            2000,
            |n| if n == 10 { 1.0 } else { 0.0 },
            |_| extension,
        );
        let peak = left
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
            .unwrap()
            .0;
        arrivals.push(peak as f64);
    }
    let near = distance(stage::bass_position(0.0), mic);
    let far = distance(stage::bass_position(TRAVEL), mic);
    let expected = db(near / far);
    let measured = db(levels[1] / levels[0]);
    let delay = (far - near) / 343.2 * RATE;
    println!(
        "shut {near:.3} m, open {far:.3} m: {measured:+.2} dB (distance {expected:+.2}); arrival {:+.1} samples (path {delay:+.1})",
        arrivals[1] - arrivals[0]
    );
    assert!((measured - expected).abs() <= 0.5, "{measured:+.2} dB");
    assert!(((arrivals[1] - arrivals[0]) - delay).abs() <= 1.0);
}

/// Prediction 3: a mounted bass-side mic, the room off -- the bass note's
/// level does not move with the bellows, within 0.1 dB.
#[test]
fn a_mounted_mic_travels_with_the_bass_box() {
    let pairs = [(parameters::MIC_LAYOUT, stage::CLIP_ON as f64)];
    let all: Vec<_> = pairs.iter().chain(DRY_ROOM.iter()).copied().collect();
    let mut levels = Vec::new();
    for extension in [0.0, TRAVEL] {
        let mut s = stage(&all);
        let (left, _) = feed(&mut s, stage::BASS_SOURCE, 12_000, sine(500.0), |_| {
            extension
        });
        levels.push(rms(&left[4000..]));
    }
    let change = db(levels[1] / levels[0]);
    println!("mounted: {change:+.3} dB");
    assert!(change.abs() <= 0.1, "{change:+.3} dB");
}

/// The times a signal crosses zero rising, between its samples.
fn rising_crossings(signal: &[f64]) -> Vec<f64> {
    let mut times = Vec::new();
    for n in 1..signal.len() {
        if signal[n - 1] < 0.0 && signal[n] >= 0.0 {
            times.push(n as f64 - 1.0 + signal[n - 1] / (signal[n - 1] - signal[n]));
        }
    }
    times
}

/// Prediction 4: the bass box moving at a steady speed past a stand mic
/// shifts its note by the path's rate over c, within 20 %.
#[test]
fn a_moving_bass_box_is_heard_shifted() {
    let pairs = [
        (parameters::MIC_LAYOUT, stage::SPOTS as f64),
        (parameters::SPOTS_PATTERN, 0.0),
    ];
    let all: Vec<_> = pairs.iter().chain(DRY_ROOM.iter()).copied().collect();
    let speed = 0.5;
    let frequency = 500.0;
    let mut s = stage(&all);
    let extension = move |n: usize| speed * n as f64 / RATE;
    let samples = (0.4 * RATE) as usize;
    let (left, _) = feed(
        &mut s,
        stage::BASS_SOURCE,
        samples,
        sine(frequency),
        extension,
    );
    let crossings = rising_crossings(&left[2000..]);
    let (first, last) = (crossings[0], *crossings.last().unwrap());
    let measured = (crossings.len() - 1) as f64 / ((last - first) / RATE);
    // The path's change over the same span, from the source's side: the
    // sound heard at t left the box r/c earlier.
    let mic = bass_stand(0.30);
    let start = distance(stage::bass_position(extension(2000 + first as usize)), mic);
    let end = distance(stage::bass_position(extension(2000 + last as usize)), mic);
    let rate = (end - start) / ((last - first) / RATE);
    let expected = frequency / (1.0 + rate / 343.2);
    let cents = |f: f64| 1200.0 * (f / frequency).log2();
    println!(
        "moving {speed} m/s: {:+.3} cents (path {:+.3} m/s: {:+.3} cents)",
        cents(measured),
        rate,
        cents(expected)
    );
    assert!(
        cents(expected).abs() > 0.5,
        "the test moves the box too little"
    );
    assert!(
        (cents(measured) - cents(expected)).abs() <= 0.2 * cents(expected).abs(),
        "{:+.3} against {:+.3} cents",
        cents(measured),
        cents(expected)
    );
}

/// Each treble quarter's level on the internal treble capsules.
fn quarters(capsules: f64) -> [f64; 4] {
    let pairs = [
        (parameters::MIC_LAYOUT, stage::INTERNAL as f64),
        (parameters::INTERNAL_TREBLE, capsules),
    ];
    let all: Vec<_> = pairs.iter().chain(DRY_ROOM.iter()).copied().collect();
    let mut levels = [0.0; 4];
    for (quarter, level) in levels.iter_mut().enumerate() {
        let mut s = stage(&all);
        let (_, right) = feed(&mut s, quarter, 12_000, sine(300.0), |_| 0.1);
        *level = db(rms(&right[4000..]));
    }
    levels
}

/// Prediction 5: internal, two treble capsules at the reed blocks' ends --
/// the notes under them at least 3 dB louder than one between; with six,
/// the spread less than half of that.
#[test]
fn notes_near_an_internal_capsule_are_louder() {
    let two = quarters(2.0);
    let six = quarters(6.0);
    let spread = |levels: [f64; 4]| {
        levels.iter().copied().fold(f64::MIN, f64::max)
            - levels.iter().copied().fold(f64::MAX, f64::min)
    };
    let ends = two[0].min(two[3]);
    let between = two[1].max(two[2]);
    println!(
        "two capsules: {:.1} {:.1} {:.1} {:.1} dB, ends over between {:+.1}; six: spread {:.1} against {:.1}",
        two[0],
        two[1],
        two[2],
        two[3],
        ends - between,
        spread(six),
        spread(two)
    );
    assert!(ends - between >= 3.0);
    assert!(spread(six) < spread(two) / 2.0);
}

/// A biquad band-pass, RBJ's, at `centre` Hz.
fn band(signal: &[f64], centre: f64, q: f64) -> Vec<f64> {
    let w = 2.0 * std::f64::consts::PI * centre / RATE;
    let alpha = w.sin() / (2.0 * q);
    let a0 = 1.0 + alpha;
    let (b0, b2, a1, a2) = (
        alpha / a0,
        -alpha / a0,
        -2.0 * w.cos() / a0,
        (1.0 - alpha) / a0,
    );
    let (mut x1, mut x2, mut y1, mut y2) = (0.0, 0.0, 0.0, 0.0);
    signal
        .iter()
        .map(|x| {
            let y = b0 * x + b2 * x2 - a1 * y1 - a2 * y2;
            x2 = x1;
            x1 = *x;
            y2 = y1;
            y1 = y;
            y
        })
        .collect()
}

/// Prediction 6: the room's tail decays at the RT60 Sabine gives for it,
/// within 15 %, in the middle band.
#[test]
fn the_tail_decays_as_sabine_says() {
    for (volume, hardness) in [(150.0, 0.4), (600.0, 0.6)] {
        let pairs = [
            (parameters::MIC_LAYOUT, stage::SINGLE as f64),
            (parameters::SINGLE_PATTERN, 0.0),
            (parameters::ROOM_SIZE, volume),
            (parameters::ROOM_HARDNESS, hardness),
        ];
        let mut s = stage(&pairs);
        let room = Room::new(volume, hardness);
        let seconds = (room.reverberation[1] * 1.5).max(1.0);
        let samples = (seconds * RATE) as usize;
        let (left, _) = feed(
            &mut s,
            1,
            samples,
            |n| if n == 0 { 1.0 } else { 0.0 },
            |_| 0.1,
        );
        let filtered = band(&left, 700.0, 1.4);
        // Schroeder's backward integral, and the time from −5 to −25 dB,
        // from 80 ms: the tail, not the direct sound and the first images.
        let start = (0.08 * RATE) as usize;
        let mut energy: Vec<f64> = filtered[start..].iter().map(|x| x * x).collect();
        for n in (0..energy.len() - 1).rev() {
            energy[n] += energy[n + 1];
        }
        let total = energy[0];
        let at = |level: f64| {
            energy
                .iter()
                .position(|e| 10.0 * (e / total).log10() <= level)
                .unwrap() as f64
                / RATE
        };
        let rt = 3.0 * (at(-25.0) - at(-5.0));
        let sabine = room.reverberation[1];
        println!(
            "{volume} m³, hardness {hardness}: tail {rt:.2} s, Sabine {sabine:.2} s ({:+.0} %)",
            (rt / sabine - 1.0) * 100.0
        );
        assert!(
            (rt / sabine - 1.0).abs() <= 0.15,
            "{rt:.2} s against {sabine:.2} s"
        );
    }
}

/// Prediction 8: the player's perspective puts the treble to the right and
/// the bass to the left; the audience's the reverse.
#[test]
fn the_perspective_says_whose_left_is_left() {
    for (perspective, treble_right) in [(parameters::PLAYER, true), (parameters::AUDIENCE, false)] {
        for (source, expect_right) in [(1, treble_right), (stage::BASS_SOURCE, !treble_right)] {
            let pairs = [
                (parameters::MIC_LAYOUT, stage::ORTF as f64),
                (parameters::PERSPECTIVE, perspective),
                (parameters::ROOM_LEVEL, -40.0),
            ];
            let mut s = stage(&pairs);
            let (left, right) = feed(&mut s, source, 12_000, sine(400.0), |_| 0.1);
            let difference = db(rms(&right[4000..]) / rms(&left[4000..]));
            println!(
                "perspective {perspective}, source {source}: right over left {difference:+.1} dB"
            );
            assert_eq!(difference > 0.0, expect_right);
        }
    }
}

/// Prediction 9: an omni capsule hears the room's tail against the direct
/// sound as Sabine says, within 2 dB: the reverberant energy 16π/A of the
/// source's power, the direct 1/r² of its level toward the capsule.
#[test]
fn the_room_is_as_loud_as_sabine_says() {
    // White noise, the same for both runs: an iterated generator (a
    // multiplier on the sample's number alone is no noise at all).
    let samples = (3.0 * RATE) as usize;
    let mut state = 7u64;
    let white: Vec<f64> = (0..samples)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            ((state >> 11) as f64 / (1u64 << 53) as f64) * 2.0 - 1.0
        })
        .collect();
    let distance_m = 1.0;
    let base = [
        (parameters::MIC_LAYOUT, stage::SINGLE as f64),
        (parameters::SINGLE_PATTERN, 0.0),
        (parameters::SINGLE_DISTANCE, distance_m),
    ];
    // The output with the room at its level and with it all but off, each
    // without its trim: the room's part is their difference, sample by
    // sample (the stage is linear).
    let heard = |room_level: f64| {
        let pairs: Vec<_> = base
            .iter()
            .copied()
            .chain([(parameters::ROOM_LEVEL, room_level)])
            .collect();
        let mut s = stage(&pairs);
        let (left, _) = feed(&mut s, 1, samples, |n| white[n], |_| 0.1);
        let trim = s.trim();
        left.into_iter().map(|x| x / trim).collect::<Vec<_>>()
    };
    let with_room = heard(0.0);
    let without = heard(-40.0);
    let room_only: Vec<f64> = with_room.iter().zip(&without).map(|(a, b)| a - b).collect();
    // In the middle band, where Sabine's area is reckoned.
    let energy = |signal: &[f64]| {
        let filtered = band(signal, 700.0, 1.4);
        let level = rms(&filtered[RATE as usize..]);
        level * level
    };
    let direct = energy(&without);
    let room_part = energy(&room_only);
    // Expected: the capsule sits in front, the quarter's directivity toward
    // it nearly whole.
    let room = Room::new(150.0, 0.4);
    let source = stage::treble_position(1);
    let mic = [0.0, 0.12 + distance_m, 1.03];
    let r = distance(source, mic);
    let toward = [mic[0] - source[0], mic[1] - source[1], mic[2] - source[2]];
    let lean = (1.0 - stage::SOURCE_DIRECTIVITY) + stage::SOURCE_DIRECTIVITY * toward[1] / r;
    let expected = (16.0 * std::f64::consts::PI / room.area
        * stage::diffuse(stage::SOURCE_DIRECTIVITY))
        / (lean * lean / (r * r));
    let measured = room_part / direct;
    println!(
        "room over direct at {r:.2} m: {:+.1} dB, Sabine {:+.1} dB (critical distance {:.2} m)",
        10.0 * measured.log10(),
        10.0 * expected.log10(),
        room.critical_distance()
    );
    assert!((10.0 * (measured / expected).log10()).abs() <= 2.0);
}

/// The loudness every layout gives a held chord and bass note, dB.
fn layout_loudness(layout: usize) -> f64 {
    let mut engine = Engine::new(RATE as f32).unwrap();
    // The bellows played: Key Touch off, or it rests (9h again).
    assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
    assert!(engine.set_parameter(parameters::MIC_LAYOUT, layout as f64));
    for key in [60, 64, 67] {
        engine.note_on(key, 0.8);
    }
    engine.channel_note_on(BASS_CHANNEL, 48, 0.8);
    let blocks = (3.0 * RATE / 256.0) as usize;
    let mut left = [0.0f32; 256];
    let mut right = [0.0f32; 256];
    let mut energy = 0.0f64;
    let mut count = 0usize;
    for block in 0..blocks {
        engine.render_stereo(&mut left, &mut right);
        if block * 256 >= RATE as usize {
            for (l, r) in left.iter().zip(&right) {
                energy += (f64::from(*l).powi(2) + f64::from(*r).powi(2)) / 2.0;
                count += 1;
            }
        }
    }
    10.0 * (energy / count as f64).log10()
}

/// Prediction 7: each layout's loudness of a held treble chord and bass
/// note within ±3 dB of the ORTF pair's.
#[test]
fn every_layout_keeps_the_loudness() {
    let reference = layout_loudness(stage::ORTF);
    let names = [
        "Internal",
        "Clip-on",
        "Two spots",
        "ORTF pair",
        "Spaced pair",
        "One mic",
        "Dry",
    ];
    for (layout, name) in names.iter().enumerate() {
        let loudness = layout_loudness(layout);
        println!(
            "{name}: {:+.1} dB against the ORTF pair",
            loudness - reference
        );
        assert!((loudness - reference).abs() <= 3.0, "{name}");
    }
}

/// Measured: what the microphones and the room cost per 48 kHz sample, the
/// four-note Master chord of milestone 7 against the mono render.
#[test]
#[ignore = "measurement: run in release, prints the cost"]
fn the_stage_costs() {
    let cost = |layout: Option<usize>| {
        let mut engine = Engine::new(RATE as f32).unwrap();
        assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
        assert!(engine.set_parameter(parameters::REGISTER, 6.0));
        if let Some(layout) = layout {
            assert!(engine.set_parameter(parameters::MIC_LAYOUT, layout as f64));
        }
        engine.bellows_mut().expression_wide(0.6);
        for key in [65, 69, 72, 77] {
            engine.note_on(key, 1.0);
        }
        let mut left = vec![0.0f32; 256];
        let mut right = vec![0.0f32; 256];
        let render = |engine: &mut Engine, left: &mut [f32], right: &mut [f32]| match layout {
            Some(_) => engine.render_stereo(left, right),
            None => engine.render(left),
        };
        for _ in 0..200 {
            render(&mut engine, &mut left, &mut right);
        }
        let blocks = 1000;
        let start = std::time::Instant::now();
        for _ in 0..blocks {
            render(&mut engine, &mut left, &mut right);
        }
        start.elapsed().as_secs_f64() / (blocks * 256) as f64 * 1e6
    };
    let mono = cost(None);
    println!("mono: {mono:.2} µs per sample");
    let names = [
        "Internal",
        "Clip-on",
        "Two spots",
        "ORTF pair",
        "Spaced pair",
        "One mic",
        "Dry",
    ];
    for (layout, name) in names.iter().enumerate() {
        let stereo = cost(Some(layout));
        println!(
            "{name}: {stereo:.2} µs per sample ({:+.2}, {:+.0} %)",
            stereo - mono,
            (stereo / mono - 1.0) * 100.0
        );
    }
}

/// Measured (2026-10-01, the player heard it clip): the peak and the level
/// the output reaches, dBFS, from a soft note to the loudest the
/// instrument plays, dry and through the ORTF pair.
#[test]
#[ignore = "measurement: prints the output's levels"]
fn the_output_levels() {
    struct Case {
        name: &'static str,
        register: f64,
        bass_register: f64,
        intent: f32,
        treble: &'static [u8],
        bass: &'static [u8],
    }
    let cases = [
        Case {
            name: "pp, C4 Clarinet",
            register: 11.0,
            bass_register: 3.0,
            intent: 0.4,
            treble: &[60],
            bass: &[],
        },
        Case {
            name: "mf, C4 Clarinet",
            register: 11.0,
            bass_register: 3.0,
            intent: 0.7,
            treble: &[60],
            bass: &[],
        },
        Case {
            name: "mf, chord and bass, Musette",
            register: 8.0,
            bass_register: 3.0,
            intent: 0.7,
            treble: &[60, 64, 67],
            bass: &[48],
        },
        Case {
            name: "ff, chord and bass, Master",
            register: 6.0,
            bass_register: 3.0,
            intent: 1.0,
            treble: &[60, 64, 67, 72],
            bass: &[48],
        },
        Case {
            name: "ff, two hands' full chords, Master",
            register: 6.0,
            bass_register: 3.0,
            intent: 1.0,
            treble: &[55, 60, 64, 67, 72, 76],
            bass: &[48, 43],
        },
    ];
    for case in &cases {
        for layout in [stage::DRY, stage::ORTF] {
            let mut engine = Engine::new(RATE as f32).unwrap();
            assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
            assert!(engine.set_parameter(parameters::REGISTER, case.register));
            assert!(engine.set_parameter(parameters::BASS_REGISTER, case.bass_register));
            assert!(engine.set_parameter(parameters::MIC_LAYOUT, layout as f64));
            engine.bellows_mut().expression_wide(case.intent);
            for key in case.treble {
                engine.note_on(*key, 1.0);
            }
            for key in case.bass {
                engine.channel_note_on(BASS_CHANNEL, *key, 1.0);
            }
            let mut left = [0.0f32; 256];
            let mut right = [0.0f32; 256];
            let mut peak = 0.0f32;
            let mut energy = 0.0f64;
            let mut count = 0;
            for block in 0..(4.0 * RATE / 256.0) as usize {
                engine.render_stereo(&mut left, &mut right);
                for (l, r) in left.iter().zip(&right) {
                    peak = peak.max(l.abs()).max(r.abs());
                    if block * 256 > RATE as usize {
                        energy += f64::from(*l).powi(2) + f64::from(*r).powi(2);
                        count += 2;
                    }
                }
            }
            println!(
                "{:<36} {:<5}: peak {:+.1} dBFS, rms {:+.1} dBFS",
                case.name,
                if layout == stage::DRY { "dry" } else { "ORTF" },
                20.0 * f64::from(peak).log10(),
                10.0 * (energy / count as f64).log10()
            );
        }
    }
}

/// Diagnosis (2026-10-01, the player heard the room ring like a closed
/// tube): the room's own impulse response -- with the room less without --
/// through one omni at 1 m, written for analysis to the file RF_ROOM_IR
/// names, one sample a line.
#[test]
#[ignore = "diagnosis: writes the room's impulse response"]
fn diagnose_the_room() {
    let path = std::env::var("RF_ROOM_IR").expect("RF_ROOM_IR");
    let base = [
        (parameters::MIC_LAYOUT, stage::SINGLE as f64),
        (parameters::SINGLE_PATTERN, 0.0),
        (parameters::SINGLE_DISTANCE, 1.0),
    ];
    let response = |room_level: f64| {
        let pairs: Vec<_> = base
            .iter()
            .copied()
            .chain([(parameters::ROOM_LEVEL, room_level)])
            .collect();
        let mut s = stage(&pairs);
        let (left, _) = feed(
            &mut s,
            1,
            (1.5 * RATE) as usize,
            |n| f64::from(u8::from(n == 0)),
            |_| 0.1,
        );
        let trim = s.trim();
        left.into_iter().map(|x| x / trim).collect::<Vec<_>>()
    };
    let with_room = response(0.0);
    let without = response(-40.0);
    let text: String = with_room
        .iter()
        .zip(&without)
        .map(|(a, b)| format!("{:e} {:e}\n", a - b, b))
        .collect();
    std::fs::write(path, text).unwrap();
}
