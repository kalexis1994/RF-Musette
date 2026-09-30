# RF-Musette working conventions

- Keep project code, documentation and CLI messages in English. Speak to the user in their preferred language.
- The project is GPL-3.0-only. No sample library, recording or third-party synthesizer source code enters this repository; the instrument is computed, and the notices say so.
- There is no reference recording: the model is judged by its physics and by the player's ear (decided 2026-09-30, `docs/RESEARCH.md`). Every constant in `docs/MODEL.md` is marked *measured* (with its source), *derived* (by stated physics from measured ones) or *voiced by ear* (with the date and what was heard). Never cite a voiced value as measured.
- Published numbers are tests on emergent behaviour, not targets to fit. When a voicing by ear pushes the model outside what real reeds do, say so and let the player decide; sound changes are always the player's call.
- Before a DSP change, measure which stage causes the symptom and state the prediction the change must meet. A metric improving is not enough on its own.
- `docs/MODEL.md` is the ledger; keep it truthful. What was tried and reverted is recorded there with its measurements.
- The engine is `no_std` and the audio path does not allocate, log, access files, lock or perform unbounded work.
- Renders meant for listening start with 1–2 s of silence (the player's wireless headset swallows the first attack after silence), and both sides of an A/B go in one file.
- Keep local build growth bounded: use `CARGO_INCREMENTAL=0`, and clean regenerable build artifacts when needed. The system disk is nearly full.
- After producing a version for the user to test, run `cargo run --locked --release -p rf-musette-lab -- audition` from this workspace. This is the standard build, package, install and Desktop launch workflow.
- The audition command owns only `dist/audition/RackForgeData`; keep the user's regular RackForge library separate. Preserve test-library audio/MIDI settings between runs.
- If RackForge is already open, report that its window must be closed before rerunning. Do not force-terminate it or replace a package behind a running host.
- Use `audition --prepare-only` for noninteractive preparation. Never launch Desktop in CI. Do not label a prepare-only run as a successful GUI/audio test.
