//! Milestone 9c: headroom. Predictions as written in docs/ROADMAP.md
//! before the code; each test says which.

use rf_musette_dsp::parameters;
use rf_musette_dsp::stage;
use rf_musette_dsp::{BASS_CHANNEL, Engine, ceiling};

const RATE: f32 = 48_000.0;

/// The peak, dBFS, of four seconds through the ORTF pair.
fn peak(register: f64, intent: f32, treble: &[u8], bass: &[u8]) -> f64 {
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(parameters::REGISTER, register));
    assert!(engine.set_parameter(parameters::MIC_LAYOUT, stage::ORTF as f64));
    engine.bellows_mut().expression_wide(intent);
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

/// Prediction 1: the loudest case peaks at -6 dBFS within 1 dB; a C4 at
/// mf near -25.
#[test]
fn the_loudest_peaks_with_headroom() {
    let loudest = peak(6.0, 1.0, &[55, 60, 64, 67, 72, 76], &[48, 43]);
    let note = peak(11.0, 0.7, &[60], &[]);
    println!("loudest {loudest:+.1} dBFS, a C4 at mf {note:+.1} dBFS");
    assert!((loudest + 6.0).abs() <= 1.0, "{loudest:+.1} dBFS");
    assert!((note + 25.0).abs() <= 3.0, "{note:+.1} dBFS");
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
