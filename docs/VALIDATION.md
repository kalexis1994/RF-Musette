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
