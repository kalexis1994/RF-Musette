//! RackForge adapter. Device access and persistence remain host responsibilities.
//!
//! It validates every block before touching anything, turns MIDI into what
//! the engine understands -- keys, and the player's bellows -- and keeps the
//! state. The bellows arrives as key velocity until Expression (CC 11, with
//! CC 43 as its low bits) speaks; see `rf_musette_dsp::Bellows`.
//!
//! Every parameter is the engine's own, in physical units, indexed as in
//! `rf_musette_dsp::parameters::SPECS`; `package/metadata/parameters.json`
//! is generated from that table.

use rackforge_plugin_sdk::{
    MIDI_FAMILY_CONTROL, MIDI_FAMILY_NOTE, MIDI2_FLAG_ORIGIN_7BIT, MIDI2_KIND_CONTROL_CHANGE,
    MIDI2_KIND_NOTE_OFF, MIDI2_KIND_NOTE_ON, MidiEvent, MidiEvent2, ParameterEvent, Processor,
    export_processor,
};
use rf_musette_dsp::{Engine, PARAMETER_COUNT, Parameters, parameters};

pub const MAX_FRAMES: u32 = 4096;
pub const MAX_EVENTS: usize = 256;
/// Version 2 carries every parameter; version 1 carried the gain alone and
/// still loads.
pub const STATE_VERSION: u32 = 2;
pub const STATE_BYTES: usize = 12 + 8 * PARAMETER_COUNT;
const STATE_V1_BYTES: usize = 16;
pub const PARAMETER_GAIN: u32 = parameters::GAIN as u32;
/// The one program the research package ships.
pub const PROGRAM_RESEARCH: &str = "research";

/// Expression's high seven bits: the bellows.
pub const CC_EXPRESSION: u8 = 11;
/// Expression's low seven bits.
pub const CC_EXPRESSION_LSB: u8 = 43;
const CC_ALL_SOUND_OFF: u8 = 120;
const CC_ALL_NOTES_OFF: u8 = 123;

const STATE_MAGIC: &[u8; 4] = b"RFMU";

pub struct MusetteProcessor {
    engine: Option<Engine>,
    /// The values, kept here too so they survive until `prepare` builds an
    /// engine, and move into every engine it builds.
    parameters: Parameters,
    maximum_frames: u32,
    channels: u32,
    mono: [f32; MAX_FRAMES as usize],
}

impl Default for MusetteProcessor {
    fn default() -> Self {
        Self {
            engine: None,
            parameters: Parameters::default(),
            maximum_frames: 0,
            channels: 0,
            mono: [0.0; MAX_FRAMES as usize],
        }
    }
}

impl MusetteProcessor {
    /// The prepared engine, for tests and the laboratory.
    pub fn engine(&self) -> Option<&Engine> {
        self.engine.as_ref()
    }

    fn apply(&mut self, values: Parameters) {
        self.parameters = values;
        if let Some(engine) = &mut self.engine {
            for (index, value) in values.values().iter().enumerate() {
                engine.set_parameter(index, *value);
            }
        }
    }

    fn midi1(&mut self, event: &MidiEvent) {
        let Some(engine) = &mut self.engine else {
            return;
        };
        let [status, index, value] = event.data;
        match status & 0xf0 {
            0x90 if value > 0 => engine.note_on(index, f32::from(value) / 127.0),
            0x80 | 0x90 => engine.note_off(index),
            0xb0 => match index {
                CC_EXPRESSION => engine.bellows_mut().expression_msb(value),
                CC_EXPRESSION_LSB => engine.bellows_mut().expression_lsb(value),
                CC_ALL_SOUND_OFF | CC_ALL_NOTES_OFF => engine.reset(),
                _ => {}
            },
            _ => {}
        }
    }

    fn midi2(&mut self, event: &MidiEvent2) {
        let Some(engine) = &mut self.engine else {
            return;
        };
        let seven_bit = event.flags & MIDI2_FLAG_ORIGIN_7BIT != 0;
        match event.kind {
            MIDI2_KIND_NOTE_ON => {
                let velocity = if seven_bit {
                    f32::from((event.value >> 9) as u8) / 127.0
                } else {
                    // A genuine MIDI 2.0 Note On with zero velocity is not a
                    // Note Off, so the quietest wide note still opens its key.
                    (event.value & 0xffff) as f32 / 65535.0
                };
                engine.note_on(event.index, velocity);
            }
            MIDI2_KIND_NOTE_OFF => engine.note_off(event.index),
            MIDI2_KIND_CONTROL_CHANGE => {
                let byte = (event.value >> 25) as u8;
                match event.index {
                    // A 7-bit origin takes the byte path exactly, so a
                    // controller behaves the same whichever protocol carried it.
                    CC_EXPRESSION if seven_bit => engine.bellows_mut().expression_msb(byte),
                    CC_EXPRESSION => engine
                        .bellows_mut()
                        .expression_wide((f64::from(event.value) / f64::from(u32::MAX)) as f32),
                    CC_EXPRESSION_LSB if seven_bit => engine.bellows_mut().expression_lsb(byte),
                    CC_ALL_SOUND_OFF | CC_ALL_NOTES_OFF => engine.reset(),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

impl Processor for MusetteProcessor {
    fn prepare(&mut self, rate: f64, frames: u32, inputs: u32, outputs: u32) -> bool {
        if frames == 0 || frames > MAX_FRAMES || inputs != 0 || !(1..=2).contains(&outputs) {
            return false;
        }
        let Ok(mut engine) = Engine::new(rate as f32) else {
            return false;
        };
        for (index, value) in self.parameters.values().iter().enumerate() {
            engine.set_parameter(index, *value);
        }
        self.engine = Some(engine);
        self.maximum_frames = frames;
        self.channels = outputs;
        true
    }

    fn set_parameter(&mut self, index: u32, value: f64) -> bool {
        if !self.parameters.set(index as usize, value) {
            return false;
        }
        if let Some(engine) = &mut self.engine {
            engine.set_parameter(index as usize, value);
        }
        true
    }

    fn get_parameter(&self, index: u32) -> Option<f64> {
        self.parameters.get(index as usize)
    }

    fn reset(&mut self) {
        if let Some(engine) = &mut self.engine {
            engine.reset();
        }
    }

    fn load_preset(&mut self, id: &str) -> bool {
        if id != PROGRAM_RESEARCH {
            return false;
        }
        self.apply(Parameters::default());
        true
    }

    fn save_state(&self, destination: &mut [u8]) -> Option<usize> {
        let bytes = destination.get_mut(..STATE_BYTES)?;
        bytes[..4].copy_from_slice(STATE_MAGIC);
        bytes[4..8].copy_from_slice(&STATE_VERSION.to_le_bytes());
        bytes[8..12].copy_from_slice(&(PARAMETER_COUNT as u32).to_le_bytes());
        for (index, value) in self.parameters.values().iter().enumerate() {
            let at = 12 + 8 * index;
            bytes[at..at + 8].copy_from_slice(&value.to_le_bytes());
        }
        Some(STATE_BYTES)
    }

    fn load_state(&mut self, state: &[u8]) -> bool {
        if state.len() < 8 || &state[..4] != STATE_MAGIC {
            return false;
        }
        let word = |at: usize| {
            u32::from_le_bytes([state[at], state[at + 1], state[at + 2], state[at + 3]])
        };
        let value = |at: usize| {
            let mut bytes = [0; 8];
            bytes.copy_from_slice(&state[at..at + 8]);
            f64::from_le_bytes(bytes)
        };
        // Every value is checked before any is applied, so a rejected state
        // leaves the instrument exactly as it was.
        let mut loaded = Parameters::default();
        match word(4) {
            1 if state.len() == STATE_V1_BYTES => {
                if !loaded.set(parameters::GAIN, value(8)) {
                    return false;
                }
            }
            STATE_VERSION if state.len() == STATE_BYTES && word(8) as usize == PARAMETER_COUNT => {
                for index in 0..PARAMETER_COUNT {
                    if !loaded.set(index, value(12 + 8 * index)) {
                        return false;
                    }
                }
            }
            _ => return false,
        }
        self.apply(loaded);
        true
    }

    fn process(
        &mut self,
        input: &[f32],
        output: &mut [f32],
        midi: &[MidiEvent],
        parameters: &[ParameterEvent],
        frames: u32,
        inputs: u32,
        outputs: u32,
    ) {
        self.process_wide(
            input,
            output,
            midi,
            &[],
            parameters,
            frames,
            inputs,
            outputs,
        );
    }

    fn process_wide(
        &mut self,
        _input: &[f32],
        output: &mut [f32],
        midi: &[MidiEvent],
        midi2: &[MidiEvent2],
        parameters: &[ParameterEvent],
        frames: u32,
        inputs: u32,
        outputs: u32,
    ) {
        output.fill(0.0);
        let samples = (frames as usize).checked_mul(outputs as usize);
        if self.engine.is_none()
            || frames > self.maximum_frames
            || inputs != 0
            || outputs != self.channels
            || samples.is_none_or(|count| count > output.len())
            || !ordered(midi.iter().map(|event| event.frame), frames)
            || !ordered(midi2.iter().map(|event| event.frame), frames)
            || !ordered(parameters.iter().map(|event| event.frame), frames)
            || midi.iter().any(|event| !valid_midi1(event))
            || midi2
                .iter()
                .any(|event| event.channel >= 16 || event.index >= 128)
            || parameters.iter().any(|event| {
                let mut probe = Parameters::default();
                !probe.set(event.index as usize, event.value)
            })
        {
            return;
        }
        let (mut p, mut m, mut w) = (0, 0, 0);
        let mut start = 0;
        // Events land on their own frame: the block is rendered in spans
        // between them, so nothing waits for the block's end.
        while start < frames {
            while p < parameters.len() && parameters[p].frame == start {
                self.set_parameter(parameters[p].index, parameters[p].value);
                p += 1;
            }
            // Explicit stable tie order: parameters, MIDI 1.0, then MIDI 2.0.
            while m < midi.len() && midi[m].frame == start {
                self.midi1(&midi[m]);
                m += 1;
            }
            while w < midi2.len() && midi2[w].frame == start {
                self.midi2(&midi2[w]);
                w += 1;
            }
            let end = [
                parameters.get(p).map(|event| event.frame),
                midi.get(m).map(|event| event.frame),
                midi2.get(w).map(|event| event.frame),
            ]
            .into_iter()
            .flatten()
            .fold(frames, u32::min);
            let span = &mut self.mono[start as usize..end as usize];
            let engine = self.engine.as_mut().expect("prepared engine");
            // The engine applies the gain itself.
            engine.render(span);
            for (offset, sample) in span.iter().enumerate() {
                let frame = (start as usize + offset) * outputs as usize;
                for channel in 0..outputs as usize {
                    output[frame + channel] = *sample;
                }
            }
            start = end;
        }
    }
}

fn ordered(frames: impl Iterator<Item = u32>, block_frames: u32) -> bool {
    let mut previous = 0;
    let mut count = 0;
    for frame in frames {
        count += 1;
        if count > MAX_EVENTS || frame >= block_frames || frame < previous {
            return false;
        }
        previous = frame;
    }
    true
}

fn valid_midi1(event: &MidiEvent) -> bool {
    if !(1..=3).contains(&event.length) || event.data[0] < 128 {
        return false;
    }
    if event.data[1..event.length as usize]
        .iter()
        .any(|byte| *byte >= 128)
    {
        return false;
    }
    match event.data[0] & 0xf0 {
        0x80 | 0x90 | 0xa0 | 0xb0 | 0xe0 => event.length == 3,
        0xc0 | 0xd0 => event.length == 2,
        _ => true,
    }
}

export_processor!(MusetteProcessor,
    max_frames = 4096, max_input_channels = 0, max_output_channels = 2,
    max_midi_events = 256, max_parameter_events = 256, max_transfer_bytes = 4096,
    midi2 = {
        max_events = 256,
        families = MIDI_FAMILY_NOTE | MIDI_FAMILY_CONTROL
    }
);
