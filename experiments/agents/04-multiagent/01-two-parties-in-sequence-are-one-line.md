# Two parties in sequence are one line

Two agents, neither told the other exists, each acting for its own party, produced **one lineage**
and not two.

```text
bbee1243  base                                 nobody
  → ddef2011  advance to the 7th               operations
    → 1cd9afbb  fork: −20 out, +60 in          operations
      → 1f1afd4e  fork: +30 in                 finance
```

Every node extends the one above it. There is no branch, no divergence, and no pair of worlds a
common ancestor sits below. `1f1afd4e` holds both parties' intentions — the 60 and the 30 — and
reports no conflict, so the *outcome* the experiment wanted from a merge arrived without a merge
happening.

## Neither party was steered into this

Finance was given an invoice and a deadline. It was not told what to extend, that another party
existed, or that a repository can hold more than one line. It read `repo/`, found four things
already there, and decided against the tip — which is what a party that reads a repository holding a
decision ought to do. Its own words: *the world at the tip is `1cd9afbb`, and it is what finance
decides against.*

Operations, running first, found one world and extended that.

So the topology is not a property of either agent's reasoning. It is a property of **reading a
repository that already holds a decision**, and both agents did the ordinary thing.

## The protocol expected the wrong failure

Phase 2 fixed the order and recorded why:

> the order in which they run is fixed by the harness and recorded, because the second agent
> necessarily reads a repository the first has extended.
>
> That last point is a limit, not a control.

The limit was named and its cost was mis-stated. What was expected to leak was **content** — that
the second party's choice might be coloured by an unexplained commitment in history. What actually
happened is that the arrangement removed the phenomenon: sequential reading cannot produce two lines,
so Phases 3 to 6 had nothing to run on.

That is worth separating from a mistake in the world, the objectives or the ontology. Nothing was
wrong with any of those. The experiment asked two parties to diverge and then handed them an order
in which divergence is not the correct thing to do.

## What it says about the architecture, and it is not nothing

A lineage is a tree, and the CLI's convergence work says so at length: *a second party's line is a
branch rather than a competing version, and the union of two parties' decisions is a lineage in the
same sense either party's was.* That is about what a repository **can** hold.

This is about what two parties **do** hold, and the two are further apart than the tree suggests:

```text
concurrent reading  →  two lines, and a merge to reason about
sequential reading  →  one line, and nothing to merge
```

Divergence is not what independent parties produce. It is what parties produce when they read the
same state *before* either has written — which is exactly the window `converge` exists for, and
exactly what a repository read one after the other never opens.

An application that serialises its parties has a coordination problem it will never see, and one
that lets them read concurrently has a merge it must handle. Neither is a defect of the engine;
which one an application is depends on something no layer here decides.

## What follows, and it is a new arrangement rather than a re-run

The record of this arrangement stands. What it measured is real and it is kept.

To reach the phases that need two lines, the experiment has to open the window it closed: a party
that reads the base **as it stood before the other wrote**, decides, and writes back into a
repository that has moved underneath it. That is `converge`'s reason for existing, and it is the
arrangement the CLI's coordination experiment built.

So run B′ is a second party under a different arrangement, not a second attempt at the same one, and
both records are kept — which is what the method requires of a run that goes somewhere unplanned:

> The record of a run is a fact about what happened on the day it happened. It is never regenerated
> to look better, and a second run is a second record rather than a replacement for the first.
