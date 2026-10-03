//! `rf-musette-lab touch`: measures Key Touch's edges -- the shallowest
//! curtain at which a pallet holds a steady tone, at each of the pressures --
//! and writes `crates/rf-musette-dsp/src/touch.rs`: every treble key, each
//! rank alone (milestone 9h again); every bass and chord button per bass
//! register, and every free-bass note (milestone 9j).

use rf_musette_analysis::{button_edge, free_edge, key_edge};
use rf_musette_dsp::compass::{BASS_KEYS, FIRST_KEY, FREE_FIRST, FREE_NOTES, KEYS};
use rf_musette_dsp::touch::{PRESSURES, RANK_KINDS};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{error::Error, fs};

/// The registers that open one rank of each kind: Bassoon (L), Clarinet
/// (M, standing for M− and M+ too), Piccolo (H). One rank alone: Musette's
/// three 8′ beat, its level wobbling a dB from half second to half second,
/// and the steady test reads the wobble as a dying note (the A3's edge came
/// out 0.53 at 200 Pa in Musette, though it sounds at 0.22).
const ONE_RANK: [f64; RANK_KINDS] = [0.0, 11.0, 13.0];

/// The bass registers, Roland's seven, each measured as it sounds.
const BASS_REGISTERS: usize = 7;

/// One measurement: a pallet, at the pressure `PRESSURES[row]`.
#[derive(Clone, Copy)]
enum Job {
    Treble {
        index: usize,
        kind: usize,
        row: usize,
    },
    Button {
        chord: bool,
        register: usize,
        pitch_class: usize,
        row: usize,
    },
    Free {
        index: usize,
        row: usize,
    },
}

impl Job {
    /// The edge, or 1 -- the pallet fully open -- where it does not sound.
    fn measure(self) -> f64 {
        let edge = match self {
            Job::Treble { index, kind, row } => {
                key_edge(FIRST_KEY + index as u8, ONE_RANK[kind], PRESSURES[row])
            }
            Job::Button {
                chord,
                register,
                pitch_class,
                row,
            } => button_edge(pitch_class, register as f64, chord, PRESSURES[row]),
            Job::Free { index, row } => free_edge(FREE_FIRST + index as u8, PRESSURES[row]),
        };
        edge.unwrap_or(1.0)
    }
}

struct Tables {
    treble: Vec<[[f64; KEYS]; RANK_KINDS]>,
    bass: Vec<[[f64; BASS_KEYS]; BASS_REGISTERS]>,
    chord: Vec<[[f64; BASS_KEYS]; BASS_REGISTERS]>,
    free: Vec<[f64; FREE_NOTES]>,
}

pub fn write() -> Result<(), Box<dyn Error>> {
    let rows = PRESSURES.len();
    let mut jobs = Vec::new();
    for row in 0..rows {
        for index in 0..KEYS {
            for kind in 0..RANK_KINDS {
                jobs.push(Job::Treble { index, kind, row });
            }
        }
        for chord in [false, true] {
            for register in 0..BASS_REGISTERS {
                for pitch_class in 0..BASS_KEYS {
                    jobs.push(Job::Button {
                        chord,
                        register,
                        pitch_class,
                        row,
                    });
                }
            }
        }
        for index in 0..FREE_NOTES {
            jobs.push(Job::Free { index, row });
        }
    }
    let tables = Mutex::new(Tables {
        treble: vec![[[1.0; KEYS]; RANK_KINDS]; rows],
        bass: vec![[[1.0; BASS_KEYS]; BASS_REGISTERS]; rows],
        chord: vec![[[1.0; BASS_KEYS]; BASS_REGISTERS]; rows],
        free: vec![[1.0; FREE_NOTES]; rows],
    });
    // Every thread takes the next job until none is left, each with a stack
    // an engine fits on.
    let next = AtomicUsize::new(0);
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    std::thread::scope(|scope| {
        for _ in 0..threads {
            std::thread::Builder::new()
                .stack_size(64 << 20)
                .spawn_scoped(scope, || {
                    loop {
                        let taken = next.fetch_add(1, Ordering::Relaxed);
                        let Some(job) = jobs.get(taken).copied() else {
                            break;
                        };
                        let edge = job.measure();
                        let mut tables = tables.lock().expect("the tables");
                        match job {
                            Job::Treble { index, kind, row } => {
                                tables.treble[row][kind][index] = edge
                            }
                            Job::Button {
                                chord: false,
                                register,
                                pitch_class,
                                row,
                            } => tables.bass[row][register][pitch_class] = edge,
                            Job::Button {
                                chord: true,
                                register,
                                pitch_class,
                                row,
                            } => tables.chord[row][register][pitch_class] = edge,
                            Job::Free { index, row } => tables.free[row][index] = edge,
                        }
                        if taken.is_multiple_of(50) {
                            eprintln!("measured {taken} of {}", jobs.len());
                        }
                    }
                })
                .expect("a measuring thread");
        }
    });
    let tables = tables.into_inner().expect("the tables");

    let mut text = String::new();
    text.push_str("//! Generated by `rf-musette-lab touch`; do not edit.\n//!\n");
    text.push_str("//! Key Touch's edges: the shallowest pallet curtain at which a key or a\n");
    text.push_str("//! button holds a steady tone, the arm's bellows asked for each of\n");
    text.push_str("//! [`PRESSURES`]; 1 where it does not sound fully down. The treble's\n");
    text.push_str("//! keys (milestone 9h again), as a share of the M rank's hole, with one\n");
    text.push_str("//! rank of each kind alone (L; M, standing for M− and M+; H); the left\n");
    text.push_str("//! hand's (milestone 9j), as a share of the 8′ rank's hole, the bass and\n");
    text.push_str("//! chord buttons with each bass register's ranks sounding, the free bass\n");
    text.push_str("//! with both its voices.\n\n");
    text.push_str("/// The pressures the edges are measured at, Pa.\n");
    text.push_str(&format!(
        "pub const PRESSURES: [f64; {}] = {:?};\n\n",
        PRESSURES.len(),
        PRESSURES
    ));
    text.push_str("/// The kinds of treble rank measured: L, M, H.\n");
    text.push_str(&format!("pub const RANK_KINDS: usize = {RANK_KINDS};\n\n"));
    text.push_str("/// By pressure, kind and key (F3-A6).\n");
    text.push_str(&format!(
        "pub const EDGE: [[[f32; {KEYS}]; {RANK_KINDS}]; {rows}] = {};\n\n",
        nested(&tables.treble, |kinds| nested(kinds, |row| flat(row)))
    ));
    text.push_str("/// By pressure, bass register and pitch class (C-B): the bass buttons.\n");
    text.push_str(&format!(
        "pub const BASS_EDGE: [[[f32; {BASS_KEYS}]; {BASS_REGISTERS}]; {rows}] = {};\n\n",
        nested(&tables.bass, |registers| nested(registers, |row| flat(row)))
    ));
    text.push_str("/// By pressure, bass register and pitch class (C-B): the chord buttons.\n");
    text.push_str(&format!(
        "pub const CHORD_EDGE: [[[f32; {BASS_KEYS}]; {BASS_REGISTERS}]; {rows}] = {};\n\n",
        nested(&tables.chord, |registers| nested(registers, |row| flat(
            row
        )))
    ));
    text.push_str("/// By pressure and free-bass note (E1-C♯6).\n");
    text.push_str(&format!(
        "pub const FREE_EDGE: [[f32; {FREE_NOTES}]; {rows}] = {};\n",
        nested(&tables.free, |row| flat(row))
    ));
    let path = super::package::workspace_root()?.join("crates/rf-musette-dsp/src/touch.rs");
    fs::write(&path, text)?;
    println!("Wrote {}", path.display());
    Ok(())
}

/// A row of edges as a Rust array, to three decimals.
fn flat(values: &[f64]) -> String {
    let values: Vec<String> = values.iter().map(|value| format!("{value:.3}")).collect();
    format!("[{}]", values.join(", "))
}

/// Arrays of arrays, each written by `inner`.
fn nested<T>(items: &[T], inner: impl Fn(&T) -> String) -> String {
    let items: Vec<String> = items.iter().map(inner).collect();
    format!("[{}]", items.join(", "))
}
