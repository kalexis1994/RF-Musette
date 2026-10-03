//! The click at a note's start, measured before anything is changed:
//! `cargo test --release -p rf-musette-analysis --test click_diagnosis --
//! --ignored --nocapture`. F4 and the 16′ C2 through the engine, the bellows
//! stiff at 300 Pa, the start on and off: the output's largest sample in the
//! first 10 ms against the steady tone's, and the energy above 5 kHz in the
//! first 5 ms against a steady 5 ms. Prints; asserts nothing.

use rf_musette_dsp::Engine;
use rf_musette_dsp::parameters::{self, STIFF};

const RATE: f32 = 48_000.0;

fn onset(note: u8, kick: f64, bass: bool) -> Vec<f64> {
    let mut engine = Engine::new(RATE).unwrap();
    // The bellows played: Key Touch off, or it rests (9h again).
    assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
    assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
    assert!(engine.set_parameter(parameters::ATTACK_KICK, kick));
    engine.bellows_mut().expression_wide(0.3f32.sqrt());
    let mut block = [0.0f32; 64];
    for _ in 0..(0.3 * RATE / 64.0) as usize {
        engine.render(&mut block);
    }
    if bass {
        engine.bass_on(note, 1.0);
    } else {
        engine.note_on(note, 1.0);
    }
    let mut out = vec![0.0f32; (1.5 * RATE) as usize];
    for chunk in out.chunks_mut(64) {
        engine.render(chunk);
    }
    out.into_iter().map(f64::from).collect()
}

/// Energy above 5 kHz in `x[a..b]`, by a first difference twice (a crude
/// high-pass, ~+12 dB/octave), per sample.
fn bright(x: &[f64], a: usize, b: usize) -> f64 {
    let d: Vec<f64> = x[a..b]
        .windows(3)
        .map(|w| w[2] - 2.0 * w[1] + w[0])
        .collect();
    d.iter().map(|v| v * v).sum::<f64>() / d.len() as f64
}

#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_click_at_the_start() {
    for (name, note, bass) in [("F4", 65u8, false), ("16′ C2", 36, true)] {
        for kick in [1.0, 0.0] {
            let x = onset(note, kick, bass);
            let ms = |m: f64| (m * 1e-3 * f64::from(RATE)) as usize;
            let head = x[..ms(10.0)].iter().fold(0.0f64, |m, v| m.max(v.abs()));
            let steady = x[ms(1000.0)..ms(1400.0)]
                .iter()
                .fold(0.0f64, |m, v| m.max(v.abs()));
            let (peak_at, _) =
                x[..ms(10.0)]
                    .iter()
                    .enumerate()
                    .fold(
                        (0, 0.0f64),
                        |(i, m), (j, v)| if v.abs() > m { (j, v.abs()) } else { (i, m) },
                    );
            let onset_bright = (0..ms(60.0))
                .step_by(ms(1.0))
                .map(|a| bright(&x, a, a + ms(5.0)))
                .fold(0.0f64, f64::max);
            let steady_bright = bright(&x, ms(1000.0), ms(1005.0));
            println!(
                "{name:7} kick {kick}: first 10 ms peak {head:.4} at sample {peak_at} (steady peak {steady:.4}); brightest 5 ms in the first 60: {:.1} dB over the steady tone's",
                10.0 * (onset_bright / steady_bright).log10()
            );
            if name == "F4" && kick == 1.0 {
                let first: Vec<String> = x[..24].iter().map(|v| format!("{:.4}", v)).collect();
                println!("  first samples: {}", first.join(" "));
            }
        }
    }
}

/// The onset pulse's shape: F4, the first 3 ms sample by sample, and the
/// level of its spectrum in bands, against the steady tone's.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_onset_pulse() {
    let x = onset(65, 1.0, false);
    let line: Vec<String> = x[..144].iter().map(|v| format!("{:+.4}", v)).collect();
    for row in line.chunks(16) {
        println!("  {}", row.join(" "));
    }
    // Band levels of a 4 ms Hann window around the pulse and of the steady.
    let band = |a: usize| {
        let n = 192;
        let mut levels = Vec::new();
        for f in [
            500.0, 1000.0, 2000.0, 4000.0, 8000.0, 12000.0, 16000.0, 20000.0,
        ] {
            let (mut re, mut im) = (0.0, 0.0);
            for i in 0..n {
                let w = 0.5 - 0.5 * (2.0 * std::f64::consts::PI * i as f64 / n as f64).cos();
                let p = 2.0 * std::f64::consts::PI * f * i as f64 / f64::from(RATE);
                re += w * x[a + i] * p.cos();
                im -= w * x[a + i] * p.sin();
            }
            levels.push(20.0 * ((re * re + im * im).sqrt() + 1e-12).log10());
        }
        levels
    };
    let pulse = band(0);
    let steady = band(48_000);
    for (i, f) in [500, 1000, 2000, 4000, 8000, 12000, 16000, 20000]
        .iter()
        .enumerate()
    {
        println!(
            "  {f:5} Hz: pulse {:6.1} dB, steady {:6.1} dB",
            pulse[i], steady[i]
        );
    }
}
