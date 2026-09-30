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
inertance, which milestone 1 treats as wide open.

**Predictions.** A finger attack (pallet opening on a pressed bellows) reaches
the note in 50–140 ms and a bellows attack (key held, pressure rising) in
190–660 ms, measured −50 → −5 dB on the first harmonic (Llanos-Vázquez 2014);
a partly opened pallet bends the pitch down by 15–35 cents (Elejalde-García
2021). A cell whose resonance sits near or just below a reed's frequency
raises its threshold "far above normal" or chokes it, one slightly above
helps it (Cottingham ICA 2019; Tonon 2005) -- carried over from milestone 1's
prediction 4.

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
