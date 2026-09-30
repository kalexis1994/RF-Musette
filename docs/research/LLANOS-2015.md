# Notes on Llanos-Vázquez's thesis

Read for RF-Musette on 2026-09-30 from the user's copy of: R. Llanos Vázquez,
*Acústica del acordeón*, PhD thesis, Universidad del País Vasco / Euskal
Herriko Unibertsitatea (UPV/EHU), Departamento de Física Aplicada I, 2015;
supervisors M. J. Elejalde-García and E. Macho-Stadler; UPV/EHU Press,
ISBN 978-84-9082-294-4; repository handle 10810/16562. Page numbers are the
thesis's own. Quotations are short, in the original Spanish with an English
translation; tables are the thesis's measurements, credited to it.

## Read this first: what the thesis does NOT contain
- **It gives no blowing or bellows pressure in Pa anywhere.** A search for `\bk?Pa\b` finds only "1 rayl = 1 Pa s m-1" on p79. Dynamics were set and measured as **sound level in dBA at 50 cm**. Every "presión" in the pitch-vs-pressure section means *sound* pressure level, not blowing pressure.
- **It gives no reed dimensions in mm**: no width, thickness, profile, set ("alzada" is never used) or clearance number. There are no cell, tone-hole, pallet or bellows dimensions, no torsional modes (a search for "torsi" finds nothing), no reed Q or damping, and no tip stiffness.
- The only reed data are length, mass and f1 for six test reeds (Table 3.1), plus the loaded-reed fits.
- It never measures the tongue's motion. Everything is microphone sound at 50 cm, apart from the large shaker-driven bars.
- **Useful for us:** the attack-time tables (finger vs bellows, p vs mf, slow key press, cassotto) with their exact metric; the growth-rate bar charts; the pitch-vs-dynamic table in cents; the luthiers' qualitative statements on clearance, set, cells, valves and "choking"; and Appendix 1's summary of Fletcher and Ricot/St Hilaire, including the equations.

---

## A. Blowing and dynamic levels (there are no Pa values, only dBA)
- p112: "La dinámica piano y la dinámica mezzo forte corresponden, respectivamente, a unos 55 dBA y 70 dBA en el sonómetro." (*Piano and mezzo forte correspond to about 55 dBA and 70 dBA on the sound level meter.*) The meter is an Extech Instruments 407727 "colocado aproximadamente a 50 cm frente a la fuente sonora".
- p122: "tres dinámicas diferentes (piano, mezzo forte y forte), que corresponden aproximadamente a 55, 70 y 80 dBA en el sonómetro". (*p, mf and f are about 55, 70 and 80 dBA.*)
- p186: the Table 4.12 levels are for the fundamental only. "Si se tomara el nivel de presión total del sonido, el valor de Lpmin estaría cerca de 55dBA (piano) y el de Lpmax cerca de 80dBA (forte)." (*The total level would be about 55 dBA at minimum (p) and 80 dBA at maximum (f).*)
- Reed-block tests (p114): "Para la excitación de las lengüetas se ha procedido a aspirar directamente a través del orificio de entrada a las cámaras de resonancia. El nivel de presión sonora ... unos 70 dBA ... mezzo forte." (*The reeds were excited by sucking directly through the cell inlet hole, at about 70 dBA, roughly mf.*) This means mouth suction, with no pressure measured.
- p184, citing Gervasoni 1986: "el nivel de presión sonora de una nota se puede variar dentro de aproximadamente 40 dB, medido a una distancia de 1 m del acordeón, sin cambiar la frecuencia". (*A note's level can vary over about 40 dB at 1 m without its frequency changing.*)
- Thresholds: none measured. Appendix 1 (p235, p237) only says in general that a minimum pressure p0 must be exceeded: "habrá que superar una presión p0 mínima para que la vibración pueda iniciarse" and "se necesita una presión mínima de soplo para que la lengüeta inicie su vibración".
- Room: p109, "una sala mediana (V≈128 m3) sin tratamiento anecoico". Microphone: B&K 4189-A21, ½", about 50 cm away (p112). The attack and steady-state microphone was "perpendicular al plano del instrumento" (p119, p121).
- A single performer was used throughout (p110): "un único intérprete".

## B. Attack transients

### B1. Method (p91, p118-p121)
- Definition, given on p91 and repeated on p118: "Los tiempos de ataque de los armónicos ... han sido calculados como los intervalos de tiempo que se extienden desde el momento en que alcanzan el umbral de los -50 dB de la amplitud máxima (en dB) hasta el momento en que alcanzan el umbral de los -5dB de la amplitud máxima (en dB)" [Braasch & Ahrens 2000]. (*Attack time of each harmonic = time from −50 dB to −5 dB relative to that harmonic's maximum amplitude.*)
- **The delay before −50 dB is excluded** (p91-p92). The accordion literature's "tiempo de respuesta" runs from the moment the player acts to a stable sound. "Una parte de este intervalo temporal queda excluido ... el intervalo inicial de tiempo que va desde el mismo momento en que se activan el fuelle o el dedo hasta el instante en que el armónico considerado alcanza el umbral de -50 dB". (*The interval from key or bellows action to −50 dB is not in T.*) p297 lists measuring from the key press as future work.
- **FFT settings as stated** (p119; the same wording is on p114 and p289): "Los espectros FFT se han tomado para 6400 frecuencias comprendidas entre 20 Hz y 20 kHz, esto es, hay una imprecisión de 3.13 Hz en la medida de la frecuencia. La duración de cada grabación ha sido de 2000 ms y la precisión en la medida del tiempo ha sido de 10 ms." (*FFT spectra with 6400 lines over 20 Hz–20 kHz, so 3.13 Hz frequency uncertainty; each recording lasted 2000 ms; time precision was 10 ms.*) The software was B&K PULSE with post-processing in MatLab (p19, p112).
- **Window length, window type, FFT size in samples, hop and overlap are never stated.**
  - *My inference, not in the thesis:* 3.125 Hz line spacing implies a 320 ms frame if it applied to the time analysis, which could not resolve a 50–60 ms rise.
  - Every frequency in the attack tables is a multiple of 12.5 Hz. A2 and A#2 are both listed as "112.5", A4 as 437.5 and A6 as 1762.5 (p152, p165, p174). That suggests an 80 ms frame (12.5 Hz lines) for the attack spectrograms.
  - Table A3.1 (p289) itself states "Error máximo en la frecuencia 10 Hz".
  - The shortest T anywhere is 50 ms. If the frame was about 80 ms, the finger-attack T values (50–140 ms) may be partly limited by the analysis window, so treat them as upper bounds on the true rise.
- Repeats: at least 6 per case in the first series (p119) and at least 15 in the second (p120). Takes the player rejected were discarded.
- Series 1 (p119): A1, A2, A3, A4 and A5 on the 8' register of the left manual, p and mf, opening and closing, finger and bellows attacks.
- Series 2 (p120): right manual, "un registro básico de una sola voz central fuera de cassotto", p and mf, finger and bellows, always opening the bellows. Notes A2, A#2, B2, A3, A#3, B3, A4, A#4, B4, A5, A#5, B5, A6, A#6, B6. Three adjacent notes were used "se minimiza la probabilidad de encontrar una lengüeta mal ajustada".
- On why reeds vary, p120: "El más pequeño cambio en la forma de la lengüeta o en su ajuste sobre la plaqueta metálica puede dar lugar a transitorios de ataque muy diferentes [Millot & Baumann 2007, Benetoux 2002, Benetoux 2005]." (*The smallest change in the tongue's shape or its fit on the plate can give very different attack transients.*)
- Definitions (p118): "duros o ataques de dedo (el fuelle se pone primero en tensión para a continuación pulsar el botón) y blandos o ataques de fuelle (el botón se pulsa antes de accionar el fuelle)". (*Hard or finger attacks: the bellows is tensioned first, then the button is pressed. Soft or bellows attacks: the button is pressed first, then the bellows moves.*)

### B2. Attack-time tables (T of the 1st harmonic)

**Table 4.7 (p151).** Left manual, 8' register, finger ("ataque duro") attacks.

| Note | p, opening | p, closing | mf, opening | mf, closing |
|---|---|---|---|---|
| A1 | 250 ms | 280 | 295 | 200 |
| A2 | 175 | 190 | 155 | 300 |
| A3 | 185 | 160 | 135 | 230 |
| A4 | 160 | 150 | 145 | 105 |
| A5 | 190 | 280 | 145 | 130 |

p151: "el tiempo de ataque no depende del sentido de movimiento del fuelle". (*Attack time does not depend on bellows direction.*)

**Table 4.8 (p152).** Right manual, single 8' voice outside cassotto, opening. The page render was checked against the text extraction.

| Note | f (Hz, as printed) | mf bellows | mf finger | p bellows | p finger |
|---|---|---|---|---|---|
| A2 | 112.5 | 450 | 80 | 660 | 110 |
| A#2 | 112.5 | 390 | 70 | — | — |
| B2 | 125.0 | 540 | 100 | — | — |
| A3 | 225.0 | 480 | 110 | 380 | 90 |
| A#3 | 237.5 | 220 | 70 | 440 | 100 |
| B3 | 250.0 | 190 | 60 | 380 | 70 |
| A4 | 437.5 | 390 | 60 | 410 | 90 |
| A#4 | 462.5 | 440 | 60 | 390 | 140 |
| B4 | 500.0 | 390 | 70 | 410 | 120 |
| A5 | 887.5 | 610 | 50 | 300 | 60 |
| A#5 | 937.5 | 630 | 50 | 320 | 80 |
| B5 | 1000.0 | 520 | 50 | 320 | 80 |
| A6 | 1762.5 | 510 | 100 | 350 | 130 |
| A#6 | 1875.0 | 430 | 60 | 330 | 80 |
| B6 | 1987.5 | 590 | 70 | 250 | 80 |

- Finger attacks in mf are 50–110 ms and in p 60–140 ms. Bellows attacks are 190–660 ms.
- p153: "con la excepción de la nota A3, los ataques de dedo en mezzo forte son más cortos que los ataques de dedo en piano. En el caso de los ataques de fuelle, el rango de dinámica no parece tener influencia alguna". (*Except for A3, mf finger attacks are shorter than p finger attacks. Dynamics has no visible effect on bellows attacks.*) The thesis blames A3 on the reed possibly being "no ... apropiadamente ajustada".
- p153: "En lo que a la influencia de la frecuencia de la nota en la duración de los ataques no se observa un comportamiento constante." (*No consistent dependence on pitch.*)
- p154: "hay una gran variación en las duraciones de los ataques, hecho que podríamos considerar inesperado para notas que difieren tan solo un semitono." The thesis attributes this to "la forma de la misma [Benetoux 2005 pp. 31-32], el ajuste de la lengüeta sobre la plaqueta metálica [Benetoux 2002 p. 41] e incluso las características de la tira de piel o de plástico ... [Benetoux 2005, p 61]", and says it "es consistente con ... Millot y Baumann ..., que encontraron diferentes comportamientos dinámicos cuando se variaban las secciones eficaces de la lengüeta". (*Large semitone-to-semitone variation, attributed to reed shape, the reed's fit on its plate and the leather or plastic valve. This is consistent with Millot & Baumann, where changing the effective sections changed the dynamics.*)

**Table 4.9 (p165).** Slow key press, first harmonic.

| Note | mf bellows | mf slow finger | mf finger | p bellows | p slow finger | p finger |
|---|---|---|---|---|---|---|
| A4 (437.5) | 390 | 80 | 60 | 410 | 220 | 90 |
| A#4 (462.5) | 440 | 150 | 60 | 390 | 180 | 140 |

- p165: "cuando disminuye la velocidad del botón, el tiempo de ataque toma un valor intermedio entre aquellos obtenidos para ataques de dedo y los obtenidos para ataques de fuelle." (*A slower button gives an attack time between finger and bellows.*)

**Table 4.10 (p174).** Cassotto, mf.

| Note | outside, bellows | outside, finger | inside, bellows | inside, finger |
|---|---|---|---|---|
| A4 | 390 | 60 | 480 | 60 |
| A#4 | 440 | 60 | 320 | 60 |

- p174: "El cassotto no tiene ningún efecto sobre la duración total de los ataques." Also: "Actúa sobre el sonido generado por la lengüeta, pero no actúa sobre la lengüeta misma." (*Cassotto has no effect on attack duration. It acts on the sound, not on the reed.*)

**Melodica, Table A3.1 (p289).** mf, Samick A-32, brass reeds on a common plate (p287).

| Note | soft | hard |
|---|---|---|
| A3 (225.0) | 260 ms | 70 ms |
| A4 (437.5) | 180 ms | 60 ms |

- p290: "Los valores de los ataques duros son equivalentes a los de la misma nota del acordeón." (*The hard attacks match the accordion's.*)

**Low notes (conclusions, p220).** "en mezzo forte, independientemente de cómo sea el resto de parámetros (dureza, sentido del fuelle) las notas de aproximadamente la primera octava más grave tienen siempre un tiempo de ataque más largo ... Todas estas lengüetas están cargadas". (*In mf the lowest octave always has a longer attack, whatever the other parameters; all those reeds are tip-loaded.*) Compare Table 4.7, where A1 is 200–295 ms.

### B3. Growth shape and rates
- **Fig 4.11 (p150).** SPL (dB) of harmonics 1–5 of A4 in mf, 0.3–0.7 s. Values are fig-read.
  - (b) Finger attack: H1 leaves the ~10–25 dB floor at about 0.43 s, is at about 38 dB at 0.475 s, about 60 dB at 0.50 s, about 80 dB at 0.53 s, and plateaus at about 88 dB by 0.56–0.57 s. The steep part is about 40→80 dB in about 55 ms, roughly 0.7–0.8 dB/ms. H2–H5 start later, at about 0.48 s, and rise more steeply to 85–92 dB.
  - (a) Bellows attack: H1 goes from about 15 dB at 0.35 s to about 72 dB at 0.50 s (about 0.4 dB/ms), then creeps to about 78 dB by 0.7 s. H2–H5 rise at about 0.43–0.45 s and plateau at 65–82 dB.
- p178: "Los armónicos del sonido generado en un ataque de fuelle tienen una pendiente de crecimiento menor que la de los armónicos del correspondiente ataque de dedo." (*Bellows-attack harmonics grow with a smaller slope.*)
- **Fig 4.15 (p159)**, A#3 mf. The rate of change of SPL (dB/ms) in each quarter of T, as 0–¼T / ¼–½T / ½–¾T / ¾T–T. Fig-read:
  - Finger, H1: 1.18 / 0.38 / 0.75 / 0.45. H2: 1.31 / 0.75 / 0.37 / 0.20. H4: 1.62 / 0.84 / 0.34 / 0.26. First-quarter rates for H1–H7 are 0.8–1.6 dB/ms.
  - Bellows, H1: 0.38 / 0.29 / 0.10 / 0.04. H4: 0.96 / 0.40 / 0.20 / 0.07. The other harmonics are about 0.3–0.55 in the first quarter and 0.0–0.2 afterwards.
- **Fig 4.16 (p160)**, A#3 p, finger (fig-read): H1 0.48 / 0.44 / 0.49 / 0.34. H3 0.48 / 0.54 / 0.81 / 0.54. H4 0.84 / 0.72 / 0.93 / 0.47. H6 and H7 are 0.1–0.56. H2 and H5 are not plotted.
  - The bellows panel (a) of Fig 4.16 appears to be the same image as Fig 4.15(a), with identical bars. This may be a figure-copy error in the thesis.
- p158: "Los ataques de fuelle muestran una velocidad más alta de cambio del nivel de presión sonora durante el primer cuarto del tiempo de ataque de cada armónico. Para el resto del ataque, esta velocidad es muy baja. En cambio, para los ataques de dedo, esta velocidad de cambio está más equilibrada durante todo el ataque." (*Bellows attacks rise fastest in the first quarter and then very slowly. Finger attacks rise at a more even rate throughout.*)
- **Fig 4.12 (p155)**, A#3 mf, start (−50 dB) and end (−5 dB) of H1–H7. Fig-read:
  - Finger: starts at 0 ms for H1 and about 20 ms for H2–H7; ends at 70 / 100 / 100 / 80 / 120 / 100 / 110 ms.
  - Bellows: starts at 0–70 ms; ends at 220 / 470 / 380 / 160 / 430 / 350 / 280 ms.
- p154: "En los ataques de dedo en mezzo forte, todos los ataques comienzan y finalizan casi simultáneamente. En los ataques de fuelle en mezzo forte, los armónicos comienzan casi simultáneamente, pero acaban en instantes diferentes." (*In mf finger attacks all harmonics start and end almost together. In bellows attacks they start together but end at different times.*)
- Spectral centroid (p163): "en el comienzo del ataque el valor del centroide está cercano a 8500-9000 Hz ... Al final del ataque todos los centroides permanecen cercanos a 2500 Hz" (A#3 mf). The finger-attack centroid dips below the bellows one "entre 0.7T y 1.7T".
- **Derived by me, not in the thesis.** T spans 45 dB, which is 5.18 nepers of amplitude. So the mean amplitude growth rate is σ ≈ 5.18/T and the e-folding time is τ ≈ T/5.18.

| Attack | T | mean rate | τ (e-folding) |
|---|---|---|---|
| Finger mf | 50 ms | 0.90 dB/ms | 9.7 ms |
| Finger mf | 60 ms | 0.75 dB/ms | 11.6 ms |
| Finger mf | 80 ms | 0.56 dB/ms | 15.4 ms |
| Finger mf | 110 ms | 0.41 dB/ms | 21 ms |
| Finger p | 140 ms | 0.32 dB/ms | 27 ms |
| Bellows | 390 ms | — | 75 ms |

  - In cycles: A4 finger mf (60 ms) is about 26 cycles over the whole 45 dB. A2 finger mf (80 ms) is about 9 cycles.
  - These rates agree with the Fig 4.15 bars: the H1 finger quarter-rates average 0.69 dB/ms × 70 ms ≈ 48 dB.
  - The fig-read steep part of Fig 4.11(b) is also about 0.8 dB/ms, so τ ≈ 11 ms.
  - To compare our model, compute the same −50→−5 dB interval of the 1st harmonic's envelope, not the time from a kick.

### B4. Pallet or valve opening speed, and attack physics as the thesis describes it
- p164: "Los ataques usuales de dedo son realizados en unos 0.05 s aproximadamente. Los ataques con pulsación lenta pueden llevarse a cabo tan lentamente como se desee, pero su duración suele ser de unos 0.50 s." (*A normal finger attack is executed in about 0.05 s. A slow press can be as slow as desired but usually lasts about 0.5 s.*) This is the only number for button or pallet travel time.
- p164: "el fuelle está tenso antes de que actúe el dedo. Por lo tanto, el espacio por donde sale el aire a través de la cámara de la lengüeta queda libre rápidamente." (*The bellows is already tense, so the air path through the reed chamber opens quickly.*) For the slow press: "el espacio a través del cual pasa el aire en la cámara de la lengüeta se va a abrir ahora lentamente ... y el flujo de aire va a crecer con menos rapidez". (*The path opens slowly and the flow grows less quickly.*)
- p178: "Una vez que el fuelle está en tensión, la velocidad a la que se pulsa el botón se convierte en el parámetro que regula la mayor o menor brusquedad ... La abertura creciente que se genera por la pulsación del botón ... es el único obstáculo que dificulta el paso del aire ..." For the bellows attack: "Dado que la caja izquierda tiene una mucha mayor inercia (masa) que los botones, el ataque de fuelle se siente más lento". (*With the bellows tensioned, button speed sets how abrupt the attack is, and the growing pallet opening is the only obstruction. The bellows attack feels slower because the left box has far more inertia than a button.*)
- **Blowing-pressure model** (p179): "Un ataque de dedo podría ser modelado mediante una presión de soplo de tipo escalón de Heaviside. Análogamente, un ataque de fuelle podría ser representado mediante una presión de tipo rampa." (*A finger attack could be modelled as a Heaviside step in blowing pressure, a bellows attack as a ramp.*) It cites Bergeot et al. 2012 and Almeida et al. 2015 for the clarinet. On p179 it adds: "Los efectos transitorios se incluyen también en el modelado ... Millot y Baumann ... con el flujo entrante correspondiendo a lo que sería un ataque de dedo." (*Millot & Baumann's transient corresponds to a finger attack.*)
- **Venturi effect on a slowly opened pallet** (p180): "tenemos un rápido flujo de aire que circula a través de una abertura muy pequeña (el hueco entre la válvula y el bloque de lengüetas). Se puede originar así un efecto Venturi que puede dificultar la apertura de la válvula ... El intérprete siente el botón más duro ... y el comienzo del ataque puede ser inestable." (*Fast flow through the small pallet gap can create a Venturi effect that resists opening; the button feels stiffer and the onset can be unstable.*) p207 says the same for partial-pallet bending: "el efecto Venturi tienda a cerrar la zapata ligeramente entreabierta".
- **Initial tongue displacement and the first cycles are not described anywhere.** The thesis never measured the tongue. Appendix 1 (p235) only says generically: "Por lo general, las pequeñas oscilaciones de la lengüeta tienden a crecer rápidamente hasta vibrar a gran amplitud." (*Small oscillations generally grow rapidly to large amplitude.*)

### B5. Reed adjustment (clearance, set), valves and cassotto, as they bear on attack
- **Clearance ("luz")** (p46): "Cuanto menor sea el espacio (menor 'luz') entre la lengüeta y el orificio de la plaqueta sobre la que se coloca aquella, más rápida será la respuesta de la lengüeta y mejores pianissimi se podrán lograr [Alberdi et al. 2005-2007]." (*The smaller the gap between tongue and slot, the faster the response and the better the pianissimi.*) There is no number.
- **Set** (p153): lutieres "recomiendan una disposición especial de la lengüeta en equilibrio (ligeramente más elevada que lo normal sobre el plano del portalengüetas) para minimizar este retardo en la respuesta de las lengüetas más graves [Alberdi & Orobengoa 2015]." (*Luthiers recommend setting the lowest reeds slightly higher than normal above the plate to reduce their response delay.*) This is a personal communication and gives no number.
- p153 on the long low attacks: "es probable que sea la carga la característica diferenciadora" (*the tip load is probably the distinguishing factor*).
- Fit precision (L2, p271): on the Sirius low reeds in a common plate, the better piano response is attributed to "la forma diferente de las lengüetas (menos trapezoidal) y el preciso corte de las placas por electroerosión creando un ajuste realmente preciso ..., las fuga de aire en el momento de la obturación en la vibración es mínima." (*A less trapezoidal tongue and an EDM-cut slot give a really precise fit, so air leakage at closure is minimal.*)
- Reglaje delays onset (L2, p267-p268): on the new Sirius, whose low-reed cells were resized, "No cree que este nuevo diseño la entrada se dé antes (de hecho, el que les ha llegado entra más tarde, por el propio reglaje de las lengüetas sobre los portalengüetas)." (*He does not think the new design speaks earlier; the one they received actually speaks later, because of how the reeds are adjusted on their plates.*)
- Tongue thickness (L1, p262): "Las lengüetas muy finas, muy delgadas empiezan muy fácil pero no tienen luego fuerza, no llegan a dar gran intensidad." (*Very thin tongues start very easily but lack force and never get loud.*)
- Plate (slot) thickness, L1 (p263): "Cuando la lengüeta está dentro del canal de la plaqueta, 'come' menos aire. Hay plaquetas que son más gruesas por la punta ... las lengüetas más graves llevan un portalengüetas más grueso." (*Inside the slot the tongue "eats" less air. Some plates are thicker at the tip, and low reeds have thicker plates.*)
- Plate (slot) thickness, L2 (p270-p271): the plate's role is "la obturación del paso del aire ... (que creo que es donde más incide y más importante es)". With a very thin plate the tongue "enseguida pasa al otro lado ... la amplitud de vibración es pequeña y la sonoridad final también". (*Closure of the air path is the key moment. With a thin plate the tongue passes through at once, so amplitude and loudness are small.*)
- Plate (slot) thickness, L3 (p280): with a thinner plate "la lengüeta tendría menos resonancia, puesto que ... tendría menos recorrido en el interior". (*Less travel inside the slot means less resonance.*)
- Leather and plastic valves ("badana"): p154 lists them among the causes of attack variability. L3 (p280): a stiff opposite valve "podría llegar a estorbar el paso del aire reduciendo la frecuencia y la sonoridad y la brillantez". (*A stiff valve could obstruct the air path, lowering frequency, loudness and brightness.*) Plastic valves in the bass "meterían mucho ruido". A defective leather valve gives "el típico ronquido".
- Cassotto: no effect on attack duration (p174, p219).
- Direction: bellows direction has no clear effect (p151, p219). Players still prefer to open for fast response, possibly for physiological reasons (p151).

---

## C. Reed and tongue data
- Materials (p45): "Las plaquetas están hechas de duraluminio ... y las lengüetas de acero templado [Alberdi et al. 2005-2007]." Young's modulus: "los aceros de baja aleación ... entre 200 y 207 GPa, los aceros inoxidables entre 190 y 200 GPa, y el aluminio unos 69 GPa." The same page prints aluminium density as "(2.7 kg/m3)", a typo for 2.7 g/cm³.
- Quality grades (p46): "comerciales", "acabadas a mano", "tipo a mano", "hechas a mano".
- Shapes (p46): "más rectangulares o más trapezoidales". L3 (p276, p278): Italian reeds are "más blanda ..., más grosor y la forma es de punta de lanza"; German and Russian reeds are harder and more rectangular. L1 (p265): the Jupiter has "lengüetas más rectangulares (en lugar de las más afiladas del Sirius)".
- Tuning (p47): "quitar material de la lengüeta: de la punta para elevar el tono y del cuerpo de la lengüeta, sobre todo cerca del remache, para bajarlo". (*Remove material at the tip to raise pitch, and from the body near the rivet to lower it.*)
- Sirius note range (p182): "desde la nota E1 (f1=41.49 Hz) hasta la nota C#8 (f1=3969 Hz) tomando para la nota A4 una frecuencia f1= 442 Hz". (*Tuned to A4 = 442 Hz.*)
- Common-plate low reeds (p48): on the best concert instruments "las lengüetas correspondientes a las notas más graves de la mano izquierda (unas dos octavas) ... se remachan directamente sobre una placa única". On p222 the Sirius used here has only the lowest left-hand reeds in a plate.
- Left vs right low reeds (p222): "las del manual derecho tienen más carga y son más cortas y notablemente más estrechas que las equivalentes del manual izquierdo (más largas y con menos carga)". (*Right-manual bass reeds are more loaded, shorter and much narrower.*)

**Test reeds, Table 3.1 (p115).** Each on its own plate, on a luthier-built block with individual cells (Fig 3.1, p114). m is the tongue mass, M the tip load.

| Reed | L (m) | m (kg) | M (kg) | f1 (Hz) |
|---|---|---|---|---|
| 1 | 0.042 | 0.68×10⁻³ | — | 142.7 |
| 2 | 0.037 | 0.29×10⁻³ | — | 226.4 |
| 3 | 0.042 | 0.68×10⁻³ | 0.08×10⁻³ | 121.9 |
| 4 | 0.052 | — | M4 | 87.5 |
| 5 | 0.052 | — | M5 | 71.9 |
| 6 | 0.052 | — | M6 | 62.5 |

- p116: M4 < M5 < M6, but the exact values are unknown because the luthier removed material while tuning.
- *Derived by me, not in the thesis:* a uniform Euler-Bernoulli bar (steel, E = 200 GPa, ρ = 7850) would give h ≈ 0.31 mm and w ≈ 6.7 mm for reed 1, and h ≈ 0.38 mm and w ≈ 2.6 mm for reed 2. Real reeds are tapered, so these are only order-of-magnitude.

**Tip-load fits, (f1/f1')² vs M/m.**
- Reed 1 (p135): (f1/f1')² = 3.5 (M/m) + 1.00, with M = 0.14m, 0.19m and 0.33m.
- Reed 2 (p136): (f1/f1')² = 3.7 (M/m) + 0.98, with M = 0.05m, 0.32m, 0.44m and 0.80m.
- Theory, Eq. 2.18 (p70): (f1/f1')² = 1 + 4.1 M/m. p137: "ligeramente más pequeños" (*the measured slopes are slightly smaller*).

**Load position, Table 4.1 (p138, reed 1)** and **Table 4.2 (p140, reed 2).** Slope of (f1/f1')² vs M/m.

| Reed 1 position | slope | R² |
|---|---|---|
| L | 3.5 | 0.999 |
| 0.88L | 2.5 | 0.999 |
| 0.76L | 1.6 | 0.988 |
| 0.64L | 0.8 | 0.988 |
| 0.52L | 0.4 | 0.957 |
| 0.40L | 0.2 | 0.976 |

| Reed 2 position | slope | R² |
|---|---|---|
| L | 3.7 | 0.996 |
| 0.86L | 2.1 | 0.997 |
| 0.73L | 1.1 | 0.998 |
| 0.59L | 0.5 | 0.989 |
| 0.46L | 0.2 | 0.927 |
| 0.32L | 0.03 | 0.765 |

**Table 4.3 (p141).** Loaded reed vs an unloaded reed with the same f1 (via Eq. 2.8).
- Reed 1:
  - 97.3 Hz: 0.90 g at L = 0.042 m, equivalent to an unloaded 0.83 g at 0.051 m.
  - 110.8 Hz: 0.81 g, equivalent to 0.78 g at 0.048 m.
  - 117.5 Hz: 0.78 g, equivalent to 0.74 g at 0.046 m.
- Reed 2:
  - 114.0 Hz: 0.52 g at L = 0.037 m, equivalent to 0.41 g at 0.052 m.
  - 144.3 Hz: 0.42 g, equivalent to 0.36 g at 0.046 m.
  - 151.0 Hz: 0.38 g, equivalent to 0.35 g at 0.045 m.
  - 211.3 Hz: 0.30 g, equivalent to 0.30 g at 0.038 m.

**Beam theory used (Ch. 2).**
- Eq. 2.7 (p66): β = 1.875, 4.694, 7.855.
- Eq. 2.8: f1 = 0.162 (h/L²) √(E/ρ).
- Eq. 2.10 (p67): the equivalent tip mass is m_e ≈ 0.24 m, giving f1 = (1/2π) √(3EI/(0.24 m L³)).
- Eq. 2.17: with a tip load, f1' = (1/2π) √(3EI/((M + 0.24m) L³)).
- p65: the thin-bar validity condition is κ/L < 0.1.
- p60: "La lengüeta vibrará de manera mantenida con una frecuencia fundamental ligeramente inferior a la frecuencia fundamental del primer caso [punteada]." (*When blown, the reed vibrates slightly below its plucked frequency.*) No number is given.

**Modes and nodes (large bars, 13.5 cm, 4.31 g, shaker-driven; p116, Table 4.6 p147).**
- Unloaded first mode at 21 Hz (p116).
- Nodes (the clamp excluded) for M/m = 0, 0.1, 1.0 and 5.0:
  - 2nd mode: 0.783L, 0.841L, 0.953L, 0.988L.
  - 3rd mode: 0.504L and 0.868L; 0.530L and 0.921L; 0.550L and 0.983L; 0.550L and 0.996L.
- Fits of (f1/f1')² over different M/m ranges (Table 4.5, p145):
  - 0–5.0: slope 3.0, intercept 1.3.
  - 0–0.63: slope 3.4, intercept 1.1.
  - 0–0.26: slope 3.7, intercept 1.1.
- The tip load moves the bar from fixed-free towards fixed-pinned.

**Damping (large bar only, Fig 4.10, p149).** Half-power bandwidth of the tip displacement squared, fig-read and approximate:

| M/m | FWHM |
|---|---|
| 0 | ≈ 2 Hz (at 21 Hz, so Q ≈ 10) |
| 0.1 | ≈ 1.7 Hz |
| 1 | ≈ 0.9 Hz |
| 5 | ≈ 0.35 Hz |

- p148: "Cuando el valor M de la carga crece, la anchura de banda disminuye." This is a shaker-clamped 13.5 cm bar, not an accordion reed.

**Reed spectrum (Fig 4.25, p182).** An unloaded reed with f1 = 144 Hz, blown. Fig-read peaks: H1 ≈ 60 dB, H2 ≈ 57, **H3 ≈ 79 (strongest)**, H4 ≈ 55, H5 ≈ 62, H6 ≈ 45, H7 ≈ 55.
- The 2nd transverse partial, near 900 Hz, is "imperceptible".
- p182: "Los parciales de la barra suelen ser mucho más débiles que los armónicos producidos por el flujo de aire y a menudo son imperceptibles."

**Fine spectral structure (Table 4.4, p142).** Loaded reeds have fss 0.79–0.90; the unloaded reed has "valores cercanos a 0.4", so odd harmonics dominate.
- Loaded reeds 4, 5 and 6 (87.5 / 71.9 / 62.5 Hz) have centroids 1298 (≈15 f1), 1257 (≈17 f1) and 970 Hz (≈16 f1).

## D. Chamber (cell), tone hole, pallet, bellows
- **No dimensions are given for any of these.**
- Construction (p44, p47-p48, p50): two reeds per plate, one on each side. The opposite slot is covered by a leather or plastic strip. Plates are waxed onto wooden blocks ("somieres, peines o bloques", usually poplar), which have one "alvéolo" (inlet) per cell. The pallet ("zapata") uncovers the alvéolo.
- Right manual (p51): a button lifts two pallets, one per pair of blocks (cassotto pair and outside pair). Register slides block alvéolos.
- **Valves on the Sirius** (p47): "todas las lengüetas, hasta las correspondientes a la nota Sol4, llevan su correspondiente tira de piel. Desde la nota Sol#4 hasta la nota Do6 tienen una tira de plástico en vez de piel. Desde esa nota hasta la nota más aguda, Do#8, ya no hay ningún tipo de tira." (*Leather up to G4, plastic from G#4 to C6, no valve from above C6 to C#8.*)
- Cell sizing:
  - L2 (p267): "Un tamaño demasiado grande o pequeño hará que la lengüeta vibre poco (poca amplitud) o se ahogue. En las lengüetas agudas la cámara es menor y normalmente la entrada está rebajada y en las más agudas están giradas para que el aire incida directamente sobre la punta de la lengüeta." (*A cell too big or too small makes the reed vibrate weakly or choke. High reeds have smaller cells with chamfered inlets, and the highest are turned so air hits the tip directly.*)
  - L2 (p267): on a demo cell that was too large, "la lengüeta no termina de abrir al batir" (*the reed never fully opens as it beats*).
  - L2 (p267): new Sirius low cells, resized, are less "ahogada": they admit "una presión un poco mayor que la normal sin bajar la frecuencia y sin sonar con exceso de aire".
- Filled cells (L2, p268): high-reed cells are "rellenadas, 'encogidas', mediante madera", with a graded entry "para que el flujo de aire no se encuentre un escalón". Register slides force equal alvéolo sizes along a block.
- Inlet size (L2, p268): "Cuanto menor sea el hueco (la superficie del alvéolo), mayor va a ser la presión." (*The smaller the inlet, the higher the pressure in the cell.*)
- Cell volume (L2, p269): "Creo que la influencia del volumen de la cámara es pequeña ... lo importante es la velocidad a la que accede el aire contra la lengüeta." (*Cell volume matters little; what matters is how fast the air reaches the reed.*) This is an opinion.
- Distance from plate to alvéolo, the "base" (L1, p262, p264): "más alta ... dificulta la emisión de sonido en el caso de lengüetas pequeñas", and "Cuanto menos base, más fácil la emisión ... En las diatónicas, en que se busca más rapidez de emisión ..., esa base está limitada al mínimo." (*A taller base makes small reeds harder to start; less base gives easier, more immediate speech.*)
- Tip near the inlet (L1, p262; also p254): mounting small reeds "cabeza abajo" (tip near the alvéolo) "favorece la emisión del sonido". p254 (Tonon plus the luthiers): the highest reeds have their valves removed and "se montan ... con la punta cerca del alvéolo".
- Larger cell (L3, p276): a bigger cell mainly raises air consumption. The cell floor is relieved "para que la punta no toque el fondo de la cámara". Excess air can block the reed, or the opposite valve "no aguante y llegue a vibrar por el centro".
- Piccolo relief (L3, p278): for piccolos he drills "un agujero en la madera del somier para que el aire escape y la presión sobre la lengüeta no sea excesiva y pueda ahogarse". (*A hole in the block lets air escape so the reed does not choke.*)
- Tonon's cavity results (p253-p254): a cell acts as a Helmholtz resonator when it is small against the wavelength. "si el modo de resonancia de la cámara coincide o es algo menor que la fundamental de la lengüeta, la interferencia es fuerte ... Esta interferencia desaparece si el modo resonante de la cavidad queda por encima de la fundamental". A quarter-wave cell conflicts with the reed's need for a velocity antinode and "puede incluso a ahogar completamente la vibración".
- Bellows (L1, p265): "El peso de la caja izquierda hace que se abra por sí mismo". No volume or geometry is given. p182: "la extensión máxima del fuelle limita la duración máxima de las notas".

## E. Pitch vs dynamic, bending, level vs dynamic, spectra
- **Pitch vs dynamic, Table 4.12 (p185).** The first harmonic during a crescendo from p to f over 10 s, sampled every 1 s, 3.13 Hz lines. The max error is 1 Hz (p186).
  - **The cents are quantized to 1 Hz steps.** At E4, 1 Hz ≈ 5 cents; at E2 ≈ 21 cents; at E1 ≈ 41 cents.
  - Lp is the fundamental's level in dBA. The 16' (E1) register is in cassotto; "cass" and "no" below mean inside or outside cassotto.

| Note, register | Bellows | f min–max (Hz) | Δf (cents) | Lp min–max (dBA) |
|---|---|---|---|---|
| E1 / 16' | open | 41–42 | −41 | 26.6–45.6 |
| E1 / 16' | close | 39–42 | −127 | 31.7–44.8 |
| E2 / 8' cass | open | 83–83 | 0 | 31.9–47.2 |
| E2 / 8' cass | close | 82–83 | −21 | 35.7–47.0 |
| E2 / 8' no | open | 83 | 0 | 47.0–62.6 |
| E2 / 8' no | close | 81–83 | −41 | 40.9–48.3 |
| E3 / 8' cass | open | 166 | 0 | 37.5–58.1 |
| E3 / 8' cass | close | 164–166 | −21 | 43.4–57.5 |
| E3 / 8' no | open | 166 | 0 | 45.5–60.2 |
| E3 / 8' no | close | 164–166 | −21 | 39.2–57.4 |
| E4 / 8' cass | open | 331–332 | −5 | 49.0–70.7 |
| E4 / 8' cass | close | 330–331 | −5 | 48.4–63.3 |
| E4 / 8' no | open | 331 | 0 | 36.1–63.3 |
| E4 / 8' no | close | 331–332 | −5 | 36.6–56.2 |
| E5 / 8' cass | open | 662–664 | −5 | 55.4–76.8 |
| E5 / 8' cass | close | 662–664 | −5 | 46.4–69.4 |
| E5 / 8' no | open | 662 | 0 | 47.0–72.2 |
| E5 / 8' no | close | 662 | 0 | 48.2–61.2 |
| E6 / 8' cass | open | 1325 | 0 | 33.7–62.3 |
| E6 / 8' cass | close | 1325 | 0 | 40.7–60.2 |
| E6 / 8' no | open | 1325–1328 | −4 | 40.4–67.1 |
| E6 / 8' no | close | 1325 | 0 | 40.3–59.7 |
| E7 / 8' cass | open | 2650–2656 | −4 | 47.8–65.8 |
| E7 / 8' cass | close | 2650 | 0 | 44.5–54.9 |
| E7 / 8' no | open | 2650–2656 | −4 | 44.5–65.6 |
| E7 / 8' no | close | 2650–2653 | −2 | 44.4–57.0 |

  - The fundamental's level spans 7.4–28.6 dB across the crescendo.
- Fig 4.26 (p186), E1 16' in cassotto (fig-read): opening, 42 Hz from 26 to 37 dBA, then 41 Hz from 39 to 46 dBA. Closing: 41–42 Hz up to about 38 dBA, 40 Hz at 40–42 dBA, 39 Hz at 42–45 dBA.
- Fig 4.27 (p187), E4 in cassotto, opening (fig-read): 332 Hz at 49–50 dBA, then 331 Hz from 54.5 to 71 dBA.
- p187: "la frecuencia se mantiene estable en un rango de la dinámica que depende de la nota, y que es más estable cuanto más alto es el tono de ésta. El valor de la frecuencia disminuye al aumentar la presión del fuelle, y el cambio de frecuencia es pequeño en la mayoría de los casos." (*Frequency is stable over a note-dependent dynamic range, more stable the higher the note. It decreases as bellows pressure increases, and the change is small in most cases.*)
  - The thesis's resolution cannot show a −9 cent drop at mid pitches: the only mid-range entries are 0 or −5 cents, which is one 1 Hz step at E4–E5.
- p123: "la lengüeta libre se caracteriza por una cierta estabilidad de la frecuencia frente a cambios de presión [Gervasoni 1986, pp. 112-113]".
- p124, the "fourth method": "la acción sobre el fuelle crece considerablemente mientras el botón se encuentra pulsado en su totalidad. Se da una pequeña caída de presión, inapreciable en la práctica". This probably means a small pitch drop, and the wording is ambiguous. It cites Cottingham 2005, Cottingham 2010 and Coyle 2009.
- p252 lists as missing from Ricot's model: "sería importante realizar un análisis de la dependencia de la frecuencia con la presión".
- **Amplitude and saturation with pressure:**
  - p183: "Cuando se le aplica más presión, la amplitud de oscilación de la lengüeta aumenta, dando lugar a un volumen sonoro mayor." (*More pressure, larger tongue amplitude, louder.*)
  - p221: "cuando la sonoridad aumenta mucho la amplitud de los armónicos puede disminuir. Este resultado es consistente con el hecho de que al aumentar la presión del fuelle una lengüeta puede 'ahogarse'." (*At very high loudness the harmonic amplitudes can fall, consistent with a reed "choking" as bellows pressure rises.*) Students must not use "una fuerza mayor que la que las lengüetas pueden soportar sin empezar a ahogarse y disminuir su respuesta".
  - L2 (p273): "¿Por qué se ahoga una lengüeta cuando se tensa mucho el fuelle? — La razón es que la presión es mucho mayor que la fuerza, elasticidad del acero, impidiéndose la oscilación." (*The pressure far exceeds the steel's restoring force, which prevents oscillation.*)
  - L3 (p276): "la lengüeta puede bloquearse, por el exceso de presión".
  - L2 (p270): harder steel gives "una vibración más precisa en frecuencias a diferentes presiones".
- Centroid vs dynamic (Table 4.15, p188), p / mf / f:
  - E1 left 8', Accordion B: 405 / 467 / 487 Hz.
  - E2 right 8' outside cassotto: 925 / 1025 / 945 Hz.
  - p189 explains the non-monotonic E2 partly by "las frecuencias de los armónicos pueden caer ligeramente al aumentar la dinámica".
- Centroid by register, E4 in mf (Table 4.16, p189): 16' cass 1078, 8' cass 1389, 8' outside 2013, 4' 1820 Hz.
- Centroid by note:
  - Right-hand 16', mf (Table 4.14, p188): E1 511, E2 731, E4 1078, E6 3369 Hz.
  - Left 8', mf (Table 4.13, p188): E1 620, A#1 885, G2 1456 Hz.
  - Left-hand registers, E4 (Table 4.17, p191): 8' 1665, 4' 2006, 2' 1942 Hz.
- Button depth (Table 4.11, p184): a partially pressed button lowers the level by 3–10 dBA. Examples:
  - 16' E1 65 → 58 dBA; E4 73 → 70.
  - 8' outside E6 74 → 64.
  - 4' E3 74 → 65.
- **Pitch bending (partial pallet):**
  - Series 1 (p200): "Todas las notas tienen porcentajes de cambio de frecuencia que varían entre el 1% y el 8%, estando la frecuencia final entre 0.99 y 0.92 veces el valor de la frecuencia original, que supone un cambio máximo de un semitono, a excepción de las notas más graves del registro de 16 pies, que sufren una caída mayor". (*Frequency drops of 1–8%, to 0.99–0.92 of the original, at most about a semitone, except the lowest 16' notes, which fall further.*)
  - Fig 4.33b (p197), 16' E1–D#3 (fig-read): 25–120 cents. E1–G1 fall about 95–120 cents; mid notes 25–75.
  - Series 2, Table 4.19 (p204), A#2–A3, 8' outside cassotto, 5 s. Mean, maximum and minimum drop are 22 / 34 / 16 cents for bellows plus finger, and 15 / 22 / 9 cents for finger only.
  - The level drop, fig-read from Fig 4.39 (p205), averaged over 20 harmonics, is about 7–13 dB for bellows plus finger and about 11–16 dB for finger only.
  - p205: "la lengüeta libre es un dispositivo controlado por el flujo ... Cuando una válvula se va cerrando gradualmente, una cantidad cada vez menor de aire llega a la lengüeta, disminuyendo la energía, así como su frecuencia y su amplitud." (*The free reed is flow-controlled: as the valve closes, less air reaches the reed and its energy, frequency and amplitude fall.*)
  - Fig 4.37 (p202): all harmonics fall by the same number of cents, so harmonicity is preserved.
  - p208: "Ha sido imposible realizar el pitch bending con aquellas lengüetas cuyas alturas van de C6 a C#8, precisamente las lengüetas que no tienen tira ... Esto podría indicar que es necesaria la hermeticidad de la cámara ... La viscosidad podría jugar un papel clave". (*Bending was impossible from C6 to C#8, exactly the reeds without a valve strip, suggesting the cell must be airtight so all air passes the vibrating reed.*)
  - The literature (p125) says a bend cannot raise pitch, the maximum is about a semitone [Buchmann 2010], and "El ruido del aire y el zumbido de la lengüeta pueden hacerse audibles".

## F. Physical models the thesis presents or cites (Appendix 1)
- Reed type (p72): the accordion reed is the (−,+) valve (Fletcher). Section 2.1.4 and A1.1.3 (p239-p240): it starts as (−,+). "si la diferencia de presión ... es suficientemente grande, pasará al otro lado del portalengüetas, y su configuración será entonces (+,+)". "Dado que las lengüetas del acordeón son largas y estrechas el flujo principal pasará por los lados de la lengüeta, por las dos rendijas". (*At large enough pressure difference the reed passes through to the other side of the plate and becomes (+,+). Being long and narrow, most flow goes through the two side slits.*)
- **Fletcher model** (p231-p236):
  - Eq. A1.1: Ū = (2/ρ)^½ W (p̄1 − p̄2)^½ x̄.
  - Eq. A1.2: ẍ + 2k ẋ + ω0² (x − x0) = (1/m)(σ1 p1 S1 + σ2 p2 S2) + (1/m) p2 S3.
  - Eq. A1.4, linearised: −ω² x̂ + 2jkω x̂ + ω0² x̂ = (2W p̄1 (σ2 μ2 Z2 − σ1 μ1 Z1)) / ((2ρ p̄1)^½ + W x̄ (Z1 + Z2)) · x̂.
  - Onset conditions, Eq. A1.5a–c. (−,+): ω < ωr and X1 + X2 > 0. (+,−): ω > ωr and X1 + X2 < 0. (+,+): (ω − ωr)/(X1² − X2²) > 0 and X1 − X2 < 0.
  - p236: a duct's low-frequency reactance is X = ω(ρl/S), an inertance, "la presencia de un tubo de entrada o de salida favorecerá el inicio de las vibraciones". **A cavity, X = −ρc²/(Vω), "añadirá rozamiento".** (*An inlet or outlet duct helps onset; a cavity volume adds friction.*)
  - On the free-reed aperture (p232): "la contribución al flujo total por parte de los lados de las lengüetas dominará sobre la contribución de la punta, y la abertura variará como x²" [Fletcher & Rossing 1998 p. 413]. (*Side flow dominates tip flow, and the aperture varies as x².*) The render of p232 confirms it prints x².
  - Also on p232: a flow-inertia correction "introduce un pequeño retraso en el flujo ... será el que explique el mantenimiento de las oscilaciones de la lengüeta libre [Ricot et al. 2005]". (*A small flow lag from air inertia in the channel is what sustains free-reed oscillation.*)
  - p233 and p236: only the fundamental mode is needed [Ricot 2005, Fletcher 1993, Cottingham, Lilly, Reed 1999].
- **Why St Hilaire / Ricot** (p239): "St. Hilaire et al. mostraron que la inercia del flujo de aire hacia la lengüeta y a través de su canal añade una inertancia acústica importante que resulta ... en un amortiguamiento negativo para la configuración (−,+) y una resistencia positiva para la configuración (+,+)", and "el pequeño canal del propio portalengüetas, favorecerá aún más la configuración (−,+) ... tendrá una presión umbral de inicio ... menor". (*The inertia of the flow towards and through the channel adds an important inertance, giving negative damping for (−,+) and positive resistance for (+,+). The plate's own short channel favours (−,+) further and gives it the lower onset threshold.*)
- **Ricot–Caussé–Misdariis** (p241-p252):
  - p242, the steady Bernoulli force does no net work: "la transferencia de energía entre un suministro de presión constante y la oscilación de la lengüeta no es posible si consideramos tan solo la fuerza de Bernoulli, deducida a partir de la ecuación estacionaria de Bernoulli, que dependa solo de la apertura ..., ya que el trabajo realizado por dicha fuerza a lo largo de un ciclo es cero [Titze 1988, Hirschberg 1994, Fabre 2012]". (*With only a steady Bernoulli force that depends on aperture alone, no energy can pass from a constant supply to the reed, because its work over a cycle is zero.*)
  - p243: "el efecto inercial del flujo aguas arriba es el responsable del mecanismo de excitación". Vortex shedding is ruled out (St Hilaire 1971).
  - Flow (p243-p244): upstream it is laminar, incompressible and inviscid, a potential flow of "dos sumideros situados a lo largo de las rendijas laterales". Separation happens at the reed edges. "el flujo saliente no toma parte en el mecanismo de oscilación". The reed channel is the outlet, with length equal to the plate thickness.
  - Unsteady Bernoulli, Eq. A1.6 (p245): ∂φ/∂t + |v|²/2 + P/ρ0 = C(t).
  - Force on the upstream face, Eq. A1.7 (p246): F1(t) = −h (ρ0/2) V²(t)(1 − A1(t)) + ρ0 h² A2(t) ∂V/∂t − ρ0 h A3(t) ∂q/∂t, with q = v·e (e is the slit width).
  - The A1 contraction term "resulta ser proporcional a e²(t), estando por tanto en fase con la apertura ..., no transfiriendo por tanto energía". The inertial terms "no están en fase con el desplazamiento". The Bernoulli term dominates.
  - Eq. A1.8 (p247): ⟨P1(t)⟩exc = −ρ0 K0(t)(1 − A1(t)) ∂q/∂t, an inertial impedance with mass M(t) = ρ0 K0(t)(1 − A1(t)). This is "la corrección de masa del extremo del lado aguas arriba de la rendija", tied to "la impedancia del orificio en la plaqueta", not to input or output ducts.
  - Motion (p247-p248): "prácticamente sinusoidal"; a one-degree-of-freedom mass-spring-damper.
    - Radiation added mass, Eq. A1.10: m_f = (8/(9π)) ρ0 l h (l² + lh + h²)/(l + h). 2m_f is added (both faces).
    - Eq. A1.12: ((m + 2m_f)/l) ∂²x2/∂t² + (c/l) ∂x2/∂t + (k/l)(x2 − x0) = F1(t).
    - p249: the system is two coupled nonlinear equations (unsteady Bernoulli and motion) in x2(t) and q(t).
  - **Energy transfer at slot entry** (p251-p252): "el movimiento de la lengüeta es casi sinusoidal en torno a una posición de equilibrio, que para baja presión de soplo podemos tomar como el propio desplazamiento estático. La presión aerodinámica alcanza un pronunciado pico en el momento en que la lengüeta entra en la ranura de la placa. ... Como la posición de equilibrio coincide casi con el plano de la placa, la fuerza inducida ... está prácticamente en fase con la velocidad de la lengüeta. ... Una variación similar de presión, esta vez negativa, ocurre al salir la lengüeta de la rendija, pero de una manera menos violenta, quedando un superávit de energía". (*Near-sinusoidal motion about an equilibrium close to the plate plane. A sharp aerodynamic pressure peak when the tongue enters the slot is nearly in phase with its velocity and feeds energy in; a gentler negative peak on leaving leaves a net surplus.*)
  - p252: the far-field spectrum "muy rico, predominando los armónicos impares". Only the violent pressure peaks show up acoustically.
  - Lighthill analogy (p249-p251): only the Reynolds-stress term remains.
  - Listed improvements (p252): the aperture model [Millot & Baumann 2007]; a 3-D tip region; "Falta un estudio del mantenimiento o no de la oscilación en el caso de flujos bajos"; frequency vs pressure; the chambers [Tonon 2005].
- Plitnik 2000 (p256): shallot volume changed with wax affects frequency and spectrum (organ reeds).
- Millot & Baumann 2007 is cited as the full literature review (p28, p61-p62) and "se construye un modelo de vibración prestando especial atención a los detalles geométricos de la lengüeta y su posición sobre la plaqueta" (p62). p179 notes it compares "tres modelos de secciones eficaces" with inflow corresponding to a finger attack.
- The thesis presents no simulation of its own.

## G. Bibliography relevant to reed physics, attacks and accordion measurements (as printed, pp. 303–311 and 317–319)
- [3] Alberdi, J.; Orobengoa, J.: Comunicación personal (2015). This is the source of the "set low reeds higher" advice.
- [2] Alberdi, J.; Baraiazarra, K.; López, J. M.: Personal communications (2005-2007). This is the source of the clearance ("luz") statement.
- [4] Almeida, A.; Bergeot, B.; Vergez, C.; Gazengel, B.: Analytical determination of the attack transient in a clarinet with time-varying blowing pressure, Acta Acustica united with Acustica 101 (2015) 1026-1038.
- [6] Bahnson, H. T.; Antaki, J. F.; Beery, Q. C.: Acoustical and physical dynamics of the diatonic harmonica, J. Acoust. Soc. Am. 103(4) (1998).
- [11] Benetoux, T.: L'Accordéon & Sa Diversité Sonore, Editions Thierry Benetoux, Le Tour (2005).
- [12] Benetoux, T.: The Ins and the Outs of the Accordion, Editions Thierry Benetoux, Le Tour (2002).
- [13] Bergeot, B.; Almeida, A.; Vergez, C.; Gazengel, B.: Attack transients in a clarinet model with time-varying blowing pressure, Acoustics 2012, Nantes (2012).
- [14] Bergeot, B.; Almeida, A.; Vergez, C.; Gazengel, B.: Measurement of attack transients in a clarinet driven by a ramp-like varying pressure, Acoustics 2012, Nantes (2012).
- [16] Braasch, J.; Ahrens, C.: Attack Transients of Free Reed Pipes in Comparison to Striking Reed Pipes and Diapason Pipes, Acustica / Acta Acustica 86 (2000) 662-670. This is the source of the −50/−5 dB definition.
- [19] Cottingham, J. P.: Analysis of free reed attack transients, Forum Acusticum 2014, Kraków (2014).
- [20] Cottingham, J. P.: Pitch Bending and Anomalous Behaviour in a Free Reed Coupled to a Pipe Resonator, Forum Acusticum 2005 (2005).
- [21] Cottingham, J. P.: Pitch bending and multiple-mode reed vibration in mechanically-blown free reed instruments, Proc. ISMA 2010, Sydney and Katoomba (2010).
- [22] Cottingham, J. P.; Lilly, C. J.; Busha, M.: Variation of frequency with blowing pressure for an air-driven free reed, 137th ASA / Forum Acusticum, Berlin (1999).
- [23] Cottingham, J. P.; Lilly, C. J.; Reed, C. H.: The motion of air-driven free reeds, 137th ASA / Forum Acusticum, Berlin (1999).
- [24] Coyle, W. L.; Behrens, S. L.; Cottingham, J. P.: Influence of accordion reed chamber geometry on reed vibration and airflow, J. Acoust. Soc. Am. 126, 2216(A) (2009).
- [25] Debut, V.; Millot, L.: Time domain simulation of the diatonic harmonica, Mosart Workshop (2001).
- [28] Fabre, B.; Gilbert, J.; Hirschberg, A.; Pelorson, X.: Aeroacoustics of Musical Instruments, Annu. Rev. Fluid Mech. (2012).
- [30] Fletcher, N. H.: Autonomous vibration of simple pressure-controlled valves in gas flow, J. Acoust. Soc. Am. 93, 2172-2180 (1993).
- [32] Fletcher, N. H.: The nonlinear physics of musical instruments, Rep. Prog. Phys. 62, 723-764 (1999).
- [33] Fletcher, N. H.; Rossing, T. D.: The Physics of Musical Instruments, 2nd ed., Springer (1998).
- [34] Gervasoni, P.: L'Accordéon, Instrument du XXème Siècle, Mazo, Paris (1986).
- [35] Gordon, J. W.: The perceptual attack time of musical tones, J. Acoust. Soc. Am. 82(1), 88-105 (1987).
- [42] Hirschberg, A.; Gilbert, J.; Wijnands, A. P. J.; Valkering, A. M. C.: Musical aero-acoustics of the clarinet, J. Physique IV C5 (1994).
- [43] Hirschberg, A. et al.: A quasi-stationary model of air flow in the reed channel of single-reed woodwind instruments, Acustica 70 (1990).
- [46] Johnston, R. B.: Pitch control in harmonica playing, Acoustics Australia 15(3) (1987).
- [56] Miklós, A.; Angster, J.; Pitsch, S.; Rossing, T. D.: Interaction of reed and resonator by sound generation in a reed organ pipe, J. Acoust. Soc. Am. 119(5) (2006).
- [57] Miklós, A.; Angster, J.; Pitsch, S.; Rossing, T. D.: Reed vibration in lingual organ pipes without the resonators, J. Acoust. Soc. Am. 113(2) (2003).
- [59] Millot, L.; Baumann, C.: A proposal for a minimal model of free reeds, Acta Acustica united with Acustica 93, 122-144 (2007).
- [60] Millot, L.; Cuesta, C.; Valette, C.: Experimental results when playing chromatically on a diatonic harmonica, Acustica 87, 262-270 (2001).
- [62] Misdariis, N.; Ricot, D.; Caussé, R.: Modélisation physique de la vibration d'une anche d'accordéon, Proc. 5th French Congress on Acoustics, 281-283, Lausanne (2000).
- [70] Plitnik, G. R.: Vibration characteristics of pipe organ reed tongues and the effect of the shallot, resonator, and reed curvature, J. Acoust. Soc. Am. 107(6) (2000).
- [74] Repetto, C. E.; Roatta, A.; Welti, R. J.: Measurements of resonant frequencies, loss factor and dynamic Young modulus of cantilever beams, Rev. Bras. Ensino Fís. 36, 1314 (2014).
- [78] Ricot, D.; Caussé, R.; Misdariis, N.: Aerodynamic excitation and sound production of blown-closed free reeds without acoustic coupling: the example of the accordion reed, J. Acoust. Soc. Am. 117(4) Pt. 1 (2005).
- [83] St. Hilaire, A. O.; Wilson, T. A.; Beavers, G. S.: Aerodynamic excitation of the harmonium reed, J. Fluid Mech. 49, 803-816 (1971).
- [84] Tarnopolsky, A. Z.; Lai, J. C. S.; Fletcher, N. H.: Flow structures generated by pressure-controlled self-oscillating reed valves, J. Sound Vib. 247, 213-226 (2001).
- [89] Titze, I. R.: The physics of small-amplitude oscillation of the vocal folds, J. Acoust. Soc. Am. 83(4), 1536-1552 (1988).
- [90] Tonon, T.: Reed cavity design and resonance, PICA 2005, http://www.concertina.org/pica/pica_2005_2/pdf/reed_cavity_design_resonance.pdf (2005).
- Author's own papers (Ch. 7):
  - [18] Llanos-Vázquez, R.; Elejalde-García, M. J.; Macho-Stadler, E.; Agos-Esparza, A.: Physical and psychoacoustic characterization of different types of attacks on the accordion, Acta Acustica united with Acustica 100, 375-384 (2014). This is "Llanos 2014": Table 4.8 above is its data.
  - [12] Llanos-Vázquez, Elejalde-García, Macho-Stadler: Attack transients in accordion, ICSV13, Vienna (2006).
  - [15] Llanos-Vázquez, Agos-Esparza, Macho-Stadler, Elejalde-García: Psychoacoustic study of attack transients in accordion, ICSV16, Kraków (2009).
  - [16] Llanos-Vázquez, Elejalde-García, Macho-Stadler: Comparison between attack transients of accordion and other free-reed instruments, Forum Acusticum 2011, Aalborg (2011).
  - [14] Llanos-Vázquez, Elejalde-García, Macho-Stadler: Controllable pitch-bending effects in the accordion playing, Acoustics'08, Paris (2008).
  - [21] Macho-Stadler, Elejalde-García, Llanos-Vázquez: Oscillations of end loaded cantilever beams, Eur. J. Phys. 36, 055007 (2015).
  - [19] Elejalde-García, Macho-Stadler, Llanos-Vázquez: Vibration of bars: an experimental study of the sound produced by free reeds, ICSV22, Florence (2015).
  - [20] Elejalde-García, Macho-Stadler, Llanos-Vázquez, Agos-Esparza: Experimental study of the sound produced from a concert accordion, Third Vienna Talk on Music Acoustics (2015).

## How this bears on our three failures (my reading, kept short)
1. **Attack speed.**
   - Measured finger attacks rise 45 dB of the 1st harmonic in 50–140 ms, a mean e-folding time of about 10–27 ms. The rise is steepest in the first quarter (about 1.2 dB/ms for H1 of A#3 in mf).
   - The "fast finger" pallet travel is about 50 ms and is itself of the order of T. The thesis models the finger attack as a blowing-pressure step and the bellows attack as a ramp.
   - Clearance ("luz") and a higher set for bass reeds are the luthiers' levers for a faster start. There are no numbers, but the statements say response is sensitive to the reed's fit.
   - Fletcher's criterion adds that a cavity (compliance) upstream "adds friction" while inertance helps. Ricot/St Hilaire place the drive in the upstream flow inertia plus the pressure spike as the tongue enters the slot.
   - Caveat: T excludes the delay before −50 dB and may be limited by an undisclosed analysis window; 50 ms is the floor of every table.
2. **Pitch vs pressure.** The thesis supports "frequency falls with bellows pressure, more at low notes". Mid-range data, however, are only 0 to −5 cents, one 1 Hz step, so they cannot confirm −9 cents. E1–E2 show −21 to −127 cents. Cottingham, Lilly & Busha 1999 is the cited source dedicated to frequency vs blowing pressure.
3. **Saturation.** The thesis gives only qualitative "choking": at very high pressure harmonic amplitudes can fall and the reed can block. Luthiers attribute this to pressure beating the steel's restoring force, and to cells that are too small or too large. There is no amplitude measurement of the tongue.
