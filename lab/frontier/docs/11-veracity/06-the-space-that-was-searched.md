# Observation 6 — The space, and what was not reached

This experiment's own methodological constraint says a negative result must name the space it
searched. The result is not negative, and the constraint applies anyway: a positive result that does
not say where it stopped looking reads as *this is the hole*, when what was measured is *this hole is
here*.

```text
generator                    coverage      what that means

an interrupted write         exhausted     `prepare` without `turn` is the only prefix a whole
                                           write has, because a turn is one `rename`
a mixture                    exhausted     all 8 combinations of three files over two generations
a readmission                sampled       two shapes: one behind every coordinate, one with new
                                           knowledge between the occurrences
an interleaving              sampled       both parties preparing before either turns
a merge                      exhausted     both orderings of two parties over one disagreement
compositions                 sampled       a merge over an interrupted write

a process stopped mid-rename  not reached  the repository makes no promise about it
a third writer                not reached  excluded throughout the row
a disk returning old bytes    not reached  durability, and a different question
```

**Exhausted** means every state the mechanism admits was produced against this subject. **Sampled**
means representative states were, chosen by shape rather than by count. **Not reached** means what it
says.

## What *exhausted* is worth, and what it is not

Exhausted is a claim about the mechanism against **this arrangement**, not about the mechanism. The
merge row is exhaustive over two parties disagreeing about one recording instant of one Event; it says
nothing about two parties disagreeing about several, or about the instant of a Commitment rather than
an Event, or about a disagreement that straddles no decision's cut.

Two of those three are predictable from what was measured and one is not:

* **Several instants** is the same state repeated; the comparison fails at the first one either way.
* **A Commitment's instant** reaches `ensure_selectable` rather than `head_as_of`, so the merged
  record would be **refused** — `CommitmentNotKnownAtCut` — rather than false, in the direction where
  the converging party recorded later. In the other direction it selects what the other party could
  not have selected. Neither was produced here.
* **A disagreement no cut straddles** answers the same and is therefore *differently arranged*. Not
  produced.

Those are the nearest unmeasured states, and they are named so that the next search starts where this
one stopped rather than where it started.

## What the sampling in the readmission row rests on

Two shapes rather than an enumeration, and the reason is that the space is not finite: a journal can
readmit any address any number of times at any position the watermark allows. The two shapes are
chosen by the only distinction the guard makes — whether the set the witness compares against gained a
member between the two occurrences — and the guard's own code is what says that is the distinction.
That is a reading of the implementation, not a measurement, and it is the weakest coverage claim on
this page.

## The bound nobody predicted, and it narrows all of it

Recording is monotonic across admission, so an instant can only move within the gap its neighbours
leave. Observation 2 measured that: the same tamper that a fifteen-entry journal accepts, a
seventeen-entry one refuses before any decision is weighed. Which means the hazard is not *the
recording instant is unchecked* in general. It is narrower and it is worse:

> Two parties whose journals agree entry for entry have **no neighbours to bound them against each
> other**. The monotonic watermark constrains each journal internally and says nothing across two,
> and the merge is the only operation that puts two of them side by side.
