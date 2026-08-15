# Pre-registration

Everything here is written before the first agent runs.

Its purpose is to make one specific dishonesty impossible: deciding after the fact that
whatever landed outside the engine was *obviously* an application concern all along, and
that whatever landed inside was *obviously* what the primitive always meant.

Predictions recorded in advance can be wrong. That is the point of recording them.

---

## Known before the run

These are properties of the engine as it stands today. They are listed so that no
observation can present them as a discovery.

**APE executes nothing.** There is no scheduler and no effect system. An Event asserts that
something was observed, by somebody else, after the fact.

**There is no stored level.** A quantifiable resource has no opening balance. Levels
accumulate from the movements of the Commitments a reading selects, starting from zero; a
Cancelled Commitment contributes nothing, a Fulfilled one still contributes its movement.

A known balance is therefore knowledge rather than configuration.

**The settled past is imposed, not selected.** A Thesis selection is two disjoint halves,
`frozen ∪ open`, where

```text
Frozen(H) = Settled(H) ∪ Ancestors(Settled(H))
```

for the Event Head the cut recognizes. A caller supplies only the open half. Every
Commitment the recognized chain closed is therefore already in the reading, whether or not
anyone asked for it — and so is the level it moved.

**No level is held or returned, but the fold is public.** A conflict carries the level that
breached a bound, and nothing returns one that held.

That is a refusal rather than a gap. A level is a sum over the movements of the Commitments
meeting some criterion, and which criterion applies is an operational question: what has
landed, what will have landed if nothing slips, and what is at stake before a deadline are
different numbers over the same knowledge. An engine offering a single `level()` would be
choosing between them silently.

What the engine does hand over is the arithmetic. `hermeneia::movement_of` answers what one
Commitment contributes — a signed magnitude against a resource instance — and a caller sums
whichever of those meet the criterion it names at the call site.

So the boundary is drawn between two things that are easy to confuse: *what a commitment
contributes* is semantics and belongs to the engine, while *which contributions count* is an
operational question and belongs to whoever is asking.

> The engine's own statement of it: "Choosing between them is the application's; knowing what
> one commitment contributes is not."

This entry is bound to a version. The extraction that made the arithmetic public was in
flight while this was written, and the experiment must run against an engine that contains
it — otherwise the run measures a boundary that is about to stop existing. The pin is
recorded with the run, and this section is re-verified against the pinned engine before
Phase 1.

**Feasibility reports conflicts, never a verdict.** There is no `Feasible`. An empty list
means nothing was found under the hypothesis that was asked, not that the graph is
realizable.

**A hypothesis is an explicit input.** Nothing about the future is claimed without saying
under what assumption.

**Settlement happens once.** A Commitment admits at most one Event; an identical fact is
idempotent and a different one is refused.

**A Thesis fixes a Knowledge Cut,** and readings are taken under it rather than under
whatever is current.

**`AgentKind` has exactly two variants:** `Company` and `Individual`.

---

## Predicted placement

Where each concept is expected to end up. `engine` means it is already an APE primitive;
`application` means it is real but belongs to the caller; `nothing` means the experiment
predicts it has no representation anywhere and needs none.

```text
the intention itself              engine        Commitment
the world it was decided under    engine        Knowledge Cut
which intentions were in view     engine        Thesis selection
the consequence of the intention  engine        Condition, feasibility conflicts
what actually happened            engine        Event
the known balance                 engine        the frozen half of the selection

whether the intention may proceed application   policy over derived consequences
the goal                          application   input to the agent
the record of the run             application   evidence, outside the knowledge graph

the justification in prose        nothing       narrative, not knowledge
confidence                        nothing
the prompt                        nothing
the model that produced it        ?             see below
```

The last row is deliberately not predicted. It is the sharpest question the experiment
carries, and it is stated as a friction rather than a placement.

---

## Predicted frictions

Each prediction states what would refute it, so a run that disagrees is legible as
disagreement rather than as noise.

**F1 — The agent has no honest kind.**

An LLM agent is neither a `Company` nor an `Individual`. If the decision must name the
deciding entity as an APE Agent, no truthful variant exists.

The obvious repair — a third variant — is exactly the ontology growth this experiment
refuses. The interesting alternative is that the responsible entity was never the model:
the accountable party may legitimately be the person or organization on whose behalf it
acted, in which case the ontology needed nothing and the friction dissolves.

*Refuted if* the agent completes the decision without ever needing to name itself as an
Agent, or names a human principal without hesitation.

**F2 — withdrawn before the run, and kept here because it was wrong.**

The prediction was that a Thesis selecting only the agent's new Commitment would project a
balance of −30 rather than 70, and that expressing a known balance would require the agent
to remember to select the history that produced it.

A probe against the engine refuted it before anything was committed. The frozen half of the
selection carries the settled past in on its own: selecting only the intention still reads
100 − 30 = 70, and no conflict is reported.

It is recorded rather than deleted because the method it is testing applies to its author
first. The reading that produced it was plausible, was wrong, and cost one throwaway test to
catch — which is the whole argument for writing predictions down where they can be checked.

**F3 — The agent will fold a level without noticing that it chose a criterion.**

Not the friction F2 claimed, and not a missing read either. The engine supplies the
arithmetic and withholds only the choice of *which* movements count.

That silence is a control, and it works on a programmer: to get a number at all, one must
name the criterion at the call site, and the name is then there to be argued with. An agent
asked whether it can afford something will fold a number too — and nothing in the boundary
obliges it to say which fold it used.

The extraction sharpens this rather than relieving it. A criterion is now three lines away
from anyone holding a projection, so the fold is cheap enough to perform without deliberating
over it — and a choice made cheaply is the kind least likely to be announced.

The prediction is that an LLM fills a deliberate silence by default, and that the criterion
it picks arrives inside its prose rather than beside its number.

*Refuted if* the agent names its criterion unprompted, or never wants a level at all and
works purely from whether its intention conflicts.

**F4 — The agent will want a verdict.**

A policy of the form *proceed if feasible* wants a boolean. What exists is a list of
conflicts under a named hypothesis.

*Refuted if* the agent names its hypothesis without being asked and treats the conflict list
as the answer.

**F5 — The agent will look for somewhere to put its reasoning.**

No knowledge object has a field for it.

*Refuted if* the agent keeps its justification outside the knowledge graph without being
told to.

**F6 — The goal has no representation.**

The engine has no concept of a goal, and the experiment predicts it needs none. This is
listed because it is the most likely place to invent a primitive that feels obviously
missing.

*Refuted if* something in the procedure genuinely cannot proceed without the goal being
knowledge.

---

## What would surprise us

Recorded so that surprise is recognizable rather than reinterpreted:

* a concept the agent needs that appears in none of the three columns above;
* a friction the agent hits that a non-agent caller would not;
* the ontology proving *more* than sufficient — a primitive that carries the agent case
  better than it carries the case it was designed for;
* the decision being reconstructible but the *reason it was defensible* not being.

The last would be the most consequential outcome available to this experiment. It would
mean the record preserves what was decided and what was knowable, but not enough to
distinguish a defensible decision from a lucky one — which is a genuine limit, and one worth
finding before anyone builds on the assumption that it is not there.
