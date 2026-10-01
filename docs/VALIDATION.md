# Validation receipts

Append-only. Each entry records what was run, where, what it showed, and what
it did not demonstrate.

## 2026-09-30 — 0.1.0, the skeleton

**Environment.** Windows Server 2022, Rust 1.98.0
(`x86_64-pc-windows-msvc`), `CARGO_INCREMENTAL=0`. Sibling RackForge checkout
at `14f486a` (0.1.24), whose SDK, plugin API, core and store crates are
identical to `988e4b7` on `main`, the commit CI pins. RackForge's validators
were the release builds already in that checkout (`rackforge-core` built
2026-09-22, `rackforge-store` 2026-09-26).

**Checks.**

| Check | Result |
| --- | --- |
| `cargo test --locked --workspace` | 24 passed (engine 9, laboratory 7, plugin contract 8) |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | Clean |
| `cargo fmt --all -- --check` | Clean |
| `cargo build --locked --release --target wasm32-unknown-unknown -p rf-musette-plugin` | 30 685-byte component |
| `rf-musette-lab package` | `PLUGIN_PACKAGE_VALID` (api 1.11); preset `research` loaded; gain round trip; `PLUGIN_SMOKE_OK peak=0.000000 state_bytes=16`; optimised 30 685 → 21 840 bytes; `RF-Musette-0.1.0.rfplugin`, 12 331 bytes, sha256 `95ca06c090e988528ad1f7efabbeebac59878f5c1589f65d2198a7afaf457bb1` |
| `rf-musette-lab render` of a two-note score with a bellows move | 4.00 s (1.5 s lead-in), peak 0, report written; a second render to the same path refused |

**Not demonstrated.** Any sound: the engine is silent by design. The Desktop
audition has not been run. CI has not run: the repository has no remote yet.

## 2026-09-30 — 0.2.0, milestone 1: one reed

**Environment.** As for 0.1.0, same host checkout and validators.

**Checks.**

| Check | Result |
| --- | --- |
| `cargo test --locked --workspace` | 45 passed (engine 20, milestone 1 predictions 8, plugin contract 9, laboratory 8); 1 prediction ignored as a known defect; 4 diagnoses ignored (run by hand) |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | Clean |
| `cargo fmt --all -- --check` | Clean |
| `rf-musette-lab package` | `PLUGIN_PACKAGE_VALID` (api 1.11); 19 parameters on 4 pages; `PLUGIN_SMOKE_OK peak=0.000000 state_bytes=164`; optimised 56 951 → 46 246 bytes; `RF-Musette-0.2.0.rfplugin`, 23 437 bytes, sha256 `0687667b1ee0d1d1c59832ec55e61c605c339278d7668e145451e9427a8e737d`. The smoke test plays no F4, so its peak says nothing about the reed. |
| Native cost, 30 s of F4 at 2× | 0.463 s wall with the WAV write: 65× real time, 321 ns per host sample |

**The predictions** (`crates/rf-musette-analysis/tests/milestone_1.rs`,
96 kHz, defaults):

| Prediction | Measured | Verdict |
| --- | --- | --- |
| 1. No reachable parameter set blows up; passive with the supply off | 119 sets (every extreme, 60 random) finite; energy never rose | Met |
| 2. Onset tens to ~100 Pa, offset below it | Onset 32.3 Pa (linear instability of the equilibrium); let down from 300 Pa, silent at 8.1 Pa | Met |
| 3. Pitch below the mode, sagging −2 to −40 cents from 100 to 900 Pa | −6.3 cents below the mode at every pressure; −0.4 cents from 100 to 900 Pa | **Not met** (known defect, MODEL.md) |
| 4. Choking by a cell near the reed's frequency | -- | Moved to milestone 2 |
| 5. Level grows over tens of dB | 40.9 dB from 60 to 3000 Pa, monotonic (Misdariis: ~40 dB) | Met |
| Tip passes through the 3 mm plate at 300 Pa (Ziegenhals: > 4 mm at mf) | 4.94 mm | Met |
| The tongue moves sinusoidally | 2nd-4th harmonics 58.8, 62.4, 72.1 dB down | Met |
| The scheme computes the model | RK4 at 32×: −0.065 and −0.035 cents, +0.26 and +0.23 % (150, 600 Pa) | Met |
| The shipping rate converges | Against 8×: −0.047 cents, +0.36 % | Met |

**How the model got here** (all measured with `tests/diagnosis.rs`): with a
uniform tongue and no cell, silent below ~3 kPa; the scheme exonerated by
RK4; every other constant swept without effect; the gap-inertia hypothesis
refuted (onset still ~600 Pa); the cell's air brought the onset to 69 Pa and
the tongue's measured profile to 32 Pa.

**Listening file.** `renders/milestone-1-f4.wav` (ignored by git; regenerate
with `rf-musette-lab render --output … --score scores/milestone-1.score
--tail 1.5`): 1.5 s of lead-in, F4 at velocities 40, 80 and 120, then a held
note under a CC 11 swell. Peak 2.64 Pa at 1 m; the copy for listening,
`renders/milestone-1-f4-listen.wav`, is the same at `--set gain=0.3` (peak
0.79).

**Heard (2026-09-30).** "Bastante bien, pero se escucha escalonado." The
steps were the score's: its swell moved CC 11 in twelve jumps half a second
apart, and the level rose in plateaus at exactly that spacing. Rendered again
with CC 11 every 10 ms, as an expression pedal sends it
(`scores/continuous-swell.score`), the level moves 0.11 dB per 20 ms window
(median); the larger movements, up to 1.3 dB per window, are all the reed
building up from rest near its threshold and dying away, monotonic, not
steps.

**Desktop audition.** `rf-musette-lab audition` built, validated, installed
and selected 0.2.0 and launched Desktop (`rackforge-desktop.exe` from the
sibling checkout; receipt `dist/audition/0.2.0-1790797878708448200-14956/`).
Its log shows 100-block windows at 14-15 µs average, 38 µs worst, of a
10 ms deadline, no misses -- with no key played, so it says nothing yet about
the reed's cost. The first run had stopped on the laboratory's own check:
since RackForge 0.1.24 the packer optimises the component, so the installed
WASM no longer equals the build output; the check now compares it with the
archive's own copy, by the CRC-32 the zip records.

**Not demonstrated.** Wasm fuel and the Raspberry Pi. The reed played in
Desktop, and heard. Anything but F4.

## 2026-09-30 — 0.3.0, milestone 2: the pallet

**Checks.** `cargo test --locked --workspace`: 51 passed (engine 22,
milestone 1 predictions 8, milestone 2 predictions 3, plugin contract 10,
laboratory 8); 2 predictions ignored as known defects (pitch sag; finger
attack); 8 diagnoses ignored (run by hand). Clippy and rustfmt clean.
`rf-musette-lab package`: `PLUGIN_PACKAGE_VALID` (22 parameters), smoke
state 188 bytes, `RF-Musette-0.3.0.rfplugin` 24 208 bytes, sha256
`ce19e6a7900374450a2b0e05a02cb4cd96f014c039f5c338a9dd16cbd1b38b96`. A state
saved by 0.2.0 (19 values) loads, the three new parameters at their defaults.

**The predictions** (`crates/rf-musette-analysis/tests/milestone_2.rs`,
96 kHz, defaults, 300 Pa unless stated):

| Prediction | Measured | Verdict |
| --- | --- | --- |
| 1. Finger attack 50-140 ms (Llanos 2014) | 249 ms at 400 Pa, 730 ms at 100 Pa; through Llanos's ~80 ms windows 260 and 750 ms, where an instant onset reads 40 ms | **Not met** (known defect) |
| 2. A part-open pallet bends down ≥ 10 cents, never up | 0.8 → 0.2 of the lift: −0.0, −0.7, −1.4, −2.5, −4.7, −6.9, −15.1 cents; silent below | Met |
| 3. Choking: onset at 0.9-1.0 of the reed's frequency ≥ 4× the onset at 1.3 | 35.5 Pa at 1.3; 119.4 (3.4×) at 1.0; 217.1 (6.1×) at 0.95; 355.8 (10×) at 0.9; 31.7 at 2.0 | Met in shape; the 4× not met at exactly 1.0. The literature's claim is qualitative ("far above normal"); the test now asserts the monotone rise and 4× by 0.95, and says so |
| 4. A closed pallet is exact silence | exact zeros after 3 s; near the host's Nyquist while closing −129, −121, −133 dB re the note | Met |

**Investigating the attack** (`tests/diagnosis.rs`):

| Measurement | Result |
| --- | --- |
| Attack against pressure (60-800 Pa) | 1470, 730, 402, 249, 139 ms: exactly 5.2/σ, the linear growth rate's e-folds |
| The pallet's kick | 0.015 mm at 100 Pa, 0.059 mm at 400 Pa: −39 and −38 dB of the swing; tip stiffness 347 N/m |
| Growth per cycle | 0.06 at 400 Pa; Cottingham & Reed measured at most 0.086 on a reed-organ C3 (Forum Acusticum 1999, Fig. 4: 4.5 /s at 0.25 kPa, 8.3 at 0.5, 11.3 at 1 kPa) |
| Q 95-1000; hole 150-25 mm² | mf 108-372 ms; p never below 339 ms; onset stays 32-104 Pa |
| Set 0.15-0.8 mm; clearances 0.015-0.2 mm | mf 175-222 ms: the adjustment barely matters, where Llanos attributes the neighbouring-semitone spread to it |

**Also found:** the swing keeps growing with pressure (4.9, 8.2, 11 mm at
0.3, 0.9, 3 kPa); a measured reed's saturates. With milestone 2's three new
parameters the stability test's random draw reached a 65 mm reed at 6 kPa
swinging 78 mm, finite and passive but unphysical; its 5 cm bound was
removed and the defect recorded.

**Not demonstrated.** Heard. Wasm fuel and the Pi. Anything but F4.

## 2026-09-30 — milestone 2b begun: Llanos-Vázquez's thesis, and one hypothesis refuted

**Read.** R. Llanos-Vázquez, *Acústica del acordeón* (UPV/EHU 2015), in full
from the user's copy; notes in `docs/research/LLANOS-2015.md`, credited in
SOURCES.md. It gives no blowing pressures or reed dimensions; it gives the
attack's growth (Fig. 4.11: ~0.7-0.8 dB/ms, τ ≈ 11 ms for A4 mf), a normal
finger attack of ~0.05 s (p164, now the pallet's opening time) and a summary
of Ricot's model (Appendix 1).

**Corrected.** The gap-inertia hypothesis had been "refuted" with an onset
read by ramping up from perfect rest, a method found ill-posed the same day;
MODEL.md now says the refutation did not stand.

**Refuted properly.** The passage inertia (Tarnopolsky's ρd/(CF) element
by element, entered through its kinetic energy, κ 0.5-2), RK4 at 1.536 MHz:

| | σ at 100 / 400 Pa | Step attack at 100 / 400 Pa | Swing at 300 Pa |
| --- | --- | --- | --- |
| Without | 7.0 / 21.2 /s | 553 / 193 ms | 4.92 mm |
| κ 1 | 6.8 / 21.1 /s | 558 / 193 ms | 4.74 mm |

Clearance 0.015-0.06 mm: σ 21.6-20.3 /s with it, 21.7-20.4 without. Set 0.5,
0.2, 0.1 mm: σ 21.1, 19.3, 10.6 /s. Predictions 1 and 2 of 2b not met; the
mechanism is not built into the shipping step.

**Checks.** 51 passed, 11 ignored (2 known defects, 8 diagnoses, 1 experiment).
The part-open pallet's deepest bend is −16.5 cents with the 50 ms opening.

## 2026-09-30 -- Milestone 2b, second attempt: the sink flow

**Read.** Misdariis, Ricot & Caussé, CFA 2000 (hal-01161356), in full, text
and figures; notes in `docs/research/MISDARIIS-2000.md`. No equations or K₀.
Fig. 3 digitised: regime 1 falls linearly at −13.6 cents/kPa to 3.4 kPa.

**Experiment** (`tests/sink_flow.rs`, RK4 at 1.536 MHz, F4 defaults). (A) is
the sink's steady suction on the upstream face; (B) is its inertance,
ρ ln(w/g)/(π dx):

| | σ(400) | Step attack 100 / 400 Pa | Small-oscillation pitch 100 / 300 / 900 Pa | Its sag | Full-swing sag | Swing(300) |
| --- | --- | --- | --- | --- | --- | --- |
| None | 21.2 /s | 553 / 193 ms | −7.02 / −7.15 / −7.01 cents | +0.01 | −0.5 | 4.92 mm |
| A | 20.4 /s | 585 / 204 ms | −7.59 / −9.40 / −14.57 cents | −6.98 | −0.0 | 4.83 mm |
| B | 21.4 /s | 545 / 190 ms | −6.55 / −6.20 / −3.81 cents | +2.74 | +0.1 | 5.01 mm |
| A+B | 20.6 /s | 578 / 201 ms | −7.18 / −8.54 / −11.28 cents | −4.10 | +0.5 | 4.92 mm |

(B) refuted (prediction 2). (A) meets prediction 1 about the equilibrium,
−8.7 cents/kPa, and not in the model's ±5 mm swing. Nothing is built into
the shipping step; the shipping tests are untouched.

## 2026-09-30 -- Milestone 2c: why the swing does not saturate

**Measured** (`tests/saturation.rs`: the tongue swung by hand at the
playing frequency, the air's work per cycle over the tongue's damping, RK4
at 1.536 MHz). The balance, where a free reed settles:

| Supply | Without drag | With the plate's drag (Keulegan & Carpenter, local KC) |
| --- | --- | --- |
| 100 Pa | ~1.7 mm | ~1.1 mm |
| 300 Pa | ~5.0 mm | ~3.4 mm |
| 900 Pa | ~8.2 mm | ~6.4 mm |
| 3 kPa | ~11 mm | ~9.1 mm |

Without drag it reproduces the free reed's swings (4.9, 8.2, 11 mm), so the
diagnosis computes the model. Almost all the energy enters above the plate;
only the tip emerging beyond the plate returns it. Drag: 85 µJ per cycle at
5 mm against Q's 108. Predictions 1 and 2 of 2c met; drag not built.

**Level against flow** (`level_against_flow`, the shipping step at
192 kHz, 1.5 s from rest then 0.5 s measured):

| Supply | Mean flow | Swing | Fundamental of dU/dt | 2nd, 3rd | dB per doubling of flow |
| --- | --- | --- | --- | --- | --- |
| 100 Pa | 7.1 l/min | 1.58 mm | −4.8 dB | −12.6, −6.8 | +9.1 |
| 200 Pa | 14.2 l/min | 3.51 mm | +2.0 dB | −10.9, −5.4 | +6.6 |
| 300 Pa | 23.4 l/min | 4.93 mm | +5.1 dB | −7.5, −2.2 | +4.3 |
| 600 Pa | 53.5 l/min | 7.01 mm | +8.5 dB | +0.6, +2.9 | +2.8 |
| 900 Pa | 78.3 l/min | 8.17 mm | +9.9 dB | +3.9, +4.8 | +2.6 |
| 3 kPa | 209 l/min | 11.1 mm | +11.9 dB | +11.4, +10.2 | +1.0 |

Measured (Nussbaumer & Agarwal, ICA 2016, Fig. 6, reed 2): ~+7.6 dB per
doubling from 20 to 52 l/min, the fundamental always the strongest.

## 2026-09-30 -- Milestone 2c: the voiced swing limit

**Built.** c = κ ρ v w L (ζ/w)² added to the tongue's damping in
`reed::step` (lagged, so the energy identity keeps an extra −c ζ′² and the
scheme stays passive) and in the RK4 reference. A new parameter, `swing_limit`
(index 22, default 0.5, "Assumed, to be voiced by ear"); the state grows to
23 values and still loads the 22-value states of 0.3.0.

**Calibrated** on the energy balance (`the_voiced_limit`), κ 0.5:
3.71, 4.76, 5.24, 5.49, 5.64 mm at 0.3, 0.6, 1, 1.5, 3 kPa.

**Free reed** (`level_against_flow`, the shipping step): 3.72, 4.76, 5.16,
5.50, 5.65 mm at 0.3, 0.6, 0.9, 1.5, 3 kPa -- the balance within 1 %. Level
+4.2-4.7 dB per doubling of flow past 600 Pa; the second harmonic passes
the fundamental only at 3 kPa.

**Milestones** with κ 0.5: onset 32.3 Pa, offset 8.5 Pa; swing 3.73 mm at
300 Pa; level span 36.0 dB from 60 Pa to 3 kPa; the scheme against RK4
within 0.07 cents and 0.4 %; the deepest bend −16.0 cents; choking 35.5,
119, 217, 356 Pa; closed pallet −105 dB or less. Pitch 100 → 900 Pa:
−2.7 cents.

**Checks.** The workspace's tests all pass; the sag and attack defects
remain ignored.

## 2026-09-30 -- Milestone 3: the plate pair (0.4.0)

**Measured first** (`tests/push_reed.rs`, RK4): the push reed (cell
downstream) and the pull reed (cell upstream) have the same threshold to
the pascal for every cell -- 32 Pa by default; 36, 53, 119, 217, 358,
761 Pa with the cell's resonance at 1.3, 1.1, 1.0, 0.95, 0.9, 0.8 of the
reed. In one series loop the order does not matter.

**Built.** Two reed states behind one pallet; the signed bellows pressure
blows each from its own side; `bellows_direction` (index 23) and CC 80;
`reversal_time` (index 24, 100 ms, assumed); the state grows to 25 values
(212 bytes) and loads every older one.

**Tests** (`tests/milestone_3.rs`, through the engine at 48 kHz, 300 Pa):
the push reed's output is the pull reed's negated, sample for sample
(worst |pull + push| 0); the idle reed's state stays exactly default;
after a reversal the level returns to +0.05 dB. The gap: 130, 190, 250,
410 ms for turns of 20, 50, 100, 200 ms, none for 5 ms -- ignored as the
known slow-growth defect. Plugin: CC 80 at 64 and 63, and a wide value on
either side of half, set the parameter, which round-trips through the state.

**Package.** `rf-musette-lab package`: PLUGIN_PACKAGE_VALID 0.4.0,
parameters=25, PLUGIN_SMOKE_OK, state_bytes=212; wasm 63251 → 52180 bytes
optimised.

**Checks.** All workspace tests pass; fmt and clippy clean; the wasm
target builds.

**Render for listening.** `scores/bellows-reversal.score`: the F4 held at
~400 Pa while the bellows turns three times, then four notes on alternating
directions.

## 2026-09-30 -- Milestone 4: ranks, tremolo and registers (0.5.0)

**Built.** Key 65 has five ranks (L, M−, M, M+, H), each a plate of two
reeds with its own cell, behind the key's one pallet. M is the measured
F4. M± are the same tongue at the Borsini's beats (Hergert 2024, Fig. 6),
scaled to `tremolo` (index 25, 4.1 Hz at A4). L and H are the F4 scaled by
a bayan maker's slot ratios (RU2233009), assumed. `register` (index 26)
opens ranks as Roland's FR-3x draws its 14 registers; Clarinet (M) is the
default. The state grows to 27 values (228 bytes).

**Tests** (`tests/milestone_4.rs`, 300 Pa):
* sounding beats M+ +3.667 Hz and M− −3.146 Hz, against 3.666 and −3.143
  asked of the modes, and M−/M+ 6.813 Hz;
* levels over Clarinet: Celeste +3.01 dB, Musette +4.77 dB, Master
  +5.94 dB;
* L −9.6 and H +2.4 cents from M's octaves, L −16.4 and H −4.4 cents from
  their modes against M's −6.8 (the half not met);
* Clarinet's closed ranks stay exactly at rest.

**Package.** PLUGIN_PACKAGE_VALID 0.5.0, parameters=27, PLUGIN_SMOKE_OK,
state_bytes=228; wasm 69553 → 58735 bytes optimised.

**Cost, rough** (a whole 30 s lab render, process and file included):
458 ns per host sample with Clarinet and 840 with Master, native x86_64.

**Checks.** All workspace tests pass; fmt and clippy clean; the wasm
target builds.

**Render for listening.** `scores/registers.score`: the F4 through nine
registers, then one held note switched Clarinet → Musette → Master.

## 2026-09-30 -- Milestone 5: one bellows for every reed (0.6.0)

**Decided.** The intent is the arm's push (user, 2026-09-30: "lo que sea
más realista"). "Stiff" keeps it the pressure, for a digital accordion.

**Built.** `wind.rs`: C P′ = A v(P) − Σ Q_holes − Q_vent, with v(P) from
Hill's force-velocity law (k 0.25). The step is backward Euler on the
linearised law; its unit tests check the push with nothing drawn, Hill's
sag, an emptying vent, and finiteness at any step. Six parameters
(indices 27-32): response, area 600 cm², volume 12 L, arm speed 1 m/s,
leak 10 mm², air valve (400 mm² open); the state grows to 33 values (276
bytes). The pallet split among the open ranks was tried and withdrawn (it
silenced Master); each rank keeps its curtain.

**Tests** (`tests/milestone_5.rs`, the push asking 300 Pa):
* Clarinet holds 288.1 Pa (−4.0 %);
* Master holds 259.3 Pa (−13.6 %), +4.98 dB over Clarinet against +5.98
  stiff;
* 24 random bellows at full push stay finite.

Ignored:
* the shared-pallet bend (withdrawn);
* the air button: −29 % and −2.9 dB, short of the −50 % and −6 dB
  predicted (assumed constants).

Earlier engine tests run with "Stiff" and pass.

**Package.** PLUGIN_PACKAGE_VALID 0.6.0, parameters=33, PLUGIN_SMOKE_OK,
state_bytes=276; wasm 72796 → 61573 bytes optimised.

**Checks.** All workspace tests pass; fmt and clippy clean; wasm builds.

**Render for listening.** `scores/one-bellows.score`, rendered Stiff then
Arm and joined (×0.48): Clarinet → Master on a held note, then the air
button pressed and let go.

## 2026-10-01 -- Milestone 6: the cassotto (0.7.0)

**Built.** `cassotto.rs`: a Helmholtz resonator, H(s) = ω₀²/(s² + (ω₀/Q)s +
ω₀²), stepped as a topology-preserving state-variable filter at the
oversampled rate. Its unit test checks gain 1 at 50 Hz, Q at resonance and
~1/15 two octaves above. L and M radiate through it when Cassotto is on
(index 33, off by default); resonance 900 Hz (index 34, Richter's measured
800-1000 Hz); Q 2 (index 35, assumed). The state grows to 36 values.

**Tests** (`tests/milestone_6.rs`, M alone, the arm pushing for ~400 Pa):
* the centroid outside 1693 Hz, inside 959 Hz, a ratio of 0.57, against
  Llanos's E4 2013 → 1389 Hz (0.69);
* the finger attack 320 ms either way.

The fundamental is read from the spectrum's peak near the reed's mode:
zero crossings of the pulse-train sound read its third harmonic, 1060 Hz,
which a first version of these tests fell into.

**Package.** PLUGIN_PACKAGE_VALID 0.7.0, parameters=36, PLUGIN_SMOKE_OK.

**Checks.** All workspace tests pass; fmt and clippy clean; wasm builds.

**Render for listening.** `scores/cassotto.score`, off then on, joined and
scaled ×0.668: Clarinet, Bassoon, Bandoneon, Musette.

## 2026-10-01 -- Milestone 7: the whole treble (0.8.0)

**Built.**
* `compass.rs`: 41 keys F3-A6 × 5 ranks, every reed the F4 scaled by the
  bayan patent's slot ratios, with Q rising as (f/F4)^0.7.
* `tuning.rs`, generated by `rf-musette-lab tune` (25 s): each mode
  corrected to sound at A440 at 300 Pa, or at the lowest pressure that
  speaks.
* The engine: per-key pallets and models built on demand (271 KiB; a unit
  test bounds it).
* Parameters `pitch_a4` (36) and `q_slope` (37); the state is 38 values,
  316 bytes.
* Section tables cut to 256 entries over a length-scaled range: on F4,
  onset 32.3 Pa, swing 3.73 mm, bend −16.0 cents and the octaves and beats,
  all as with 1024.

**Found on the way.**
* With Q 250 everywhere, nothing from G6 up spoke at 300 Pa. Diagnosis:
  Q dominates the threshold (×4 → A6 M 378 → 154 Pa); the cell does not
  (kept at F4's proportion it rises slightly).
* Research: Q measured 83 at 137 Hz, ~233 at 598 Hz, 200-400 at
  236-743 Hz; none higher. A tuner's thresholds: 40-70 Pa low treble,
  100-250 Pa top piccolos.

**Tests** (`tests/milestone_7.rs`):
* all 205 reeds within ±2 cents where tuned (worst −0.01);
* M within ±15 cents at 100 and 600 Pa;
* the tremolo's lines at every fifth key within 5 %;
* the table matches a fresh tuning.

Ignored:
* H at keys 90-93 silent at 300 Pa (need 350-440);
* the swing's share of the length falls from 12.7 % (F3) to 4.4 % (A5)
  at 300 Pa, but is ~9 % at eight times each reed's threshold.

**Cost.** Four-note Master: 4.6 µs per 48 kHz sample, native (22 % of a
core).

**Package.** The first try trapped: a stack overflow (memory fault at
0xfffeacb0), the engine built by value. Linked with an 8 MiB stack,
RackForge's convention: PLUGIN_PACKAGE_VALID 0.8.0, parameters=38,
PLUGIN_SMOKE_OK peak 0.0058 (A4 sounds now), state_bytes=316. The
laboratory overflowed its 1 MiB main thread the same way and now works on
a 64 MiB thread.

**Render for listening.** `scores/frere-jacques.score`: the tune
(traditional), once in Clarinet, once in Musette.

## 2026-10-01 -- Milestone 7b: the finger attack (0.8.1)

**Measured first** (`tests/attack_diagnosis.rs`): F4 1165/396 ms at p/mf,
growing slower with pitch (A6 721 ms at mf). No assumed constant alone
reaches 50-140 ms; the best (upstream inertia ×4, a 5 ms pallet) bring mf
to ~200 ms and leave p above 600.

**Refuted** (`tests/two_modes.rs`, RK4):
* the second bending mode, which leaves the attack unchanged (747/296 ms,
  q₂ < 0.01 mm);
* the cell's pressure on the tongue's face, which makes it slower (mf
  296 → 455 ms at β = 1).

**Built.** The voiced start: when a key opens, each reed the bellows blows
gets a velocity ω·κ·set·P/(P + 20 Pa) into its frame (`attack_kick`,
index 38, κ 1). The state grows to 39 values.

**Tests** (`tests/milestone_7b.rs`, through the engine, stiff bellows):
* F4 85 ms at 100 Pa, 99 ms at 400 Pa;
* the true 8′ at 400 Pa: 106, 103, 99, 83, 73, 53 ms from F3 to A5;
* no sound with no air.

Ignored: D♯6 22 ms, A6 13 ms (too fast).

**Checks.** Every earlier test passes; fmt and clippy clean.

**Render for listening.** "Frère Jacques" with κ 0 then κ 1 (RMS 0.139 →
0.206): the eighth notes reach their tone.

## 2026-10-01 -- The start at the air's arrival (0.8.2)

**Measured first** (`tests/musette_diagnosis.rs`, and the render note by
note above 600 Hz):
* In Clarinet, every note but the first reaches −5 dB of its level in
  100-150 ms.
* The first note, with the arm and the key moving together, got no start.
  At that block's start the pressure was still 0 Pa. It was silent until
  ~250 ms.
* In Musette each reed attacks as in Clarinet and the bellows holds
  (351 ± 15 Pa). The three reeds, started together by one pallet, dip the
  sum's fundamental 35 dB at 120 ms (C4, beats of 2.4 and 3.1 Hz). Left
  as the physics.

**Built.** Each reed is owed its start while its key is down, its register
open and its side's pressure above 20 Pa. It is given the rise of
P/(P + 20 Pa) as the pressure comes, checked at every step, and owed again
once that air has gone.

**Tests** (`tests/milestone_7c.rs`, the arm, C4 Clarinet):
* the arm and the key together: 112 ms (was 303);
* key held, then the air: 92 ms (was 212).

Through a reversal (diagnosis): ~240 ms after the turn begins (was ~410).

**Checks.**
* 7b is unchanged.
* Milestones 4 and 5 now read their levels over 20 s. Over 1.5-2.5 s
  Master's level hung on the beats' phase.
  * Stiff Master: +6.32 dB (was recorded +5.94).
  * The arm's Master: +5.50 dB, the arm costing 0.82 dB.
  * Milestone 5's test asserts that cost (0.44-1.44 dB, the band it
    predicted).
* fmt and clippy clean.

**Render for listening.** `renders/frere-jacques-k2.wav`: the first note
speaks like the others (−26 dB above 600 Hz at 100 ms; was silent).

## 2026-10-01 -- Milestone 8: the Stradella bass (0.9.0)

**Built.**
* The bass side: twelve pitch classes, five ranks, 16′, 8′, 8-4′, 4′, 2′,
  each one octave wide, a pitch class's reed shared by its bass button and
  every chord that holds it.
* A bass pallet opens every rank; a chord pallet opens the 8-4′, 4′ and 2′.
* Roland's seven bass registers (`bass_register`, index 39; the state
  grows to 40 values).
* MIDI as a V-Accordion sends it: channel 2 the bass buttons, 3 the chords,
  every other the treble.
* Every reed carries the least tip load that lets it speak from 50 Pa to
  1 kPa (`tip_load`, found by `rf-musette-lab tune` and written beside the
  cents). The treble's 16′ of keys F3-B3 is loaded too: unloaded it choked
  above 400-800 Pa, unseen since milestone 4.

**Measured first** (`tests/bass_diagnosis.rs`):
* the slots carried down, unloaded, make a C2 that is silent at 300 Pa,
  its tongue held 2.2 mm into the slot;
* yielding no more than B2 is not enough to speak like B2.

**Tests** (`tests/milestone_8.rs`, the plugin's contracts):
* the 60 reeds in tune at 300 Pa;
* a bass button on every open rank, a chord on the chord ranks only;
* a shared reed sounds once, sample for sample;
* the channels, in MIDI 1.0 and 2.0;
* no reed chokes at 1 kPa;
* the load and tuning tables current.

Not met, kept as ignored tests:
* the 16′ C2 is 64.6 mm long (Llanos: 52 mm);
* the loaded 16′ start at up to 54 Pa;
* the 16′ C2 attacks in 467 ms;
* the high reeds do not start at 50 Pa, which they should not.

**Checks.** Every earlier test passes; the treble retuned within ±2 cents.
Native tests run on 8 MiB threads; fmt and clippy clean.

**Cost.** Both hands, 7.5 µs per sample natively; 357 KiB.

**Render for listening.** `scores/frere-jacques-bass.score`: the tune in
Clarinet over C bass, C major, G counter-bass, C major in every bar. The
16′ stays 30 dB under its level through each 440 ms bass note.

## 2026-10-01 -- Milestone 8c: the low reeds' attack (0.9.1)

**Measured first** (`tests/bass_diagnosis.rs`, and a literature search):
* The model's growth is not too slow: the direct measurement of a C3
  reed's growth (Cottingham, Reed & Busha 1999) is no faster than the
  model's. 7b's premise is withdrawn.
* Against Llanos 2014's Table I note by note:
  * A3-B4 agree;
  * A2-B2 are 2.5 times too slow;
  * the top at p is too fast.
* In the engine, A2's start fired with its pallet still shut; the air
  arriving afterwards partly cancelled it.

**Built.** The start waits for the air in the reed's own cell, then comes
whole.

**Tests** (`tests/milestone_8c.rs`): A3-B4 82-102 ms; C2 faster. Ignored, not
met: A2-B2 151-179 ms (90-112 through a 5 ms pallet).

**Checks.** 7b's F4 79/98 ms, 7c's 106/90 ms; every earlier test passes.

**Also tried and reverted (8d):**
* the bellows' moving mass;
* tone holes and loads found on a simulation fed by the bellows' air.

The 16′ C2-D♯2 do not speak fed by the bellows' air, massive or not. The
cause is open (docs/ROADMAP.md, 8d). milestone 8's test of "speaks" proved
too weak: it passed a slowly dying reed.

## 2026-10-01 -- Milestone 8e: the bass's inlet ducts (0.9.2)

**Measured first** (`tests/bass_diagnosis.rs`, on a reed fed by the
bellows' air, `simulate_fed`):
* The lowest 16′ do not speak with the bellows' air behind their holes.
* A longer inlet duct makes them speak: ×10 for C2, ×6 for C♯2-D♯2.
* A higher set, a thicker plate or a smaller hole alone do not reach.

The diagnosis tool's first form sagged the mean pressure under any draw;
it was corrected and checked on A2, F3 and F4 before being trusted.

**Built.**
* The bellows is its air at audio frequencies: the arm delivers the mean
  draw and holds Hill's mean pressure, over 10 ms each.
* The honest test of speaking, for the loads and the ducts.
* An inlet duct for each reed under 300 Hz, found by
  `rf-musette-lab tune` on the engine's bellows (`DUCTS`, `BASS_DUCTS`).

**Tests** (`tests/milestone_8e.rs`):
* the 16′ C2 under the arm reaches 4.9 mm;
* every reed under 300 Hz speaks at 300 Pa and 1 kPa on the bellows.

Now met, earlier marked not met:
* the 16′ thresholds (16-8 Pa);
* the 16′ C2 attack (130 ms);
* A2-B2 (129-139 ms).

**Checks.**
* Every earlier test passes: milestone 5's droops, 7b's F4 79/98 ms,
  7c's 111/109 ms.
* Milestone 8's range test reads speaking honestly; the top-4′ defect
  shows from key 87.
* Both hands 7.7 µs per sample; 357 KiB.
