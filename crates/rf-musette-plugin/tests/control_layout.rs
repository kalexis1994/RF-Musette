//! Milestone 9i: the registers on a controller's pads. The package's
//! `metadata/control-layout.json` read as RackForge reads it. Predictions as
//! written in docs/ROADMAP.md before the layout; each test says which.

use rackforge_midi_api::ParameterLinkMode;
use rackforge_midi_api::control_layout::{CONTROL_LAYOUT_FILE, ControlLayout};
use rf_musette_dsp::parameters::{self, SPECS};

const LAYOUT: &str = include_str!("../../../package/metadata/control-layout.json");
const PARAMETERS: &str = include_str!("../../../package/metadata/parameters.json");

fn layout() -> ControlLayout {
    serde_json::from_str(LAYOUT).expect("the layout parses")
}

/// The n-th switch slot, from 0: `switch-1.1`-`1.8`, then `switch-2.1`-
/// `2.8` -- a layout's switches in order; each controller package says
/// which of its pads fills each (on the KeyLab Essential mk3, pad n of
/// bank A fills `switch-1.n`).
fn switch(n: usize) -> String {
    format!("switch-{}.{}", 1 + n / 8, 1 + n % 8)
}

/// What a press of `slot` does: the parameter and its mode.
fn at(layout: &ControlLayout, slot: &str) -> Option<(String, ParameterLinkMode)> {
    layout
        .slots
        .iter()
        .find(|mapping| mapping.slot.to_string() == slot)
        .map(|mapping| (mapping.parameter_id.clone(), mapping.mode.clone()))
}

/// 9i, prediction 1: RackForge's own validation passes; every parameter is
/// RF-Musette's, every value one of its choices.
#[test]
fn the_layout_is_one_rackforge_accepts() {
    assert_eq!(CONTROL_LAYOUT_FILE, "metadata/control-layout.json");
    let layout = layout();
    layout.validate().expect("RackForge accepts the layout");
    assert_eq!(layout.plugin_id, "org.rackforge.musette");
    for mapping in &layout.slots {
        let spec = SPECS
            .iter()
            .find(|spec| spec.id == mapping.parameter_id)
            .unwrap_or_else(|| panic!("no parameter {:?}", mapping.parameter_id));
        assert!(!spec.choices.is_empty(), "{}: not a choice", spec.id);
        let chosen = |value: f64| spec.choices.iter().any(|(v, _)| f64::from(*v) == value);
        let values: Vec<f64> = match &mapping.mode {
            ParameterLinkMode::Set { value } => vec![value.get()],
            ParameterLinkMode::Toggle { first, second } => vec![first.get(), second.get()],
            ParameterLinkMode::Cycle { values } => values.iter().map(|v| v.get()).collect(),
            ParameterLinkMode::Step { .. } => Vec::new(),
            other => panic!("{}: an unexpected mode {other:?}", mapping.slot),
        };
        for value in values {
            assert!(
                chosen(value),
                "{}: {value} is not a choice of {}",
                mapping.slot,
                spec.id
            );
        }
    }
}

/// The registers from the most common to the rarest (9i again): a register
/// sounds only on an accordion with every rank it opens, and the builds go
/// two-voice MM, three-voice LMM, four-voice LMMH, then the musette's M−.
/// So: M alone; with M+; with L; with H; with M−. Within a tier, fewer
/// reeds first; at equal reeds, the switch row's (Roland's) order.
fn by_how_common() -> Vec<u32> {
    use parameters::{RANK_FLAT, RANK_HIGH, RANK_LOW, RANK_SHARP, register_ranks};
    let mut registers: Vec<u32> = SPECS[parameters::REGISTER]
        .choices
        .iter()
        .map(|(value, _)| *value)
        .collect();
    registers.sort_by_key(|register| {
        let ranks = register_ranks(*register as usize).unwrap();
        let tier = if ranks[RANK_FLAT] {
            4
        } else if ranks[RANK_HIGH] {
            3
        } else if ranks[RANK_LOW] {
            2
        } else if ranks[RANK_SHARP] {
            1
        } else {
            0
        };
        let reeds = ranks.iter().filter(|open| **open).count();
        (tier, reeds, *register)
    });
    registers
}

/// 9i again, by the slots' order, prediction 1 as the layout says it: the
/// n-th switch slot sets the n-th register from the most common; the 15th
/// the bass register, cycled, and the 16th Key Touch, toggled; the step
/// pairs the register and the bass register, not wrapping.
#[test]
fn each_register_is_one_pad() {
    let layout = layout();
    let register = SPECS[parameters::REGISTER].id;
    let order = by_how_common();
    let name = |value: u32| {
        SPECS[parameters::REGISTER]
            .choices
            .iter()
            .find(|(v, _)| *v == value)
            .unwrap()
            .1
    };
    let names: Vec<&str> = order.iter().map(|value| name(*value)).collect();
    assert_eq!(
        names,
        [
            "Clarinet",
            "Celeste",
            "Bassoon",
            "Bandoneon",
            "Cello",
            "Piccolo",
            "Organ",
            "Oboe",
            "Harmonium",
            "Violin",
            "Tremolo",
            "Musette",
            "Accordion",
            "Master",
        ]
    );
    for (index, value) in order.iter().enumerate() {
        let slot = switch(index);
        let (id, mode) = at(&layout, &slot).expect("a pad");
        assert_eq!(id, register, "{slot}");
        assert_eq!(
            mode,
            ParameterLinkMode::Set {
                value: f64::from(*value).try_into().unwrap()
            },
            "{slot}, {}",
            name(*value)
        );
    }
    let (id, mode) = at(&layout, &switch(14)).expect("a pad");
    assert_eq!(id, SPECS[parameters::BASS_REGISTER].id);
    assert!(matches!(mode, ParameterLinkMode::Cycle { values } if values.len() == 7));
    let (id, mode) = at(&layout, &switch(15)).expect("a pad");
    assert_eq!(id, SPECS[parameters::KEY_TOUCH].id);
    assert!(matches!(mode, ParameterLinkMode::Toggle { .. }));
    for (pair, parameter) in [
        ("step", parameters::REGISTER),
        ("step-2", parameters::BASS_REGISTER),
    ] {
        for direction in ["down", "up"] {
            let (id, mode) = at(&layout, &format!("{pair}.{direction}")).expect("a step");
            assert_eq!(id, SPECS[parameter].id);
            assert!(matches!(mode, ParameterLinkMode::Step { wrap: false, .. }));
        }
    }
    // Switches and steps only: the knobs keep RackForge's roles, the wheel
    // is the bellows.
    assert!(layout.slots.iter().all(|m| !m.slot.is_continuous()));
}

/// 9i again, the panel, predictions 1 and 2: the register's choices are
/// listed in the pads' order -- the PLAY panel draws its switches by the
/// list, and RackForge lists them and steps ⏪ ⏩ through them by the
/// schema's -- their values Roland's, as saved.
#[test]
fn the_panel_and_the_steps_follow_the_pads() {
    let listed: Vec<u32> = SPECS[parameters::REGISTER]
        .choices
        .iter()
        .map(|(value, _)| *value)
        .collect();
    assert_eq!(listed, by_how_common());
    let schema: serde_json::Value = serde_json::from_str(PARAMETERS).unwrap();
    let register = schema["parameters"]
        .as_array()
        .unwrap()
        .iter()
        .find(|parameter| parameter["id"] == "register")
        .unwrap();
    let choices: Vec<u32> = register["kind"]["choices"]
        .as_array()
        .or_else(|| register["choices"].as_array())
        .expect("the register's choices")
        .iter()
        .map(|choice| choice["value"].as_u64().unwrap() as u32)
        .collect();
    assert_eq!(choices, listed);
    // The values themselves are unchanged: 0 Bassoon ... 13 Piccolo.
    let name = |value: u32| {
        SPECS[parameters::REGISTER]
            .choices
            .iter()
            .find(|(v, _)| *v == value)
            .unwrap()
            .1
    };
    assert_eq!(
        (name(0), name(8), name(11), name(13)),
        ("Bassoon", "Musette", "Clarinet", "Piccolo")
    );
    assert_eq!(SPECS[parameters::REGISTER].minimum, 0.0);
    assert_eq!(SPECS[parameters::REGISTER].maximum, 13.0);
}
