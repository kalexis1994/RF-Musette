//! The bellows' air: one volume every reed draws from, kept up by the
//! player's arm.
//!
//! The intent is the arm's push, the pressure it would make in a still
//! bellows. The arm's force falls as it moves faster -- Hill's force-velocity
//! law, F = F₀ (1 − v/v_max)/(1 + v/(k v_max)), k ≈ 0.25 (A. V. Hill, Proc.
//! R. Soc. B 126, 1938, measured on muscle) -- so the more air the reeds,
//! the leaks and the air button spend, the faster the bellows must move and
//! the less pressure the same push keeps: on average, P̄ = F(Q̄/A)/A.
//!
//! The arm moves the bellows' half, and that mass cannot follow a reed's
//! cycle: at audio frequencies the bellows is its air, a compliance
//! C = V/(ρc²), and the arm answers only the mean (docs/ROADMAP.md, 8d and
//! 8e). Given the arm's speed at every sample, as before, it was a
//! resistance across the air at every frequency and held the lowest reeds
//! still. So the arm delivers the mean air drawn, Q̄, followed over τ_m,
//! and brings the pressure back to Hill's P̄ over τ_h:
//!
//! ```text
//! C P' = Q̄ − Q − Q_vent + C (P̄(Q̄) − P)/τ_h,     τ_m Q̄' = Q + Q_vent − Q̄
//! ```
//!
//! τ_m and τ_h are the time scale the moving half's mass sets with the air,
//! 1/ω₀ = √(M C)/A ≈ 10 ms for 4 kg, taken critically damped: the mass and
//! the air would ring near 16 Hz, which no player reports, so the hand and
//! the folds are taken to damp it. Both assumed. Every constant is assumed and voiced by ear
//! (docs/MODEL.md); no accordion bellows has been measured.

use crate::math;
use crate::reed::AIR_DENSITY;

/// Hill's k, a/F₀ in his notation: the curvature of the force-velocity law.
const HILL_K: f64 = 0.25;

/// How long the arm takes to deliver a change in the air drawn, s.
const MEAN_TIME: f64 = 0.01;

/// How long it takes to bring the pressure to its push, s: the same time
/// scale, as a pushed mass on a spring of air critically damped takes.
const HOLD_TIME: f64 = 0.01;

/// The bellows' constants.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindDesign {
    /// Cross-section the arm pushes, m².
    pub area: f64,
    /// The air's compliance, m⁵/N.
    pub compliance: f64,
    /// The arm's unloaded speed, m/s.
    pub arm_speed: f64,
    /// The leaks and the air button, as one orifice, m².
    pub vent: f64,
}

/// The vena contracta of the leaks and the air button, as of every orifice
/// here (Tarnopolsky et al. 2000).
const VENT_CONTRACTION: f64 = 0.61;

/// The bellows' pressure, Pa, without its sign, and the mean air the arm
/// delivers, m³/s.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Wind {
    pub pressure: f64,
    pub mean: f64,
}

impl Wind {
    /// One step of `h` seconds: the arm pushing as hard as `ask` Pa would
    /// need in a still bellows, the reeds drawing `draw` m³/s. Implicit in
    /// the pressure, with the vent linearised at the start of the step: the
    /// hold and the vent's flow only steady it, at any step. With no push
    /// the hand holds the bellows still.
    pub fn step(&mut self, design: &WindDesign, ask: f64, draw: f64, h: f64) -> f64 {
        let p = self.pressure.max(0.0);
        // The vent: an orifice, Q = α a √(2P/ρ), and dQ/dP = Q/(2P).
        let floor = p.max(1.0e-6);
        let vent = VENT_CONTRACTION * design.vent * math::sqrt(2.0 * floor / AIR_DENSITY);
        let vent_slope = vent / (2.0 * floor);
        let c = design.compliance / h;
        if ask <= 0.0 || design.arm_speed <= 0.0 {
            self.mean = 0.0;
            self.pressure = (((c + vent_slope) * p - draw - vent) / (c + vent_slope)).max(0.0);
            return self.pressure;
        }
        self.mean += (h / MEAN_TIME).min(1.0) * (draw + vent - self.mean);
        // Hill's pressure for the mean speed the arm moves at.
        let u = (self.mean / (design.area * design.arm_speed)).max(-0.9 * HILL_K);
        let held = (ask * (1.0 - u) / (1.0 + u / HILL_K)).max(0.0);
        let hold = design.compliance / HOLD_TIME;
        self.pressure = (((c + vent_slope) * p + self.mean - draw - vent + hold * held)
            / (c + vent_slope + hold))
            .max(0.0);
        self.pressure
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bellows() -> WindDesign {
        WindDesign {
            area: 0.06,
            compliance: 12.0e-3 / (AIR_DENSITY * 343.2 * 343.2),
            arm_speed: 1.0,
            vent: 0.0,
        }
    }

    /// Where it settles: three seconds, many times the arm's hold.
    fn settle(design: &WindDesign, ask: f64, draw: f64) -> f64 {
        let mut wind = Wind::default();
        for _ in 0..3 * 96_000 {
            wind.step(design, ask, draw, 1.0 / 96_000.0);
        }
        wind.pressure
    }

    #[test]
    fn with_nothing_drawn_the_push_is_the_pressure() {
        let p = settle(&bellows(), 300.0, 0.0);
        assert!((p - 300.0).abs() < 1e-6, "{p}");
    }

    #[test]
    fn drawing_air_lowers_it_as_hill_says() {
        let design = bellows();
        let draw = 2.0e-3;
        let p = settle(&design, 300.0, draw);
        // The arm moves at the speed that delivers the air drawn; Hill gives
        // the force it still has there.
        let u = draw / design.area / design.arm_speed;
        let force = 300.0 * (1.0 - u) / (1.0 + u / HILL_K);
        assert!((p - force).abs() < 1e-3, "{p} against {force}");
    }

    #[test]
    fn no_push_and_a_vent_empty_it() {
        let mut design = bellows();
        design.vent = 400.0e-6;
        let mut wind = Wind {
            pressure: 300.0,
            mean: 0.0,
        };
        for _ in 0..96_000 {
            wind.step(&design, 0.0, 0.0, 1.0 / 96_000.0);
        }
        assert!(wind.pressure < 1.0, "{}", wind.pressure);
    }

    #[test]
    fn any_step_stays_finite_and_non_negative() {
        let design = bellows();
        let mut wind = Wind::default();
        for h in [1.0e-6, 1.0e-3, 0.1] {
            for ask in [0.0, 1.0e-3, 300.0, 6000.0] {
                for draw in [0.0, 1.0e-2, 10.0] {
                    let p = wind.step(&design, ask, draw, h);
                    assert!(p.is_finite() && p >= 0.0, "{p} at {h} {ask} {draw}");
                }
            }
        }
    }
}
