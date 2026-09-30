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
