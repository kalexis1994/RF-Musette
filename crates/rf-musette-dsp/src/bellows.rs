//! What the player asks of the bellows.
//!
//! A real accordion has one bellows for every reed, and the key only opens a
//! pallet: how loud, how bright and how far the pitch sags all come from the
//! pressure the player's arm puts in it (see `docs/RESEARCH.md`). A MIDI
//! keyboard has no bellows, so the player's intent arrives through a
//! controller, and only through one:
//!
//! * the modulation wheel (CC 1, CC 33; decided 2026-10-01, milestone 8f): an
//!   accordion has no vibrato control, and a keyboard player's free hand on
//!   the wheel is the arm on the bellows;
//! * Expression (CC 11, CC 43 its low seven bits) -- an expression pedal, or
//!   a digital accordion's bellows, which is what Roland's FR-series sends.
//!
//! Of the two, the last moved leads. Until either speaks the bellows rests at
//! [`RESTING_PUSH`]. Key velocity never moves it: an accordion's keys have
//! none (the player, 2026-10-02 -- until then velocity set the push while no
//! controller had spoken, and a soft note took the air from the whole
//! instrument whenever the wheel had not been moved since the plugin was
//! built). (Milestone 8i made the wheel, optionally, where the bellows is; it
//! was withdrawn, docs/ROADMAP.md 8i.)
//!
//! The intent is a fraction of the instrument's range, not a pressure. Which
//! pressure in pascals a given intent means belongs to the model, and is
//! stated in `docs/MODEL.md` when the reed exists to be driven by it.

/// The push the bellows rests at until a controller speaks: 300 Pa with the
/// default ceiling (1 kPa) and curve (2), the IfM Zwota's playing pressure,
/// at which the reeds are tuned.
pub const RESTING_PUSH: f32 = 0.547_722_6;

/// Where the bellows intent is coming from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BellowsSource {
    /// No bellows controller has spoken: the bellows rests at
    /// [`RESTING_PUSH`].
    Resting,
    /// The wheel or Expression (CC 1 or 11, with CC 33 or 43 as the low
    /// bits) owns the bellows.
    Expression,
}

/// The player's bellows intent, from 0 (no push) to 1 (the instrument's
/// hardest).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bellows {
    source: BellowsSource,
    intent: f32,
    msb: u8,
    lsb: u8,
}

impl Default for Bellows {
    fn default() -> Self {
        Self::new()
    }
}

impl Bellows {
    /// Resting at [`RESTING_PUSH`], no controller heard yet.
    pub const fn new() -> Self {
        Self {
            source: BellowsSource::Resting,
            intent: RESTING_PUSH,
            msb: 0,
            lsb: 0,
        }
    }

    pub fn source(&self) -> BellowsSource {
        self.source
    }

    pub fn intent(&self) -> f32 {
        self.intent
    }

    /// Expression's high seven bits (CC 11). As the MIDI 1.0 specification
    /// asks of a 14-bit controller, a new high half clears the low one.
    pub fn expression_msb(&mut self, value: u8) {
        self.source = BellowsSource::Expression;
        self.msb = value.min(127);
        self.lsb = 0;
        self.intent = self.fourteen_bit();
    }

    /// Expression's low seven bits (CC 43). A low half with no high half
    /// before it has nothing to refine and is ignored.
    pub fn expression_lsb(&mut self, value: u8) {
        if self.source != BellowsSource::Expression {
            return;
        }
        self.lsb = value.min(127);
        self.intent = self.fourteen_bit();
    }

    /// Expression at MIDI 2.0 width, already a fraction of its range.
    pub fn expression_wide(&mut self, value: f32) {
        if !value.is_finite() {
            return;
        }
        self.source = BellowsSource::Expression;
        self.intent = value.clamp(0.0, 1.0);
        let fourteen = (self.intent * 16383.0 + 0.5) as u16;
        self.msb = (fourteen >> 7) as u8;
        self.lsb = (fourteen & 0x7f) as u8;
    }

    fn fourteen_bit(&self) -> f32 {
        f32::from(u16::from(self.msb) << 7 | u16::from(self.lsb)) / 16383.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bellows_rests_at_300_pa_until_a_controller_speaks() {
        let bellows = Bellows::new();
        assert_eq!(bellows.source(), BellowsSource::Resting);
        // The default ceiling and curve: 1 kPa times the intent squared.
        let pressure = 1000.0 * f64::from(bellows.intent()).powi(2);
        assert!((pressure - 300.0).abs() < 0.01, "{pressure} Pa");
    }

    #[test]
    fn a_controller_takes_the_bellows_over() {
        let mut bellows = Bellows::new();
        bellows.expression_msb(0);
        assert_eq!(bellows.source(), BellowsSource::Expression);
        assert_eq!(bellows.intent(), 0.0);
    }

    #[test]
    fn the_two_halves_make_fourteen_bits() {
        let mut bellows = Bellows::new();
        bellows.expression_msb(127);
        assert_eq!(bellows.intent(), 16256.0 / 16383.0);
        bellows.expression_lsb(127);
        assert_eq!(bellows.intent(), 1.0);
        bellows.expression_msb(64);
        assert_eq!(
            bellows.intent(),
            8192.0 / 16383.0,
            "a new high half clears the low one"
        );
    }

    #[test]
    fn a_low_half_alone_is_ignored() {
        let mut bellows = Bellows::new();
        bellows.expression_lsb(100);
        assert_eq!(bellows.source(), BellowsSource::Resting);
        assert_eq!(bellows.intent(), RESTING_PUSH);
    }

    #[test]
    fn wide_expression_is_kept_at_its_width_and_bounded() {
        let mut bellows = Bellows::new();
        bellows.expression_wide(0.123_456);
        assert_eq!(bellows.intent(), 0.123_456);
        bellows.expression_wide(2.0);
        assert_eq!(bellows.intent(), 1.0);
        bellows.expression_wide(f32::NAN);
        assert_eq!(bellows.intent(), 1.0, "a non-number changes nothing");
        bellows.expression_lsb(0);
        assert_eq!(
            bellows.intent(),
            16256.0 / 16383.0,
            "the halves follow a wide value"
        );
    }
}
