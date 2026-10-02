//! Milestone 8b: the left hand on one keyboard. Predictions as written in
//! docs/ROADMAP.md before the code; each test says which.

use rf_musette_dsp::compass::BASS_KEYS;
use rf_musette_dsp::parameters::{self, BASS_8_4, BASS_16, BASS_RANKS, STIFF};
use rf_musette_dsp::reed::ReedState;
use rf_musette_dsp::{Engine, PULL_REED};

const RATE: f32 = 48_000.0;

/// On the heap, as the plugin keeps it: a test here builds several, and two
/// 1.2 MiB engines by value with the render's frames overflowed even the
/// 8 MiB test threads (.cargo/config.toml).
fn engine() -> Box<Engine> {
    let mut engine = Box::new(Engine::new(RATE).unwrap());
    assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
    engine.bellows_mut().expression_wide(0.3f32.sqrt());
    render(&mut engine, 0.1);
    engine
}

fn render(engine: &mut Engine, seconds: f32) {
    let mut block = [0.0f32; 256];
    for _ in 0..(seconds * RATE / 256.0) as usize {
        engine.render(&mut block);
    }
}

/// Which bass-side reeds move, as (pitch class, rank).
fn moving(engine: &Engine) -> Vec<(usize, usize)> {
    let mut moving = Vec::new();
    for pitch_class in 0..BASS_KEYS {
        for rank in 0..BASS_RANKS {
            if engine
                .bass_reed(pitch_class, rank, PULL_REED)
                .is_some_and(|(_, state)| *state != ReedState::default())
            {
                moving.push((pitch_class, rank));
            }
        }
    }
    moving
}

/// Prediction 1: on channel 1, Left Hand on, F3 and up play the treble, F2-E3
/// the chord ranks, E2 and below the bass buttons.
#[test]
fn the_keyboard_splits_into_three() {
    let mut engine = engine();
    engine.channel_note_on(0, 53, 1.0);
    assert!(engine.is_held(53), "F3 is the treble's");
    engine.channel_note_off(0, 53);

    let mut engine = self::engine();
    // E3: the chord octave's top, E on the chord ranks only.
    engine.channel_note_on(0, 52, 1.0);
    render(&mut engine, 0.2);
    assert!(!engine.is_held(52));
    let moved = moving(&engine);
    assert!(
        moved.iter().all(|(pc, rank)| *pc == 4 && *rank >= BASS_8_4),
        "{moved:?}"
    );
    assert!(!moved.is_empty());

    let mut engine = self::engine();
    // F2 is still the chord octave's.
    engine.channel_note_on(0, 41, 1.0);
    render(&mut engine, 0.2);
    let moved = moving(&engine);
    assert!(
        moved.iter().all(|(pc, rank)| *pc == 5 && *rank >= BASS_8_4),
        "{moved:?}"
    );

    let mut engine = self::engine();
    // E2: a bass button, every rank.
    engine.channel_note_on(0, 40, 1.0);
    render(&mut engine, 0.2);
    assert_eq!(
        moving(&engine),
        (0..BASS_RANKS).map(|rank| (4, rank)).collect::<Vec<_>>()
    );
}

/// Prediction 2: a note held while the split moves is let go where it was
/// played.
#[test]
fn a_note_is_let_go_where_it_was_played() {
    let mut engine = engine();
    engine.channel_note_on(0, 40, 1.0);
    // The split goes down to C2: E2 would now be the treble's.
    assert!(engine.set_parameter(parameters::SPLIT_POINT, 36.0));
    engine.channel_note_off(0, 40);
    render(&mut engine, 3.0);
    assert!(
        engine
            .bass_reed(4, BASS_16, PULL_REED)
            .is_some_and(|(_, state)| *state == ReedState::default()),
        "the E bass button stayed open"
    );
}

/// Prediction 3: Left Hand off, channel 1 under F3 is silent.
#[test]
fn with_the_left_hand_off_the_low_keys_are_silent() {
    let mut engine = engine();
    assert!(engine.set_parameter(parameters::LEFT_HAND, 0.0));
    engine.channel_note_on(0, 40, 1.0);
    engine.channel_note_on(0, 50, 1.0);
    let mut out = [1.0f32; 9600];
    engine.render(&mut out);
    assert!(out.iter().all(|x| *x == 0.0));
    assert!(moving(&engine).is_empty());
}

/// Prediction 4: channels 2 and 3 are as they were, whatever the split.
#[test]
fn the_bass_and_chord_channels_are_unchanged() {
    let mut engine = engine();
    assert!(engine.set_parameter(parameters::SPLIT_POINT, 96.0));
    engine.channel_note_on(1, 60, 1.0);
    render(&mut engine, 0.2);
    assert_eq!(
        moving(&engine),
        (0..BASS_RANKS).map(|rank| (0, rank)).collect::<Vec<_>>()
    );
}
