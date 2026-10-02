//! The scenes the engine's sound is held to while its cost is cut
//! (milestone 10), after RF-5's audition fingerprints: short performances
//! through every path the audio takes -- each program, the mono and the
//! stereo render, every oversampling factor, three sample rates, the
//! bellows by velocity, by expression, by the wheel and turning by itself,
//! a reversal, the air button, the idle path and the room's tail.
//!
//! A scene's fingerprint folds every output sample's bits into FNV-1a: a
//! change that is meant to be exact must leave every one alone; one that is
//! not is measured against the samples themselves (`compare`).

use rf_musette_dsp::Engine;
use rf_musette_dsp::parameters::{self, Parameters};
use rf_musette_dsp::programs::PROGRAMS;

/// What happens at a moment of a scene.
#[derive(Clone, Copy)]
pub enum Event {
    Note(u8, f32),
    NoteOff(u8),
    Bass(u8, f32),
    BassOff(u8),
    Chord(u8, f32),
    ChordOff(u8),
    Parameter(usize, f64),
    Expression(f32),
    /// The wheel, 0..1, at once.
    Wheel(f32),
}

pub struct Scene {
    pub name: String,
    pub rate: f32,
    pub stereo: bool,
    pub seconds: f64,
    pub parameters: Parameters,
    /// Seconds from the start, in order.
    pub events: Vec<(f64, Event)>,
    /// The wheel moved smoothly from one place to another over a time:
    /// (start s, end s, from, to), applied once per block.
    pub wheel_ramps: Vec<(f64, f64, f32, f32)>,
}

const BLOCK: usize = 128;

fn settings(pairs: &[(usize, f64)]) -> Parameters {
    let mut parameters = Parameters::default();
    for &(index, value) in pairs {
        assert!(parameters.set(index, value), "{index} = {value}");
    }
    parameters
}

/// A phrase both hands play: a triad rolled in, a bass and a chord, a
/// melody note over them, everything let go before the end so the release
/// and the room's tail are heard.
fn phrase() -> Vec<(f64, Event)> {
    vec![
        (0.0, Event::Note(60, 0.8)),
        (0.05, Event::Bass(36, 0.8)),
        (0.12, Event::Note(64, 0.7)),
        (0.2, Event::Chord(48, 0.7)),
        (0.24, Event::Note(67, 0.9)),
        (0.45, Event::NoteOff(64)),
        (0.47, Event::Note(72, 1.0)),
        (0.7, Event::BassOff(36)),
        (0.72, Event::ChordOff(48)),
        (0.8, Event::NoteOff(60)),
        (0.8, Event::NoteOff(67)),
        (0.85, Event::NoteOff(72)),
    ]
}

/// Milestone 7's four-note Master chord, held, under expression.
fn master_chord() -> Vec<(f64, Event)> {
    let mut events = vec![(0.0, Event::Expression(0.6))];
    events.extend([65, 69, 72, 77].map(|key| (0.0, Event::Note(key, 1.0))));
    events.extend([65, 69, 72, 77].map(|key| (0.8, Event::NoteOff(key))));
    events
}

pub fn scenes() -> Vec<Scene> {
    let mut scenes = Vec::new();
    let scene = |name: &str, rate: f32, stereo: bool, seconds: f64, parameters, events| Scene {
        name: name.to_owned(),
        rate,
        stereo,
        seconds,
        parameters,
        events,
        wheel_ramps: Vec::new(),
    };
    // Every program, the phrase through its microphones.
    for program in PROGRAMS {
        let mut events = phrase();
        if program.id == "keyboard-61" {
            // The left hand on one keyboard: below the split.
            events = vec![
                (0.0, Event::Note(65, 0.8)),
                (0.05, Event::Note(36, 0.8)),
                (0.2, Event::Note(52, 0.7)),
                (0.7, Event::NoteOff(36)),
                (0.72, Event::NoteOff(52)),
                (0.8, Event::NoteOff(65)),
            ];
        }
        let mut entry = scene(
            &format!("program {}", program.id),
            48_000.0,
            true,
            1.2,
            program.parameters(),
            events,
        );
        if program.id == "auto-bellows" {
            // Two litres of travel and a full chord held: the bellows runs
            // out and turns by itself mid-chord.
            entry.seconds = 2.5;
            entry.events = vec![(0.0, Event::Parameter(parameters::BELLOWS_TRAVEL, 2.0))];
            entry
                .events
                .extend([53, 60, 64, 67, 72].map(|key| (0.0, Event::Note(key, 1.0))));
            entry
                .events
                .extend([53, 60, 64, 67, 72].map(|key| (2.2, Event::NoteOff(key))));
        }
        if program.id == "wheel-bellows" {
            // The wheel opens the bellows, then closes it: a reversal.
            entry.wheel_ramps = vec![(0.0, 0.5, 0.0, 1.0), (0.6, 1.1, 1.0, 0.2)];
        }
        scenes.push(entry);
    }
    let master = settings(&[(parameters::REGISTER, 6.0)]);
    scenes.push(scene(
        "master chord, mono",
        48_000.0,
        false,
        1.0,
        master,
        master_chord(),
    ));
    for factor in [1.0, 4.0] {
        let mut p = master;
        assert!(p.set(parameters::OVERSAMPLING, factor));
        scenes.push(scene(
            &format!("master chord, mono, {factor}x"),
            48_000.0,
            false,
            1.0,
            p,
            master_chord(),
        ));
    }
    scenes.push(scene(
        "phrase at 44.1 kHz, ORTF",
        44_100.0,
        true,
        1.2,
        settings(&[(parameters::MIC_LAYOUT, 3.0)]),
        phrase(),
    ));
    scenes.push(scene(
        "phrase at 96 kHz, dry",
        96_000.0,
        true,
        1.2,
        settings(&[(parameters::MIC_LAYOUT, 6.0)]),
        phrase(),
    ));
    // A reversal mid-note, then the air button.
    scenes.push(scene(
        "reversal and air button",
        48_000.0,
        true,
        1.2,
        settings(&[(parameters::MIC_LAYOUT, 0.0)]),
        vec![
            (0.0, Event::Note(62, 0.9)),
            (0.0, Event::Bass(38, 0.9)),
            (
                0.4,
                Event::Parameter(parameters::BELLOWS_DIRECTION, parameters::PUSH),
            ),
            (0.75, Event::Parameter(parameters::AIR_VALVE, 1.0)),
            (0.95, Event::Parameter(parameters::AIR_VALVE, 0.0)),
            (1.0, Event::NoteOff(62)),
            (1.0, Event::BassOff(38)),
        ],
    ));
    // The wheel as pressure, swelling and falling under a held chord.
    let mut swell = scene(
        "wheel as pressure",
        48_000.0,
        true,
        1.2,
        settings(&[(parameters::MOD_WHEEL, parameters::WHEEL_PRESSURE)]),
        vec![
            (0.0, Event::Wheel(0.1)),
            (0.0, Event::Note(57, 0.7)),
            (0.0, Event::Note(64, 0.7)),
            (1.0, Event::NoteOff(57)),
            (1.0, Event::NoteOff(64)),
        ],
    );
    swell.wheel_ramps = vec![(0.05, 0.5, 0.1, 0.9), (0.5, 0.9, 0.9, 0.3)];
    scenes.push(swell);
    // Silence first -- the idle path -- then a short note, and the room's
    // tail after it in a hard hall.
    scenes.push(scene(
        "silence, a note, the hall's tail",
        48_000.0,
        true,
        1.5,
        settings(&[
            (parameters::MIC_LAYOUT, 4.0),
            (parameters::ROOM_SIZE, 1500.0),
            (parameters::ROOM_HARDNESS, 0.8),
        ]),
        vec![(0.3, Event::Note(69, 1.0)), (0.5, Event::NoteOff(69))],
    ));
    scenes
}

fn apply(engine: &mut Engine, event: Event) {
    match event {
        Event::Note(key, velocity) => engine.note_on(key, velocity),
        Event::NoteOff(key) => engine.note_off(key),
        Event::Bass(key, velocity) => engine.bass_on(key, velocity),
        Event::BassOff(key) => engine.bass_off(key),
        Event::Chord(key, velocity) => engine.chord_on(key, velocity),
        Event::ChordOff(key) => engine.chord_off(key),
        Event::Parameter(index, value) => {
            assert!(engine.set_parameter(index, value), "{index} = {value}");
        }
        Event::Expression(value) => engine.bellows_mut().expression_wide(value),
        Event::Wheel(value) => engine.wheel_wide(value),
    }
}

/// The scene's output, interleaved left and right (mono scenes twice).
pub fn render(scene: &Scene) -> Vec<f32> {
    let mut engine = Engine::new(scene.rate).expect("a usable rate");
    for index in 0..parameters::COUNT {
        if let Some(value) = scene.parameters.get(index) {
            assert!(engine.set_parameter(index, value), "{index} = {value}");
        }
    }
    let frames = (scene.seconds * f64::from(scene.rate)).round() as usize;
    let rate = f64::from(scene.rate);
    let mut out = Vec::with_capacity(2 * frames);
    let mut next = 0;
    let mut left = vec![0.0f32; BLOCK];
    let mut right = vec![0.0f32; BLOCK];
    let mut frame = 0;
    while frame < frames {
        // Events land at the start of the block they fall in, cut so each
        // lands on its own frame.
        while next < scene.events.len() && (scene.events[next].0 * rate).round() as usize <= frame {
            apply(&mut engine, scene.events[next].1);
            next += 1;
        }
        let until = scene
            .events
            .get(next)
            .map_or(frames, |(at, _)| ((at * rate).round() as usize).min(frames));
        let length = BLOCK.min(frames - frame).min(until.max(frame + 1) - frame);
        let t = frame as f64 / rate;
        for &(start, end, from, to) in &scene.wheel_ramps {
            if (start..=end).contains(&t) {
                let along = ((t - start) / (end - start)) as f32;
                engine.wheel_wide(from + (to - from) * along);
            }
        }
        if scene.stereo {
            engine.render_stereo(&mut left[..length], &mut right[..length]);
            for (l, r) in left[..length].iter().zip(&right[..length]) {
                out.extend([*l, *r]);
            }
        } else {
            engine.render(&mut left[..length]);
            for sample in &left[..length] {
                out.extend([*sample, *sample]);
            }
        }
        frame += length;
    }
    out
}

/// FNV-1a over every sample's bits.
pub fn fingerprint(samples: &[f32]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for sample in samples {
        for byte in sample.to_bits().to_le_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    hash
}

/// How a render differs from a reference of the same scene.
pub struct Difference {
    /// Samples whose bits differ.
    pub differing: usize,
    /// The largest difference against the reference's peak, dB.
    pub peak_error_db: f64,
    /// The difference's energy against the reference's, dB.
    pub error_db: f64,
    /// The same over the last tenth of the scene: a difference that grows
    /// shows here.
    pub late_error_db: f64,
}

pub fn compare(reference: &[f32], render: &[f32]) -> Difference {
    assert_eq!(reference.len(), render.len());
    let db = |x: f64| 10.0 * x.max(1e-300).log10();
    let energy = |a: &[f32]| a.iter().map(|x| f64::from(*x).powi(2)).sum::<f64>();
    let error = |a: &[f32], b: &[f32]| {
        a.iter()
            .zip(b)
            .map(|(x, y)| (f64::from(*x) - f64::from(*y)).powi(2))
            .sum::<f64>()
    };
    let peak = reference
        .iter()
        .fold(0.0f64, |m, x| m.max(f64::from(x.abs())));
    let worst = reference.iter().zip(render).fold(0.0f64, |m, (x, y)| {
        m.max((f64::from(*x) - f64::from(*y)).abs())
    });
    let late = reference.len() - reference.len() / 10;
    Difference {
        differing: reference
            .iter()
            .zip(render)
            .filter(|(x, y)| x.to_bits() != y.to_bits())
            .count(),
        peak_error_db: 2.0 * db(worst / peak.max(1e-30)),
        error_db: db(error(reference, render) / energy(reference).max(1e-300)),
        late_error_db: db(
            error(&reference[late..], &render[late..]) / energy(&reference[late..]).max(1e-300)
        ),
    }
}

/// The same scene with A4 one ulp from where it is: every reed's frequency
/// moved by one part in 10^16. How far that moves a scene is the
/// instrument's own sensitivity, the yardstick for a change that is not
/// exact (docs/ROADMAP.md, 10, prediction 3).
pub fn nudged(scene: &Scene) -> Scene {
    let mut parameters = scene.parameters;
    let a4 = parameters.get(parameters::PITCH_A4).expect("A4");
    assert!(parameters.set(parameters::PITCH_A4, f64::from_bits(a4.to_bits() + 1)));
    Scene {
        name: scene.name.clone(),
        rate: scene.rate,
        stereo: scene.stereo,
        seconds: scene.seconds,
        parameters,
        events: scene.events.clone(),
        wheel_ramps: scene.wheel_ramps.clone(),
    }
}

/// In-place radix-2 FFT of `re` + i `im`; the length a power of two.
fn fft(re: &mut [f64], im: &mut [f64]) {
    let n = re.len();
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut length = 2;
    while length <= n {
        let angle = -2.0 * std::f64::consts::PI / length as f64;
        for start in (0..n).step_by(length) {
            for k in 0..length / 2 {
                let (s, c) = (angle * k as f64).sin_cos();
                let (a, b) = (start + k, start + k + length / 2);
                let (tr, ti) = (re[b] * c - im[b] * s, re[b] * s + im[b] * c);
                re[b] = re[a] - tr;
                im[b] = im[a] - ti;
                re[a] += tr;
                im[a] += ti;
            }
        }
        length <<= 1;
    }
}

/// Band levels, dB: a third of an octave wide from 50 Hz to 16 kHz, in
/// Hann windows of about 50 ms overlapping by half, each channel of an
/// interleaved stereo signal. Indexed [window][channel * bands + band].
pub fn band_levels(samples: &[f32], rate: f32) -> Vec<Vec<f64>> {
    let rate = f64::from(rate);
    let size = ((0.05 * rate) as usize).next_power_of_two() / 2 * 2;
    let size = if (size as f64) < 0.04 * rate {
        size * 2
    } else {
        size
    };
    let centres: Vec<f64> = (-13..=12)
        .map(|k| 1000.0 * 2f64.powf(k as f64 / 3.0))
        .collect();
    let frames = samples.len() / 2;
    let mut levels = Vec::new();
    let mut start = 0;
    while start + size <= frames {
        let mut row = Vec::with_capacity(2 * centres.len());
        for channel in 0..2 {
            let mut re: Vec<f64> = (0..size)
                .map(|n| {
                    let w = 0.5 - 0.5 * (2.0 * std::f64::consts::PI * n as f64 / size as f64).cos();
                    w * f64::from(samples[2 * (start + n) + channel])
                })
                .collect();
            let mut im = vec![0.0; size];
            fft(&mut re, &mut im);
            for centre in &centres {
                let (low, high) = (
                    centre * 2f64.powf(-1.0 / 6.0),
                    centre * 2f64.powf(1.0 / 6.0),
                );
                let (a, b) = (
                    (low / rate * size as f64).ceil() as usize,
                    ((high / rate * size as f64).floor() as usize).min(size / 2),
                );
                let energy: f64 = (a..=b.max(a)).map(|k| re[k] * re[k] + im[k] * im[k]).sum();
                row.push(10.0 * energy.max(1e-30).log10());
            }
        }
        levels.push(row);
        start += size / 2;
    }
    levels
}

/// The 99th percentile of the band-level differences between two renders of
/// a scene, dB, over the bands of the reference within 60 dB of its
/// loudest.
pub fn level_difference(reference: &[f32], render: &[f32], rate: f32) -> f64 {
    let (a, b) = (band_levels(reference, rate), band_levels(render, rate));
    let loudest = a.iter().flatten().fold(f64::NEG_INFINITY, |m, x| m.max(*x));
    let mut differences: Vec<f64> = a
        .iter()
        .flatten()
        .zip(b.iter().flatten())
        .filter(|(x, _)| **x > loudest - 60.0)
        .map(|(x, y)| (x - y).abs())
        .collect();
    if differences.is_empty() {
        return 0.0;
    }
    differences.sort_by(f64::total_cmp);
    differences[(differences.len() - 1) * 99 / 100]
}
