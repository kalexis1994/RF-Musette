//! The panel map: which parameter sits on which page, in which group. A
//! test holds it to the engine's parameter table.

use rf_musette_dsp::parameters::SPECS;

pub struct Group {
    pub id: &'static str,
    pub title: &'static str,
    pub parameters: &'static [&'static str],
    /// Shown only while this choice has this value: a microphone layout's
    /// own settings, under that layout.
    pub shown_when: Option<(&'static str, f64)>,
}

/// Parameters kept only so those after them keep their places, and on no
/// page: the modulation wheel's mode, once Pressure or Bellows -- the wheel
/// as the bellows was withdrawn (docs/ROADMAP.md, 8i); and the smoothing of
/// velocity's push, since velocity no longer moves the bellows.
pub const RETIRED: &[&str] = &["mod_wheel", "bellows_smoothing"];

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
                shown_when: None,
            },
            Group {
                id: "bass",
                title: "Bass",
                parameters: &["bass_register", "left_hand", "split_point"],
                shown_when: None,
            },
            Group {
                id: "bellows",
                title: "Bellows",
                parameters: &[
                    "bellows_direction",
                    "air_valve",
                    "auto_reverse",
                    "bellows_travel",
                ],
                shown_when: None,
            },
            Group {
                id: "voice",
                title: "Voice",
                parameters: &["gain", "tremolo", "cassotto", "pitch_a4"],
                shown_when: None,
            },
        ],
    },
    Page {
        id: "mics",
        label: "Microphones",
        groups: &[
            Group {
                id: "layout",
                title: "Layout",
                parameters: &["mic_layout", "stereo_width", "perspective"],
                shown_when: None,
            },
            Group {
                id: "room",
                title: "Room",
                parameters: &["room_size", "room_hardness", "room_level"],
                shown_when: None,
            },
            Group {
                id: "internal",
                title: "Internal",
                parameters: &[
                    "internal_treble",
                    "internal_bass",
                    "internal_balance",
                    "internal_highpass",
                ],
                shown_when: Some(("mic_layout", 0.0)),
            },
            Group {
                id: "clip",
                title: "Clip-on",
                parameters: &[
                    "clip_distance",
                    "clip_spacing",
                    "clip_pattern",
                    "clip_balance",
                    "clip_highpass",
                ],
                shown_when: Some(("mic_layout", 1.0)),
            },
            Group {
                id: "spots",
                title: "Two spots",
                parameters: &[
                    "spots_treble_distance",
                    "spots_bass_distance",
                    "spots_pattern",
                    "spots_ambience",
                ],
                shown_when: Some(("mic_layout", 2.0)),
            },
            Group {
                id: "ortf",
                title: "ORTF pair",
                parameters: &["ortf_distance", "ortf_height", "ortf_pattern"],
                shown_when: Some(("mic_layout", 3.0)),
            },
            Group {
                id: "spaced",
                title: "Spaced pair",
                parameters: &["spaced_distance", "spaced_spacing", "spaced_pattern"],
                shown_when: Some(("mic_layout", 4.0)),
            },
            Group {
                id: "single",
                title: "One mic",
                parameters: &["single_distance", "single_pattern"],
                shown_when: Some(("mic_layout", 5.0)),
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
                shown_when: None,
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
                shown_when: None,
            },
            Group {
                id: "attack",
                title: "Attack",
                parameters: &["attack_kick"],
                shown_when: None,
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
                shown_when: None,
            },
            Group {
                id: "pallet",
                title: "Pallet",
                parameters: &[
                    "pallet_lift",
                    "pallet_opening",
                    "pallet_closing",
                    "pad_seating",
                ],
                shown_when: None,
            },
            Group {
                id: "cassotto",
                title: "Cassotto",
                parameters: &["cassotto_resonance", "cassotto_q"],
                shown_when: None,
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
                shown_when: None,
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
                shown_when: None,
            },
            Group {
                id: "engine",
                title: "Engine",
                parameters: &["oversampling"],
                shown_when: None,
            },
        ],
    },
];

/// Controls that do nothing unless another is set so: shown dimmed
/// otherwise. Each is the control and what it waits on -- any of the
/// switches at its value.
pub const IDLE_UNLESS: &[(&str, &[(&str, f64)])] = &[
    ("split_point", &[("left_hand", 1.0)]),
    // The travel is Auto Reverse's, and the wheel's range as the bellows.
    ("bellows_travel", &[("auto_reverse", 1.0)]),
    ("cassotto_resonance", &[("cassotto", 1.0)]),
    ("cassotto_q", &[("cassotto", 1.0)]),
];

/// Whether a group is shown with these values.
pub fn shown(group: &Group, values: &[f64]) -> bool {
    group.shown_when.is_none_or(|(id, value)| {
        index_of(id)
            .and_then(|index| values.get(index))
            .is_some_and(|current| *current == value)
    })
}

/// Whether changing this parameter shows or hides a group.
pub fn reveals(index: usize) -> bool {
    PAGES
        .iter()
        .flat_map(|page| page.groups.iter())
        .any(|group| {
            group
                .shown_when
                .is_some_and(|(id, _)| index_of(id) == Some(index))
        })
}

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

/// Whether the control at `index` does nothing with these values.
pub fn idle(index: usize, values: &[f64]) -> bool {
    IDLE_UNLESS
        .iter()
        .find(|(control, _)| index_of(control) == Some(index))
        .is_some_and(|(_, switches)| {
            !switches.iter().any(|(switch, on)| {
                index_of(switch)
                    .and_then(|switch| values.get(switch))
                    .is_some_and(|value| value == on)
            })
        })
}

/// The controls that wait on this switch.
pub fn waiting_on(switch: usize) -> impl Iterator<Item = usize> {
    IDLE_UNLESS
        .iter()
        .filter(move |(_, switches)| {
            switches
                .iter()
                .any(|(owner, _)| index_of(owner) == Some(switch))
        })
        .filter_map(|(control, _)| index_of(control))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rf_musette_dsp::parameters::{COUNT, REGISTER};

    /// Prediction 1: every public parameter is on the panel exactly once;
    /// a retired one on none.
    #[test]
    fn the_panel_maps_every_parameter_exactly_once() {
        let mut placed = vec![0usize; COUNT];
        for id in RETIRED {
            let index = index_of(id).unwrap_or_else(|| panic!("{id} is no parameter"));
            placed[index] += 1;
        }
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

    /// Each microphone layout shows its own settings and no other's.
    #[test]
    fn a_layout_shows_its_own_settings() {
        let mut values: Vec<f64> = SPECS.iter().map(|spec| spec.default).collect();
        let layout = index_of("mic_layout").unwrap();
        let mics = page("mics");
        for choice in 0..SPECS[layout].choices.len() {
            values[layout] = choice as f64;
            let shown: Vec<_> = mics
                .groups
                .iter()
                .filter(|group| group.shown_when.is_some() && shown(group, &values))
                .collect();
            // Dry, the last, has none.
            let expected = usize::from(choice < SPECS[layout].choices.len() - 1);
            assert_eq!(shown.len(), expected, "layout {choice}");
        }
        assert!(reveals(layout));
        assert!(!reveals(index_of("gain").unwrap()));
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
        for (control, switches) in IDLE_UNLESS {
            assert!(index_of(control).is_some(), "{control}");
            for (switch, value) in *switches {
                let switch = index_of(switch).unwrap();
                assert!(
                    SPECS[switch]
                        .choices
                        .iter()
                        .any(|(v, _)| f64::from(*v) == *value),
                    "{control} waits on a value {} does not have",
                    SPECS[switch].id
                );
                assert!(waiting_on(switch).any(|index| Some(index) == index_of(control)));
            }
        }
    }

    #[test]
    fn the_travel_works_for_auto_reverse() {
        let travel = index_of("bellows_travel").unwrap();
        let mut values: Vec<f64> = SPECS.iter().map(|spec| spec.default).collect();
        assert!(idle(travel, &values));
        values[index_of("auto_reverse").unwrap()] = 1.0;
        assert!(!idle(travel, &values));
        assert!(!idle(index_of("gain").unwrap(), &values));
    }

    #[test]
    fn each_parameter_gets_its_control() {
        assert_eq!(control(REGISTER), Control::Registers);
        assert_eq!(control(index_of("oversampling").unwrap()), Control::Choice);
        assert_eq!(control(index_of("cassotto").unwrap()), Control::Toggle);
        assert_eq!(control(index_of("gain").unwrap()), Control::Knob);
    }
}
