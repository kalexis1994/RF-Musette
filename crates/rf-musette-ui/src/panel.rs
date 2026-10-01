//! The panel map: which parameter sits on which page, in which group. A
//! test holds it to the engine's parameter table.

use rf_musette_dsp::parameters::SPECS;

pub struct Group {
    pub id: &'static str,
    pub title: &'static str,
    pub parameters: &'static [&'static str],
}

pub struct Page {
    pub id: &'static str,
    pub label: &'static str,
    pub groups: &'static [Group],
}

/// How a parameter is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Control {
    /// The treble's register switches.
    Registers,
    /// The bass side's register switches.
    BassRegisters,
    /// The air button: open while held.
    Air,
    /// An Off/On switch.
    Toggle,
    /// A choice among a few named values.
    Choice,
    Knob,
}

pub const PAGES: &[Page] = &[
    Page {
        id: "play",
        label: "Play",
        groups: &[
            Group {
                id: "treble",
                title: "Treble",
                parameters: &["register"],
            },
            Group {
                id: "bass",
                title: "Bass",
                parameters: &["bass_register", "left_hand", "split_point"],
            },
            Group {
                id: "bellows",
                title: "Bellows",
                parameters: &[
                    "bellows_direction",
                    "air_valve",
                    "auto_reverse",
                    "bellows_travel",
                    "bellows_smoothing",
                ],
            },
            Group {
                id: "voice",
                title: "Voice",
                parameters: &["gain", "tremolo", "cassotto", "pitch_a4"],
            },
        ],
    },
    Page {
        id: "reed",
        label: "Reed",
        groups: &[
            Group {
                id: "tongue",
                title: "Tongue",
                parameters: &[
                    "reed_frequency",
                    "reed_q",
                    "q_slope",
                    "reed_mode_ratio",
                    "reed_length",
                    "reed_width",
                ],
            },
            Group {
                id: "plate",
                title: "Plate",
                parameters: &[
                    "reed_set",
                    "plate_thickness",
                    "side_clearance",
                    "tip_clearance",
                ],
            },
            Group {
                id: "attack",
                title: "Attack",
                parameters: &["attack_kick"],
            },
        ],
    },
    Page {
        id: "cell",
        label: "Cell & Pallet",
        groups: &[
            Group {
                id: "cell",
                title: "Cell",
                parameters: &[
                    "cell_volume",
                    "tone_hole_area",
                    "tone_hole_depth",
                    "end_correction",
                ],
            },
            Group {
                id: "pallet",
                title: "Pallet",
                parameters: &["pallet_lift", "pallet_opening", "pallet_closing"],
            },
            Group {
                id: "cassotto",
                title: "Cassotto",
                parameters: &["cassotto_resonance", "cassotto_q"],
            },
        ],
    },
    Page {
        id: "air",
        label: "Air & Bellows",
        groups: &[
            Group {
                id: "jet",
                title: "Jet",
                parameters: &["contraction", "near_field_inertance", "swing_limit"],
            },
            Group {
                id: "bellows-model",
                title: "Bellows",
                parameters: &[
                    "bellows_response",
                    "bellows_ceiling",
                    "bellows_curve",
                    "bellows_area",
                    "bellows_volume",
                    "arm_speed",
                    "bellows_leak",
                    "reversal_time",
                ],
            },
            Group {
                id: "engine",
                title: "Engine",
                parameters: &["oversampling"],
            },
        ],
    },
];

/// Controls that do nothing while another is off: shown dimmed then. Each
/// is (the control, the switch it waits on).
pub const IDLE_UNLESS: &[(&str, &str)] = &[
    ("split_point", "left_hand"),
    ("bellows_travel", "auto_reverse"),
    ("cassotto_resonance", "cassotto"),
    ("cassotto_q", "cassotto"),
];

/// The page with this id, or the first.
pub fn page(id: &str) -> &'static Page {
    PAGES.iter().find(|page| page.id == id).unwrap_or(&PAGES[0])
}

/// The parameter's index in the engine's table.
pub fn index_of(id: &str) -> Option<usize> {
    SPECS.iter().position(|spec| spec.id == id)
}

/// How the parameter at `index` is drawn.
pub fn control(index: usize) -> Control {
    let spec = &SPECS[index];
    match spec.id {
        "register" => Control::Registers,
        "bass_register" => Control::BassRegisters,
        "air_valve" => Control::Air,
        _ if spec.choices == [(0, "Off"), (1, "On")] => Control::Toggle,
        _ if !spec.choices.is_empty() => Control::Choice,
        _ => Control::Knob,
    }
}

/// The switch a control waits on, if any.
pub fn waits_on(index: usize) -> Option<usize> {
    IDLE_UNLESS
        .iter()
        .find(|(control, _)| index_of(control) == Some(index))
        .and_then(|(_, switch)| index_of(switch))
}

/// The controls that wait on this switch.
pub fn waiting_on(switch: usize) -> impl Iterator<Item = usize> {
    IDLE_UNLESS
        .iter()
        .filter(move |(_, owner)| index_of(owner) == Some(switch))
        .filter_map(|(control, _)| index_of(control))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rf_musette_dsp::parameters::{COUNT, REGISTER};

    /// Prediction 1: every public parameter is on the panel exactly once.
    #[test]
    fn the_panel_maps_every_parameter_exactly_once() {
        let mut placed = vec![0usize; COUNT];
        for page in PAGES {
            for group in page.groups {
                assert!(!group.parameters.is_empty(), "{} is empty", group.id);
                for id in group.parameters {
                    let index = index_of(id).unwrap_or_else(|| panic!("{id} is no parameter"));
                    placed[index] += 1;
                }
            }
        }
        for (index, count) in placed.iter().enumerate() {
            assert_eq!(*count, 1, "{} is placed {count} times", SPECS[index].id);
        }
    }

    /// Prediction 7: the schema's pages are the panel's -- the same ids and
    /// names, in the same order, each parameter on the page it sits on here.
    #[test]
    fn the_schema_pages_are_the_panel_pages() {
        let ids: Vec<_> = PAGES.iter().map(|page| (page.id, page.label)).collect();
        assert_eq!(ids, rf_musette_dsp::parameters::PAGES.to_vec());
        for page in PAGES {
            for group in page.groups {
                for id in group.parameters {
                    let spec = &SPECS[index_of(id).unwrap()];
                    assert_eq!(spec.page, page.id, "{id} is on {} in the schema", spec.page);
                }
            }
        }
    }

    #[test]
    fn page_and_group_ids_are_unique() {
        let pages: Vec<_> = PAGES.iter().map(|page| page.id).collect();
        let groups: Vec<_> = PAGES
            .iter()
            .flat_map(|page| page.groups.iter().map(|group| group.id))
            .collect();
        for ids in [pages, groups] {
            for (place, id) in ids.iter().enumerate() {
                assert!(!ids[place + 1..].contains(id), "{id} repeats");
            }
        }
        assert_eq!(page("nowhere").id, "play");
    }

    #[test]
    fn idle_controls_wait_on_a_switch() {
        for (control, switch) in IDLE_UNLESS {
            let switch = index_of(switch).unwrap();
            assert!(index_of(control).is_some(), "{control}");
            assert_eq!(super::control(switch), Control::Toggle);
            assert!(waiting_on(switch).any(|index| Some(index) == index_of(control)));
        }
    }

    #[test]
    fn each_parameter_gets_its_control() {
        assert_eq!(control(REGISTER), Control::Registers);
        assert_eq!(control(index_of("oversampling").unwrap()), Control::Choice);
        assert_eq!(control(index_of("cassotto").unwrap()), Control::Toggle);
        assert_eq!(control(index_of("gain").unwrap()), Control::Knob);
    }
}
