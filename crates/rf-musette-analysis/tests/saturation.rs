//! Why the swing does not saturate, measured: `cargo test --release -p
//! rf-musette-analysis --test saturation -- --ignored --nocapture`.
//!
//! The tongue is moved by hand, ζ = ζ̄ + A sin ωt, and the air is left to
//! respond: the near field, the hole and the cell integrated with RK4 on
//! the shipping model's equations. In that model the air acts on the
//! tongue through the jet's Bernoulli pressure alone, F = S_r Δp, so the
//! work it does per cycle, W = ∮ S_r Δp ζ' dt, against what the tongue's
//! own damping takes, π M_r (ω0/Q) ω A², says at which swing a free reed
//! settles (where they are equal) and why it does not settle sooner. The
//! work is split by where the tip is: above the plate, inside the slot,
//! beyond it.

#![allow(clippy::needless_range_loop)]

use core::f64::consts::PI;
use rf_musette_dsp::parameters::Parameters;
use rf_musette_dsp::reed::{AIR_DENSITY as RHO, ReedDesign, ReedModel};
use rf_musette_dsp::tongue::{SPAN_POINTS, TongueMode};

const RATE: f64 = 16.0 * 96_000.0;

struct Work {
    above: f64,
    inside: f64,
    beyond: f64,
    dissipated: f64,
    /// What the air's drag on the tongue takes, where it is out of the slot.
    drag: f64,
}

impl Work {
    fn total(&self) -> f64 {
        self.above + self.inside + self.beyond
    }
}

/// The air's work on a tongue swung at `amplitude` about `mean` (m from
/// rest) at `frequency`, per cycle, J, once the air has settled.
fn work(design: ReedDesign, supply: f64, mean: f64, amplitude: f64, frequency: f64) -> Work {
    let model = ReedModel::new(design);
    let d = model.design;
    let s_r = model.effective_area;
    let omega = 2.0 * PI * frequency;
    let tongue = |t: f64| {
        let (s, c) = (omega * t).sin_cos();
        (mean + amplitude * s, amplitude * omega * c)
    };
    let jet_pressure = |t: f64, u: f64| {
        let (zeta, w) = tongue(t);
        let v = (u - s_r * w) / (d.contraction * model.section(zeta));
        0.5 * RHO * v * v.abs()
    };
    // State: flow onto the reed u, hole flow a, cell pressure p.
    let derivative = |t: f64, s: [f64; 3]| -> [f64; 3] {
        let [u, a, p] = s;
        [
            (p - jet_pressure(t, u)) / model.inertance,
            (supply - p) / model.hole_inertance,
            (a - u) / model.cell_compliance,
        ]
    };
    let h = 1.0 / RATE;
    let period = RATE / frequency;
    let settle = (40.0 * period) as usize;
    let measure = (10.0 * period).round() as usize;
    let flow = d.contraction * model.section(mean) * (2.0 * supply / RHO).sqrt();
    let mut s = [flow, flow, supply];
    let mut out = Work {
        above: 0.0,
        inside: 0.0,
        beyond: 0.0,
        dissipated: 0.0,
        drag: 0.0,
    };
    let mode = TongueMode::with_ratio(d.mode_ratio);
    let dx = d.length / (SPAN_POINTS - 1) as f64;
    let drag_coefficient: Vec<f64> = (0..SPAN_POINTS)
        .map(|i| {
            let weight = if i == 0 || i == SPAN_POINTS - 1 {
                0.5
            } else {
                1.0
            };
            let kc = 2.0 * PI * amplitude * mode.shape[i] / d.width;
            0.5 * RHO * plate_drag(kc) * d.width * mode.shape[i].powi(3) * weight * dx
        })
        .collect();
    let plate_and_tongue = d.plate_thickness + model.root_thickness;
    for n in 0..settle + measure {
        let t = n as f64 * h;
        if n >= settle {
            let (zeta, w) = tongue(t);
            let power = s_r * jet_pressure(t, s[0]) * w * h;
            // Where the tip is, from flat: negative above the plate.
            let y = zeta - d.set;
            for i in 0..SPAN_POINTS {
                let depth = y * mode.shape[i];
                let thickness = model.root_thickness * mode.thickness[i];
                if !(0.0..=d.plate_thickness + thickness).contains(&depth) {
                    out.drag += drag_coefficient[i] * w.abs() * w * w * h;
                }
            }
            if y < 0.0 {
                out.above += power;
            } else if y <= plate_and_tongue {
                out.inside += power;
            } else {
                out.beyond += power;
            }
        }
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
        for i in 0..3 {
            s[i] += h / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        }
    }
    let cycles = measure as f64 / period;
    out.above /= cycles;
    out.inside /= cycles;
    out.beyond /= cycles;
    out.drag /= cycles;
    out.dissipated = PI * model.modal_mass * (model.omega / d.q) * omega * amplitude * amplitude;
    out
}

/// An oscillating flat plate's drag coefficient against the
/// Keulegan–Carpenter number, measured by Keulegan & Carpenter (J. Res.
/// NBS 60, 1958, Table 4, plates in water at U_m D/ν 4.5e3-1.4e4; a
/// selection of their runs), interpolated in log KC and held at the ends.
fn plate_drag(kc: f64) -> f64 {
    const TABLE: [(f64, f64); 12] = [
        (1.7, 11.55),
        (2.2, 10.21),
        (3.5, 8.81),
        (5.2, 7.06),
        (7.5, 5.15),
        (7.7, 5.48),
        (8.8, 5.21),
        (13.6, 4.28),
        (16.3, 4.11),
        (29.9, 3.36),
        (62.2, 2.43),
        (118.2, 1.81),
    ];
    if kc <= TABLE[0].0 {
        return TABLE[0].1;
    }
    for pair in TABLE.windows(2) {
        let ((k0, c0), (k1, c1)) = (pair[0], pair[1]);
        if kc <= k1 {
            return c0 + (c1 - c0) * (kc / k0).ln() / (k1 / k0).ln();
        }
    }
    TABLE[TABLE.len() - 1].1
}

#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn where_the_air_puts_its_energy() {
    let design = Parameters::default().reed_design();
    let model = ReedModel::new(design);
    // The model plays 6-7 cents below its mode.
    let frequency = design.frequency * 2f64.powf(-6.5 / 1200.0);
    println!(
        "tip {:.2} mm above the plate at rest, plate {:.1} mm, root {:.2} mm thick",
        design.set * 1e3,
        design.plate_thickness * 1e3,
        model.root_thickness * 1e3
    );
    println!("air's work over the tongue's damping, per cycle (1 = where a free reed settles):");
    for supply in [100.0, 300.0, 900.0, 3000.0] {
        let mean = model.mu * supply / (model.omega * model.omega);
        println!("{supply} Pa:");
        for a_mm in [0.25, 0.5, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 12.0] {
            let w = work(design, supply, mean, a_mm * 1e-3, frequency);
            println!(
                "  A {a_mm:>5} mm: ratio {:>6.3}, with drag {:>6.3} | above {:>+8.2} inside {:>+8.2} beyond {:>+8.2} µJ | damping {:>7.2}, drag {:>7.2} µJ",
                w.total() / w.dissipated,
                w.total() / (w.dissipated + w.drag),
                w.above * 1e6,
                w.inside * 1e6,
                w.beyond * 1e6,
                w.dissipated * 1e6,
                w.drag * 1e6
            );
        }
    }
}

/// The fundamental's level against the mean flow, the way Nussbaumer &
/// Agarwal measured real reeds (ICA 2016, Fig. 6: reed 2's fundamental rose
/// 10.5 dB as the flow went from 20 to 52 l/min, ~7.6 dB per doubling; a
/// swing that held constant would give 6). The shipping step, from rest.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn level_against_flow() {
    use rf_musette_analysis::Trace;
    use rf_musette_dsp::reed::{self, ReedState};
    let design = Parameters::default().reed_design();
    let model = ReedModel::new(design);
    let rate = 192_000.0;
    let h = 1.0 / rate;
    let mut previous: Option<(f64, f64)> = None;
    println!("supply | mean flow | swing, mean | fundamental | 2nd, 3rd | dB per doubling of flow");
    for supply in [
        40.0, 60.0, 100.0, 150.0, 200.0, 300.0, 450.0, 600.0, 900.0, 1500.0, 3000.0,
    ] {
        let mut state = ReedState::default();
        let (settle, measure) = ((1.5 * rate) as usize, (0.5 * rate) as usize);
        let mut trace = Trace {
            rate,
            zeta: Vec::new(),
            flow_rate: Vec::new(),
        };
        let mut flow = 0.0;
        for n in 0..settle + measure {
            let rate_of_flow = reed::step(&model, &mut state, supply, f64::INFINITY, h);
            if n >= settle {
                trace.zeta.push(state.zeta);
                trace.flow_rate.push(rate_of_flow);
                flow += state.hole_flow;
            }
        }
        let flow = flow / measure as f64;
        let Some(tone) = trace.tone(0.0, 0.5) else {
            println!("{supply:>6} Pa: silent");
            continue;
        };
        let omega = 2.0 * PI * tone.frequency / rate;
        let (mut re, mut im, mut norm) = (0.0, 0.0, 0.0);
        for (i, x) in trace.flow_rate.iter().enumerate() {
            let w = 0.5 - 0.5 * (2.0 * PI * i as f64 / measure as f64).cos();
            re += w * x * (omega * i as f64).cos();
            im -= w * x * (omega * i as f64).sin();
            norm += w;
        }
        let fundamental = 20.0 * (2.0 * (re * re + im * im).sqrt() / norm).log10();
        let harmonics = Trace::harmonics(&trace.flow_rate, rate, tone.frequency, 3);
        let slope = previous.map_or(String::new(), |(f0, l0)| {
            format!("{:+.1}", (fundamental - l0) / (flow / f0).log2())
        });
        println!(
            "{supply:>6} Pa | {:>5.1} l/min | {:>5.2} mm, {:+.2} mm | {fundamental:>6.1} dB | {:>+6.1}, {:>+6.1} dB | {slope}",
            flow * 6.0e4,
            tone.amplitude * 1e3,
            tone.mean * 1e3,
            harmonics[1],
            harmonics[2]
        );
        previous = Some((flow, fundamental));
    }
}

/// Would a mean that moves toward the plate pin the swing? The balance of
/// `where_the_air_puts_its_energy` with the mean set by hand: where it
/// crosses 1, per supply and mean, without drag.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn a_mean_moved_toward_the_plate() {
    let design = Parameters::default().reed_design();
    let frequency = design.frequency * 2f64.powf(-6.5 / 1200.0);
    let amplitudes: Vec<f64> = (1..=48).map(|i| 0.25e-3 * i as f64).collect();
    println!("swing where the balance crosses 1, mm (mean in mm toward the plate from rest):");
    for supply in [100.0, 300.0, 900.0, 3000.0] {
        let row: Vec<String> = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.7]
            .iter()
            .map(|mean_mm| {
                let mut previous: Option<(f64, f64)> = None;
                for &a in &amplitudes {
                    let w = work(design, supply, mean_mm * 1e-3, a, frequency);
                    let ratio = w.total() / w.dissipated;
                    if let Some((a0, r0)) = previous {
                        if r0 >= 1.0 && ratio < 1.0 {
                            let at = a0 + (a - a0) * (r0 - 1.0) / (r0 - ratio);
                            return format!("{mean_mm}: {:>5.2}", at * 1e3);
                        }
                    } else if ratio < 1.0 {
                        return format!("{mean_mm}: silent");
                    }
                    previous = Some((a, ratio));
                }
                format!("{mean_mm}: >12")
            })
            .collect();
        println!("{supply:>6} Pa | {}", row.join(" | "));
    }
}
