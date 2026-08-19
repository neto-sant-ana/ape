# Observation 4 — The contention was in the medium, not in the content

Phase 2 repaired the loss, and the repair turned out to be smaller than the problem looked. Nothing
had to be arbitrated, because nothing was ever in conflict.

## The measurement

Three decisions taken, three on disk, and each party's decision is present **byte for byte as that
party took it**:

```text
both read           lineage.json = [ shared ]                            1
one decides         + staffing
the other decides   + equipping
one converges       [ shared, staffing ]                                 2
the other converges [ shared, ?, ? ]                                     3
```

Both worlds extend the shared ancestor and neither extends the other. There was never a version of
this in which one of them had to give way — the two parties are on different branches, and a branch
is not a competing version of anything.

```text
staffing   parent = shared
equipping  parent = shared
```

## And the case that looks like a conflict is not one either

A third party reading the merged repository and deciding **the same fork one of them already
decided** adds nothing:

```text
lineage.json stays at 3 decisions
```

Not because anything detects a duplicate as a special case. Because a decision is a value and a
world is derived from its content, so the same decision decided twice is one record and one world —
which provenance measured for a different reason and this inherits whole.

## What that says about the Canon's answer one layer up

The engine's `append_event` is a compare-and-append against one head, and it is right there:
canonical history is a chain, a chain has one head, and two writers extending it are extending the
same thing.

A lineage is a tree. Two writers extending different branches are not extending the same thing, so
a compare-and-append against "the head" would have **manufactured** a conflict where the shape has
none — one party refused, told to re-read, and asked to decide again something that was never in
anybody's way.

> What contended in Phase 1 was the file. Two parties never contended for a world.

The whole-file write made the two parties compete for a byte range, and the loss looked like a
concurrency problem about intention because that is where it surfaced. It was a problem about a
sequence being asked to hold something that is not a sequence.

## The one thing that had to be chosen

A file is a sequence, so the tree has to be written down in some order, and two decisions on
different branches have no order of their own. The merged sequence is therefore a **linearization**,
and it is chosen rather than observed: ordered by where in the journal each decision was taken, then
by the decision's own content, with a parent always before its children.

Content, not encoding. Every field a decision holds is an identity or an instant, so the order comes
out of what the decisions *are* — a comparison over the JSON would have made the encoding
load-bearing, which is the objection the world witness already answers for itself.

The criterion this was built for holds: the same two decisions converged in either order produce one
repository, compared file by file. Blinding the tie-break so that only the journal coordinate
ordered them made the two runs disagree at `lineage.json` line 32 — the two forks swap, and the
worlds file swaps with them, which is exactly the arrival order showing through into the result.
