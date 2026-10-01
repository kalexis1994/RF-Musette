//! The wheel as where the bellows is (milestone 8i): from the positions a
//! controller sends, how fast the bellows moves.
//!
//! A wheel sends steps: 128 positions at seven bits, each, at 12 L of
//! travel, 94 mL of air. Taken as positions they would compress the bellows
//! in lumps of a kilopascal, so the speed is measured instead, between
//! messages:
//!
//! * the last two steps over the time they took, when they went the same
//!   way -- an arm moves smoothly, and a wheel's steps fall unevenly -- or
//!   the last alone when it turned; held until the last step's time has
//!   passed again;
//! * then no more than the step over the time since -- a wheel that has not
//!   moved another step since cannot be moving faster;
//! * and still once [`STOP_INTERVALS`] of its intervals pass with no step.
//!
//! A step after a rest has no interval of its own to be measured over: it is
//! guessed, [`START_INTERVAL`], so the bellows answers at once, and the wheel
//! is not taken as still again until [`STILL_AFTER_START`] passes with no
//! step. Measured over the guess, the wheel would rest before every step of
//! a slow hand, and each step would be a burst (docs/ROADMAP.md, 8i).
//!
//! Positions are fractions of the wheel's range, 0 to 1; time is counted in
//! samples by the caller.

/// A step after a rest is taken over at most this long, s, so a bellows
/// starting from rest answers at once rather than one step late: a soft
/// note's pace, 188 mL/s at 12 L of travel. Assumed.
pub const START_INTERVAL: f64 = 0.5;

/// After a step from rest, the wheel is taken as still only once this long
/// passes with no step, s.
pub const STILL_AFTER_START: f64 = 2.0;

/// After this many of its intervals with no step, the wheel is still.
pub const STOP_INTERVALS: f64 = 4.0;

/// A low half arriving within this long, s, of its high half refines that
/// step rather than making one: controllers send the two together, or
/// nearly.
pub const REFINE_WINDOW: f64 = 0.003;

/// A jump bigger than this share of the range after a rest is the wheel
/// being placed -- a controller recalling a position -- and moves no air.
pub const PLACED: f64 = 1.0 / 16.0;

/// The shortest interval a step is measured over, s: two messages in one
/// sample are one position.
const SHORTEST: f64 = 1.0e-4;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Motion {
    /// A position has been heard since the last forgetting.
    known: bool,
    /// The last step's end, and its start.
    position: f64,
    time: u64,
    before: f64,
    before_time: u64,
    /// The last step, signed, over the interval it is taken across, s; none
    /// while the interval is zero.
    step: f64,
    interval: f64,
    /// The step before it, and whether the two are measured together.
    previous_step: f64,
    previous_interval: f64,
    joined: bool,
    /// The wheel is moving: a step was taken and has not yet run out.
    moving: bool,
    /// The last step was the wheel being placed: it stays so, refined.
    placed: bool,
    /// The last step came after a rest, its interval guessed.
    from_rest: bool,
}

impl Motion {
    /// Forgets the wheel: its next position only says where it is.
    pub fn forget(&mut self) {
        *self = Self::default();
    }

    pub fn is_known(&self) -> bool {
        self.known
    }

    /// The wheel is at `position` (0..=1) at sample `now`, of `rate` samples
    /// a second. `refine`: a low half, which may complete the step its high
    /// half began.
    pub fn moved(&mut self, position: f64, now: u64, rate: f64, refine: bool) {
        if !position.is_finite() || rate <= 0.0 {
            return;
        }
        let position = position.clamp(0.0, 1.0);
        if !self.known {
            *self = Self {
                known: true,
                position,
                time: now,
                before: position,
                before_time: now,
                ..Self::default()
            };
            return;
        }
        let since = now.saturating_sub(self.time) as f64 / rate;
        if since < SHORTEST || (refine && since < REFINE_WINDOW) {
            // The same step, its end refined -- if there is a step yet.
            self.position = position;
            if self.interval > 0.0 && !self.placed {
                self.step = position - self.before;
                self.moving = self.step != 0.0;
                self.joined &= self.step * self.previous_step > 0.0;
            }
            return;
        }
        let resting = self.speed(now, rate) == 0.0;
        let step = position - self.position;
        let interval = if resting {
            since.min(START_INTERVAL)
        } else {
            since
        };
        // A step is measured with the one before only when that one's own
        // interval was measured, not guessed.
        let joined = !resting && self.moving && !self.from_rest && step * self.step > 0.0;
        self.previous_step = self.step;
        self.previous_interval = self.interval;
        self.before = self.position;
        self.before_time = self.time;
        self.position = position;
        self.time = now;
        self.placed = resting && step.abs() > PLACED;
        if self.placed {
            // Placed, not moved.
            self.step = 0.0;
            self.interval = 0.0;
            self.joined = false;
            self.moving = false;
            self.from_rest = false;
            return;
        }
        self.step = step;
        self.interval = interval.max(SHORTEST);
        self.joined = joined;
        self.moving = step != 0.0;
        self.from_rest = resting;
    }

    /// How fast the wheel moves at sample `now`, in shares of its range a
    /// second: above zero rising, below falling.
    pub fn speed(&self, now: u64, rate: f64) -> f64 {
        if !self.moving || rate <= 0.0 {
            return 0.0;
        }
        let pace = if self.joined {
            (self.step + self.previous_step) / (self.interval + self.previous_interval)
        } else {
            self.step / self.interval
        };
        let since = now.saturating_sub(self.time) as f64 / rate;
        let still = if self.from_rest {
            STILL_AFTER_START.max(STOP_INTERVALS * self.interval)
        } else {
            STOP_INTERVALS * self.interval
        };
        if since <= self.interval {
            pace
        } else if since < still {
            // No faster than one more step would have come.
            pace.signum() * pace.abs().min((self.step / since).abs())
        } else {
            0.0
        }
    }

    /// Where the wheel is, 0 to 1, once heard.
    pub fn position(&self) -> Option<f64> {
        self.known.then_some(self.position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: f64 = 48_000.0;

    fn at(seconds: f64) -> u64 {
        (seconds * RATE) as u64
    }

    #[test]
    fn the_first_position_moves_nothing() {
        let mut motion = Motion::default();
        motion.moved(0.5, at(1.0), RATE, false);
        assert!(motion.is_known());
        assert_eq!(motion.speed(at(1.0), RATE), 0.0);
        assert_eq!(motion.speed(at(1.1), RATE), 0.0);
    }

    #[test]
    fn steady_steps_give_a_steady_speed() {
        let mut motion = Motion::default();
        let step = 1.0 / 127.0;
        for n in 0..20 {
            motion.moved(
                0.2 + step * f64::from(n),
                at(0.25 * f64::from(n)),
                RATE,
                false,
            );
            // From the third step on, two steps of the same length.
            if n >= 3 {
                for k in 0..25 {
                    let now = at(0.25 * f64::from(n) + 0.01 * f64::from(k));
                    let speed = motion.speed(now, RATE);
                    assert!((speed - step / 0.25).abs() < 1e-9, "{n} {k}: {speed}");
                }
            }
        }
    }

    #[test]
    fn a_wheel_that_stops_slows_then_stills() {
        let mut motion = Motion::default();
        let step = 1.0 / 127.0;
        for n in 0..5 {
            motion.moved(step * f64::from(n), at(0.1 * f64::from(n)), RATE, false);
        }
        let last = 0.4;
        let full = motion.speed(at(last + 0.05), RATE);
        assert!((full - step / 0.1).abs() < 1e-9);
        // Twice its interval since: no faster than a step over that.
        let slower = motion.speed(at(last + 0.2), RATE);
        assert!((slower - step / 0.2).abs() < 1e-9, "{slower}");
        assert_eq!(motion.speed(at(last + 0.41), RATE), 0.0);
    }

    #[test]
    fn a_step_after_a_rest_answers_at_once() {
        let mut motion = Motion::default();
        motion.moved(0.5, at(0.0), RATE, false);
        motion.moved(0.5 + 1.0 / 127.0, at(5.0), RATE, false);
        let speed = motion.speed(at(5.0), RATE);
        assert!(
            (speed - 1.0 / 127.0 / START_INTERVAL).abs() < 1e-9,
            "{speed}"
        );
    }

    #[test]
    fn a_wheel_placed_after_a_rest_moves_no_air() {
        let mut motion = Motion::default();
        motion.moved(0.1, at(0.0), RATE, false);
        motion.moved(0.9, at(3.0), RATE, false);
        assert_eq!(motion.speed(at(3.0), RATE), 0.0);
        // From there it moves as usual.
        motion.moved(0.9 - 1.0 / 127.0, at(3.1), RATE, false);
        assert!(motion.speed(at(3.1), RATE) < 0.0);
    }

    #[test]
    fn a_low_half_refines_its_step() {
        let mut motion = Motion::default();
        motion.moved(0.5, at(0.0), RATE, false);
        motion.moved(0.51, at(0.1), RATE, false);
        // The low half a millisecond later: the same step, finer.
        motion.moved(0.515, at(0.101), RATE, true);
        let speed = motion.speed(at(0.12), RATE);
        assert!((speed - 0.015 / 0.1).abs() < 1e-9, "{speed}");
        // A low half alone, later, is a step of its own -- alone, since the
        // one before came from rest and its interval was a guess.
        motion.moved(0.52, at(0.2), RATE, true);
        let speed = motion.speed(at(0.2), RATE);
        assert!((speed - 0.005 / 0.1).abs() < 1e-9, "{speed}");
        // The next is measured with it.
        motion.moved(0.53, at(0.3), RATE, false);
        let speed = motion.speed(at(0.3), RATE);
        assert!((speed - 0.015 / 0.2).abs() < 1e-9, "{speed}");
    }

    /// A slow hand, one step a second: never taken as resting between its
    /// steps, so never a burst (docs/ROADMAP.md, 8i, the repair).
    #[test]
    fn a_slow_hand_is_not_a_string_of_starts() {
        let mut motion = Motion::default();
        let step = 1.0 / 128.0;
        motion.moved(0.2, at(0.0), RATE, false);
        for n in 1..10 {
            motion.moved(0.2 + step * f64::from(n), at(f64::from(n)), RATE, false);
            if n >= 2 {
                for k in 0..10 {
                    let speed = motion.speed(at(f64::from(n) + 0.099 * f64::from(k)), RATE);
                    assert!((speed - step).abs() < 1e-9, "{n} {k}: {speed}");
                }
            }
        }
    }

    #[test]
    fn a_first_position_refined_moves_nothing() {
        let mut motion = Motion::default();
        motion.moved(20.0 / 127.0, at(0.0), RATE, false);
        motion.moved(20.0 * 128.0 / 16383.0, at(0.0), RATE, true);
        assert_eq!(motion.speed(at(0.0), RATE), 0.0);
        assert_eq!(motion.speed(at(0.1), RATE), 0.0);
    }

    #[test]
    fn uneven_steps_are_measured_together_and_a_turn_alone() {
        let mut motion = Motion::default();
        let step = 1.0 / 127.0;
        motion.moved(0.5, at(0.0), RATE, false);
        motion.moved(0.5 + step, at(0.2), RATE, false);
        motion.moved(0.5 + 2.0 * step, at(0.5), RATE, false);
        motion.moved(0.5 + 3.0 * step, at(0.6), RATE, false);
        // 0.3 s then 0.1 s: two steps over 0.4 s.
        let speed = motion.speed(at(0.6), RATE);
        assert!((speed - 2.0 * step / 0.4).abs() < 1e-9, "{speed}");
        // Turned: the step back alone.
        motion.moved(0.5 + 2.0 * step, at(0.7), RATE, false);
        let speed = motion.speed(at(0.7), RATE);
        assert!((speed + step / 0.1).abs() < 1e-9, "{speed}");
    }

    #[test]
    fn nonsense_changes_nothing() {
        let mut motion = Motion::default();
        motion.moved(f64::NAN, at(0.0), RATE, false);
        assert!(!motion.is_known());
        motion.moved(2.0, at(0.0), RATE, false);
        assert_eq!(motion.position(), Some(1.0));
    }
}
