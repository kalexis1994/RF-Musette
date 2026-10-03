# RF-Musette

A physically modelled accordion for [RackForge](https://github.com/kalexis1994/rackforge):
free reeds driven by a bellows, computed rather than recorded.

> **Status: both hands, and a face to play them (0.10.0).** The treble's 41 keys, F3-A6, each with
> five ranks -- L, M−, M, M+, H -- and the Stradella bass's twelve pitch
> classes on five ranks, 16′ to 2′, all tuned to A440 where they sound; a reed
> for each way the bellows moves, one bellows the arm pushes, and a cassotto.
> Every reed is scaled from the accordion F4 the IfM Zwota measured, the low
> ones loaded at the tip so they speak across the bellows' range. RackForge
> shows its own PLAY surface: the registers as an accordion's switches, the
> bellows, the voice and the model's pages; a controller's attack, release
> and LFO-rate knobs reach it by meaning. Branding and factory programs are
> still to come. What it does and does not yet do against measurements:
> [docs/MODEL.md](docs/MODEL.md).

## What it is meant to be

The treble side of a piano accordion first — bassoon (16′), clarinet (8′),
the two tremolo ranks that make the musette, and piccolo (4′) — with the
Stradella bass after it. Each reed is a self-oscillating free reed; the key
only opens a pallet, and loudness, brightness and the small sag of pitch come
from the pressure in one bellows that every reed shares.

The bellows is the modulation wheel (CC 1, with CC 33 as its low bits) or
Expression (CC 11, with CC 43) -- the last moved leads -- and rests at
300 Pa until one of them moves -- with Key Touch off. Digital accordions
send Expression (their program, Digital Accordion, has Key Touch off); a
keyboard player with Key Touch off moves the wheel as the arm, shaking it
for the accordion's own vibrato. With Auto
Reverse on, the bellows runs out after its travel (12 L) and turns on its
own, at a gap between notes once 70 % is spent, as a player turns it. The push is the arm's: one bellows
feeds every reed and gives way a little as more of them draw air (Bellows
Response "Stiff" makes it the pressure itself, for a digital accordion that
measures it). Which way the bellows moves, and so
which reed of each plate sounds, is the Bellows Direction parameter or CC 80
as a switch (below 64 pull, 64 and above push), since no MIDI accordion
sends it. The Register parameter opens the ranks as Roland's FR-3x draws its
14 treble registers, from Clarinet (M alone) to Master; Tremolo sets the
musette's beat; Cassotto puts the 16′ and the true 8′ in a tone chamber.

The bass side listens as a Roland V-Accordion sends it: the bass buttons on
MIDI channel 2 (any octave of a note is its button), the chords on channel 3
(each note sounds its pitch class on the chord ranks, so a keyboard's
left-hand chord works too), the treble on every other channel. On one
keyboard, Left Hand (on by default) splits it at the Split Point (F3): the
octave just below plays the chords, each key its note on the chord ranks,
and everything lower the bass buttons; Bass Register opens the bass ranks as
Roland's seven bass registers do.

## Playing it in RackForge

RackForge shows RF-Musette's own PLAY surface (`crates/rf-musette-ui`, Rust
built to WebAssembly). Its first page is what an accordionist reaches for:
the 14 treble registers and the 7 bass registers as ivory switches, each
with the symbol Roland prints for it; the left hand and its split; the
bellows -- its direction, the air button (open while held), Auto Reverse and
its travel, the smoothing; and the voice. The model's parameters follow on
three pages of knobs, each with where its value comes from as its tooltip.

It is heard as it is recorded: the Microphones page chooses how it is
miked -- inside the instrument or clipped to it as on stage, one mic per
side, an ORTF pair at 1 m, a spaced pair or one mic in front as in the
studio -- each with its own settings, in a room whose size and walls set
its reverberation as Sabine's law does. The treble side and the bass box
are two sources, and the bass box moves with the bellows: a stand mic
hears it come and go; a mic on the instrument travels with it.

Twenty programs come with it: the accordion as eleven traditions tune and
record it -- Paris musette, Scottish dance band, Italian, Alpine,
Oberkrainer, Cleveland polka, American, Irish, jazz, tango, concert --;
five instruments by their build -- student 48- and 72-bass, an Italian
80-bass, a full-size 120-bass, a professional with a cassotto -- and setups
for a 61-key keyboard, a bellows that turns by itself, and a digital
accordion.

SAVE on PLAY keeps the panel as a program of your own, up to 64, listed
after the factory's; saved over the program it was turned from, it keeps
what was turned. CONFIG carries them off the machine and back as
`.rfmusette` files: a file holds one program or all of them, sealed with
their SHA-256, and one changed or damaged after it was exported is refused
whole. An imported program always comes in as a new one. A program saved by
an older RF-Musette always loads: a setting it lacks takes its default.

Played from a keyboard, Key Touch (on by default) lets each note's velocity
set how far its key goes down, as an accordionist holds a key part-way:
softer notes are quieter and a little flat, down to the shallowest each key
still holds its tone at (some 15 dB at F3, 10 at F4, 4 at A5). With it on
the bellows rests at 300 Pa and the wheel and Expression move nothing; off,
every key goes fully down and the bellows is theirs.

The left hand is Stradella -- bass buttons and chords -- or, with Bass
System on Free Bass, a converter accordion's: every key under the split its
own note, E1 to C♯6, in two voices an octave apart, for chords built at
will from a keyboard.

A controller's knobs reach it by meaning: where a controller package gives a
knob the attack, release or LFO-rate role, it turns the pallet's opening and
closing and the tremolo's beat. The pads choose the treble register, as an
accordion's switches do: on a keyboard with 16 pads in two banks (an
Arturia's A and B, most others), the first 14 are the 14 registers from
the most common to the rarest -- Clarinet, which every accordion has,
first, Master last -- the 15th steps the bass register round and the 16th
turns Key Touch on and off; ⏪ ⏩ step the register. Clarinet is pad 1 by
the number the maker prints, wherever the maker puts it. Everything else can be linked by hand from RackForge's menu on any
control. Why each role is or
is not published, and what each controller gets:
[docs/RACKFORGE_CONTROL_MAPPING.md](docs/RACKFORGE_CONTROL_MAPPING.md).

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
| [docs/RACKFORGE_CONTROL_MAPPING.md](docs/RACKFORGE_CONTROL_MAPPING.md) | Which controller roles RF-Musette answers to, and why |

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
