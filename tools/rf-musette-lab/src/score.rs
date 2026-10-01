//! Scores: what a render plays, one event per line.
//!
//! ```text
//! # comments and blank lines are ignored
//! 0     2000  69  100     # onset_ms duration_ms note velocity
//! 500   bellows 90        # onset_ms bellows 0..127 (Expression, CC 11)
//! 1500  direction push    # onset_ms direction pull|push (CC 80)
//! ```
//!
//! The same shape as the Concert Grand laboratory's scores, with the
//! bellows in place of the pedals.

use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    NoteOn {
        note: u8,
        velocity: u8,
    },
    NoteOff {
        note: u8,
    },
    Bellows {
        value: u8,
    },
    /// The bellows turns: true pushing, false pulling.
    Direction {
        push: bool,
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
            [_, duration, note, velocity] => {
                let duration: f64 = duration.parse().map_err(|_| fail("bad duration"))?;
                let note: u8 = note.parse().map_err(|_| fail("bad note"))?;
                let velocity: u8 = velocity.parse().map_err(|_| fail("bad velocity"))?;
                if !duration.is_finite() || duration <= 0.0 {
                    return Err(fail("duration must be positive").into());
                }
                if note > 127 || !(1..=127).contains(&velocity) {
                    return Err(fail("note is 0..127 and velocity 1..127").into());
                }
                events.push(Event {
                    at_ms,
                    action: Action::NoteOn { note, velocity },
                });
                events.push(Event {
                    at_ms: at_ms + duration,
                    action: Action::NoteOff { note },
                });
            }
            _ => {
                return Err(fail(
                    "expected `onset duration note velocity`, `onset bellows value` or `onset direction pull|push`",
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
        Action::Bellows { .. } | Action::Direction { .. } => 1,
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
                        velocity: 100
                    }
                },
                Event {
                    at_ms: 250.0,
                    action: Action::Bellows { value: 90 }
                },
                Event {
                    at_ms: 500.0,
                    action: Action::NoteOff { note: 69 }
                },
            ]
        );
    }

    #[test]
    fn a_repeated_note_is_let_go_before_it_is_struck_again() {
        let events = parse("0 500 69 100\n500 500 69 100\n").unwrap();
        assert_eq!(events[1].action, Action::NoteOff { note: 69 });
        assert_eq!(
            events[2].action,
            Action::NoteOn {
                note: 69,
                velocity: 100
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
}
