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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PalletDesign {
    /// The pallet's lift when the key is fully down, m.
    pub lift: f64,
    /// Seconds from closed to fully open.
    pub opening_time: f64,
    /// Seconds from fully open to closed.
    pub closing_time: f64,
}

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

    /// Moves the pallet by one step of `h` seconds, at constant speed.
    #[inline]
    pub fn advance(&mut self, design: &PalletDesign, h: f64) {
        if self.position < self.target {
            let step = h / design.opening_time.max(1e-6);
            self.position = (self.position + step).min(self.target);
        } else if self.position > self.target {
            let step = h / design.closing_time.max(1e-6);
            self.position = (self.position - step).max(self.target);
        }
    }

    pub fn is_closed(&self) -> bool {
        self.position <= 0.0 && self.target <= 0.0
    }

    /// The curtain's area, m², for a tone hole of `hole_area` m².
    #[inline]
    pub fn area(&self, design: &PalletDesign, hole_area: f64) -> f64 {
        if self.position <= 0.0 {
            return 0.0;
        }
        (rim(hole_area) * design.lift * self.position).min(hole_area)
    }
}

/// The rim of a rectangular hole of `area` m², `HOLE_ASPECT` times as long as
/// wide.
pub fn rim(area: f64) -> f64 {
    let width = math::sqrt(area / HOLE_ASPECT);
    2.0 * (width + HOLE_ASPECT * width)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DESIGN: PalletDesign = PalletDesign {
        lift: 3.0e-3,
        opening_time: 0.01,
        closing_time: 0.02,
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
        for _ in 0..1000 {
            pallet.advance(&DESIGN, 1.0e-5);
        }
        assert!(pallet.is_closed());
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
