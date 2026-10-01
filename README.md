# RF-Musette

A physically modelled accordion for [RackForge](https://github.com/kalexis1994/rackforge):
free reeds driven by a bellows, computed rather than recorded.

> **Status: both hands (0.9.0).** The treble's 41 keys, F3-A6, each with
> five ranks -- L, M−, M, M+, H -- and the Stradella bass's twelve pitch
> classes on five ranks, 16′ to 2′, all tuned to A440 where they sound; a reed
> for each way the bellows moves, one bellows the arm pushes, and a cassotto.
> Every reed is scaled from the accordion F4 the IfM Zwota measured, the low
> ones loaded at the tip so they speak across the bellows' range. The
> product's interface is still to come. What it does and does not yet do
> against measurements: [docs/MODEL.md](docs/MODEL.md).

## What it is meant to be

The treble side of a piano accordion first — bassoon (16′), clarinet (8′),
the two tremolo ranks that make the musette, and piccolo (4′) — with the
Stradella bass after it. Each reed is a self-oscillating free reed; the key
only opens a pallet, and loudness, brightness and the small sag of pitch come
from the pressure in one bellows that every reed shares.

With no bellows controller, key velocity sets the push, which the arm
reaches over Bellows Smoothing (150 ms) rather than at once; once
Expression (CC 11, with CC 43 as its low bits) or the modulation wheel
(CC 1, with CC 33) arrives, it drives the bellows instead -- the last moved
leads. Digital accordions send Expression; a keyboard player can move the
wheel as the arm, shaking it for the accordion's own vibrato. The push is the arm's: one bellows
feeds every reed and gives way a little as more of them draw air (Bellows
Response "Stiff" makes it the pressure itself, for a digital accordion that
measures it). Which way the bellows moves, and so
The bass side listens as a Roland V-Accordion sends it: the bass buttons on
MIDI channel 2 (any octave of a note is its button), the chords on channel 3
(each note sounds its pitch class on the chord ranks, so a keyboard's
left-hand chord works too), the treble on every other channel. On one
keyboard, Left Hand (on by default) splits it at the Split Point (F3): the
octave just below plays the chords, each key its note on the chord ranks,
and everything lower the bass buttons;
Bass Register opens the bass ranks as Roland's seven bass registers do.
which reed of each plate sounds, is the Bellows Direction parameter or CC 80
as a switch (below 64 pull, 64 and above push), since no MIDI accordion
sends it. The Register parameter opens the ranks as Roland's FR-3x draws its
14 treble registers, from Clarinet (M alone) to Master; Tremolo sets the
musette's beat; Cassotto puts the 16′ and the true 8′ in a tone chamber.

## How it is judged

There is no reference recording. Every mechanism is taken from published
measurements, the numbers the literature gives are asserted as tests on what
the model does, and the voicing is done by ear. The ledger of what is
measured, derived and voiced is [docs/MODEL.md](docs/MODEL.md); the survey
behind it is [docs/RESEARCH.md](docs/RESEARCH.md).

## Documents

| Document | For |
| --- | --- |
| [docs/RESEARCH.md](docs/RESEARCH.md) | Literature, prior art, reference data, and the decisions they led to |
| [docs/MODEL.md](docs/MODEL.md) | The model's ledger |
| [docs/SOURCES.md](docs/SOURCES.md) | Primary sources and what each is used for |
| [docs/ROADMAP.md](docs/ROADMAP.md) | Milestones and the predictions each must meet |
| [docs/VALIDATION.md](docs/VALIDATION.md) | Dated validation receipts |
| [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) | Toolchain, commands, layout |
| [docs/AUDITION.md](docs/AUDITION.md) | Build, install and launch in RackForge Desktop |

## Credits

The model stands on published work, above all L. Millot and C. Baumann's
minimal model of the free reed, G. Ziegenhals's measurements of an accordion
reed at the IfM Zwota, T. Tonon's reed cavities, J. P. Cottingham's decades
of free-reed measurements, D. Ricot, R. Caussé and N. Misdariis's study of
the accordion reed, A. Z. Tarnopolsky, N. H. Fletcher and J. C. S. Lai's reed
valves, and R. Llanos-Vázquez, M. J. Elejalde-García and E. Macho-Stadler's
accordion acoustics. Every work read, what was taken from it and where it is
used: [docs/SOURCES.md](docs/SOURCES.md).

## License

GPL-3.0-only. See [LICENSE](LICENSE) and [NOTICE.md](NOTICE.md).
