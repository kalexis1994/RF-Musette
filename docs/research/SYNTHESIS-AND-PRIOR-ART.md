# RF-Musette: free-reed synthesis, open projects and prior art

Research notes, 2026-09-30. Scope: sound synthesis and modelling of free reeds and accordions, open implementations, commercial prior art, patents, MIDI bellows conventions. The physics itself belongs to the other researchers and appears here only where a synthesis paper depends on it.

**Legend.**
- **VERIFIED**: I read the primary source myself: the paper text or abstract, the repo code or README, a manufacturer PDF, or a patent page.
- **PARTLY**: some claims are verified and others come only from a search snippet or a secondary page. The note says which.
- **UNVERIFIED**: search snippet or secondary source only. Do not build on it without checking.

**Method.**
- WebSearch and WebFetch.
- GitHub through `gh search repos/code` and `gh api` (READMEs and source read in place, nothing cloned).
- HAL through its public API (the hal.science pages are behind a bot wall).
- PubMed through E-utilities.
- PDFs were read with `pdf_tool.py` on the copies WebFetch saved. The harness saves those copies automatically in this session's `tool-results` folder. I did not deliberately download anything else, and nothing went into the pdf-cache.

---

## 1. Academic physical models and synthesis of free reeds

### 1.1 IRCAM: Caussé, Misdariis, Ricot (accordion reed)

- **R. Caussé, N. Misdariis, D. Ricot, "Studies of accordion reed vibrations — Applications in sound synthesis", JASA 106(4) Suppl. 2287 (1999), ASA meeting abstract; HAL hal-01105526.** VERIFIED (abstract through the HAL API).
  - Measurements on accordion reeds at several blowing pressures:
    - near-field pressure above and below the reed;
    - capacitive and laser-vibrometer displacement and velocity;
    - stroboscopy;
    - flow visualisation.
  - The acoustic model is derived from the flow-rate variation through the reed-plate slot.
  - The stated goal is a physical model "as simple as possible to be computationally efficient".
  - The abstract says the accordion model *can be programmed in Modalys* (IRCAM's modal synthesis language).
  - Only an abstract. No numerical scheme or cost figures.
- **N. Misdariis, D. Ricot, R. Caussé, "Modélisation physique de la vibration d'une anche d'accordéon", CFA 2000; HAL hal-01161356.** Abstract VERIFIED (HAL API). The full text was blocked by HAL's Anubis wall.
  - A nonlinear self-oscillation model of the accordion reed, built for sound synthesis.
  - **Two self-oscillation regimes**, each with a different law for frequency versus supply pressure.
  - The excitation force comes from **inertia of the fluid upstream of the reed**, not from jet turbulence.
  - The authors claim "very realistic" synthesised accordion-reed sounds.
- **D. Ricot, R. Caussé, N. Misdariis, "Aerodynamic excitation and sound production of blown-closed free reeds without acoustic coupling: the example of the accordion reed", JASA 117(4):2279–2290 (2005); HAL hal-01106161 (also hal-01105812, dated 2002).** Abstract VERIFIED.
  - The accordion reed is a **blown-closed** free reed that self-oscillates **without acoustic coupling**.
  - Visualisations in water support incompressible, potential flow.
  - The model is solved in the time domain.
- **Relevance.**
  - Primary physical justification for an accordion model where no bore or resonator is needed to sustain oscillation. The upstream-inertia mechanism is what makes a lumped model sufficient.
  - No code and no real-time figures were published. Modalys is proprietary.

### 1.2 Millot and Baumann: the "minimal model" line (harmonica, generic free reed)

- **L. Millot, C. Baumann, "A proposal for a minimal model of free reeds", Acta Acustica united with Acustica 93:122–144 (2007).** Author's copy on arXiv 2401.01606 (Jan 2024, arXiv non-exclusive licence); HAL hal-04369449. VERIFIED: I read the model outline and the numerical appendix pp. 37–38.
  - **State.**
    - 1-DOF reed: tip displacement on the first cantilever mode. The motion is taken as sinusoidal, following Misdariis/Ricot laser-vibrometer data.
    - Upstream volume V1 and an upstream pipe (L1/L2, S2).
    - Jet flow through a "useful section" that accounts for side and front escape areas.
    - A pumped flow from reed motion.
  - Blown-closed (-,+) and blown-open (+,-) reeds are both handled.
  - A linear stability analysis shows that an upstream volume plus pipe is needed to destabilise both kinds of reed.
  - Results:
    - playing frequency depends strongly on the upstream volume;
    - it is largely independent of excitation level;
    - it depends strongly on reed thickness.
  - **Numerical scheme (appendix 5.4, following Gazengel et al.).**
    - Backward Euler for everything except the reed.
    - The reed uses a bilinear transform with frequency pre-warping.
    - The main nonlinear equation (mass conservation in V1) is solved each sample by **Newton–Raphson on p2[n]**. When NR oscillates, it falls back to **bisection**.
    - Useful-section tables and dSu/dp2 are **precomputed**.
    - No sample rate, cost or real-time claim appears in the paper. Its demos are 3000-sample bursts.
  - The paper says the model had **not been confronted with experiments** for these configurations.
- **Harmonica series by Millot** (HAL abstracts VERIFIED):
  - PhD thesis on valve instabilities applied to the diatonic harmonica, 1999 (tel-04370152);
  - V. Debut & L. Millot, "Time domain simulation of the diatonic harmonica", MOSART workshop 2001;
  - "Free reed instruments: clues for a minimal model", SMAC 2003;
  - "Very-low frequency range influence…", Acoustics'08;
  - "Source-resonator modeling: a rough paradox", Acoustics'08;
  - "Chromatical playing on diatonic harmonica: from physical modeling to sound synthesis", ASA 162nd meeting, 2011.
- **Findings from that series that matter for synthesis:**
  - Energy in the 0–50 Hz band is important. The model uses **flow descriptions only, with no waves**.
  - For a short pipe loaded by a free reed, a reflection-function (waveguide) model does **not** give the same result as an unsteady incompressible Bernoulli model. Millot argues the latter is the valid one.
  - The escape area he derives **has no closure discontinuity**.
- **Relevance.** This is the reference lumped model for any free-reed synthesiser. Its known weakness is the per-sample implicit NR solve with an ad hoc bisection fallback. No stability guarantee comes with it. See 1.3 for what happened when others used it live.

### 1.3 McGill CAML: Puranik and Scavone (Indian hand harmonium)

- **N. Puranik, G. Scavone, "Physical modelling synthesis of a harmonium", Proc. Meetings on Acoustics 49, 035015 (Vienna Talk 2022).** PARTLY. The page returned 403; content is taken from the authors' own summary in the DAFx-23 paper.
  - Adapts Millot–Baumann: an added **bellows chamber p0** feeds V1, so bellows pressure becomes the playing control.
  - The reed is a lumped mass-spring-damper (ω0, Q).
  - Source-filter structure: the wooden enclosure is an all-pole filter estimated by IAIF (glottal inverse filtering) from a recorded note.
- **N. Puranik, G. Scavone, "Physically inspired signal model for harmonium sound synthesis", DAFx-23, Copenhagen, pp. 379–382 (CC BY 4.0 paper).** VERIFIED (full text read).
  - **Key negative result for us.** The physical model's **numerical stability depended on non-playing parameters** (reed-chamber volume, reed clearance). To change the playing frequency, those parameters had to be retuned by **manual trial and error**, which "is not possible in a live-performance context". The authors then replaced the physical model with a signal model.
  - The signal model:
    - a band-limited **wavetable** from the first 37 LTAS peaks of the simulated reed flow;
    - the enclosure filter reduced to a cascade of **10 biquads**, with a peaks-and-valleys parametrisation (9 peaks, 9 valleys);
    - bellows pressure drives the amplitude and adds a small f0 offset. The measured shift is under 1–2 Hz; **f0 rises as pressure drops**.
  - Implemented as a **Pure Data** patch. The patch is not published: the CAML project page lists no code.
  - They note that the IAIF-estimated reed flow shows **a discontinuity each period when the reed passes through the plate**.
- **N. Puranik, G. Scavone, "Clamped bar model for free reeds", Forum Acusticum 2023, Turin (CC BY 3.0 paper).** VERIFIED.
  - Laser Doppler vibrometer on a B5 (990 Hz) harmonium reed. Reed motion is nearly sinusoidal at about 0.2 kPa. **Harmonics grow at 1.4–1.6 kPa.** Harmonium reed-chamber pressures run **0.2–1.5 kPa**.
  - Model: Euler–Bernoulli clamped-free bar (4th-order PDE), **N = 40 grid sections**.
    - The implicit scheme of Chaigne–Doutaut, as used by Avanzini and van Walstijn, for the bar.
    - Backward differences for the flow equations.
    - The Millot Newton–Raphson for the flow.
    - **Sample rate 4 × 44.1 kHz.**
  - Their reading: the reed acts partly as a filter of the harmonic driving pressure p2.
  - The authors say torsional modes and thickness non-uniformity are missing.
- **T. J. West, N. Puranik, M. M. Wanderley, G. Scavone, "Towards the Continuous Harmonium: Replicating the Continuous Keyboard", NIME 2025 (CC BY 4.0).** VERIFIED (first pages).
  - Optical continuous-key sensing (McPherson design), each sensor read at 1 kHz, for pitch-glide ornaments.
  - An interface paper, with no synthesis content.
- **Relevance.**
  - The most recent and most directly comparable work: a bellows-driven, keyboard free-reed instrument with a synthesis goal.
  - It shows a clean trajectory: lumped physical model, then instability, then a wavetable plus biquad signal model.
  - That is the failure mode RF-Musette has to beat.

### 1.4 Harmonica bending and overblow: Adachi et al. (2026)

- **S. Adachi, H. Okada, T. Samejima, K. Nishimiya, "Time-domain simulation of harmonica pitch bending and overblowing", JASA 159(5):4191–4203 (May 2026), doi 10.1121/10.0043784.** Abstract VERIFIED via PubMed; the full text returned 403.
  - Couples reed vibration, flow through the reed openings, and a **mouth-side acoustic resonator** (cylindrical tubes of varying diameter and length stand in for the vocal tract).
  - Reproduces blow bends (hole 7), draw bends (hole 4) and overblow (hole 6).
  - A small-amplitude analysis gives self-excitation conditions that agree with experiment and simulation.
  - Pitch bends change **continuously** with tube length. **Overblow switches abruptly after a silent interval.**
  - No real-time or cost figures in the abstract.
- **Relevance.** Low for the accordion itself, which has no player vocal tract. Moderate as proof that the lumped-reed plus flow plus resonator framework predicts regime changes, and for any future harmonica voice.

### 1.5 Bandoneon 2.0 (Argentina): measured-parameter synthesis

- **J. M. Ramos, E. R. Calcagno, P. E. Riera, "An embedded wavetable synthesizer for the electronic bandoneon with parameter mappings based on acoustical measurements", NIME 2023 (CC BY 4.0).** VERIFIED (full text read).
  - **Synthesis.**
    - Recorded reed-by-reed with synchronous bellows pressure.
    - About 50 cycles extracted per reed, reduced to **5 wavetables per reed**, each labelled with pressure, RMS and spectral centroid.
    - At run time the wavetables are interpolated linearly by current pressure.
    - Separate tables for the fundamental and the octave reed.
    - Pressure-dependent **detuning map**, plus filtered wind noise and key-click samples.
    - Attack time ∝ (−n + c)(Pmax − p).
    - Residual-oscillation reuse for fast repeats (trills).
  - **Measured behaviour.** With rising pressure, **low notes go flat, by almost half a semitone at extreme pressure, and the highest notes go sharp.**
  - **Platform and cost.**
    - Bela Mini (1 GHz Cortex-A8).
    - **10–12 voices at block 256 with CPU peaking at 90 %.** Each voice is both reeds plus noise plus key sound.
    - About 22 MB memory.
    - 6 voices is the stated minimum, 8–10 is safer.
  - **Controller "Alfa".**
    - Measures bellows **differential pressure, signed** (positive when compressing, negative when expanding).
    - Sends it as MIDI CC. They mention 14-bit CC was not feasible on their Arduino, so they apply **cube-root companding to a 7-bit CC** and cube again on the synth side. The exact CC number was not stated in the parts I read.
  - The earlier Faust model is described in the CMJ 2022 paper (next item).
  - Code: I found no public repository. The paper points to bandoneon.ar.
- **Same authors, "An Electronic Bandoneon with a Dynamic Sound Synthesis System Based on Measured Acoustic Parameters", Computer Music Journal 46(1–2):40 (2022).** PARTLY. The full text returned 403; the facts come from the abstract snippet and the NIME paper.
  - Synthesis model written in **Faust**, driven by acoustic measurements of sound and air pressure.
- **Relevance. High.**
  - The only published *measured* pressure-to-(level, brightness, pitch) maps for a bisonoric bellows free reed.
  - It gives a sign convention (signed differential pressure) and a real embedded cost baseline, on hardware weaker than a Pi 4.
  - It shows the signal-model ceiling that a physical model has to justify itself against.

### 1.6 Sheng

- **Kuang Wei, Ji Peifeng, Philip Leistner, Judit Angster, Yang Jun, "A physical model and sound synthesis method of Chinese Sheng", Acta Acustica (Chinese, 声学学报) 41(5):628–637 (2016), doi 10.15949/j.cnki.0371-0025.2016.05.011.** Abstract VERIFIED.
  - A reed-tongue vibration model coupled to an equivalent circuit of the pipe: transfer matrices for the tone hole and the tuning slot.
  - Validated against multi-channel recordings.
  - No cost or code information.
- **Relevance.** Low. Pipe-coupled free reeds are a different regime from the accordion's uncoupled reeds.

### 1.7 Generalised parametric reed: Smyth and Abel (ICMC 2005)

- **T. Smyth, J. Abel, "A generalized parametric reed model for virtual musical instruments", ICMC 2005.** PARTLY. The authors and abstract come from search snippets; quod.lib returned 403.
  - One configurable pressure-controlled valve model covering blown-closed, blown-open (harmonica, harmonium, larynx) and symmetric "swinging door" reeds.
- **Relevance.** Conceptual: one reed class parametrised across reed types. Details not verified.

### 1.8 Euphonics (online book): harmonica model with sound examples

- **euphonics.org §11.6 "Free reeds" and §11.6.3 "Modelling the harmonica".** VERIFIED (pages read). Licence **CC BY-NC-SA 4.0**. The author is not named on the fetched pages; the site is generally attributed to J. Woodhouse, which is UNVERIFIED here.
  - Explains Fletcher's linearised model and Millot's model. Millot's adds a tube, and the Helmholtz frequency decides which reed (opening or closing) speaks.
  - Harmonica model:
    - two 1-DOF reeds with Q = 95;
    - mouth cavity V plus tube (L = 28 mm, S = 20 mm²), Helmholtz Q = 4;
    - Bernoulli gap flow with vena contracta 0.61;
    - reeds at 440 and 392 Hz, gap 0.2 mm.
  - Synthesised sound examples: cavity-volume sweep and a bend.
- **Relevance.** A clear pedagogical parameter set. The NC licence means we reference it and never copy it.

### 1.9 Other free-reed physics touched by synthesis papers (brief)

- **Q. Foti, M. Sabin, J. P. Cottingham, "Experimental tests of free reed instrument simulations", JASA 153(3) Suppl. A40 (2023).** PARTLY (snippet).
  - FEM (fluid + acoustics + structure) simulations of harmonica and accordion reeds.
  - Confirms the frequency-versus-blowing-pressure dependence near playing pressures.
- **Cottingham, reed-chamber resonances and attack transients in free reeds, ICA 2016 (ICA2016-0748). "Aeroacoustics of free reeds", ICA2016-0756.** UNVERIFIED. Listed in search results; I did not read them.
  - The snippet says chamber resonances are high and small in effect on reed vibration, but can colour tone.
- **G. Volpatti, "A matrix framework for detuned-unison accordion musette tuning…", Am. J. Math. & Physics (2026), HAL hal-05715038.** Abstract VERIFIED.
  - Closed-form beat, pitch-centre and roughness descriptors for 2- and 3-reed musette.
  - Example tuning: −5 / 0 / +15 cents.
  - Useful for register design and tests. Theoretical only.
- **G. Volpatti, "Beyond the Bellows: a critical review of free reed instrument research", Am. J. Arts & Human Science (2025), HAL hal-05083707.** Abstract VERIFIED.
  - A review of more than 100 studies. The field is dominated by experiment and FEM, with little computational or synthesis work, and Asian free reeds are under-studied.
  - This independently supports the gap described in (b).
- I searched for **Aalto (Välimäki, Karjalainen), KTH and Padova** free-reed synthesis and found none. Their reed work is on single and double reeds and flutes. Padova's relevance is numerical (next section).

### 1.10 Numerical schemes transferable to reed–flow nonlinearities

None of the items below targets free reeds. Each addresses the exact failure Puranik reported.

- **F. Avanzini, D. Rocchesso, "Efficiency, accuracy, and stability issues in discrete-time simulations of single reed wind instruments", JASA 111(5):2293–2301 (2002).** VERIFIED (abstract).
  - Discretising the reed–flow system creates a **delay-free loop**. It is resolved by a geometric transformation of the equations (the K-method family).
  - Four discretisations are compared for stability and parameter mapping.
- **Avanzini and van Walstijn, distributed clarinet reed (Acta Acustica 2004) with the Chaigne–Doutaut implicit bar scheme.** PARTLY (cited by Puranik 2023; the QUB PDF was listed but not read). This is the bar scheme Puranik reused.
- **S. Bilbao, J. Bensa, R. Kronland-Martinet, "The wave digital reed: a passive formulation", DAFx-03, London, pp. 225–230.** PARTLY (abstract through snippets).
  - A circuit and WDF formulation of the lumped reed nonlinearity, with simple stability verification.
  - Handles the nonlinear algebraic solve and removes delay-free loops.
- **S. Bilbao, "Direct Simulation of Reed Wind Instruments", Computer Music Journal 33(4):43–55 (2009); also the book *Numerical Sound Synthesis*, Wiley 2009.** PARTLY (bibliographic only). Energy-based FDTD for reed-bore systems.
- **C. C. Darabundit, G. Scavone, "Discrete port-Hamiltonian system model of a single-reed woodwind instrument", Frontiers in Signal Processing (2025), doi 10.3389/frsip.2025.1519450.** VERIFIED (article read).
  - Lumped reed with Hunt–Crossley contact.
  - Proves the **Bernoulli-flow dissipation is non-negative**, so the port stays passive.
  - **Linearly implicit energy-quadratisation (SAV/IEQ) scheme: no iteration, a quadratic solved directly.** Symplectic bore.
  - 48 kHz. **Real-time factor about 1/16 in Python on Apple M3** (about 1/5 without viscothermal losses).
  - No code release mentioned.
  - **The single most transferable numerical recipe for RF-Musette's reed–flow core.**
- **T. Hélie (IRCAM), port-Hamiltonian guaranteed-passive simulation**, NESS workshop slides and related work. PARTLY. Also **Risse et al., a PHS vocal synthesiser (flow plus vocal folds)** in the same Frontiers research topic. The vocal folds are a *blown-open valve*, the valve class closest to a free reed that already has an energy-consistent treatment.
- **Aliasing.**
  - **M. Holters, "Antiderivative antialiasing for stateful systems", DAFx 2019.** PARTLY (URL and snippet).
  - An ICMC 1999 paper, "Antialiasing for nonlinearities: acoustic modeling and synthesis applications" (authors not verified), names the saturator and rectifier nonlinearities in single-reed models as major alias sources.
  - For free reeds the relevant nonlinearity is the **aperture or useful-section function**. Its sharp change as the reed passes the plate produces the per-period flow discontinuity Puranik observed, which is broadband.
  - The only oversampling figure in the free-reed literature is Puranik's **4 × 44.1 kHz** (bar model).
- **J. Kergomard, P. Guillemain, T. Voinier (CNRS), US 7,534,953 B2, "Method for simulation and digital synthesis of an oscillating phenomenon".** VERIFIED (Google Patents).
  - Impedance-domain (not travelling-wave) real-time reed plus cylindrical-bore synthesis, **clarinet type, not free reed**.
  - Status: **expired**.

---

## 2. Open-source implementations

### 2.1 Major DSP libraries: none contains a free-reed model

| Library | Licence | Free reed? | Evidence | Status |
|---|---|---|---|---|
| STK (thestk/stk) | STK licence (GitHub reports NOASSERTION; permissive MIT-style text) | **No.** The only "harmonica" is the `BandedWG` preset "Glass Harmonica", a banded-waveguide bar and bowl model. Reed classes (`Clarinet`, `Saxofony`, `BlowHole`, `ReedTable`) are beating single reeds. | include/ list and BandedWG.h read | VERIFIED |
| Faust `physmodels.lib` | LGPL-2.1+ with the GRAME exception (compiled output free to relicense) | **No.** Only `reedTable`, `clarinetReed` and `clarinetModel` (single-reed lookup tables). | source grep | VERIFIED |
| Csound | LGPL-2.1 | No free-reed opcode found. Hofmann et al.'s FDTD `resontube` opcode (ICSC 2019 / NIME 2019) is a closed-open tube for clarinet or sax. | code search + snippet | PARTLY |
| SuperCollider core and sc3-plugins | GPL-3.0 | None found (code search for accordion, harmonica, harmonium, free reed, concertina, bandoneon). | code search | VERIFIED (negative search) |
| Pure Data | BSD-3 | None in core. Puranik's harmonium patch is **unpublished**. | code search + CAML page | VERIFIED (negative) |
| IRCAM Modalys | proprietary (IRCAM Forum) | The `reed` connection is documented for single-reed mouthpieces. The 1999 IRCAM abstract says an accordion model *can be programmed* in Modalys. No shipped accordion model found. | doc page + HAL abstract | PARTLY |
| Mutable Instruments, VCV Rack | various | No free-reed module found. | plugin.json code search | UNVERIFIED (negative search only, not exhaustive) |

### 2.2 GitHub free-reed synthesis code (none has a licence that allows reuse and a physical model)

- **rohitbams/Harmonium-Physical-Model**: C++/JUCE VST3, created 2025-11. **No licence file, so all rights are reserved.** VERIFIED (README read).
  - Implements the Puranik–Scavone equations: bellows p0, reed chamber p1, jet p2, 1-DOF reed, total flow.
  - **Forward Euler at the audio rate. "Clamping applied to state variables to prevent divergence."**
  - An *adaptive Q* "artificially extends decay time when air pressure is present".
  - Spectral shaping on top.
  - Bellows is pumped with **CC1** (mod wheel) into an air-mass accumulator. 6 voices.
  - Relevance: a concrete open example of the naive route: explicit integration plus clamps plus a fake Q. It confirms that stability is the practical wall. A repo `rohitbams/harmonium-v2` is empty.
- **libraz/libsonare `src/midi/synth/free_reed_voice.{h,cpp}`**: **Apache-2.0**. VERIFIED (header read).
  - An explicitly **non-physical, feed-forward** "free reed core": phase accumulator, then an asymmetric soft-clipped saw *or* a "slot duty" two-hump flow shaper that places a null near the 7th partial, then a body low-pass.
  - Two detuned tongues for musette.
  - Radiation control: 0 = flow, 1 = d/dt flow (monopole tilt); "harmonium and bandoneon measure at 1, harmonica at 0".
  - Bellows is **CC2 (breath)**, and brightness is CC74.
  - "Unconditionally stable" because there is no feedback.
  - Relevance: a good fast baseline and A/B reference, and it is permissively licensed. We still would not copy it.
- **MegaLoler/Accordion-Synth**: C++, RtAudio/RtMidi, **no licence**. VERIFIED (source read).
  - A *kinematic* reed: the tip moves as a prescribed sinusoid.
  - The aperture area is computed from a static cantilever deflection shape, integrating the side escape. Output is the high-passed area.
  - Three detuned oscillators per note. Pressure sets amplitude only.
  - A cheap "useful-section" idea, with no dynamics.
- **Toys** (VERIFIED, trivial): `nafeu/dronmakr` `accordion_reed.dsp` (GPL-3.0, square + triangle detune); `mogenson/organelle-orac-modules` "Reeds" Faust patch (organ-style additive plus tremolo); `john-holland/system-drawer` `FreeReedInstrumentSim.cs` (MIT, Unity game-audio parameters). No modelling content.
- **Freeware or commercial closed plugins** (not open source):
  - **Harmoniac** (Mokafix): Win32 VST(i) "physical model reed" for harmonica; its VST mode drives an envelope follower from audio input. VERIFIED (listing).
  - **Harmodion** (Syntheway): **additive** drawbar, reed-organ, harmonium, accordion and bandoneon, key-click simulation. VERIFIED (KVR description).
- No open **accordion** physical model exists on GitHub: searches for "accordion physical model", "free reed model/synthesis", "harmonica physical model" and "reed organ physical model" returned nothing relevant.

### 2.3 Open MIDI-accordion and bellows hardware: what they measure and send

| Project | Licence | Bellows sensor | What it sends | Direction | Status |
|---|---|---|---|---|---|
| **bandolibre/bandolibre** (142-key Rheinische bandoneon, STM32G474) | **CC BY-NC-SA 4.0** (LICENSE.md) | Blade spring + **two Hall sensors**, sampled up to **4 kHz**, 1€ filter, deadzone and hysteresis classifier, cubic-Bézier response curve | **CC#11 as 14-bit (CC#43 LSB)** on both hand channels, rate-limited by `bellow_cc_period_ms`; table mode pins CC11 | **Proprietary SysEx** "bellows direction" push notification (0 = pull, 1 = push, 2 = neutral), sent *before* the note-ons it explains. Intensity is **unsigned** 0..1. | VERIFIED (bellow.h/.c, midi.cc) |
| **bvavra/MIDI_Accordion** (retrofit acoustic piano accordion) | **none** | BMP180 barometer inside the bellows, calibrated at start | **CC#11**. Expression = cubic of **|ΔP|**. The comments give a working range of about **0–250 Pa**, with a 15 Pa threshold. Sent at −6 and −12 offsets to channels 2 and 3. | Discarded (absolute value) | VERIFIED (code) |
| **accordion-mega/AccordionMega** (the base of the above) | GPL-3.0 | BMP085 | (not read) | – | PARTLY |
| **schult/anglo-midi** (30-button Anglo concertina) | **none** | **1 kg load cell + HX711 at 80 Hz** | **CC#7** (channel volume) = sqrt(\|force\|)/9, on channels 1 and 2 | Sign of the reading selects the push or pull note table (bisonoric) | VERIFIED (code) |
| **Rowlhouse/bandonino** (Teensy 4.1 bandoneon) | MIT | 3 kg load cell | L/R on separate channels (CC not read) | – | PARTLY |
| **PierreBanwarth/MIDI-Accordina** (Teensy 4.1) | MIT | not read | code has `sens_soufflet` (bellows direction) passed to note selection | direction used internally | PARTLY |
| **bixvongoeler/chordian** (student project) | none | **Time-of-flight distance** sensor on a scissor bellows, adaptive filtering | expression via a Max patch | – | VERIFIED (README) |
| Bandoneon 2.0 "Alfa" (paper, §1.5) | n/a | Differential **pressure**, signed | 7-bit CC with **cube-root companding** | Sign of pressure | VERIFIED (paper) |
| Musictech Q-Select / Music Maker (Castelfidardo, commercial retrofit) | proprietary | Bellows **air-pressure** sensor, zeroed at power-on **with the bellows closed** | "Dynamic bellows expression" (CC not stated in the manual pages read) plus an option for **aftertouch controlled by the bellows sensor**; bank CC0/CC32 + PC per section | not stated | VERIFIED (manual pp. 6, 11–12, 16) |

- **Tuners** (data tools, not synthesis):
  - **dan-vine/accordion-tuner** (MIT, Python): 1–4 simultaneous reeds; FFT phase-vocoder, zero-padded SimpleFFT for reeds under 1 Hz apart, and ESPRIT; beat-frequency readout; CSV/TSV tremolo profiles.
  - **smegg99/SmeggTuner** (MIT).
  - **billthefarmer/tuner** (Android; no licence detected by the API).
  - Relevance: validation tools. Render RF-Musette output and measure the musette beat rates and reed spread against the target tuning.

---

## 3. Commercial prior art and how control is exposed

### 3.1 Roland V-Accordion (FR series): the reference product

- **Engine claim.** "PBM (Physical Behavior Modeling) … complex algorithms". "The pitch of each simulated reed is varied individually"; tone is shaped by a DSP. VERIFIED (rolandus.com technology page). Marketing; no algorithm is disclosed.
  - The patent (3.3) shows the underlying method is **per-reed samples modulated by bellows pressure**, not a physical model.
- **FR-8x MIDI Implementation v1.00 (2013-06-25, static.roland.com).** VERIFIED (PDF read).
  - Transmits and receives **CC#11 Expression, 7-bit, 0–127**, gated by "Expression" filters in Global and per-part MIDI TX.
  - Also transmits CC1, CC7, CC10, CC64, CC91, CC93, Bank Select CC0/CC32, Program Change, Pitch Bend, and **Channel Pressure** (gated by "Aftertouch TX").
  - **No bellows-direction message and no SysEx for bellows** appear in the implementation.
  - **Registers by Bank Select + PC.** Treble registers 1–14 are PC 0–13 (bank 0/0); Orchestra 1–28; Bass, Free Bass, Orch Bass and Chord 1–7; Organ is bank 0/1; Sets 1–100 on the basic channel (default ch 14). A PC takes effect from the next note-on.
  - **Default channels:** Accordion 1, Bass/Free Bass 2, Chord 3, Orch1/Organ 4, Orch Bass 5, …
- **FR-4x MIDI Implementation v1.00 (2016-11-01).** VERIFIED. Same CC#11 Expression scheme and the same register-via-PC structure. No direction message.
- **FR-8X owner's manual (the later model with wireless LAN), p. 114 via manua.ls.** PARTLY: a third-party render of the Roland manual. Say so if quoted.
  - "Bellows TX" with values Super / High / Normal / **Low (default)**. The bellows movement is **transmitted as expression data**. "Super" gives the most natural volume change but the most MIDI traffic. Use Normal or Low if a receiver chokes.
  - "Expression MIDI TX" is per part, default On.
- **FR-4x/FR-4xb owner's manual.** VERIFIED (PDF read).
  - Mechanical **bellows resistance regulator** plus a **"bellows curve"**: FIX.L / FIX.M / FIX.H (fixed volume, no bellows needed), X-LT, LT, STD, HEVY, X-HV.
  - "Bellows shake" is named as a supported technique.
- **FR-8x "Dynamic Bellows Behavior"** mimics the varying resistance of air through the bellows and can be disabled. PARTLY (retailer and Roland snippets); mechanism in patents EP1752966 / EP2779154 below.
- **Resolution.** The service notes describe a pressure-sensor calibration with the bellows fully closed and air purged (manualslib snippet, PARTLY). No bit depth is published that I could find. Externally the resolution is **7-bit CC11** with a selectable transmit density.

### 3.2 Other makers

- **Ketron** (P37 piano, B46 button). VERIFIED (ketron.it).
  - "Natural Air Flow System" plus "Dynamic Bellows Behavior".
  - Bellows curves Standard / Light / Heavy / Fixed Low / Med / High, the same menu set as Roland's.
  - MIDI IN/OUT and USB MIDI. **No CC mapping published** on the page.
- **Cavagnolo** Essentiel MIDI kit / Digit AiR. PARTLY.
  - Page VERIFIED: "Bellows expression and velocity control are both available, but the choice is yours."
  - The claim that it uses **"MIDI 11 controller"** comes only from a search snippet (UNVERIFIED).
- **Musictech** (Q-Select, Music Maker). See the §2.3 table. Pressure sensor, zeroed closed, with expression or aftertouch from the bellows. VERIFIED.
- **Dexibell, Hohner**: no digital-accordion product with published bellows or MIDI documentation found. UNVERIFIED (negative).
- **Pianoteq (Modartt)**: **no accordion, harmonium or free reed** in the instrument-pack list. VERIFIED (modartt.com overview). Modartt forum threads show users requesting it. Organteq is pipes.
- **Audio Modeling SWAM**: **no free reed** in the product line (strings, woodwinds including double reeds, brass). VERIFIED (negative, product pages and search).
- **Sample libraries.**
  - **Accordions 2** is by **Best Service / Engine Audio, produced by Eduardo Tarilonte**, not Sonokinetic. VERIFIED (bestservice.com).
    - "Dynamic bellows behavior with smooth crossfade across 3 layers; controlled via Modwheel or velocity".
    - 3 round-robins, "Bellow Shake" in all instruments, bandoneon "Sforzato option (with threshold control)".
    - Engine Player. No push/pull articulation mentioned.
  - **Sonokinetic "Accordion"** (Kontakt): chord-analysing sequencer, no modelling claims, no bellows CC documented on the page. VERIFIED.

### 3.3 Patents (technical content only; legal review is out of scope)

- **US 6,946,594 B2, "Method for reproducing the sound of an accordion electronically"** (Roland Europe → Roland Corp.; inventors L. Bruti, D. Cuccu, R. Gaetani; priority IT 2001-04-27; granted 2005-09-20). Google Patents says **"Expired – Fee Related"**. EP family **EP1258861B1**, listed as **Expired – Lifetime**. VERIFIED (Google Patents).
  - **Claimed method:**
    - store **samples of individual reeds**, several per key per register;
    - sense bellows pressure (A), keys (T) and registers (R);
    - generate each reed's sound independently, with amplitude, envelope and **pitch modulated per reed by bellows pressure**;
    - **per-reed speaking thresholds Pon/Poff** with exponential ramps;
    - above a pressure Ps, pitch drops by a per-reed amount (the musette behaviour);
    - claim 25: separate open-valve and **closing-valve samples** played on key release.
  - Relevance: documents the *behaviours* Roland considered essential: per-reed thresholds, pressure-dependent per-reed pitch, and valve-closing sounds. A physical model produces these as emergent behaviour rather than from samples.
- **EP1752966B1, "Electronic accordion"** (Roland Corp.; inventors Miki Jun'ichi, Hiraiwa Yoshifumi; priority 2005-08-08, granted 2008-11-19). VERIFIED (EPO text).
  - An **air valve whose opening is servoed from the bellows-pressure sensor**, with separate settings for compression (above atmospheric) and expansion (below), and a factor α for reeds per note.
  - It makes the bellows *feel* depend on how many reeds are sounding.
  - Google lists the "anticipated expiration" as **2026-08-08**; current status not independently checked.
- **EP2779154A1, electrically controlled bellows air-stream valve** (Roland; Bruti, Cuccu, Ferrari, Verdecchia; priority 2013-03-11). **Withdrawn**. VERIFIED.
  - Air passage opens with the **number of pressed keys and active registers**. A pressure tube feeds the sensor.
- **Physical fact embodied in these patents:** air consumption, and therefore the bellows load, scales with the number of open reeds. **No open synthesis model couples this back into the audio model.** This is a gap (see (b)).
- **Citing and cited family members** (not examined): US8653350B2 (Casio, "Performance apparatus"), US9520112B1 / US9747875B2 (Ua-Aphithorn, electronic accordion), US4383463A (Farfisa, electro-pneumatic bellows control), US3918343A (accordion pickup).

---

## 4. MIDI conventions for bellows: what hardware actually sends

| Source | Bellows message | Resolution | Direction | Registers |
|---|---|---|---|---|
| Roland FR-8x / FR-4x (MIDI impl. PDFs) | **CC#11** Expression, per part/channel | 7-bit; transmit density via "Bellows TX" (FR-8X) | **Not transmitted** | **Bank Select CC0/32 + Program Change** (treble registers = PC 0–13) |
| Bandolibre (open) | **CC#11 + CC#43 (14-bit)** on both hand channels | 14-bit, sensor at up to 4 kHz, rate-limited | **SysEx** push/pull/neutral, sent before notes | n/a (layout buttons) |
| bvavra MIDI_Accordion (open) | **CC#11** | 7-bit, from \|ΔP\| (about 0–250 Pa) | Discarded | n/a |
| schult/anglo-midi (open) | **CC#7** | 7-bit, sqrt of load-cell force | Selects note tables | n/a |
| Bandoneon 2.0 Alfa (paper) | 7-bit CC, **cube-root companded**, signed pressure | 7-bit (14-bit tried, too slow on the MCU) | Pressure sign | n/a |
| Musictech Q-Select (manual) | "Dynamic bellows expression"; optional **aftertouch from the bellows** | not stated | not stated | CC0/CC32 + PC per section |
| Cavagnolo kit | CC11 (**UNVERIFIED** snippet) or velocity | – | – | – |
| libsonare (open synth, receiver side) | **CC2** breath = bellows force | – | – | – |
| Harmonium-Physical-Model (open synth) | **CC1** pumps an air reservoir | – | – | – |
| Sample libraries (Accordions 2) | **Mod wheel (CC1)** or velocity crossfades 3 layers | – | – | – |

- **No MIDI Association standard or MIDI 2.0 Profile** for accordion bellows was found. Profiles work lists orchestral articulation, MPE and piano. VERIFIED (negative, midi.org search result).
- **The de facto convention is CC#11 on each channel, unsigned magnitude, 7-bit** (Roland). Direction is absent or proprietary.

---

## (a) Technical landscape: approaches and trade-offs for a real-time Pi 4 engine (no_std Rust, wasm)

1. **Sample or wavetable per reed, modulated by pressure** (Roland patent, Bandoneon 2.0, Puranik's DAFx signal model, sample libraries).
   - Cheapest per voice. The measured bandoneon engine ran 10–12 full voices on a 1 GHz A8.
   - Needs recorded or measured data for every reed and register. Behaviours (thresholds, pitch versus pressure, detune) are hand-mapped.
   - Transients, reed interaction and bellows load are faked.
   - This is what "PBM" is (per Roland's own patent). It is not what a physically modelled sibling of the Concert Grand should be.
2. **Feed-forward "reed shaper"** (libsonare, MegaLoler kinematic aperture).
   - Unconditionally stable and very cheap.
   - The slot-geometry idea (flow humps, duty and return pass, monopole d/dt radiation) gets the partial ladder right.
   - No self-oscillation, so no emergent onset, pressure-dependent pitch, threshold or regime behaviour.
   - Good as a **reference and A/B baseline**, and as a fallback "eco" voice.
3. **Lumped self-oscillating model**: 1-DOF (or a few-mode) reed + nonlinear useful-section flow + upstream chamber / neck (Helmholtz) + bellows source. This is Millot–Baumann as adapted by Puranik and Scavone.
   - The **physically right minimum for accordion reeds**: IRCAM showed there is no acoustic coupling and the drive comes from upstream inertia.
   - State per reed is tiny: about 2 reed states + about 2–3 flow and pressure states.
   - **Problems:**
     - an implicit per-sample Newton solve (Millot) or an unstable explicit Euler (rohitbams);
     - **stability depends on non-playing parameters** (Puranik), so it cannot be retuned live per note;
     - the aperture nonlinearity has a per-period discontinuity, hence aliasing. Puranik used 4× oversampling for the bar model.
   - Cost per reed-step with an iterative solve: roughly tens to about 100 flops, plus 1–3 iterations with sqrt.
   - **Rough estimate, not measured:** 100 simultaneous reeds × 48 kHz × 2× oversampling is about 10 M reed-steps/s. At about 100 flops each that is about 1 GFLOP/s. That fits in a Pi 4's 4× A72 at 1.5 GHz with NEON only if it is SIMD-batched across reeds and parallelised across cores. RackForge's wasm-versus-native tax (memory note: about 66 % of native) makes a **non-iterative update mandatory**.
4. **Distributed reed** (clamped Euler–Bernoulli bar, N = 40, implicit, 4 × 44.1 kHz; Puranik FA 2023) or FEM.
   - Adds harmonics at high pressure and higher modes.
   - About 40× the reed state and an implicit solve per sample. **Not viable at accordion polyphony on a Pi 4.**
   - A **modal truncation to 2–3 cantilever modes** (plus the first torsional mode, per Cottingham's reports of higher and torsional modes) is the affordable middle ground. Modes are diagonal, so the cost is linear in modes.
5. **Energy-consistent discretisation** (port-Hamiltonian, SAV/IEQ, WDF).
   - Proven for **single reeds** (Darabundit & Scavone 2025: passive Bernoulli port, linearly implicit, no iteration) and vocal folds (Risse et al.). WDF version: Bilbao et al. 2003.
   - Gives stability **by construction, independent of reed and chamber parameters**. That removes exactly the failure that made McGill abandon the physical model.
   - Cost: a small linear solve per sample. For a lumped reed plus two chambers that is a 3×3 to 5×5 system, cheap and SIMD-friendly.
   - Their Python RTF of 1/16 is dominated by the bore, which a free reed does not need.
6. **Waveguides.** Of little use for accordion reeds, which have no bore to model. Millot argues that for short loaded pipes the wave description disagrees with the valid incompressible-flow description.
   - Only chamber and body colouration remains. A few biquads suffice: Puranik found an enclosure fitted well by 10 biquads, parametrised by 8–9 peaks.

**Conclusion.**
- For RF-Musette the defensible core is **(3) with the numerics of (5)**:
  - a lumped or few-mode reed;
  - Millot-style useful-section flow, with passivity shown as in Darabundit;
  - upstream chamber and neck inertia;
  - a shared bellows reservoir;
  - discretised linearly implicitly or with WDF so that any parameter set is stable;
  - modest oversampling (2×) plus antiderivative or table-integrated aperture functions against aliasing;
  - a biquad-bank body.
- **(2) serves as the reference and "eco" voice.**
- Validate against the bandoneon and harmonium measurements. Their pressure → pitch trends run opposite in sign: harmonium f0 rises as pressure falls; bandoneon low notes go flat and high notes sharp as pressure rises.

## (b) What nobody has done openly (the gap RF-Musette could fill)

1. **No open, licensed, real-time physical model of an accordion exists.**
   - Every open free-reed synth found is either a signal model (libsonare Apache-2.0; toys), unlicensed (MegaLoler; rohitbams, which uses Euler plus clamps), or unpublished (Puranik's Pd patch; Bandoneon 2.0's Faust and Bela code).
   - STK, Faust, Csound, SuperCollider, Pd, Modalys, Pianoteq and SWAM have **no** free reed.
2. **No free-reed model with guaranteed numerical stability.**
   - The published free-reed schemes are Millot's NR-plus-bisection and Puranik's implicit bar with NR. Puranik reports parameter-dependent instability.
   - Energy-consistent (PHS/SAV/WDF) treatments exist only for beating single reeds and vocal folds.
3. **No model couples the bellows air budget to the number of sounding reeds.**
   - Roland solved this *mechanically* (servo valve, EP1752966 / EP2779154) and *sample-wise* (US6946594 thresholds).
   - No synthesis model feeds a shared bellows reservoir through per-register reed chambers, where opening more reeds or registers drops the pressure every reed sees.
   - That is the physical origin of "Dynamic Bellows Behavior", of chord-versus-single-note dynamics and of the LMMH register balance.
4. **No open model of multi-reed registers with emergent musette.**
   - That is: per-reed pressure-dependent detune (measured sign changes across the range), speaking thresholds, and push/pull reed differences with **leather-valve closing transients** (claimed as samples in US6946594 claim 25).
5. **No open, measured reference data set** for accordion reeds (pressure → f0, level, spectral centroid, onset time).
   - Bandoneon 2.0 measured one, but the data is not public. The Puranik LDV data is in paper figures only.
   - RF-Musette could publish its own under a clear licence. Tools exist to check tuning: dan-vine/accordion-tuner (MIT).
6. **No standard for signed or high-resolution bellows MIDI.**
   - Commercial practice is unsigned 7-bit CC11. Bandolibre's 14-bit CC11 plus SysEx direction is the only open richer scheme, and its licence is non-commercial (reference only).

## (c) Recommended MIDI control mapping for bellows (backed by what hardware sends)

Receiver side. RF-Musette must play correctly from a Roland FR and from the open controllers without configuration.

1. **Bellows magnitude = CC#11 (Expression), per channel. Default ON.**
   - This is what Roland FR-8x and FR-4x transmit (7-bit), what Bandolibre and bvavra send, and reportedly Cavagnolo.
   - **Accept 14-bit CC#11 + CC#43 (LSB)** when an LSB arrives (Bandolibre). Otherwise treat the value as 7-bit.
   - Interpret it as **bellows pressure (a target for the physical bellows or reservoir source), not output gain**. The model then makes level, pitch drift and brightness emerge from pressure.
   - Per-user curve option: linear, or the **cube / cube-root companding** used by Bandoneon 2.0 to get low-pressure resolution out of 7 bits. Roland and Ketron expose equivalent "bellows curves": Light / Std / Heavy / Fixed.
2. **Alternates, selectable:**
   - **CC#2 Breath** (libsonare convention, breath controllers);
   - **CC#7** (schult/anglo-midi; some retrofits);
   - **CC#1 Mod wheel** for keyboard players without a bellows sensor (sample-library and rohitbams practice);
   - **Channel Pressure** (Musictech offers bellows-driven aftertouch; Roland transmits channel pressure).
   - Only one source is active at a time, so a controller that sends CC7 as master volume does not double-drive.
3. **Bellows direction** (needed physically: push and pull use different reeds and valves, and bisonoric layouts need it). No commercial standard exists, so:
   - **Default: no direction information, assume a single direction.** This matches Roland output. Keep the push/pull reed pair symmetric so nothing breaks.
   - **Optional direction CC** (proposal, not a standard): a switch CC (for example a general-purpose controller in 80–83; ≥ 64 = pull, < 64 = push). Or a **signed 14-bit bellows CC pair centred at 8192** for controllers that measure signed differential pressure (Bandoneon 2.0 Alfa style). Document it as RF-Musette's own convention.
   - **Optionally recognise Bandolibre's direction SysEx** as an input. Implementing a message format from a CC BY-NC-SA project is interoperability, not copying code, but confirm with the provenance owner.
4. **Registers = Bank Select (CC0/CC32) + Program Change**, following Roland (treble registers 1–14 = PC 0–13, bank 0/0; bass and chord registers on their own channels) and Musictech.
   - **Apply a register change at the next note-on for held notes**, as Roland does: "sound will change beginning with the next Note-on". A physical model could also switch live by opening and closing reed ranks.
5. **Channel layout:** default treble on channel 1, bass on 2, chord on 3 (Roland FR-8x defaults). **Bellows is global.** Apply CC11 from any bellows-carrying channel to the shared reservoir: Bandolibre and bvavra send it on several channels at once, which is physically one bellows.
6. **Smoothing:** CC streams are sparse and stepped. Roland's default "Low" density and Bandolibre's rate limit both reduce them. Smooth to audio rate inside the engine (a one-pole or 1€-style filter on the pressure target) before it drives the reservoir, so steps never reach the reed ODE as impulses.
7. **Velocity:** accordion keys are not velocity-sensitive in acoustic instruments. Roland's FR-4x treble keyboard is. Default velocity to a small attack-shaping role only, with an option (Cavagnolo-style "expression **or** velocity") to derive the initial bellows pressure from velocity when no bellows CC has been received.

---

### Licences at a glance (provenance record; nothing is to be copied)

- **Permissive:**
  - libsonare (Apache-2.0);
  - bandonino (MIT);
  - MIDI-Accordina (MIT);
  - accordion-tuner (MIT);
  - SmeggTuner (MIT);
  - Faust libs (LGPL-2.1+ with the GRAME exception);
  - STK (MIT-style STK licence).
- **Copyleft:** AccordionMega (GPL-3.0), dronmakr (GPL-3.0), SuperCollider (GPL-3.0), Csound (LGPL-2.1).
- **Non-commercial:** Bandolibre (CC BY-NC-SA 4.0), Euphonics (CC BY-NC-SA 4.0).
- **No licence (all rights reserved):**
  - rohitbams/Harmonium-Physical-Model;
  - MegaLoler/Accordion-Synth;
  - bvavra/MIDI_Accordion;
  - schult/anglo-midi;
  - chordian;
  - AdamLauz;
  - jonHickam teensy synth;
  - billthefarmer/tuner (per the API).
- **Papers:**
  - DAFx-23 (CC BY 4.0, the proceedings add "non-commercial" wording);
  - FA 2023 (CC BY 3.0);
  - NIME 2023 and NIME 2025 (CC BY 4.0);
  - Millot 2007 (arXiv non-exclusive);
  - JASA, CMJ and PoMA are publisher copyright.
