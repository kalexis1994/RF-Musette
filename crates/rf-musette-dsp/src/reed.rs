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
    /// A load riveted at the tongue's tip, as makers weight bass reeds
    /// (Llanos-Vázquez, thesis 2015, Table 3.1), over the modal mass of the
    /// same tongue unloaded. The tongue is then thicker for the same
    /// frequency, and stiffer. Taken on the unloaded mode's shape
    /// (Rayleigh): (f/f′)² = 1 + M/M_r, Llanos's Eq. 2.18, whose measured
    /// slopes are a little smaller (p137).
    pub tip_load: f64,
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
    /// M_n, the near-field inertance, kg/m⁴: the air the slot's far side
    /// accelerates, beside that side's radiation resistance ρc/S_slot, Pa·s/m³
    /// (milestone 8m), as the hole's outer end.
    pub inertance: f64,
    pub slot_radiation: f64,
    /// M_h, the tone hole's inertance, kg/m⁴: its depth and the inner end's
    /// correction.
    pub hole_inertance: f64,
    /// The hole's outer end, radiating (milestone 8m): the air its
    /// correction carries, kg/m⁴, beside ρc/A, Pa·s/m³.
    pub radiation_mass: f64,
    pub radiation_resistance: f64,
    /// C, the cell's compliance, m³/Pa.
    pub cell_compliance: f64,
    /// The slot's own area, m²: no more can ever pass.
    pub slot_area: f64,
    /// The tone hole's rim, m, for the pallet's curtain ([`pallet::rim`]).
    pub hole_rim: f64,
    /// 12 μ w R², for the curtain's viscous resistance 12 μ w R²/A³ (8m).
    seat_viscosity: f64,
    /// What every step asks of the design, computed once, by the same
    /// operations it used to repeat (milestone 10): ω², ω/Q, the swing
    /// limit's ρ, and the deflections the section table covers.
    omega2: f64,
    linear_damping: f64,
    swing_density: f64,
    section_low: f64,
    section_span: f64,
    /// And reciprocals the step multiplies by where it divided (milestone
    /// 10c): of the tongue's width, of its modal mass, and the section
    /// table's points per metre of deflection.
    inverse_width: f64,
    inverse_mass: f64,
    section_scale: f64,
    /// The cell as a tube with the reed along it (milestone 8m): the time
    /// its waves take end to end, s; 1 / the cell's volume; what survives
    /// the walls' resistance, Pa·s/m³; and where along it, from the opening,
    /// the slot's four points lie and how much of the slot's air each gives
    /// and takes.
    pub tube_seconds: f64,
    inverse_cell_volume: f64,
    pub wall_resistance: f64,
    slot_points: [f64; SLOT_POINTS],
    slot_weights: [f64; SLOT_POINTS],
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
        let unloaded = mode.root_thickness(
            design.frequency,
            design.length,
            design.modulus,
            design.density,
        );
        // The tongue's own modal mass per unit of root thickness, a: with a
        // tip load M the frequency asks a·c·h³ = ω²(a·h + M), c·h² being the
        // unloaded tongue's ω². Newton from the unloaded thickness, where
        // the cubic is negative and from where it only rises.
        let per_thickness = design.density * design.width * design.length * mode.mass_integral;
        let tip_mass = design.tip_load * per_thickness * unloaded;
        let root_thickness = if tip_mass > 0.0 {
            let omega = 2.0 * core::f64::consts::PI * design.frequency;
            let omega2 = omega * omega;
            let c = omega2 / (unloaded * unloaded);
            let mass = tip_mass / per_thickness;
            let mut h = unloaded;
            for _ in 0..50 {
                let f = c * h * h * h - omega2 * (h + mass);
                let slope = 3.0 * c * h * h - omega2;
                let next = h - f / slope;
                if (next - h).abs() <= 1e-12 * h {
                    h = next;
                    break;
                }
                h = next;
            }
            h
        } else {
            unloaded
        };
        let modal_mass = per_thickness * root_thickness + tip_mass;
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
        // That end correction is the far side's radiation mass, so beside it,
        // as at the hole's outer end, its resistance ρc/S (8m).
        let slot_radiation = AIR_DENSITY * SPEED_OF_SOUND / slot_area;
        // Tonon's Helmholtz cell: the hole's effective length is its depth
        // plus k times its equivalent diameter. Of that correction the outer
        // end's -- a flanged opening, the pallet's board its flange, 0.82
        // times the radius (Rayleigh; Norris & Sheng, JSV 135, 1989) -- is
        // not a mass alone but the opening's radiation (8m): that air in
        // parallel with ρc/A, the classic approximation of a piston's
        // radiation impedance, a mass at low frequency and a resistance ρc/A
        // at high. It is what takes a cell's high resonances' energy out
        // through its hole; without it they rang, the player's "silbido".
        let diameter = math::sqrt(4.0 * design.tone_hole_area / core::f64::consts::PI);
        let outer = (0.8216 * 0.5 * diameter).min(design.end_correction * diameter);
        let hole_length = design.tone_hole_depth + design.end_correction * diameter - outer;
        let hole_inertance = AIR_DENSITY * hole_length / design.tone_hole_area;
        let radiation_mass = AIR_DENSITY * outer / design.tone_hole_area;
        let radiation_resistance = AIR_DENSITY * SPEED_OF_SOUND / design.tone_hole_area;
        let cell_compliance = design.cell_volume / (AIR_DENSITY * SPEED_OF_SOUND * SPEED_OF_SOUND);
        // The cell as a tube (8m): CELL_LENGTH_RATIO times the tongue, its
        // section the cell's volume over that. Kirchhoff's wall losses at
        // 2 kHz, α ≈ 3·10⁻⁵ √f / a per metre (Fletcher & Rossing, The
        // Physics of Musical Instruments, §8.2), a the radius of the section:
        // a resistance R' = 2Z₀α along it, which loses αL of a wave in a
        // pass. Taken whole at the opening, in series with the hole, R =
        // Z₀αL: half of R'L, as the flow along a tube closed at one end
        // falls from the opening's to nothing. A resistance in the flow, so
        // the cell's still air loses nothing -- as multiplying the waves by
        // e^(-αL) at the ends did, a leak at no frequency (8m).
        let tube_length = CELL_LENGTH_RATIO * design.length;
        let tube_section = design.cell_volume / tube_length;
        let radius = math::sqrt(tube_section / core::f64::consts::PI);
        let attenuation = 3.0e-5 * math::sqrt(2000.0) / radius * tube_length;
        let wall_resistance = AIR_DENSITY * SPEED_OF_SOUND / tube_section * attenuation;
        // The slot along the tongue, rivet toward the opening and tip toward
        // the closed end -- turned round from TURNED_FROM_HZ -- giving air
        // at four points by the mode shape there: its sides open as the
        // tongue bends, nothing at the rivet and most at the tip.
        let turned = design.frequency >= TURNED_FROM_HZ;
        let mut slot_points = [0.0; SLOT_POINTS];
        let mut slot_weights = [0.0; SLOT_POINTS];
        for (k, (point, weight)) in slot_points
            .iter_mut()
            .zip(slot_weights.iter_mut())
            .enumerate()
        {
            let along = (k + 1) as f64 / SLOT_POINTS as f64;
            let index = math::round(along * (crate::tongue::SPAN_POINTS - 1) as f64) as usize;
            *weight = mode.shape[index].abs();
            let from_rivet = RIVET_AT + along * (TIP_AT - RIVET_AT);
            *point = if turned { 1.0 - from_rivet } else { from_rivet };
        }
        let total: f64 = slot_weights.iter().sum();
        for weight in slot_weights.iter_mut() {
            *weight /= total;
        }
        let omega = 2.0 * core::f64::consts::PI * design.frequency;
        let scale = design.length / DEFLECTION_LENGTH;
        let (low, high) = (DEFLECTION_LOW * scale, DEFLECTION_HIGH * scale);
        let mut model = Self {
            design,
            omega,
            mu: effective_area / modal_mass,
            effective_area,
            modal_mass,
            root_thickness,
            inertance,
            slot_radiation,
            hole_inertance,
            radiation_mass,
            radiation_resistance,
            cell_compliance,
            slot_area,
            hole_rim: crate::pallet::rim(design.tone_hole_area),
            seat_viscosity: {
                let rim = crate::pallet::rim(design.tone_hole_area);
                12.0 * AIR_VISCOSITY * SEAT_WIDTH * rim * rim
            },
            omega2: omega * omega,
            linear_damping: omega / design.q,
            swing_density: design.swing_limit * AIR_DENSITY,
            section_low: low,
            section_span: high - low,
            inverse_width: 1.0 / design.width,
            inverse_mass: 1.0 / modal_mass,
            section_scale: (SECTION_POINTS - 1) as f64 / (high - low),
            tube_seconds: tube_length / SPEED_OF_SOUND,
            inverse_cell_volume: 1.0 / design.cell_volume,
            wall_resistance,
            slot_points,
            slot_weights,
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
            // The tongue at rest is the set's shape, not the mode's: each
            // element sits set·φ(x) above the plate, so it crosses it when
            // the tip has moved set·φ(x)/ψ(x), not all at the tip's set.
            let x = i as f64 / (SPAN_POINTS - 1) as f64;
            let offset = d.set * (mode.shape[i] - set_shape(x));
            sides += weight * gap(y * mode.shape[i] + offset, thickness, d.side_clearance);
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

    fn build_section(&mut self, mode: &TongueMode) {
        let (low, span) = (self.section_low, self.section_span);
        for i in 0..SECTION_POINTS {
            let y = low + span * i as f64 / (SECTION_POINTS - 1) as f64;
            self.section[i] = self.section_at(y, mode) as f32;
        }
    }

    /// The useful section at displacement `zeta` from rest, from the table.
    #[inline]
    pub fn section(&self, zeta: f64) -> f64 {
        let y = zeta - self.design.set;
        let position = (y - self.section_low) * self.section_scale;
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

/// The tongue's shape at rest over its length, from the rivet (0) to the tip
/// (1), as a share of the set (milestone 8m). A technician's description:
/// two thirds of the tongue "almost in line with the slot", the last third
/// "gently curves up to give you your desired gap" (tcabot, accordionists.info,
/// "A question about reed profiles"); the curve a parabola, assumed. Until 8m
/// the rest shape was the mode's own, so every element crossed the plate at
/// the same instant and the flow was cut at once along the whole slot -- a
/// pulse whose highs the tube's resonances rang with (the player's
/// "silbido"). Closing from the rivet toward the tip spreads the cut, as the
/// IfM measured a gap opening toward the tip lowering the upper partials and
/// keeping the fundamental and first overtones (Baltrusch, Schetelich &
/// Ziegenhals, bandoneon, 2008).
///
/// How much lies flat is between two technicians: tcabot's two thirds, and
/// dak's tongue that "closes off the reed plate over its full length when
/// passing through", none (the same thread). See [`SET_FLAT`].
fn set_shape(x: f64) -> f64 {
    if x < SET_FLAT {
        0.0
    } else {
        let rise = (x - SET_FLAT) / (1.0 - SET_FLAT);
        rise * rise
    }
}

/// The share of the tongue, from the rivet, that lies flat at rest: a third,
/// chosen between the two descriptions by what it does (2026-10-02). Two
/// thirds flat left a low tongue's slot all but shut at rest, and the 16′
/// C2 took 391 ms to speak at 400 Pa (79 with the mode's shape; 50-140
/// measured); a third, 225 ms at the same start, keeping most of what two
/// thirds took from the cell's bands (the F3's 9-14 kHz −44 dB against −47,
/// the mode's −38).
const SET_FLAT: f64 = 1.0 / 3.0;

/// The viscosity of air at 20 °C, Pa·s.
const AIR_VISCOSITY: f64 = 1.81e-5;

/// How far the pallet's pad lies over the hole's rim, its seat, m: the
/// length of the slit the air passes as the pad comes down (milestone 8m).
/// Assumed: a felt-and-leather pad overlapping its hole by a few
/// millimetres.
const SEAT_WIDTH: f64 = 2.0e-3;

/// The cell's length against the tongue's (milestone 8m). Tonon's G4 cell
/// is 46 mm (PICA 2, 2005, Table 1), a maker's F4 cell some 50 mm, and the
/// F4 tongue 36 mm: derived, roughly.
pub const CELL_LENGTH_RATIO: f64 = 1.4;

/// Where the tongue lies in its cell, as shares of the cell from its
/// opening: the rivet and the tip, as a reed is mounted (patents
/// US2051621, US5824927; technicians). Assumed within that.
const RIVET_AT: f64 = 0.25;
const TIP_AT: f64 = 0.95;

/// From A#6 up makers turn the reeds round, tip to the opening: they speak
/// better so (technicians; Tonon, PICA 2005).
const TURNED_FROM_HZ: f64 = 1800.0;

/// The points along the slot its air enters the cell at.
const SLOT_POINTS: usize = 4;

/// The lines' length in steps; a cell's tube is at most three fewer: at
/// 192 kHz, the highest rate the reed runs at, 109 mm -- the longest cell,
/// the lowest bass reed's (a 64.6 mm tongue), is 90 mm; at 384 kHz, where
/// the analysis checks the scheme converges, 54 mm, the measured F4's 50.
/// Longer cells are taken as that long.
pub const TUBE_SAMPLES: usize = 64;

/// Where along a model's cell, from its opening, the slot's points lie, and
/// how much of the slot's air each gives and takes (milestone 8m).
impl ReedModel {
    pub fn slot(&self) -> impl Iterator<Item = (f64, f64)> + '_ {
        self.slot_points.into_iter().zip(self.slot_weights)
    }

    /// The same reed turned round in its cell, its tip to the opening or
    /// away from it: what a maker does to the highest reeds (diagnosis).
    pub fn turned_round(mut self) -> Self {
        for point in self.slot_points.iter_mut() {
            *point = 1.0 - *point;
        }
        self
    }
}

/// The cell as a tube with the reed along it (milestone 8m): a waveguide
/// from the opening, under the pallet's hole, to the closed end; one line
/// of the waves running toward the closed end and one back, each written
/// at its starting end and read where it has got to. The slot gives and
/// takes air at points along it. A cell is a lumped volume only while it is
/// short against the wavelength -- every dimension under ~0.15 of it
/// (Tonon, PICA 2, 2005): a 50 mm cell, under ~1 kHz; above, a tube passes
/// the jet's high harmonics through its resonances where a volume
/// low-passes them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tube {
    toward_closed: [f32; TUBE_SAMPLES],
    toward_opening: [f32; TUBE_SAMPLES],
    at: usize,
}

impl Default for Tube {
    fn default() -> Self {
        Self {
            toward_closed: [0.0; TUBE_SAMPLES],
            toward_opening: [0.0; TUBE_SAMPLES],
            at: 0,
        }
    }
}

impl Tube {
    /// A tube holding a steady `pressure` throughout: half of it each way.
    pub fn steady(pressure: f64) -> Self {
        let half = (0.5 * pressure) as f32;
        Self {
            toward_closed: [half; TUBE_SAMPLES],
            toward_opening: [half; TUBE_SAMPLES],
            at: 0,
        }
    }

    /// The line's slot written `back` steps ago.
    fn back(&self, back: usize) -> usize {
        (self.at + TUBE_SAMPLES - back) % TUBE_SAMPLES
    }

    /// The tube at `h`: its length in steps, D, which need not be whole --
    /// a tube rounded to whole steps is a different tube at every rate, and
    /// with it the harmonics its resonances lift (8m). The waves back toward
    /// the opening take ⌊D⌋ steps, those toward the closed end the rest of
    /// the round trip, 2D - ⌊D⌋ = K + φ, read between two steps by a straight
    /// line. Returns (D, ⌊D⌋, K, φ).
    fn lengths(model: &ReedModel, h: f64) -> (f64, usize, usize, f64) {
        let delay = (model.tube_seconds / h).clamp(2.0, (TUBE_SAMPLES - 3) as f64);
        let whole = delay as usize;
        let there = 2.0 * delay - whole as f64;
        let steps = there as usize;
        (delay, whole, steps, there - steps as f64)
    }

    /// The tube's characteristic impedance ρc/S, its section the cell's
    /// volume over its length D -- so its compliance at low frequency is
    /// exactly the volume's.
    fn impedance(model: &ReedModel, delay: f64, h: f64) -> f64 {
        AIR_DENSITY * SPEED_OF_SOUND * SPEED_OF_SOUND * delay * h * model.inverse_cell_volume
    }

    /// The energy its waves hold, J, between steps: a step's length of a
    /// wave p holds p² h / Z; of the line read between two steps, the older
    /// of them only its share φ. Read so, the straight line never adds:
    /// ((1 - φ)a + φb)² ≤ (1 - φ)a² + φb², so the lines are passive.
    pub fn energy(&self, model: &ReedModel, h: f64) -> f64 {
        let (delay, whole, steps, share) = Self::lengths(model, h);
        let per_square = h / Self::impedance(model, delay, h);
        let square = |line: &[f32; TUBE_SAMPLES], back: usize| {
            let wave = f64::from(line[self.back(back)]);
            wave * wave
        };
        let toward_closed: f64 = (1..=steps)
            .map(|back| square(&self.toward_closed, back))
            .sum::<f64>()
            + share * square(&self.toward_closed, steps + 1);
        let toward_opening: f64 = (1..=whole)
            .map(|back| square(&self.toward_opening, back))
            .sum();
        (toward_closed + toward_opening) * per_square
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
    /// The pallet's curtain at the last step, m²: while it changes, the
    /// pallet is moving (see [`step`]).
    pub pallet: f64,
    /// ρ / (2 α² A_p²) for that curtain, kept while it does not change
    /// (milestone 10c).
    pub pallet_factor: f64,
    /// The flow through the hole's radiating mass, m³/s (milestone 8m);
    /// the rest of the hole's flow passes its radiation resistance.
    pub radiation_flow: f64,
    /// The flow through the near field's mass, m³/s (milestone 8m); the
    /// rest of the slot's flow passes the far side's radiation resistance.
    pub near_flow: f64,
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
            // No curtain: ρ / (2 α² ∞²) is nothing.
            pallet: f64::INFINITY,
            pallet_factor: 0.0,
            // A steady flow passes the masses, not the resistances.
            radiation_flow: flow,
            near_flow: flow,
        }
    }

    /// Stored energy, J: the tongue's kinetic and elastic energy, the
    /// kinetic energy of the air in both inertances and the cell's
    /// compressed air.
    pub fn energy(&self, model: &ReedModel) -> f64 {
        0.5 * model.modal_mass
            * (self.velocity * self.velocity + model.omega * model.omega * self.zeta * self.zeta)
            + 0.5 * model.inertance * self.near_flow * self.near_flow
            + 0.5 * model.hole_inertance * self.hole_flow * self.hole_flow
            + 0.5 * model.radiation_mass * self.radiation_flow * self.radiation_flow
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
pub fn step(
    model: &ReedModel,
    state: &mut ReedState,
    tube: &mut Tube,
    supply: f64,
    pallet: f64,
    h: f64,
) -> f64 {
    let d = &model.design;
    let omega2 = model.omega2;
    let lift = (state.zeta + 0.5 * h * state.velocity) * model.inverse_width;
    let speed = math::sqrt(2.0 * state.cell_pressure.max(0.0) / AIR_DENSITY);
    let limit = model.swing_density * speed * d.width * d.length * lift * lift;
    let damping = model.linear_damping + limit * model.inverse_mass;
    let s_r = model.effective_area;
    let (m_n, m_h) = (model.inertance, model.hole_inertance);
    // The cell, a tube (8m). At its opening its pressure is twice the wave
    // arriving there plus its impedance times the hole's flow in: p_h = 2q⁻
    // + Z a. Its closed end sends back what reaches it. At each of the
    // slot's points the pressure is the two waves passing plus Z/2 times
    // the air the slot puts in there, -w_k u; the tongue feels Σ w_k p_k =
    // A - Z_s u, A the waves' share and Z_s = (Z/2) Σ w_k². The waves take
    // a step or more between points, so each is solved alone.
    let (delay, whole, steps, share) = Tube::lengths(model, h);
    let z = Tube::impedance(model, delay, h);
    let arriving_at_opening = f64::from(tube.toward_opening[tube.back(whole)]);
    let arriving_at_closed = (1.0 - share) * f64::from(tube.toward_closed[tube.back(steps)])
        + share * f64::from(tube.toward_closed[tube.back(steps + 1)]);
    // Points that fall on one step of a short tube are one point, their
    // weights summed: a step given air twice over would be given the cross
    // term 2g₁g₂ of energy from nowhere. The points run along the tube in
    // order, so only neighbours can meet.
    let mut points = [(0usize, 0usize, 0.0f64); SLOT_POINTS];
    let mut count = 0;
    for (at, weight) in model.slot_points.into_iter().zip(model.slot_weights) {
        let i = (math::round(at * delay) as usize).clamp(1, whole - 1);
        let point = (tube.back(i), tube.back(whole - i));
        match points[..count].last_mut() {
            Some(last) if (last.0, last.1) == point => last.2 += weight,
            _ => {
                points[count] = (point.0, point.1, weight);
                count += 1;
            }
        }
    }
    let points = &points[..count];
    let mut waves = 0.0;
    let mut squares = 0.0;
    for (toward_closed, toward_opening, weight) in points {
        waves += weight
            * (f64::from(tube.toward_closed[*toward_closed])
                + f64::from(tube.toward_opening[*toward_opening]));
        squares += weight * weight;
    }
    let slot_impedance = 0.5 * z * squares;
    let jet_flow = state.flow - s_r * state.velocity;
    let section = model.section(state.zeta + 0.5 * h * state.velocity);
    let alpha_section = d.contraction * section;
    let r = AIR_DENSITY * jet_flow.abs() / (2.0 * alpha_section * alpha_section);
    // Midpoint unknowns: tongue velocity w, flow onto the reed u, hole flow
    // a. Rows: tongue, near field (p = A - Z_s u), hole (p_h = 2q⁻ + Z a).
    //   (2 + hγ + h²ω²/2 + hμRS_r) w - hμR u            = 2w₀ - hω²ζ₀
    //   -hRS_r w + (hR + hZ_s + hR_e) u                  = h A + h R_e m₀  (below)
    //   (2M_h + hR_p + hZ + hR_w + hR_e) a               = 2M_h a₀ + hP + hR_e m₀ - 2h q⁻
    // R_w the walls' resistance, R_e the radiating end's (below).
    // (Until 8m the cell was a volume, a fourth row: h u - h a + 2C p =
    // 2C p₀.)
    // R_p is the pallet's curtain, an orifice linearised like the reed's own
    // jet: R_p = ρ |a₀| / (2 α² A_p²) ≥ 0. A closed pallet is a seal: the
    // hole passes nothing, and its row becomes a = 0. While the pallet moves,
    // the linearisation lags a curtain that changes faster than the flow
    // through it: as it first lifts, a₀ = 0 lets the hole pass for a step as
    // if no pallet were there, and the flow bursts and collapses by turns,
    // whose rate of change radiated as a click (docs/ROADMAP.md, 8h). So
    // while it moves R_p is never less than Bernoulli's orifice at the drop
    // across it, √(ρΔp/2)/(α A_p), which steady flow's linearisation equals.
    // Settled, as before.
    let moving = pallet != state.pallet;
    // The curtain's air has mass too, ρw/A over the seat's width w (8m): the
    // hole's end correction grows as the pallet comes close (Tonon's k, higher
    // with the pallet near), and as the pad comes down the tone is filtered
    // away, the highs first -- a real note let go fades over the pallet's
    // travel, -1, -4, -8, -11 dB at 10, 20, 30, 40 ms (FreePats' Hohner
    // releases, CC0), where the curtain's resistance alone kept it whole to
    // the seat and then cut it. While the mass grows the air keeps its
    // momentum, M₁a = M₀a₀, and while it shrinks its speed: either way the
    // air's energy can only fall.
    let curtain_mass = |area: f64| {
        if area > 0.0 && area.is_finite() {
            AIR_DENSITY * SEAT_WIDTH / area
        } else {
            0.0
        }
    };
    let m_h = m_h + curtain_mass(pallet);
    let before = model.hole_inertance + curtain_mass(state.pallet);
    if pallet > 0.0 && m_h > before {
        state.hole_flow *= before / m_h;
    }
    state.pallet = pallet;
    // The hole's row solved for a: a = (b3 - h p) / dh, kept as 1/dh, which
    // is zero for a closed pallet. While the curtain is all but shut the row
    // is stiff -- hR_p > 2M_h -- and the trapezoid answers a decay faster
    // than a step by flipping the flow's sign at every step, a burst at the
    // Nyquist frequency (8k; the "bursts by turns" of 8h). There the row is
    // weighted θ = 1 - M_h/(hR_p) instead of ½, the least that does not
    // flip: a = θ a₁ + (1 - θ) a₀ in M_h(a₁ - a₀)/h = P - p - R_p a. Below
    // that stiffness θ is ½ and the step is the trapezoid, bit for bit.
    // The hole's radiating end, its mass M_r beside R_r: the flow m through
    // the mass at the midpoint, (2M_r/h)(m - m₀) = R_r (a - m), so m = (1 -
    // g) m₀ + g a, g = R_r/(2M_r/h + R_r), and the drop across it R_r (a -
    // m) = R_e (a - m₀), R_e = R_r (1 - g): one more resistance in the
    // hole's row, and a push R_e m₀ from the air still moving in the mass.
    let radiation_weight = 2.0 * model.radiation_mass / h;
    let toward_mass = model.radiation_resistance / (radiation_weight + model.radiation_resistance);
    let radiating = model.radiation_resistance * (1.0 - toward_mass);
    let mut theta = 0.5;
    let (b3, inverse_dh) = if pallet > 0.0 {
        let alpha_pallet = d.contraction * pallet;
        if moving {
            state.pallet_factor = AIR_DENSITY / (2.0 * alpha_pallet * alpha_pallet);
        }
        let linearised = state.pallet_factor * state.hole_flow.abs();
        let bernoulli = if moving {
            let drop = (supply - state.cell_pressure).abs();
            linearised.max(math::sqrt(0.5 * AIR_DENSITY * drop) / alpha_pallet)
        } else {
            linearised
        };
        // The curtain is also a thin slit, the pad's gap g = A/R over the
        // seat's width w, round the hole's rim R: laminar, its resistance is
        // Poiseuille's, 12 μ w/(g³ R) = 12 μ w R²/A³ (8m). Nothing while the
        // pallet is open -- a 3 mm gap, a few hundred Pa·s/m³ -- it is what
        // chokes the flow as the pad comes down, smoothly and before it
        // touches: Bernoulli's alone falls with the flow it throttles, so the
        // hole passed the tone until the curtain was nothing, then cut it, a
        // click as a note was let go under the bellows' push.
        let viscous = model.seat_viscosity / (pallet * pallet * pallet);
        let r_p = bernoulli + viscous;
        let resistance = r_p + z + model.wall_resistance + radiating;
        let pushed =
            h * (supply + radiating * state.radiation_flow) - 2.0 * h * arriving_at_opening;
        if h * resistance > 2.0 * m_h {
            theta = 1.0 - m_h / (h * resistance);
            let weight = m_h / theta;
            (
                weight * state.hole_flow + pushed,
                1.0 / (weight + h * resistance),
            )
        } else {
            (
                2.0 * m_h * state.hole_flow + pushed,
                1.0 / (2.0 * m_h + h * resistance),
            )
        }
    } else {
        state.hole_flow = 0.0;
        // Forgotten with the curtain, so a reed rung down to nothing is the
        // reed at rest again (`ReedState::default()`).
        state.pallet_factor = 0.0;
        (0.0, 0.0)
    };
    // The hole alone; a shut pallet passes nothing and the tube's end is
    // rigid there, p_h = 2q⁻.
    let a = b3 * inverse_dh;
    let hole_pressure = 2.0 * arriving_at_opening + z * a;
    let mass_flow = (1.0 - toward_mass) * state.radiation_flow + toward_mass * a;
    state.radiation_flow = 2.0 * mass_flow - state.radiation_flow;
    // The reed and its near field. While u carried the near field's mass
    // itself, its row turned stiff with the tube's Z_s in it, h(R + Z_s) >
    // 2M_n, and the flow's end value flipped at the internal Nyquist
    // frequency -- beaten against the tone through the jet's nonlinearity,
    // tones at fs/2 - n f0 (8m, the player's "squeal"); it was weighted θ
    // then, as the hole's row is (8k). Now u carries no mass of its own.
    //
    // Since the slot's far side radiates (8m) the slot's flow u passes the
    // jet and then the far side: its mass M_n beside R_s = ρc/S_slot. The
    // flow m through the mass, weighted θ = 1 - M_n/(hR_s) where hR_s >
    // 2M_n, else ½: (M_n/(θh))(m - m₀) = R_s (u - m), m = (W m₀ + R_s u)/(W +
    // R_s), W = M_n/(θh); the far side's drop R_s (u - m) = R_e (u - m₀),
    // R_e = R_s W/(W + R_s). The near-field row is then u's alone:
    //   -hRS_r w + (hR + hZ_s + hR_e) u = h A + h R_e m₀
    // where it was (2M_n + hR + hZ_s) u = 2M_n u₀ + h A.
    let near_theta = if h * model.slot_radiation > 2.0 * m_n {
        1.0 - m_n / (h * model.slot_radiation)
    } else {
        0.5
    };
    let near_weight = m_n / (near_theta * h);
    let to_mass = model.slot_radiation / (near_weight + model.slot_radiation);
    let far_side = model.slot_radiation * near_weight / (near_weight + model.slot_radiation);
    let a11 = 2.0 + h * damping + 0.5 * h * h * omega2 + h * model.mu * r * s_r;
    let a12 = -h * model.mu * r;
    let a21 = -h * r * s_r;
    let a22 = h * (r + slot_impedance + far_side);
    let b1 = 2.0 * state.velocity - h * omega2 * state.zeta;
    let b2 = h * (waves + far_side * state.near_flow);
    let inverse_det = 1.0 / (a11 * a22 - a12 * a21);
    let w = (b1 * a22 - a12 * b2) * inverse_det;
    let u = (a11 * b2 - a21 * b1) * inverse_det;
    let p = waves - slot_impedance * u;
    // The ends send their waves on; the slot's points put their air in.
    tube.toward_closed[tube.at] = (hole_pressure - arriving_at_opening) as f32;
    tube.toward_opening[tube.at] = arriving_at_closed as f32;
    for (toward_closed, toward_opening, weight) in points {
        let given = (-0.5 * z * weight * u) as f32;
        tube.toward_closed[*toward_closed] += given;
        tube.toward_opening[*toward_opening] += given;
    }
    tube.at = (tube.at + 1) % TUBE_SAMPLES;
    let previous_hole_flow = state.hole_flow;
    state.zeta += h * w;
    state.velocity = 2.0 * w - state.velocity;
    state.flow = u;
    let mass_flow = (1.0 - to_mass) * state.near_flow + to_mass * u;
    state.near_flow = (mass_flow - (1.0 - near_theta) * state.near_flow) / near_theta;
    state.hole_flow = if theta == 0.5 {
        2.0 * a - state.hole_flow
    } else {
        (a - (1.0 - theta) * state.hole_flow) / theta
    };
    state.cell_pressure = p;
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
            pallet: f64::INFINITY,
            pallet_factor: 0.0,
            radiation_flow: 0.0,
            near_flow: 4.0e-5,
        };
        let mut state = start;
        let h = 1.0 / 96_000.0;
        // The cell's air is the tube's (8m): its waves hold what the cell's
        // pressure held, so the whole is the tongue's, the near field's, the
        // hole's and the tube's.
        let mut tube = Tube::steady(start.cell_pressure);
        let total = |state: &ReedState, tube: &Tube| {
            state.energy(&model) - 0.5 * model.cell_compliance * state.cell_pressure.powi(2)
                + tube.energy(&model, h)
        };
        let mut energy = total(&state, &tube);
        let first = energy;
        // At Q 95 and 355 Hz the tongue's energy falls with a 42 ms time
        // constant: 0.625 s is fifteen of them.
        for _ in 0..60_000 {
            step(&model, &mut state, &mut tube, 0.0, f64::INFINITY, h);
            let next = total(&state, &tube);
            assert!(next <= energy * (1.0 + 1e-9), "{next} > {energy}");
            energy = next;
        }
        assert!(energy < 1e-3 * first, "{energy}");
    }
}
