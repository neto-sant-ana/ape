# Observation 1 — The cheap arrangement had to be built

The protocol says arrangement A is the default:

> **A is the default, and nobody chooses it.** It is what happens when an application interprets a
> candidate the cheap way.

In this application it was not available. Nothing here could weigh a world without keeping it.

## What was in the way

`lineage::decide` did two things in one call — it produced the world a decision makes, and it
recorded that world in the lineage:

```text
decide(knowledge, &mut lineage, decision)
    ├── the engine builds the Thesis          ← what arrangement A wants
    └── lineage.record(thesis)                ← what arrangement A must not do
```

There had never been a caller that wanted only the first half, because until this phase everything
an application did here ended in a repository. Four experiments of planning, diverging, corroborating
and coordinating are all arrangements that **keep** what they decide.

The split is two lines moved:

```text
produced(knowledge, &lineage, decision)  -> (Thesis, imposed)
decide(knowledge, &mut lineage, ...)      = produced, then record
```

## Why this is not a note about refactoring

The engine never needed the seam. `Thesis::fork` has always returned a world and put it nowhere —
weighing without keeping is what the engine does by default, and an archive is something a caller
goes out of its way to hand a world to. The fusion was the **application's**, and it happened for a
reason that reads as obvious only afterwards: the application's vocabulary is a persistence
vocabulary. `Decision`, `Taken`, a witness, `worlds.json`. Every noun it owns exists so that
something can be written down.

So the protocol's claim is right about one kind of application and inverted for the other:

```text
vocabulary is the engine's       A is free, and B costs a decision record
vocabulary is a repository's     B is free, and A costs a seam nobody had cut
```

An application built the way this one is does not *fall into* A. It falls into **B** — it reaches for
the verb it has, and the verb it has keeps things. Which makes E3's cost the one an application
arrives at by not thinking about it, and E2's silence the one it has to choose.

## That the seam is load-bearing, measured

Turning `considered` back into a recording verb — one signature and three lines — and running
Phase 1 unchanged:

```text
after the FIRST of twelve candidates
    lineage bytes   1305  ->  2752
    worlds               1  ->  2
```

The phase turns red on candidate one of twelve, at the arrangement rather than at the objective.

## What it does not say

Nothing about APE. `produced` is a rearrangement of two lines the application already had, and no
kernel or engine type moved. The finding is about where the boundary between *weighing* and
*keeping* sits in an application, and the answer is that it did not sit anywhere until something
wanted to be on only one side of it.
