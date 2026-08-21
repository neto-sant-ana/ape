# Observation 2 — The common ancestor is derived, and nothing copied it

C2 predicted that two repositories founded from the same admissions hold the same entry identities
without either being cloned. Phase 2 measured it, and the measurement is the most consequential thing
in this experiment.

```text
relation      shared as a prefix      shared as a set
──────────────────────────────────────────────────────
Disjoint      0                       0
Shared        13                      13
Extending     14                      14
```

No operation ran between the two repositories. Neither has read the other, nothing was copied, and the
arrangement was forbidden a clone precisely so that this could not be answered by construction. Both
were built by one function called twice.

And the shared prefix survives both sides growing: each admitted a plan of its own afterwards — 14
entries each — and the first 13 are still the same 13, diverging at the first entry each admitted
alone.

## What that means for the thing this row is shaped after

The distributed system this engine acknowledges as an influence needs a **merge-base**: a commit both
sides can name, found by walking two histories until they touch, held because both sides have the
object. It is a thing a record *has*.

Here it is a thing a record *is*. An `EntryId` is derived from what admitting produced, so two
repositories that said the same thing hold the same address for it — and the common ancestor is not
found, not transferred, and not stored anywhere as a relation. It is a consequence of having said the
same thing.

> **Two repositories share exactly the history they independently said the same way, and neither has to
> be told.**

## And the row derived it in its first experiment without knowing what for

Observation 1 of the reconstruction experiment established that canonical knowledge is reconstructed by
**replaying admissions**, and that an entry is addressed by the identity admitting it produced rather
than by a position. The reason given at the time was that a journal reordered, split across files or
re-encoded still holds the entry an address names, where an offset would hold none of them.

That is the same property, and this is what it was for. Nine experiments have been building on a
distributed capability while measuring one repository at a time.

## What it does not say

That two repositories can therefore meet. They cannot — Observation 1 measured the refusal, and
Observation 3 gives the reason it is not an oversight. What this says is narrower and is the half
nobody had checked: **whatever a meeting would need to know about common history, the record already
knows without being asked.**
