//! The factory programs (milestone 9e): the accordion as each tradition
//! tunes, registers and records it, and the ways to play it.
//!
//! Each program is the defaults with a few parameters set. Where a value
//! comes from is beside it; the tremolo is the M+ reed's beat against M at
//! A4, 1 Hz ≈ 4 cents there, and follows Hergert's measured lines across the
//! keyboard as every program does. Sources, read 2026-10-01
//! (docs/SOURCES.md, "The programs"):
//! * Roland's V-Accordion musette detunes, unpublished by Roland, measured
//!   in cents on an FR-3s (Big Squeezy Accordions, accordionists.info,
//!   2024) and an FR-3 (B. Young, melodeon.net, 2009), the two agreeing on
//!   French (±23 c) and Scottish (±26-27 c);
//! * makers' and tuners' tables: Victoria (1997 list), Castagnari, Liberty
//!   Bellows, G. Pellegrini (mon-accordeon), A. Weirig, S. Dumpleton;
//! * the registers each style uses, from players and makers as reported
//!   there, and Roland's register maps (FR-3x Owner's Manual p. 27).

use crate::parameters::{self, Parameters};

pub struct Program {
    pub id: &'static str,
    pub name: &'static str,
    pub bank: &'static str,
    pub category: &'static str,
    /// For the player, in the program list.
    pub description: &'static str,
    /// Parameter and value, over the defaults.
    pub settings: &'static [(usize, f64)],
}

/// The banks, in order: id and name.
pub const BANKS: [(&str, &str); 5] = [
    // The id kept from the research package, so sessions that recall its
    // one program still find it.
    ("research", "Factory"),
    ("styles", "Styles"),
    ("setups", "Setups"),
    ("instruments", "Instruments"),
    // The player's own programs, which the plugin lists after the factory's;
    // no program here is in it.
    (USER_BANK, "User"),
];

/// The bank the player's own programs are listed in.
pub const USER_BANK: &str = "user";

use parameters::{
    AUTO_REVERSE, BASS_REGISTER, BELLOWS_AREA, BELLOWS_CEILING, BELLOWS_LEAK, BELLOWS_RESPONSE,
    BELLOWS_VOLUME, CASSOTTO, LEFT_HAND, MIC_LAYOUT, ORTF_DISTANCE, ORTF_HEIGHT, REGISTER,
    ROOM_HARDNESS, ROOM_SIZE, SINGLE_DISTANCE, SPACED_DISTANCE, SPLIT_POINT, TREMOLO,
};

// The registers, in Roland's order.
const BANDONEON: f64 = 1.0;
const CELLO: f64 = 2.0;
const HARMONIUM: f64 = 3.0;
const ACCORDION: f64 = 5.0;
const MASTER: f64 = 6.0;
const MUSETTE: f64 = 8.0;
const OBOE: f64 = 10.0;
const CELESTE: f64 = 12.0;
// The microphones.
const INTERNAL: f64 = 0.0;
const CLIP_ON: f64 = 1.0;
const SPOTS: f64 = 2.0;
const ORTF: f64 = 3.0;
const SPACED: f64 = 4.0;
const SINGLE: f64 = 5.0;

pub const PROGRAMS: &[Program] = &[
    Program {
        id: "research",
        name: "Accordion",
        bank: "research",
        category: "Accordion",
        description: "A full piano accordion, every reed modelled from the physics: Clarinet in the treble, all five bass reeds, an ORTF pair in a small room.",
        settings: &[],
    },
    Program {
        id: "musette-paris",
        name: "Musette Paris",
        bank: "styles",
        category: "Musette",
        description: "The Paris bal-musette: three middle reeds tuned wide, open, in a dance hall.",
        settings: &[
            (REGISTER, MUSETTE),
            // Roland's French, ±23 c measured twice; Liberty Bellows ±5 Hz;
            // Pellegrini 4-6 Hz.
            (TREMOLO, 5.9),
            (CASSOTTO, 0.0),
            (MIC_LAYOUT, ORTF),
            (ROOM_SIZE, 300.0),
            (ROOM_HARDNESS, 0.5),
        ],
    },
    Program {
        id: "scottish",
        name: "Scottish Dance Band",
        bank: "styles",
        category: "Musette",
        description: "The Scottish dance band's very wet musette, on stage with clip-on mics.",
        settings: &[
            (REGISTER, MUSETTE),
            // Roland's Scottish, ±26-27 c measured twice; J. Crawford's
            // +26 c; Liberty Bellows 7 Hz.
            (TREMOLO, 6.7),
            (CASSOTTO, 0.0),
            (MIC_LAYOUT, CLIP_ON),
            (ROOM_SIZE, 800.0),
            (ROOM_HARDNESS, 0.5),
        ],
    },
    Program {
        id: "italian",
        name: "Italian Folk",
        bank: "styles",
        category: "Musette",
        description: "The Italian tremolo: three middle reeds, a moderate shimmer, a spaced pair.",
        settings: &[
            (REGISTER, MUSETTE),
            // Victoria ±16 c, Castagnari +15 c, Roland Italian ±15-17 c.
            (TREMOLO, 4.0),
            (CASSOTTO, 0.0),
            (MIC_LAYOUT, SPACED),
            (ROOM_SIZE, 400.0),
            (ROOM_HARDNESS, 0.5),
        ],
    },
    Program {
        id: "alpine",
        name: "Alpine",
        bank: "styles",
        category: "Folk",
        description: "The German and Alpine sound: bassoon and two middle reeds, a gentle tremolo.",
        settings: &[
            (REGISTER, CELLO),
            // Roland German L ±13 c; Liberty Bellows 3-4 Hz for German and
            // Alpine.
            (TREMOLO, 3.3),
            (CASSOTTO, 0.0),
            (MIC_LAYOUT, SINGLE),
            (SINGLE_DISTANCE, 0.6),
            (ROOM_SIZE, 250.0),
            (ROOM_HARDNESS, 0.4),
        ],
    },
    Program {
        id: "oberkrainer",
        name: "Oberkrainer",
        bank: "styles",
        category: "Folk",
        description: "The Slovenian polka sound: three middle reeds, the true one in the tone chamber.",
        settings: &[
            (REGISTER, MUSETTE),
            // Liberty Bellows 3 Hz for Slovenian; MMM, the straight M in the
            // cassotto (Nadvesnik).
            (TREMOLO, 3.0),
            (CASSOTTO, 1.0),
            (MIC_LAYOUT, SPOTS),
            (ROOM_SIZE, 300.0),
            (ROOM_HARDNESS, 0.5),
        ],
    },
    Program {
        id: "cleveland",
        name: "Cleveland Polka",
        bank: "styles",
        category: "Folk",
        description: "The American Slovenian polka: three middle reeds nearly dry, open, on stage.",
        settings: &[
            (REGISTER, MUSETTE),
            // Liberty Bellows 0.5 Hz; musette tuned dry, no cassotto
            // (Nadvesnik).
            (TREMOLO, 0.5),
            (CASSOTTO, 0.0),
            (MIC_LAYOUT, INTERNAL),
            (ROOM_SIZE, 600.0),
            (ROOM_HARDNESS, 0.5),
        ],
    },
    Program {
        id: "american",
        name: "American",
        bank: "styles",
        category: "Folk",
        description: "The American tuning: two middle reeds with a light beat.",
        settings: &[
            (REGISTER, CELESTE),
            // Victoria ±10 c, Castagnari +10 c, Liberty Bellows 2.5 Hz.
            (TREMOLO, 2.5),
            (CASSOTTO, 0.0),
            (MIC_LAYOUT, SINGLE),
            (SINGLE_DISTANCE, 0.5),
            (ROOM_SIZE, 200.0),
            (ROOM_HARDNESS, 0.4),
        ],
    },
    Program {
        id: "irish",
        name: "Irish Swing",
        bank: "styles",
        category: "Folk",
        description: "The Irish dance tuning: two middle reeds in swing, a slow beat, in a pub.",
        settings: &[
            (REGISTER, CELESTE),
            // S. Dumpleton's swing, 1-1.5 Hz; Ireland moved from musette to
            // swing (players' reports).
            (TREMOLO, 1.2),
            (CASSOTTO, 0.0),
            (MIC_LAYOUT, CLIP_ON),
            (ROOM_SIZE, 150.0),
            (ROOM_HARDNESS, 0.6),
        ],
    },
    Program {
        id: "jazz",
        name: "Jazz",
        bank: "styles",
        category: "Jazz",
        description: "The jazz accordion: bassoon and middle reed in the tone chamber, dry and round.",
        settings: &[
            (REGISTER, BANDONEON),
            // Weirig: jazz about one beat a second; jazz players favour the
            // cassotto (players' reports).
            (TREMOLO, 1.0),
            (CASSOTTO, 1.0),
            (MIC_LAYOUT, SINGLE),
            (SINGLE_DISTANCE, 0.5),
            (ROOM_SIZE, 120.0),
            (ROOM_HARDNESS, 0.3),
        ],
    },
    Program {
        id: "tango",
        name: "Tango",
        bank: "styles",
        category: "Tango",
        description: "Towards the bandoneon: a middle reed and its octave, dry, close-miked.",
        settings: &[
            // A bandoneon is tuned in dry octaves, 8′ and 4′ (OA Bandoneon,
            // bandoneonist.ch): Roland's Oboe, M and H.
            (REGISTER, OBOE),
            (TREMOLO, 0.5),
            (CASSOTTO, 0.0),
            (MIC_LAYOUT, SPOTS),
            (ROOM_SIZE, 100.0),
            (ROOM_HARDNESS, 0.3),
        ],
    },
    Program {
        id: "concert",
        name: "Concert",
        bank: "styles",
        category: "Classical",
        description: "The concert accordion: bassoon, middle reed and piccolo, dry, through the tone chamber, in a hall.",
        settings: &[
            (REGISTER, HARMONIUM),
            // Concert instruments are tuned dry (Weirig "SEC 0 Hz"; Liberty
            // Bellows), LMMH with the cassotto.
            (TREMOLO, 0.0),
            (CASSOTTO, 1.0),
            (MIC_LAYOUT, ORTF),
            (ORTF_DISTANCE, 1.5),
            (ORTF_HEIGHT, 1.4),
            (ROOM_SIZE, 1500.0),
            (ROOM_HARDNESS, 0.6),
        ],
    },
    Program {
        id: "keyboard-61",
        name: "61-Key Keyboard",
        bank: "setups",
        category: "Setup",
        description: "Both hands on one 61-key keyboard: chords in the octave below middle C, bass notes below them.",
        settings: &[(LEFT_HAND, 1.0), (SPLIT_POINT, 60.0)],
    },
    Program {
        id: "auto-bellows",
        name: "Auto Bellows",
        bank: "setups",
        category: "Setup",
        description: "The bellows runs out and turns by itself, between notes where it can.",
        settings: &[(AUTO_REVERSE, 1.0)],
    },
    Program {
        id: "digital-accordion",
        name: "Digital Accordion",
        bank: "setups",
        category: "Setup",
        description: "For a digital accordion: its bellows sensor sets the pressure directly, bass and chords on their own MIDI channels.",
        settings: &[
            (BELLOWS_RESPONSE, parameters::STIFF),
            (LEFT_HAND, 0.0),
            (BASS_REGISTER, 3.0),
        ],
    },
    // The instruments (9f): other accordions by their mechanics. The
    // bellows' cross-section from each class's body, less ~3 cm of fold;
    // its air in proportion to the default's 12 L at 600 cm²; the leak
    // from technicians' drop test; the ceiling the same arm's force over
    // the area, 1 kPa at 600 cm² (docs/ROADMAP.md, 9f; SOURCES.md, "The
    // instruments"). 15 cents of tremolo at A4 is 3.8 Hz.
    Program {
        id: "student-48",
        name: "Student 48-Bass",
        bank: "instruments",
        category: "Instrument",
        description: "A small 26-key, 48-bass student accordion: two middle reeds with a standard tremolo, a small bellows that runs out of air sooner and a little leaky, played in a practice room.",
        settings: &[
            (REGISTER, CELESTE),
            (TREMOLO, 3.8),
            (CASSOTTO, 0.0),
            (BELLOWS_AREA, 430.0),
            (BELLOWS_VOLUME, 8.6),
            (BELLOWS_LEAK, 40.0),
            (BELLOWS_CEILING, 1400.0),
            (MIC_LAYOUT, SINGLE),
            (SINGLE_DISTANCE, 0.5),
            (ROOM_SIZE, 40.0),
            (ROOM_HARDNESS, 0.4),
        ],
    },
    Program {
        id: "student-72",
        name: "Student 72-Bass",
        bank: "instruments",
        category: "Instrument",
        description: "A 34-key, 72-bass student accordion: bassoon and two middle reeds, a standard tremolo, a mid-sized bellows, in a practice room.",
        settings: &[
            (REGISTER, CELLO),
            (TREMOLO, 3.8),
            (CASSOTTO, 0.0),
            (BELLOWS_AREA, 560.0),
            (BELLOWS_VOLUME, 11.2),
            (BELLOWS_LEAK, 30.0),
            (BELLOWS_CEILING, 1070.0),
            (MIC_LAYOUT, SINGLE),
            (SINGLE_DISTANCE, 0.6),
            (ROOM_SIZE, 60.0),
            (ROOM_HARDNESS, 0.4),
        ],
    },
    Program {
        id: "italian-80",
        name: "Italian 80-Bass",
        bank: "instruments",
        category: "Instrument",
        description: "A 37-key, 80-bass Italian accordion, as built in Castelfidardo: bassoon and two middle reeds with an Italian tremolo, no tone chamber, a well-played bellows, in a living room.",
        settings: &[
            (REGISTER, CELLO),
            (TREMOLO, 4.0),
            (CASSOTTO, 0.0),
            (BELLOWS_AREA, 600.0),
            (BELLOWS_VOLUME, 12.0),
            (BELLOWS_LEAK, 30.0),
            (BELLOWS_CEILING, 1000.0),
            (MIC_LAYOUT, SPACED),
            (SPACED_DISTANCE, 1.2),
            (ROOM_SIZE, 80.0),
            (ROOM_HARDNESS, 0.45),
        ],
    },
    Program {
        id: "full-120",
        name: "Full-Size 120-Bass",
        bank: "instruments",
        category: "Instrument",
        description: "A full-size 41-key, 120-bass accordion: four voices, a standard tremolo, a large, tight bellows.",
        settings: &[
            (REGISTER, MASTER),
            (TREMOLO, 3.8),
            (CASSOTTO, 0.0),
            (BELLOWS_AREA, 700.0),
            (BELLOWS_VOLUME, 14.0),
            (BELLOWS_LEAK, 15.0),
            (BELLOWS_CEILING, 860.0),
            (MIC_LAYOUT, ORTF),
            (ROOM_SIZE, 300.0),
            (ROOM_HARDNESS, 0.5),
        ],
    },
    Program {
        id: "cassotto-pro",
        name: "Cassotto Professional",
        bank: "instruments",
        category: "Instrument",
        description: "A professional five-voice accordion with a tone chamber: deeper and heavier, the largest bellows, built tight; bassoon and middle reeds through the cassotto.",
        settings: &[
            (REGISTER, ACCORDION),
            (TREMOLO, 3.8),
            (CASSOTTO, 1.0),
            (BELLOWS_AREA, 880.0),
            (BELLOWS_VOLUME, 17.6),
            (BELLOWS_LEAK, 10.0),
            (BELLOWS_CEILING, 680.0),
            (MIC_LAYOUT, ORTF),
            (ROOM_SIZE, 400.0),
            (ROOM_HARDNESS, 0.5),
        ],
    },
];

/// The program with this id.
pub fn program(id: &str) -> Option<&'static Program> {
    PROGRAMS.iter().find(|program| program.id == id)
}

impl Program {
    /// The defaults with this program's settings.
    pub fn parameters(&self) -> Parameters {
        let mut values = Parameters::default();
        for (index, value) in self.settings {
            values.set(*index, *value);
        }
        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameters::SPECS;

    /// Every program's settings are values its parameters take, its bank is
    /// a bank, and no id, nor name, is used twice.
    #[test]
    fn every_program_is_one_the_instrument_takes() {
        for program in PROGRAMS {
            assert!(
                BANKS.iter().any(|(id, _)| *id == program.bank),
                "{}",
                program.id
            );
            for (index, value) in program.settings {
                let mut probe = Parameters::default();
                assert!(
                    probe.set(*index, *value),
                    "{}: {} = {value}",
                    program.id,
                    SPECS[*index].id
                );
            }
            let parameters = program.parameters();
            for (index, value) in program.settings {
                assert_eq!(parameters.get(*index), Some(*value), "{}", program.id);
            }
        }
        for (place, program) in PROGRAMS.iter().enumerate() {
            for other in &PROGRAMS[place + 1..] {
                assert_ne!(program.id, other.id);
                assert_ne!(program.name, other.name);
            }
        }
        assert_eq!(program("research").unwrap().name, "Accordion");
        assert!(program("nothing").is_none());
    }

    /// A program's name and description speak to the player: no notes of
    /// the work, and no one else's trademarks.
    #[test]
    fn descriptions_are_for_the_player() {
        for program in PROGRAMS {
            for text in [program.name, program.description] {
                for word in [
                    "milestone",
                    "Milestone",
                    "2026",
                    "voiced",
                    "Assumed",
                    "docs/",
                    "Roland",
                    "V-Accordion",
                    "Hohner",
                    "Pigini",
                    "Excelsior",
                ] {
                    assert!(!text.contains(word), "{}: {word}", program.id);
                }
            }
        }
    }
}
