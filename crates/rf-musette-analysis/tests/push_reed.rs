//! Milestone 3's first experiment, run by hand: `cargo test --release -p
//! rf-musette-analysis --test push_reed -- --ignored --nocapture`.
//!
//! The push reed: the plate's other reed, sounding when the bellows closes.
//! Its air comes straight from the bellows, through only the slot's near
//! field, and its cell lies downstream, between the plate and the tone
//! hole. The same tongue, jet and cell as the pull reed, in the other
//! order:
//!
//! ```text
//! pull:  M_h a' = P - p,  C p' = a - u,  M_n u' = p - Δp
//! push:  M_n u' = P - p - Δp,  C p' = u - a,  M_h a' = p
//! both:  M_r ζ'' = -M_r γ ζ' - K ζ + S_r Δp,  Δp = ½ ρ v|v|,
//!        v = (u - S_r ζ') / (α S_u(ζ))
//! ```
//!
//! p is the cell's pressure (gauge), a the flow through the hole, u the
//! flow onto the reed; the pallet is wide open. Integrated with RK4; the
//! threshold is where a small disturbance of the equilibrium neither grows
//! nor decays. It prints; it asserts nothing.

use core::f64::consts::PI;
use rf_musette_dsp::parameters::Parameters;
use rf_musette_dsp::reed::{AIR_DENSITY as RHO, ReedDesign, ReedModel, SPEED_OF_SOUND};

const RATE: f64 = 16.0 * 96_000.0;

#[derive(Clone, Copy, PartialEq)]
enum Side {
    Pull,
    Push,
}

/// The growth rate, 1/s, of a small disturbance of the equilibrium under
/// a steady `supply`, for the reed on `side`.
fn growth(design: ReedDesign, side: Side, supply: f64) -> f64 {
    let model = ReedModel::new(design);
    let d = model.design;
    let (m_r, s_r, m_n) = (model.modal_mass, model.effective_area, model.inertance);
    let k = m_r * model.omega * model.omega;
    let gamma = model.omega / d.q;
    let (m_h, c) = (model.hole_inertance, model.cell_compliance);
    let derivative = |s: [f64; 5]| -> [f64; 5] {
        let [zeta, w, u, a, p] = s;
        let v = (u - s_r * w) / (d.contraction * model.section(zeta));
        let dp = 0.5 * RHO * v * v.abs();
        let tongue = (-m_r * gamma * w - k * zeta + s_r * dp) / m_r;
        match side {
            Side::Pull => [w, tongue, (p - dp) / m_n, (supply - p) / m_h, (a - u) / c],
            Side::Push => [w, tongue, (supply - p - dp) / m_n, p / m_h, (u - a) / c],
        }
    };
    // The equilibrium: the whole supply across the reed, the cell at the
    // supply (pull) or at the room (push), the flows steady and equal.
    let zeta = model.mu * supply / (model.omega * model.omega);
    let flow = d.contraction * model.section(zeta) * (2.0 * supply / RHO).sqrt();
    let cell = if side == Side::Pull { supply } else { 0.0 };
    let mut s = [zeta + 1.0e-6, 0.0, flow, flow, cell];
    let h = 1.0 / RATE;
    let period = (RATE / design.frequency) as usize;
    let mut peaks = [0.0; 2];
    for n in 0..45 * period {
        let k1 = derivative(s);
        let k2 = derivative(core::array::from_fn(|i| s[i] + 0.5 * h * k1[i]));
        let k3 = derivative(core::array::from_fn(|i| s[i] + 0.5 * h * k2[i]));
        let k4 = derivative(core::array::from_fn(|i| s[i] + h * k3[i]));
        for i in 0..5 {
            s[i] += h / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        }
        let deviation = (s[0] - zeta).abs();
        if (10 * period..15 * period).contains(&n) {
            peaks[0] = f64::max(peaks[0], deviation);
        } else if (40 * period..45 * period).contains(&n) {
            peaks[1] = f64::max(peaks[1], deviation);
        }
    }
    (peaks[1] / peaks[0]).ln() / (30.0 * period as f64 / RATE)
}

/// The lowest supply at which the reed on `side` grows from rest, Pa, by a
/// log scan then bisection, or `None` below `high`.
fn threshold(design: ReedDesign, side: Side, high: f64) -> Option<f64> {
    let mut low = 5.0;
    let mut p = low;
    while growth(design, side, p) <= 0.0 {
        low = p;
        p *= 1.25;
        if p > high {
            return None;
        }
    }
    let mut top = p;
    for _ in 0..12 {
        let mid = (low * top).sqrt();
        if growth(design, side, mid) > 0.0 {
            top = mid;
        } else {
            low = mid;
        }
    }
    Some(top)
}

/// The cell volume, m³, that puts its Helmholtz resonance with the hole at
/// `ratio` times the reed's frequency.
fn cell_for(design: ReedDesign, ratio: f64) -> f64 {
    let model = ReedModel::new(design);
    let omega = ratio * 2.0 * PI * design.frequency;
    RHO * SPEED_OF_SOUND * SPEED_OF_SOUND / (model.hole_inertance * omega * omega)
}

#[test]
#[ignore = "experiment: prints, asserts nothing"]
fn the_push_reed_against_the_pull_reed() {
    let base = Parameters::default().reed_design();
    let model = ReedModel::new(base);
    let resonance = 1.0 / (2.0 * PI * (model.hole_inertance * model.cell_compliance).sqrt());
    println!(
        "default cell {:.1} cm³, its resonance {resonance:.0} Hz ({:.2} of the reed)",
        base.cell_volume * 1e6,
        resonance / base.frequency
    );
    for side in [Side::Pull, Side::Push] {
        let name = if side == Side::Pull { "pull" } else { "push" };
        println!(
            "{name}: threshold {:?} Pa with the default cell",
            threshold(base, side, 20_000.0).map(|p| p.round())
        );
    }
    println!("against the cell's resonance (cell volume moved):");
    for ratio in [6.0, 3.0, 2.0, 1.5, 1.3, 1.1, 1.0, 0.95, 0.9, 0.8] {
        let design = ReedDesign {
            cell_volume: cell_for(base, ratio),
            ..base
        };
        println!(
            "  resonance {ratio:>4} × reed ({:>6.1} cm³): pull {:>8?} Pa, push {:>8?} Pa",
            design.cell_volume * 1e6,
            threshold(design, Side::Pull, 20_000.0).map(|p| p.round()),
            threshold(design, Side::Push, 20_000.0).map(|p| p.round())
        );
    }
}
