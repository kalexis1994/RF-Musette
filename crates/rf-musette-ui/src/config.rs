//! The CONFIG surface: the player's own programs, exported to and imported
//! from `.rfmusette` files ([`crate::archive`]). The face is PLAY's.
//!
//! RackForge gives a plugin page no way to read or write its programs but a
//! program draft (RackForge `docs/WEB_PLUGIN_API.md`), so both go through
//! one, a program at a time:
//! * exporting opens a draft of the saved program, takes its document from
//!   the context, and cancels the draft;
//! * importing opens a new draft, puts the imported program in it
//!   (`plugin.replace_program_draft`, CONFIG's alone) and saves it.
//!
//! Every step waits for RackForge's answer, and for the context that shows
//! the draft opened or closed, at most [`TIMEOUT`] ms. When a step fails the
//! draft this page opened is cancelled, so nothing is left half saved.
//!
//! A draft makes RackForge audition it, and closing one sets the instrument
//! back to the program it was on, as that program was saved. The panel the
//! player had, turned knobs and all, is read before and set again after.

use crate::{archive, escape_html};
use js_sys::{Array, Date, Object, Reflect};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{cell::RefCell, collections::BTreeMap, collections::VecDeque, rc::Rc};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::{
    Blob, BlobPropertyBag, Document, Element, Event, File, HtmlAnchorElement, HtmlInputElement,
    MessageEvent, MouseEvent, Url, Window,
};

const PROTOCOL: &str = "rackforge.plugin.web@1";
/// How long a step waits for RackForge, ms: a save writes a file.
const TIMEOUT: i32 = 8_000;
/// The catalog ids of the player's own programs begin with this.
const CUSTOM: &str = "custom.";

type Handle = Rc<RefCell<Config>>;
type Answer = Box<dyn FnOnce(&Handle, Result<JsValue, String>)>;
type DraftAnswer = Box<dyn FnOnce(&Handle, Result<Option<Draft>, String>)>;

#[derive(Deserialize)]
struct HostContext {
    instance: Instance,
    #[serde(default)]
    program_draft: Option<Draft>,
    #[serde(default)]
    host: Option<HostState>,
}

#[derive(Deserialize)]
struct Instance {
    #[serde(default)]
    sounds: Vec<Sound>,
}

#[derive(Deserialize, Clone)]
struct Sound {
    id: String,
    name: String,
    #[serde(default)]
    category: Option<String>,
    #[serde(default)]
    editable: bool,
}

#[derive(Deserialize, Clone)]
struct Draft {
    draft_id: u64,
    #[serde(default)]
    name: String,
    #[serde(default)]
    original_program_id: Option<String>,
    document_json: String,
}

#[derive(Deserialize)]
struct HostState {
    #[serde(default)]
    lighting: Option<String>,
}

#[derive(Deserialize)]
struct Snapshot {
    values: Vec<ParameterValue>,
}

#[derive(Deserialize, Clone, Copy)]
struct ParameterValue {
    index: u32,
    value: f64,
}

#[derive(Serialize)]
struct Request<'a> {
    protocol: &'static str,
    kind: &'static str,
    request_id: &'a str,
    method: &'a str,
    params: Value,
}

/// What the page waits to see in a context.
enum Want {
    /// A draft other than `not`, of `program` (`None`: a new one).
    Opened {
        program: Option<String>,
        not: Option<u64>,
    },
    /// No draft.
    Closed,
}

struct Waiter {
    want: Want,
    answer: DraftAnswer,
    generation: u64,
}

enum Status {
    Idle,
    Working(String),
    Done(String),
    Failed(String),
}

struct Config {
    window: Window,
    document: Document,
    root: Element,
    file_input: HtmlInputElement,
    host_origin: String,
    /// A context has come: the programs below are RackForge's.
    connected: bool,
    programs: Vec<Sound>,
    draft: Option<Draft>,
    pending: BTreeMap<String, Answer>,
    sequence: u64,
    waiter: Option<Waiter>,
    generation: u64,
    /// A job is running: export or import.
    busy: bool,
    /// The draft this page opened and has not yet closed.
    ours: Option<u64>,
    /// The panel as it was before the job.
    panel: Option<Vec<ParameterValue>>,
    status: Status,
}

impl Config {
    fn render(&self) {
        let mut html = String::from("<div class=\"musette config\">");
        html.push_str(
            "<header class=\"grille\"><div class=\"nameplate\" role=\"img\" aria-label=\"RF-Musette, RackForge Instruments\"><span class=\"title\">RF-Musette</span><span class=\"maker\">RackForge Instruments</span></div><div class=\"surface-plate\"><span>Config</span></div></header>",
        );
        // What a job is doing, or how it ended, above the list it changes.
        html.push_str(&self.render_status());
        html.push_str("<main class=\"panel config-panel\">");
        if let Some(draft) = &self.draft {
            html.push_str(&self.render_pending(draft));
        }
        html.push_str(&self.render_programs());
        html.push_str(&self.render_import());
        html.push_str("</main>");
        html.push_str("</div>");
        self.root.set_inner_html(&html);
    }

    /// Nothing can be exported or imported while a draft is open.
    fn blocked(&self) -> bool {
        self.busy || !self.connected || self.draft.is_some()
    }

    fn disabled(&self) -> &'static str {
        if self.blocked() { " disabled" } else { "" }
    }

    fn render_pending(&self, draft: &Draft) -> String {
        let name = if draft.name.trim().is_empty() {
            "a program".to_owned()
        } else {
            format!("“{}”", escape_html(draft.name.trim()))
        };
        let discard = if self.busy {
            String::new()
        } else {
            "<button type=\"button\" class=\"save-button\" data-action=\"discard\">Discard edit</button>".to_owned()
        };
        format!(
            "<section class=\"group group-pending\"><h2>Unsaved edit</h2><p class=\"note\">{name} is being saved or edited. Programs are exported and imported once it is saved, or discarded here.</p><div class=\"actions\">{discard}</div></section>",
        )
    }

    fn render_programs(&self) -> String {
        let disabled = self.disabled();
        let list = if !self.connected {
            "<p class=\"note\">Waiting for RackForge…</p>".to_owned()
        } else if self.programs.is_empty() {
            "<p class=\"note\">Programs you save on PLAY are listed here, to export.</p>".to_owned()
        } else {
            let mut rows = String::new();
            for program in &self.programs {
                let category = program
                    .category
                    .as_deref()
                    .map(|category| {
                        format!(
                            "<span class=\"program-category\">{}</span>",
                            escape_html(category)
                        )
                    })
                    .unwrap_or_default();
                rows.push_str(&format!(
                    "<li class=\"program-row\"><span class=\"program-text\"><span class=\"program-name\">{}</span>{category}</span><button type=\"button\" class=\"row-button\" data-action=\"export-one\" data-id=\"{}\"{disabled}>Export</button></li>",
                    escape_html(&program.name),
                    escape_html(&program.id),
                ));
            }
            format!("<ul class=\"program-list\">{rows}</ul>")
        };
        let count = if self.connected {
            format!(
                "<span class=\"count\">{} of {}</span>",
                self.programs.len(),
                archive::MAX_PROGRAMS
            )
        } else {
            String::new()
        };
        let all = if self.programs.len() > 1 {
            format!(
                "<div class=\"actions\"><button type=\"button\" class=\"save-button\" data-action=\"export-all\"{disabled}>Export all</button></div>"
            )
        } else {
            String::new()
        };
        format!(
            "<section class=\"group group-programs\"><h2>Your programs {count}</h2>{list}{all}</section>"
        )
    }

    fn render_import(&self) -> String {
        format!(
            "<section class=\"group group-import\"><h2>Import</h2><p class=\"note\">Programs from an <b>.rfmusette</b> file come in as new programs, after yours; none is replaced. A file that was changed after it was exported is refused whole.</p><div class=\"actions\"><button type=\"button\" class=\"save-button\" data-action=\"import\"{}>Import file…</button></div></section>",
            self.disabled()
        )
    }

    fn render_status(&self) -> String {
        let (class, text) = match &self.status {
            Status::Idle => return "<p class=\"status\" aria-live=\"polite\"></p>".to_owned(),
            Status::Working(text) => ("working", text),
            Status::Done(text) => ("done", text),
            Status::Failed(text) => ("failed", text),
        };
        format!(
            "<p class=\"status {class}\" aria-live=\"polite\">{}</p>",
            escape_html(text)
        )
    }
}

fn set_status(app: &Handle, status: Status) {
    app.borrow_mut().status = status;
    app.borrow().render();
}

fn request(
    app: &Handle,
    method: &str,
    params: Value,
    answer: impl FnOnce(&Handle, Result<JsValue, String>) + 'static,
) {
    let (id, window, origin) = {
        let mut state = app.borrow_mut();
        state.sequence += 1;
        let id = format!("rf-musette-config-{}", state.sequence);
        state.pending.insert(id.clone(), Box::new(answer));
        (id, state.window.clone(), state.host_origin.clone())
    };
    let message = Request {
        protocol: PROTOCOL,
        kind: "request",
        request_id: &id,
        method,
        params,
    };
    let sent = message
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|error| error.to_string())
        .and_then(|message| {
            let parent = window
                .parent()
                .ok()
                .flatten()
                .ok_or_else(|| "RackForge's window is missing.".to_owned())?;
            parent
                .post_message(&message, &origin)
                .map_err(|error| format!("postMessage failed: {error:?}"))
        });
    if let Err(error) = sent {
        resolve(app, &id, Err(error));
        return;
    }
    let weak = Rc::downgrade(app);
    let timeout_id = id.clone();
    let timeout = Closure::once_into_js(move || {
        if let Some(app) = weak.upgrade() {
            resolve(
                &app,
                &timeout_id,
                Err("RackForge did not answer in time.".to_owned()),
            );
        }
    });
    let _ = window
        .set_timeout_with_callback_and_timeout_and_arguments_0(timeout.unchecked_ref(), TIMEOUT);
}

fn resolve(app: &Handle, id: &str, result: Result<JsValue, String>) {
    let answer = app.borrow_mut().pending.remove(id);
    if let Some(answer) = answer {
        answer(app, result);
    }
}

/// Waits until a context shows what is wanted -- at once if the last one
/// did -- or [`TIMEOUT`] passes.
fn wait_for(app: &Handle, want: Want, answer: DraftAnswer) {
    let generation = {
        let mut state = app.borrow_mut();
        state.generation += 1;
        state.generation
    };
    app.borrow_mut().waiter = Some(Waiter {
        want,
        answer,
        generation,
    });
    check_waiter(app);
    let weak = Rc::downgrade(app);
    let timeout = Closure::once_into_js(move || {
        let Some(app) = weak.upgrade() else {
            return;
        };
        let waiter = {
            let mut state = app.borrow_mut();
            match &state.waiter {
                Some(waiter) if waiter.generation == generation => state.waiter.take(),
                _ => None,
            }
        };
        if let Some(waiter) = waiter {
            (waiter.answer)(
                &app,
                Err("RackForge did not show the program in time.".to_owned()),
            );
        }
    });
    let window = app.borrow().window.clone();
    let _ = window
        .set_timeout_with_callback_and_timeout_and_arguments_0(timeout.unchecked_ref(), TIMEOUT);
}

fn check_waiter(app: &Handle) {
    let met = {
        let state = app.borrow();
        let Some(waiter) = &state.waiter else {
            return;
        };
        match (&waiter.want, &state.draft) {
            (Want::Closed, None) => Some(None),
            (Want::Opened { program, not }, Some(draft))
                if Some(draft.draft_id) != *not
                    && draft.original_program_id.as_deref() == program.as_deref() =>
            {
                Some(Some(draft.clone()))
            }
            _ => None,
        }
    };
    if let Some(draft) = met {
        let waiter = app.borrow_mut().waiter.take();
        if let Some(waiter) = waiter {
            (waiter.answer)(app, Ok(draft));
        }
    }
}

/// Opens a draft of one of the player's programs, or a new one.
fn open_draft(app: &Handle, program: Option<String>, answer: DraftAnswer) {
    let not = app.borrow().draft.as_ref().map(|draft| draft.draft_id);
    request(
        app,
        "plugin.begin_program_edit",
        json!({ "program_id": program }),
        move |app, result| match result {
            Ok(_) => wait_for(
                app,
                Want::Opened { program, not },
                Box::new(|app, result| {
                    if let Ok(Some(draft)) = &result {
                        app.borrow_mut().ours = Some(draft.draft_id);
                    }
                    answer(app, result);
                }),
            ),
            Err(error) => answer(app, Err(error)),
        },
    );
}

/// Saves or cancels the draft this page opened.
fn close_draft(app: &Handle, method: &'static str, draft_id: u64, answer: DraftAnswer) {
    request(
        app,
        method,
        json!({ "draft_id": draft_id }),
        move |app, result| match result {
            Ok(_) => {
                app.borrow_mut().ours = None;
                wait_for(app, Want::Closed, answer);
            }
            Err(error) => answer(app, Err(error)),
        },
    );
}

/// Starts a job once the panel is read, so it can be set again after.
fn begin_job(app: &Handle, label: &str, next: impl FnOnce(&Handle) + 'static) {
    if app.borrow().blocked() {
        return;
    }
    {
        let mut state = app.borrow_mut();
        state.busy = true;
        state.status = Status::Working(label.to_owned());
    }
    app.borrow().render();
    request(
        app,
        "plugin.parameters",
        json!({}),
        move |app, result| match result.and_then(|value| {
            serde_wasm_bindgen::from_value::<Snapshot>(value).map_err(|error| error.to_string())
        }) {
            Ok(snapshot) => {
                app.borrow_mut().panel = Some(snapshot.values);
                next(app);
            }
            Err(error) => fail(app, format!("The panel could not be read: {error}")),
        },
    );
}

/// The panel as it was before the job, where RackForge set it back to the
/// saved program.
fn restore_panel(app: &Handle) {
    let Some(before) = app.borrow_mut().panel.take() else {
        return;
    };
    request(app, "plugin.parameters", json!({}), move |app, result| {
        let Ok(now) = result.and_then(|value| {
            serde_wasm_bindgen::from_value::<Snapshot>(value).map_err(|error| error.to_string())
        }) else {
            return;
        };
        for value in before {
            let changed = now
                .values
                .iter()
                .find(|now| now.index == value.index)
                .is_none_or(|now| now.value != value.value);
            if changed {
                request(
                    app,
                    "plugin.set_parameter",
                    json!({ "parameter_index": value.index, "value": value.value }),
                    |_, _| {},
                );
            }
        }
    });
}

fn finish(app: &Handle, status: Status) {
    restore_panel(app);
    {
        let mut state = app.borrow_mut();
        state.busy = false;
        state.status = status;
    }
    app.borrow().render();
}

/// A step failed: the draft this page opened is cancelled, and the panel
/// set back.
fn fail(app: &Handle, error: String) {
    app.borrow_mut().waiter = None;
    let ours = app.borrow_mut().ours.take();
    match ours {
        Some(draft_id) => request(
            app,
            "plugin.cancel_program",
            json!({ "draft_id": draft_id }),
            move |app, _| finish(app, Status::Failed(error)),
        ),
        None => finish(app, Status::Failed(error)),
    }
}

fn export(app: &Handle, ids: Vec<String>, file_name: String) {
    let total = ids.len();
    begin_job(app, "Reading programs…", move |app| {
        export_next(app, ids.into(), Vec::new(), total, file_name);
    });
}

fn export_next(
    app: &Handle,
    mut ids: VecDeque<String>,
    mut documents: Vec<Value>,
    total: usize,
    file_name: String,
) {
    let Some(id) = ids.pop_front() else {
        let text = archive::write(
            &documents,
            &format!("RF-Musette {}", env!("CARGO_PKG_VERSION")),
        );
        let status = match download(app, &file_name, &text) {
            Ok(()) if total == 1 => Status::Done(format!("Exported to {file_name}.")),
            Ok(()) => Status::Done(format!("Exported {total} programs to {file_name}.")),
            Err(error) => Status::Failed(format!("The file could not be saved: {error}")),
        };
        finish(app, status);
        return;
    };
    set_status(
        app,
        Status::Working(format!(
            "Reading program {} of {total}…",
            documents.len() + 1
        )),
    );
    open_draft(
        app,
        Some(id),
        Box::new(move |app, result| {
            let draft = match result {
                Ok(Some(draft)) => draft,
                Ok(None) => return fail(app, "RackForge opened no program.".to_owned()),
                Err(error) => return fail(app, error),
            };
            match serde_json::from_str::<Value>(&draft.document_json) {
                Ok(document) => documents.push(document),
                Err(_) => return fail(app, "RackForge gave a malformed program.".to_owned()),
            }
            close_draft(
                app,
                "plugin.cancel_program",
                draft.draft_id,
                Box::new(move |app, result| match result {
                    Ok(_) => export_next(app, ids, documents, total, file_name),
                    Err(error) => fail(app, error),
                }),
            );
        }),
    );
}

fn import(app: &Handle, file: File) {
    if file.size() > archive::MAX_FILE_BYTES as f64 {
        set_status(
            app,
            Status::Failed(archive::ImportError::TooLarge.to_string()),
        );
        return;
    }
    let weak = Rc::downgrade(app);
    let read = Closure::once(move |text: JsValue| {
        let Some(app) = weak.upgrade() else {
            return;
        };
        let text = text.as_string().unwrap_or_default();
        let programs = match archive::read(text.as_bytes()) {
            Ok(programs) => programs,
            Err(error) => {
                set_status(&app, Status::Failed(error.to_string()));
                return;
            }
        };
        let saved = app.borrow().programs.len();
        if saved + programs.len() > archive::MAX_PROGRAMS {
            set_status(
                &app,
                Status::Failed(format!(
                    "RF-Musette keeps {} programs: {saved} are saved and the file holds {}.",
                    archive::MAX_PROGRAMS,
                    programs.len()
                )),
            );
            return;
        }
        let total = programs.len();
        begin_job(&app, "Importing…", move |app| {
            import_next(app, programs.into(), Vec::new(), total);
        });
    });
    let weak = Rc::downgrade(app);
    let refused = Closure::once(move |_: JsValue| {
        if let Some(app) = weak.upgrade() {
            set_status(
                &app,
                Status::Failed("The file could not be read.".to_owned()),
            );
        }
    });
    let blob: &Blob = file.as_ref();
    let _ = blob.text().then2(&read, &refused);
    // One of the two runs; both live until then.
    read.forget();
    refused.forget();
}

fn import_next(
    app: &Handle,
    mut programs: VecDeque<archive::Imported>,
    mut names: Vec<String>,
    total: usize,
) {
    let Some(program) = programs.pop_front() else {
        let status = if total == 1 {
            format!("Imported “{}”.", names.join(""))
        } else {
            format!("Imported {total} programs: {}.", names.join(", "))
        };
        finish(app, Status::Done(status));
        return;
    };
    set_status(
        app,
        Status::Working(format!("Importing program {} of {total}…", names.len() + 1)),
    );
    let done = names.len();
    let partial = move |error: String| {
        if done == 0 {
            error
        } else {
            format!("{done} of {total} programs were imported before this one failed: {error}")
        }
    };
    open_draft(
        app,
        None,
        Box::new(move |app, result| {
            let draft = match result {
                Ok(Some(draft)) => draft,
                Ok(None) => return fail(app, partial("RackForge opened no program.".to_owned())),
                Err(error) => return fail(app, partial(error)),
            };
            let document = serde_json::from_str::<Value>(&draft.document_json)
                .ok()
                .and_then(|draft| archive::adopt(&draft, &program));
            let Some(document) = document else {
                return fail(
                    app,
                    partial(format!("“{}” does not fit a program.", program.name)),
                );
            };
            let draft_id = draft.draft_id;
            request(
                app,
                "plugin.replace_program_draft",
                json!({ "draft_id": draft_id, "document": document }),
                move |app, result| {
                    if let Err(error) = result {
                        return fail(
                            app,
                            partial(format!("RF-Musette refused “{}”: {error}", program.name)),
                        );
                    }
                    close_draft(
                        app,
                        "plugin.save_program",
                        draft_id,
                        Box::new(move |app, result| match result {
                            Ok(_) => {
                                names.push(program.name);
                                import_next(app, programs, names, total);
                            }
                            Err(error) => fail(app, partial(error)),
                        }),
                    );
                },
            );
        }),
    );
}

fn discard(app: &Handle) {
    let Some(draft_id) = app.borrow().draft.as_ref().map(|draft| draft.draft_id) else {
        return;
    };
    if app.borrow().busy {
        return;
    }
    request(
        app,
        "plugin.cancel_program",
        json!({ "draft_id": draft_id }),
        |app, result| {
            let status = match result {
                Ok(_) => Status::Done("The edit was discarded.".to_owned()),
                Err(error) => Status::Failed(format!("The edit could not be discarded: {error}")),
            };
            set_status(app, status);
        },
    );
}

/// Offers the text as a file to save.
fn download(app: &Handle, file_name: &str, text: &str) -> Result<(), String> {
    let (document, window) = {
        let state = app.borrow();
        (state.document.clone(), state.window.clone())
    };
    let parts = Array::of1(&JsValue::from_str(text));
    let options = BlobPropertyBag::new();
    options.set_type("application/json");
    let blob = Blob::new_with_str_sequence_and_options(&parts, &options)
        .map_err(|error| format!("{error:?}"))?;
    let url = Url::create_object_url_with_blob(&blob).map_err(|error| format!("{error:?}"))?;
    let anchor: HtmlAnchorElement = document
        .create_element("a")
        .map_err(|error| format!("{error:?}"))?
        .unchecked_into();
    anchor.set_href(&url);
    anchor.set_download(file_name);
    if let Some(body) = document.body() {
        let _ = body.append_child(&anchor);
        anchor.click();
        let _ = body.remove_child(&anchor);
    }
    // The URL outlives the click a little, as some browsers start the
    // download after it returns.
    let revoke = Closure::once_into_js(move || {
        let _ = Url::revoke_object_url(&url);
    });
    let _ = window
        .set_timeout_with_callback_and_timeout_and_arguments_0(revoke.unchecked_ref(), 60_000);
    Ok(())
}

fn today() -> String {
    let date = Date::new_0();
    format!(
        "{:04}-{:02}-{:02}",
        date.get_full_year(),
        date.get_month() + 1,
        date.get_date()
    )
}

fn on_click(app: &Handle, event: &MouseEvent) {
    if event.button() != 0 {
        return;
    }
    let Some(element) = event
        .target()
        .and_then(|target| target.dyn_into::<Element>().ok())
        .and_then(|element| element.closest("[data-action]").ok().flatten())
    else {
        return;
    };
    if element.has_attribute("disabled") {
        return;
    }
    match element.get_attribute("data-action").as_deref() {
        Some("export-one") => {
            let program = element.get_attribute("data-id").and_then(|id| {
                app.borrow()
                    .programs
                    .iter()
                    .find(|program| program.id == id)
                    .cloned()
            });
            if let Some(program) = program {
                export(app, vec![program.id], archive::file_name(&program.name));
            }
        }
        Some("export-all") => {
            let ids = app
                .borrow()
                .programs
                .iter()
                .map(|program| program.id.clone())
                .collect();
            export(
                app,
                ids,
                archive::file_name(&format!("RF-Musette programs {}", today())),
            );
        }
        Some("import") => {
            if !app.borrow().blocked() {
                let input = app.borrow().file_input.clone();
                input.set_value("");
                input.click();
            }
        }
        Some("discard") => discard(app),
        _ => {}
    }
}

fn on_message(app: &Handle, event: &MessageEvent) {
    let from_parent = app
        .borrow()
        .window
        .parent()
        .ok()
        .flatten()
        .zip(event.source())
        .is_some_and(|(parent, source)| Object::is(parent.as_ref(), source.as_ref()));
    if !from_parent || event.origin() != app.borrow().host_origin {
        return;
    }
    let data = event.data();
    let field = |name: &str| Reflect::get(&data, &JsValue::from_str(name)).ok();
    if field("protocol")
        .and_then(|value| value.as_string())
        .as_deref()
        != Some(PROTOCOL)
    {
        return;
    }
    match field("kind").and_then(|value| value.as_string()).as_deref() {
        Some("context") => {
            let Ok(context) = serde_wasm_bindgen::from_value::<HostContext>(data.clone()) else {
                return;
            };
            if let Some(lighting) = context.host.and_then(|host| host.lighting)
                && let Some(element) = app.borrow().document.document_element()
            {
                let _ = element.set_attribute("data-lighting", &lighting);
            }
            {
                let mut state = app.borrow_mut();
                state.connected = true;
                state.programs = context
                    .instance
                    .sounds
                    .into_iter()
                    .filter(|sound| sound.editable && sound.id.starts_with(CUSTOM))
                    .collect();
                state.draft = context.program_draft;
            }
            check_waiter(app);
            app.borrow().render();
        }
        Some("response") => {
            let Some(request_id) = field("request_id").and_then(|value| value.as_string()) else {
                return;
            };
            let result = if field("ok").and_then(|value| value.as_bool()) == Some(true) {
                Ok(field("result").unwrap_or(JsValue::UNDEFINED))
            } else {
                Err(field("error")
                    .and_then(|value| value.as_string())
                    .unwrap_or_else(|| "RackForge refused the request.".to_owned()))
            };
            resolve(app, &request_id, result);
        }
        _ => {}
    }
}

/// Starts the CONFIG page in `root`.
pub fn start(window: Window, document: Document, root: Element) -> Result<(), JsValue> {
    let host_origin = window.location().origin()?;
    // The file chooser lives outside the redrawn page.
    let file_input: HtmlInputElement = document.create_element("input")?.unchecked_into();
    file_input.set_type("file");
    file_input.set_accept(&format!(".{},.json,application/json", archive::EXTENSION));
    file_input.set_hidden(true);
    if let Some(body) = document.body() {
        body.append_child(&file_input)?;
    }
    let app: Handle = Rc::new(RefCell::new(Config {
        window: window.clone(),
        document,
        root: root.clone(),
        file_input: file_input.clone(),
        host_origin: host_origin.clone(),
        connected: false,
        programs: Vec::new(),
        draft: None,
        pending: BTreeMap::new(),
        sequence: 0,
        waiter: None,
        generation: 0,
        busy: false,
        ours: None,
        panel: None,
        status: Status::Idle,
    }));
    app.borrow().render();

    let click_app = app.clone();
    let click = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
        on_click(&click_app, &event);
    });
    root.add_event_listener_with_callback("click", click.as_ref().unchecked_ref())?;
    click.forget();

    let change_app = app.clone();
    let change = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let file = event
            .target()
            .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
            .and_then(|input| input.files())
            .and_then(|files| files.get(0));
        if let Some(file) = file {
            import(&change_app, file);
        }
    });
    file_input.add_event_listener_with_callback("change", change.as_ref().unchecked_ref())?;
    change.forget();

    let message_app = app.clone();
    let message = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
        on_message(&message_app, &event);
    });
    window.add_event_listener_with_callback("message", message.as_ref().unchecked_ref())?;
    message.forget();

    let ready = json!({ "protocol": PROTOCOL, "kind": "ready" })
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())?;
    let parent = window
        .parent()?
        .ok_or_else(|| JsValue::from_str("missing parent"))?;
    parent.post_message(&ready, &host_origin)?;
    Ok(())
}
