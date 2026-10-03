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
| `synth.envelope.amp.decay`, `synth.envelope.amp.sustain` | A free reed has no decay: it speaks as long as the key is down and the bellows moves, at the level the bellows sets. The bellows is played live (the modulation wheel or Expression; key velocity does nothing), not stored. |
| `synth.filter.cutoff`, `synth.filter.resonance` | The accordion's one filter is the cassotto, and its resonance and Q do nothing while it is off -- as it is by default. A cutoff knob that is silent most of the time would mislead. |
| `synth.filter.envelope.amount`, `synth.filter.lfo.amount`, `synth.filter.key_tracking` | No such thing in the instrument. |
| `synth.lfo.depth`, `synth.lfo.delay` | The tremolo's depth is the registers' choice of ranks; it has no delay. |
| `synth.oscillator.*` | No oscillators: reeds, and their make is the model. |
| `synth.amplifier.level`, `plugin.output.level`, `mixer.channel.level` | RackForge's master level owns the controller's volume, as on RF-5; Output Gain stays on the panel and in a hand-made link. |
| `mixer.channel.pan` | RF-Musette is mono before RackForge's stereo. |
| `performance.modulation`, `performance.expression`, `performance.sustain` | Performance MIDI, passed through to the plugin, which reads the wheel (CC 1/33) and Expression (CC 11/43) as the bellows itself -- the wheel as its push. An accordion has no sustain. |
| `rackforge.master.level`, `rackforge.master.pan` | RackForge's own, never a plugin's. |

Control Profile v1 has no role for a register, the bellows' direction or the
air button. The registers have a layout instead (below); the direction and
the air button are linked by hand -- RackForge's link modes toggle the
direction, or hold the air valve open while a pad is held -- and the
bellows' direction also answers CC 80 as a switch.

## The registers on the pads (milestone 9i)

`package/metadata/control-layout.json` lays RF-Musette out on RackForge's
slots, which every controller package fills with its own pads and buttons
(RackForge `plugins/control-layouts/README.md`). Switches and steps only:
the knobs keep the roles above, and the wheel is the bellows.

The pads go from the most common register to the rarest (milestone 9i
again): a register sounds only on an accordion with every rank it opens, so
M alone (Clarinet, on every accordion), then with M+ (a two-voice MM), with
L (a three-voice LMM), with H (a four-voice LMMH), and last with M− (a
musette or a five-voice); within each, fewer reeds first. The register's
choices are listed in the same order (their values Roland's, as saved), so
the PLAY panel's switches and ⏪ ⏩, which step through the list, agree
with the pads.

| Slot | Parameter | How |
| --- | --- | --- |
| `switch-1.1`-`1.8` | Register: Clarinet, Celeste, Bassoon, Bandoneon, Cello, Piccolo, Organ, Oboe | set |
| `switch-2.1`-`2.6` | Register: Harmonium, Violin, Tremolo, Musette, Accordion, Master | set |
| `switch-2.7` | Bass Register | cycle, all seven |
| `switch-2.8` | Key Touch | toggle |
| `step.down`, `step.up` (⏪ ⏩) | Register | step, not wrapping |
| `step-2.down`, `step-2.up` | Bass Register | step, not wrapping |

The layout says what each slot does; each controller package says which of
its pads fills it. RackForge's catalog numbers the slots by the pads' own
numbers (since 2026-10-02; before, by the notes they send): pad 1 fills
`switch-1.1` on every keyboard -- the top left on a KeyLab or a Launchkey,
the bottom left on an Akai -- so Clarinet is pad 1, Cello pad 5 and
Harmonium pad 9 or bank B's pad 1. A grid with unnumbered pads (an APC's)
takes its two bottom rows, the upper row's left half first.

A mapped pad does not play its note (RackForge's rule): with RF-Musette
loaded, the pads choose, they do not sound -- where before an MPK2's pads,
on channel 2, struck bass buttons. A player's own map, or a link learnt in
the rack, wins over the layout.

What each of the 64 bundled controller packages gets (derived with
RackForge's `factory_maps`, 2026-10-02, the catalog numbered by pad):

* **All 14 registers on pads or buttons (37):** Arturia KeyLab Essential
  mk3, KeyLab mkII 49/61/88, MiniLab 3,
  MiniLab 37, MiniLab mkII (pads 9-16 its CC buttons), BeatStep; Akai MPK
  mini mk3, MPK mini Plus, MPK249, MPK261, MPD218, APC Key 25 mk2, APC mini
  mk2, APC40 mkII; M-Audio Oxygen Pro 25/49/61, Hammer 88 Pro; Novation
  Launchkey Mini mk2/mk3/mk4, Launchkey mk3 and mk4, SL MkIII, Launch
  Control XL and XL 3; Korg nanoKONTROL2. Most of them step the register
  with ⏪ ⏩ too.
* **Eight on pads or buttons, Clarinet to Oboe -- every register of a two-
  or three-voice accordion and three of a four-voice's -- the rest on the Fn
  layer** (once the player names an Fn button) **or by stepping:** Akai
  LPD8, LPD8 mk2, MPK225 (steps); Alesis V; M-Audio Oxygen Pro Mini (steps);
  Novation FLkey 49/61, FLkey 2 49/61, Launch Control (steps), Launch
  Control 3.
* **Twelve on its DAW pads, Clarinet to Musette; Accordion and Master by
  stepping:** Arturia KeyLab mk3.
* **By stepping only:** Arturia KeyLab Essential (mk1), M-Audio Oxygen MKV,
  Novation Launchkey mk2.
* **Nothing:** controllers whose packages name no pad or step slot -- Akai
  MPK mini mkII, mini IV, mini Play mk3; M-Audio Keystation mk3; Novation
  FLkey 37, FLkey Mini, FLkey 2 37 and Mini 25. Their keys play as before.

No pad lights the register it chose: RackForge's packages send LED states
only on connecting, so far.

## LITTLE and the schema's pages

RackForge's LITTLE screen lists a plugin's parameters by the schema's pages
in the schema's order. The schema takes the PLAY surface's pages and order
(`crates/rf-musette-ui/src/panel.rs`): Play -- the registers, the bass, the
bellows, the voice -- then Reed, Cell & Pallet and Air & Bellows, the model.
Only Play's parameters are marked as not advanced.
