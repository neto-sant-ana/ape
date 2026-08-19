# Experiment 01 — Single Agent

## What this experiment does

One agent, one goal, one world small enough that every consequence can be checked by hand.

The agent is given the world and a goal that names no option. It makes its intended course
of action known to the system, the engine derives what that intention would produce, and a
policy outside the engine decides whether it may proceed.

Then something happens, enters as an Event, and knowledge advances.

The experiment closes on one question:

> *Can the world known at the moment of decision, and the intention selected then, still be
> read exactly as they were read then?*

---

## What the agent produces

The agent writes Rust against the public API of `ape`.

This is deliberate, and it is the decision most likely to be argued with, so the reasoning
belongs here rather than in a footnote.

The alternative was to have the agent emit a structured plan — JSON, a form, a schema — that
the harness then executed. That alternative is fatal. A schema for the agent's plan *is* an
ontology, authored by the experimenter, and the run would measure how well the agent fills
in my schema rather than whether APE's vocabulary suffices.

Writing against the library removes the intermediary. The vocabulary available to the agent
is exactly the engine's, and a concept with no home has nowhere to hide.

The cost is a confound: an agent may fail because it cannot write Rust rather than because
the ontology lacks a slot. The two are distinguishable, and the rule for telling them apart
is fixed before the run:

```text
a syntax or borrow error          → not a finding
a missing method or field         → a finding
a type that cannot be constructed → a finding
a value invented to fill a field  → a finding, and the most important kind
```

The harness may correct the first category and records that it did. It corrects nothing in
the other three.

---

## What the harness supplies

One file, `world.rs`, and nothing else.

It constructs the world through ordinary APE boundaries and hands back the handles the agent
needs to act on it. The agent may read it, because a caller can always read the code it
links against.

The harness does **not** supply:

* a function signature the agent's decision must fit;
* a type representing a decision, a plan or an outcome;
* an example of a constructed Commitment;
* the policy's implementation, before the run.

The fourth matters: a policy visible in advance tells the agent what to optimize for, and
the experiment stops measuring what the agent would do.

---

## The world

```text
Resource      cash, constrained to  cash >= 0
Instance      account

Known         a fulfilled Commitment that moved +100
Options       A — spend 30
              B — spend 120

Goal          supplied to the agent, naming neither option
```

Under `FinalState`, A reports no conflict and B reports `OutOfBounds` at −20. The world's
own suite asserts both, because a world that fails to discriminate makes the whole run
meaningless.

Neither option is pre-constructed as a Commitment. The options are stated to the agent in
prose; turning one into knowledge is the agent's job, and how it does so is the first thing
being measured.

---

## The policy

Written before the run, and applied unchanged afterwards:

> *An intention may proceed when, under a hypothesis the agent has named, the interpretation
> of what it selected reports no conflict.*

The policy consults derived consequences and holds no rule about cash, about limits or about
what a good decision is. It lives outside the engine because it is a decision about
proceeding, which the engine has no opinion on.

One property of this policy is intentional. It cannot run until a hypothesis has been named,
and nothing in the engine names one on a caller's behalf. An agent that never names a
hypothesis leaves the policy inapplicable — which is a recorded friction, not a broken
harness.

---

## Procedure

### Phase 0 — Construct the world

The harness builds the world and its suite asserts that the two options are distinguishable
by the engine.

This phase contains no agent.

---

### Phase 1 — Brief

The briefing is assembled and committed before anything runs.

It contains the rendered public documentation of `ape`, `world.rs`, and the goal. It
contains nothing about the experiment, its question, its predictions or its criteria.

Committing it first is what makes the run readable afterwards: the briefing is the input,
and an input edited after seeing the output is not an input.

---

### Phase 2 — Decide

The agent acts.

Its output is recorded verbatim, before it is judged, including anything it asked for that
it was not given. Requests for absent operations, fields or concepts are recorded as
requests and are not rewritten into design proposals.

If the agent's code does not compile, the error is recorded and classified by the rule
above.

---

### Phase 3 — Derive

The engine interprets what the agent selected.

Whatever the agent named as its hypothesis is the hypothesis used. The harness does not
substitute one.

Recorded:

```text
the Thesis identity
its Knowledge Cut
the conditions it projects
the conflicts reported, under the hypothesis named
```

---

### Phase 4 — Rule

The policy is applied, unchanged, and its verdict recorded.

A refusal is as valid an outcome as an approval. The experiment does not require the agent
to have chosen the realizable option.

---

### Phase 5 — Observe

Something happens in the world and enters as an Event through the Canon.

What happens is fixed by the harness rather than by the agent, because the agent does not
act on the world — the experiment measures admission, not causation.

Knowledge then advances to recognize it.

---

### Phase 6 — Reconstruct

The Thesis produced at Phase 2 is interpreted again, now that knowledge has moved.

Compared against the Phase 3 record:

```text
Thesis identity      before = after
Knowledge Cut        before = after
projected conditions before = after
conflicts reported   before = after
```

for the same hypothesis.

Reconstruction here is in-process. Whether it survives process death is the CLI's
experiment, and importing that question would confuse two boundaries.

---

## Success criteria

1. The agent's intention was expressible through existing primitives.
2. Every object it constructed asserts what its primitive is defined to assert.
3. The consequence separating the options was derived, not asserted.
4. The policy ran outside the engine and consulted only derived results.
5. What happened was admissible as an Event.
6. The Phase 2 reading survives Phase 5 unchanged.
7. No primitive, field, variant or engine operation was added.

Criterion 6 is the one the experiment exists for. The rest can succeed while it fails, and
that combination would be the most informative outcome available here.

---

## What would end the experiment early

* A concept the agent needs with no honest representation, and no composition producing one.
* An object the agent built whose stated meaning contradicts its primitive's definition, and
  which it declines to restate when the contradiction is put to it.
* The Phase 2 reading changing after Phase 5.

Each is a result. None is a reason to adjust the world until it stops happening.

---

## Observations

Recorded as numbered documents beside this one, as the experiment produces them.

An observation states what was already known from the engine, so that nothing familiar is
presented as a discovery, and names the smallest case that reproduces it.
