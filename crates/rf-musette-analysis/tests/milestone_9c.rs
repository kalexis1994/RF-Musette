//! Milestone 9c: headroom. Predictions as written in docs/ROADMAP.md
//! before the code; each test says which.

use rf_musette_dsp::parameters;
use rf_musette_dsp::stage;
use rf_musette_dsp::{BASS_CHANNEL, Engine, ceiling};

const RATE: f32 = 48_000.0;

/// The peak, dBFS, of four seconds through the ORTF pair: the bellows
/// played at `intent`, Key Touch off; or, `None`, Key Touch on and the
/// bellows resting.
fn peak(register: f64, intent: Option<f32>, treble: &[u8], bass: &[u8]) -> f64 {
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(parameters::KEY_TOUCH, f64::from(u8::from(intent.is_none()))));
    assert!(engine.set_parameter(parameters::REGISTER, register));
    assert!(engine.set_parameter(parameters::MIC_LAYOUT, stage::ORTF as f64));
    if let Some(intent) = intent {
        engine.bellows_mut().expression_wide(intent);
    }
    for key in treble {
        engine.note_on(*key, 1.0);
    }
    for key in bass {
        engine.channel_note_on(BASS_CHANNEL, *key, 1.0);
    }
    let mut left = [0.0f32; 256];
    let mut right = [0.0f32; 256];
    let mut peak = 0.0f32;
    for _ in 0..(4.0 * RATE / 256.0) as usize {
        engine.render_stereo(&mut left, &mut right);
        for (l, r) in left.iter().zip(&right) {
            peak = peak.max(l.abs()).max(r.abs());
        }
    }
    20.0 * f64::from(peak).log10()
}

/// 9c again, predictions 2 and 3: the loudest case, both hands' full chords
/// in Master at the bellows' ceiling, meets the soft ceiling and stays under
/// full scale; a C4 at mf near -18 dBFS. (First: the loudest at -6 dBFS,
/// the C4 near -25.)
#[test]
fn the_loudest_meets_the_ceiling() {
    let loudest = peak(6.0, Some(1.0), &[55, 60, 64, 67, 72, 76], &[48, 43]);
    let note = peak(11.0, Some(0.7), &[60], &[]);
    println!("loudest {loudest:+.1} dBFS, a C4 at mf {note:+.1} dBFS");
    assert!(loudest > -6.0 && loudest < 0.0, "{loudest:+.1} dBFS");
    assert!((note + 18.0).abs() <= 3.0, "{note:+.1} dBFS");
}

/// The player's playing -- Musette Paris, Pallet Opening 35 ms, a held A4
/// over F2 and its chord -- its peak, dBFS: the wheel at 79 % (CC 1 = 100)
/// with Key Touch off, the bellows resting with it on.
fn players_playing(key_touch: bool) -> f64 {
    use rf_musette_dsp::{CHORD_CHANNEL, programs};
    let mut engine = Engine::new(RATE).unwrap();
    let values = programs::program("musette-paris").unwrap().parameters();
    for (index, value) in values.values().iter().enumerate() {
        assert!(engine.set_parameter(index, *value));
    }
    assert!(engine.set_parameter(parameters::KEY_TOUCH, f64::from(u8::from(key_touch))));
    assert!(engine.set_parameter(parameters::PALLET_OPENING, 35.0));
    engine.wheel_msb(100);
    engine.note_on(69, 1.0);
    engine.channel_note_on(BASS_CHANNEL, 41, 1.0);
    engine.channel_note_on(CHORD_CHANNEL, 53, 1.0);
    let mut left = [0.0f32; 256];
    let mut right = [0.0f32; 256];
    let mut peak = 0.0f32;
    for _ in 0..(4.0 * RATE / 256.0) as usize {
        engine.render_stereo(&mut left, &mut right);
        for (l, r) in left.iter().zip(&right) {
            peak = peak.max(l.abs()).max(r.abs());
        }
    }
    20.0 * f64::from(peak).log10()
}

/// 9c again, prediction 1: the player's playing, the wheel at 79 %, peaks
/// at -6 dBFS within 1.5 dB (since the tuning as played: -4.9).
#[test]
fn the_players_playing_peaks_with_headroom() {
    let peak = players_playing(false);
    println!("the player's playing {peak:+.1} dBFS");
    assert!((peak + 6.0).abs() <= 1.5, "{peak:+.1} dBFS");
}

/// 9c for Key Touch, prediction 1: with Key Touch on, the same playing
/// peaks at -6 dBFS within 0.5 dB (at the bellows' level: -9.3).
#[test]
fn with_key_touch_the_playing_peaks_at_the_same_headroom() {
    let peak = players_playing(true);
    println!("the player's playing, Key Touch on, {peak:+.1} dBFS");
    assert!((peak + 6.0).abs() <= 0.5, "{peak:+.1} dBFS");
}

/// 9c for Key Touch, prediction 3: with Key Touch on, the loudest it can
/// play -- both hands' full chords in Master, the bellows resting -- stays
/// under full scale.
#[test]
fn with_key_touch_the_loudest_stays_under_full_scale() {
    let loudest = peak(6.0, None, &[55, 60, 64, 67, 72, 76], &[48, 43]);
    println!("loudest with Key Touch on {loudest:+.1} dBFS");
    assert!(loudest < 0.0, "{loudest:+.1} dBFS");
}

/// Prediction 2: under -6 dBFS the ceiling changes no sample; nothing it
/// puts out reaches full scale.
#[test]
fn the_ceiling_leaves_the_music_and_rounds_the_peaks() {
    for n in 0..=5000 {
        let x = n as f32 / 10_000.0;
        assert_eq!(ceiling(x), x);
        assert_eq!(ceiling(-x), -x);
    }
    let mut previous = 0.5;
    for n in 1..=1000 {
        let x = 0.5 + n as f32 * 0.02;
        let y = ceiling(x);
        assert!(y < 1.0 && y >= previous, "{x}: {y}");
        assert_eq!(ceiling(-x), -y);
        previous = y;
    }
    assert_eq!(ceiling(f32::NAN), 0.0);
    assert_eq!(ceiling(f32::INFINITY), 0.0);
}
