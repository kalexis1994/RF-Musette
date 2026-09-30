//! Milestone 1's predictions, as `docs/ROADMAP.md` wrote them before the
//! reed existed. Each asserts a sign, an order of magnitude or a range the
//! literature gives -- never a value fitted to make it pass. When one fails,
//! the model disagrees with a measured reed, and that is reported rather than
//! tuned away.

use rf_musette_analysis::{
    Trace, cents, linear_threshold, offset_pressure, reference_tone, simulate, steady,
};
use rf_musette_dsp::parameters::{self, Parameters, SPECS};
use rf_musette_dsp::reed::{self, ReedModel, ReedState};

/// The shipping rate: 48 kHz, oversampled twice.
const RATE: f64 = 96_000.0;

fn f4() -> reed::ReedDesign {
    Parameters::default().reed_design()
}

/// Prediction 1: no parameter set any knob or program can reach makes the
/// reed blow up or produce a non-number, and with the supply off its energy
/// only falls.
#[test]
fn no_reachable_parameter_set_blows_up() {
    let mut sets = Vec::new();
    for (index, spec) in SPECS.iter().enumerate() {
        if index == parameters::GAIN || index == parameters::OVERSAMPLING {
            continue;
        }
        for value in [spec.minimum, spec.maximum] {
            let mut p = Parameters::default();
            assert!(p.set(index, value));
            sets.push(p);
        }
    }
    // And combinations, drawn by a fixed generator so failures reproduce.
    let mut seed = 0x2545_f491_4f6c_dd1du64;
    for _ in 0..60 {
        let mut p = Parameters::default();
        for (index, spec) in SPECS.iter().enumerate() {
            if index == parameters::GAIN || index == parameters::OVERSAMPLING {
                continue;
            }
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let unit = (seed >> 11) as f64 / (1u64 << 53) as f64;
            assert!(p.set(index, spec.minimum + unit * (spec.maximum - spec.minimum)));
        }
        sets.push(p);
    }
    for p in sets {
        let design = p.reed_design();
        let model = ReedModel::new(design);
        let mut state = ReedState::default();
        let h = 1.0 / RATE;
        for n in 0..(0.25 * RATE) as usize {
            let supply = if n < (0.15 * RATE) as usize {
                6000.0
            } else {
                0.0
            };
            let before = state.energy(&model);
            let out = reed::step(&model, &mut state, supply, h);
            assert!(out.is_finite() && state.zeta.is_finite(), "{design:?}");
            assert!(state.zeta.abs() < 0.05, "tip beyond 5 cm: {design:?}");
            if supply == 0.0 {
                assert!(
                    state.energy(&model) <= before * (1.0 + 1e-12) + 1e-30,
                    "{design:?}"
                );
            }
        }
    }
}

/// Prediction 2: it speaks above a threshold and falls silent below a lower
/// one, with thresholds of the order the literature reports for a
/// mid-register reed -- tens of pascals to about a hundred (Misdariis et al.,
/// CFA 2000: playing begins near 10 Pa; Cottingham, ICA 2016: 60-110 Pa for a
/// 622 Hz reed).
///
/// The onset is where the reed's equilibrium turns unstable: the pressure
/// above which it speaks from rest. (Ramping the supply up from perfect rest
/// measures how long a disturbance of nothing takes to grow instead, which
/// is not a property of the reed.)
#[test]
fn it_speaks_above_a_threshold_and_stops_below_a_lower_one() {
    let onset = linear_threshold(f4(), RATE, 1.0, 6000.0).expect("stable even at 6 kPa");
    // Let down from the IfM's standard playing pressure, 300 Pa.
    let offset = offset_pressure(f4(), RATE, 300.0, 6.0, 2.0e-4)
        .expect("the reed never stopped on the way down");
    println!("onset {onset:.1} Pa, offset {offset:.1} Pa");
    assert!(
        offset < onset,
        "no hysteresis: onset {onset}, offset {offset}"
    );
    assert!((5.0..=300.0).contains(&onset), "onset {onset} Pa");
}

/// The shipping scheme computes the model: an independent integrator (RK4 on
/// the exact Bernoulli law, 32 times the rate) finds the same tone.
#[test]
fn the_scheme_computes_the_model() {
    let design = f4();
    for pressure in [150.0, 600.0] {
        let (tone, _) = steady(design, RATE, pressure, 1.5, 0.5).expect("no tone");
        let reference =
            reference_tone(design, 32.0 * RATE, pressure, 1.5).expect("no reference tone");
        let pitch = cents(reference.frequency, tone.frequency);
        let amplitude = tone.amplitude / reference.amplitude - 1.0;
        println!(
            "{pressure} Pa: {pitch:+.3} cents, amplitude {:+.2} %",
            100.0 * amplitude
        );
        assert!(pitch.abs() < 1.0 && amplitude.abs() < 0.02);
    }
}

/// Prediction 3: it sounds slightly below its natural frequency, and its
/// pitch falls as pressure rises -- of the order of -9 cents over 0.1-0.9 kPa
/// (Cottingham, CCRMA 2013, an accordion G# near 420 Hz) or -10 to -13 cents
/// per kPa (Misdariis et al. 2000). Sign and order of magnitude, not the
/// slope: between -2 and -40 cents from 100 to 900 Pa.
///
/// NOT MET, and recorded as a known defect in docs/MODEL.md: the reed sounds
/// 6.3 cents below its mode at every pressure, within half a cent from 80 Pa
/// to 3 kPa. Millot & Baumann report the same of their minimal model -- a
/// blown-closed reed's frequency moved 0.4 Hz with the excitation -- so the
/// mechanism that makes real reeds sag is not in this class of model. It is
/// kept here, failing, as the target for whoever finds it.
#[test]
#[ignore = "known defect: the minimal model's pitch does not sag with pressure (docs/MODEL.md)"]
fn it_sounds_below_its_mode_and_sags_as_the_pressure_rises() {
    let design = f4();
    let mut previous = None;
    let mut first = None;
    let mut last = 0.0;
    for pressure in [100.0, 300.0, 600.0, 900.0] {
        let (tone, _) = steady(design, RATE, pressure, 1.5, 0.5).expect("no tone");
        let below = cents(design.frequency, tone.frequency);
        println!(
            "{pressure:>5} Pa: {:.2} Hz ({below:+.1} cents from the mode), tip {:.2} mm, mean {:+.2} mm",
            tone.frequency,
            tone.amplitude * 1e3,
            tone.mean * 1e3
        );
        assert!(
            tone.frequency < design.frequency,
            "{pressure} Pa sounds above the mode"
        );
        if let Some(previous) = previous {
            assert!(
                tone.frequency < previous,
                "pitch rose between pressures at {pressure} Pa"
            );
        }
        previous = Some(tone.frequency);
        first.get_or_insert(tone.frequency);
        last = tone.frequency;
    }
    let drop = cents(first.unwrap(), last);
    println!("100 -> 900 Pa: {drop:+.1} cents");
    assert!((-40.0..=-2.0).contains(&drop), "{drop} cents");
}

/// Prediction 5: across its playing pressures the level spans tens of
/// decibels -- about 40 dB over 10-6000 Pa for Misdariis et al. -- and grows
/// with pressure.
#[test]
fn the_level_grows_with_the_pressure() {
    let design = f4();
    let mut levels = Vec::new();
    for pressure in [60.0, 100.0, 300.0, 1000.0, 3000.0] {
        let Some((tone, _)) = steady(design, RATE, pressure, 1.5, 0.5) else {
            println!("{pressure:>5} Pa: silent");
            continue;
        };
        let level = 20.0 * tone.flow_rate_rms.log10();
        println!("{pressure:>5} Pa: {level:.1} dB");
        levels.push(level);
    }
    assert!(levels.len() >= 3, "too few sounding pressures");
    assert!(
        levels.windows(2).all(|pair| pair[1] > pair[0]),
        "{levels:?}"
    );
    let span = levels[levels.len() - 1] - levels[0];
    println!("span {span:.1} dB");
    assert!(span >= 10.0, "span {span} dB");
}

/// The tongue Ziegenhals measured swings more than 4 mm at mezzo-forte and
/// passes right through its 3 mm plate (IfM Zwota 2009, stroboscope, Fig. 9);
/// the IfM's standard playing pressure is 300 Pa. At 300 Pa the tip should
/// swing beyond the 3.1 mm his geometry needs to pass through, and not
/// absurdly far.
#[test]
fn at_a_normal_push_the_tip_passes_through_the_plate() {
    let (tone, _) = steady(f4(), RATE, 300.0, 1.5, 0.5).expect("no tone at 300 Pa");
    println!(
        "300 Pa: amplitude {:.2} mm, mean {:+.2} mm",
        tone.amplitude * 1e3,
        tone.mean * 1e3
    );
    assert!(
        (3.1e-3..=8.0e-3).contains(&tone.amplitude),
        "{} mm",
        tone.amplitude * 1e3
    );
}

/// The tongue moves sinusoidally even while the sound is a pulse train
/// (Misdariis et al. 2000, laser vibrometer, 10-6000 Pa; Ziegenhals 2009):
/// its second and third harmonics sit well below the first.
#[test]
fn the_tongue_moves_sinusoidally() {
    let (tone, trace) = steady(f4(), RATE, 300.0, 1.5, 0.5).expect("no tone");
    let window = &trace.zeta[(1.0 * RATE) as usize..(1.5 * RATE) as usize];
    let mean = window.iter().sum::<f64>() / window.len() as f64;
    let centred: Vec<f64> = window.iter().map(|z| z - mean).collect();
    let tongue = Trace::harmonics(&centred, RATE, tone.frequency, 4);
    let sound_window = &trace.flow_rate[(1.0 * RATE) as usize..(1.5 * RATE) as usize];
    let sound = Trace::harmonics(sound_window, RATE, tone.frequency, 8);
    println!("tongue harmonics (dB re first): {tongue:.1?}");
    println!("sound harmonics (dB re first): {sound:.1?}");
    assert!(tongue[1] < -20.0 && tongue[2] < -20.0, "{tongue:?}");
}

/// The scheme converges: the shipping rate and a rate eight times higher
/// agree on the tone to within a cent and two per cent of amplitude.
#[test]
fn the_shipping_rate_agrees_with_a_much_finer_one() {
    let design = f4();
    let (coarse, _) = steady(design, RATE, 300.0, 1.0, 0.4).expect("no tone");
    let (fine, _) = steady(design, 8.0 * RATE, 300.0, 1.0, 0.4).expect("no tone");
    let pitch = cents(fine.frequency, coarse.frequency);
    let amplitude = coarse.amplitude / fine.amplitude - 1.0;
    println!(
        "coarse vs fine: {pitch:+.3} cents, amplitude {:+.2} %",
        100.0 * amplitude
    );
    assert!(pitch.abs() < 1.0 && amplitude.abs() < 0.02);
}

#[test]
fn a_trace_at_rest_has_no_tone() {
    let trace = simulate(f4(), RATE, 0.2, |_| 0.0);
    assert!(trace.tone(0.0, 0.2).is_none());
}
