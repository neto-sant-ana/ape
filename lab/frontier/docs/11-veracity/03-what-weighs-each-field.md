# Observation 3 — The audit, and V1 is refuted on both halves

V1 predicted that every field a repository holds is content, a reference, or a derivation compared
with a stored copy — except two, and it named them: `Taken::after` and every `recorded_at`.

**Both halves are wrong.** `after` is checked now, and there is a third.

## What weighs each field

The classification is declared, because *what weighs a field* is a claim about the application and
not a property of its bytes. What is **derived** is the coverage: the keys are read out of the three
files as JSON and compared against the classified set, so a field added to any record breaks the
guard before it can be quietly unclassified. Forty rows, against a sweep that reaches at least
thirty keys — the second assertion is there because this family of guard fails by scanning nothing
and reporting agreement.

```text
file      field                                what weighs it

journal   every field but `recorded_at`        the address. It is in the identity admitting
          (24 of them, across nine variants)   produced, so moving it moves the `EntryId` — which
                                               the witness holds a copy of and later entries resolve

journal   recorded_at                          the worlds file, and only through `event_head`,
                                               and only where a cut resolves against it

lineage   decides, known_at, selection,        the worlds file. The decision produces the world,
          extends, omitted, introduced         and the world is written down and recompared

lineage   after                                the witness
lineage   witness                              the journal, replayed to `after`
lineage   by                                   that the party existed at the coordinate,
                                               and nothing else

worlds    every field                          the decisions, replayed
```

## The third claim, measured

`by` was changed by hand from the party that decided to the other party. The other party is an agent
the base knows, admitted before the coordinate, so `attributed` resolves it. Nothing else is a
function of who decided.

```text
every answer                        unchanged, all four worlds
decided_by(the party that did not)  one world
```

The record now attributes a world to a party that did not decide it, and reads without complaint.
That is the exact standing `after` had before the witness existed: **an identity that resolves and
could be the wrong one.**

It is not this experiment's hazard, and saying why is the point. `by` is the authenticity candidate's
field — its discriminator asks *is the distinguishing fact in the record?* and the answer for an
attribution is no, so the remedy is a key or an anchor outside. What the audit adds is that the
candidate's field is **reachable from inside the same table** as the one this experiment is hunting,
and the two are told apart by whether an accident can produce them rather than by what checks them.
No generator in Observation 4 writes a `by` neither party wrote.

## The shape V1 got wrong, and it is worth naming

V1 asked *what does nothing check*, and that question has three answers. The question that separates
them is narrower:

> **What does nothing check, and what can reach it without a hand?**

`after` — checked. `by` — unchecked and unreachable by accident. `recorded_at` — checked through one
derivation, and Observation 5 is about the operation where that derivation is written by the same
process that would have to disagree with it.
