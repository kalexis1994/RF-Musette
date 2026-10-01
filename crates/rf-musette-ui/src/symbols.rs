//! The register symbols, as Roland prints them (FR-3x Owner's Manual,
//! pp. 27 and 30), drawn from the engine's own register tables.
//!
//! The treble's circle is cut in three: L in the lower part, M−, M and M+
//! across the middle, H in the upper. The bass side's is cut in four: 2′, 4′,
//! 8′ and 16′ downward, and the 8-4′ at the right of the rim.

use rf_musette_dsp::parameters::{
    BASS_2, BASS_4, BASS_8, BASS_8_4, BASS_16, BASS_RANKS, RANK_FLAT, RANK_HIGH, RANK_LOW,
    RANK_MIDDLE, RANK_SHARP, RANKS, bass_register_ranks, register_ranks,
};

const CENTRE: f64 = 20.0;
const RADIUS: f64 = 16.0;

/// Where each treble rank's dot sits, and its radius.
const TREBLE_DOTS: [(usize, f64, f64); RANKS] = [
    (RANK_LOW, 20.0, 30.2),
    (RANK_FLAT, 12.0, 20.0),
    (RANK_MIDDLE, 20.0, 20.0),
    (RANK_SHARP, 28.0, 20.0),
    (RANK_HIGH, 20.0, 9.8),
];
const TREBLE_DOT: f64 = 3.7;

/// Where each bass-side rank's dot sits.
const BASS_DOTS: [(usize, f64, f64); BASS_RANKS] = [
    (BASS_16, 20.0, 31.5),
    (BASS_8, 20.0, 24.0),
    (BASS_8_4, 32.0, 20.0),
    (BASS_4, 20.0, 16.0),
    (BASS_2, 20.0, 8.5),
];
const BASS_DOT: f64 = 3.3;

/// The horizontal rules at these heights, each as long as the circle is
/// wide there.
fn rules(heights: &[f64]) -> String {
    let mut path = String::new();
    for y in heights {
        let half = (RADIUS * RADIUS - (y - CENTRE).powi(2)).sqrt();
        path.push_str(&format!(
            "M{:.2} {y:.2}H{:.2}",
            CENTRE - half,
            CENTRE + half
        ));
    }
    path
}

fn symbol(heights: &[f64], dots: &[(usize, f64, f64)], open: &[bool], radius: f64) -> String {
    let mut reeds = String::new();
    for (rank, x, y) in dots {
        if open[*rank] {
            reeds.push_str(&format!(
                "<circle class=\"reed\" cx=\"{x}\" cy=\"{y}\" r=\"{radius}\"></circle>"
            ));
        }
    }
    format!(
        "<svg class=\"register-symbol\" viewBox=\"0 0 40 40\" aria-hidden=\"true\"><circle class=\"rim\" cx=\"{CENTRE}\" cy=\"{CENTRE}\" r=\"{RADIUS}\"></circle><path class=\"rule\" d=\"{}\"></path>{reeds}</svg>",
        rules(heights)
    )
}

/// The treble register's symbol, or `None` past the last register.
pub fn treble(register: usize) -> Option<String> {
    let open = register_ranks(register)?;
    let third = 2.0 * RADIUS / 3.0;
    let top = CENTRE - RADIUS;
    Some(symbol(
        &[top + third, top + 2.0 * third],
        &TREBLE_DOTS,
        &open,
        TREBLE_DOT,
    ))
}

/// The bass register's symbol, or `None` past the last register.
pub fn bass(register: usize) -> Option<String> {
    let open = bass_register_ranks(register)?;
    Some(symbol(&[12.0, 20.0, 28.0], &BASS_DOTS, &open, BASS_DOT))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rf_musette_dsp::parameters::{BASS_REGISTER, REGISTER, SPECS};

    fn dots(svg: &str) -> usize {
        svg.matches("class=\"reed\"").count()
    }

    /// Prediction 2: each symbol has as many dots as its register opens
    /// ranks, read from the engine's table, and no two registers look alike.
    #[test]
    fn every_symbol_shows_the_ranks_its_register_opens() {
        let mut seen = Vec::new();
        for register in 0..SPECS[REGISTER].choices.len() {
            let svg = treble(register).unwrap();
            let open = register_ranks(register).unwrap();
            assert_eq!(dots(&svg), open.iter().filter(|open| **open).count());
            assert!(!seen.contains(&svg), "register {register} repeats a symbol");
            seen.push(svg);
        }
        assert!(treble(SPECS[REGISTER].choices.len()).is_none());
        seen.clear();
        for register in 0..SPECS[BASS_REGISTER].choices.len() {
            let svg = bass(register).unwrap();
            let open = bass_register_ranks(register).unwrap();
            assert_eq!(dots(&svg), open.iter().filter(|open| **open).count());
            assert!(!seen.contains(&svg), "bass register {register} repeats");
            seen.push(svg);
        }
        assert!(bass(SPECS[BASS_REGISTER].choices.len()).is_none());
    }

    /// Prediction 2: the dots sit where Roland draws them -- Musette's three
    /// across the middle, Bassoon's one in the lower part, Piccolo's in the
    /// upper; the bass's 8-4′ at the rim.
    #[test]
    fn the_dots_sit_where_roland_draws_them() {
        let musette = treble(8).unwrap();
        for x in ["12", "20", "28"] {
            assert!(
                musette.contains(&format!("cx=\"{x}\" cy=\"20\"")),
                "{musette}"
            );
        }
        assert!(treble(0).unwrap().contains("cy=\"30.2\""));
        assert!(treble(13).unwrap().contains("cy=\"9.8\""));
        assert!(bass(2).unwrap().contains("cx=\"32\" cy=\"20\""));
        assert!(bass(0).unwrap().contains("cy=\"8.5\""));
    }

    #[test]
    fn every_dot_lies_inside_the_rim() {
        for (_, x, y) in TREBLE_DOTS {
            assert!(
                ((x - CENTRE).powi(2) + (y - CENTRE).powi(2)).sqrt() + TREBLE_DOT <= RADIUS + 0.5
            );
        }
        for (_, x, y) in BASS_DOTS {
            assert!(
                ((x - CENTRE).powi(2) + (y - CENTRE).powi(2)).sqrt() + BASS_DOT <= RADIUS + 0.5
            );
        }
    }
}
