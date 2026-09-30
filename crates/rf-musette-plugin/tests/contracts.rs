//! The plugin's contract with RackForge: what it accepts, what it refuses,
//! and that the package's metadata says what the code does.

use rackforge_plugin_sdk::{
    MIDI2_FLAG_ORIGIN_7BIT, MIDI2_KIND_CONTROL_CHANGE, MIDI2_KIND_NOTE_ON, MidiEvent, MidiEvent2,
    ParameterEvent, Processor,
};
use rf_musette_dsp::BellowsSource;
use rf_musette_plugin::{
    CC_EXPRESSION, CC_EXPRESSION_LSB, MAX_FRAMES, MusetteProcessor, PARAMETER_GAIN,
    PROGRAM_RESEARCH, STATE_BYTES,
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
    let mut state = [0u8; 64];
    let length = plugin.save_state(&mut state).unwrap();
    assert_eq!(length, STATE_BYTES);

    let mut restored = prepared();
    assert!(restored.load_state(&state[..length]));
    assert_eq!(restored.get_parameter(PARAMETER_GAIN), Some(0.75));

    let mut corrupt = state;
    corrupt[8..16].copy_from_slice(&f64::NAN.to_le_bytes());
    assert!(!restored.load_state(&corrupt[..length]));
    assert!(!restored.load_state(&state[..length - 1]));
    assert_eq!(restored.get_parameter(PARAMETER_GAIN), Some(0.75));
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
    let schema: Value = serde_json::from_str(PARAMETERS).unwrap();
    let parameters = schema["parameters"].as_array().unwrap();
    assert_eq!(parameters.len(), 1);
    let gain = &parameters[0];
    assert_eq!(gain["index"], PARAMETER_GAIN);
    let plugin = MusetteProcessor::default();
    assert_eq!(
        gain["kind"]["default"].as_f64(),
        plugin.get_parameter(PARAMETER_GAIN)
    );
    assert_eq!(gain["kind"]["minimum"].as_f64(), Some(0.0));
    assert_eq!(
        gain["kind"]["maximum"].as_f64(),
        Some(f64::from(rf_musette_dsp::GAIN_MAX))
    );
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
