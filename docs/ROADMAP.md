# Roadmap

Each milestone names the physics it adds, the source it takes it from, and the
predictions it must meet before anything is built on it. A prediction is
written before the code, so a wrong hypothesis is visible rather than tuned
away. Numbers with their sources are in [RESEARCH.md](RESEARCH.md); they are
asserted in the stated sense (a sign, an order of magnitude, a range), never
fitted.

## 0. Skeleton — done (0.1.0)

Workspace, `no_std` engine, SDK adapter, the bellows contract (velocity until
CC 11), state, laboratory (render, inspect, package, audition), CI, and a
package RackForge's own validators accept. Silent by design. Receipt in
[VALIDATION.md](VALIDATION.md).

## 1. One reed — done (0.2.0), one prediction not met

As planned: one 8′ reed at A4, the bellows opening, no chamber detail beyond
what the excitation itself needs. **As built, and why it changed:**

* **F4, not A4.** Ziegenhals (IfM Zwota 2009) published an accordion F4
  tongue's geometry and modes; no A4 reed's dimensions are published.
* **The cell came in.** Without its air the reed was silent below ~3 kPa;
  the cause was measured, not guessed (MODEL.md, "The cell is not optional").
  The pallet stays in milestone 2.
* **The tongue is profiled.** The measured 1 : 4.6 mode ratio contradicts a
  uniform tongue; the profile is derived from it.
* **Q is 250, not 95.** 95 was a brass harmonica reed's; 250 is the other
  measured free-reed Q (Woodhouse). Assumed either way.

Results, prediction by prediction, in [MODEL.md](MODEL.md) ("What it does")
and [VALIDATION.md](VALIDATION.md). Prediction 3 -- the pitch sagging with
pressure -- is not met, and is a known defect of this class of model.
Prediction 4 moves to milestone 2 with the cell's resonance. Prediction 6 is
measured natively (321 ns per host sample); wasm fuel and the Pi remain.

**Physics.** Millot & Baumann's minimal model (2007): a one-mode clamped reed,
the useful section through which the jet passes (with the reed's thickness and
clearance), and the upstream compliance and inertance that make a blown-closed
free reed self-oscillate (St. Hilaire 1971; Ricot 2005). Discretised so it is
passive by construction and solved without iteration, after Darabundit &
Scavone (2025) — adapting their single-reed scheme to a free reed is new work.

**Predictions** (as written before the code).

1. No parameter set any knob or program can reach makes the reed blow up or
   produce a non-number; with the supply off, its energy only falls.
2. It speaks above a threshold pressure and falls silent below a lower one
   (hysteresis), with thresholds of the order the literature reports for a
   mid-register reed: tens of pascals to about a hundred (Misdariis 2000;
   Cottingham ICA 2016).
3. It sounds slightly below its natural frequency, and its pitch falls as
   pressure rises: of the order of −9 cents over 0.1–0.9 kPa at ~420 Hz
   (Cottingham), −10 to −13 cents per kPa (Misdariis). Sign and order of
   magnitude are asserted; the exact slope is not a target.
4. Oscillation needs the upstream Helmholtz frequency below the reed's, and a
   chamber tuned near the reed chokes it (Millot & Baumann; Cottingham 2019).
5. Across its playing pressures the level spans roughly 40 dB (Misdariis).
6. Its cost per sample is measured in fuel on the wasm component, and the
   number is what decides whether an instrument of them fits the Pi 4.

**Voicing.** The reed's constants become parameters, as the Concert Grand's
knobs are: live while playing in RackForge, saved in programs, each marked
measured, derived or voiced in [MODEL.md](MODEL.md). A render for listening
is made once the predictions hold, with its lead-in.

## 2. The pallet, and what the cell does to high reeds

The key opens the tone hole: a variable orifice in series with the hole's
inertance, which milestone 1 treats as wide open. The bellows now holds its
pressure whether or not a key is down; the pallet is what lets air through.

**Physics.** The pallet lifts off the hole and the air passes through the
curtain between them, perimeter times lift, never more than the hole: an
orifice with a Bernoulli jet (as the reed's own gap), a resistance in series
with the hole's inertance, kept passive the same way. How fast the pallet
lifts and how high are not published: assumed, and bounded by the attack
prediction below.

**Predictions** (written before the code, 2026-09-30).

1. **Finger attack.** With the bellows already pressed, the pallet opens and
   the first harmonic goes from −50 to −5 dB in 50–140 ms (Llanos-Vázquez et
   al. 2014: mf 50–110 ms, p 60–140 ms). Asserted at 100 and 400 Pa.
2. **A partly open pallet bends the pitch down**, by 15–35 cents in the
   hands of players and up to about a semitone (Elejalde-García et al. 2021;
   Llanos 2008), and on a laboratory reed by up to 2.5 Hz of 96 (Cottingham
   2013). Asserted: closing the pallet part-way lowers the pitch by at least
   10 cents before the reed stops, never raises it.
3. **The cell chokes a reed tuned above its resonance.** In this model's
   order -- hole, cell, reed -- the reed feels the hole's air as inertia only
   below the cell's resonance. A resonance at or just below the reed's
   frequency raises the threshold "far above normal" or stops it; one just
   above helps it (Tonon 2005; Cottingham ICA 2019: a 344 Hz reed on a
   327 Hz chamber strongly affected, a 268 Hz reed on the same chamber not).
   Asserted by moving the cell's resonance across the F4 reed's 355 Hz: the
   onset at 0.9-1.0 of the reed's frequency is at least four times the onset
   at 1.3 of it.
4. The pallet closed, the reed stops and the output is exactly silent.

A *bellows attack* (190–660 ms, Llanos) is not a prediction here: its length
is how fast the player's arm builds pressure, which nothing measures.

**Status (2026-09-30): built; two met, one met in shape, one not.** The bend
emerges (−15.1 cents before silence, never up) and the closed pallet is exact
silence. Choking has the predicted shape, but the 4× this project wrote for
"far above normal" was 3.4× at exactly the reed's frequency. The finger
attack is 3-5× too slow, and investigating it found two more symptoms of the
same gap (MODEL.md, known defects). Numbers in [VALIDATION.md](VALIDATION.md).

## 2b. The aerodynamics the minimal model lacks

Three measured behaviours are out of reach of the one-mode, uniform-pressure
reed, and all three are aerodynamic: the pitch sags with pressure (about −9
cents over 0.1-0.9 kPa); the finger attack is near-instant, starts with the
tongue displaced into its frame, and depends on the reed's adjustment; the
swing saturates past the onset. The literature names the mechanisms but, in
what could be read, does not quantify them:

* the pressure *distribution* on the tongue from the unsteady flow into the
  gaps, rather than a uniform Δp -- Ricot, Caussé & Misdariis (JASA 117,
  2005), Misdariis et al. (CFA 2000: a 2-D unsteady potential flow with a
  point sink);
* nonlinear dissipation by the flow's higher harmonics, which limits the
  swing -- St. Hilaire & Vaidya (J. Fluid Mech. 67, 1975);
* the second transverse and first torsional modes in the first cycles --
  Cottingham (ICA 2019), Behrens et al. (JASA 126, 2009).

**What the thesis settled (2026-09-30).** Llanos-Vázquez's thesis, read in
full, gives no pressures or dimensions but three things: the real attack's
growth, ~0.7-0.8 dB/ms on the first harmonic of A4 mf (Fig. 4.11, p150; an
e-folding time near 11 ms, where the model's is 48 ms); a normal finger
attack executed in ~0.05 s (p164); and its Appendix 1 summary of Ricot,
Caussé & Misdariis -- the excitation is the inertia of the flow through the
slot of the plate itself, an end-correction mass M(t) that depends on the
aperture (eq. A1.8, p247), and energy enters with a pressure peak as the
tongue enters the slot, nearly in phase with its velocity (pp251-252). The
builders it interviews say a smaller clearance makes a faster response (p46).

**The mechanism to build.** An inertance that depends on where the tongue
is: the air in the narrow channel between each element of the tongue's edge
and the slot's wall, ρ ℓ/(α g) per unit of edge (Tarnopolsky's ρ d/(C F),
element by element), ℓ the length of that channel -- how much of the
tongue's thickness is inside the plate -- and g the local gap. Written
through its kinetic energy ½ M(ζ) u², it adds −½ M′(ζ) ζ′ u to the flow's
equation and a force ½ M′(ζ) u² on the tongue: as the tongue enters the
slot and the channel lengthens, the air's inertia pulls it on, in phase with
its velocity -- Ricot's peak. Nothing in it is a new constant; it is the
geometry the model already has.

**Predictions** (written before building it):

1. The first harmonic's growth in a finger attack at 400 Pa is at least
   twice as fast as now (e-folding ≤ 24 ms, from 48), toward the measured
   ~11 ms.
2. The attack becomes sensitive to the clearance, faster as it narrows (the
   builders' rule, p46): at least 1.5× between 0.06 and 0.015 mm, where now
   it moves 6 %.
3. Milestones 1 and 2 stay met: onset 10-110 Pa, offset below it, swing
   > 3.1 mm at 300 Pa, the bend, choking, passivity, the scheme computing
   the model.
4. Watched, not asserted: whether the pitch now sags with pressure and the
   swing saturates.

**Result (2026-09-30): refuted as formulated, and what that leaves.** Built
as an experiment -- RK4 on the exact Lagrangian equations, κ 0.5-2
(`tests/passage_inertia.rs`) -- the passage inertia moves nothing:
σ 21.2 → 21.1 /s at 400 Pa, the attack 193 ms either way, the clearance's
effect unchanged. The reason is in its own numbers: the passage inertia
rises from 26 to 124 kg/m⁴ only once the tongue enters the slot, and growth
from rest happens with the tongue oscillating above the plate, where it is
constant. Resting the tongue nearer the plate makes growth slower (σ 19.3 at
0.2 mm, 10.6 at 0.1 mm), not faster. So the real reed's ~4× faster
small-amplitude growth (τ ≈ 11 ms, Llanos Fig. 4.11) needs a larger
negative damping *about the rest position*: what Ricot's force on the
tongue's upstream face (eq. A1.7, as the thesis transcribes it) carries in
its coefficients A₁, A₂, A₃ and K₀ -- which only Ricot et al. 2005 gives.
The tongue is not the culprit: the thesis's reed 2 (Table 3.1, 0.29 g,
37 mm, 226 Hz) scaled to 355 Hz has the tip stiffness the model derives,
~348 N/m against 347.

**Second attempt: the sink flow of Misdariis, Ricot & Caussé (CFA 2000).**
Read in full (2026-09-30), the paper publishes no equation and no value of
K₀, but it describes the flow precisely enough to derive from: upstream, a
laminar two-dimensional *sink* into the gap, in planes across the tongue;
pressure acts on the upstream face only, the downstream side is "dead"
fluid; the excitation is the inertia of the upstream fluid. A half-plane
sink of strength q per unit edge length, φ = (q/π) ln r, puts on the face
next to each gap g, out to the tongue's half-width w/2:

* a steady Bernoulli deficit ½ρ|∇φ|², which integrates to
  (2/π²) Δp g (1 − g/w) per unit edge length (Δp the jet's ½ρv²): it grows
  with the gap, so it is a negative stiffness proportional to pressure --
  a pitch that falls linearly with pressure, which is what their Fig. 3
  shows (−13.6 cents/kPa, linear to 3.4 kPa, as digitised here). Estimated
  for the F4 reed with the tongue above the plate: about −9 cents/kPa.
  Call it (A).
* an unsteady term ρ ∂φ/∂t: the sink's inertance, ρ ln(w/g)/(π dx) per
  edge element, between the gap and the half-width where the near-field
  inertance M_n takes over. Unlike the refuted passage inertia, whose end
  correction κg made it constant above the plate, it depends on the gap
  there, logarithmically. Call it (B).

Neither adds a constant: both are the geometry the model has. They are our
derivation from the paper's description, not their model, which is
unpublished (Ricot 1999, a thesis of the École Centrale de Lyon; Ricot et
al. 2005).

**Predictions** (written before building it, 2026-09-30):

1. (A) makes the pitch fall with pressure, by −2 to −40 cents from 100 to
   900 Pa -- milestone 1's prediction 3, unmet since (now −0.4).
2. (B) makes the growth at 400 Pa at least twice as fast (e-folding
   ≤ 24 ms, from 48).
3. Milestones 1 and 2 stay met, as in the first attempt.

**Result (2026-09-30): (B) refuted; (A) right in the small, masked in the
large.** Built as an experiment (`tests/sink_flow.rs`, RK4 on the exact
equations, as the first):

* **(B)** is 15-24 kg/m⁴ across the swing above the plate, beside
  M_n = 47.6, and moves nothing: σ(400) 21.2 → 21.4 /s, attacks 553/193 →
  545/190 ms. Prediction 2 fails.
* **(A)** is what the paper's Fig. 3 asks for, where it can act.
  * About the equilibrium, in small oscillations, it takes the pitch from
    flat (+0.01 cents from 100 to 900 Pa) to **−7.0 cents** (−7.6, −9.4,
    −14.6 cents from the mode at 100, 300, 900 Pa). That is −8.7 cents/kPa,
    the order of Cottingham's (≈ −11) and Misdariis's (−13.6), with no
    constant added.
  * In the full oscillation the effect vanishes (sag −0.5 → 0.0 cents),
    because the model's tongue swings ±5 mm and spends half of every cycle
    inside the slot, where the gap is the clearance and (A) is constant.
  * Prediction 1 is met in the regime a real reed plays in -- the swing
    saturates past onset (Koopman & Cottingham), keeping the tongue near the
    plate -- and not met in the model's own large swing.

So the pitch sag is not missing physics any more: it waits on the swing
saturating, the third defect. **Next:** the swing's limit (St. Hilaire &
Vaidya 1975, nonlinear dissipation by the flow's higher harmonics), then
(A) again on top of it. The attack remains Ricot et al. 2005's question.

## 2c. The swing's limit

**Measured first** (`tests/saturation.rs`, MODEL.md): the air feeds the
tongue while its tip is above the plate, about in proportion to the
pressure; the tongue's only loss, Q, does not grow with pressure or speed;
only the tip coming through the plate stops the growth. So the swing grows
with pressure: 1.7, 5, 8.2, 11 mm at 0.1, 0.3, 0.9, 3 kPa. Measured reeds
swing about 15 % of their length, ~5.4 mm here (Braasch & Cottingham,
*Acoustics Today* 2023), jump to it past the onset and hold it (Cottingham
et al. 1999).

**First candidate: the air's drag on the moving tongue.** At 5 mm the tip
moves at ~11 m/s, in the order of the jet's speed, and nothing in the model
resists it. An oscillating flat plate's drag is measured: Keulegan &
Carpenter (J. Res. NBS 60, 1958, Table 4) give C_d against the
Keulegan–Carpenter number KC = 2πA/w, 5.2-5.5 near this reed's KC ≈ 8.
Taken along the tongue with the local KC as Bidkar et al. do for
cantilevers (JFM 634, 2009, eq. 3.10), ½ ρ C_d(KC(x)) w v|v| per unit
length, only where the element is out of the slot (inside it the air is
confined, and the pumped flow already carries it). No constant is voiced.

**Predictions** (written before building it, 2026-09-30):

1. The drag lowers the swing at every pressure and more at high pressure,
   but does not make it nearly constant: estimated by hand ~3 mm at 300 Pa,
   ~6 at 900, ~8.5 at 3 kPa.
2. If so, drag alone is not what limits a real reed, and at 300 Pa it would
   take the swing below Ziegenhals's "more than 4 mm at mf" -- unless mf is
   well above 300 Pa, which nothing measured says.

**Result (2026-09-30): both met, so the drag is not the limit.** In the
energy balance (`tests/saturation.rs`) the drag takes about as much as the
tongue's own Q at every swing (85 µJ against 108 at 5 mm), and moves the
balance to 1.1, 3.4, 6.4 and 9.1 mm at 0.1, 0.3, 0.9 and 3 kPa. It is real
physics, but the swing still grows with pressure, relatively more
(×2.7 from 0.3 to 3 kPa, against ×2.2), and at 300 Pa it falls below
Ziegenhals's 4 mm. It is not built.

What the balance says instead: a swing that holds with pressure needs a
loss that grows with pressure as the feed does -- an aerodynamic loss, not
a mechanical one -- and a swing pinned at ~15 % of the tongue's length
suggests geometry pins it. In the model the only aerodynamic loss is the tip
coming through the plate, past 3.5 mm of travel (set 0.5 + plate 3 +
thickness), and it bites only from ~6 mm. What the air does to a tongue
emerging from the slot's far side is where to look next. Tarnopolsky et al.
saw their flap sit in the aperture nearly half of each cycle and emerge
from the back above ~5 mm; the builders in Llanos's thesis say that with a
thin plate the tongue "passes straight to the other side" (p270). St.
Hilaire & Vaidya (1975) put the limit in the potential flow's higher
harmonics, but only their abstract has been read.

**A measured target, and a turn (2026-09-30).** Nussbaumer & Agarwal (ICA
2016, read in full) filmed and measured real reeds.
* Their reed 1 (236 Hz, 21 l/min) swings ±0.72 mm and enters the slot
  0.4 mm below its mean: it never comes through the plate. So the far side
  of the plate cannot be what limits that reed, and the path above is
  contradicted.
* Its mean moves toward the plate as the flow rises, with a growing share
  of each cycle inside the slot.
* Their Fig. 6 gives what can be compared without knowing the reed:
  * the fundamental's level against the mean flow: reed 2 +10.5 dB from 20
    to 52 l/min, ~7.6 dB per doubling;
  * the second harmonic stays below the fundamental and rises faster.

The model, measured the same way (`level_against_flow`):
* +9 to +7 dB per doubling up to 150 Pa, then +4.3 at 300 Pa, +2.6 at
  900 Pa, +1.0 at 3 kPa;
* the 2nd and 3rd harmonics louder than the fundamental from 600 Pa.

Its mean flow grows with its swing, because the gap above the plate opens
with it, and the level runs out. The same defect, from the sound's side.

**Nor does the mean moving toward the plate pin it**
(`a_mean_moved_toward_the_plate`, the balance with the mean set by hand).
Moving the mean 0.4 mm toward the plate lowers the settled swing by at most
0.5 mm at any pressure: 4.97 → 4.44 mm at 300 Pa, 11.6 → 11.0 at 3 kPa. With
the tip resting at the plate the reed no longer starts from small swings.

**The linear base is sound (2026-09-30).** Cottingham, Reed & Busha
(Forum Acusticum 1999, Fig. 4, digitised here) measured a reed-organ C3's
small-amplitude growth against pressure:
* damping below ~0.05 kPa;
* 2.0, 3.4, 4.5, 4.6, 8.3 /s at 0.07, 0.09, ~0.2, 0.27, 0.52 kPa;
* a maximum of **11.5 /s at 1.0 kPa**;
* then 10.3, 9.1, 6.9, 3.3 /s at 1.5, 2.0, 2.5, 3.0 kPa.

The model's (`growth_against_pressure`) has the same shape:
* 0.4, 7.0, 21.2, 36.6 /s at 35, 100, 400, 1000 Pa;
* a maximum of 46.5 /s near 2 kPa, 39.1 at 3 kPa;
* none at 4.5 kPa, where the tongue's static deflection has brought it to
  the plate.

Normalised by each reed's own damping, the peaks are 7.8 and 11.4. The
reeds differ, so this is a match of shape, not of numbers. So the
small-amplitude excitation is not the defect: the swing's limit is purely
nonlinear, as St. Hilaire & Vaidya say.

**The measured swing against pressure (2026-09-30).** Cottingham, Lilly
& Reed ("The motion of air-driven free reeds", Forum Acusticum 1999,
Fig. 2, read in full and digitised here, ±0.03 mm) measured an A♯ organ
reed (118 Hz, 4 cm tongue) at mid-tongue, 2 cm from the tip:

| P (kPa) | 0.03 | 0.31 | 0.60 | 0.89 | 1.19 | 1.95 | 2.44 | 2.94 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Half swing (mm) | 0.73 | 1.17 | 1.20 | 1.17 | 1.21 | 1.10 | 1.04 | 1.03 |
| Mean (mm) | +0.07 | 0.00 | −0.08 | −0.15 | −0.24 | −0.39 | −0.50 | −0.64 |

* From 0.3 to 1.2 kPa the swing holds within 3 %, and by 2.9 kPa, ten times
  the pressure, it has fallen 12 %.
* At the tip that is ~3.4 mm (mode shape 0.34 at mid-length).
* The mean drifts with the flow, ~0.24 mm/kPa at mid-tongue.
* The model's swing grows 2.2× from 0.3 to 3 kPa.

This is 2c's quantitative target: **a swing that, past the onset, changes
by less than ~15 % over a tenfold rise in pressure**.

**What feeds the swing, exactly (`which_term_feeds_it`).**
* The air's work splits 70 : 30, at every swing and pressure, between the
  cell's pressure and the near field's inertia. Below the cell's resonance
  (~2 kHz) both are one thing: the inertia upstream of the gap, hole
  (~110 kg/m⁴) and near field (47.6) in series, times the rate of change
  of the flow through the gap.
* Quasi-steadily, that flow is α S(ζ) v, so the work per cycle is about
  −S_r M_up α v ∮ S′(ζ) ζ′² dt.
* While the tongue is above the plate the gap's area grows in proportion
  to the lift (S′ constant, ~32 mm²/mm here). The feed therefore grows as
  A², exactly as the tongue's damping does, and nothing limits the swing
  until the geometry changes: the area capped at the slot's, or the tip
  emerging from the plate.

**So the derivation has a precise target:** how the escape area -- the
flow a lifted tongue lets through, at a given pressure -- grows with the
lift once the lift is no longer small against the tongue's width and the
slot. A swing that holds over a tenfold rise in pressure needs S′ to fall
off at a lift of the order of the measured swings (~3.4 mm at the tip of a
4 cm organ reed, more than 4 mm for this F4). This is static, measurable
flow physics, the same for any pressure -- which is exactly why it would
pin the swing independently of pressure. The model now takes the
area as the lift times the edges, hard-capped at the slot's area.

**What valve engineering measured.** Compressor reed and plate valves --
a plate lifted over a port -- are the same flow, and their literature
measured it:
* Ferreira & Driessen, ICEC 1986, circular port, measured;
* Lin, Pan & Shu, ICEC 1994, slot passages, measured;
* Tsui, Oliver & Cohen, ICEC 1972, a 2-D slot valve in water;
* Schwerzler & Hamilton, ICEC 1972, analytical against measurement;
* Park et al., *Energies* 16, 2023, 380 CFD cases.

All agree on the form. The effective area is the curtain (lift × edges)
and the port in series, A_eff = 1/√(1/(C_c A_curtain)² + 1/(C_p A_port)²),
with C_c 0.59-0.70 and C_p 0.70-0.85. It saturates gradually: where the
curtain equals the port, A_eff is only ~0.45-0.5 of the port, and ~0.7-0.85
of it only once the curtain is 1.5-2× the port. The model's hard cap at the
slot's area is the corner this replaces. Taken here with C_c = 0.61 (the
model's contraction; Tsui's free-streamline value is 0.611) and C_p = 0.707
(Park's fit), small openings are unchanged, so the onset, growth and bend
are untouched by construction.

**Predictions** (written before building it, 2026-09-30), on the energy
balance:
1. The swing settles lower at every pressure, ~3.5-4.5 mm at 300 Pa (the
   area's slope is already down ~20 % at 2 mm of lift and ~50 % at 4).
2. Alone it does not meet the target: estimated ~7-9 mm at 3 kPa, a
   swing still ~2× that at 0.3 kPa, because the feed at 3 kPa outruns the
   damping ~8× at small swings and the area's slope falls that far only
   past ~9 mm of lift.

**Result (2026-09-30): the area law is right, and is not the limit.**
Measured by the energy balance (`the_series_area_against_the_cap`):

| Escape area | 0.1 kPa | 0.3 | 0.6 | 0.9 | 1.5 | 3 kPa |
| --- | --- | --- | --- | --- | --- | --- |
| Cap (shipping) | 1.58 mm | 4.93 | 6.99 | 8.12 | 9.51 | 10.9 |
| Series, C_p 0.707 | 1.55 | 4.72 | 6.78 | 7.98 | 9.60 | > 12 |
| Series, C_p 0.85 | 1.56 | 4.79 | 6.92 | 8.19 | 9.96 | > 12 |

* Prediction 1 failed: the swing barely moves (4.93 → 4.72 mm at 300 Pa).
* Prediction 2 held: it does not meet the target.

The reason is in the balance. Past ~3.7 mm of travel (set 0.5 + plate 3 +
tip 0.23) the tip comes through the plate, and what then takes the energy
back is the same mechanism as the feed above it -- upstream inertia times a
gap area opening, now on the far side. The series law shrinks both: less
feed above, less loss beyond. In this model's class the swing settles
where the loss beyond nearly balances the feed above. The feed outruns the
damping more as the pressure rises, so that balance needs ever more travel
beyond the plate, and the swing grows with pressure.

So what pins a real reed's swing must make the far side, or the
large-lift side, act differently from the near side. That is not an area,
and not anything in the quasi-steady, lumped picture. It is where St.
Hilaire & Vaidya's finite-amplitude potential flow would have to come in.
The series area is kept as the better-measured law, to be built when
something else limits the swing.

**Decision (2026-09-30): a voiced swing limit, until the mechanism is
derived.** A damping on the tongue that grows with its displacement and
scales with the flow, as the feed does:
* c = κ ρ v w L (ζ/w)², with v = √(2p/ρ) the jet speed the cell's
  pressure gives, w and L the tongue's width and length, and ζ the tip's
  displacement from rest;
* one voiced constant, κ, a live knob ("Swing Limit");
* zero at small swings, so the onset, growth, attack and bend stand as
  derived;
* in proportion to v, so the swing it settles at hardly depends on the
  pressure, which is what Cottingham, Lilly & Reed measured;
* dissipative, so the scheme stays passive (an extra −c ζ′² in the energy
  identity).

It stands in for St. Hilaire & Vaidya's nonlinear dissipation, and says so
in MODEL.md: it is the one place where the model is voiced rather than
derived.

On the energy balance (`the_voiced_limit`), κ 0.5 settles at 3.7, 4.8,
5.2, 5.5, 5.6 mm at 0.3, 0.6, 1, 1.5, 3 kPa: within the "more than 4 mm at
mf" Ziegenhals measured on this reed and the ~15 % of the length (5.4 mm)
Braasch & Cottingham report. It is therefore the default; the ear sets it
after.

**Predictions** (written before building it into the step, 2026-09-30):
1. The free reed settles where the balance says, within 10 %, at 0.3, 1
   and 3 kPa.
2. From 0.6 to 3 kPa the swing changes by less than 20 %. Not met from
   0.3 kPa: the balance still climbs there, where the measured reed had
   levelled off.
3. Milestones 1 and 2 stay met: onset, offset, growth, the bend,
   choking, silence, passivity, and the scheme against RK4.
4. The fundamental's level against flow climbs ≥ 5 dB per doubling above
   600 Pa (now 2.6-2.8), and the fundamental stays the strongest component
   to 3 kPa (now overtaken at 600 Pa).
5. Watched, not asserted: whether the pitch now sags with pressure.

**Result (2026-09-30), the free reed with κ 0.5 (`level_against_flow`,
the shipping step):**

| Supply | 100 Pa | 300 | 600 | 900 | 1500 | 3000 |
| --- | --- | --- | --- | --- | --- | --- |
| Swing, mm | 1.51 | 3.72 | 4.76 | 5.16 | 5.50 | 5.65 |
| Before (κ 0) | 1.58 | 4.93 | 6.99 | 8.17 | 9.59 | 11.1 |
| dB per doubling of flow | +8.7 | +5.2 | +4.2 | +4.2 | +4.5 | +4.7 |
| 2nd harmonic against the fundamental | −12.3 | −7.4 | −3.5 | −1.8 | −0.2 | +1.6 |

1. **Met.** The free reed settles where the balance said, within 1 %.
2. **Met, barely.** +19 % from 0.6 to 3 kPa.
3. **Met.** Every milestone test passes, the scheme against RK4 included
   (the reference integrator carries the term too).
4. **Not met, improved.** The level climbs 4.2-4.7 dB per doubling past
   600 Pa (from 2.6-2.8), short of 5 and of Nussbaumer's ~7.6. The second
   harmonic now overtakes the fundamental only at 3 kPa (from 600 Pa).
   With the swing held, what still grows is the flow. The mean flow at
   3 kPa is 138 l/min, and nothing measured says what an accordion reed
   spends.
5. **Watched.** The pitch now falls 2.7 cents from 100 to 900 Pa (from
   0.4), inside milestone 1's bound of −2 to −40 but a third of the
   measured ~−9, and by the limit's own action rather than by a derived
   mechanism. The known defect stands; the sink suction (A) can now be
   re-tried on the held swing.

**Where 2c stands.** Tried and not the limit: the tongue's drag in the air,
the tip coming through the plate (a measured reed never does), and the mean
moving toward the plate. What is left is the one thing all three leave
untouched: the energy the air feeds in above the plate grows about in
proportion to the pressure at any swing. What would make it level off is in
the two works not yet read:
* St. Hilaire & Vaidya (*J. Fluid Mech.* 67, 1975, 377-396): the limit from
  the potential flow's higher harmonics;
* Ricot et al. (*JASA* 117, 2005).

A lead from builders, unverified: "the best excitation is when the minimum
depth of the inlet chamber slightly exceeds the maximum tip amplitude" (a
Soviet textbook reprinted at poigarmonika.ru). If so, the chamber's wall
would pin the swing on an instrument, but not on the open laboratory rigs
where reeds also saturate.

**Open from milestone 1.** The absolute level and the air the reed spends
(MODEL.md, known defects) want a measurement before anything moves them; the
pitch-pressure mechanism wants Ricot et al. 2005 or Misdariis's potential-flow
model.

## 3. The plate pair: push and pull

Each note has one reed per bellows direction, the idle one closed by a valve —
leather to G4, plastic to C6, none above on the instrument measured (Llanos).
The top octave, with no valves, cannot bend. Bellows reversal: the other reed
takes over and the valves flip.

**Decided (2026-09-30): how direction reaches the plugin.** No MIDI
accordion found sends it:
* the Roland FR-1x senses the bellows opening and closing but sends only
  CC 11;
* the FR-8x and FR-4x MIDI implementation charts carry no direction message
  (read in the survey, SYNTHESIS-AND-PRIOR-ART);
* Brendan Vavra's open MIDI accordion sends only CC 11;
* only Bandolibre, an open bandoneon, sends a direction, by SysEx, under a
  non-commercial licence: a reference, not a standard.

So the direction is a parameter, "Bellows Direction" (Pull / Push), live
and automatable. A switch CC moves it too: below 64 pull, 64 and above
push, on CC 80 (General Purpose 5, defined as a switch and otherwise
unused). CC 11 (and CC 43) stays the pressure, unchanged.

**Which reed is which.** The plate sits between the bellows and the cell,
and the cell opens to the tone hole and its pallet:
* **Pull** (the bellows opening): air goes outside → pallet → hole → cell →
  plate → bellows, so the sounding reed is the one inside the cell, with
  the cell **upstream**. That is the reed built so far.
* **Push** (the bellows closing): air goes bellows → plate → cell → hole →
  pallet → outside, so the sounding reed is the one on the bellows side,
  with the cell **downstream** and only the bellows' open space upstream.
* **The valves.** Each valve lies over its reed's slot on the far face of
  the plate. It seals that slot when its reed would be blown the wrong
  way, and its own reed's jet must lift it when that reed sounds.

**What is measured of the push side.** Cottingham (ICA 2016, "Reed chamber
resonances and attack transients", read in full) blew a Hohner Verdi I
plate (622 Hz) under an artificial Helmholtz chamber, both ways:
* **Flow through the chamber, then the plate** -- our pull: thresholds
  0.06-0.11 kPa, highest when the chamber's resonance is near the reed's.
* **Flow through the plate, then the chamber** -- our push: thresholds
  **0.8-1.7 kPa**, ten times higher, and **lowest** near the reed's
  frequency, the opposite trend Tonon predicted.

Milestone 1 found the same order in this model: without the cell's air
upstream, the reed was silent below ~3 kPa.

**Predictions** (written before building it, 2026-09-30):
1. The pull reed is unchanged: milestones 1, 2 and 2c hold.
2. The push reed, with only the near field upstream and the cell
   downstream, starts from rest only at the order of a kilopascal
   (0.5-5 kPa), ten times its pull partner or more.
3. Its threshold falls as the downstream cell's resonance approaches the
   reed's frequency, the opposite of the pull reed (Cottingham 2016;
   Tonon).
4. If 2 holds, the push reed cannot speak at normal playing pressure (to
   ~300 Pa), and a real accordion's does (Llanos: attacks do not depend on
   the bellows' direction). Then something upstream of the push reed on an
   instrument -- the narrow space between the reed blocks, the bellows'
   own air -- supplies the inertia the open rig lacks. That would be the
   next thing to find, not something to tune.

**Result (2026-09-30): 2 and 3 refuted, and why it had to be so.**
Measured by RK4 (`tests/push_reed.rs`), the push reed's threshold equals
the pull reed's to the pascal, whatever the cell:
* 32 Pa with the default cell;
* 36, 53, 119, 217, 358, 761 Pa as the cell's resonance moves to 1.3,
  1.1, 1.0, 0.95, 0.9, 0.8 of the reed's frequency.

This is not a slip but a property of the class of model. The reed is one
element in a single series loop with the bellows; what it feels is the
loop's total impedance, near field + (hole ∥ cell), and a sum does not
depend on its order.

No lumped model can tell push from pull. Cottingham's tenfold difference
must come from what the loop leaves out:
* where the tongue sits relative to the cell (inside it on pull, outside
  on push);
* the jet discharging into a small cavity rather than open space;
* the near field's own geometry on each side.

Tonon's own account is spatial ("the effect depends on where the reed sits
in the cavity").

**What follows for the instrument.** On a real accordion, Llanos measured
that the attack does not depend on the bellows' direction (thesis, p151,
p219). Cottingham's figures are his own "preliminary" ones, on an
artificial chamber, at pressures far above playing. So two identical
reeds, one per direction, are what the measurements of the instrument
support. The push side's own physics waits for a measurement taken on an
accordion. Milestone 3 then is:
* the second reed;
* the direction (the parameter and CC 80);
* the valves, which are measured only qualitatively (no study of valve
  dynamics exists, INSTRUMENT-ACOUSTICS §2.6);
* the reversal.

**As built.**
* **Two reeds.** Each has its own state and the same design, behind the
  key's one pallet.
* **The bellows' pressure is signed.** Pull below zero, push above. Each
  reed is blown by its own side and sees nothing from the other: its
  valve is shut, taken as an ideal seal (F4 has a leather valve).
* **The cell is one cell on an instrument.** Here each reed keeps its own
  copy, which matters only while both still move, during a reversal.
* **A reversal.** Turning the direction takes the signed pressure through
  zero over "Reversal Time": the arm stopping and turning. It is assumed,
  100 ms: nothing measures it, and players describe "a slight
  interruption, like a bow change" (McMahan 2016; Llanos et al. 2002).
  When the bellows controller itself passes through zero, as a digital
  accordion's does, the ramp hides inside its dip.
* **Direction.** The `bellows_direction` parameter (0 pull, 1 push,
  default pull: the reed milestones 1-2 built), or CC 80.

**More predictions** (written before building it):
5. With the direction held, the other reed stays exactly at rest.
6. A reversal at constant intent leaves a gap in the sound no longer than
   the reversal time, and the new reed returns to the old reed's level
   within 0.5 dB.

**Status (2026-09-30): built (0.4.0); five met, one half met**
(`tests/milestone_3.rs`, through the engine).
* **Met:**
  * the pull reed is unchanged (every earlier test passes);
  * the push reed is its exact twin: the same samples, of opposite
    polarity, because the hole's flow is reversed;
  * the other reed stays exactly at rest;
  * after a reversal the new reed comes back to the old one's level
    (+0.05 dB).
* **Not met:** the gap. It is about the turn plus ~150 ms (130, 190, 250,
  410 ms for turns of 20, 50, 100, 200 ms), because the new reed starts
  from rest and grows at the model's small-amplitude rate -- the slow
  finger attack again, a known defect. Only a 5 ms turn leaves none, the
  old reed still ringing down in its cell while the new one grows.

The flap of the valves on a reversal, and the leak where a note has no
valve, wait: the first for a measurement, the second for the notes above
C6 (milestone 7).

## 4. Ranks, registers and the musette

L (16′), M (8′), M+ and M− (tremolo), H (4′), and register switches. The
tremolo curve across the compass is voiced by ear inside its published bounds:
0.5–7 Hz at A4 from dry to wet, doubling roughly every 1.5 octaves, capped at
10–15 Hz (Hergert 2023/2024). The beat comes from separate reeds, never an LFO.

**Plan (2026-09-30).** Key 65 gets five ranks, each its own plate of two
reeds (milestone 3), each with its own cell:
* **M (8′):** the measured F4.
* **M− and M+ (8′):** the same tongue retuned flat and sharp. A tuner
  files the tip or the root, which moves the frequency and little else.
  So these are the F4's geometry at a detuned frequency, the profile
  re-derived for it.
* **L (16′) and H (4′):** the reeds sounding F3 and F5. Their geometry is
  what the literature must give (in research); they wait for it.

The rest of the plan:
* **The tremolo.** "Tremolo" is the beat between M and M+ at A4, in Hz:
  a choice of style, so voiced by taste within Hergert's 0.5-7 Hz at A4,
  "dry" to "wet". It is carried to each note by Hergert's recommendation,
  doubling every 1.5 octaves, beat(f) = t (f/440)^(2/3). The musette is
  flat, true and sharp at −Δf, 0, +Δf (Hergert: pair beats Δf and 2Δf).
* **Registers** select ranks, as the switches open and close the reed
  blocks. Their names and combinations wait for a manufacturer's table (in
  research).
* **The bellows.** Every reed still sees the bellows' full pressure; one
  bellows drained by every reed is milestone 5.

**What the research found (2026-09-30), and the plan revised before any
test ran:**
* **A measured tremolo.** Hergert (*Acta Acustica* 8, 2024, 33) was read
  in full from the Internet Archive's copy of the open HTML, its figures
  read off. A Borsini Super Star LMMMH, five voices like this instrument,
  has the builder's lines:
  * M+ at +4.1 Hz at A4, +1.4 Hz per octave;
  * M− at −3.7 Hz at A4, −1.8 Hz per octave;
  * so asymmetric, and straight on a log-pitch axis.

  The tremolo parameter is therefore M+'s beat at A4 (default 4.1). Both
  lines are scaled to it with their measured shape:
  * M+ = t (1 + 0.341 log₂(f/440));
  * M− = −t (0.902 + 0.439 log₂(f/440)).

  At this M (355 Hz) that is +3.67 and −3.14 Hz. It replaces Hergert's
  recommendation, which is a recommendation, not a measurement.
* **Registers.** Roland's FR-3x manual (p. 27) gives 14 treble registers
  with their reeds drawn as dots: Bassoon L, Bandoneon LM, Cello L M M+,
  Harmonium LMH, Organ LH, Accordion L M− M H, Master L M− M M+ H, Tremolo
  M− M+, Musette M− M M+, Violin M M+ H, Oboe MH, Clarinet M, Celeste
  M M+, Piccolo H. Taken as drawn, Accordion and Violin included. Clarinet
  (M alone) is the default, the instrument until now.
* **L and H.** No maker publishes tongue dimensions. A bayan maker's patent
  (RU2233009, Table 3) gives the slot per note, F3 35.4 mm, F4 27.8, F5
  20.5, with widths and plate thicknesses. Its F4 is shorter than
  Ziegenhals's, so makers scale differently. The patent's octave ratios
  are applied to the measured F4, and that is **assumed**:
  * L (F3, 177.5 Hz): length ×1.27, width ×1.25, set ×1.27;
  * H (F5, 710 Hz): length ×0.74, width ×0.79, set ×0.74, plate
    ×2.2/2.7.

  The profile is re-derived for each frequency; the cell is kept the
  same. Whether a real 16′ reed carries a tip weight is not settled.

**Predictions** (written before building it, revised with the above
before any test ran):
1. M alone is unchanged: every earlier milestone's test passes.
2. M and M+ beat at 3.67 Hz, M and M− at 3.14 Hz, within 5 %, at 300 Pa.
   The reeds' sounding frequencies keep their modes' detuning, all being
   ~6.5 cents below their modes.
3. In the musette, M− and M+ beat at the sum, 6.81 Hz, within 5 %.
4. Two ranks sound ~3 dB above one and three ~4.8 dB, within 1 dB, on the
   long-term level: incoherent reeds add their powers, and nothing shares
   the air yet.
5. L and H speak at 300 Pa, an octave below and above M within 15 cents.
   Each sounds below its own mode like M, by about as much: the minimal
   model's offset depends little on scale.

**Status (2026-09-30): built (0.5.0); all met but one half**
(`tests/milestone_4.rs`).
1. **Met.** Every earlier test passes; Clarinet opens M alone, and the
   closed ranks stay exactly at rest.
2. **Met.** At 300 Pa M+ sounds +3.667 Hz above M and M− −3.146 Hz below,
   against 3.666 and −3.143 asked of the modes.
3. **Met.** M− against M+: 6.813 Hz.
4. **Met.** Celeste +3.01 dB, Musette +4.77 dB over Clarinet; Master
   +5.94 dB.
5. **Half met.** L sounds −9.6 cents from M's octave and H +2.4, within 15.
   But L sits −16.4 cents below its own mode and H −4.4, against M's
   −6.8: the offset does grow with scale.

A real reed is tuned where it sounds, by a tuner at a playing pressure,
not by its mode. The model sets the mode, so the larger reed comes out
flat. Tuning each reed as a tuner does -- its mode moved until it sounds
in tune at a stated pressure -- is the fix. It waits for a measured tuning
pressure, which nothing read so far gives.

## 5. One bellows for every reed

A compliant reservoir fed by the player's intent and drained by every open reed
and the plates' leakage, so more ranks and more notes lower the pressure each
reed sees; the air button. Bellows compliance and leak are unmeasured in the
literature: voiced by ear, and stated so.

**Decided (2026-09-30): the bellows control is the arm.** The user asked
for whatever is most realistic, playable from a MIDI controller (the
modulation wheel may be mapped later). So the intent -- velocity, or CC 11
-- is the player's push, and the bellows makes the pressure from it. A
"Stiff" response keeps the old meaning, the intent as the pressure itself,
for a digital accordion whose sensor already measures it. "Arm" is the
default.

**What is measured, and what is not.** No study gives an accordion
bellows' compliance, moving mass, the flow each reed draws, or how far the
pressure falls when many reeds sound (INSTRUMENT-ACOUSTICS §2.7: a gap).
What is known:
* **The arm's force falls as it moves faster.** Hill's force-velocity law
  (*Proc. R. Soc. B* 126, 1938, measured on muscle):
  F = F₀ (1 − v/v_max)/(1 + v/(k v_max)), with k ≈ 0.25 in his data. The
  push F₀ is the intent's pressure times the bellows' area.
* **The bellows is a volume of air**, compliance V/(ρc²).
* **Leaks:** a sealed accordion should hold air more than 30 s under
  gentle pressure (repair folklore), and the reeds' clearances pass a DC
  flow that adds up in chords (Ziegenhals 2009).
* **The air button** vents the bellows so it moves without sounding.

**The model.** The arm is taken as quasi-static: its mass, and the
bellows' moving mass, are left out. Both are unmeasured, and with any
plausible value they ring as a lightly damped resonance near 15-20 Hz
that no player reports. The bellows pressure P then follows:
* C Ṗ = A v(P) − Q_reeds − Q_leak − Q_air;
* v(P) from Hill's law at the pushed force;
* the leak and the air button are orifices, α a √(2P/ρ).

The assumed constants are voiced by ear:
* the bellows' area 600 cm² and volume 12 L, a full-size accordion's
  order;
* v_max 1 m/s, the arm's unloaded push;
* the leak 10 mm², inside the "more than 30 s" folklore;
* the air button 400 mm².

**The key's pallet is shared.** A key's one pallet feeds every rank behind
it, and milestone 4 gave each rank its own copy. Now each of the n open
ranks sees the curtain's area over n -- exact when they draw alike, and
milestone 2's single reed unchanged.

**Predictions** (written before building it, 2026-09-30):
1. With "Stiff", every earlier result stands: the earlier engine-level
   tests are run with it.
2. With "Arm" and M alone at the intent that asked 300 Pa, the steady
   pressure falls ~3 % (~9 Pa): Hill's slope (1 + 1/k) F₀/v_max times the
   reed's ~0.39 L/s, over A².
3. With Master the fall is ~15 % (~45 Pa), so Master sits +4.5 to +5.5 dB
   over Clarinet, against +5.94 with the bellows stiff.
4. With the key part-way down, the shared curtain makes Musette's pitch
   bend deeper than Clarinet's at the same depth: three reeds share the
   throttle.
5. The air button fully open, the intent held: the pressure falls by at
   least half and the level by at least 6 dB.
6. No reachable parameter set makes the engine blow up or produce a
   non-number.

**Status (2026-09-30): built (0.6.0); four met, one withdrawn, one not
met** (`tests/milestone_5.rs`).
1. **Met.** Every earlier result stands, the engine-level tests run with
   "Stiff".
2. **Met.** M alone holds 288.1 Pa against the 300 the push asks: 4.0 %
   below.
3. **Met.** Master holds 259.3 Pa (13.6 % below), +4.98 dB over Clarinet
   against +5.98 stiff.
4. **Withdrawn: its premise was wrong.** Built as written -- the open
   ranks sharing one hole's curtain -- it silenced Master: five reeds
   cannot draw through one 150 mm² hole. On an instrument each rank's cell
   has its own hole under the key's pallet, and the curtain grows with the
   holes it covers, so each rank keeps about its own. What they still share
   depends on the pallet's geometry, unmeasured, so each rank keeps its
   curtain.
5. **Not met with the assumed constants.** The air button fully open
   lowers the pressure 29 % (289 → 206 Pa) and the level 2.9 dB. The arm at
   v_max 1 m/s keeps most of its push against the 5.4 L/s a 400 mm² vent
   lets out. Neither constant is moved to meet the prediction.
6. **Met.** 24 random bellows, Master at full push, render finite.

The air button's size and the arm's speed on a bellows are what would
settle 5, and the pallet's geometry 4.

## 6. Cassotto, grille and body

The cassotto's resonance (0.8–1 kHz, Richter 1989) and its high-frequency loss
(Llanos: centroid lower, attack durations unchanged); the grille and the body
as a filter. No published transfer function exists: the body is derived from
geometry where it can be and voiced where it cannot.

**What is measured (2026-09-30).**
* **Richter** (IfM Zwota, Demusa report 1989, read in full): the long
  cassotto shaft resonates at 800 Hz-1 kHz, "in an acoustically favourable
  formant region", little different between makers. A closed grille
  (Weltmeister "Stella") resonates near 500 Hz.
* **Llanos-Vázquez's thesis** (read in full):
  * E4 at mf, the spectral centroid of an 8′ voice is 1389 Hz inside the
    cassotto and 2013 Hz outside (Table 4.16, p189): a ratio of 0.69.
    The 16′ inside is 1078 Hz.
  * "El cassotto no tiene ningún efecto sobre la duración total de los
    ataques ... actúa sobre el sonido generado por la lengüeta, pero no
    actúa sobre la lengüeta misma" (p174).
  * Each key lifts two pallets, one for the pair of blocks inside the
    cassotto and one for the pair outside (p51).
* **Which ranks sit inside:** on Llanos's Pigini Sirius, the 16′ and one
  8′; a double cassotto keeps bassoon (16′) and clarinet (8′) in two
  chambers (Wikipedia, "Cassotto"). Here: L and M.
* **Not published:** a cassotto's inner dimensions (only whole-instrument
  sizes are listed), and any measured transfer function of a cassotto or
  a body.

**The model.** The cassotto is a box its reed blocks sound into, with a
narrow way out: below the box's own standing waves, a Helmholtz resonator.
Its transfer from the flow the blocks send in to the flow that leaves is
H(s) = ω₀²/(s² + (ω₀/Q) s + ω₀²): a lift around ω₀ and −12 dB per octave
above, the "attenuating higher frequencies and reinforcing lower ones" of
the descriptions.
* **ω₀:** Richter's resonance, 900 Hz, the middle of his range
  (measured).
* **Q:** assumed, 2, voiced by ear. Wood, felt and a slot that is not a
  neck make a low one; nothing measures it.

It acts on the sound of L and M only, never on the reeds (Llanos, p174).
The grille and body wait for a measurement: Richter's one grille
resonance is not a transfer function.

**Predictions** (written before building it, 2026-09-30):
1. With the cassotto off (default until heard), every earlier result is
   unchanged.
2. On, M's spectral centroid at mf (~400 Pa in a still bellows) falls to
   0.4-0.8 of its centroid outside: Llanos measured 0.69 on E4.
3. On, the finger attack's duration (−50 → −5 dB of the first harmonic)
   moves by less than 10 %: the filter acts on the sound, not the reed.

**Status (2026-10-01): built (0.7.0); all three met** (`tests/milestone_6.rs`,
M alone, the arm pushing for ~400 Pa).
1. **Met.** With the cassotto off, the default, every earlier test passes.
2. **Met.** M's centroid falls from 1693 Hz to 959 Hz, a ratio of 0.57;
   Llanos measured 2013 and 1389 Hz on his E4, 0.69. The model's own
   centroid outside, 1693 Hz, is of the order of his.
3. **Met.** The finger attack is 320 ms inside and out (+0.0 %).

Measuring it found a trap: a pulse-train sound crosses its mean several
times a period, so zero crossings read the third harmonic (1060 Hz). The
fundamental is taken from the spectrum's peak near the reed instead.

The grille and the body are not built: Richter's one grille resonance
(~500 Hz, one model) is not a transfer function, and nothing else is
measured.

## 7. The whole treble compass, and what it costs

Every key of a 41-key treble, every rank, and a full-register chord inside the
Pi 4's budget — with `parallel_render_v1` if one core is not enough, decided
by the fuel measured in milestone 1, not assumed.

## 8. Stradella bass

Bass and chord buttons on octave-wrapped ranks.

## 9. The product

A schema 3 package with branding, a PLAY surface, and factory programs.
