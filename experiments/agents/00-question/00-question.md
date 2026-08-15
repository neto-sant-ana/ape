# The Question

## Abstract

APE models operational knowledge as immutable knowledge objects, and derives every
operational condition from them rather than storing it.

The engine was designed for a caller that knows what it intends. Every existing exercise of
it — the suites, the CLI — has a planner that decided beforehand what would be committed.

This experiment replaces that planner with an autonomous LLM agent.

> *Can an autonomous LLM agent express, evaluate, execute and later reconstruct an
> operational decision through APE without extending its ontology?*

The experiment does not ask whether the agent decides well. It asks whether whatever the
agent decided can be stated, derived from, and later recovered through the primitives that
already exist.

---

## Question

An agent given a goal and a world will produce two things.

It produces a **decision** — something it intends to make true — and a **justification**,
in prose, for why that decision was the one it took.

APE already has a place for the first. A Commitment asserts intended operational reality;
an Event asserts observed operational reality; a Thesis fixes which knowledge a reading was
taken under.

It has no place for the second, and the experiment must not create one.

So the question decomposes into four:

```text
express       Can the agent's intention be stated as knowledge APE admits?
evaluate      Can its consequences be derived rather than asserted?
execute       Can what actually happened be admitted as observed reality?
reconstruct   Can the decision be recovered later, under what was knowable then?
```

Each is answered against the current public boundary, with no primitive added and no
existing one repurposed.

---

## Hypothesis

APE's existing ontology is sufficient to carry an agent's operational decision, and the
concepts an agent additionally produces — goal, prompt, reasoning, confidence, model
identity, narrative justification — belong outside the engine.

If the hypothesis holds, then accountability for an autonomous decision is not a feature
somebody must add to APE.

It is a consequence of two properties the engine already has: knowledge is immutable, and a
Thesis fixes the Knowledge Cut a reading was taken under.

---

## On execution

One of the four verbs is weaker than it sounds, and saying so now avoids presenting a known
boundary as a later discovery.

APE executes nothing. It has no scheduler, no effect system and no way to cause anything to
happen in the world. An Event asserts that something *was observed*, after the fact and by
somebody else.

So `execute` in the question above means:

> *Can what the world did in response to the agent's intention be admitted as an Event
> settling the Commitment the agent created?*

The experiment measures admission, not causation. Anything that actually performs work is
outside the boundary and stays there.

---

## What counts as extending the ontology

The hypothesis is worth nothing if the failure is not defined before the run.

The experiment fails the *no extension* criterion if any of the following is required to
express, evaluate or reconstruct the decision:

**A new primitive.** A new entity, a new field on an existing input, a new variant of a
kernel enum.

**A new engine operation.** Something the engine must compute that it does not compute
today.

**Repurposing.** An existing primitive made to carry a meaning the ontology does not give
it — a confidence encoded as an action value, a prompt stored as an Identifier, a model name
admitted as an Agent whose Role is fictional.

The third is the dangerous one, because it compiles.

An LLM asked to express an unfamiliar concept through a fixed vocabulary will find a slot
that type-checks, and the result reads as a success. The experiment therefore requires that
every knowledge object the agent constructs be accompanied by a plain sentence stating what
it asserts, and that sentence be checked against the ontology's own definition of that
primitive.

An object whose stated meaning does not match its primitive's definition is a refutation
recorded as such, never a mapping to be tidied up afterwards.

---

## Experimental boundary

It includes:

* a world small enough that every consequence can be checked by hand;
* one agent, acting alone;
* one goal, stated without naming an option;
* construction of intended reality by the agent, through ordinary APE boundaries;
* derivation of consequences through Hermeneia;
* an application-level policy deciding whether the intention may proceed;
* admission of what subsequently happened;
* reconstruction of the decision under the knowledge available when it was taken;
* comparison of the agent's narrative against the derived record.

It deliberately excludes:

* more than one agent;
* Synthesis;
* tool use beyond the APE boundary;
* persistence across process death, which is the CLI's experiment and not this one;
* any measurement of decision quality;
* prompt optimization;
* an agent framework, a runtime or an orchestration layer.

Those may become later experiments. None of them may influence the structure introduced
here unless this experiment requires it.

---

## The world

The subject is deliberately too small to be interesting, so that anything that goes wrong is
attributable.

```text
Resource      cash, which may not go below zero

Known         the balance is 100

Options       A — spend 30
              B — spend 120

Goal          given to the agent, naming neither option
```

Under this world one option is realizable and the other is not, and the difference is
derivable rather than asserted.

The agent is not told which is which.

The exact domain is irrelevant. The world exists only to produce one intention whose
consequence the engine can decide, and one later fact that changes how the decision looks
without changing what was knowable when it was taken.

---

## The sequence

Three experiments, each of which must survive before the next is worth running.

**01 — Single agent.** The agent receives the world and the goal, constructs an intention,
and a policy external to the engine decides whether it may proceed. Then something happens,
enters as an Event, and knowledge advances. The question closing the experiment is whether
the world known at the moment of decision, and the intention selected then, can be
reconstructed exactly.

**02 — Hindsight.** Knowledge advances again, this time with information that makes the
original decision look bad — an obligation nobody could have known about. The experiment
asks whether the engine distinguishes, without being taught to, between the decision judged
under what was knowable and the same decision judged under what is known now.

**03 — Narrative mismatch.** The agent is made to justify its choice with a claim the
derived record contradicts — that its option was the only realizable one, when two were.
The experiment asks what, if anything, in the record exposes the mismatch, and where the
boundary between a narrative trace and operational evidence actually falls.

The interesting outcome here is not the one that sounds like a lie. A claim about what was
*viable* is only true relative to a hypothesis and to a criterion for which movements count,
and the engine deliberately fixes neither on the caller's behalf. So the mismatch to expect
is an unstated criterion rather than a false statement — a sentence that is true under the
fold the agent silently used and false under the one its reader assumes.

If all three survive, multiple agents and Synthesis become the next question. That is not
in this boundary and must not shape it.

---

## Success criteria

The hypothesis is confirmed for this boundary when all of the following hold:

1. The agent's intention is expressible through existing primitives, with each object's
   asserted meaning matching that primitive's definition.
2. The consequence separating a realizable option from an unrealizable one is derived by the
   engine, not asserted by the agent.
3. The policy deciding whether the intention proceeds lives entirely outside the engine.
4. What subsequently happened is admissible as observed reality.
5. The knowledge available at the moment of decision is recoverable afterwards.
6. Later knowledge does not alter what the original reading was taken under.
7. Nothing in the record requires the engine to know that a decision was made by an LLM.
8. No primitive, field, variant or engine operation is added.

The seventh criterion is the one that makes the result interesting. If the engine has to
know its caller is an agent, the property being claimed is not structural.

---

## Failure conditions

The hypothesis is refuted, or narrowed, if:

* a concept the agent genuinely needs has no honest representation;
* a representation exists but only by giving a primitive a meaning the ontology does not
  give it;
* a consequence the policy requires cannot be derived and must be asserted;
* the knowledge available at the moment of decision cannot be recovered once knowledge
  advances;
* later knowledge changes the reading a Thesis previously produced;
* distinguishing a defensible decision from a bad one requires information the engine
  cannot represent.

A refutation is a valid result and is not rewritten once the engine changes. A later
experiment may test a revised engine; this one records what the current engine did.

---

## Non-goals

This experiment is not:

* an agent framework;
* a benchmark of any model's judgment;
* research into prompting;
* an argument that APE is for AI;
* a case for adding governance, audit or explainability features to the engine.

The claim under test is the opposite of a feature request. It is that the property is
already there, or that it is not.

---

## Methodological constraint

The experiment follows one rule, inherited from the reconstruction experiment and applied
here to the ontology rather than to storage:

> *Do not extend the ontology to accommodate the hypothesis. Let the agent use the engine
> that exists, and let the friction show what is missing.*

The temptation this rule exists to refuse is concrete and immediate: `Decision`,
`Reasoning`, `AgentRun`, `Prompt`, `Evidence`, `Confidence`. Every one of them is plausible,
and not one of them has yet been shown to be necessary.

Complexity is meant to arrive by composition, not by growth of the ontology. An experiment
that grows the ontology in order to succeed has answered a different question.

---

## Open questions

Deliberately unanswered before the run:

* What, if anything, must be recorded about *who* decided, beyond the Agent the ontology
  already has?
* Is a narrative justification an application concern, or is it nothing at all?
* Does an agent need to read the engine differently than a program does?
* What does an agent reach for that the public boundary does not offer?
* Is a policy that consults derived consequences an application concern in general, or only
  in a world this small?
* Does anything change when the agent is not alone?

These are candidates for later experiments. They are not requirements for this one.
