//! `rf-musette-lab midi IN.mid OUT.score`: a Standard MIDI File as a score
//! (`score.rs`), so a melody can be rendered, edited and kept as text.
//!
//! Notes keep their MIDI channel (the treble on 1, the bass buttons on 2,
//! the chords on 3, as Roland's V-Accordions send them); the modulation
//! wheel (CC 1) and Expression (CC 11) become `wheel` and `bellows` lines.
//! Tempo changes are followed; SMPTE time divisions are refused.

use std::{error::Error, fs, path::Path};

/// One line of the score to be.
#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub at_ms: f64,
    pub text: String,
}

/// A note or controller as read, in ticks.
enum Raw {
    On { note: u8, velocity: u8, channel: u8 },
    Off { note: u8, channel: u8 },
    Controller { number: u8, value: u8 },
    Tempo { micros_per_quarter: u32 },
}

fn variable(data: &[u8], at: &mut usize) -> Result<u32, Box<dyn Error>> {
    let mut value = 0u32;
    for _ in 0..4 {
        let byte = *data.get(*at).ok_or("truncated length")?;
        *at += 1;
        value = (value << 7) | u32::from(byte & 0x7f);
        if byte & 0x80 == 0 {
            return Ok(value);
        }
    }
    Err("a variable-length number longer than four bytes".into())
}

fn chunk<'a>(data: &'a [u8], at: &mut usize, kind: &[u8; 4]) -> Result<&'a [u8], Box<dyn Error>> {
    let header = data.get(*at..*at + 8).ok_or("truncated chunk")?;
    if &header[..4] != kind {
        return Err(format!("expected a {} chunk", String::from_utf8_lossy(kind)).into());
    }
    let length = u32::from_be_bytes(header[4..8].try_into()?) as usize;
    let body = data
        .get(*at + 8..*at + 8 + length)
        .ok_or("truncated chunk body")?;
    *at += 8 + length;
    Ok(body)
}

/// A track's events, each with its absolute tick.
fn track(body: &[u8]) -> Result<Vec<(u64, Raw)>, Box<dyn Error>> {
    let mut events = Vec::new();
    let (mut at, mut tick, mut running) = (0usize, 0u64, 0u8);
    while at < body.len() {
        tick += u64::from(variable(body, &mut at)?);
        let mut status = *body.get(at).ok_or("truncated event")?;
        if status & 0x80 != 0 {
            at += 1;
        } else {
            status = running;
        }
        match status {
            0xff => {
                let kind = *body.get(at).ok_or("truncated meta event")?;
                at += 1;
                let length = variable(body, &mut at)? as usize;
                let payload = body.get(at..at + length).ok_or("truncated meta event")?;
                at += length;
                if kind == 0x51 && length == 3 {
                    let micros_per_quarter =
                        u32::from_be_bytes([0, payload[0], payload[1], payload[2]]);
                    events.push((tick, Raw::Tempo { micros_per_quarter }));
                }
                if kind == 0x2f {
                    break;
                }
            }
            0xf0 | 0xf7 => {
                let length = variable(body, &mut at)? as usize;
                at += length;
            }
            0x80..=0xef => {
                running = status;
                let channel = status & 0x0f;
                let size = if matches!(status & 0xf0, 0xc0 | 0xd0) {
                    1
                } else {
                    2
                };
                let data = body.get(at..at + size).ok_or("truncated channel event")?;
                at += size;
                match status & 0xf0 {
                    0x90 if data[1] > 0 => events.push((
                        tick,
                        Raw::On {
                            note: data[0],
                            velocity: data[1],
                            channel,
                        },
                    )),
                    0x80 | 0x90 => events.push((
                        tick,
                        Raw::Off {
                            note: data[0],
                            channel,
                        },
                    )),
                    0xb0 => events.push((
                        tick,
                        Raw::Controller {
                            number: data[0],
                            value: data[1],
                        },
                    )),
                    _ => {}
                }
            }
            _ => return Err(format!("unexpected status byte {status:#04x}").into()),
        }
    }
    Ok(events)
}

/// The score's lines for a Standard MIDI File, in time order.
pub fn convert(data: &[u8]) -> Result<Vec<Line>, Box<dyn Error>> {
    let mut at = 0;
    let header = chunk(data, &mut at, b"MThd")?;
    if header.len() < 6 {
        return Err("short MThd".into());
    }
    let tracks = u16::from_be_bytes([header[2], header[3]]);
    let division = u16::from_be_bytes([header[4], header[5]]);
    if division & 0x8000 != 0 {
        return Err("SMPTE time divisions are not read".into());
    }
    let mut events = Vec::new();
    for _ in 0..tracks {
        events.extend(track(chunk(data, &mut at, b"MTrk")?)?);
    }
    // Stable: within a tick, each track's order is kept.
    events.sort_by_key(|(tick, _)| *tick);

    // Ticks to milliseconds through the tempo map (120 bpm until told).
    let (mut micros_per_quarter, mut last_tick, mut last_ms) = (500_000u32, 0u64, 0.0f64);
    let mut open: Vec<(u8, u8, u8, f64)> = Vec::new();
    let mut lines = Vec::new();
    for (tick, raw) in events {
        let ms = last_ms
            + (tick - last_tick) as f64 * f64::from(micros_per_quarter)
                / f64::from(division)
                / 1000.0;
        (last_tick, last_ms) = (tick, ms);
        match raw {
            Raw::Tempo {
                micros_per_quarter: tempo,
            } => micros_per_quarter = tempo,
            Raw::On {
                note,
                velocity,
                channel,
            } => open.push((note, velocity, channel, ms)),
            Raw::Off { note, channel } => {
                if let Some(index) = open
                    .iter()
                    .position(|(n, _, c, _)| *n == note && *c == channel)
                {
                    let (note, velocity, channel, start) = open.remove(index);
                    let duration = (ms - start).max(1.0);
                    lines.push(Line {
                        at_ms: start,
                        text: format!("{start:.1} {duration:.1} {note} {velocity} {}", channel + 1),
                    });
                }
            }
            Raw::Controller { number, value } => {
                let name = match number {
                    1 => "wheel",
                    11 => "bellows",
                    _ => continue,
                };
                lines.push(Line {
                    at_ms: ms,
                    text: format!("{ms:.1} {name} {value}"),
                });
            }
        }
    }
    if let Some((note, _, channel, _)) = open.first() {
        return Err(format!("note {note} on channel {} is never let go", channel + 1).into());
    }
    lines.sort_by(|a, b| a.at_ms.total_cmp(&b.at_ms));
    Ok(lines)
}

pub fn run(arguments: &[String]) -> Result<(), Box<dyn Error>> {
    let [input, output] = arguments else {
        return Err("midi takes IN.mid OUT.score".into());
    };
    let output = Path::new(output);
    if output.exists() {
        return Err(format!("refusing to overwrite {}", output.display()).into());
    }
    let lines = convert(&fs::read(input)?)?;
    let mut text = format!("# From {input}, by `rf-musette-lab midi`.\n");
    for line in &lines {
        text.push_str(&line.text);
        text.push('\n');
    }
    // What the renderer will make of it.
    crate::score::parse(&text)?;
    fs::write(output, text)?;
    println!("Wrote {} ({} lines)", output.display(), lines.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A format-0 file at 480 ticks a quarter: 60 bpm, then a C4 on channel
    /// 1 for a quarter and a wheel move, then a C3 on channel 2 by running
    /// status with a note-on of velocity 0 as its end.
    fn file() -> Vec<u8> {
        let mut track = vec![
            0x00, 0xff, 0x51, 0x03, 0x0f, 0x42, 0x40, // tempo 1 000 000 µs
            0x00, 0x90, 60, 100, // C4 on
            0x00, 0xb0, 1, 64, // wheel 64
            0x83, 0x60, 0x80, 60, 0, // 480 ticks later, C4 off
            0x00, 0x91, 48, 90, // C3 on, channel 2
            0x81, 0x70, 48, 0, // 240 ticks later, off by running status
            0x00, 0xff, 0x2f, 0x00,
        ];
        let mut data = b"MThd".to_vec();
        data.extend([0, 0, 0, 6, 0, 0, 0, 1, 0x01, 0xe0]);
        data.extend(b"MTrk");
        data.extend((track.len() as u32).to_be_bytes());
        data.append(&mut track);
        data
    }

    #[test]
    fn notes_keep_their_channels_and_time_follows_the_tempo() {
        let lines: Vec<String> = convert(&file())
            .unwrap()
            .into_iter()
            .map(|l| l.text)
            .collect();
        assert_eq!(
            lines,
            [
                "0.0 wheel 64",
                "0.0 1000.0 60 100 1",
                "1000.0 500.0 48 90 2"
            ]
        );
    }

    #[test]
    fn the_score_parses() {
        let text: String = convert(&file())
            .unwrap()
            .into_iter()
            .map(|l| l.text + "\n")
            .collect();
        assert_eq!(crate::score::parse(&text).unwrap().len(), 5);
    }
}
