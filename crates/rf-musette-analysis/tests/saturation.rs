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
    /// The part of the air's work done by the cell's pressure, ∮ S_r p ζ';
    /// the rest is the near field's inertia, -∮ S_r M_n u' ζ'.
    cell: f64,
    /// What the voiced swing limit takes at κ = 1: ∮ c ζ'² dt with
    /// c = ρ v w L (ζ/w)², v = √(2p/ρ) the jet speed the cell's pressure
    /// would give. It scales with κ.
    voiced: f64,
}

impl Work {
    fn total(&self) -> f64 {
        self.above + self.inside + self.beyond
    }
}

/// How the escape area grows with the lift.
#[derive(Clone, Copy)]
enum Area {
    /// The shipping model's: the curtain, hard-capped at the slot's area.
    Shipping,
    /// The curtain and the slot in series, as valve engineering measured
    /// it (Schwerzler & Hamilton, ICEC 1972, eq. 9; Park et al., Energies
    /// 16, 2023, eq. 2): α A_eff = 1/√(1/(C_c A_curtain)² + 1/(C_p A_slot)²),
    /// with C_c the model's contraction and C_p the port's coefficient.
    Series { port: f64 },
}

/// The escape area, m², at displacement `zeta` from rest, for the model's
/// contraction α to multiply.
struct Section {
    model: ReedModel,
    curtain: Vec<f64>,
    area: Area,
}

const LIFT_LOW: f64 = -12.0e-3;
const LIFT_HIGH: f64 = 12.0e-3;
const LIFT_POINTS: usize = 4096;

impl Section {
    fn new(model: &ReedModel, area: Area) -> Self {
        let mode = TongueMode::with_ratio(model.design.mode_ratio);
        let mut uncapped = model.clone();
        uncapped.slot_area = f64::INFINITY;
        let curtain = (0..LIFT_POINTS)
            .map(|i| {
                let y = LIFT_LOW + (LIFT_HIGH - LIFT_LOW) * i as f64 / (LIFT_POINTS - 1) as f64;
                uncapped.section_at(y, &mode)
            })
            .collect();
        Self {
            model: model.clone(),
            curtain,
            area,
        }
    }

    fn at(&self, zeta: f64) -> f64 {
        match self.area {
            Area::Shipping => self.model.section(zeta),
            Area::Series { port } => {
                let y = zeta - self.model.design.set;
                let step = (LIFT_HIGH - LIFT_LOW) / (LIFT_POINTS - 1) as f64;
                let position = ((y - LIFT_LOW) / step).clamp(0.0, (LIFT_POINTS - 2) as f64);
                let i = position as usize;
                let f = position - i as f64;
                let curtain = self.curtain[i] + (self.curtain[i + 1] - self.curtain[i]) * f;
                // In the model's own terms: the port's coefficient over α.
                let slot = port / self.model.design.contraction * self.model.slot_area;
                1.0 / (1.0 / (curtain * curtain) + 1.0 / (slot * slot)).sqrt()
            }
        }
    }
}

/// The air's work on a tongue swung at `amplitude` about `mean` (m from
/// rest) at `frequency`, per cycle, J, once the air has settled.
fn work(design: ReedDesign, supply: f64, mean: f64, amplitude: f64, frequency: f64) -> Work {
    work_with(design, supply, mean, amplitude, frequency, Area::Shipping)
}

fn work_with(
    design: ReedDesign,
    supply: f64,
    mean: f64,
    amplitude: f64,
    frequency: f64,
    area: Area,
) -> Work {
    let model = ReedModel::new(design);
    let section = Section::new(&model, area);
    let d = model.design;
    let s_r = model.effective_area;
    let omega = 2.0 * PI * frequency;
    let tongue = |t: f64| {
        let (s, c) = (omega * t).sin_cos();
        (mean + amplitude * s, amplitude * omega * c)
    };
    let jet_pressure = |t: f64, u: f64| {
        let (zeta, w) = tongue(t);
        let v = (u - s_r * w) / (d.contraction * section.at(zeta));
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
    let flow = d.contraction * section.at(mean) * (2.0 * supply / RHO).sqrt();
    let mut s = [flow, flow, supply];
    let mut out = Work {
        above: 0.0,
        inside: 0.0,
        beyond: 0.0,
        dissipated: 0.0,
        drag: 0.0,
        cell: 0.0,
        voiced: 0.0,
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
            out.cell += s_r * s[2] * w * h;
            let speed = (2.0 * s[2].max(0.0) / RHO).sqrt();
            let lift = zeta / d.width;
            out.voiced += RHO * speed * d.width * d.length * lift * lift * w * w * h;
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
    out.cell /= cycles;
    out.voiced /= cycles;
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

/// The small-amplitude growth rate against pressure, the shape Cottingham,
/// Reed & Busha measured on a reed-organ C3 (Forum Acusticum 1999, Fig. 4):
/// rising from damping at the lowest pressures to a maximum near 1 kPa
/// (11.5 /s), then falling to a third of it by 3 kPa.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn growth_against_pressure() {
    let design = Parameters::default().reed_design();
    let model = ReedModel::new(design);
    let mechanical = model.omega / (2.0 * design.q);
    println!("mechanical damping {mechanical:.2} /s");
    for supply in [
        20.0, 35.0, 50.0, 100.0, 200.0, 400.0, 700.0, 1000.0, 1500.0, 2000.0, 3000.0, 4500.0,
        6000.0,
    ] {
        let sigma = rf_musette_analysis::growth_rate(design, 32.0 * 96_000.0, supply);
        println!(
            "{supply:>6} Pa: σ {sigma:>7.2} /s, (σ + δ)/δ {:>6.2}",
            (sigma + mechanical) / mechanical
        );
    }
}

/// Which term feeds the tongue: the cell's pressure or the near field's
/// inertia, against the swing.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn which_term_feeds_it() {
    let design = Parameters::default().reed_design();
    let model = ReedModel::new(design);
    let frequency = design.frequency * 2f64.powf(-6.5 / 1200.0);
    for supply in [300.0, 3000.0] {
        let mean = model.mu * supply / (model.omega * model.omega);
        println!("{supply} Pa:");
        for a_mm in [0.5, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0] {
            let w = work(design, supply, mean, a_mm * 1e-3, frequency);
            println!(
                "  A {a_mm:>4} mm: total {:>+8.2} = cell {:>+8.2} + near-field inertia {:>+8.2} µJ | damping {:>7.2} µJ",
                w.total() * 1e6,
                w.cell * 1e6,
                (w.total() - w.cell) * 1e6,
                w.dissipated * 1e6
            );
        }
    }
}

/// Where the balance crosses 1 for an escape-area law, scanning the swing
/// upward from small: the swing a free reed settles at, or `None` if it
/// does not start.
fn settled(design: ReedDesign, supply: f64, area: Area) -> Option<f64> {
    let model = ReedModel::new(design);
    let frequency = design.frequency * 2f64.powf(-6.5 / 1200.0);
    let mean = model.mu * supply / (model.omega * model.omega);
    let mut previous: Option<(f64, f64)> = None;
    for i in 1..=96 {
        let a = 0.125e-3 * i as f64;
        let w = work_with(design, supply, mean, a, frequency, area);
        let ratio = w.total() / w.dissipated;
        match previous {
            None if ratio < 1.0 => return None,
            Some((a0, r0)) if ratio < 1.0 => {
                return Some(a0 + (a - a0) * (r0 - 1.0) / (r0 - ratio));
            }
            _ => previous = Some((a, ratio)),
        }
    }
    None
}

/// Milestone 2c: the curtain and the slot in series, against the shipping
/// hard cap.
#[test]
#[ignore = "experiment: prints, asserts nothing"]
fn the_series_area_against_the_cap() {
    let design = Parameters::default().reed_design();
    let model = ReedModel::new(design);
    let shipping = Section::new(&model, Area::Shipping);
    let series = Section::new(&model, Area::Series { port: 0.707 });
    println!("escape area, mm² (lift from flat, negative above the plate):");
    for y_mm in [-0.5, -1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -8.0] {
        let zeta = y_mm * 1e-3 + design.set;
        println!(
            "  {y_mm:>5} mm: cap {:>6.1}, series {:>6.1}",
            shipping.at(zeta) * 1e6,
            series.at(zeta) * 1e6
        );
    }
    let laws = [
        ("cap", Area::Shipping),
        ("series C_p 0.707", Area::Series { port: 0.707 }),
        ("series C_p 0.85", Area::Series { port: 0.85 }),
    ];
    for (label, area) in laws {
        let swings: Vec<Option<f64>> = [100.0, 300.0, 600.0, 900.0, 1500.0, 3000.0]
            .iter()
            .map(|p| settled(design, *p, area).map(|a| a * 1e3))
            .collect();
        let ratio = match (swings[1], swings[5]) {
            (Some(a), Some(b)) => b / a,
            _ => f64::NAN,
        };
        println!(
            "{label:>17}: swing at 0.1/0.3/0.6/0.9/1.5/3 kPa {:.2?} mm | 3 kPa over 0.3 kPa {ratio:.2}",
            swings
        );
    }
}

/// Calibrating the voiced swing limit: where the balance settles, per
/// pressure, for a range of κ.
#[test]
#[ignore = "calibration: prints, asserts nothing"]
fn the_voiced_limit() {
    let design = Parameters::default().reed_design();
    let model = ReedModel::new(design);
    let frequency = design.frequency * 2f64.powf(-6.5 / 1200.0);
    let supplies = [60.0, 100.0, 300.0, 600.0, 1000.0, 1500.0, 3000.0];
    let amplitudes: Vec<f64> = (1..=96).map(|i| 0.125e-3 * i as f64).collect();
    // (air's work, damping, voiced at κ = 1) per supply and swing.
    let table: Vec<Vec<(f64, f64, f64)>> = supplies
        .iter()
        .map(|&p| {
            let mean = model.mu * p / (model.omega * model.omega);
            amplitudes
                .iter()
                .map(|&a| {
                    let w = work(design, p, mean, a, frequency);
                    (w.total(), w.dissipated, w.voiced)
                })
                .collect()
        })
        .collect();
    for kappa in [0.0, 0.05, 0.1, 0.2, 0.3, 0.5, 1.0, 2.0] {
        let swings: Vec<String> = table
            .iter()
            .map(|row| {
                let ratio =
                    |&(air, damping, voiced): &(f64, f64, f64)| air / (damping + kappa * voiced);
                if ratio(&row[0]) < 1.0 {
                    return "silent".to_owned();
                }
                for i in 1..row.len() {
                    let (r0, r1) = (ratio(&row[i - 1]), ratio(&row[i]));
                    if r0 >= 1.0 && r1 < 1.0 {
                        let a = amplitudes[i - 1]
                            + (amplitudes[i] - amplitudes[i - 1]) * (r0 - 1.0) / (r0 - r1);
                        return format!("{:.2}", a * 1e3);
                    }
                }
                ">12".to_owned()
            })
            .collect();
        println!("κ {kappa:>5}: swing at 60/100/300/600/1000/1500/3000 Pa: {swings:?} mm");
    }
}
