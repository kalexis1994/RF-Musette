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

## 9. The product

A schema 3 package with branding, a PLAY surface, and factory programs.
