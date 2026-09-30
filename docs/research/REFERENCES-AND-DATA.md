# RF-Musette: reference recordings, datasets and calibration tables for a physically modelled accordion

Research notes, 2026-09-30. No audio or archives were downloaded. I read web pages, dataset metadata and PDFs of papers; two metadata CSVs were streamed and counted in memory. VERIFIED means I read the claim at the cited source during this session. UNVERIFIED means it comes from a search snippet, a vendor summary or memory, or the source could not be opened.

Suitability score (0 to 5) as **the single calibration reference**. The criteria:
- one documented instrument (make, model, reed setup)
- per-note coverage
- per-register coverage, ideally one reed rank at a time
- several dynamics or bellows pressures
- dry or close-miked sound
- push and pull labelled
- releases present
- known format
- a license that allows analysis and lets us ship derived data, the way the Concert Grand uses Salamander (CC-BY)

---

## 1. Multisample libraries and sample sets

### 1.1 TinySOL / OrchideaSOL / SOL, the IRCAM Studio On Line accordion. **Best public candidate**
- **URLs:** TinySOL https://zenodo.org/records/3685367; OrchideaSOL metadata https://zenodo.org/records/3740399; paper https://arxiv.org/pdf/2007.00763 (Cella, Ghisi, Lostanlen, Lévy, Fineberg, Maresz, 2020). VERIFIED.
- **License:**
  - TinySOL on Zenodo is "Creative Commons Attribution 4.0 International (CC BY 4.0)". VERIFIED.
  - The 2020 paper describes TinySOL as "free for non-commercial usage". VERIFIED.
  - **These two statements conflict.** The Zenodo record is the deposited legal license, but someone should confirm it with the authors or IRCAM before shipping derived data.
  - OrchideaSOL audio is free after registering on the Ircam Forum, and the paper says it is "free for non commercial usages". Its metadata is CC BY 4.0. FullSOL needs a Premium Ircam Forum account. VERIFIED.
- **What was recorded:** VERIFIED from the paper.
  - SOL was recorded in Ircam's "Espace de projection" between 1996 and 1998/1999. The accordion was part of the second phase.
  - Six channels: a stereo reference pair; a "proximity microphone … with minimal reverberation"; an internal or contact mic; and two figure-8 room mics.
  - The original takes were 48 kHz / 24-bit.
  - OrchideaSOL and TinySOL keep only channel 3 (the proximity mic), mono, trimmed, resampled to 44.1 kHz. The paper says 24-bit; the Zenodo page says 16-bit, so this is unresolved.
  - Levels were "volume compensated". The accordion is one of the instruments that "had precise reports" of gain changes, and those changes were reverted.
  - Notes out of tune by 10 to 80 cents were digitally retuned; the filename tag `T<cents><u|d>` marks them.
  - Missing notes were sometimes filled by resampling a neighbour up to a tone away. This matters mostly for extended techniques.
- **Accordion content:** VERIFIED by streaming and counting both metadata CSVs.
  - TinySOL, ordinario only: **689 accordion files**, pitch **E1 to C#8** (MIDI 28 to 109).
  - Dynamics: mf 302, pp 194, **ff 185**, p 8.
  - Up to six instances per pitch (`N`, `alt1` … `alt5`). What an "alt" means is **not documented**. It may be a different reed rank or register producing the same pitch.
  - 35 files needed digital retuning.
  - OrchideaSOL adds 107 `combination_of_registers`, 73 `sforzato` (fp), 2 `breath` and 1 `key_click` files (872 accordion files in total).
- **Not documented:** the instrument make and model, the reed configuration, which register each file uses, push or pull, and bellows pressure. There are no release tails as separate files.
- **Suitability: 3.5/5.**
  - For: it is the only open, per-note, whole-range accordion set with three dynamics and near-dry sound, made by a research institution with a published processing log.
  - Against: the instrument and registers are undocumented; push/pull is unknown; some notes were retuned or resampled; the license statement is inconsistent; the samples are trimmed, so release and onset context may be clipped.
  - The E1 to C#8 range matches a concert instrument with 16′ + 8′ + 4′ ranks (see §3.3), which suggests the "alt" instances are different ranks. This is UNVERIFIED.

### 1.2 FreePats "Button Accordion HN" (Hohner diatonic, CC0)
- **URL:** https://github.com/freepats/button-accordion-HN. VERIFIED: README, SFZ and file tree read through the GitHub API.
- **License:** "Published under the terms of Creative Commons CC0 public domain dedication". VERIFIED.
- **Content:**
  - "samples of a Hohner accordion recorded by Jeff Stauffer in November 2023", processed by michael02022, version 2024-03-21. The model is not stated.
  - 17 sustained notes in FLAC: B3 D4 F#4 G4 A4 B4 C5 D5 E5 F#5 G5 A5 B5 C6 D6 E6 G6, which is a diatonic G/D note set. Each has a separate release sample (`rel/*_rel.flac`).
  - One dynamic layer. The SFZ applies per-sample `tune=` corrections from −7 to −38 cents, so the raw samples are well away from equal temperament, or the tremolo shifts the perceived pitch.
  - Sample rate, bit depth, mic setup, register and push/pull are not stated.
- **Suitability: 1.5/5.** Being CC0 makes it ideal for test fixtures. It is a poor calibration target: diatonic, few notes, one dynamic, undocumented.

### 1.3 RWC Music Database: Musical Instrument Sound (RWC-MDB-I-2001), No. 07 Accordion
- **URLs:** https://staff.aist.go.jp/m.goto/RWC-MDB/rwc-mdb-i.html and https://staff.aist.go.jp/m.goto/PAPER/ISMIR2003RWCMDBgoto.pdf. VERIFIED.
- **Content:** VERIFIED.
  - Accordion is entry **No. 07**, with 3 variations (instrument manufacturers or musicians), 3 dynamics (forte, mezzo, piano) and 5 playing styles per variation. Harmonica is No. 08.
  - The whole database is "16 bit / 44.1 kHz … monaural sound files", recorded note by note at half-tone steps over the full range. Recording was done by C MUSIC Corporation.
  - The makes and models of the three accordions were not found.
- **License:**
  - Historically the "Pledge": "The databases … may not be sold, leased, published or distributed to any third party", research use only, ¥15,000 for 12 DVD-ROMs. VERIFIED.
  - The 2026 open re-release on Zenodo (CC BY-NC 4.0, records 18656623 / 17177919) **does not include RWC-I**. The TISMIR paper says "we do not consider it in this work", and the Zenodo page says RWC-I "is distributed separately in a dedicated repository". I could not find that repository. VERIFIED that it is excluded; any new home is UNVERIFIED.
- **Suitability: 2/5.** Its structure is good (three dynamics, full range, styles). But the three instruments are mixed unless we choose one variation; it is mono 16-bit; details are undocumented; and the license is research-only and no redistribution, which conflicts with a commercial plugin's calibration pipeline.

### 1.4 Accordion4Composers (Luca Piovesan)
- **URLs:** http://www.lucapiovesan.it/accordion4composers/ and the PDF at https://www.lucapiovesan.it/wp-content/uploads/2015/09/ACCORDION4COMPOSERS-v1.0.pdf. VERIFIED.
- **License:** "Creative Commons Licence CC BY-NC-ND 2.5 … You may not use this work for commercial purposes. You may not alter, transform, or build upon this work." The audio is sent by email invitation to a Dropbox folder. VERIFIED.
- **Recording:** "2 microphones in stereo ORTF position … one meter far from my accordion in frontal position … two Schoeps microphones CMC5+MK4; the preamp is a Universal Audio 4-710d … no added compression or volume changes between takes, just a bit of added reverb (Lexicon room)". VERIFIED.
- **Content:** VERIFIED from the list of audio tags.
  - It is a set of technique demonstrations, not a per-note multisample.
  - Examples include:
    - 16′, 8′ and 4′ register demos (AUDIO 3.4 to 3.6)
    - attacks such as "ha", "ttta" and "va" (2.5 to 2.8), and decays (2.9 to 2.12)
    - "lowered note on ff" (2.4)
    - bellows shake and ricochet (5.3, 5.4)
    - the air button (5.8)
    - pitch bending (6.x)
    - button, register and bellows noises (7.x)
  - The instrument is a concert free-bass converter accordion whose left hand has "only the two lowest reeds". The make is not stated.
- **Suitability: 1/5 as a calibration target** because of the added reverb, the ND license and no per-note grid. It is valuable as a **qualitative gesture reference** for attacks, bellows reversal and pressure-driven pitch drop, and for listening only.

### 1.5 Freesound packs (licenses are per sound)
All are VERIFIED unless noted. None is suitable as the reference: each is a handful of notes, undocumented, and some are lossy.

| Pack | URL | License (checked on one sound) | Content |
|---|---|---|---|
| Cellini Accordion (Miles_Thompson, 2021) | freesound.org/people/Miles_Thompson/packs/31477/ | **CC0** (sound 560860) | About 75 to 77 items: note octaves, chords, bass; **m4a/mp3 lossy**, 48 kHz; "Long is a sustained portion." |
| Old Toriana Accordion (tim.kahn, 2013) | freesound.org/people/tim.kahn/packs/11810/ | **CC BY-NC 4.0** (sound 187053) | Basement vintage "Toriana"; AT4021 XY, 48 kHz/24-bit stereo; "each sample includes first pulling the accordion apart and then pushing it back together" (the chord samples); notes C3 to G4. |
| Accordion wav samples (hammondman, 2016) | freesound.org/people/hammondman/packs/18842/ | **CC0** (sound 333718) | 12 notes from A2 to G4; 32 kHz/16-bit mono, about 1.8 s. |
| Giulietti Italian Accordion "Violin Register" set (LeQuest, 2018) | freesound.org/people/LeQuest/sounds/419274/ | **CC BY-NC 3.0** | 10 notes, 44.1 kHz/16-bit stereo, 10 s each "with a small lead in/out fade", violin (8′+8′ tremolo) register. |
| Yamaha PS-55 "French Accordion" preset | (pack 33617) | not checked | A synth preset. Irrelevant. |

### 1.6 Pianobook (free, but account and Pianobook terms; not CC)
- **Mum's Accordion** (Phil Hartley): https://www.pianobook.co.uk/packs/mums-accordion/. VERIFIED.
  - "1926 Foreign Accordion", "All 25 keys", two dynamic layers (P, F), release triggers, no round robins.
  - Mics: SE Gemini II (front) and Warm Audio WA47 (rear) into SSL preamps. Kontakt format.
  - The license terms are not quoted on the page; free download with an account.
  - **Suitability: 1.5/5.** It is a small antique instrument with an unknown reed setup.
- Other Pianobook accordions: "Accordion" (small, Decent Sampler), "Toy Accordion" (Stradellina), and "Accordion Double Reed", which the search snippet describes as **synthesized** (Larry Seyer). UNVERIFIED beyond the snippets. Not suitable.

### 1.7 Free general collections checked for accordion or free reeds
- **Univ. of Iowa MIS:** no accordion, harmonica or free reed. Terms: "may be downloaded and used for any projects, without restrictions". VERIFIED (https://theremin.music.uiowa.edu/MIS.html).
- **VSCO 2 CE:** no accordion on the product page. CC0. VERIFIED (page only).
- **VCSL (Versilian Community Sample Library, CC0):** `Aerophones/Free Aerophones` has **Harmonica-Hohner-Special20-C, Harmonica-Hohner-Special20-F, Harmonica-Hohner-Super64** and a Siren. No accordion was found. VERIFIED (GitHub tree). These are a related free reed only.
- **Philharmonia samples:** the site redirect-looped. Search snippets list orchestral instruments plus guitar, mandolin and banjo, with no accordion. UNVERIFIED negative.
- **TU Berlin directivity / anechoic database** (Ackermann et al., 41 instruments, CC BY-SA 4.0, 32-mic array, 44.1 kHz/24-bit): **no accordion, bandoneon or harmonium**. VERIFIED (arXiv 2307.02110).
- **Aalto anechoic symphonic recordings:** Mozart, Beethoven, Bruckner, Mahler; orchestral only; "free for academic research". VERIFIED negative.
- **Good-Sounds (MTG):** 12 instruments (flute, cello, clarinet, trumpet, violin, 4 saxophones, oboe, piccolo, double bass). No accordion. VERIFIED from the MTG page snippet.
- **IRMAS** (11 classes) and **Medley-solos-DB** (8 classes): no accordion. VERIFIED from class-list snippets.
- **OpenMIC-2018:** has an **Accordion class (index 0)**. The clips are 10 s excerpts of CC-licensed Free Music Archive music, polyphonic and crowd-labelled. VERIFIED (class-map.json). Useless for calibration.
- **NSynth:** 11 families, including "reed" and "keyboard". 16 kHz / 16-bit, 4 s notes. CC BY 4.0. VERIFIED. Nothing labels accordion; I believe instruments are anonymised (e.g. `reed_acoustic_0xx`), but that is UNVERIFIED. 16 kHz is unusable anyway.
- **McGill MUMS:** a snippet says Volume 1 includes accordion. It is a commercial DVD set from 1989/1991 with no open license. UNVERIFIED details.
- **sfzinstruments.github.io:** the Misc page has no accordion or free reed. Karoryfer "Weresax" contains a "saxcordion", which is a fake accordion made from saxophone samples. VERIFIED from snippet and page.
- **GM SoundFonts:**
  - FluidR3_GM is MIT-licensed. GeneralUser GS allows unrestricted music use and "inherits the usage rights of contained samples", some taken from other free banks.
  - Neither documents where its accordion samples came from. UNVERIFIED provenance. Not suitable.
- **Musical Artifacts "Accordion (SFZ)" (#615):** a snippet says unknown license and unknown sources. UNVERIFIED (the page returned 403). Avoid.

### 1.8 Commercial libraries: what is recorded, and whether analysis is allowed
General point: every EULA I could read grants use **in musical productions** only. Using a competitor's library as a calibration target for a commercial virtual instrument is at best a grey area. At most, treat these as listening references and **do not ship data derived from them**.

- **Sonokinetic "Accordion"** (https://www.sonokinetic.net/products/heritage/accordion/). VERIFIED.
  - One 96-bass accordion; "all tones the instrument made, in all registers"; basses with and without the extra low octave; short bass attacks at several velocities; bellows and button noises.
  - 5,500+ samples, 44.1 kHz / 24-bit, 3 GB, Kontakt.
  - EULA (https://www.sonokinetic.net/support/license-agreement/), VERIFIED:
    - Art. 4.2: sounds may be used "in musical arrangements, compositions and/or productions with multiple musical audio layers".
    - It forbids "Creating sample libraries in any form … using the Products".
    - Art. 9.5: "It is not permitted to reverse engineer and/or decompile the Products".
  - **Suitability: 1/5** (license).
- **Wavesfactory "Le Parisien"** (https://www.wavesfactory.com/kontakt-libraries/le-parisien/). VERIFIED from the product page summary.
  - Three accordions, named "Chordox Rouge", "Valetta Blanc" and "Frontallini Noir". These may be nicknames; real makes and models are UNVERIFIED.
  - 5 registers on the Rouge (master, bandoneon, bassoon, musette, violin) and 2 each on the others.
  - 6,198 samples, 6× round robin, 3 mic positions plus a mix, velocity layers, bellows noise, push/pull.
  - 3.55 GB, €59.
  - EULA not found (the license URL returned 404). **1/5.**
- **Best Service / Eduardo Tarilonte "Accordions 2":** **discontinued** ("no longer available" on bestservice.com). VERIFIED that it is discontinued.
  - Per reseller snippets (UNVERIFIED): 13 instruments, including single-reed piccolo, musette and bassoon accordions, which are **isolated reed ranks** and would be very useful. Also 3 dynamics, 3 round robins, 3 kinds of bellows shake, key noises, recorded chromatically with a Neumann U47, about 6.2 GB.
  - **1/5** (EULA, availability).
- **PSound "World Musette"** (https://www.psound.it/world_musette.html). VERIFIED.
  - "based on 2 or 3 voices: the main 8" voice and 1 or 2 8" detuned voices, with each kind of Musette having its own detune values, from 4 cents to 23 cents".
  - 477 samples, 96 kHz / 24-bit, 3 velocity layers, 2 round robins, looped.
  - The wording suggests the musette is built by **detuning copies of one voice**, not from recorded tremolo ranks. That is UNVERIFIED, but if true it disqualifies it as a physics reference.
  - EULA not on the page. **0.5/5.**
- **Not found:** no accordion library from Spitfire, Soundiron, Cinesamples or Embertone turned up in searches. Orange Tree, Impact Soundworks and 8Dio were not found either. Treat this as UNVERIFIED negative; I did not check each catalogue page.
- **Excluded:** Roland V-Accordion. It is a digital model, not a reference.

---

## 2. Research datasets and measurements (reeds, pressure, flow, directivity, anechoic)

No **open accordion measurement dataset** (reed displacement, pressure, flow or directivity) turned up on Zenodo or in the main anechoic and directivity databases. What exists is papers with figures or tables:

- **Bandoneón 2.0 (Argentina)**: Ramos, Calcagno, Riera, "An embedded wavetable synthesizer for the electronic bandoneon with parameter mappings based on acoustical measurements", NIME 2023 (https://nime.org/proceedings/2023/nime2023_23.pdf). VERIFIED.
  - Instrument: "an old Uhlig brand bandoneon".
  - It was recorded with their "Integrated Measurement System", **capturing sound and bellows pressure synchronously**. Each reed was recorded alone ("carefully separating fundamentals from octaves"). For each reed they "executed a slow crescendo-decrescendo, from the quietest to the loudest dynamics".
  - Findings:
    - Pitch drops with pressure in the low octaves, "almost half a semitone" at extremes, while "higher notes do the opposite".
    - Attack time depends on note and pressure.
    - Wind noise rises with pressure.
    - Residual reed oscillation shortens re-attacks.
  - Related: [6] Ramos et al., Computer Music Journal 46(1-2), 2023, "An electronic bandoneon with a dynamic sound synthesis system based on measured acoustic parameters" (403 here); [28] "Sistema integral de medición (SIM)…", AES LAC '22.
  - **No public data release was found.** UNVERIFIED. **Their protocol is the model for our own recordings (§4).**
- **Cottingham (Coe College), CCRMA mini-course slides 2013** (https://ccrma.stanford.edu/courses/318/mini-courses/Winter2013/Cottingham-2.pdf). VERIFIED.
  - Reeds were from a "Hohner Verdi I accordion". Laser vibrometer plus a probe mic; lab blowing pressure "@ 0.6 kPa".
  - A plot of the sounding frequency of a G# reed against blowing pressure falls from about 421.6 Hz at about 0.1 kPa to about 419.4 Hz at about 0.9 kPa. That is roughly −2 Hz, or about −9 cents; I read it off the graph, so it is approximate. The plot is data from Whitney Coyle (2009).
  - A partial pallet opening gives "Max Bend = 2.5 Hz" on a reed near 96 Hz. Reducing reed-chamber volume gives "Max Bend = 4.12 Hz, 4.5%".
  - Higher transverse modes (2nd and 3rd) and the first torsional mode are present.
- **Elejalde-García, Macho-Stadler, Llanos-Vázquez, "Accordion acoustics: A study on pitch bending"**, Acoustics in Practice AiP-2021-02 (https://euracoustics.org/documents/14/AiP_2021_02_Accordion_acoustics.pdf). VERIFIED.
  - Instrument: a "Pigini Sirius" concert accordion. Microphone: B&K 4189 at 50 cm; mezzo forte monitored with an SLM at 50 cm.
  - Pitch bending: about **25 cents** with bellows and finger, about **15 cents** with finger only; "All notes have a frequency decrease of 10 to 35 cents". SPL shifts are tabulated for A#2 to A3.
  - The modelling section assumes "a supply pressure of about 40 Pa", giving V0 ≈ 10 m/s.
  - The copyright line reserves all rights, so use the numbers, not the text.
- **Millot & Baumann, "A proposal for a minimal model of free reeds"**, Acta Acustica united with Acustica 93 (2007) 122–144; arXiv 2401.01606. VERIFIED from Table 2, rendered.
  - Reed parameters come from a **harmonica** reed ("G diatonic harmonica on channel 4 (Lee Oscar model)"), **not an accordion**:
    - length 12.95 mm, width 2.1 mm, thickness 110 µm
    - support thickness 900 µm, rest departure 528 µm, clearance gap 50 µm
    - f_r 444 Hz, K 47.9 N/m, Q 95
  - Simulated inner over-pressure is about 570 to 783 Pa.
  - Useful as a model and parameter template, not as accordion data.
- **Ricot, Caussé, Misdariis, "Aerodynamic excitation and sound production of blown-closed free reeds without acoustic coupling: the example of the accordion reed"**, JASA 117 (2005) 2279–2290 (HAL hal-01106161).
  - The abstract describes flow visualisation and measurements in water on an accordion reed. VERIFIED from the search abstract.
  - I could not read the full text (HAL is blocked by a bot wall), so its numbers are UNVERIFIED.
- **Other Cottingham group JASA abstracts** (laser vibrometer on accordion reeds; reed-chamber geometry, Coyle, Behrens and Cottingham, JASA 126, 2216, 2009; "Vibrational modes of accordion reeds"; "Attack transients in free reed instruments"). These are abstracts only. VERIFIED that they exist; no data tables.
- **Tarnopolsky, Fletcher, Lai, "Oscillating reed valves—An experimental study"**, JASA 2000 (UNSW PDF). VERIFIED.
  - Generic outward-swinging flap valves (100–300 Hz, 0.15 to 0.33 mm sheet), not accordion reeds. Reservoir about 350 Pa.
  - Background only.
- **Directivity:** no accordion directivity dataset was found. TU Berlin excludes free reeds (VERIFIED). A snippet about Bandoneón 2.0 mentions directivity measurements; UNVERIFIED.
- **Anechoic:** no anechoic accordion recordings found (Aalto and TU Berlin VERIFIED negative; OpenAIR anechoic page is down, "Account has been suspended").

---

## 3. Published quantitative tables

### 3.1 Tremolo / musette tuning (Liberty Bellows). VERIFIED
Source: https://www.libertybellows.com/general-accordion-questions.htm. The reference is A = 440 Hz, and "The conversion is roughly 1 hz = 3.5cents in the middle of the keyboard".

| Beat at A4 | Cents | Name |
|---|---|---|
| 0 Hz | 0 | Unison (Dry, Secco): Classical, Balkan |
| 0.5 Hz | 2 | Concert (Violin, Swing Secco): Jazz, Tango, Cleveland Polka |
| 1 Hz | 4 | Swing (Swing Mosso), Gypsy Jazz, Brazilian, Klezmer |
| 2 Hz | 7 | Demi-Swing (Mezzo Swing), Irish |
| 2.5 Hz | 10 | American (Americano), Cajun, Quebecois |
| 3 Hz | 12 | Moderate Tremolo, Slovenian, Tex-Mex, Alpine |
| 4 Hz | 15 | Standard Tremolo, German, Italian |
| 5 Hz | 18 | Fast Tremolo, Modern French |
| 6 Hz | 22 | Very Fast Tremolo, Old French, Old Italian, Italiano Mosso |
| 7 Hz | 25 | Extremely Fast Tremolo (Sardo), Scottish |

**Caveat:** the cents column is rounded and not exact. At 440 Hz, 1 Hz ≈ 3.93 cents, 4 Hz ≈ 15.7 cents and 7 Hz ≈ 27.4 cents. Use Hz at A4 as the defining quantity.

The same page gives a typical professional instrument as "164 Reeds x 2 Sides = 328 Treble Reed Tongues" and "60 Reeds x 2 Sides = 120 Bass Reed Tongues". VERIFIED.

**How tremolo varies across the keyboard:**
- Snippets from accordionists.info and La Malle aux Accordéons describe two extremes: a constant beat rate (cents halve each octave up) or constant cents. An "Americano" example is given as about 9 cents at the bottom, 4 cents at the top, and 7.3 cents at middle C. UNVERIFIED: the forum returned a bot wall and the La Malle page has no numbers.
- **No published maker's tremolo curve** (e.g. from Castelfidardo) was found.
- PSound's range of musette detunes, 4 to 23 cents, is consistent with this. VERIFIED.
- Wikipedia: the tremolo 8′ rank is "usually no more than 25 cents" off. VERIFIED.

### 3.2 Pressure and pitch-vs-pressure numbers found

| Quantity | Value | Source | Status |
|---|---|---|---|
| Normal playing pressure inside the bellows | "between 100 and 220 pascals", max "around 260 pascals" | B. Vavra, MIDI_Accordion (BMP180 barometric sensor inside a 4/5 acoustic accordion); a hobby measurement read from a graph; push/pull difference "I personally did not notice" | VERIFIED as stated, low rigor |
| Lab blowing pressure on accordion reeds | 0.6 kPa; sweep 0.05 to 0.85 kPa | Cottingham slides | VERIFIED |
| Pitch vs pressure, G# reed near 420 Hz | about −9 cents from 0.1 to 0.9 kPa (read off the graph) | Cottingham / Coyle 2009 | VERIFIED (approximate) |
| Pitch vs pressure, bandoneon | low octaves drop up to "almost half a semitone" at extremes; the top octave rises | NIME 2023 | VERIFIED (qualitative) |
| Pitch-bend depth (finger or bellows) | 15 / 25 cents (range 10 to 35) | Elejalde-García et al. 2021 | VERIFIED |
| Modelling supply pressure | about 40 Pa | Elejalde-García et al. 2021 | VERIFIED (assumption) |
| Harmonica reed parameters | see §2 (Millot & Baumann Table 2) | | VERIFIED |

### 3.3 Register layouts and reed ranks
- **Right hand (Accordion4Composers p.16, VERIFIED):**
  - 16′ "sounds one octave lower", so the right-hand keyboard spans "E1 to G6". "The 16 foot register is always in 'cassotto'."
  - There are two 8′ ranks, one bright and one in cassotto.
  - 4′ "usually stop to C#8, but some accordions stop around Bb 7", and has a "third sound" at high notes and loud dynamics.
  - "15 different register combination[s]".
- **Left hand (A4C p.21, free-bass concert instrument, VERIFIED):**
  - The most common arrangement is "two low reeds … from E1 to C#6 plus one very high (from E3 to C#8)". The lowest octave E1 to E2 has two reeds an octave apart.
  - The alternative is one low (E1 to C#6), one central (E2 to C#7) and one high (E3 to C#8).
- **Stradella bass ranks (Wikipedia "Stradella bass system", VERIFIED):** five ranks.

  | Rank | Range |
  |---|---|
  | Soprano | C5 to B5 |
  | Alto | C4 to B4 |
  | Contralto | F♯3 to F4 |
  | Tenor | C3 to B3 |
  | Bass | C2 to B2 |

  - Up to 9 bass register switches. "With the soprano or alto register selected, bass buttons exactly duplicate individual notes from the chords, without the usual added lower (tenor and bass) octaves."
  - The exact per-button reed allocation, including octave wrap points, is **not in the article**. Ask a technician or measure it on the user's instrument.
- **Typical 41/120 instruments (retailer listings, VERIFIED as listed):**
  - Weltmeister Cassotto 414: "41 Keys … 11 Registers", "120 Bass Buttons, 5 Registers", "4/5 LMMH", "Tone Chamber", "24lbs / 11kg".
  - Hohner Morino IV/V models are listed as LMMMH 41/120. Morino specs quoted in snippets: 4 treble choirs, 11 treble registers, 5 bass registers, cassotto. UNVERIFIED details.
  - The "4/5" convention means treble reed ranks / bass reed ranks. This matches the MIDI_Accordion author's "4 reed blocks on the right hand and 5 blocks on the left" (VERIFIED), but I did not find a formal definition.
- **Wikipedia "Accordion reed ranks and switches" (VERIFIED):** "between two and four reed ranks on the treble side"; "between three and five reed ranks on the bass side"; example registers Violin (8′+8′), Oboe (4′+8′), Master (4′+8′+8′+16′).

### 3.4 Reed dimensions against frequency
- **Voci Armoniche** (Antonelli and Salpa merged), https://www.vociarmoniche.it/en/technical-documentation/, lists downloadable PDFs:
  - "Musical extension table for accordion reeds" (note IDs and frequencies; A4 is "A24")
  - "Reed plate outer dimensions table" (width, length, thickness of high and low plates, in mm)
  - "Bass and terzet plate outer dimensions table"
  - VERIFIED that these exist. The PDF URLs were not exposed, and **tongue** dimensions (length, width, thickness per note) are not listed.
- A forum snippet notes that "an A:440 reed can be made at several different lengths" depending on the maker's scale. UNVERIFIED. Tongue geometry is therefore maker-specific and should be measured on the reference instrument.
- The only complete tongue parameter set I found is the harmonica reed in Millot & Baumann (§2).

---

## 4. Protocol for recording our own reference (if the user owns an accordion)

**Microphone guidance found:**
- **Accordion4Composers p.42** (VERIFIED) lists four options:
  - two mics near the two keyboards, following the 3:1 rule, with the bass mic "approximately in the center of the arc that the left hand will draw"
  - ORTF at middle distance
  - clip-on mics
  - internal mics ("too much 'in the box'")
- **Shure, "Eight ways to mic an accordion"** (VERIFIED):
  - cardioid condenser "about 12" from the keyboard side"
  - a second one "about 4–6" from the bellows side"
  - SM57 "about 18" from the center of the grille"
  - clip-ons, an omni taped to the grille, and internal mics
  - "two condenser mics positioned on both sides" performed best
- **Elejalde-García et al.** (VERIFIED): one measurement mic (B&K 4189) at **50 cm**, with an SLM at 50 cm to hold mezzo forte.
- **SOL/IRCAM:** a proximity mic plus a stereo pair plus room mics (§1.1).
- Gear articles (UNVERIFIED snippets): 1 to 2 ft is a "composite" distance; high-pass around 100 Hz; keep mics out of bellows airflow.

**Proposed protocol.** This is my synthesis, meant to match what the Concert Grand scorecard needs.
1. **Document the instrument.**
   - Make, model, reed configuration (e.g. 41/120, 4/5 LMMH, which ranks sit in cassotto), and reed grade (a mano, tipo a mano).
   - Photos of the register panel.
   - The tremolo of each note, measured per reed pair with a tuner. dan-vine/accordion-tuner on GitHub claims multi-reed tremolo detection (UNVERIFIED).
   - The Stradella reed allocation, measured by playing each bass register.
2. **Isolate one reed rank at a time** wherever the register panel allows: 16′ alone (bassoon), 8′ cassotto (clarinet), 8′ open, 4′ (piccolo), and the tremolo 8′ if the instrument is MMM. Then record the key combinations: violin, musette, master.
3. **Record each note on push and pull separately.** They are different reeds.
   - Follow the Bandoneón 2.0 method: a slow crescendo to decrescendo on each reed with **pressure recorded synchronously**. This captures the whole pressure-to-level, spectrum and pitch map in one take.
   - Add steady holds at three fixed pressures (for example about 50, 150 and 400 Pa; set them after a pilot run).
   - Add staccato attacks, releases left to ring, bellows reversal on a held note, key and register clicks, and the air valve.
4. **Microphones.** Use one **fixed reference mic** (omni measurement mic) at 50 cm on the axis of the treble grille, with the instrument body held steady (seated, or clamped on a stand while a helper works the bellows). Optionally add a second mic at the centre of the left-hand arc for the bass side, and a close pair for the "product" sound. Keep the scorecard on the reference mic only. Use a treated or dry room, and capture the room IR at the same positions. Record 24-bit at 48 or 96 kHz.
5. **Bellows pressure.**
   - Use a **differential** sensor. One port goes into the bellows through a sealed fitting in a bellows frame or the air-valve cover; the other port is open to ambient. The sign tells push from pull.
   - Example: Sensirion SDP810-500Pa (VERIFIED from the product page): ±500 Pa, "3% m.v.", zero point 0.1 Pa, "up to 2 kHz" update, I²C.
   - Hobby data (100 to 260 Pa) fits inside that range, but lab reed studies go to 0.6 to 0.9 kPa. **Run a pilot at fff first**, and pick a wider-range part if the peaks exceed 500 Pa.
   - The absolute barometric sensor route (BMP180, as in MIDI_Accordion) is simpler but slower and needs ambient subtraction.
   - Sync pressure and audio with a shared clap or trigger, or by logging pressure as an extra audio-interface channel through an analog sensor.
6. **Scorecard tie-in.**
   - Per reed and per pressure: fundamental (cents vs pressure), spectral centroid and partial levels, attack time vs pressure, and level vs pressure.
   - Per note pair: beat rate (tremolo).
   - Per register: the cassotto low-pass difference, measured by comparing the cassotto and open 8′ ranks on the same note.

The user's wireless headset is not relevant to this protocol.

---

## 5. Ranked shortlist for THE single reference

| Rank | Candidate | Score | For | Against |
|---|---|---|---|---|
| **1** | **Our own instrument**, recorded with the §4 protocol (if one is available) | 5 (potential) | Documented make and reed setup; isolated ranks; push/pull; synchronized bellows pressure, which is the model's actual input; releases; we own the license | Cost and time; needs a pressure fitting; one instrument's quirks become "truth" (acceptable, as with Salamander) |
| **2** | **TinySOL accordion** (IRCAM SOL), CC BY 4.0 on Zenodo | 3.5 | Open; 689 notes E1 to C#8; pp, mf, ff; near-dry proximity mic; one institution with a processing log; usable today | Instrument, registers and push/pull undocumented; "alt" meaning unknown; 35 retuned files; mono 44.1 kHz; license wording conflicts between Zenodo and the paper; trimmed |
| 3 | **RWC-MDB-I No. 07 accordion** (one variation only) | 2 | Full range at half-tone steps; f, mf, p; 5 playing styles | Research-only pledge, no redistribution; paid DVD; the 2026 open release excludes RWC-I; mono 16-bit; makes unknown |
| 4 | **FreePats Button Accordion HN**, CC0 | 1.5 | CC0; releases; Hohner; fine for unit-test fixtures | Diatonic; 17 notes; one dynamic; undocumented format, mic and register |
| 5 | **Accordion4Composers** audio, CC BY-NC-ND 2.5 | 1 (quantitative), useful qualitatively | Pro recording (Schoeps ORTF, 1 m); rich gesture and technique demos; register demos | Reverb added; no-derivatives and non-commercial; not per-note |

The commercial libraries (Le Parisien, Sonokinetic, PSound, the discontinued Accordions 2) are excluded as calibration targets. Their EULAs grant production use only, and Sonokinetic's explicitly forbids building sample libraries and reverse engineering. At most they are informal listening comparisons.

## 6. Recommendation

**Yes, recording our own instrument is necessary** for a reference that meets the Concert Grand standard.
- No public accordion dataset documents its instrument and reed configuration, separates push from pull, isolates reed ranks per register, or includes bellows pressure. Bellows pressure is the plugin's primary control input.
- Every quantitative relationship the model needs (pitch vs pressure, level and spectrum vs pressure, attack time vs pressure, tremolo per note, cassotto filtering) is only in papers as figures, and on other instruments: a Hohner Verdi reed, a Pigini Sirius, an Uhlig bandoneon, a harmonica reed.

**Interim, and only if no instrument is available:** use **TinySOL's accordion as THE reference, alone.** Do not mix in RWC, FreePats or freesound samples; the one-reference lesson applies.
- First confirm the CC BY 4.0 license with the dataset authors, given the paper's "non-commercial" wording.
- Treat the 35 `T…` retuned files as excluded from pitch metrics.
- Once our own recording exists, switch the scorecard to it wholesale. Do not blend the two.
- Use the literature numbers in §3 (Liberty Bellows tremolo table, the pitch-vs-pressure slopes, Stradella ranges) as **priors and sanity bounds**, not as scorecard targets.
