# RF-Musette literature notes: physics of the free reed (the sound source)

Researched 2026-09-30. Scope: reed mechanics, aerodynamic self-excitation, pitch/amplitude/spectrum vs pressure, thresholds and transients, coupling to cavities, theses, reviews.

## How to read the tags

- **VERIFIED-FT**: I read the full text (open PDF). Numbers quoted from it are first-hand.
- **VERIFIED-ABS**: I saw the record and abstract on a publisher or index page (Crossref, OpenAlex, PubMed E-utilities, HAL API, conference site). Numbers come from the abstract only.
- **VERIFIED-CIT**: I saw the citation only in the reference list of a VERIFIED-FT source. I could not open the source itself.
- **UNVERIFIED**: secondhand mention only.
- "(fig-read)" marks a number I read off a plotted figure, not from text. Treat it as ±1 grid unit.

Access limits during this session:
- pubs.aip.org (JASA, POMA, Physics Today PDFs), ResearchGate, and PubMed HTML returned 403.
- HAL's web UI is behind Anubis, but its PDFs came through by curl and the HAL API worked.
- Crossref and OpenAlex rate-limited me (HTTP 429) late in the session.
- I could not access the full texts of **Ricot 2005**, **St. Hilaire 1971**, **Bahnson 1998**, **Fletcher & Rossing** or the **Cottingham POMA papers**. Their entries rest on abstracts plus how verified sources describe them.

---

## 1. Reviews and overviews

### 1.1 Cottingham, J. P. (2011). "Acoustics of free-reed instruments." *Physics Today* 64(3), 44–48. DOI 10.1063/1.3563819
- URL: https://physicstoday.aip.org/features/acoustics-of-free-reed-instruments ; PDF https://pubs.aip.org/physicstoday/article-pdf/64/3/44/9880657/44_1_online.pdf (403 for me)
- Access: HTML summary page fetched. PDF paywalled or blocked. **VERIFIED-ABS** (DOI via OpenAlex; content via the HTML page).
- Content:
  - Two families: Asian (symmetric reed, blown-open, needs a pipe) and Western (offset reed, blown-closed, sounds without a resonator).
  - Tip amplitude is about **15 % of tongue length**.
  - Harmonica mean flow is "hundreds of mL/s".
  - The radiated waveform comes in two puffs per cycle.
  - Uses Fletcher's linear model: the reed is a damped driven oscillator, and oscillation needs Re(admittance) < 0.
  - Western (blown-closed) reeds sound slightly **below** their natural frequency.
- Informs: overall architecture and sanity checks.

### 1.2 Braasch, J. & Cottingham, J. P. (2023). "Free Reeds: An Intertwined Tale of Asian and Western Musical Instruments." *Acoustics Today* 19(4), 14–22. DOI 10.1121/AT.2023.19.4.14
- URL: https://acousticstoday.org/wp-content/uploads/2023/11/Free-Reeds-An-Intertwined-Tale-of-Asian-and-Western-Musical-Instruments-Jonas-Braasch-and-James-P.-Cottingham-3.pdf
- Access: open PDF. **VERIFIED-FT**
- Content:
  - Amplitude is about 15 % of reed length.
  - Near-field pressure of a blown accordion reed is **roughly square-wave**, "siren-like".
  - Flow per Millot–Baumann: two large puffs per cycle plus smaller puffs when the reed passes below the plate.
  - Onset: the fundamental sounds first and higher harmonics appear progressively. Onset is "fairly long" compared with beating reeds.
  - Free reeds show an **upward shift in fundamental during the onset**, "often in the ascending order of semitones", over "several dozen milliseconds". Not quantified for accordion.
  - Only free reeds keep their pitch under wind-pressure variation. Flue and striking-reed stops rise in pitch with pressure.
  - For Asian blown-open reeds with pipes, the sounding frequency is above both the reed and pipe frequencies (Hikichi et al. 2003).
- Informs: onset-transient targets (harmonic ordering, onset pitch glide) and pitch stability under bellows pressure.

### 1.3 Woodhouse, J. (euphonics.org). "11.6 Free reeds." *Euphonics* (online book).
- URL: https://euphonics.org/11-6-free-reeds/
- Access: open web page. **VERIFIED-FT**, though I read it through an automated summary. Figures were not inspected.
- Content:
  - Reimplements the Fletcher / Tarnopolsky / Millot lumped model.
  - Example harmonica geometry: reed 14.5 × 2 mm, plate 0.8 mm, stand-off 0.5 mm, clearance 0.2 mm all round.
  - Key finding (from Millot's configuration): with a supply tube, the behaviour depends on whether the **Helmholtz resonance of chamber + tube is above or below the reed frequency**. When it is lower, the *closing* (blown-closed) reed plays readily and the opening reed does not.
  - Simulated pressure has "a pair of peaks in every cycle". During the transient the second peak grows to roughly half the first.
  - Author's own rig: reed Q ≈ **250**, pressures up to ~2.5 kPa, volumes up to ~2.5 L. The reversed (closing) reed vibrated in its **second (twisting) mode**.
  - Concedes that Ricot's purely fluid-dynamical mechanism "is also present", biasing towards closing reeds.
- Informs: which instability branch an accordion reed sits on, and a Q upper bound.

### 1.4 Fletcher, N. H. & Rossing, T. D. *The Physics of Musical Instruments*, 2nd ed., Springer 1998, chaps. 13–15
- **VERIFIED-CIT** (cited by Tarnopolsky 2000 as the summary of free-reed work). I did not read the chapter.

### 1.5 Fabre, B., Gilbert, J., Hirschberg, A. & Pelorson, X. (2012). "Aeroacoustics of Musical Instruments." *Annu. Rev. Fluid Mech.* 44, 1–25. DOI 10.1146/annurev-fluid-120710-101031
- **VERIFIED-ABS** (Crossref). Full text not read.
- Puranik & Scavone 2023 cite it for the claim that a quasi-steady Bernoulli mechanism alone cannot produce the pressure asymmetry needed for self-oscillation. That content is **UNVERIFIED** first-hand.

### 1.6 Volpatti, G. (2025). "Beyond the Bellows: A Critical Review of Free Reed Instrument Research…" *Am. J. Arts & Human Science* 4(1). DOI 10.54536/ajahs.v4i1.3842
- Also Volpatti (2026), "A Matrix Framework for Detuned-Unison Accordion Musette Tuning…", *Am. J. Math. & Physics*.
- **VERIFIED-ABS** (OpenAlex, HAL). Venue quality is doubtful (e-palli journals). **Do not cite for constants.** It is listed here only so nobody mistakes it for a primary source.

---

## 2. Reed mechanics: modes, profile, damping, material

### 2.1 Cantilever-beam basis (textbook)
- Uniform Euler–Bernoulli clamped-free bar: f1 = (1.875²/2π)(t/L²)·√(E/12ρ) ≈ 0.1615·(t/L²)·√(E/ρ). Mode ratios are **1 : 6.27 : 17.5**.
- Cottingham et al. 1999 (§2.3) quote f2 = 6.27 f1 and fit the measured blown-reed profile with **Rayleigh's first-mode curve** (*Theory of Sound* vol. 1, p. 289).
- Tarnopolsky et al. 2000 (§3.2) project the Euler–Bernoulli PDE onto mode 1 (they cite Morse, *Vibration and Sound*, 1948).
- **Measured blown reeds deviate strongly from the uniform-bar ratios:**

| Reed | f1 | f2 | f2/f1 | Other modes | Source |
|---|---|---|---|---|---|
| Reed-organ F# | 96 Hz | 811 Hz | ≈ 8.4 | torsion 1610 Hz | Cottingham ICA 2019 |
| Reed coupled to a mismatched pipe | 542 Hz | 2140 Hz | ≈ 3.9 | 3rd ≈ 5192 Hz | Cottingham CCRMA 2013 slides |
| Harmonium B5 | 990 Hz | ≈ 6000 Hz | — | 3rd ≈ 16000 Hz | Puranik & Scavone 2023 |

  Taper, tip curvature, tip loading and the rivet all matter. **Constants must be measured per reed, not derived from a uniform bar.**
- Sanity check of the uniform formula: Puranik's harmonium brass reed (L = 16 mm, t = 0.4 mm, E = 125 GPa, ρ = 8490 kg/m³) gives about 968 Hz against a measured 990 Hz. Millot's harmonica reed (12.95 mm, 110 µm, brass) gives about 406 Hz against 444 Hz. My arithmetic, not from the papers.

### 2.2 Cottingham, J. P. (2013). "Modes of reed vibration and transient phenomena in free reed instruments." *Proc. Meet. Acoust.* 19, 035061. DOI 10.1121/1.4799611
- Also JASA 133, 3501 (abstract), DOI 10.1121/1.4806225.
- **VERIFIED-ABS** (OpenAlex).
- Findings:
  - Harmonica, accordion and reed-organ reeds are dominated by the first transverse mode, but **higher transverse modes and the first torsional mode are usually present in steady oscillation, even at low amplitude**.
  - A **lateral** (in-plane) mode is sometimes seen.
  - Accordion **reed-cavity resonances can interfere with self-excitation**.
  - Harmonica: near-closed reed → **torsional flutter**.
  - Slow attack is worse for longer, lower reeds. The 2nd transverse and 1st torsional modes may help initiate oscillation.
- Informs: whether to carry modes 2 and T1 in the reed state (at least for transients).

### 2.3 Cottingham, J. P., Lilly, C. J. & Reed, C. H. (1999). "The motion of air-driven free reeds." Proc. Forum Acusticum / 137th ASA, Berlin 1999
- Abstract: JASA 105, 940, DOI 10.1121/1.426334.
- URL (full paper): https://www.cs.bu.edu/fac/snyder/TablatureWeb/BERLIN1.PDF
- Access: open PDF. **VERIFIED-FT**
- Reed-organ reeds, 100–200 Hz, tongue about **4 cm**:
  - **Threshold ≈ 0.2 kPa**. Above threshold the tip amplitude "quickly rises to a large value of several millimeters", stays nearly constant with pressure, then decreases at high pressure.
  - **The equilibrium shifts 1–2 mm in the flow direction.**
  - Motion is nearly sinusoidal at normal pressure (~0.8 kPa for a 118 Hz A reed) and becomes irregular with smaller amplitude at 1.2 kPa. The second beam mode shows up at higher pressures.
  - Measured profile at maximum amplitude matches the Rayleigh mode-1 shape.
  - The near-field pressure is pulse-like even though the reed motion is sinusoidal.

### 2.4 Paquette, A., Vines, J. E. & Cottingham, J. P. (2003). "Modes of vibration of air-driven free reeds in transient and steady state oscillation." JASA 114, 2348 (abstract). DOI 10.1121/1.4781137
- **VERIFIED-ABS**
- The second beam mode is present even at low amplitudes and is often seen before full amplitude.
- Two attack protocols: restrained-and-released, and pallet-valve rush.

### 2.5 Behrens, S., Coyle, W., Goodweiler, N. & Cottingham, J. P. (2009). "Vibrational modes of accordion reeds." JASA 126, 2216 (abstract). DOI 10.1121/1.3248800
- **VERIFIED-ABS**. **Accordion-specific.**
- Transverse modes up to the **4th** plus the first torsional mode are present. Modes 2 and 3 are observable even at low amplitude. A **lateral mode** is observed for the first time.
- **The secondary (non-sounding) reed of the pair also vibrates**, with relatively more high-mode content than the primary.
- Finite-element mode calculations confirm the mode identifications.
- Informs: the second reed of each reed plate is not acoustically inert.

### 2.6 Puranik, N. & Scavone, G. (2023). "Clamped bar model for free reeds." Proc. Forum Acusticum 2023 (Turin), pp. 1809–1815. DOI 10.61782/fa.2023.0853
- URL: https://dael.euracoustics.org/confs/fa2023/data/articles/000853.pdf
- Access: open (CC BY 3.0). **VERIFIED-FT**
- LDV on a B5 (990 Hz) harmonium reed:
  - Motion is largely sinusoidal at ≈0.2 kPa. **Harmonics become prominent at 1.4–1.6 kPa.**
  - Real harmonium reed-chamber pressures are **0.2–1.5 kPa**.
- Model: replaces the lumped reed with an Euler–Bernoulli beam, ρA ÿ + R ẏ + EI y'''' = Sr(p2 − patm), clamped-free BCs, Chaigne–Doutaut implicit scheme, N = 40 segments, fs = 4 × 44.1 kHz.
- Parameters: E = 125 GPa, L = 16 mm, W = 2 mm, t = 0.4 mm, R = 0.065, V1 = 1.3 L, L2 = 50 mm, S2 = 6.4e-5 m², ρ_reed = 8490 kg/m³, initial gap Δy = −0.1 mm.
- Result: the reed acts as a filter with formant peaks at its natural frequencies.
- **Admits the lumped Millot–Baumann model needed "unrealistic" parameters (reed thickness several times its length) to be stable.**
- Near-field and far-field geometry (V1, L2, S2) remain arbitrary.

### 2.7 Macho-Stadler, E., Elejalde-García, M. J. & Llanos-Vázquez, R. (2015). "Oscillations of end loaded cantilever beams." *Eur. J. Phys.* 36, 055007. DOI 10.1088/0143-0807/36/5/055007
- **VERIFIED-ABS**
- Tip mass lowers f1 approximately linearly in their range. They explicitly apply this to free-reed tuning.
- Informs: tuning model (tip weights on bass reeds, tip or root scraping).

### 2.8 Damping (Q) values found

| Reed | Q | Source | Status |
|---|---|---|---|
| Brass flap, 28 × 52 mm, bare | ≈ 55 (44–52 in another set); 10 with tape at the root | Tarnopolsky 2000 | VERIFIED-FT |
| Diatonic harmonica reed | **95** (from Millot's thesis measurements) | Millot & Baumann 2007, Table 2 | VERIFIED-FT |
| Reed-organ C reed (Wolfinger) | damping coefficient **D = 0.012** in Fletcher/Johnston notation, i.e. Q ≈ 1/D ≈ 83 *if* D = 1/Q (my inference) | Cottingham ICA paper | VERIFIED-FT |
| Harmonica model reed | D_r = 0.05 (Q ≈ 20 under the same reading) | Johnston 1987, Fig. 6 caption | VERIFIED-FT (scanned) |
| Euphonics bench reed | ≈ 250 | euphonics.org | VERIFIED-FT |
| Harmonium reed | R = 0.065 (beam damping, units not given) | Puranik 2023 | VERIFIED-FT |

- **No Q measurement for a steel accordion reed was found.** This is an open item.
- Tarnopolsky add an aerodynamic damping term k = ω0/2Q + β ρ v (W + 0.8L) x0 / m with β ≈ 1.

### 2.9 Material
- Steel for accordion reeds: only industry sources and forum text say blue-tempered spring steel, RC 48–52. **UNVERIFIED.**
- Harmonium reeds are brass (Puranik: E = 125 GPa, ρ = 8490).
- Pinelli, R. (2022), "Discovering Italian Free Metal Reeds: A Fieldwork Enquiry", *Eur. J. Musicology* 20, 152–172 (HAL hal-03637033, open PDF, **VERIFIED-ABS**), is an ethnographic study of the Voci Armoniche reed factory (Castelfidardo). It is the only scholarly source on Italian reed manufacture I found. It is not a physics paper.

---

## 3. Aerodynamics and self-excitation

### 3.1 St. Hilaire, A. O., Wilson, T. A. & Beavers, G. S. (1971). "Aerodynamic excitation of the harmonium reed." *J. Fluid Mech.* 49(4), 803–816. DOI 10.1017/S0022112071002374
- **VERIFIED-ABS** (OpenAlex). Full text paywalled.
- Amplitude **grows exponentially**. Growth rate was measured against flow.
- **Jet and wake instability are not responsible** (flow visualization).
- Unsteady **potential-flow** analysis: the exciting pressure is of order **ρ U0 ȧ** (U0 = flow speed, ȧ = reed velocity), i.e. a pressure in phase with reed velocity, which is negative damping.
- Puranik 2023 summarises it as upstream flow inertia feeding a narrow jet.
- Informs: **the core excitation term.**
- Follow-ups:
  - St. Hilaire, A. O. & Vaidya, P. G. (1975), "Finite amplitude analysis of a flow-structure interaction problem," *J. Fluid Mech.* 67(2), 377–396, DOI 10.1017/S0022112075000365. **VERIFIED-ABS.** Nonlinear dissipation from induced higher-harmonic potential-flow disturbances sets the **limit amplitude**.
  - St. Hilaire, A. O. (1976), "Analytical prediction of the non-linear response of a self-excited structure," *J. Sound Vib.* 47(2), 185–205, DOI 10.1016/0022-460X(76)90717-3. Record only.

### 3.2 Fletcher, N. H. (1979). "Excitation mechanisms in woodwind and brass instruments." *Acustica* 43, 63–72
- Erratum/companion: Fletcher, Silk & Douglas, *Acustica* 50, 155–159 (1982).
- URL: https://openresearch-repository.anu.edu.au/handle/1885/125600
- Access: open. **VERIFIED-FT** (abstract section read).
- The reed admittance seen from the mouthpiece must have **negative real part larger than the pipe's**. This implies a **minimum blowing pressure**.
- Inward-striking (blown-closed) reeds run **below** the reed resonance. Outward-striking reeds run above it.
- This is the linear frame later used for free reeds by Johnston 1987 and Cottingham.

### 3.3 Fletcher, N. H. (1993). "Autonomous vibration of simple pressure-controlled valves in gas flows." JASA 93(4), 2172–2180. DOI 10.1121/1.406857
- URL: https://www.phys.unsw.edu.au/music/people/publications/Fletcher1993.pdf
- Access: open. **VERIFIED-FT** (text extracted; I read the key passages).
- Introduces the **(σ1, σ2)** classification.
- **States that harmoniums and harmonicas use reeds of types (+,−) and (−,+) in an apparently free configuration.**
- Necessary condition: σ1X1 − σ2X2 < 0 (per Tarnopolsky's restatement). With short ducts (< λ/4) oscillation is near the valve resonance.

### 3.4 Tarnopolsky, A. Z., Fletcher, N. H. & Lai, J. C. S. (2000). "Oscillating reed valves — An experimental study." JASA 108(1), 400–406. DOI 10.1121/1.429473
- URL: https://www.phys.unsw.edu.au/music/people/publications/Tarnopolskyetal2000.pdf
- Access: open. **VERIFIED-FT.** The most complete validated lumped model with numbers.
- Valve type: **(+,−)**, blown-open. The accordion reed is (−,+). The equations carry over; the signs do not.
- **Equations:**
  - Exit area (eq. 2): F(x) = W√(x² + b²) + 2L√(a(x)² + b²).
  - Side opening (eq. 5 / A5): a(x) ≈ 0.6 x0 + 0.4 x, planar flap, valid only while the flap is out of the aperture.
  - Pressure–flow with contraction and gap inertia (eq. 3): p = ρU²/(2C²F²) + ∂/∂t[ρ U d/(C F)].
  - Reed (eq. 4): ẍ + 2k ẋ + ω0²(x − x0) = 1.5 W L p / m, where 1.5 = ∫ψ / ∫ψ² for the mode-1 shape and m ≈ ρ_v W L h + 4m′ for a tip mass m′.
  - Damping (eq. 6): k = ω0/2Q + β ρ v (W + 0.8L) x0 / m, with β = 1.
  - Reservoir (eq. 7): dp/dt = (ρc²/V)(U0 − U − 0.4 W L ẋ). The last term is the **pumped (displacement) flow**.
  - Linear frequency shift (eq. 9): ω² ≈ ω0² + 0.6 ρc² W² L² / (mV). **It overestimates the shift.** The large-signal shift is much smaller.
- **Numbers:**
  - C = 0.61 for a sharp slit; **C can range 0.5–1.0** with geometry.
  - Flap: brass, 0.15 or 0.33 mm thick, 28 mm long, 52 mm wide, clearance b ≈ 0.5 mm, aperture plate 4 mm thick, x0 = 0.5 or 1.0 mm.
  - f0 = 100–300 Hz. Q ≈ 55 bare, down to ~10 with tape.
  - Threshold pressure depends on reservoir volume, with an optimum volume. Threshold rises with damping.
  - **Oscillation grows abruptly with a pressure jump**, and **hysteresis** lets the flow be reduced below threshold without stopping.
  - **Frequency does not depend significantly on blowing pressure or damping** (for this (+,−) valve).
  - In the limit cycle the flap sits inside the aperture for nearly half of each cycle. Above ~5 mm amplitude it emerges from the back.
  - Reservoir pressure example: 350 Pa mean at V = 0.8 L. The reservoir pressure is ~180° behind the opening, so it is dominated by displacement flow.
  - A downstream vortex forms during closing (Schlieren) but "does not have a major effect".
  - Striking valves: amplitude < 2 mm and higher cantilever modes.
- Informs: area function, pumped flow, reservoir compliance, contraction coefficient, threshold/hysteresis behaviour.
- Companion: Tarnopolsky, A. Z., Lai, J. C. S. & Fletcher, N. H. (2001), "Flow structures generated by pressure-controlled self-oscillating reed valves," *J. Sound Vib.* 247(2), 213–226, DOI 10.1006/jsvi.2001.3677. **VERIFIED-ABS** (record); content per search snippet, **UNVERIFIED**: a stable downstream vortex during closing that moves off and dissipates during opening.

### 3.5 Ricot, D., Caussé, R. & Misdariis, N. (2005). "Aerodynamic excitation and sound production of blown-closed free reeds without acoustic coupling: The example of the accordion reed." JASA 117(4), 2279–2290. DOI 10.1121/1.1852546. PMID 15898668
- HAL record hal-01106161 (no PDF deposited).
- Access: abstract via PubMed/OpenAlex. Full text paywalled. **VERIFIED-ABS**
- **The reference paper for the accordion reed.**
  - Water flow visualisation and measurements: flow is **incompressible and potential**.
  - Excitation is the **inertial load of the unsteady flow through the reed gaps**, giving gap-velocity fluctuations and an unsteady Bernoulli force.
  - Adds a pressure term from local reciprocal air motion around the reed.
  - The 2-D model agrees reasonably with very-near-field pressure.
  - Far-field via permeable **Ffowcs Williams–Hawkings**: **sound is dominated by the dipole** from strong momentum fluctuations of the gap flow.
- Informs: excitation term; the radiation model (dipole, not monopole).
- **Must be read in full; constants unknown to me.**

### 3.6 Misdariis, N., Ricot, D. & Caussé, R. (2000). "Modélisation physique de la vibration d'une anche d'accordéon." Proc. 5e Congrès Français d'Acoustique (CFA 2000), Lausanne, pp. 281–284
- URL: https://hal.science/hal-01161356/document
- Access: open. **VERIFIED-FT** (French). **The only accordion-reed paper with pressure numbers that I could read.**
- **Normal playing pressure: 10–300 Pa for a mid-register reed.** Measured range 10–6000 Pa.
- **Reed displacement is sinusoidal over the whole 10–6000 Pa range.** Checked with a laser vibrometer. The reed vibrates on its first clamped-free mode slightly **below** its eigenfrequency.
- **Two regimes with hysteresis:**
  - Regime 1: reed equilibrium outside the plate. Frequency falls ~linearly with P0.
  - Regime 2: equilibrium inside the slot. Frequency rises fast and can exceed the eigenfrequency. "Observable for some reeds in normal playing conditions."
  - Fig. 3 (fig-read): regime 1 falls from ~304 Hz near 0 Pa to ~294 Hz at 6 kPa (about −58 cents over 6 kPa, **~−10 to −13 cents/kPa**). Regime 2 rises from ~296.5 Hz at ~4 kPa to ~306 Hz at ~5.7 kPa.
  - **Within the 10–300 Pa normal range the fig-read drift is only a few cents.**
- **Playing dynamic ≈ 40 dB.**
- Sustained spectrum is broad. **For the first partials, even harmonics are much weaker than odd ones.** Upstream/downstream resonators (reservoir, reed cavities) have **negligible influence on the spectrum** in their setup.
- Flow speed ~**15 m/s** in air (2 m/s in water). Strouhal ~0.15. The same reed runs at 330 Hz in air and 142 Hz in water.
- Upstream flow is laminar, sink-like and 2-D near the reed. Downstream is a jet attached to the slot wall.
- **Excitation comes from inertia of upstream fluid, not jet turbulence.** The downstream face is "dead" fluid.
- Key parameters: P0 and "K0, the amount of fluid available in the reservoir that takes part in the inertia".
- Model: mass-spring-damper plus a 2-D unsteady potential flow (point sink simplification). Sound is a linear combination of monopole and dipole sources.
- Earlier work cited: D. Ricot, *Modélisation physique de la vibration d'une anche d'accordéon*, end-of-studies/DEA report, École Centrale de Lyon / IRCAM, 1999 (**VERIFIED-CIT**).
- Also: Caussé, Misdariis & Ricot (1999), "Studies of accordion reed vibrations — Applications in sound synthesis," JASA 106, 2287 (abstract), DOI 10.1121/1.427818, **VERIFIED-ABS**. Measured near-field pressure above and below the reed, capacitive displacement, LDV, reed profile, stroboscopy, flow visualisation. Aimed at a Modalys (modal) implementation.

### 3.7 Millot, L. & Baumann, C. (2007). "A proposal for a minimal model of free reeds." *Acta Acustica united with Acustica* 93, 122–144
- arXiv:2401.01606 (2024 posting of the published text); HAL hal-04369449.
- URL: https://arxiv.org/abs/2401.01606
- Access: open. **VERIFIED-FT**
- **Reed:** single DOF on the first clamped-free mode, d²ξ/dt² + ω0 Q⁻¹ dξ/dt + ω0² ξ = µ Δp (eq. 4). ξ is the neutral-section tip displacement from rest. The reed is assumed to adopt the mode-1 profile at rest.
- **Flow:**
  - Pumped flow up = Sr dξ/dt (eq. 5).
  - Jet flow ut = α Su(hn) vj (eq. 6).
  - Quasi-steady Bernoulli Δp = ½ρ vj² (eq. 7), justified by a gap Strouhal of 0.024–0.027. Mach < 0.1, Re > 1235.
- **Useful section Su:**
  - Reviews the front-only model W·h, Cuesta–Valette's perimeter model, Millot's zero-inside-slot model, Tarnopolsky's F(x), and Debut–Millot's integral model.
  - Proposes a new model that separates **neutral, upstream and downstream faces**, so **reed thickness** enters, and integrates the whole escape area: front trapezoid (W + hmin)·h_end plus sides plus missing corner areas.
  - **Reed thickness strongly affects amplitude, playing frequency and sound level.**
- **Minimal configuration:** constant inflow u0 into V1 (adiabatic compliance), then a duct of length L2 and area S2 (**inertance**, dynamic Bernoulli p1 − p2 = ρ(L2/S2) du/dt), then the reed, then atmosphere.
- **Instability condition (linear):** for a (−,+) reed it needs 1 − V1 L2 ω² / (c² S2) < 0, i.e. **the upstream Helmholtz frequency must lie below the reed frequency**. The opposite holds for (+,−).
- **Table 2 constants** (diatonic G harmonica, channel 4, (+,−) reed, Lee Oscar, from Millot et al. 2001):
  - Length 12.95 mm, width 2.1 mm, thickness 110 µm, support thickness 900 µm, rest departure 528 µm, clearance hmin 50 µm.
  - **fr = 444 Hz, K = 47.9 N/m, Q = 95.**
  - The table columns were garbled in extraction; this row mapping is my reconstruction and is consistent with beam theory.
- **Table 3:** feeding pipe 30 mm², V1 cross-section 800 mm², L2 = 20 mm, S2 = 25 mm².
- **Results:**
  - Motion is sinusoidal. The (−,+) reed plays **below** fr and the (+,−) above.
  - The reed enters the plate thickness for a noticeable part of the period.
  - Near-field p2 amplitudes 570 and 783 Pa. Total flow peaks ~160–180 mL/s with pumped flow ±40–50 mL/s (fig-read). **The pumped flow is not negligible.**
  - L1 changes the (−,+) frequency by only 2.6 Hz versus 22.2 Hz for (+,−).
  - Excitation level changes frequency by only **0.4 Hz** for (−,+) and 2 Hz for (+,−), but the p2 amplitude changes by 2780 Pa and 3177 Pa respectively.
  - **Suspected inverse (subcritical) bifurcation:** violent departure from rest and sound maintained below threshold.
- Not validated experimentally for the single-reed case (the authors say so).
- Informs: the backbone of RF-Musette's reed + flow + upstream model.

### 3.8 Puranik, N. V. & Scavone, G. P. (2022). "Physical modelling synthesis of a harmonium." *Proc. Meet. Acoust.* 49, 035015. DOI 10.1121/2.0001679
- **VERIFIED-ABS**. Equations as restated in Puranik 2023 (**VERIFIED-FT**).
- Adapts Millot–Baumann to a bellows-driven harmonium. Adds a **bellows chamber p0** that drives u0, so bellows pressure becomes the control input.
- A source-filter structure (all-pole filter fitted to recordings) stands in for the enclosure.
- Equations: V1/c² · d(p1 − patm)/dt = ρ(u0 − u); p1 = p2 + ρ(L2/S2) du/dt; p2 = patm + ½ρvj²; reed eq. 4; u = Sr ζ̇ + α Su vj; Su = W√((y + Δy)² + h²).
- Informs: **the closest published precedent for a bellows-driven free-reed synth**, which is the RF-Musette use case.
- Follow-up: Puranik, N. & Scavone, G. (2024), "Aeroacoustic modeling of blown-closed free reeds," JASA 155, A196 (abstract), DOI 10.1121/10.0027284, **VERIFIED-ABS**. Replaces the two-zone flow with a **continuous potential-flow** description and discusses real-time stability.

### 3.9 Cottingham et al. finite-element and CFD attempts (abstracts)
- Kaufinger, P., Wynne, L. & Cottingham, J. P. (2021), JASA 150, A211, DOI 10.1121/10.0008149. **VERIFIED-ABS.** FE multiphysics reproduces frequency–pressure trends and khaen impedance but **"fail[s] to capture the essential feature of free reed sound production via the periodic interruption of airflow through the reed plate."**
- Foti, Q., Sabin, M. & Cottingham, J. P. (2023), "Experimental tests of free reed instrument simulations," JASA 153, A40, DOI 10.1121/10.0018076. **VERIFIED-ABS.** Confirms the frequency–pressure benchmark near normal playing pressures, with "a variety of anomalous results" at higher pressure.
- Hassard, B. R. & Cottingham, J. P. (2020), JASA 148, 2611, DOI 10.1121/1.5147256. **VERIFIED-ABS.** Reed–resonator FE coupling (khaen, Helmholtz). An unexpectedly large role for the 2nd transverse mode.
- **No PIV study of accordion-reed flow was found.** Water visualisation (Misdariis/Ricot) and Schlieren on a large flap (Tarnopolsky 2001) are all that exist.

### 3.10 Van Hassel, R. R. & Hirschberg, A.
- "Comments on linear theory of harmonium reed oscillation," ISMA'95 (Dourdan), pp. 130–133.
- "Linear theory of harmonium reed oscillation," ISMA 2001 (Perugia).
- **VERIFIED-CIT** (Millot & Baumann reference list). Per Millot: 3-D is no harder than St. Hilaire's 2-D/3-D matching.

### 3.11 Tonon, T.: "free reed tongue tip offset theory"
- Forum post (concertina.net), summarised in Cottingham ICA 2019. **UNVERIFIED and not peer reviewed.**
- Proposes vortex-/eddy-induced start with start pressure Ps = a·E·I / (W·L⁴) (a = offset) and eddy frequency f = vS/W.
- Cottingham reports that experimental verification was "underway". Do not use.

---

## 4. Pitch, amplitude and spectrum vs pressure; threshold; transients; extinction

### 4.1 Cottingham, J. P., Reed, C. H. & Busha, M. (1999). "Variation of frequency with blowing pressure for an air-driven free reed." Proc. Forum Acusticum / ASA Berlin
- Abstract JASA 105, 1001, DOI 10.1121/1.425800.
- URL: https://www.cs.bu.edu/fac/snyder/TablatureWeb/BERLIN2.PDF
- Access: open. **VERIFIED-FT**
- Frequency falls approximately linearly with pressure in the normal range, with anomalies:
  - A small **rising** region at the lowest pressures.
  - Levelling off, then a **rapid rise** at very high pressure.
- **Measured growth/damping coefficient vs pressure:** negative (extra damping) at low pressure, **maximum growth around 1.0–1.5 kPa**, decreasing at higher pressure. This confirms St. Hilaire: aerodynamic forces add damping at low flow and negative damping at high flow.
- Fletcher's linear admittance model with this pressure-dependent damping reproduces the frequency–pressure curve qualitatively. Reed-organ C3 reed at 136.9 Hz, data spanning roughly 127–134 Hz over 0–3.5 kPa (fig-read).
- Reed-organ C reed parameters (the pairs use Johnston's notation): pressure range 0–3000 Pa; U0 = C·P0^z with **z = 0.67** (empirical); reed resonance ωr ≈ 800–900 s⁻¹.

### 4.2 Cottingham, J. P. "Theoretical and experimental investigation of the air-driven free reed" (2-page proceedings paper, pp. 761–762)
- URL: https://www.cs.bu.edu/fac/snyder/TablatureWeb/ICA.PDF
- Access: open. **VERIFIED-FT**. The PDF carries no venue or year; the file name suggests an ICA proceedings, and it cites a 1998 ISMA paper, so it dates from 1998 or later. The venue is unconfirmed.
- **Measured constants for one reed (Wolfinger reed-organ C):**
  - ωr = 860 s⁻¹ (≈137 Hz).
  - Tip spring constant ≈ **150 N/m**, measured by hanging masses. The value is partly garbled in the scan; it is consistent with the mass and frequency.
  - Effective mass Mr = **2.1 × 10⁻⁴ kg**.
  - Damping coefficient **D = 0.012**.
  - Width b = 4.0 mm. Effective length a = 8.0 mm (Sr = ab).
  - z = 0.67.
- The offset reed (X0 > 0) behaves like a closing reed: min Re(Yr) lies below ωr. The symmetric reed (X0 = 0) behaves like an opening reed.
- **The linear model matches the measurements only at 0.6–1.0 kPa, not at low or high pressure.**
- Informs: a complete measured parameter set that shows the measurement method; the limits of the linear model.

### 4.3 Koopman, P. D., Hanzelka, C. D. & Cottingham, J. P. (1996). "Acoustical properties of free reeds: A study of reeds from American reed organs." JASA 100, 2745 (abstract). DOI 10.1121/1.416872
- Also Koopman & Cottingham, *Reed Organ Soc. Bull.* 15(3–4), 17–23 (1997) (**VERIFIED-CIT**).
- **VERIFIED-ABS**: frequency first falls ~linearly with pressure, then anomalies at high pressure including a rise; **hysteresis in frequency–pressure**; constrictions simulating a partly opened pallet were also studied.

### 4.4 Coyle, W. L., Behrens, S. L. & Cottingham, J. P. (2009). "Influence of accordion reed chamber geometry on reed vibration and airflow." JASA 126, 2216 (abstract). DOI 10.1121/1.3248803
- **VERIFIED-ABS**. Data seen in Cottingham's CCRMA slides (§5.4).
- **Accordion reeds tested in the instrument**, with the bellows replaced by an acrylic wind chest and organ blower.
- Measured: LDV, probe mic, airflow, chamber geometry, partial pallet, frequency and amplitude vs pressure.
- Fig-read from the slides:
  - **G# reed ≈ 421.6 Hz at 0.1 kPa to ≈ 419.4 Hz at 0.9 kPa**, i.e. about −9 cents over 0.8 kPa. The slope is steeper below 0.3 kPa.
  - Low G# reed ≈ 96 Hz, full pallet: ~96.9 to ~96.4 Hz over 0.2–0.8 kPa.

### 4.5 Busha, Cottingham, Koopman and related 1999–2007 abstracts
- Busha, M., Cottingham, J. P. & Koopman, P. D. (2002), "Laboratory measurements on free reeds from the reed organ, accordion, and khaen," JASA 111, 2376 (abstract), DOI 10.1121/1.4778054. **VERIFIED-ABS.**
- Busha, M. & Cottingham, J. P. (1999), "Experimental investigation of air-driven free reeds using a laser vibrometer system," JASA 106, 2288, DOI 10.1121/1.427821. **VERIFIED-ABS.** Compares accordion reeds with reed-organ reeds.
- Toussaint, E. & Cottingham, J. P. (2007), "Reed vibration, pressure, and airflow in Western free-reed instruments," JASA 122, 3055, DOI 10.1121/1.2942895. **VERIFIED-ABS.** Airflow waveform from the integrated near-field pressure and a computed area function, calibrated to measured mean and minimum flow.
- CCRMA slides (§5.4): for accordion reeds at 0.6 kPa the near-field probe pressure axis spans ±300 Pa; reed-organ ±150 Pa. **"Because the accordion reed never passes below the (thick) reed frame, the accordion area function differs from that of the organ reed"**, which gives a siren-like waveform.

### 4.6 Attack transients
- **Biernat, J. & Cottingham, J. P. (2014).** "Attack transients in free reed instruments." *Proc. Meet. Acoust.* 20, 035002. DOI 10.1121/1.4891627 (JASA abstract 134, 3993). **VERIFIED-ABS.**
  - Pallet-valve-initiated attacks on reed-organ reeds.
  - **Growth rates were measured over a range of pressures** (values not in the abstract).
  - Spoon-shaped curvature and a slight tip twist are said to speed the attack of large low reeds.
- **Cottingham, J. P. (2019).** "A survey of recent studies on initial transients in free reed oscillation." Proc. ICA 2019 Aachen, pp. 5530–5535. URL https://pub.dega-akustik.de/ICA2019/data/articles/001430.pdf. Open. **VERIFIED-FT.**
  - Western reed starts with a displacement **into** the frame, then grows.
  - 96 Hz F# reed: second transverse mode (811 Hz) and first torsional mode (1610 Hz) appear in the first 10 cycles. Torsion peaks in cycles 10–20, then mode 2 dominates.
  - **Chamber (Helmholtz) resonance near the reed frequency raises the threshold "far above normal playing pressure" or prevents sounding. This is "choking".**
  - 268 Hz reed data with thresholds TP1 (rising) and TP2 (falling), so there is threshold **hysteresis**.
  - 344 Hz reed on a 327 Hz chamber: strong interference and distorted reed waveform. 268 Hz reed on a 327 Hz chamber: negligible effect.
  - Also JASA 145, 1676 (2019) abstract, DOI 10.1121/1.5101149, **VERIFIED-ABS**.
- **Henessee, Wolff & Cottingham (2014).** High-speed-video transients, JASA 136, 2201, DOI 10.1121/1.4899981. **VERIFIED-ABS.**
- **Llanos-Vázquez, R., Elejalde-García, M. J., Macho-Stadler, E. & Agos-Esparza, A. (2014).** "Physical and psychoacoustic characterization of the different types of attacks on the accordion." *Acta Acustica united with Acustica* 100(2), 375–384. DOI 10.3813/AAA.918716.
  - URL: https://dael.euracoustics.org/bin/EAA/aaua_dl?document_id=45451 (DAEL copy). **VERIFIED-FT.**
  - **Accordion, Pigini concert instrument.**
  - Attack time is measured from −50 dB to −5 dB of maximum on harmonic 1:
    - **Finger attacks: 50–110 ms (mf), 60–140 ms (p).**
    - **Bellows attacks: 190–630 ms (mf), 250–660 ms (p).**
  - No systematic dependence on note frequency across A2–B6.
  - Slow keystroke gives intermediate values. Normal finger attack ≈ 0.05 s; slow attack ≈ 0.5 s.
  - Spectral centroid starts near **8500–9000 Hz** early in the attack and ends near **2500 Hz**.
  - Cassotto has no effect on attack duration and attenuates SPL by ~30 % above 6 kHz.
  - Describes the reed plate: two reeds per plate with leather or plastic valves covering the opposite slot, and the reed/register layout.
- **Braasch, J. & Ahrens, C. (2000).** "Attack transients of free reed pipes in comparison to striking reed pipes and diapason pipes." *Acta Acustica united with Acustica* 86(4), 662–670. **VERIFIED-CIT** (in Acoustics Today 2023 references). Content per search snippet only (**UNVERIFIED**): free-reed pipe rise time is shorter than striking-reed pipes and comparable to diapasons, and the perceived slowness comes from delayed partial onsets.

### 4.7 Threshold, amplitude and extinction: summary of what is measured
- **Threshold:**
  - Accordion mid-register reed: normal play starts at **~10 Pa** (Misdariis 2000).
  - Reed organ, 100–200 Hz: ~200 Pa (Cottingham 1999).
  - Harmonica (Johnston 1987, blow 8 on G): **0.1–0.5 kPa with the top reed alone, 4.6–6 kPa with the bottom reed alone, 0.3–1.5 kPa with both free.** Values depend on the supply-tube length.
- **Amplitude:**
  - Jumps to several mm at threshold and then plateaus (reed organ, Cottingham 1999).
  - About 15 % of tongue length (Physics Today 2011 / Acoustics Today 2023).
  - Increases with pressure in the sho (Hikichi & Osaka 2002 abstract, DOI 10.1121/1.4778049), in contrast to the reed organ.
- **Extinction and hysteresis:** Tarnopolsky 2000, Misdariis 2000 (regime change), Koopman 1996, Cottingham 2019 (TP1 > TP2), Millot & Baumann 2007 (suspected inverse bifurcation). No accordion-specific extinction pressure was found.
- **Spectrum vs pressure:**
  - Reed-motion harmonics grow at high pressure (Puranik 2023; Cottingham 1999).
  - HF content of the sound rises with pressure (Hikichi et al. 2003 sho simulation and measurement).
  - Accordion mezzo-forte harmonics extend to ~10 kHz (Elejalde-García 2021).
  - Playing dynamic is ~40 dB (Misdariis 2000).

---

## 5. Coupling to cavities and resonators

### 5.1 Johnston, R. B. (1987). "Pitch control in harmonica playing." *Acoustics Australia* 15(3), 69–75
- URL: http://webdiis.unizar.es/~briz/harps/BendingPhysics.pdf (scanned)
- Access: open. **VERIFIED-FT** (pages 1–4 read as images).
- The mechanically blown harmonica is fed through a variable-length cylinder that stands in for the vocal tract. Bends are reproduced.
- Applies Fletcher's admittance model: the frequency satisfies tan φh = tan φv.
  - Closing reed: sounds just **below** its resonance.
  - Opening reed: sounds just **above** its resonance.
  - With both reeds, the pitch can sit anywhere between the two reed frequencies when the higher reed is the closing one. That is the bend.
- Fig. 6 caption parameters (scan): Xu = 0.2 mm, Mr = 0.01 g, ωr = 6300 rad/s, damping Dr = 0.05, P0 = 1.0 kPa, z = 0.5, b = 2.00 mm.
- Informs: coupling logic if RF-Musette models both reeds of a plate.

### 5.2 Bahnson, H. T., Antaki, J. F. & Beery, Q. C. (1998). "Acoustical and physical dynamics of the diatonic harmonica." JASA 103(4), 2134–2144. DOI 10.1121/1.421359
- **VERIFIED-ABS**
- Three speaking modes: closing (normal), bend (between the two reeds), and overblow/overdraw (opening, outside the interval).
- Methods: videostroboscopy and displacement gauges.
- Per Acoustics Today: vocal-tract shape rather than volume enables overblows. **Both reeds move.**

### 5.3 Millot, L., Cuesta, C. & Valette, C. (2001). "Experimental results when playing chromatically on a diatonic harmonica." *Acta Acustica united with Acustica* 87, 262–270
- HAL hal-04369502 (no PDF).
- **VERIFIED-ABS.** Strain gauges and a differential pressure transducer. **Reed motion is sinusoidal in all modes.**
- Millot's PhD thesis: Millot, L., *Étude des instabilités des valves : application à l'harmonica diatonique*, Université Paris 6, defended 1999-12-02, advisor Claude Valette. HAL tel-04370152 (no PDF). **VERIFIED-ABS.**
- **Adachi, S., Okada, H., Samejima, T. & Nishimiya, K. (2026).** "Time-domain simulation of harmonica pitch bending and overblowing." JASA 159(5), 4191–4203. DOI 10.1121/10.0043784. PMID 42138536. **VERIFIED-ABS** (PubMed).
  - Couples reed vibration, gap airflow and the mouth-side resonance.
  - Reproduces blow bend (hole 7), draw bend (hole 4) and overblow (hole 6), validated on a real instrument.
  - Derives small-amplitude self-excitation conditions.
  - **The newest validated free-reed time-domain model found.**
- Förtsch, A., "Linear stability analysis for bends and overbends on the blues harmonica" (2023). https://bluesharpscience.de/StabilityAnalysis.pdf. **UNVERIFIED** (self-published; not opened).

### 5.4 Cottingham, J. P. (2013). "Reed Vibration and Pitch Bending in Western Free Reed Instruments." CCRMA Music 318 course slides, 15 Feb 2013
- URL: https://ccrma.stanford.edu/courses/318/mini-courses/Winter2013/Cottingham-2.pdf
- Access: open. **VERIFIED-FT**
- **Accordion pitch bending (Coyle 2009 data):**
  - **Partial pallet opening:** less key depression gives a larger drop. Max bend **2.5 Hz at ~96 Hz**, "½ semitone".
  - **Reed-chamber volume reduced from ~17 to ~9 cm³:** 91.4 → 89.2 Hz, and ≈ 87.3 Hz with partial pallet. "Max bend 4.12 Hz, 4.5 %". Fig-read.
- Blown-closed reed on a pipe:
  - The pipe pulls the frequency **below** the reed frequency.
  - A 646 Hz reed tracks pipe length over ~400–750 Hz.
  - Mismatched pipes excite **modes 2 and 3** alone or together (542 / 2140 / ~5192 Hz).
- Harmonica external-pipe data (Brock & Cottingham 2011).
- Informs: pallet-throttling and chamber-volume pitch effects (musical "bellows shake" and "half-key" effects).

### 5.5 Elejalde-García, M. J., Macho-Stadler, E. & Llanos-Vázquez, R. (2021). "Accordion acoustics: A study on pitch bending." *Acoustics in Practice* AiP-2021-02, EAA
- URL: https://euracoustics.org/documents/14/AiP_2021_02_Accordion_acoustics.pdf
- Access: open. **VERIFIED-FT**
- Pigini Sirius concert accordion, A#2–A3, 8′ register out of cassotto.
- **Bend of 10–35 cents** (≈25 cents bellows-and-finger, ≈15 cents finger-only), harmonicity preserved.
- **No bend possible from C6 to C#8.**
- SPL falls more with finger-only bends.
- Explanation offered: throttling at the pallet reduces the energy reaching the reed.
- Earlier: Llanos-Vázquez, R., Elejalde-García, M. J. & Macho-Stadler, E. (2008), "Controllable pitch-bending effects in the accordion playing," JASA 123, 3662 (abstract), DOI 10.1121/1.2934981, **VERIFIED-ABS**. The Acoustics'08 full paper URL now returns 404.

### 5.6 Tonon, T.
- (2009), "Accordion reeds, cavity resonance, and pitch bend," JASA 126, 2217 (abstract), DOI 10.1121/1.3248806. **VERIFIED-ABS.**
  - In conventional accordions the cavities "have little effect on the vibration of the reed itself, because resonances between the reed and cavity are rarely encountered".
  - Near-resonant cavities can interfere with self-excitation.
  - Cavities can be designed to produce pitch bend.
- Also: Tonon, "Reed cavity design and resonance," *Papers of the Int. Concertina Assoc.* 2 (2005) (**VERIFIED-CIT**), and US Patent 5,824,927 (1998) (**VERIFIED-CIT**).
- **Cottingham, J. P. (2017).** "Reed chamber resonances in free reed instruments," JASA 141, 3960 (abstract), DOI 10.1121/1.4989008. **VERIFIED-ABS.** Chamber resonances are usually far above the reed frequency, so the effect is small. When close, they affect rise time and final amplitude or prevent sounding.

### 5.7 Upstream vs downstream cavity: what the literature says
- The **upstream compliance plus inertance** (reed chamber / wind chest volume plus the air mass feeding the slot) is part of the **excitation mechanism** itself (St. Hilaire; Ricot; Millot–Baumann; Euphonics).
  - For a blown-closed reed, its Helmholtz frequency must sit **below** the reed frequency (Millot–Baumann condition).
  - If it approaches the reed frequency it can **choke** the reed (Cottingham 2019; Tonon).
- A **downstream pipe** coupled to a blown-closed reed pulls pitch below both the reed and the pipe resonance (Cottingham 2009, "Blown-closed free reeds with and without pipe resonators," JASA 126, 2198, DOI 10.1121/1.3248598, **VERIFIED-ABS**). **Caution:** that abstract as indexed also says the Asian blown-open reed plays *below* both. This contradicts Fletcher 1979 (outward reeds run above), Cottingham 2000 (JASA 107, 2896, DOI 10.1121/1.428770: "slightly above") and Braasch & Cottingham 2023 ("greater than both"). Treat the 2009 abstract wording as an error.
- In Misdariis 2000's accordion rig, upstream and downstream resonators had **negligible influence on the spectrum**.
- Historical: Carrière, Z. (1924), "Entretien des vibrations d'une anche libre," *J. Phys. Radium* 5, 338–344, DOI 10.1051/jphysrad:01924005011033800 (**VERIFIED-FT**). A large steel reed (180 × 61 × 3 mm) on a pipe resonator. The wind box (non-tuned) *takes* energy from the reed and the tuned resonator supplies it; phase lag about ⅛ period. Relevant to pipe-coupled reeds, not the accordion.

### 5.8 Asian pipe-coupled reeds (context only)
- Hikichi, T., Osaka, N. & Itakura, F. (2003), "Time-domain simulation of sound production of the sho," JASA 113(2), 1092–1101, DOI 10.1121/1.1534605. **VERIFIED-ABS.** The sho reed acts as an outward-striking valve. HF content increases with blowing pressure.
- Hikichi & Osaka (2002), *Acoust. Sci. Tech.* 23(1), 25–27, DOI 10.1250/ast.23.25. Open, **VERIFIED-FT** (partial). Pipe resonances 380, 1130, 1890 and 2630 Hz, a 1:3:5:7 closed–open tube.
- Fetzer & Cottingham (1997), khaen, JASA 101, 3143, DOI 10.1121/1.419324, and Cottingham & Fetzer (1997), JASA 102, 3084, DOI 10.1121/1.420210 (admittance model predicts frequency vs pressure). **VERIFIED-ABS.**
- Bader, R., Linke, S. & Mores, R. (2019), "Measurements and impulse pattern formulation (IPF) model of phase transitions in free-reed wind instruments," JASA, DOI 10.1121/1.5136626. **VERIFIED-ABS** (volume/page not retrieved). Hulusheng/khaen phase transitions, modelled with the Impulse Pattern Formulation.

---

## 6. Theses and long reports

| Author | Title | Type, institution, date | Status |
|---|---|---|---|
| Millot, L. | *Étude des instabilités des valves : application à l'harmonica diatonique* | PhD, Univ. Paris 6, 1999-12-02 (dir. C. Valette) | VERIFIED-ABS |
| Ricot, D. | *Modélisation physique de la vibration d'une anche d'accordéon — application à la synthèse sonore* | DEA/end-of-studies report, IRCAM / ÉC Lyon, 1999 | VERIFIED-CIT |
| Singhal, V. K. | *Acoustics of ducts with flow and its relation to acoustically induced valve-pipe instabilities* | PhD, MIT, 1976 | VERIFIED-CIT (Millot & Baumann refs) |
| Pinelli, R. | *Accordéon diatonique : anthropologie, histoire, facture d'un instrument semi-industriel* | PhD, Univ. Côte d'Azur, 2024-12-03 | VERIFIED-ABS; organology, not physics |

**Not found:** any Italian (Ancona/Castelfidardo) engineering thesis on accordion-reed vibration, any Russian bayan-reed acoustics paper, or any Chinese sheng-reed physics thesis accessible online. Searches in Italian and Russian returned only trade or wiki material.

---

## (a) Proposed minimal physical model of one accordion reed, as the literature supports it

**Scope:** one blown-closed (−,+) steel reed on its slot, fed from a reed chamber that is connected through the pallet to the bellows. The second reed on the same plate is held by its valve and treated as inert for v1 (see open question 4).

### State variables
- q1(t): first bending-mode amplitude, normalised to tip displacement ξ from rest. Optional q2 (2nd bending) and qT (1st torsion) for the attack.
- u(t): volume flow through the gap region, a state because of inertance.
- p1(t): reed-chamber (upstream) overpressure. Optional pb(t) for the bellows volume.
- p2: pressure just upstream of the gap. Algebraic in Millot's form.

### Equations and sources

1. **Reed (modal oscillator):** ξ̈ + (ω0/Q) ξ̇ + ω0² ξ = µ Δp.
   - Sources: Millot & Baumann 2007, eq. 4; Tarnopolsky 2000, eq. 4 / A2 with µ = γ W L / m and γ ≈ 1.5 for mode 1.
   - Optional modes 2 and T1 with their own measured f and Q (Cottingham 2013/2019; Behrens 2009).
   - Distributed alternative: Puranik & Scavone 2023 (Euler–Bernoulli, 40 segments, fs = 176.4 kHz). Probably too costly per voice.
   - ω0: **measured** per reed (or beam theory as a first guess). Accordion reeds play slightly *below* ω0 (Misdariis 2000; Fletcher 1979).
   - Q: **assumed**. The literature range for metal reeds is ~50–95 (Tarnopolsky 55; Millot 95; reed-organ D = 0.012). **No accordion Q is published.**
   - Aerodynamic damping term: Tarnopolsky eq. 6 (β ≈ 1, assumed).

2. **Escape (useful) area Su(ξ):**
   - Millot & Baumann 2007 new model: front trapezoid plus integrated sides plus missing corners, with reed thickness and clearance hmin.
   - Fallback: Tarnopolsky eq. 2 with a(x) ≈ 0.6x0 + 0.4x (planar tongue, outside the slot only).
   - Accordion-specific: the plate is thick, so **the tongue never exits the far side** (Cottingham CCRMA slides). While the tongue is inside the slot, Su is clearance-limited. This is what produces the siren-like waveform.
   - Geometry (length, width, thickness profile, set/offset, clearance, plate thickness): **measured per reed; none published for accordion reeds.**

3. **Jet flow:** ut = α Su vj with Δp_gap = ½ ρ vj² (quasi-steady; Millot eqs. 6–7, justified by St ≈ 0.02–0.03).
   - α (vena contracta) = 0.61 for a sharp slit, range 0.5–1.0 (Tarnopolsky 2000). **Assumed / calibrated.**
   - Measured check: jet speed ≈ 15 m/s for an accordion reed (Misdariis 2000).

4. **Pumped flow:** up = Sr ξ̇ with Sr ≈ 0.4 W L for mode 1 (Tarnopolsky eq. 7; Millot eq. 5). Derived from geometry.

5. **Excitation by upstream flow inertia:**
   - Lumped: p1 − p2 = ρ (L2/S2) du/dt (Millot–Baumann; Puranik 2022). Gap-inertia alternative: Tarnopolsky eq. 3 term ∂/∂t(ρ U d/(C F)).
   - Physical basis: St. Hilaire 1971 (pressure ~ρU0ȧ); Ricot 2005 and Misdariis 2000 (upstream inertia, not jet instability).
   - L2 and S2 are **not measurable quantities**. They stand in for the near-field potential flow and must be **calibrated** (Puranik 2023 calls them arbitrary).
   - **Constraint:** the Helmholtz frequency of (V1, L2/S2) must be below ω0 for the (−,+) reed to oscillate (Millot–Baumann linear condition; Euphonics).

6. **Reed chamber compliance:** (V1/ρc²) dp1/dt = u_in − u − up (Tarnopolsky eq. 7; Millot / Puranik eq. 1).
   - V1 is measurable for a real accordion reed chamber (Coyle 2009 used ~9–17 cm³ inserts).
   - u_in comes from the bellows through the pallet as an orifice law (Puranik 2022 bellows chamber). Pallet opening is the "half-key" control (Coyle 2009; Elejalde-García 2021).

7. **Radiation:**
   - Dipole from gap-flow momentum fluctuation (d/dt of jet momentum flux) — Ricot 2005.
   - Plus monopole from total flow u + up — Tarnopolsky 2000; Misdariis 2000 used a monopole + dipole mix.
   - Mixing weights: **assumed/calibrated**.
   - Enclosure, cassotto and reed block belong to other modules (source-filter as in Puranik 2022).

### Constants: measured vs assumed

| Status | Constants |
|---|---|
| **Measured / measurable** | ω0; tip stiffness K (hanging-mass method, Cottingham ICA); effective mass m = K/ω0²; reed length, width, thickness profile, rest offset; clearance; plate thickness; chamber volume V1; free-decay Q (flick test, Tarnopolsky) |
| **From the literature, not accordion-specific** | α ≈ 0.61 (0.5–1.0); γ ≈ 1.5; Sr ≈ 0.4 W L; β ≈ 1 |
| **Calibrated** (no physical measurement defines them) | L2/S2 inertance (Millot, Puranik); monopole/dipole mix; possibly α |

### Validation targets from the literature
- **Pitch vs pressure:** only a few cents of drift over the normal 10–300 Pa accordion range. About −9 cents over 0.1–0.9 kPa for a 420 Hz accordion reed (Coyle, fig-read). About −10 to −13 cents/kPa at higher pressure (Misdariis, fig-read). A regime-2 rise when the equilibrium enters the slot. Small low-pressure rise and high-pressure rise (Cottingham 1999).
- **Displacement stays sinusoidal** from 10 Pa to 6 kPa (Misdariis). Mode-2 content grows at high pressure (Cottingham 1999; Puranik 2023).
- **Near-field pressure** is siren/square-like and rich in harmonics, with even partials weaker at low order (Misdariis; Braasch & Cottingham).
- **Hysteresis** between onset and extinction (Tarnopolsky; Cottingham 2019).
- **Dynamic range** ≈ 40 dB (Misdariis).
- **Attack:** finger attack 50–110 ms, bellows attack 190–660 ms (Llanos-Vázquez 2014). Harmonics enter progressively; small upward pitch glide during onset (Braasch & Cottingham 2023).
- **Pallet-throttle bend:** up to ~25–35 cents (Elejalde-García 2021).

---

## (b) Open questions the literature does not answer (as far as I could access)

1. **No published Q, stiffness, effective mass or thickness profile for steel accordion reeds**, and no reed-dimension tables across the accordion range. Ricot 2005 may hold some; its full text was not accessible. RF-Musette will need its own measurements (flick-decay Q, hanging-mass K, profile micrometry).
2. **Pitch-vs-pressure data for accordion reeds are sparse:** one IRCAM reed (~304 Hz) and one or two Coe College reeds (~96 and ~420 Hz), all fig-read. No register-wide dataset. No separation of the push and pull reed of the same pair.
3. **Accordion bellows and reed-chamber pressures in real playing** are almost unmeasured. The only figure is "10–300 Pa normal, mid register" (Misdariis 2000). No pp–ff pressure map, no time series of bellows pressure, no data for bass reeds.
4. **The second reed and its leather/plastic valve on the same plate:** it vibrates (Behrens 2009), but its effect on flow, pitch and spectrum, and the valve's own dynamics (flap noise, sealing), are unstudied in the physics literature I found.
5. **The accordion slot geometry's effect on the area function:** plate thickness, the 4–5° slot taper claimed by makers (forum source, UNVERIFIED), and the fact that the tongue never exits the far side. Millot's useful-section model has not been validated on accordion reeds. No PIV or CFD exists, and the FE attempts failed to reproduce periodic flow interruption (Kaufinger 2021).
6. **How to fix the "near-field" inertance (L2, S2) from geometry** rather than by calibration. Puranik 2024 moves towards continuous potential flow; that work is abstract-only.
7. **Attack physics:** the role of torsion and mode 2 in onset. The onset pitch glide ("semitones over dozens of ms") is qualitative only. Accordion reed growth rates vs pressure are unpublished (Biernat 2014 measured them on reed-organ reeds; values not in the abstract).
8. **Thresholds and extinction pressures for accordion reeds**, and the nature of the bifurcation. Millot suspects subcritical; this is not experimentally confirmed for accordion.
9. **Regime 2** (equilibrium inside the slot): which accordion reeds enter it in normal play, and at what pressures? Misdariis says "some reeds". This matters for ff realism.
10. **Temperature, humidity and ageing effects** on reed pitch: none found in the physics literature.
11. **Radiation directivity and monopole/dipole weights** for a reed inside a reed block and chamber. Ricot's FW-H result is for a free reed in open air.
12. **Choking thresholds** in accordion (not reed-organ) chambers. Tonon's cavity-design work is patent/abstract-level only.

---

## (c) The five sources to read in full first

1. **Ricot, Caussé & Misdariis 2005, JASA 117, 2279–2290** (DOI 10.1121/1.1852546). The only journal paper on the accordion reed itself: excitation mechanism, flow visualisation, FW-H radiation (dipole). Paywalled; get it through a library. Read Misdariis et al. CFA 2000 (open, 4 pp.) alongside it for the pressure numbers.
2. **Millot & Baumann 2007, Acta Acustica 93, 122–144** (arXiv 2401.01606, open). The model backbone: equations, escape-area definition, instability condition, simulation algorithm (appendix), harmonica constants.
3. **Tarnopolsky, Fletcher & Lai 2000, JASA 108, 400–406** (open at UNSW). The validated lumped model with every coefficient: contraction, pumped flow, area function, aerodynamic damping, measured Q, hysteresis and pressure jump.
4. **St. Hilaire, Wilson & Beavers 1971, J. Fluid Mech. 49, 803–816** (DOI 10.1017/S0022112071002374), with St. Hilaire & Vaidya 1975 for the limit amplitude. The foundational measured growth-rate vs flow data and the potential-flow excitation term. Paywalled.
5. **Puranik & Scavone 2022 (POMA 49, 035015) plus 2023 (Forum Acusticum, open).** The closest precedent to RF-Musette: a bellows-driven harmonium synthesiser built on Millot–Baumann. It documents the numerical pitfalls ("unrealistic parameters" needed for stability) and the distributed-beam fix.

Honourable mentions, cheap and open:
- Cottingham ICA 2019 (transients, choking).
- Cottingham CCRMA 2013 slides (accordion pressure and chamber data).
- Llanos-Vázquez 2014 (accordion attack times).
- Fletcher 1993 (valve classification).
