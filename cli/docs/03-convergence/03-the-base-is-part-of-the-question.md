# Observation 3 — The Base is part of the question, so it is part of the record

Phase 2 had to say where the two lines last agreed, and reached for the engine to answer it.
The engine does not, and says so:

```text
coherent_base(Base, Source, Target) iff
    Base ∈ ancestors(Source) and Base ∈ ancestors(Target)
```

Any common ancestor qualifies. Synthesis *verifies* a Base it is handed and never searches for
one, and the relation is deliberately reflexive so that the degenerate choices stay informative:
a Base equal to the Target is a fast-forward, a Base equal to the Source leaves an empty
difference, and both are answers rather than errors.

So there is no privileged Base to be discovered. Which world a difference is measured against is
part of what is being asked — *"what did the Source decide, relative to where?"* — and a
different admissible Base is a different, equally legitimate question with a different answer.

None of that is a finding of this experiment. It is stated in the Synthesis layer, and the
archive port exists to make the verification walk possible. What follows for a repository is
ours, and it is one sentence:

> A report is derived from four things, and only three of them are worlds the record already
> holds. The fourth is a choice, and a transfer recorded without it is not reproducible.

That is the question Phase 4 inherits. It is not *whether* the Base can be recomputed — it
cannot, because it was never computed — but whether a repository that keeps transfers keeps the
question each one answered, in the same way it keeps every other intention.

## What Phase 2 actually established

The four ancestry walks stand; the reason given for them was wrong. Two of them say both tips
descend from the ancestor, which is what makes it an **admissible** Base rather than the right
one. The other two say neither tip descends from anything the other line decided, which is what
Phase 2 was asked to produce: two lines that diverged, neither a prefix of the other.

## What this subject cannot say

For either direction of transfer between the two tips, the admissible Bases are:

```text
ancestors(provisioning) = { provisioning, advanced, equipping, ancestor }
ancestors(maintaining)  = { maintaining, stocking, ancestor }
                                                   └─ the intersection, entire
```

Exactly one. So this arrangement can exercise a Base but cannot exercise *choosing* one, and
nothing here measures what a second admissible Base would have changed. An arrangement that
branched twice on the same side would offer two, and the difference between the reports would be
the measurement. That is a candidate for a later experiment and not a gap in this one — but a
phase that claimed to have weighed the choice would be claiming something this subject cannot
support.

## A smaller thing, from the same phase

Every experiment before this one read a lineage's tip with `.last()`. There are two tips now and
neither is at the end: the decisions were taken in the order an application actually thinks in,
returning to whichever line it was working on, so the record's order interleaves the branches.

```text
0  ancestor       1  equipping      2  stocking
3  maintaining    4  advanced       5  provisioning
```

Position still means something — it is the order the decisions were taken, and `worlds.json` is
positional for exactly that reason — but it no longer tells a reader the shape. The subject names
its worlds now, after the decisions that produced them rather than after where they sit.
