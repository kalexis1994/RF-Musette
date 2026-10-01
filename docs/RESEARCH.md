# RF-Musette research ledger

Survey of the literature, open projects, prior art and reference data for a
physically modelled accordion, made on 2026-09-30 before any code exists. It
sets what the model may claim, what it will have to measure itself, and which
decisions the evidence leaves to the player.

The detailed notes live beside this file, one per front, each source with its
full citation, access route and what it reports:

| File | Front |
| --- | --- |
| [research/REED-PHYSICS.md](research/REED-PHYSICS.md) | The free reed as a sound source: mechanics, aerodynamics, pitch against pressure, transients, cavities |
| [research/INSTRUMENT-ACOUSTICS.md](research/INSTRUMENT-ACOUSTICS.md) | The instrument around the reeds: bellows, pallets, chambers, valves, registers, tremolo, cassotto, Stradella |
| [research/SYNTHESIS-AND-PRIOR-ART.md](research/SYNTHESIS-AND-PRIOR-ART.md) | Published synthesis models, numerical schemes, open code, patents, MIDI conventions of digital accordions |
| [research/REFERENCES-AND-DATA.md](research/REFERENCES-AND-DATA.md) | Candidate reference recordings and datasets, with licences, and a capture protocol |

**Evidence tags used throughout.** VERIFIED means the source was seen on a
publisher, index or author page; the notes say whether the full text was read
(FT) or only the abstract (ABS). UNVERIFIED means a secondhand, forum or
practitioner statement. `(fig-read)` marks a number read off a plotted figure.
Nothing below may be quoted in `MODEL.md` with a stronger status than it has
here.

## What the survey concludes

1. **No open, real-time physical model of an accordion exists.** The one
   recent academic attempt (Puranik & Scavone, McGill: PoMA 49, 2022; DAFx-23;
   Forum Acusticum 2023) adapted the Millot–Baumann reed to a harmonium with a
   bellows chamber, found its stability depended on parameters the player does
   not control (chamber volume, reed clearance), and fell back to a wavetable
   through ten biquads. Commercial instruments are sample-based: Roland's
   US 6,946,594 (expired) claims per-reed samples modulated by bellows
   pressure, per-reed on/off thresholds and a per-reed pitch drop — the
   behaviour described, not simulated. Open code found is either unlicensed
   and numerically clamped (a forward-Euler harmonium) or non-physical.
2. **The accordion reed needs no bore.** It is a blown-closed free reed that
   self-oscillates without acoustic coupling; the excitation is the inertia of
   the unsteady flow upstream of the gap, which makes the gap flow lag the
   reed (St. Hilaire, Wilson & Beavers, *J. Fluid Mech.* 49, 1971; Ricot,
   Caussé & Misdariis, *JASA* 117, 2005; Misdariis, Ricot & Caussé, CFA 2000,
   the only accordion paper with pressure numbers we could read in full). The
   radiation is dipole-dominated (Ricot 2005). A lumped model is therefore
   physically sufficient, and waveguides have little to do.
3. **The model backbone is published; its numerics are the open problem.**
   Millot & Baumann (*Acta Acustica* 93, 2007; arXiv 2401.01606) give the
   minimal model — one-mode reed, upstream compliance and inertance, jet
   through a "useful section" that accounts for reed thickness and clearance —
   solved with Newton–Raphson falling back to bisection. Tarnopolsky, Fletcher
   & Lai (*JASA* 108, 2000) give a validated lumped valve with every
   coefficient (for the other valve class, see below). The recipe that removes
   McGill's failure is energy-consistent and non-iterative: Darabundit &
   Scavone (*Frontiers in Signal Processing*, 2025, doi
   10.3389/frsip.2025.1519450) prove the Bernoulli flow port passive and
   discretise a single reed with a linearly implicit scheme that solves a
   quadratic directly. It was written for a beating clarinet reed; applying it
   to a free reed with Millot's section function is new work, and it is the
   work this project exists to do.
4. **The expression is the bellows, not the key.** The key opens a pallet;
   level, brightness and a small pitch shift come from pressure. Roland's
   FR-series MIDI implementations send bellows as 7-bit CC 11 Expression,
   with no direction message, and select registers with Bank Select + Program
   Change. Open controllers send CC 11 as 14-bit (with CC 43) and carry
   direction in their own SysEx (Bandolibre), or use CC 7 (a load-cell
   concertina). No standard exists for signed or high-resolution bellows.
5. **There is no public reference instrument of Salamander quality.** The
   only usable open set is the IRCAM TinySOL accordion (Zenodo 3685367: 689
   notes E1–C#8, pp/mf/ff, near-dry proximity mic, mono 44.1 kHz), with its
   instrument, registers and bellows direction undocumented and a licence that
   reads CC BY 4.0 on Zenodo but "free for non-commercial usage" in the 2020
   paper. Bellows pressure — the model's input — is in no dataset. A reference
   that meets the Concert Grand's standard has to be recorded.

## The signal path, and how much of it is measured

| Stage | What it does | Best source | Evidence |
| --- | --- | --- | --- |
| Player → bellows | Force and travel become a pressure; opening and closing differ slightly (bellows rigidity) | Ramos et al., NIME 2022/2023 (bandoneon, pressure recorded with sound) | Described; in-play pressures on an accordion unmeasured in any readable source |
| Bellows reservoir | One compliant volume feeding every open reed, plus gap and valve leakage; more ranks drain it faster | Puranik & Scavone DAFx-23 (harmonium); Ziegenhals, IfM Zwota 2009 (gap flow adds up across ranks) | Described; compliance, volume and leak rate unmeasured |
| Reversal and air button | Flow changes sign, the other reed of each pair speaks, valves flip, a brief gap | McMahan 2016; Roland US 6,946,594 | Described only |
| Keyboard pallet | A variable orifice: its opening rate sets the finger attack, partial opening bends the pitch | Llanos-Vázquez et al., *Acta Acustica* 100, 2014; Elejalde-García et al., AiP 2021 | Measured |
| Reed chamber | Upstream Helmholtz/quarter-wave cavity; sets the threshold, can choke a reed tuned near it, moves pitch slightly with volume | Cottingham ICA 2016/2019; Tonon, PICA 2, 2005 | Measured in the lab, not per note on a real accordion |
| Reed plate pair and valves | Leather to G4, plastic G#4–C6, none above (Pigini Sirius); the idle slot leaks where there is no valve | Llanos 2008/2014; Harmonikas.cz gap specification | Layout measured; valve dynamics unstudied |
| Reed and gap flow | The self-oscillating source | Millot & Baumann 2007; Ricot 2005; Misdariis 2000; Cottingham group | Measured on a few reeds; almost no steel-reed constants |
| Ranks and tremolo | 16′ / 8′ / 4′ and detuned 8′ ranks beating across the compass | Hergert, Forum Acusticum 2023 and *Acta Acustica* 8, 33 (2024); Porvenkov 1979 | Measured curves (in figures) and perceptual rules |
| Cassotto | A folded chamber in front of some ranks: resonance and high-frequency loss | Richter, IfM Zwota 1989; Llanos 2014 | Sparsely measured; no transfer function published |
| Grille, tone holes, body | Treble and bass sides radiate differently | Richter 1989; Puranik DAFx-23 (enclosure as ten biquads, by inverse filtering) | Partly measured; no accordion directivity data |

## Numbers the literature does give

| Quantity | Value | Source | Status |
| --- | --- | --- | --- |
| Playing pressure, mid-register reed | 10–300 Pa normal play; oscillation sinusoidal from 10 to 6000 Pa | Misdariis et al., CFA 2000 | VERIFIED-FT |
| Lab reference pressure | 600 Pa; sweeps 0.05–0.85 kPa | Cottingham, CCRMA 2013 slides; Coyle et al. 2009 | VERIFIED |
| Onset/offset threshold | 60–110 Pa for a 622 Hz reed in its favourable direction, 0.8–1.7 kPa the other way; ~200 Pa for reed-organ reeds | Cottingham ICA 2016; Cottingham et al. 1999 | VERIFIED |
| In-play range | ~30 Pa threshold, 200–300 Pa average, 600–800 Pa very loud, ~1 kPa where valves start to blow | Technician on musiker-board.de | UNVERIFIED |
| Dynamic range of one note | ~40 dB (Misdariis); 40–50 dB at 1 m (Llanos et al. 2002); p ≈ 55 dBA, mf ≈ 70 dBA at 50 cm (Llanos 2014) | as named | VERIFIED |
| Attack time, harmonic 1, −50 → −5 dB | Finger 50–110 ms (mf), 60–140 ms (p); bellows 190–630 ms (mf), 250–660 ms (p); no trend with note | Llanos-Vázquez et al. 2014 | VERIFIED-FT |
| Spectral centroid in the attack | ~8.5–9 kHz early, settling near 2.5 kHz | Llanos-Vázquez et al. 2014 | VERIFIED-FT |
| Pallet pitch bend | ~15 ct finger only, ~25 ct bellows and finger, up to ~1 semitone; none from C6 to C#8 (no valves) | Elejalde-García et al. 2021; Llanos 2008 | VERIFIED-FT |
| Chamber volume and pitch | 17 → 9 cm³: 91.4 → 89.2 Hz; a chamber resonance near the reed frequency raises the threshold "far above normal" or chokes it (344 Hz reed on a 327 Hz chamber) | Cottingham slides 2013 (fig-read); Cottingham ICA 2019 | VERIFIED |
| Tremolo beat at A4 | 0.5–7 Hz from dry to wet, capped at 10–15 Hz, doubling about every 1.5 octaves; Porvenkov: 1.5 Hz at E2, 2.75 Hz at B4, 13.2 Hz at A7; one Borsini: −3.7 Hz / +4.1 Hz | Hergert 2023/2024; Porvenkov 1979 | VERIFIED (Porvenkov via Hergert) |
| Style names | ~4 Hz standard, 5–6 Hz "French" at A4 | Liberty Bellows (repair shop) | Practitioner |
| Cassotto | Resonance 0.8–1 kHz; "about 30 % attenuation in SPL above 6 kHz" (unit ambiguous); centroid lower, attack duration unchanged | Richter 1989; Llanos 2014 | VERIFIED, sparse |
| Reed plate | F4 tongue 36 × 4 mm, 0.5 mm set, 3 mm plate, > 4 mm tip amplitude at mf; air gap 0.03–0.06 mm modern, ~0.1 mm on old plates | IfM Zwota 2008 poster; Harmonikas.cz | VERIFIED (maker specification for gaps) |
| Tip amplitude | ~15 % of tongue length | Cottingham, *Physics Today* 2011 | VERIFIED-ABS |
| Reed Q | Harmonica reed 95 (Millot & Baumann); reed-organ D = 0.012; brass flap ~55 bare; a bench reed ~250. **None for a steel accordion reed.** | as named | VERIFIED |
| Mode ratios of blown reeds | Far from the uniform bar's 1 : 6.27 : 17.5 — f2/f1 ≈ 8.4 on one reed-organ reed, ≈ 3.9 on another; torsion present in the attack | Cottingham ICA 2019; CCRMA 2013 | VERIFIED |
| Stradella ranks | Bass C2–B2, tenor C3–B3, contralto F#3–F4, alto C4–B4, soprano C5–B5; octave breaks staggered between ranks | Wikipedia (unreferenced); P. M. Haas | UNVERIFIED |

## Where the sources disagree, and how they reconcile

**Pitch against pressure.** Four statements look incompatible:

* "Free reeds keep their pitch under changing wind pressure" (Braasch &
  Cottingham, *Acoustics Today* 2023); "no noticeable pitch change" across
  40–50 dB (Llanos et al. 2002); harmonium f0 moves under 1–2 Hz (Puranik &
  Scavone 2023); Tarnopolsky 2000 found frequency insensitive to pressure.
* A mid-register accordion reed falls about 9 cents over 0.1–0.9 kPa,
  steeper below 0.3 kPa (Cottingham, fig-read); Misdariis measures about
  −10 to −13 cents/kPa out to 6 kPa, and a second regime — the reed's rest
  position inside the slot — in which the pitch *rises*.
* A bandoneon's low notes fall almost half a semitone at pressure extremes
  while its highest notes rise (Ramos et al. 2023); an amateur accordion
  measurement finds up to 10 ct near C4 and 40 ct in the bass (Faragó &
  Wiech 2024, unverified); Roland's patent claims a drop that is larger for
  low reeds and possibly zero at the top.

They are consistent once the scale is read. "Stable" is said against other
wind instruments, where pressure moves pitch far more; Tarnopolsky's valve is
the other class of free reed (a (+,−) flap, not the accordion's (−,+)); and the
shift is register-dependent, a few cents mid-range, tens of cents in the bass,
of the opposite sign near the top. Misdariis's second regime is a candidate
*cause* for that sign change, not yet a demonstrated one. The model must not
be given a pitch-against-pressure law: the shift has to come out of the reed
and flow equations, and this ledger's numbers are what it will be checked
against. That is a prediction the first reed model can fail.

**Thresholds** span 10 Pa (Misdariis, one accordion reed) to 200 Pa
(reed-organ reeds) to 0.8–1.7 kPa (a reed blown the wrong way). They depend
on the reed, the direction and the chamber; no single number is "the"
threshold, and the model's thresholds must also emerge.

**The cassotto's "30 %"** is ambiguous between a fraction of the sound level
in decibels and a fraction of the pressure. Llanos's thesis is the place to
settle it; until then it is quoted, not used.

## What the model will have to measure for itself

The literature leaves these open, so the project's own reference has to
supply them or the model has to state them as assumptions:

1. Steel accordion-reed constants: Q, tip stiffness, effective mass, the
   thickness profile, and dimensions across the compass.
2. Pitch, level and brightness against pressure, per reed, push and pull
   separately.
3. Bellows pressure during real playing; bellows compliance and leak;
   the pressure trace and sound gap across a reversal.
4. Flow per reed and the pressure drop when many ranks sound.
5. Valve dynamics: opening pressure, closing transient, buzz.
6. Per-note reed-chamber geometry, which places each chamber's resonance.
7. A cassotto transfer function per register.
8. Directivity, treble side against bass side.
9. A real 120-bass instrument's rank layout and octave breaks.
10. The size of the onset pitch glide that Braasch & Cottingham describe but
    do not quantify, and whether onset is a subcritical bifurcation.

## Reading list, in order

Open access unless stated:

1. **Llanos-Vázquez, R., *Acústica del acordeón*,** PhD thesis, UPV/EHU 2015,
   hdl 10810/16562. The only whole-instrument study of an accordion (a Pigini
   Sirius): attacks, cassotto, bend, probably register and bellows-direction
   spectra. Behind a reCAPTCHA: it has to be downloaded by hand.
2. **Millot & Baumann 2007,** arXiv 2401.01606. The model backbone.
3. **Ricot, Caussé & Misdariis 2005,** *JASA* 117, 2279–2290 (paywalled),
   read with **Misdariis et al., CFA 2000** (hal-01161356).
4. **Tarnopolsky, Fletcher & Lai 2000,** *JASA* 108, 400–406 (open at UNSW).
   Every coefficient of a validated lumped valve.
5. **Darabundit & Scavone 2025,** *Frontiers in Signal Processing*. The
   numerical recipe.
6. **Puranik & Scavone,** PoMA 49 (2022), DAFx-23, Forum Acusticum 2023. The
   bellows-driven attempt and the failure it documents.
7. **Hergert 2024,** *Acta Acustica* 8, 33 (doi 10.1051/aacus/2024020). Measured
   tremolo curves and the psychoacoustics of detuned unisons.
8. **Ramos et al. 2022,** *Computer Music Journal* 46(1–2), 40–57 (paywalled).
   Pressure-synchronised measurement of a bellows instrument.
9. **Cottingham, ICA 2019** (Aachen, pp. 5530–5535) and the **CCRMA 2013
   slides.** Transients, choking, accordion pressure and chamber data.
10. **Richter, *Akkordeon. Handbuch für Musiker und Instrumentenbauer*,** 4th
    ed., Noetzel 2008 (a book, not open). The IfM Zwota measurements.

## Prior art and what it permits

* **Patents.** US 6,946,594 (Roland) has expired and describes behaviour, not
  an implementation we would follow. EP1752966 (a valve servoed from bellows
  pressure) was due to expire on 2026-08-08; its status has not been checked.
  None claims a simulated reed, which is what this project builds.
* **Code.** Nothing found is worth reading for the engine, and nothing will be
  copied. Two tools are useful to *validate* output: dan-vine/accordion-tuner
  (MIT) detects several reeds per note and their beat rate.
* **Commercial libraries** (Le Parisien, Sonokinetic, PSound, Accordions 2) are
  listening comparisons at most. Sonokinetic's licence forbids building sample
  libraries and reverse engineering.

## Decisions the research leaves to the player

1. **The reference instrument. Decided 2026-09-30: there is none.** No
   accordion is available to record, so the model is judged by its physics
   and by the player's ear, not by a scorecard against a recording. What that
   commits the project to:
   * every constant in `MODEL.md` carries one of three statuses — *measured*
     (with its source above), *derived* (from measured ones by stated
     physics), or *voiced by ear* (with the date and what was heard) — and a
     voiced value is never later cited as if it were measured;
   * the published numbers above become tests on *emergent* behaviour, not
     targets to fit: the pitch shift against pressure, onset/offset
     hysteresis, attack times, the dynamic range, the tremolo beat, each
     asserted inside its published range, so a voicing by ear that pushes the
     physics outside what real reeds do is caught rather than shipped;
   * the laboratory — every constant live while playing, as in the Concert
     Grand — is the main instrument of calibration, and it comes before
     breadth;
   * recordings (TinySOL, Accordion4Composers) may be listened to beside the
     model to refresh the ear, and are never fitted against.
2. **The bellows control. Decided 2026-09-30: velocity, overridden by
   CC 11.** Without a bellows controller the key velocity sets the bellows
   pressure for the phrase; once CC 11 arrives (an expression pedal, or a
   digital accordion's bellows, which is what Roland hardware sends) it
   drives the pressure instead, accepting 14-bit CC 11 + CC 43. **The
   direction, decided 2026-09-30:** the Bellows Direction parameter, or CC 80
   as a switch (ROADMAP, milestone 3), since no standard carries it.
3. **Scope of the first instrument. Decided 2026-09-30: the treble side
   first**, a piano keyboard with L, M, M+, M− and H ranks; Stradella is a
   later stage. Whether a cassotto is part of it is still open.
   The project is named RF-Musette and is GPL-licensed, like RF-Organ and
   RF-7.
4. **Numerics.** The survey points at an energy-consistent, non-iterative
   scheme with modest oversampling; the prediction it must meet — stable for
   every parameter set a program or knob can reach, at a cost the Pi 4 can
   pay for a full register chord on both hands — is to be measured on the
   first reed, before the rest of the instrument is built on it.
