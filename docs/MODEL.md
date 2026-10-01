# The RF-Musette model

This document is the model's ledger. Each mechanism the engine implements names
the physics it comes from and the paper that measured it; each simplification is
stated rather than hidden; each constant says where its value came from. The
tests hold the model to what this document claims -- it is allowed to be
approximate, not to drift from what is written here.

**Status (0.8.0, in progress): the whole treble.** F3-A6, 41 keys, five
ranks each (410 reeds), tuned to A440 where they sound, one bellows, a
cassotto. The text below grew milestone by milestone around the one F4 the
IfM Zwota measured; "The compass" says how every other reed comes from it.

**Earlier status (0.7.0): one key, five ranks, one bellows, a
cassotto.** Key 65, F4, behind
the pallet it lifts, with five plates: L (16′), M− M M+ (the 8′ tremolo
and musette) and H (4′), each with a reed for each way the bellows moves.
The registers open them. The M reed is the F4 the IfM Zwota measured.
They are blown by a bellows that holds its pressure and turns. Every other
key is silent.

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

### The plate's two reeds and the bellows' direction (tested)

Each plate carries two reeds, one per bellows direction:
* **pull:** the reed inside the cell, with the cell upstream (the reed of
  milestones 1-2);
* **push:** the reed on the bellows side, with the cell downstream.

A valve on each slot's far face seals it when its reed would be blown the
wrong way. Here that is an ideal seal: the idle reed sees nothing of the
bellows. F4 has a leather valve; notes from C6 up have none on the
instrument Llanos measured, and their leak waits for those notes.

**The two reeds are the same model.** In a lumped loop the reed feels the
loop's total impedance, near field + (hole ∥ cell), whatever its order. So
a push reed with the cell downstream behaves exactly as a pull reed with
it upstream: measured, to the pascal, for every cell (`tests/push_reed.rs`).
Cottingham (ICA 2016) found push thresholds ten times higher on an
artificial chamber. He calls that preliminary, and it is spatial physics
this class of model cannot hold. On an instrument, Llanos found the attack
independent of direction. Each reed keeps its own copy of the cell, which
differs from the one real cell only while both still move.

**The direction and the turn.** `bellows_direction` (or CC 80, a switch)
chooses the side. A change takes the signed pressure through zero over
`reversal_time`, assumed 100 ms: nothing measures it, and players describe
"a slight interruption, like a bow change". The radiated sound is the
hole's outward flow, so push and pull have opposite polarity.

### Ranks, tremolo and registers (tested)

Five ranks for the key, each its own plate and cell behind the key's one
pallet:
* **M:** the measured F4.
* **M− and M+:** the same tongue at a frequency a beat away, as a tuner
  files it, the profile re-derived. The beats follow the builder's lines
  Hergert measured on a Borsini Super Star LMMMH (Acta Acustica 8, 2024,
  Fig. 6): M+ +4.1 Hz at A4, +1.4 Hz per octave; M− −3.7 Hz, −1.8 Hz per
  octave. Both are scaled to the Tremolo parameter, M+'s beat at A4.
* **L and H:** the F4 scaled an octave down and up by the ratios of a
  bayan maker's slots (RU2233009, Table 3) -- **assumed**, as no maker
  publishes tongue dimensions.

The Register parameter opens ranks as Roland's FR-3x draws its 14 treble
registers (Owner's Manual, p. 27).

Each rank keeps its own copy of the pallet's curtain: each rank's cell has
its own hole under the key's pallet, and how much the ranks still share
there depends on the pallet's geometry, unmeasured (ROADMAP 5).

Each rank is set by its mode, not by where it sounds: L comes out 9.6
cents flat of M's octave (ROADMAP 4).

### One bellows for every reed (assumed constants, voiced by ear)

    C P′ = A v(P) − Σ Q_holes − Q_vent,    F₀ (1 − v/v_max)/(1 + v/(k v_max)) = A P

The intent, by default, is the player's push: F₀ = A P_ask, P_ask being
what the intent asks of a still bellows. The arm's force falls as it moves
faster, by Hill's force-velocity law (Proc. R. Soc. B 126, 1938; k = 0.25).
So the air the reeds' holes pass, the leaks and the air button spend, make
the bellows move faster and the same push hold less pressure. The air's
compliance C = V/(ρc²) smooths it over milliseconds. The step is backward
Euler on the law linearised at the step's start, stable at any step.

What is left out:
* the arm's and the bellows' masses: unmeasured, and with any plausible
  value they would ring near 15-20 Hz, which no player reports.

Every constant is assumed: the area 600 cm², the volume 12 L, v_max
1 m/s, the leak 10 mm², the air button 400 mm². "Stiff" makes the intent
the pressure itself, as before, for a digital accordion that measures it.

Measured on F4 (ROADMAP 5):
* M alone holds 4.0 % below the push, Master 13.6 %;
* the air button fully open takes the pressure down 29 %.

### The compass (tested; geometry and Q's rise assumed)

41 keys, F3-A6, with L an octave below and H an octave above. Every reed is
the measured F4 scaled to the pitch it is made for, by the ratios of a
bayan maker's slots (RU2233009, Table 3):
* the length and width interpolated in log-frequency, and the plate;
* the set with the length;
* the cell with the length cubed, its hole with the slot's area;
* the clearances as machined.

Two further rules:
* **Q rises with pitch as the measured reeds do,** Q = Q_F4 (f/F4)^0.7.
  That is the exponent between a reed-organ C3's Q 83 (Cottingham, ICA
  1998) and a harmonica reed's ~233 at 598 Hz (Förtsch 2021), extrapolated
  above ~750 Hz where nothing is measured. No damping mechanism falls for
  small reeds.
* **Tuned as a tuner tunes.** Each reed's mode is set so it sounds on
  equal temperament at "Pitch A4" (440 Hz) at 300 Pa, the pressure a tuner
  uses because every reed sounds there. The reeds that do not speak there
  are tuned at the lowest pressure they do. The corrections, 4-20 cents,
  are a generated table, `tuning.rs`, checked against a fresh tuning.
  Moving a reed's constants afterwards detunes it, as on an instrument.

Thresholds climb with pitch, 13 Pa at F3 to 170 at A6 on the true 8′. The
4′ reeds from ~3 kHz need 350-440 Pa (a known defect). Every reed swings
~9 % of its length at eight times its own threshold.

### The cassotto (tested; Q assumed)

    H(s) = ω₀² / (s² + (ω₀/Q) s + ω₀²)

When the instrument has one (Cassotto "On"), L and M sound into a box with
a narrow way out. Below its standing waves that is a Helmholtz resonator:
unity at low frequency, a lift around ω₀, −12 dB per octave above.
* **ω₀:** Richter's measured shaft resonance, 800 Hz-1 kHz (IfM Zwota
  1989), 900 Hz by default.
* **Q:** assumed, 2.

It filters the flow those ranks radiate and nothing else: the reeds do not
feel it, as Llanos-Vázquez found (thesis, p174). Discretised as a
topology-preserving state-variable filter at the oversampled rate.

Measured on M at mf:
* the spectral centroid falls from 1693 Hz to 959 Hz (0.57), against
  Llanos's 2013 and 1389 Hz on E4 (0.69);
* the attack does not move.

The grille and the body are not modelled: no transfer function is
published.

### The swing limit (voiced: the one term that is not derived)

    c = κ ρ v w L (ζ/w)²,        v = √(2p/ρ)

A damping on the tongue, added to its own ω0/Q. It grows with the tip's
displacement from rest ζ and with the jet speed v the cell's pressure gives;
w and L are the tongue's width and length, and κ is a live knob ("Swing
Limit").

It stands in for the nonlinear dissipation St. Hilaire & Vaidya (J. Fluid
Mech. 67, 1975) found limits a free reed's swing, which this model does not
derive. Their paper could not be read: neither it nor its sequel could be
bought. Every derived mechanism tried for the limit failed (ROADMAP 2c):
* the tongue's drag in the air;
* the tip coming through the plate;
* the mean moving toward the plate;
* the valves' measured series escape area.

The form is chosen, not derived, for three properties:
* **zero at small swings,** so the onset, the growth, the attack and the
  bend stand as derived;
* **in proportion to the flow, as the feed is,** so the swing it settles at
  hardly depends on the pressure, as Cottingham, Lilly & Reed measured;
* **dissipative,** so the scheme stays passive (an extra −c ζ′² in the
  energy identity). The RK4 reference carries it too.

κ 0.5 puts the swing near 5 mm from ~1 kPa up, inside what is measured of
this reed. Heard on 2026-09-30 against κ 0 on a continuous swell, it was
preferred and kept.

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
missing mass of air. *Also tried, and wrongly counted as refuted:* the
inertia of the air inside the gap (Tarnopolsky, Fletcher & Lai, *JASA* 108,
2000, eq. 3, d the plate thickness); it left the onset near 600 Pa -- but that
onset was read by ramping the supply up from perfect rest, a method found
ill-posed later the same day (it measures how long nothing takes to grow).
The refutation does not stand. Ricot, Caussé & Misdariis place the accordion
reed's excitation in exactly that channel (as summarised in Llanos-Vázquez's
thesis, Appendix 1), and milestone 2b takes it up again.

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

### The pallet (tested)

The key lifts a pallet off the tone hole. The air passes through the
curtain between them -- the rim's perimeter times the lift, never more than
the hole -- as an orifice with a Bernoulli jet, a resistance
R_p = ρ|u_h|/(2α²A_p²) in series with the hole's inertance, kept passive the
same way as the reed's jet. A closed pallet is a seal: the hole passes
nothing. The bellows holds its pressure whether or not a key is down; the
pallet is what lets air through, so a finger attack is the pallet opening on
a pressed bellows, as Llanos-Vázquez et al. describe it.

How far the pallet lifts (3 mm), how fast it closes (10 ms) and the hole's
shape (four times as long as wide) are assumed: nothing is published. It
opens in 50 ms, the time Llanos-Vázquez's thesis gives for a normal finger
attack (p164). A key held part-way holds the pallet part-way open
(`Engine::press`); no MIDI control is mapped to it yet.

**The bend emerges.** Nothing about pitch is written into the pallet. Held
part-way open at 300 Pa, it lowers the pitch monotonically -- −0.7 cents at
60 % of the lift, −4.7 at 30 %, −15.1 at 20 % (with a 10 ms opening;
−16.5 at 20 % with the 50 ms the thesis gives) -- and below that the reed
stops. Players bend 15-35 cents (Elejalde-García et al. 2021); the model's
−15 cents before silence is at the low end, and it never bends upward.

### Radiation

Outside, the reed is heard through the air the tone hole passes: a monopole
`p = ρ/(4π r) u_h'` at r = 1 m. Ziegenhals: the sound is the derivative of the
volume flow, and the tongue's own radiation is negligible. Ricot finds a
bare reed dipole-dominated because the flow leaving one face enters from the
other; in an accordion the other face is inside the bellows.

### The bellows intent (decided)

The player's intent reaches the engine as a fraction from 0 to 1: the latest
key's velocity until Expression (CC 11, 14 bits with CC 43) arrives, then
Expression for good. It asks `ceiling × intent^curve`, 1 kPa and 2 by
default (assumed: half the push is 250 Pa, inside normal play). That is the
push the arm makes, in pascals of a still bellows: the bellows (above)
turns it into the pressure the reeds see, or, "Stiff", it is that pressure.
A 1 ms smoothing keeps a stepped controller from reaching the bellows as a
step; it is numerical, not physics.

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
| Pallet lift | 3 mm | Assumed | Not published |
| Pallet opening | 50 ms | Reported | "Los ataques usuales de dedo son realizados en unos 0.05 s" (Llanos-Vázquez, thesis 2015, p164) |
| Pallet closing | 10 ms | Assumed | Not published |
| Tone hole shape | 4 : 1 rectangle | Assumed | Not published; sets the pallet's rim |
| Tremolo | 4.1 Hz at A4 (M+), lines' shape | Measured (one instrument), a style to voice by taste | A Borsini Super Star LMMMH (Hergert 2024, Fig. 6) |
| Registers | Roland FR-3x's 14 | Measured as a maker draws them | FR-3x Owner's Manual p. 27 |
| L, H geometry | F4 × slot ratios 1.27 / 0.74 (length), 1.25 / 0.79 (width) | Assumed | A bayan maker's slots, RU2233009 Table 3 |
| Compass | F3-A6, 41 keys | Decided | A full-size piano accordion |
| Pitch A4 | 440 Hz, equal temperament, at 300 Pa | Decided; measured reference | Richter, "Stimmung", IfM Zwota |
| Reed geometry across the compass | the F4 × a bayan maker's slot ratios | Assumed | RU2233009, Table 3 |
| Attack kick κ, its pressure scale | 1 × set, 20 Pa | Voiced, on Cottingham's observation; the scale assumed | Brings the finger attack to Llanos's 50-140 ms |
| Q slope | 0.7 | Measured trend, extrapolated above ~750 Hz | Cottingham 1998 (83 at 137 Hz), Förtsch 2021 (~233 at 598 Hz) |
| Cassotto | off; L and M when on | A choice of instrument | Pigini Sirius (Llanos p51); double cassotto (bassoon, clarinet) |
| Cassotto resonance | 900 Hz | Measured range 800-1000 Hz | Richter, IfM Zwota 1989 |
| Cassotto Q | 2 | Assumed, to be voiced | Unmeasured |
| Bellows response | Arm | Decided | The intent is the push (Stiff: the pressure) |
| Bellows area, volume | 600 cm², 12 L | Assumed, to be voiced | A full-size accordion's order; unmeasured |
| Arm speed v_max, Hill's k | 1 m/s, 0.25 | Assumed; k from Hill 1938 | No arm-on-bellows measurement |
| Bellows leak, air button | 10 mm², 400 mm² | Assumed | "Holds air > 30 s" (folklore) bounds the leak |
| Reversal time | 100 ms | Assumed | Not measured; "a slight interruption, like a bow change" (McMahan 2016; Llanos et al. 2002) |
| Swing limit κ | 0.5 | Voiced by ear, 2026-09-30 | Heard on a continuous swell of the F4 (A/B against κ 0, the bellows to 1 kPa): preferred. The one underived term; chosen so the swing settles near 5 mm from ~1 kPa (Ziegenhals: > 4 mm at mf; Braasch & Cottingham: ~15 % of the length) and holds with pressure (Cottingham, Lilly & Reed 1999) |

## What it does (0.2.0, measured; `milestone_1.rs` and `diagnosis.rs`)

| Behaviour | Model | Literature | |
| --- | --- | --- | --- |
| Onset from rest | 32 Pa | 10 Pa (Misdariis), 60-110 Pa (Cottingham, a 622 Hz reed), ~30 Pa (technicians) | Met |
| Offset, let down from 300 Pa | 8 Pa (22 Pa from 65 Pa) | Below onset, always (Cottingham ICA 2016) | Met |
| Tip swing | 3.7 mm at 300 Pa, 4.8 at 600 (κ 0.5; 4.9 at 300 with κ 0) | > 4 mm "at mf" (Ziegenhals, this reed; mf's pressure not stated) | Met if mf is above ~450 Pa |
| Tongue motion | 2nd-4th harmonics 59-72 dB down | Sinusoidal (Misdariis; Ziegenhals) | Met |
| Sound | Pulse train, harmonics within a few dB to the 7th | Pulse-like "Summton" (Ziegenhals) | Met |
| Level, 60 → 3000 Pa | 36.0 dB (40.9 with κ 0) | ~40 dB (Misdariis) | Met, at the low side |
| Pitch vs pressure, 100 → 900 Pa | −2.7 cents (−0.4 with κ 0) | about −9 cents (Cottingham) | **Not met** (a third, and by the voiced term) |
| Bend, pallet part-way open (300 Pa) | down to −16.0 cents before silence, never up | 15-35 cents (Elejalde-García 2021) | Met |
| Choking: cell resonance at 1.3, 1.0, 0.95, 0.9 of the reed | onset 35, 119, 217, 356 Pa | "far above normal" at or just below (Tonon; Cottingham ICA 2019) | Met in shape (see VALIDATION) |
| Closing the pallet | exact silence; −121 dB near the host's Nyquist | -- | Met |
| Bass-side ranks and their octaves | 16′ C2-B2, 8′ C3-B3, 8-4′ F♯3-F4, 4′ C4-B4, 2′ C5-B5 | Assumed | Wikipedia (unreferenced), agreeing with Roland's footages |
| Low reeds' tip loads | the least that lets a reed hold a tone at 50 Pa, 300 Pa and 1 kPa: 0.001-0.25 of the tongue's mass on the 16′, C2 the heaviest; none above B2 | Derived by search, on an assumed rule | Makers load low reeds (Llanos, Table 3.1, p222) |
| Finger attack, −50 → −5 dB | with the voiced start, F4 85 ms (100 Pa), 99 ms (400 Pa), F3-A5 53-106 ms, D♯6-A6 22-13 ms; as derived 249/730 ms | 50-110 ms mf, 60-140 ms p, no trend (Llanos 2014) | Met to A5, by a voiced term; too fast above |
| The start at the air's arrival, C4 | arm and key together 112 ms (was 303); key held, then the air 92 ms (was 212); through a reversal −5 dB ~240 ms after the turn begins (was ~410) | as the finger attack | Met |
| Swing against pressure | κ 0.5: 3.7, 4.8, 5.2, 5.5, 5.65 mm at 0.3, 0.6, 0.9, 1.5, 3 kPa (κ 0: 4.9 → 11 mm) | within 3 % from 0.3 to 1.2 kPa, 12 % lower at 2.9 kPa (Cottingham, Lilly & Reed 1999, Fig. 2) | Met from 0.6 kPa (+19 % to 3 kPa), by the voiced term; still climbing at 0.3 kPa |
| Tremolo beats at 300 Pa | M+ +3.667, M− −3.146, M−/M+ 6.813 Hz | asked 3.666, −3.143 (Borsini lines at 355 Hz) | Met |
| Ranks together, bellows stiff | Celeste +3.03, Musette +4.77, Master +6.32 dB over Clarinet, over 20 s (the beats averaged) | powers add | Met |
| One bellows, the arm pushing for 300 Pa | Clarinet 287.8 Pa (−4.1 %), Master 252.2 Pa (−15.9 %), +5.50 dB over Clarinet: the arm costs 0.82 dB | more reeds draw the pressure down (McMahan; no measurement) | Met (as predicted) |
| Tuning across the compass, at 300 Pa | all 205 reeds within ±2 cents (worst −0.01); M within ±15 cents at 100 and 600 Pa | in tune where a tuner tunes | Met |
| Thresholds across the compass | M: 13 Pa (F3) → 48 (C5) → 170 (A6); H at F♯7-A7 350-440 Pa | lower treble 40-70 Pa, top piccolos 100-250 Pa (a tuner) | Low ends met; the top 4′ too high |
| A four-note Master chord, native | 4.6 µs per sample, 22 % of a core; 5.3 µs with the bass side's keys walked too | -- | Measured; the Pi not yet |
| Both hands, native: a bass, a chord, a four-note Master | 7.5 µs per sample, 36 % of a core | under 12 µs | Met; the Pi not yet |
| The bass side, tuned at 300 Pa | all 60 reeds within ±2 cents (worst +0.00) | in tune where a tuner tunes | Met |
| The low reeds across the bellows' range | loaded, every reed below 300 Hz holds from 50 Pa to 1 kPa; none chokes at 1 kPa (unloaded, the 16′ below B2 choked from 400-800 Pa) | a reed speaks across a player's range | Met, by a load found as a maker finds it |
| 16′ C2 length | 64.6 mm | 52 mm at 62.5-87.5 Hz, loaded (Llanos, Table 3.1) | Not met |
| 16′ thresholds, C2-B2 | 16 → 8 Pa (54 → 9 before the ducts) | -- (predicted under 20) | Met |
| 16′ C2 finger attack at 400 Pa | 130 ms (467 in 8, 399 in 8c) | none measured below A2; A1 200-295 ms (Llanos thesis, Table 4.7) | Faster than A1 measured |
| Finger attacks against Llanos 2014, Table I, 400 Pa, 50 ms pallet | A2-B2 129-139 ms (151-179 before the ducts); A3-B4 82-102; A5-B5 39-52; A6 13 | 70-100; 60-110; 50; 100 | A2-B2 in range, slower than measured; the top too fast |
| The low reeds' inlet ducts | the least that lets each reed under 300 Hz speak on the bellows: the 16′ C2 ×9.06 (45 mm) down to B2 ×1.58; none above | an inlet duct favours the onset, a cavity adds friction (Fletcher & Rossing, after Llanos p236) | Derived by search, on an assumed rule |
| The bellows at audio frequencies | its air: the arm delivers the mean draw over 10 ms and holds Hill's mean pressure over 10 ms | the moving half cannot follow a reed's cycle | Assumed time scale, √(MC)/A for 4 kg |
| A free reed's growth, C3 | the model's 8′ C3 18.8/s at 400 Pa | 4.5/s at 0.3 kPa, 8.3/s at 0.5 kPa (Cottingham, Reed & Busha 1999, Fig. 4) | As fast as measured or faster |
| Cassotto, M at mf | centroid 1693 → 959 Hz (0.57); attack unchanged | 2013 → 1389 Hz (0.69, Llanos E4); attack unchanged (p174) | Met |
| Air button fully open, note held | −29 % pressure, −2.9 dB | the bellows moves without sounding | Weaker than predicted (assumed arm and vent) |
| L, H against M's octaves | −9.6, +2.4 cents | in tune, as a tuner sets them | L flat: set by its mode |
| Push against pull | the same samples, opposite polarity | attacks independent of direction (Llanos); push thresholds ×10 on an artificial chamber (Cottingham 2016, preliminary) | Met for the instrument; the rig's difference is out of reach |
| Bellows reversal, 400 Pa held | a gap of the turn + ~150 ms (250 ms at 100 ms), then the same level (+0.05 dB) | "a slight interruption" (McMahan; Llanos 2002); no gap is measured | Level met; gap long, by the slow attack |
| Level against mean flow, past 600 Pa | +4.2-4.7 dB per doubling (κ 0: 2.6-2.8) | ~7.6 dB per doubling (Nussbaumer & Agarwal 2016) | **Not met** |

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
  (8× softer gives −2.8 cents and swings of 10 mm). A candidate mechanism
  was found (ROADMAP 2b, second attempt). It is the steady suction of the
  upstream sink flow Misdariis et al. (CFA 2000) describe, on the face next
  to each gap. It adds no constant, and it gives −7.0 cents from 100 to
  900 Pa in small oscillations. It vanishes in the model's full ±5 mm
  swing, so it waits on the swing saturating and is not built yet. The failing
  prediction is kept, ignored, in `milestone_1.rs`.
* **The absolute level may be ~20 dB high.** At 50 cm the render measures
  75 dB (pp, ~100 Pa) and 91 dB (mf, ~400 Pa) unweighted, against
  Llanos-Vázquez et al.'s 55 and 70 dBA for p and mf. Their pressures are not
  known and the weighting differs, so this is a lead, not yet a measurement.
  Related: the reed spends 390 mL/s at 300 Pa and 1.3 L/s at 900 Pa, and
  nothing published says what an accordion reed spends.
* **The finger attack is three to five times too slow as derived; a
  voiced start brings it to a player's** (ROADMAP 7b; parameter "Attack
  Kick"). Not the growth: 7b read the radiated harmonic's dB/ms as the
  reed's growth, but the one direct measurement of a free reed's growth
  (Cottingham, Reed & Busha 1999, Fig. 4: a C3 reed, 4.5/s at 0.3 kPa,
  8.3/s at 0.5, 11.5/s at its 1 kPa peak) is no faster than the model's
  (ROADMAP 8c). What is missing is the start. Each reed starts
  κ·set·P/(P + 20 Pa) into its frame, P the side's pressure, at the
  moment the air reaches its cell -- its key down, its register open, the
  side's pressure above 20 Pa -- and again each time that air has gone and
  come back (the bellows stopped or turned), standing on Cottingham's observed "initial
  displacement of the reed tongue into the reed frame". F4 attacks in 85 ms
  at p and 99 at mf, the true 8′ in 53-106 ms from F3 to A5, too fast above
  (22 ms at D♯6). Tried and refuted before it: the second bending mode
  (attack unchanged, q₂ < 0.01 mm) and the cell's pressure on the tongue's
  face (slower). At κ 0 the start is as derived, and what follows describes
  it. As derived, what sets it is not what sets a real reed's. Measured on
  the model: the attack is
  exactly the equilibrium's exponential growth, 5.2/σ, from the kick the
  opening pallet gives the tongue -- its static deflection μP/ω0², 0.06 mm at
  400 Pa, −38 dB of the final swing. The growth rate itself is realistic:
  0.06 per cycle at 400 Pa against 0.086 per cycle, the maximum Cottingham
  & Reed measured on a reed-organ reed (Forum Acusticum 1999, Fig. 4), and
  organ reeds are the slow ones. But Llanos-Vázquez et al. measured finger
  attacks of 50-140 ms on notes from A2 to B6 with no trend in frequency --
  on A2 that would need 0.47 per cycle -- so a real finger attack cannot be
  growth from near zero: the tongue must start with a large excursion.
  Cottingham (ICA 2019): "the motion of this type of reed begins with an
  initial displacement of the reed tongue into the reed frame"; the second
  transverse and first torsional modes appear in the first 10-20 cycles and
  "may be significant in initiating reed oscillation". And in the model the
  reed's adjustment barely moves the attack (set 0.15-0.8 mm: 179-222 ms at
  400 Pa; clearances 0.015-0.06 mm: 175-186 ms), where Llanos attributes
  the 60-against-140 ms spread between neighbouring semitones to exactly
  that adjustment. Their numbers were read through ~80 ms analysis windows
  (their frequencies step by 12.5 Hz): through such a window an instant
  onset already reads 40 ms, so their finger attacks are near-instant; the
  model's, read the same way, are 260 and 750 ms. Tried and not the answer:
  more air inertia (a smaller tone hole) brings mf to ~110 ms but leaves p
  above 340 ms; Q from 95 to 1000 moves mf between 272 and 155 ms; the
  inertia of the air in the passages themselves, entered through its
  kinetic energy as Ricot's mechanism suggests, moves nothing (ROADMAP 2b),
  because growth from rest happens with the tongue above the plate; nor
  does the inertia of the upstream sink flow Misdariis et al. describe,
  whose logarithmic dependence on the gap does act there (σ 21.2 → 21.4 /s). The
  real reed grows ~4× faster at small amplitude (τ ≈ 11 ms against 48:
  Llanos-Vázquez, thesis, Fig. 4.11), and what supplies that is the force
  on the tongue's face about its rest position, whose coefficients only
  Ricot et al. 2005 gives.
* **The swing does not saturate by itself; a voiced term holds it**
  (above, "The swing limit"; ROADMAP 2c). Past the onset a measured reed's swing
  jumps to several millimetres and then stays nearly constant, falling at
  high pressure (Koopman & Cottingham 1997, via Cottingham et al. 1999); the
  model's keeps growing, and at an extreme corner of the parameters -- a
  65 mm reed at 6 kPa -- reaches 78 mm. St. Hilaire & Vaidya (J. Fluid Mech.
  67, 1975) attribute the limit to nonlinear dissipation by the flow's
  higher harmonics, which the minimal model does not have. The tongue has
  no mechanical stops either. Measured (`tests/saturation.rs`, the tongue
  swung by hand and the air's work per cycle against the tongue's own
  damping): the balance falls at 1.7, 5, 8.2 and 11 mm at 100, 300, 900 Pa
  and 3 kPa, the free reed's swings. Almost all the energy enters while the
  tip is above the plate, roughly in proportion to the pressure, and the
  only thing that takes it back is the tip emerging beyond the plate, past
  ~3.5 mm of travel. The one loss the model has, Q, does not grow with
  pressure or speed, so the swing grows until the tongue comes through.
  The voiced limit (κ 0.5) now holds it near 5 mm from ~1 kPa; with κ at
  0 the model is as derived and the defect is back. What the limit does
  not fix: the swing is still climbing at 0.3 kPa, where the measured
  reed had levelled off, and the level grows ~4.5 dB per doubling of flow
  past 600 Pa, against Nussbaumer & Agarwal's ~7.6.
* **One mode.** Accordion tongues carry their 2nd-4th bending modes and a
  torsional mode, most visibly in the attack (Behrens et al. 2009; Cottingham
  ICA 2019).

All three -- the pitch that does not sag, the slow attack indifferent to
adjustment, the swing that does not saturate -- point at the same place:
the aerodynamics of the minimal model. The one paper that models the
accordion reed's flow in detail, Ricot, Caussé & Misdariis (JASA 117, 2005),
is paywalled, as is the growth-rate study of Biernat & Cottingham (PoMA 20,
2014); Llanos-Vázquez's thesis (UPV/EHU 2015) is open behind a reCAPTCHA.

## What is deliberately not modelled yet

A MIDI control for a part-pressed key, the reed plate's second reed and its
valve, the bellows
direction, ranks and tremolo, the bellows reservoir, the cassotto, the body,
the rest of the compass and the Stradella bass. In order, with their
predictions: [ROADMAP.md](ROADMAP.md).
