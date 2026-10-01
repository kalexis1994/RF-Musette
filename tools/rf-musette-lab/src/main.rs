//! Offline laboratory. All rendering, WAV writing and reporting runs in Rust.
//! No audio device is opened and no existing file is overwritten.

mod audition;
mod package;
mod schema;
mod score;
mod tune;
mod wav;

use rf_musette_dsp::{Engine, PARAMETER_SPECS, REED_KEY, SAMPLE_RATES, parameters};
use score::Action;
use serde_json::json;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

const HELP: &str = "RF-Musette laboratory
Usage:
  rf-musette-lab render --output PATH.wav [options]
  rf-musette-lab inspect PATH.wav
  rf-musette-lab schema
  rf-musette-lab tune               tunes the treble, writes the tuning table
  rf-musette-lab package
  rf-musette-lab audition [--prepare-only]
Render options:
  --score PATH      A score: `onset_ms duration_ms note velocity` per line,
                    `onset_ms bellows 0..127` or `onset_ms direction
                    pull|push`. Without it, one note.
  --set ID=VALUE    Sets a parameter by its id, in its own units; repeat for
                    more. The ids are in package/metadata/parameters.json.
  --note N          MIDI 0..127 for the single note (default 65, F4; the
                    treble has reeds from 53, F3, to 93, A6)
  --velocity N      MIDI 1..127 for the single note (default 100)
  --hold S          Key hold of the single note, seconds (default 2)
  --lead-in S       Silence before the first event, 0..10 (default 1.5)
  --tail S          Time rendered after the last event, 0..30 (default 1)
  --sample-rate HZ  8000..384000 (default 48000)
WAV is mono IEEE float, without normalisation or clipping. Every render also
writes a JSON report beside it. The lead-in is there for listening: a
wireless headset wakes on the first sound and swallows it, so an attack at
the very start of a file is heard as a fade.
";

/// The laboratory's work runs on a thread with a large stack: the engine
/// carries the whole treble by value (~271 KiB) and is moved a few times as it
/// is built, more than a main thread's 1 MiB on Windows allows -- the same
/// reason the WebAssembly component is linked with an 8 MiB stack.
const STACK_BYTES: usize = 64 << 20;

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let worker = std::thread::Builder::new()
        .stack_size(STACK_BYTES)
        .spawn(move || dispatch(&arguments).map_err(|error| error.to_string()))
        .expect("the laboratory's thread starts");
    match worker.join() {
        Ok(Ok(())) => {}
        Ok(Err(error)) => {
            eprintln!("rf-musette-lab: {error}");
            std::process::exit(1);
        }
        Err(_) => std::process::exit(101),
    }
}

fn dispatch(arguments: &[String]) -> Result<(), Box<dyn Error>> {
    let Some((command, rest)) = arguments.split_first() else {
        print!("{HELP}");
        return Ok(());
    };
    match command.as_str() {
        "render" => render(&Options::parse(rest)?),
        "inspect" => inspect(&single_path(rest)?),
        "schema" => schema::write(),
        "tune" => tune::write(),
        "package" => package::build(),
        "audition" => audition::run(rest),
        "help" | "--help" | "-h" => {
            print!("{HELP}");
            Ok(())
        }
        other => Err(format!("unknown command: {other}\n\n{HELP}").into()),
    }
}

fn single_path(arguments: &[String]) -> Result<PathBuf, Box<dyn Error>> {
    match arguments {
        [path] => Ok(PathBuf::from(path)),
        _ => Err("expected exactly one path".into()),
    }
}

struct Options {
    output: PathBuf,
    score: Option<PathBuf>,
    note: u8,
    velocity: u8,
    hold: f64,
    lead_in: f64,
    tail: f64,
    sample_rate: f32,
    /// Parameter index and value, in the order given.
    settings: Vec<(usize, f64)>,
}

impl Options {
    fn parse(arguments: &[String]) -> Result<Self, Box<dyn Error>> {
        let mut output = None;
        let mut options = Self {
            output: PathBuf::new(),
            score: None,
            settings: Vec::new(),
            note: REED_KEY,
            velocity: 100,
            hold: 2.0,
            lead_in: 1.5,
            tail: 1.0,
            sample_rate: 48_000.0,
        };
        let mut seen = std::collections::BTreeSet::new();
        let mut index = 0;
        while index < arguments.len() {
            let flag = arguments[index].as_str();
            if flag != "--set" && !seen.insert(flag.to_owned()) {
                return Err(format!("duplicate option: {flag}").into());
            }
            let value = arguments
                .get(index + 1)
                .ok_or_else(|| format!("{flag} needs a value"))?;
            let seconds = |low: f64, high: f64| -> Result<f64, Box<dyn Error>> {
                let seconds: f64 = value.parse()?;
                if !(low..=high).contains(&seconds) {
                    return Err(format!("{flag} is {low}..{high} seconds").into());
                }
                Ok(seconds)
            };
            match flag {
                "--output" => output = Some(PathBuf::from(value)),
                "--score" => options.score = Some(PathBuf::from(value)),
                "--set" => {
                    let (id, number) = value.split_once('=').ok_or("--set takes ID=VALUE")?;
                    let parameter = PARAMETER_SPECS
                        .iter()
                        .position(|spec| spec.id == id)
                        .ok_or_else(|| format!("no parameter is called {id}"))?;
                    options.settings.push((parameter, number.parse()?));
                }
                "--note" => {
                    options.note = value.parse()?;
                    if options.note > 127 {
                        return Err("--note is 0..127".into());
                    }
                }
                "--velocity" => {
                    options.velocity = value.parse()?;
                    if !(1..=127).contains(&options.velocity) {
                        return Err("--velocity is 1..127".into());
                    }
                }
                "--hold" => options.hold = seconds(0.01, 60.0)?,
                "--lead-in" => options.lead_in = seconds(0.0, 10.0)?,
                "--tail" => options.tail = seconds(0.0, 30.0)?,
                "--sample-rate" => {
                    options.sample_rate = value.parse()?;
                    if !SAMPLE_RATES.contains(&options.sample_rate) {
                        return Err("--sample-rate is 8000..384000".into());
                    }
                }
                other => return Err(format!("unknown option: {other}\n\n{HELP}").into()),
            }
            index += 2;
        }
        if options.score.is_some()
            && ["--note", "--velocity", "--hold"]
                .iter()
                .any(|flag| seen.contains(*flag))
        {
            return Err(
                "--note, --velocity and --hold describe the single note; a score has its own"
                    .into(),
            );
        }
        options.output = output.ok_or("render needs --output PATH.wav")?;
        Ok(options)
    }
}

fn render(options: &Options) -> Result<(), Box<dyn Error>> {
    let (events, score_text) = match &options.score {
        Some(path) => {
            let text = fs::read_to_string(path)?;
            (score::parse(&text)?, Some(text))
        }
        None => (
            score::parse(&format!(
                "0 {} {} {}",
                options.hold * 1000.0,
                options.note,
                options.velocity
            ))?,
            None,
        ),
    };
    let rate = options.sample_rate;
    let lead_in = (options.lead_in * f64::from(rate)).round() as usize;
    let frame_of = |at_ms: f64| lead_in + (at_ms * f64::from(rate) / 1000.0).round() as usize;
    let last = events.last().map_or(0, |event| frame_of(event.at_ms));
    let frames = last + (options.tail * f64::from(rate)).round() as usize;

    let mut engine = Engine::new(rate).map_err(|error| format!("{error:?}"))?;
    for (index, value) in &options.settings {
        let spec = &PARAMETER_SPECS[*index];
        if !engine.set_parameter(*index, *value) {
            return Err(format!(
                "{} takes {}..{} {}",
                spec.id, spec.minimum, spec.maximum, spec.unit
            )
            .into());
        }
    }
    let mut samples = vec![0.0f32; frames];
    let mut cursor = 0;
    for event in &events {
        let at = frame_of(event.at_ms).min(frames);
        engine.render(&mut samples[cursor..at]);
        cursor = at;
        match event.action {
            Action::NoteOn { note, velocity } => engine.note_on(note, f32::from(velocity) / 127.0),
            Action::NoteOff { note } => engine.note_off(note),
            Action::Bellows { value } => engine.bellows_mut().expression_msb(value),
            Action::Direction { push } => {
                let way = if push {
                    parameters::PUSH
                } else {
                    parameters::PULL
                };
                engine.set_parameter(parameters::BELLOWS_DIRECTION, way);
            }
            Action::Register { value } => {
                engine.set_parameter(parameters::REGISTER, f64::from(value));
            }
            Action::Air { opening } => {
                engine.set_parameter(parameters::AIR_VALVE, opening);
            }
        }
    }
    engine.render(&mut samples[cursor..]);

    // Both files are new or neither is written: a WAV without its report
    // cannot be told apart from a render made some other way.
    let report_path = options.output.with_extension("json");
    if report_path.exists() {
        return Err(format!("refusing to overwrite {}", report_path.display()).into());
    }
    let report = wav::write(&options.output, &samples, rate as u32)?;
    let summary = json!({
        "schema_version": 1,
        "laboratory_version": env!("CARGO_PKG_VERSION"),
        "wav": options.output,
        "sample_rate": report.sample_rate,
        "frames": report.frames,
        "seconds": report.frames as f64 / f64::from(report.sample_rate),
        "lead_in_seconds": options.lead_in,
        "peak": report.peak,
        "rms": report.rms,
        "events": events.len(),
        "score": score_text,
        // Every parameter as rendered, so a render can be reproduced.
        "parameters": PARAMETER_SPECS
            .iter()
            .enumerate()
            .map(|(index, spec)| (spec.id.to_owned(), json!(engine.parameter(index))))
            .collect::<serde_json::Map<_, _>>(),
    });
    write_report(&options.output, &summary)?;
    println!(
        "{}: {:.2} s, peak {:.4}, rms {:.4}",
        options.output.display(),
        report.frames as f64 / f64::from(report.sample_rate),
        report.peak,
        report.rms
    );
    Ok(())
}

fn inspect(path: &Path) -> Result<(), Box<dyn Error>> {
    let (_, report) = wav::read(path)?;
    println!(
        "{}: {} frames at {} Hz ({:.2} s), peak {:.4}, rms {:.4}",
        path.display(),
        report.frames,
        report.sample_rate,
        report.frames as f64 / f64::from(report.sample_rate),
        report.peak,
        report.rms
    );
    Ok(())
}

fn write_report(wav: &Path, summary: &serde_json::Value) -> Result<(), Box<dyn Error>> {
    let path = wav.with_extension("json");
    let file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|error| format!("{}: {error}", path.display()))?;
    serde_json::to_writer_pretty(file, summary)?;
    Ok(())
}
