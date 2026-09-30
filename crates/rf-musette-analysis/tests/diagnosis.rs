//! Diagnoses, run by hand: `cargo test --release -p rf-musette-analysis
//! --test diagnosis -- --ignored --nocapture`. They print; they do not
//! assert. What they found is recorded in docs/MODEL.md.

use rf_musette_analysis::{linear_threshold, offset_pressure};
use rf_musette_dsp::parameters::{self, Parameters};
use rf_musette_dsp::reed::ReedModel;

const RATE: f64 = 96_000.0;

fn row(label: &str, p: Parameters) {
    let model = ReedModel::new(p.reed_design());
    let helmholtz =
        1.0 / (2.0 * std::f64::consts::PI * (model.hole_inertance * model.cell_compliance).sqrt());
    let onset = linear_threshold(p.reed_design(), RATE, 1.0, 6000.0);
    let offset = onset.and_then(|on| offset_pressure(p.reed_design(), RATE, 2.0 * on, 6.0, 2.0e-4));
    println!(
        "{label:<28} M_n {:>6.1} M_h {:>6.1} f_H {helmholtz:>7.0} Hz | onset {onset:>8.1?} offset {offset:>8.1?}",
        model.inertance, model.hole_inertance
    );
}

/// How much air the reed spends: the mean flow through the tone hole.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn air_consumption_against_pressure() {
    use rf_musette_dsp::reed::{self, ReedState};
    let design = Parameters::default().reed_design();
    let model = ReedModel::new(design);
    let h = 1.0 / RATE;
    for pressure in [100.0, 300.0, 600.0, 900.0] {
        let mut state = ReedState::default();
        let (mut sum, mut peak, mut count) = (0.0, 0.0f64, 0);
        for n in 0..(2.0 * RATE) as usize {
            reed::step(&model, &mut state, pressure, h);
            if n > (1.5 * RATE) as usize {
                sum += state.hole_flow;
                peak = peak.max(state.hole_flow);
                count += 1;
            }
        }
        println!(
            "{pressure:>5} Pa: mean flow {:.0} mL/s, peak {:.0} mL/s",
            sum / count as f64 * 1e6,
            peak * 1e6
        );
    }
}

/// The steady tone across the playing range.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn tone_against_pressure() {
    use rf_musette_analysis::{cents, steady};
    let design = Parameters::default().reed_design();
    println!("   Pa |       Hz | cents re mode | tip mm | mean mm | level dB");
    for pressure in [
        80.0, 100.0, 150.0, 200.0, 300.0, 450.0, 600.0, 900.0, 1500.0, 3000.0,
    ] {
        match steady(design, RATE, pressure, 2.0, 0.5) {
            Some((tone, _)) => println!(
                "{pressure:>5} | {:>8.3} | {:>13.2} | {:>6.2} | {:>7.3} | {:>8.1}",
                tone.frequency,
                cents(design.frequency, tone.frequency),
                tone.amplitude * 1e3,
                tone.mean * 1e3,
                20.0 * tone.flow_rate_rms.log10()
            ),
            None => println!("{pressure:>5} | silent"),
        }
    }
}

/// Whether a softer tongue at the same frequency -- less modal mass, so less
/// stiffness at the tip -- makes the pitch sag with pressure.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn pitch_sag_against_tongue_stiffness() {
    use rf_musette_analysis::{cents, steady};
    use rf_musette_dsp::reed::ReedDesign;
    let base = Parameters::default().reed_design();
    for density_scale in [1.0, 0.5, 0.25, 0.125] {
        // Scale the modal mass alone, leaving the geometry the air sees.
        let design = ReedDesign {
            density: base.density * density_scale,
            ..base
        };
        let model = ReedModel::new(design);
        let stiffness = model.modal_mass * model.omega * model.omega;
        let tones: Vec<_> = [100.0, 300.0, 900.0]
            .iter()
            .map(|p| steady(design, RATE, *p, 2.0, 0.5).map(|(t, _)| t))
            .collect();
        let describe = |t: &Option<rf_musette_analysis::Tone>| match t {
            Some(t) => format!(
                "{:.2} Hz {:.2} mm mean {:+.3}",
                t.frequency,
                t.amplitude * 1e3,
                t.mean * 1e3
            ),
            None => "silent".into(),
        };
        let sag = match (&tones[0], &tones[2]) {
            (Some(a), Some(b)) => cents(a.frequency, b.frequency),
            _ => f64::NAN,
        };
        println!(
            "K {stiffness:>6.0} N/m | 100 Pa: {} | 300 Pa: {} | 900 Pa: {} | sag {sag:+.1} cents",
            describe(&tones[0]),
            describe(&tones[1]),
            describe(&tones[2])
        );
    }
}

/// How the onset moves with each constant, one at a time.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn onset_against_each_constant() {
    row("defaults", Parameters::default());
    let sweeps = [
        (parameters::REED_Q, [95.0, 250.0, 600.0]),
        (parameters::CELL_VOLUME, [2.0, 8.0, 18.0]),
        (parameters::TONE_HOLE_AREA, [60.0, 150.0, 400.0]),
        (parameters::TONE_HOLE_DEPTH, [2.0, 5.0, 10.0]),
        (parameters::NEAR_FIELD, [0.5, 1.0, 2.0]),
        (parameters::REED_SET, [0.25, 0.5, 1.0]),
        (parameters::SIDE_CLEARANCE, [0.02, 0.035, 0.06]),
    ];
    for (index, values) in sweeps {
        for value in values {
            let mut p = Parameters::default();
            assert!(p.set(index, value));
            row(&format!("{} = {value}", parameters::SPECS[index].id), p);
        }
    }
}
