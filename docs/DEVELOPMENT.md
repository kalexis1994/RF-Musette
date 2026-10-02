# Development

## Toolchain

Rust 1.98.0 is pinned, with the `wasm32-unknown-unknown` target. The engine
has no dependencies at all; the laboratory uses `serde_json` for its reports,
and the plugin uses the public RackForge SDK from a sibling `rackforge`
checkout through an explicit Cargo path.

The PLAY surface (`crates/rf-musette-ui`) is Rust built for the browser with
`wasm-bindgen`, pinned at 0.2.127; building it needs the matching CLI:

```text
cargo install --locked wasm-bindgen-cli --version 0.2.127
```

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
cargo run --release -p rf-musette-lab -- render --output renders/pair.wav --score scores/frere-jacques-bass.score --stereo --set mic_layout=3
```

A render is the instrument alone at 1 m, mono, as every measurement takes
it; `--stereo` renders it through the microphones and the room instead.

A score has one event per line: `onset_ms duration_ms note velocity`,
`onset_ms bellows 0..127` for Expression, `onset_ms wheel 0..127` for the
modulation wheel (the bellows' position with `--set mod_wheel=1`), `onset_ms direction pull|push`
for the bellows' direction, `onset_ms register NAME` for a register
(Clarinet, Musette, Master... as the parameter names them), or
`onset_ms air 0..1` for the air button. `--help` lists every option. Every
render starts with 1.5 s of silence by default, for listening; outputs use
create-new semantics and nothing is overwritten. Each render writes a JSON
report beside the WAV with the peak, the RMS and the score it played.

## Tune

Every reed is tuned as a tuner tunes: its mode set so it sounds on its pitch
at 300 Pa. The corrections live in the generated
`crates/rf-musette-dsp/src/tuning.rs`. After changing anything a reed is
made of, regenerate it (about 25 s), then format:

```text
cargo run --release -p rf-musette-lab -- tune
cargo fmt --all
```

`tests/milestone_7.rs` fails if the table no longer matches a fresh tuning.

## The PLAY and CONFIG surfaces

```text
cargo run --release -p rf-musette-lab -- web-ui
```

builds `crates/rf-musette-ui` for wasm32 and writes `package/web/app.js` and
`app_bg.wasm` with the wasm-bindgen CLI. Both are committed, so a checkout
shows the pages without the tools; `package` and `audition` rebuild them
first. `play.html`, `config.html` and `styles.css` are written by hand; the
one app draws CONFIG where the page's root says `data-surface="config"`.
The panel map is
`crates/rf-musette-ui/src/panel.rs`: it places every parameter, a test holds
it to the parameter table, and `rf-musette-lab schema` takes its pages and
order. What the pages can be tested on off the browser -- the map, the knobs'
taper, the register symbols, the `.rfmusette` files (`archive.rs`) -- is
plain Rust under `cargo test`.

The player's own programs are the plugin's (`crates/rf-musette-plugin/src/program.rs`):
RackForge stores the documents and hands each back when an instance starts,
and an instance whose saved program is refused does not start, so the payload
-- every parameter by its id -- is read leniently and forever. A parameter
added later needs nothing; one removed or renamed must still be read from
the old id.

To see the page without RackForge, copy the host's plugin kit (the program
selector and save dialog) next to the preview and serve the repository:

```text
xcopy /e /i ..\rackforge\web\dist\rackforge-plugin-kit tools\rackforge-plugin-kit
python -m http.server 8141
```

then open `http://localhost:8141/tools/ui-preview.html` (`?lighting=stage`
for the stage light). It plays RackForge's side of the bridge: the context,
the parameters from the package's schema, each set echoed back. In the
browser console, `__requests` lists what the page asked, `__consoleErrors`
what failed, and `__hostSet(index, value)` changes a value from outside, as a
MIDI link would. `?surface=config` opens CONFIG, with a stand-in for
RackForge's program drafts and three saved programs (`&programs=0` for none):
`__programs` holds what is saved, and `__fail = "plugin.save_program"` (or
any method) has the next such request refused, to see a job fail midway.

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
crates/rf-musette-ui/      the PLAY surface: bridge client, panel map, knobs, symbols
tools/rf-musette-lab/      rendering, scores, WAV, reports, packaging, audition
package/                   RackForge manifest, metadata and the built web/ surface
docs/                      research, the model ledger, roadmap, receipts
renders/                   ignored generated WAV and JSON
dist/                      ignored distributable and validation output
```

Unsafe Rust is forbidden across the workspace, the PLAY surface included:
its `wasm-bindgen` entry point compiles under the same rule. The plugin's export macro
contains the SDK's own raw ABI implementation; every handwritten line here is
safe Rust. Event lists are validated before anything is mutated, and an invalid
block is silenced without a partial edit.
