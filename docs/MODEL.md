# The RF-Musette model

This document is the model's ledger. Each mechanism the engine implements names
the physics it comes from and the paper that measured it; each simplification is
stated rather than hidden; each constant says where its value came from. The
tests hold the model to what this document claims — it is allowed to be
approximate, not to drift from what is written here.

**Status: nothing sounds yet.** The skeleton fixes the instrument's contract
(keys, bellows, gain, state). The first mechanism, one free reed, is
milestone 1 in [ROADMAP.md](ROADMAP.md).

## How a value earns its place

There is no reference recording to fit against (decided 2026-09-30, see
[RESEARCH.md](RESEARCH.md)). So every constant carries one of three statuses,
and keeps it:

| Status | Means | Must record |
| --- | --- | --- |
| **Measured** | Taken from a published measurement | The source, and whether it was read in full or from an abstract or figure |
| **Derived** | Computed from measured values by stated physics | The derivation, in the code beside it and here |
| **Voiced by ear** | Set by listening | The date, what was heard, and the range the literature allows |

A voiced value is never later cited as measured. Where the literature gives a
number for something the model *produces* rather than *takes* — how far the
pitch sags with pressure, how long an attack lasts — that number becomes a
test on the output, not a constant to fit, so voicing by ear cannot move the
instrument outside what real reeds do without the tests saying so.

## The contract so far

### The bellows intent (decided, not modelled)

A key only opens a pallet; everything dynamic comes from the bellows. The
player's intent reaches the engine as a fraction from 0 to 1:

* with no bellows controller, the velocity of the latest struck key;
* once Expression (CC 11) arrives, Expression — 14 bits with CC 43 as its low
  half, or a MIDI 2.0 controller at its own width — and velocity stops
  moving it.

This is an interface decision (2026-09-30), not physics, and the intent is not
a pressure. The mapping from intent to pascals belongs to the bellows model
and will be entered here with its status when it exists. Tests:
`bellows.rs` (the five rules above) and `contracts.rs` (the MIDI 1.0 and
MIDI 2.0 paths agree on a 7-bit origin).

## What is modelled

Nothing yet.

## What is deliberately not modelled yet

Everything in the signal path of [RESEARCH.md](RESEARCH.md): the bellows
reservoir, reversal and air button, the pallets, the reed chambers, the reed
plate pairs and their valves, the reeds, the ranks and their tremolo, the
cassotto, the grille and the body, and the Stradella bass. The order they
arrive in, and what each must demonstrate, is [ROADMAP.md](ROADMAP.md).
