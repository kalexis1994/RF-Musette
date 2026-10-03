//! The pallet: the valve a key lifts off the reed's tone hole.
//!
//! The air passes through the curtain between the pallet and the hole's rim
//! -- the rim's perimeter times the lift -- and never through more than the
//! hole itself. `reed::step` treats that opening as an orifice with a
//! Bernoulli jet, in series with the hole's inertance.
//!
//! How far a pallet lifts and how fast it moves are not published for any
//! accordion. The lift, the opening and closing times are parameters,
//! assumed, and bounded by the finger-attack times Llanos-Vázquez et al.
//! measured (Acta Acustica 100, 2014): the reed must reach its note in
//! 50-140 ms from a pallet opening on a pressed bellows. The hole is taken
//! as a rectangle four times as long as it is wide, which sets its rim.

use crate::math;

/// Length over width of the tone hole: assumed.
pub const HOLE_ASPECT: f64 = 4.0;

/// The pad's felt and leather, as a share of the full lift (8k). Within it
/// the pad is not clear of the rim: the felt and the leather it presses
/// give way, and what passes shrinks smoothly to nothing at the seat
/// rather than in proportion to the lift down to a corner. A corner there
/// ended the flow's fall with a corner too, and its rate of change -- what
/// radiates -- with a step: a click at every release. Voiced: no pad's
/// compression is published; technicians put the release's sound in this
/// felt. Taken only as the pad comes down to the seat, the key let go: on
/// the felt both ways it slowed every attack, which 8h voiced with the
/// curtain as it was -- the felt is there both ways, and this is a choice,
/// not the physics.
pub const FELT: f64 = 0.1;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PalletDesign {
    /// The pallet's lift when the key is fully down, m.
    pub lift: f64,
    /// Seconds from closed to fully open.
    pub opening_time: f64,
    /// Seconds from fully open to closed.
    pub closing_time: f64,
    /// The time constant, s, with which the pad slows on its felt as it
    /// comes down to the seat (milestone 8m).
    pub seating_time: f64,
}

/// Where the pad is taken as seated, as a share of [`FELT`]: its curtain then
/// 2·10⁻⁶ of the felt's, and the slit's viscosity has long shut the flow.
const SEATED: f64 = 1.0e-3;

/// A pallet's position: how far it is lifted, as a fraction of its full
/// lift, and where the key is taking it.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Pallet {
    pub position: f64,
    pub target: f64,
}

impl Pallet {
    /// The key goes to `depth`, from 0 (up) to 1 (fully down). A key
    /// partly down holds its pallet partly open.
    pub fn press(&mut self, depth: f64) {
        self.target = if depth.is_finite() {
            depth.clamp(0.0, 1.0)
        } else {
            0.0
        };
    }

    /// Moves the pallet by one step of `h` seconds, at constant speed --
    /// except as the pad comes down onto its felt, the key let go: there the
    /// felt and the leather give way under it and it slows, approaching the
    /// seat as e^(-t/τ) with the seating time τ (milestone 8m). That last
    /// tenth is where the tone is shut off; at constant speed it took a
    /// millisecond, and the note's highs fell 65 dB in 1.5 ms where a real
    /// accordion's fade over tens of milliseconds.
    #[inline]
    pub fn advance(&mut self, design: &PalletDesign, h: f64) {
        if self.position < self.target {
            let step = h / design.opening_time.max(1e-6);
            self.position = (self.position + step).min(self.target);
        } else if self.position > self.target {
            if self.target <= 0.0 && self.position <= FELT && design.seating_time > 0.0 {
                self.position *= math::exp(-h / design.seating_time);
                if self.position < FELT * SEATED {
                    self.position = 0.0;
                }
            } else {
                let step = h / design.closing_time.max(1e-6);
                self.position = (self.position - step).max(self.target);
            }
        }
    }

    pub fn is_closed(&self) -> bool {
        self.position <= 0.0 && self.target <= 0.0
    }

    /// The curtain's area, m², for a tone hole of `hole_area` m².
    #[inline]
    pub fn area(&self, design: &PalletDesign, hole_area: f64) -> f64 {
        self.area_by_rim(design, hole_area, rim(hole_area))
    }

    /// The same, for a hole whose [`rim`] is already known: the engine keeps
    /// each reed's with its model rather than take a square root at every
    /// step (milestone 10).
    #[inline]
    pub fn area_by_rim(&self, design: &PalletDesign, hole_area: f64, rim: f64) -> f64 {
        if self.position <= 0.0 {
            return 0.0;
        }
        // On the felt the curtain goes as 2p²/FELT - p³/FELT²: nothing and
        // no slope at the seat, and the lift's own curtain, slope and all,
        // where the felt ends.
        let lift = if self.position < FELT && self.target <= 0.0 {
            let x = self.position / FELT;
            self.position * x * (2.0 - x)
        } else {
            self.position
        };
        (rim * design.lift * lift).min(hole_area)
    }
}

/// The rim of a rectangular hole of `area` m², `HOLE_ASPECT` times as long as
/// wide.
pub fn rim(area: f64) -> f64 {
    let width = math::sqrt(area / HOLE_ASPECT);
    2.0 * (width + HOLE_ASPECT * width)
}

/// What Key Touch's floor stands above a key's measured edge: a chord
/// draws the arm's bellows down a little, and the edge rises with it
/// (milestone 9h again). Assumed.
pub const TOUCH_MARGIN: f64 = 1.2;

/// Key Touch's floor for the treble key at `index` (from F3) at `pressure`
/// Pa with the ranks `open` sounding, as its curtain's share of the hole:
/// the open ranks' highest edge -- the shallowest at which each holds a
/// steady tone, [`crate::touch::EDGE`], read between the pressures it was
/// measured at and at the nearest outside them -- raised by
/// [`TOUCH_MARGIN`], never past the hole (milestone 9h again). First one
/// floor for every key, 0.35, which the A3's and the G♯6's edges set.
pub fn touch_floor(index: usize, pressure: f64, open: [bool; crate::parameters::RANKS]) -> f64 {
    use crate::touch::{EDGE, PRESSURES};
    let index = index.min(EDGE[0][0].len() - 1);
    // L; M−, M and M+ as M; H.
    let kind = |rank: usize| match rank {
        0 => 0,
        4 => 2,
        _ => 1,
    };
    let edge = |row: usize| {
        (0..open.len())
            .filter(|rank| open[*rank])
            .map(|rank| f64::from(EDGE[row][kind(rank)][index]))
            .fold(0.0, f64::max)
    };
    let last = PRESSURES.len() - 1;
    let measured = if pressure <= PRESSURES[0] {
        edge(0)
    } else if pressure >= PRESSURES[last] {
        edge(last)
    } else {
        let row = (0..last)
            .find(|&row| pressure <= PRESSURES[row + 1])
            .unwrap_or(last - 1);
        let share = (pressure - PRESSURES[row]) / (PRESSURES[row + 1] - PRESSURES[row]);
        edge(row) + (edge(row + 1) - edge(row)) * share
    };
    (measured * TOUCH_MARGIN).min(1.0)
}

/// How far a key goes down, 0-1, for a velocity, 0-1, with Key Touch on: at
/// full velocity fully down, as with it off, and below it a curtain c of
/// the hole spread evenly over 1/c -- over which a note's level falls about
/// linearly (9g: −1.4 to −1.9 dB a unit of 1/depth in the F4 at 300 Pa) -- so
/// the velocity's steps are about even in decibels, down to `floor`, the
/// key's [`touch_floor`]. `hole_area`, m², with `design` sets the depth where
/// the curtain is the whole hole.
pub fn touch_depth(velocity: f32, design: &PalletDesign, hole_area: f64, floor: f64) -> f64 {
    let v = f64::from(velocity).clamp(0.0, 1.0);
    if v >= 1.0 {
        return 1.0;
    }
    let knee = (hole_area / (rim(hole_area) * design.lift)).min(1.0);
    let floor = floor.clamp(0.01, 1.0);
    let inverse = 1.0 + (1.0 / floor - 1.0) * (1.0 - v);
    knee / inverse
}

#[cfg(test)]
mod tests {
    use super::*;

    const DESIGN: PalletDesign = PalletDesign {
        lift: 3.0e-3,
        opening_time: 0.01,
        closing_time: 0.02,
        seating_time: 0.001,
    };

    #[test]
    fn the_pallet_travels_at_its_own_speed_and_stops_where_the_key_is() {
        let mut pallet = Pallet::default();
        assert!(pallet.is_closed());
        pallet.press(1.0);
        for _ in 0..500 {
            pallet.advance(&DESIGN, 1.0e-5);
        }
        assert!((pallet.position - 0.5).abs() < 1e-9, "half open after 5 ms");
        for _ in 0..1000 {
            pallet.advance(&DESIGN, 1.0e-5);
        }
        assert_eq!(pallet.position, 1.0);
        pallet.press(0.3);
        for _ in 0..3000 {
            pallet.advance(&DESIGN, 1.0e-5);
        }
        assert!((pallet.position - 0.3).abs() < 1e-12, "held part-way");
        pallet.press(0.0);
        // 4 ms down to the felt, then ln(1000) seating times to the seat.
        for _ in 0..1200 {
            pallet.advance(&DESIGN, 1.0e-5);
        }
        assert!(pallet.is_closed());
    }

    #[test]
    fn the_pad_slows_on_its_felt_and_seats() {
        let mut pallet = Pallet {
            position: FELT,
            target: 0.0,
        };
        // One seating time on the felt: e⁻¹ of the way left.
        for _ in 0..100 {
            pallet.advance(&DESIGN, 1.0e-5);
        }
        assert!(
            (pallet.position / FELT - (-1.0f64).exp()).abs() < 1e-9,
            "{}",
            pallet.position / FELT
        );
        // Opening, it does not slow: the felt gives way only coming down.
        pallet.press(1.0);
        let before = pallet.position;
        pallet.advance(&DESIGN, 1.0e-5);
        assert!((pallet.position - before - 1.0e-3).abs() < 1e-12);
        pallet.press(0.0);
        for _ in 0..2000 {
            pallet.advance(&DESIGN, 1.0e-5);
        }
        assert!(pallet.is_closed(), "seated, not approaching for ever");
    }

    #[test]
    fn the_curtain_never_passes_more_than_the_hole() {
        let hole = 150.0e-6;
        let mut pallet = Pallet {
            position: 0.1,
            target: 0.1,
        };
        let small = pallet.area(&DESIGN, hole);
        assert!((small - rim(hole) * 0.3e-3).abs() < 1e-15);
        pallet.position = 1.0;
        assert_eq!(pallet.area(&DESIGN, hole), hole);
        pallet.position = 0.0;
        assert_eq!(pallet.area(&DESIGN, hole), 0.0);
        // A 150 mm² hole at 1 : 4 is 6.1 by 24.5 mm.
        assert!((rim(hole) - 61.2e-3).abs() < 0.1e-3, "{}", rim(hole));
    }
}
