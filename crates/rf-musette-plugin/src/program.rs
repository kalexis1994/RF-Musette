//! The player's own programs: what SAVE keeps, and what CONFIG exports and
//! imports as `.rfmusette` files.
//!
//! A program is the panel: every parameter by its stable id
//! (`rf_musette_dsp::parameters::SPECS`), in its physical units, carried in a
//! RackForge program document as `{ "parameters": { "<id>": value } }`.
//! RackForge owns the files and hands every saved document back through
//! `prepare` and `install` when an instance starts, and an instance whose
//! saved program is refused does not start. So a document is read leniently
//! and forever: an id this build does not know is ignored, one it lacks
//! keeps its default, and a value out of its range is brought to the nearest
//! it takes. Whatever comes in, `prepare` writes back the program as this
//! build reads it, so what is stored is always what plays.
//!
//! The documents and their envelopes are RackForge's (`rackforge-program-api`);
//! see "Portable individual-program editing" in its docs/PLUGIN_DEVELOPMENT.md.

use std::collections::BTreeMap;

use rackforge_program_api::{
    PROGRAM_EDIT_SCHEMA_VERSION, PROGRAM_EDITOR_SCHEMA_VERSION, PROGRAM_SCHEMA_VERSION,
    PreparedProgram, ProgramDocument, ProgramEditRequest, ProgramEditorChoice, ProgramEditorField,
    ProgramEditorFieldKind, ProgramEditorPage, ProgramEditorValue, ProgramEditorView,
};
use rf_musette_dsp::Parameters;
use rf_musette_dsp::parameters::{self, SPECS};
use rf_musette_dsp::programs::{self, USER_BANK};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Map, Value, json};

/// The plugin, as its manifest names it.
pub const PLUGIN_ID: &str = "org.rackforge.musette";
/// How many of their own programs a player keeps.
pub const MAX_PROGRAMS: usize = 64;
/// The longest name RackForge keeps.
const MAX_NAME_CHARS: usize = 64;
/// What `payload` holds: `{ "parameters": { "<id>": value } }`.
const PAYLOAD_VERSION: u32 = 1;
/// The catalog id of an own program is this and its document's id.
pub const CUSTOM: &str = "custom.";
/// Their catalog order, after the factory programs.
const USER_ORDER: i64 = 1_000;
/// Not part of a program: the two withdrawn parameters, which take one
/// value or none, and the bellows' direction, which is the player's hand
/// at the moment, not the sound. A program recalled sets them as a factory
/// program does, to their defaults.
const NOT_STORED: [usize; 3] = [
    parameters::MOD_WHEEL,
    parameters::BELLOWS_SMOOTHING,
    parameters::BELLOWS_DIRECTION,
];

const FACTORY_CATALOG: &str = include_str!("../../../package/metadata/presets.json");
const RUNTIME: &str = include_str!("../../../package/metadata/runtime.json");

/// The player's own programs, by document id.
pub type Library = BTreeMap<String, ProgramDocument>;

fn read<T: DeserializeOwned>(bytes: &[u8]) -> Option<T> {
    serde_json::from_slice(bytes).ok()
}

fn write<T: Serialize>(value: &T, destination: &mut [u8]) -> Option<usize> {
    let bytes = serde_json::to_vec(value).ok()?;
    destination.get_mut(..bytes.len())?.copy_from_slice(&bytes);
    Some(bytes.len())
}

/// The state version the runtime descriptor declares, stamped on new documents.
fn state_version() -> Option<u32> {
    let runtime: Value = serde_json::from_str(RUNTIME).ok()?;
    u32::try_from(runtime.get("state_version")?.as_u64()?).ok()
}

fn stored(index: usize) -> bool {
    !NOT_STORED.contains(&index)
}

/// The payload of a panel: every stored parameter by its id.
pub fn payload(values: &Parameters) -> Value {
    let parameters: Map<String, Value> = SPECS
        .iter()
        .zip(values.values())
        .enumerate()
        .filter(|(index, _)| stored(*index))
        .map(|(_, (spec, value))| (spec.id.to_owned(), json!(value)))
        .collect();
    json!({ "parameters": parameters })
}

/// The value a stored number gives a parameter: within its range, and for a
/// choice one of its choices, or else its default.
fn admitted(index: usize, value: &Value) -> f64 {
    let spec = &SPECS[index];
    let Some(value) = value.as_f64().filter(|value| value.is_finite()) else {
        return spec.default;
    };
    if spec.choices.is_empty() {
        return value.clamp(spec.minimum, spec.maximum);
    }
    if spec
        .choices
        .iter()
        .any(|(choice, _)| f64::from(*choice) == value)
    {
        value
    } else {
        spec.default
    }
}

/// The panel a document carries, when it is one of RF-Musette's. Any state
/// version is accepted, as the payload names every value by its id.
pub fn values(document: &ProgramDocument) -> Option<Parameters> {
    document.validate().ok()?;
    let name = document.name.trim();
    if document.plugin_id != PLUGIN_ID
        || document.payload_version != PAYLOAD_VERSION
        || name.is_empty()
        || name.chars().count() > MAX_NAME_CHARS
    {
        return None;
    }
    let stored_values = document.payload.get("parameters")?.as_object()?;
    let mut values = Parameters::default();
    for (index, spec) in SPECS.iter().enumerate().filter(|(index, _)| stored(*index)) {
        if let Some(value) = stored_values.get(spec.id) {
            // `admitted` gives only values `set` takes.
            values.set(index, admitted(index, value));
        }
    }
    Some(values)
}

/// The envelope RackForge stores a document in and selects it by, its
/// payload written as this build reads it.
fn envelope(mut document: ProgramDocument) -> Option<PreparedProgram> {
    document.payload = payload(&values(&document)?);
    document.name = document.name.trim().to_owned();
    let prepared = PreparedProgram {
        schema_version: PROGRAM_EDIT_SCHEMA_VERSION,
        storage_path: format!("programs/{}.rackforge-program.json", document.id),
        preview_sound_id: format!("{CUSTOM}{}", document.id),
        document,
        artifacts: Vec::new(),
    };
    prepared.validate().ok()?;
    Some(prepared)
}

/// A prepared program exactly as `envelope` makes it, or nothing.
pub fn validated_prepared(bytes: &[u8]) -> Option<ProgramDocument> {
    let prepared: PreparedProgram = read(bytes)?;
    prepared.validate().ok()?;
    let expected = envelope(prepared.document.clone())?;
    (prepared == expected).then_some(prepared.document)
}

fn new_document(
    id: String,
    name: String,
    category: Option<String>,
    values: &Parameters,
) -> Option<ProgramDocument> {
    Some(ProgramDocument {
        schema_version: PROGRAM_SCHEMA_VERSION,
        id,
        name,
        plugin_id: PLUGIN_ID.to_owned(),
        plugin_version: env!("CARGO_PKG_VERSION").to_owned(),
        plugin_state_version: state_version()?,
        payload_version: PAYLOAD_VERSION,
        category,
        tags: vec!["musette".to_owned(), "user".to_owned()],
        payload: payload(values),
    })
}

/// Starts a draft: of one of the player's own programs, to save over it; of
/// a factory program, as a copy; or, with no id, of what the panel holds now.
///
/// The panel is where a program is edited, so one of the player's own that
/// is the program last loaded (`loaded`, a catalog id) is drafted as the
/// panel holds it now, under its own id and name: SAVE over it keeps what
/// was turned since. Any other is drafted as it was saved, which is what
/// CONFIG exports.
pub fn begin(
    library: &Library,
    panel: &Parameters,
    loaded: Option<&str>,
    request: &[u8],
    destination: &mut [u8],
) -> Option<usize> {
    let request: ProgramEditRequest = read(request)?;
    request.validate().ok()?;
    if let Some(catalog_id) = request
        .program_id
        .as_deref()
        .filter(|id| id.starts_with(CUSTOM))
    {
        let mut document = library.get(&catalog_id[CUSTOM.len()..])?.clone();
        if loaded == Some(catalog_id) {
            document.payload = payload(panel);
        }
        return write(&envelope(document)?, destination);
    }
    let (values, name, category) = match request.program_id.as_deref() {
        None => (*panel, "New program".to_owned(), None),
        Some(id) => {
            let program = programs::program(id)?;
            (
                program.parameters(),
                program.name.to_owned(),
                Some(program.category.to_owned()),
            )
        }
    };
    let id = (1..=MAX_PROGRAMS)
        .map(|number| format!("user-{number:02}"))
        .find(|id| !library.contains_key(id))?;
    write(
        &envelope(new_document(id, name, category, &values)?)?,
        destination,
    )
}

/// Checks a document (a renamed draft, an imported one, or a saved one
/// coming back) and envelopes it for RackForge to store and install.
pub fn prepare(document: &[u8], destination: &mut [u8]) -> Option<usize> {
    write(&envelope(read(document)?)?, destination)
}

/// Adds a prepared program to the library, or replaces the one with its id.
/// A new one is refused when the library is full or would no longer fit the
/// catalog in `transfer_bytes`.
pub fn install(library: &mut Library, prepared: &[u8], transfer_bytes: usize) -> bool {
    let Some(document) = validated_prepared(prepared) else {
        return false;
    };
    if !library.contains_key(&document.id) && library.len() >= MAX_PROGRAMS {
        return false;
    }
    let mut next = library.clone();
    next.insert(document.id.clone(), document);
    if catalog(&next, &mut vec![0; transfer_bytes]).is_none() {
        return false;
    }
    *library = next;
    true
}

/// The catalog: the factory programs, then the player's own in the user bank.
pub fn catalog(library: &Library, destination: &mut [u8]) -> Option<usize> {
    let mut catalog: Value = serde_json::from_str(FACTORY_CATALOG).ok()?;
    let presets = catalog.get_mut("presets")?.as_array_mut()?;
    for (order, document) in (USER_ORDER..).zip(library.values()) {
        let mut entry = json!({
            "id": format!("{CUSTOM}{}", document.id),
            "name": document.name.trim(),
            "bank": USER_BANK,
            "order": order,
            "tags": ["musette", "user"],
            "editable": true,
        });
        if let Some(category) = &document.category {
            entry["category"] = json!(category);
        }
        presets.push(entry);
    }
    write(&catalog, destination)
}

/// The panel of one of the player's own programs, by its catalog id.
pub fn own_values(library: &Library, catalog_id: &str) -> Option<Parameters> {
    values(library.get(catalog_id.strip_prefix(CUSTOM)?)?)
}

/// The editor RackForge's controller surfaces show for a draft. A program
/// is the whole panel, so the tree only says what it is stored as; the
/// panel itself is where it is edited.
pub fn view(document: &[u8], destination: &mut [u8]) -> Option<usize> {
    let document: ProgramDocument = read(document)?;
    values(&document)?;
    let view = ProgramEditorView {
        schema_version: PROGRAM_EDITOR_SCHEMA_VERSION,
        title: "RF-Musette program".to_owned(),
        pages: vec![ProgramEditorPage {
            id: "program".to_owned(),
            label: "Program".to_owned(),
            detail: "The whole panel, every parameter by its id".to_owned(),
            enabled: true,
            pages: Vec::new(),
            fields: vec![ProgramEditorField {
                id: "panel".to_owned(),
                label: "Panel".to_owned(),
                detail: "Edited on PLAY; saved as it stands".to_owned(),
                value: ProgramEditorValue::Choice("panel".to_owned()),
                kind: ProgramEditorFieldKind::Choice {
                    options: vec![ProgramEditorChoice {
                        value: "panel".to_owned(),
                        label: "RF-Musette panel".to_owned(),
                        detail: None,
                    }],
                },
                live_preview: false,
            }],
        }],
    };
    view.validate().ok()?;
    write(&view, destination)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MAX_TRANSFER_BYTES, MusetteProcessor};
    use rackforge_plugin_sdk::Processor;

    fn call(
        processor: &mut MusetteProcessor,
        method: fn(&mut MusetteProcessor, &[u8], &mut [u8]) -> Option<usize>,
        input: &[u8],
    ) -> Option<Vec<u8>> {
        let mut out = vec![0; MAX_TRANSFER_BYTES];
        method(processor, input, &mut out).map(|length| out[..length].to_vec())
    }

    fn begin(
        processor: &mut MusetteProcessor,
        program_id: Option<&str>,
    ) -> Option<PreparedProgram> {
        let request =
            serde_json::to_vec(&ProgramEditRequest::new(program_id.map(str::to_owned))).unwrap();
        let bytes = call(processor, MusetteProcessor::begin_program_edit, &request)?;
        Some(serde_json::from_slice(&bytes).unwrap())
    }

    fn prepare(processor: &mut MusetteProcessor, document: &ProgramDocument) -> Option<Vec<u8>> {
        call(
            processor,
            MusetteProcessor::prepare_program_save,
            &serde_json::to_vec(document).unwrap(),
        )
    }

    /// What `plugin.set_program_name` then `plugin.save_program` do.
    fn save(
        processor: &mut MusetteProcessor,
        mut document: ProgramDocument,
        name: &str,
    ) -> PreparedProgram {
        document.name = name.to_owned();
        let bytes = prepare(processor, &document).expect("a renamed draft prepares");
        assert!(processor.install_program(&bytes));
        serde_json::from_slice(&bytes).unwrap()
    }

    fn catalog(processor: &mut MusetteProcessor) -> Value {
        let mut out = vec![0; MAX_TRANSFER_BYTES];
        let length = processor
            .write_program_catalog(&mut out)
            .expect("catalog fits");
        serde_json::from_slice(&out[..length]).unwrap()
    }

    fn panel(processor: &MusetteProcessor) -> Vec<f64> {
        (0..parameters::COUNT as u32)
            .map(|index| processor.get_parameter(index).unwrap())
            .collect()
    }

    #[test]
    fn save_keeps_the_panel_and_recall_brings_it_back() {
        let mut processor = MusetteProcessor::default();
        assert!(processor.prepare(48_000.0, 256, 0, 2));
        assert!(processor.load_preset("musette-paris"));
        assert!(processor.set_parameter(parameters::PALLET_OPENING as u32, 35.0));
        let before = panel(&processor);

        let draft = begin(&mut processor, None).expect("a new program begins");
        assert_eq!(draft.document.id, "user-01");
        assert_eq!(draft.preview_sound_id, "custom.user-01");
        assert_eq!(
            draft.storage_path,
            "programs/user-01.rackforge-program.json"
        );
        assert_eq!(
            draft.document.plugin_state_version,
            state_version().unwrap()
        );
        let saved = save(&mut processor, draft.document, "  Mi Musette  ");
        assert_eq!(saved.document.name, "Mi Musette", "the name is trimmed");

        let listed = catalog(&mut processor);
        let entry = listed["presets"]
            .as_array()
            .unwrap()
            .iter()
            .find(|preset| preset["id"] == "custom.user-01")
            .expect("the saved program is listed");
        assert_eq!(entry["name"], "Mi Musette");
        assert_eq!(entry["bank"], "user");
        assert_eq!(entry["editable"], true);
        assert!(
            listed["banks"]
                .as_array()
                .unwrap()
                .iter()
                .any(|bank| bank["id"] == "user")
        );

        assert!(processor.load_preset("research"));
        assert_ne!(panel(&processor), before);
        assert!(processor.load_preset(&saved.preview_sound_id));
        assert_eq!(panel(&processor), before);
    }

    #[test]
    fn a_saved_program_is_saved_over_and_a_factory_one_copied() {
        let mut processor = MusetteProcessor::default();
        let first = begin(&mut processor, None).unwrap();
        save(&mut processor, first.document, "Pad");

        let again = begin(&mut processor, Some("custom.user-01")).expect("its own program reopens");
        assert_eq!(again.document.id, "user-01");
        save(&mut processor, again.document, "Pad 2");
        assert_eq!(processor.programs.len(), 1);
        assert_eq!(processor.programs["user-01"].name, "Pad 2");

        // Loaded and then turned, it is saved over as the panel holds it.
        assert!(processor.load_preset("custom.user-01"));
        assert!(processor.set_parameter(parameters::TREMOLO as u32, 6.7));
        let turned = begin(&mut processor, Some("custom.user-01")).unwrap();
        assert_eq!(
            values(&turned.document).unwrap().get(parameters::TREMOLO),
            Some(6.7)
        );
        save(&mut processor, turned.document, "Pad 3");
        // Another loaded, it is drafted as it was saved: what CONFIG exports.
        assert!(processor.load_preset("research"));
        let saved = begin(&mut processor, Some("custom.user-01")).unwrap();
        assert_eq!(
            values(&saved.document).unwrap().get(parameters::TREMOLO),
            Some(6.7)
        );
        assert!(processor.set_parameter(parameters::TREMOLO as u32, 2.0));
        let saved = begin(&mut processor, Some("custom.user-01")).unwrap();
        assert_eq!(
            values(&saved.document).unwrap().get(parameters::TREMOLO),
            Some(6.7)
        );

        let copy = begin(&mut processor, Some("musette-paris")).expect("a factory program copies");
        assert_eq!(copy.document.id, "user-02");
        assert_eq!(copy.document.name, "Musette Paris");
        assert_eq!(copy.document.category.as_deref(), Some("Musette"));
        assert_eq!(
            values(&copy.document).map(|values| *values.values()),
            Some(
                *programs::program("musette-paris")
                    .unwrap()
                    .parameters()
                    .values()
            )
        );
        assert!(begin(&mut processor, Some("custom.user-99")).is_none());
        assert!(begin(&mut processor, Some("no-such-program")).is_none());
    }

    #[test]
    fn saved_programs_come_back_when_an_instance_starts() {
        let mut recorder = MusetteProcessor::default();
        assert!(recorder.load_preset("scottish"));
        let draft = begin(&mut recorder, None).unwrap();
        let saved = save(&mut recorder, draft.document, "Stored");
        // RackForge keeps the document and hands it back to a new instance:
        // prepare, then install, before it first reads the catalog.
        let mut restarted = MusetteProcessor::default();
        let prepared = prepare(&mut restarted, &saved.document).unwrap();
        assert!(restarted.install_program(&prepared));
        assert!(restarted.load_preset("custom.user-01"));
        assert_eq!(panel(&restarted), panel(&recorder));

        // A document an older state version wrote still installs.
        let mut older = saved.document.clone();
        older.plugin_state_version = 1;
        older.plugin_version = "0.13.0".to_owned();
        let prepared = prepare(&mut restarted, &older).unwrap();
        assert!(restarted.install_program(&prepared));
    }

    /// A document from another build: ids it has that this one does not are
    /// ignored, those it lacks keep their defaults, values outside a range
    /// come within it, and what is stored is the program as this build reads it.
    #[test]
    fn documents_from_other_builds_are_read_leniently_and_written_back_exactly() {
        let mut processor = MusetteProcessor::default();
        let draft = begin(&mut processor, None).unwrap();
        let mut other = draft.document.clone();
        other.payload = json!({
            "parameters": {
                "tremolo": 5.9,
                "register": 8.0,
                "a_parameter_from_the_future": 3.0,
                "pallet_opening": 1.0e9,
                "mic_layout": 17.0,
                "gain": "loud",
            },
            "something_else": true,
        });
        let prepared: PreparedProgram =
            serde_json::from_slice(&prepare(&mut processor, &other).unwrap()).unwrap();
        let values = values(&prepared.document).unwrap();
        let spec = |index: usize| &SPECS[index];
        assert_eq!(values.get(parameters::TREMOLO), Some(5.9));
        assert_eq!(values.get(parameters::REGISTER), Some(8.0));
        assert_eq!(
            values.get(parameters::PALLET_OPENING),
            Some(spec(parameters::PALLET_OPENING).maximum)
        );
        assert_eq!(
            values.get(parameters::MIC_LAYOUT),
            Some(spec(parameters::MIC_LAYOUT).default)
        );
        assert_eq!(
            values.get(parameters::GAIN),
            Some(spec(parameters::GAIN).default)
        );
        assert_eq!(
            values.get(parameters::ROOM_SIZE),
            Some(spec(parameters::ROOM_SIZE).default)
        );
        let written = prepared.document.payload["parameters"].as_object().unwrap();
        assert!(!written.contains_key("a_parameter_from_the_future"));
        assert!(!written.contains_key("bellows_direction"));
        assert!(!written.contains_key("mod_wheel"));
        assert_eq!(written.len(), parameters::COUNT - NOT_STORED.len());
        assert_eq!(prepared.document.payload.as_object().unwrap().len(), 1);
        assert!(processor.install_program(&serde_json::to_vec(&prepared).unwrap()));
    }

    fn factory_count() -> usize {
        let catalog: Value = serde_json::from_str(FACTORY_CATALOG).unwrap();
        catalog["presets"].as_array().unwrap().len()
    }

    #[test]
    fn the_library_holds_its_limit_and_the_catalog_still_fits() {
        let mut processor = MusetteProcessor::default();
        for number in 0..MAX_PROGRAMS {
            let draft = begin(&mut processor, None).unwrap();
            let name = format!("{number:02} {}", "x".repeat(MAX_NAME_CHARS - 3));
            save(&mut processor, draft.document, &name);
        }
        assert_eq!(processor.programs.len(), MAX_PROGRAMS);
        assert!(
            begin(&mut processor, None).is_none(),
            "a full library starts no new program"
        );
        let listed = catalog(&mut processor);
        assert_eq!(
            listed["presets"].as_array().unwrap().len(),
            factory_count() + MAX_PROGRAMS
        );
        // Saving over one still works when full.
        let again = begin(&mut processor, Some("custom.user-64")).unwrap();
        save(&mut processor, again.document, "Last");
    }

    #[test]
    fn foreign_or_damaged_documents_are_refused() {
        let mut processor = MusetteProcessor::default();
        let draft = begin(&mut processor, None).unwrap();
        let mut foreign = draft.document.clone();
        foreign.plugin_id = "org.rackforge.rf-5".to_owned();
        assert!(prepare(&mut processor, &foreign).is_none());
        let mut newer = draft.document.clone();
        newer.payload_version = 2;
        assert!(prepare(&mut processor, &newer).is_none());
        let mut empty = draft.document.clone();
        empty.payload = json!({ "memory": "00ff" });
        assert!(prepare(&mut processor, &empty).is_none());
        let mut unnamed = draft.document.clone();
        unnamed.name = "   ".to_owned();
        assert!(prepare(&mut processor, &unnamed).is_none());
        // A prepared program that is not exactly RF-Musette's own envelope.
        let mut tampered = draft.clone();
        tampered.storage_path = "elsewhere.json".to_owned();
        assert!(!processor.install_program(&serde_json::to_vec(&tampered).unwrap()));
        let mut unwritten = draft.clone();
        unwritten.document.payload["parameters"]["tremolo"] = json!(1.0e9);
        assert!(!processor.install_program(&serde_json::to_vec(&unwritten).unwrap()));
    }

    #[test]
    fn a_draft_previews_and_has_an_editor_view() {
        let mut processor = MusetteProcessor::default();
        assert!(processor.prepare(48_000.0, 256, 0, 2));
        let copy = begin(&mut processor, Some("italian")).unwrap();
        assert!(processor.preview_program(&serde_json::to_vec(&copy).unwrap()));
        assert_eq!(
            panel(&processor),
            programs::program("italian")
                .unwrap()
                .parameters()
                .values()
                .to_vec()
        );
        let view = call(
            &mut processor,
            MusetteProcessor::program_editor_view,
            &serde_json::to_vec(&copy.document).unwrap(),
        )
        .expect("an editor view");
        let view: ProgramEditorView = serde_json::from_slice(&view).unwrap();
        assert!(view.validate().is_ok());
        assert_eq!(
            processor.program_editing_capabilities(),
            rackforge_plugin_sdk::PROGRAM_EDIT_KNOWN_CAPABILITIES
        );
    }
}
