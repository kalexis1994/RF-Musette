//! The page in the browser: RackForge's bridge, the panel drawn from
//! [`crate::panel`], and the controls' gestures.
//!
//! The parameters' kinds, ranges and names come from the engine's own table
//! ([`SPECS`]), which the package's `parameters.json` is written from; the
//! host gives the values.

use crate::{
    dial, escape_html, help,
    panel::{self, Control},
    symbols,
};
use js_sys::{Function, Object, Reflect};
use rf_musette_dsp::parameters::{AIR_VALVE, COUNT, SPECS};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};
use wasm_bindgen::{JsCast, JsValue, closure::Closure, prelude::wasm_bindgen};
use web_sys::{
    Document, Element, Event, HtmlElement, KeyboardEvent, MessageEvent, MouseEvent, PointerEvent,
    Window,
};

const PROTOCOL: &str = "rackforge.plugin.web@1";

/// How long a request waits for RackForge, ms.
const TIMEOUT: i32 = 4_000;

type AppHandle = Rc<RefCell<App>>;
type ResponseHandler = Box<dyn FnOnce(&AppHandle, Result<JsValue, String>)>;

#[derive(Deserialize)]
struct HostContext {
    instance: Instance,
    #[serde(default)]
    host: Option<HostState>,
}

#[derive(Deserialize)]
struct Instance {
    selected_sound_id: String,
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

#[derive(Deserialize)]
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
    params: serde_json::Value,
}

#[derive(Serialize)]
struct Ready {
    protocol: &'static str,
    kind: &'static str,
}

/// A gesture in progress: a knob dragged, or the air button held.
enum Gesture {
    Knob {
        pointer: i32,
        index: usize,
        surface: Element,
        start_y: f64,
        start: f64,
    },
    Air {
        pointer: Option<i32>,
        surface: Element,
    },
}

struct App {
    window: Window,
    document: Document,
    root: Element,
    /// RackForge's `<rf-program-select>`, made once and put back in its slot
    /// after every render, so an open list survives.
    program_selector: Element,
    host_origin: String,
    sound: Option<String>,
    values: [f64; COUNT],
    loaded: bool,
    pending: BTreeMap<String, ResponseHandler>,
    sequence: u64,
    generation: u64,
    page: &'static str,
    /// The parameter the player holds: its echoes from the host are
    /// ignored until it is let go.
    held: Option<usize>,
    /// The values were read while a control was held: the panel is drawn
    /// again when it is let go.
    stale: bool,
    error: String,
}

impl App {
    fn new() -> Result<AppHandle, JsValue> {
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("missing window"))?;
        let document = window
            .document()
            .ok_or_else(|| JsValue::from_str("missing document"))?;
        let root = document
            .get_element_by_id("plugin-root")
            .ok_or_else(|| JsValue::from_str("missing #plugin-root"))?;
        let host_origin = window.location().origin()?;
        let program_selector = document.create_element("rf-program-select")?;
        for (name, value) in [
            ("id", "program-selector"),
            ("label", "Program"),
            ("placeholder", "Search RF-Musette programs"),
            ("empty-label", "Waiting for programs…"),
            ("fit", "0.7"),
        ] {
            program_selector.set_attribute(name, value)?;
        }
        // RackForge's save dialog: it draws nothing until SAVE opens it, and
        // lives outside the redrawn panel so a half-typed name survives.
        let program_save = document.create_element("rf-program-save")?;
        for (name, value) in [
            ("id", "program-save"),
            ("heading", "Save program"),
            ("default-name", "New program"),
        ] {
            program_save.set_attribute(name, value)?;
        }
        if let Some(body) = document.body() {
            body.append_child(&program_save)?;
        }
        Ok(Rc::new(RefCell::new(Self {
            window,
            document,
            root,
            program_selector,
            host_origin,
            sound: None,
            values: core::array::from_fn(|index| SPECS[index].default),
            loaded: false,
            pending: BTreeMap::new(),
            sequence: 0,
            generation: 0,
            page: panel::PAGES[0].id,
            held: None,
            stale: false,
            error: String::new(),
        })))
    }

    fn render(&self) {
        let mut html = String::from("<div class=\"musette\">");
        html.push_str(
            "<header class=\"grille\"><div class=\"nameplate\" role=\"img\" aria-label=\"RF-Musette, RackForge Instruments\"><span class=\"title\">RF-Musette</span><span class=\"maker\">RackForge Instruments</span></div><div class=\"program-bar\"><div class=\"program-selector-slot\" id=\"program-selector-slot\"></div><button type=\"button\" class=\"save-button\" data-action=\"save\">Save</button></div></header>",
        );
        html.push_str(&self.render_pages());
        html.push_str(&self.render_panel());
        html.push_str("</div>");
        self.root.set_inner_html(&html);
        if let Some(slot) = self.document.get_element_by_id("program-selector-slot") {
            let _ = slot.append_child(&self.program_selector);
        }
    }

    fn render_pages(&self) -> String {
        let mut keys = String::new();
        for page in panel::PAGES {
            keys.push_str(&format!(
                "<button type=\"button\" class=\"page-key\" data-action=\"page\" data-page=\"{}\" aria-pressed=\"{}\">{}</button>",
                page.id,
                page.id == self.page,
                escape_html(page.label)
            ));
        }
        format!("<nav class=\"pages\" aria-label=\"Panel pages\">{keys}</nav>")
    }

    fn render_panel(&self) -> String {
        if !self.loaded {
            let message = if self.error.is_empty() {
                "Reading the instrument…".to_owned()
            } else {
                escape_html(&self.error)
            };
            return format!(
                "<main class=\"panel loading\"><p>{message}</p><button type=\"button\" class=\"retry-button\" data-action=\"retry\">Retry</button></main>"
            );
        }
        let page = panel::page(self.page);
        let mut groups = String::new();
        for group in page.groups {
            if !panel::shown(group, &self.values) {
                continue;
            }
            let mut controls = String::new();
            for id in group.parameters {
                if let Some(index) = panel::index_of(id) {
                    controls.push_str(&self.render_control(index));
                }
            }
            groups.push_str(&format!(
                "<section class=\"group group-{}\"><h2>{}</h2><div class=\"controls\">{controls}</div></section>",
                group.id,
                escape_html(group.title)
            ));
        }
        let error = if self.error.is_empty() {
            String::new()
        } else {
            format!("<p class=\"bridge-error\">{}</p>", escape_html(&self.error))
        };
        format!(
            "<main class=\"panel page-{}\">{groups}</main>{error}",
            page.id
        )
    }

    /// The control's frame: its class, whether it is idle, and what it does
    /// as its tooltip.
    fn frame(&self, index: usize, kind: &str, inner: &str) -> String {
        let idle = if self.idle(index) { " idle" } else { "" };
        format!(
            "<div class=\"control {kind}-control{idle}\" data-control-index=\"{index}\" title=\"{}\">{inner}</div>",
            escape_html(help::help(SPECS[index].id))
        )
    }

    fn idle(&self, index: usize) -> bool {
        panel::idle(index, &self.values)
    }

    fn render_control(&self, index: usize) -> String {
        match panel::control(index) {
            Control::Registers => self.render_registers(index, symbols::treble, "treble"),
            Control::BassRegisters => self.render_registers(index, symbols::bass, "bass"),
            Control::Air => self.render_air(index),
            Control::Toggle => self.render_toggle(index),
            Control::Choice => self.render_choice(index),
            Control::Knob => self.render_knob(index),
        }
    }

    fn render_knob(&self, index: usize) -> String {
        let spec = &SPECS[index];
        let value = self.values[index];
        let name = escape_html(spec.name);
        let readout = escape_html(&readout(index, value));
        self.frame(
            index,
            "knob",
            &format!(
                "<span class=\"label\">{name}</span><div class=\"knob\" data-knob-index=\"{index}\" data-rackforge-parameter-index=\"{index}\" style=\"--turn:{:.2}deg\">{}<span class=\"knob-cap\" aria-hidden=\"true\"><span class=\"knob-pointer\"></span></span><input class=\"knob-input\" type=\"range\" data-action=\"parameter\" data-index=\"{index}\" min=\"0\" max=\"{}\" step=\"1\" value=\"{:.0}\" aria-label=\"{name}\" aria-valuetext=\"{readout}\"></div><output data-output-index=\"{index}\">{readout}</output>",
                dial::angle(spec, value),
                knob_scale(),
                dial::INPUT_SPAN,
                dial::position(spec, value) * dial::INPUT_SPAN,
            ),
        )
    }

    fn render_toggle(&self, index: usize) -> String {
        let spec = &SPECS[index];
        let value = self.values[index];
        let name = escape_html(spec.name);
        self.frame(
            index,
            "toggle",
            &format!(
                "<span class=\"label\">{name}</span><button type=\"button\" class=\"rocker\" data-action=\"toggle\" data-index=\"{index}\" data-rackforge-parameter-index=\"{index}\" aria-pressed=\"{}\" aria-label=\"{name}\"><span class=\"lamp\" aria-hidden=\"true\"></span></button><output data-output-index=\"{index}\">{}</output>",
                value == 1.0,
                escape_html(&readout(index, value))
            ),
        )
    }

    fn render_choice(&self, index: usize) -> String {
        let spec = &SPECS[index];
        let value = self.values[index];
        let name = escape_html(spec.name);
        let mut buttons = String::new();
        for (choice, label) in spec.choices {
            buttons.push_str(&format!(
                "<button type=\"button\" class=\"choice\" data-action=\"choose\" data-index=\"{index}\" data-value=\"{choice}\" role=\"radio\" aria-checked=\"{}\">{}</button>",
                f64::from(*choice) == value,
                escape_html(label)
            ));
        }
        self.frame(
            index,
            "choice",
            &format!(
                "<span class=\"label\">{name}</span><div class=\"choices\" role=\"radiogroup\" aria-label=\"{name}\" data-rackforge-parameter-index=\"{index}\">{buttons}</div>"
            ),
        )
    }

    fn render_registers(
        &self,
        index: usize,
        symbol: fn(usize) -> Option<String>,
        side: &str,
    ) -> String {
        let spec = &SPECS[index];
        let value = self.values[index];
        let mut tabs = String::new();
        for (choice, label) in spec.choices {
            let label = escape_html(&label.replace('\'', "′"));
            tabs.push_str(&format!(
                "<button type=\"button\" class=\"register-tab\" data-action=\"choose\" data-index=\"{index}\" data-value=\"{choice}\" role=\"radio\" aria-checked=\"{}\" aria-label=\"{label}\" title=\"{label}\">{}<span class=\"tab-name\">{label}</span></button>",
                f64::from(*choice) == value,
                symbol(*choice as usize).unwrap_or_default(),
            ));
        }
        self.frame(
            index,
            &format!("{side}-register register"),
            &format!(
                "<div class=\"register-row\" role=\"radiogroup\" aria-label=\"{}\" data-rackforge-parameter-index=\"{index}\">{tabs}</div><output data-output-index=\"{index}\">{}</output>",
                escape_html(spec.name),
                escape_html(&readout(index, value).replace('\'', "′"))
            ),
        )
    }

    /// The air button. RackForge opens its MIDI-link menu on a touch held
    /// 560 ms, and the air button is made to be held: the menu is offered
    /// on its label instead.
    fn render_air(&self, index: usize) -> String {
        let value = self.values[index];
        self.frame(
            index,
            "air",
            &format!(
                "<span class=\"label\" data-rackforge-parameter-index=\"{index}\">Air</span><button type=\"button\" class=\"air-button\" data-action=\"air\" data-index=\"{index}\" aria-pressed=\"{}\" aria-label=\"Air button: hold to let air through\"><span class=\"air-cap\" aria-hidden=\"true\"></span></button><output data-output-index=\"{index}\">{}</output>",
                value > 0.0,
                escape_html(&readout(index, value))
            ),
        )
    }
}

/// The ticks round every knob.
fn knob_scale() -> String {
    let mut ticks = String::new();
    for tick in 0..=10 {
        let angle = -dial::SWEEP + 2.0 * dial::SWEEP * f64::from(tick) / 10.0;
        ticks.push_str(&format!(
            "<line x1=\"50\" y1=\"3\" x2=\"50\" y2=\"{}\" transform=\"rotate({angle} 50 50)\"></line>",
            if tick % 5 == 0 { 10 } else { 8 }
        ));
    }
    format!("<svg class=\"knob-scale\" viewBox=\"0 0 100 100\" aria-hidden=\"true\">{ticks}</svg>")
}

/// What the panel prints for a parameter: the air valve as how far it is
/// open, the rest as [`dial::readout`].
fn readout(index: usize, value: f64) -> String {
    if index == AIR_VALVE {
        return if value > 0.0 {
            format!("{:.0} %", value * 100.0)
        } else {
            "Closed".to_owned()
        };
    }
    dial::readout(&SPECS[index], value)
}

fn request(
    app: &AppHandle,
    method: &str,
    params: serde_json::Value,
    handler: impl FnOnce(&AppHandle, Result<JsValue, String>) + 'static,
) {
    let (id, window, origin) = {
        let mut state = app.borrow_mut();
        state.sequence += 1;
        let id = format!("rf-musette-ui-{}", state.sequence);
        state.pending.insert(id.clone(), Box::new(handler));
        (id, state.window.clone(), state.host_origin.clone())
    };
    let message = Request {
        protocol: PROTOCOL,
        kind: "request",
        request_id: &id,
        method,
        params,
    };
    let serializer = serde_wasm_bindgen::Serializer::json_compatible();
    let message = match message.serialize(&serializer) {
        Ok(message) => message,
        Err(error) => {
            resolve(app, &id, Err(error.to_string()));
            return;
        }
    };
    match window.parent().ok().flatten() {
        Some(parent) => {
            if let Err(error) = parent.post_message(&message, &origin) {
                resolve(app, &id, Err(format!("postMessage failed: {error:?}")));
                return;
            }
        }
        None => {
            resolve(app, &id, Err("RackForge's window is missing.".to_owned()));
            return;
        }
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

fn resolve(app: &AppHandle, id: &str, result: Result<JsValue, String>) {
    let handler = app.borrow_mut().pending.remove(id);
    if let Some(handler) = handler {
        handler(app, result);
    }
}

fn refresh_parameters(app: &AppHandle) {
    let generation = {
        let mut state = app.borrow_mut();
        state.generation += 1;
        state.generation
    };
    request(
        app,
        "plugin.parameters",
        serde_json::json!({}),
        move |app, result| {
            if app.borrow().generation != generation {
                return;
            }
            match result.and_then(|value| {
                serde_wasm_bindgen::from_value::<Snapshot>(value).map_err(|error| error.to_string())
            }) {
                Ok(snapshot) => {
                    let mut state = app.borrow_mut();
                    for value in snapshot.values {
                        let index = value.index as usize;
                        if index < COUNT && value.value.is_finite() {
                            state.values[index] = value.value;
                        }
                    }
                    state.loaded = true;
                    state.error.clear();
                }
                Err(error) => app.borrow_mut().error = error,
            }
            if app.borrow().held.is_some() {
                app.borrow_mut().stale = true;
            } else {
                app.borrow().render();
            }
        },
    );
}

fn send_parameter(app: &AppHandle, index: usize, value: f64) {
    app.borrow_mut().values[index] = value;
    request(
        app,
        "plugin.set_parameter",
        serde_json::json!({ "parameter_index": index, "value": value }),
        move |app, result| {
            if let Err(error) = result {
                app.borrow_mut().error = error;
                refresh_parameters(app);
            }
        },
    );
}

/// Sends a value the player chose and shows it, unless it is the one
/// already there.
fn set(app: &AppHandle, index: usize, value: f64) {
    if app.borrow().values[index] != value {
        send_parameter(app, index, value);
    }
    update_parameter_dom(app, index);
}

fn query(document: &Document, selector: &str) -> Vec<Element> {
    let Ok(nodes) = document.query_selector_all(selector) else {
        return Vec::new();
    };
    (0..nodes.length())
        .filter_map(|place| nodes.item(place)?.dyn_into::<Element>().ok())
        .collect()
}

fn set_property(element: &Element, name: &str, value: &str) {
    let _ = Reflect::set(
        element.as_ref(),
        &JsValue::from_str(name),
        &JsValue::from_str(value),
    );
}

fn update_parameter_dom(app: &AppHandle, index: usize) {
    let (document, value) = {
        let state = app.borrow();
        (state.document.clone(), state.values[index])
    };
    let spec = &SPECS[index];
    let text = readout(index, value).replace('\'', "′");
    for output in query(&document, &format!("[data-output-index='{index}']")) {
        output.set_text_content(Some(&text));
    }
    for knob in query(&document, &format!("[data-knob-index='{index}']")) {
        let _ = knob.set_attribute(
            "style",
            &format!("--turn:{:.2}deg", dial::angle(spec, value)),
        );
    }
    for input in query(
        &document,
        &format!("[data-action=parameter][data-index='{index}']"),
    ) {
        set_property(
            &input,
            "value",
            &format!("{:.0}", dial::position(spec, value) * dial::INPUT_SPAN),
        );
        let _ = input.set_attribute("aria-valuetext", &text);
    }
    for button in query(
        &document,
        &format!("[data-action=toggle][data-index='{index}']"),
    ) {
        let _ = button.set_attribute("aria-pressed", if value == 1.0 { "true" } else { "false" });
    }
    for button in query(
        &document,
        &format!("[data-action=air][data-index='{index}']"),
    ) {
        let _ = button.set_attribute("aria-pressed", if value > 0.0 { "true" } else { "false" });
    }
    for button in query(
        &document,
        &format!("[data-action=choose][data-index='{index}']"),
    ) {
        let chosen = button
            .get_attribute("data-value")
            .and_then(|choice| choice.parse::<f64>().ok())
            == Some(value);
        let _ = button.set_attribute("aria-checked", if chosen { "true" } else { "false" });
    }
    // A choice that shows another layout's settings redraws the page.
    if panel::reveals(index) {
        app.borrow().render();
        return;
    }
    for waiting in panel::waiting_on(index) {
        let idle = app.borrow().idle(waiting);
        for control in query(&document, &format!("[data-control-index='{waiting}']")) {
            let _ = control.class_list().toggle_with_force("idle", idle);
        }
    }
}

/// SAVE opens RackForge's program save dialog, once the host has given the
/// page its plugin kit.
fn open_program_save(app: &AppHandle) {
    let document = app.borrow().document.clone();
    let Some(dialog) = document.get_element_by_id("program-save") else {
        return;
    };
    if let Ok(open) = Reflect::get(&dialog, &JsValue::from_str("open"))
        && let Some(open) = open.dyn_ref::<Function>()
    {
        let _ = open.call0(&dialog);
    }
}

fn index_attribute(element: &Element) -> Option<usize> {
    element
        .get_attribute("data-index")?
        .parse()
        .ok()
        .filter(|index| *index < COUNT)
}

fn element_from_event(event: &Event) -> Option<Element> {
    event
        .target()?
        .dyn_into::<Element>()
        .ok()?
        .closest("[data-action]")
        .ok()
        .flatten()
}

fn numeric_value(element: &Element) -> Option<f64> {
    Reflect::get(element.as_ref(), &JsValue::from_str("value"))
        .ok()
        .and_then(|value| value.as_string())
        .and_then(|value| value.parse().ok())
        .filter(|value: &f64| value.is_finite())
}

/// The air button pressed: the valve opens all the way.
fn press_air(app: &AppHandle) {
    app.borrow_mut().held = Some(AIR_VALVE);
    set(app, AIR_VALVE, 1.0);
}

/// The air button let go: the valve shuts.
fn release_air(app: &AppHandle) {
    app.borrow_mut().held = None;
    set(app, AIR_VALVE, 0.0);
}

/// A gesture ends, however it ends.
fn finish(app: &AppHandle, gesture: Gesture) {
    match gesture {
        Gesture::Knob {
            pointer, surface, ..
        } => {
            let _ = surface.release_pointer_capture(pointer);
            let stale = {
                let mut state = app.borrow_mut();
                state.held = None;
                core::mem::take(&mut state.stale)
            };
            if stale {
                app.borrow().render();
            }
        }
        Gesture::Air { pointer, surface } => {
            if let Some(pointer) = pointer {
                let _ = surface.release_pointer_capture(pointer);
            }
            release_air(app);
        }
    }
}

fn install_events(app: &AppHandle) -> Result<(), JsValue> {
    let root = app.borrow().root.clone();
    let window = app.borrow().window.clone();
    let gesture: Rc<RefCell<Option<Gesture>>> = Rc::new(RefCell::new(None));

    let click_app = app.clone();
    let click = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
        if event.button() != 0 {
            return;
        }
        let Some(element) = element_from_event(&event) else {
            return;
        };
        let app = &click_app;
        match element.get_attribute("data-action").as_deref() {
            Some("page") => {
                if let Some(page) = element
                    .get_attribute("data-page")
                    .and_then(|id| panel::PAGES.iter().find(|page| page.id == id))
                {
                    app.borrow_mut().page = page.id;
                    app.borrow().render();
                }
            }
            Some("save") => open_program_save(app),
            Some("retry") => refresh_parameters(app),
            Some("toggle") => {
                if let Some(index) = index_attribute(&element) {
                    let value = if app.borrow().values[index] == 1.0 {
                        0.0
                    } else {
                        1.0
                    };
                    set(app, index, value);
                }
            }
            Some("choose") => {
                if let Some(index) = index_attribute(&element)
                    && let Some(value) = element
                        .get_attribute("data-value")
                        .and_then(|value| value.parse().ok())
                {
                    set(app, index, value);
                }
            }
            _ => {}
        }
    });
    root.add_event_listener_with_callback("click", click.as_ref().unchecked_ref())?;
    click.forget();

    // The native range under a knob, moved by anything but our own keys
    // and drags: a position along the taper.
    let input_app = app.clone();
    let input = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let Some(element) = element_from_event(&event) else {
            return;
        };
        if element.get_attribute("data-action").as_deref() == Some("parameter")
            && let Some(index) = index_attribute(&element)
            && let Some(position) = numeric_value(&element)
        {
            let value = dial::value_at(&SPECS[index], position / dial::INPUT_SPAN);
            set(&input_app, index, value);
        }
    });
    root.add_event_listener_with_callback("input", input.as_ref().unchecked_ref())?;
    root.add_event_listener_with_callback("change", input.as_ref().unchecked_ref())?;
    input.forget();

    let key_app = app.clone();
    let key_gesture = gesture.clone();
    let keydown = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
        let Some(element) = element_from_event(&event) else {
            return;
        };
        let Some(index) = index_attribute(&element) else {
            return;
        };
        match element.get_attribute("data-action").as_deref() {
            Some("parameter") => {
                let current = key_app.borrow().values[index];
                if let Some(value) = dial::nudged(&SPECS[index], current, &event.key()) {
                    event.prevent_default();
                    set(&key_app, index, value);
                }
            }
            Some("air") if matches!(event.key().as_str(), " " | "Enter") => {
                event.prevent_default();
                if !event.repeat() && key_gesture.borrow().is_none() {
                    *key_gesture.borrow_mut() = Some(Gesture::Air {
                        pointer: None,
                        surface: element,
                    });
                    press_air(&key_app);
                }
            }
            _ => {}
        }
    });
    root.add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref())?;
    keydown.forget();

    let keyup_app = app.clone();
    let keyup_gesture = gesture.clone();
    let keyup = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
        if !matches!(event.key().as_str(), " " | "Enter") {
            return;
        }
        let held = keyup_gesture
            .borrow_mut()
            .take_if(|gesture| matches!(gesture, Gesture::Air { pointer: None, .. }));
        if let Some(held) = held {
            event.prevent_default();
            finish(&keyup_app, held);
        }
    });
    root.add_event_listener_with_callback("keyup", keyup.as_ref().unchecked_ref())?;
    keyup.forget();

    for name in [
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
        "lostpointercapture",
    ] {
        let pointer_app = app.clone();
        let active = gesture.clone();
        let handler = Closure::<dyn FnMut(PointerEvent)>::new(move |event: PointerEvent| {
            let app = &pointer_app;
            let pointer = event.pointer_id();
            match name {
                "pointerdown" => {
                    if !event.is_primary() || event.button() != 0 || active.borrow().is_some() {
                        return;
                    }
                    let Some(target) = event.target().and_then(|t| t.dyn_into::<Element>().ok())
                    else {
                        return;
                    };
                    if let Ok(Some(knob)) = target.closest("[data-knob-index]")
                        && let Some(index) = knob
                            .get_attribute("data-knob-index")
                            .and_then(|index| index.parse::<usize>().ok())
                            .filter(|index| *index < COUNT)
                    {
                        event.prevent_default();
                        let _ = knob.set_pointer_capture(pointer);
                        if let Ok(Some(input)) = knob.query_selector(".knob-input")
                            && let Ok(input) = input.dyn_into::<HtmlElement>()
                        {
                            let _ = input.focus();
                        }
                        app.borrow_mut().held = Some(index);
                        *active.borrow_mut() = Some(Gesture::Knob {
                            pointer,
                            index,
                            surface: knob,
                            start_y: f64::from(event.client_y()),
                            start: app.borrow().values[index],
                        });
                    } else if let Ok(Some(button)) = target.closest("[data-action=air]") {
                        event.prevent_default();
                        let _ = button.set_pointer_capture(pointer);
                        *active.borrow_mut() = Some(Gesture::Air {
                            pointer: Some(pointer),
                            surface: button,
                        });
                        press_air(app);
                    }
                }
                "pointermove" => {
                    let drag = match &*active.borrow() {
                        Some(Gesture::Knob {
                            pointer: id,
                            index,
                            start_y,
                            start,
                            ..
                        }) if *id == pointer => Some((*index, *start_y, *start)),
                        _ => None,
                    };
                    if let Some((index, start_y, start)) = drag {
                        event.prevent_default();
                        let rise = start_y - f64::from(event.client_y());
                        set(app, index, dial::dragged(&SPECS[index], start, rise));
                    }
                }
                _ => {
                    let ended = active.borrow_mut().take_if(|gesture| match gesture {
                        Gesture::Knob { pointer: id, .. } => *id == pointer,
                        Gesture::Air { pointer: id, .. } => *id == Some(pointer),
                    });
                    if let Some(ended) = ended {
                        finish(app, ended);
                    }
                }
            }
        });
        root.add_event_listener_with_callback(name, handler.as_ref().unchecked_ref())?;
        handler.forget();
    }

    // The page losing the player's attention lets go of whatever is held:
    // the air valve must never stay open behind their back.
    for name in ["blur", "pagehide"] {
        let lost_app = app.clone();
        let lost = gesture.clone();
        let handler = Closure::<dyn FnMut(Event)>::new(move |_event: Event| {
            let ended = lost.borrow_mut().take();
            if let Some(ended) = ended {
                finish(&lost_app, ended);
            }
        });
        window.add_event_listener_with_callback(name, handler.as_ref().unchecked_ref())?;
        handler.forget();
    }

    // A secondary press on a control belongs to RackForge's MIDI-link menu,
    // not to the control (RF-5's guard).
    let secondary = Rc::new(RefCell::new(None::<Element>));
    for name in [
        "pointerdown",
        "pointerup",
        "pointercancel",
        "lostpointercapture",
    ] {
        let pressed = secondary.clone();
        let guard =
            Closure::<dyn FnMut(PointerEvent)>::new(move |event: PointerEvent| match name {
                "pointerdown" if event.pointer_type() == "mouse" && event.button() != 0 => {
                    if let Some(element) = element_from_event(&event) {
                        if let Some(previous) = pressed.borrow_mut().replace(element.clone()) {
                            let _ = previous.class_list().remove_1("rackforge-context-press");
                        }
                        let _ = element.class_list().add_1("rackforge-context-press");
                        event.prevent_default();
                        event.stop_immediate_propagation();
                    }
                }
                "pointerup" | "pointercancel" | "lostpointercapture" => {
                    if let Some(element) = pressed.borrow_mut().take() {
                        let _ = element.class_list().remove_1("rackforge-context-press");
                        event.prevent_default();
                        event.stop_immediate_propagation();
                    }
                }
                _ => {}
            });
        root.add_event_listener_with_callback_and_bool(name, guard.as_ref().unchecked_ref(), true)?;
        guard.forget();
    }

    let message_app = app.clone();
    let message = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
        on_message(&message_app, &event);
    });
    window.add_event_listener_with_callback("message", message.as_ref().unchecked_ref())?;
    message.forget();
    Ok(())
}

fn on_message(app: &AppHandle, event: &MessageEvent) {
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
            let changed =
                app.borrow().sound.as_deref() != Some(&context.instance.selected_sound_id);
            if changed || !app.borrow().loaded {
                app.borrow_mut().sound = Some(context.instance.selected_sound_id);
                refresh_parameters(app);
            }
        }
        Some("parameter_changed") => {
            let index = field("parameter_index")
                .and_then(|value| value.as_f64())
                .filter(|value| value.fract() == 0.0 && *value >= 0.0 && *value < COUNT as f64)
                .map(|value| value as usize);
            let value = field("value")
                .and_then(|value| value.as_f64())
                .filter(|value| value.is_finite());
            if let (Some(index), Some(value)) = (index, value)
                && app.borrow().held != Some(index)
            {
                app.borrow_mut().values[index] = value;
                update_parameter_dom(app, index);
            }
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

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("missing window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("missing document"))?;
    // One light for the whole panel: every shadow and gradient reads it.
    if let Some(root) = document.document_element() {
        let existing = root.get_attribute("style").unwrap_or_default();
        root.set_attribute(
            "style",
            &format!("{existing}{}", crate::light::css_variables()),
        )?;
    }
    // config.html marks its root; play.html is the panel.
    let root = document
        .get_element_by_id("plugin-root")
        .ok_or_else(|| JsValue::from_str("missing #plugin-root"))?;
    if root.get_attribute("data-surface").as_deref() == Some("config") {
        return crate::config::start(window, document, root);
    }
    let app = App::new()?;
    app.borrow().render();
    install_events(&app)?;
    let ready = Ready {
        protocol: PROTOCOL,
        kind: "ready",
    }
    .serialize(&serde_wasm_bindgen::Serializer::json_compatible())?;
    let (parent, origin) = {
        let state = app.borrow();
        (
            state
                .window
                .parent()?
                .ok_or_else(|| JsValue::from_str("missing parent"))?,
            state.host_origin.clone(),
        )
    };
    parent.post_message(&ready, &origin)?;
    Ok(())
}
