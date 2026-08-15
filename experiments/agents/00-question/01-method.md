# Method

## The hazard

This experiment has a problem the reconstruction experiment did not.

There, the subject was a process and the experimenter was a person. Here the subject is an
LLM agent, and the experiment is being designed with an LLM.

An experiment whose author and whose subject are the same kind of thing can demonstrate its
own design and call the demonstration a result. If the agent already knows the hypothesis,
already knows which mapping is the intended one, and already knows what a friction would
mean, then the run proves nothing except that a plan can be followed.

The method below exists for that reason and no other.

---

## The briefing

The agent receives exactly one artifact, fixed before the run and unchanged during it.

It contains what any legitimate caller of the engine would have:

```text
the public API surface
the ontology
the world it may act in
the goal
```

It does not contain:

* this directory, or any part of it;
* the question, the hypothesis or the success criteria;
* the pre-registered placements;
* the fact that anything is being measured;
* the names of the concepts the experiment predicts it will reach for;
* any indication that one option is realizable and the other is not.

The agent is a caller with a job, not a participant in an experiment.

Everything the agent is given is committed to the record before the run, so the briefing
can be read afterwards against what the agent did with it.

---

## What is measured

Three things, in decreasing order of how much they can be argued with.

**What the agent built.** The knowledge objects it constructed and admitted, and the calls
it made to derive consequences. This is the least deniable evidence, because it either
compiles and admits or it does not.

**What the agent said each object asserts.** One plain sentence per object, required by the
briefing as part of the job rather than as an instrument. Without it, a repurposed primitive
is indistinguishable from a correct one.

**What the agent reached for and did not find.** The single most informative signal in the
experiment: a request for an operation, a field or a concept that does not exist. Those
requests are recorded verbatim as *requests*, never rewritten into design proposals.

A request the engine cannot satisfy is the friction the methodological constraint exists to
surface. It is data, and it is not acted on inside the experiment that produced it.

---

## Evidence and test

An LLM is not deterministic, so nothing about the run can be asserted by a test.

The experiment therefore splits in two, and the split is not a compromise — it is the same
boundary the third experiment studies, applied to the experiment's own machinery:

```text
the agent's run          → recorded once, verbatim, as evidence
the objects it produced  → frozen, and asserted by tests
```

The record of a run is a fact about what happened on the day it happened. It is never
regenerated to look better, and a second run is a second record rather than a replacement
for the first.

The objects, once produced, are ordinary APE knowledge. Everything the experiment claims
about them — what the engine derives, what a later reading reproduces — is asserted by tests
that fail loudly, exactly as the reconstruction experiment asserts its phases.

The narrative belongs to the record. The consequence belongs to the suite.

---

## The leakage that cannot be removed

The agent will be an LLM, and the engine's documentation was written with one. General
priors about words like *commitment* and *event* exist in any such model regardless of what
the briefing contains, and no amount of isolation removes them.

This is worth stating plainly because it decides how the two possible outcomes must be
read, and it decides them asymmetrically:

**A success is weak evidence.** An agent predisposed to the intended mapping arriving at the
intended mapping tells us less than it appears to. It shows the mapping is reachable, not
that it is forced.

**A friction is strong evidence.** An agent predisposed toward success that still cannot
express something, or that reaches for a concept the engine does not have, has found a gap
its bias was working against.

The experiment is therefore run to find frictions, and a clean run is reported as the
weaker result it is.

Reporting a clean run as a confirmation would be the failure this method exists to prevent.

---

## Who runs the agent

The agent must be a session that has not read this directory.

The operator decides how that session is produced. What the method requires is only that it
be reproducible from the record: the briefing that was given, the model that received it,
and what came back.

A session that has read the experiment is not an agent for these purposes. It may help build
the world, write the harness or judge the result — none of which is the thing being
measured.

---

## Recording rules

* The engine the run is measured against is pinned by commit, and the pin is recorded with
  the run. A boundary is a property of a version, and a finding about a boundary that has
  since moved is a finding about nothing unless it says which one it met.
* The briefing is *provably* fixed before the run, by one of two means: committed first, or
  its digest published in the session record before the agent is invoked. What the rule
  protects against is an input edited after seeing the output, and a digest settles that
  without requiring a commit round-trip to stand between the briefing and the run.

  This replaced a stricter-sounding *commit it first*, and the replacement was written at
  the moment the original bound its author — which is the circumstance under which rules are
  usually weakened. It is recorded here so the substitution can be judged rather than
  noticed: a digest is a stronger claim than an ordering, because ordering relies on nobody
  having amended in between and a digest does not.
* The agent's output is recorded verbatim, before it is judged.
* A run that goes badly is recorded, not discarded.
* An observation names the smallest case that reproduces it.
* An observation states what was already known from the engine, so nothing familiar is
  presented as a discovery.
* No consequence is accepted because it makes the next step convenient.
