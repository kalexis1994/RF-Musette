//! Milestone 2b's experiment, run by hand: `cargo test --release -p
//! rf-musette-analysis --test passage_inertia -- --ignored --nocapture`.
//!
//! The inertia of the air in the escape passages, M_loc(ζ), added to the
//! reed through its kinetic energy ½ M_loc(ζ) ũ² (ũ the flow through the
//! passages, the jet), integrated with RK4 on the exact equations:
//!
//! ```text
//! M_r ζ'' + M_n S_r u'              = -M_r γ ζ' - K ζ + S_r p + ½ M_loc' ũ²
//! M_n u' + M_loc (u' - S_r ζ'')     = p - Δp - M_loc' ζ' ũ
//! M_h a' = P - p,     C p' = a - u,     Δp = ½ ρ v|v|,  v = ũ/(α S_u)
//! ```
//!
//! With M_loc = 0 this is exactly the shipping model. It prints; it asserts
//! nothing. What it finds decides whether the mechanism is built.

use rf_musette_analysis::{Trace, attack_time, component_envelope};
use rf_musette_dsp::parameters::{Parameters, SIDE_CLEARANCE};
use rf_musette_dsp::reed::AIR_DENSITY as RHO;
use rf_musette_dsp::reed::{AIR_DENSITY, ReedDesign, ReedModel};
use rf_musette_dsp::tongue::{SPAN_POINTS, TongueMode};

/// The inertance of the air in the escape passages themselves, kg/m⁴, at
/// a tip deflection `y` from flat: milestone 2b's mechanism, not yet in
/// the shipping step (tested and not kept; see the file's header).
///
/// Each element of the tongue's edge (and its tip) passes air through a
/// narrow channel between the edge and the slot's wall, of width the
/// local gap g and length ℓ: how much of the tongue's thickness is
/// inside the plate there (zero while the edge is above or beyond it).
/// Its inertance is Tarnopolsky, Fletcher & Lai's ρ d/(C F) (JASA 108,
/// 2000, eq. 3) taken element by element, ρ (ℓ + κ g)/(α g dx), with an
/// end correction κ g of the order of the gap; the elements are in
/// parallel. Ricot, Caussé & Misdariis locate the accordion reed's
/// excitation in this mass (as summarised in Llanos-Vázquez's thesis,
/// Appendix 1, eq. A1.8).
pub fn local_inertance_at(model: &ReedModel, y: f64, mode: &TongueMode, kappa: f64) -> f64 {
    let d = model.design;
    let plate = d.plate_thickness;
    // (gap, channel length) for one element.
    let element = |depth: f64, thickness: f64, clearance: f64| -> (f64, f64) {
        if depth < 0.0 {
            (f64::sqrt(depth * depth + clearance * clearance), 0.0)
        } else if depth - thickness <= plate {
            let overlap = (depth.min(plate) - (depth - thickness).max(0.0)).max(0.0);
            (clearance, overlap)
        } else {
            let beyond = depth - thickness - plate;
            (f64::sqrt(beyond * beyond + clearance * clearance), 0.0)
        }
    };
    let admittance = |gap: f64, length: f64, width: f64| {
        d.contraction * gap * width / (RHO * (length + kappa * gap))
    };
    let dx = d.length / (SPAN_POINTS - 1) as f64;
    let mut total = 0.0;
    for i in 0..SPAN_POINTS {
        let weight = if i == 0 || i == SPAN_POINTS - 1 {
            0.5
        } else {
            1.0
        };
        let thickness = model.root_thickness * mode.thickness[i];
        let (gap, length) = element(y * mode.shape[i], thickness, d.side_clearance);
        total += 2.0 * admittance(gap, length, weight * dx);
    }
    let tip_thickness = model.root_thickness * mode.thickness[SPAN_POINTS - 1];
    let slope = y * mode.tip_slope / d.length;
    let tilt = 0.5 * tip_thickness * (slope / f64::sqrt(1.0 + slope * slope)).abs();
    let drawn_back = y * y / (2.0 * d.length) * mode.slope_squared;
    let front_clearance = (d.tip_clearance + drawn_back - tilt).max(0.1 * d.tip_clearance);
    let (gap, length) = element(y, tip_thickness, front_clearance);
    total += admittance(gap, length, d.width + 2.0 * d.side_clearance);
    1.0 / total
}

const RATE: f64 = 16.0 * 96_000.0;
const TABLE: usize = 4096;
const LOW: f64 = -9.0e-3;
const HIGH: f64 = 12.0e-3;

struct Local {
    values: Vec<f64>,
}

impl Local {
    fn new(model: &ReedModel, design: ReedDesign, kappa: Option<f64>) -> Self {
        let mode = TongueMode::with_ratio(design.mode_ratio);
        let values = (0..TABLE)
            .map(|i| {
                let y = LOW + (HIGH - LOW) * i as f64 / (TABLE - 1) as f64;
                kappa.map_or(0.0, |k| local_inertance_at(model, y, &mode, k))
            })
            .collect();
        Self { values }
    }

    /// M_loc and dM_loc/dζ at displacement `zeta` from rest.
    fn at(&self, zeta: f64, set: f64) -> (f64, f64) {
        let y = zeta - set;
        let step = (HIGH - LOW) / (TABLE - 1) as f64;
        let position = ((y - LOW) / step).clamp(0.0, (TABLE - 2) as f64);
        let i = position as usize;
        let f = position - i as f64;
        let (a, b) = (self.values[i], self.values[i + 1]);
        (a + (b - a) * f, (b - a) / step)
    }
}

fn simulate(
    design: ReedDesign,
    kappa: Option<f64>,
    supply: f64,
    start: [f64; 5],
    seconds: f64,
) -> Trace {
    let model = ReedModel::new(design);
    let local = Local::new(&model, design, kappa);
    let d = model.design;
    let (m_r, s_r, m_n) = (model.modal_mass, model.effective_area, model.inertance);
    let k = m_r * model.omega * model.omega;
    let gamma = model.omega / d.q;
    let derivative = |s: [f64; 5]| -> [f64; 5] {
        let [zeta, w, u, a, p] = s;
        let jet = u - s_r * w;
        let v = jet / (d.contraction * model.section(zeta));
        let dp = 0.5 * AIR_DENSITY * v * v.abs();
        let (m_loc, slope) = local.at(zeta, d.set);
        let f1 = -m_r * gamma * w - k * zeta + s_r * p + 0.5 * slope * jet * jet;
        let f2 = p - dp - slope * w * jet;
        // [m_r, m_n s_r; -m_loc s_r, m_n + m_loc] [ζ'', u'] = [f1, f2]
        let (a11, a12, a21, a22) = (m_r, m_n * s_r, -m_loc * s_r, m_n + m_loc);
        let det = a11 * a22 - a12 * a21;
        let zeta2 = (f1 * a22 - a12 * f2) / det;
        let u2 = (a11 * f2 - a21 * f1) / det;
        [
            w,
            zeta2,
            u2,
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

/// The equilibrium's growth rate, 1/s, under a steady supply.
fn growth(design: ReedDesign, kappa: Option<f64>, supply: f64) -> f64 {
    let model = ReedModel::new(design);
    let zeta = model.mu * supply / (model.omega * model.omega);
    let flow = design.contraction * model.section(zeta) * (2.0 * supply / AIR_DENSITY).sqrt();
    let trace = simulate(
        design,
        kappa,
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
    (second / first).ln() / (30.0 * period as f64 / RATE)
}

fn finger_attack(design: ReedDesign, kappa: Option<f64>, supply: f64) -> Option<f64> {
    let trace = simulate(design, kappa, supply, [0.0; 5], 1.5);
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
fn the_passage_inertia_against_none() {
    let base = Parameters::default().reed_design();
    let model = ReedModel::new(base);
    let mode = TongueMode::with_ratio(base.mode_ratio);
    println!("M_loc across the swing (κ = 1), kg/m^4:");
    for y_mm in [-3.0, -1.0, -0.5, 0.0, 0.2, 0.5, 1.0, 2.0, 3.0, 3.5, 4.0] {
        println!(
            "  y {y_mm:>5} mm: {:>8.1}",
            local_inertance_at(&model, y_mm * 1e-3, &mode, 1.0)
        );
    }
    for kappa in [None, Some(0.5), Some(1.0), Some(2.0)] {
        let label = kappa.map_or("none".to_owned(), |k| format!("κ {k}"));
        let rates: Vec<f64> = [100.0, 400.0]
            .iter()
            .map(|p| growth(base, kappa, *p))
            .collect();
        let attacks: Vec<Option<f64>> = [100.0, 400.0]
            .iter()
            .map(|p| finger_attack(base, kappa, *p))
            .collect();
        let swing = simulate(base, kappa, 300.0, [0.0; 5], 1.5).tone(1.0, 1.5);
        println!(
            "{label:>8}: σ {:>6.1} / {:>6.1} /s (e-fold {:>5.1} / {:>5.1} ms) | step attack {:>5.0?} / {:>5.0?} ms | 300 Pa: {:.2} Hz, swing {:.2} mm",
            rates[0],
            rates[1],
            1e3 / rates[0],
            1e3 / rates[1],
            attacks[0].map(|a| a * 1e3),
            attacks[1].map(|a| a * 1e3),
            swing.map_or(0.0, |t| t.frequency),
            swing.map_or(0.0, |t| t.amplitude * 1e3)
        );
    }
    println!("with the tongue resting nearer the plate (κ = 1), 400 Pa:");
    for set in [0.5, 0.2, 0.1, 0.05, 0.0] {
        let mut p = Parameters::default();
        assert!(p.set(rf_musette_dsp::parameters::REED_SET, set));
        let design = p.reed_design();
        println!(
            "  set {set} mm: σ none {:>6.1}, with {:>6.1} /s | attack none {:>5.0?}, with {:>5.0?} ms",
            growth(design, None, 400.0),
            growth(design, Some(1.0), 400.0),
            finger_attack(design, None, 400.0).map(|a| a * 1e3),
            finger_attack(design, Some(1.0), 400.0).map(|a| a * 1e3)
        );
    }
    println!("clearance sensitivity at 400 Pa (κ = 1):");
    for clearance in [0.015, 0.035, 0.06] {
        let mut p = Parameters::default();
        assert!(p.set(SIDE_CLEARANCE, clearance));
        let design = p.reed_design();
        println!(
            "  {clearance} mm: σ none {:>6.1}, with {:>6.1} /s",
            growth(design, None, 400.0),
            growth(design, Some(1.0), 400.0)
        );
    }
}
