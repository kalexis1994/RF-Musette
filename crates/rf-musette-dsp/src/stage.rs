//! The microphones and the room (milestone 9b): the instrument as it is
//! recorded, from where it sounds to where each microphone is.
//!
//! The instrument is two sides in space: the treble's reed blocks, which run
//! the keyboard's length, as four sources -- its quarters, the low notes at
//! the top -- and the bass box as one, which moves away from the treble side
//! as the bellows opens. Each radiates the monopole the engine computes, p =
//! ρ/(4π r) u′, with a broad first-order directivity toward where its grille
//! faces (assumed: no accordion's directivity is measured; a free reed is a
//! monopole, Nussbaumer & Agarwal, ICA 2016).
//!
//! A capsule hears each source along its own path: a fractional delay of
//! r/c and a gain of 1/r, its pattern (1 − b) + b cos θ toward the source,
//! and the source's directivity toward it. A source or capsule that moves
//! changes its path as it moves, and Doppler is that and nothing more.
//!
//! The room, after Concert Grand's (RackForge, `plugins/concert-grand`): a
//! box of the chosen volume, Sabine's reverberation time per band from the
//! walls' absorption, first-order images of the instrument read by every
//! capsule, and a feedback delay network for the tail, its level set by
//! Sabine's reverberant field against the direct sound at 1 m, 16π/A.
//!
//! Every position and shape here is assumed (docs/MODEL.md); the layouts
//! follow engineers' practice (docs/SOURCES.md, "The microphones").

use crate::math;
use crate::parameters::{self, Parameters};
use crate::reed::SPEED_OF_SOUND;

/// The treble's quarters, then the bass box.
pub const TREBLE_SOURCES: usize = 4;
pub const BASS_SOURCE: usize = 4;
pub const SOURCES: usize = 5;

/// The most capsules a layout uses: six over the treble's reed blocks and
/// two in the bass box.
pub const MAX_CAPSULES: usize = 8;

/// The direct paths' lines, samples: 3.6 m at 192 kHz, the farthest a
/// stand is put.
const LINE: usize = 2048;
/// The room's images: 50 m of path at 48 kHz.
const EARLY_LINE: usize = 8192;
const IMAGES: usize = 6;
const LATE_LINES: usize = 8;
const LATE_LENGTH: usize = 8192;
/// The tail's input, alternating in sign: the uniform vector is the
/// Householder mixer's own and would keep the lines in step.
const LATE_SIGNS: [f64; LATE_LINES] = [1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
/// The allpasses' buffers: inside each line, and before the network.
/// Inside each line, up to 85 ms at 48 kHz; before the network, 21 ms.
const LOOP_DIFFUSER: usize = 4096;
const INPUT_DIFFUSER: usize = 1024;
const DIFFUSERS_IN: usize = 4;
/// Inside each line, its allpass's delay, s, and gain: the echoes multiply
/// at every pass (milestone 9d).
const LOOP_ALLPASS: [f64; LATE_LINES] = [
    0.0031, 0.0037, 0.0041, 0.0047, 0.0053, 0.0059, 0.0067, 0.0073,
];
const LOOP_ALLPASS_GAIN: f64 = 0.6;
/// Each loop allpass's delay wanders this far either side, s, at its own
/// slow rate, Hz: not the room's physics but a reverberator's means (Dattorro
/// 1997; Lexicon's) of letting a few modes stand for a room's many --
/// fixed, they ring; moving, they blur. At most ~1 cent of pitch.
const WANDER_DEPTH: f64 = 0.000_3;
const WANDER_RATE: [f64; LATE_LINES] = [0.31, 0.37, 0.43, 0.53, 0.61, 0.71, 0.79, 0.89];
/// Before the network, four in a row, so the field is diffuse from its
/// first milliseconds.
const INPUT_ALLPASS: [f64; DIFFUSERS_IN] = [0.0011, 0.0017, 0.0027, 0.0039];
const INPUT_ALLPASS_GAIN: f64 = 0.7;

// Where things are, m: x toward the player's right, y forward from the
// player's chest, z up from the floor. A seated player; assumed.
/// The treble's reed blocks: a line at this x and y, from its top to its
/// bottom.
const TREBLE_X: f64 = 0.12;
const FACE_Y: f64 = 0.12;
const TREBLE_TOP: f64 = 1.18;
const TREBLE_BOTTOM: f64 = 0.88;
/// The bass box with the bellows shut, and the instrument's middle height.
const BASS_X: f64 = -0.12;
const CENTRE_Z: f64 = 1.03;
/// How far each side's radiation leans toward where its grille faces.
pub const SOURCE_DIRECTIVITY: f64 = 0.3;
/// The closest a capsule is taken to be to a source, m.
const NEAREST: f64 = 0.02;

/// The ORTF pair's spacing and its capsules' angle either side of the
/// instrument (the ORTF standard: 17 cm, ±55°).
const ORTF_SPACING: f64 = 0.17;
const ORTF_ANGLE: f64 = 55.0 * core::f64::consts::PI / 180.0;

/// The room: a box of proportions length : width : height, the player a
/// third of the way down its length and in its middle. Assumed.
const ROOM_SHAPE: [f64; 3] = [1.25, 1.6, 1.0];
const PLAYER_ALONG: f64 = 0.3;
/// The late tail's lines, s: 11 to 43 ms, and with their allpasses the
/// network holds some 0.5 s -- a mode in every ~2 Hz, overlapping in a
/// room's decay (milestone 9d; Concert Grand's six lines of 0.62-1.43 mean
/// free paths, 60 ms at 150 m³, rang as a tube). The lines are short and the
/// allpasses long, so the tail begins early and dense. The room's size is in
/// its first reflections and Sabine's decay, not here. Each is rounded to a
/// prime number of samples, so no two share a divisor.
const LATE_SPREAD: [f64; LATE_LINES] = [0.007, 0.011, 0.017, 0.023, 0.083, 0.107, 0.131, 0.163];
/// Air's absorption in the high band, per metre (ISO 9613-1, order of
/// magnitude at 4 kHz, as Concert Grand takes it).
const AIR_HIGH: f64 = 0.0022;
const AIR_MID: f64 = 0.0002;
/// The bands' reference frequencies, Hz: the low shelf's and the loop
/// low-pass's.
const LOW_BAND: f64 = 150.0;
const HIGH_BAND: f64 = 4000.0;

/// [`parameters::MIC_LAYOUT`]'s values.
pub const INTERNAL: usize = 0;
pub const CLIP_ON: usize = 1;
pub const SPOTS: usize = 2;
pub const ORTF: usize = 3;
pub const SPACED: usize = 4;
pub const SINGLE: usize = 5;
pub const DRY: usize = 6;

pub type Point = [f64; 3];

fn sub(a: Point, b: Point) -> Point {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn add(a: Point, b: Point) -> Point {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn scale(a: Point, s: f64) -> Point {
    [a[0] * s, a[1] * s, a[2] * s]
}

fn dot(a: Point, b: Point) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn length(a: Point) -> f64 {
    math::sqrt(dot(a, a))
}

fn unit(a: Point) -> Point {
    let l = length(a);
    if l > 0.0 {
        scale(a, 1.0 / l)
    } else {
        [0.0, 1.0, 0.0]
    }
}

/// Where a treble quarter's reeds sound, the low notes' at the top.
pub fn treble_position(quarter: usize) -> Point {
    let share = (quarter as f64 + 0.5) / TREBLE_SOURCES as f64;
    [
        TREBLE_X,
        FACE_Y,
        TREBLE_TOP - share * (TREBLE_TOP - TREBLE_BOTTOM),
    ]
}

/// Where the bass box sounds, the bellows opened `extension` metres.
pub fn bass_position(extension: f64) -> Point {
    [BASS_X - extension.max(0.0), FACE_Y, CENTRE_Z]
}

/// Which way each side's grille faces: the treble's forward, the bass box's
/// out to the left and forward.
fn source_axis(source: usize) -> Point {
    if source == BASS_SOURCE {
        unit([-1.0, 1.0, 0.0])
    } else {
        [0.0, 1.0, 0.0]
    }
}

fn source_position(source: usize, extension: f64) -> Point {
    if source == BASS_SOURCE {
        bass_position(extension)
    } else {
        treble_position(source)
    }
}

/// The instrument's middle, for the room's images.
fn instrument_centre() -> Point {
    [0.0, FACE_Y, CENTRE_Z]
}

/// A capsule's pattern toward a direction, from omni (0) through cardioid
/// (0.5) to figure-of-eight (1); its sign kept, as a figure-of-eight
/// inverts what reaches it from behind.
pub fn polar(pattern: f64, aim: Point, toward: Point) -> f64 {
    (1.0 - pattern) + pattern * dot(aim, unit(toward))
}

/// A capsule's share of a diffuse field's energy: (1 − b)² + b²/3.
pub fn diffuse(pattern: f64) -> f64 {
    (1.0 - pattern) * (1.0 - pattern) + pattern * pattern / 3.0
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Capsule {
    /// Where it is, with the bellows shut if it rides on the bass box.
    position: Point,
    aim: Point,
    pattern: f64,
    /// Mounted on the bass box: it moves with it.
    on_bass: bool,
    gain: f64,
    /// −1 left to +1 right, the player's right taken as right.
    pan: f64,
}

impl Capsule {
    fn at(position: Point, aim: Point, pattern: f64, pan: f64) -> Self {
        Self {
            position,
            aim: unit(aim),
            pattern: pattern.clamp(0.0, 1.0),
            on_bass: false,
            gain: 1.0,
            pan,
        }
    }

    fn position_at(&self, extension: f64) -> Point {
        if self.on_bass {
            add(self.position, [-extension, 0.0, 0.0])
        } else {
            self.position
        }
    }

    /// The direct path from a source: its delay, s, and its gain.
    fn hears(&self, source: usize, from: Point, at: Point) -> (f64, f64) {
        let path = sub(from, at);
        let r = length(path).max(NEAREST);
        let toward_source = path;
        let toward_capsule = scale(path, -1.0);
        let gain = polar(self.pattern, self.aim, toward_source)
            * ((1.0 - SOURCE_DIRECTIVITY)
                + SOURCE_DIRECTIVITY * dot(source_axis(source), unit(toward_capsule)))
            / r;
        (r / SPEED_OF_SOUND, gain)
    }
}

/// A layout's capsules and its high-pass.
#[derive(Debug, Clone, Copy, Default)]
struct Layout {
    capsules: [Capsule; MAX_CAPSULES],
    count: usize,
    highpass: f64,
}

impl Layout {
    fn push(&mut self, capsule: Capsule) {
        if self.count < MAX_CAPSULES {
            self.capsules[self.count] = capsule;
            self.count += 1;
        }
    }

    fn capsules(&self) -> &[Capsule] {
        &self.capsules[..self.count]
    }
}

fn decibels(db: f64) -> f64 {
    math::pow(10.0, db / 20.0)
}

/// The ORTF pair at `distance` in front of the instrument and `height` up,
/// its capsules ±55° from the instrument's middle.
fn ortf_pair(layout: &mut Layout, distance: f64, height: f64, pattern: f64, gain: f64) {
    let centre = [0.0, FACE_Y + distance, height];
    let toward = unit(sub(instrument_centre(), centre));
    let (sin, cos) = math::sin_cos(ORTF_ANGLE);
    for side in [-1.0, 1.0] {
        // Turned about the vertical, toward its own side.
        let aim = [
            toward[0] * cos - toward[1] * sin * side,
            toward[0] * sin * side + toward[1] * cos,
            toward[2],
        ];
        let mut capsule = Capsule::at(
            add(centre, [side * ORTF_SPACING / 2.0, 0.0, 0.0]),
            aim,
            pattern,
            side,
        );
        capsule.gain = gain;
        layout.push(capsule);
    }
}

/// The capsules of the chosen layout, from the parameters.
fn layout(parameters: &Parameters, travel: f64) -> Layout {
    let value = |index: usize| parameters.get(index).unwrap_or(0.0);
    let mut layout = Layout::default();
    let bass_rest = bass_position(0.0);
    let bass_axis = source_axis(BASS_SOURCE);
    match value(parameters::MIC_LAYOUT) as usize {
        INTERNAL => {
            // A bar of capsules over the treble's reed blocks, 3 cm off
            // them, from end to end; one or two in the bass box, 4 cm off
            // its reeds. Omni (assumed).
            let treble = value(parameters::INTERNAL_TREBLE).clamp(2.0, 6.0) as usize;
            let bass = value(parameters::INTERNAL_BASS).clamp(1.0, 2.0) as usize;
            let balance = value(parameters::INTERNAL_BALANCE);
            for n in 0..treble {
                let share = n as f64 / (treble - 1) as f64;
                let z = TREBLE_TOP - 0.02 - share * (TREBLE_TOP - TREBLE_BOTTOM - 0.04);
                let mut capsule =
                    Capsule::at([TREBLE_X, FACE_Y + 0.03, z], [0.0, -1.0, 0.0], 0.0, 1.0);
                capsule.gain = decibels(balance / 2.0);
                layout.push(capsule);
            }
            for n in 0..bass {
                let lift = if bass == 2 {
                    if n == 0 { 0.06 } else { -0.06 }
                } else {
                    0.0
                };
                let mut capsule = Capsule::at(
                    add(add(bass_rest, scale(bass_axis, 0.04)), [0.0, 0.0, lift]),
                    scale(bass_axis, -1.0),
                    0.0,
                    -1.0,
                );
                capsule.on_bass = true;
                capsule.gain = decibels(-balance / 2.0);
                layout.push(capsule);
            }
            layout.highpass = value(parameters::INTERNAL_HIGHPASS);
        }
        CLIP_ON => {
            // Two goosenecks over the treble grille, one off the bass side
            // half as far again (K&K: 2 in and 3 in).
            let distance = value(parameters::CLIP_DISTANCE) * 0.01;
            let spacing = value(parameters::CLIP_SPACING) * 0.01;
            let pattern = value(parameters::CLIP_PATTERN);
            let balance = value(parameters::CLIP_BALANCE);
            for side in [-0.5, 0.5] {
                let mut capsule = Capsule::at(
                    [TREBLE_X, FACE_Y + distance, CENTRE_Z + side * spacing],
                    [0.0, -1.0, 0.0],
                    pattern,
                    1.0,
                );
                capsule.gain = decibels(balance / 2.0);
                layout.push(capsule);
            }
            let mut capsule = Capsule::at(
                add(bass_rest, scale(bass_axis, 1.5 * distance)),
                scale(bass_axis, -1.0),
                pattern,
                -1.0,
            );
            capsule.on_bass = true;
            capsule.gain = decibels(-balance / 2.0);
            layout.push(capsule);
            layout.highpass = value(parameters::CLIP_HIGHPASS);
        }
        SPOTS => {
            // One stand on each side: the treble's facing its grille, the
            // bass's at the middle of the arc the bass box travels (Piovesan).
            let pattern = value(parameters::SPOTS_PATTERN);
            let treble = value(parameters::SPOTS_TREBLE_DISTANCE) * 0.01;
            let bass = value(parameters::SPOTS_BASS_DISTANCE) * 0.01;
            layout.push(Capsule::at(
                [TREBLE_X, FACE_Y + treble, CENTRE_Z],
                [0.0, -1.0, 0.0],
                pattern,
                1.0,
            ));
            let middle = bass_position(travel / 2.0);
            layout.push(Capsule::at(
                add(middle, scale(bass_axis, bass)),
                scale(bass_axis, -1.0),
                pattern,
                -1.0,
            ));
            let ambience = value(parameters::SPOTS_AMBIENCE);
            if ambience > 0.0 {
                ortf_pair(
                    &mut layout,
                    value(parameters::ORTF_DISTANCE),
                    value(parameters::ORTF_HEIGHT),
                    value(parameters::ORTF_PATTERN),
                    ambience,
                );
            }
        }
        ORTF => ortf_pair(
            &mut layout,
            value(parameters::ORTF_DISTANCE),
            value(parameters::ORTF_HEIGHT),
            value(parameters::ORTF_PATTERN),
            1.0,
        ),
        SPACED => {
            let distance = value(parameters::SPACED_DISTANCE);
            let spacing = value(parameters::SPACED_SPACING);
            let pattern = value(parameters::SPACED_PATTERN);
            for side in [-1.0, 1.0] {
                let position = [side * spacing / 2.0, FACE_Y + distance, CENTRE_Z];
                layout.push(Capsule::at(
                    position,
                    sub(instrument_centre(), position),
                    pattern,
                    side,
                ));
            }
        }
        SINGLE => {
            let distance = value(parameters::SINGLE_DISTANCE);
            layout.push(Capsule::at(
                [0.0, FACE_Y + distance, CENTRE_Z],
                [0.0, -1.0, 0.0],
                value(parameters::SINGLE_PATTERN),
                0.0,
            ));
        }
        _ => {}
    }
    layout
}

/// The smallest prime at or above `n`.
fn prime_at_least(n: usize) -> usize {
    let mut candidate = n.max(2);
    loop {
        if (2..)
            .take_while(|d| d * d <= candidate)
            .all(|d| !candidate.is_multiple_of(d))
        {
            return candidate;
        }
        candidate += 1;
    }
}

/// A Schroeder allpass: w = x + g w(n − M), y = w(n − M) − g w.
#[derive(Debug, Clone, Copy)]
struct Allpass<const N: usize> {
    buffer: [f32; N],
    length: usize,
    position: usize,
    gain: f64,
    /// A slow wander of the delay, samples either side, and its phase as a
    /// turning unit vector, and its turn per sample.
    depth: f64,
    phase: (f64, f64),
    step: (f64, f64),
}

impl<const N: usize> Allpass<N> {
    const fn new() -> Self {
        Self {
            buffer: [0.0; N],
            length: 1,
            position: 0,
            gain: 0.0,
            depth: 0.0,
            phase: (1.0, 0.0),
            step: (1.0, 0.0),
        }
    }

    /// The delay wanders `depth` seconds either side at `hertz`, from
    /// `phase` radians.
    fn wander(&mut self, depth: f64, hertz: f64, phase: f64, rate: f64) {
        self.depth = (depth * rate)
            .min((self.length as f64 - 2.0).max(0.0))
            .min((N - 2 - self.length) as f64);
        let (sin, cos) = math::sin_cos(2.0 * core::f64::consts::PI * hertz / rate);
        self.step = (cos, sin);
        let (sin, cos) = math::sin_cos(phase);
        self.phase = (cos, sin);
    }

    fn tune(&mut self, seconds: f64, rate: f64, gain: f64) {
        self.length = prime_at_least((seconds * rate) as usize).clamp(1, N - 1);
        self.gain = gain;
    }

    fn reset(&mut self) {
        self.buffer = [0.0; N];
    }

    fn process(&mut self, x: f64) -> f64 {
        let delayed = if self.depth > 0.0 {
            let (c, s) = self.phase;
            let (dc, ds) = self.step;
            let turned = (c * dc - s * ds, c * ds + s * dc);
            // Kept on the unit circle as it turns.
            let size = 1.5 - 0.5 * (turned.0 * turned.0 + turned.1 * turned.1);
            self.phase = (turned.0 * size, turned.1 * size);
            // w(n − d), read between two samples: the wander is slow and
            // small, and the tail's highs are damped anyway.
            let delay = self.length as f64 + self.depth * self.phase.1;
            let whole = delay as usize;
            let f = delay - whole as f64;
            let a = f64::from(self.buffer[(self.position + N - whole) % N]);
            let b = f64::from(self.buffer[(self.position + N - whole - 1) % N]);
            a + (b - a) * f
        } else {
            f64::from(self.buffer[(self.position + N - self.length) % N])
        };
        let w = x + self.gain * delayed;
        self.buffer[self.position] = w as f32;
        self.position = (self.position + 1) % N;
        delayed - self.gain * w
    }
}

/// A one-pole low-pass y = (1 − a) x + a y₁, unity at DC, whose gain at the
/// frequency whose cosine (of ω) is `cos` is `gain`: solved on
/// |H|² = (1 − a)²/(1 − 2a cos ω + a²).
fn one_pole_for(gain: f64, cos: f64) -> f64 {
    let target = gain * gain;
    let qa = 1.0 - target;
    let qb = -2.0 + 2.0 * target * cos;
    let qc = 1.0 - target;
    if qa.abs() < 1e-12 {
        0.0
    } else {
        ((-qb - math::sqrt((qb * qb - 4.0 * qa * qc).max(0.0))) / (2.0 * qa)).clamp(0.0, 0.99)
    }
}

/// A delay line read between its samples: four-point Lagrange.
fn read(line: &[f32], written: usize, delay: f64) -> f64 {
    let mask = line.len() - 1;
    let delay = delay.clamp(1.0, (line.len() - 4) as f64);
    let whole = delay as usize;
    let f = delay - whole as f64;
    let at = |back: usize| f64::from(line[written.wrapping_sub(back) & mask]);
    let (ym1, y0, y1, y2) = (at(whole - 1), at(whole), at(whole + 1), at(whole + 2));
    // Lagrange through (−1, ym1), (0, y0), (1, y1), (2, y2) at f.
    let c0 = -f * (f - 1.0) * (f - 2.0) / 6.0;
    let c1 = (f + 1.0) * (f - 1.0) * (f - 2.0) / 2.0;
    let c2 = -(f + 1.0) * f * (f - 2.0) / 2.0;
    let c3 = (f + 1.0) * f * (f - 1.0) / 6.0;
    c0 * ym1 + c1 * y0 + c2 * y1 + c3 * y2
}

/// Sabine's room for a volume, m³, and the walls' hardness, 0-1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Room {
    /// Width (x), length (y), height (z), m.
    pub size: Point,
    /// Where the player sits, m from the room's corner.
    pub player: Point,
    /// The walls' absorption, low, middle and high bands.
    pub absorption: [f64; 3],
    /// Sabine's reverberation time per band, s.
    pub reverberation: [f64; 3],
    /// The absorption area in the middle band, m².
    pub area: f64,
    pub surface: f64,
    pub volume: f64,
}

impl Room {
    /// Concert Grand's absorption from the hardness (voiced there): α_mid =
    /// 0.5 e^(−2.6 h) + 0.035, the highs more absorbed in a soft room and the
    /// lows less in a hard one.
    pub fn new(volume: f64, hardness: f64) -> Self {
        let volume = volume.max(1.0);
        let h = hardness.clamp(0.0, 1.0);
        let k = math::pow(
            volume / (ROOM_SHAPE[0] * ROOM_SHAPE[1] * ROOM_SHAPE[2]),
            1.0 / 3.0,
        );
        let size = [ROOM_SHAPE[0] * k, ROOM_SHAPE[1] * k, ROOM_SHAPE[2] * k];
        let surface = 2.0 * (size[0] * size[1] + size[0] * size[2] + size[1] * size[2]);
        let mid = 0.5 * math::exp(-2.6 * h) + 0.035;
        let absorption = [
            mid * (0.55 + 0.65 * h),
            mid,
            (mid * (1.0 + 1.3 * (1.0 - h))).min(0.99),
        ];
        let air = [0.0, AIR_MID, AIR_HIGH];
        let mut reverberation = [0.0; 3];
        for band in 0..3 {
            reverberation[band] = (0.161 * volume
                / (surface * absorption[band] + 4.0 * air[band] * volume))
                .clamp(0.05, 12.0);
        }
        Self {
            size,
            player: [size[0] / 2.0, PLAYER_ALONG * size[1], 0.0],
            absorption,
            reverberation,
            area: surface * absorption[1],
            surface,
            volume,
        }
    }

    /// Where a point given about the player is in the room, kept 10 cm off
    /// its walls.
    fn place(&self, point: Point) -> Point {
        let mut at = add(self.player, point);
        for (value, size) in at.iter_mut().zip(self.size) {
            *value = value.clamp(0.1, size - 0.1);
        }
        at
    }

    /// The instrument's six first-order images, in the room.
    fn images(&self) -> [Point; IMAGES] {
        let s = self.place(instrument_centre());
        let [w, l, h] = self.size;
        [
            [-s[0], s[1], s[2]],
            [2.0 * w - s[0], s[1], s[2]],
            [s[0], -s[1], s[2]],
            [s[0], 2.0 * l - s[1], s[2]],
            [s[0], s[1], -s[2]],
            [s[0], s[1], 2.0 * h - s[2]],
        ]
    }

    /// The critical distance, m: where the reverberant field is as loud as
    /// an omni source's direct sound, √(A/16π).
    pub fn critical_distance(&self) -> f64 {
        math::sqrt(self.area / (16.0 * core::f64::consts::PI))
    }
}

/// One capsule's fixed paths: from each treble quarter (and the bass box,
/// when the capsule rides on it), and from the room's images.
#[derive(Debug, Clone, Copy, Default)]
struct Paths {
    direct: [(f64, f64); SOURCES],
    early: [(usize, f64); IMAGES],
    left: f64,
    right: f64,
}

/// The stage: the layout's capsules, the room, and their delay lines.
pub struct Stage {
    rate: f64,
    layout: Layout,
    paths: [Paths; MAX_CAPSULES],
    /// The engineer's preamp: the layout's loudness brought to the dry
    /// instrument's at 1 m.
    trim: f64,
    /// The room's level, linear.
    room_level: f64,
    lines: [[f32; LINE]; SOURCES],
    early_line: [f32; EARLY_LINE],
    written: usize,
    late: [[f32; LATE_LENGTH]; LATE_LINES],
    late_length: [usize; LATE_LINES],
    late_position: usize,
    late_gain: [f64; LATE_LINES],
    late_input: f64,
    late_left: f64,
    late_right: f64,
    /// The loop's low-pass and low shelf, per line, each for its own
    /// length; its allpass; the allpasses before it.
    damp: [f64; LATE_LINES],
    damp_state: [f64; LATE_LINES],
    low_coefficient: f64,
    low_gain: [f64; LATE_LINES],
    /// The shelf's low-pass, second order: a first order's skirt reached the
    /// middle band and lengthened its decay (milestone 9d).
    low_state: [[f64; 2]; LATE_LINES],
    loop_allpass: [Allpass<LOOP_DIFFUSER>; LATE_LINES],
    input_allpass: [Allpass<INPUT_DIFFUSER>; DIFFUSERS_IN],
    /// The first reflections' walls: what the high band loses against the
    /// middle, a one-pole per capsule.
    early_damp: f64,
    early_state: [f64; MAX_CAPSULES],
    /// The output's high-pass, per channel.
    highpass: f64,
    highpass_state: [(f64, f64); 2],
    active: bool,
    /// Samples with nothing coming in, and how many it takes for the room to
    /// fall silent: then the stage computes nothing until the instrument
    /// sounds again.
    quiet: usize,
    ringing: usize,
}

impl Stage {
    pub fn new(rate: f64) -> Self {
        Self {
            rate,
            layout: Layout::default(),
            paths: [Paths::default(); MAX_CAPSULES],
            trim: 1.0,
            room_level: 1.0,
            lines: [[0.0; LINE]; SOURCES],
            early_line: [0.0; EARLY_LINE],
            written: 0,
            late: [[0.0; LATE_LENGTH]; LATE_LINES],
            late_length: [1; LATE_LINES],
            late_position: 0,
            late_gain: [0.0; LATE_LINES],
            late_input: 0.0,
            late_left: 0.0,
            late_right: 0.0,
            damp: [0.0; LATE_LINES],
            damp_state: [0.0; LATE_LINES],
            low_coefficient: 0.0,
            low_gain: [0.0; LATE_LINES],
            low_state: [[0.0; 2]; LATE_LINES],
            loop_allpass: [Allpass::new(); LATE_LINES],
            input_allpass: [Allpass::new(); DIFFUSERS_IN],
            early_damp: 0.0,
            early_state: [0.0; MAX_CAPSULES],
            highpass: 0.0,
            highpass_state: [(0.0, 0.0); 2],
            active: false,
            quiet: usize::MAX,
            ringing: 0,
        }
    }

    /// The layout's trim, for measuring: the engineer's preamp.
    pub fn trim(&self) -> f64 {
        self.trim
    }

    /// Whether a layout is chosen: Dry has no stage.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Silences the lines and the tail.
    pub fn reset(&mut self) {
        self.lines = [[0.0; LINE]; SOURCES];
        self.early_line = [0.0; EARLY_LINE];
        self.late = [[0.0; LATE_LENGTH]; LATE_LINES];
        self.damp_state = [0.0; LATE_LINES];
        self.low_state = [[0.0; 2]; LATE_LINES];
        self.early_state = [0.0; MAX_CAPSULES];
        for allpass in self.loop_allpass.iter_mut() {
            allpass.reset();
        }
        for allpass in self.input_allpass.iter_mut() {
            allpass.reset();
        }
        self.highpass_state = [(0.0, 0.0); 2];
        self.quiet = usize::MAX;
    }

    /// Sets the stage from the parameters: the layout, the room, and where
    /// the bass box rests for what does not follow it (`travel`, m, the
    /// bellows' whole opening).
    pub fn tune(&mut self, parameters: &Parameters, travel: f64) {
        let value = |index: usize| parameters.get(index).unwrap_or(0.0);
        self.active = (value(parameters::MIC_LAYOUT) as usize) < DRY;
        self.layout = layout(parameters, travel);
        let room = Room::new(
            value(parameters::ROOM_SIZE),
            value(parameters::ROOM_HARDNESS),
        );
        self.room_level = decibels(value(parameters::ROOM_LEVEL));
        let width = value(parameters::STEREO_WIDTH).clamp(0.0, 1.0);
        // The audience's perspective mirrors the player's.
        let side = if value(parameters::PERSPECTIVE) == parameters::AUDIENCE {
            -1.0
        } else {
            1.0
        };
        let images = room.images();
        let reflect = math::sqrt(1.0 - room.absorption[1]);
        let rest = travel / 2.0;
        let mut energy = 0.0;
        let mut late_left = 0.0;
        let mut late_right = 0.0;
        let mut room_left = 0.0;
        let mut room_right = 0.0;
        let mut loudness = [[0.0f64; 2]; SOURCES];
        for (capsule, paths) in self.layout.capsules().iter().zip(self.paths.iter_mut()) {
            let angle = (side * capsule.pan * width + 1.0) * core::f64::consts::FRAC_PI_4;
            let (right, left) = math::sin_cos(angle);
            paths.left = left;
            paths.right = right;
            let at = capsule.position_at(rest);
            for (source, (direct, heard)) in
                paths.direct.iter_mut().zip(loudness.iter_mut()).enumerate()
            {
                let (delay, gain) = capsule.hears(source, source_position(source, rest), at);
                *direct = (delay * self.rate, gain * capsule.gain);
                heard[0] += gain * capsule.gain * left;
                heard[1] += gain * capsule.gain * right;
            }
            let placed = room.place(at);
            for (image, early) in images.iter().zip(paths.early.iter_mut()) {
                let path = sub(*image, placed);
                let r = length(path).max(NEAREST);
                *early = (
                    (r / SPEED_OF_SOUND * self.rate) as usize,
                    reflect
                        * math::sqrt(diffuse(SOURCE_DIRECTIVITY))
                        * polar(capsule.pattern, capsule.aim, path)
                        / r
                        * capsule.gain,
                );
            }
            // Sabine's reverberant field, 16π/A of a source's energy at 1 m --
            // its power, the forward lean taken out -- as this capsule takes
            // a diffuse field. Its early images are
            // part of it, and the tail brings the rest.
            let reverberant = 16.0 * core::f64::consts::PI / room.area
                * diffuse(SOURCE_DIRECTIVITY)
                * diffuse(capsule.pattern)
                * capsule.gain
                * capsule.gain;
            let early: f64 = paths.early.iter().map(|(_, gain)| gain * gain).sum();
            let tail = (reverberant - early).max(0.1 * reverberant);
            late_left += tail * left * left;
            late_right += tail * right * right;
            room_left += (early + tail) * left * left;
            room_right += (early + tail) * right * right;
        }
        for source in loudness {
            energy += (source[0] * source[0] + source[1] * source[1]) / 2.0;
        }
        energy /= SOURCES as f64;
        // Every source reaches the room: its share as loud as it is set.
        energy += self.room_level * self.room_level * (room_left + room_right) / 2.0;
        self.trim = if energy > 0.0 {
            1.0 / math::sqrt(energy)
        } else {
            1.0
        };
        self.tune_late(&room);
        // Silent once the longest band's tail is 90 dB down and every line
        // has emptied.
        let longest = room.reverberation.iter().copied().fold(0.0, f64::max);
        self.ringing = ((1.5 * longest + 0.2) * self.rate) as usize + EARLY_LINE;
        // The tail heard by every capsule, uncorrelated between them.
        self.late_left = math::sqrt(late_left);
        self.late_right = math::sqrt(late_right);
        self.highpass = self.layout.highpass;
    }

    fn tune_late(&mut self, room: &Room) {
        let mut energy = 0.0;
        let (_, cos) = math::sin_cos(2.0 * core::f64::consts::PI * HIGH_BAND / self.rate);
        for line in 0..LATE_LINES {
            let samples =
                prime_at_least((LATE_SPREAD[line] * self.rate) as usize).clamp(8, LATE_LENGTH - 1);
            self.late_length[line] = samples;
            self.loop_allpass[line].tune(LOOP_ALLPASS[line], self.rate, LOOP_ALLPASS_GAIN);
            self.loop_allpass[line].wander(
                WANDER_DEPTH,
                WANDER_RATE[line],
                line as f64 * 0.785,
                self.rate,
            );
            // The gain per pass over the whole loop, the line as it is,
            // clamped or not, and its allpass: on average an allpass delays
            // by its length.
            let seconds = (samples + self.loop_allpass[line].length) as f64 / self.rate;
            let gain = math::pow(10.0, -3.0 * seconds / room.reverberation[1]);
            self.late_gain[line] = gain;
            energy += gain * gain;
            // The loop low-pass: what the high band loses more per pass than
            // the middle. A one-pole y = (1 − a) x + a y₁ whose gain at the
            // high band is that: solved on |H|² = (1 − a)²/(1 − 2a cos ω + a²).
            let high = math::pow(
                10.0,
                -3.0 * seconds * (1.0 / room.reverberation[2] - 1.0 / room.reverberation[1]),
            )
            .clamp(0.05, 1.0);
            self.damp[line] = one_pole_for(high, cos);
            // The low shelf: what the low band keeps more per pass.
            let low = math::pow(
                10.0,
                -3.0 * seconds * (1.0 / room.reverberation[0] - 1.0 / room.reverberation[1]),
            );
            self.low_gain[line] = (low - 1.0).clamp(-0.6, 0.35);
        }
        energy /= LATE_LINES as f64;
        // Unit input in every line, four lines to each output: the impulse
        // response's energy is about 4 b²/(1 − ḡ²) (an orthogonal mixer
        // and allpasses lose nothing). The input is set so it is 1.
        self.late_input = math::sqrt((1.0 - energy) / 4.0);
        self.low_coefficient = math::exp(-2.0 * core::f64::consts::PI * LOW_BAND / self.rate);
        for (allpass, seconds) in self.input_allpass.iter_mut().zip(INPUT_ALLPASS) {
            allpass.tune(seconds, self.rate, INPUT_ALLPASS_GAIN);
        }
        // The first reflections: the walls' high band against their middle,
        // in pressure.
        let walls = math::sqrt((1.0 - room.absorption[2]) / (1.0 - room.absorption[1]));
        self.early_damp = one_pole_for(walls.clamp(0.05, 1.0), cos);
    }

    /// One sample: what each source sends (Pa at 1 m), the bellows open
    /// `extension` m; the left and right channels.
    pub fn process(&mut self, sources: &[f64; SOURCES], extension: f64) -> (f64, f64) {
        if sources.iter().all(|source| *source == 0.0) {
            self.quiet = self.quiet.saturating_add(1);
            if self.quiet > self.ringing {
                return (0.0, 0.0);
            }
        } else {
            self.quiet = 0;
        }
        self.written = self.written.wrapping_add(1);
        let mut sum = 0.0;
        for (line, value) in self.lines.iter_mut().zip(sources) {
            line[self.written & (LINE - 1)] = *value as f32;
            sum += value;
        }
        self.early_line[self.written & (EARLY_LINE - 1)] = sum as f32;
        let bass = bass_position(extension);
        let mut left = 0.0;
        let mut right = 0.0;
        for ((capsule, paths), early_state) in self
            .layout
            .capsules()
            .iter()
            .zip(self.paths.iter())
            .zip(self.early_state.iter_mut())
        {
            let mut heard = 0.0;
            for source in 0..TREBLE_SOURCES {
                let (delay, gain) = paths.direct[source];
                heard += gain * read(&self.lines[source], self.written, delay);
            }
            // The bass box: fixed to a capsule on it, moving for the rest.
            let (delay, gain) = if capsule.on_bass {
                paths.direct[BASS_SOURCE]
            } else {
                let (delay, gain) = capsule.hears(BASS_SOURCE, bass, capsule.position);
                (delay * self.rate, gain * capsule.gain)
            };
            heard += gain * read(&self.lines[BASS_SOURCE], self.written, delay);
            let mut early = 0.0;
            for (delay, gain) in paths.early {
                if delay < EARLY_LINE {
                    early += gain
                        * f64::from(
                            self.early_line[self.written.wrapping_sub(delay) & (EARLY_LINE - 1)],
                        );
                }
            }
            // The walls take more of the high band than of the middle.
            *early_state = (1.0 - self.early_damp) * early + self.early_damp * *early_state;
            heard += *early_state * self.room_level;
            left += heard * paths.left;
            right += heard * paths.right;
        }
        let (late_left, late_right) = self.late(sum);
        left = self.trim * (left + late_left * self.late_left * self.room_level);
        right = self.trim * (right + late_right * self.late_right * self.room_level);
        if self.highpass > 0.0 {
            let a = 1.0 / (1.0 + 2.0 * core::f64::consts::PI * self.highpass / self.rate);
            for (value, (input, output)) in [&mut left, &mut right]
                .into_iter()
                .zip(self.highpass_state.iter_mut())
            {
                let y = a * (*output + *value - *input);
                *input = *value;
                *output = y;
                *value = y;
            }
        }
        (left, right)
    }

    /// The tail: an eight-line feedback delay network, a Householder mixer,
    /// each line with an allpass, damped above and lifted below; four
    /// allpasses before it.
    fn late(&mut self, input: f64) -> (f64, f64) {
        let mut diffused = input;
        for allpass in self.input_allpass.iter_mut() {
            diffused = allpass.process(diffused);
        }
        let mut out = [0.0; LATE_LINES];
        for (line, value) in out.iter_mut().enumerate() {
            let at =
                (self.late_position + LATE_LENGTH - self.late_length[line]) & (LATE_LENGTH - 1);
            *value = f64::from(self.late[line][at]);
        }
        let mean = out.iter().sum::<f64>() * 2.0 / LATE_LINES as f64;
        for (line, value) in out.iter().enumerate() {
            let mut fed = (value - mean) * self.late_gain[line];
            fed = self.loop_allpass[line].process(fed);
            self.damp_state[line] =
                (1.0 - self.damp[line]) * fed + self.damp[line] * self.damp_state[line];
            fed = self.damp_state[line];
            let [first, second] = &mut self.low_state[line];
            *first = (1.0 - self.low_coefficient) * fed + self.low_coefficient * *first;
            *second = (1.0 - self.low_coefficient) * *first + self.low_coefficient * *second;
            fed += self.low_gain[line] * *second;
            self.late[line][self.late_position] =
                (fed + diffused * self.late_input * LATE_SIGNS[line]) as f32;
        }
        self.late_position = (self.late_position + 1) & (LATE_LENGTH - 1);
        // Each side two short lines and two long: the long hold less, losing
        // more at each pass, and a side of either alone would lean.
        (
            out[0] - out[1] + out[4] - out[5],
            out[2] - out[3] + out[6] - out[7],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    /// The tail's own energy: white noise in, each output's power out, in
    /// units of the input's -- 1 by its calibration.
    #[test]
    fn the_tail_gives_back_the_energy_it_is_given() {
        for volume in [1500.0, 150.0, 60.0] {
            let mut stage = Stage::new(48_000.0);
            stage.tune_late(&Room::new(volume, 0.4));
            // The formula is for the lossless mixer and the lines' gains:
            // the bands' filters are measured apart.
            stage.damp = [0.0; LATE_LINES];
            stage.low_gain = [0.0; LATE_LINES];
            let mut seed = 1u64;
            let mut input = 0.0;
            let mut output = [0.0; 2];
            let count = 1_000_000;
            // In the middle band, where Sabine is reckoned: the wandering
            // reads lose a little of the very top.
            let mut bands = [Band::new(700.0); 3];
            for n in 0..count {
                seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
                let x = ((seed >> 11) as f64 / (1u64 << 53) as f64) * 2.0 - 1.0;
                let (l, r) = stage.late(x);
                let (x, l, r) = (bands[0].pass(x), bands[1].pass(l), bands[2].pass(r));
                if n > 480_000 {
                    input += x * x;
                    output[0] += l * l;
                    output[1] += r * r;
                }
            }
            // Within 1 dB, with its measured calibration.
            // Within 1.5 dB, measured at the steady state: the lines are
            // not wholly independent (-0.1 to +1.4 dB, milestone 9d).
            for side in output {
                let db = 10.0 * libm_log10(side / input);
                assert!(db.abs() < 1.5, "{volume} m³: {db:+.2} dB");
            }
        }
    }

    /// RBJ's band-pass, Q 1.4, at 48 kHz.
    #[derive(Clone, Copy)]
    struct Band {
        b0: f64,
        a1: f64,
        a2: f64,
        x: [f64; 2],
        y: [f64; 2],
    }

    impl Band {
        fn new(centre: f64) -> Self {
            let (sin, cos) = math::sin_cos(2.0 * core::f64::consts::PI * centre / 48_000.0);
            let alpha = sin / 2.8;
            let a0 = 1.0 + alpha;
            Self {
                b0: alpha / a0,
                a1: -2.0 * cos / a0,
                a2: (1.0 - alpha) / a0,
                x: [0.0; 2],
                y: [0.0; 2],
            }
        }

        fn pass(&mut self, x: f64) -> f64 {
            let y = self.b0 * (x - self.x[1]) - self.a1 * self.y[0] - self.a2 * self.y[1];
            self.x = [x, self.x[0]];
            self.y = [y, self.y[0]];
            y
        }
    }

    fn libm_log10(x: f64) -> f64 {
        math::ln(x) / core::f64::consts::LN_10
    }

    /// The tail's energy by band, its filters on, for diagnosis.
    #[test]
    #[ignore]
    fn diagnose_the_tail_by_band() {
        let band = |signal: &[f64], centre: f64| {
            let w = 2.0 * core::f64::consts::PI * centre / 48_000.0;
            let (sin, cos) = math::sin_cos(w);
            let alpha = sin / 2.8;
            let a0 = 1.0 + alpha;
            let (b0, b2, a1, a2) = (alpha / a0, -alpha / a0, -2.0 * cos / a0, (1.0 - alpha) / a0);
            let (mut x1, mut x2, mut y1, mut y2) = (0.0, 0.0, 0.0, 0.0);
            let mut energy = 0.0;
            for (n, x) in signal.iter().enumerate() {
                let y = b0 * x + b2 * x2 - a1 * y1 - a2 * y2;
                x2 = x1;
                x1 = *x;
                y2 = y1;
                y1 = y;
                if n > 48_000 {
                    energy += y * y;
                }
            }
            energy
        };
        for filters in [false, true] {
            let mut stage = Stage::new(48_000.0);
            stage.tune_late(&Room::new(150.0, 0.4));
            if !filters {
                stage.damp = [0.0; LATE_LINES];
                stage.low_gain = [0.0; LATE_LINES];
            }
            let mut seed = 1u64;
            let mut input = std::vec::Vec::new();
            let mut left = std::vec::Vec::new();
            for _ in 0..200_000 {
                seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
                let x = ((seed >> 11) as f64 / (1u64 << 53) as f64) * 2.0 - 1.0;
                input.push(x);
                left.push(stage.late(x).0);
            }
            for centre in [100.0, 300.0, 700.0, 2000.0, 6000.0] {
                std::println!(
                    "filters {filters}: {centre} Hz {:.3}",
                    band(&left, centre) / band(&input, centre)
                );
            }
            std::println!("damp {:?} low {:?}", stage.damp, stage.low_gain);
        }
    }

    #[test]
    fn the_lagrange_read_is_exact_on_samples_and_on_lines() {
        let mut line = [0.0f32; 64];
        for (n, value) in line.iter_mut().enumerate() {
            *value = n as f32;
        }
        // Written up to 40: a delay of d reads 40 − d.
        assert!((read(&line, 40, 3.0) - 37.0).abs() < 1e-9);
        assert!((read(&line, 40, 3.25) - 36.75).abs() < 1e-9);
    }

    #[test]
    fn sabine_gives_a_longer_tail_to_a_larger_harder_room() {
        let small = Room::new(60.0, 0.3);
        let large = Room::new(1500.0, 0.3);
        let hard = Room::new(60.0, 0.9);
        assert!(large.reverberation[1] > small.reverberation[1]);
        assert!(hard.reverberation[1] > small.reverberation[1]);
        let k = 0.161 * small.volume
            / (small.surface * small.absorption[1] + 4.0 * AIR_MID * small.volume);
        assert!((small.reverberation[1] - k).abs() < 1e-9);
    }

    #[test]
    fn the_bass_box_moves_left_as_the_bellows_opens() {
        assert_eq!(bass_position(0.1)[0], BASS_X - 0.1);
        assert!(
            treble_position(0)[2] > treble_position(3)[2],
            "low notes at the top"
        );
    }

    #[test]
    fn patterns_take_their_share_of_a_diffuse_field() {
        assert_eq!(diffuse(0.0), 1.0);
        assert!((diffuse(0.5) - 1.0 / 3.0).abs() < 1e-12);
        assert!((diffuse(1.0) - 1.0 / 3.0).abs() < 1e-12);
        assert!((polar(0.5, [0.0, 1.0, 0.0], [0.0, -1.0, 0.0])).abs() < 1e-12);
    }
}
