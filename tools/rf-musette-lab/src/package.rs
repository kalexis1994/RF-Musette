//! Package and smoke-test through RackForge's own Rust tools.

use std::{error::Error, fs, path::Path, process::Command};

pub fn build() -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    build_to(
        &root
            .join("dist")
            .join(format!("RF-Musette-{}.rfplugin", env!("CARGO_PKG_VERSION"))),
    )
}

pub(crate) fn workspace_root() -> Result<std::path::PathBuf, Box<dyn Error>> {
    Ok(Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("cannot resolve the workspace root")?
        .to_path_buf())
}

pub(crate) fn host_root() -> Result<std::path::PathBuf, Box<dyn Error>> {
    Ok(workspace_root()?
        .parent()
        .ok_or("cannot resolve the sibling RackForge checkout")?
        .join("rackforge"))
}

/// The RackForge revision `Cargo.lock` pins the SDK to, if it pins exactly
/// one: the same value `tools/pins.sh rackforge` prints, read without a shell.
pub(crate) fn pinned_host(lock: &str) -> Option<String> {
    const PREFIX: &str = "source = \"git+https://github.com/kalexis1994/rackforge?rev=";
    let mut revisions = lock
        .lines()
        .filter_map(|line| line.strip_prefix(PREFIX))
        .filter_map(|rest| rest.split_once('#'))
        .map(|(_, commit)| commit.trim_end_matches('"').to_owned())
        .collect::<Vec<_>>();
    revisions.sort();
    revisions.dedup();
    match revisions.as_slice() {
        [one] if one.len() == 40 => Some(one.clone()),
        _ => None,
    }
}

/// Says so when the sibling checkout whose tools check and pack the package
/// is not at the revision the SDK is pinned to. It does not refuse: building
/// against newer host work is sometimes the point. It only stops a package
/// checked by other tools than CI's from passing for the one CI checks.
fn warn_if_host_skewed(root: &Path, host: &Path) {
    let Some(pinned) = fs::read_to_string(root.join("Cargo.lock"))
        .ok()
        .as_deref()
        .and_then(pinned_host)
    else {
        return;
    };
    let Ok(output) = Command::new("git")
        .arg("-C")
        .arg(host)
        .args(["rev-parse", "HEAD"])
        .output()
    else {
        return;
    };
    let checked_out = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if output.status.success() && checked_out != pinned {
        eprintln!(
            "warning: {} is at {}, but the SDK is pinned to {}; this package is \
             checked and packed by other RackForge tools than CI's. To match: \
             git -C {} checkout {}",
            host.display(),
            &checked_out[..checked_out.len().min(12)],
            &pinned[..12],
            host.display(),
            pinned
        );
    }
}

pub(crate) fn build_to(output: &Path) -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    // The surfaces first, so the package never carries the pages of an
    // earlier edit either.
    super::web::build()?;
    // The component is built here rather than taken from wherever the target
    // directory happens to be, so a package can never be validated against
    // the WASM of an earlier edit.
    run(Command::new("cargo").current_dir(&root).args([
        "build",
        "--locked",
        "--release",
        "--target",
        "wasm32-unknown-unknown",
        "-p",
        "rf-musette-plugin",
    ]))?;
    let host = host_root()?;
    warn_if_host_skewed(&root, &host);
    let tools = host.join("target/release");
    let store = tools.join(format!("rackforge-store{}", std::env::consts::EXE_SUFFIX));
    let core = tools.join(format!("rackforge-core{}", std::env::consts::EXE_SUFFIX));
    let component = root.join("target/wasm32-unknown-unknown/release/rf_musette_plugin.wasm");
    let package = root.join("package");
    let dist = root.join("dist");
    if output.exists() {
        return Err(format!("refusing to overwrite {}", output.display()).into());
    }
    for file in [&store, &core, &component] {
        if !file.is_file() {
            return Err(format!("build the required artifact first: {}", file.display()).into());
        }
    }
    fs::create_dir_all(&dist)?;
    fs::copy(&component, package.join("component.wasm"))?;
    // The licence and the notices travel in the package, as they do in the
    // other official instruments; the repository's own copies are the source.
    for file in ["LICENSE", "NOTICE.md"] {
        let source = root.join(file);
        if !source.is_file() {
            return Err(format!("the package needs {}", source.display()).into());
        }
        fs::copy(&source, package.join(file))?;
    }
    run(Command::new(&core).arg("inspect").arg(&package))?;
    run(Command::new(&core)
        .arg("smoke")
        .arg(&package)
        .arg("--preset")
        .arg("research")
        .arg("--data-root")
        .arg(dist.join("smoke-data")))?;
    run(Command::new(&store)
        .arg("pack-wasm")
        .arg(&package)
        .arg(&component)
        .arg(output))?;
    println!("Validated package: {}", output.display());
    Ok(())
}

pub(crate) fn run(command: &mut Command) -> Result<(), Box<dyn Error>> {
    hide_console(command);
    let status = command.status()?;
    if !status.success() {
        return Err(format!("RackForge validation failed: {status}").into());
    }
    Ok(())
}

pub(crate) fn hide_console(command: &mut Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW for CLI helpers.
    }
    #[cfg(not(windows))]
    let _ = command;
}

#[cfg(test)]
mod pin_tests {
    use super::pinned_host;

    const COMMIT: &str = "6fdbe643fe470ce62cb17c7ea982d5d8881b3854";

    fn entry(commit: &str) -> String {
        format!(
            "[[package]]\nname = \"rackforge-plugin-sdk\"\nversion = \"0.1.29\"\n\
             source = \"git+https://github.com/kalexis1994/rackforge?rev={commit}#{commit}\"\n"
        )
    }

    #[test]
    fn reads_the_one_revision_the_lock_pins() {
        let lock = entry(COMMIT) + &entry(COMMIT);
        assert_eq!(pinned_host(&lock).as_deref(), Some(COMMIT));
    }

    #[test]
    fn refuses_a_lock_that_pins_two_revisions_or_none() {
        let other = "0000000000000000000000000000000000000000";
        assert_eq!(pinned_host(&(entry(COMMIT) + &entry(other))), None);
        assert_eq!(pinned_host("[[package]]\nname = \"serde\"\n"), None);
    }

    #[test]
    fn the_repository_lock_pins_exactly_one_revision() {
        let lock = include_str!("../../../Cargo.lock");
        assert!(
            pinned_host(lock).is_some(),
            "Cargo.lock pins no single RackForge revision"
        );
    }
}
