//! One free reed, blown closed, and the air that drives it.
//!
//! The model is Millot & Baumann's minimal free reed (Acta Acustica 93,
//! 2007, arXiv 2401.01606), with the useful section rebuilt for an
//! accordion plate and a discretisation that cannot gain energy. Every
//! equation below names its source; `docs/MODEL.md` carries the same
//! ledger, with the status of every constant.
//!
//! **Mechanics.** The tongue moves on the first mode of a clamped-free beam
//! (Millot eq. 4; measured sinusoidal on accordion reeds by Misdariis, Ricot &
//! Caussé, CFA 2000, and by Ziegenhals, IfM Zwota 2009):
//!
//! ```text
//! ζ'' + (ω0/Q) ζ' + ω0² ζ = μ Δp,     μ = S_r / M_r
//! ```
//!
//! `ζ` is the tip's displacement from rest, positive into the slot, and `Δp`
//! the pressure across the reed. `S_r = W L ∫ψ` is the reed's effective area
//! and `M_r = m ∫ψ²` its modal mass (Millot appendix, eq. 11-12).
//!
//! **Flow.** The volume flow is what the tongue pumps plus the jet through
//! the useful section, with a quasi-steady Bernoulli jet (Millot eqs. 5-7):
//!
//! ```text
//! u = S_r ζ' + α S_u(ζ) v_j,     Δp = ½ ρ v_j |v_j|
//! ```
//!
//! **Excitation.** A blown-closed free reed needs no resonator tuned to it:
//! the air upstream must be accelerated into the gap, and that inertia makes
//! the pressure on the reed lag its motion (St. Hilaire, Wilson & Beavers,
//! J. Fluid Mech. 49, 1971; Ricot, Caussé & Misdariis, JASA 117, 2005). How
//! much air, decides whether it speaks at all: measured on this model, the
//! near-field air alone leaves the F4 reed silent below ~3 kPa, where real
//! reeds speak from tens of pascals (`docs/MODEL.md`, "The cell is not
//! optional").
//!
//! **The cell.** On the bellows' opening stroke the sounding reed is the one
//! inside its cell in the reed block: air comes in from outside through the
//! tone hole, fills the cell, and leaves through the reed into the bellows.
//! The cell is Tonon's Helmholtz resonator (Papers of the International
//! Concertina Association 2, 2005): the tone hole an inertance
//! `M_h = ρ (t + k d) / A`, the cell a compliance `C = V / ρc²`. Then the
//! near-field inertance `M_n` of the air converging on the gap, then the
//! reed. With the supply `P` across the whole path:
//!
//! ```text
//! M_h u_h' = P - p_c        C p_c' = u_h - u        M_n u' = p_c - Δp
//! ```
//!
//! This is Millot's minimal configuration (his Fig. 8, p. 13) with the
//! volume and the inertance in the order an accordion puts them. When the
//! cell's resonance is well above the reed, the reed feels the hole's air as
//! inertia, which is what lets it speak; near or below it, the cell chokes
//! the reed, as Cottingham (ICA 2019) and Tonon measured. The pallet --
//! the hole partly open -- is milestone 2.
//!
//! **Radiation.** Outside the instrument the reed is heard through the air
//! the tone hole passes: a monopole, `p = ρ/(4π r) u_h'` at r = 1 m
//! (Ziegenhals: the sound is the derivative of the volume flow, and the
//! tongue's own radiation is negligible). Ricot finds a bare reed's free
//! field dipole-dominated because the flow leaving one face enters from the
//! other; in an accordion the other face is inside the bellows.

use crate::math;
use crate::tongue::{SPAN_POINTS, TongueMode};

/// Air density, kg/m³ (20 °C).
pub const AIR_DENSITY: f64 = 1.204;

/// Entries of the useful-section table: few enough that a whole treble of
/// reeds fits the engine, which never allocates.
const SECTION_POINTS: usize = 256;
/// Deflections the section table covers for a 36 mm tongue, as tip
/// displacement from flat, m; a tongue of another length covers them in
/// proportion. Beyond them the section has saturated at the slot's own area.
const DEFLECTION_LOW: f64 = -9.0e-3;
const DEFLECTION_HIGH: f64 = 12.0e-3;
const DEFLECTION_LENGTH: f64 = 36.0e-3;

/// What one reed is made of and how it sits in its plate. All lengths in
/// metres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReedDesign {
    /// First bending-mode frequency, Hz.
    pub frequency: f64,
    /// Quality factor of that mode in still air.
    pub q: f64,
    pub length: f64,
    pub width: f64,
    /// The second bending mode's frequency over the first's: sets the
    /// tongue's profile (see [`crate::tongue`]).
    pub mode_ratio: f64,
    /// Young's modulus of the tongue's metal, Pa.
    pub modulus: f64,
    /// Density of the tongue's metal, kg/m³.
    pub density: f64,
    /// How far the tip is set away from the plate at rest (the
    /// "Aufbiegung"), on the upstream side.
    pub set: f64,
    pub plate_thickness: f64,
    /// Gap between each long edge of the tongue and the slot.
    pub side_clearance: f64,
    /// Gap between the tongue's tip and the end of the slot.
    pub tip_clearance: f64,
    /// Vena contracta coefficient of the jet.
    pub contraction: f64,
    /// The near-field inertance, as a multiple of the derived estimate.
    pub inertance_scale: f64,
    /// The reed's cell in the block, m³.
    pub cell_volume: f64,
    /// The tone hole the cell speaks through, m².
    pub tone_hole_area: f64,
    /// The tone hole's length through the pallet board, m.
    pub tone_hole_depth: f64,
    /// Tonon's end-correction factor k, times the hole's equivalent diameter.
    pub end_correction: f64,
    /// κ of the voiced swing limit (see [`step`]); 0 leaves the model as
    /// derived.
    pub swing_limit: f64,
}

/// Speed of sound, m/s (20 °C).
pub const SPEED_OF_SOUND: f64 = 343.2;

/// Everything the audio step needs, derived from a [`ReedDesign`] at control
/// rate.
#[derive(Clone)]
pub struct ReedModel {
    pub design: ReedDesign,
    pub omega: f64,
    /// μ, m²/kg.
    pub mu: f64,
    /// S_r, m².
    pub effective_area: f64,
    /// M_r, kg.
    pub modal_mass: f64,
    /// The tongue's thickness at the root, m, derived from its frequency and
    /// profile.
    pub root_thickness: f64,
    /// M_n, the near-field inertance, kg/m⁴.
    pub inertance: f64,
    /// M_h, the tone hole's inertance, kg/m⁴.
    pub hole_inertance: f64,
    /// C, the cell's compliance, m³/Pa.
    pub cell_compliance: f64,
    /// The slot's own area, m²: no more can ever pass.
    pub slot_area: f64,
    section: [f32; SECTION_POINTS],
}

impl ReedModel {
    /// Builds the model, solving the tongue's profile from its mode ratio.
    /// Costly (milliseconds): an engine keeps the [`TongueMode`] and passes
    /// it to [`Self::with_mode`] while the ratio does not change.
    pub fn new(design: ReedDesign) -> Self {
        Self::with_mode(design, &TongueMode::with_ratio(design.mode_ratio))
    }

    pub fn with_mode(design: ReedDesign, mode: &TongueMode) -> Self {
        let root_thickness = mode.root_thickness(
            design.frequency,
            design.length,
            design.modulus,
            design.density,
        );
        let modal_mass =
            design.density * design.width * root_thickness * design.length * mode.mass_integral;
        let effective_area = design.width * design.length * mode.shape_integral;
        let slot_area =
            (design.length + design.tip_clearance) * (design.width + 2.0 * design.side_clearance);
        // The fluid accelerated into the slot, as the one-sided end
        // correction of a flanged opening of the slot's area:
        // Δℓ = 8/(3π) √(S/π) (Rayleigh). A rough estimate -- the slot is
        // long and narrow.
        let end_correction =
            8.0 / (3.0 * core::f64::consts::PI) * math::sqrt(slot_area / core::f64::consts::PI);
        let inertance = design.inertance_scale * AIR_DENSITY * end_correction / slot_area;
        // Tonon's Helmholtz cell: the hole's effective length is its depth
        // plus k times its equivalent diameter.
        let diameter = math::sqrt(4.0 * design.tone_hole_area / core::f64::consts::PI);
        let hole_length = design.tone_hole_depth + design.end_correction * diameter;
        let hole_inertance = AIR_DENSITY * hole_length / design.tone_hole_area;
        let cell_compliance = design.cell_volume / (AIR_DENSITY * SPEED_OF_SOUND * SPEED_OF_SOUND);
        let mut model = Self {
            design,
            omega: 2.0 * core::f64::consts::PI * design.frequency,
            mu: effective_area / modal_mass,
            effective_area,
            modal_mass,
            root_thickness,
            inertance,
            hole_inertance,
            cell_compliance,
            slot_area,
            section: [0.0; SECTION_POINTS],
        };
        model.build_section(mode);
        model
    }

    /// The tongue's stiffness at the tip, N/m: what a static push there meets.
    pub fn tip_stiffness(&self) -> f64 {
        self.modal_mass * self.omega * self.omega
    }

    /// The useful section S_u, m², at a tip deflection `y` from flat
    /// (positive into the slot), computed from the geometry.
    ///
    /// Millot & Baumann's section integrates the escape area along both
    /// long edges and adds the front (their eq. 9). Their law keeps growing
    /// while the tongue is inside the plate (their Fig. 9); here, as in
    /// Ziegenhals's phase 2, a tongue inside the slot leaves only the
    /// clearance, because that is the narrowest the air can pass. Three
    /// phases per edge element (Ziegenhals 2009): above the plate, the gap
    /// from the tongue's face to the slot's upstream edge; inside, the
    /// clearance; through and out the far side, the gap from the tongue's
    /// upstream face -- its local thickness behind -- to the slot's
    /// downstream edge. The tip moves on an arc, which draws it back from the
    /// slot's end by δ = y²/(2L) ∫ψ'², and its end face tilts by the tip
    /// slope; both change the front gap, which is of the order of the
    /// clearance itself. The total never exceeds the slot's own area
    /// (Ziegenhals).
    pub fn section_at(&self, y: f64, mode: &TongueMode) -> f64 {
        let d = self.design;
        let gap = |depth: f64, thickness: f64, clearance: f64| -> f64 {
            if depth < 0.0 {
                math::sqrt(depth * depth + clearance * clearance)
            } else if depth - thickness <= d.plate_thickness {
                clearance
            } else {
                let beyond = depth - thickness - d.plate_thickness;
                math::sqrt(beyond * beyond + clearance * clearance)
            }
        };
        let mut sides = 0.0;
        for i in 0..SPAN_POINTS {
            let weight = if i == 0 || i == SPAN_POINTS - 1 {
                0.5
            } else {
                1.0
            };
            let thickness = self.root_thickness * mode.thickness[i];
            sides += weight * gap(y * mode.shape[i], thickness, d.side_clearance);
        }
        let sides = 2.0 * d.length * sides / (SPAN_POINTS - 1) as f64;
        let tip_thickness = self.root_thickness * mode.thickness[SPAN_POINTS - 1];
        let slope = y * mode.tip_slope / d.length;
        let tilt = 0.5 * tip_thickness * (slope / math::sqrt(1.0 + slope * slope)).abs();
        let drawn_back = y * y / (2.0 * d.length) * mode.slope_squared;
        let front_clearance = (d.tip_clearance + drawn_back - tilt).max(0.1 * d.tip_clearance);
        let front = (d.width + 2.0 * d.side_clearance) * gap(y, tip_thickness, front_clearance);
        (sides + front).min(self.slot_area)
    }

    /// The deflections the section table covers for this tongue, m.
    fn deflections(&self) -> (f64, f64) {
        let scale = self.design.length / DEFLECTION_LENGTH;
        (DEFLECTION_LOW * scale, DEFLECTION_HIGH * scale)
    }

    fn build_section(&mut self, mode: &TongueMode) {
        let (low, high) = self.deflections();
        for i in 0..SECTION_POINTS {
            let y = low + (high - low) * i as f64 / (SECTION_POINTS - 1) as f64;
            self.section[i] = self.section_at(y, mode) as f32;
        }
    }

    /// The useful section at displacement `zeta` from rest, from the table.
    #[inline]
    pub fn section(&self, zeta: f64) -> f64 {
        let y = zeta - self.design.set;
        let (low, high) = self.deflections();
        let position = (y - low) * ((SECTION_POINTS - 1) as f64) / (high - low);
        if position <= 0.0 {
            return f64::from(self.section[0]);
        }
        let last = (SECTION_POINTS - 1) as f64;
        if position >= last {
            return f64::from(self.section[SECTION_POINTS - 1]);
        }
        let index = position as usize;
        let fraction = position - index as f64;
        let a = f64::from(self.section[index]);
        let b = f64::from(self.section[index + 1]);
        a + (b - a) * fraction
    }
}

/// The reed's state: tip displacement from rest (m), its velocity (m/s),
/// the volume flow onto the reed (m³/s), the flow through the tone hole
/// (m³/s) and the cell's pressure (Pa).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ReedState {
    pub zeta: f64,
    pub velocity: f64,
    pub flow: f64,
    pub hole_flow: f64,
    pub cell_pressure: f64,
}

impl ReedState {
    /// The equilibrium under a steady supply: the whole supply falls across
    /// the reed, the tongue sits where that pressure holds it, and the same
    /// jet flows through the hole, the cell and the gap.
    pub fn equilibrium(model: &ReedModel, supply: f64) -> Self {
        let zeta = model.mu * supply / (model.omega * model.omega);
        let velocity = math::sqrt(2.0 * supply.max(0.0) / AIR_DENSITY);
        let flow = model.design.contraction * model.section(zeta) * velocity;
        Self {
            zeta,
            velocity: 0.0,
            flow,
            hole_flow: flow,
            cell_pressure: supply,
        }
    }

    /// Stored energy, J: the tongue's kinetic and elastic energy, the
    /// kinetic energy of the air in both inertances and the cell's
    /// compressed air.
    pub fn energy(&self, model: &ReedModel) -> f64 {
        0.5 * model.modal_mass
            * (self.velocity * self.velocity + model.omega * model.omega * self.zeta * self.zeta)
            + 0.5 * model.inertance * self.flow * self.flow
            + 0.5 * model.hole_inertance * self.hole_flow * self.hole_flow
            + 0.5 * model.cell_compliance * self.cell_pressure * self.cell_pressure
    }
}

/// One step of `h` seconds under supply pressure `supply` (Pa, the mean over
/// the step), with the pallet's curtain open by `pallet` m² (zero: closed;
/// infinity: no restriction at all). Returns the rate of change of the flow
/// through the tone hole over the step, m³/s² -- what radiates.
///
/// Implicit midpoint on every linear part, with the jet's Bernoulli loss
/// linearised about the flow at the start of the step: Δp = R ũ with
/// R = ρ |ũ₀| / (2 α² S_u²) ≥ 0, ũ the jet flow, and the pallet's curtain
/// likewise with R_p. Whatever R and R_p are, the step satisfies exactly
///
/// ```text
/// H₁ - H₀ = h [ P u_h,m - (M_r ω0/Q + c) ζ'_m² - R ũ_m² - R_p u_h,m² ]
/// ```
///
/// c ≥ 0 is the voiced swing limit, the one voiced term of the reed:
/// c = κ ρ v w L (ζ/w)², with v = √(2p/ρ) the jet speed the cell's
/// pressure gives, w and L the tongue's width and length, ζ the tip's
/// displacement from rest, all taken at the start of the step like R. It
/// stands in for the nonlinear dissipation St. Hilaire & Vaidya (J. Fluid
/// Mech. 67, 1975) found limits a free reed's swing, which this model does
/// not derive: zero at small swings, in proportion to the flow as the feed
/// is (docs/ROADMAP.md, 2c).
///
/// When a resistance grows much larger than 2M/h the midpoint values stay
/// right but the end values alternate at the internal Nyquist frequency; the
/// oversampling decimator removes that band, and a pallet that has closed
/// zeroes its flow outright.
///
/// so with the supply off the stored energy can only fall: the scheme is
/// passive for every parameter set, with no iteration. The idea -- a
/// Bernoulli port kept dissipative by construction, solved linearly -- is
/// Darabundit & Scavone's for a clarinet reed (Frontiers in Signal
/// Processing, 2025); this form of it is derived here. The four midpoint
/// unknowns reduce by substitution to a 2x2 system whose determinant is
/// always positive.
#[inline]
pub fn step(model: &ReedModel, state: &mut ReedState, supply: f64, pallet: f64, h: f64) -> f64 {
    let d = &model.design;
    let omega2 = model.omega * model.omega;
    let lift = (state.zeta + 0.5 * h * state.velocity) / d.width;
    let speed = math::sqrt(2.0 * state.cell_pressure.max(0.0) / AIR_DENSITY);
    let limit = d.swing_limit * AIR_DENSITY * speed * d.width * d.length * lift * lift;
    let damping = model.omega / d.q + limit / model.modal_mass;
    let s_r = model.effective_area;
    let (m_n, m_h, c) = (model.inertance, model.hole_inertance, model.cell_compliance);
    let jet_flow = state.flow - s_r * state.velocity;
    let section = model.section(state.zeta + 0.5 * h * state.velocity);
    let alpha_section = d.contraction * section;
    let r = AIR_DENSITY * jet_flow.abs() / (2.0 * alpha_section * alpha_section);
    // Midpoint unknowns: tongue velocity w, flow onto the reed u, hole flow
    // a, cell pressure p. Rows: tongue, near field, hole, cell.
    //   (2 + hγ + h²ω²/2 + hμRS_r) w - hμR u            = 2w₀ - hω²ζ₀
    //   -hRS_r w + (2M_n + hR) u - h p                   = 2M_n u₀
    //   (2M_h + hR_p) a + h p                            = 2M_h a₀ + hP
    //   h u - h a + 2C p                                 = 2C p₀
    // R_p is the pallet's curtain, an orifice linearised like the reed's own
    // jet: R_p = ρ |a₀| / (2 α² A_p²) ≥ 0. A closed pallet is a seal: the
    // hole passes nothing, and its row becomes a = 0.
    let (b3, dh) = if pallet > 0.0 {
        let alpha_pallet = d.contraction * pallet;
        let r_p = AIR_DENSITY * state.hole_flow.abs() / (2.0 * alpha_pallet * alpha_pallet);
        (
            2.0 * m_h * state.hole_flow + h * supply,
            2.0 * m_h + h * r_p,
        )
    } else {
        state.hole_flow = 0.0;
        (0.0, f64::INFINITY)
    };
    let b4 = 2.0 * c * state.cell_pressure + h * b3 / dh;
    let dc = 2.0 * c + h * h / dh;
    // p = (b4 - h u) / dc, then the near-field row in w and u alone.
    let a11 = 2.0 + h * damping + 0.5 * h * h * omega2 + h * model.mu * r * s_r;
    let a12 = -h * model.mu * r;
    let a21 = -h * r * s_r;
    let a22 = 2.0 * m_n + h * r + h * h / dc;
    let b1 = 2.0 * state.velocity - h * omega2 * state.zeta;
    let b2 = 2.0 * m_n * state.flow + h * b4 / dc;
    let det = a11 * a22 - a12 * a21;
    let w = (b1 * a22 - a12 * b2) / det;
    let u = (a11 * b2 - a21 * b1) / det;
    let p = (b4 - h * u) / dc;
    let a = (b3 - h * p) / dh;
    let previous_hole_flow = state.hole_flow;
    state.zeta += h * w;
    state.velocity = 2.0 * w - state.velocity;
    state.flow = 2.0 * u - state.flow;
    state.hole_flow = 2.0 * a - state.hole_flow;
    state.cell_pressure = 2.0 * p - state.cell_pressure;
    (state.hole_flow - previous_hole_flow) / h
}

/// The pressure across the reed, Pa, for a state: what the jet needs.
pub fn pressure_across(model: &ReedModel, state: &ReedState) -> f64 {
    let jet = state.flow - model.effective_area * state.velocity;
    let alpha_section = model.design.contraction * model.section(state.zeta);
    let v = jet / alpha_section;
    0.5 * AIR_DENSITY * v * v.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn f4() -> ReedDesign {
        crate::parameters::Parameters::default().reed_design()
    }

    #[test]
    fn the_derived_tongue_is_the_one_measured() {
        let model = ReedModel::new(f4());
        let mode = TongueMode::with_ratio(model.design.mode_ratio);
        assert!((mode.ratio - 4.6).abs() < 1e-3);
        // The derived root thickness puts the first mode back at 355 Hz.
        let d = model.design;
        let omega2 = mode.eigenvalue * d.modulus * model.root_thickness * model.root_thickness
            / (12.0 * d.density * d.length.powi(4));
        let frequency = omega2.sqrt() / (2.0 * core::f64::consts::PI);
        assert!((frequency - d.frequency).abs() < 1e-6, "{frequency}");
        // And the tongue thins toward its tip.
        assert!(
            mode.thickness[SPAN_POINTS - 1] < 0.5,
            "{}",
            mode.thickness[SPAN_POINTS - 1]
        );
    }

    #[test]
    fn the_section_has_three_phases_and_a_ceiling() {
        let model = ReedModel::new(f4());
        let mode = TongueMode::with_ratio(model.design.mode_ratio);
        let d = model.design;
        let inside = model.section_at(0.5 * d.plate_thickness, &mode);
        let at_rest = model.section_at(-d.set, &mode);
        let far_above = model.section_at(-8.0e-3, &mode);
        let through = model.section_at(d.plate_thickness + model.root_thickness + 3.0e-3, &mode);
        assert!(inside < at_rest, "the slot closes as the tongue enters it");
        assert!(at_rest < far_above && through > inside);
        assert!(far_above <= model.slot_area && through <= model.slot_area);
        // Inside, what is left is the clearance around the tongue.
        let clearance_only = 2.0 * d.length * d.side_clearance;
        assert!(
            inside >= clearance_only && inside < 4.0 * clearance_only,
            "{inside}"
        );
    }

    #[test]
    fn with_the_supply_off_the_reed_can_only_lose_energy() {
        let model = ReedModel::new(ReedDesign { q: 95.0, ..f4() });
        let start = ReedState {
            zeta: 2.0e-3,
            velocity: -3.0,
            flow: 4.0e-5,
            hole_flow: -2.0e-5,
            cell_pressure: 150.0,
        };
        let mut state = start;
        let h = 1.0 / 96_000.0;
        let mut energy = state.energy(&model);
        // At Q 95 and 355 Hz the tongue's energy falls with a 42 ms time
        // constant: 0.625 s is fifteen of them.
        for _ in 0..60_000 {
            step(&model, &mut state, 0.0, f64::INFINITY, h);
            let next = state.energy(&model);
            assert!(next <= energy * (1.0 + 1e-12), "{next} > {energy}");
            energy = next;
        }
        assert!(energy < 1e-3 * start.energy(&model), "{energy}");
    }
}
