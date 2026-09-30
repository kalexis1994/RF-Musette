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

/// The finger attack against the pressure, next to the equilibrium's linear
/// growth rate there: an attack of ~5 e-folds (from -50 to -5 dB) at a rate
/// σ lasts about 5.2/σ.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn finger_attack_against_pressure() {
    use rf_musette_analysis::{attack_time, component_envelope, growth_rate, simulate_keyed};
    let p = Parameters::default();
    let (design, pallet) = (p.reed_design(), p.pallet_design());
    for pressure in [60.0, 100.0, 200.0, 400.0, 800.0] {
        let trace = simulate_keyed(design, pallet, RATE, 2.0, pressure, |t| {
            if t >= 0.05 { 1.0 } else { 0.0 }
        });
        let Some(tone) = trace.tone(1.5, 2.0) else {
            println!("{pressure:>5} Pa: silent");
            continue;
        };
        let envelope = component_envelope(&trace.flow_rate, RATE, tone.frequency, 4.0, 0.001);
        let attack = attack_time(&envelope);
        let sigma = growth_rate(design, RATE, pressure);
        println!(
            "{pressure:>5} Pa: attack {:>6.1?} ms | linear growth {sigma:>6.1} /s -> 5.2/σ = {:>6.0} ms",
            attack.map(|a| a * 1e3),
            5.2 / sigma * 1e3
        );
    }
}

/// The finger attack measured as Llanos-Vázquez et al. measured it: their
/// table's frequencies step by 12.5 Hz, so their spectra came from ~80 ms
/// windows every 10 ms; the attack runs from the first harmonic's -50 dB to
/// its -5 dB of the maximum. Beside it, the step response the pallet gives
/// the tongue: the static deflection μP/ω0², against the steady swing.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn finger_attack_measured_as_llanos() {
    use rf_musette_analysis::{attack_time, component_envelope, simulate_keyed};
    let p = Parameters::default();
    let (design, pallet) = (p.reed_design(), p.pallet_design());
    let model = ReedModel::new(design);
    println!("tip stiffness {:.0} N/m", model.tip_stiffness());
    for pressure in [100.0, 400.0] {
        let trace = simulate_keyed(design, pallet, RATE, 2.0, pressure, |t| {
            if t >= 0.1 { 1.0 } else { 0.0 }
        });
        let tone = trace.tone(1.5, 2.0).expect("no tone");
        let fine = attack_time(&component_envelope(
            &trace.flow_rate,
            RATE,
            tone.frequency,
            4.0,
            0.001,
        ));
        // Their window: 80 ms, rectangular as an FFT frame is, every 10 ms.
        let periods = 0.080 * tone.frequency;
        let coarse = attack_time(&component_envelope(
            &trace.flow_rate,
            RATE,
            tone.frequency,
            periods,
            0.010,
        ));
        let kick = model.mu * pressure / (model.omega * model.omega);
        println!(
            "{pressure:>4} Pa: attack {:>5.0?} ms fine, {:>5.0?} ms through an 80 ms window | kick {:.3} mm against a {:.2} mm swing ({:.0} dB)",
            fine.map(|a| a * 1e3),
            coarse.map(|a| a * 1e3),
            kick * 1e3,
            tone.amplitude * 1e3,
            20.0 * (kick / tone.amplitude).log10()
        );
    }
    // An instantaneous onset through the same window: what the method
    // itself reads as an attack.
    let rate = RATE;
    let f = 353.7;
    let step: Vec<f64> = (0..(2.0 * rate) as usize)
        .map(|n| {
            let t = n as f64 / rate;
            if t < 0.5 {
                0.0
            } else {
                (2.0 * std::f64::consts::PI * f * t).sin()
            }
        })
        .collect();
    let floor = attack_time(&component_envelope(&step, rate, f, 0.080 * f, 0.010));
    println!(
        "an instantaneous onset reads as {:.0?} ms through the 80 ms window",
        floor.map(|a| a * 1e3)
    );
}

/// What the reed's adjustment does to its attack: the growth rate at 100 and
/// 400 Pa, and the attack it implies (~33 dB from the pallet's kick at
/// -38 dB to -5 dB, 3.8 e-folds), against the set and the clearances --
/// what a reed maker adjusts, and what practitioners say makes "a mano"
/// reeds respond faster.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn attack_against_the_reeds_adjustment() {
    use rf_musette_analysis::growth_rate;
    use rf_musette_dsp::parameters::{REED_Q, REED_SET, SIDE_CLEARANCE, TIP_CLEARANCE};
    let sweeps = [
        (REED_SET, "set mm", [0.15, 0.25, 0.5, 0.8]),
        (
            SIDE_CLEARANCE,
            "side clearance mm",
            [0.015, 0.025, 0.035, 0.06],
        ),
        (TIP_CLEARANCE, "tip clearance mm", [0.015, 0.04, 0.1, 0.2]),
        (REED_Q, "Q", [95.0, 250.0, 600.0, 1000.0]),
    ];
    for (index, name, values) in sweeps {
        for value in values {
            let mut p = Parameters::default();
            assert!(p.set(index, value));
            let design = p.reed_design();
            let onset = linear_threshold(design, RATE, 1.0, 6000.0);
            let rates: Vec<f64> = [100.0, 400.0]
                .iter()
                .map(|pr| growth_rate(design, RATE, *pr))
                .collect();
            println!(
                "{name:>18} {value:<6} onset {:>6.1?} Pa | σ {:>5.1} / {:>5.1} /s -> ~{:>4.0} / {:>4.0} ms",
                onset,
                rates[0],
                rates[1],
                3.8 / rates[0] * 1e3,
                3.8 / rates[1] * 1e3
            );
        }
    }
}

/// Onset and the attack's growth rate together, against Q and the air's
/// inertia (raised through the hole's area): which unmeasured constants
/// could meet both measured facts -- onset ~30 Pa, attack 50-140 ms.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn onset_and_attack_against_q_and_inertia() {
    use rf_musette_analysis::growth_rate;
    use rf_musette_dsp::parameters::{REED_Q, TONE_HOLE_AREA};
    println!("    Q | hole mm² | M_h+M_n | onset Pa | attack ~5.2/σ at 100 / 400 Pa");
    for q in [95.0, 150.0, 250.0] {
        for area in [150.0, 75.0, 40.0, 25.0] {
            let mut p = Parameters::default();
            assert!(p.set(REED_Q, q) && p.set(TONE_HOLE_AREA, area));
            let design = p.reed_design();
            let model = ReedModel::new(design);
            let onset = linear_threshold(design, RATE, 1.0, 6000.0);
            let attack = |pressure: f64| 5.2 / growth_rate(design, RATE, pressure) * 1e3;
            println!(
                "{q:>5} | {area:>8} | {:>7.0} | {:>8.1?} | {:>6.0} / {:>4.0} ms",
                model.hole_inertance + model.inertance,
                onset,
                attack(100.0),
                attack(400.0)
            );
        }
    }
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
            reed::step(&model, &mut state, pressure, f64::INFINITY, h);
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
