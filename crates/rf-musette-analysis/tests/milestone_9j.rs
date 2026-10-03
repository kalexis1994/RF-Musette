//! Milestone 9j: Key Touch in the left hand -- a bass button's, a chord
//! button's and a free-bass note's velocity setting how far it goes down.
//! Predictions as written in docs/ROADMAP.md before the code; each test says
//! which. Musette Paris, Pallet Opening 35 ms, the bellows resting at 300 Pa,
//! unless a test says otherwise.

use rf_musette_dsp::parameters::{self, BASS_REGISTER, BASS_SYSTEM, FREE_BASS, KEY_TOUCH};
use rf_musette_dsp::{BASS_CHANNEL, CHORD_CHANNEL, Engine, programs};

const RATE: f32 = 48_000.0;

/// What the left hand plays: a bass button, a chord button or a free-bass
/// note, by MIDI key.
#[derive(Clone, Copy, Debug)]
enum Button {
    Bass(u8),
    Chord(u8),
    Free(u8),
}

fn engine(touch: bool, register: f64, free: bool) -> Box<Engine> {
    let mut engine = Box::new(Engine::new(RATE).unwrap());
    let values = programs::program("musette-paris").unwrap().parameters();
    for (index, value) in values.values().iter().enumerate() {
        assert!(engine.set_parameter(index, *value));
    }
    assert!(engine.set_parameter(parameters::PALLET_OPENING, 35.0));
    assert!(engine.set_parameter(KEY_TOUCH, if touch { 1.0 } else { 0.0 }));
    assert!(engine.set_parameter(BASS_REGISTER, register));
    if free {
        assert!(engine.set_parameter(BASS_SYSTEM, FREE_BASS));
    }
    engine
}

/// `button` struck at `velocity` and held 2 s, the instrument alone.
fn render(button: Button, velocity: f32, touch: bool, register: f64) -> Vec<f32> {
    let mut engine = engine(touch, register, matches!(button, Button::Free(_)));
    let mut block = [0.0f32; 256];
    for _ in 0..(0.2 * RATE / 256.0) as usize {
        engine.render(&mut block);
    }
    match button {
        Button::Bass(key) | Button::Free(key) => {
            engine.channel_note_on(BASS_CHANNEL, key, velocity)
        }
        Button::Chord(key) => engine.channel_note_on(CHORD_CHANNEL, key, velocity),
    }
    let mut out = Vec::new();
    for _ in 0..(2.0 * RATE / 256.0) as usize {
        engine.render(&mut block);
        out.extend_from_slice(&block);
    }
    out
}

fn level(samples: &[f32]) -> f64 {
    let power = samples.iter().map(|x| f64::from(*x).powi(2)).sum::<f64>() / samples.len() as f64;
    10.0 * (power + 1e-300).log10()
}

/// The held tone's level, when it is a steady tone: its last two halves
/// second within 3 dB, as 9h judges the treble (a pallet at its edge dies
/// away).
fn steady(samples: &[f32]) -> Option<f64> {
    let half = (0.5 * RATE) as usize;
    let n = samples.len();
    let (early, late) = (
        level(&samples[n - 2 * half..n - half]),
        level(&samples[n - half..]),
    );
    ((early - late).abs() < 3.0 && late > -120.0).then_some(late)
}

/// The default bass register, all five ranks.
const ALL_RANKS: f64 = 3.0;

/// 9j, prediction 1: velocity 127 renders exactly what Key Touch off
/// renders, for each kind of button.
#[test]
fn full_velocity_is_the_button_fully_down() {
    for button in [Button::Bass(48), Button::Chord(48), Button::Free(48)] {
        assert_eq!(
            render(button, 1.0, true, ALL_RANKS),
            render(button, 1.0, false, ALL_RANKS),
            "{button:?}"
        );
    }
}

/// 9j, prediction 2: every bass and chord button, in every bass register,
/// and every free-bass note, that holds a steady tone fully down still holds
/// one at velocity 1.
#[test]
fn the_softest_button_still_sounds() {
    let mut silent = Vec::new();
    let mut check = |button: Button, register: f64| {
        if steady(&render(button, 1.0, true, register)).is_none() {
            return;
        }
        if steady(&render(button, 1.0 / 127.0, true, register)).is_none() {
            silent.push(format!("{button:?} in bass register {register}"));
        }
    };
    for register in 0..7 {
        for pitch_class in 0..12u8 {
            check(Button::Bass(48 + pitch_class), f64::from(register));
            check(Button::Chord(48 + pitch_class), f64::from(register));
        }
    }
    for note in rf_musette_dsp::compass::FREE_FIRST
        ..rf_musette_dsp::compass::FREE_FIRST + rf_musette_dsp::compass::FREE_NOTES as u8
    {
        check(Button::Free(note), ALL_RANKS);
    }
    assert!(silent.is_empty(), "silent at velocity 1: {silent:?}");
}

/// 9j, prediction 3: velocity 1 against 127 at 300 Pa, a C bass button in
/// the default register falls 8 dB or more, a C chord 6 or more, a free-bass
/// C3 8 or more.
#[test]
fn each_button_reaches_its_floor() {
    let mut short = Vec::new();
    for (button, least) in [
        (Button::Bass(48), 8.0),
        (Button::Chord(48), 6.0),
        (Button::Free(48), 8.0),
    ] {
        let full = steady(&render(button, 1.0, true, ALL_RANKS)).expect("a steady tone");
        let middle = steady(&render(button, 64.0 / 127.0, true, ALL_RANKS)).expect("a steady tone");
        let soft = steady(&render(button, 1.0 / 127.0, true, ALL_RANKS)).expect("a steady tone");
        println!(
            "{button:?}: velocity 64 {:+.1} dB, velocity 1 {:+.1} dB",
            middle - full,
            soft - full
        );
        if full - soft < least {
            short.push(format!("{button:?}: {:.1} dB, asked {least}", full - soft));
        }
    }
    assert!(short.is_empty(), "{short:?}");
}
