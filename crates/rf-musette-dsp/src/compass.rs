//! The treble's compass: which keys have reeds, what each rank's reed of a
//! key sounds, and what that reed is made of.
//!
//! A full-size piano accordion's 41 keys, F3-A6 (MIDI 53-93) on the 8′
//! ranks; the 16′ sounds an octave below, the 4′ an octave above. Every
//! true 8′ is tuned to equal temperament on "Pitch A4" (decided
//! 2026-10-01: 440 Hz, the IfM Zwota's reference), where it sounds at
//! 300 Pa, as a tuner tunes; the tremolo's ranks the beat away; L and H the
//! octaves.
//!
//! Only the F4 tongue is measured (Ziegenhals 2009). Every other reed is it,
//! scaled to its pitch by a bayan maker's slots (patent RU2233009, Table 3)
//! -- **assumed**, as no maker publishes tongue dimensions. Between the
//! patent's notes the ratios are interpolated in log-frequency; beyond them
//! extrapolated with the nearest octave's. The set scales with the length;
//! the cell's volume with the length cubed and its hole with the slot's
//! area, keeping their proportions to the reed; the clearances are machined
//! and stay as specified. Q follows the measured rise with pitch from the
//! F4's (parameter "Q Slope"), extrapolated above ~750 Hz.

use crate::math;
use crate::parameters::{self, Parameters, RANK_FLAT, RANK_HIGH, RANK_LOW, RANK_SHARP, RANKS};
use crate::reed::ReedDesign;

/// The lowest key with reeds, F3.
pub const FIRST_KEY: u8 = 53;
/// Keys with reeds: F3-A6.
pub const KEYS: usize = 41;

/// The patent's notes, as their equal-tempered frequencies at A = 440 Hz,
/// with the slot's length, root width and plate thickness, mm (RU2233009,
/// Table 3, the first F3 row).
const SLOTS: [(f64, f64, f64, f64); 5] = [
    (174.614, 35.4, 4.23, 2.7),
    (349.228, 27.8, 3.37, 2.7),
    (698.456, 20.5, 2.66, 2.2),
    (1396.913, 14.7, 2.00, 2.2),
    (2793.826, 9.6, 1.25, 1.7),
];
/// The patent's F4, to which the measured F4 is the anchor.
const ANCHOR: usize = 1;

/// The patent's (length, width, plate) at `frequency`, against its F4.
fn ratios(frequency: f64) -> (f64, f64, f64) {
    let octave = |f: f64| math::ln(f) / core::f64::consts::LN_2;
    let x = octave(frequency);
    // The segment that holds x, or the nearest one beyond the table's ends.
    let segment = (0..SLOTS.len() - 1)
        .find(|&i| x <= octave(SLOTS[i + 1].0))
        .unwrap_or(SLOTS.len() - 2);
    let (a, b) = (SLOTS[segment], SLOTS[segment + 1]);
    let t = (x - octave(a.0)) / (octave(b.0) - octave(a.0));
    // Log-linear between the two, and on along the same line beyond them.
    let between = |p: f64, q: f64| math::exp(math::ln(p) + t * (math::ln(q) - math::ln(p)));
    let anchor = SLOTS[ANCHOR];
    (
        between(a.1, b.1) / anchor.1,
        between(a.2, b.2) / anchor.2,
        between(a.3, b.3) / anchor.3,
    )
}

/// Equal temperament on `a4`, Hz.
pub fn pitch(key: u8, a4: f64) -> f64 {
    a4 * math::pow(2.0, (f64::from(key) - 69.0) / 12.0)
}

/// Where a rank's reed of a key should sound, Hz: the 8′ on the key's
/// pitch, the tremolo's ranks the measured beats away, the 16′ and 4′ the
/// octaves.
pub fn target(parameters: &Parameters, key: u8, rank: usize) -> f64 {
    let a4 = parameters.get(parameters::PITCH_A4).unwrap_or(440.0);
    let true_eight = pitch(key, a4);
    let (sharp, flat) = parameters.tremolo_beats_at(true_eight);
    match rank {
        RANK_LOW => true_eight / 2.0,
        RANK_FLAT => true_eight + flat,
        RANK_SHARP => true_eight + sharp,
        RANK_HIGH => true_eight * 2.0,
        _ => true_eight,
    }
}

/// A rank's reed of a key, before tuning: the measured F4's geometry scaled
/// to the pitch it is made for (the tremolo's ranks are the true 8′ tongue,
/// retuned), its mode on its target. `None` outside the compass.
pub fn untuned(parameters: &Parameters, key: u8, rank: usize) -> Option<ReedDesign> {
    let index = key.checked_sub(FIRST_KEY).map(usize::from)?;
    if index >= KEYS || rank >= RANKS {
        return None;
    }
    let measured = parameters.reed_design();
    // The pitch the tongue is made for, at the patent's A = 440 Hz: another
    // A retunes the same tongues.
    let made_for = match rank {
        RANK_LOW => pitch(key, 440.0) / 2.0,
        RANK_HIGH => pitch(key, 440.0) * 2.0,
        _ => pitch(key, 440.0),
    };
    let (length, width, plate) = ratios(made_for);
    let cube = length * length * length;
    // Q follows the measured trend from the F4's (parameter "Q Slope").
    let frequency = target(parameters, key, rank);
    let slope = parameters.get(parameters::Q_SLOPE).unwrap_or(0.0);
    let q = measured.q * math::pow(frequency / SLOTS[ANCHOR].0, slope);
    Some(ReedDesign {
        frequency,
        q,
        length: measured.length * length,
        width: measured.width * width,
        set: measured.set * length,
        plate_thickness: measured.plate_thickness * plate,
        cell_volume: measured.cell_volume * cube,
        tone_hole_area: measured.tone_hole_area * length * width,
        ..measured
    })
}

/// A rank's reed of a key as built: [`untuned`], its mode moved by the
/// tuning table so it sounds on its target at 300 Pa.
pub fn design(parameters: &Parameters, key: u8, rank: usize) -> Option<ReedDesign> {
    let mut design = untuned(parameters, key, rank)?;
    let cents = f64::from(crate::tuning::CENTS[rank][usize::from(key - FIRST_KEY)]);
    design.frequency *= math::pow(2.0, cents / 1200.0);
    Some(design)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameters::RANK_MIDDLE;

    #[test]
    fn the_anchor_is_the_measured_reed() {
        let (l, w, p) = ratios(349.228);
        assert!((l - 1.0).abs() < 1e-6 && (w - 1.0).abs() < 1e-6 && (p - 1.0).abs() < 1e-6);
        let p = Parameters::default();
        let f4 = untuned(&p, 65, RANK_MIDDLE).unwrap();
        assert!((f4.length / p.reed_design().length - 1.0).abs() < 1e-6);
        assert!((f4.frequency - 349.228).abs() < 0.01);
    }

    #[test]
    fn the_patents_notes_are_met_and_the_ends_extend() {
        for (f, length, width, plate) in SLOTS {
            let (l, w, p) = ratios(f);
            assert!((l * 27.8 / length - 1.0).abs() < 1e-6, "{f}");
            assert!((w * 3.37 / width - 1.0).abs() < 1e-6, "{f}");
            assert!((p * 2.7 / plate - 1.0).abs() < 1e-6, "{f}");
        }
        // An octave below F3 goes on as F3 against F4: ×35.4/27.8 again.
        let (l, _, _) = ratios(87.307);
        assert!((l / ((35.4 / 27.8) * (35.4 / 27.8)) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn only_the_compass_has_reeds() {
        let p = Parameters::default();
        assert!(untuned(&p, 52, RANK_MIDDLE).is_none());
        assert!(untuned(&p, 53, RANK_MIDDLE).is_some());
        assert!(untuned(&p, 93, RANK_HIGH).is_some());
        assert!(untuned(&p, 94, RANK_MIDDLE).is_none());
    }
}
