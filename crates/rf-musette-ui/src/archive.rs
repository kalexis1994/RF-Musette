//! `.rfmusette` files: the player's own programs, carried off the machine
//! and brought back (CONFIG).
//!
//! A file holds RackForge program documents as RF-Musette's plugin prepares
//! them (`crates/rf-musette-plugin/src/program.rs`), in an envelope that
//! names the format and seals the programs with their SHA-256:
//!
//! ```json
//! { "format": "org.rackforge.musette.program", "schema_version": 1,
//!   "plugin_id": "org.rackforge.musette", "exported_by": "RF-Musette 0.13.8",
//!   "programs": [ { ...ProgramDocument... } ],
//!   "integrity": { "algorithm": "SHA-256", "digest": "<64 hex digits>" } }
//! ```
//!
//! The digest is over the programs written canonically -- keys sorted at
//! every depth, no spaces -- so it does not hang on how a JSON library
//! orders keys. A bare RackForge program document (`.rackforge-program.json`)
//! is read too, as one program with nothing to check it against.
//!
//! Reading checks everything a program can be refused for before anything
//! is imported, so a file goes in whole or not at all. Each program then
//! comes in as a new one: what is kept of it is its name, category and
//! payload; its identity is the new draft's (see [`adopt`]). The plugin
//! reads the payload itself, leniently, as it reads every saved program.

use serde::Serialize;
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::fmt::{self, Write};

pub const FORMAT: &str = "org.rackforge.musette.program";
pub const SCHEMA_VERSION: u64 = 1;
pub const EXTENSION: &str = "rfmusette";
/// The plugin, as its manifest names it.
pub const PLUGIN_ID: &str = "org.rackforge.musette";
/// What the plugin's payload is written as (`program.rs`).
pub const PAYLOAD_VERSION: u64 = 1;
/// As many programs as the plugin keeps.
pub const MAX_PROGRAMS: usize = 64;
/// The largest file read: 64 programs are about 200 KB.
pub const MAX_FILE_BYTES: usize = 1024 * 1024;
/// The largest document RackForge lets CONFIG put in a draft.
pub const MAX_DOCUMENT_BYTES: usize = 16 * 1024;
/// The longest name RackForge keeps.
pub const MAX_NAME_CHARS: usize = 64;
const DIGEST: &str = "SHA-256";

/// Why a file was not imported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportError {
    TooLarge,
    NotJson,
    NotAProgramFile,
    NewerFormat(u64),
    NoPrograms,
    TooManyPrograms(usize),
    Damaged,
    OtherPlugin(String),
    /// The program at this place (from 1) is malformed.
    Malformed(usize),
    /// The program at this place was written by a newer RF-Musette.
    NewerProgram(usize),
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLarge => write!(f, "The file is larger than a program file can be (1 MB)."),
            Self::NotJson => write!(f, "The file is not a program file: it is not JSON."),
            Self::NotAProgramFile => write!(f, "The file is not an RF-Musette program file."),
            Self::NewerFormat(version) => write!(
                f,
                "The file was written by a newer RF-Musette (format {version}). Update RF-Musette to import it."
            ),
            Self::NoPrograms => write!(f, "The file holds no programs."),
            Self::TooManyPrograms(count) => write!(
                f,
                "The file holds {count} programs; RF-Musette keeps at most {MAX_PROGRAMS}."
            ),
            Self::Damaged => write!(
                f,
                "The file was changed or damaged after it was exported: its programs do not match its seal."
            ),
            Self::OtherPlugin(plugin) => write!(
                f,
                "The file holds a program for another plugin ({plugin}), not RF-Musette."
            ),
            Self::Malformed(place) => write!(f, "Program {place} in the file is malformed."),
            Self::NewerProgram(place) => write!(
                f,
                "Program {place} was written by a newer RF-Musette. Update RF-Musette to import it."
            ),
        }
    }
}

/// A program read from a file: what of it is imported.
#[derive(Debug, Clone, PartialEq)]
pub struct Imported {
    pub name: String,
    pub category: Option<String>,
    pub payload: Value,
}

/// JSON written canonically: object keys sorted at every depth, no spaces.
pub fn canonical(value: &Value) -> String {
    let mut out = String::new();
    write_canonical(value, &mut out);
    out
}

fn write_canonical(value: &Value, out: &mut String) {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            out.push('{');
            for (place, key) in keys.into_iter().enumerate() {
                if place > 0 {
                    out.push(',');
                }
                out.push_str(&Value::String(key.clone()).to_string());
                out.push(':');
                write_canonical(&map[key], out);
            }
            out.push('}');
        }
        Value::Array(items) => {
            out.push('[');
            for (place, item) in items.iter().enumerate() {
                if place > 0 {
                    out.push(',');
                }
                write_canonical(item, out);
            }
            out.push(']');
        }
        scalar => out.push_str(&scalar.to_string()),
    }
}

/// The programs' seal: SHA-256 of their canonical JSON, in hex.
pub fn digest(programs: &[Value]) -> String {
    let hash = Sha256::digest(canonical(&Value::Array(programs.to_vec())).as_bytes());
    hash.iter()
        .fold(String::with_capacity(64), |mut hex, byte| {
            let _ = write!(hex, "{byte:02x}");
            hex
        })
}

/// A file of these program documents, as the plugin prepared them.
/// `exported_by` names what wrote it, for the player reading the file.
pub fn write(programs: &[Value], exported_by: &str) -> String {
    // A struct, so the file reads in this order: what it is, then what it
    // holds, then its seal.
    #[derive(Serialize)]
    struct File<'a> {
        format: &'static str,
        schema_version: u64,
        plugin_id: &'static str,
        exported_by: &'a str,
        programs: &'a [Value],
        integrity: Value,
    }
    let file = File {
        format: FORMAT,
        schema_version: SCHEMA_VERSION,
        plugin_id: PLUGIN_ID,
        exported_by,
        programs,
        integrity: json!({ "algorithm": DIGEST, "digest": digest(programs) }),
    };
    let mut text = serde_json::to_string_pretty(&file).unwrap_or_default();
    text.push('\n');
    text
}

/// The programs a file holds, every one checked, or why none is imported.
pub fn read(bytes: &[u8]) -> Result<Vec<Imported>, ImportError> {
    if bytes.len() > MAX_FILE_BYTES {
        return Err(ImportError::TooLarge);
    }
    // A byte-order mark, as some editors write one.
    let bytes = bytes.strip_prefix(b"\xef\xbb\xbf").unwrap_or(bytes);
    let file: Value = serde_json::from_slice(bytes).map_err(|_| ImportError::NotJson)?;
    let Some(object) = file.as_object() else {
        return Err(ImportError::NotAProgramFile);
    };
    let programs = if object.contains_key("format") {
        envelope_programs(object)?
    } else if object.contains_key("payload") && object.contains_key("plugin_id") {
        // A bare RackForge program document.
        vec![file.clone()]
    } else {
        return Err(ImportError::NotAProgramFile);
    };
    programs
        .iter()
        .enumerate()
        .map(|(place, program)| program_of(program, place + 1))
        .collect()
}

fn envelope_programs(object: &Map<String, Value>) -> Result<Vec<Value>, ImportError> {
    if object.get("format").and_then(Value::as_str) != Some(FORMAT) {
        return Err(ImportError::NotAProgramFile);
    }
    match object.get("schema_version").and_then(Value::as_u64) {
        Some(SCHEMA_VERSION) => {}
        Some(version) if version > SCHEMA_VERSION => {
            return Err(ImportError::NewerFormat(version));
        }
        _ => return Err(ImportError::NotAProgramFile),
    }
    if let Some(plugin) = object.get("plugin_id").and_then(Value::as_str)
        && plugin != PLUGIN_ID
    {
        return Err(ImportError::OtherPlugin(plugin.to_owned()));
    }
    let programs = object
        .get("programs")
        .and_then(Value::as_array)
        .ok_or(ImportError::NotAProgramFile)?;
    if programs.is_empty() {
        return Err(ImportError::NoPrograms);
    }
    if programs.len() > MAX_PROGRAMS {
        return Err(ImportError::TooManyPrograms(programs.len()));
    }
    let integrity = object.get("integrity").ok_or(ImportError::Damaged)?;
    let sealed = integrity.get("algorithm").and_then(Value::as_str) == Some(DIGEST)
        && integrity.get("digest").and_then(Value::as_str) == Some(&digest(programs));
    if !sealed {
        return Err(ImportError::Damaged);
    }
    Ok(programs.clone())
}

fn program_of(program: &Value, place: usize) -> Result<Imported, ImportError> {
    let object = program.as_object().ok_or(ImportError::Malformed(place))?;
    match object.get("plugin_id").and_then(Value::as_str) {
        Some(PLUGIN_ID) => {}
        Some(other) => return Err(ImportError::OtherPlugin(other.to_owned())),
        None => return Err(ImportError::Malformed(place)),
    }
    match object.get("payload_version").and_then(Value::as_u64) {
        Some(PAYLOAD_VERSION) => {}
        Some(version) if version > PAYLOAD_VERSION => {
            return Err(ImportError::NewerProgram(place));
        }
        _ => return Err(ImportError::Malformed(place)),
    }
    let payload = object
        .get("payload")
        .filter(|payload| payload.get("parameters").is_some_and(Value::is_object))
        .ok_or(ImportError::Malformed(place))?;
    let name = object
        .get("name")
        .and_then(Value::as_str)
        .ok_or(ImportError::Malformed(place))?;
    let category = object
        .get("category")
        .and_then(Value::as_str)
        .map(clean_name)
        .filter(|category| !category.is_empty());
    let imported = Imported {
        name: Some(clean_name(name))
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| format!("Imported program {place}")),
        category,
        payload: payload.clone(),
    };
    // What will go into the draft must fit it.
    if canonical(&imported.payload).len() + 1024 > MAX_DOCUMENT_BYTES {
        return Err(ImportError::Malformed(place));
    }
    Ok(imported)
}

/// A name RackForge keeps: no control or format characters, spaces
/// collapsed, at most 64 characters.
pub fn clean_name(name: &str) -> String {
    let words: Vec<&str> = name
        .split(|c: char| c.is_whitespace() || c.is_control() || is_format(c))
        .filter(|word| !word.is_empty())
        .collect();
    words
        .join(" ")
        .chars()
        .take(MAX_NAME_CHARS)
        .collect::<String>()
        .trim_end()
        .to_owned()
}

/// Unicode's format characters (Cf) that a name might carry: the zero-width
/// and direction marks, the soft hyphen and the byte-order mark.
fn is_format(c: char) -> bool {
    matches!(
        c,
        '\u{00ad}'
            | '\u{0600}'..='\u{0605}'
            | '\u{061c}'
            | '\u{06dd}'
            | '\u{070f}'
            | '\u{180e}'
            | '\u{200b}'..='\u{200f}'
            | '\u{202a}'..='\u{202e}'
            | '\u{2060}'..='\u{2064}'
            | '\u{2066}'..='\u{206f}'
            | '\u{feff}'
            | '\u{fff9}'..='\u{fffb}'
    )
}

/// The document a new draft becomes with an imported program in it: the
/// draft's identity -- its id, the plugin and the version that stores it --
/// with the imported name, category and payload. `None` when the draft is
/// not a document or the result would not fit a draft.
pub fn adopt(draft: &Value, imported: &Imported) -> Option<Value> {
    let mut document = draft.as_object()?.clone();
    if document.get("plugin_id").and_then(Value::as_str) != Some(PLUGIN_ID) {
        return None;
    }
    document.insert("name".to_owned(), json!(imported.name));
    match &imported.category {
        Some(category) => document.insert("category".to_owned(), json!(category)),
        None => document.remove("category"),
    };
    document.insert("payload_version".to_owned(), json!(PAYLOAD_VERSION));
    document.insert("payload".to_owned(), imported.payload.clone());
    let document = Value::Object(document);
    (serde_json::to_string(&document).ok()?.len() <= MAX_DOCUMENT_BYTES).then_some(document)
}

/// A file name for a program: its name in characters every system takes.
pub fn file_name(name: &str) -> String {
    let mut stem = String::new();
    for c in name.chars() {
        let keep = c.is_alphanumeric() || matches!(c, ' ' | '-' | '_' | '(' | ')' | '.');
        let c = if keep && !c.is_control() { c } else { '-' };
        if !(c == '-' && stem.ends_with('-')) {
            stem.push(c);
        }
    }
    let stem: String = stem
        .trim_matches(|c: char| c == '.' || c == '-' || c.is_whitespace())
        .chars()
        .take(60)
        .collect();
    let stem = stem.trim_end();
    format!(
        "{}.{EXTENSION}",
        if stem.is_empty() { "program" } else { stem }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn document(id: &str, name: &str, tremolo: f64) -> Value {
        json!({
            "schema_version": 1,
            "id": id,
            "name": name,
            "plugin_id": PLUGIN_ID,
            "plugin_version": "0.13.8",
            "plugin_state_version": 2,
            "payload_version": 1,
            "category": "Musette",
            "tags": ["musette", "user"],
            "payload": { "parameters": { "tremolo": tremolo, "register": 8.0 } },
        })
    }

    #[test]
    fn a_written_file_reads_back_as_its_programs() {
        let programs = [
            document("user-01", "Paris", 5.9),
            document("user-02", "Glasgow", 6.7),
        ];
        let text = write(&programs, "RF-Musette 0.13.8");
        let read = read(text.as_bytes()).unwrap();
        assert_eq!(read.len(), 2);
        assert_eq!(read[0].name, "Paris");
        assert_eq!(read[0].category.as_deref(), Some("Musette"));
        assert_eq!(read[1].payload["parameters"]["tremolo"], 6.7);
    }

    #[test]
    fn the_seal_does_not_hang_on_key_order() {
        let a: Value = serde_json::from_str(r#"{"b":1,"a":{"y":2.5,"x":"é"}}"#).unwrap();
        let b: Value = serde_json::from_str(r#"{"a":{"x":"é","y":2.5},"b":1}"#).unwrap();
        assert_eq!(canonical(&a), r#"{"a":{"x":"é","y":2.5},"b":1}"#);
        assert_eq!(digest(&[a]), digest(&[b]));
        // SHA-256 of "[]", the empty file's seal.
        assert_eq!(
            digest(&[]),
            "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945"
        );
    }

    #[test]
    fn a_changed_or_unsealed_file_is_refused_whole() {
        let text = write(&[document("user-01", "Paris", 5.9)], "RF-Musette");
        let changed = text.replace("5.9", "9.5");
        assert_eq!(read(changed.as_bytes()), Err(ImportError::Damaged));
        let mut file: Value = serde_json::from_str(&text).unwrap();
        file.as_object_mut().unwrap().remove("integrity");
        assert_eq!(read(file.to_string().as_bytes()), Err(ImportError::Damaged));
    }

    #[test]
    fn foreign_newer_and_malformed_files_are_refused() {
        let mut foreign = document("user-01", "Brass", 1.0);
        foreign["plugin_id"] = json!("org.rackforge.rf-5");
        assert_eq!(
            read(write(&[foreign.clone()], "x").as_bytes()),
            Err(ImportError::OtherPlugin("org.rackforge.rf-5".to_owned()))
        );
        assert_eq!(
            read(foreign.to_string().as_bytes()),
            Err(ImportError::OtherPlugin("org.rackforge.rf-5".to_owned()))
        );
        let mut newer: Value =
            serde_json::from_str(&write(&[document("a", "A", 1.0)], "x")).unwrap();
        newer["schema_version"] = json!(2);
        assert_eq!(
            read(newer.to_string().as_bytes()),
            Err(ImportError::NewerFormat(2))
        );
        let mut payload = document("user-01", "A", 1.0);
        payload["payload_version"] = json!(2);
        assert_eq!(
            read(write(&[payload], "x").as_bytes()),
            Err(ImportError::NewerProgram(1))
        );
        let mut empty = document("user-01", "A", 1.0);
        empty["payload"] = json!({ "memory": "00" });
        assert_eq!(
            read(write(&[document("a", "A", 1.0), empty], "x").as_bytes()),
            Err(ImportError::Malformed(2))
        );
        assert_eq!(read(b"not json"), Err(ImportError::NotJson));
        assert_eq!(read(b"{\"hello\":1}"), Err(ImportError::NotAProgramFile));
        assert_eq!(read(b"[1,2]"), Err(ImportError::NotAProgramFile));
        assert_eq!(
            read(write(&[], "x").as_bytes()),
            Err(ImportError::NoPrograms)
        );
        let many: Vec<Value> = (0..=MAX_PROGRAMS)
            .map(|n| document(&format!("u{n}"), "A", 1.0))
            .collect();
        assert_eq!(
            read(write(&many, "x").as_bytes()),
            Err(ImportError::TooManyPrograms(MAX_PROGRAMS + 1))
        );
        assert_eq!(
            read(&vec![b' '; MAX_FILE_BYTES + 1]),
            Err(ImportError::TooLarge)
        );
    }

    #[test]
    fn a_bare_program_document_is_read_as_one_program() {
        let read = read(document("user-07", "Bare", 4.0).to_string().as_bytes()).unwrap();
        assert_eq!(read.len(), 1);
        assert_eq!(read[0].name, "Bare");
        let marked = format!("\u{feff}{}", write(&[document("a", "Marked", 1.0)], "x"));
        assert_eq!(super::read(marked.as_bytes()).unwrap()[0].name, "Marked");
    }

    #[test]
    fn names_are_made_ones_rackforge_keeps() {
        assert_eq!(clean_name("  Mi\u{200b} Musette\n\t2  "), "Mi Musette 2");
        assert_eq!(clean_name(&"x".repeat(80)).chars().count(), MAX_NAME_CHARS);
        let mut unnamed = document("user-01", "\u{202e}  ", 1.0);
        unnamed["category"] = Value::Null;
        let read = read(write(&[unnamed], "x").as_bytes()).unwrap();
        assert_eq!(read[0].name, "Imported program 1");
        assert_eq!(read[0].category, None);
    }

    #[test]
    fn an_imported_program_takes_the_drafts_identity() {
        let draft = document("user-05", "New program", 0.0);
        let mut draft = draft;
        draft.as_object_mut().unwrap().remove("category");
        let imported = Imported {
            name: "Paris".to_owned(),
            category: Some("Musette".to_owned()),
            payload: json!({ "parameters": { "tremolo": 5.9 } }),
        };
        let adopted = adopt(&draft, &imported).unwrap();
        assert_eq!(adopted["id"], "user-05");
        assert_eq!(adopted["name"], "Paris");
        assert_eq!(adopted["category"], "Musette");
        assert_eq!(adopted["payload"]["parameters"]["tremolo"], 5.9);
        assert_eq!(adopted["plugin_version"], "0.13.8");
        let mut foreign = draft.clone();
        foreign["plugin_id"] = json!("org.rackforge.rf-5");
        assert!(adopt(&foreign, &imported).is_none());
    }

    #[test]
    fn file_names_are_safe_everywhere() {
        assert_eq!(file_name("Musette Paris"), "Musette Paris.rfmusette");
        assert_eq!(file_name("a/b\\c:d*?"), "a-b-c-d.rfmusette");
        assert_eq!(file_name("..."), "program.rfmusette");
        assert_eq!(file_name("Señor (2)"), "Señor (2).rfmusette");
    }
}
