//! What the engine's parts cost, natively, for milestone 10 (the cost of
//! the audio path): each part timed alone at a state it meets while
//! playing, and how many times a played chord calls it per output sample.
//! Measurements, run in release:
//! `cargo test --release -p rf-musette-analysis --test cost -- --ignored --nocapture`.

use rf_musette_dsp::cassotto::{Cassotto, CassottoTuning};
use rf_musette_dsp::parameters::{self, Parameters, RANKS};
use rf_musette_dsp::reed::{self, ReedModel, ReedState, Tube};
use rf_musette_dsp::stage::{self, Stage};
use rf_musette_dsp::wind::Wind;
use rf_musette_dsp::{Decimator, Engine};
use std::hint::black_box;
use std::time::Instant;

const RATE: f64 = 48_000.0;

/// Nanoseconds per call of `step`, over `calls` after as many to warm.
fn time(calls: usize, mut step: impl FnMut(usize)) -> f64 {
    for n in 0..calls {
        step(n);
    }
    let start = Instant::now();
    for n in 0..calls {
        step(n);
    }
    start.elapsed().as_secs_f64() * 1e9 / calls as f64
}

/// The four-note Master chord of milestone 7, warmed for a second.
fn chord(layout: Option<f64>) -> Engine {
    let mut engine = Engine::new(RATE as f32).unwrap();
    // The bellows played: Key Touch off, or it rests (9h again).
    assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
    assert!(engine.set_parameter(parameters::REGISTER, 6.0));
    if let Some(layout) = layout {
        assert!(engine.set_parameter(parameters::MIC_LAYOUT, layout));
    }
    engine.bellows_mut().expression_wide(0.6);
    for key in [65, 69, 72, 77] {
        engine.note_on(key, 1.0);
    }
    let mut out = vec![0.0f32; 48_000];
    engine.render(&mut out);
    engine
}

#[test]
#[ignore = "measurement: run in release, prints what each part costs"]
fn the_parts_cost() {
    let p = Parameters::default();
    let h = 1.0 / (2.0 * RATE);
    // One reed, F4, blown at 300 Pa through an open pallet.
    let model = ReedModel::new(p.reed_design());
    let area = model.design.tone_hole_area;
    let mut state = ReedState::equilibrium(&model, 300.0);
    let mut tube = Tube::steady(300.0);
    let reed = time(2_000_000, |_| {
        black_box(reed::step(
            &model,
            &mut state,
            &mut tube,
            black_box(300.0),
            area,
            h,
        ));
    });
    let mut wind = Wind::default();
    let design = p.wind_design().expect("the arm's bellows");
    let wind_cost = time(2_000_000, |_| {
        black_box(wind.step(&design, black_box(300.0), black_box(2.0e-4), h));
    });
    let mut decimator = Decimator::new(2);
    let chunk = [0.1f32, -0.2];
    let decimate = time(2_000_000, |_| {
        black_box(decimator.decimate(black_box(&chunk)));
    });
    let tuning = CassottoTuning::new(400.0, 2.0, h);
    let mut cassotto = Cassotto::default();
    let boxed = time(2_000_000, |n| {
        black_box(cassotto.process(&tuning, black_box((n % 7) as f64 * 1e-4)));
    });
    let mut stage = Stage::new(RATE);
    let mut ortf = p;
    assert!(ortf.set(parameters::MIC_LAYOUT, 3.0));
    stage.tune(&ortf, 0.2);
    let staged = time(500_000, |n| {
        let x = (n % 13) as f64 * 1e-4;
        black_box(stage.process(&[x, -x, x, -x, x], black_box(0.1)));
    });
    println!("reed step   {reed:7.1} ns per call (per reed, per substep)");
    println!("wind step   {wind_cost:7.1} ns per call (per substep)");
    println!("decimate x2 {decimate:7.1} ns per call (per source, per sample)");
    println!("cassotto    {boxed:7.1} ns per call (per quarter, per substep)");
    println!("stage ORTF  {staged:7.1} ns per call (per sample)");
}

/// How many reeds the chord steps per substep: every state not at rest.
#[test]
#[ignore = "measurement: run in release, prints the count"]
fn how_many_reeds_the_chord_steps() {
    let engine = chord(None);
    let mut moving = 0;
    for key in [65u8, 69, 72, 77] {
        for rank in 0..RANKS {
            for which in 0..2 {
                if let Some((_, state)) = engine.reed(key, rank, which)
                    && *state != ReedState::default()
                {
                    moving += 1;
                }
            }
        }
    }
    println!(
        "four-note Master: {moving} reeds stepped per substep, {} substeps per sample",
        engine.parameters().oversampling()
    );
    let _ = stage::SOURCES;
}

/// Whether the engine's software square root is the correctly rounded one
/// (IEEE 754's, which the hardware gives): if it always is, the hardware's
/// can replace it bit for bit. Random values over the engine's ranges and
/// over every exponent.
#[test]
#[ignore = "measurement: run in release, prints how often they differ"]
fn the_software_square_root_against_the_hardwares() {
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    let mut next = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    // Per kind: normal doubles of any exponent, subnormals, the engine's.
    let mut differ = [0u64; 3];
    let mut worst = [0i64; 3];
    let trials = 200_000_000u64;
    for n in 0..trials {
        let bits = next();
        let x = if n % 2 == 0 {
            // Any positive finite double.
            f64::from_bits(bits >> 1).abs()
        } else {
            // The engine's: 0 to 2e4 (2p/rho for p to 12 kPa).
            (bits >> 11) as f64 / (1u64 << 53) as f64 * 2.0e4
        };
        if !x.is_finite() || x <= 0.0 {
            continue;
        }
        let kind = if n % 2 == 1 {
            2
        } else if x.is_normal() {
            0
        } else {
            1
        };
        let software = rf_musette_dsp::math::sqrt(x);
        let hardware = x.sqrt();
        if software.to_bits() != hardware.to_bits() {
            differ[kind] += 1;
            let ulps = software.to_bits() as i64 - hardware.to_bits() as i64;
            if ulps.abs() > worst[kind].abs() {
                worst[kind] = ulps;
            }
        }
    }
    for (kind, name) in ["normal", "subnormal", "the engine's range"]
        .iter()
        .enumerate()
    {
        println!(
            "{name}: {} differ, the worst by {} ulp ({trials} trials in all)",
            differ[kind], worst[kind]
        );
    }
}

/// The engine's old software root (dd7e761), kept here to compare.
fn software_sqrt(x: f64) -> f64 {
    if x <= 0.0 || !x.is_finite() {
        return if x.is_finite() || x < 0.0 { 0.0 } else { x };
    }
    let mut y = f64::from_bits((x.to_bits() >> 1) + (1023u64 << 51));
    for _ in 0..6 {
        y = 0.5 * (y + x / y);
    }
    y
}

/// How often the old root differed on the arguments a sounding reed
/// actually takes: 2p/rho of its cell, step by step.
#[test]
#[ignore = "measurement: prints how often the old root differed in a reed"]
fn the_old_root_on_a_reeds_own_arguments() {
    let p = Parameters::default();
    for (rate, supply) in [(96_000.0, 300.0), (48_000.0, 300.0), (96_000.0, 1000.0)] {
        let model = ReedModel::new(p.reed_design());
        let mut state = ReedState::default();
        let mut tube = Tube::default();
        let area = model.design.tone_hole_area;
        let (mut differ, mut total) = (0, 0);
        for _ in 0..(rate as usize) {
            let x = 2.0 * state.cell_pressure.max(0.0) / 1.2;
            if x > 0.0 {
                total += 1;
                if software_sqrt(x).to_bits() != x.sqrt().to_bits() {
                    differ += 1;
                }
            }
            reed::step(&model, &mut state, &mut tube, supply, area, 1.0 / rate);
        }
        println!("{rate} Hz, {supply} Pa: {differ} of {total} steps differ");
    }
}

/// What a block costs with nothing sounding: after a chord has rung out,
/// the defaults (an ORTF pair in a room), natively.
#[test]
#[ignore = "measurement: run in release, prints the idle cost"]
fn the_idle_cost() {
    for stereo in [false, true] {
        let mut engine = Engine::new(RATE as f32).unwrap();
        assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
        for key in [48, 55, 60, 64, 67] {
            engine.note_on(key, 0.8);
        }
        let mut left = vec![0.0f32; 128];
        let mut right = vec![0.0f32; 128];
        for _ in 0..375 {
            engine.render_stereo(&mut left, &mut right);
        }
        for key in [48, 55, 60, 64, 67] {
            engine.note_off(key);
        }
        // Ten seconds for every reed, the filter and the room to stop.
        for _ in 0..3750 {
            engine.render_stereo(&mut left, &mut right);
        }
        let blocks = 3750;
        let start = Instant::now();
        for _ in 0..blocks {
            if stereo {
                engine.render_stereo(&mut left, &mut right);
            } else {
                engine.render(&mut left);
            }
        }
        let per_sample = start.elapsed().as_secs_f64() / (blocks * 128) as f64 * 1e6;
        let peak = left
            .iter()
            .chain(&right)
            .fold(0.0f32, |m, x| m.max(x.abs()));
        println!(
            "idle, {}: {per_sample:.3} µs per sample (peak {peak:e})",
            if stereo { "stereo" } else { "mono" }
        );
    }
}
