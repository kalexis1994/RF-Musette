//! What each control does, for the player: the panel's tooltips. Where a
//! value comes from -- measured, derived, assumed or voiced -- is the
//! engine's `ParameterSpec::source` and `docs/MODEL.md`, not the panel's.

/// Each parameter's tooltip, by id.
pub const HELP: &[(&str, &str)] = &[
    // Play.
    (
        "register",
        "The treble register: which sets of reeds sound, as on an accordion's register switches. The dots show the reeds: the bassoon below, the three middle reeds across, the piccolo above.",
    ),
    (
        "bass_register",
        "The bass register: which sets of reeds the bass buttons and chords sound, from the deep 16′ to the bright 2′.",
    ),
    (
        "left_hand",
        "Retired: the left hand is always there, under the split point.",
    ),
    (
        "split_point",
        "Where the left hand ends and the treble begins on a single keyboard.",
    ),
    (
        "bass_system",
        "Stradella: the standard bass -- under the split point the octave below it plays chords, everything lower bass notes. Free Bass: as a converter accordion, every key under the split plays its own note, E1 to C#6, in two voices an octave apart.",
    ),
    (
        "mod_wheel",
        "The modulation wheel is how hard the bellows is pushed.",
    ),
    (
        "bellows_direction",
        "Which way the bellows moves. Each note has a reed for pulling and one for pushing, and they sound slightly different. MIDI CC 80 switches it too.",
    ),
    (
        "air_valve",
        "The air button: hold it to let air out of the bellows without sounding, as accordionists do to reset the bellows. It also softens what is playing.",
    ),
    (
        "auto_reverse",
        "Lets the bellows run out of air and turn by itself, as a player turns it, preferably between notes.",
    ),
    (
        "bellows_travel",
        "How much air the bellows holds in one direction: with Auto Reverse, how long it plays before turning.",
    ),
    (
        "bellows_smoothing",
        "Retired: key velocity no longer moves the bellows, so there is nothing to smooth.",
    ),
    ("gain", "The output level."),
    (
        "tremolo",
        "How fast the tremolo reeds beat against the middle reed: low is a dry, pure sound, high is the wet shimmer of musette.",
    ),
    (
        "cassotto",
        "The tone chamber: puts the bassoon and middle reeds inside a chamber, for a rounder, darker sound.",
    ),
    (
        "pitch_a4",
        "The tuning reference: the frequency of A above middle C.",
    ),
    // Microphones.
    (
        "mic_layout",
        "How the accordion is miked. Internal and Clip-on travel with the instrument, as on stage; the others stand on stands in the room, as in a studio. Dry is the instrument alone.",
    ),
    (
        "stereo_width",
        "How widely the microphones are spread across the stereo image.",
    ),
    (
        "perspective",
        "Whose left is left: the player's, or the audience's -- microphones facing the player hear the treble on their left.",
    ),
    ("room_size", "The size of the room the accordion plays in."),
    (
        "room_hardness",
        "How reflective the room's walls are: soft rooms are dry, hard rooms ring longer and brighter.",
    ),
    (
        "room_level",
        "How much of the room is heard. 0 dB is the room as it would really sound at these microphones.",
    ),
    (
        "internal_treble",
        "How many microphone capsules run over the treble reeds inside the instrument. Fewer capsules favour the notes nearest them.",
    ),
    (
        "internal_bass",
        "How many microphone capsules sit inside the bass side.",
    ),
    (
        "internal_balance",
        "The treble side's level against the bass side's.",
    ),
    (
        "internal_highpass",
        "Cuts the lowest frequencies, against handling and bellows noise.",
    ),
    (
        "clip_distance",
        "How far the clip-on microphones sit from the treble grille; the bass one sits a little farther.",
    ),
    (
        "clip_spacing",
        "The distance between the two treble microphones.",
    ),
    (
        "clip_pattern",
        "The microphones' pickup pattern: omni hears all around, cardioid mostly the front, figure-of-eight front and back.",
    ),
    (
        "clip_balance",
        "The treble side's level against the bass side's.",
    ),
    (
        "clip_highpass",
        "Cuts the lowest frequencies, against handling and bellows noise.",
    ),
    (
        "spots_treble_distance",
        "How far the treble microphone stands from the treble grille.",
    ),
    (
        "spots_bass_distance",
        "How far the bass microphone stands from the bass side. The bass side moves with the bellows, so it comes and goes.",
    ),
    (
        "spots_pattern",
        "The microphones' pickup pattern: omni hears all around, cardioid mostly the front, figure-of-eight front and back.",
    ),
    (
        "spots_ambience",
        "Adds a stereo pair farther back, as set in the ORTF pair, for room and blend.",
    ),
    (
        "ortf_distance",
        "How far in front of the instrument the stereo pair stands.",
    ),
    ("ortf_height", "How high the stereo pair stands."),
    (
        "ortf_pattern",
        "The microphones' pickup pattern: omni hears all around, cardioid mostly the front, figure-of-eight front and back.",
    ),
    (
        "spaced_distance",
        "How far in front of the instrument the two microphones stand.",
    ),
    (
        "spaced_spacing",
        "The distance between the two microphones.",
    ),
    (
        "spaced_pattern",
        "The microphones' pickup pattern: omni hears all around, cardioid mostly the front, figure-of-eight front and back.",
    ),
    (
        "single_distance",
        "How far in front of the instrument the microphone stands.",
    ),
    (
        "single_pattern",
        "The microphone's pickup pattern: omni hears all around, cardioid mostly the front, figure-of-eight front and back.",
    ),
    // Reed.
    (
        "reed_frequency",
        "The pitch of the reference reed the instrument is built from; every reed is scaled from it.",
    ),
    (
        "reed_q",
        "How long a reed rings on its own once struck: higher rings longer.",
    ),
    (
        "q_slope",
        "How much more the low reeds ring than the high ones.",
    ),
    (
        "reed_mode_ratio",
        "The shape of the reed tongue, from even to tapered: it moves the reed's overtones.",
    ),
    ("reed_length", "The length of the reference reed's tongue."),
    ("reed_width", "The width of the reference reed's tongue."),
    (
        "reed_set",
        "How far the reed tongue's tip is raised from its plate: it changes how readily and how strongly a reed speaks.",
    ),
    (
        "plate_thickness",
        "The thickness of the reed plate the tongue swings through.",
    ),
    (
        "side_clearance",
        "The gap beside the tongue in its slot: smaller speaks sooner, as accordion makers say.",
    ),
    ("tip_clearance", "The gap at the tongue's tip."),
    (
        "attack_kick",
        "How readily a reed starts when its key opens: higher gives a crisper attack.",
    ),
    // Cell & Pallet.
    (
        "cell_volume",
        "The size of the small air chamber each reed sounds into.",
    ),
    (
        "tone_hole_area",
        "The size of the hole under each key's pad.",
    ),
    (
        "tone_hole_depth",
        "The depth of the hole under each key's pad.",
    ),
    (
        "end_correction",
        "How much air beyond the hole moves with it.",
    ),
    ("pallet_lift", "How far each key's pad lifts off its hole."),
    (
        "pallet_opening",
        "How quickly a key's pad opens: shorter is a quicker attack.",
    ),
    (
        "pallet_closing",
        "How quickly a key's pad closes: shorter is a quicker release.",
    ),
    (
        "pad_seating",
        "How gently the pad settles on its felt at the end of a release: longer fades the note out, shorter cuts it.",
    ),
    (
        "key_touch",
        "On, how hard you strike a key sets how far it goes down: softer notes quieter and a little flat, as a key held part-way. Off, every key goes fully down; the wheel or Expression is the bellows either way.",
    ),
    (
        "cassotto_resonance",
        "The pitch the tone chamber resonates at.",
    ),
    ("cassotto_q", "How sharply the tone chamber resonates."),
    // Air & Bellows.
    (
        "contraction",
        "How much the air jet through a reed narrows as it passes.",
    ),
    (
        "near_field_inertance",
        "The weight of the air moving right at the reed's slot.",
    ),
    (
        "swing_limit",
        "How strongly a reed is held back from swinging too far.",
    ),
    (
        "bellows_response",
        "Arm: the bellows gives a little as more reeds draw air, as a player's arm does. Stiff: the pressure is exactly what is asked, for digital accordions that measure it.",
    ),
    ("bellows_ceiling", "The bellows pressure at full push."),
    (
        "bellows_curve",
        "How the push grows with velocity or expression: higher keeps soft playing softer.",
    ),
    ("bellows_area", "The size of the bellows."),
    ("bellows_volume", "How much air the bellows holds."),
    (
        "arm_speed",
        "How fast the player's arm can move the bellows.",
    ),
    ("bellows_leak", "How much air the bellows leaks."),
    (
        "reversal_time",
        "How long the bellows takes to change direction.",
    ),
    (
        "oversampling",
        "Higher sounds cleaner in the highest notes and uses more processing.",
    ),
];

/// A parameter's tooltip.
pub fn help(id: &str) -> &'static str {
    HELP.iter()
        .find(|(each, _)| *each == id)
        .map_or("", |(_, text)| text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rf_musette_dsp::parameters::SPECS;

    /// Every parameter has its tooltip, once, and no tooltip names a
    /// parameter there is not.
    #[test]
    fn every_parameter_has_a_tooltip() {
        for spec in SPECS.iter() {
            let count = HELP.iter().filter(|(id, _)| *id == spec.id).count();
            assert_eq!(count, 1, "{}", spec.id);
            assert!(!help(spec.id).is_empty());
        }
        assert_eq!(HELP.len(), SPECS.len());
    }

    /// A tooltip speaks to the player: no milestones, dates or sources.
    #[test]
    fn tooltips_are_for_the_player() {
        for (id, text) in HELP {
            for word in [
                "milestone",
                "Milestone",
                "2026",
                "voiced",
                "Assumed",
                "Decided",
                "docs/",
            ] {
                assert!(!text.contains(word), "{id}: {word}");
            }
        }
    }
}
