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

/// The pages are the PLAY surface's (`crates/rf-musette-ui`, `panel.rs`),
/// so RackForge's own screens group the parameters as the panel does: what
/// a player reaches for first, then the model.
pub const PAGE_PLAY: &str = "play";
pub const PAGE_MICS: &str = "mics";
pub const PAGE_REED: &str = "reed";
pub const PAGE_CELL: &str = "cell";
pub const PAGE_AIR: &str = "air";

pub const PAGES: [(&str, &str); 5] = [
    (PAGE_PLAY, "Play"),
    (PAGE_MICS, "Microphones"),
    (PAGE_REED, "Reed"),
    (PAGE_CELL, "Cell & Pallet"),
    (PAGE_AIR, "Air & Bellows"),
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
pub const SWING_LIMIT: usize = 22;
pub const BELLOWS_DIRECTION: usize = 23;
pub const REVERSAL_TIME: usize = 24;
pub const TREMOLO: usize = 25;
pub const REGISTER: usize = 26;
pub const BELLOWS_RESPONSE: usize = 27;
pub const BELLOWS_AREA: usize = 28;
pub const BELLOWS_VOLUME: usize = 29;
pub const ARM_SPEED: usize = 30;
pub const BELLOWS_LEAK: usize = 31;
pub const AIR_VALVE: usize = 32;
pub const CASSOTTO: usize = 33;
pub const CASSOTTO_RESONANCE: usize = 34;
pub const CASSOTTO_Q: usize = 35;
pub const PITCH_A4: usize = 36;
pub const Q_SLOPE: usize = 37;
pub const ATTACK_KICK: usize = 38;
pub const BASS_REGISTER: usize = 39;
pub const LEFT_HAND: usize = 40;
pub const SPLIT_POINT: usize = 41;
pub const BELLOWS_SMOOTHING: usize = 42;
pub const AUTO_REVERSE: usize = 43;
pub const BELLOWS_TRAVEL: usize = 44;
pub const MOD_WHEEL: usize = 45;
/// The microphones and the room (milestone 9b).
pub const MIC_LAYOUT: usize = 46;
pub const STEREO_WIDTH: usize = 47;
pub const PERSPECTIVE: usize = 48;
pub const ROOM_SIZE: usize = 49;
pub const ROOM_HARDNESS: usize = 50;
pub const ROOM_LEVEL: usize = 51;
pub const INTERNAL_TREBLE: usize = 52;
pub const INTERNAL_BASS: usize = 53;
pub const INTERNAL_BALANCE: usize = 54;
pub const INTERNAL_HIGHPASS: usize = 55;
pub const CLIP_DISTANCE: usize = 56;
pub const CLIP_SPACING: usize = 57;
pub const CLIP_PATTERN: usize = 58;
pub const CLIP_BALANCE: usize = 59;
pub const CLIP_HIGHPASS: usize = 60;
pub const SPOTS_TREBLE_DISTANCE: usize = 61;
pub const SPOTS_BASS_DISTANCE: usize = 62;
pub const SPOTS_PATTERN: usize = 63;
pub const SPOTS_AMBIENCE: usize = 64;
pub const ORTF_DISTANCE: usize = 65;
pub const ORTF_HEIGHT: usize = 66;
pub const ORTF_PATTERN: usize = 67;
pub const SPACED_DISTANCE: usize = 68;
pub const SPACED_SPACING: usize = 69;
pub const SPACED_PATTERN: usize = 70;
pub const SINGLE_DISTANCE: usize = 71;
pub const SINGLE_PATTERN: usize = 72;
pub const PAD_SEATING: usize = 73;
pub const KEY_TOUCH: usize = 74;
pub const BASS_SYSTEM: usize = 75;

pub const COUNT: usize = 76;

/// [`BASS_SYSTEM`]'s values.
pub const STRADELLA: f64 = 0.0;
pub const FREE_BASS: f64 = 1.0;

/// The pressure below which the air is too weak to push a tongue into its
/// frame at a key's opening, Pa: half the start is reached here. Assumed, of
/// the order of the lowest thresholds.
pub const KICK_PRESSURE: f64 = 20.0;

/// [`BELLOWS_RESPONSE`]'s values.
pub const ARM: f64 = 0.0;
pub const STIFF: f64 = 1.0;

/// [`MOD_WHEEL`]'s one value: the wheel as the push (milestone 8f). The
/// wheel as where the bellows is (8i, its value 1) was withdrawn; a state
/// saved with it loads as this.
pub const WHEEL_PRESSURE: f64 = 0.0;

/// [`PERSPECTIVE`]'s values: whose left is left.
pub const PLAYER: f64 = 0.0;
pub const AUDIENCE: f64 = 1.0;

/// The treble's ranks, in the order the engine keeps them.
pub const RANK_LOW: usize = 0;
/// M−, the tremolo's flat 8′.
pub const RANK_FLAT: usize = 1;
/// M, the true 8′: the reed Ziegenhals measured.
pub const RANK_MIDDLE: usize = 2;
/// M+, the tremolo's sharp 8′.
pub const RANK_SHARP: usize = 3;
pub const RANK_HIGH: usize = 4;
pub const RANKS: usize = 5;

/// The bass side's ranks, in the order the engine keeps them (Roland's
/// footages, FR-8x Owner's Manual p. 75).
pub const BASS_16: usize = 0;
pub const BASS_8: usize = 1;
/// The contralto, wrapping from below the 4′ into it: Roland's "8-4′".
pub const BASS_8_4: usize = 2;
pub const BASS_4: usize = 3;
pub const BASS_2: usize = 4;
pub const BASS_RANKS: usize = 5;

/// [`BELLOWS_DIRECTION`]'s values.
pub const PULL: f64 = 0.0;
pub const PUSH: f64 = 1.0;

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

/// The smallest of a choice's values: the list may run in another order
/// than its values (the treble registers, from the most common).
const fn least(choices: &[(u32, &str)]) -> u32 {
    let mut least = choices[0].0;
    let mut i = 1;
    while i < choices.len() {
        if choices[i].0 < least {
            least = choices[i].0;
        }
        i += 1;
    }
    least
}

/// The largest of a choice's values.
const fn most(choices: &[(u32, &str)]) -> u32 {
    let mut most = choices[0].0;
    let mut i = 1;
    while i < choices.len() {
        if choices[i].0 > most {
            most = choices[i].0;
        }
        i += 1;
    }
    most
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
        minimum: least(choices) as f64,
        maximum: most(choices) as f64,
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
        PAGE_PLAY,
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
        (20.0, 600.0, 100.0, 1.0),
        Taper::Logarithmic,
        "Reported: the hole each treble cell speaks through, in the block's foot over the pallet -- about a centimetre, round or square, the same under every key, a piccolo's some 2 mm smaller (technicians on accordionists.info, milestone 8m). Was 150 mm² scaled with the tongue, assumed.",
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
        (1.0, 500.0, 50.0, 0.1),
        Taper::Logarithmic,
        "Reported: \"los ataques usuales de dedo son realizados en unos 0.05 s\" -- a normal finger attack takes about 0.05 s; a slow keystroke about 0.5 s (Llanos-Vázquez, thesis 2015, p164).",
    ),
    spec(
        "pallet_closing",
        "Pallet Closing Time",
        PAGE_CELL,
        "ms",
        (1.0, 100.0, 10.0, 0.1),
        Taper::Logarithmic,
        "Assumed: fully open to closed when the key is let go, under the pallet's spring. Not published. 10 ms, chosen by ear: tried at 30 (2026-10-02), the closing's sound stood further apart from the note.",
    ),
    spec(
        "swing_limit",
        "Swing Limit",
        PAGE_AIR,
        "",
        (0.0, 4.0, 0.5, 0.01),
        Taper::Linear,
        "Voiced: the one voiced constant of the reed. A damping that grows with the tongue's swing and with the flow, standing in for the nonlinear dissipation St. Hilaire & Vaidya (J. Fluid Mech. 67, 1975) found limits a free reed and that this model does not derive. At 0.5 the swing settles near 5 mm from 1 kPa up, as this reed's is measured (Ziegenhals 2009: more than 4 mm at mf) and holds nearly constant with pressure (Cottingham, Lilly & Reed 1999). At 0 the model is as derived and its swing keeps growing with pressure. Heard 2026-09-30 against 0 on a continuous swell of the F4: 0.5 preferred.",
    ),
    choice(
        "bellows_direction",
        "Bellows Direction",
        PAGE_PLAY,
        &[(0, "Pull"), (1, "Push")],
        0,
        "Decided 2026-09-30: which way the bellows moves, and so which reed of the plate sounds -- on pull the one inside the cell, on push the one on the bellows side. No MIDI accordion sends it (Roland FR-1x and Brendan Vavra's both send only CC 11), so it is this parameter, or CC 80 as a switch: below 64 pull, 64 and above push.",
    ),
    spec(
        "reversal_time",
        "Reversal Time",
        PAGE_AIR,
        "ms",
        (5.0, 1000.0, 100.0, 1.0),
        Taper::Logarithmic,
        "Assumed: how long the bellows takes to stop and turn when the direction changes, the pressure passing through zero on the way. Not measured; players describe \"a slight interruption in the sound\", like a bow change (McMahan 2016; Llanos et al. 2002).",
    ),
    spec(
        "tremolo",
        "Tremolo",
        PAGE_PLAY,
        "Hz",
        (0.0, 15.0, 4.1, 0.01),
        Taper::Linear,
        "Measured, and a style to voice by taste: the beat between the true and the sharp 8′ at A4. 4.1 Hz is a Borsini Super Star LMMMH's, whose builder tuned M+ at +4.1 Hz at A4 rising 1.4 Hz per octave and M− at −3.7 Hz falling 1.8 Hz per octave (Hergert, Acta Acustica 8, 2024, Fig. 6, read off); both lines keep that shape, scaled to this value. Accordions run from \"dry\" to \"wet\", 0.5-7 Hz at A4 (Hergert, Forum Acusticum 2023).",
    ),
    choice(
        "register",
        "Register",
        PAGE_PLAY,
        // Listed from the most common register to the rarest (milestone 9i
        // again): M alone, with M+, with L, with H, with M−; fewer reeds
        // first. The values are Roland's switch row's, as saved.
        &[
            (11, "Clarinet"),
            (12, "Celeste"),
            (0, "Bassoon"),
            (1, "Bandoneon"),
            (2, "Cello"),
            (13, "Piccolo"),
            (4, "Organ"),
            (10, "Oboe"),
            (3, "Harmonium"),
            (9, "Violin"),
            (7, "Tremolo"),
            (8, "Musette"),
            (5, "Accordion"),
            (6, "Master"),
        ],
        11,
        "Measured as a maker draws it: the 14 treble registers of Roland's FR-3x, with the reeds each opens (Owner's Manual, p. 27): Bassoon L, Bandoneon LM, Cello L M M+, Harmonium LMH, Organ LH, Accordion L M− M H, Master L M− M M+ H, Tremolo M− M+, Musette M− M M+, Violin M M+ H, Oboe MH, Clarinet M, Celeste M M+, Piccolo H.",
    ),
    choice(
        "bellows_response",
        "Bellows Response",
        PAGE_AIR,
        &[(0, "Arm"), (1, "Stiff")],
        0,
        "Decided 2026-09-30: what the intent (velocity, or CC 11) is. Arm: the player's push; the bellows makes the pressure from it, the arm's force falling as it moves faster (Hill 1938) and the air its reeds, leaks and air button spend. Stiff: the pressure itself, for a digital accordion whose sensor already measures it.",
    ),
    spec(
        "bellows_area",
        "Bellows Area",
        PAGE_AIR,
        "cm²",
        (100.0, 2000.0, 600.0, 1.0),
        Taper::Logarithmic,
        "Assumed, voiced by ear: the bellows' cross-section, which turns the push into pressure and the air spent into the arm's speed. A full-size accordion's order (about 35 × 17 cm); no measurement published.",
    ),
    spec(
        "bellows_volume",
        "Bellows Volume",
        PAGE_AIR,
        "L",
        (1.0, 40.0, 12.0, 0.1),
        Taper::Logarithmic,
        "Assumed, voiced by ear: the air in the bellows, whose compliance V/(ρc²) smooths the pressure over a few milliseconds. A full-size accordion half open; no measurement published.",
    ),
    spec(
        "arm_speed",
        "Arm Speed",
        PAGE_AIR,
        "m/s",
        (0.1, 5.0, 1.0, 0.01),
        Taper::Logarithmic,
        "Assumed, voiced by ear: v_max of Hill's force-velocity law (Proc. R. Soc. B 126, 1938, k = 0.25), the speed at which the arm could move the bellows with no air to push. Slower makes the pressure sag more as more reeds draw.",
    ),
    spec(
        "bellows_leak",
        "Bellows Leak",
        PAGE_AIR,
        "mm²",
        (0.0, 200.0, 10.0, 0.1),
        Taper::Linear,
        "Assumed: the bellows' and the closed valves' leaks, as one orifice. A sealed accordion should hold air for more than 30 s under gentle pressure (repair folklore, unverified), which bounds it to a few tens of mm².",
    ),
    spec(
        "air_valve",
        "Air Valve",
        PAGE_PLAY,
        "",
        (0.0, 1.0, 0.0, 0.001),
        Taper::Linear,
        "How far the air button is pressed: it vents the bellows through an orifice of up to 400 mm² (assumed), so the bellows moves without sounding (Llanos et al. 2002; McMahan 2016).",
    ),
    choice(
        "cassotto",
        "Cassotto",
        PAGE_PLAY,
        &[(0, "Off"), (1, "On")],
        0,
        "Whether the 16′ and the true 8′ sound into a cassotto, as on Llanos-Vázquez's Pigini Sirius (thesis 2015, p51) and in a double cassotto (bassoon and clarinet chambers). A choice of instrument; off until heard.",
    ),
    spec(
        "cassotto_resonance",
        "Cassotto Resonance",
        PAGE_CELL,
        "Hz",
        (500.0, 1500.0, 900.0, 1.0),
        Taper::Logarithmic,
        "Measured: the cassotto shaft's resonance, 800 Hz-1 kHz and little different between makers (Richter, IfM Zwota, Demusa report 1989); the middle of that range.",
    ),
    spec(
        "cassotto_q",
        "Cassotto Q",
        PAGE_CELL,
        "",
        (0.5, 10.0, 2.0, 0.01),
        Taper::Logarithmic,
        "Assumed, voiced by ear: how sharp the cassotto's resonance is. Wood, felt and a slot that is no neck make it low; nothing measures it. Against Llanos-Vázquez's E4 at mf, whose 8′ centroid falls from 2013 Hz outside the cassotto to 1389 inside (thesis, Table 4.16).",
    ),
    spec(
        "pitch_a4",
        "Pitch A4",
        PAGE_PLAY,
        "Hz",
        (415.0, 466.0, 440.0, 0.1),
        Taper::Linear,
        "Decided 2026-10-01: the A every true 8′ reed is tuned to, in equal temperament, where it sounds at 300 Pa. 440 Hz is the IfM Zwota's reference (Richter, \"Stimmung\", Demusa '90).",
    ),
    spec(
        "q_slope",
        "Q Slope",
        PAGE_REED,
        "",
        (0.0, 1.5, 0.7, 0.01),
        Taper::Linear,
        "A measured trend, extrapolated: each reed's Q is the F4's times (f / F4)^slope. 0.7 is the exponent between the two free reeds whose Q is measured, a reed-organ C3's 83 at 137 Hz (Cottingham, ICA 1998) and a harmonica reed's ~233 at 598 Hz (Förtsch 2021); Nussbaumer & Agarwal found 200-400 at 236-743 Hz. Nothing above ~750 Hz is measured, and no damping mechanism falls for small reeds.",
    ),
    spec(
        "attack_kick",
        "Attack Kick",
        PAGE_REED,
        "",
        (0.0, 3.0, 0.5, 0.01),
        Taper::Linear,
        "Voiced, standing on a measurement: when a key opens, each reed the bellows blows starts this many times its set into the frame, scaled by P/(P + 20 Pa). Cottingham (ICA 2019): the motion of a free reed \"begins with an initial displacement of the reed tongue into the reed frame\". Voiced to Llanos-Vázquez et al.'s measured finger attack, 50-140 ms at p and mf: 1 with the cell a volume (docs/ROADMAP.md, 7b); 0.3, chosen by ear, once the cell was a tube (8m); 0.5 with the set a third flat (2026-10-02), between the F4, 58 ms at 100 Pa and 46 at 400, and the 16′ C2, 207 ms at 400 Pa -- no one start meets both, the F4 wanting less and the bass more. At 0 the start is as derived.",
    ),
    choice(
        "bass_register",
        "Bass Register",
        PAGE_PLAY,
        &[
            (0, "2'"),
            (1, "4'"),
            (2, "8-4'"),
            (3, "16'/8'/8-4'/4'/2'"),
            (4, "8'/4'/2'"),
            (5, "16'/8'/8-4'"),
            (6, "16'/2'"),
        ],
        3,
        "As a maker gives them: the 7 bass registers of Roland's FR-3x (Owner's Manual, pp. 30, 72), one for the bass and chord rows alike. The bass buttons sound every open rank, the chord buttons the open 8-4', 4' and 2' (FR-8x, p. 75). The default opens all five.",
    ),
    choice(
        "left_hand",
        "Left Hand",
        PAGE_PLAY,
        &[(1, "On")],
        1,
        "Retired (2026-10-02, milestone 8p): the left hand is always there, under the Split Point of a MIDI keyboard on one channel and on channels 2 and 3. Kept, one value, so states and links by index still find what follows; a state saved with it off loads as on.",
    ),
    spec(
        "split_point",
        "Split Point",
        PAGE_PLAY,
        "note",
        (24.0, 96.0, 53.0, 1.0),
        Taper::Linear,
        "The lowest note the treble keeps, as a MIDI note: F3 (53), the treble's first key, by default. Under it the left hand: with Stradella the octave below it plays the chords and the rest the bass buttons; with Free Bass every key its own note.",
    ),
    spec(
        "bellows_smoothing",
        "Bellows Smoothing",
        PAGE_PLAY,
        "ms",
        (0.0, 1000.0, 150.0, 1.0),
        Taper::Linear,
        "Retired 2026-10-02: key velocity no longer moves the bellows (an accordion's keys have none), so nothing is smoothed. Kept so saved states load. Was (milestone 8f): when key velocity set the push, the push moved to each new strike's over this time.",
    ),
    choice(
        "auto_reverse",
        "Auto Reverse",
        PAGE_PLAY,
        &[(0, "Off"), (1, "On")],
        0,
        "Decided 2026-10-01 (milestone 8g): whether the bellows runs out and turns on its own, as a player turns it -- at a gap between notes once 70 % of its travel is spent, or when all of it is. Off, the bellows never runs out. Setting Bellows Direction takes the bellows back.",
    ),
    spec(
        "bellows_travel",
        "Bellows Travel",
        PAGE_PLAY,
        "L",
        (2.0, 40.0, 12.0, 0.1),
        Taper::Logarithmic,
        "Assumed, voiced by ear: the air the bellows gives in one direction before it must turn, when Auto Reverse is on. A full-size bellows' 600 cm² over some 20 cm of the stroke a player uses.",
    ),
    choice(
        "mod_wheel",
        "Mod Wheel",
        PAGE_PLAY,
        &[(0, "Pressure")],
        0,
        "Retired 2026-10-01, kept so the parameters after it keep their places: the modulation wheel is how hard the arm pushes (milestone 8f). It could also be where the bellows is (milestone 8i, the value 1), which was withdrawn at the player's asking: it did not play well.",
    ),
    choice(
        "mic_layout",
        "Microphones",
        PAGE_MICS,
        &[
            (0, "Internal"),
            (1, "Clip-on"),
            (2, "Two spots"),
            (3, "ORTF pair"),
            (4, "Spaced pair"),
            (5, "One mic"),
            (6, "Dry"),
        ],
        3,
        "Decided 2026-10-01 (milestone 9b), at the player's asking: how the instrument is miked, as engineers do it (docs/SOURCES.md, \"The microphones\"). On stage, inside the instrument (Rumberger, Nalbantov) or clipped to it (K&K, DPA 4099); in the studio, one mic per side (Shure, Piovesan), an ORTF pair at 1 m (Piovesan's reference recordings, the default), a spaced pair (Sound On Sound) or one mic in front (Shure). Dry is the instrument alone at 1 m, as before.",
    ),
    spec(
        "stereo_width",
        "Stereo Width",
        PAGE_MICS,
        "",
        (0.0, 1.0, 0.7, 0.01),
        Taper::Linear,
        "Voiced by ear: how far apart the capsules are panned. Sound On Sound (2010): hard panning gives \"a 20-foot-wide accordion\".",
    ),
    choice(
        "perspective",
        "Perspective",
        PAGE_MICS,
        &[(0, "Player"), (1, "Audience")],
        1,
        "Decided 2026-10-01: whose left is left. Microphones facing the player hear the treble on their left, so the audience's is the default.",
    ),
    spec(
        "room_size",
        "Room Size",
        PAGE_MICS,
        "m³",
        (20.0, 3000.0, 150.0, 1.0),
        Taper::Logarithmic,
        "Assumed: the room's volume. A box of 1.25 : 1.6 : 1, the player a third of the way down it; Sabine's reverberation from its walls.",
    ),
    spec(
        "room_hardness",
        "Wall Hardness",
        PAGE_MICS,
        "",
        (0.0, 1.0, 0.2, 0.01),
        Taper::Linear,
        "Concert Grand's absorption from hardness, voiced there: alpha_mid = 0.5 e^(-2.6 h) + 0.035, the highs more absorbed in a soft room. Default voiced by the player (2026-10-02): 0.2, a room with people in it.",
    ),
    spec(
        "room_level",
        "Room Level",
        PAGE_MICS,
        "dB",
        (-40.0, 12.0, 0.0, 0.1),
        Taper::Linear,
        "Derived at 0 dB: the room's reflections and tail as loud as Sabine's reverberant field makes them, 16 pi/A of the direct sound at 1 m. Away from 0, an engineer's choice.",
    ),
    spec(
        "internal_treble",
        "Treble Capsules",
        PAGE_MICS,
        "",
        (2.0, 6.0, 3.0, 1.0),
        Taper::Linear,
        "Internal: how many capsules run over the treble's reed blocks, end to end, 3 cm off them -- K&K two, MusicTech and Owsinski three, Nalbantov five, Rumberger up to six.",
    ),
    spec(
        "internal_bass",
        "Bass Capsules",
        PAGE_MICS,
        "",
        (1.0, 2.0, 1.0, 1.0),
        Taper::Linear,
        "Internal: one or two capsules in the bass box, 4 cm off its reeds (Owsinski two; the rest one).",
    ),
    spec(
        "internal_balance",
        "Balance",
        PAGE_MICS,
        "dB",
        (-12.0, 12.0, 0.0, 0.1),
        Taper::Linear,
        "Internal: the treble against the bass, as the systems' own control (Rumberger TA20).",
    ),
    spec(
        "internal_highpass",
        "High-Pass",
        PAGE_MICS,
        "Hz",
        (20.0, 300.0, 80.0, 1.0),
        Taper::Logarithmic,
        "Internal: the high-pass a live rig puts on close capsules, against handling and bellows noise (Audio-Technica ATM350, 80 Hz).",
    ),
    spec(
        "clip_distance",
        "Distance",
        PAGE_MICS,
        "cm",
        (2.0, 15.0, 5.0, 0.1),
        Taper::Linear,
        "Clip-on: the goosenecks off the treble grille; the bass one half as far again (K&K: 2 in, 3 in).",
    ),
    spec(
        "clip_spacing",
        "Spacing",
        PAGE_MICS,
        "cm",
        (15.0, 35.0, 27.0, 0.5),
        Taper::Linear,
        "Clip-on: between the two treble goosenecks (K&K: 10-12 in).",
    ),
    spec(
        "clip_pattern",
        "Pattern",
        PAGE_MICS,
        "",
        (0.0, 1.0, 0.5, 0.01),
        Taper::Linear,
        "Assumed, voiced by ear: the capsules' pattern, from omni (0) through cardioid (0.5) and supercardioid (~0.63) to figure-of-eight (1). Clip-on mics are cardioid or supercardioid (DPA 4099, AKG C516).",
    ),
    spec(
        "clip_balance",
        "Balance",
        PAGE_MICS,
        "dB",
        (-12.0, 12.0, 0.0, 0.1),
        Taper::Linear,
        "Clip-on: the treble against the bass; K&K's treble channel \"may need\" more gain.",
    ),
    spec(
        "clip_highpass",
        "High-Pass",
        PAGE_MICS,
        "Hz",
        (20.0, 300.0, 80.0, 1.0),
        Taper::Logarithmic,
        "Clip-on: as Internal's.",
    ),
    spec(
        "spots_treble_distance",
        "Treble Distance",
        PAGE_MICS,
        "cm",
        (10.0, 100.0, 30.0, 0.5),
        Taper::Linear,
        "Two spots: the treble's stand off its grille (Shure: about 12 in).",
    ),
    spec(
        "spots_bass_distance",
        "Bass Distance",
        PAGE_MICS,
        "cm",
        (10.0, 100.0, 30.0, 0.5),
        Taper::Linear,
        "Two spots: the bass's stand off the middle of the arc the bass box travels (Piovesan), a foot away (Stamler).",
    ),
    spec(
        "spots_pattern",
        "Pattern",
        PAGE_MICS,
        "",
        (0.0, 1.0, 0.5, 0.01),
        Taper::Linear,
        "Assumed, voiced by ear: the capsules' pattern, from omni (0) through cardioid (0.5) and supercardioid (~0.63) to figure-of-eight (1).",
    ),
    spec(
        "spots_ambience",
        "Ambient Pair",
        PAGE_MICS,
        "",
        (0.0, 1.0, 0.0, 0.01),
        Taper::Linear,
        "Two spots: how much of the ORTF pair, as set for it, is mixed in -- Piovesan's four-mic take.",
    ),
    spec(
        "ortf_distance",
        "Distance",
        PAGE_MICS,
        "m",
        (0.3, 3.0, 1.0, 0.01),
        Taper::Logarithmic,
        "ORTF pair: in front of the instrument (Piovesan: 1 m).",
    ),
    spec(
        "ortf_height",
        "Height",
        PAGE_MICS,
        "m",
        (0.5, 2.0, 1.2, 0.01),
        Taper::Linear,
        "Assumed: the pair's height off the floor; the seated instrument's middle is 1.03 m.",
    ),
    spec(
        "ortf_pattern",
        "Pattern",
        PAGE_MICS,
        "",
        (0.0, 1.0, 0.5, 0.01),
        Taper::Linear,
        "Assumed, voiced by ear: the capsules' pattern, from omni (0) through cardioid (0.5) and supercardioid (~0.63) to figure-of-eight (1). ORTF is cardioid.",
    ),
    spec(
        "spaced_distance",
        "Distance",
        PAGE_MICS,
        "m",
        (0.2, 2.0, 0.5, 0.01),
        Taper::Logarithmic,
        "Spaced pair: \"slightly forward of the instrument\" (Sound On Sound, 2010).",
    ),
    spec(
        "spaced_spacing",
        "Spacing",
        PAGE_MICS,
        "m",
        (0.3, 1.5, 0.9, 0.01),
        Taper::Linear,
        "Spaced pair: about 18 in wider than the instrument (Sound On Sound, 2010).",
    ),
    spec(
        "spaced_pattern",
        "Pattern",
        PAGE_MICS,
        "",
        (0.0, 1.0, 0.5, 0.01),
        Taper::Linear,
        "Assumed, voiced by ear: the capsules' pattern, from omni (0) through cardioid (0.5) and supercardioid (~0.63) to figure-of-eight (1). Sound On Sound's are cardioids.",
    ),
    spec(
        "single_distance",
        "Distance",
        PAGE_MICS,
        "m",
        (0.2, 2.0, 0.5, 0.01),
        Taper::Logarithmic,
        "One mic: in front, centred, \"one or two feet\" (Shure's booklet; Owsinski 2-3 ft).",
    ),
    spec(
        "single_pattern",
        "Pattern",
        PAGE_MICS,
        "",
        (0.0, 1.0, 0.5, 0.01),
        Taper::Linear,
        "Assumed, voiced by ear: the capsules' pattern, from omni (0) through cardioid (0.5) and supercardioid (~0.63) to figure-of-eight (1).",
    ),
    spec(
        "pad_seating",
        "Pad Seating",
        PAGE_CELL,
        "ms",
        (1.0, 50.0, 12.0, 0.1),
        Taper::Logarithmic,
        "Voiced against a measurement (2026-10-02): the time constant with which the pad, on its felt and leather, slows to the seat once a key is let go -- the last tenth of the pallet's travel, which is where the tone is shut off. No pad's compression is published. Measured instead on FreePats' Hohner releases (CC0, 17 notes): the steepest fall of the band above 3 kHz in any 1.5 ms is −6 to −23 dB, median −12, the D4 −17. A pad at constant speed made it −49 dB (the closing's sound); at 12 ms the D4 here is −16 (8 ms −20, 24 ms −13).",
    ),
    choice(
        "key_touch",
        "Key Touch",
        PAGE_PLAY,
        &[(0, "Off"), (1, "On")],
        1,
        "Decided by the player (2026-10-02, milestones 9h, 9h again and 9j): played from a keyboard without the wheel, a note's velocity sets how far its key or button goes down, and so how far its pallets open -- as an accordionist holds a key part-way -- in the treble, the bass and chord buttons and the free bass alike. The bellows rests, and the wheel and Expression move nothing. Off, every key and button goes fully down and the bellows is theirs. The curve is derived from milestone 9g's measurement, even in decibels, down to each key's and button's own floor: the shallowest curtain at which it holds a steady tone, measured (touch.rs), raised 20 % for a chord's sag.",
    ),
    choice(
        "bass_system",
        "Bass System",
        PAGE_PLAY,
        &[(0, "Stradella"), (1, "Free Bass")],
        0,
        "Decided by the player (2026-10-02, milestone 8p): Stradella, the standard bass -- bass buttons and chords; or Free Bass, as a converter accordion's left hand, single notes at their own pitch from E1 to C#6 (the bayan's 58), two voices an octave apart (8' and 4', assumed from makers' \"octave tuned\" free bass).",
    ),
];

/// The air button's opening when fully pressed, m²: assumed.
pub const AIR_VALVE_AREA: f64 = 400.0e-6;

/// Which ranks each [`REGISTER`] opens, in the order of its choices:
/// L, M−, M, M+, H (Roland FR-3x Owner's Manual, p. 27).
const REGISTERS: [[bool; RANKS]; 14] = [
    [true, false, false, false, false], // Bassoon
    [true, false, true, false, false],  // Bandoneon
    [true, false, true, true, false],   // Cello
    [true, false, true, false, true],   // Harmonium
    [true, false, false, false, true],  // Organ
    [true, true, true, false, true],    // Accordion
    [true, true, true, true, true],     // Master
    [false, true, false, true, false],  // Tremolo
    [false, true, true, true, false],   // Musette
    [false, false, true, true, true],   // Violin
    [false, false, true, false, true],  // Oboe
    [false, false, true, false, false], // Clarinet
    [false, false, true, true, false],  // Celeste
    [false, false, false, false, true], // Piccolo
];

/// Which bass-side ranks each [`BASS_REGISTER`] opens, in the order of its
/// choices: 16′, 8′, 8-4′, 4′, 2′ (Roland FR-3x Owner's Manual, p. 30).
const BASS_REGISTERS: [[bool; BASS_RANKS]; 7] = [
    [false, false, false, false, true], // 2'
    [false, false, false, true, false], // 4'
    [false, false, true, false, false], // 8-4'
    [true, true, true, true, true],     // 16'/8'/8-4'/4'/2'
    [false, true, false, true, true],   // 8'/4'/2'
    [true, true, true, false, false],   // 16'/8'/8-4'
    [true, false, false, false, true],  // 16'/2'
];

/// Which ranks a [`REGISTER`] value opens, L to H: the panel draws its
/// symbols from this, not from a copy.
pub fn register_ranks(register: usize) -> Option<[bool; RANKS]> {
    REGISTERS.get(register).copied()
}

/// The RackForge Control Profile v1 roles the instrument answers to: a
/// controller's knob published with a role turns the parameter named here
/// (RackForge `docs/MIDI_PARAMETER_LINKS.md`). A role is given only where the
/// accordion has the thing it names; `docs/RACKFORGE_CONTROL_MAPPING.md`
/// says why each other role is left unbound.
pub const SEMANTIC_CONTROLS: [(&str, usize); 3] = [
    // A key's attack is its pallet opening: the finger attack, about 0.05 s
    // played normally (Llanos-Vázquez 2015, p164).
    ("synth.envelope.amp.attack", PALLET_OPENING),
    // Its release, the pallet closing.
    ("synth.envelope.amp.release", PALLET_CLOSING),
    // The tremolo is a beat, M− against M+, heard as a periodic swell: its
    // rate is what an LFO rate names.
    ("synth.lfo.rate", TREMOLO),
];

/// Which bass-side ranks a [`BASS_REGISTER`] value opens, 16′ to 2′.
pub fn bass_register_ranks(register: usize) -> Option<[bool; BASS_RANKS]> {
    BASS_REGISTERS.get(register).copied()
}

/// Where a note sounds: the treble's keys, the bass buttons, or the chord
/// ranks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Treble,
    Bass,
    Chord,
    /// The free bass's single notes (milestone 8p).
    Free,
}

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
            swing_limit: v[SWING_LIMIT],
            // The measured F4 carries no load.
            tip_load: 0.0,
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

    /// +1 when the bellows pushes, -1 when it pulls.
    pub fn direction(&self) -> f64 {
        if self.values[BELLOWS_DIRECTION] == PUSH {
            1.0
        } else {
            -1.0
        }
    }

    /// Seconds the bellows takes to turn.
    pub fn reversal_time(&self) -> f64 {
        self.values[REVERSAL_TIME] * 1.0e-3
    }

    /// The beats of M+ and M− against M at this key, Hz (the second
    /// negative): the tremolo t asked at A4, along the Borsini's measured
    /// lines scaled to it -- M+ = t (1 + 0.341 log₂(f/440)), M− = −t (0.902 +
    /// 0.439 log₂(f/440)) (Hergert 2024, Fig. 6).
    pub fn tremolo_beats(&self) -> (f64, f64) {
        self.tremolo_beats_at(self.values[REED_FREQUENCY])
    }

    /// The same beats for an M reed at `frequency`, Hz.
    pub fn tremolo_beats_at(&self, frequency: f64) -> (f64, f64) {
        let t = self.values[TREMOLO];
        let octaves = crate::math::ln(frequency / 440.0) / core::f64::consts::LN_2;
        (
            t * (1.0 + 1.4 / 4.1 * octaves),
            -t * (3.7 / 4.1 + 1.8 / 4.1 * octaves),
        )
    }

    /// The bellows' air and the arm that moves it, whichever way the arm is
    /// read.
    pub fn bellows_air(&self) -> crate::wind::WindDesign {
        let area = self.values[BELLOWS_AREA] * 1.0e-4;
        let volume = self.values[BELLOWS_VOLUME] * 1.0e-3;
        crate::wind::WindDesign {
            area,
            compliance: volume
                / (crate::reed::AIR_DENSITY
                    * crate::reed::SPEED_OF_SOUND
                    * crate::reed::SPEED_OF_SOUND),
            arm_speed: self.values[ARM_SPEED],
            vent: self.values[BELLOWS_LEAK] * 1.0e-6 + self.values[AIR_VALVE] * AIR_VALVE_AREA,
        }
    }

    /// The air the bellows gives in one direction, m³.
    pub fn travel(&self) -> f64 {
        self.values[BELLOWS_TRAVEL] * 1.0e-3
    }

    /// The bellows' air, when the intent is the arm's push; `None` when it
    /// is the pressure itself.
    pub fn wind_design(&self) -> Option<crate::wind::WindDesign> {
        if self.values[BELLOWS_RESPONSE] == STIFF {
            return None;
        }
        Some(self.bellows_air())
    }

    /// The cassotto's resonance, Hz, and Q, when the instrument has one.
    pub fn cassotto(&self) -> Option<(f64, f64)> {
        (self.values[CASSOTTO] == 1.0)
            .then(|| (self.values[CASSOTTO_RESONANCE], self.values[CASSOTTO_Q]))
    }

    /// Whether a rank sounds into the cassotto: L and M, as on the Pigini
    /// Sirius Llanos-Vázquez measured.
    pub fn in_cassotto(rank: usize) -> bool {
        matches!(rank, RANK_LOW | RANK_MIDDLE)
    }

    /// Which ranks the register lets the bellows reach.
    pub fn open_ranks(&self) -> [bool; RANKS] {
        REGISTERS[self.values[REGISTER] as usize]
    }

    /// Where a note on a treble channel goes: the treble from the split up;
    /// under it the left hand -- with Stradella the chords in the octave
    /// under the split and the bass buttons below it, with Free Bass the
    /// note itself (milestone 8p).
    pub fn left_hand_side(&self, key: u8) -> Side {
        let split = self.values[SPLIT_POINT];
        let key = f64::from(key);
        if key >= split {
            Side::Treble
        } else if self.values[BASS_SYSTEM] == FREE_BASS {
            Side::Free
        } else if key >= split - 12.0 {
            Side::Chord
        } else {
            Side::Bass
        }
    }

    /// Which bass-side ranks the bass register lets the bellows reach.
    pub fn open_bass_ranks(&self) -> [bool; BASS_RANKS] {
        BASS_REGISTERS[self.values[BASS_REGISTER] as usize]
    }

    /// Whether a bass-side rank sounds for the chord buttons: the 8-4′, 4′
    /// and 2′ (Roland FR-8x Owner's Manual, p. 75). The 16′ and 8′ sound for
    /// the bass buttons only.
    pub fn is_chord_rank(rank: usize) -> bool {
        matches!(rank, BASS_8_4 | BASS_4 | BASS_2)
    }

    /// The design of one rank's reed for this key ([`RANK_LOW`] ..
    /// [`RANK_HIGH`]), or `None` where that rank has no reed.
    ///
    /// The tremolo's ranks are the measured tongue retuned, as a tuner
    /// files it: the same geometry at a frequency the beat away, its profile
    /// derived again for it. L and H are the measured tongue scaled an
    /// octave down and up by the ratios of a bayan maker's slots (patent
    /// RU2233009, Table 3: F3, F4, F5 slots 35.4, 27.8, 20.5 mm long, root
    /// widths 4.23, 3.37, 2.66 mm, plates 2.7, 2.7, 2.2 mm) -- assumed, as no
    /// maker publishes tongue dimensions; the profile again derived for the
    /// frequency.
    pub fn rank_design(&self, rank: usize) -> Option<ReedDesign> {
        let middle = self.reed_design();
        let (sharp, flat) = self.tremolo_beats();
        let scaled = |frequency: f64, length: f64, width: f64, plate: f64| ReedDesign {
            frequency,
            length: middle.length * length,
            width: middle.width * width,
            set: middle.set * length,
            plate_thickness: middle.plate_thickness * plate,
            ..middle
        };
        match rank {
            RANK_LOW => Some(scaled(
                middle.frequency / 2.0,
                35.4 / 27.8,
                4.23 / 3.37,
                1.0,
            )),
            RANK_FLAT => Some(ReedDesign {
                frequency: middle.frequency + flat,
                ..middle
            }),
            RANK_MIDDLE => Some(middle),
            RANK_SHARP => Some(ReedDesign {
                frequency: middle.frequency + sharp,
                ..middle
            }),
            RANK_HIGH => Some(scaled(
                middle.frequency * 2.0,
                20.5 / 27.8,
                2.66 / 3.37,
                2.2 / 2.7,
            )),
            _ => None,
        }
    }

    pub fn pallet_design(&self) -> crate::pallet::PalletDesign {
        crate::pallet::PalletDesign {
            lift: self.values[PALLET_LIFT] * 1.0e-3,
            opening_time: self.values[PALLET_OPENING] * 1.0e-3,
            closing_time: self.values[PALLET_CLOSING] * 1.0e-3,
            seating_time: self.values[PAD_SEATING] * 1.0e-3,
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
