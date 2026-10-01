//! Writes `package/metadata/parameters.json` from the engine's own table,
//! so the manifest can never describe a parameter the code does not have.
//! The plugin's contract test reads the file back against the table. Each
//! page lists its parameters in the PLAY surface's order, so RackForge's
//! own screens read the panel as the page does; the controller roles come
//! from the engine's [`SEMANTIC_CONTROLS`].

use rf_musette_dsp::parameters::{PAGES, SEMANTIC_CONTROLS, SPECS, Taper};
use serde_json::{Value, json};
use std::{error::Error, fs};

pub fn document() -> Value {
    let pages: Vec<Value> = PAGES
        .iter()
        .enumerate()
        .map(|(order, (id, name))| json!({ "id": id, "name": name, "order": order }))
        .collect();
    let parameters: Vec<Value> = SPECS
        .iter()
        .enumerate()
        .map(|(index, spec)| {
            let kind = if spec.choices.is_empty() {
                let mut kind = json!({
                    "type": "float",
                    "minimum": spec.minimum,
                    "maximum": spec.maximum,
                    "default": spec.default,
                    "step": spec.step,
                    "taper": match spec.taper {
                        Taper::Linear => "linear",
                        Taper::Logarithmic => "logarithmic",
                    },
                });
                if !spec.unit.is_empty() {
                    kind["unit"] = json!(spec.unit);
                }
                kind
            } else {
                json!({
                    "type": "enum",
                    "default": spec.default as u32,
                    "choices": spec
                        .choices
                        .iter()
                        .map(|(value, name)| json!({ "value": value, "name": name }))
                        .collect::<Vec<_>>(),
                })
            };
            json!({
                "index": index,
                "id": spec.id,
                "name": spec.name,
                "page": spec.page,
                "order": panel_order(spec.id),
                "kind": kind,
                "flags": {
                    "automatable": true,
                    "modulatable": false,
                    "read_only": false,
                    "advanced": spec.page != PAGES[0].0,
                },
                "suggested_control": if spec.choices.is_empty() { "knob" } else { "list" },
            })
        })
        .collect();
    let semantic_controls: Vec<Value> = SEMANTIC_CONTROLS
        .iter()
        .map(|(role, index)| json!({ "role": role, "parameter_index": index }))
        .collect();
    // Schema 2: the first that carries controller roles.
    json!({
        "schema_version": 2,
        "pages": pages,
        "parameters": parameters,
        "semantic_controls": semantic_controls,
    })
}

/// Where the parameter sits on its panel page, counting through its groups.
fn panel_order(id: &str) -> usize {
    rf_musette_ui::panel::PAGES
        .iter()
        .find_map(|page| {
            page.groups
                .iter()
                .flat_map(|group| group.parameters.iter())
                .position(|placed| *placed == id)
        })
        .unwrap_or_else(|| panic!("{id} is not on the panel"))
}

/// The program catalog, from the engine's own table of programs.
pub fn programs() -> Value {
    use rf_musette_dsp::programs::{BANKS, PROGRAMS};
    let banks: Vec<Value> = BANKS
        .iter()
        .enumerate()
        .map(|(order, (id, name))| json!({ "id": id, "name": name, "order": order }))
        .collect();
    let presets: Vec<Value> = PROGRAMS
        .iter()
        .map(|program| {
            let order = PROGRAMS
                .iter()
                .filter(|other| other.bank == program.bank)
                .position(|other| other.id == program.id)
                .unwrap_or(0);
            let tags = if program.id == "research" {
                vec!["default".to_owned()]
            } else {
                vec![program.category.to_lowercase()]
            };
            json!({
                "id": program.id,
                "name": program.name,
                "bank": program.bank,
                "category": program.category,
                "order": order,
                "tags": tags,
                "description": program.description,
            })
        })
        .collect();
    json!({ "schema_version": 1, "banks": banks, "presets": presets })
}

/// Writes the parameter schema and the program catalog.
pub fn write() -> Result<(), Box<dyn Error>> {
    let root = super::package::workspace_root()?;
    for (name, document) in [
        ("parameters.json", document()),
        ("presets.json", programs()),
    ] {
        let path = root.join("package/metadata").join(name);
        let mut text = serde_json::to_string_pretty(&document)?;
        text.push('\n');
        fs::write(&path, text)?;
        println!("Wrote {}", path.display());
    }
    Ok(())
}
