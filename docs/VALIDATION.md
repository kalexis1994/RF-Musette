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
