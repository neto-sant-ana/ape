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

---

## Correction

Everything above the line was written before the experiment's purpose was stated precisely,
and two thirds of it does not survive that statement. The original text is kept because a
record that quietly improves its own past reasoning is worth less than one that shows where it
turned.

**The two claims were not two claims.** The observation rested on

```text
the house intends to spend 120        ← what admitting the Commitment asserts
the agent is considering spending 120 ← what was actually the case
```

and the first line is wrong about the subject. An agent acting with delegated authority does
not report the principal's intention from outside it — the intention it forms *is* the
principal's. That is what delegating means. Where the graph exists to govern the agent, there
is no third party being misrepresented, because there is no third party.

**And the pollution is the audit trail.** Under the purpose this experiment serves — making an
autonomous agent's decisions reconstructible and auditable — a candidate the agent priced and
abandoned is not noise left in operational history. It is the evidence that alternatives were
weighed, which is the first thing anyone auditing a decision asks for.

The thing recorded above as a cost is the mechanism that supplies it. Having to write a
candidate down in order to ask about it is why the record can later show what was considered.

**What survives, and it is narrower.** Intent and consideration remain different states of the
same subject, and the ontology carries no third one. An intention that was weighed and dropped
is expressed as a Commitment cancelled by an Event — formed, then observed not to happen.

Whether that is a distortion or an accurate account of deliberation is a question this
observation is no longer in a position to answer, and experiment 02 is where it gets asked.

One thing is settled, by reading rather than by argument: the *reason* an intention died is
expressible today. `Settlement` holds a set of cancelling observations, not one, so a Statement
may declare `Cancelled: Infeasible` beside `Cancelled: Superseded`, and the Event that ends a
candidate says which. Composition, and nothing added.
