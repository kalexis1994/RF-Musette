# Sources and credits

RF-Musette is built on other people's measurements and models. This is the
ledger of every work it has read: what was taken from each, where in the
code it lives, and how much of it was read -- **in full**, from an
**abstract**, from **slides** or a **summary**, or read off a **figure**.
Nothing here was copied as code or data; equations and numbers were
re-derived or cited, and each is credited again beside the code that uses
it. The survey behind the project, with every source found whether used or
not, is [RESEARCH.md](RESEARCH.md) and the notes under
[research/](research/).

If you are one of these authors and something here misstates your work,
please say so: the ledger is meant to be accurate.

## The reed

| Work | What RF-Musette takes from it | Where | Read |
| --- | --- | --- | --- |
| L. Millot, C. Baumann, "A proposal for a minimal model of free reeds", *Acta Acustica united with Acustica* 93 (2007) 122-144; arXiv 2401.01606 | The model's backbone: the one-mode tongue (eq. 4), pumped and jet flow and the Bernoulli jet (eqs. 5-7), the useful section integrated along the edges with the tongue's thickness (eq. 9, appendix 5.1), μ = S_r/M and S_r = W∫ψ (appendix 5.2), the minimal configuration and its instability condition (§3.1, appendix 5.3); their report that the (−,+) reed's pitch barely moves with excitation | `reed.rs` | In full |
| G. Ziegenhals, "Schallabstrahlung und Biegeschwingungen von Tonzungen und Stimmplatten", Institut für Musikinstrumentenbau Zwota (2009) | The accordion F4 reed itself: tongue 36 × 4 mm, set 0.5 mm, plate 3 mm, modes at 355 and 1645 Hz (1 : 4.6), a swing above 4 mm at mf; the three phases of the escape area and the slot's area as its ceiling; the sound as the derivative of the volume flow | `parameters.rs`, `reed.rs`, `tongue.rs` | In full |
| A. Z. Tarnopolsky, N. H. Fletcher, J. C. S. Lai, "Oscillating reed valves -- an experimental study", *JASA* 108 (2000) 400-406 | The jet's contraction coefficient, 0.61 (range 0.5-1); the inertia of the air inside the gap (eq. 3), tried as the missing inertia and refuted | `parameters.rs`; `tests/diagnosis.rs` history | In full |
| T. Tonon, "Reed cavity design and resonance", *Papers of the International Concertina Association* 2 (2005) | The cell as a Helmholtz resonator, the hole's end correction k = 0.43-0.80, and when a cavity chokes or helps a reed | `reed.rs`, `parameters.rs`, `milestone_2.rs` | In full (web edition) |
| N. Misdariis, D. Ricot, R. Caussé, "Modélisation physique de la vibration d'une anche d'accordéon", 5e Congrès Français d'Acoustique (2000) 281-284 | Normal playing pressure 10-300 Pa for a mid-register reed; sinusoidal tongue motion; about 40 dB of dynamic range; the two regimes; excitation by the inertia of the upstream fluid; Fig. 3's pitch against pressure, digitised (−13.6 cents/kPa); the upstream flow as a 2-D sink with pressure on the upstream face only, from which milestone 2b's second attempt derives its suction (A) and sink inertia (B) | `milestone_1.rs` bounds; MODEL.md; `tests/sink_flow.rs`; [notes](research/MISDARIIS-2000.md) | In full, text and figures |
| D. Ricot, R. Caussé, N. Misdariis, "Aerodynamic excitation and sound production of blown-closed free reeds without acoustic coupling: the example of the accordion reed", *JASA* 117 (2005) 2279-2290 | The excitation mechanism (inertia of the unsteady flow through the gaps) and dipole radiation of a bare reed | `reed.rs` docs; MODEL.md | Abstract |
| A. O. St. Hilaire, T. A. Wilson, G. S. Beavers, "Aerodynamic excitation of the harmonium reed", *J. Fluid Mech.* 49 (1971) 803-816 | The excitation of a free reed by unsteady potential flow | `reed.rs` docs | Abstract |
| A. O. St. Hilaire, P. G. Vaidya, "Finite amplitude analysis of a flow-structure interaction problem", *J. Fluid Mech.* 67 (1975) 377-396 | Nonlinear dissipation by the flow's higher harmonics as what limits the swing | MODEL.md, known defects; ROADMAP 2c | Abstract |
| M. Nussbaumer, A. Agarwal, "Aeroacoustics of free reeds", *22nd International Congress on Acoustics*, Buenos Aires (2016), ICA2016-756 | A measured reed's swing and where it enters the slot (Fig. 5: ±0.72 mm, closing 0.4 mm below its mean); the mean moving toward the plate with flow; the fundamental's level and harmonics against mean flow (Fig. 6), the target of 2c; modal Q of 200-400 on real reeds, beside the assumed 250 | ROADMAP 2c; `tests/saturation.rs` | In full |
| G. H. Keulegan, L. H. Carpenter, "Forces on cylinders and plates in an oscillating fluid", *J. Res. Natl. Bur. Stand.* 60 (1958) 423-440 | The drag coefficient of an oscillating flat plate against KC (Table 4, p432), used as the tongue's drag in milestone 2c's experiment | `tests/saturation.rs` | In full |
| S. Bidkar, M. Kimber, A. Raman, A. Bajaj, S. Garimella, "Nonlinear aerodynamic damping of sharp-edged flexible beams oscillating at low Keulegan–Carpenter numbers", *J. Fluid Mech.* 634 (2009) 269-289 | Taking a plate's drag along a cantilever with the local KC(x) = 2πÂψ(x)/c (eqs. 3.10, 3.14); their own coefficients (Fig. 8) cover KC ≤ 5 and were not used | `tests/saturation.rs` | In full (accepted manuscript, Purdue e-Pubs, via the Wayback Machine) |
| M. Luhar, H. Nepf, "Wave-induced dynamics of flexible blades", arXiv 1510.01237 | The fit C_D = 10 KC^(−1/3) to Keulegan & Carpenter's plates, after Graham (JFM 97, 1980, abstract only): a cross-check of the table at this reed's KC | ROADMAP 2c | In full (the 15 % checked by us, p3) |
| J. Braasch, J. P. Cottingham, "Free reeds: an intertwined tale of Asian and Western musical instruments", *Acoustics Today* (2023) | A free reed's tip commonly swings about 15 % of its length (p3): ~5.4 mm here, the target for 2c | ROADMAP 2c | In full (the 15 % checked by us, p3) |
| C. C. Darabundit, G. Scavone, "Discrete port-Hamiltonian system model of a single-reed woodwind instrument", *Frontiers in Signal Processing* (2025) | The idea of a Bernoulli port kept dissipative by construction and solved without iteration; RF-Musette's scheme is derived separately for its own equations | `reed.rs` | In full (by the research survey) |
| J. Woodhouse, "11.6 Free reeds", *Euphonics* (online book) | A free reed's measured Q ≈ 250, used as the assumed Q | `parameters.rs` | Summary |
| J. P. Cottingham, C. H. Reed, M. Busha, "Variation of frequency with blowing pressure for an air-driven free reed", Forum Acusticum / 137th ASA, Berlin (1999) | Growth and damping rates against pressure (Fig. 4: at most 11.3 /s near 1 kPa on a reed-organ C3), against which the model's growth per cycle was compared | MODEL.md, VALIDATION.md | In full |
| J. P. Cottingham, C. J. Lilly, C. H. Reed, "The motion of air-driven free reeds", Forum Acusticum / 137th ASA, Berlin (1999) | Sinusoidal tongue motion; the swing jumping to several millimetres at onset and then saturating; the equilibrium's shift under oscillation | MODEL.md, known defects | In full |
| J. P. Cottingham, "A survey of recent studies on initial transients in free reed oscillation", 23rd ICA, Aachen (2019) 5530-5535 | Attacks beginning with the tongue displaced into its frame; the second transverse and first torsional modes in the first cycles; choking by a chamber tuned near the reed (344 against 268 Hz on a 327 Hz chamber) | `milestone_2.rs`; MODEL.md | In full |
| J. P. Cottingham, "Reed chamber resonances and attack transients in free reed instruments", 22nd ICA, Buenos Aires (2016) | Onset and offset thresholds, 60-110 Pa for a 622 Hz accordion reed; onset always above offset | `milestone_1.rs` bounds | In full (by the research survey) |
| J. P. Cottingham, "Reed vibration and pitch bending in Western free reed instruments", CCRMA Music 318 slides (2013), summarising W. L. Coyle, S. L. Behrens, J. P. Cottingham, *JASA* 126 (2009) 2216 | Accordion pitch against pressure (about −9 cents over 0.1-0.9 kPa, fig-read); laboratory chamber volumes of 8-18 cm³ | `parameters.rs`, `milestone_1.rs` | Slides |
| S. Behrens, W. Coyle, N. Goodweiler, J. P. Cottingham, "Vibrational modes of accordion reeds", *JASA* 126 (2009) 2216 | Accordion tongues carry transverse modes to the fourth and a torsional mode | MODEL.md | Abstract |
| N. Puranik, G. Scavone: "Physical modelling synthesis of a harmonium", *PoMA* 49 (2022); DAFx-23; "Clamped bar model for free reeds", Forum Acusticum 2023 | The only published bellows-driven free-reed synthesis, and the instability that ended it | RESEARCH.md; the reason the scheme is passive | DAFx-23 and FA 2023 in full |
| N. H. Fletcher, "Autonomous vibration of simple pressure-controlled valves in gas flows", *JASA* 93 (1993) 2172-2180 | The (−,+) / (+,−) classification of reed valves | `reed.rs` docs | In full (by the research survey) |
| J. W. S. Rayleigh, *The Theory of Sound* | The flanged end correction, 8/(3π)·√(S/π), behind the near-field inertance | `reed.rs` | Standard result |

## The instrument

| Work | What RF-Musette takes from it | Where | Read |
| --- | --- | --- | --- |
| R. Llanos-Vázquez, M. J. Elejalde-García, E. Macho-Stadler, A. Agos-Esparza, "Physical and psychoacoustic characterization of the different types of attacks on the accordion", *Acta Acustica united with Acustica* 100 (2014) 375-384 | Finger and bellows attack times, note by note (Table I), and how they were measured; p ≈ 55 and mf ≈ 70 dBA at 50 cm | `milestone_2.rs`; MODEL.md | In full |
| M. J. Elejalde-García, E. Macho-Stadler, R. Llanos-Vázquez, "Accordion acoustics: a study on pitch bending", *Acoustics in Practice* AiP-2021-02 (2021) | Players' pitch bend, 15-35 cents; valves leather to G4, plastic to C6, none above | `milestone_2.rs`; ROADMAP | In full (by the research survey) |
| R. Llanos-Vázquez et al., 2002 (as summarised in the research notes) | Steel tongues on duralumin plates; 40-50 dB of dynamic range at 1 m | `parameters.rs` | By the research survey |
| R. Llanos-Vázquez, *Acústica del acordeón*, PhD thesis, Universidad del País Vasco UPV/EHU (2015), supervised by M. J. Elejalde-García and E. Macho-Stadler; ISBN 978-84-9082-294-4, hdl 10810/16562 | The attack's growth (Fig. 4.11, p150: the first harmonic rising ~0.7-0.8 dB/ms in a finger attack); a normal finger attack executed in about 0.05 s (p164), used as the pallet's opening time; the attack-time tables and their definition (pp91, 118-121, 151-174); pitch against dynamics (Table 4.12, p185); the part-pressed button's level drop (Table 4.11, p184) and bend (pp197-208); its summary of Fletcher's and of Ricot, Caussé & Misdariis's models (Appendix 1, pp231-252) -- the flow's inertia in the plate's own channel, and the pressure peak as the tongue enters the slot -- which sets milestone 2b; the luthiers' interviews (pp262-280) on clearance, set, plate thickness and cells. Notes: [research/LLANOS-2015.md](research/LLANOS-2015.md) | `parameters.rs`; ROADMAP 2b; MODEL.md | In full (the user's copy), with figures read off renders |
| J. Alberdi, K. Baraiazarra, J. M. López, J. Orobengoa, accordion builders, personal communications to R. Llanos-Vázquez (2005-2015), as reported in his thesis | "The smaller the clearance, the faster the response" (p46); low reeds set slightly higher to speak sooner (p153) | ROADMAP 2b predictions | As reported |
| J. Braasch, C. Ahrens, "Attack transients of free reed pipes in comparison to striking reed pipes and diapason pipes", *Acta Acustica united with Acustica* 86 (2000) 662-670 | The −50 dB to −5 dB attack-time definition Llanos-Vázquez uses and RF-Musette measures with | `rf-musette-analysis` | As cited by Llanos-Vázquez |
| Harmonikas.cz, reed plate specifications (maker's web page) | Machined clearances: 0.03 mm at the rivet, 0.04 mm at the tip | `parameters.rs` | Web page |

## Not yet used in code

Read for the milestones ahead and credited in [RESEARCH.md](RESEARCH.md):
F. Hergert on detuned unisons (Forum Acusticum 2023; *Acta Acustica* 8, 2024);
V. G. Porvenkov on optimum beat rates (1979); G. Richter on cassotto and
grille resonances (IfM Zwota, 1989); J. Ramos, E. Calcagno, P. Riera et al.
on the Bandoneon 2.0 measurements (NIME 2022, 2023; *Computer Music Journal*
46, 2022); J. Braasch and J. P. Cottingham (*Acoustics Today* 19, 2023); and
the reference-recording survey (TinySOL, IRCAM).
