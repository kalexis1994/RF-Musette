//! The plugin's contract with RackForge: what it accepts, what it refuses,
//! and that the package's metadata says what the code does.

use rackforge_plugin_sdk::{
    MIDI2_FLAG_ORIGIN_7BIT, MIDI2_KIND_CONTROL_CHANGE, MIDI2_KIND_NOTE_ON, MidiEvent, MidiEvent2,
    ParameterEvent, Processor,
};
use rf_musette_dsp::{BellowsSource, parameters};
use rf_musette_plugin::{
    CC_BELLOWS_DIRECTION, CC_EXPRESSION, CC_EXPRESSION_LSB, CC_MOD_WHEEL, CC_MOD_WHEEL_LSB,
    MAX_FRAMES, MusetteProcessor, PARAMETER_GAIN, PROGRAM_RESEARCH, STATE_BYTES,
};
use serde_json::Value;

const MANIFEST: &str = include_str!("../../../package/rackforge-plugin.toml");
const PARAMETERS: &str = include_str!("../../../package/metadata/parameters.json");
const PRESETS: &str = include_str!("../../../package/metadata/presets.json");
const RUNTIME: &str = include_str!("../../../package/metadata/runtime.json");

fn prepared() -> MusetteProcessor {
    let mut plugin = MusetteProcessor::default();
    assert!(plugin.prepare(48_000.0, 512, 0, 2));
    plugin
}

fn midi(frame: u32, data: [u8; 3]) -> MidiEvent {
    MidiEvent::new(frame, data, 3).expect("valid event")
}

fn run(plugin: &mut MusetteProcessor, midi: &[MidiEvent], midi2: &[MidiEvent2]) -> Vec<f32> {
    let mut output = vec![f32::NAN; 512 * 2];
    plugin.process_wide(&[], &mut output, midi, midi2, &[], 512, 0, 2);
    output
}

#[test]
fn prepare_accepts_only_what_the_component_was_exported_for() {
    let mut plugin = MusetteProcessor::default();
    assert!(!plugin.prepare(48_000.0, 0, 0, 2));
    assert!(!plugin.prepare(48_000.0, MAX_FRAMES + 1, 0, 2));
    assert!(
        !plugin.prepare(48_000.0, 512, 1, 2),
        "an instrument takes no input"
    );
    assert!(!plugin.prepare(48_000.0, 512, 0, 3));
    assert!(!plugin.prepare(0.0, 512, 0, 2));
    assert!(plugin.prepare(44_100.0, 512, 0, 1));
}

#[test]
fn velocity_never_moves_the_bellows_and_a_controller_does() {
    let mut plugin = prepared();
    run(&mut plugin, &[midi(0, [0x90, 69, 64])], &[]);
    let engine = plugin.engine().unwrap();
    assert!(engine.is_held(69));
    assert_eq!(engine.bellows().source(), BellowsSource::Resting);
    assert_eq!(engine.bellows().intent(), rf_musette_dsp::RESTING_PUSH);

    run(
        &mut plugin,
        &[
            midi(0, [0xb0, CC_EXPRESSION, 100]),
            midi(1, [0xb0, CC_EXPRESSION_LSB, 50]),
            midi(2, [0x90, 72, 127]),
            midi(3, [0x90, 69, 0]),
        ],
        &[],
    );
    let engine = plugin.engine().unwrap();
    assert_eq!(engine.bellows().source(), BellowsSource::Expression);
    assert_eq!(engine.bellows().intent(), (100.0 * 128.0 + 50.0) / 16383.0);
    assert!(
        engine.is_held(72) && !engine.is_held(69),
        "velocity 0 lets a key go"
    );

    // Prepared again -- a new rate, or the host's audio restarting -- the
    // bellows stays where the controller left it.
    assert!(plugin.prepare(44_100.0, 512, 0, 2));
    let engine = plugin.engine().unwrap();
    assert_eq!(engine.bellows().source(), BellowsSource::Expression);
    assert_eq!(engine.bellows().intent(), (100.0 * 128.0 + 50.0) / 16383.0);
}

#[test]
fn a_seven_bit_origin_takes_the_byte_path_exactly() {
    let wide = |kind, index, value| MidiEvent2 {
        frame: 0,
        kind,
        channel: 0,
        index,
        flags: MIDI2_FLAG_ORIGIN_7BIT,
        value,
        extra: 0,
    };
    let mut bytes = prepared();
    run(
        &mut bytes,
        &[midi(0, [0x90, 60, 90]), midi(0, [0xb0, CC_EXPRESSION, 77])],
        &[],
    );
    let mut upscaled = prepared();
    run(
        &mut upscaled,
        &[],
        &[
            wide(MIDI2_KIND_NOTE_ON, 60, 90 << 9),
            wide(MIDI2_KIND_CONTROL_CHANGE, CC_EXPRESSION, 77 << 25),
        ],
    );
    assert_eq!(
        bytes.engine().unwrap().bellows(),
        upscaled.engine().unwrap().bellows()
    );
    assert!(upscaled.engine().unwrap().is_held(60));
}

/// Milestone 8f, prediction 1: the modulation wheel asks what Expression
/// asks, at seven bits, at fourteen and at MIDI 2.0 width; wheel down is
/// silent.
#[test]
fn the_wheel_is_the_bellows() {
    let intent = |events: &[MidiEvent], wide: &[MidiEvent2]| {
        let mut plugin = prepared();
        run(&mut plugin, events, wide);
        let engine = plugin.engine().unwrap();
        (engine.bellows().source(), engine.bellows().intent())
    };
    assert_eq!(
        intent(&[midi(0, [0xb0, CC_MOD_WHEEL, 90])], &[]),
        intent(&[midi(0, [0xb0, CC_EXPRESSION, 90])], &[])
    );
    assert_eq!(
        intent(
            &[
                midi(0, [0xb0, CC_MOD_WHEEL, 90]),
                midi(0, [0xb0, CC_MOD_WHEEL_LSB, 33])
            ],
            &[]
        ),
        intent(
            &[
                midi(0, [0xb0, CC_EXPRESSION, 90]),
                midi(0, [0xb0, CC_EXPRESSION_LSB, 33])
            ],
            &[]
        )
    );
    let wide = |index| MidiEvent2 {
        frame: 0,
        kind: MIDI2_KIND_CONTROL_CHANGE,
        channel: 0,
        index,
        flags: 0,
        value: 0x9000_0000,
        extra: 0,
    };
    assert_eq!(
        intent(&[], &[wide(CC_MOD_WHEEL)]),
        intent(&[], &[wide(CC_EXPRESSION)])
    );
    assert_eq!(
        intent(&[midi(0, [0xb0, CC_MOD_WHEEL, 90])], &[]).0,
        BellowsSource::Expression
    );

    // Wheel down: a key pressed, nothing sounds -- with Key Touch off; on,
    // the bellows rests and the wheel moves nothing (9h again).
    let mut plugin = prepared();
    assert!(plugin.set_parameter(parameters::KEY_TOUCH as u32, 0.0));
    let out = run(
        &mut plugin,
        &[midi(0, [0xb0, CC_MOD_WHEEL, 0]), midi(0, [0x90, 69, 100])],
        &[],
    );
    assert!(out.iter().all(|x| *x == 0.0), "a sound with the wheel down");
}

/// Milestone 8i, withdrawn: a state saved with the wheel as where the
/// bellows is loads, and the wheel is the push, as it now always is.
#[test]
fn a_state_with_the_wheel_as_the_bellows_still_loads() {
    let plugin = prepared();
    let mut state = vec![0; STATE_BYTES];
    assert_eq!(plugin.save_state(&mut state), Some(STATE_BYTES));
    let at = 12 + 8 * parameters::MOD_WHEEL;
    state[at..at + 8].copy_from_slice(&1.0f64.to_le_bytes());
    let mut restored = prepared();
    assert!(restored.load_state(&state));
    assert_eq!(
        restored.get_parameter(parameters::MOD_WHEEL as u32),
        Some(parameters::WHEEL_PRESSURE)
    );
    run(&mut restored, &[midi(0, [0xb0, CC_MOD_WHEEL, 90])], &[]);
    assert_eq!(
        restored.engine().unwrap().bellows().source(),
        BellowsSource::Expression
    );
}

/// Milestone 8, prediction 6: the channel picks the side, as a V-Accordion
/// sends it -- the bass buttons on 2, the chords on 3, the treble on 1.
#[test]
fn the_channel_picks_the_side() {
    use rf_musette_dsp::PULL_REED;
    use rf_musette_dsp::parameters::{BASS_4, BASS_16};
    // A reed not yet built has not moved either.
    let moves = |plugin: &MusetteProcessor, pitch_class: usize, rank: usize| {
        plugin
            .engine()
            .unwrap()
            .bass_reed(pitch_class, rank, PULL_REED)
            .is_some_and(|(_, state)| state.zeta != 0.0 || state.velocity != 0.0)
    };
    let mut plugin = prepared();
    run(&mut plugin, &[midi(0, [0x91, 48, 100])], &[]);
    run(&mut plugin, &[], &[]);
    assert!(moves(&plugin, 0, BASS_16), "the C bass button");
    assert!(!plugin.engine().unwrap().is_held(48), "not the treble");

    let mut plugin = prepared();
    run(&mut plugin, &[midi(0, [0x92, 52, 100])], &[]);
    run(&mut plugin, &[], &[]);
    assert!(moves(&plugin, 4, BASS_4), "E on the chord ranks");
    assert!(!moves(&plugin, 4, BASS_16), "and not on the bass ranks");

    let mut plugin = prepared();
    run(&mut plugin, &[midi(0, [0x90, 60, 100])], &[]);
    assert!(plugin.engine().unwrap().is_held(60), "the treble");
    assert!(!moves(&plugin, 0, BASS_16));

    // The same at MIDI 2.0 width.
    let mut plugin = prepared();
    let note = MidiEvent2 {
        frame: 0,
        kind: MIDI2_KIND_NOTE_ON,
        channel: 1,
        index: 43,
        flags: 0,
        value: 0x8000,
        extra: 0,
    };
    run(&mut plugin, &[], &[note]);
    run(&mut plugin, &[], &[]);
    assert!(moves(&plugin, 7, BASS_16), "the G bass button");
}

#[test]
fn the_direction_switch_turns_the_bellows_as_a_saved_parameter() {
    let direction = parameters::BELLOWS_DIRECTION as u32;
    let mut plugin = prepared();
    assert_eq!(plugin.get_parameter(direction), Some(parameters::PULL));
    run(
        &mut plugin,
        &[midi(0, [0xb0, CC_BELLOWS_DIRECTION, 64])],
        &[],
    );
    assert_eq!(plugin.get_parameter(direction), Some(parameters::PUSH));
    assert_eq!(
        plugin
            .engine()
            .unwrap()
            .parameter(parameters::BELLOWS_DIRECTION),
        Some(parameters::PUSH)
    );
    let mut state = vec![0; STATE_BYTES];
    assert_eq!(plugin.save_state(&mut state), Some(STATE_BYTES));
    let mut restored = prepared();
    assert!(restored.load_state(&state));
    assert_eq!(restored.get_parameter(direction), Some(parameters::PUSH));

    run(
        &mut plugin,
        &[midi(0, [0xb0, CC_BELLOWS_DIRECTION, 63])],
        &[],
    );
    assert_eq!(plugin.get_parameter(direction), Some(parameters::PULL));
    // At MIDI 2.0 width the upper half of the range pushes.
    let wide = |value| MidiEvent2 {
        frame: 0,
        kind: MIDI2_KIND_CONTROL_CHANGE,
        channel: 0,
        index: CC_BELLOWS_DIRECTION,
        flags: 0,
        value,
        extra: 0,
    };
    run(&mut plugin, &[], &[wide(0x8000_0000)]);
    assert_eq!(plugin.get_parameter(direction), Some(parameters::PUSH));
    run(&mut plugin, &[], &[wide(0x7fff_ffff)]);
    assert_eq!(plugin.get_parameter(direction), Some(parameters::PULL));
}

#[test]
fn an_invalid_block_is_silenced_and_changes_nothing() {
    let mut plugin = prepared();
    let out_of_order = [midi(5, [0x90, 69, 64]), midi(2, [0x90, 70, 64])];
    let output = run(&mut plugin, &out_of_order, &[]);
    assert!(output.iter().all(|sample| *sample == 0.0));
    assert_eq!(plugin.engine().unwrap().held_count(), 0);

    let mut output = vec![1.0; 1024];
    let wrong_parameter = [ParameterEvent {
        frame: 0,
        index: 7,
        value: 0.5,
    }];
    plugin.process(&[], &mut output, &[], &wrong_parameter, 512, 0, 2);
    assert!(output.iter().all(|sample| *sample == 0.0));
}

#[test]
fn the_state_round_trips_and_a_bad_one_is_refused_whole() {
    let mut plugin = prepared();
    assert!(plugin.set_parameter(PARAMETER_GAIN, 0.75));
    assert!(plugin.set_parameter(parameters::REED_Q as u32, 180.0));
    let mut state = vec![0u8; 1024];
    let length = plugin.save_state(&mut state).unwrap();
    assert_eq!(length, STATE_BYTES);

    let mut restored = prepared();
    assert!(restored.load_state(&state[..length]));
    assert_eq!(restored.get_parameter(PARAMETER_GAIN), Some(0.75));
    assert_eq!(
        restored.get_parameter(parameters::REED_Q as u32),
        Some(180.0)
    );

    // One bad value refuses the whole state: the gain that came before it
    // in the file is not applied either.
    let mut corrupt = state.clone();
    corrupt[12..20].copy_from_slice(&0.5f64.to_le_bytes());
    let q = 12 + 8 * parameters::REED_Q;
    corrupt[q..q + 8].copy_from_slice(&f64::NAN.to_le_bytes());
    assert!(!restored.load_state(&corrupt[..length]));
    assert!(!restored.load_state(&state[..length - 1]));
    assert_eq!(restored.get_parameter(PARAMETER_GAIN), Some(0.75));
}

#[test]
fn a_state_from_a_build_with_fewer_parameters_still_loads() {
    // 0.2.0 saved 19 values; later builds append.
    let mut old = Vec::new();
    old.extend_from_slice(b"RFMU");
    old.extend_from_slice(&2u32.to_le_bytes());
    old.extend_from_slice(&19u32.to_le_bytes());
    for (index, spec) in parameters::SPECS.iter().take(19).enumerate() {
        let value = if index == parameters::REED_Q {
            180.0
        } else {
            spec.default
        };
        old.extend_from_slice(&value.to_le_bytes());
    }
    let mut plugin = prepared();
    assert!(plugin.set_parameter(parameters::PALLET_LIFT as u32, 5.0));
    assert!(plugin.load_state(&old));
    assert_eq!(plugin.get_parameter(parameters::REED_Q as u32), Some(180.0));
    assert_eq!(
        plugin.get_parameter(parameters::PALLET_LIFT as u32),
        Some(parameters::SPECS[parameters::PALLET_LIFT].default),
        "what the old state did not carry is the default"
    );
    // A count that does not match the length, or exceeds this build's, is refused.
    let mut wrong = old.clone();
    wrong[8..12].copy_from_slice(&20u32.to_le_bytes());
    assert!(!plugin.load_state(&wrong));
}

#[test]
fn a_first_version_state_still_loads() {
    let mut v1 = Vec::new();
    v1.extend_from_slice(b"RFMU");
    v1.extend_from_slice(&1u32.to_le_bytes());
    v1.extend_from_slice(&0.25f64.to_le_bytes());
    let mut plugin = prepared();
    assert!(plugin.set_parameter(parameters::REED_Q as u32, 180.0));
    assert!(plugin.load_state(&v1));
    assert_eq!(plugin.get_parameter(PARAMETER_GAIN), Some(0.25));
    // Everything else a first-version state did not carry is the default.
    assert_eq!(
        plugin.get_parameter(parameters::REED_Q as u32),
        Some(parameters::SPECS[parameters::REED_Q].default)
    );
}

#[test]
fn the_package_describes_this_build() {
    let version = env!("CARGO_PKG_VERSION");
    let runtime: Value = serde_json::from_str(RUNTIME).unwrap();
    assert_eq!(runtime["version"], version);
    assert!(
        MANIFEST.contains(&format!("version = \"{version}\"")),
        "rackforge-plugin.toml must carry the workspace version"
    );
    assert_eq!(runtime["state_version"], rf_musette_plugin::STATE_VERSION);
}

/// The surfaces the manifest declares are in the package, with what their
/// pages load (milestone 9a): PLAY, and CONFIG, which the one app starts
/// from its root's mark.
#[test]
fn the_surfaces_are_in_the_package() {
    let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../package/web");
    for surface in ["play", "config"] {
        assert!(
            MANIFEST.contains(&format!(
                "kind = \"{surface}\"\nentry = \"web/{surface}.html\""
            )),
            "the manifest declares {surface}"
        );
        let page = std::fs::read_to_string(web.join(format!("{surface}.html"))).unwrap();
        assert!(page.contains("id=\"plugin-root\""));
        assert_eq!(
            page.contains("data-surface=\"config\""),
            surface == "config",
            "{surface}.html's root"
        );
        for asset in ["styles.css", "app.js"] {
            assert!(
                page.contains(&format!("{asset}?v=")),
                "{surface}.html loads {asset}"
            );
            assert!(web.join(asset).is_file(), "{asset} is built");
        }
    }
    assert!(MANIFEST.contains("config_mode = true"));
    assert!(web.join("app_bg.wasm").is_file(), "app_bg.wasm is built");
    let glue = std::fs::read_to_string(web.join("app.js")).unwrap();
    assert!(
        glue.trim_end().ends_with("__wbg_init();"),
        "app.js starts the page"
    );
}

/// What follows the instrument in PLAY is suggested, not built in: RF-EQ,
/// flat, so it changes nothing until the player turns it.
#[test]
fn the_suggested_chain_is_an_eq_left_flat() {
    assert_eq!(MANIFEST.matches("[[suggested_chain]]").count(), 1);
    assert!(
        MANIFEST
            .contains("[[suggested_chain]]\nplugin = \"org.rackforge.rf-eq\"\npreset = \"flat\"")
    );
}

#[test]
fn the_parameter_schema_is_the_one_the_code_answers_to() {
    // The file is generated by `rf-musette-lab schema`; this holds it to the
    // table it was generated from, and to what the plugin answers.
    let schema: Value = serde_json::from_str(PARAMETERS).unwrap();
    let entries = schema["parameters"].as_array().unwrap();
    assert_eq!(
        entries.len(),
        parameters::COUNT,
        "run rf-musette-lab schema"
    );
    let plugin = MusetteProcessor::default();
    for (index, (entry, spec)) in entries.iter().zip(parameters::SPECS.iter()).enumerate() {
        assert_eq!(entry["index"], index, "{}", spec.id);
        assert_eq!(entry["id"], spec.id);
        assert_eq!(entry["page"], spec.page, "{}", spec.id);
        let kind = &entry["kind"];
        assert_eq!(
            kind["default"].as_f64(),
            plugin.get_parameter(index as u32),
            "{}",
            spec.id
        );
        if spec.choices.is_empty() {
            assert_eq!(kind["minimum"].as_f64(), Some(spec.minimum), "{}", spec.id);
            assert_eq!(kind["maximum"].as_f64(), Some(spec.maximum), "{}", spec.id);
        } else {
            let values: Vec<f64> = kind["choices"]
                .as_array()
                .unwrap()
                .iter()
                .map(|choice| choice["value"].as_f64().unwrap())
                .collect();
            let expected: Vec<f64> = spec.choices.iter().map(|(v, _)| f64::from(*v)).collect();
            assert_eq!(values, expected, "{}", spec.id);
        }
    }
}

/// Prediction 6 of milestone 9a: RackForge's own validator takes the
/// schema, and the controller roles land where the accordion has what they
/// name, on official roles only.
#[test]
fn the_controller_roles_are_ones_rackforge_knows() {
    use rackforge_plugin_api::{ParameterSchema, SemanticControlId, semantic_roles};
    let schema: ParameterSchema = serde_json::from_str(PARAMETERS).unwrap();
    schema.validate().unwrap();
    assert_eq!(schema.schema_version, 2);
    assert_eq!(
        schema.semantic_controls.len(),
        parameters::SEMANTIC_CONTROLS.len()
    );
    for (role, index) in parameters::SEMANTIC_CONTROLS {
        assert!(
            semantic_roles::V1.contains(&role),
            "{role} is no official role"
        );
        let role = SemanticControlId::new(role).unwrap();
        let parameter = schema.parameter_for_semantic_role(&role).unwrap();
        assert_eq!(parameter.index as usize, index);
    }
    let bound = |role: &str| {
        schema
            .parameter_for_semantic_role(&SemanticControlId::new(role).unwrap())
            .map(|parameter| parameter.id.as_str())
    };
    assert_eq!(
        bound(semantic_roles::SYNTH_AMP_ENVELOPE_ATTACK),
        Some("pallet_opening")
    );
    assert_eq!(
        bound(semantic_roles::SYNTH_AMP_ENVELOPE_RELEASE),
        Some("pallet_closing")
    );
    assert_eq!(bound(semantic_roles::SYNTH_LFO_RATE), Some("tremolo"));
    // The level belongs to RackForge's master, as on RF-5.
    for role in [
        semantic_roles::PLUGIN_OUTPUT_LEVEL,
        semantic_roles::SYNTH_AMPLIFIER_LEVEL,
        semantic_roles::RACKFORGE_MASTER_LEVEL,
    ] {
        assert_eq!(bound(role), None, "{role}");
    }
}

/// The catalog is the engine's table of programs, as `rf-musette-lab
/// schema` writes it, and loading a program sets what it says (milestone
/// 9e).
#[test]
fn the_program_catalog_is_the_engines() {
    use rf_musette_dsp::programs::PROGRAMS;
    let catalog: Value = serde_json::from_str(PRESETS).unwrap();
    let presets = catalog["presets"].as_array().unwrap();
    assert_eq!(presets.len(), PROGRAMS.len(), "run rf-musette-lab schema");
    for (preset, program) in presets.iter().zip(PROGRAMS) {
        assert_eq!(preset["id"], program.id);
        assert_eq!(preset["name"], program.name);
        assert_eq!(preset["bank"], program.bank);
        assert_eq!(preset["description"], program.description);
        let mut plugin = prepared();
        assert!(plugin.load_preset(program.id));
        for (index, value) in program.settings {
            assert_eq!(
                plugin.get_parameter(*index as u32),
                Some(*value),
                "{}: {}",
                program.id,
                parameters::SPECS[*index].id
            );
        }
    }
}

#[test]
fn every_shipped_program_is_one_the_instrument_answers_to() {
    let catalog: Value = serde_json::from_str(PRESETS).unwrap();
    let presets = catalog["presets"].as_array().unwrap();
    assert!(!presets.is_empty());
    let mut plugin = prepared();
    for preset in presets {
        let id = preset["id"].as_str().unwrap();
        assert!(plugin.load_preset(id), "{id} is shipped but refused");
    }
    assert!(plugin.load_preset(PROGRAM_RESEARCH));
    assert!(!plugin.load_preset("no-such-program"));
}
