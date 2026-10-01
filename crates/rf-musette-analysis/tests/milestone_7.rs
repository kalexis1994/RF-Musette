#![allow(clippy::needless_range_loop)]
//! Milestone 7: the whole treble, F3-A6, every rank tuned as a tuner tunes,
//! and what it costs. Predictions as written in docs/ROADMAP.md before the
//! code; each test says which.

use rf_musette_analysis::{TUNING_PRESSURE, cents, sounding, tune_reed, tuning_pressure};
use rf_musette_dsp::Engine;
use rf_musette_dsp::compass::{FIRST_KEY, KEYS, design, target};
use rf_musette_dsp::parameters::{self, Parameters, RANK_FLAT, RANK_MIDDLE, RANK_SHARP, RANKS};

const NAMES: [&str; RANKS] = ["L", "M−", "M", "M+", "H"];

/// Prediction 1: every reed of every rank sounds within ±2 cents of its
/// pitch where it is tuned -- 300 Pa, or for the few that do not speak
/// there, the lowest pressure they do.
#[test]
fn every_reed_sounds_in_tune() {
    let p = Parameters::default();
    let mut worst = (0.0f64, String::new());
    for rank in 0..RANKS {
        for index in 0..KEYS {
            let key = FIRST_KEY + index as u8;
            let reed = design(&p, key, rank).unwrap();
            let pressure = tuning_pressure(reed)
                .unwrap_or_else(|| panic!("{} at key {key} never speaks", NAMES[rank]));
            let tone = sounding(reed, pressure).unwrap();
            let off = cents(target(&p, key, rank), tone.frequency);
            if off.abs() > worst.0.abs() {
                worst = (off, format!("{} at key {key}", NAMES[rank]));
            }
            assert!(
                off.abs() < 2.0,
                "{} at key {key}: {off:+.2} cents",
                NAMES[rank]
            );
        }
    }
    println!("worst: {:+.2} cents, {}", worst.0, worst.1);
}

/// Predictions 2 and 9: every reed speaks from rest at the 300 Pa tuning
/// pressure, as a tuner says every reed does.
///
/// NOT MET at the very top: with Q following the measured trend, the 4′
/// reeds from about 3 kHz (keys 90-93, F#7-A7) need 350-440 Pa. They are
/// tuned where they first speak. A tuner says the top piccolos start at
/// 100-250 Pa, some near 300 (musiker-board, 2014); what makes the smallest
/// reeds speak on an instrument -- their cell, their set, the tip turned to
/// the inlet (Tonon 2005) -- is not measured.
#[test]
#[ignore = "known defect: the highest 4′ reeds need more than 300 Pa (docs/ROADMAP.md, 7)"]
fn every_reed_speaks_at_the_tuning_pressure() {
    let p = Parameters::default();
    let mut silent = Vec::new();
    for rank in 0..RANKS {
        for index in 0..KEYS {
            let key = FIRST_KEY + index as u8;
            if sounding(design(&p, key, rank).unwrap(), TUNING_PRESSURE).is_none() {
                silent.push(format!("{} {key}", NAMES[rank]));
            }
        }
    }
    println!("silent at 300 Pa: {silent:?}");
    assert!(silent.is_empty(), "{silent:?}");
}

/// The tuning table is the one a fresh tuning makes (every fifth key, to keep
/// the check quick).
#[test]
fn the_tuning_table_is_current() {
    let p = Parameters::default();
    for rank in 0..RANKS {
        for index in (0..KEYS).step_by(5) {
            let stored = f64::from(rf_musette_dsp::tuning::CENTS[rank][index]);
            let now = tune_reed(&p, FIRST_KEY + index as u8, rank).unwrap();
            assert!(
                (stored - now).abs() < 0.01,
                "{} at key {}: table {stored:.2}, fresh {now:.2}; run `rf-musette-lab tune`",
                NAMES[rank],
                FIRST_KEY + index as u8
            );
        }
    }
}

/// Prediction 1, its second half: the true 8′ at 100 and 600 Pa stays within
/// ±15 cents of its pitch.
#[test]
fn the_true_eight_foot_holds_its_pitch_across_the_dynamics() {
    let p = Parameters::default();
    for index in (0..KEYS).step_by(4) {
        let key = FIRST_KEY + index as u8;
        let reed = design(&p, key, RANK_MIDDLE).unwrap();
        for pressure in [100.0, 600.0] {
            let Some(tone) = sounding(reed, pressure) else {
                continue;
            };
            let off = cents(target(&p, key, RANK_MIDDLE), tone.frequency);
            assert!(
                off.abs() < 15.0,
                "key {key} at {pressure} Pa: {off:+.1} cents"
            );
        }
    }
}

/// Prediction 3: the tremolo's beats follow the Borsini's lines at every
/// key, within 5 %.
#[test]
fn the_tremolo_follows_the_measured_lines_across_the_compass() {
    let p = Parameters::default();
    for index in (0..KEYS).step_by(5) {
        let key = FIRST_KEY + index as u8;
        let sound = |rank| sounding(design(&p, key, rank).unwrap(), TUNING_PRESSURE).unwrap();
        let middle = sound(RANK_MIDDLE).frequency;
        for rank in [RANK_FLAT, RANK_SHARP] {
            let got = sound(rank).frequency - middle;
            let want = target(&p, key, rank) - target(&p, key, RANK_MIDDLE);
            assert!(
                (got / want - 1.0).abs() < 0.05,
                "key {key} {}: {got:+.3} Hz against {want:+.3}",
                NAMES[rank]
            );
        }
    }
}

/// Prediction 4: at the tuning pressure each tongue swings a similar share
/// of its length across the compass, 5-20 % (about 15 %: Braasch &
/// Cottingham).
///
/// NOT MET above ~A5: the share falls from 12.7 % at F3 to 4.4 % at A5. It
/// also prints the share at eight times each reed's own threshold, to see
/// whether it is only that the high reeds sit nearer theirs at 300 Pa.
#[test]
#[ignore = "known defect: high tongues swing a smaller share (docs/ROADMAP.md, 7)"]
fn the_swing_scales_with_the_tongue() {
    let p = Parameters::default();
    for index in (0..KEYS).step_by(4) {
        let key = FIRST_KEY + index as u8;
        let reed = design(&p, key, RANK_MIDDLE).unwrap();
        let tone = sounding(reed, TUNING_PRESSURE).unwrap();
        let share = tone.amplitude / reed.length;
        let threshold =
            rf_musette_analysis::linear_threshold(reed, 192_000.0, 1.0, 3000.0).unwrap();
        let far = sounding(reed, 8.0 * threshold).map(|t| t.amplitude / reed.length);
        println!(
            "key {key}: {:.1} mm long, swing {:.2} mm, {:.1} % at 300 Pa ({:.1}× its threshold); at 8× threshold {:.1?} %",
            reed.length * 1e3,
            tone.amplitude * 1e3,
            100.0 * share,
            TUNING_PRESSURE / threshold,
            far.map(|s| 100.0 * s)
        );
        assert!(
            (0.05..0.20).contains(&share),
            "key {key}: {:.1} %",
            100.0 * share
        );
    }
}

/// Prediction 5, measured: a four-note Master chord's cost per 48 kHz
/// sample, natively.
#[test]
#[ignore = "measurement: run in release, prints the cost"]
fn a_full_chord_costs() {
    let rate = 48_000.0f32;
    let mut engine = Engine::new(rate).unwrap();
    assert!(engine.set_parameter(parameters::REGISTER, 6.0));
    engine.bellows_mut().expression_wide(0.6);
    for key in [65, 69, 72, 77] {
        engine.note_on(key, 1.0);
    }
    let mut warm = vec![0.0f32; 48_000];
    engine.render(&mut warm);
    let mut out = vec![0.0f32; 5 * 48_000];
    let start = std::time::Instant::now();
    for block in out.chunks_mut(256) {
        engine.render(block);
    }
    let per_sample = start.elapsed().as_secs_f64() / out.len() as f64;
    println!(
        "four-note Master: {:.2} µs per sample, {:.0} % of real time at 48 kHz",
        per_sample * 1e6,
        100.0 * per_sample * f64::from(rate)
    );
}
