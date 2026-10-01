//! A knob's travel and its readout: where a value sits on the knob's sweep,
//! by the parameter's taper, and how the panel prints it.

use rf_musette_dsp::parameters::{ParameterSpec, Taper};

/// The knob turns this far either side of top, degrees.
pub const SWEEP: f64 = 135.0;

/// The pointer's travel over the whole range, CSS pixels (RF-5's).
pub const DRAG_SPAN: f64 = 180.0;

/// The native range input under a knob runs over positions, not values, so
/// a screen reader or a host's own keys move it along the taper too.
pub const INPUT_SPAN: f64 = 1000.0;

/// An arrow key's share of the travel, and Page Up/Down's.
const ARROW: f64 = 0.01;
const PAGE: f64 = 0.1;

fn logarithmic(spec: &ParameterSpec) -> bool {
    spec.taper == Taper::Logarithmic && spec.minimum > 0.0
}

/// How many decimals the parameter's step shows.
pub fn decimals(step: f64) -> usize {
    let mut scaled = step;
    let mut places = 0;
    while places < 6 && scaled > 0.0 && (scaled - scaled.round()).abs() > 1.0e-9 {
        scaled *= 10.0;
        places += 1;
    }
    places
}

/// The value on its step and in range, without the step's binary residue.
pub fn quantise(spec: &ParameterSpec, value: f64) -> f64 {
    if !value.is_finite() {
        return spec.default;
    }
    let stepped = if spec.step > 0.0 {
        spec.minimum + ((value - spec.minimum) / spec.step).round() * spec.step
    } else {
        value
    };
    let scale = 10f64.powi(decimals(spec.step) as i32 + 3);
    ((stepped * scale).round() / scale).clamp(spec.minimum, spec.maximum)
}

/// Where the value sits along the knob's travel, 0 to 1.
pub fn position(spec: &ParameterSpec, value: f64) -> f64 {
    if !value.is_finite() || spec.maximum <= spec.minimum {
        return 0.0;
    }
    let value = value.clamp(spec.minimum, spec.maximum);
    let position = if logarithmic(spec) {
        (value / spec.minimum).ln() / (spec.maximum / spec.minimum).ln()
    } else {
        (value - spec.minimum) / (spec.maximum - spec.minimum)
    };
    position.clamp(0.0, 1.0)
}

/// The value at a place along the travel, on its step.
pub fn value_at(spec: &ParameterSpec, position: f64) -> f64 {
    let position = if position.is_finite() {
        position.clamp(0.0, 1.0)
    } else {
        0.0
    };
    let raw = if logarithmic(spec) {
        spec.minimum * (spec.maximum / spec.minimum).powf(position)
    } else {
        spec.minimum + position * (spec.maximum - spec.minimum)
    };
    quantise(spec, raw)
}

/// The knob's angle for the value, degrees from top.
pub fn angle(spec: &ParameterSpec, value: f64) -> f64 {
    -SWEEP + 2.0 * SWEEP * position(spec, value)
}

/// The value a drag reaches: `rise` pixels up from where it started.
pub fn dragged(spec: &ParameterSpec, start: f64, rise: f64) -> f64 {
    if !rise.is_finite() {
        return quantise(spec, start);
    }
    value_at(spec, position(spec, start) + rise / DRAG_SPAN)
}

/// The value a key moves the knob to: arrows a hundredth of the travel,
/// pages a tenth, and always at least one step.
pub fn nudged(spec: &ParameterSpec, current: f64, key: &str) -> Option<f64> {
    let travel = match key {
        "ArrowUp" | "ArrowRight" => ARROW,
        "ArrowDown" | "ArrowLeft" => -ARROW,
        "PageUp" => PAGE,
        "PageDown" => -PAGE,
        "Home" => return Some(spec.minimum),
        "End" => return Some(spec.maximum),
        _ => return None,
    };
    let current = quantise(spec, current);
    let moved = value_at(spec, position(spec, current) + travel);
    if moved == current && spec.step > 0.0 {
        return Some(quantise(spec, current + travel.signum() * spec.step));
    }
    Some(moved)
}

const NOTE_NAMES: [&str; 12] = [
    "C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B",
];

/// A MIDI note's name, middle C (60) as C4.
pub fn note_name(note: f64) -> String {
    let note = note.round() as i32;
    format!(
        "{}{}",
        NOTE_NAMES[note.rem_euclid(12) as usize],
        note.div_euclid(12) - 1
    )
}

/// The choice's name for a value, or `None` for a continuous parameter.
pub fn choice_name(spec: &ParameterSpec, value: f64) -> Option<&'static str> {
    spec.choices
        .iter()
        .min_by(|a, b| {
            (f64::from(a.0) - value)
                .abs()
                .total_cmp(&(f64::from(b.0) - value).abs())
        })
        .map(|(_, name)| *name)
}

/// What the panel prints under a control.
pub fn readout(spec: &ParameterSpec, value: f64) -> String {
    if let Some(name) = choice_name(spec, value) {
        return name.to_owned();
    }
    if spec.unit == "note" {
        return note_name(value);
    }
    let places = decimals(spec.step);
    let number = format!("{:.*}", places, quantise(spec, value));
    match spec.unit {
        "" => number,
        "x" => format!("{number}×"),
        unit => format!("{number} {unit}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rf_musette_dsp::parameters::{
        BELLOWS_TRAVEL, GAIN, REED_FREQUENCY, REGISTER, SIDE_CLEARANCE, SPECS, SPLIT_POINT,
    };

    fn knobs() -> impl Iterator<Item = &'static ParameterSpec> {
        SPECS.iter().filter(|spec| spec.choices.is_empty())
    }

    /// Prediction 3: on a logarithmic knob equal travel is an equal ratio,
    /// and its middle is the geometric mean.
    #[test]
    fn a_logarithmic_knob_turns_by_ratios() {
        for spec in knobs().filter(|spec| spec.taper == Taper::Logarithmic) {
            let middle = (spec.minimum * spec.maximum).sqrt();
            assert!(
                (value_at(spec, 0.5) - middle).abs() <= spec.step,
                "{}: {} against {middle}",
                spec.id,
                value_at(spec, 0.5)
            );
            // The first quarter of the travel multiplies the value as much
            // as the last, within what the step rounds away.
            let first = value_at(spec, 0.25) / spec.minimum;
            let last = spec.maximum / value_at(spec, 0.75);
            assert!(
                (first / last - 1.0).abs() < 0.05,
                "{}: {first} against {last}",
                spec.id
            );
        }
        let frequency = &SPECS[REED_FREQUENCY];
        assert_eq!(value_at(frequency, 0.0), 150.0);
        assert_eq!(value_at(frequency, 1.0), 800.0);
    }

    /// Prediction 3: value to position and back returns within one step.
    #[test]
    fn a_knob_returns_its_value() {
        for spec in knobs() {
            for share in 0..=20 {
                let value = value_at(spec, f64::from(share) / 20.0);
                let back = value_at(spec, position(spec, value));
                assert!(
                    (back - value).abs() <= spec.step * 1.0001,
                    "{}: {value} came back {back}",
                    spec.id
                );
            }
            let default = quantise(spec, spec.default);
            assert!(
                (value_at(spec, position(spec, default)) - default).abs() <= spec.step * 1.0001,
                "{}",
                spec.id
            );
        }
    }

    /// Prediction 3: an arrow key always moves at least one step, and stops
    /// at the ends.
    #[test]
    fn an_arrow_always_moves() {
        for spec in knobs() {
            for share in 1..20 {
                let value = value_at(spec, f64::from(share) / 20.0);
                let up = nudged(spec, value, "ArrowUp").unwrap();
                let down = nudged(spec, value, "ArrowDown").unwrap();
                // A coarse step can put a near end on the end itself.
                if value < spec.maximum {
                    assert!(up > value, "{}: up from {value} stayed", spec.id);
                }
                if value > spec.minimum {
                    assert!(down < value, "{}: down from {value} stayed", spec.id);
                }
            }
            assert_eq!(
                nudged(spec, spec.maximum, "PageUp"),
                Some(spec.maximum),
                "{}",
                spec.id
            );
            assert_eq!(
                nudged(spec, spec.minimum, "ArrowDown"),
                Some(spec.minimum),
                "{}",
                spec.id
            );
        }
        assert_eq!(nudged(&SPECS[GAIN], 1.0, "Enter"), None);
    }

    #[test]
    fn a_drag_covers_the_range_in_its_span() {
        let travel = &SPECS[BELLOWS_TRAVEL];
        assert_eq!(dragged(travel, travel.minimum, DRAG_SPAN), travel.maximum);
        assert_eq!(dragged(travel, travel.maximum, -DRAG_SPAN), travel.minimum);
        assert_eq!(dragged(travel, 12.0, 0.0), 12.0);
        assert_eq!(angle(travel, travel.minimum), -SWEEP);
        assert_eq!(angle(travel, travel.maximum), SWEEP);
    }

    #[test]
    fn the_readout_speaks_the_parameter() {
        assert_eq!(readout(&SPECS[GAIN], 1.0), "1.00×");
        assert_eq!(readout(&SPECS[REED_FREQUENCY], 355.0), "355.0 Hz");
        assert_eq!(readout(&SPECS[SIDE_CLEARANCE], 0.035), "0.035 mm");
        assert_eq!(readout(&SPECS[SPLIT_POINT], 53.0), "F3");
        assert_eq!(readout(&SPECS[SPLIT_POINT], 60.0), "C4");
        assert_eq!(readout(&SPECS[REGISTER], 8.0), "Musette");
        assert_eq!(note_name(24.0), "C1");
        assert_eq!(note_name(61.0), "C♯4");
    }

    #[test]
    fn steps_print_their_places() {
        assert_eq!(decimals(1.0), 0);
        assert_eq!(decimals(0.1), 1);
        assert_eq!(decimals(0.05), 2);
        assert_eq!(decimals(0.001), 3);
        assert_eq!(quantise(&SPECS[GAIN], 0.30000000000000004), 0.3);
    }
}
