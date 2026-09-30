//! Milestone 2b's second experiment, run by hand: `cargo test --release -p
//! rf-musette-analysis --test sink_flow -- --ignored --nocapture`.
//!
//! Misdariis, Ricot & Caussé (CFA 2000) describe the flow upstream of an
//! accordion reed as a laminar two-dimensional sink into the gap, in planes
//! across the tongue, with pressure acting on the upstream face only. They
//! publish no equation; what follows is derived here from that description.
//! A half-plane sink of strength q per unit edge length, φ = (q/π) ln r,
//! acts on the face between the gap (r = g/2) and the tongue's half-width
//! (r = w/2), where the slot's near-field inertance takes over:
//!
//! * (A) the steady Bernoulli deficit ½ρ|∇φ|² integrates, with q = v g and
//!   ½ρv² = Δp the jet's, to (2/π²) Δp g (1 - g/w) per unit edge length --
//!   a force away from the plate that grows with the gap;
//! * (B) the unsteady term ρ ∂φ/∂t is the sink's inertance,
//!   ρ ln(w/g) / (π dx) per edge element, taken like the first
//!   experiment's passage inertia, through the kinetic energy ½ M(ζ) ũ²:
//!
//! ```text
//! M_r ζ'' + M_n S_r u'          = -M_r γ ζ' - K ζ + S_r p - F_A(ζ) Δp + ½ M' ũ²
//! M_n u' + M (u' - S_r ζ'')     = p - Δp - M' ζ' ũ
//! M_h a' = P - p,     C p' = a - u,     Δp = ½ ρ v|v|,  v = ũ/(α S_u)
//! ```
//!
//! With neither, this is exactly the shipping model. It prints; it asserts
//! nothing. What it finds decides whether either is built.

use core::f64::consts::PI;
use rf_musette_analysis::{Trace, attack_time, cents, component_envelope};
use rf_musette_dsp::parameters::Parameters;
use rf_musette_dsp::reed::{AIR_DENSITY as RHO, ReedDesign, ReedModel};
use rf_musette_dsp::tongue::{SPAN_POINTS, TongueMode};

const RATE: f64 = 16.0 * 96_000.0;
const TABLE: usize = 4096;
const LOW: f64 = -9.0e-3;
const HIGH: f64 = 12.0e-3;

/// The gap between an element of the tongue's edge and the slot, at a
/// depth `depth` of the element below the plate's upper face (negative
/// above it), as the useful section takes it.
fn gap(depth: f64, thickness: f64, plate: f64, clearance: f64) -> f64 {
    if depth < 0.0 {
        f64::sqrt(depth * depth + clearance * clearance)
    } else if depth - thickness <= plate {
        clearance
    } else {
        let beyond = depth - thickness - plate;
        f64::sqrt(beyond * beyond + clearance * clearance)
    }
}

/// Every edge element at tip deflection `y`: (gap, edge length, mode shape
/// there). Both sides and the tip.
fn edges(model: &ReedModel, y: f64, mode: &TongueMode) -> Vec<(f64, f64, f64)> {
    let d = model.design;
    let dx = d.length / (SPAN_POINTS - 1) as f64;
    let mut out = Vec::with_capacity(SPAN_POINTS + 1);
    for i in 0..SPAN_POINTS {
        let weight = if i == 0 || i == SPAN_POINTS - 1 {
            0.5
        } else {
            1.0
        };
        let thickness = model.root_thickness * mode.thickness[i];
        let g = gap(
            y * mode.shape[i],
            thickness,
            d.plate_thickness,
            d.side_clearance,
        );
        out.push((g, 2.0 * weight * dx, mode.shape[i]));
    }
    let tip_thickness = model.root_thickness * mode.thickness[SPAN_POINTS - 1];
    let slope = y * mode.tip_slope / d.length;
    let tilt = 0.5 * tip_thickness * (slope / f64::sqrt(1.0 + slope * slope)).abs();
    let drawn_back = y * y / (2.0 * d.length) * mode.slope_squared;
    let front = (d.tip_clearance + drawn_back - tilt).max(0.1 * d.tip_clearance);
    out.push((
        gap(y, tip_thickness, d.plate_thickness, front),
        d.width + 2.0 * d.side_clearance,
        1.0,
    ));
    out
}

/// (A): the modal area on which the jet's Δp is lost, m².
fn suction_area(model: &ReedModel, y: f64, mode: &TongueMode) -> f64 {
    let w = model.design.width;
    edges(model, y, mode)
        .iter()
        .map(|&(g, length, shape)| 2.0 / (PI * PI) * g * (1.0 - g / w).max(0.0) * length * shape)
        .sum()
}

/// (B): the sink's inertance, kg/m⁴, its elements in parallel.
fn sink_inertance(model: &ReedModel, y: f64, mode: &TongueMode) -> f64 {
    let w = model.design.width;
    let mut admittance = 0.0;
    for (g, length, _) in edges(model, y, mode) {
        let log = (w / g).ln();
        if log <= 0.0 {
            return 0.0;
        }
        admittance += PI * length / (RHO * log);
    }
    1.0 / admittance
}

#[derive(Clone, Copy)]
struct Variant {
    suction: bool,
    inertia: bool,
}

impl Variant {
    fn label(self) -> &'static str {
        match (self.suction, self.inertia) {
            (false, false) => "none",
            (true, false) => "A",
            (false, true) => "B",
            (true, true) => "A+B",
        }
    }
}

struct Tables {
    inertance: Vec<f64>,
    suction: Vec<f64>,
}

impl Tables {
    fn new(model: &ReedModel, design: ReedDesign, variant: Variant) -> Self {
        let mode = TongueMode::with_ratio(design.mode_ratio);
        let y = |i: usize| LOW + (HIGH - LOW) * i as f64 / (TABLE - 1) as f64;
        Self {
            inertance: (0..TABLE)
                .map(|i| {
                    if variant.inertia {
                        sink_inertance(model, y(i), &mode)
                    } else {
                        0.0
                    }
                })
                .collect(),
            suction: (0..TABLE)
                .map(|i| {
                    if variant.suction {
                        suction_area(model, y(i), &mode)
                    } else {
                        0.0
                    }
                })
                .collect(),
        }
    }

    /// (M, dM/dζ, F_A) at displacement `zeta` from rest.
    fn at(&self, zeta: f64, set: f64) -> (f64, f64, f64) {
        let step = (HIGH - LOW) / (TABLE - 1) as f64;
        let position = ((zeta - set - LOW) / step).clamp(0.0, (TABLE - 2) as f64);
        let i = position as usize;
        let f = position - i as f64;
        let (a, b) = (self.inertance[i], self.inertance[i + 1]);
        let (c, e) = (self.suction[i], self.suction[i + 1]);
        (a + (b - a) * f, (b - a) / step, c + (e - c) * f)
    }
}

fn simulate(
    design: ReedDesign,
    variant: Variant,
    supply: f64,
    start: [f64; 5],
    seconds: f64,
) -> Trace {
    let model = ReedModel::new(design);
    let tables = Tables::new(&model, design, variant);
    let d = model.design;
    let (m_r, s_r, m_n) = (model.modal_mass, model.effective_area, model.inertance);
    let k = m_r * model.omega * model.omega;
    let gamma = model.omega / d.q;
    let derivative = |s: [f64; 5]| -> [f64; 5] {
        let [zeta, w, u, a, p] = s;
        let jet = u - s_r * w;
        let v = jet / (d.contraction * model.section(zeta));
        let dp = 0.5 * RHO * v * v.abs();
        let (m, slope, suction) = tables.at(zeta, d.set);
        let f1 =
            -m_r * gamma * w - k * zeta + s_r * p - suction * dp.max(0.0) + 0.5 * slope * jet * jet;
        let f2 = p - dp - slope * w * jet;
        let (a11, a12, a21, a22) = (m_r, m_n * s_r, -m * s_r, m_n + m);
        let det = a11 * a22 - a12 * a21;
        [
            w,
            (f1 * a22 - a12 * f2) / det,
            (a11 * f2 - a21 * f1) / det,
            (supply - p) / model.hole_inertance,
            (a - u) / model.cell_compliance,
        ]
    };
    let h = 1.0 / RATE;
    let frames = (seconds * RATE) as usize;
    let mut s = start;
    let mut trace = Trace {
        rate: RATE,
        zeta: Vec::with_capacity(frames),
        flow_rate: Vec::with_capacity(frames),
    };
    for _ in 0..frames {
        let k1 = derivative(s);
        let k2 = derivative(core::array::from_fn(|i| s[i] + 0.5 * h * k1[i]));
        let k3 = derivative(core::array::from_fn(|i| s[i] + 0.5 * h * k2[i]));
        let k4 = derivative(core::array::from_fn(|i| s[i] + h * k3[i]));
        for i in 0..5 {
            s[i] += h / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        }
        trace.zeta.push(s[0]);
        trace.flow_rate.push(k1[3]);
    }
    trace
}

/// The equilibrium's growth rate, 1/s, under a steady supply: found by
/// letting the reed settle there first (the suction moves it), then
/// kicking it.
fn growth(design: ReedDesign, variant: Variant, supply: f64) -> f64 {
    linear(design, variant, supply).0
}

/// (growth rate 1/s, frequency Hz) of small oscillations about the
/// equilibrium.
fn linear(design: ReedDesign, variant: Variant, supply: f64) -> (f64, f64) {
    let model = ReedModel::new(design);
    // Settle with damping raised so it cannot oscillate, then kick.
    let mut still = design;
    still.q = 0.5;
    let settled = simulate(still, variant, supply, [0.0; 5], 0.05);
    let zeta = *settled.zeta.last().unwrap();
    let flow = design.contraction * model.section(zeta) * (2.0 * supply / RHO).sqrt();
    let trace = simulate(
        design,
        variant,
        supply,
        [zeta + 1.0e-6, 0.0, flow, flow, supply],
        0.25,
    );
    let period = (RATE / design.frequency) as usize;
    let peak = |from: usize| {
        trace.zeta[from..from + 5 * period]
            .iter()
            .map(|z| (z - zeta).abs())
            .fold(0.0, f64::max)
    };
    let (first, second) = (peak(10 * period), peak(40 * period));
    // Upward crossings of the equilibrium, from the 10th period to the 40th.
    let crossings: Vec<f64> = (10 * period..40 * period)
        .filter(|&i| trace.zeta[i - 1] < zeta && trace.zeta[i] >= zeta)
        .map(|i| {
            let (a, b) = (trace.zeta[i - 1] - zeta, trace.zeta[i] - zeta);
            (i as f64 - 1.0 + a / (a - b)) / RATE
        })
        .collect();
    let frequency = (crossings.len() - 1) as f64 / (crossings[crossings.len() - 1] - crossings[0]);
    (
        (second / first).ln() / (30.0 * period as f64 / RATE),
        frequency,
    )
}

fn finger_attack(design: ReedDesign, variant: Variant, supply: f64) -> Option<f64> {
    let trace = simulate(design, variant, supply, [0.0; 5], 1.5);
    let tone = trace.tone(1.0, 1.5)?;
    attack_time(&component_envelope(
        &trace.flow_rate,
        RATE,
        tone.frequency,
        4.0,
        0.001,
    ))
}

#[test]
#[ignore = "experiment: prints, asserts nothing"]
fn the_sink_flow_against_none() {
    let base = Parameters::default().reed_design();
    let model = ReedModel::new(base);
    let mode = TongueMode::with_ratio(base.mode_ratio);
    println!(
        "S_r {:.2} mm², M_n {:.1} kg/m^4, K_tip {:.0} N/m",
        model.effective_area * 1e6,
        model.inertance,
        model.tip_stiffness()
    );
    println!("across the swing (y from flat, negative above the plate):");
    for y_mm in [-3.0, -1.0, -0.5, -0.2, 0.0, 1.0, 3.0, 4.0] {
        let y = y_mm * 1e-3;
        println!(
            "  y {y_mm:>5} mm: F_A {:>6.3} mm² ({:>5.2} % of S_r), sink {:>6.1} kg/m^4",
            suction_area(&model, y, &mode) * 1e6,
            100.0 * suction_area(&model, y, &mode) / model.effective_area,
            sink_inertance(&model, y, &mode)
        );
    }
    for variant in [
        Variant {
            suction: false,
            inertia: false,
        },
        Variant {
            suction: true,
            inertia: false,
        },
        Variant {
            suction: false,
            inertia: true,
        },
        Variant {
            suction: true,
            inertia: true,
        },
    ] {
        let sigma = growth(base, variant, 400.0);
        let small: Vec<f64> = [100.0, 300.0, 900.0]
            .iter()
            .map(|p| linear(base, variant, *p).1)
            .collect();
        println!(
            "{:>5}: small oscillations {:+.2} / {:+.2} / {:+.2} cents at 100/300/900 Pa, sag {:+.2}",
            variant.label(),
            cents(base.frequency, small[0]),
            cents(base.frequency, small[1]),
            cents(base.frequency, small[2]),
            cents(small[0], small[2])
        );
        let attacks: Vec<Option<f64>> = [100.0, 400.0]
            .iter()
            .map(|p| finger_attack(base, variant, *p).map(|a| a * 1e3))
            .collect();
        let tones: Vec<_> = [100.0, 300.0, 900.0]
            .iter()
            .map(|p| simulate(base, variant, *p, [0.0; 5], 2.0).tone(1.5, 2.0))
            .collect();
        let pitch = |t: &Option<rf_musette_analysis::Tone>| {
            t.as_ref()
                .map_or(f64::NAN, |t| cents(base.frequency, t.frequency))
        };
        let sag = match (&tones[0], &tones[2]) {
            (Some(a), Some(b)) => cents(a.frequency, b.frequency),
            _ => f64::NAN,
        };
        println!(
            "{:>5}: σ(400) {sigma:>6.1} /s (e-fold {:>5.1} ms) | attack {:>5.0?} / {:>5.0?} ms | pitch {:+.1} / {:+.1} / {:+.1} cents at 100/300/900 Pa, sag {sag:+.1} | swing(300) {:.2} mm",
            variant.label(),
            1e3 / sigma,
            attacks[0],
            attacks[1],
            pitch(&tones[0]),
            pitch(&tones[1]),
            pitch(&tones[2]),
            tones[1].as_ref().map_or(0.0, |t| t.amplitude * 1e3)
        );
    }
}
