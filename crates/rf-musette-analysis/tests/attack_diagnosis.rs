//! The finger attack, measured before anything is changed: `cargo test
//! --release -p rf-musette-analysis --test attack_diagnosis -- --ignored
//! --nocapture`. The bellows already pressed, the key goes down, and the
//! first harmonic rises from −50 to −5 dB (Llanos-Vázquez et al. 2014's
//! metric; they measured 50-110 ms at mf and 60-140 ms at p, with no trend in
//! pitch). Prints; asserts nothing.

#![allow(clippy::type_complexity)]

use rf_musette_analysis::{attack_time, component_envelope, linear_threshold, simulate_keyed};
use rf_musette_dsp::compass::design;
use rf_musette_dsp::pallet::PalletDesign;
use rf_musette_dsp::parameters::{Parameters, RANK_MIDDLE};
use rf_musette_dsp::reed::ReedDesign;

const RATE: f64 = 96_000.0;

/// The finger attack at `pressure`, s, the key going down at 50 ms.
fn attack(reed: ReedDesign, pallet: PalletDesign, pressure: f64) -> Option<f64> {
    let seconds = 2.0;
    let trace = simulate_keyed(reed, pallet, RATE, seconds, pressure, |t| {
        if t >= 0.05 { 1.0 } else { 0.0 }
    });
    let frequency = trace.tone(seconds - 0.5, seconds)?.frequency;
    let envelope = component_envelope(&trace.flow_rate, RATE, frequency, 4.0, 0.001);
    attack_time(&envelope)
}

fn line(reed: ReedDesign, pallet: PalletDesign) -> String {
    let ms = |a: Option<f64>| a.map_or("  --".to_owned(), |a| format!("{:4.0}", a * 1e3));
    format!(
        "p (100 Pa) {} ms, mf (400 Pa) {} ms, threshold {:>4.0?} Pa",
        ms(attack(reed, pallet, 100.0)),
        ms(attack(reed, pallet, 400.0)),
        linear_threshold(reed, 192_000.0, 1.0, 3000.0)
    )
}

#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_finger_attack_across_the_compass_and_its_levers() {
    let p = Parameters::default();
    let pallet = p.pallet_design();
    println!("true 8′ across the compass:");
    for key in [53u8, 59, 65, 69, 77, 81, 89, 93] {
        let reed = design(&p, key, RANK_MIDDLE).unwrap();
        println!(
            "  key {key} ({:6.1} Hz): {}",
            reed.frequency,
            line(reed, pallet)
        );
    }
    let f4 = design(&p, 65, RANK_MIDDLE).unwrap();
    println!("F4, one assumed constant moved at a time:");
    let levers: [(&str, fn(ReedDesign, f64) -> ReedDesign, [f64; 2]); 7] = [
        (
            "tone hole area ×",
            |d, x| ReedDesign {
                tone_hole_area: d.tone_hole_area * x,
                ..d
            },
            [0.25, 0.5],
        ),
        (
            "tone hole depth ×",
            |d, x| ReedDesign {
                tone_hole_depth: d.tone_hole_depth * x,
                ..d
            },
            [2.0, 4.0],
        ),
        (
            "cell volume ×",
            |d, x| ReedDesign {
                cell_volume: d.cell_volume * x,
                ..d
            },
            [0.25, 4.0],
        ),
        (
            "near field ×",
            |d, x| ReedDesign {
                inertance_scale: d.inertance_scale * x,
                ..d
            },
            [2.0, 4.0],
        ),
        (
            "set ×",
            |d, x| ReedDesign {
                set: d.set * x,
                ..d
            },
            [0.5, 2.0],
        ),
        (
            "side clearance ×",
            |d, x| ReedDesign {
                side_clearance: d.side_clearance * x,
                ..d
            },
            [0.5, 2.0],
        ),
        ("Q ×", |d, x| ReedDesign { q: d.q * x, ..d }, [0.5, 2.0]),
    ];
    println!("  as built: {}", line(f4, pallet));
    for (label, change, factors) in levers {
        for x in factors {
            println!("  {label:<18}{x:<5}: {}", line(change(f4, x), pallet));
        }
    }
    for opening in [0.005, 0.02] {
        let fast = PalletDesign {
            opening_time: opening,
            ..pallet
        };
        println!(
            "  pallet opening {:>3.0} ms : {}",
            opening * 1e3,
            line(f4, fast)
        );
    }
}
