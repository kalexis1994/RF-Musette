//! Builds the PLAY surface (`crates/rf-musette-ui`) into `package/web/`:
//! the crate for wasm32, then the wasm-bindgen CLI for its JavaScript glue,
//! as RF-5's `tools/build-web-ui` does. `app.js` and `app_bg.wasm` are
//! committed, so a checkout runs without the tools.

use super::package::{run, workspace_root};
use std::{error::Error, fs, process::Command};

/// The wasm-bindgen the crate pins; the CLI must be the same version.
const WASM_BINDGEN: &str = "0.2.127";

pub fn build() -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let version = Command::new("wasm-bindgen")
        .arg("--version")
        .output()
        .map_err(|error| {
            format!(
                "the web UI needs the wasm-bindgen CLI {WASM_BINDGEN} \
                 (cargo install wasm-bindgen-cli --version {WASM_BINDGEN}): {error}"
            )
        })?;
    let version = String::from_utf8_lossy(&version.stdout);
    if version.split_whitespace().nth(1) != Some(WASM_BINDGEN) {
        return Err(format!(
            "the web UI needs wasm-bindgen {WASM_BINDGEN}, found {}",
            version.trim()
        )
        .into());
    }
    run(Command::new("cargo").current_dir(&root).args([
        "build",
        "--locked",
        "--release",
        "--target",
        "wasm32-unknown-unknown",
        "-p",
        "rf-musette-ui",
    ]))?;
    let web = root.join("package/web");
    run(Command::new("wasm-bindgen")
        .arg(root.join("target/wasm32-unknown-unknown/release/rf_musette_ui.wasm"))
        .arg("--out-dir")
        .arg(&web)
        .args(["--out-name", "app", "--target", "web", "--no-typescript"]))?;
    let app = web.join("app.js");
    let mut glue = fs::read_to_string(&app)?;
    glue.push_str(
        "\n// Generated bootstrap: all UI behaviour lives in Rust WebAssembly.\n__wbg_init();\n",
    );
    fs::write(&app, glue)?;
    println!("Built the web UI: {}", web.join("app_bg.wasm").display());
    Ok(())
}
