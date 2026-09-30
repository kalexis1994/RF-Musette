//! The instrument's parameters: one registry, in physical units.
//!
//! Every constant of the model that anyone may want to move is a parameter
//! here -- live while playing, saved in programs, automatable -- as the
//! Concert Grand's knobs are. Unlike those, they are not process-wide
//! statics: each engine owns its values, so two instances never share one.
//!
//! Each entry states where its default comes from, with the same three
//! statuses `docs/MODEL.md` uses: measured, derived, voiced by ear (or,
//! until someone has listened, assumed). `package/metadata/parameters.json`
//! is generated from this table by the laboratory, and a test fails if the
//! two disagree.

use crate::reed::ReedDesign;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Taper {
    Linear,
    Logarithmic,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParameterSpec {
    pub id: &'static str,
    pub name: &'static str,
    pub page: &'static str,
    pub unit: &'static str,
    pub minimum: f64,
    pub maximum: f64,
    pub default: f64,
    pub step: f64,
    pub taper: Taper,
    /// For a choice among a few values: each value and its name. Empty for
    /// a continuous parameter.
    pub choices: &'static [(u32, &'static str)],
    /// Where the default comes from.
    pub source: &'static str,
}

pub const PAGE_OUTPUT: &str = "output";
pub const PAGE_REED: &str = "model_reed";
pub const PAGE_CELL: &str = "model_cell";
pub const PAGE_AIR: &str = "model_air";

pub const PAGES: [(&str, &str); 4] = [
    (PAGE_OUTPUT, "Output"),
    (PAGE_REED, "Model · Reed"),
    (PAGE_CELL, "Model · Cell & Pallet"),
    (PAGE_AIR, "Model · Air & Bellows"),
];

pub const GAIN: usize = 0;
pub const REED_FREQUENCY: usize = 1;
pub const REED_Q: usize = 2;
pub const REED_LENGTH: usize = 3;
pub const REED_WIDTH: usize = 4;
pub const REED_MODE_RATIO: usize = 5;
pub const REED_SET: usize = 6;
pub const PLATE_THICKNESS: usize = 7;
pub const SIDE_CLEARANCE: usize = 8;
pub const TIP_CLEARANCE: usize = 9;
pub const CONTRACTION: usize = 10;
pub const NEAR_FIELD: usize = 11;
pub const BELLOWS_CEILING: usize = 12;
pub const BELLOWS_CURVE: usize = 13;
pub const OVERSAMPLING: usize = 14;
pub const CELL_VOLUME: usize = 15;
pub const TONE_HOLE_AREA: usize = 16;
pub const TONE_HOLE_DEPTH: usize = 17;
pub const END_CORRECTION: usize = 18;
pub const PALLET_LIFT: usize = 19;
pub const PALLET_OPENING: usize = 20;
pub const PALLET_CLOSING: usize = 21;

pub const COUNT: usize = 22;

/// Steel, kg/m³. Tongues are tempered spring steel (Llanos-Vázquez et al.
/// 2002; the maker Harmonikas.cz); the density of steel is not in doubt.
pub const STEEL_DENSITY: f64 = 7850.0;

/// Young's modulus of steel, Pa: 200 GPa, the textbook value for carbon and
/// spring steels, which vary by a few per cent.
pub const STEEL_MODULUS: f64 = 200.0e9;

const fn spec(
    id: &'static str,
    name: &'static str,
    page: &'static str,
    unit: &'static str,
    range: (f64, f64, f64, f64),
    taper: Taper,
    source: &'static str,
) -> ParameterSpec {
    ParameterSpec {
        id,
        name,
        page,
        unit,
        minimum: range.0,
        maximum: range.1,
        default: range.2,
        step: range.3,
        taper,
        choices: &[],
        source,
    }
}

const fn choice(
    id: &'static str,
    name: &'static str,
    page: &'static str,
    choices: &'static [(u32, &'static str)],
    default: u32,
    source: &'static str,
) -> ParameterSpec {
    ParameterSpec {
        id,
        name,
        page,
        unit: "",
        minimum: choices[0].0 as f64,
        maximum: choices[choices.len() - 1].0 as f64,
        default: default as f64,
        step: 1.0,
        taper: Taper::Linear,
        choices,
        source,
    }
}

pub const SPECS: [ParameterSpec; COUNT] = [
    spec(
        "gain",
        "Output Gain",
        PAGE_OUTPUT,
        "x",
        (0.0, 4.0, 1.0, 0.01),
        Taper::Linear,
        "Output scaling. At 1, full scale is 1 Pa of sound pressure at 1 m (94 dB SPL).",
    ),
    spec(
        "reed_frequency",
        "Reed Frequency",
        PAGE_REED,
        "Hz",
        (150.0, 800.0, 355.0, 0.1),
        Taper::Logarithmic,
        "Measured: first bending mode of an accordion F4 tongue, plucked (Ziegenhals, IfM Zwota 2009, Fig. 4).",
    ),
    spec(
        "reed_q",
        "Reed Q",
        PAGE_REED,
        "",
        (10.0, 1000.0, 250.0, 1.0),
        Taper::Logarithmic,
        "Assumed: the Q Woodhouse measured on a bench free reed (Euphonics 11.6), 250. No steel accordion tongue's Q is published; the other measured value, 95, is a brass harmonica reed's (Millot & Baumann 2007), and brass loses more than tempered steel.",
    ),
    spec(
        "reed_length",
        "Tongue Length",
        PAGE_REED,
        "mm",
        (10.0, 80.0, 36.0, 0.1),
        Taper::Linear,
        "Measured: the F4 tongue (Ziegenhals 2009).",
    ),
    spec(
        "reed_width",
        "Tongue Width",
        PAGE_REED,
        "mm",
        (1.5, 8.0, 4.0, 0.05),
        Taper::Linear,
        "Measured: the F4 tongue (Ziegenhals 2009).",
    ),
    spec(
        "reed_mode_ratio",
        "Tongue Mode Ratio",
        PAGE_REED,
        "",
        (3.0, 6.25, 4.6, 0.01),
        Taper::Linear,
        "Measured: the F4 tongue's second bending mode over its first, 1645 / 355 Hz (Ziegenhals 2009, Fig. 4). The tongue's taper and thickness are derived from it and from the frequency.",
    ),
    spec(
        "reed_set",
        "Tongue Set",
        PAGE_REED,
        "mm",
        (0.0, 2.0, 0.5, 0.01),
        Taper::Linear,
        "Measured: the F4 tongue's rest offset from the plate, the Aufbiegung (Ziegenhals 2009).",
    ),
    spec(
        "plate_thickness",
        "Plate Thickness",
        PAGE_REED,
        "mm",
        (0.5, 6.0, 3.0, 0.05),
        Taper::Linear,
        "Measured: the F4 reed plate (Ziegenhals 2009).",
    ),
    spec(
        "side_clearance",
        "Side Clearance",
        PAGE_REED,
        "mm",
        (0.005, 0.2, 0.035, 0.001),
        Taper::Logarithmic,
        "Specified: 0.03 mm at the rivet to 0.04 mm at the tip, per side, on a maker's machined plates (Harmonikas.cz); the mean.",
    ),
    spec(
        "tip_clearance",
        "Tip Clearance",
        PAGE_REED,
        "mm",
        (0.005, 0.3, 0.04, 0.001),
        Taper::Logarithmic,
        "Specified: the tip gap of the same plates (Harmonikas.cz).",
    ),
    spec(
        "contraction",
        "Jet Contraction",
        PAGE_AIR,
        "",
        (0.4, 1.0, 0.61, 0.01),
        Taper::Linear,
        "Measured: the vena contracta of a sharp slit, 0.61, which geometry moves between 0.5 and 1 (Tarnopolsky, Fletcher & Lai, JASA 108, 2000).",
    ),
    spec(
        "near_field_inertance",
        "Near-Field Inertance",
        PAGE_AIR,
        "x",
        (0.05, 20.0, 1.0, 0.01),
        Taper::Logarithmic,
        "Derived, roughly: the air accelerated into the slot as the flanged end correction of an opening of the slot's area (Rayleigh). A multiple of that estimate; nothing measures it for an accordion.",
    ),
    spec(
        "bellows_ceiling",
        "Bellows Ceiling",
        PAGE_AIR,
        "Pa",
        (100.0, 6000.0, 1000.0, 1.0),
        Taper::Logarithmic,
        "Assumed: the hardest push, ~1 kPa, where most accordions start to blow their valves open (a technician's figure, unverified; normal play is 10-300 Pa, Misdariis et al. 2000).",
    ),
    spec(
        "bellows_curve",
        "Bellows Curve",
        PAGE_AIR,
        "",
        (0.5, 4.0, 2.0, 0.01),
        Taper::Linear,
        "Assumed: pressure grows as the intent to this power, so half the push is a quarter of the ceiling -- inside normal play.",
    ),
    choice(
        "oversampling",
        "Oversampling",
        PAGE_AIR,
        &[(1, "1x"), (2, "2x"), (4, "4x")],
        2,
        "Numerical: the reed runs at this multiple of the host rate.",
    ),
    spec(
        "cell_volume",
        "Cell Volume",
        PAGE_CELL,
        "cm³",
        (1.0, 40.0, 8.0, 0.1),
        Taper::Logarithmic,
        "Assumed: the reed's cell in the block. No accordion cell is published per note; laboratory accordion chambers measured 8-18 cm³ for a reed near 90 Hz (Coyle, Behrens & Cottingham 2009, via Cottingham's 2013 slides), and an F4 cell is smaller, so the bottom of that range.",
    ),
    spec(
        "tone_hole_area",
        "Tone Hole Area",
        PAGE_CELL,
        "mm²",
        (20.0, 600.0, 150.0, 1.0),
        Taper::Logarithmic,
        "Assumed: the hole the cell speaks through, taken as about the slot's own area. Not published.",
    ),
    spec(
        "tone_hole_depth",
        "Tone Hole Depth",
        PAGE_CELL,
        "mm",
        (1.0, 15.0, 5.0, 0.1),
        Taper::Linear,
        "Assumed: the pallet board's thickness. Not published.",
    ),
    spec(
        "end_correction",
        "End Correction",
        PAGE_CELL,
        "",
        (0.3, 1.0, 0.6, 0.01),
        Taper::Linear,
        "Measured range: Tonon's k, 0.43-0.80 times the hole's diameter, higher when the pallet sits close (Tonon, PICA 2, 2005); the middle of it.",
    ),
    spec(
        "pallet_lift",
        "Pallet Lift",
        PAGE_CELL,
        "mm",
        (0.5, 8.0, 3.0, 0.1),
        Taper::Linear,
        "Assumed: how far the pallet lifts with the key fully down. Not published; at 3 mm the curtain passes the whole hole from about 80 % of the travel.",
    ),
    spec(
        "pallet_opening",
        "Pallet Opening Time",
        PAGE_CELL,
        "ms",
        (1.0, 100.0, 10.0, 0.1),
        Taper::Logarithmic,
        "Assumed: closed to fully open under a finger. Not published; bounded by the finger attack, 50-140 ms from pallet to note (Llanos-Vázquez et al. 2014).",
    ),
    spec(
        "pallet_closing",
        "Pallet Closing Time",
        PAGE_CELL,
        "ms",
        (1.0, 100.0, 10.0, 0.1),
        Taper::Logarithmic,
        "Assumed: fully open to closed when the key is let go, under the pallet's spring. Not published.",
    ),
];

/// One engine's parameter values, in the units of [`SPECS`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Parameters {
    values: [f64; COUNT],
}

impl Default for Parameters {
    fn default() -> Self {
        let mut values = [0.0; COUNT];
        for (value, spec) in values.iter_mut().zip(SPECS.iter()) {
            *value = spec.default;
        }
        Self { values }
    }
}

impl Parameters {
    pub fn get(&self, index: usize) -> Option<f64> {
        self.values.get(index).copied()
    }

    /// Sets one value. Anything outside its range, not a number, or not a
    /// parameter is refused and changes nothing.
    pub fn set(&mut self, index: usize, value: f64) -> bool {
        let Some(spec) = SPECS.get(index) else {
            return false;
        };
        if !value.is_finite() || value < spec.minimum || value > spec.maximum {
            return false;
        }
        if !spec.choices.is_empty() && !spec.choices.iter().any(|(v, _)| f64::from(*v) == value) {
            return false;
        }
        self.values[index] = value;
        true
    }

    pub fn values(&self) -> &[f64; COUNT] {
        &self.values
    }

    pub fn reed_design(&self) -> ReedDesign {
        let mm = 1.0e-3;
        let v = &self.values;
        ReedDesign {
            frequency: v[REED_FREQUENCY],
            q: v[REED_Q],
            length: v[REED_LENGTH] * mm,
            width: v[REED_WIDTH] * mm,
            mode_ratio: v[REED_MODE_RATIO],
            modulus: STEEL_MODULUS,
            density: STEEL_DENSITY,
            set: v[REED_SET] * mm,
            plate_thickness: v[PLATE_THICKNESS] * mm,
            side_clearance: v[SIDE_CLEARANCE] * mm,
            tip_clearance: v[TIP_CLEARANCE] * mm,
            contraction: v[CONTRACTION],
            inertance_scale: v[NEAR_FIELD],
            cell_volume: v[CELL_VOLUME] * 1.0e-6,
            tone_hole_area: v[TONE_HOLE_AREA] * 1.0e-6,
            tone_hole_depth: v[TONE_HOLE_DEPTH] * mm,
            end_correction: v[END_CORRECTION],
        }
    }

    /// The supply pressure, Pa, the player's bellows intent (0..=1) means.
    pub fn bellows_pressure(&self, intent: f32) -> f64 {
        let intent = f64::from(intent.clamp(0.0, 1.0));
        self.values[BELLOWS_CEILING] * crate::math::pow(intent, self.values[BELLOWS_CURVE])
    }

    pub fn oversampling(&self) -> usize {
        self.values[OVERSAMPLING] as usize
    }

    pub fn pallet_design(&self) -> crate::pallet::PalletDesign {
        crate::pallet::PalletDesign {
            lift: self.values[PALLET_LIFT] * 1.0e-3,
            opening_time: self.values[PALLET_OPENING] * 1.0e-3,
            closing_time: self.values[PALLET_CLOSING] * 1.0e-3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_default_is_inside_its_range_and_ids_are_unique() {
        for (index, spec) in SPECS.iter().enumerate() {
            assert!(
                spec.minimum <= spec.default && spec.default <= spec.maximum,
                "{}",
                spec.id
            );
            assert!(
                !spec.source.is_empty(),
                "{} says where it comes from",
                spec.id
            );
            assert!(PAGES.iter().any(|(id, _)| *id == spec.page), "{}", spec.id);
            for other in &SPECS[index + 1..] {
                assert_ne!(spec.id, other.id);
            }
        }
    }

    #[test]
    fn values_outside_their_range_are_refused() {
        let mut parameters = Parameters::default();
        assert!(!parameters.set(REED_Q, 5.0));
        assert!(!parameters.set(REED_Q, f64::NAN));
        assert!(!parameters.set(OVERSAMPLING, 3.0));
        assert!(parameters.set(OVERSAMPLING, 4.0));
        assert!(!parameters.set(COUNT, 1.0));
        assert_eq!(parameters.get(REED_Q), Some(250.0));
    }

    #[test]
    fn the_bellows_curve_maps_intent_to_pressure() {
        let parameters = Parameters::default();
        assert_eq!(parameters.bellows_pressure(0.0), 0.0);
        assert!((parameters.bellows_pressure(1.0) - 1000.0).abs() < 1e-9);
        assert!((parameters.bellows_pressure(0.5) - 250.0).abs() < 1e-9);
    }
}
