# Observation 2 — An intention must be asserted before it can be examined

The agent found this one, and said so without being asked:

> *"There is no way to ask 'would this be feasible?' without asserting it first. […] history now
> permanently contains an intention that never could be realized."*

To learn that spending 120 would put the account below its floor, it had to admit the
Commitment to spend 120. Canonical history holds it still.

---

## What was already known from the engine

Realizability is not a property of a Commitment. It is a verdict over the whole graph a world
selects, which is why it is asked of a Thesis and not of an intention. A Thesis selects
Commitment *ids*, and both selectability and the fold resolve them out of canonical
knowledge.

And a Commitment is never invalid. It asserts that somebody intended something, and that
either was or was not the case; nothing later makes the assertion false, only ineffective.

Both are deliberate, both are documented, and neither is news.

---

## The consequence of applying it

A human planner weighs options before committing to one. The weighing leaves no trace,
because it happens somewhere the system cannot see.

An agent has no such somewhere. To ask the engine about a candidate, it must first tell the
engine the candidate is intended.

That is where the friction lands, and it is not about volume — though volume is real, since an
agent may weigh many more candidates than a person would. It is about what the assertion
says:

```text
the house intends to spend 120        ← what admitting the Commitment asserts
the agent is considering spending 120 ← what was actually the case
```

Those are different claims, and only the first has a representation. An agent that admits
candidates in order to price them is asserting intentions it does not hold — which is
precisely the failure the protocol names as the dangerous one, because it compiles and reads
as success.

This run does not fall into it. The agent formed the priority intention genuinely, was told it
could not be carried out, and recorded that it would not happen. Its own note says as much:
it cancelled rather than omitted *because the house did form the intention*. The distinction
survived because the agent was careful, not because anything forced it to be.

An agent weighing five candidates and admitting five Commitments would assert four intentions
that were never held, and nothing in the boundary would object.

---

## What the boundary already allows

The escape does not require the ontology to grow, and the agent identified it:

> *"An application that wanted one would have to implement `Knowledge` + `CanonicalKnowledge`
> itself as an overlay of the real history plus the candidate, and interpret against that —
> possible, since both are traits, but nothing in the crate offers it."*

That is correct, and it matters for the experiment's question. A dry run is reachable by
composition, at the application layer, over interfaces the engine already exposes. Nothing
here argues for a new primitive.

What it argues for is smaller and is recorded rather than acted on: every application that
wants to price a candidate before intending it will write that overlay, and each will write it
alone. This is one occurrence. It is named here so that a second one, in another experiment or
another application, is recognizable as a second rather than met as if it were the first.

---

## Smallest reproducing case

`run-01/main.rs`, lines admitting the priority Commitment before any question is put to it,
and `run-01/output.txt`, where the world it was admitted into reports the conflict that made
it pointless.

No test accompanies this observation. There is nothing to assert: the behaviour is the
engine's ordinary one, and what the observation records is the cost of it to a caller that
thinks by writing.
