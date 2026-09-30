# RF-Musette

A physically modelled accordion for [RackForge](https://github.com/kalexis1994/rackforge):
free reeds driven by a bellows, computed rather than recorded.

> **Status: one reed (0.3.0).** Key 65 plays the accordion F4 reed the IfM
> Zwota measured, in its cell, behind its pallet; every other key is
> silent. What it does and does not yet do against measurements:
> [docs/MODEL.md](docs/MODEL.md).

## What it is meant to be

The treble side of a piano accordion first — bassoon (16′), clarinet (8′),
the two tremolo ranks that make the musette, and piccolo (4′) — with the
Stradella bass after it. Each reed is a self-oscillating free reed; the key
only opens a pallet, and loudness, brightness and the small sag of pitch come
from the pressure in one bellows that every reed shares.

With no bellows controller, key velocity sets the push; once Expression
(CC 11, with CC 43 as its low bits) arrives, it drives the bellows instead —
which is what digital accordions send.

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
