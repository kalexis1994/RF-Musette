//! Milestone 7b's experiment, run by hand: `cargo test --release -p
//! rf-musette-analysis --test two_modes -- --ignored --nocapture`.
//!
//! The tongue with its first two bending modes, q₁ψ₁ + q₂ψ₂ (the second the
//! profiled F4's own, at the measured 1 : 4.6), against the same harness
//! with the first alone. RK4 on the shipping model's equations:
//!
//! ```text
//! M_k (q_k'' + (ω_k/Q) q_k' + ω_k² q_k) = S_k Δp,   S_k = W L ∫ψ_k
//! M_n u' = p − Δp,   M_h a' = P(t) − p,   C p' = a − u
//! Δp = ½ ρ v|v|,   v = (u − Σ S_k q_k') / (α S_u(q₁, q₂))
//! ```
//!
//! with the useful section integrated element by element from the deflected
//! shape, as the shipping table does for the first mode alone, and the
//! voiced swing limit on the first mode as shipped. The supply rises over
//! 50 ms, as the pallet opens. Prints; asserts nothing.

use rf_musette_analysis::{attack_time, cents, component_envelope};
use rf_musette_dsp::compass::design;
use rf_musette_dsp::parameters::{Parameters, RANK_MIDDLE};
use rf_musette_dsp::reed::{AIR_DENSITY as RHO, ReedDesign, ReedModel};
use rf_musette_dsp::tongue::{SPAN_POINTS, TongueMode};

const RATE: f64 = 16.0 * 96_000.0;
const OPENING: f64 = 0.05;

struct Tongue {
    model: ReedModel,
    mode: TongueMode,
    /// ψ₂'(1), from the sampled shape.
    second_tip_slope: f64,
}

impl Tongue {
    fn new(design: ReedDesign) -> Self {
        let mode = TongueMode::with_ratio(design.mode_ratio);
        let n = SPAN_POINTS - 1;
        let second_tip_slope = (mode.second_shape[n] - mode.second_shape[n - 1]) * n as f64;
        Self {
            model: ReedModel::with_mode(design, &mode),
            mode,
            second_tip_slope,
        }
    }

    /// The useful section, m², for modal amplitudes q₁ (from rest) and q₂.
    fn section(&self, q1: f64, q2: f64) -> f64 {
        let d = self.model.design;
        let gap = |depth: f64, thickness: f64, clearance: f64| -> f64 {
            if depth < 0.0 {
                (depth * depth + clearance * clearance).sqrt()
            } else if depth - thickness <= d.plate_thickness {
                clearance
            } else {
                let beyond = depth - thickness - d.plate_thickness;
                (beyond * beyond + clearance * clearance).sqrt()
            }
        };
        let y1 = q1 - d.set;
        let depth = |i: usize| y1 * self.mode.shape[i] + q2 * self.mode.second_shape[i];
        let dx = d.length / (SPAN_POINTS - 1) as f64;
        let mut sides = 0.0;
        let mut drawn_back = 0.0;
        for i in 0..SPAN_POINTS {
            let weight = if i == 0 || i == SPAN_POINTS - 1 {
                0.5
            } else {
                1.0
            };
            let thickness = self.model.root_thickness * self.mode.thickness[i];
            sides += weight * gap(depth(i), thickness, d.side_clearance) * dx;
            if i > 0 {
                let slope = (depth(i) - depth(i - 1)) / dx;
                drawn_back += 0.5 * slope * slope * dx;
            }
        }
        let tip_thickness = self.model.root_thickness * self.mode.thickness[SPAN_POINTS - 1];
        let slope = (y1 * self.mode.tip_slope + q2 * self.second_tip_slope) / d.length;
        let tilt = 0.5 * tip_thickness * (slope / (1.0 + slope * slope).sqrt()).abs();
        let front_clearance = (d.tip_clearance + drawn_back - tilt).max(0.1 * d.tip_clearance);
        let front =
            (d.width + 2.0 * d.side_clearance) * gap(y1 + q2, tip_thickness, front_clearance);
        (2.0 * sides + front).min(self.model.slot_area)
    }
}

struct Run {
    attack: Option<f64>,
    frequency: f64,
    first: f64,
    second: f64,
}

/// A finger attack at `pressure`, the tongue with one mode or two, its face
/// feeling `face` of the cell's pressure and the rest of the jet's drop.
fn run(design: ReedDesign, pressure: f64, two: bool, seconds: f64, face: f64) -> Run {
    let tongue = Tongue::new(design);
    let m = &tongue.model;
    let d = m.design;
    let (m1, s1) = (m.modal_mass, m.effective_area);
    let mass_per_integral = m1 / tongue.mode.mass_integral;
    let m2 = mass_per_integral * tongue.mode.second_mass_integral;
    let s2 = d.width * d.length * tongue.mode.second_shape_integral;
    let (w1, w2) = (m.omega, m.omega * tongue.mode.ratio);
    let (g1, g2) = (w1 / d.q, w2 / d.q);
    let derivative = |t: f64, s: [f64; 7]| -> [f64; 7] {
        let [q1, v1, q2, v2, u, a, p] = s;
        let supply = pressure * (t / OPENING).min(1.0);
        let (q2, v2) = if two { (q2, v2) } else { (0.0, 0.0) };
        let jet = u - s1 * v1 - s2 * v2;
        let v = jet / (d.contraction * tongue.section(q1, q2));
        let dp = 0.5 * RHO * v * v.abs();
        // The pressure the tongue's face feels: the jet's drop (as shipped,
        // face = 0), the cell's (face = 1), or between.
        let felt = (1.0 - face) * dp + face * p;
        // The voiced swing limit on the first mode, as the shipping step has it.
        let lift = q1 / d.width;
        let speed = (2.0 * p.max(0.0) / RHO).sqrt();
        let limit = d.swing_limit * RHO * speed * d.width * d.length * lift * lift / m1;
        [
            v1,
            -(g1 + limit) * v1 - w1 * w1 * q1 + s1 / m1 * felt,
            if two { v2 } else { 0.0 },
            if two {
                -g2 * v2 - w2 * w2 * q2 + s2 / m2 * felt
            } else {
                0.0
            },
            (p - dp) / m.inertance,
            (supply - p) / m.hole_inertance,
            (a - u) / m.cell_compliance,
        ]
    };
    let h = 1.0 / RATE;
    let frames = (seconds * RATE) as usize;
    let mut s = [0.0; 7];
    let mut flow_rate = Vec::with_capacity(frames / 16);
    let mut tip = Vec::with_capacity(frames / 16);
    let (mut first, mut second) = (0.0f64, 0.0f64);
    for n in 0..frames {
        let t = n as f64 * h;
        let k1 = derivative(t, s);
        let k2 = derivative(
            t + 0.5 * h,
            core::array::from_fn(|i| s[i] + 0.5 * h * k1[i]),
        );
        let k3 = derivative(
            t + 0.5 * h,
            core::array::from_fn(|i| s[i] + 0.5 * h * k2[i]),
        );
        let k4 = derivative(t + h, core::array::from_fn(|i| s[i] + h * k3[i]));
        for i in 0..7 {
            s[i] += h / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        }
        // Kept at 96 kHz for the analysis.
        if n % 16 == 0 {
            flow_rate.push(k1[5]);
            tip.push(s[0] + s[2]);
        }
        if t > seconds - 0.5 {
            first = first.max(s[0].abs());
            second = second.max(s[2].abs());
        }
    }
    let rate = RATE / 16.0;
    let tail = &tip[tip.len() - (0.5 * rate) as usize..];
    let mean = tail.iter().sum::<f64>() / tail.len() as f64;
    let crossings: Vec<f64> = (1..tail.len())
        .filter(|&i| tail[i - 1] < mean && tail[i] >= mean)
        .map(|i| {
            let (a, b) = (tail[i - 1] - mean, tail[i] - mean);
            i as f64 - 1.0 + a / (a - b)
        })
        .collect();
    let frequency = if crossings.len() > 2 {
        (crossings.len() - 1) as f64 * rate / (crossings[crossings.len() - 1] - crossings[0])
    } else {
        f64::NAN
    };
    let attack = frequency
        .is_finite()
        .then(|| attack_time(&component_envelope(&flow_rate, rate, frequency, 4.0, 0.001)))
        .flatten();
    Run {
        attack,
        frequency,
        first,
        second,
    }
}

#[test]
#[ignore = "experiment: prints, asserts nothing"]
fn the_second_mode_against_the_first_alone() {
    let p = Parameters::default();
    for key in [65u8, 81] {
        let reed = design(&p, key, RANK_MIDDLE).unwrap();
        for pressure in [100.0, 400.0] {
            let one = run(reed, pressure, false, 2.0, 0.0);
            let two = run(reed, pressure, true, 2.0, 0.0);
            println!(
                "key {key} at {pressure} Pa: attack one mode {:>5.0?} ms, two {:>5.0?} ms | pitch {:+.2} cents | swing q1 {:.2} mm, q2 {:.3} mm",
                one.attack.map(|a| a * 1e3),
                two.attack.map(|a| a * 1e3),
                cents(one.frequency, two.frequency),
                two.first * 1e3,
                two.second * 1e3
            );
        }
    }
}

/// Which pressure the tongue's face feels: the jet's drop, as the minimal
/// model has it, or the cell's, where the face is. One mode.
#[test]
#[ignore = "experiment: prints, asserts nothing"]
fn the_pressure_the_face_feels() {
    let p = Parameters::default();
    let reed = design(&p, 65, RANK_MIDDLE).unwrap();
    for face in [0.0, 0.25, 0.5, 1.0] {
        let quiet = run(reed, 30.0, false, 2.0, face);
        let soft = run(reed, 100.0, false, 2.0, face);
        let loud = run(reed, 400.0, false, 2.0, face);
        println!(
            "face {face}: at 30 Pa swing {:.2} mm | p attack {:>5.0?} ms, swing {:.2} mm | mf attack {:>5.0?} ms, swing {:.2} mm, pitch {:+.1} cents from face 0",
            quiet.first * 1e3,
            soft.attack.map(|a| a * 1e3),
            soft.first * 1e3,
            loud.attack.map(|a| a * 1e3),
            loud.first * 1e3,
            cents(run(reed, 400.0, false, 2.0, 0.0).frequency, loud.frequency)
        );
    }
}
