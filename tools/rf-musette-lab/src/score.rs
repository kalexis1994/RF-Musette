//! Scores: what a render plays, one event per line.
//!
//! ```text
//! # comments and blank lines are ignored
//! 0     2000  69  100     # onset_ms duration_ms note velocity
//! 0     2000  48  100  2  # ... and a MIDI channel 1..16: 2 bass, 3 chords
//! 500   bellows 90        # onset_ms bellows 0..127 (Expression, CC 11)
//! 1500  direction push    # onset_ms direction pull|push (CC 80)
//! 1600  wheel 64          # onset_ms wheel 0..127 (the modulation wheel, CC 1)
//! 3000  register musette  # onset_ms register NAME (as the parameter names it)
//! 3000  bass-register 2'  # onset_ms bass-register NAME
//! 4000  air 1             # onset_ms air 0..1 (the air button, how far pressed)
//! ```
//!
//! The same shape as the Concert Grand laboratory's scores, with the
//! bellows in place of the pedals.

use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    /// A note on a MIDI channel, counted from 0: the treble on 0, the bass
    /// buttons on 1, the chords on 2 (Roland's V-Accordions).
    NoteOn {
        note: u8,
        velocity: u8,
        channel: u8,
    },
    NoteOff {
        note: u8,
        channel: u8,
    },
    Bellows {
        value: u8,
    },
    /// The modulation wheel (CC 1): the push or, with Mod Wheel on Bellows,
    /// where the bellows is.
    Wheel {
        value: u8,
    },
    /// The bellows turns: true pushing, false pulling.
    Direction {
        push: bool,
    },
    /// A register switch: the register parameter's value.
    Register {
        value: u32,
    },
    /// A bass register switch: the bass register parameter's value.
    BassRegister {
        value: u32,
    },
    /// The air button, pressed this far (0..=1).
    Air {
        opening: f64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Event {
    pub at_ms: f64,
    pub action: Action,
}

/// Parses a score into events sorted by time. At one instant, a key is let
/// go before a key goes down, so a repeated note re-strikes.
pub fn parse(text: &str) -> Result<Vec<Event>, Box<dyn Error>> {
    let mut events = Vec::new();
    for (number, line) in text.lines().enumerate() {
        let line = line.split('#').next().unwrap_or_default().trim();
        if line.is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split_whitespace().collect();
        let fail = |what: &str| format!("line {}: {what}: {line}", number + 1);
        let at_ms: f64 = fields[0].parse().map_err(|_| fail("bad onset"))?;
        if !at_ms.is_finite() || at_ms < 0.0 {
            return Err(fail("onset must be a time from zero").into());
        }
        match fields.as_slice() {
            [_, "bellows", value] => {
                let value: u8 = value.parse().map_err(|_| fail("bad bellows value"))?;
                if value > 127 {
                    return Err(fail("bellows is 0..127").into());
                }
                events.push(Event {
                    at_ms,
                    action: Action::Bellows { value },
                });
            }
            [_, "wheel", value] => {
                let value: u8 = value.parse().map_err(|_| fail("bad wheel value"))?;
                if value > 127 {
                    return Err(fail("wheel is 0..127").into());
                }
                events.push(Event {
                    at_ms,
                    action: Action::Wheel { value },
                });
            }
            [_, "direction", way] => {
                let push = match *way {
                    "push" => true,
                    "pull" => false,
                    _ => return Err(fail("direction is pull or push").into()),
                };
                events.push(Event {
                    at_ms,
                    action: Action::Direction { push },
                });
            }
            [_, "air", opening] => {
                let opening: f64 = opening.parse().map_err(|_| fail("bad air opening"))?;
                if !(0.0..=1.0).contains(&opening) {
                    return Err(fail("air is 0..1").into());
                }
                events.push(Event {
                    at_ms,
                    action: Action::Air { opening },
                });
            }
            [_, which @ ("register" | "bass-register"), name] => {
                let bass = *which == "bass-register";
                let index = if bass {
                    rf_musette_dsp::parameters::BASS_REGISTER
                } else {
                    rf_musette_dsp::parameters::REGISTER
                };
                let spec = &rf_musette_dsp::PARAMETER_SPECS[index];
                let Some((value, _)) = spec
                    .choices
                    .iter()
                    .find(|(_, choice)| choice.eq_ignore_ascii_case(name))
                else {
                    return Err(fail("no such register").into());
                };
                let value = *value;
                events.push(Event {
                    at_ms,
                    action: if bass {
                        Action::BassRegister { value }
                    } else {
                        Action::Register { value }
                    },
                });
            }
            [_, duration, note, velocity, rest @ ..] if rest.len() <= 1 => {
                let duration: f64 = duration.parse().map_err(|_| fail("bad duration"))?;
                let note: u8 = note.parse().map_err(|_| fail("bad note"))?;
                let velocity: u8 = velocity.parse().map_err(|_| fail("bad velocity"))?;
                if !duration.is_finite() || duration <= 0.0 {
                    return Err(fail("duration must be positive").into());
                }
                if note > 127 || !(1..=127).contains(&velocity) {
                    return Err(fail("note is 0..127 and velocity 1..127").into());
                }
                let channel = match rest.first() {
                    Some(channel) => {
                        let channel: u8 = channel.parse().map_err(|_| fail("bad channel"))?;
                        if !(1..=16).contains(&channel) {
                            return Err(fail("channel is 1..16").into());
                        }
                        channel - 1
                    }
                    None => 0,
                };
                events.push(Event {
                    at_ms,
                    action: Action::NoteOn {
                        note,
                        velocity,
                        channel,
                    },
                });
                events.push(Event {
                    at_ms: at_ms + duration,
                    action: Action::NoteOff { note, channel },
                });
            }
            _ => {
                return Err(fail(
                    "expected `onset duration note velocity [channel]`, `onset bellows value`, `onset wheel value`, `onset direction pull|push`, `onset register NAME`, `onset bass-register NAME` or `onset air 0..1`",
                )
                .into());
            }
        }
    }
    events.sort_by(|a, b| {
        a.at_ms
            .total_cmp(&b.at_ms)
            .then_with(|| rank(&a.action).cmp(&rank(&b.action)))
    });
    Ok(events)
}

fn rank(action: &Action) -> u8 {
    match action {
        Action::NoteOff { .. } => 0,
        Action::Bellows { .. }
        | Action::Wheel { .. }
        | Action::Direction { .. }
        | Action::Register { .. }
        | Action::BassRegister { .. }
        | Action::Air { .. } => 1,
        Action::NoteOn { .. } => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notes_become_pairs_and_the_bellows_stands_alone() {
        let events = parse("# a phrase\n0 500 69 100\n250 bellows 90 # push\n").unwrap();
        assert_eq!(
            events,
            [
                Event {
                    at_ms: 0.0,
                    action: Action::NoteOn {
                        note: 69,
                        velocity: 100,
                        channel: 0
                    }
                },
                Event {
                    at_ms: 250.0,
                    action: Action::Bellows { value: 90 }
                },
                Event {
                    at_ms: 500.0,
                    action: Action::NoteOff {
                        note: 69,
                        channel: 0
                    }
                },
            ]
        );
    }

    #[test]
    fn a_repeated_note_is_let_go_before_it_is_struck_again() {
        let events = parse("0 500 69 100\n500 500 69 100\n").unwrap();
        assert_eq!(
            events[1].action,
            Action::NoteOff {
                note: 69,
                channel: 0
            }
        );
        assert_eq!(
            events[2].action,
            Action::NoteOn {
                note: 69,
                velocity: 100,
                channel: 0
            }
        );
    }

    #[test]
    fn nonsense_is_refused_with_its_line() {
        assert!(parse("0 500 128 100").is_err());
        assert!(parse("0 500 60 0").is_err());
        assert!(parse("-5 500 60 64").is_err());
        assert!(parse("0 bellows 200").is_err());
        assert!(parse("0 direction sideways").is_err());
        let error = parse("0 500 60 64\nnope").unwrap_err().to_string();
        assert!(error.starts_with("line 2"), "{error}");
    }

    #[test]
    fn the_bellows_turns_by_name() {
        let events = parse("1000 direction push\n2000 direction pull").unwrap();
        assert_eq!(events[0].action, Action::Direction { push: true });
        assert_eq!(events[1].action, Action::Direction { push: false });
    }

    #[test]
    fn registers_are_named_as_the_parameter_names_them() {
        let events = parse("0 register Musette\n10 register master").unwrap();
        assert_eq!(events[0].action, Action::Register { value: 8 });
        assert_eq!(events[1].action, Action::Register { value: 6 });
        assert!(parse("0 register kazoo").is_err());
    }

    #[test]
    fn a_channel_picks_the_side() {
        let events = parse("0 500 48 100 2\n0 500 52 100 3\n").unwrap();
        assert_eq!(
            events[0].action,
            Action::NoteOn {
                note: 48,
                velocity: 100,
                channel: 1
            }
        );
        assert_eq!(
            events[1].action,
            Action::NoteOn {
                note: 52,
                velocity: 100,
                channel: 2
            }
        );
        assert!(parse("0 500 48 100 17").is_err());
        assert!(parse("0 500 48 100 2 9").is_err());
        let events = parse("0 bass-register 2'").unwrap();
        assert_eq!(events[0].action, Action::BassRegister { value: 0 });
    }

    #[test]
    fn the_air_button_is_pressed_by_a_fraction() {
        let events = parse("0 air 0.5").unwrap();
        assert_eq!(events[0].action, Action::Air { opening: 0.5 });
        assert!(parse("0 air 2").is_err());
    }
}
