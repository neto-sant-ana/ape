# Observation 2 — A lineage records states, and the operations are inferred

The pre-registered friction G1 asked whether Thesis lineage is a record of deliberation or a
record of the agent's method. The auditor answered a narrower question first, and the answer
is a qualified yes:

> *"A Thesis records no operation — only a parent, a cut and a selection. Which of the three
> produced each edge is derived by comparison."*

It recovered every edge correctly, and it recovered them by comparing consecutive states:

```text
cut moved, selection unchanged        → knowledge was recognized
cut unchanged, selection grew         → something was chosen
```

Both readings were right. Neither was read.

---

## What was already known from the engine

A Thesis is `parent`, `cut`, `selection`, and its identity is derived from exactly those. That
is the whole record. Nothing in the layer offers to say which call produced a node.

This is consistent rather than accidental. Recording the operation would be recording a
derivation beside the thing it derives — a second representation of something the states
already determine, and the first candidate to disagree with them. The engine declines that
everywhere: it is the same reasoning that keeps a level out of the record and, in the
reconstruction experiment, keeps `previous_event` out of a journal entry.

---

## The consequence of applying it

For this lineage, the inference is sound and not merely lucky. The two operations available
change disjoint halves of the state: `advance` moves the cut and preserves the selection,
`fork` preserves the cut and changes the selection. No edge can be produced by both, and no
edge can change nothing — identity is content-derived, so an operation that changed nothing
would produce the same Thesis rather than a new one.

So *given the operations that exist today*, the derivation is total and exact.

That qualification is the observation. The soundness belongs to the current operation set, not
to the lineage. An operation that moved the cut and the selection together would make the two
cases indistinguishable, and every existing lineage would silently become ambiguous —
including the ones already recorded, since the ambiguity is in the reading and not in the data.

---

## What it does not answer

G1 asked whether lineage records deliberation or method, and this observation only establishes
that lineage records *states*. Whether the states amount to deliberation depends on the agent
having forked rather than discarded, which the auditor also identified from the other side:

> *"an abandoned sibling — a fork taken and discarded — would be invisible from the entry point
> given."*

An agent that weighs candidates by opening worlds and dropping the ones it dislikes leaves a
lineage showing only what it kept. The record is honest about what it contains and silent about
what it omits, and nothing distinguishes a deliberation with one option from a deliberation
whose losers were never linked.

That half of G1 stands, and belongs to the result rather than here.

---

## Smallest reproducing case

`02-hindsight/run-01/main.rs`, the section that classifies each edge, and `output.txt` beside
it, where the classification appears as a derived column rather than a read one.
