# Misdariis, Ricot & Caussé, CFA 2000: notes from a full reading

N. Misdariis, D. Ricot, R. Caussé, "Modélisation physique de la vibration
d'une anche d'accordéon", *5e Congrès Français d'Acoustique*, Lausanne,
September 2000, pp. 281-284. IRCAM. Open access: HAL hal-01161356
(https://hal.science/hal-01161356). Four pages, in French. Read in full on
2026-09-30, text and figures; Fig. 3 digitised here from a 400 dpi render.

The research survey summarised it earlier ([REED-PHYSICS.md](REED-PHYSICS.md)
§3.6). This file records what a full reading adds and corrects.

## What it says

* **The reed.** Types +E and −I (Bouasse's classification, after
  Helmholtz): the tongue closes the slot when blown from outside or drawn
  from inside. Free (it passes through the slot, never beats) and strong (its
  own frequency sets the pitch). It vibrates on its first clamped-free mode,
  slightly below its natural frequency (their ref. [3], Koopman &
  Cottingham 1996).
* **The sound.** A jagged near-field pressure waveform with one depression
  and several localised overpressures per cycle, the same when pushed and
  drawn, inverted. The spectrum is broad; among the first partials the even
  harmonics are much weaker than the odd. The reservoir and the reed's
  small cavities, upstream or downstream, have a negligible influence on the
  spectrum *in their rig*.
* **Measurements (Fig. 2).** Tongue displacement, a hydrodynamic pressure
  "a few millimetres downstream of the tongue" (labelled "pressure
  fluctuations on the tongue"), and the near-field acoustic pressure,
  recorded together.
  * The displacement is sinusoidal over the whole supply range used,
    10-6000 Pa. Normal playing is 10-300 Pa for a mid-register reed. A laser
    vibrometer confirmed both the sinusoid and the deflected shape.
  * The level spans about 40 dB over the useful range of P₀ (the curve is
    not shown).
* **Two regimes with hysteresis (Fig. 3).** In regime 1 the frequency falls
  linearly below the natural frequency as P₀ rises. In regime 2 it rises
  fast and can pass above it. They correspond to the tongue's equilibrium
  outside and inside the plate respectively. Regime 2 is "observable for
  some reeds in normal playing conditions".
* **The flow, visualised in water.**
  * Visualisation in air failed. The reeds oscillate in water too, on their
    first mode: one reed plays 330 Hz in air and 142 Hz in water, with the
    same frequency-against-P₀ behaviour.
  * Flow speed is ~15 m/s in air and 2 m/s in water, so the Reynolds numbers
    are of the same order.
  * Upstream the flow is laminar and of "sink" type. Near the tongue it is
    two-dimensional: fluid in a plane across the tongue's long dimension is
    drawn into the gap in that same plane.
  * Downstream it is a one-directional jet that stays attached to the slot's
    wall.
  * The jet's turbulent instabilities do not drive the reed (as St. Hilaire
    et al. 1971 found). The Strouhal number for such a configuration is
    ~0.15.
* **The mechanism.**
  * The exciting force comes from the inertia of the fluid upstream of the
    tongue, as Bouasse foresaw.
  * Pressure forces act only on the upstream face, bordered by the two jets.
    The downstream face sits in "dead" fluid.
  * The important parameters are P₀ and K₀, "the quantity of fluid
    available in the reservoir that takes part in the inertia".
* **The model.**
  * A mass-spring-damper tongue, plus a two-dimensional, unsteady, inviscid,
    irrotational, incompressible potential flow: a sink upstream and a jet
    downstream. The pressure on a section of the tongue is computed from its
    position, which sets the opening.
  * It has no analytical solution and is solved numerically.
  * Simplification: the sink, really of variable length, is taken as a
    point, and the flow's changes of intensity are assumed to dominate the
    forces.
  * Fig. 4 compares simulated displacement, hydrodynamic and acoustic
    pressure with Fig. 2. The acoustic pressure is a linear combination of a
    monopole and a dipole fed by the model's flow.
  * The authors call the results "very encouraging" but the validation
    incomplete, and the acoustic-pressure model in need of improvement.
* **Open questions they list:**
  * the influence of the reservoir's size on the inertia effects;
  * in-vivo measurements of P₀ over time in playing, of reeds across the
    register, and of the small cavities.

## What it does not give

No equation, no value of K₀, no reed dimensions, no force law, no
coefficients. The model it summarises is in D. Ricot's end-of-studies thesis
(*Modélisation physique de la vibration d'une anche d'accordéon*, École
Centrale de Lyon, 1999; its ref. [1], not found online) and in Ricot, Caussé
& Misdariis, *JASA* 117 (2005). A slip in the paper: the text cites Bouasse
as [1] while the bibliography lists him as [2] and Ricot 1999 as [1].

## Read off the figures

**Fig. 3, frequency against P₀** (one reed, ~304 Hz; digitised from a
400 dpi render, ±0.1 Hz, ±40 Pa):

| P₀ (Pa) | 30 | 230 | 425 | 625 | 840 | 1250 | 1810 | 2230 | 2610 | 3040 | 3410 | 3790 | 4430 | 6070 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| f (Hz), regime 1 | 304.1 | 303.6 | 303.1 | 302.6 | 302.1 | 301.3 | 299.9 | 298.9 | 298.0 | 297.2 | 296.0 | 295.9 | 295.4 | 294.0 |

Regime 2: 296.6 Hz at 3970 and 4630 Pa, then 297.6 (4780), 298.6 (5000),
302.2 (5370), 305.6 (5570), 305.9 (5820).

Regime 1 is linear up to 3.4 kPa at −2.4 Hz/kPa, **−13.6 cents/kPa**, and
flatter beyond. The survey's reading, "~−10 to −13 cents/kPa", is
confirmed. Over normal play (10-300 Pa) that is about −4 cents.

**Fig. 2 against Fig. 4.** They are not the same reed: the measured period
is 1.26 ms (~790 Hz), the simulated one 3.33 ms (~300 Hz). The comparison
is of waveform shape only.
* In the measurement, the "pressure on the tongue" is roughly in phase with
  the displacement, leading it by about 20°, with a sharp dip (A) at each
  passage through the plate.
* The simulation reproduces the dip and the sign.
* The sign convention of the displacement is not stated, so no negative
  damping is inferred from this phase.

## What RF-Musette took from it

* Milestone 1's bounds: the playing range, the sinusoidal tongue, ~40 dB
  (unchanged from the survey).
* The measured pitch slope above, now as a number.
* The flow's description, from which milestone 2b's second attempt derives
  two forces on the tongue (ROADMAP 2b; `tests/sink_flow.rs`). Both are this
  project's derivation from the paper's words, not the authors' model:
  * **(A)** the steady Bernoulli deficit of a half-plane sink on the
    upstream face, (2/π²) Δp g (1 − g/w) per unit edge length;
  * **(B)** the sink's inertance, ρ ln(w/g)/(π dx) per edge element.
