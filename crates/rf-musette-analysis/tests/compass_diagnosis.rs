#![allow(clippy::type_complexity)]
//! Milestone 7's diagnosis, run by hand: `cargo test --release -p
//! rf-musette-analysis --test compass_diagnosis -- --ignored --nocapture`.
//! Prints, for keys across the compass, each rank's reed: its size, its
//! cell's resonance against it, its threshold and whether it speaks at
//! 300 Pa. Asserts nothing.

use rf_musette_analysis::{linear_threshold, sounding};
use rf_musette_dsp::Parameters;
use rf_musette_dsp::compass::untuned;
use rf_musette_dsp::reed::ReedModel;

/// What moves the top reeds' thresholds: each assumed constant scaled in
/// turn, the rest as built.
#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn what_moves_the_top_thresholds() {
    use rf_musette_dsp::reed::ReedDesign;
    let p = Parameters::default();
    for (key, rank, name) in [(93u8, 2usize, "A6 M"), (93, 4, "A6 H"), (81, 2, "A5 M")] {
        let base = untuned(&p, key, rank).unwrap();
        let threshold = |d: ReedDesign| linear_threshold(d, 192_000.0, 1.0, 5000.0);
        println!(
            "{name} ({:.0} Hz): as built {:?} Pa",
            base.frequency,
            threshold(base)
        );
        let variants: [(&str, fn(ReedDesign, f64) -> ReedDesign, [f64; 3]); 7] = [
            (
                "tone hole ×",
                |d, x| ReedDesign {
                    tone_hole_area: d.tone_hole_area * x,
                    ..d
                },
                [0.25, 4.0, 9.0],
            ),
            (
                "cell ×",
                |d, x| ReedDesign {
                    cell_volume: d.cell_volume * x,
                    ..d
                },
                [0.25, 4.0, 16.0],
            ),
            (
                "set ×",
                |d, x| ReedDesign {
                    set: d.set * x,
                    ..d
                },
                [0.25, 0.5, 2.0],
            ),
            (
                "side clearance ×",
                |d, x| ReedDesign {
                    side_clearance: d.side_clearance * x,
                    ..d
                },
                [0.5, 2.0, 4.0],
            ),
            (
                "near field ×",
                |d, x| ReedDesign {
                    inertance_scale: d.inertance_scale * x,
                    ..d
                },
                [0.5, 2.0, 4.0],
            ),
            (
                "Q ×",
                |d, x| ReedDesign { q: d.q * x, ..d },
                [0.4, 2.0, 4.0],
            ),
            (
                "width ×",
                |d, x| ReedDesign {
                    width: d.width * x,
                    ..d
                },
                [0.5, 1.5, 2.0],
            ),
        ];
        for (label, change, factors) in variants {
            let line: Vec<String> = factors
                .iter()
                .map(|x| format!("{x}: {:?}", threshold(change(base, *x)).map(|t| t.round())))
                .collect();
            println!("  {label:<17} {}", line.join(" | "));
        }
    }
}

#[test]
#[ignore = "diagnosis: prints, asserts nothing"]
fn the_compass_reed_by_reed() {
    let p = Parameters::default();
    let names = ["L", "M−", "M", "M+", "H"];
    for key in [53u8, 59, 65, 71, 72, 77, 81, 85, 87, 89, 91, 93] {
        for (rank, name) in names.iter().enumerate() {
            let design = untuned(&p, key, rank).unwrap();
            let model = ReedModel::new(design);
            let cell = 1.0
                / (2.0
                    * core::f64::consts::PI
                    * (model.hole_inertance * model.cell_compliance).sqrt());
            let threshold = linear_threshold(design, 192_000.0, 1.0, 3000.0);
            let speaks = sounding(design, 300.0).map(|t| t.frequency);
            // The same reed with its cell kept at the F4's proportion to it:
            // the resonance 5.87 times the reed's, as builders keep the top
            // cells small and filled (Llanos's luthiers, pp267-268).
            let omega = 5.87 * 2.0 * core::f64::consts::PI * design.frequency;
            let kept = rf_musette_dsp::reed::ReedDesign {
                cell_volume: rf_musette_dsp::reed::AIR_DENSITY
                    * rf_musette_dsp::reed::SPEED_OF_SOUND
                    * rf_musette_dsp::reed::SPEED_OF_SOUND
                    / (model.hole_inertance * omega * omega),
                ..design
            };
            let kept_threshold = linear_threshold(kept, 192_000.0, 1.0, 3000.0);
            println!(
                "key {key} {name:>2}: {:>7.1} Hz, {:>4.1} mm, cell {:>5.2}× | threshold {:>6.0?} Pa | at 300 Pa {:>8.1?} Hz | cell kept at 5.87×: threshold {:>6.0?} Pa",
                design.frequency,
                design.length * 1e3,
                cell / design.frequency,
                threshold,
                speaks,
                kept_threshold
            );
        }
    }
}
