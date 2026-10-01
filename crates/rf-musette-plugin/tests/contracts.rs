//! The plugin's contract with RackForge: what it accepts, what it refuses,
//! and that the package's metadata says what the code does.

use rackforge_plugin_sdk::{
    MIDI2_FLAG_ORIGIN_7BIT, MIDI2_KIND_CONTROL_CHANGE, MIDI2_KIND_NOTE_ON, MidiEvent, MidiEvent2,
    ParameterEvent, Processor,
};
use rf_musette_dsp::{BellowsSource, parameters};
use rf_musette_plugin::{
    CC_BELLOWS_DIRECTION, CC_EXPRESSION, CC_EXPRESSION_LSB, MAX_FRAMES, MusetteProcessor,
    PARAMETER_GAIN, PROGRAM_RESEARCH, STATE_BYTES,
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
fn velocity_drives_the_bellows_until_expression_takes_it() {
    let mut plugin = prepared();
    run(&mut plugin, &[midi(0, [0x90, 69, 64])], &[]);
    let engine = plugin.engine().unwrap();
    assert!(engine.is_held(69));
    assert_eq!(engine.bellows().source(), BellowsSource::Velocity);
    assert_eq!(engine.bellows().intent(), 64.0 / 127.0);

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
