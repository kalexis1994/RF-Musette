# The RF-Musette model

This document is the model's ledger. Each mechanism the engine implements names
the physics it comes from and the paper that measured it; each simplification is
stated rather than hidden; each constant says where its value came from. The
tests hold the model to what this document claims -- it is allowed to be
approximate, not to drift from what is written here.

**Status (0.2.0): one reed.** The accordion F4 tongue the IfM Zwota measured,
in its cell, blown by the bellows, on key 65. Every other key is silent.

## How a value earns its place

There is no reference recording to fit against (decided 2026-09-30, see
[RESEARCH.md](RESEARCH.md)). So every constant carries one of four statuses,
and keeps it:

| Status | Means | Must record |
| --- | --- | --- |
| **Measured** | Taken from a published measurement | The source, and whether it was read in full or from an abstract or figure |
| **Derived** | Computed from measured values by stated physics | The derivation, in the code beside it and here |
| **Assumed** | Chosen where nothing is published, before anyone listened | Why this value, and what bounds it |
| **Voiced by ear** | Set by listening | The date, what was heard, and the range the literature allows |

A voiced or assumed value is never later cited as measured. Where the
literature gives a number for something the model *produces* rather than
*takes* -- where a reed starts to speak, how far it swings -- that number is a
test on the output (`crates/rf-musette-analysis/tests/milestone_1.rs`), not a
constant to fit.

## What is modelled

### The tongue: one mode, of a profiled beam (tested)

The tongue moves on its first bending mode (Millot & Baumann, *Acta Acustica*
93, 2007, eq. 4; measured sinusoidal on accordion reeds by Misdariis, Ricot &
Caussé, CFA 2000, and by Ziegenhals, IfM Zwota 2009):

    ζ'' + (ω0/Q) ζ' + ω0² ζ = μ Δp,        μ = S_r / M_r

**The profile is derived, not assumed.** A uniform beam has its first two
modes at 1 : 6.27; the F4 tongue Ziegenhals measured has them at 355 and
1645 Hz, 1 : 4.6, "because of the profile". The engine takes the tongue as a
linear taper of thickness, solves the Euler-Bernoulli problem by
Rayleigh-Ritz, finds the taper that gives 4.6 and then the root thickness
that puts the first mode at 355 Hz (`tongue.rs`; a test checks the solver
against the uniform beam's closed form to 1e-5). For the F4: taper 0.56,
0.51 mm at the root, 0.23 mm at the tip. Modal mass, effective area
S_r = W L ∫ψ, and the mode shape the air sees all follow.

This mattered. With the tongue assumed uniform (0.564 mm throughout) its tip
stiffness was 790 N/m, it swung 2.7 mm at 300 Pa and only began to speak at
69 Pa; profiled, it swings 4.9 mm -- Ziegenhals measured "more than 4 mm
already at mf" -- and speaks from 32 Pa.

### The useful section (tested)

The air escapes around the tongue's edges and tip. Millot & Baumann integrate
the escape area along both long edges and add the front (their eq. 9). Their
law keeps growing while the tongue is inside the plate (their Fig. 9); here,
as in Ziegenhals's "phase 2", a tongue inside the slot leaves only the
clearance, since that is the narrowest the air can pass. Each edge element
has three phases (Ziegenhals 2009): above the plate, the gap from the
tongue's face to the slot's upstream edge; inside, the clearance; out through
the far side, the gap from the tongue's upstream face (its local thickness
behind) to the downstream edge. The tip moves on an arc and is drawn back
from the slot's end by δ = y²/(2L)∫ψ'², and its end face tilts; both change
the front gap, which is of the order of the clearance. The total never
exceeds the slot's own area (Ziegenhals's F_max = l b). Tabulated at control
rate over −9..12 mm.

### The jet and the pumped flow

    u = S_r ζ' + α S_u(ζ) v_j,        Δp = ½ ρ v_j |v_j|

Millot & Baumann eqs. 5-7: what the tongue pumps plus a quasi-steady
Bernoulli jet through the useful section, with a vena contracta α.

### The cell is not optional (tested)

A blown-closed free reed needs no resonator tuned to it, but it does need air
to accelerate: the inertia of the flow onto the gap makes the pressure on the
tongue lag its motion (St. Hilaire, Wilson & Beavers, *J. Fluid Mech.* 49,
1971; Ricot, Caussé & Misdariis, *JASA* 117, 2005).

*Tried first, and measured wrong:* the reed between a pressure source and the
atmosphere with only the air converging on the slot as inertance (estimated
as a flanged end correction, 48 kg/m⁴). The F4 reed then stays silent below
about 3 kPa, where real reeds speak from tens of pascals. Measured, not
assumed: RK4 at 32× the rate agrees with the shipping scheme to 0.08 cents,
so it was the model and not the numerics; a sweep of every other constant
(Q, set, clearances, contraction) left it above 1.6 kPa; the linear stability
of the equilibrium gives the onset as ∝ 1/(Q·M)², M the inertance -- the
missing mass of air. *Also tried and refuted:* the inertia of the air inside
the gap (Tarnopolsky, Fletcher & Lai, *JASA* 108, 2000, eq. 3, d the plate
thickness) as the missing term; alone it left the onset near 600 Pa.

The mass of air is the cell's. On the bellows' opening stroke the sounding
reed is the one inside its cell in the reed block: air comes from outside
through the tone hole, fills the cell and leaves through the reed. The cell
is Tonon's Helmholtz resonator (*Papers of the International Concertina
Association* 2, 2005):

    M_h u_h' = P - p_c          tone hole, M_h = ρ (t + k d) / A
    C p_c'   = u_h - u          the cell,  C = V / ρc²
    M_n u'   = p_c - Δp         the air converging on the gap

Millot's minimal configuration (his Fig. 8) is the same pair of elements.
With the cell's resonance far above the reed's (2.05 kHz for the defaults),
the reed feels the hole's air as inertia -- and speaks from 32 Pa, a figure
insensitive to what is assumed about the cell (31-41 Pa across 2-18 cm³,
60-400 mm² of hole, 2-10 mm of board). Near or below the reed's frequency the
cell chokes it, as Cottingham (ICA 2019) and Tonon measured; that matters in
the top octaves and is milestone 2's to demonstrate.

### Radiation

Outside, the reed is heard through the air the tone hole passes: a monopole
`p = ρ/(4π r) u_h'` at r = 1 m. Ziegenhals: the sound is the derivative of the
volume flow, and the tongue's own radiation is negligible. Ricot finds a
bare reed dipole-dominated because the flow leaving one face enters from the
other; in an accordion the other face is inside the bellows.

### The bellows intent (decided, not modelled)

The player's intent reaches the engine as a fraction from 0 to 1: the latest
key's velocity until Expression (CC 11, 14 bits with CC 43) arrives, then
Expression for good. The supply pressure is `ceiling × intent^curve`, 1 kPa
and 2 by default (assumed: half the push is 250 Pa, inside normal play). A
1 ms smoothing keeps a stepped controller from reaching the reed as a step;
it is numerical, not physics -- the bellows' own compliance is milestone 5.

## How it is computed (tested)

Implicit midpoint on every linear part, with the jet's Bernoulli loss
linearised about the flow at the start of the step: Δp = R ũ with
R = ρ|ũ₀|/(2α²S_u²) ≥ 0. Whatever R is, the step satisfies exactly

    H₁ − H₀ = h [ P u_h − M_r (ω0/Q) ζ'² − R ũ² ]      (midpoint values)

so with the supply off the stored energy can only fall: passive for every
parameter set, with no iteration. The idea -- a Bernoulli port kept
dissipative by construction, solved linearly -- is Darabundit & Scavone's for
a clarinet reed (*Frontiers in Signal Processing*, 2025); this form is
derived here. The four midpoint unknowns reduce to a 2×2 system whose
determinant is always positive. Tested: energy never rises with the supply
off; 119 parameter sets (every extreme and 60 random combinations) stay
finite; the tone agrees with RK4 at 32× the rate to 0.07 cents and 0.3 %,
and with the same scheme at 8× to 0.05 cents.

The reed runs at twice the host rate (a parameter: 1, 2 or 4) and one
windowed-sinc FIR (Blackman, 96 taps per unit of the factor) decimates the
instrument's sum: flat to 0.40 of the host rate, 70 dB down past its Nyquist
frequency (tested).

Cost, native x86_64: 321 ns per host sample for one reed at 2× -- 65 times
real time. Not yet measured as wasm fuel, nor on the Raspberry Pi.

## The constants (0.2.0)

| Parameter | Value | Status | Source |
| --- | --- | --- | --- |
| Reed frequency | 355 Hz | Measured | Ziegenhals 2009, Fig. 4 (plucked) |
| Mode ratio | 4.6 | Measured | Ziegenhals 2009: 1645 / 355 Hz |
| Tongue length, width | 36, 4 mm | Measured | Ziegenhals 2009 |
| Set (Aufbiegung) | 0.5 mm | Measured | Ziegenhals 2009 |
| Plate thickness | 3 mm | Measured | Ziegenhals 2009 |
| Taper, root and tip thickness | 0.56; 0.51, 0.23 mm | Derived | From the four above, steel |
| Steel E, ρ | 200 GPa, 7850 kg/m³ | Textbook | Spring steel (Llanos 2002; Harmonikas.cz) |
| Side, tip clearance | 0.035, 0.04 mm | Specified | Harmonikas.cz machined plates |
| Jet contraction α | 0.61 | Measured | Tarnopolsky et al. 2000 (range 0.5-1) |
| Q | 250 | Assumed | Woodhouse's bench free reed (Euphonics 11.6); a brass harmonica reed measures 95 (Millot & Baumann) |
| Near-field inertance | 48 kg/m⁴ (×1) | Derived, roughly | Rayleigh end correction of the slot's area |
| Cell volume | 8 cm³ | Assumed | Lab accordion chambers 8-18 cm³ for a ~90 Hz reed (Cottingham 2013 slides) |
| Tone hole area, depth | 150 mm², 5 mm | Assumed | Not published |
| End correction k | 0.6 | Measured range | Tonon 2005: 0.43-0.80 |
| Bellows ceiling, curve | 1 kPa, 2 | Assumed | Technician's ~1 kPa, unverified |

## What it does (0.2.0, measured; `milestone_1.rs` and `diagnosis.rs`)

| Behaviour | Model | Literature | |
| --- | --- | --- | --- |
| Onset from rest | 32 Pa | 10 Pa (Misdariis), 60-110 Pa (Cottingham, a 622 Hz reed), ~30 Pa (technicians) | Met |
| Offset, let down from 300 Pa | 8 Pa (22 Pa from 65 Pa) | Below onset, always (Cottingham ICA 2016) | Met |
| Tip swing at 300 Pa | 4.9 mm | > 4 mm "at mf" (Ziegenhals, this reed) | Met |
| Tongue motion | 2nd-4th harmonics 59-72 dB down | Sinusoidal (Misdariis; Ziegenhals) | Met |
| Sound | Pulse train, harmonics within a few dB to the 7th | Pulse-like "Summton" (Ziegenhals) | Met |
| Level, 60 → 3000 Pa | 40.9 dB | ~40 dB (Misdariis) | Met |
| Pitch vs pressure, 100 → 900 Pa | −0.4 cents | about −9 cents (Cottingham) | **Not met** |

## Known defects

* **The pitch does not sag with pressure.** The reed sounds 6.3 cents below
  its mode at every pressure, within half a cent from 80 Pa to 3 kPa; a
  measured accordion reed falls about 9 cents from 0.1 to 0.9 kPa
  (Cottingham, fig-read), a bandoneon's low notes by up to half a semitone.
  Millot & Baumann report the same of their minimal model (a blown-closed
  reed's frequency moved 0.4 Hz with the excitation), and the linear
  analysis says why: in this class of model the aerodynamic stiffness is
  independent of the supply pressure unless the tongue's mean position
  moves, and it moves 0.07 mm at 900 Pa. A softer tongue does not fix it
  (8× softer gives −2.8 cents and swings of 10 mm). The mechanism is not in
  the model; Ricot et al. 2005 (paywalled) and Misdariis's unsteady potential
  flow are where to look. It is not scripted in the meantime. The failing
  prediction is kept, ignored, in `milestone_1.rs`.
* **The absolute level may be ~20 dB high.** At 50 cm the render measures
  75 dB (pp, ~100 Pa) and 91 dB (mf, ~400 Pa) unweighted, against
  Llanos-Vázquez et al.'s 55 and 70 dBA for p and mf. Their pressures are not
  known and the weighting differs, so this is a lead, not yet a measurement.
  Related: the reed spends 390 mL/s at 300 Pa and 1.3 L/s at 900 Pa, and
  nothing published says what an accordion reed spends.
* **One mode.** Accordion tongues carry their 2nd-4th bending modes and a
  torsional mode, most visibly in the attack (Behrens et al. 2009; Cottingham
  ICA 2019).

## What is deliberately not modelled yet

The pallet (the tone hole opening with the key), the choking of high reeds by
their cells, the reed plate's second reed and its valve, the bellows
direction, ranks and tremolo, the bellows reservoir, the cassotto, the body,
the rest of the compass and the Stradella bass. In order, with their
predictions: [ROADMAP.md](ROADMAP.md).
