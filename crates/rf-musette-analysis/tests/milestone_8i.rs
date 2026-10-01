//! Milestone 8i: the wheel as the bellows. Predictions as written in
//! docs/ROADMAP.md before the code; each test says which.

use rf_musette_dsp::parameters::{self, RANK_MIDDLE};
use rf_musette_dsp::{BellowsSource, Engine, PUSH_REED};

const RATE: f32 = 48_000.0;
const RATE_SAMPLES: usize = RATE as usize;

/// The reeds' start: below it a reed is not set going.
const SPEAKS: f64 = parameters::KICK_PRESSURE;

fn engine() -> Engine {
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(parameters::MOD_WHEEL, parameters::WHEEL_BELLOWS));
    engine
}

/// A wheel's messages: (sample, high half, low half if any, the low half's
/// delay in samples).
type Message = (usize, u8, Option<(u8, usize)>);

/// A 7-bit wheel stepping by `direction` every `interval` samples from
/// `start`, between `from` and `to` seconds.
fn steps(start: u8, direction: i32, interval: usize, from: f32, to: f32) -> Vec<Message> {
    let mut messages = Vec::new();
    let mut at = (from * RATE) as usize;
    let mut value = i32::from(start);
    while at < (to * RATE) as usize && (0..=127).contains(&value) {
        messages.push((at, value as u8, None));
        value += direction;
        at += interval;
    }
    messages
}

/// Renders `seconds`, sending the messages on their samples -- or on the
/// start of the `block` they fall in -- and calling `each` with the sample.
fn run(
    engine: &mut Engine,
    seconds: f32,
    messages: &[Message],
    block: usize,
    mut each: impl FnMut(&mut Engine, usize),
) {
    let mut pending: Vec<(usize, bool, u8)> = Vec::new();
    for (at, high, low) in messages {
        let at = at / block * block;
        pending.push((at, false, *high));
        if let Some((low, delay)) = low {
            pending.push(((at + delay) / block * block, true, *low));
        }
    }
    pending.sort_by_key(|(at, low, _)| (*at, *low));
    let mut next = 0;
    let mut one = [0.0f32; 1];
    for n in 0..(seconds * RATE) as usize {
        while next < pending.len() && pending[next].0 == n {
            let (_, low, value) = pending[next];
            if low {
                engine.wheel_lsb(value);
            } else {
                engine.wheel_msb(value);
            }
            next += 1;
        }
        engine.render(&mut one);
        each(engine, n);
    }
}

/// The spread of the pressure over a window, as a share of its mean: the
/// largest departure either side.
fn spread(pressures: &[f64]) -> (f64, f64) {
    let mean = pressures.iter().sum::<f64>() / pressures.len() as f64;
    let worst = pressures
        .iter()
        .map(|p| (p - mean).abs())
        .fold(0.0, f64::max);
    (mean, worst / mean.abs())
}

/// Prediction 1: the wheel heard and still, a key pressed does not sound.
#[test]
fn a_still_wheel_holds_the_bellows_still() {
    let mut engine = engine();
    engine.wheel_msb(64);
    assert_eq!(engine.bellows().source(), BellowsSource::Motion);
    engine.note_on(60, 1.0);
    let mut loudest = 0.0f64;
    let mut peak = 0.0f32;
    let mut one = [0.0f32; 1];
    for _ in 0..2 * RATE_SAMPLES {
        engine.render(&mut one);
        loudest = loudest.max(engine.supply().abs());
        peak = peak.max(one[0].abs());
    }
    println!("still: pressure {loudest} Pa, output {peak}");
    assert_eq!(loudest, 0.0);
    assert_eq!(peak, 0.0);
}

/// One true 8′ held while the wheel steps at 4 a second: the pressure's
/// spread over 1.5-9 s.
fn steady(block: usize, low: Option<usize>) -> (f64, f64) {
    let mut engine = engine();
    let mut messages = steps(20, 1, RATE_SAMPLES / 4, 0.0, 9.5);
    if let Some(delay) = low {
        for message in messages.iter_mut() {
            message.2 = Some((0, delay));
        }
    }
    engine.note_on(60, 1.0);
    let mut pressures = Vec::new();
    run(&mut engine, 9.0, &messages, block, |engine, n| {
        if n >= 3 * RATE_SAMPLES / 2 {
            pressures.push(engine.supply());
        }
    });
    spread(&pressures)
}

/// Prediction 2: a steady 7-bit wheel holds a steady pressure, its steps on
/// their frames or on a host's 256-frame blocks; and with a low half sent in
/// the same frame or a millisecond later.
#[test]
fn a_steady_wheel_holds_a_steady_pressure() {
    for (label, block, low) in [
        ("on their frames", 1, None),
        ("on 256-frame blocks", 256, None),
        ("with the low half in the frame", 1, Some(0)),
        ("with the low half 1 ms later", 1, Some(48)),
    ] {
        let (mean, spread) = steady(block, low);
        println!("{label}: {mean:.1} Pa, ±{:.2} %", spread * 100.0);
        assert!(
            mean < -SPEAKS,
            "{label}: the wheel rising pulls ({mean} Pa)"
        );
        assert!(spread < 0.05, "{label}: ±{:.2} %", spread * 100.0);
    }
}

/// For the record, not predicted: a hand's own unevenness, each interval
/// ±20 % at random, on 256-frame blocks.
#[test]
fn an_uneven_hand_for_the_record() {
    let mut engine = engine();
    let mut seed = 12_345u32;
    let mut random = || {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        f64::from(seed >> 8) / f64::from(1u32 << 24)
    };
    let mut messages = Vec::new();
    let mut at = 0usize;
    for value in 20..60u8 {
        messages.push((at, value, None));
        at += (f64::from(RATE) / 4.0 * (0.8 + 0.4 * random())) as usize;
    }
    engine.note_on(60, 1.0);
    let mut pressures = Vec::new();
    run(&mut engine, 9.0, &messages, 256, |engine, n| {
        if n >= 3 * RATE_SAMPLES / 2 {
            pressures.push(engine.supply());
        }
    });
    let (mean, spread) = spread(&pressures);
    let deviation =
        (pressures.iter().map(|p| (p - mean).powi(2)).sum::<f64>() / pressures.len() as f64).sqrt();
    println!(
        "uneven hand: {mean:.1} Pa, ±{:.1} % at worst, {:.1} % typical",
        spread * 100.0,
        deviation / mean.abs() * 100.0
    );
}

/// Prediction 3: up then down with the note held, the pressure pulls while
/// the wheel rises and pushes while it falls, crossing zero once; the push
/// reed sounds after the turn.
#[test]
fn the_wheel_turning_turns_the_bellows() {
    let mut engine = engine();
    let quarter = RATE_SAMPLES / 4;
    let mut messages = steps(40, 1, quarter, 0.0, 3.0);
    let top = 40 + messages.len() as u8 - 1;
    messages.extend(steps(top - 1, -1, quarter, 3.0 + 0.25, 6.0));
    engine.note_on(60, 1.0);
    let mut crossings = 0;
    let mut last_sign = 0.0;
    let mut pulled = true;
    let mut pushed = true;
    let mut push_peak = 0.0f64;
    run(&mut engine, 6.0, &messages, 1, |engine, n| {
        let p = engine.supply();
        if p != 0.0 {
            if last_sign != 0.0 && p.signum() != last_sign {
                crossings += 1;
            }
            last_sign = p.signum();
        }
        let t = n as f32 / RATE;
        if (0.5..3.0).contains(&t) {
            pulled &= p < 0.0;
        }
        if (4.0..6.0).contains(&t) {
            pushed &= p > 0.0;
            let (_, push) = engine.reed(60, RANK_MIDDLE, PUSH_REED).unwrap();
            push_peak = push_peak.max(push.zeta.abs());
        }
    });
    println!(
        "crossings {crossings}, push reed's swing {:.2} mm",
        push_peak * 1e3
    );
    assert!(pulled, "the pressure pulls while the wheel rises");
    assert!(pushed, "and pushes while it falls");
    assert_eq!(crossings, 1);
    assert!(push_peak > 2.0e-3, "{:.2} mm", push_peak * 1e3);
}

/// The mean pressure a held chord or note keeps at the steady wheel.
fn held(keys: &[u8]) -> f64 {
    let mut engine = engine();
    let messages = steps(20, 1, RATE_SAMPLES / 4, 0.0, 6.0);
    for key in keys {
        engine.note_on(*key, 1.0);
    }
    let mut pressures = Vec::new();
    run(&mut engine, 6.0, &messages, 1, |engine, n| {
        if n >= 2 * RATE_SAMPLES {
            pressures.push(engine.supply().abs());
        }
    });
    pressures.iter().sum::<f64>() / pressures.len() as f64
}

/// Prediction 4: at the same wheel speed a four-note chord holds a
/// pressure at least 6 dB under one note's.
#[test]
fn a_chord_takes_more_air_than_a_note() {
    let note = held(&[60]);
    let chord = held(&[60, 64, 67, 72]);
    let db = 20.0 * (chord / note).log10();
    println!("note {note:.1} Pa, chord {chord:.1} Pa: {db:.1} dB");
    assert!(db <= -6.0, "{db:.1} dB");
}

/// Prediction 5: the wheel stopping, the pressure falls under the reeds'
/// start within 1.5 s of the last step.
#[test]
fn a_wheel_that_stops_lets_the_note_die() {
    let mut engine = engine();
    let messages = steps(20, 1, RATE_SAMPLES / 4, 0.0, 3.0);
    let last = messages.last().unwrap().0;
    engine.note_on(60, 1.0);
    let mut quiet_at = None;
    let mut before = 0.0f64;
    run(&mut engine, 6.0, &messages, 1, |engine, n| {
        let p = engine.supply().abs();
        if n < last {
            before = before.max(p);
        } else if quiet_at.is_none() && p < SPEAKS {
            quiet_at = Some(n);
        }
    });
    let quiet_at = quiet_at.expect("the note never died");
    let seconds = (quiet_at - last) as f32 / RATE;
    println!("{before:.0} Pa while moving; under {SPEAKS} Pa {seconds:.2} s after the last step");
    assert!(before > SPEAKS);
    assert!(seconds <= 1.5, "{seconds:.2} s");
}

/// Prediction 6: the wheel moving with nothing open, the pressure rises,
/// but never above the ceiling.
#[test]
fn a_shut_bellows_stops_at_the_arm() {
    let mut engine = engine();
    let ceiling = engine.parameters().bellows_pressure(1.0);
    let messages = steps(100, -1, RATE_SAMPLES / 20, 0.0, 2.0);
    let mut most = 0.0f64;
    run(&mut engine, 2.0, &messages, 1, |engine, _| {
        most = most.max(engine.supply().abs());
    });
    println!("shut: {most:.0} Pa of {ceiling:.0}");
    assert!(most > 0.5 * ceiling, "{most:.0} Pa");
    assert!(most <= ceiling, "{most:.0} Pa");
}

/// Expression, moved, takes the bellows back from the wheel; setting Mod
/// Wheel back to Pressure gives it to velocity.
#[test]
fn expression_takes_the_bellows_back() {
    let mut engine = engine();
    engine.wheel_msb(64);
    assert_eq!(engine.bellows().source(), BellowsSource::Motion);
    engine.bellows_mut().expression_msb(100);
    assert_eq!(engine.bellows().source(), BellowsSource::Expression);
    engine.wheel_msb(65);
    assert_eq!(engine.bellows().source(), BellowsSource::Motion);
    assert!(engine.set_parameter(parameters::MOD_WHEEL, parameters::WHEEL_PRESSURE));
    assert_eq!(engine.bellows().source(), BellowsSource::Velocity);
    // On Pressure the wheel is the push again.
    engine.wheel_msb(127);
    assert_eq!(engine.bellows().source(), BellowsSource::Expression);
}

/// Diagnosis (2026-10-01): the player heard the sound break up with the
/// wheel moving slowly. The pressure, the reed's swing and the arm's air
/// every 50 ms, at one step a second.
#[test]
#[ignore]
fn diagnose_a_slow_wheel() {
    let mut engine = engine();
    // One step a second, each interval ±30 % at random (RF_JITTER=0 for a
    // machine's).
    let jitter: f64 = std::env::var("RF_JITTER")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(0.3);
    let mut seed = 1u32;
    let mut messages = Vec::new();
    let mut at = 0.1f64;
    for value in 10..30u8 {
        messages.push(((at * f64::from(RATE)) as usize, value, None));
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let random = f64::from(seed >> 8) / f64::from(1u32 << 24);
        at += 1.0 + jitter * (2.0 * random - 1.0);
    }
    engine.note_on(60, 1.0);
    let mut swing = 0.0f64;
    let mut lowest = f64::MAX;
    let mut highest = 0.0f64;
    run(&mut engine, 8.0, &messages, 1, |engine, n| {
        let (_, reed) = engine
            .reed(60, RANK_MIDDLE, rf_musette_dsp::PULL_REED)
            .unwrap();
        swing = swing.max(reed.zeta.abs());
        let p = engine.supply().abs();
        lowest = lowest.min(p);
        highest = highest.max(p);
        if n % (RATE_SAMPLES / 10) == 0 && n > 0 {
            println!(
                "{:5.2} s  {:7.1} Pa (low {:6.1}, high {:6.1})  swing {:5.2} mm  drawn {:6.1} mL/s",
                n as f32 / RATE,
                engine.supply(),
                lowest,
                highest,
                swing * 1e3,
                engine.draw() * 1e6
            );
            swing = 0.0;
            lowest = f64::MAX;
            highest = 0.0;
        }
    });
}

/// The held 8′'s level, dB under its median at its lowest, after its first
/// two seconds, the wheel stepping `rate` a second, each interval ±`jitter`
/// at random. Also the median pressure.
fn lowest_dip(rate: f64, jitter: f64) -> (f64, f64) {
    let mut engine = engine();
    let mut seed = 7u32;
    let mut messages: Vec<Message> = Vec::new();
    let mut at = 0.1f64;
    for value in 10..60u8 {
        messages.push(((at * f64::from(RATE)) as usize, value, None));
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let random = f64::from(seed >> 8) / f64::from(1u32 << 24);
        at += (1.0 + jitter * (2.0 * random - 1.0)) / rate;
    }
    engine.note_on(60, 1.0);
    let window = RATE_SAMPLES / 50;
    let mut energy = 0.0f64;
    let mut levels = Vec::new();
    let mut pressures = Vec::new();
    let mut one = [0.0f32; 1];
    let mut next = 0;
    for n in 0..10 * RATE_SAMPLES {
        while next < messages.len() && messages[next].0 == n {
            engine.wheel_msb(messages[next].1);
            next += 1;
        }
        engine.render(&mut one);
        if n >= 2 * RATE_SAMPLES {
            energy += f64::from(one[0]).powi(2);
            pressures.push(engine.supply().abs());
            if (n + 1 - 2 * RATE_SAMPLES).is_multiple_of(window) {
                levels.push(10.0 * (energy / window as f64 + 1e-30).log10());
                energy = 0.0;
            }
        }
    }
    levels.sort_by(f64::total_cmp);
    pressures.sort_by(f64::total_cmp);
    let median = levels[levels.len() / 2];
    (levels[0] - median, pressures[pressures.len() / 2])
}

/// Prediction 8 (the repair): a slow wheel, steady or uneven, never falls
/// 6 dB under its median.
#[test]
fn a_slow_wheel_holds_its_note() {
    for (rate, jitter) in [(2.0, 0.0), (2.0, 0.3), (1.0, 0.0)] {
        let (dip, pressure) = lowest_dip(rate, jitter);
        println!(
            "{rate} steps a second ±{:.0} %: {pressure:.0} Pa, lowest {dip:.1} dB",
            jitter * 100.0
        );
        assert!(dip > -6.0, "{rate} steps a second ±{jitter}: {dip:.1} dB");
    }
}

/// For the record: one step a second and an uneven hand keep the reed at its
/// threshold, and it stops and starts (docs/ROADMAP.md, 8i).
#[test]
fn a_hand_at_the_threshold_for_the_record() {
    let (dip, pressure) = lowest_dip(1.0, 0.3);
    println!("1 step a second ±30 %: {pressure:.0} Pa, lowest {dip:.1} dB");
}

/// Prediction 9 (the repair): from rest, a wheel at 20 steps a second sounds
/// within 120 ms of its first step.
#[test]
fn a_quick_start_answers_quickly() {
    let mut engine = engine();
    engine.wheel_msb(10);
    engine.note_on(60, 1.0);
    let first = RATE_SAMPLES / 2;
    let messages = steps(11, 1, RATE_SAMPLES / 20, 0.5, 2.0);
    let mut sounding = None;
    run(&mut engine, 2.0, &messages, 1, |engine, n| {
        if sounding.is_none() && n >= first && engine.supply().abs() > SPEAKS {
            sounding = Some(n);
        }
    });
    let ms = (sounding.expect("never sounded") - first) as f64 / f64::from(RATE) * 1e3;
    println!("sounds {ms:.0} ms after the first step");
    assert!(ms <= 120.0, "{ms:.0} ms");
}
