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
   +5.94 dB. (Over 20 s, which averages the beats (7b, 2026-10-01):
   Celeste +3.03, Musette +4.77, Master +6.32.)
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
   against +5.98 stiff. (Over 20 s, which averages the beats (7b,
   2026-10-01): 252 Pa (15.9 % below), +5.50 dB against +6.32 stiff. The
   arm costs 0.82 dB, inside the 0.44-1.44 the band meant.)
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

**Decided (2026-10-01).**
* **The compass:** a full-size piano accordion's 41 keys, F3-A6 (MIDI
  53-93) for the 8′ ranks; L sounds an octave below, H an octave above.
* **The pitch:** A4 = 440 Hz, equal temperament, the IfM Zwota's reference
  (Richter, "Stimmung", Demusa '90), with "Pitch A4" a parameter. The
  measured F4 vibrates at 355 Hz, ~28 cents above A440's F4, so it is
  retuned like any other.

**The reeds across the compass (assumed, from the one measured reed).** No
maker publishes tongue dimensions. The measured F4 is scaled to each pitch
by a bayan maker's slots, patent RU2233009, Table 3:
* slot length F3 35.4, F4 27.8, F5 20.5, F6 14.7, F7 9.6 mm;
* root widths 4.23, 3.37, 2.66, 2.00, 1.25 mm;
* plates 2.7, 2.7, 2.2, 2.2, 1.7 mm.

Between those notes the ratios are interpolated in log-frequency, and
beyond them extrapolated with the nearest octave's ratio, down to the
16′'s F2 and up to the 4′'s A7. The set and clearances scale with the
length, and the cell and tone hole with the reed: the cell's volume as the
length cubed, the hole as the slot's area. A real treble's top cells are
smaller and filled (Llanos's luthiers, p267-268). The profile is derived
for each frequency, as at F4.

**Tuned as a tuner tunes.** Each reed's mode is set so that it *sounds* at
its pitch at 300 Pa, the IfM's playing pressure for its comparisons (2008
poster). The correction, mode against sound, is computed by simulating
every reed and kept as a generated table, checked by a test against a fresh
computation. Moving a reed's constants afterwards detunes it, as it would
on an instrument.

**Memory and cost.** The engine never allocates. A whole treble -- 205
designs, 410 reeds -- fits once each reed's section table is 256 entries
over a range scaled to its length: measured unchanged against 1024 on F4
(onset 32.3 Pa, swing 3.73 mm, the bend −16.0 cents, the octaves and beats
alike). Only a sounding reed is computed.

**Predictions** (written before building it, 2026-10-01):
1. Every reed of every rank sounds within ±2 cents of its pitch at 300 Pa
   (by construction of the table); the M rank at 100 and 600 Pa within
   ±15 cents.
2. Every one of the 205 reeds speaks from rest below 300 Pa.
3. The tremolo's beats follow the Borsini's lines at every key, within
   5 %.
4. The swing at 300 Pa stays a similar share of each tongue's length
   across the compass, 5-20 % (Braasch & Cottingham: about 15 %).
5. Natively a four-note Master chord (20 reeds) costs under 10 µs per
   48 kHz sample, half the real-time budget. The Pi is measured, not
   predicted.
6. Every earlier milestone's test still passes; key 65's M reed now
   sounds F4 at 349.2 Hz.

**The top did not speak (2026-10-01), and what was measured about it.**
Built as above, with Q 250 for every reed, the tuner could not tune the top.
Thresholds rise as about f^1.6 (`tests/compass_diagnosis.rs`):
* 9 Pa at 175 Hz, 31 at 349, 100 at 698, 297 at 1397, 378 at 1760, 861 at
  3520 Hz;
* the 622 Hz point is 82 Pa, inside Cottingham's measured 60-110 Pa for a
  622 Hz accordion reed (ICA 2016).

From G6 the M ranks, and H from A5, stay silent at 300 Pa. What moves it:
* **Q dominates.** ×4 takes A6 M from 378 to 154 Pa.
* **The cell does not:** keeping it at F4's proportion even raises the
  threshold.
* The tone hole, near field and set move it little; a larger hole or gap
  makes it worse.

A real E7 8′ speaks from piano (Llanos, Table 4.12).

The research (2026-10-01) found the following.
* **No Q is measured above ~750 Hz.** Below:
  * 83 at 137 Hz (Cottingham's reed-organ C3, ICA 1998, D = 0.012);
  * ~233 at 598 Hz (Förtsch 2021, a harmonica draw reed's half-life of
    85 ms in its instrument);
  * 200-400 at 236-743 Hz (Nussbaumer & Agarwal 2016).

  These rise with pitch.
* **No damping mechanism falls for small reeds.** Steel's own loss gives Q
  3,000-50,000 (Cremer & Heckl, via Irvine), viscous air and
  thermoelasticity give thousands, and the clamp is negligible at these
  slendernesses. So Q ≈ 250 is the mount's, gap's and radiation's, and how
  those scale is unmeasured.
* **Thresholds, from a tuner** (testimony, musiker-board 2014):
  * a good plate's lower treble starts at 40-70 Pa;
  * the top piccolos at 100-250 Pa, some near 300;
  * "300 Pa is the tuning pressure because every reed is sure to sound
    there".

**Revised (before measuring).** Q follows the measured trend: Q ∝ f^0.7,
the exponent of the two measured reeds, 83 at 137 Hz and 233 at 598. It is
anchored at the F4's 250, so every earlier result stands. It is a trend
from two reeds of different kinds, extrapolated above 750 Hz, and stated
so: a parameter, "Q Slope".

**Predictions** (written before building it, 2026-10-01):
7. The lower treble's M reeds (F3-C5) start at 70 Pa or less (the tuner:
   40-70).
8. The top piccolos (H at A5-A6, 1.76-3.52 kHz) start at 100-300 Pa (the
   tuner: 100-250, some near 300).
9. Every reed speaks at the 300 Pa tuning pressure (the tuner's reason
   for it).

**Status (2026-10-01): built (0.8.0); six met, two not met at the top**
(`tests/milestone_7.rs`, `tests/compass_diagnosis.rs`).
1. **Met.** Every one of the 205 reeds sounds within ±2 cents of its pitch
   where it is tuned (worst −0.01 cents). The true 8′ at 100 and 600 Pa
   stays within ±15 cents. The tuning table is generated
   (`rf-musette-lab tune`) and checked against a fresh tuning; its
   corrections run from 4 to 20 cents, more for the larger reeds.
2. and 9. **Not met at the very top.** 201 of 205 reeds speak at 300 Pa.
   The 4′ reeds of keys 90-93 (F♯7-A7, ~3-3.5 kHz) need 350-440 Pa, and
   are tuned at the lowest pressure they speak at, as a tuner would have
   to.
3. **Met.** The tremolo's beats follow the Borsini's lines at every key.
4. **Not met above A5, and the cause measured.** At 300 Pa the swing falls
   from 12.7 % of the tongue's length at F3 to 4.4 % at A5. At eight times
   each reed's own threshold every reed swings ~9 % (8.8-10.1 %), so the
   fall is only the high reeds sitting nearer their thresholds: the same
   cause as 2.
5. **Met.** A four-note Master chord costs 4.6 µs per 48 kHz sample
   natively, 22 % of a core.
6. **Met.** Every earlier test passes.
7. **Met.** The lower treble's M reeds start at 13-48 Pa.
8. **Met but at the top.** H at A5-F6 starts at 170-297 Pa; at G7 and A7,
   356 and 438 Pa.

**Built with it:**
* the engine carries the whole treble (271 KiB) and builds a key's reeds
  when it is first needed, or two keys per block after a parameter moves;
* the WebAssembly component is linked with an 8 MiB stack (RackForge's
  convention for a processor built by value), and the laboratory runs on
  a 64 MiB thread: both overflowed their 1 MiB on the first build.

What is left for the top: a measurement of what makes the smallest reeds
speak on an instrument -- their cells, their set, the tip turned to the
inlet (Tonon) -- or of their Q.

## 7b. The finger attack, now that it can be heard

The user, playing the first tune (2026-10-01): the short notes never reach
their tone. With the bellows already pushing, Llanos-Vázquez et al. measured
finger attacks of 50-110 ms at mf and 60-140 ms at p, from A2 to B6 with no
trend in pitch. The model is slower and grows slower still with pitch
(`tests/attack_diagnosis.rs`):
* F3 595/220 ms at p/mf;
* F4 1165/396 ms;
* A6 --/721 ms.

None of the assumed constants reaches the target alone:
* the inertia upstream (a hole 4× deeper, or the near field ×4) or a 5 ms
  pallet bring mf to ~200 ms;
* p stays above 600 ms.

The model's growth scales with the pressure above threshold, so p is slow;
a real reed attacks about as fast at p as at mf. The measured growth is
τ ≈ 11 ms (Llanos, Fig. 4.11) against the model's ~76 ms at F4 mf.

**First, what can be derived: the second mode.** Cottingham (ICA 2019,
read in full):
* "the motion of this type of reed begins with an initial displacement of
  the reed tongue into the reed frame";
* the second transverse and first torsional modes appear in the first
  10-20 cycles and "may be significant in initiating reed oscillation";
* reed-organ tongues curved to excite them attack faster.

The tongue's solver already finds the F4's second mode (the measured
1:4.6). The experiment (`tests/two_modes.rs`, RK4) gives the tongue both
modes:
* the deflection along it is q₁ψ₁ + q₂ψ₂, with the section integrated
  from it;
* each mode is driven by its share of the pressure, S_k = W L ∫ψ_k;
* it pumps its own flow;
* Q₂ is taken as Q₁ (assumed).

The supply rises over 50 ms, as the pallet opens.

**Predictions** (written before running it, 2026-10-01):
1. The hypothesis (Cottingham): with the second mode, the finger attack at
   400 Pa shortens by at least 30 % against the same harness with one mode.
   My expectation is otherwise: a flat tongue feeds its second mode only
   through the section's nonlinearity, so little should change.
2. In the steady state the second mode stays small and the pitch moves
   less than 2 cents: the tongue stays sinusoidal (its 2nd-4th harmonics
   56-72 dB down, as measured).

**Result (2026-10-01).**
1. **Refuted.** The attack is identical with the second mode and without:
   F4 747/296 ms at 100/400 Pa either way, q₂ under 0.01 mm. A flat,
   symmetric tongue hardly drives its second mode; on real tongues their
   curve and asymmetries do.
2. **Met.** The pitch moves 0.01 cents.

**Also tried: which pressure the tongue's face feels.** The minimal model
puts the jet's drop on it, Δp, which starts from zero while the flow
builds; the face sits in the cell. A share β of the cell's pressure in its
place makes the attack slower, not faster (mf 296 → 455 ms at β = 1), the
swing smaller and the pitch +2 cents. The near field's inertial pressure on
the face is part of what feeds the reed.

**Decision: a voiced start, standing on what Cottingham measured.**
Nothing derivable has reached the measured attack:
* the second mode, the face's pressure;
* the passages' and the sink's inertia (2b);
* every assumed constant.

What is measured is the start itself: the tongue "begins with an initial
displacement into the reed frame" (Cottingham, ICA 2019).

From where the model's own growth would take it, an excursion the size of
the reed's set reaches the target at p and at mf alike:
* at F4 the growth time τ is 144 ms at 100 Pa and 57 ms at 400 Pa;
* a 0.5 mm start is −10 dB of the swing at 100 Pa and −19 dB at 400 Pa;
* so the first harmonic reaches −5 dB in ~80 and ~90 ms.

So when a key opens, each reed the bellows blows is started that far into
the frame:
* d = κ · set · P/(P + P₀), as a velocity ω d toward the plate;
* κ, "Attack Kick", is voiced, 1 by default;
* P₀ = 20 Pa (assumed, of the order of the lowest thresholds), so with no
  air there is no start, as on an instrument a key pressed with the
  bellows still says nothing.

It acts at the key's opening and nowhere else: thresholds, the steady tone
and the tuning do not see it.

**Predictions** (written before building it, 2026-10-01):
3. F4's finger attack (−50 → −5 dB of the first harmonic) is 50-140 ms at
   100 Pa and at 400 Pa (Llanos: p 60-140, mf 50-110).
4. At 400 Pa the true 8′'s attack is 50-140 ms from F3 to A6: no trend.
5. Nothing else moves: every earlier test passes, the tuning included.
6. With the bellows still (no pressure), pressing a key is silent.

**Status (2026-10-01): built (0.8.1); three met, one met to A5**
(`tests/milestone_7b.rs`).
3. **Met.** F4: 85 ms at 100 Pa, 99 ms at 400 Pa (were 1165 and 396).
4. **Met from F3 to A5, too fast above.** 106, 103, 99, 83, 73, 53 ms from
   F3 to A5, then 22 ms at D♯6 and 13 at A6. At 400 Pa the high reeds sit
   near their thresholds and swing little more than the start itself.
   Llanos's ~80 ms windows read an instant onset as ~40 ms, so part of the
   measured floor is the window's; the prediction stands as written.
5. **Met.** Every earlier test passes.
6. **Met.** With no air, a key is silent.

On "Frère Jacques" the eighth notes now reach their tone (the render's
RMS 0.139 → 0.206).

**Then (2026-10-01): "siguen habiendo notas que no llegan a salir".**
Measured note by note on the render, before changing anything
(`tests/musette_diagnosis.rs`, and the render's envelopes above 600 Hz,
which is what a phone plays):
* In Clarinet every note reaches its tone except the first. Each reaches
  −5 dB of its level in 100-150 ms.
* The first note never got its start. The score moves the arm and the key
  at the same instant. The start is given once, when the pallet leaves its
  seat, from the pressure at that block's start, which was still 0 Pa. So
  the start was spent before the air came, and the reed grew from rest. It
  was silent for ~250 ms and reached its tone at ~360 ms.

The same happens in play:
* whenever a key goes down before the bellows moves;
* to every held key's other reed when the bellows turns;
* to a rank whose register opens under a held key.

The physics: the start is the tongue pushed into its frame by the air
reaching it. It belongs to the air's arrival, not to the key's.
* In Musette each of the three reeds attacks as in Clarinet, and the
  bellows holds (351 ± 15 Pa). Started together, the three beat:
  * at C4, 2.4 and 3.1 Hz apart, the fundamental of the sum dips 35 dB at
    120 ms;
  * the harmonics dip at other moments, so the sound does not.

  The three reeds share the key's pallet, so starting them together is the
  physics. It is left as it is.

**The change:** each reed is owed its start from the moment air can reach
it:
* its key down;
* its register open;
* its side's pressure above P₀.

As that pressure rises, the reed is given the rise of P/(P + P₀) not yet
given. It is owed again once its air is gone: the key up, the register
shut, or its side's pressure at or below P₀ (as at a reversal, or with the
bellows stopped). Below P₀ no reed speaks anyway. A key pressed into a
steady bellows gets exactly the start it got before.

**Predictions** (written before building it, 2026-10-01):
7. The arm and the key moving together, C4 in Clarinet: the first
   harmonic reaches −5 dB of its level (−50 → −5 dB, as in 3) in
   50-140 ms. It was ~360 ms.
8. A key held with the bellows still, then the bellows starting at 400 Pa:
   the same, 50-140 ms.
9. Every earlier test passes, 7b's included, unchanged.

**Status (2026-10-01): built (0.8.2); all three met**
(`tests/milestone_7c.rs`).
7. **Met.** 112 ms. Before the change the same metric read 303 ms; the
   ~360 ms above was read off the tongue's trace.
8. **Met.** 92 ms (was 212 ms).
9. **Met, after one earlier test's window was mended.** 7b reads the same:
   F4 85/99 ms, F3-A5 106-53 ms. Milestone 5's prediction 3 moved, and not
   because the steady state did. Its level was the RMS over 1.5 s, and
   Master's tremolo ranks beat at ~3.5 Hz, so the level hung on the phase
   they started at.
   * Over 20 s it settles to +5.50 dB over Clarinet with the arm. Without
     the start it is +5.47.
   * The stiff Master is +6.32, not the +5.94 recorded. That +5.94 came
     from milestone 4's 2.5 s window; Celeste and Musette moved under
     0.03 dB.
   * The prediction's band (+4.5 to +5.5, against +5.94 stiff) is what the
     arm costs, 0.44-1.44 dB. It costs 0.82 dB.

   Both tests now read 20 s, and milestone 5's asserts the cost.

The start is checked at every step, not once per block: the laboratory
renders from one event to the next in one call, and the first build gave
the start once per call.

A key held through a reversal: the push reed reaches −5 dB of its swing
~240 ms after the turn begins (the pressure crosses zero at 50 ms). With
no start, as before, it took ~410 ms. The pressure comes through the
100 ms turn gradually, so the start spreads over several cycles.

On "Frère Jacques" the first note now speaks like the others. Above
600 Hz it is −26 dB at 100 ms; it was silent until ~250 ms.

## 8. Stradella bass

Bass and chord buttons on octave-wrapped ranks.

**What is known (2026-10-01).** No measured study of a Stradella bass was
found; the sources are descriptive, and two of them agree:
* Roland's FR-3x and FR-8x manuals give five bass-side ranks: 16′, 8′,
  "8-4′", 4′ and 2′.
  * In the FR-8x's Bass Edit, 16′ and 8′ sound for the bass buttons only.
  * 8-4′, 4′ and 2′ sound for the chords, or for the chords and the basses.
  * Seven bass registers: 2′; 4′; 8-4′; 16′/8′/8-4′/4′/2′; 8′/4′/2′;
    16′/8′/8-4′; 16′/2′. One register serves the bass and chord rows
    alike.
* Wikipedia ("Stradella bass system", unreferenced) gives five sets, each
  one octave wide:
  * bass C2-B2;
  * tenor C3-B3;
  * contralto F♯3-F4;
  * alto C4-B4;
  * soprano C5-B5.

  The contralto, wrapping from below 4′ into it, is Roland's "8-4′".
* A bass button sounds one pitch class, a chord button three. The fifth is
  left out of the seventh and diminished chords (Wikipedia; Roland FR-1x,
  p53).
* MIDI (FR-3x, pp57-59, 52): the treble is on channel 1, the bass buttons
  on 2 and the chords on 3. The bass buttons send one octave, C3 = 48.
* The reeds: Llanos-Vázquez measured left-hand bass reeds 52 mm long at
  62.5-87.5 Hz, tip-loaded (thesis, Table 3.1). The left hand's low reeds
  are longer and less loaded than the right hand's (p222).

**The model, assumed where the sources are only descriptive:**
* Twelve bass "keys", one per pitch class, each with the five ranks above.
* On each rank a pitch class has one reed, wrapped into that rank's
  octave. A pitch class's reed is shared: the bass button and every chord
  that holds the pitch class open the same reed, as the Stradella
  mechanism does. A reed opened by two buttons sounds once.
* A bass button opens its pitch class on every open rank. A chord button
  opens its three pitch classes on the open chord ranks (8-4′, 4′, 2′)
  only.
* Each reed is the treble's model: the measured F4 scaled by the slot
  ratios carried on down, its profile derived for its frequency, tuned at
  300 Pa by the generated table. No tip loads, which Llanos's "less loaded"
  allows for the first build.
* The bass registers are Roland's seven; the default is 4, all five ranks.
* MIDI:
  * channel 2: bass buttons, any octave of a pitch class being that
    button;
  * channel 3: each note sounds its pitch class on the chord ranks, so a
    V-Accordion's chord and a keyboard player's left-hand chord both work;
  * every other channel plays the treble as before.

  Most players have a MIDI keyboard, not a V-Accordion. Many keyboards can
  send a split zone on another channel, and that path is the one built
  first. A keyboard split inside the instrument is milestone 8b, once the
  bass can be heard.

**Predictions** (written before building it, 2026-10-01):
1. Every bass-side reed, 60 of them, is in tune within ±2 cents at 300 Pa
   once tuned, and sounds at 300 Pa.
2. The geometry carried down lands on the measured left-hand reeds: the
   16′ C2's tongue is 47-57 mm long (Llanos: 52 mm at 62.5-87.5 Hz).
3. A bass button sounds its pitch class on each open rank at that rank's
   octave, and nothing else. In register 4, C sounds C2, C3, C4 (twice,
   8-4′ and 4′) and C5.
4. A chord button sounds its pitch classes on the chord ranks only:
   * C major C-E-G;
   * C7 C-E-B♭, without the fifth;
   * C dim C-E♭-A, without the fifth.
5. C bass and F major held together blow the 4′ C reed once: its state is
   the same as with F major alone.
6. Through the plugin: a note on channel 2 plays its bass button, on 3 its
   chord pitch class, on 1 the treble, and the earlier contracts hold.
7. The 16′ reeds' thresholds are under 20 Pa. Large, slow reeds speak
   easily.
8. A bass button's finger attack at 400 Pa is 50-140 ms on the 16′ C2
   (Llanos measured A2, a bass note, in that range).
9. The engine still fits its stack test, under 384 KiB, and a bass,
   a chord and a four-note Master chord together (34 reeds) cost under
   12 µs per sample natively.

**First measurement (2026-10-01): the 16′ C2 does not speak.**
`tests/bass_diagnosis.rs` measured the slots carried down:
* The 16′ runs from 64.6 mm (C2) to 51.7 mm (B2). The 8′ C3 is 50.7 mm.
* Thresholds are 4-8 Pa, and every reed but C2 sounds at 300 Pa.
* C2 starts, then stops by 2 s. At 300 Pa its tongue sits 2.2 mm into the
  slot, 2.5 times its set, and its swing dies there.

In this model a tongue's static yield under a pressure goes as
1/(L·f³): its thickness is set by its frequency (h ∝ f L²), so its
stiffness falls with f³. From F3 down to C2 that is ×13. A tongue made long
enough for 65 Hz is too soft for the bellows. Prediction 2 is therefore
already in doubt: C2 is 64.6 mm, longer than the 47-57 asked.

Real bass reeds carry a load riveted at the tip (Llanos, Table 3.1:
52 mm at 62.5-87.5 Hz, loaded). For the same frequency, a loaded tongue is
thicker and its stiffness grows as h³.

**The load, derived from a rule:**
* Each bass-side reed whose static yield ζ/set at a pressure exceeds that
  of the treble's lowest reed (the 16′ of key F3, made for 87 Hz, which
  sounds) is loaded just enough to yield no more than it does.
* The rule is assumed: a maker loads the reeds that would otherwise give
  way. The treble's lowest reed is the softest the model already plays.
* The stiffness asked is r times the unloaded one, with
  r = (L_ref² f_ref³ set_ref)/(L² f³ set). The tongue thickens by r^⅓, and
  the load is M = M_r (r − r^⅓).

**Predictions** (written before building it):
10. Loaded so, every bass-side reed sounds at 300 Pa and tunes (1 above
    then holds).
11. The loads, as M over the whole tongue's mass, are 0-0.3. Llanos's
    reed 3 carries 0.12, and the loads he added for his fits ran
    0.05-0.8.

**Built, then measured again: the rule asked too little.**
* Loaded so, every bass-side reed sounds and tunes at 300 Pa (10 met;
  loads 0.009-0.055, 11 met).
* Through the engine, at 400 Pa, the 16′ C2 starts and dies, with the
  start and without it.

Which pressures each low reed holds a tone at, from rest, between 50 Pa
and 1 kPa (`tests/bass_diagnosis.rs`):
* The reference itself, the treble's 16′ of key F3, is silent from
  600 Pa. The treble's lowest 16′ reeds have been choking at forte since
  milestone 4, unseen: every test of them ran at 300 Pa.
* The loaded 16′ C2-F♯2 stop above 400 Pa, G2-A♯2 above 600-800 Pa. Only
  B2, unloaded, holds across the whole range, as every 8′ does.

A player's range runs to the bellows' ceiling, 1 kPa. A maker who loads a
reed so it speaks loads it for the whole range, not one pressure.

**The rule, corrected (written before building it):**
* The reference is the softest unloaded reed that holds a tone from
  50 Pa to 1 kPa: the reed made for B2, 123.5 Hz, of the slots carried
  down.
* Every reed softer than it is loaded to yield no more than it does, on
  either side. On the treble that loads the 16′ of the lowest keys, as
  right-hand bass reeds are loaded, more than the left's (Llanos, p222).
* The load is stored as M over the unloaded tongue's modal mass,
  r − r^⅓, so the reed's model, which knows the tongue's profile, makes it
  kilograms.

**Predictions:**
12. Every reed of both sides holds a tone at 50, 300 and 1000 Pa.
13. The treble's tuning, retuned for its loaded 16′ reeds, again holds
    ±2 cents (milestone 7's tests pass), and nothing else in the treble
    moves.

**Built and measured: 12 not met.** Yielding no more than B2 is not enough
to speak like B2.
* The loaded C2 and C♯2 hold to 600 Pa, D2-A2 to 800 Pa. The treble's
  lowest 16′ reeds now hold to 800 Pa.
* What chokes a reed is not its static yield alone. The lower its pitch,
  the lower its Q and the slower its growth.
* Through the engine the loaded C2 now sounds at 400 Pa. Its attack is
  slow: −19 dB at 31 ms, rising 0.025 dB/ms. 7b's growth defect is worst
  at the lowest pitch.

**The load, as a maker finds it (written before building it):**
* A maker loads a low reed and tries it until it speaks. So does the
  laboratory: each reed's load is the least that lets it hold a tone at
  50 Pa, at 300 Pa and at the bellows' ceiling, 1 kPa.
* The search starts from the yield rule, raises the stiffening r by a
  quarter until the reed holds, then halves the bracket six times.
* `rf-musette-lab tune` writes the loads beside the cents, as generated
  tables; the tuning is made on the loaded reeds.

Prediction 12 stands as written for this build, and 13 with it.
Prediction 8, the 16′ C2's attack at 400 Pa, is expected to fail by 7b's
known defect; it is kept as written.

**Status (2026-10-01): built (0.9.0).** Eight predictions met, five not met
(`tests/milestone_8.rs`, `crates/rf-musette-plugin/tests/contracts.rs`).
1. **Met.** Every bass-side reed sounds at 300 Pa, the worst at
   +0.00 cents.
2. **Not met.** The 16′ C2 is 64.6 mm, not 47-57. Llanos's reeds are
   loaded and shorter; here the load stiffens a tongue the slots already
   made long.
3. **Met.** A bass button sounds its pitch class on every open rank and
   nothing else. 8′/4′/2′ opens only those. Any octave of the note is the
   same button.
4. **Met.** A chord sounds its pitch classes on the 8-4′, 4′ and 2′ only
   (C major, C7 and C dim as a V-Accordion sends them).
5. **Met.** C bass with F major blows the 4′ C reed exactly as F major
   alone does, sample for sample.
6. **Met.** Channel 2 plays the bass buttons and 3 the chords, in MIDI 1.0
   and 2.0; 1 the treble.
7. **Not met for the loaded reeds.** 16′ thresholds run 54, 40, 31, 25, 20,
   17, 14, 12, 11, 10, 9, 9 Pa from C2 to B2. The load that keeps C2-E2
   speaking at 1 kPa also stiffens them.
8. **Not met.** The 16′ C2 attacks in 467 ms. 7b's start is not enough
   where the reed's own growth is slowest.
9. **Met.** 357 KiB. Both hands, a bass, a chord and a four-note Master
   chord, cost 7.5 µs per sample natively; the four-note chord alone is now
   5.3 µs (was 4.6), the bass side's idle keys walked too.
10. **Met** with the load search: every bass-side reed sounds and tunes.
11. **Met.** The loads run 0.001-0.25 of the tongue's own mass, on the 16′
    alone, C2 the heaviest.
12. **Not met as written; the prediction was wrong to ask it.**
    * No reed of either side chokes at 1 kPa any more. Every reed below
      300 Hz speaks from 50 Pa to 1 kPa.
    * Above ~600 Hz the reeds do not start at 50 Pa, as their thresholds
      (milestone 7) say they should not.
    * The top four 4′ still need more than 300 Pa (milestone 7's defect).
13. **Met.** The treble, its 16′ of keys F3-B3 now loaded (0.007-0.99 of
    their unloaded modal mass), tunes within ±2 cents again, and every
    earlier test passes.

Also changed: native tests run on 8 MiB threads, as the WebAssembly
component does (`.cargo/config.toml`). One test keeps two 357 KiB engines.

**Heard on "Frère Jacques" with a bass and chord accompaniment**
(`scores/frere-jacques-bass.score`):
* The chords sound at the melody's level.
* Of a bass note, the 8′, 8-4′, 4′ and 2′ sound.
* The 16′ stays 30 dB under its level for the whole 440 ms note: the slow
  growth of the lowest reeds is now the bass side's first defect.

## 8b. The left hand on one keyboard

The user (2026-10-01): many players have a MIDI keyboard, not a
V-Accordion. Decided: two fixed zones under a split, no guessing and no
latency.

**The design:**
* On every channel but the bass's and the chords', a note at or above the
  split point plays the treble as before.
* The octave just under it plays the chord ranks, each key its pitch class:
  a triad held there sounds as the chord it is.
* Everything below that plays the bass buttons, any octave of a pitch
  class being its button.
* Two parameters:
  * "Left Hand", on by default -- under F3 the treble has no reeds, so
    turning it on takes nothing from anyone;
  * "Split Point", F3 (53) by default.
* A note is let go where it was played, so moving the split while a key
  is held leaves nothing stuck.

On a 61-key keyboard (C2-C7) the chord octave is F2-E3 and the bass
buttons C2-E2. A player who wants all twelve bass buttons moves the split
up or the keyboard's octave down.

**Predictions** (written before building it, 2026-10-01):
1. Channel 1, Left Hand on:
   * F3 and above play the treble;
   * E3 down to F2 sound their pitch classes on the chord ranks only;
   * E2 and below sound their bass buttons.
2. A note held while the split moves is let go where it was played: no
   reed stays open.
3. Left Hand off: channel 1 under F3 is silent, as before.
4. Channels 2 and 3 are unchanged, and every earlier test passes.

**Status (2026-10-01): built (0.9.3); all four met**
(`tests/milestone_8b.rs`).
1. **Met.** On channel 1:
   * F3 plays the treble;
   * E3 and F2 sound E and F on the chord ranks only;
   * E2 sounds the E bass button on every rank.
2. **Met.** E2 held, the split moved to C2, let go: the bass button shuts.
3. **Met.** Left Hand off, E2 and D3 on channel 1 are exactly silent.
4. **Met.** Channel 2 plays its bass button with the split at C7; every
   earlier test passes.

"Frère Jacques" with its accompaniment on channel 1 alone
(`scores/frere-jacques-keyboard.score`) renders sample for sample as the
score that sends it on channels 2 and 3.

## 8c. The low reeds' attack

The user, after milestone 8: the attack of the low notes. Measured first
(`tests/bass_diagnosis.rs`, 2026-10-01), with a literature search.

**7b's premise does not hold.**
* 7b took Llanos's ~0.75 dB/ms rise of the radiated first harmonic
  (thesis Fig. 4.11) as the reed's growth, τ ≈ 11 ms, and judged the
  model's ~20/s four times too slow.
* The one direct measurement of a free reed's growth coefficient
  (Cottingham, Reed & Busha 1999, Fig. 4, read off; a C3 reed-organ reed,
  131-137 Hz) is:
  * negative below ~0.1 kPa;
  * 4.5/s at 0.3 kPa and 8.3/s at 0.5 kPa;
  * 11.5/s at its peak near 1 kPa, falling beyond.
* The model's 8′ C3 grows 18.8/s at 400 Pa: as fast as measured, or
  faster. A radiated harmonic's slope in dB/ms is not a growth rate; it
  carries the start and the pallet with it.
* A harmonium B5's onset, read off a vibrometer trace, is 20-25/s
  (Puranik & Scavone, Forum Acusticum 2023, Fig. 6).
* The linear theory gives σ of order ρv/(ρ_steel t), independent of
  pitch. A heavier tongue per unit area grows slower, so a loaded bass
  reed is slower.

**Llanos's Table I against the model**, note by note: finger attacks of the
8′, 400 Pa for mf and 100 Pa for p, the bellows stiff.
* A3-B4 agree (model 83-103 ms at mf; measured 60-110).
* A2-B2 do not: 220-249 ms against 70-100 at mf, 155-191 against 110 at p.
  Llanos measured nothing below A2. The technical literature he cites says
  low reeds "need a certain time to respond", counting from the trigger
  (p377).
* A5-A6 at p come out at 0-3 ms against 60-130. The start is already
  nearly the whole of the small swing they settle to at 100 Pa.

**What makes A2 slow in the engine and not in the reed alone**
(`the_a2_opening_traced`):
* The pallet's 50 ms opening. With a 5 ms opening, A2-B2 attack in
  90-112 ms.
* Not by throttling: the cell holds its 400 Pa by 20 ms.
* The start fires when the bellows' pressure is there, with the pallet
  still shut and the cell empty. The air reaching the tongue a few
  milliseconds later pushes it into its frame too, and in an A2, whose
  period (9 ms) is about as long as that rise, the two partly cancel. Its
  swing at 20 ms:
  * 0.19 mm with the start;
  * 0.35 mm without it;
  * 0.82 mm with the start and a 5 ms opening, where both arrive together.

**The change:** the start belongs to the air's arrival, as 7c said: the
arrival at the reed. Each reed is given the rise of P/(P + P₀) of its own
cell's pressure, not of the bellows' side, while its key is down and its
register open. It is owed again when its cell's pressure falls to P₀.

**Predictions** (written before building it, 2026-10-01):
1. A2-B2 (the 16′), finger attack at 400 Pa through a 50 ms pallet:
   50-140 ms (Llanos 70-100 at mf).
2. A3-B4 stay in 50-140 ms at 400 Pa.
3. 7b's tests and 7c's pass as written (F4 50-140 ms at 100 and 400 Pa;
   a note begun with the bellows, a key held before the air).
4. The 16′ C2 attacks faster than its 467 ms. Nothing measured bounds it.
5. A5-A6 at p stay too fast: this change does not touch them.

**Built, first as written, then corrected.**
* As written, the start given as the cell's pressure rises spread over
  that rise, several of a mid reed's periods, and partly cancelled itself:
  * mid notes slowed to 118-128 ms at mf;
  * A5 at p to over a second;
  * A2 gained only 249 → 204 ms.
* Corrected: nothing until the air is in the reed's cell (above P₀); then
  the whole start the bellows' pressure gives, at once; later rises of the
  bellows' pressure as before. This is what was meant: the start and the
  air's push arrive together.

**Status (2026-10-01): built (0.9.1); three met, one not, one as foreseen**
(`tests/milestone_8c.rs`).
1. **Not met.** A2, A♯2, B2: 179, 164, 151 ms (were 249, 235, 220), against
   50-140. Through a 5 ms pallet they are 90-112. What still slows them is
   the pallet half open through its first tens of milliseconds, which
   throttles a large reed's growing flow more than a small one's. The
   pallet's lift, opening and shape are assumed; nothing measures them.
2. **Met.** A3-B4: 82-102 ms.
3. **Met.** 7b: F4 79/98 ms at 100/400 Pa (were 85/99). 7c: 106 and 90 ms.
   Every earlier test passes.
4. **Met.** The 16′ C2: 399 ms over milestone 8's 2 s render (was 467),
   427 over 8c's 2.5 s.
5. **As foreseen.** A5-A6 at p: 0-3 ms, unchanged.

## 8d. The bellows' mass -- tried, reverted, open

**Measured first** (2026-10-01): in "Frère Jacques" the 16′ C2 never grows.
* Under the arm, at CC 11 = 80 (~370 Pa), its swing stays at the start's
  1.1 mm for two seconds.
* At the same pressure from a stiff bellows it grows to 3.5 mm. With the
  arm five times faster it grows to 3.1 mm.
* Redesigning the tongue is no lever: over 40-65 mm and every load, its
  growth at 300 Pa stays under 7/s.

**First hypothesis: the arm at audio rate.** `wind.rs` makes the arm's
speed answer the pressure at every sample (Hill's law, no mass between
them). The arm is then a resistance across the bellows' air,
≈ 1.3 kPa·s/m³ at 400 Pa, at every frequency, and dissipates the flow a
reed modulates.

**Tried: the moving half's mass** (4 kg, assumed), M v' = F(v) − A P.
* At audio frequencies the bellows becomes its air.
* The C2 then died faster: 1.1 → 0.5 mm in 2 s.
* Note by note at ~370 Pa (swing at 0.5 → 2 s):
  * C2 (66 Hz) 1.1 → 0.5 mm, against 2.0 → 3.5 stiff;
  * E2 (83 Hz) 1.4 → 1.8, against 3.8 → 5.2 stiff;
  * A2, F3 and F4 nearly as stiff.
* The bellows' air and a reed's tone hole make a Helmholtz resonator,
  53-77 Hz. The lowest 16′ sit on it.

**Tried: tone holes and loads found as a maker finds them**, on a
simulation of the reed fed by the bellows' air:
* It worked for the treble and the bass from E2 up.
* For C2-D♯2 nothing worked, once "speaks" was measured honestly.

**A flaw in my own measurement, found on the way.** "Holds a tone" asked
only for an oscillation over 1 µm in the last half second. A reed started
by the pressure's step and slowly dying passes that after six seconds. The
honest test asks for a tone that does not die and is at least as wide as
the reed's set. Under it:
* on the 12 L bellows, the 16′ C2-D♯2 do not speak at 300 Pa with any load
  or hole;
* from an ideal pressure they do;
* a larger bellows (up to 40 L) does not help.

milestone 8's load search used the weak test. Its loads hold, but the
claim "every reed below 300 Hz speaks from 50 Pa to 1 kPa" rests on it and
is to be measured again.

**Reverted** (nothing of it ships): the mass, the hole tables, the joint
search. The experiments' code went with them; the numbers are above.
**Open:** why a bellows with air in it, massive or not, holds the lowest
reeds, when real 16′ basses down to C2 speak. Candidates, none measured:
* the path between the bellows and the bass pallets (a plenum, the bass
  mechanism's box);
* walls that give;
* the bass blocks' cells and holes, which here are the treble's scaled.

The next step is reading and measuring, not voicing.

**Found in 8e.**
* The bellows' air behind a big tone hole is, for the lowest reeds, a
  cavity below its resonance, which "adds friction" (Fletcher, after
  Llanos p236).
* An inlet duct moves the resonance under the reed's pitch.
* At audio frequencies the bellows is its air, the arm on the mean.

## 8e. The bass's inlet ducts

**What the sources say** (read 2026-10-01, the user's copy of Llanos's
thesis):
* An inlet duct "favours the onset of the vibrations", while a cavity,
  whose reactance is −ρc²/(Vω), "adds friction" (Llanos p236, after
  Fletcher & Rossing 1998 p228). The onset of a (−,+) reed asks the
  reactance upstream to be inertive (Appendix 1, Eq. A1.5a).
* Luthiers set the lowest reeds "slightly higher than normal" to shorten
  their delay (p153).
* In mf the lowest octave always attacks slower, and all those reeds are
  loaded (p220; Table 4.7, A1 200-295 ms).

**Measured** (`tests/bass_diagnosis.rs`, `the_bass_coupling_levers`,
`depth_against_set_for_the_lowest`). The reed is fed by a bellows that at
audio frequencies is its air (12 L), the arm delivering the mean draw and
holding the mean pressure (`simulate_fed`). "Speaks" means sustained, at
least as wide as its set.
* Seen from the reed, the bellows' air behind the tone hole is a cavity
  below the hole+bellows resonance (53-77 Hz for the big holes). The
  lowest 16′ sit there and do not speak.
* A deeper tone hole -- a longer inlet duct, more inertance -- moves that
  resonance under the reed. Its depth is today the assumed 5 mm of a
  pallet board, everywhere.
  * ×10 (50 mm): C2 speaks at 100, 300 and 1000 Pa.
  * ×6 (30 mm): C♯2-D♯2 do.
  * A higher set helps D2-E2 at 1 kPa. A thicker plate, or a smaller hole
    alone, do not reach.
* Whether the arm answers the mean draw in 50 ms or in 10 ms (the time
  scale the bellows' mass sets) changes nothing.

On an instrument the bass reeds stand on blocks above the bass board, and
the air reaches each cell through a channel in the block. The duct is
plausible; its length is not published.

**The change** (written before building it, 2026-10-01):
* `wind.rs`: at audio frequencies the bellows is its air. The arm delivers
  the mean air drawn (over 10 ms, the time scale of the moving half's
  mass, critically damped -- assumed, as no player reports a ring), and
  holds the mean pressure on Hill's law over 0.1 s. The steady states are
  unchanged.
* "Speaks", for the loads and the new search, means sustained and at
  least as wide as the set.
* Each reed under 300 Hz gets the least inlet duct (a multiple of the 5 mm
  hole) with which it speaks, fed by the engine's own bellows, at 50 Pa
  (or the least pressure it can), 300 Pa and 1 kPa.
* Written by `rf-musette-lab tune` beside the loads and cents.

**Predictions:**
1. Steady states unchanged: milestone 5's droops pass.
2. Ducts are found for the lowest 16′ only, C2 the longest, near ×10
   (50 mm); from A2 up nothing changes.
3. On the engine's bellows, under the arm at CC 11 = 80, the 16′ C2 grows:
   its swing passes 3 mm within 2 s.
4. Every reed under 300 Hz speaks, sustained, at 300 Pa and 1 kPa on the
   engine's bellows.
5. Every earlier test passes. A3-B4's attacks stay within 50-140 ms.

**Status (2026-10-01): built (0.9.2); four met, one not.**
1. **Met.** Milestone 5's droops pass. Master holds 259.9 Pa (13.4 %
   below); the arm costs it 0.89 dB.
2. **Not met: more reeds than predicted get a duct.** Every 16′ does:
   * from C2 ×9.06 (45 mm) down to B2 ×1.58;
   * the treble's 16′ of keys F3-B3, the same pitches, ×3.23-1.58.

   The loads are unchanged.
3. **Met.** Under the arm at CC 11 = 80 the 16′ C2's swing reaches
   4.9 mm (it stayed at the start's 1.1 mm).
4. **Met.** Every reed under 300 Hz speaks, sustained, at 300 Pa and 1 kPa
   on the engine's bellows.
5. **Met.** Every earlier test passes. A3-B4: 82-102 ms.

The duct also feeds the reed from an ideal pressure, and three earlier
predictions are now met:
* milestone 8's 7: the 16′ thresholds are 16-8 Pa;
* milestone 8's 8: the 16′ C2 attacks in 130 ms;
* 8c's 1: A2-B2 attack in 139, 134, 129 ms, still slower than Llanos's
  70-100.

The bellows' first form held the pressure over 0.1 s. A note begun with
the bellows then took 203 ms (7c), so the hold took the mass's 10 ms too.
7c: 111 and 109 ms.

The honest test of speaking shows milestone 7's top-4′ defect from key 87,
not 90. Both hands now cost 7.7 µs per sample.

## 8f. The keyboard player's bellows

The user (2026-10-01): a keyboard player with both hands busy and no
expression pedal still needs a bellows, and one playing with one hand can
move the modulation wheel with the other. An accordion has no wheel and
no vibrato control. Its own vibrato is a bellows shaken by the arm, which
the model already makes from the pressure; so the wheel is better spent as
the bellows. Decided: the wheel and a smoothing first; the bellows' travel
and turning on its own later.

**The design:**
* The modulation wheel (CC 1, with CC 33 as its low bits; MIDI 2.0 at full
  width) takes the bellows as Expression does. Wheel down is the bellows
  still, and silent; wheel up is the hardest push. Of the wheel and an
  expression pedal, the last moved leads.
* "Bellows Smoothing": when velocity sets the push, the push moves to each
  new strike's over this time (first order), not at once. An arm does not
  jump. 150 ms by default, voiced by ear later. A wheel or a pedal is the
  player's hand already, and the bellows follows it as it comes.

**Predictions** (written before building it, 2026-10-01):
1. CC 1 at a value asks the pressure CC 11 at that value asks; wheel down
   leaves the instrument silent; 14-bit and MIDI 2.0 work as for CC 11.
2. Velocity alone: from a soft strike to a hard one, the asked pressure
   rises 63 % of the way in the smoothing time (150 ± 15 ms), and does not
   jump.
3. With a controller the pressure follows it at once, as before: every
   earlier test passes.

**Status (2026-10-01): built (0.9.4); met, one as written read on the push**
(`tests/milestone_8f.rs`, the plugin's contracts).
1. **Met.** CC 1 asks what CC 11 asks, at seven bits, at fourteen
   (CC 1 + CC 33) and at MIDI 2.0 width. With the wheel down a key is
   silent.
2. **Met on the push.** It moves 63 % of the way in 150 ms, and does not
   jump. As written, of the pressure, the prediction overlooked the curve:
   the pressure is the push squared, and moves 51 % in that time.
3. **Met.** A controller's move is whole within 5 ms. Every earlier test
   passes.

## 8g. The bellows' travel

The rest of the user's idea (2026-10-01): a bellows runs out. A player
turns it, and the short break as it turns is part of the accordion's
sound (McMahan 2016; Llanos et al. 2002: "a slight interruption"). With
no bellows controller there is no hand to turn it, so the instrument can
turn it itself.

**The design:**
* "Auto Reverse", off by default. Off, the bellows never runs out, as
  before.
* "Bellows Travel", the air the bellows gives in one direction before it
  must turn: 12 L by default, assumed. A full-size bellows' area of
  600 cm² over some 20 cm of the stroke a player uses; voiced by ear.
* On, the air the reeds draw is counted. The bellows turns, as a player
  would:
  * at a gap -- no key, button or chord note held -- once 70 % of the
    travel is spent;
  * or, if no gap comes, when all of it is.
* The turn is the existing reversal, over Reversal Time. The direction the
  player set stays the parameter's; the instrument's turns ride on it, and
  a player who sets the direction again takes the bellows back, its travel
  counted afresh.

**Predictions** (written before building it, 2026-10-01):
1. Off: nothing changes; a render is identical, sample for sample.
2. On, a chord held: the bellows turns once the air drawn reaches the
   travel, within 5 %.
3. On, notes with gaps between them: every turn falls in a gap, none
   before 70 % of the travel.
4. After a turn the other reeds sound, the push reed as the pull reed
   did; every earlier test passes.

**Status (2026-10-01): built (0.9.5); all four met** (`tests/milestone_8g.rs`).
1. **Met.** Off, a Master chord held 10 s on a 2 L travel never turns the
   bellows. Off, the code path is skipped, so nothing else changes.
2. **Met.** A Master chord held turns the bellows after 2.05 L of a 2 L
   travel (+2.5 %).
3. **Met.** One true 8′ in notes of 400 ms with gaps of 100 ms, 2 L of
   travel: every turn begins in a gap, after 1.49-1.50 L (70 % is 1.4 L).
4. **Met.** After the turn the push reed swings 3.8 mm; every earlier test
   passes.

## 8h. The click at a note's start

The user (2026-10-01): "a fine click every time it plays, for many
commits now ... it feels like a thin click that makes no sense."

**Measured first** (`tests/click_diagnosis.rs`). F4 and the 16′ C2, the
bellows stiff at 300 Pa:
* 1 ms after the key, the output peaks at 0.046 (a tenth of the steady
  tone's peak) whether the start (7b) is on or off.
* Before the peak it rings, the sign alternating each sample and growing:
  ~24 kHz.
* The first 4 ms carry 20-30 dB more than the steady tone at 12-20 kHz.

The cause is numerical. The pallet's curtain is linearised in each step
about the hole's flow at the step's start, R_p = ρ|a₀|/(2α²A_p²).
* As the pallet first lifts, a₀ = 0, so R_p = 0. For one step the hole
  passes as if no pallet were there, however little it has lifted.
* The air bursts into the cell; the next step's R_p is enormous and the
  flow collapses.
* The radiated sound is the flow's rate of change, so the burst is an
  impulse, and the decimator's steep filter rings with it near 20 kHz.

**The change:** the curtain never resists less than Bernoulli's orifice
does at the pressure drop across it:
R_p = max(ρ|a₀|/(2α²A_p²), √(ρΔp/2)/(αA_p)), with Δp = P − p_cell.
* In steady flow the two are equal (a = αA√(2Δp/ρ)), so nothing settled
  moves.
* R_p stays ≥ 0, so the step stays passive.

**Predictions** (written before building it, 2026-10-01):
1. The onset no longer rings: no sign-alternating run before the first
   peak, and the first 4 ms carry no more above 8 kHz than the steady tone
   (within 6 dB).
2. Nothing settled moves: every earlier test passes, the tuning tables
   current.

**Built, in three tries.**
* As written, the floor at every step lowered the steady tone: F4's peak
  0.445 → 0.363 (−1.8 dB). An oscillating hole's flow nears zero each
  cycle, where the floor added resistance.
* On the pallet's first lift alone, the steady tone held, but the onset
  still rang (+18-22 dB at 12-20 kHz): the alternation runs through the
  whole opening.
* Built: the floor while the pallet moves, opening or closing. The reed
  keeps the last step's curtain to know it (`ReedState::pallet`). Settled,
  the step is as before.

**Status (2026-10-01): built (0.9.6); both met.**
1. **Met.** F4's onset no longer rings. Its brightest 5 ms are 13 dB under
   the steady tone's (were 8 dB over), and at 8-20 kHz the first 4 ms sit
   at −54 to −59 dB, under the steady tone. In "Frère Jacques" with the
   bass, the onsets' peaks above 8 kHz fell 28 dB (median 0.31 → 0.013);
   the level is unchanged (−11.05 → −10.99 dB).
2. **Met.** F4's steady peak is 0.4449 as before. Every earlier test
   passes, the tuning, load and duct tables current.

## 8i. The wheel as the bellows

The user's idea (2026-10-01): the modulation wheel as where the bellows is,
not how hard it is pushed. 0 is the bellows shut, 127 open its whole
travel; moving the wheel moves the air, and a wheel standing still holds
the bellows still, so a key pressed then does not sound. It is how an
accordionist's arm works: what sounds is the bellows moving.

**The physics.** The arm becomes a source of flow, Q = Ẋ × travel, the
wheel's speed times the air its whole range moves (Bellows Travel, 12 L).
The air in the bellows takes what the reeds, the leaks and the air button
do not, the same compliance as milestone 8e's, C = V/(ρc²):

```text
C P' = Q − Q_reeds(P) − Q_vent(P)
```

So the pressure is whatever pushes that flow through what is open: one
note at a given speed sounds louder than a chord at the same speed, as an
accordionist must move faster for a chord. A bellows with nothing open
takes the wheel's air into its own compliance -- the arm compresses it, the
"finger attack" a player prepares -- up to the most the arm can push: the
ceiling, by Hill's law at that speed, as in the arm's other mode. The
moving half's mass smooths the flow over the arm's 10 ms (8e). Which way
the bellows moves is which way the wheel moves, up opening (pull), down
closing (push); turning, the pressure drains through zero and only then
builds on the other side, as long as the reeds take to drain it. Bellows
Direction, Auto Reverse and Reversal Time are the other mode's; the ends of
the wheel are the ends of the bellows.

**The wheel's steps.** A wheel sends 128 positions. At 12 L each is 94 mL;
delivered at once each would compress the bellows by γP₀ ΔV/V ≈ 1.1 kPa,
a stutter at every step. So the position is not the input; the speed is,
measured between messages: the last step over the time it took, held until
that time has passed again, then no more than the step over the time since
(a wheel that has not moved another step since cannot be moving faster),
and still once four of its intervals pass with no step. (As built, the last
two steps when they went the same way: see the status.) The first step
after the wheel rested is taken over at most 0.2 s, assumed, so a bellows
starting from rest answers at once. A jump of more than a sixteenth of the
range after a rest is the wheel being placed, not moved, and moves no air.
A low half (CC 33) arriving within 3 ms of its high half refines that step
rather than making one.

**The design:** a parameter "Mod Wheel": Pressure (the default, milestone
8f as it is) or Bellows. In Bellows, the wheel takes the bellows over once
it is heard, as Expression does; velocity no longer moves it, and
Expression, moved, takes it back.

**Predictions** (written before building it, 2026-10-01):
1. Bellows mode, the wheel heard and still: a key pressed does not sound;
   the bellows' pressure stays at zero.
2. A 7-bit wheel moving at a steady 4 steps a second, one true 8′ held:
   after the attack the pressure stays within ±5 % of its mean -- its steps
   on the frames they fall on, or on a host's 256-frame blocks. The same
   with the low half sent in the same frame or a millisecond later.
3. Moving the wheel up then down with the note held: the pressure is below
   zero (pull) while it rises and above (push) while it falls, crossing zero
   once at the turn, and the push reed sounds after it.
4. At the same wheel speed a four-note chord holds a pressure at least 6 dB
   below one note's.
5. The wheel stopping: the pressure falls below the reeds' 20 Pa start
   within 1.5 s of the last step.
6. Moving the wheel with nothing open: the pressure rises, but never above
   the ceiling.
7. Pressure mode is unchanged: every earlier test passes.

**Status (2026-10-01): built (0.10.1); all seven met** (`tests/milestone_8i.rs`,
`motion.rs`, the plugin's `the_wheel_can_be_where_the_bellows_is`).
1. **Met.** The wheel heard at 64 and still, C4 pressed: the pressure and
   the output stay exactly zero for 2 s.
2. **Met.** One true 8′ at 4 steps a second: -169 Pa, within ±2.34 % on
   the steps' frames, ±3.37 % on 256-frame blocks, ±2.34 % with the low
   half in the frame or 1 ms later. The spread left is the reed's own
   ripple in the bellows.
3. **Met.** Up 3 s then down: pulling throughout the rise, pushing
   throughout the fall, one crossing; the push reed swings 3.0 mm after it.
4. **Met.** Note 169 Pa, C4, E4, G4 and C5 together 58 Pa: -9.3 dB.
5. **Met.** 314 Pa while moving; under 20 Pa 1.02 s after the last step.
6. **Met.** Nothing open, the wheel falling 20 steps a second: 862 Pa, the
   ceiling 1 kPa.
7. **Met.** Every earlier test passes; with Pressure, the wheel goes to
   Expression's path exactly as before.

Found in building, not predicted:
* The first build took the wheel's position at seven bits as n/127 and, once
  a low half came, as n·128/16383: the change of scale was a false step,
  over an interval of zero, and drove the bellows to its ceiling. Both are
  now one 14-bit scale, and a position refined before any step moves
  nothing.
* With the last step alone, a hand stepping unevenly -- each interval ±20 %
  at random -- swung the pressure ±32 % at worst. Measured over the last two
  steps when they go the same way, ±27 % at worst and ±9 % typical: the
  flow such a hand moves. A turn is still taken alone, so a shake answers
  at once.

Heard: `scores/wheel-bellows.score` (C4 held still and then moved; "Frère
Jacques" in Musette, pulled then pushed; a shaken chord), sent to the
player.

**Heard by the player (2026-10-01): the sound breaks up when the wheel
moves slowly.** Measured (`diagnose_a_slow_wheel`, renders at 1-4 steps a
second): at 2 and 4 steps a second the level holds within 3 dB; at one step
a second, steady as a machine, it falls 22 dB below its median a quarter of
the time. The cause is the speed's rule, not the steps: a step after a rest
is taken over at most 0.2 s, and the wheel was taken as still after four of
*that* interval, 0.8 s -- so a wheel slower than one step in 0.8 s rested
before every step, and each step was a burst of air 5× too fast followed
by nothing: ~260 Pa then zero, once a second.

**The repair:** a step after a rest is not measured with the step before,
and the wheel is taken as still only once its own steps stop -- four of its
measured intervals -- or, after a step from rest, once no step comes for
2 s. The guess for a first step becomes 0.5 s (assumed), so a slow start is
not a burst; a fast start is measured right at its second step.

**Predictions for the repair** (written before it, 2026-10-01):
8. At one and two steps a second, steady or each interval ±30 % at
   random, a held true 8′ never falls 6 dB below its median after its
   first two seconds.
9. From rest, a wheel moving 20 steps a second sounds -- the pressure past
   the reeds' 20 Pa -- within 120 ms of its first step.
10. Predictions 1-7 still hold.

**Status of the repair (2026-10-01): built (0.10.2).**
8. **Met, but for the hand at the reed's threshold.** Lowest level after
   two seconds under the median: two steps a second -0.1 dB steady, -3.2 dB
   ±30 % uneven; one step a second steady -1.6 dB. One step a second ±30 %
   uneven: not met, -53 dB -- and not the measurement's doing. At 12 L of
   travel one step a second is 94 mL/s, and C4's true 8′ sits at 16-30 Pa,
   its threshold: when a step comes late the pressure falls under it and
   the reed stops, as a reed at its threshold does. The prediction mixed the
   rule's fault with the reed's threshold; only the first was the repair's
   to remove. The wheel moves more air per step with a longer Bellows
   Travel.
9. **Met.** From rest at 20 steps a second the pressure passes 20 Pa 27 ms
   after the first step.
10. **Met.** Predictions 1-7 as before (5: 1.01 s).

## 9. The product

A schema 3 package with branding, a PLAY surface, and factory programs.

## 9a. The PLAY surface

RackForge hosts a plugin's own web page in a sandboxed frame and talks to
it over `rackforge.plugin.web@1` (RackForge `docs/WEB_PLUGIN_API.md`). RF-5's
surface is the model (`rackforge-plugin-rf-5`, `docs/UI_ARCHITECTURE.md`): a
Rust WebAssembly client of that bridge, a static panel map that a test holds
to the parameter table, RackForge's own program selector and save dialog in
the page, and knobs that drag by pointer capture with a native range input
under them for the keyboard. RF-Musette takes that architecture, not RF-5's
look.

One surface, PLAY. RackForge keeps CONFIG for a workflow apart from playing
(files, tapes); an instrument with nothing to load has none.

The front page is what an accordionist reaches for: the treble's 14 registers
and the bass side's 7 as a row of switches, each with the symbol Roland prints
for it (FR-3x Owner's Manual pp. 27, 30) -- a circle cut in three for the
treble, L under, M−, M, M+ across, H over; cut in four for the bass, 2′, 4′,
8′, 16′ downward and the 8-4′ at the rim; then the left hand, the bellows (its
direction, the air button, Auto Reverse and its travel, the smoothing) and the
voice (gain, tremolo, cassotto, A4). The model's three pages follow, as
knobs.

**Predictions, as tests:**

1. Every public parameter appears on the panel exactly once.
2. The register symbols are drawn from the engine's own register tables, not
   a copy: each has as many dots as its register opens ranks, where Roland
   draws them.
3. A knob follows its parameter's taper: on a logarithmic one equal travel is
   an equal ratio and the middle is the geometric mean; value to position and
   back returns within one step; an arrow key always moves at least one
   step.
4. In the preview host (`tools/ui-preview.html`) the page says `ready`, reads
   the parameters, sends a set for a turned knob and a chosen register,
   follows a `parameter_changed` from outside, and logs no error.
5. The air button opens the air valve while held and closes it on release,
   also when the pointer is cancelled or the page loses it.

**Controllers.** RackForge maps a hardware controller's knobs by meaning,
not by number (RackForge `docs/MIDI_PARAMETER_LINKS.md`, Control Profile
v1): a controller package names what a knob is for -- most of the 64
bundled ones give knobs and faders `synth.envelope.amp.attack`, `decay`,
`sustain`, `release`, `synth.filter.cutoff`, `resonance`, `envelope.amount`
and `synth.lfo.rate` -- and a plugin names which of its parameters answers to
each role. A role is published only where the accordion has the thing it
names, as RF-5 does (its `docs/RACKFORGE_CONTROL_MAPPING.md`): the attack is
the pallet opening (the finger attack, Llanos 2015 p164), the release its
closing, the LFO rate the tremolo's beat. The rest stay unbound, and every
control on the panel can still be linked by hand from RackForge's menu.

RackForge's LITTLE screen shows a plugin's parameters by the schema's pages,
in the schema's order; the schema takes the panel's pages and order, so the
registers and the bellows come first there too.

6. The parameter schema passes RackForge's own validator, at schema 2, with
   the attack, release and LFO-rate roles on Pallet Opening Time, Pallet
   Closing Time and Tremolo, and no role on a control that is not what the
   role names.
7. Every parameter's schema page is its panel page, and within a page the
   schema's order is the panel's.

**Status (2026-10-01): built (0.10.0); all seven met.**
1. **Met.** `panel.rs` places the 45 parameters once each (a test).
2. **Met.** The 14 treble and 7 bass symbols come from `register_ranks` and
   `bass_register_ranks`, the engine's tables; dots counted and placed as
   Roland draws them, no two alike (tests).
3. **Met.** On every logarithmic knob the middle is the geometric mean
   within a step and the first and last quarters of travel multiply alike
   within 5 %; every knob returns its value within a step; an arrow moves at
   least one step everywhere and stops at the ends (tests).
4. **Met** in the preview host: `ready` answered, one `plugin.parameters`,
   a set for a register (bass register 4), a dragged gain (1.67, 2.31) and a
   toggle (Auto Reverse); a register and the gain set from outside shown
   with nothing sent back; arrows and pages on a knob (4.10 → 4.25, 4.40,
   2.90, 15.00 Hz); the model pages; at 375 px no sideways scroll; the stage
   light; no error logged.
5. **Met** in the preview host: the air button sends 1 while held and 0 on
   release, on a cancelled pointer and on the page losing focus.
6. **Met.** RackForge's own `ParameterSchema::validate` takes the schema at
   version 2, with the three roles on Pallet Opening Time, Pallet Closing
   Time and Tremolo and none on a level (contract test); `rackforge-core
   inspect` and `smoke` pass the package with its `web_ui` surface.
7. **Met.** Every parameter's schema page is its panel page, and the schema's
   order the panel's (a test; the schema is written from the panel map).

Not yet heard in RackForge Desktop by the player: the surface was driven in
the preview host, not in the Desktop frame.

Not in 9a: a live bellows gauge (it needs read-only meter parameters, which
RackForge polls like the rest), the branding (schema 3 artwork) and the
factory programs.

## 9b. The microphones and the room

The user's ask (2026-10-01): the instrument heard as it is recorded, from
where it sounds to where the microphone is, as Concert Grand takes the piano
-- with the ways accordions are actually miked to choose from, each with its
own settings.

**What practice says** (research 2026-10-01; SOURCES.md, "The
microphones"): engineers hear the accordion as two sources, the treble grille
and the bass box, the second moving with the bellows. In the studio: one mic
per side at about 30 cm (Shure; Piovesan, his usual), a coincident ORTF pair
at 1 m (Piovesan's reference recordings), a spaced pair a little forward
(Sound On Sound), or one mic 30-90 cm in front (Shure's booklet; Owsinski).
On stage: mics on the instrument, either inside -- a bar of 2-6 capsules
over the treble's reed blocks and 1-2 in the bass box (Rumberger, Nalbantov,
MusicTech) -- or clipped outside, two goosenecks about 5 cm from the treble
grille 25-30 cm apart and one 7.5 cm from the bass side (K&K, DPA 4099). The
stage mics travel with the instrument, so the bass side's level stays put
("slightly artificial", Piovesan); the studio's do not. No directivity of an
accordion has been measured (neither TU Berlin's 41 instruments nor BYU's
set has one); a free reed radiates as a monopole, the flow through its slot
(Nussbaumer & Agarwal, ICA 2016).

**What Concert Grand teaches** (rackforge, `plugins/concert-grand`): a room
from Sabine's reverberation per band into a feedback delay network, first-
order image sources read by each capsule, a capsule's pattern (1 − b) + b
cos θ from omni to figure-of-eight with its diffuse pickup √((1 − b)² +
b²/3), and the distance law. What not to take: its board-to-mic and room
stages place the same pair in two places, and its proximity shelf is
computed and never heard. RF-Musette's room is its own.

**The design.**
* **Two sources in space.** The treble side as four sources along its reed
  blocks, the keyboard's quarters -- the reed blocks run the keyboard's
  length, low notes at the top -- and the bass side as one, its box. Each
  radiates the flow of the holes it covers, the monopole the model already
  has. The bass side's position is the bellows' opening: the air it has let
  through over the bellows' area (12 L over 600 cm², 20 cm), or the wheel's
  position as the bellows (8i). Each source has a broad first-order
  directivity toward where its grille faces; assumed, as no accordion's is
  measured.
* **Capsules.** Each has a position, an aim, a pattern, and a frame: the
  room's (a stand) or the instrument's (mounted, inside or clipped), moving
  with the treble side or the bass box. The direct path is a fractional
  delay of each source, so moving sources and capsules change arrival time
  and level as distance does; Doppler is that and nothing more (at bellows
  speeds, 0.1-2 cents).
* **The room.** A box of the chosen volume and wall hardness: Sabine's RT
  per band, first-order images read by each capsule from the instrument's
  centre, and a feedback delay network for the tail. Concert Grand's
  equations; RF-Musette's own code, no_std, f64 where it counts.
* **Six layouts, each with its own settings:** Internal; Clip-on; Two
  spots (with an ambient ORTF pair to blend, Piovesan's four-mic take); ORTF
  pair; Spaced pair; One mic in front. Each layout's capsules are trimmed so
  switching layouts keeps the loudness -- the engineer's preamp -- and a
  Perspective sets whose left is left: the player's or the audience's.
* **The dry instrument stays.** The mono render at 1 m, every earlier test
  and measurement, is unchanged; the microphones and the room are a stereo
  stage after it.

**Predictions** (written before building it, 2026-10-01):
1. The dry render is unchanged: every earlier test passes.
2. A stand mic, the room off: opening the bellows moves a held bass note's
   level by 20 log₁₀ of the distance ratio within 0.5 dB, and its arrival by
   the path's change over c within one sample.
3. A mounted bass-side mic, the room off: the bass note's level does not
   move with the bellows, within 0.1 dB.
4. A bass note heard on a stand mic while the bass end moves at a steady
   speed toward it shifts up by v/c (in cents, 1200 log₂(1 + v/c)) within
   20 %.
5. Internal, two treble capsules at the reed blocks' ends: the notes under
   them are at least 3 dB louder than one between; with six, the spread is
   less than half of that.
6. The room's tail decays at the RT60 Sabine gives for it, within 15 %, in
   the middle band.
7. Each layout's loudness of a reference -- a held treble chord and bass
   note -- is within ±3 dB of the ORTF pair's.
8. The player's perspective puts the treble to the right and the bass to
   the left (by the level between the channels); the audience's the
   reverse.
9. The room's level against the direct sound follows Sabine: an omni
   capsule at the critical distance √(A/16π) (A the room's absorption area)
   hears the tail as loud as the direct sound, within 2 dB.

**Status (2026-10-01): built (0.11.0); all nine met** (`tests/milestone_9b.rs`,
`stage.rs`).
1. **Met.** The mono render is the instrument alone, as before; every
   earlier test passes.
2. **Met.** Two spots, omni, the room off: the bellows shut to open brings
   the bass box from 0.377 to 0.240 m of its stand, +3.90 dB against the
   distance's +3.93, and its arrival 19.0 samples sooner against the path's
   19.2.
3. **Met.** Clip-on: the bass note's level moves 0.000 dB with the
   bellows.
4. **Met.** The bass box moving 0.5 m/s, the path to the stand shortening
   0.336 m/s: +1.694 cents heard, +1.696 from the path.
5. **Met.** Internal, two treble capsules: the quarters under them 7.4 dB
   over those between; six capsules, a spread of 1.5 dB against 7.4.
6. **Met.** The tail in the middle band: 0.65 s against Sabine's 0.66
   (150 m³, hardness 0.4), 1.53 s against 1.57 (600 m³, 0.6).
7. **Met.** A held chord and bass note, against the ORTF pair: Internal
   -1.6 dB, Clip-on -0.1, Two spots +0.5, Spaced pair -1.0, One mic -1.1,
   Dry +2.2.
8. **Met.** The player's perspective puts a treble quarter 0.8 dB to the
   right and the bass box 1.8 dB to the left in the ORTF pair; the
   audience's the reverse.
9. **Met.** One omni mic 1.01 m from a treble quarter, in the middle band:
   the room 1.4 dB under the direct sound, Sabine's 1.4.

Found in building, not predicted:
* The tail's network was fed the same signal in all six lines; that is the
  Householder mixer's own vector, which keeps the lines in step. Fed with
  alternating signs it gives back its input within 1 dB (1.15-1.19 of it,
  the loop filters off; `the_tail_gives_back_the_energy_it_is_given`).
* The first measures of 6 and 9 were the measurement's: Schroeder's
  integral taken over the direct sound, and a "noise" that was a multiplier
  on the sample's number, nearly periodic. Taken over the tail from 80 ms,
  and with real noise, both met as above.
* What the room costs (the four-note Master chord, natively): 5.73 µs a
  sample alone, 6.11-6.43 with a layout (+7-12 %), Dry +0. The stage stops
  computing once its room has fallen silent.
* The engine is 547 KiB with its microphones and room; its size test spoke
  of a 1 MiB stack the component left behind at milestone 7, and now bounds
  it at 768 KiB of the 8 MiB.

Heard: the same phrase through the seven layouts, sent to the player.


## 9c. Headroom

**Heard by the player (2026-10-01): it plays so loud it clips, and did
before the microphones.** Measured (`the_output_levels`): the engine's
output is pascals at 1 m, and the plugin sent it as is, 1 Pa to full scale
(94 dB SPL). A C4 at mf peaks at -4 dBFS; a chord and bass at mf +9.7;
both hands' full chords in Master at ff +15.8 dB over full scale, clipped by
the host.

**The design:** what an engineer does at the preamp. The stereo output --
the plugin's, the laboratory's `--stereo` -- is brought down by a fixed
recording level, set so the loudest the instrument plays peaks at -6 dBFS,
the headroom of a recording: -21 dB, full scale 115 dB SPL at the 1 m
reference. Output Gain stays the player's, ×0-4, and behind it a soft
ceiling: nothing under -6 dBFS is touched, and what rises above it rounds
toward full scale instead of being cut there. The mono render, every
measurement, stays in pascals.

**Predictions** (written before building it, 2026-10-01):
1. The loudest case, both hands' full chords in Master at ff through the
   ORTF pair, peaks at -6 dBFS within 1 dB; a C4 at mf near -25.
2. Under -6 dBFS the ceiling changes no sample; nothing it puts out
   reaches full scale.
3. The mono render is unchanged: every earlier test passes.

**Status (2026-10-01): built (0.11.1); all three met** (`tests/milestone_9c.rs`).
1. **Met.** Both hands' full chords in Master at the ceiling, through the
   ORTF pair: -6.1 dBFS; a C4 at mf -26.3 dBFS. The level is -22 dB, full
   scale 116 dB SPL at 1 m -- first -21 dB, measured again after 9d's room
   changed each layout's balance and its peaks.
2. **Met.** Under 0.5 of full scale the ceiling returns every sample as it
   came; above, it rises steadily toward 0.98 (-0.2 dBFS), so even a float
   whose tanh rounds to one stays short of full scale (the first build
   reached it at 4.7 times full scale).
3. **Met.** Every earlier test passes.

## 9d. A diffuse room

**Heard by the player (2026-10-01): the ambience rebounds, as if in a
closed resonating tube.** Measured (`diagnose_the_room`, the room's own
impulse response through an omni at 1 m, 150 m³): the tail's spectrum
ripples 16.8 dB about its third-octave smoothing between 300 Hz and 4 kHz,
against the ~5.6 dB of a diffuse field, with 173 peaks 8 dB proud of it at
a median spacing of 18.3 Hz -- a tube's regular resonances; its echo
density (Abel & Huang's, 1 for a diffuse field) is 0.10 at 5 ms, 0.22 at 15
and 0.63 at 30. The cause is the tail's network: six lines of 6-14 ms, 60
ms in all, one mode in every ~17 Hz, where a room of 150 m³ has dozens in
every hertz. The first reflections make a comb with the direct sound too
(28 dB deep), the walls reflecting every frequency alike.

**The repair:** the network a reverberator needs to sound diffuse.
* Eight lines, from one to four mean free paths long and sharing no
  divisor: some 200 ms in all at 150 m³, a mode in every ~5 Hz.
* An allpass inside each line, so the echoes multiply at every pass, and
  four short allpasses before the network, so the field is diffuse from
  its first milliseconds. Allpasses lose nothing: Sabine's decay and level
  stand, each line's gain reckoned over its whole loop.
* The first reflections take the walls' high band: each capsule's images
  pass a one-pole whose loss at 4 kHz is the walls' there against the
  middle band.

**Predictions** (written before it, 2026-10-01):
1. The tail's ripple between 300 Hz and 4 kHz is within 1.5 dB of a
   diffuse field's 5.6 dB, at 30, 150 and 1500 m³.
2. Its echo density reaches 0.9 by 50 ms.
3. Milestone 9b's predictions still hold: the decay within 15 % of Sabine's,
   the room's level within 2 dB.
4. The stage costs at most 15 % over the mono render.

**Status (2026-10-01): built (0.11.1); 1, 3 and 4 met, 2 at two rooms of
three** (`tests/milestone_9d.rs`, `stage.rs`).
1. **Met.** The tail's ripple: 7.0 dB at 30 m³, 6.6 at 150, 6.9 at 1500
   (was 16.8 at 150), a diffuse field's 5.6 measuring 5.7 on the same
   analysis; no regular peaks.
2. **Met at 30 and 150 m³, not at 1500.** Echo density at 50 ms: 0.91,
   0.95, 0.79 (was 0.63 at 30 ms at 150). In the large room the first
   images still arrive apart at 50 ms. Kept, ignored, in the test.
3. **Met.** The decay 0.67 s against Sabine's 0.66 (150 m³) and 1.47
   against 1.57 (600 m³); the room 0.8 dB under the direct sound at 1.01 m
   against Sabine's 1.4.
4. **Met.** +10-15 % over the mono render (Internal 15 %), nothing when
   silent.

How it was reached, as measured:
* Eight lines alone gave 11.2 dB: a network of 0.2 s holds a mode in every
  5 Hz, and a decay of 0.66 s gives each mode 3.3 Hz -- they do not
  overlap. A diffuse tail needs ~1 s of delay; memory holds ~0.6.
* Long allpasses inside the loops (13-53 ms) added delay but dispersed it:
  some frequencies stayed in the loop four times longer and the decay ran
  29 % long against Sabine. Short ones (3-7 ms) keep the decay.
* What memory cannot hold, motion stands for: each loop allpass's delay
  wanders ±0.3 ms at 0.3-0.9 Hz -- at most ~2.4 cents of pitch, the
  reverberator's means (Dattorro 1997) of letting few modes stand for a
  room's many. Not the room's physics; stated as such in MODEL.md.
* The lines: four short (7-23 ms) for the onset, four long (83-163 ms) for
  the modes. Each side of the output takes two of each; one side of the
  long ones alone was 4 dB quieter, as long lines lose more at each pass.
* The shelf lifting the lows is second order: a first order's skirt
  reached the middle band.
* The first reflections take the walls' high band, a one-pole per capsule.

## 9e. Factory programs

The user's ask (2026-10-01): variety -- the package had one program.
Research first (SOURCES.md, "The programs"): how each tradition tunes its
tremolo, which registers and cassotto it uses. The tremolo values come from
Roland's V-Accordion musette detunes, which Roland does not publish but two
players measured in cents on an FR-3s and an FR-3, agreeing on French
(±23 c) and Scottish (±26-27 c), and from makers' and tuners' tables
(Victoria, Castagnari, Liberty Bellows, Pellegrini, Weirig, Dumpleton).
None gives a rule for how the beat runs across the keyboard by tradition;
every program keeps Hergert's measured lines.

**Built (0.12.0):** sixteen programs in three banks, a table in the engine
(`programs.rs`) that `rf-musette-lab schema` writes into `presets.json`.
* Factory: Accordion, the defaults (its id, `research`, kept).
* Styles: Musette Paris (Musette, 5.9 Hz), Scottish Dance Band (Musette,
  6.7 Hz, clip-on mics on stage), Italian Folk (Musette, 4.0 Hz), Alpine
  (Cello, 3.3 Hz), Oberkrainer (Musette, 3.0 Hz, cassotto), Cleveland Polka
  (Musette, 0.5 Hz, internal mics), American (Celeste, 2.5 Hz), Irish Swing
  (Celeste, 1.2 Hz), Jazz (Bandoneon, cassotto), Tango (Oboe -- a
  bandoneon is tuned in dry octaves, 8′ and 4′), Concert (Harmonium, dry,
  cassotto, an ORTF pair in a hall); each with the room and microphones of
  where it is played.
* Setups: 61-Key Keyboard (the split at C4), Wheel as Bellows, Auto
  Bellows, Digital Accordion (the bellows' pressure direct, bass on its
  channels; not "V-Accordion", Roland's trademark).

**Checked:** every setting is a value its parameter takes, ids and names
unique, descriptions for the player (`programs.rs`); the catalog is the
table and loading each program sets what it says (the plugin's
`the_program_catalog_is_the_engines`); RackForge loads them.

**The Accordion register, checked against Roland:** the FR-3x manual's
symbol (p. 27), the source of `REGISTERS`, draws Accord as L, M−, M, H; the
FR-4x's reed tables (Supplementary Manual 2017, set NOR CASS) give L, M,
M+, H. Roland's own instruments differ by model and set; `REGISTERS` keeps
the FR-3x's, the manual it cites. The difference is which side the second
8′ beats on.

Heard: the same phrase through Accordion and the eleven styles, sent to the
player.

## 10. The cost of the audio path

The user's ask (2026-10-01): the audio costs too much; cut it without
losing the sound or the behaviour, by mathematics that answer bit for bit
where they can, and with several cores, as RF-5 did (its methods:
`rackforge-plugin-rf-5`, the audition fingerprints, the parallel render).

**Measured first (x86-64, this server, native, release):** milestone 7's
four-note Master chord costs 6.26 µs per 48 kHz sample (30 % of real
time); both hands 8.85 µs (43 %); the stereo stage adds 0.8-1.0 µs. By
part (`tests/cost.rs`): a reed step 71 ns, and the chord steps 20 reeds at
two substeps a sample -- 2.85 µs, 46 %; the wind 66 ns a substep; the
decimator 191 ns a sample per source (five in stereo); the stage 413 ns a
sample. Some 3 µs were in none of these: the per-key loop.

Found in the code:
* The pallet's curtain takes the rim of its hole, `rim(hole_area)` -- a
  square root, in software, of a number fixed by the design -- at every
  substep for every rank.
* Every key is asked whether it is idle at every substep, which compares
  all ten of its reeds' states field by field: 53 keys, ~3000 comparisons
  a substep while four sound.
* `math::sqrt` is a software square root written for control rate ("never
  per sample", it says) that the reed step calls every substep. It is not
  the correctly rounded root: 1 ulp off for a quarter of the engine's
  arguments, and wrong for subnormals (`the_software_square_root_against_
  the_hardwares`).
* The decimator walks a ring buffer with a branch per tap.

**The guard (10a, built):** 24 scenes (`scenes.rs`) -- every program, mono
and stereo, the three oversampling factors, 44.1/48/96 kHz, the bellows by
velocity, expression, the wheel as pressure and as the bellows, turning by
itself, a reversal, the air button, the idle path and a hall's tail --
each rendered and folded into an FNV-1a fingerprint of every sample's bits,
taken at dd7e761 (`tests/fingerprints.rs`). A change meant to be exact
must leave all 24. One that is not is measured against the reference
renders: samples that differ, the peak difference and the error's energy
against the signal's, and the same over the last tenth of each scene, so
a difference that grows shows.

**10b, exact (predictions):**
1. The rim kept in the reed's model, the idle keys kept in a list, the
   loop-invariant arithmetic taken out of the loops, the decimator
   without its branches: all 24 fingerprints unchanged.
2. The four-note Master chord at most 4.5 µs per sample natively (−28 %),
   the ORTF stereo of it at most 5.0 µs.

**10c, not exact (predictions, before the code):**
3. The correctly rounded square root (the hardware's: `f64.sqrt` in wasm)
   in the reed step: every scene's error at least 120 dB below its signal,
   and no more than 6 dB worse over the last tenth than over the whole --
   a rounding, not a drift.
4. Each such change heard by the player before it is kept.

**Prediction 3, NOT MET as written -- and the prediction was the wrong
measure (measured 2026-10-01).** With the correctly rounded root the
scenes leave their references by −30 to −60 dB of waveform, not −120. The
cause, measured: the instrument's own sensitivity, not the root. Each
scene is bit-identical until a moment (0.36 s in Accordion), its first
difference one f32 ulp (−150 dB), and from there the difference grows to
−60 dB within 50 ms and wanders near −50 dB. The engine as it was, with A4
moved one ulp from 440 Hz and nothing else, leaves itself the same way:
identical to 0.35 s, −62 dB at 0.40 s, −47 to −57 dB after
(`how_a_one_ulp_difference_grows`). The reeds coupled through the bellows
amplify any rounding; no change that is not bit-exact can stay near its
reference's waveform, however exact it is -- this one is more exact than
what it replaces. Where the bellows takes no rounding of the root (the
expression scenes) the scenes did not move at all.

**Prediction 3, as it should have been asked:** a change that is not
exact is the same instrument when it moves each scene no further than a
one-ulp nudge moves it. Measured on what is heard: band levels, a third of
an octave wide, 50 ms at a time, both channels, against the reference's;
the 99th percentile of the level differences, where the reference band is
within 60 dB of the scene's loudest. The change's at most 1.5 times the
nudge's, plus 0.2 dB.

**10d, several cores:** RackForge renders a plugin across cores in units
(`docs/PARALLEL_RENDER.md` in RackForge): a coordinator's `begin_block`,
units with no shared state, an `end_block` that sums them in order, and
the same bits on any number of workers. Every reed here is coupled to
every other through the bellows at every substep -- the air one draws
lowers the pressure all are blown with. Units that run a block alone must
see that coupling a block late, or not at all; Concert Grand took its
sections' coupling a block late and kept its soundboard serial. Which the
accordion can afford is measured before it is proposed, and is the
user's call: it changes behaviour.

**Built, exact (10b) -- all 24 fingerprints unchanged:**
* The reed's model keeps what every step asked again: the hole's rim (a
  square root per rank per substep), ω², ω/Q, the swing limit's ρ, the
  section table's span.
* The keys with anything to compute are listed once per render call:
  between calls is the only time a key is pressed, and computing a key
  that has fallen idle changes nothing.
* The decimator reads its history from a doubled buffer, without a branch
  per tap -- little gained (191 to 176 ns): its cost was the chain of
  additions, kept in order.

Prediction 1 MET; prediction 2 MET: the four-note chord 6.26 to 3.16 µs
(−49 %), both hands 8.85 to 4.99 µs, the ORTF stereo 6.80 to 3.71 µs.

**Built, not exact (10c) -- each scene moves no further than a one-ulp
nudge moves it (prediction 3 as revised: all 24 "same"):**
* The correctly rounded square root (`math::sqrt`): WebAssembly's
  `f64x2.sqrt` lane in the plugin (the scalar one is not yet stable in
  Rust), the standard library's natively -- the same bits on every target.
* Reciprocals where the step divided by the same number again: the 2x2
  determinant, the cell's and the hole's rows, the tongue's width and modal
  mass, the section table's scale; the pallet's ρ/(2α²A²) kept while the
  curtain does not move. Twelve divisions a step become five.
* The decimator sums in four lanes, which WebAssembly's SIMD runs side by
  side: 176 to 45 ns. Its output feeds nothing back, so its rounding stays
  where it is made: −129 to −133 dB in the scenes the bellows does not
  amplify.
* A reed behind a shut pallet stops once its energy is negligible (below
  1e-12 J, ~98 dB under a sounding reed), as an unblown one always did: the
  shut pallet seals its cell and the bellows' pressure no longer reaches
  it. It used to be computed until it underflowed -- tens of seconds of
  silence at the cost of a chord.

Measured, natively: the four-note chord 2.35 µs a sample (−62 %), both
hands 3.93 µs (−56 %), ORTF 2.72 µs (−60 %), silence 1.03 to 0.12 µs.
Through RackForge as the plugin runs (WebAssembly, `tail-cost`, x86-64,
128-frame blocks, deadline 2667 µs), dd7e761 against now, twice each:
a five-note chord 1245 to 436 µs a block (47 % to 16 % of the deadline),
its ring-down 770 to 360 µs, silence 697 to 34 µs, the worst block 6.8 to
2.4 ms. That worst block is the first press of a key: its reeds' models
are built then, milliseconds each -- on the Pi, a dropout.

**10d, measured before proposing:** the reeds' coupling through the
bellows taken one block late -- what units on several cores would need --
emulated in the serial engine: every scene whose bellows takes the reeds'
draw moved 2.6-14.5 dB in its band levels (p99), against 0.0-0.4 dB for a
nudge, several by more than their own signal: the delayed loop rings.
Only Digital Accordion, whose bellows gives exactly the pressure asked,
stayed the same. The emulation was not kept.

## 8j. The start at an opening

The user's report (2026-10-01): a short "chick" at the start of every
opening of the bellows -- "the sound begins from a certain level, a jump
from nothing to it".

**Measured first** (`tests/opening_click.rs`, the wheel as the bellows, F4
held, dry, the wheel opening from rest a step every 10 ms): the high band
shows nothing, but the envelope does. The bellows' pressure rises from 0
to ~600 Pa in ~12 ms; when the cell passes the kick's threshold (20 Pa,
at 11-12 ms) the output jumps from −69 dB of the steady tone to −33 dB
within a millisecond, then grows smoothly. With the start kick off there
is no jump -- and no start within 40 ms either.

The cause: a reed is given the share P/(P + P₀) of its start the moment its
cell passes P₀. A key pressed into a blowing bellows meets ~400 Pa at once,
and 0.95 of the start at once is what was meant (8h): the air arrives
whole. A bellows that starts to move meets P₀ itself first, and is given
0.57 of the start at 27 Pa -- the start of a full note, at a pressure whose
tone is ~30 dB quieter.

**Change:** the share (P − P₀)/P -- nothing at the threshold, the same as
before where a note starts (0.950 against 0.952 at 400 Pa), the rest given
as the pressure rises, as before.

**Predictions:**
1. An opening from rest: no millisecond more than 12 dB above the one
   before it in its first 40 ms (36 dB before).
2. The reed still starts: the level 40 ms into the opening within 3 dB of
   before (−21.8 dB of the steady tone).
3. A note's start into a blowing bellows unchanged: every test of 8h and
   of the earlier attacks passes as it stands.

## 8k. The release

The user's report (2026-10-01): a noise as a note is let go -- heard on
real accordions, sometimes, but here always and strongly.

**Measured first** (`tests/opening_click.rs`, `the_release_of_a_note`, F4,
C5, A3 under a steady push, dry): the tone holds near its level for ~8 ms
after the key is let go, then collapses within 1-2 ms as the pallet's
curtain closes the last of the hole; in that millisecond the band above
3 kHz stands as high as the held tone's (A3: −1.3 dB) while the level is
−53 dB -- a burst, a click -- and from 12 ms the output is exactly zero.
Inside its sealed cell the tongue rings on: −10 dB at 50 ms, −20 dB at
120 ms, −30 dB at 280 ms (`the_curtain_and_the_sealed_reed`). The model
radiates only through the tone hole, so a ringing reed is silent.

**What the sources say** (SOURCES.md, "The release"): no measurement of
an accordion's release is published. Roland's patent (US6946594B2) and
FR-8x manual describe the reed ringing on after its valve shuts, "metallic
and partially distorted", fading exponentially, louder for low reeds and
higher pressure, and model the valve's closing noise apart. Accordion reed
plates ring with Q 200-400 unblown (Nussbaumer & Agarwal 2016): tens to
hundreds of milliseconds, not one or two. Blown, a reed radiates mostly as
the monopole of its pulsing flow; plucked without air, far more weakly.
Technicians: the release "clack" comes from the pallet's felt and leather;
internal microphones take it, distant ones hardly.

**Changes:**
1. The reed's flow into the bellows is heard too, through the bellows'
   walls: a second, muffled path that sounds while the note does and
   carries the tongue's ringing once the pallet has shut. Through a wall
   that obeys the mass law the radiated pressure follows the volume
   velocity itself (6 dB per octave below the hole's path); the walls pass
   nothing steady, a high-pass at 60 Hz. Its level is voiced: no
   measurement exists.
2. The pallet seats on felt and leather: over the last tenth of its
   travel it slows as the pad compresses (an exponential approach), so the
   hole is not chopped shut. Voiced likewise.

**Predictions:**
1. No millisecond after the release whose band above 3 kHz exceeds the
   held tone's while its level is more than 20 dB below it (the burst:
   A3 −1.3 dB at −53 dB).
2. A tail: 50 ms after the release the output lies between −50 and −25 dB
   of the held tone, and falls with the tongue.
3. The held tone moves no more than 1 dB in its band levels (p99).
4. Every earlier test of attacks and starts passes as it stands.
**8j, the start at an opening -- withdrawn (measured 2026-10-01).** The
share (P − P₀)/P took the 36 dB step out of an opening, at ~20 ms more to
reach the tone; predictions 1 and 2 were NOT MET as written (a 17 dB rise
remained at 10-11 ms, below −50 dB and there with no kick at all: the
corner where the wheel's flow starts; 3.8 dB quieter at 40 ms). Worse, it
was not limited to openings: with the bellows on velocity the pressure
rises from zero with the note too, and the attacks of every such scene
moved by up to 73 dB in a band; and a wheel at one step a second no longer
held its note -- held before by half-starts at every crossing of P₀, at
17-21 Pa, the very mechanism of the click. Spreading the start over
1-4 ms instead did not work: an impulse spread over a reed's period
partly cancels itself, and the reed was slower still. The old share is
back; the click at an opening remains open.

**8k, results (2026-10-01):** prediction 1 MET -- no millisecond after a
release with its band above 3 kHz over the held tone's while its level is
20 dB under it (the last: high band 23-38 dB under the held tone).
Prediction 2 NOT MET as written: 50 ms after the release the tail is at
−55 to −58 dB of the held tone, not −25 to −50 -- the level is voiced, and
the player hears 30 and 20 dB under the hole's path. Prediction 3 MET with
the felt taken only on the way down: the held tone 0.13-0.17 dB from the
reference, as a nudge moves it; taken both ways the felt slowed the
attacks and moved the held stretches 2.5-14 dB (measured by switching each
piece off). Prediction 4 MET: every earlier test passes.

What the click was, measured (`the_flow_as_the_pallet_seals`): not the flow
cut -- it falls to −55 dB before the seal -- but its rate of change: the
pallet's curtain ended the flow's fall in a corner, and the rate stepped
from a tenth of the tone to nothing; and as the curtain all but shut the
trapezoid flipped the hole's flow at every step. The θ weighting ends the
flipping; the seat on felt ends the corner; the path through the bellows
carries the tongue on after both.

**Heard (2026-10-01):** the player listened to the A/B of 10c (Accordion
and Musette Paris, before and after) and to 8k's releases, and kept them:
prediction 4 of 10 MET; 8k's tail kept at 30 dB under the hole's path.
The fingerprints were taken again for that sound.

## 9f. Instruments

The user's ask (2026-10-01): programs that are other accordions -- an
Italian 80-bass like the player's own among them -- not by taking reeds
from the panel but by their mechanics: sizes, air, build.

**Research first** (SOURCES.md, "The instruments"): makers' sheets give
size, weight and voices for every class; nothing published gives a
bellows' inside dimensions, a cell, a tone hole or a pallet spring by
class. What the programs take:
* The bellows' cross-section from the body's height and depth less ~3 cm
  of fold (derived): 26/48 ≈ 430 cm², 34/72 ≈ 560, 37/80 ≈ 600, 41/120
  ≈ 700, a cassotto professional ≈ 880 (Hohner, Paolo Soprani sheets). The
  default, 600 cm², is an 80-bass's. Its air in proportion (derived).
* The leak from technicians' drop test -- the open bellows closing under
  its own weight, keys up: 70 s tight, 35 s typical, 20 s leaky -- as an
  orifice: ≈ 10-15, 25-35, 50 mm² (derived). The default, 10 mm², is a
  tight instrument; a played student instrument 30-40.
* The same arm on a smaller bellows makes more pressure ("same force,
  smaller bellows", stated; derived as the force over the area): the
  ceiling 1 kPa × 600 cm² / area.
* Tremolo: 4 Hz / 15 cents "standard, German, Italian", 6 Hz "old
  Italian" (Liberty Bellows' chart); 15 cents at A4 is 3.8 Hz.
* Voices by size (makers' sheets): 48 MM, 72 LMM, 80 MM or LMM, 120 LMMH,
  the cassotto professional five voices with the cassotto.

Not taken, for want of a number: plate thickness, the clearances of
machine reeds ("more tolerance", no figure), tone holes, pallet springs.

**Programs (a new bank, Instruments):** Student 48-Bass (MM: Celeste),
Student 72-Bass (LMM: Cello), Italian 80-Bass (LMM: Cello, Italian 4 Hz),
Full-Size 120-Bass (LMMH: Master), Cassotto Professional (Accordion through
the cassotto); each with the bellows, leak and ceiling of its size and the
microphones of where it is played.

**Predictions:**
1. Every setting is a value its parameter takes; the catalog is the table
   (the contract tests as they stand).
2. The arm on the smaller bellows sags further: under a full Master chord
   pushed hard, the 48-bass's pressure falls further below its ceiling, as
   a share of it, than the cassotto professional's.
3. Heard by the player before they are kept.

**8k, the bellows' walls derived (2026-10-01).** The player: the cut at a
release is still there. Measured: no burst now, but the tone falls 38 dB
within 3 ms as the pallet seats, onto a tail at 30 dB under the hole's
path; a tail at −15 dB makes that fall 22 dB, a slower pallet (25 ms) only
delays it. The player asked that the tail's level come from something
real. It does: the bellows' walls are 0.8-1 mm manila card (makers;
`How to make Bellows`), solid board ~615 g/m² a millimetre, with a cloth
lining: m ≈ 0.75 kg/m². A limp wall passes 1/(1 + jωm/2ρc) of the
pressure on it -- the mass law, its corner at ρc/(πm) ≈ 176 Hz, −7 dB at
350 Hz, −15 dB at 1 kHz. The 30 dB voiced before was a wall of
~10 kg/m². The path becomes the tongue's own monopole, S_r ζ'', through
that wall: derived, not voiced. Neither the bellows' air between tongue
and wall nor the walls' area is in it -- a cavity and its walls make a
resonance, near 270 Hz by a rough estimate, which this leaves out.

**Predictions:**
1. After a release no 3 ms falls more than 25 dB (38 dB now).
2. The held tone moves no more than 3 dB in its band levels (p99): the
   path sounds while the note does too.
3. Every earlier test passes.

**8i withdrawn (2026-10-01).** The player: the wheel as the bellows does
not play well; the choice need not be shown. Taken out of the engine --
the wheel's motion (`motion.rs`), the driven bellows, the Wheel as Bellows
program, the 8i tests -- and the Mod Wheel parameter retired in its place
(index 45: states and links are by index, and parameters are only ever
added at the end), one value, Pressure, not on the panel, read-only in the
schema; a state saved with Bellows loads as Pressure (contract test
`a_state_with_the_wheel_as_the_bellows_still_loads`). Exact: every other
scene's fingerprint unchanged by the removal. 8j's click at an opening,
heard on the wheel, goes with it.

## 8l. The cell as a tube

The player (2026-10-01): the highs lack brightness -- perhaps equalisation,
perhaps the model. Measured first: a held F4, dry, keeps its harmonics 2-6
within 7 dB of the fundamental and then falls -- −24 dB at 2.8 kHz, −30 at
4.2, −47 at 5.6, −50 at 8.4 -- before any microphone; the microphones and
the room take only a few dB more. Halving the cell or doubling its hole
moved the fall up an octave (`tests/cost.rs` renders, 2026-10-01): the
cell's Helmholtz resonance, ~2 kHz, and its −12 dB/octave above, set it.

The sources (SOURCES.md, "The cell"): an A4 of a concert accordion, one
8′ reed outside the cassotto at 50 cm, keeps its harmonics within −1 to
−21 dB of the fundamental to ~4.8 kHz and −25 to −40 dB to 10 kHz
(Elejalde-García, Macho-Stadler & Llanos-Vázquez 2021, Fig. 1): the model
lies 10-20 dB under it from 3.5 to 9 kHz. A cell may be taken as a lumped
volume only while every dimension is under ~0.15 of a wavelength (Tonon,
PICA 2, 2005): a 50 mm cell, under ~1 kHz. Above, a tube.

**Change:** the cell becomes a tube, closed at the reed and open to the
hole -- a waveguide, its waves either way delayed by its length over c --
its length 1.4 times the tongue's (Tonon's G4 cell is 46 mm; an F4
tongue is 36 mm), its section the cell's volume over that length, so its
compliance at low frequencies is the volume's as before; the walls' losses
Kirchhoff's at 2 kHz (Fletcher & Rossing).

**Predictions:**
1. The held F4, dry: its harmonics from 2 to 5 kHz between −10 and −22 dB
   of the fundamental, from 5 to 9 kHz between −25 and −40 dB.
2. The low end as before: the tube's compliance at low frequency is the
   volume's (a test), and the F4's pitch within 3 cents of the lumped
   cell's.
3. The reed step costs no more than a quarter more.
4. Every earlier test passes, or says how the cell was in it.

**8l, tried and withdrawn (2026-10-01).** Built as above, the tube gave
the highs back: the F4's harmonics from 2 to 5 kHz at −9 to −25 dB
(mean −16, the measured A4's −18), from 5 to 9 kHz at −17 to −34 (mean
−25, the measured −30); the centroid 1.49 to 2.16 kHz. Prediction 1 met
in the mean, a few harmonics 5-8 dB over above 5 kHz. Prediction 2 NOT
MET: the F4 sounded 12 cents flat -- a short closed tube is the volume's
compliance and a third of its air as mass besides, as a lumped cell is
not -- which a maker's finishing would file out; but the finishing failed
(`rf-musette-lab tune`: a reed that does not speak). Prediction 4 NOT
MET, and the reason it was withdrawn: from ~1.3 kHz up the reeds no longer
spoke at 300 Pa (E6's middle reeds needed 1 kPa; with the volume, 300 Pa),
and the top piccolos not at all (`tests/tube_diagnosis.rs`, against
dd7e761) -- where real players start them softly. The tube, as joined to
the reed here (at the cell's closed end, the reed's flow into the tube's
impedance ρc/S at once), loads the short high cells' reeds wrongly; where
along the cell the reed sits, and how its slot's flow enters a cell that
is no straight tube, are not modelled, and nothing measured says how a
cell loads a reed at those frequencies. Reverted exactly (every scene's
fingerprint as before). The brightness stays open, its cause measured:
the cell, lumped.

## 8m. The cell as a tube, the reed along it

The player (2026-10-01): build the tube properly, as real as it can be.

What the sources give (SOURCES.md, "The cell"): the reed plate is one long
face of its cell; the cell opens through a hole in the block's foot at one
end; a reed is mounted with its rivet toward that opening and its tip
toward the closed end, except the highest -- from A#6 -- which makers turn
round, tip to the opening, because they speak better so (patents US2051621,
US5824927; technicians). Tonon (PICA 2005): in a cell long enough to be a
tube, a tip at the closed end hinders the reed, a tip at the opening helps
it. The slot's air enters along the whole tongue, through its sides, as
the tongue bends: nothing at the rivet, most at the tip (Misdariis, Ricot
& Caussé 2000, water visualisation; the mode shape). A blown-closed reed
is helped by an inertive load (Tarnopolsky, Fletcher & Lai 2000; Millot &
Baumann). And what a player meets: the low treble starts at 40-70 Pa, the
highest piccolos at 100-250, "at 300 Pa all notes speak" (a technician,
musiker-board 2014).

8l's tube put the whole slot at the closed end, where Tonon says it hinders
most, and the reed met the tube's impedance ρc/S at once: the high reeds no
longer started.

**Change:** the cell a tube, its opening at one end and closed at the
other, 1.4 times the tongue long; the tongue along it, rivet near the
opening and tip near the closed end (turned round from A#6, ~1.8 kHz); its
slot giving and taking air at four points along the tongue, each by the
tongue's mode shape there, and the tongue feeling those points' pressure
the same way. The section the cell's volume over the length, so the
compliance at low frequency is the volume's; Kirchhoff's losses.

**Predictions:**
1. The tube's compliance at low frequency is the volume's (a test).
2. Every treble reed speaks at 300 Pa, the highest piccolos too (the
   volume needs a kilopascal for them); none needs more than it did.
3. The held F4, dry: harmonics from 2 to 5 kHz between −10 and −22 dB of
   the fundamental, from 5 to 9 kHz between −25 and −40 dB.
4. Finished again (`rf-musette-lab tune`), every reed in tune, as the
   tuning test asks.
5. The reed step costs no more than a third more.
6. Every earlier test passes, or says how the cell was in it.

**On the way (2026-10-01).** The hole made the reported one: about a
centimetre, the same under every treble key (100 mm², a piccolo's 64), the
bass side's still 150 mm² scaled, assumed. Finishing then failed on a few
reeds about 1.5 kHz. The builders' remedy Tonon reports -- a shallower
cell -- was tried as a finishing step and made every one of them worse
(the G6 M− reed: 600 Pa in its scaled cell, 1 kPa in a third of it, never
in a fifth; 300 Pa in one and a half times it): in this model they are
not choked by their cell's resonance, so the step was not kept. Their
threshold rises smoothly with pitch instead (`tests/tube_diagnosis.rs`,
`the_thresholds`): the true 8′ speaks from 50 Pa to 311 Hz, 100 Pa to
587, 150 to 831, 200 to 1047, 300 to 1.3 kHz and 400 above; the piccolos
to 600 Pa at A6, the last not turned round, and 300-400 once turned. The
tuner had been tuning such reeds on a dying tone at 300 Pa, which moved by
its correction vanished: `tuning_pressure` now asks the reed to hold its
tone, as its documentation said.

Measured, finished again: prediction 3 met in the mean -- the held F4's
harmonics from 2 to 5 kHz at −12.5 to −28.9 dB (mean −18.7; the measured
A4's −18), from 5 to 9 kHz at −18.5 to −50.6 (mean −33; measured −30).
Prediction 2 NOT MET: ~25 of the 205 treble reeds need 400-600 Pa. The
tube lowers each reed further than the volume did, which the tuner files
out: 10-63 cents against 4-19. The low reeds speak more easily -- the
loads and ducts they need fell (the bass side's ducts from 9.1 to 2.5
times at most). A passage rendered against dd7e761 (`tube-ab.score`): the
tube 3-7 dB quieter note for note, its 4 kHz octave 6 dB and its 8 kHz
octave 25 dB up against 250 Hz. Prediction 6 NOT MET as it stands: the
attacks slower (finger attack 18 ms, bellows starts 32-46 ms, over their
bounds), the peaks louder, the L rank's octave 28 cents off, the reference
integrator and the convergence test still the lumped cell's, the energy
test failing by f32 rounding in the lines, and the engine over its stack
budget (1.2 MB) by the lines' 275 KB. Awaiting the player's ear before
any of it is worked further.

**The player (2026-10-02):** the tube approved by ear -- Greensleeves,
Mutopia's accordion setting (public domain), rendered both ways -- "but
there is a kind of squeal in the long notes, like a piano's beating".

Measured: a single reed's held A4 carried tones that were not harmonics,
-26 dB under them (the volume's -45), and sidebands 16-20 Hz apart round
every harmonic. Alone, at a steady pressure, the reed was clean at 192
kHz (-70 dB) and not at 96 (-25): numerical. The tones lay at fs/2 - n f0.
The near field's row had turned stiff -- the tube's Z_s now in it, h(R +
Z_s) > 2M_n -- and the trapezoid flipped the flow's end value at the
internal Nyquist frequency, which the jet's nonlinearity beat against the
tone. Weighted as the hole's row is (8k), θ = 1 - M_n/(h(R + Z_s)): -58
dB at 96 kHz, no change of level or pitch. Heard: "better".

Then made robust:
- The tube's length is no longer rounded to whole steps -- a different
  tube, and different lifted harmonics, at every host rate: the waves back
  toward the opening take ⌊D⌋ steps, those toward the closed end the rest
  of 2D, read between two steps by a straight line, which is passive.
  The F4 at 96 and 192 kHz: 440.00 Hz both (439.67 before).
- Slot points falling on one step of a short tube are merged: given air
  twice over, a step gained the cross term 2g₁g₂ from nowhere (a 10 mm
  reed's four points all fell on one).
- The walls' losses were the waves multiplied by e^(-αL) at the ends,
  which leaked even the cell's still air -- the reed saw ~2 % less
  pressure. Now one resistance in series with the hole, Z₀αL, half the
  distributed R'L.
- The reference integrator is the tube's too, by the method of lines (40
  segments; 80 agree); the scheme meets it within 0.7 % and 0.1 cent, and
  converges, 96 against 384 kHz, within 0.9 % and 0.2 cent.
- The tuner tunes what the engine builds: the loads and ducts kept as
  the tables keep them, f32 to three places.
- The lines are 64 steps, the engine 1187 KiB, inside a sixth of the 8 MiB
  stack; the recording level -18.7 dB (-22 before), the loudest case again
  at -6.0 dBFS.

Measured against the volume (dd7e761), what the earlier milestones checked
against measurements:

| | volume | tube | measured |
|---|---|---|---|
| F4's swing at 300 Pa | 3.73 mm | 2.41 mm | > 4 mm (Ziegenhals) |
| deepest bend, part-open pallet | −16 c | −7 c | 15-35 c |
| Master's pressure fall, arm | 13.4 % | 3.8 % | ~15 % |
| F4's finger attack | 80-96 ms | 18 ms | 50-140 ms (Llanos) |
| A3-B4 attacks, bellows | 81-99 ms | 34 ms | 50-140 ms |
| L under M's octave (untuned) | −9.6 c | −28.3 c | -- |

All four move one way: the reed more strongly loaded, swinging less,
drawing less air, starting faster. The likely cause, not yet measured:
the tube's air between the hole and the slot, a mass ρL/S some 2.5 times
the hole's own inertance for the A4, in series with it -- with the hole
itself made smaller (150 to 100 mm²) in the same milestone. Real cells
have that air; whether this one-dimensional tube, its section the volume
over 1.4 tongues, puts the right amount of it in the slot's way is the
question. Open; these tests fail and say so.

**The investigation (2026-10-02).** What moves the F4 (`tests/coupling_probe.rs`,
192 kHz, 150 mm² hole): the cell's volume hardly (doubled, swing 2.78 →
2.65 mm); its length much (a quarter as long, the same volume: 3.49 mm,
325 cm³/s); and where the tongue lies most of all -- turned round, its tip
to the opening, the tube is the volume again (3.61 mm against 3.77, 326
cm³/s against 371, growth 7.2/s at 100 Pa against 7.2). The slot's air
passes mostly at the tip, and the tip lies at the closed end, some 40 mm of
the cell's air from the hole -- a mass ρL/S beside which the hole's own is
small, because an accordion's hole is about as wide as its cell. Tonon's
Helmholtz cell puts all the mass in the hole, right only for a hole much
narrower than the cell; he reports the tip's end only as interference or
amplification near the cell's resonances, with no attack or swing. Where
the tongue lies is sourced (patents; a technician files inside reeds
"through the soundhole... towards the tip"): kept.

The attack is mostly the voiced start (7b): at Attack Kick 1 the F4's
finger attack is 18 ms at 100 Pa and 31 at 400; 0.5, 68 and 41; 0.25, 125
and 51; 0, 427 and 101. Re-voiced to Llanos's 50-140 ms against which it
was voiced: 0.3, chosen by ear on Greensleeves (the player, 2026-10-02) --
110 ms and 49, the second a millisecond under. The swing's measurements
never agreed (±0.72 mm, Nussbaumer & Agarwal; > 4 mm, Ziegenhals); 2.4 mm
lies between: not acted on. The part-open pallet's bend (−7 c against
15-35) and the treble above 1.4 kHz needing 400-600 Pa stay open.

Built as 0.13.0 for the player to try. The plugin keeps its engine on the
heap, allocated once in `prepare`: at 1.2 MiB, four processors held by
value in one test overflowed the 8 MiB stack. The component through the
host (`tail-cost`, 128 frames, this machine): 615 µs a block playing (436
before the tube: prediction 5, a third more, NOT MET -- 41 %), 27 µs
silent; the first block 2.48 ms of its 2.67, the models built at a key's
first press, the open risk of 10.

**The silbido and the release (2026-10-02).** The player, always with
Musette Paris, the wheel at 79 % and a quick pallet, heard a fine whistle
through the long notes, and a cut as a note was let go with the bellows
still pushing. Measured through the plugin's WebAssembly (`wasm-render`,
a scratch example of the host): the cell's longitudinal modes, every c/2L
(2.7 kHz for the F3's 64 mm), rang as narrow bands up to 20 kHz, the three
detuned reeds' harmonics beating in them. In turn:
- The hole's outer end radiates: its end correction's air beside ρc/A
  (the classic piston approximation), so the cell's high modes lose energy
  through the hole. −2 to −5 dB on the bands; heard as "a little better".
- The slot's far side the same, ρc/S_slot beside the near field's mass:
  0-3 dB, kept for the same physics. The slot's flow no longer carries a
  mass of its own, so the near-field flip of the first squeal cannot
  return.
- A continuous slot, the cell's volume halved (real cells are as deep as
  the tip's swing, a Soviet maker's text), and wall losses: none helped
  without dulling the tone; the first was heard as "it takes the
  brightness", the second made the bands stronger. Not kept.
- The source: the pulse. With the tongue's rest shape the mode's, every
  element crossed the plate at once and cut the flow in microseconds. A
  technician's set -- two thirds flat, the last third curving up -- closes
  the slot from the rivet toward the tip: −8 to −16 dB on the bands, −2 to
  −3 in 2-5 kHz, as the IfM found a gap opening toward the tip lowering
  the upper partials. Kept (0.13.1); heard as better, "some brightness
  lost".
- The release: no click is added; the hole stayed whole until the pad
  seated and then the tone stopped. Pallet Closing Time 30 ms by ear
  (0.13.2). The curtain's slit is viscous, Poiseuille's 12μw/(g³R)
  (0.13.3). And its air has mass, ρw/A, which grows as the pad comes down
  (Tonon's k, higher with the pallet near). Against FreePats' Hohner
  releases (CC0, 17 notes; a D4 fades −1, −4, −8, −11 dB at 10-40 ms, −10
  dB taking 30-140 ms across them) the model's D4 now fades −1, −8, −22,
  −28 dB: no longer a cut. The part-open pallet's bend came back to −27
  cents (measured 15-35), its test passing again; the F4's finger attack
  117 and 72 ms.

Open: the onset at threshold heard as a blow; the first new chord's block
over its deadline (3.4 ms of 2.67); the fingerprints.

**No velocity (2026-10-02).** The player: "sometimes the strength drops as
I play, as if it ran out of air -- when I play a note softly; accordions'
keys have no velocity", and the wheel is for that. Until then key velocity
set the push while no controller had spoken, and a controller speaks only
when it moves: a wheel left at 79 % since the plugin was built had never
spoken, so each soft note took the air from the whole instrument. Now
velocity moves nothing; the bellows rests at 300 Pa (the defaults' push
for the IfM's playing pressure) until the wheel or Expression moves, and
the plugin carries the bellows into every engine it builds again. Bellows
Smoothing, which only smoothed velocity's push, is retired.

Found running the suite after the day's changes, not caused by this one:
the 16′ C2's attack at 400 Pa is 391 ms (50-140 measured; 130 in 8e), and
the 16′ C's threshold 21 Pa (under 20 asked). Open.

**The slow bass (2026-10-02).** Turning off each of the day's changes in
turn (`coupling_probe.rs`, `the_bass_attack_against_the_kick`): the set's
shape alone slowed the 16′ C2 (391 ms at 400 Pa; 79 with the mode's shape
at Attack Kick 1). Two thirds flat left a low tongue's slot all but shut at
rest. The player chose a third flat and Attack Kick 0.5: retuned, the 16′
C2 attacks in 143 ms, A2-B2 in 84-101 (Llanos 70-100), the 16′ thresholds
9-10 Pa; the F4 58 ms at 100 Pa and 46 at 400; the part-open pallet −16
cents. Pallet Closing Time back to 10 ms by ear (30 set the closing's
sound apart).

**The grille (2026-10-02).** The player's thought: the closing's sound may
be right, its reaching the microphone so strongly not. The stage sends
each hole's radiation straight to the capsules. No measurement of a treble
grille's transmission was found; Richter (IfM Zwota, the summary page)
writes that through the open grille the highs radiate "unhindered", his
closed dome reinforcing ~500 Hz, and no accordion's directivity is
measured. A grille filter would be invented, and would dull every note.
Measured instead, at half a millisecond: at 10 and at 30 ms the highs stay
whole to the pad's seat and then fall 65 dB in 1.5 ms -- the closing's
sound -- where the Hohner's releases fade over ~40 ms. Open.

**The pad's seating (2026-10-02).** The pallet came down at constant speed,
the felt only shaping its curtain over the last tenth of the travel -- a
millisecond at 10 ms -- and that last tenth is where the tone is shut off.
Now the pad slows on its felt, e^(-t/τ), Pad Seating (a new parameter,
voiced against a measurement). On FreePats' Hohner releases the steepest
fall above 3 kHz in any 1.5 ms is −6 to −23 dB (median −12; D4 −17); here
the D4 fell −49 dB, and at τ = 12 ms −16. The whole fade stays quicker than
the Hohner's (−10 dB at 10 ms against −1): Pallet Closing Time is 10 ms, the
player's choice.

## 9g. The keys' own dynamics

The player (2026-10-02): a playing mode where the wheel is not the
bellows' opening but each key has its own, from its velocity -- selectable
against the wheel. Not a bellows per note: the accordion has one, and what
a key opens is its pallet. The engine already holds a key part-way down
(`Engine::press`, the pallet's curtain the rim times the lift times the
depth), which is how a player bends a note. Before any mode is built: how
much does a part-open pallet change a note's level, and what does it cost
in pitch and attack?

**Derived, before measuring.** The curtain is never more than the hole: at
the defaults (100 mm², its rim 50 mm, a 3 mm lift) it reaches the hole at
two thirds of the key's travel, the piccolo's 64 mm² at 0.43. Above that a
key's depth changes nothing. Below it the curtain throttles the cell, and
the reed hears less than the bellows' pressure; it stops where that falls
under its threshold. Already measured (`milestone_2`, a lone F4 on a
laboratory chamber, 300 Pa): the pitch bends −0.3 cents at depth 0.6, −3.6
at 0.3, −16 at 0.15, and the reed stops by 0.12. So the range is the
reed's own between the bellows' pressure and its threshold, squeezed into
the bottom of the key's travel.

**Predictions** (through the engine, Musette Paris -- three middle reeds --
with Pallet Opening at 35 ms; the bellows resting at 300 Pa and at the
wheel's 79 %; F3, D4, F4, A5; level over the steady tone):
1. From full depth to two thirds, the level moves less than 0.5 dB and the
   pitch less than 1 cent (the curtain is the hole).
2. From full depth to the shallowest depth that still sounds, the F4 falls
   8-15 dB at 300 Pa: its threshold is ~60-100 Pa, and the reed's level
   climbs ~4-9 dB per doubling of what drives it in that span.
3. The range shrinks with pitch: the A5 (threshold ~200 Pa) falls under
   6 dB at 300 Pa; the F3 more than the F4.
4. With the wheel at 79 % every range is wider than at 300 Pa.
5. Most of the fall is in the bottom third of the travel: from two thirds
   to a third, under a third of the range.
6. At the shallowest sounding depth the pitch is 10-30 cents flat, and the
   attack is slower than at full depth.

If 2-3 hold, a velocity curve has a few decibels to work with, flat in
pitch only over its upper part; the wheel's bellows spans far more. That
is what decides whether the mode is worth building, and how velocity maps
to depth.

**Measured (2026-10-02, `tests/milestone_9g.rs`, run with `--ignored`).** A
note counts as sounding when it holds its level (two windows within 1 dB)
within 40 dB of the full tone; the first run counted tones still dying at
the edge, and its "ranges" ran to 75 dB. The bellows read 297 and 601 Pa.
Level and pitch from full depth to the shallowest steady tone:

| note | 300 Pa: range, depth, bend | the wheel's 79 % (601 Pa) |
| --- | --- | --- |
| F3 | 16.5 dB, 0.17, −90 cents | 23.2 dB, 0.15, −214 cents |
| D4 | 12.5 dB, 0.15, −27 cents | 19.7 dB, 0.13, −77 cents |
| F4 | 13.7 dB, 0.13, −22 cents | 24.7 dB, 0.12, −74 cents |
| A5 | 6.0 dB, 0.11, −5 cents | 9.4 dB, 0.09, −10 cents |

1. **Met.** At 0.67, 0.8 and 1 every note is the same to 0.05 dB and
   0.05 cents.
2. **Met.** The F4 at 300 Pa spans 13.7 dB.
3. **Met, at the edge.** A5 6.0 dB; F3 16.5 above F4's 13.7.
4. **Met.** Every range is wider at 601 Pa.
5. **Met but for the F3 at 300 Pa** (5.6 of 16.5 dB above a third).
6. **Not met.** The pitch: the mid notes are within 10-30 cents at
   300 Pa, but the F3 bends −90 and −214 cents and the mid notes −74 to −77
   at 601 Pa; the A5 only −5 and −10. The attack: faster, not slower -- the
   pallet travels at its own speed, so a shallow key reaches its depth
   sooner (0.2 of 35 ms in 7), and the metric reads the smaller tone as
   steady sooner. The premise was wrong, not the model.

What it means for a velocity mode:
* The quietest steady tone is 6-17 dB under the full one at the resting
  bellows, 9-25 at the wheel's 79 %: a key gives an accent, not the 30-40
  dB a keyboard player's velocity spans.
* The decibels cost pitch, at every note, the more the softer: about
  1.5-2 cents a dB in the middle at 300 Pa, ~3 at 601 Pa, and in the F3
  5-9. Against Musette's ±23 cents a soft note detunes the chord.
* Under the shallowest steady depth the note does not fade, it is gone,
  and that depth moves with the note and the bellows (0.09-0.17): a
  velocity curve must stop short of it.
* The F3's −214 cents is past the "up to about a semitone" measured on
  players' bends (Elejalde-García et al. 2021; milestone 2): the low reeds
  may bend too far here. Open.

## 8n. The cut at a release

The player (2026-10-02): the noise as a note is cut is still loud against
a real accordion. Measured first, with the player's combination (Musette
Paris, the wheel's 79 %, Pallet Opening 35 ms), against FreePats' Hohner
release samples (D4-D6), in four bands (50 Hz-1 kHz, 1-3, 3-8, 8-20 kHz),
2 ms frames relative to the held tone:
* The real releases' steepest fall in any 6 ms is −3.5 to −15 dB in every
  band (median about −7); above 1 kHz no band reaches −20 dB within
  120 ms in most notes (partly the recording's room, which is not known).
* The instrument alone here: above 1 kHz −22 to −36 dB in 6 ms, −20 dB
  reached 20-22 ms after the release, as the pad comes onto its felt; the
  band under 1 kHz, which the bellows' walls pass, falls −7 to −12.
* Through the microphones and the room (what the player hears) the room's
  tail softens it: −4.5 to −16.6 dB in 6 ms, the D4's 1-3 kHz −13.5 and
  the A5's −16.6 against the Hohner's −4.6 and −3.4.
* A rise of 2-6 dB in some band in the first 24 ms is in both (the
  tremolo's beat); nothing in the model rises above the held tone more
  than the real ones do. The reed itself rings down slowly once sealed
  (−10 dB at 40 ms, `release_diagnosis`); what collapses is the jet's
  sound, the highs, at the seat.

So the cut is the highs' collapse as the pad seats. Pad Seating (8m, τ
12 ms) is voiced, against a 1.5 ms statistic; no pad's compression is
published. Re-voiced here against the 6 ms one.

**Predictions:**
1. A longer seating brings the instrument alone's steepest 6 ms fall above
   1 kHz within the Hohner's (no worse than −15 dB) for the D4, F4, A5
   and D6, somewhere in 25-50 ms.
2. The held tone does not move: the seating acts only on a key let go
   (exact; every fingerprint of a held note unchanged).
3. The note still ends: −30 dB within 150 ms above 1 kHz.

**Measured (2026-10-02), the instrument alone; steepest 6 ms fall above
1 kHz, worst band:** D4 −35.6 (12 ms), −20.3 (25), −18.1 (35), −14.1
(50); F4 −35.1, −26.4, −17.5, −21.3; A5 −31.0, −19.5, −15.6, −19.5; D6
−29.9, −20.6, −15.9, −20.8. −30 dB above 1 kHz is reached in 34-46 ms at
35. Prediction 1 not quite met: at 35 ms every note is within 3 dB of the
Hohner's −15, and 50 is worse again for three of the four. 3 met. The A5's
band under 1 kHz -- its fundamental -- falls −22 dB in 6 ms at every
seating: that fall comes before the felt, where the curtain no longer
keeps the reed going (milestone 9g's edge, 0.09 of the travel). An A/B for
the player, 12 against 35 ms through the microphones
(`scores/releases-8n.score`).

**The player located it (2026-10-02).** A file of the same releases by
path and band: the noise is in the instrument alone, not the room, and
nearly alone in 1-4 kHz -- "a very short, high puff, almost a click for
how short it is". Measured in 4 ms windows: over the last 10-15 ms of the
cut the sound brightens instead of darkening -- the 0.5-4.5 kHz centroid
from ~1.1 to 1.5-1.7 kHz in the D4 (8-22 ms), from ~1.7 to 2.6 kHz in the
A5 (10-24 ms) -- while the Hohner's G4 darkens (its highs −5 to −15 dB
against its fundamental) and its D4 stays about even. The peaks stay on
the note's harmonics: no separate ringing, a change of the spectrum's
shape. Ruled out, each by an experiment reverted:
* Pad Seating (12-50 ms): the brightening stays.
* The curtain's Bernoulli floor (8h) only while opening: no change.
* The walls' loss along the tube while the curtain throttles (Kirchhoff's
  αL ≈ 2 % a pass): no change.
* The engine putting a reed to rest: at 98 dB down, not a click.
Open: the cell's resonance as the curtain shuts -- c/4L, 1.6 kHz for the
D4 and 2.6 kHz for the A5, sits where the brightening does -- to be
measured with one reed, no tremolo, pitch-synchronously.

**One reed, cycle by cycle (2026-10-02, `release_diagnosis.rs`,
`the_release_cycle_by_cycle`, `the_tone_at_fixed_curtains`,
`the_burst_against_the_closing_time`).** At 601 Pa, Pallet Opening 35 ms:
while the curtain passes ~55 to ~10 mm² the 8th harmonic -- 2.35 kHz in the
D4, 2.8 in the F4 -- rises +8 to +12 dB above its held level for a few
cycles as the fundamental falls 9-11 dB; in the A5 +2 to +5 dB across
several. The same at any closing time, 3 to 100 ms (+11.0, +10.7, +12.3,
+8.7 dB in the D4): not the curtain's motion. Not the cell's filter either:
held still at 60, 40 or 30 mm² the steady tone's 8th harmonic is −4 to +4
dB. What differs is the tongue: let go, it still swings almost fully (its
Q 250) while the curtain throttles the hole; held still, it has settled
lower. Supposed, to be measured next: a full swing against a collapsing
drop lets the air through in short spurts, bright. The Hohner D4's 8th
harmonic also rises against its fundamental in the cut (+6 dB, 15-40 ms,
noisy), so some of it is the instrument's; how much, and for how long, is
the question.

**The reed's own flow in the bright cycles (`the_reed_flow_in_the_burst`,
D4, 601 Pa).** The supposition was wrong: the slot passes air for the same
share of each cycle as held (67-71 % against 66 %) and its flow is less
peaked, not more (crest 1.5-1.7 against 2.0). What changes is its 8th
harmonic, +15 to +17 dB over its held level while the 1st falls 5-10 dB,
and the hole flow's 8th +7 to +10: a ringing at ~2.35 kHz in the reed's
flow and the hole's. Held still at 30 mm² the same flow has no such rise
(−1.3 dB), and at 15 mm² the reed does not keep going at all. Read as: the
cell's resonance, moving up from its open-hole place (~1.2 kHz) towards a
closed end's (c/2L, 3.2 kHz) as the curtain shuts, crosses a harmonic of
the reed still swinging, and while the hole's end turns from radiating to
nearly closed it loses little -- so the crossing rings. Kirchhoff's wall
loss along the tube (αL ≈ 2 % a pass) changed nothing (above); the slit's
oscillatory viscous loss at 2.35 kHz, gap 0.2-1.1 mm against a 45 μm
boundary layer, comes to ~2·10⁵ Pa·s/m³, an order under the curtain's
other terms (estimated): neither is enough by itself.

Whether a real cell rings as much is not known: the Hohner's D4 shows its
8th harmonic up some 6 dB against its fundamental in the cut, but through
an unknown room and in windows too short to be sure. A recording of the
player's own instrument -- the same notes let go under a held bellows,
close to the grille -- would settle how much, and the cell's end can then
be measured against it rather than voiced.

**No instrument to record (2026-10-02).** The player has no accordion:
the cell's end is to be voiced by ear, its mechanism physical and its
constants bounded. The two unmeasured constants that act only near the
seat, swept (`the_burst_against_the_closing_time`, BURST_SWEEP=lift; the
seat's width by a reverted edit), worst rise of h5-h10 at 10 ms closing:
* Pallet Lift 1.5 / 2 / 3 / 4 / 6 mm: D4 +7.1 / +9.9 / +10.7 / +13.0 /
  +13.4 dB; F4 +19.4 / +12.1 / +9.5 / +11.2 / +12.7; A5 about +5 at every
  lift. No direction helps every note.
* The seat's width 1 / 2 / 4 / 8 mm: D4 +11.3 / +10.7 / +8.9 / +6.8; F4
  +9.3 / +9.5 / +13.3 / +3.6; A5 +4.9 / +5.1 / +6.1 / +4.2. Only 8 mm helps
  all, wider than a pad's overlap of a 5 mm hole is likely to be, and the
  curtain's mass it sets is there with the pallet open too: the held tone
  would move.
Neither is the cell's missing loss. What a real pad is, and the model's
end is not: felt under leather, a surface that absorbs, which becomes the
cell's end as it comes down.

**The pad's absorption, bounded and tried (2026-10-02).** No measurement
of a leather-faced felt pad's absorption was found (a pad patent, US
4,114,500, speaks of pads' "detrimental sound absorbing qualities", no
figure). Delany & Bazley's model for 3 mm of felt (flow resistivity ~30
kPa·s/m², assumed) on a rigid back gives ~6 % at 2 kHz, less under
leather (derived). Tried before building it, by a reverted edit: the wave
the hole's end sends back loses 6, 20 or 50 % of its energy once the
curtain is narrower than the hole. The burst stays: D4 +10.5, +10.1,
+10.5 dB; F4 +9.5, +9.4, +9.4. Not built. With the walls' loss before it,
this rules out a resonance that rings for want of damping: the burst is
driven.

Nor is it the slot's flow reversing: it runs backwards 26-32 % of each
bright cycle, 30 % of each held one. Open: what the reed's flow does
differently in those cycles. What does change there is the drop across
the reed, from 55 to 330-560 Pa below the bellows within 6 ms, its swing
still nearly full: the tongue's mean place moves while it swings, so the
slot's phases it passes through change.

**Found by ear (2026-10-02).** The player heard five builds of the same
releases (`scores` in the scratch; reed.rs edited and restored for each):
A as it is; B without the curtain's air mass; C with it but without the
momentum kept as it grows; D without the viscous slit; E without the
Bernoulli floor while closing. "C reduces it, D reduces it, E removes it
completely." The floor (8h) was put there for the opening, where the
linearisation's lag let the first step through as if no pallet were
there; while closing it read the drop across the curtain, which swings
with the cell's tone, and so moved the curtain's resistance within every
cycle of the last few. Now the floor is the opening's alone (0.13.9).

The 8th-harmonic measure above did not see it: with the floor left to the
opening it moved by under a decibel (the first experiment of this
section). What it measured -- one harmonic's rise above its held level --
is not what the player hears. Open: a measure that does.

What the floor added, measured as the difference of the two builds (A
less E, through the microphones; identical to the bit before each
release): a component 15-19 dB under the held tone at its peak, 8-10 ms
after the key is let go, 25-28 dB under it over 60 ms, 58-70 % of it in
1-3 kHz (D4, F4, A5). The puff, as the player placed it.

## 8o. The click at a note's start, where the curtain meets the hole

The player (2026-10-02, 0.13.9): a small click as a note starts, placed on
a render of four F4s at 25 ms after the second's onset (Pallet Opening
35 ms). The curtain is the hole's area at two thirds of the travel, 23.3
ms: there the area the reed sees, min(curtain, hole), turns a corner --
its growth stops in one step -- and with it the opening's Bernoulli floor
(8h, 8n) ends. Measured, the high band (> 4 kHz) in 0.5 ms frames against
its own ±3 ms median: the largest rise of the onset, +11.1 dB at 24.0 ms
at 35 ms opening, +13.8 dB at 34.0 ms at 50 ms (the corner at 33.3) -- it
moves with the corner; the onset's other rises are ≤ 8 dB.

The hole and the curtain are two orifices in series, 1/A² = 1/A_c² +
1/A_h², which has no corner. Taken as it stands it narrows the open
pallet too, 100 to 83 mm² at full lift: the held F4 −0.7 dB, its centroid
1023 to 991 Hz. Taken with the hole's area as the open pallet's -- the
parameter voiced with the pad lifted -- 1/A² = 1/A_c² + 1/A_h² − 1/A_c,full²
is the hole's at full lift and the series law below it. Tried by a
reverted edit: no rise at the corner at 35 or 50 ms; the held F4 66.54 dB
and 1023 Hz, as before; left, a +7 dB rise as the pallet ends its travel
(35-36 ms), as now (+6). An A/B for the player (`onsets-8o-AB`).

Found by the suite meanwhile, not caused by it: `milestone_8b`'s
`the_keyboard_splits_into_three` overflowed its 8 MiB test thread (two
engines by value); its engines are now boxed, as the plugin's is.

**Heard (2026-10-02).** The series law (B) kept the click. What also
changes at the corner is the opening's floor: it held while the curtain
moved and ended as it stopped -- at the corner with the plain minimum, at
the end of travel with the series law. Two more builds: C, the curtain's
Bernoulli loss solved at the step's own flow -- c₁a + c₂|a|a = b, one
root of the sign of b -- with no floor; D, C and the series law. "In C and
in D there is no click." C kept (0.13.10): it is the cause -- the
linearisation's lag that 8h's floor stood in for -- and needs nothing
else; the minimum stays. The quadratic needs the curtain's k, kept as
before (10c), and one square root a sounding reed a step.

Against 0.13.9's renders every sample of every scene moves -- the
oscillation's phase -- but not the sound: each scene's total level within
0.3 dB; the held F4, D4 and A5's third-octave levels within 0.13 dB (mean
0.01); long-term spectra within 3 dB in their worst band, which the attacks
and releases and the tremolo's beats account for. The one-ulp nudge, the
yardstick of 10c, moved nothing this time (0.00 dB in every scene), so it
could not judge. Fingerprints taken again.

## 9c again. The preamp set for the player's playing

The player (2026-10-02): "it lacks output; I turn Output Gain up". The
recording level was set so the loudest the model can play -- both hands'
full chords in Master at the bellows' 1 kPa ceiling -- peaks at -6 dBFS.
Played as the player plays (Musette Paris, the wheel at 79 %, a melody
over bass and chords, `play.score` in the scratch) it peaks at -13.2 dBFS,
-28 dBFS RMS: an engineer who set the preamp for a fortissimo nobody plays
-- a real ff is about 300 Pa, the ceiling 1 kPa (8m, open). What the
plugin delivers is the instrument recorded, preamp included, as RF
Concert Grand does; equalisation is the mix's, after it (`suggested_chain`:
RF-EQ, flat). The level up +7 dB, at the player's word.

**Predictions:**
1. The player's playing -- Musette Paris, the wheel at 79 %, a held A4
   over a bass and its chord -- peaks at -6 dBFS within 1.5 dB.
2. The loudest case now reaches the soft ceiling (above -6 dBFS) and
   stays under full scale; nothing under the knee is touched (9c's 2,
   unchanged).
3. A C4 at mf near -18 dBFS (-25 + 7).

**Status (2026-10-02): built (0.13.11); all three met**
(`tests/milestone_9c.rs`).
1. **Met.** The player's playing peaks at -5.3 dBFS.
2. **Met.** The loudest case -1.0 dBFS, in the soft ceiling; under the
   knee nothing changes (`the_ceiling_leaves_the_music_and_rounds_the_peaks`).
3. **Met.** A C4 at mf -18.9 dBFS.
The 25 stereo scenes' fingerprints taken again; the three mono ones did
not move. With it, the walls' hardness 0.2 by default and in Musette Paris
(voiced by the player: 0.5 put the ORTF pair at the hall's critical
distance, the room as loud as the instrument; 0.2 puts it 7 dB under, the
mid tail 0.53 s), and RF-EQ suggested after the instrument, flat.
`milestone_9d`'s room tests took the default hardness; at 0.2 the 1500 m³
room's echo density at 50 ms fell from 0.79 to 0.70, under its guard. They
ask after the network's diffusion, not the walls: pinned at the 0.4 they
were written and measured at, they read 0.91, 0.95 and 0.79 again.

## 9h. Key Touch

The player (2026-10-02): the plugin will be played from a MIDI keyboard,
without the wheel, nine times in ten; 9g's A/B with velocity setting each
key's depth "sounds almost the same" -- so let it be there, and on by
default. A new parameter, Key Touch (Off / On, default On): On, a treble
note's velocity sets how far its key goes down; the bellows is untouched,
resting at 300 Pa and moved by the wheel or Expression as before. The bass
and chord buttons stay fully down (9g measured the treble; its F3 already
bent −90 to −214 cents).

**The curve, derived from 9g.** Over the travel that matters the level
falls about linearly with 1/depth (the F4 at 300 Pa: −1.4 to −1.9 dB per
unit of 1/depth from 0.5 to 0.2). So velocity is spread evenly over 1/d:
1/d = 1.5 + (1/d_min − 1.5)(1 − v), v = velocity/127, from 2/3 (the curtain
is the hole) at v = 1 down to d_min. d_min = 0.18: the shallowest steady
depth measured was 0.17 (the F3 at 300 Pa), and the edge moves with note
and bellows (0.09-0.17 in 9g). Velocity 127 takes the key fully down, as
with Key Touch off.

**Predictions:**
1. Velocity 127 renders exactly what Key Touch off renders.
2. Every treble key that sounds at full depth still sounds, steadily, at
   velocity 1 -- at 300 Pa and at the wheel's 79 %, Musette Paris.
3. From velocity 127 to 1 the F4 at 300 Pa falls 7-10 dB (9g: −6.5 dB at
   0.2, −8.4 at 0.17), in steps about even in decibels: velocity 64 within
   2 dB of halfway.
4. The held tone at velocity 127 does not move; with Key Touch off nothing
   moves at all (every fingerprint whose scene plays at 127, or plays with
   it off, unchanged).

**Status (2026-10-02): built (0.13.12); 1 and 4 met, 2 met but for the
A6 at 300 Pa, 3 not met** (`tests/milestone_9h.rs`).
1. **Met.** Velocity 127 renders exactly Key Touch off.
2. **Met but for the A6 at 300 Pa.** At 0.27 of the hole the A3 and the
   G♯6 fell silent at velocity 1 at 300 Pa (`where_the_edge_keys_stop`:
   they stop under 0.35; their neighbours sound at 0.27). The floor rose to
   0.35, and every key sounds at velocity 1 at 300 Pa and at the wheel's
   79 % -- except the A6 at 300 Pa, at its threshold fully down (-32.6 dB,
   stopping under 0.6): the high reeds' open threshold, not a curve's.
3. **Not met.** At 0.27 the F4 at 300 Pa fell 7.7 dB, velocity 64 at -3.4
   (met); at 0.35 it falls 5.0, velocity 64 at -2.2 -- even, but under the
   range written. At velocity 1, 300 Pa: F3 -10.5, A3 -9.3, F4 -5.0, A5
   -1.2, G6 -3.9 dB; at the wheel's 79 %: F3 -11.6, F4 -7.0, A5 -1.8.
4. **Met.** The scenes played at velocity 127 did not move; the others'
   fingerprints taken again.
The laboratory's --key-depth (9g's study, a linear curve) is gone: Key
Touch is the parameter (--set key_touch=0 for the keys fully down).

## 8p. The free bass

The player (2026-10-02): to build chords freely from a MIDI keyboard, the
left hand as single notes down as far as an accordion's reeds go -- "a
fictional mode". Not fictional: free-bass and converter accordions play
the left hand as single notes, the bayan's E1-C♯6, 58 notes (SOURCES.md,
"The free bass"). A selector, Bass System: Stradella (the default) or Free
Bass. Left Hand is retired -- the left hand is always there: it stays at
On, off the panel; a state or program saved with it off loads as on.

**The design.**
* With Free Bass, what the left hand plays -- channel 1 under the Split
  Point, or channels 2 and 3 -- sounds as single notes at their own pitch,
  E1 (MIDI 28) to C♯6 (85); outside it nothing. The Split Point still
  divides the hands.
* Two voices, 8′ and 4′, an octave apart: "octave tuned" (Pigini's 42/B),
  **assumed** as which; the Bass Register does not apply.
* Each note its own pallet and cell, its reeds the bass side's -- the
  bayan maker's slots carried down (`compass::unloaded`), from the 8′ E1 at
  41 Hz, 8 semitones under the bass side's lowest reed (C2): **derived**,
  an extrapolation -- each finished by the tuner (`rf-musette-lab tune`).
  They sound from the bass box.
* Not 58 keys held: 16 voices, each a pallet and two reeds' cells, given
  to the notes as they are played and built then (a model is ~1.4 KiB and
  shares the engine's tongue profile). 58 keys would take the engine from
  1201 to ~1509 KiB, over its stack guard (a sixth of 8 MiB); 16 voices,
  ~1286.

**Predictions:**
1. Stradella renders exactly as before (every fingerprint unmoved), Left
   Hand retired included.
2. Every free-bass note's 8′ reed speaks at 300 Pa, and once tuned sounds
   within 5 cents of its pitch; the 4′ likewise wherever it speaks at
   300 Pa (the top 4′, C♯7, may not, as the treble's top does not).
3. A note played under the split with Free Bass sounds at its own pitch:
   its fundamental within 5 cents of the note's.
4. The free bass's lowest notes attack slower than the bass side's C2
   (Llanos: the attack lengthens as the pitch falls): E1 over 143 ms at
   400 Pa.
5. Four free-bass notes struck at once, at 48 kHz in blocks of 128, take
   under the block's 2.67 ms here (as the treble's chords are judged).

**Status (2026-10-02): built (0.13.13); 1, 2, 4 and 5 met, 3 not met**
(`tests/milestone_8p.rs`).
1. **Met.** With Stradella every fingerprint is unmoved, Left Hand retired
   included; the shared reed step (`blow_rank`) keeps the treble's and the
   bass's arithmetic as it was.
2. **Met.** Every 8′ speaks at 300 Pa and, alone as the tuner blows it,
   within 5 cents; the 4′ too but for G♯6, A6 and C♯7 (the notes 80, 81
   and 85's octave), which do not speak at 300 Pa, as the treble's top
   does not. The tuner finished all 116 reeds (3 min 24 s): the E1's 8′
   loaded 10.3 times its modal mass, its duct 17.7 times its hole's depth.
3. **Not met at the bottom.** In the engine the E1 sounds -11.9 cents:
   -7.7 with the bellows stiff, the rest the arm's sag under so large a
   reed (297 to 223-251 Pa). The cause is older than the free bass: the
   tuner blows a reed with its pallet away, the engine through the
   pallet's curtain, whose mass and loss pull the low reeds flat -- free
   bass E1 -7.7, A1 -7.7, E2 -6.2, B2 -3.7, E3 -1.5, E5 -0.2 cents; the
   Stradella's 16′ C2 -7.4, E2 -6.2, A2 -4.3, the same. Open: tune with
   the curtain as played.
4. **Met.** The E1 attacks in 191 ms at 400 Pa (the bass side's C2: 143).
5. **Met.** Four free-bass notes at once: the slowest block 0.27 ms of 2.67
   (release).
Found on the way: Pad Seating, added after the microphones' parameters,
returned before the pallet was rebuilt (8m); it rebuilds it now. And a
fresh tuning now differs from the shipped tables in three cells (the
largest 0.11 cents), since 8o's curtain: the shipped tables kept, the
free bass's added.

**Tuning as played (2026-10-02).** The player: correct it. The tuner's
last step -- the cents it files a reed's mode to so it sounds on its target
at 300 Pa -- now blows the reed through its pallet's curtain fully open, as
the engine plays it: the curtain min(rim × lift, hole), its mass and its
Bernoulli and viscous loss, where it blew it with the pallet away. The
loads and ducts, what makes a low reed speak at all, stay as they were
found. Every table retuned.

**Predictions:**
1. In the engine, the bellows stiff at 300 Pa, the free bass's E1, A1, E2
   and B2 and the Stradella's 16′ C2, E2 and A2 sound within 2 cents of
   their targets (-7.7 to -3.7 before).
2. The treble moves by under 1 cent in its middle (F4's M).
3. Every reed still finishes: the tuner writes every cell.

**Retuned (2026-10-02, 3 min 11 s); 1 and 3 met, 2 not met.**
1. **Met.** In the engine, stiff at 300 Pa: free bass E1 -0.0, A1 +0.0,
   E2 -0.1, B2 -0.4 cents; the Stradella's 16′ C2, E2 and A2 -0.1.
2. **Not met:** the F4's M moved +1.8 cents, not under 1 -- it was that
   flat in the engine. The curtain had pulled every reed flat: the
   8′ ranks +4.7 to +4.8 cents at F3, +2 through the middle, +0.6 at the
   top; the L rank (16′) +23.7 at F3, falling to +0.5 -- the 16′ registers'
   low notes had been a quarter-tone flat. The three 8′ ranks moved alike,
   so the tremolo's beats did not. Now in the engine: L F3 -0.7, F4 +0.1,
   A5 +0.1; M F3 +0.1, F4 +0.2, A5 -0.3; H -0.4 to +0.2 cents.
3. **Met.** Every reed finished; the loads unchanged, two ducts moved by
   hundredths.
With it 8p's prediction 3 is met: E1 -4.8 cents with the arm (its sag),
E2 +2.8, B2 +2.4. Every fingerprint moves with the tuning.
The tests that hold every reed to its pitch -- the treble's (7), the bass
side's (8), the free bass's (8p) -- blew it with the pallet away; they now
blow it through its open curtain, as it is tuned and played, and all pass
within their 2 and 5 cents.

## 9h again. Key Touch holds the bellows; each key its own floor

The player (2026-10-02): the velocity's range is short -- "barely touching
the key should sound softer" -- and with Key Touch on the wheel should not
work. Physically a part-open key cannot reach pianissimo: under its edge
the reed stops, it does not fade (9g); a real accordion's soft playing is
the bellows'. But the one floor, 0.35 of the hole, is set by the A3's and
the G♯6's edges, and holds every other key far from its own (the F4's at
300 Pa: steady to 0.13 of the travel, -13.7 dB, where 0.35 gives -5).

**The design.**
* Key Touch on: the bellows rests -- the wheel and Expression move nothing
  (their position kept for when it is off); the pressure is the resting
  push's for the program's ceiling: 300 Pa by default, ~205-420 Pa in the
  instruments' programs (9f). Off: as before.
* Each treble key its own floor: the shallowest curtain, as a share of the
  hole, at which it holds a steady tone, measured as a maker would try it
  -- each rank alone (L, M, H: Bassoon, Clarinet, Piccolo), at 200, 300 and
  400 Pa, the arm's bellows; the worst rank's, raised 20 % for a chord's
  sag. `rf-musette-lab touch` measures it into `touch.rs`; a key's floor
  at the resting pressure is read from it, between the measured pressures.
* Digital Accordion, whose bellows sensor sends Expression, has Key Touch
  off; the scenes that play the wheel or Expression likewise.

**Predictions:**
1. With Key Touch on, the wheel and Expression change nothing: a phrase
   with them moving renders exactly as with them still.
2. Every treble key, each rank alone, holds a steady tone at velocity 1 at
   the resting 300 Pa (and at 200 and 400).
3. At 300 Pa, velocity 1 against 127: the F4 falls 10 dB or more, the F3
   14 or more, the A5 4 or more (with 0.35: 5.0, 10.5, 1.2).
4. Key Touch off renders as before.

**Status (2026-10-02): built (0.13.14); 1, 2 and 4 met, 3 not met by a few
tenths** (`tests/milestone_9h.rs`).
1. **Met.** With Key Touch on, the wheel at 20, 100 and 127 renders the
   F4 bit for bit as with it still.
2. **Met**, after two corrections. The floor first took the worst edge
   over every rank, open or not, and the piccolo's, the highest, held the
   whole keyboard up; it is now the worst over the ranks the register
   opens. And the steady test, 1 dB between the last two half seconds,
   read Musette's A3 as silent: its three reeds beat, the level wobbling
   about a dB at every depth -- steady at 0.22 and 0.3 of the hole,
   "dead" at 0.25 and 0.27, heard as one held tone at all of them. The
   test takes 3 dB; the edges stay measured with one rank alone, where
   nothing beats (measured in Musette they came out ragged: the A3's 0.53
   at 200 Pa). Every key sounds at velocity 1 in every case asked, the
   F3 in Musette 14.7 dB under 127, the A5 3.9.
3. **Not met.** The F4 falls 9.7 dB, the F3 14.7, the A5 3.9 (velocity 64:
   4.3, 7.2, 1.4). The floor is the edge raised 20 % for a chord's sag; the
   prediction reckoned with the edge alone. Kept, failing, as written.
4. **Met.** Key Touch off takes the same path as before; the master
   chords, now with it off, and every scene struck at 127 keep their
   fingerprints. 23 scenes taken again: those struck under 127, Digital
   Accordion and the wheel as pressure (Key Touch off there, as designed).

## 9c for Key Touch. The preamp set for the resting bellows

The player (2026-10-02): "subí el preamp para Key Touch". 9c again set the
recording level for the player's playing with the wheel at 79 % (~620 Pa):
-4.9 dBFS now. With Key Touch on, the default, the bellows rests at 300 Pa
and the same playing peaks at -9.3 dBFS. An engineer sets the preamp for
the way the instrument is played; Key Touch is a way of playing in which
the bellows never moves, so its level is known.

**The design.** Two recording levels: with Key Touch off, 9c again's,
unchanged; with it on, 3.3 dB higher (0.2917 × 1.462 = 0.4265), the same
playing at -6 dBFS. Derived from the measured -9.3, not voiced.

**Predictions:**
1. With Key Touch on, the player's playing (Musette Paris, Pallet Opening
   35 ms, a held A4 over F2 and its chord) peaks at -6 dBFS within 0.5 dB.
2. With Key Touch off every sample is as before: 9c again's tests and the
   fingerprints of the scenes with it off unmoved.
3. With Key Touch on, the loudest it can play -- both hands' full chords
   in Master at the resting 300 Pa -- stays under full scale; under the
   soft ceiling's -6 dBFS start or rounded by it, never clipped.

**Status (2026-10-02): built (0.13.15); all three met** (`tests/milestone_9c.rs`).
1. **Met.** With Key Touch on the player's playing peaks at -6.0 dBFS.
2. **Met.** With it off: -4.9 dBFS, the loudest -0.9, the C4 at mf -18.9,
   as before; Digital Accordion and the wheel as pressure keep their
   fingerprints. The 23 stereo scenes with Key Touch on taken again.
3. **Met.** The loudest with Key Touch on, both hands' full chords in
   Master at the resting 300 Pa: -3.7 dBFS, rounded by the ceiling.

## 9i. The registers on a controller's pads

The player (2026-10-02): the treble registers have no controls -- "on the
Arturia and others the pads could switch between them; with banks A and B,
16 pads hold the 14". An accordionist changes register with the switches
over the keyboard, by a press, mid-phrase; on a MIDI keyboard the pads are
where the hand goes for that.

RackForge already lays a plugin out on any keyboard by slots: a plugin's
`metadata/control-layout.json` names its parameters on `switch-1.*` (pad
bank 1: notes 40-43 and 36-39), `switch-2.*` (bank 2: 48-51, 44-47),
`step.*` and `step-2.*` (⏪ ⏩ and a second pair), each controller package
says which of its controls fill them, and a `set` press selects one value
(RF-Organ's preset keys). RF-Musette carries no layout yet, so its pads
play notes -- an MPK2's, on channel 2, bass buttons.

**The design.** RF-Musette's own layout, switches and steps only (its
knobs keep RackForge's roles; the wheel is the bellows, not laid out):
* By note, 36 up: the 14 registers in Roland's order, Bassoon to Piccolo,
  each a `set` -- pad 1 Bassoon, pad 8 Musette, pad 9 Violin, pad 14
  Piccolo; notes 36-39 and 40-43 are `switch-1.5-1.8` and `switch-1.1-1.4`,
  44-47 and 48-49 `switch-2.5-2.8` and `switch-2.1-2.2`.
* Note 50 (`switch-2.3`): the bass register, cycled. Note 51
  (`switch-2.4`): Key Touch, toggled on and off.
* ⏪ ⏩: the register, stepped, not wrapping; the second pair the bass
  register.
* A mapped pad no longer plays its note (RackForge's rule).

**Predictions:**
1. The layout passes RackForge's own validation (`ControlLayout::validate`)
   and names only RF-Musette's parameters, every value in its range.
2. On every 16-pad controller in RackForge's catalog that sends notes
   36-51, each of the 14 registers is one pad; on the 8-pad ones the first
   eight (Bassoon to Musette), and with step buttons all 14 by stepping.
3. Nothing a keyboard without pads or step buttons sends changes: the
   layout adds links, it takes none.

**Status (2026-10-02): built (0.13.15); 1 and 3 met, 2 met but on two 8-pad
keyboards** (`crates/rf-musette-plugin/tests/control_layout.rs`; the
controllers through RackForge's `slotted_controllers` and `factory_maps`,
a scratch program, not kept).
1. **Met.** `ControlLayout::validate` accepts it; every value is one of its
   parameter's choices.
2. **Met on the 16-pad keyboards and button rows:** 35 of the 64 packages
   give all 14 registers a pad or a button. On the 8-pad keyboards the
   first eight, Bassoon to Tremolo (the prediction's "to Musette" was a
   miscount: the eighth is Tremolo) -- but for two whose pads do not start
   at note 36: the Oxygen Pro Mini (40-51) gets Organ to Tremolo, Celeste
   and Piccolo, and the Alesis V (scattered) eight scattered registers;
   the catalog's rule of slots by note, not RF-Musette's layout, decides
   them. Where a keyboard has step buttons all 14 are reached by stepping.
3. **Met.** The ten packages that name no pad or step slot get no map;
   the layout lays out no knob or the wheel.

## 9i again. The pads from the most common register to the rarest

The player (2026-10-02), on 0.13.15: the pads should go "from the most
basic to the most complex" -- Clarinet, which every accordion has, on pad
1 -- "ordered by relevance". Roland's order, which 9i followed, is the
FR-3x's switch row, not how common a register is.

**The order.** A register sounds only on an accordion that has every rank
it opens (Roland FR-3x, p. 27: measured, as `REGISTERS`). The builds, from
the most common to the rarest (assumed, from the makers' ranges: the
two-voice student, the three-voice, the four-voice, and the musette's
third middle reed, M−, only in musette and five-voice builds):
1. M alone (every accordion): Clarinet.
2. M and M+ (two-voice MM): Celeste.
3. With L (three-voice LMM): Bassoon, Bandoneon, Cello.
4. With H (four-voice LMMH): Piccolo, Organ, Oboe, Harmonium, Violin.
5. With M− (musette MMM, five-voice LMMMH): Tremolo, Musette, Accordion,
   Master.
Within a tier, fewer reeds first; at equal reeds, the switch row's order.
So pads 1-14: Clarinet, Celeste, Bassoon, Bandoneon, Cello, Piccolo,
Organ, Oboe, Harmonium, Violin, Tremolo, Musette, Accordion, Master. Pads
15 and 16, ⏪ ⏩ and the second pair as in 9i.

⏪ ⏩ step the parameter's choices in their order -- the PLAY panel's,
Roland's -- not the pads': RackForge steps an enum by its schema's list,
and that list's order also sets the parameter's range. Left as it is.

**Predictions:**
1. The layout still passes RackForge's validation; by note from 36, pad n
   sets the n-th register of the order above.
2. On every keyboard that gave all 14 a pad in 9i, all 14 still have one;
   on the 8-pad keyboards that start at note 36, the eight are Clarinet
   to Oboe -- every register of the two-, three-voice builds and three of
   the four-voice.
3. No sound moves: the fingerprints unmoved (the layout is not the engine).

**Status (2026-10-02): built (0.13.16); all three met**
(`crates/rf-musette-plugin/tests/control_layout.rs`, which derives the
order from `REGISTERS` by the rule; the controllers through RackForge's
`factory_maps` again).
1. **Met.** RackForge accepts the layout; pad n sets the n-th register.
2. **Met.** The same 35 keyboards give all 14 a pad; the ten 8-pad ones
   from note 36 give Clarinet to Oboe. The two whose pads start elsewhere
   get others: the Oxygen Pro Mini Cello, Piccolo, Organ, Oboe, Accordion
   and Master; the Alesis V Clarinet, Celeste, Bassoon, Bandoneon,
   Piccolo, Organ, Tremolo and Master.
3. **Met.** The fingerprints unmoved.

## 9i again, the panel. One order for the pads, the panel and ⏪ ⏩

The player (2026-10-02): "that same order should be in the UI too". The
register's choices are listed from the most common to the rarest (9i
again's rule), their values kept: a register is still the number it was,
so saved programs, presets, states and the engine's `REGISTERS` table are
untouched. The PLAY panel draws its switches in the list's order, RackForge
lists the choices and steps ⏪ ⏩ through them in it -- so pads, panel and
steps agree. The choice's range, taken until now from its first and last
entries, is taken from the smallest and largest values.

**Predictions:**
1. The PLAY panel's treble switches run Clarinet, Celeste, Bassoon, ...,
   Master; each still sets the register it names (its value unchanged).
2. RackForge's schema lists the register's choices in that order, and its
   step goes from Clarinet up to Celeste and from Accordion up to Master,
   stopping there.
3. No sound moves: every fingerprint unmoved; a state or program saved by
   0.13.15 loads to the same register.

**Status (2026-10-02): built (0.13.16); all three met**
(`crates/rf-musette-plugin/tests/control_layout.rs`; the panel seen in
`tools/ui-preview.html`). 9i again's "left as it is" for ⏪ ⏩ is withdrawn:
they now step the same order.
1. **Met.** The panel's treble switches, two rows of seven: Clarinet,
   Celeste, Bassoon, Bandoneon, Cello, Piccolo, Organ / Oboe, Harmonium,
   Violin, Tremolo, Musette, Accordion, Master; Musette pressed sets 8.
2. **Met.** `parameters.json` lists the choices in that order, values 0-13
   unchanged, range 0-13; RackForge's step walks the list's order without
   wrapping (`step_value`, `rackforge-core/src/parameter_link.rs`).
3. **Met.** Every fingerprint unmoved; the state and program tests pass
   unchanged.

## 9i again, by the slots' order. Pad 1 is the first register

The player (2026-10-02), on 0.13.16 and the KeyLab Essential mk3: "pad 1
should be Clarinet; pad 5 is" -- the lower row, pads 5-8, holds the first
four, the upper, pads 1-4, the next four. 9i laid the registers out by
note from 36, but a slot is not a note: `switch-1.1` is a layout's first
switch, and each controller package says which of its pads fills it. The
KeyLab's pad 1 sends note 40 and fills `switch-1.1` (`MIDI-MAP.md`); note
36 is its pad 5, `switch-1.5`.

How the 46 packages with pad or button slots number them (read from their
`inputs`): pad or button 1 fills `switch-1.1` on the KeyLab Essential mk3,
the Launchkeys, Oxygen Pro, Hammer 88 Pro, nanoKONTROL2, Launch Controls
and FLkeys; it fills `switch-1.5` -- the catalog's rule places pads by
note, and their maker numbers from note 36 -- on the Akai MPK2, MPK mini,
LPD8, MPD218 and APCs, the Arturia MiniLabs, KeyLab mkII and BeatStep. No
order of slots is pad 1 on both; the layout says what each slot does, the
catalog where the slot is.

**The design.** The registers in the slots' order: `switch-1.1`-`1.8` the
first eight, `switch-2.1`-`2.6` the next six; `switch-2.7` the bass
register, cycled; `switch-2.8` Key Touch. Steps as before.

**Predictions:**
1. On the KeyLab Essential mk3, pad n of bank A is the n-th register
   (pad 1 Clarinet, pad 5 Cello), pad n of bank B the (8+n)-th, B7 the
   bass register, B8 Key Touch.
2. On every package whose pad or button 1 fills `switch-1.1`, pad 1 is
   Clarinet; on those whose pad 1 fills `switch-1.5`, Clarinet moves to
   their pad 5 until the catalog numbers their slots by their pads.
3. The layout still passes RackForge's validation; no sound moves.

**Status (2026-10-02): built (0.13.17); all three met**
(`crates/rf-musette-plugin/tests/control_layout.rs`; the controllers through
RackForge's `factory_maps` again).
1. **Met.** On the KeyLab Essential mk3 Clarinet is `pad-1`; the n-th
   register is pad n of bank A, then of bank B.
2. **Met.** Clarinet is pad or button 1 on the 28 packages whose 1 fills
   `switch-1.1` (KeyLab Essential mk3, Launchkeys, Oxygen Pro and Pro Mini,
   Hammer, nanoKONTROL2, Launch Controls, FLkeys); `pad-5` on the Akai
   MPK2, MPK mini, LPD8 and the Arturia MiniLabs and KeyLab mkII, `pad-a5`
   on the MPD218, `clip-9` on the APCs, `pad-13` on the BeatStep. Open, in
   RackForge: number the catalog's pad slots by the maker's pad numbers.
3. **Met.** RackForge accepts the layout; the fingerprints unmoved.

**After (2026-10-02), in RackForge:** the player asked for the catalog to be
numbered by pad. Its rule is now "pads by the number their maker gives
them": pads 1-8 `switch-1`, 9-16 (or bank B's 1-8) `switch-2`; 350 slot
lines moved in 34 packages, and the KeyLab mk3's and SL MkIII's DAW pads
and the MiniLab mkII's CC pads, left out by the note rule, took their
numbers' slots. Through `factory_maps` again: Clarinet is pad 1 (or button
1) on every package with numbered pads, `clip-9` on the APCs' unnumbered
grids; 37 packages give all 14 registers a pad or a button.

## 9j. Key Touch in the left hand

The player (2026-10-03): with Key Touch on, the bass buttons have no
velocity. 9h gave the treble's keys their depth; the left hand still went
fully down whatever the velocity -- the bass buttons, the chord buttons and
the free bass alike -- while the bellows rests at 300 Pa, so the left hand
had one level only.

A Stradella button lifts its pallets through a rod, a free-bass button
through its own lever; held part-way, either holds its pallets part-open
as a treble key does. So the same mechanism, measured again for the reeds
it opens: their holes, their pallets, the bellows they share.

**The design.**
* Key Touch on: a bass button's, a chord button's and a free-bass note's
  velocity sets how far it goes down, as `touch_depth` sets a treble key's,
  down to its own floor at the resting pressure. Off: fully down, as before.
* The floors are measured as 9h measured the treble's, with
  `rf-musette-lab touch`: the shallowest curtain, as a share of the 8′
  rank's hole, at which the button holds a steady tone, at 200, 300 and
  400 Pa, raised 20 % for a chord's sag --
  - the bass buttons per pitch class and per bass register (the register's
    open ranks drawing together, as the button sounds them);
  - the chord buttons the same, on the chord ranks the register opens;
  - the free bass per note, both its voices.
* A pitch class held in more than one octave opens as far as its deepest.

**Predictions:**
1. Velocity 127 on the left hand renders exactly what Key Touch off does.
2. With Key Touch on, every bass button, chord and free-bass note that
   sounds fully down still holds a steady tone at velocity 1, at the resting
   300 Pa, in every bass register.
3. Velocity 1 against 127 at 300 Pa, a C bass button in the default
   register falls 8 dB or more; a C chord 6 or more; a free-bass C3 8 or
   more. (Assumed from the treble's low keys, 10-15 dB: the bass holes are
   larger, so the knee sits deeper and the range is likely narrower.)
4. Key Touch off renders as before: every fingerprint of a scene with it
   off unmoved.

**Status (2026-10-03): built (0.13.19); all four met**
(`tests/milestone_9j.rs`; the floors in `touch.rs`).
1. **Met.** Velocity 127 renders a C bass button, a C chord and a
   free-bass C3 bit for bit as with Key Touch off.
2. **Met.** Every bass and chord button in all seven bass registers, and
   all 58 free-bass notes, hold a steady tone at velocity 1 at 300 Pa.
   The edges were first read with the treble's 1 dB steadiness, and in the
   registers of several ranks, and on the free bass in octaves, they came
   out 0.5 to 1 -- the C bass button in the five-rank register "never held",
   even fully down: the ranks' slow beat moved the level a dB or two from
   half second to half second. Read within 3 dB, as the tests read, every
   left-hand edge at 300 Pa is 0.11-0.15 of the 8′ hole; four free-bass
   notes at 200 Pa, A♯5 to C♯6, stand at 0.33-0.78. The treble's edges,
   measured again by the same run with 1 dB, came out identical.
3. **Met.** Velocity 1 against 127: the C bass button -14.3 dB, the C chord
   -13.1, the free-bass C3 -15.5 (velocity 64: -7.5, -6.4, -7.2). The
   prediction's 8, 6 and 8 assumed the larger holes would narrow it; the
   floors sit as deep in their holes as the treble's.
4. **Met.** Of the fingerprints, the 20 scenes that strike the left hand
   under 127 with Key Touch on moved; Digital Accordion, the master chords
   and the wheel as pressure, with it off, did not.

## 10e. The step's constants, once

The user's report (2026-10-03): on a handheld (ROG Ally, Ryzen Z1
Extreme, low-power profile) RF-Musette under RackForge misses blocks at
256 frames. The user's ask: make it cheaper without losing anything --
first what is bit for bit, then two reeds at a time (SIMD).

**Measured first.** The load: 30 s of a triad every half second (roots
C4 to A4), a low note (C2-B2) that the default split makes a bass button,
velocities 40-126, and four short melody notes above at 100; the Accordion
program as it comes, Key Touch on, stereo, 256-frame blocks at 48 kHz.
* Through RackForge on the handheld: 3.0-3.5 ms a block of a 5.33 ms
  deadline, 0-5 blocks late a run.
* Natively on the handheld, `perf`: 86 % in the reeds' step (`blow_rank`,
  `reed::step` inlined), 7 % the per-key loop, 2.6 % the stage, 1.6 % the
  decimator; no single hot spot inside the step. Some 13 treble and 17
  bass reeds move a block; without the low notes the block costs 40 %
  less.
* RackForge already runs every plugin through `wasm-opt -O3` as it loads
  it (its `optimize.rs`), so the package gains nothing from running it
  again.
* Through RackForge's own plugin host on this server (a harness outside
  the repository: the package loaded as the host loads it, subnormals
  flushed, the load above): 4590 µs a block, three runs within 15 µs;
  the output's fingerprint 5f54dee8600d2327.

Found in the step: eleven divisions and two square roots a reed a
substep. Of them, the cell's tube (its length in steps, its impedance,
where the slot's points fall on it), the hole's radiating end and the
near field's weights depend on the reed's model and the step's length
and nothing else; the curtain's mass and its viscous resistance on the
pallet's opening, which does not change while the pallet is still.

**Change (exact):** what depends only on the model and `h` computed once,
when the engine builds a reed, by the same operations in the same order;
the curtain's mass and viscous resistance kept with the reed's state while
the pallet does not move, as its Bernoulli factor already is (10c). A step
at an `h` its model was not prepared for computes them as it did.

**Predictions:**
1. Every fingerprint unchanged (`tests/fingerprints.rs`), every test
   passes, and the handheld load through RackForge's host gives the same
   fingerprint, 5f54dee8600d2327.
2. That load at most 3900 µs a block on this server (−15 %).
3. The engine still inside a sixth of its stack
   (`a_whole_instrument_fits_the_stack`).

**Status (2026-10-03): built; predictions 1 and 3 met, 2 NOT MET.**
1. **Met.** Every fingerprint unchanged, every test passes, and the
   handheld load through RackForge's host gives 5f54dee8600d2327 again.
2. **NOT MET.** 4590-4612 µs a block to 3978-3988 (four runs): −13.5 %,
   not the −15 % predicted -- 85 µs short of 3900.
3. **Met.** The engine is 1342 KiB, of 1365 (it was 1286): the prepared
   constants take 56 KiB, a reed's two curtain values 9.

## 10f. Two reeds at a time

The plugin is built with WebAssembly's SIMD (`+simd128`), and the reed's
step uses it only for a square root: the step is scalar, one reed after
another, and it is 86 % of the block (10e).

Within a substep the reeds do not see each other: each is blown by the
bellows' pressure the substep began with, through its own pallet, and what
they draw is summed after. So any two can be stepped side by side, in the
two lanes of an `f64x2`, each lane doing the operations its reed's own
step does, in the same order -- IEEE 754 per lane -- with what is a table,
a tube or a branch on one reed taken a reed at a time; and what they give
the bellows and the microphones summed in the order it always was.

**Change (exact):** each substep queues its reeds as it always visited
them, steps them two at a time, and sums them in that order.

**Predictions:**
1. Every fingerprint unchanged, every test passes, and the handheld load
   through RackForge's host gives 5f54dee8600d2327.
2. That load at most 3600 µs a block on this server (−10 % from 10e's
   3985): the step's divisions and roots, two for the price of one, against
   what packing two reeds into lanes costs.
3. The engine no larger: the queue lives in the render's frame.

**Status (2026-10-03): the two-lane step built, measured and withdrawn;
the queue kept.** Measured as 10e, through RackForge's host on this
server, the handheld load, several runs each.
1. **Met,** both ways: with the reeds stepped in lanes and with the queue
   alone, every fingerprint unchanged and 5f54dee8600d2327 again.
2. **NOT MET as written.** Two reeds in the lanes of an `f64x2`:
   4343-4401 µs a block, 10 % *slower* than 10e's 3974-3988. Taken apart:
   the queue with each pair stepped as two scalar steps, 3526-3533; the
   lanes loaded field by field rather than through closures, 4343-4376.
   Under RackForge's compiler the lanes cost more than they save: packing
   two reeds' numbers into lanes and taking them apart again for the
   section table, the cell's tube and the curtain, and both branches of the
   hole's row computed -- a shut pallet's divisions included -- where one
   reed takes one. Withdrawn. Kept: the queue, its reeds stepped one at a
   time in order: 3460-3487 µs, −12.8 % from 10e and −24 % from where 10e
   began (4581-4612), bit for bit. Why queuing alone is faster is not
   measured; the step now runs in a loop of its own, with the starts, the
   pallets and the sums outside it.
3. **Met.** The engine is 1342 KiB, as after 10e; the queue, 594 reeds at
   40 bytes, is in the render's frame.

## 10g. Every reed built before it is played

The user's report (2026-10-04): on the Raspberry Pi 4 RF-Musette has many
underruns.

**Measured first** (the handheld load of 10e through RackForge's host, run
on the Pi itself, out of real time): 15.4-16.0 ms a 256-frame block on
average against a 5.33 ms deadline, after 10e and 10f (0.13.19:
19.9-20.2); the worst block 25.6-25.8 ms, in the first half second, and
19.7-21.6 after it. A key's reeds' models are built when it is first
pressed (a fresh engine marks every key stale, and builds two idle ones a
block): natively 0.064 ms a model, 17 ms for all 265; on the Pi, in the
plugin, some ten times that -- 3 ms a five-rank key, and the load's first
chord, bass button and melody note build some 25 models in one block.

**Change (exact):** the plugin, as it is prepared -- out of real time, and
after it has set the parameters it carries -- has the engine build every
key's reeds (`Engine::prepare_reeds`); the reeds built are the ones the
first press built. A program loaded later still builds them as before.

**Predictions:**
1. Every fingerprint unchanged, every test passes, and the handheld load
   through RackForge's host gives 5f54dee8600d2327.
2. On the Pi the load's worst block in its first half second at most 10 ms
   (25.6-25.8); its average within 3 % of 15.4-16.0.
3. Preparing every reed at most 30 ms natively: the price, paid when the
   plugin is prepared.

**Status (2026-10-04): built; all three met.** Measured as before: the
handheld load through RackForge's host, on the Pi and on this server, the
previous build against this one, twice each.
1. **Met.** Every fingerprint unchanged, every test passes, and the load
   gives 5f54dee8600d2327 on both machines.
2. **Met.** On the Pi the first half second's worst block 26.0-27.5 ms to
   7.8-8.0; the average 15.6-15.8 to 15.5-15.6. The worst block after it,
   20.1-20.6 ms, did not move: it is the load's busiest moments, not
   building. On this server the first half second 4.1-4.2 ms to 1.8.
3. **Met.** Preparing every reed takes 17 ms natively.

## 10h. A reed at rest says so

**Measured first** (natively on the Pi, the handheld load, `perf`): 9.2 ms
a block natively against the plugin's 15.5 (the plugin's epoch checks are
8.5 % of that; the rest is the compiler). The load runs at 1.18
instructions a cycle, and 13.7 % of its L1 data accesses miss (4.7 of 34.1
billion) while almost none miss L2 (16 million), with 410 million TLB
refills: what a substep touches is larger than the Pi's 32 KiB of L1 and
spread over many pages. A reed's model is 1600 bytes, its state 88, its
cell 520; a key's five ranks some 14 KiB apart from the next key's. By
function, `queue_rank` -- which computes almost nothing -- is 30 % of the
cycles and 41 % of the L1 misses: for every rank of every sounding key it
compares each reed's whole state with the state at rest, two cache lines a
reed, a key's ten reeds 2.8 KiB apart; and so do `Rank::is_still` for
every sounding key at every substep and the check that everything has
stopped at every sample.

**Change (exact):** each plate keeps, beside the share of its start, a flag
per reed that is set exactly when its state is the state at rest, kept
wherever a state is written; a reed at rest with no air is passed over on
that flag, and a plate is still when both flags are set.

**Predictions:**
1. Every fingerprint unchanged, every test passes, and the handheld load
   through RackForge's host gives 5f54dee8600d2327.
2. Natively on the Pi the load's L1 data refills at least 20 % fewer
   (4.7 billion) and its time at least 10 % less (9.2-9.4 ms a block).
3. Through RackForge's host on the Pi at least 8 % less (15.5-15.6 ms).

**Status (2026-10-04): built; all three met.**
1. **Met.** Every fingerprint unchanged, every test passes, and the load
   through RackForge's host gives 5f54dee8600d2327, on this server and on
   the Pi.
2. **Met.** Natively on the Pi: 9.24 ms a block to 7.58 (−18 %); its L1
   data refills 4.57 to 3.47 billion (−24 %), its cycles 95.0 to 77.7
   billion. Its TLB refills did not move (412 million): the reeds' data is
   still spread over many pages.
3. **Met.** Through RackForge's host on the Pi 15.4-15.6 ms to 13.0
   (−16 %); on this server 3.47-3.49 to 3.16-3.18 (−9 %). Since 0.13.19 the
   load on the Pi has gone from 19.9-20.2 ms to 13.0 (−35 %), bit for bit.

## 10i. A cell's tube no longer than its waves

The user's report (2026-10-05): on the Raspberry Pi 4, with 0.13.20's
native build, Student 72 underruns when many right-hand keys are held. The
user's ask: lossless only; 1x oversampling sounds poor and is not an option.

**Measured first** (natively on the Pi, a bench outside the repository: the
plugin's processor, Student 72, ten right-hand keys C4-E5 held at velocity
100, 256-frame blocks at 48 kHz, the first half second left out): 5115-5161
µs a block against a 5.33 ms deadline; six keys 3087-3289, two 1319. At 1x
the ten keys cost 2598: the reeds' substeps are the cost. `perf stat`: 1.25
instructions a cycle; 408 million L1 data refills of 4.2 billion accesses
(9.7 %), 1.7-2.3 million L2 refills, 56 million TLB refills. By function the
L1 refills are 60 % `render_with` (the step inlined), 27 % `queue_rank`,
6.5 % the stage, 6 % the decimator; by instruction, some 30 % are the reads
of the cells' two lines of waves, 7 % the section table, some 15 % the
model's step constants. Ten keys of Cello are some 30 reeds, and what one
substep touches is about the Pi's 32 KiB of L1.

Each cell's two lines are rings of 64 waves, read up to its delay back --
some 15 steps at 2x for a 50 mm cell -- and written at a position that
walks the ring: over 64 substeps a reed's lines pass through all eight of
their cache lines, two at a time.

**Change (exact):** each reed's rings are as long as the smallest power of
two above the farthest its step reads back, and the position the ring is
written and read at is the one it was, taken modulo that length. The waves
read are the ones they were, bit for bit, and a reed's lines stay in the
same one or two cache lines. The one place that reads differently is a
cell whose delay changes while its reed sounds -- the oversampling changed
mid-note -- where the old rings reinterpreted their history too.

**Predictions:**
1. Every fingerprint unchanged and every test passes; the native build
   against 0.13.20's packaged component (`rackforge-core compare-native`)
   identical in all twenty programs.
2. On the Pi, the bench above: at least 15 % fewer L1 data refills (from
   407-409 million) and the block at least 8 % cheaper (at most 4700 µs).
3. The engine no larger: the rings keep their 64 slots, only fewer are
   touched.

**Status (2026-10-05): built, measured and withdrawn; prediction 1 met, 2
NOT MET.** Measured on the Pi with the bench, the previous build and this
one in turn, three runs each, `perf stat`.
1. **Met.** Every test passes, and against 0.13.20's packaged component
   the native build was identical in all twenty programs.
2. **NOT MET.** The block 5002-5286 µs to 5382-5421 (+5 %), and the L1 data
   refills 371-380 million to 389-399 (+5 %), not 15 % fewer. Not
   measured, the likeliest reason: the rings walking their 64 slots spread
   the reeds' lines over the L1's sets, and shortened they all sit at the
   same offsets of their structures, which the Pi's two-way L1 cannot hold
   at once. Withdrawn: the code is as it was.
3. Not reached.

## 10j. A shut register's plate, passed over

Student 72 sounds Cello, three of a treble key's five ranks; the other two
are shut. At every substep each sounding key's loop still reads, for every
rank, the model's tone hole and rim -- two of its cache lines -- and computes
the pallet's curtain for it, before `queue_rank` finds the register shut and
both reeds at rest and only clears their starts. With ten keys held that is
some forty model lines a substep for nothing.

**Change (exact):** a rank whose register is shut and whose plate is still
-- both reeds' resting flags set, read beside its starts -- has its starts
cleared, as `queue_rank` would, and nothing else read or computed. A shut
rank still ringing down is handled as before.

**Predictions:**
1. Every fingerprint unchanged and every test passes; the native build
   against 0.13.20's packaged component identical in all twenty programs.
2. On the Pi, 10i's bench, the previous build and this one in turn, five
   runs each: at least 8 % fewer L1 data refills and the block at least 4 %
   cheaper.
3. Nothing changes where every register is open (Musette Paris and the
   other full registers): within the runs' spread there.

**Status (2026-10-05): built; predictions 1 and 2 met, 3 NOT MET as
written.** Measured on the Pi with 10i's bench, the previous build and this
one in turn.
1. **Met.** Every test passes, and against 0.13.20's packaged component the
   native build is identical in all twenty programs.
2. **Met.** Student 72, ten keys, five runs each: the block 5015-5316 µs
   (median 5136) to 4723-5060 (median 4881), −5.0 %; the L1 data refills
   372-422 million (median 382) to 333-362 (median 343), −10.1 %; the
   cycles 10.95 billion to 10.46 (medians), −4.5 %.
3. **NOT MET as written,** for a wrong premise: Musette Paris is not a full
   register -- Musette is the three middle reeds, the bassoon and the
   piccolo shut -- so it gained too: 5014-5382 µs to 4793-4846, three runs
   each.

## 10k. What a substep shares, computed once

**Measured first** (the Pi, 10i's bench at 10j, `perf stat` by key count):
the cycles a held key costs fall as keys are added -- 1.12, 0.98 and 0.95
billion for two, four and ten keys over six seconds -- while its L1 refills
rise, 15, 19 and 33 million: the misses are hidden behind the work, and the
work is the instructions, some 720 a reed's step at 1.3 a cycle. Gathering
the sounding reeds' data into one place to fit the L1, the next step this
roadmap considered, would not pay; it is not done. What remains is fewer
instructions for the same results.

Two of them are the same number computed many times:
* the share of its start a reed is owed, P/(P + P₀), is divided out for
  every blown reed at every substep, and P is its side's pressure, the same
  for every reed on that side;
* the bellows' target pressure is a power of the intent, computed at every
  sample, and the intent does not change within a block.

**Change (exact):** each substep computes the share once for each side, and
each reed takes its side's; the target is computed once a block. The same
operations on the same numbers.

**Predictions:**
1. Every fingerprint unchanged and every test passes; the native build
   against 0.13.20's packaged component identical in all twenty programs.
2. On the Pi, 10i's bench, Student 72 with ten keys, 10j's build and this
   one in turn, five runs each: the block at least 2 % cheaper (median).

**Status (2026-10-05): built; both met.** Measured on the Pi with 10i's
bench, 10j's build and this one in turn, five runs each.
1. **Met.** Every test passes, and against 0.13.20's packaged component the
   native build is identical in all twenty programs.
2. **Met.** Student 72, ten keys: the block 4972-5104 µs (median 5038) to
   4841-4877 (median 4854), −3.7 %; the cycles 10.36-10.73 billion (median
   10.57) to 9.93-10.23 (median 9.99), −5.5 %; the instructions 12.94 to
   12.70 billion, −1.9 %. With 10j, Student 72's ten keys have gone from
   5136 µs a block to 4854 on the Pi, bit for bit; 91 % of the deadline.

## 10l. A program change that does not click

The user's report (2026-10-07, through RackForge on the Raspberry Pi 4):
every change of program clicks.

**Measured first** (RackForge's engine on the Pi, and a bench of the native
processor alone, nothing held): loading a program takes 12-16 µs, but the
26 blocks after it take 6.8-7.1 ms each against a 5.33 ms deadline. A
program moves the reed parameters, so every key is marked stale and two
idle ones are built a block (10g's arrangement); `perf` puts 95 % of that
time in `ReedModel::section_at`, the useful-section table: 256 points, each
summing 65 points along the tongue, a square root at most of them -- some
16,600 a model, five models a key.

**Change (exact):**
* one idle key is built a block, not two; an idle key does not sound, so
  when it is built changes nothing it gives;
* the section's square roots are taken two at a time (NEON), the sum kept
  in the same order: IEEE square roots round the same in a vector as alone;
* the models built for a program are kept, by design, for the programs
  used last, so going back to one builds nothing.

**Predictions:**
1. Every fingerprint unchanged and every test passes; the native build
   against 0.13.21's packaged component identical in all twenty programs;
   every reed's section table, in every program, identical bit for bit to
   the one the scalar loop builds.
2. On the Pi, the bench of a change between Student 72 and Musette Paris
   (nothing held, the first change from each): no block over the deadline
   after it (26 now); the worst at most 3.5 ms (6.8-7.1).
3. Changing back to a program used just before builds nothing: the first
   block after it under 1 ms.
4. Through RackForge on the Pi, an hour changing program every five
   minutes with a chord every minute: no underrun at the changes (6-9 an
   hour now).

**After the first two changes** (measured, before the rest is written):
the section's roots now run in the vector unit and the table is the same
bits for every reed of every program, but the Pi 4's Cortex-A72 takes a
double's root unpipelined, some thirty cycles a lane, so a key went from
3.45 ms to 3.1 ms, not to a half: what is left is the hardware's. With one
key a block the Pi's worst block after a change is 3.1-3.2 ms, none over
the deadline; the rebuild lasts 53 blocks, 280 ms. Keeping the models of
past programs (the third change) is withdrawn: the engine allocates
nothing, and a program's models would add some 265 KB to it, which 8p
kept small for its stack.

What remains is a key pressed within those 280 ms: it is built in the
block it is pressed in, 3.1 ms a key. Two more changes, exact, since an
idle key's models change nothing it gives:
* idle keys are built nearest the last key played first, the treble's
  and the bass's, where the player's hands are;
* they are built only while no key is held, so a chord being played is
  not given a key's build on top.

**Predictions** (in addition to 1-4):
5. On the Pi, the bench's change with a chord of four keys around C5 50 ms
   after it (as a player changing and playing on): no block over the
   deadline (the same chord's keys, built on press, in 10l's first part:
   measured at the start of this part).

**Measured before the last change:** with those two, the chord bench's
first block after a change took 18 ms: the keys still ringing from before
it are rebuilt at once, as exactness asks (their next sample is the new
program's), five or six keys of 3.1 ms. Blocks 1-8, one idle key each on
top of the tails, 5.5 ms.

One more change, exact: a model is a function of its design, the tongue's
mode and the step alone, so a key's model whose three are the same bits is
kept through the rebuild rather than built again. Measured across every
pair of the twenty programs, 71 % of the reeds keep their design (Student
72 and Musette Paris: 183 of 265). A key that keeps all five of its models
is not counted against the block's build.

**Predictions** (in addition to 1-5):
6. Prediction 1's checks all hold (a kept model is the model a build
   would have given).
7. On the Pi, the chord bench's first block after a change at most 9 ms
   (18), and the idle bench's rebuild over in at most 25 blocks (53).

**Measured, and the budget changed** (the bench on the Pi, the first change
from each program and three more): kept models took the idle rebuild from
53 blocks to 41 and its worst block from 3.1 ms to 1.4, but a key now
builds one or two models, not five, and the chord bench's first block
after a change was 8.6 ms. So the budget became models, not keys: five a
block, a key's worth. The idle rebuild then took 21 blocks, its worst
2.75 ms, but the chord bench's first block grew to 10.2 ms and the block
releasing the chord went over (5.4-5.6 ms): five models on top of keys
still ringing. Last change, exact like the rest: while anything sounds,
held or ringing, a single model a block; five only in silence. A key may
be left half built, so the step and mode each model was built for are
kept per rank, and the key stays stale until its last model is built.
This replaces building only while nothing is held: a player who changes
program and plays on, never letting go, still sees the rest built.

**Results** (2026-10-07):
1. MET. Every test passes (195), the fingerprints with them; the native
   build against 0.13.21's packaged component is identical in all twenty
   programs; every reed's section table, in every program, is the scalar
   loop's to the bit.
2. MET. Idle bench: no block over the deadline (26 before); the worst
   3.26-3.32 ms (6.8-7.1); the rebuild lasts 17 blocks, 91 ms.
3. WITHDRAWN with the third change (keeping past programs' models). A
   change back now builds what differs between the two programs only,
   the same 17 blocks either way between Student 72 and Musette Paris.
4. Pending: the hour through RackForge.
5. NOT MET as written. The chord 50 ms after a change no longer costs a
   block: blocks 1-59 are all under the deadline, the chord's block among
   them (it was 11.2 ms on the first change). But the change's own block
   goes over when the previous chord still rings, 8.1-8.2 ms: its keys are
   built at once, as exactness asks, and a key that rings for seconds at
   -40 dB and below still rings.
6. MET: the checks of prediction 1, and a test plays every program after
   every other, at once and with a key sounding through the change, bit
   for bit against an engine built for the program alone.
7. MET. The chord bench's first block 8.1-8.2 ms (18), the idle rebuild
   17 blocks (53).

What remains, the keys still ringing at a change, is the hardware's root
and exactness: making it cheaper would mean a ringing reed keeping the old
program's model until it is still, which is a change of sound, the user's
to decide.

**After 0.13.22:** its record of the step and mode each model was built
for, per rank, added 6 KiB to the engine, and the debug build of a test
holding two engines (milestone 3's) overflowed CI's 8 MiB test stack: it
needed 7.5-7.8 MiB already, at 0.13.21. The record is gone: a rebuild that
changes the tongue's mode or the oversampling lets every model go, and a
kept model is one whose design is the same bits. A model's design carries
the mode ratio, and one prepared for another step plays the same bits
(it computes its step's constants every step instead), so this is about
cost alone; the test covers a change of step as well. Back to 0.13.21's
7.8 MiB.

**Prediction 4, the hour through RackForge** (2026-10-07, the Pi, 0.13.22
native, RackForge's soak changing between Student 72 and Musette Paris
every five minutes, a four-key chord around C5 every minute): NOT MET.
Twelve overruns in the hour, exactly one at each of the twelve changes,
none in the other 48 minutes; the late block 5.4-6.9 ms. The soak strikes
its chord the moment it has changed program, and the bench, the chord
moved from 50 ms after the change to the blocks right after it, gives the
same: struck within 0-3 blocks (16 ms) of the change, that block takes
6.5-8.4 ms, its four keys built on press; struck 50 ms after, none over.
The "6-9 an hour" the prediction named came from soaks changing between
instruments, not programs (77-81 overruns an hour, most from the
instruments' resets): there is no like-for-like hour of 0.13.21, whose
bench gave 26 late blocks after every change.

So a program change no longer clicks unless a chord lands within some
16 ms of it, or keys from before it still ring. Both are keys that must
sound the new program at once, built in the block they need it.
