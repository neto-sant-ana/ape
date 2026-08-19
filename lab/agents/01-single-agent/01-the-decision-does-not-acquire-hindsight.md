# Observation 1 — A decision does not acquire the innocence of hindsight

Phase 6 asked whether the reading taken at the moment of decision survives the knowledge
that arrived after it.

The run gave that question a sharper form than the protocol anticipated, because of what the
agent did with the answer it got. Told that the arrangement it wanted could not be carried
out, it did not omit the intention — it cancelled it, by admitting an Event asserting that
the priority slot will not happen.

That is what makes the comparison worth making. A cancelled Commitment moves no level. So
under knowledge as it stands after the run, the very same intention conflicts with nothing:

```text
read under the cut it was decided under → out of bounds at -20
read under the cut current afterwards   → nothing found
```

Both readings are correct. They answer different questions, and the experiment's claim is
that they do not contaminate each other.

They do not. The Thesis produced at the moment of decision reports `OutOfBounds` at −20
under all three hypotheses, before and after, with its cut still recognizing the chain it
recognized rather than the one that grew past it.

---

## What was already known from the engine

This is not a discovery about the engine's mechanism, and presenting it as one would be
dishonest.

A `KnowledgeCut` carries the Event Head it recognizes. `Interpretation::of` resolves the
chain from that head, and folds it into an `Accumulation::recognizing(head)`, which refuses
an event beyond it and refuses to interpret before reaching it. The layer documents both
halves of that boundary in its own words.

The mechanism was built for this. What the experiment contributes is not the mechanism but
the consequence of applying it, at the public boundary, to a caller the engine was not
designed for.

---

## The consequence of applying it

An autonomous agent produces decisions somebody will later be asked to defend, and the
defence has a specific shape: *was this reasonable under what could be known then?*

That question is normally answered by a governance feature — a decision log, an audit
record, a snapshot of inputs written alongside the decision. Such a feature has to be built,
and it has to be trusted, and it is the first thing to rot when the system changes.

Here, nothing was built. Neither the agent nor the harness recorded anything for the purpose
of accountability. The defensibility of the decision is a consequence of two properties the
engine already had for other reasons — that knowledge is immutable, and that a Thesis fixes
the cut a reading is taken under.

> *Through the current public boundary, the reading a decision was taken under remains
> available and unchanged after knowledge moves, without anything having been recorded for
> that purpose.*

For the experiment's question this means the accountability property is not something APE
would need to gain in order to carry an agent's decision. It is already there, as a
consequence of a design choice made about history rather than about agents.

---

## What this does not show

**It is weak evidence, and the method says to report it as such.** An agent predisposed
toward the intended mapping arriving at it shows the mapping is reachable, not that it is
forced.

**It is in-process.** Nothing here says the property survives process death. That is the
CLI's experiment, under a different boundary, and importing its conclusion would confuse the
two.

**It says nothing about judgment.** The reading is stable; whether a stable reading is enough
to tell a defensible decision from a lucky one is the second experiment's question, and this
run does not answer it. The pre-registration named that gap as the most consequential outcome
available here, and it remains open.

**The cancellation was the agent's choice, not the protocol's.** Had it omitted the intention
in a fork instead, the comparison would have been weaker — the contrast that keeps this from
being a tautology exists because the agent chose to record that its intention would not
happen. A protocol that wanted this contrast should ask for it rather than hope for it.

---

## Smallest reproducing case

`tests/run_01.rs`, three tests: the reading at the moment of decision, the same reading after
knowledge moved, and the contrast under a later cut.

The comparison was seen to fail. Reading the decision from a cut taken after the cancellation
— what an engine resolving the chain from the current head instead of from the Thesis's own
cut would produce — turns three `OutOfBounds` at −20 into three empty reports.
