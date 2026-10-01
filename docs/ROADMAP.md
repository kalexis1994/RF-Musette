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
takes over and the valves flip. **Open decision:** how direction reaches the
plugin, since no MIDI standard carries it.

## 4. Ranks, registers and the musette

L (16′), M (8′), M+ and M− (tremolo), H (4′), and register switches. The
tremolo curve across the compass is voiced by ear inside its published bounds:
0.5–7 Hz at A4 from dry to wet, doubling roughly every 1.5 octaves, capped at
10–15 Hz (Hergert 2023/2024). The beat comes from separate reeds, never an LFO.

## 5. One bellows for every reed

A compliant reservoir fed by the player's intent and drained by every open reed
and the plates' leakage, so more ranks and more notes lower the pressure each
reed sees; the air button. Bellows compliance and leak are unmeasured in the
literature: voiced by ear, and stated so.

## 6. Cassotto, grille and body

The cassotto's resonance (0.8–1 kHz, Richter 1989) and its high-frequency loss
(Llanos: centroid lower, attack durations unchanged); the grille and the body
as a filter. No published transfer function exists: the body is derived from
geometry where it can be and voiced where it cannot.

## 7. The whole treble compass, and what it costs

Every key of a 41-key treble, every rank, and a full-register chord inside the
Pi 4's budget — with `parallel_render_v1` if one core is not enough, decided
by the fuel measured in milestone 1, not assumed.

## 8. Stradella bass

Bass and chord buttons on octave-wrapped ranks.

## 9. The product

A schema 3 package with branding, a PLAY surface, and factory programs.
