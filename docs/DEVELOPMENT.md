# Development

## Toolchain

Rust 1.98.0 is pinned, with the `wasm32-unknown-unknown` target. The engine
has no dependencies at all; the laboratory uses `serde_json` for its reports,
and the plugin uses the public RackForge SDK from a sibling `rackforge`
checkout through an explicit Cargo path.

`.cargo/config.toml` is tracked, not ignored: it builds the component with
`+simd128`, which the RackForge SDK requires of every plugin.

```text
set CARGO_INCREMENTAL=0
cargo test --locked --workspace
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
cargo build --locked --release --workspace
cargo build --locked --release --target wasm32-unknown-unknown -p rf-musette-plugin
```

## The SDK pin, and the lock-file trap

`Cargo.lock` records the version of the sibling SDK it was resolved against:
RackForge **0.1.24**. CI checks out `kalexis1994/rackforge` at
`988e4b70b0bb49eb8f99438dc6f332394b073c50`, the 0.1.24 release on `main`, so
the two agree. That commit's SDK, plugin API, core and store crates are
byte-identical to the local checkout this skeleton was built against.

When the sibling checkout moves to another RackForge version, a `--locked`
build fails here because the lock names the old one. Do not drop `--locked`
to get past it: that rewrites the lock silently, and CI breaks. Update both
together — regenerate the lock (`cargo update -w --offline`) against the new
host and move the CI `ref` to the commit of that release — in one commit.

## Render and inspect

```text
cargo run --release -p rf-musette-lab -- render --output renders/a4.wav
cargo run --release -p rf-musette-lab -- render --output renders/phrase.wav --score phrase.txt
cargo run --release -p rf-musette-lab -- inspect renders/a4.wav
```

A score has one event per line: `onset_ms duration_ms note velocity`, or
`onset_ms bellows 0..127` for Expression. `--help` lists every option. Every
render starts with 1.5 s of silence by default, for listening; outputs use
create-new semantics and nothing is overwritten. Each render writes a JSON
report beside the WAV with the peak, the RMS and the score it played.

## Package

For the whole build, install and launch cycle use
`cargo run --locked --release -p rf-musette-lab -- audition`. See
[Desktop audition](AUDITION.md).

The standalone `package` command produces a versioned archive without launching
a host. It needs RackForge's own tools built first, in the sibling checkout:

```text
cargo build --locked --release -p rackforge-store -p rackforge-core
```

Then, from here:

```text
cargo run --release -p rf-musette-lab -- package
```

The laboratory copies the current WASM into the ignored `package/component.wasm`,
validates the metadata and smoke-tests it through the host, and creates the
archive only after both succeed. It never overwrites an existing archive.

## Layout

```text
crates/rf-musette-dsp/     the engine: no_std, no allocation, no dependencies
crates/rf-musette-plugin/  SDK adapter, MIDI validation, bellows mapping, state
tools/rf-musette-lab/      rendering, scores, WAV, reports, packaging, audition
package/                   RackForge manifest and metadata
docs/                      research, the model ledger, roadmap, receipts
renders/                   ignored generated WAV and JSON
dist/                      ignored distributable and validation output
```

Unsafe Rust is forbidden across the workspace. The plugin's export macro
contains the SDK's own raw ABI implementation; every handwritten line here is
safe Rust. Event lists are validated before anything is mutated, and an invalid
block is silenced without a partial edit.
