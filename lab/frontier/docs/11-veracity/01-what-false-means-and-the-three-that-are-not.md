# Observation 1 — *False* is a comparison, and three states that look like it are not

The definition was written before the subject existed, and the subject was then built so that all
four states it distinguishes could be produced.

> A record answers falsely when it answers a question differently from the record the same events
> would have produced, and nothing in it refuses.

The anchor is the arrangement. It constructs the events, so it holds the faithful record and the
record does not — which is what makes falsity a comparison of two records' answers rather than a
judgement about truth.

## The anchor

One line of thinking, written whole, read once. Every world's answer is a pair: what has **settled**
and what the world **intends**, because the two move for different reasons and a phase comparing only
one would miss half the arrangement.

```text
                                                    settled   intended
D1  genesis at day 10, selecting the fund and           120        420
    the claim, with the Event recognized
D2  fork introducing the outflow                        120        370
D3  advance to day 15                                   120        370
D4  fork introducing the inflow recorded on day 12      120        450

                                    17 entries, 4 worlds
```

`D4` is why `D3` exists. The inflow is recorded after the instant `D2` recognizes, so introducing it
under `D2`'s cut is refused — a commitment's recording instant decides whether a cut can select it,
which is one of the two things V3 needed the subject to hold.

## The two generations, which answer differently

```text
generation one    D1                     15 entries, 1 world      tip intends 420
generation two    D1 D2 D3 D4            17 entries, 4 worlds     tip intends 450
```

Written down and asserted as a literal, because an arrangement whose two generations answer the same
number cannot produce a false interrupted write — the phase would report *nothing false* about
something that could not have been false. This is the hazard experiment 10 met at its own Phase 1,
answered the same way.

## The two parties, and the one thing they disagree about

Both read one base of fourteen entries. Each then admits **the same Event** — the same commitment,
the same observation, the same instant of occurrence — and each records it on the day it learned of
it. Both decide at day 10, which lies between the two.

```text
                  recorded    the cut resolves to     settled   intended
ledger            day 3       the Event                   120        420
counterparty      day 11      nothing                       0        120
```

And the measurement that makes the rest of the experiment possible:

> **The two journals are the same journal by address.** Fifteen entries, entry for entry, identical
> `EntryId`s — because an `EntryId` is derived from what admitting produced, and no identity in the
> engine contains a recording instant.

## The three exclusions, produced rather than asserted

**Incomplete.** A whole write that prepares and never turns leaves the previous generation live,
entire. A reader reads generation one and answers `[(120, 420)]`. Every answer is true of the record
that is there; it is a shorter history and not a wrong one.

**Differently arranged.** The two inflows are admitted at the same instant and neither identity is a
function of where it sits, so swapping them produces a **different file** that answers all four
worlds identically. The prefix a decision witnesses is a set, so nothing disagrees. This is
experiment 06's finding arriving from the other side: there, two histories produced one byte-identical
record; here, two records produce one set of answers.

The swap that *is* refused, one row down, moves the Event — because an Event's identity contains its
predecessor, so every Event address after the first is a function of the order.

**Refused.** Every calibration in Observation 2, and every mixture in Observation 5.

## What this fixes about the two protocols that asked before

Experiment 07 asked for *a partial state that reconstructs and answers something false* and experiment
08 for *an interleaving whose result is neither party's state*. Neither defined the word, and with the
definition in hand both questions turn out to have been asking for something their own arrangements
could produce and their own vocabularies could not name: 07's surviving partial state is **incomplete**,
and 08's surviving mixture is **incomplete** too. Reported as *not false*, both are correct. Reported
as *we looked and found nothing*, both were unfalsifiable.
