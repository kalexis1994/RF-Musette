# RF-Musette: the accordion as an instrument (around the reeds): literature notes

Scope: everything between the player's arms and the listener's ear except the isolated free reed, which another researcher covers. Research date: 2026-09-30.

## How to read this

- **VERIFIED**: I saw it myself on the publisher page, an index or the author's page. Where I also read the full text, the entry says so.
- **UNVERIFIED**: I saw it only second-hand (a search snippet, a reprint, a citation in another paper), or it is an unsourced statement by a practitioner.
- **Access** is one of: open, abstract only, paywalled, or blocked. "Blocked" means the page is open but a 403, a bot wall or a reCAPTCHA stopped me. A human can usually get past it; I did not try to.
- **Mech.** names the model mechanism the source informs.
- The numbers are the ones the source reports. Where the unit or the meaning is unclear, the entry says so rather than reinterpreting it.
- Octave naming differs between sources. Scientific pitch (A4 = 440 Hz) is used unless noted. The Russian sources use Helmholtz-style names, which I have converted: большая октава = C2–B2, первая = C4–B4, четвёртая = C7–B7.
- Web-search budget ran out near the end (200 queries). The remaining leads are listed under the gaps.

---

## 1. Reed blocks, reed chambers, cassotto, pallets, grille, radiation

### 1.1 Cottingham, J. P. (2016). "Reed chamber resonances and attack transients in free reed instruments." Proc. 22nd ICA, Buenos Aires, paper ICA2016-748
- URL: http://www.ica2016.org.ar/ica2016proceedings/ica2016/ICA2016-0748.pdf
- Status: open. **VERIFIED, read in full.**
- **Setup:**
  - A reed plate from a Hohner Verdi I accordion, with a reed pair sounding at 622 Hz, on a wooden wind chest with a manometer. The flow direction could be reversed.
  - An artificial Helmholtz chamber over the reed, tuned from 600 to 1500 Hz. Its resonance was measured with a swept sine and a probe microphone and agreed with the calculation.
- **Thresholds.** TP1 is the onset pressure on a rising ramp; TP2 is the offset pressure on a falling ramp. TP1 > TP2 always.
  - Inward flow: roughly 0.06–0.11 kPa. Thresholds are highest when the Helmholtz frequency is near the sounding frequency, with a local TP1 maximum near twice the sounding frequency.
  - Outward flow: the plotted axis spans 0.8–1.7 kPa. Thresholds are lower near the sounding frequency, the opposite trend.
  - These are the opposite effects for the two flow directions that Tonon predicted.
  - Caveat: the outward-flow magnitudes are far above normal playing pressure. The paper calls all results preliminary.
- **Mech.:**
  - Onset/offset hysteresis per reed (Pon > Poff).
  - The chamber resonance moves the thresholds, with a different sign for each reed of the pair.

### 1.2 Cottingham, J. P. (2019). "A survey of recent studies on initial transients in free reed oscillation." Proc. 23rd ICA, Aachen, pp. 5530–5535
- URL: https://pub.dega-akustik.de/ICA2019/data/articles/001430.pdf
- Status: open. **VERIFIED, read in full.**
- **Numbers:**
  - Reed-organ reed at 268 Hz on a wind chamber whose Helmholtz frequency was varied by adding pipe sections.
  - Near chamber resonance, sounding needs "excessive pressure (far above normal playing pressure)". In a few cases the reed would not sound at the pressures available.
  - 268 Hz reed on a 327 Hz chamber: negligible interference.
  - 344 Hz reed on the 327 Hz chamber: major effect, with a distorted waveform.
  - Choking is described as a known problem of high-pitched accordion and harmonica reeds.
- **Mech.:** reed-chamber choking and threshold elevation near cavity resonance, which matters in the top octaves.

### 1.3 Cottingham, J. P. (2013). "Reed Vibration and Pitch Bending in Western Free Reed Instruments." CCRMA mini-course slides (Stanford Music 318)
- URL: https://ccrma.stanford.edu/courses/318/mini-courses/Winter2013/Cottingham-2.pdf
- Status: open. **VERIFIED, read in full.** These are slides that summarise Coyle, Behrens & Cottingham 2009.
- **Setup:** reeds from a Hohner Verdi I accordion. Lab waveforms are taken at 0.6 kPa (probe microphone a few mm from the tongue); peak pressure about ±300 Pa.
- **Numbers:**
  - A G# reed near 420 Hz: the plot spans 422 down to 419 Hz as pressure rises from 0.05 to 0.85 kPa. That is a small monotonic fall, of the order of 10 cents (my reading of the axis limits).
  - Partial pallet opening on a 96 Hz reed: less key depression gives a larger drop. Maximum bend 2.5 Hz, which the slide calls "1/2 of a semitone".
  - Reducing the reed-chamber volume from 18 to 8 cm³, with the pallet partly open: maximum bend 4.12 Hz, or 4.5 % (the slide notes a semitone is 6 %), on a reed near 90 Hz.
  - Tonon's resonant-chamber accordion (patent WO 97/44777) bends notes by about a semitone.
- **Mech.:**
  - The pallet aperture acts as a flow restriction in series with the reed.
  - Chamber volume moves the pitch.
  - Pressure moves the pitch slightly.

### 1.4 Coyle, W. L., Behrens, S. L., Cottingham, J. P. (2009). "Influence of accordion reed chamber geometry on reed vibration and airflow." J. Acoust. Soc. Am. 126(4), 2216 (abstract)
- DOI: 10.1121/1.3248803
- URL: https://pubs.aip.org/asa/jasa/article/126/4_Supplement/2216/654226/
- Status: abstract only; blocked (403). **VERIFIED via index.**
- **Content:** reeds measured in the instrument, with the bellows replaced by a clear acrylic wind chest driven by an organ blower. They measured reed displacement and velocity with a laser vibrometer and sound pressure with a probe microphone, and computed airflow from the pressure and an area function.
- **Mech.:** method template for in-instrument chamber measurements. The numbers appear in 1.3.

### 1.5 Tonon, T. (2005). "Reed cavity design and resonance." Papers of the International Concertina Association (PICA) 2
- URL: https://concertina.org/pica-volume-2-2005/reed-cavity-design-and-resonance/ (the HTML carries an erratum for Eqs. 5 and 7)
- Status: open. **VERIFIED, read** (HTML).
- **Models:**
  - Helmholtz cavity: ν0 = (c/2π)·√(A / (V·(t + k·d))), with the end correction k = 0.43–0.80. The higher values apply when the pallet sits close to the aperture.
  - Quarter-wave tube: L_eff ≈ L + t + k·d.
  - Full-wave tube: L_eff ≈ L + 2(t + k·d).
- **Results** (concertina ranges G1–C8):
  - The Helmholtz model predicts interference with the fundamental from about C5–C6 upward.
  - Quarter-wave resonance can hit the fundamentals at C7–C8.
  - Resonance effects matter mostly for partials below the 4th.
  - A Helmholtz resonance at or slightly below the fundamental hampers or chokes the reed; one slightly above it enhances the reed.
  - The effect depends on where the reed sits in the cavity: tip at the closed end chokes, tip at the open end amplifies.
- **Builders' remedies for choking:**
  - a vent hole
  - removing the valves
  - minimising the cavity height
  - partitioning the cavity
  - moving the reed tip
- **Bi-sonorous cavities:** design for the average of the two pitches.
- **Mech.:** per-note chamber filter and chamber–reed coupling. This is a design-level model. The author calls the experiments "limited".

### 1.6 Tonon, T. (2009). "Accordion reeds, cavity resonance, and pitch bend." J. Acoust. Soc. Am. 126, 2217 (abstract)
- Status: abstract only; blocked. **VERIFIED via index** (AIP and ResearchGate listings).
- **Mech.:** pitch bend by deliberately coupling a resonant cavity. See also the patent WO 97/44777, cited by Elejalde-García et al. 2021.

### 1.7 Richter, G. (1989). "Einfluß von Resonanzräumen auf die Klangabstrahlung des Akkordeons." Demusa report '89 (Institut für Musikinstrumentenbau Zwota)
- URL: https://www.ifm-zwota.de/akkreso.htm
- Status: open, a short summary page. **VERIFIED, read in full.**
- **Method** (measured):
  - All reeds of one rank (for example 16') are sounded simultaneously.
  - Third-octave spectra are taken with and without the grille or hood.
  - The difference is plotted as a frequency response: positive means reinforcement, negative means damping.
- **Numbers:**
  - The resonance of the long cassotto shaft is 800 Hz – 1 kHz, "in an acoustically favourable formant region". The shaft dimensions differ little between makers.
  - A closed grille/hood construction (Weltmeister "Stella") resonates at about 500 Hz. It reinforces the bass and lets the treble radiate freely.
  - A bass-reflex cassotto (Weltmeister "Cantora") extends the resonance down to about 200 Hz.
  - Proposed: dividing the cassotto into per-note chambers, tuned narrowly, would raise individual partials rather than a formant.
  - Sound leaves through the tone holes (Tonlöcher), so shaping the tone-hole sound is the main timbre lever.
- **Mech.:** a cassotto or grille response to apply as a formant filter. This is the only source found with a cassotto resonance frequency.

### 1.8 Llanos-Vázquez, R., Elejalde-García, M. J., Macho-Stadler, E. (2008). "Controllable pitch-bending effects in the accordion playing." Proc. Acoustics'08 Paris, pp. 8565–8570; abstract in J. Acoust. Soc. Am. 123(5), 3662
- DOI (abstract): 10.1121/1.2934981
- URL: http://www.conforg.fr/acoustics2008/cdrom/data/articles/001900.pdf
- Status: open. **VERIFIED, read in full.**
- **Instrument:** Pigini Sirius free-bass concert accordion.
  - Right hand: 16' + 8' + 8' + 4', with 15 register combinations. The 16' and one 8' sit in the cassotto.
  - Free-bass left hand: 8', 4', 2'.
- **Valves on this instrument:**
  - leather strips on reeds up to G4
  - plastic strips from G#4 to C6
  - no strip from C6 to C#8
- **Cassotto:** "for frequencies higher than 6000 Hz, our measurements show that this attenuation is around 30 % in sound pressure level".
  - The unit is ambiguous: 30 % of the level in dB, or 30 % of the pressure (about −3 dB)? This is the origin of the "30 % above 6 kHz" claims that circulate on the web.
  - The cassotto does not change the frequencies or the bend. It acts only as a filter on the emitted sound.
- **Levels:** mezzo forte ≈ 70 dBA at 50 cm; B&K 4189 microphone at about 50 cm; FFT 3200 lines to 10 kHz.
- **Bend:**
  - 1–8 % (0.99–0.92 f), a semitone at most.
  - Slightly larger when closing the bellows, possibly for physiological reasons.
  - Even harmonics lose more level than odd ones. The authors attribute this to the chamber changing from an open–open to an open–closed configuration as the valve nearly closes.
  - No bend is possible from C6 to C#8, the reeds without strips.
- **Mech.:** cassotto high-shelf attenuation, the valve-strip layout by register, and the pallet as a throttle.

### 1.9 Elejalde-García, M. J., Macho-Stadler, E., Llanos-Vázquez, R. (2021). "Accordion acoustics: A study on pitch bending." Acoustics in Practice (EAA) AiP-2021-02
- URL: https://euracoustics.org/documents/14/AiP_2021_02_Accordion_acoustics.pdf
- Status: open. **VERIFIED, read in full.**
- **Setup:** Pigini Sirius. Notes A#2–A3 on the 8' out of cassotto, bellows opening, 15 repetitions each. Mezzo forte ≈ 70 dBA at 50 cm.
- **Numbers:**
  - Harmonics reach about 10 kHz in mf, and their level decays with frequency up to 10 kHz.
  - Bend: about 25 cents with bellows and finger, about 15 cents with finger only (range 10–35 cents). Harmonicity is preserved.
  - The level drops more with finger-only bends.
  - Among the first six harmonics, the even ones drop more; from the 7th to the 20th harmonic, the odd ones drop more.
  - A Reynolds-number estimate uses "a supply pressure of about 40 Pa" (V0 ≈ 10 m/s, Re ≈ 133). This is an assumption for the estimate, not a measurement.
  - The upper reeds without strips leak air through the idle slot, so they cannot be bent.
- **Mech.:** partial pallet opening; valve leakage in the treble.

### 1.10 Other chamber and cassotto sources
- **Richter, G. (2009). "Akkordeon-Cassotto – das non plus ultra?" INTERMUSIK 18(4).** Listed in the IfM publication list. **VERIFIED (listing), not read.**
- **Wikipedia "Cassotto" (EN) and "Cassotto" (DE).** They say 500–1000 Hz is reinforced unevenly and everything above 1 kHz is damped. The statements are unreferenced, although the articles list Richter's books. Descriptive only.
- **Unattributed Soviet textbook chapter 7, reprinted at poigarmonika.ru.** URL: https://poigarmonika.ru/garmon-tehnicheskie-aspekty/40-tipovye-nabory-golosovyh-planok-i-razmery-vhodnyh-kamer.html
  - "Experimentally, the best excitation is when the minimum depth of the inlet chamber slightly exceeds the maximum tip amplitude."
  - "The excitation threshold falls when the chamber's natural frequency substantially exceeds the reed's."
  - Its tables (7.7–7.12, including chamber parameters for a bayan) are images I could not read. **UNVERIFIED** (the source book is not identified).

### 1.11 Radiation and directivity
- **No accordion directivity measurement was found.**
- The TU Berlin anechoic database of 41 instruments (Weinzierl et al.; Ackermann, Brinkmann & Weinzierl, arXiv:2307.02110, 2023) contains **no accordion**. VERIFIED.
- IfM Zwota (Gilbert & Ziegenhals, DAGA 2019) describes a 15-microphone, 29-point frequency-curve directivity method. Instruments other than the accordion; a method template only.
- Descriptive only (Kirstein retailer guide, UNVERIFIED): bass tones radiate roughly omnidirectionally, while the treble is more directional toward the audience.
- Llanos 2014 notes that the highest harmonics depend on where the microphone sits relative to the instrument. They fixed a "well-balanced" position about 50 cm in front.
- **Mech.:** the directivity model is an open gap (section 7b).

---

## 2. Bellows, pressure, flow, valves, reversal, air button

### 2.1 Playing pressures

**No peer-reviewed paper that I could read reports bellows pressures measured on players of an acoustic accordion.** What exists:

| Value | Context | Source | Status |
|---|---|---|---|
| 0.6 kPa | Standard lab blowing pressure for accordion reeds | Cottingham 2013 slides; Coyle et al. 2009 | VERIFIED |
| 0.05–0.85 kPa | Pressure sweep for a pitch-vs-pressure curve | Cottingham 2013 | VERIFIED |
| 60–110 Pa | Onset/offset thresholds, 622 Hz reed, inward flow | Cottingham ICA2016 | VERIFIED |
| 300 Pa | "Spieldruck" used for bandoneon spectral comparisons | IfM Zwota poster 2008 (section 5.4) | VERIFIED |
| ~40 Pa | Assumed supply pressure (not measured) | Elejalde-García et al. 2021 | VERIFIED (as an assumption) |
| ~30 Pa threshold (fine mid-range reeds); 200–300 Pa average; 600–800 Pa very loud; ~1000 Pa maximum, where most accordions start to blow the valves open | Technician statement | musiker-board.de thread "Druck(messer) im Balg" (moderator "Maxito"): https://www.musiker-board.de/threads/druck-messer-im-balg.606569/ | UNVERIFIED (unsourced forum statement, but consistent with the lab values) |
| ~10 Pa response of a good instrument (Horch) | Forum statement | musiker-board "Stimmzungenkunde" | UNVERIFIED |

- **Dynamics in sound level** (VERIFIED, Llanos 2014): piano ≈ 55 dBA and mezzo forte ≈ 70 dBA at 50 cm.
- Llanos et al. 2002 (section 5.8): a single note has a dynamic range of 40–50 dB at 1 m without noticeable pitch change.

### 2.2 Ramos, J., Calcagno, E., Vergara, R., Riera, P., Rizza, J. (2022). "Bandoneon 2.0: an interdisciplinary project for research and development of electronic bandoneons in Argentina." Proc. NIME 2022
- DOI: 10.21428/92fbeb44.c38bfb86
- URL: https://nime.pubpub.org/pub/31l4lgcd/release/1
- Status: open; blocked for WebFetch but readable as HTML. **VERIFIED, read in full.**
- **Integral Measurement System** (LAPSo, Universidad Nacional de Quilmes):
  - A real bandoneon bellows on 4 steel rails, with a sealed moving end.
  - BMP280 pressure and temperature sensors inside and outside the bellows.
  - A VL53L0X laser distance sensor and a rotary encoder for bellows extension and speed.
  - A synchronised microphone, with data stored as CSV + WAV.
- **Measured but not published in this paper:**
  - the pressure differential for every note
  - equivalent test holes giving the same airflow, for the sustain phase
- Their controller "Alfa" samples bellows pressure at 100 Hz and has an air-valve lever.
- **Mech.:**
  - bellows as a pressure source with a per-note load (flow resistance)
  - a template for our own measurements

### 2.3 Ramos, J., Calcagno, E., Vergara, R., Rizza, J., Riera, P. (2022). "An Electronic Bandoneon with a Dynamic Sound Synthesis System Based on Measured Acoustic Parameters." Computer Music Journal 46(1–2), 40–57
- URL: https://direct.mit.edu/comj/article-abstract/46/1-2/40/114878/
- Status: paywalled; blocked. **VERIFIED via index, not read.**
- It reports measured sound and pressure signals of an acoustic bandoneon, with the synthesis in Faust. It is probably the best source for pressure → level, timbre and pitch maps (see section 2.4). **Read first.**

### 2.4 Ramos, J. M., Calcagno, E. R., Riera, P. E. (2023). "An embedded wavetable synthesizer for the electronic bandoneon with parameter mappings based on acoustical measurements." Proc. NIME 2023, paper 23
- URL: https://nime.org/proceedings/2023/nime2023_23.pdf
- Status: open. **VERIFIED, read in full.**
- **Instrument:** 71 keys, range C2–C7. Each note is two reeds (8' + 4' "octave"), except the top.
- **Measurement:** an old Uhlig bandoneon. Slow crescendo–decrescendo per reed, with pressure and sound recorded synchronously.
- **Findings:**
  - Crescendo and decrescendo differ subtly because of bellows rigidity.
  - Pressure raises intensity, brightness, detuning and wind noise.
  - Pitch falls with pressure for low notes and rises for the highest notes. At low notes the detuning can reach almost half a semitone at pressure extremes, "seldom used". The pressure-to-pitch map was fitted per note by linear regression.
  - The fundamental and octave reeds are not a perfect octave.
  - Attack time shortens with frequency. Their model is attack ∝ (−n + c)(Pmax − p), with n the note.
  - Release: residual reed oscillation shortens a re-attack.
  - The left side has a "resonator structure" that makes its timbre differ from the right.
  - Wind noise grows at high pressure.
- **Caveat:** the numeric pressure axis appears only in figures.
- **Mech.:**
  - pressure → pitch (sign depends on register)
  - pressure → level and spectrum
  - attack depends on note and pressure
  - retrigger memory
  - wind noise
  - left/right body difference

### 2.5 Roland Corporation, patent US6946594B2 (2005; priority 2001-04-27). Inventors: Bruti, L., Cuccu, D., Gaetani, R. "Method for reproducing the sound of an accordion electronically"
- URL: https://patents.google.com/patent/US6946594B2/en
- **VERIFIED, read.**
- This describes the behaviour the V-Accordion models, from measurements the patent does not publish:
  - Onset and offset thresholds per reed, Pon > Poff.
  - Above a threshold Ps, pitch falls by a per-reed amount that is larger for lower reeds and possibly zero for the highest.
  - Amplitude follows an experimentally obtained monotonic function of P.
  - The exponential attack ramp is shorter at higher P, and shorter if the reed is still moving from a previous note.
  - On key release a "metallic and partially distorted" valve-closing sound occurs. It is louder for bigger reeds and depends on P at closing and on how long the note sounded.
  - Musette detuning is set per reed.
- No numbers.
- **Mech.:** threshold hysteresis, register-dependent pressure pitch-drop, key-off noise.

### 2.6 Valves (flaps)
- **Layout by register (VERIFIED, Llanos 2008 and 2014, Pigini Sirius):**
  - leather strips to G4
  - plastic strips G#4–C6
  - none from C6 up
- **Behaviour:**
  - The valves "moderate the airflow and avoid air passage through the idle reed's slot".
  - Without a strip the idle slot leaks: no bend is possible, and the leak lowers the effective pressure.
- **Llanos 2014:** the attack duration depends on "the characteristics and adjustment of the leather or plastic strip". They cite Lips's bayan method, *The Art of Bayan Playing*, p. 61.
- **Valve noise and buzz** (all descriptive, UNVERIFIED):
  - A flap sound on bellows reversal with a key held (de.wikipedia "Akkordeon"; musiker-board "Vom Schnurren und Schnarren", technician posts).
  - Buzzing when a bulged valve vibrates with the reed, or when a curled "tired" valve leaks.
  - Plastic valves are said to sound brighter than leather (unreferenced, de.wikipedia).
- **Bandoneon:** Argentine players list leather valves among the most important timbre factors. The reference AA 1925 instrument had new leather valves stiffened with thin foil strips (IfM 2008 poster, section 5.4).
- **Mech.:**
  - valve leakage per register
  - the valve closing and opening transient ("flap") on reversal
  - optional buzz
- No measured study of valve dynamics was found (gap).

### 2.7 Flow demand and leakage
- **Ziegenhals, G. (2009). "Schallabstrahlung und Biegeschwingungen von Tonzungen und Stimmplatten." IfM Zwota.**
  - URL: https://www.ifm-zwota.de/tonzun09.pdf
  - Status: open. **VERIFIED, read in full.**
  - The air gap leaves a DC flow that does not produce tone. In multi-rank chord playing it adds up to a noticeable amount that the player must supply with the bellows.
  - IfM standard measurement: reeds waxed onto single chambers with a substitute volume, on a silenced blower.
- **McMahan, R. Y. (2016). "Composer's Guide to the Piano Accordion." American Accordionists' Association.**
  - URL: http://www.ameraccord.com/artwork/2016/Feb16/Composersguidetotheaccordion.pdf
  - **VERIFIED, read.** Descriptive only.
  - More ranks and louder playing use more air, so phrases before a bellows reversal get shorter.
  - Both manuals share one pressure; balance comes only from registration and texture.
- **Cottingham, Physics Today 2011** (DOI 10.1063/1.3563819, VERIFIED): the average flow in harmonica chambers is "hundreds of mL/s". No accordion figure.
- **Harmonikas.cz reed-plate spec** (Czech maker, VERIFIED on maker page), which bounds the leak area:
  - air gap 0.03 mm per side at the rivet and 0.04 mm at the tip
  - bass plates 0.06 mm
  - Forum values (UNVERIFIED): a 1930s Horch low F has 0.1 mm; machine plates 0.03–0.05 mm.
- **de.wikipedia "Akkordeon"**: gaps from 0.2 mm (simple) to 0.02 mm (superior), unreferenced.
- **Bellows leak test (repair folklore, UNVERIFIED):** a sealed accordion should hold air for more than 30 s under gentle pressure.
- **Mech.:**
  - a bellows volume-flow budget = Σ(active reeds' flow) + gap leakage + valve leakage + air-button flow
  - phrase length before reversal
- **Gap:** no measured flow per reed, and no measured bellows compliance or stiffness.

### 2.8 Bellows as a source; reversal; air button
- **Llanos-Vázquez et al. 2014** (section 5.1): a finger attack is like a Heaviside pressure step (bellows pre-tensioned); a bellows attack is like a ramp. Players prefer opening (pulling) for vivid attacks.
- **Ramos et al. 2023:** bellows rigidity makes the crescendo and decrescendo slightly different (hysteresis).
- **Reversal:** "a slight interruption in the sound", comparable to a bow change (McMahan 2016; Llanos et al. 2002). The other reed of each pair takes over, and the valves flip. **No measured gap duration was found.**
- **Air button:** a valve that lets the bellows move without sounding reeds. Used for phrasing and as an audible "wind" effect (Llanos 2002; McMahan 2016; the NIME 2022 Alfa has an air-valve lever).
- **Harmonium analogue: Puranik, N., Scavone, G. (2023). "Physically inspired signal model for harmonium sound synthesis." Proc. DAFx23, Copenhagen**
  - URL: https://www.dafx.de/paper-archive/2023/DAFx23_paper_47.pdf
  - Status: open. **VERIFIED, read in full.**
  - Millot–Baumann reed model, with an added upstream chamber at bellows pressure p0. This gave a better match to measured reed-chamber pressures.
  - The enclosure is treated as a source-filter system, estimated by IAIF: an all-pole filter with 49 coefficients showing 8–9 peaks, approximated by a cascade of 10 biquads.
  - Bellows pressure mainly scales amplitude. f0 changes by less than 1–2 Hz. Reservoir pressure decays as air leaks.
  - Their earlier model: Proc. Meetings on Acoustics 49, 035015 (2022). VERIFIED via index; not read (403).
- **Mech.:**
  - the bellows modelled as a volume with the player's force as input
  - a leak term
  - the enclosure as a filter bank

---

## 3. Registers and tuning (tremolo / musette)

### 3.1 Hergert, F. (2023). "'Celeste' ranks in pipe organs and accordions: tonal timbre and consonance of detuned unison intervals." Proc. Forum Acusticum 2023, Torino, pp. 4455–4460
- DOI: 10.61782/fa.2023.0326
- URL: https://dael.euracoustics.org/confs/fa2023/data/articles/000326.pdf
- Status: open. **VERIFIED, read in full.**
- **Stops:** the accordion "violin" stop is two ranks, true + sharp. "Musette" is three ranks: flat, true and sharp.
- **Beat frequency:**
  - usually 0.5–15 Hz, rising with pitch
  - accordion traditions run from "dry" to "wet" as 0.5–7 Hz at A4
- **Tuning curves:**
  - Measured curves on two accordions and a Harmona (a reed organ with accordion reeds) are shown in Fig. 3; the values appear only in the figure.
  - Studies with listening tests say the curve should rise linearly in the bass and exponentially in the treble, or follow an S-curve (Porvenkov). The beat frequency should not exceed the just-noticeable variation of frequency, nor about 15 Hz. The treble may be capped at about 10 Hz.
  - Recommendation: double the beat frequency every 1.5 octaves.
- **Tables for the "type d)" curve** (beat frequency in Hz):

| f(A4) | F#2 | D#3 | C4 | A4 | F#5 | D#6 | C7 | A7 |
|---|---|---|---|---|---|---|---|---|
| 1.4 Hz | 0.49 | 0.70 | 0.99 | 1.40 | 1.98 | 2.80 | 3.96 | 5.60 |
| 1.7 Hz | 0.60 | 0.85 | 1.20 | 1.70 | 2.40 | 3.40 | 4.81 | 6.80 |
| 2.0 Hz | 0.71 | 1.00 | 1.41 | 2.00 | 2.83 | 4.00 | 5.66 | 8.00 |
| 3.5 Hz | 1.24 | 1.75 | 2.47 | 3.50 | 4.95 | 7.00 | 9.90 | 14.0 |

  - The 3.5 Hz row in cents, at F#2 → A7: 23.1, 19.5, 16.4, 13.8, 11.6, 9.7, 8.2, 6.9.
- **Three-rank musette:** with −Δf, 0, +Δf the pair beats are Δf and 2Δf. Other ratios give irregular beating. An asymmetric 0, +Δf, +2Δf also works.
- **Just-noticeable variation of frequency:** an empirical fit (Eq. 4, valid 50 Hz – 20 kHz) with coefficients a−1 = 7.76, a0 = 319.6, a1 = 20.2, a2 = 39.57, a3 = 2.12, a4 = 30.17.
- **Porvenkov-based accordion curve:** Eq. 5, valid E2–A#7, with b0 = 1.15, b1 = 4.42e−3, b2 = −3.34e−6, b3 = 1.63e−9, b4 = −2.21e−13.
  - Caution: the signs and exponents come from PDF text extraction and must be checked against the PDF, and the form of Eq. 5 is not in the text layer.
- **Timbre:** a beating dyad of rich tones modulates the spectral centroid as well as the loudness. For an organ-pipe example, c/f₀ is 2.67 without beats and swings between 2.12 and 3.55 with them. Accordion reeds, being rich in harmonics, are the case where timbre fluctuation matters. Fluctuation strength peaks at 4 Hz.
- **Mech.:**
  - tremolo detuning curve across the compass
  - the flat/true/sharp arrangement
  - a psychoacoustic ceiling on beat rate

### 3.2 Hergert, F. (2024). "Targeted detuning aiming for sensory pleasantness – A case study of Pipe Organs and Accordions." Acta Acustica 8, 33
- DOI: 10.1051/aacus/2024020
- URL: https://acta-acustica.edpsciences.org/articles/aacus/full_html/2024/01/aacus240029/aacus240029.html
- Status: open access; blocked (403 to me). **VERIFIED via index and author page, not read.**
- It extends 3.1. The figure list includes "Tuning progression curves of the Celeste ranks in two different Accordions and a Harmona". **Read first.**

### 3.3 Porvenkov, V. G. (1979). "Optimum beat rates in well-tuned musical reed instruments." Proc. 18th Acoustical Conference of Czechoslovakia, Český Krumlov, pp. 127–131 (in Russian)
- The citation is as given by Hergert; the conference paper itself was not seen.
- **Book:** Porvenkov, V. G. (1990). *Акустика и настройка музыкальных инструментов* (Acoustics and Tuning of Musical Instruments). Moscow: Muzyka. VERIFIED as a citation and in book listings; not read.
- **Russian web reprint** of the article: https://poigarmonika.ru/garmon-tehnicheskie-aspekty/26-optimalnyi-rozliv-v-nastroike-yazychkovih.html. **UNVERIFIED**: a secondary reprint, but the text and numbers were read.
- **Research context:** the НИКТИ musical-industry institute lab.
- **Old practice:** beat rate ("розлив") rising linearly from 1 Hz in the great octave to 10 Hz in the 4th octave.
- **Proposal, based on the difference limen:**
  - The limen is about 1.5–1.8 Hz below 500 Hz, and proportional to frequency above it (factor 0.003).
  - So: 1.5 Hz at E2, rising almost linearly to 2.75 Hz at B4, then proportionally, reaching 13.2 Hz at A7. Note that 13.2 Hz is more than 0.003 × 3520 Hz, so the curve is not literally 0.003f.
- **Validation:** an expert listening test on 7 accordions from the "Krasny Partizan" factory, Leningrad. The best-rated one lay within about 0.8 Hz of the optimum in the low octaves. Beat-rate measurement error ≤ 4 %.
- A range "1–3 beats/s for the lower reeds, 4.5 to 15–18 for the upper" is quoted without clear context.
- **Mech.:** a measured and perceptually validated tremolo curve. This is the most "measured" tremolo source.

### 3.4 Practitioner tremolo conventions (not measured)
- **Liberty Bellows FAQ**, a US repair shop: https://www.libertybellows.com/general-accordion-questions.htm. Seen, practitioner.
  - Beat at A4 between M and M+, by style:

| Beat at A4 | Cents | Style |
|---|---|---|
| 0 Hz | 0 | dry / classical / Balkan |
| 0.5 Hz | 2 | concert, swing secco (jazz, tango) |
| 1 Hz | 4 | swing |
| 2 Hz | 7 | demi-swing / Irish |
| 2.5 Hz | 10 | American / Cajun |
| 3 Hz | 12 | moderate (Slovenian, Alpine) |
| 4 Hz | 15 | standard German / Italian |
| 5 Hz | 18 | modern French |
| 6 Hz | 22 | old French, "Italiano mosso" |
| 7 Hz | 25 | Scottish |

  - Their cents are rounded: 1 Hz at 440 Hz is 3.93 cents.
- **Forum tuner rules** (accordionists.info, search snippets only, UNVERIFIED):
  - Cents at A5 are about 0.7× those at A4, and about 0.5× at A6. A3 is about 1.3×.
  - Example: 15 cents at A4 → about 18 at C4, 10 at A5, 6.5 at A6.
  - A tremolo of 18 cents at A4 is about 5 Hz. In the 1970s ±20 cents was common; today ±10–12.
  - These are consistent in shape with Hergert's "double every 1.5 octaves".
- **Wikipedia "Accordion reed ranks and switches"**: tremolo detuning is "no more than 25 cents". Register names: violin 8'+8', musette 8'+8'+8' (in a three-8' instrument), bandoneon 16'+8', master = all.
  - Caution: Wikipedia gives musette as 4'+8'+8', which contradicts the standard L/M/M+/M− naming. Musette is three 8' ranks per Hergert.
  - Descriptive.
- **Tuning tolerance (VERIFIED; Richter, Demusa report '90):** https://ifm-zwota.de/stimmakk.htm
  - Frequency scatter about ±20 ct after the tongues are ground, about ±5 ct after bench tuning, ±2 ct target with automatic tuning. Reference a1 = 440 Hz, equal temperament.
  - Harmonikas.cz states ±20 ct from manufacture and ±5 ct final.
  - **Mech.:** per-reed random detune of a few cents for realism.
- **Pressure-dependent pitch** (measured, non-peer-reviewed): Faragó, Z., Wiech, S. (2024-02-22). "Wohltemperiertes Akkordeon (Teil 2)." akkordeon.online.
  - URL: https://akkordeon.online/wohltemperiertes-akkordeon-obertoene-schwebungen-und-temperierung-2/
  - Method: USB microphone, FFT of 65536 points with a Blackman-Harris window, on a Weltmeister Stella 60.
  - pp→ff pitch drop of up to 10 ct around C4, and up to 40 ct in the lower octaves.
  - Staccato or bellows-shake onsets below 500 Hz are up to 50 ct flat relative to the settled pitch; low notes take 0.3–0.4 s to settle.
  - Seen on the author's page. Amateur measurement, but consistent in sign with 2.4 and 2.5.
  - **Mech.:** pressure detuning interacts with the tremolo beat, since the ranks may drop differently.

---

## 4. Stradella bass and free bass

- **No measured study found.** All sources are descriptive.
- **McMahan 2016** (VERIFIED): Stradella has 5 ranks with 7 switch combinations. Single notes are limited to one octave, so octave displacement occurs. Free bass comes as "quint" and "chromatic" converter systems of about 3+ octaves, switched from Stradella.
- **Llanos 2008/2014** (VERIFIED): the Pigini Sirius free bass has 8', 4', 2'. There is no cassotto on the left.
- **Llanos et al. 2002** (VERIFIED): a "usual" accordion has 4 treble and 5 bass sets, at least 448 tongues.
- **Wikipedia "Stradella bass system"** (descriptive, unreferenced figures):
  - Five bass sets with ranges soprano C5–B5, alto C4–B4, contralto F#3–F4, tenor C3–B3, bass C2–B2.
  - Modern chord buttons sound 3 pitches: the 5th is omitted in 7th and diminished chords.
- **Peter M. Haas, "Ratgeber Bassregister am Akkordeon"** (musician and educator, UNVERIFIED): https://www.petermhaas.de/ratgeber-bassregister-am-akkordeon/
  - Basses are generally 4- or 5-rank. The deep "Bass" reeds are "very deep and very soft".
  - The counterbass and "Begleiter" (chord) ranks couple in.
  - Builders stagger the octave breaks between ranks, "the infinite octave", to hide the bass's true height: a Shepard-like design.
- **Russian sources** (booksite.ru encyclopedia, via search snippets, UNVERIFIED): a bayan bass button sounds 3, 4 or 5 octave-related reeds, one base plus 2–4 higher.
- **Adam Matlock, "Accordion Guide for Composers"** (blog, UNVERIFIED): the bass span is one octave, usually C to B, or F to E on many settings.
- **Mech.:**
  - a bass-button → set of octave-stacked reeds, with staggered wrap-around points per rank
  - chord buttons → 3 pitch classes on the chord ranks only
  - Values should come from inspecting a real instrument, or from Richter's handbook (section 6.2).

---

## 5. Measured spectra, timbre, attack; bayan, bandoneon, concertina

### 5.1 Llanos-Vázquez, R., Elejalde-García, M. J., Macho-Stadler, E., Agos-Esparza, A. (2014). "Physical and Psychoacoustic Characterization of the Different Types of Attacks on the Accordion." Acta Acustica united with Acustica 100(2), 375–384
- DOI: 10.3813/AAA.918716
- Status: I got the full text through the DAEL portal (dael.euracoustics.org, document 45451); the publisher version is paywalled. **VERIFIED, read in full.**
- **Setup:** Pigini Sirius, 8' out of cassotto. Notes A, A#, B in octaves 2–6. Always bellows opening. p ≈ 55 dBA and mf ≈ 70 dBA at about 50 cm. At least 15 repetitions. FFT of 6400 lines over 20 Hz – 20 kHz (3.125 Hz resolution); time precision 10 ms.
- **Attack time** of the first harmonic, from −50 dB to −5 dB of its maximum:

| Dynamic | Finger attack | Bellows attack |
|---|---|---|
| mf | 50–110 ms | 190–630 ms |
| p | 60–140 ms | 250–660 ms |

  - Variation between neighbouring semitones is large. The authors attribute it to reed shape, reed adjustment and the valve strip.
  - A slow keystroke gives an intermediate attack: A4 80 ms in mf, 220 ms in p.
- **Spectral centroid:**
  - At attack start it is about 8.5–9 kHz for both attack types, falling to about 2.5 kHz at the end (A#3 mf).
  - The finger attack's centroid dips below the bellows attack's between about 0.7T and 1.7T.
  - The cassotto lowers the centroid but does not change attack durations (A4: bellows 390 → 480 ms, finger 60 → 60 ms).
- **Harmonics:**
  - In finger attacks all harmonics start and end together.
  - In bellows attacks they start together but end at different times, with a steep dB/ms rise in the first quarter.
- A slow keystroke through a nearly closed pallet at high bellows pressure produces a Venturi effect: the key feels harder and the onset can be unstable.
- **Mech.:**
  - the attack is driven by the pallet opening rate (finger) or the pressure-rise rate (bellows)
  - a pressure step vs a ramp as the model input
  - the cassotto as a filter

### 5.2 Llanos-Vázquez, R. (2015). *Acústica del acordeón*. PhD thesis, Universidad del País Vasco UPV/EHU, 322 pp.
- Handle: 10810/16562
- URL: https://addi.ehu.eus/handle/10810/16562
- Status: open access, but now behind a reCAPTCHA; not read. **VERIFIED (repository index).**
- Probably the most complete accordion-as-instrument acoustics study in one place. It covers attacks, cassotto, pitch bend, and probably spectra by register and bellows direction. **Read first.**
- Related papers:
  - Llanos-Vázquez, Elejalde-García, Macho-Stadler, "Attack transients in accordion", ICSV13, Vienna, 2006
  - the same authors, "Comparison between attack transients of accordion and other free-reed instruments", Forum Acusticum, Aalborg, 2011
  - Both are VERIFIED as citations only.

### 5.3 Baltrusch, M., Schetelich, F., Ziegenhals, G. (2008). "Wiederbelebung des 142-tönigen Alfred-Arnold Bandonions." IfM Zwota poster (DAGA 2002 origin; 2008 version)
- URL: https://www.ifm-zwota.de/bandonio.pdf
- Status: open. **VERIFIED, read in full.**
- **What was measured:**
  - tongue stiffness
  - air gaps
  - air consumption ("Luftverbrauch")
  - tuning and octave purity
  - response and response time
  - loudness
  - octave-wise timbre
  - key force
- **Findings:**
  - The classic Alfred-Arnold bandoneon is clearly poorer in overtones than an accordion. The authors attribute this to much larger air gaps and to saw-rough reed chambers ("sägeraue Kanzellen") that damp overtones.
  - Bass notes have a strong fundamental and strong partials up to about 1 kHz, in the vowel formant regions of U, O and A. Treble notes have strong partials up to 8.5 kHz.
  - A conical air gap, up to 0.10 mm larger at the tongue tip than at the foot, plus a front gap of 0.10 mm in the great and small octaves, lowers the upper partials without losing the fundamental or the first overtones, with less air loss than a wide parallel gap.
  - Tongue stiffness has little influence on timbre.
  - In the low range up to the small octave, loudness rises and sharpness falls as tongue stiffness increases. From the 2-line octave (c'') upward no influence can be detected.
  - The spectral comparisons were made at 300 Pa playing pressure. Separate curves exist for push and pull ("Druck/Zug") and for the fundamental and octave rows (figure only).
- Players named as important: the plate material (zinc), a beat-free octave rank, leather valves, the reed block, and air-tightness, especially the treble-side air flap.
- **Mech.:**
  - chamber-surface damping of high partials
  - the air gap as a timbre control
  - push/pull curves exist but the numbers are graphic only
- Further reading: Schetelich, F. (2006). "Untersuchungen zum Einfluss der Stimmplatten- und Stimmstockparameter auf die Klangfarbe der Zungeninstrumente." Musicon Valley Report, ISBN 3-00-019671-4. VERIFIED (listing), not read. It covers chamber length, depth and material, and reed-block material.

### 5.4 Ziegenhals 2009 (section 2.7) on the blown reed's radiated spectrum
- VERIFIED.
- The blown reed radiates a strictly harmonic series on the first mode (about 350 Hz for the F4 reed). The sound is pulse-like, "Summton", from the throttled airflow; the tongue's own radiation is negligible.
- Sound couples back into the tongue at the harmonic frequencies.

### 5.5 Puranik & Scavone, DAFx23 (section 2.8)
- The harmonium enclosure response has 8–9 peaks. This is a practical method for estimating a body/reed-block filter from recordings. VERIFIED.

### 5.6 Other items
- **Maciejewski, T. (2013). "Analysis of the accordion sound."** Blog, https://maciejewski.com/en/analysis-of-the-accordion-sound/. FFT pictures of registers; no numbers in the text. Low value.
- **Faragó & Wiech (2024), section 3.4:**
  - Weltmeister Stella 60 partials are near-harmonic, deviating by hundredths of a percent except the 10th and 12th.
  - "The first 7 overtones [are] louder than the fundamental". This is a single instrument, measured by an amateur.
- **Bayan / Russian:** no open measured spectral study found. Leads:
  - Porvenkov 1990
  - Mirek, A. M. (1968). *Справочник по гармоникам*. Moscow: Muzyka. UNVERIFIED details; it contains reed-plate and chamber tables according to web reprints.
  - Fadeev, I., Kuznetsov, I. *Ремонт гармоник, баянов и аккордеонов* (as cited by poigarmonika.ru): resonator woods are spruce and fir; bass resonator centre in 3–4 mm birch plywood. UNVERIFIED.
- **Concertina:** Tonon 2005 (section 1.5).
- **Bandoneon:** 2.2, 2.3, 2.4 and 5.3.
- **Italian (Castelfidardo / UNIVPM) and Finnish/Slovenian research:** nothing quantitative found. The UNIVPM thesis PDFs returned 403, and their topic could not be confirmed.

---

## 6. Construction data

### 6.1 Numbers found
- **Accordion F4 ("f1") reed (VERIFIED; Ziegenhals 2009):**
  - tongue length 36 mm, width 4 mm
  - set (Aufbiegung) 0.5 mm
  - plate thickness 3 mm
  - tip amplitude above 4 mm already at mf, by stroboscope
  - modes at 355 and 1645 Hz, a ratio of 1 : 4.6 because of the profile
- **Harmonikas.cz reed plates** (maker specification, VERIFIED on maker page): http://www.harmonikas.cz/de/dix.html
  - Plates: duralumin, 160 HB, machined by EDM to ±0.01 mm.
  - Air gap: 0.03 mm per side at the rivet, 0.04 mm at the tip; bass plates 0.06 mm.
  - Tongues: Swedish steel at 2000 N/mm², from 8 mm strip, CNC-ground to ±0.002 mm.
  - Rivet head 7–9 mm.
  - Grooved plate sides for wax adhesion.
- **Llanos et al. 2002** (VERIFIED): duralumin plates; blued, tempered steel tongues; reed blocks "usually made of poplar"; reed quality classes commercial / hand-finished / hand-type (tipo a mano) / hand-made.
- **IfM 2008** (VERIFIED): a conical air gap of up to 0.10 mm in bandoneon bass plates; a zinc-alloy plate tradition, called "rather an accidental impurity" in the poster.
- **Voci Armoniche** (Castelfidardo reed maker): https://www.vociarmoniche.it/en/technical-documentation/ publishes per-note plate outer dimensions (width, length, thickness) and a plate-ID mapping as PDFs. **Login required, not read.** The plate dimensions would determine reed-block cell sizes.
- **Valve sizes:** forum snippet, treble valves from 39 × 7.5 mm down to 21 × 6 mm. UNVERIFIED.
- **"A mano" vs machine reeds:** only practitioner claims, UNVERIFIED. A mano reeds are said to have a tighter air gap, so less air use and faster response, and to be stamped along the strip grain. Llanos 2014 shows reed adjustment changes attack time a lot. **No measured comparison was found.**
- **Waxing:** Harmonikas.cz grooves its plate sides for wax adhesion, and IfM waxes reeds onto test chambers. **No acoustic study of wax found.**

### 6.2 Key books (not read)
All VERIFIED as bibliographic entries.
- **Richter, G. (1990; 2nd ed. 2000; 3rd 2003; 4th 2008). *Akkordeon. Handbuch für Musiker und Instrumentenbauer*.** Leipzig: Fachbuchverlag (1st ed.); Wilhelmshaven: Noetzel (later editions). ISBN 3-7959-0569-9.
  - Its contents include "objective measurement methods and quality criteria", "reed plates", and "other parts and their influence on sound".
  - The DHV "Jahr des Akkordeons 2026" page cites pp. 156–161 of the 1990 edition on reed physics.
- **Richter, G. (1985?). *Akustische Probleme bei Akkordeons und Mundharmonikas*, Teil 1 (ISBN 3-925572-00-7) and Teil 2, "Untersuchungen spezieller Phänomene" (ISBN 3-925572-01-5).**
  - Tonon cites it as 1985, with the author given as "Gerhard".
  - The author is Gotthard Richter, the IfM Zwota physicist.
- **Macerollo, J. (1980). *Accordion Resource Manual*. Avondale Press.** Cited by Llanos.
- **Gervasoni, P. (1986). *L'Accordéon, instrument du XXème siècle*. Mazo.** Cited by Llanos.
- **Lips, F. (2000). *The Art of Bayan Playing*. Karthause-Schmülling.** Cited by Llanos.

---

## 7. Synthesis

### 7a. Signal path as the literature supports it

For each stage: the best source, and whether its data is measured or only described.

1. **Player → bellows force and displacement.**
   - Best: Ramos et al. 2022 (NIME), the method with pressure inside and outside, bellows extension and speed. Data measured but not yet published; the CMJ paper is unread.
   - Pressure levels: lab values of 0.06–0.85 kPa (Cottingham, measured). In-play levels of 30 Pa up to about 1000 Pa are practitioner claims only.
   - **Status: described or partly measured. No published in-play pressure distribution.**
2. **Bellows as a pressure source.**
   - Model it as a compliant volume with the player's force as input, plus a leak.
   - Best: Puranik & Scavone 2023 (the p0-chamber idea, harmonium) and Ziegenhals 2009 (gap leakage adds up across ranks).
   - Crescendo/decrescendo hysteresis from bellows rigidity: Ramos 2023 (observed).
   - **Status: described. Compliance, volume and leak rate are unmeasured in the literature.**
3. **Bellows reversal.**
   - The flow sign flips, the other reed of each pair speaks, and the valves flip, with a brief interruption and a flap noise.
   - Best: McMahan 2016 and the Roland patent (key-off noise). **Status: described only.**
4. **Air button.** Llanos 2002; McMahan 2016; NIME 2022. **Status: described only.**
5. **Keyboard pallet.** A variable orifice in series with the reed chamber.
   - Opening rate sets the finger-attack time: 50–110 ms (mf) versus 190–630 ms for bellows attacks.
   - Partial opening throttles the flow, giving a bend of 15–25 cents typical and up to about a semitone, plus a level drop and a shift of even harmonics relative to odd.
   - Venturi effect on slow opening.
   - Best: Llanos 2014 and Elejalde-García 2021 (measured); Cottingham 2013 (measured, lab).
   - **Status: measured.**
6. **Reed chamber (cell in the reed block).**
   - A Helmholtz or quarter-wave resonator between pallet and reed.
   - It changes the thresholds (60–110 Pa vs 0.8–1.7 kPa depending on flow direction), can choke the top-octave reeds, and moves the pitch slightly with volume (4.5 % over 18 → 8 cm³).
   - It filters the timbre, and rough chamber walls damp the high partials (IfM 2008).
   - Best: Tonon 2005 (model); Cottingham 2016 and 2019 (measured); IfM 2008 (measured, bandoneon).
   - **Status: measured in the lab, not per note on a real accordion.**
7. **Reed plate pair and valves.**
   - A leather/plastic/none layout by register (Pigini: leather to G4, plastic to C6, none above).
   - Leakage through the idle slot where there is no valve; gap leakage (0.03–0.1 mm).
   - Valve flap and buzz.
   - Best: Llanos 2008/2014 (layout, observed); Harmonikas.cz (gaps, specification); Roland patent (behaviour).
   - **Status: layout and gap sizes measured or specified; valve dynamics undescribed.**
8. **Reed (other researcher)** → pressure-dependent pitch.
   - Low reeds flatten with pressure (10–40 ct pp→ff); top reeds are flat-neutral or sharpen.
   - Onset/offset hysteresis; attacks faster with pressure and with residual motion.
   - Best: Ramos 2023; Faragó 2024; Roland patent. **Status: measured (bandoneon, amateur accordion) plus patent descriptions.**
9. **Multiple ranks per key** (registers).
   - 16' / 8' / 4' plus M+/M− tremolo, with a beat curve across the compass.
   - Best: Hergert 2023/2024; Porvenkov 1979.
   - **Status: measured curves (in figures), a listening-test-based rule, and practitioner tables.**
10. **Cassotto** (for the ranks inside it).
    - A long shaft, resonant at 800 Hz – 1 kHz, with high-frequency attenuation ("about 30 % in SPL above 6 kHz", unit ambiguous). It lowers the spectral centroid and does not affect pitch or attack duration.
    - Best: Richter 1989 (measured method, frequency only); Llanos 2008/2014 (measured).
    - **Status: measured but sparsely reported. No published transfer function.**
11. **Tone holes → grille or hood → air** (treble).
    - A closed-grille hood resonates at about 500 Hz. Vented grilles are normal. The bass side has its own tone holes; a bass-reflex cassotto reaches about 200 Hz.
    - The left side of the bandoneon has a "resonator structure" that makes its timbre differ.
    - Best: Richter 1989 (measured method); Ramos 2023 (observed).
    - **Status: partly measured.**
12. **Body and directivity → room.**
    - No accordion directivity data. A source-filter enclosure estimate (Puranik & Scavone) is a practical substitute.
    - **Status: gap.**
13. **Also unmeasured:** sound radiated into the bellows and out through the bellows walls, and the player's body shadowing.

### 7b. Open gaps
1. **In-play bellows pressure** (Pa) for real players across dynamics, registers and push/pull. No readable peer-reviewed source; only lab values and technician claims. Ramos et al. CMJ 2022 is the most likely to have it (unread).
2. **Bellows mechanics:** interior volume vs extension, compliance or stiffness, leak rate, the force-to-pressure relation, and behaviour through reversal (pressure trace across the flip, the length of the sound gap).
3. **Flow per reed and per register** (L/s at a given Pa), and the pressure drop when many ranks sound. IfM measured "Luftverbrauch" on bandoneons (poster, no numbers).
4. **Valve (flap) dynamics:** opening pressure, closing transient and noise spectrum, buzz conditions. Nothing measured.
5. **Cassotto transfer function per register:** only one resonance range (Richter) and one ambiguous high-frequency attenuation (Llanos).
6. **Per-note reed-chamber geometry** on real accordions (volume, outlet area, neck length). This is needed to place Helmholtz frequencies per note. It is in maker drawings (Voci Armoniche, login-gated) and in Soviet or Richter tables (not read).
7. **Directivity and radiation** of treble vs bass side, the grille effect in angle, and bellows-wall radiation.
8. **Push vs pull spectra on an accordion.** The IfM bandoneon curves exist only as figures; Llanos measured only opening in 2014 and 2021.
9. **Stradella:** measured rank layout, octave-break positions per rank, and relative rank levels for a real 120-bass. Descriptive sources only.
10. **Tremolo curves measured on real accordions**, as numbers rather than figures. The Hergert 2024 figures would need digitising; Porvenkov's table is not legible in the reprint.
11. **"A mano" vs machine reeds**, and waxing: no acoustic measurement.
12. **Bayan/Russian and Italian industry measurement literature:** not reachable online. Leads are Mirek 1968, Porvenkov 1990 and the Soviet textbook chapter 7 tables.

### 7c. The five sources to read in full first
1. **Llanos-Vázquez, R. (2015). *Acústica del acordeón*.** PhD thesis, UPV/EHU, hdl 10810/16562. Open access; download manually past the reCAPTCHA. The only whole-instrument accordion acoustics study found: attacks, cassotto, pitch bend, and probably register and bellows-direction spectra, on one concert instrument.
2. **Richter, G. *Akkordeon. Handbuch für Musiker und Instrumentenbauer*** (4th ed., Noetzel 2008; ISBN 3-7959-0569-9). Written by the IfM Zwota physicist behind the cassotto and grille measurements. Expect objective measurement methods, pressures, chamber, valve and cassotto data, and bass-rank layout. Pair it with his *Akustische Probleme bei Akkordeons und Mundharmonikas* I/II if obtainable.
3. **Hergert, F. (2024). "Targeted detuning aiming for sensory pleasantness – A case study of Pipe Organs and Accordions." Acta Acustica 8, 33.** DOI 10.1051/aacus/2024020. Open access. Measured tremolo curves on two accordions and a Harmona, plus psychoacoustic tuning rules. The FA2023 companion (read) gives the tables above.
4. **Ramos, J. et al. (2022). "An Electronic Bandoneon with a Dynamic Sound Synthesis System Based on Measured Acoustic Parameters." Computer Music Journal 46(1–2), 40–57.** Measured pressure-synchronised sound of a bellows free-reed instrument. The most likely source of real Pa ranges and of pressure → level, timbre, pitch and attack maps.
5. **Tonon, T. (2005). "Reed cavity design and resonance." PICA 2** (open HTML; read, but its tables and equations should be implemented carefully). Together with Cottingham ICA2016, it is the basis for the per-note reed-chamber resonator model and the top-octave choking and threshold behaviour.

### Suggested starting constants
Each one is traceable to a source above.
- **Lab reference pressure:** 600 Pa (Cottingham).
- **Thresholds:** 60–110 Pa for a 622 Hz reed in its favourable direction (Cottingham 2016).
- **Playing range:** 30 Pa at threshold to about 1 kPa at maximum (practitioner, UNVERIFIED).
- **Loudness:** p ≈ 55 dBA and mf ≈ 70 dBA at 50 cm (Llanos 2014).
- **Attack (−50 → −5 dB):** finger 50–140 ms, bellows 190–660 ms (Llanos 2014).
- **Pressure pitch drop, pp→ff:** 10 ct around C4, up to 40 ct in the bass, about 0 or sharp in the top octave (Faragó 2024; Ramos 2023; Roland patent).
- **Pallet bend:** 15 ct finger-only, 25 ct with bellows and finger, up to about 1 semitone (Elejalde-García 2021; Llanos 2008).
- **Valves:** leather ≤ G4, plastic G#4–C6, none above C6 (Pigini; Llanos 2008).
- **Cassotto:** resonance 0.8–1 kHz (Richter 1989); high-frequency attenuation above 6 kHz, "about 30 % SPL" (Llanos).
- **Tremolo:** at A4, 0.5–7 Hz from dry to wet; about 4 Hz (15 ct) standard, about 5–6 Hz French. Scale about ×2 every 1.5 octaves (Hergert), or use Porvenkov's 1.5 Hz at E2 → 2.75 Hz at B4 → 13.2 Hz at A7. Cap at 10–15 Hz.
- **Per-reed tuning scatter:** ±2–5 ct (Richter 1990; Harmonikas).
- **Air gap:** 0.03–0.06 mm on modern machine-made plates; up to 0.1 mm on old or bandoneon plates (Harmonikas; IfM 2008).
