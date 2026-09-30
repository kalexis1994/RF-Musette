//! Writes `package/metadata/parameters.json` from the engine's own table,
//! so the manifest can never describe a parameter the code does not have.
//! The plugin's contract test reads the file back against the table.

use rf_musette_dsp::parameters::{PAGES, SPECS, Taper};
use serde_json::{Value, json};
use std::{error::Error, fs};

pub fn document() -> Value {
    let pages: Vec<Value> = PAGES
        .iter()
        .enumerate()
        .map(|(order, (id, name))| json!({ "id": id, "name": name, "order": order }))
        .collect();
    let mut orders = std::collections::HashMap::new();
    let parameters: Vec<Value> = SPECS
        .iter()
        .enumerate()
        .map(|(index, spec)| {
            let order = orders.entry(spec.page).or_insert(0);
            let this = *order;
            *order += 1;
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
                "order": this,
                "kind": kind,
                "flags": {
                    "automatable": true,
                    "modulatable": false,
                    "read_only": false,
                    "advanced": spec.page != "output",
                },
                "suggested_control": if spec.choices.is_empty() { "knob" } else { "list" },
            })
        })
        .collect();
    json!({ "schema_version": 1, "pages": pages, "parameters": parameters })
}

pub fn write() -> Result<(), Box<dyn Error>> {
    let path = super::package::workspace_root()?.join("package/metadata/parameters.json");
    let mut text = serde_json::to_string_pretty(&document())?;
    text.push('\n');
    fs::write(&path, text)?;
    println!("Wrote {}", path.display());
    Ok(())
}
