# RackForge control mapping

RackForge maps a hardware controller's knobs by meaning, not by number: a
controller package says what each knob is for, as a RackForge Control Profile
v1 role, and a plugin's parameter schema says which of its parameters
answers to each role (RackForge `docs/MIDI_PARAMETER_LINKS.md`). RF-Musette
publishes a role only where the accordion has the thing the role names, as
RF-5 does. These are defaults owned by the host: a link the player makes
always wins, and every control on the PLAY surface can be linked by hand from
RackForge's menu (right click, or a held touch; on the air button, its
label).

The roles live beside the parameter table, in `SEMANTIC_CONTROLS`
(`crates/rf-musette-dsp/src/parameters.rs`); `rf-musette-lab schema` writes
them into `package/metadata/parameters.json`, and the plugin's contract test
runs RackForge's own schema validator on the result.

## Published roles

| RackForge v1 role | RF-Musette parameter | Why |
| --- | --- | --- |
| `synth.envelope.amp.attack` | Pallet Opening Time | A key's attack is its pallet opening: the finger attack, about 0.05 s played normally (Llanos-Vázquez 2015, p164). |
| `synth.envelope.amp.release` | Pallet Closing Time | A key's release is its pallet closing. |
| `synth.lfo.rate` | Tremolo | The tremolo is the beat of M− against M+, heard as a periodic swell; its rate is what an LFO rate names. It sounds in the registers that open M− or M+ (Tremolo, Musette, Cello, Violin, Celeste, Accordion, Master). |

Of the 64 controller packages RackForge bundles (counted 2026-10-01), 53
give attack and release a knob or fader and 55 give one to LFO rate, so on
most of them three controls turn RF-Musette as soon as it is loaded.

## Deliberately unbound roles

| RackForge v1 role | Why RF-Musette does not publish it |
| --- | --- |
| `synth.envelope.amp.decay`, `synth.envelope.amp.sustain` | A free reed has no decay: it speaks as long as the key is down and the bellows moves, at the level the bellows sets. The bellows is played live (Expression, the modulation wheel or velocity), not stored. |
| `synth.filter.cutoff`, `synth.filter.resonance` | The accordion's one filter is the cassotto, and its resonance and Q do nothing while it is off -- as it is by default. A cutoff knob that is silent most of the time would mislead. |
| `synth.filter.envelope.amount`, `synth.filter.lfo.amount`, `synth.filter.key_tracking` | No such thing in the instrument. |
| `synth.lfo.depth`, `synth.lfo.delay` | The tremolo's depth is the registers' choice of ranks; it has no delay. |
| `synth.oscillator.*` | No oscillators: reeds, and their make is the model. |
| `synth.amplifier.level`, `plugin.output.level`, `mixer.channel.level` | RackForge's master level owns the controller's volume, as on RF-5; Output Gain stays on the panel and in a hand-made link. |
| `mixer.channel.pan` | RF-Musette is mono before RackForge's stereo. |
| `performance.modulation`, `performance.expression`, `performance.sustain` | Performance MIDI, passed through to the plugin, which reads the wheel (CC 1/33) and Expression (CC 11/43) as the bellows itself -- the wheel as its push or, with Mod Wheel on Bellows, as where it is. An accordion has no sustain. |
| `rackforge.master.level`, `rackforge.master.pan` | RackForge's own, never a plugin's. |

Control Profile v1 has no role for a register, the bellows' direction or the
air button. They are linked by hand -- RackForge's link modes set a register
from a button, toggle the direction, or hold the air valve open while a pad is
held -- and the bellows' direction also answers CC 80 as a switch. They gain
automatic defaults only through official RackForge roles, not names of
RF-Musette's own that no controller publishes.

## LITTLE and the schema's pages

RackForge's LITTLE screen lists a plugin's parameters by the schema's pages
in the schema's order. The schema takes the PLAY surface's pages and order
(`crates/rf-musette-ui/src/panel.rs`): Play -- the registers, the bass, the
bellows, the voice -- then Reed, Cell & Pallet and Air & Bellows, the model.
Only Play's parameters are marked as not advanced.
