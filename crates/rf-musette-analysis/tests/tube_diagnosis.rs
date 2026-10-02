//! Which treble reeds speak, and from what pressure: run against the cell
//! as a volume and as a tube in milestone 8l (docs/ROADMAP.md). Diagnosis;
//! prints.

use rf_musette_analysis::{simulate, speaks};
use rf_musette_dsp::Parameters;
use rf_musette_dsp::compass::{FIRST_KEY, design};

#[test]
#[ignore = "diagnosis: prints which treble reeds speak at which pressures"]
fn which_reeds_speak() {
    let p = Parameters::default();
    for rank in 0..5 {
        for index in (0..41).step_by(5).chain([35, 36, 38, 40]) {
            let key = FIRST_KEY + index as u8;
            let Some(reed) = design(&p, key, rank) else {
                continue;
            };
            let seconds = (600.0 / reed.frequency).clamp(1.5, 6.0);
            let speaking: Vec<String> = [50.0, 100.0, 300.0, 1000.0]
                .iter()
                .map(|pressure| {
                    let trace = simulate(reed, 192_000.0, seconds, |_| *pressure);
                    format!(
                        "{pressure}:{}",
                        if speaks(&trace, seconds, reed.set) {
                            "y"
                        } else {
                            "-"
                        }
                    )
                })
                .collect();
            println!(
                "rank {rank} key {key} ({:.0} Hz, tongue {:.1} mm): {}",
                reed.frequency,
                reed.length * 1e3,
                speaking.join(" ")
            );
        }
    }
}

/// Where each cell resonates against its reed: the first frequency at which
/// the tube, closed at the reed's tip end and loaded at its opening by the
/// hole's air, is in resonance (tan kL = Z₀/(ωM_h)), and the lumped
/// volume's Helmholtz frequency, both over the reed's own frequency.
#[test]
#[ignore = "diagnosis: prints each cell's resonance against its reed"]
fn where_the_cells_resonate() {
    use rf_musette_dsp::reed::{AIR_DENSITY, CELL_LENGTH_RATIO, ReedModel, SPEED_OF_SOUND};
    let p = Parameters::default();
    for rank in [2usize, 4] {
        for index in [0usize, 10, 20, 25, 30, 33, 35, 38, 40] {
            let key = FIRST_KEY + index as u8;
            let Some(reed) = design(&p, key, rank) else {
                continue;
            };
            let model = ReedModel::new(reed);
            let length = CELL_LENGTH_RATIO * reed.length;
            let section = reed.cell_volume / length;
            let z0 = AIR_DENSITY * SPEED_OF_SOUND / section;
            let mass = model.hole_inertance;
            let helmholtz =
                1.0 / (2.0 * std::f64::consts::PI * (mass * model.cell_compliance).sqrt());
            // Scan for tan kL = Z₀/(ωM_h).
            let mut tube = 0.0;
            let mut previous = f64::NAN;
            for step in 1..200_000 {
                let f = step as f64 * 0.1;
                let w = 2.0 * std::f64::consts::PI * f;
                let k = w / SPEED_OF_SOUND;
                let g = (k * length).tan() - z0 / (w * mass);
                if previous < 0.0 && g >= 0.0 {
                    tube = f;
                    break;
                }
                previous = g;
            }
            println!(
                "rank {rank} key {key}: reed {:.0} Hz; tube resonance {:.0} Hz = {:.2} f0; volume's {:.0} Hz = {:.2} f0",
                reed.frequency,
                tube,
                tube / reed.frequency,
                helmholtz,
                helmholtz / reed.frequency
            );
        }
    }
}

/// One reed that chokes, against its cell's volume and the pressure.
#[test]
#[ignore = "diagnosis: prints one choking reed against its cell"]
fn a_choking_reed() {
    use rf_musette_dsp::compass::{bare, target};
    let p = Parameters::default();
    for (rank, index) in [(1usize, 38u8), (4, 26)] {
        let key = FIRST_KEY + index;
        let mut reed = bare(&p, key, rank).unwrap();
        reed.frequency = target(&p, key, rank);
        let seconds = (600.0 / reed.frequency).clamp(1.5, 6.0);
        for (cell, hole) in [
            (1.0, 1.0),
            (1.5, 1.0),
            (2.5, 1.0),
            (4.0, 1.0),
            (1.0, 1.5),
            (1.0, 2.5),
            (0.5, 2.5),
        ] {
            let trial = rf_musette_dsp::reed::ReedDesign {
                cell_volume: reed.cell_volume * cell,
                tone_hole_area: reed.tone_hole_area * hole,
                ..reed
            };
            let speaking: Vec<String> = [100.0, 300.0, 600.0, 1000.0]
                .iter()
                .map(|pressure| {
                    let trace = simulate(trial, 192_000.0, seconds, |_| *pressure);
                    let tone = trace.tone(seconds - 0.5, seconds);
                    format!(
                        "{pressure}:{}{}",
                        if speaks(&trace, seconds, trial.set) {
                            "y"
                        } else {
                            "-"
                        },
                        tone.map(|t| format!("({:.0})", t.frequency))
                            .unwrap_or_default()
                    )
                })
                .collect();
            println!(
                "rank {rank} key {key} ({:.0} Hz, cell {:.0} mm³ x{cell}, hole {:.0} mm² x{hole}): {}",
                reed.frequency,
                reed.cell_volume * 1e9,
                reed.tone_hole_area * 1e6,
                speaking.join(" ")
            );
        }
    }
}

/// Each treble reed's threshold, as built before tuning, against its pitch.
#[test]
#[ignore = "diagnosis: prints every treble reed's threshold"]
fn the_thresholds() {
    use rf_musette_dsp::compass::{bare, target};
    let p = Parameters::default();
    for rank in [2usize, 4] {
        for index in 0..41u8 {
            let key = FIRST_KEY + index;
            let mut reed = bare(&p, key, rank).unwrap();
            reed.frequency = target(&p, key, rank);
            let seconds = (600.0 / reed.frequency).clamp(1.5, 6.0);
            let threshold = [50.0, 100.0, 150.0, 200.0, 300.0, 400.0, 600.0, 1000.0]
                .into_iter()
                .find(|pressure| {
                    let trace = simulate(reed, 192_000.0, seconds, |_| *pressure);
                    speaks(&trace, seconds, reed.set)
                });
            println!(
                "rank {rank} key {key} {:.0} Hz tongue {:.1} mm: {threshold:?}",
                reed.frequency,
                reed.length * 1e3
            );
        }
    }
}

/// Where the tuner fails on the reeds it cannot tune.
#[test]
#[ignore = "diagnosis: prints the tuner's steps on two reeds"]
fn the_tuner_on_two_reeds() {
    use rf_musette_analysis::{cents, sounding, tune_design, tuning_pressure};
    use rf_musette_dsp::compass::{bare, target};
    let p = Parameters::default();
    for (rank, index) in [(1usize, 38u8), (4, 26)] {
        let key = FIRST_KEY + index;
        let mut reed = bare(&p, key, rank).unwrap();
        let aim = target(&p, key, rank);
        reed.frequency = aim;
        let pressure = tuning_pressure(reed);
        println!("rank {rank} key {key}: tuning pressure {pressure:?}");
        if let Some(pressure) = pressure {
            let mut correction = 0.0;
            for pass in 0..2 {
                reed.frequency = aim * 2f64.powf(correction / 1200.0);
                let tone = sounding(reed, pressure);
                println!(
                    "  pass {pass}: mode {:.1} Hz -> {:?}",
                    reed.frequency,
                    tone.map(|t| t.frequency)
                );
                let Some(tone) = tone else { break };
                correction += cents(tone.frequency, aim);
            }
        }
        reed.frequency = aim;
        println!("  tune_design: {:?}", tune_design(reed, aim));
    }
}

/// Prediction 3: the held F4, dry -- the rate of the reed's flow, the source
/// before any microphone -- its harmonics from 2 to 5 kHz and 5 to 9 kHz.
#[test]
#[ignore = "diagnosis: prints the held F4's harmonics"]
fn the_held_f4s_spectrum() {
    use rf_musette_analysis::{Trace, steady};
    use rf_musette_dsp::parameters::RANK_MIDDLE;
    let p = Parameters::default();
    let rate = 192_000.0;
    let (tone, trace) =
        steady(design(&p, 65, RANK_MIDDLE).unwrap(), rate, 300.0, 1.5, 0.5).unwrap();
    let window = &trace.flow_rate[(1.0 * rate) as usize..(1.5 * rate) as usize];
    let count = (9_000.0 / tone.frequency) as usize;
    let levels = Trace::harmonics(window, rate, tone.frequency, count);
    let band = |low: f64, high: f64| {
        let picked: Vec<f64> = levels
            .iter()
            .enumerate()
            .filter(|(n, _)| (low..high).contains(&((*n + 1) as f64 * tone.frequency)))
            .map(|(_, level)| *level)
            .collect();
        let mean = picked.iter().sum::<f64>() / picked.len() as f64;
        let min = picked.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = picked.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        (min, max, mean)
    };
    println!("F4 at {:.1} Hz: {levels:.1?}", tone.frequency);
    println!("2-5 kHz: {:.1?}", band(2_000.0, 5_000.0));
    println!("5-9 kHz: {:.1?}", band(5_000.0, 9_000.0));
}

/// The held A4 of the middle rank, alone at a steady pressure and on the
/// engine's bellows: its flow's rate written out to look for what is not
/// harmonic (the player's "squeal", 8m).
#[test]
#[ignore = "diagnosis: writes the A4's radiated flow, steady and on the bellows"]
fn the_a4_alone() {
    use rf_musette_analysis::simulate_bellows;
    use rf_musette_dsp::parameters::RANK_MIDDLE;
    let p = Parameters::default();
    let reed = design(&p, 69, RANK_MIDDLE).unwrap();
    let out = std::env::var("RF_MUSETTE_OUT").unwrap();
    for rate in [96_000.0, 192_000.0] {
        let steady = simulate(reed, rate, 5.0, |_| 300.0);
        let bellows = simulate_bellows(&p, reed, rate, 5.0, 300.0);
        for (name, trace) in [("steady", steady), ("bellows", bellows)] {
            let text: String = trace.flow_rate.iter().map(|v| format!("{v:e}\n")).collect();
            std::fs::write(format!("{out}/a4-{name}-{}.txt", rate as u32), text).unwrap();
        }
    }
}

/// The longest cell on the instrument, in steps at 192 kHz.
#[test]
#[ignore = "diagnosis: prints the longest cell"]
fn the_longest_cell() {
    use rf_musette_dsp::compass::{BASS_KEYS, KEYS, bass_design};
    use rf_musette_dsp::reed::CELL_LENGTH_RATIO;
    let p = Parameters::default();
    let mut longest = 0.0f64;
    for rank in 0..5 {
        for index in 0..KEYS {
            if let Some(d) = design(&p, FIRST_KEY + index as u8, rank) {
                longest = longest.max(d.length);
            }
        }
        for class in 0..BASS_KEYS {
            if let Some(d) = bass_design(&p, class, rank) {
                longest = longest.max(d.length);
            }
        }
    }
    let cell = CELL_LENGTH_RATIO * longest;
    println!(
        "longest tongue {:.1} mm, cell {:.1} mm, {:.1} steps at 192 kHz",
        longest * 1e3,
        cell * 1e3,
        cell / 343.2 * 192_000.0
    );
}

/// The measured F4 at 96 and 192 kHz, steady at 150, 300 and 600 Pa.
#[test]
#[ignore = "diagnosis: prints the F4 at two rates"]
fn the_f4_at_two_rates() {
    use rf_musette_analysis::{cents, steady};
    let reed = Parameters::default().reed_design();
    for pressure in [150.0, 300.0, 600.0] {
        let (coarse, _) = steady(reed, 96_000.0, pressure, 1.5, 0.5).unwrap();
        let (fine, _) = steady(reed, 192_000.0, pressure, 1.5, 0.5).unwrap();
        println!(
            "{pressure} Pa: {:+.3} cents, amplitude {:+.2} %",
            cents(fine.frequency, coarse.frequency),
            100.0 * (coarse.amplitude / fine.amplitude - 1.0)
        );
    }
}

/// The scheme at 96 and 384 kHz against the method-of-lines reference.
#[test]
#[ignore = "diagnosis: prints the scheme against the reference"]
fn the_scheme_against_the_reference() {
    use rf_musette_analysis::{cents, reference_tone, steady};
    let reed = Parameters::default().reed_design();
    for pressure in [150.0, 600.0] {
        let reference = reference_tone(reed, 32.0 * 96_000.0, pressure, 1.5).unwrap();
        for rate in [96_000.0, 384_000.0] {
            let (tone, _) = steady(reed, rate, pressure, 1.5, 0.5).unwrap();
            println!(
                "{pressure} Pa at {rate}: {:+.3} cents, amplitude {:+.2} %",
                cents(reference.frequency, tone.frequency),
                100.0 * (tone.amplitude / reference.amplitude - 1.0)
            );
        }
    }
}

/// The cell's modes as the reed plays: two runs of the F3's middle reed at
/// the player's 624 Pa, one nudged in its hole's flow at 1.5 s; the
/// difference rings at the cell's modes and decays. Written out.
#[test]
#[ignore = "diagnosis: writes the F3's ringing difference"]
fn the_modes_as_the_reed_plays() {
    use rf_musette_dsp::parameters::RANK_MIDDLE;
    use rf_musette_dsp::reed::{self, ReedModel, ReedState, Tube};
    let p = Parameters::default();
    let model = ReedModel::new(design(&p, 53, RANK_MIDDLE).unwrap());
    let rate = 96_000.0;
    let h = 1.0 / rate;
    let mut runs = [
        (ReedState::default(), Tube::default()),
        (ReedState::default(), Tube::default()),
    ];
    let mut difference = Vec::new();
    for n in 0..(2.0 * rate) as usize {
        let mut out = [0.0; 2];
        for (k, (state, tube)) in runs.iter_mut().enumerate() {
            if k == 1 && n == (1.5 * rate) as usize {
                state.hole_flow += 1.0e-7;
            }
            out[k] = reed::step(&model, state, tube, 624.0, f64::INFINITY, h);
        }
        if n >= (1.5 * rate) as usize {
            difference.push(format!("{:e}", out[1] - out[0]));
        }
    }
    let path = std::env::var("RF_MUSETTE_OUT").unwrap() + "/ringing-f3.txt";
    std::fs::write(path, difference.join("\n")).unwrap();
}

/// The F4's slot flow at the player's 624 Pa, at 96 and 384 kHz: written
/// out with the tip's position and the section, for the pulse's shape.
#[test]
#[ignore = "diagnosis: writes the F4's slot flow"]
fn the_pulse() {
    use rf_musette_dsp::parameters::RANK_MIDDLE;
    use rf_musette_dsp::reed::{self, ReedModel, ReedState, Tube};
    let p = Parameters::default();
    let model = ReedModel::new(design(&p, 65, RANK_MIDDLE).unwrap());
    for rate in [96_000.0, 384_000.0] {
        let h = 1.0 / rate;
        let mut state = ReedState::default();
        let mut tube = Tube::default();
        let mut lines = Vec::new();
        for n in 0..(2.0 * rate) as usize {
            reed::step(&model, &mut state, &mut tube, 624.0, f64::INFINITY, h);
            if n as f64 >= 1.0 * rate {
                lines.push(format!(
                    "{:e} {:e} {:e} {:e}",
                    state.zeta,
                    state.flow,
                    state.hole_flow,
                    model.section(state.zeta)
                ));
            }
        }
        let path = format!(
            "{}/pulse-{}.txt",
            std::env::var("RF_MUSETTE_OUT").unwrap(),
            rate as u32
        );
        std::fs::write(path, lines.join("\n")).unwrap();
    }
}
