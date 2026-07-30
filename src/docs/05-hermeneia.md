# Hermeneia

## Introduction

The Kernel defines the concepts through which operational knowledge is represented.

The Axiom ensures that Assertions are structurally consistent.

The Canon preserves the integrity of their admission into canonical history.

However, accumulated knowledge does not directly express operational state.

> _**Hermeneia** is a deterministic interpretation of operational knowledge under a specific historical and temporal context._

Its responsibility is not to create, modify or admit Assertions.

Its responsibility is to derive the operational consequences of immutable Commitments and Events.

---

## Purpose

A Projection transforms accumulated operational knowledge into an interpretable operational view.

Commitments describe intended reality.

Events describe observed reality.

Dependencies describe the relationships between intentions.

Constraints define the boundaries within which their combined effects remain feasible.

Projection evaluates these elements together and derives their operational consequences.

Conditions such as Unsettled, Fulfilled, Cancelled, Breached, or waiting on a dependency exist only within a Projection.

They are never stored as canonical knowledge.

---

## Vocabulary

A Commitment within a Projection is **Settled** or **Unsettled**, according to whether an Event in the history has already settled it. This is the Settlement axis described below.

Nothing further qualifies it. A Projection reads one selection of Commitments, and within that selection every Commitment carries the same standing.

Events are never qualified either, for a different reason. An Event is a fact, so there is no hypothetical Event and no Event needs to be called canonical.

_Canonical_ retains the single meaning established by the Canon: admitted into canonical history.

---

## Inputs

A Projection is derived from:

- A selected Commitment graph.
- Knowledge as known at an instant.
- An effective time.

```text
Projection =
    Selected Commitment Graph
    + Knowledge as known at K
    + Effective Time T
```

The Commitment graph already contains the operational context required for projection, including referenced Statements, Actions, Resources, dependencies, values and Constraints.

Resources and Constraints are therefore not separate projection inputs.

They are part of the operational knowledge reachable through the Commitment graph.

---

## Knowledge Time

Canonical recording time determines when knowledge became available.

Commitment and Event times determine when operational intentions and observations apply.

Recording time does not replace factual time.

Knowledge time is therefore addressed by an instant of its own, distinct from every factual time in the ontology.

A knowledge instant does not address a single structure. It addresses two, because the ontology records knowledge in two places.

```text
Knowledge as known at K
├── Commitments recorded no later than K
└── Event chain reachable from the head resolved at K
```

The head is resolved from `K` as the latest Event recorded no later than it, and the chain reachable from that head is the Event history as known at `K`.

Commitments are selected by their own recording time. A Commitment never enters the chain, so its recording time is its only knowledge-time coordinate and cannot be derived from the head.

A head and a knowledge instant are consequently not interchangeable. The head delimits Events; only the instant delimits both families. This is why recording time is carried by every canonical record rather than by the chain alone.

Knowledge time only ever addresses the past. There is no future knowledge, and the chain always stops at the head.

### What the Canon must guarantee

Addressing knowledge by an instant is only meaningful while the knowledge that instant selects cannot change. The Canon therefore keeps recording **monotonic across admission**: no assertion may be recorded before knowledge already admitted.

Without it, an assertion admitted today carrying a recording instant from last year would enter every projection taken as of that period. The same question would answer differently before and after, and a past interpretation would stop being reproducible.

The guarantee has to span every family rather than the chain alone. A Commitment never enters the chain, so back-dating one is invisible to any guarantee the chain could offer — and it is precisely a Commitment appearing retroactively that rewrites the past without touching a single Event.

Spanning every family also settles the chain as a special case. An Event cannot be recorded before the Event it extends, since that one was admitted earlier. This is what makes the walk backwards from a head a search rather than a scan: without it the walk could reach an Event recorded after the requested instant, leaking later knowledge into an earlier interpretation.

Resolution is bounded by the resolution of the recording instant. Several Events recorded at the same instant collapse into a single addressable cut, and the latest of them in chain order is the head. Addressing a head directly remains the finer selection.

---

## Operational Time

Historical knowledge and operational time are distinct dimensions.

The head defines which Events are available as knowledge and, consequently, which Commitment settlements can be derived.

The effective time defines the operational instant being interpreted.

```text
Knowledge time
    determined by the recording instant, resolved into a head

Operational time
    determined by the effective time
```

An unsettled Commitment may remain within its deadline at one effective time and become Breached at another, even when no new Assertion has been admitted.

Projection is therefore a function of both accumulated knowledge and time.

```text
Projection = f(Knowledge as known at K, Effective Time T)
```

Unlike knowledge time, operational time addresses the past and the future alike.

This separation allows the engine to answer different questions over the same history.

Examples include:

- What is the current operational state?
- What was the operational state at an earlier time?
- What was known at an earlier instant about an earlier operational time?
- What does current knowledge imply about a future operational time?
- How has the interpretation changed as new knowledge was admitted?

### Interpreting the future requires a hypothesis

Nothing is known about what has not happened yet, so any statement about a future operational time rests on a hypothesis. Different derived conditions rest on different ones.

- **Timeliness assumes that nothing further is observed.** An unsettled Commitment whose deadline precedes the effective time is projected as Breached. This carries the status quo forward.
- **Feasibility evaluates hypothetical completions**, each under a hypothesis it names.

The two are not contradictory. They answer different questions over the same knowledge.

No projected result about the future is produced without the hypothesis that produced it.

---

## Determinism

A Projection must be determined exclusively by its inputs.

Given the same Commitment graph, knowledge and effective time, the same Projection must always be produced.

```text
project(C, K, T) = P
```

Where:

- `C` is the selected Commitment graph;
- `K` is the knowledge instant;
- `T` is the effective time;
- `P` is the resulting Projection.

Projection is determined by admitted knowledge, not by the order in which applications presented it.

The Canon, by contrast, is order-sensitive by design: references must exist before the Assertions naming them, and a Commitment is settled by the first Event admitted against it. Which submissions are admitted therefore does depend on their order. Determinism belongs to the interpretation of what *was* admitted, and does not extend to admission itself.

Admission preserves facts.

Projection derives their operational consequences.

---

## Accumulation and Interpretation

A Projection is not required to read the whole history from the first Event.

It separates what accumulates over knowledge from what is interpreted at the end.

```text
P = view( fold(knowledge) , T )
```

**fold** accumulates only what is factual and monotonic:

- which Commitments are known, from their recording time;
- which settlements are established, from the chain;
- the factual level of each affected Resource.

It does not depend on `T`.

**view** derives the conditions — settlement, availability, timeliness, feasibility — from that accumulation and `T`. It accumulates nothing.

Nothing in `fold` depends on a later Event. Settlement is terminal and established at most once, and factual Resource movements accumulate. Whether a dependency is still pending is derived by `view` from the settlements, never accumulated.

### The factual level

Only what happened moves a Resource.

```text
Fulfilled Commitment    → contributes its Action effect
Cancelled Commitment    → contributes nothing; its effect never occurred
Unsettled Commitment    → contributes nothing to the factual level
```

The level in `fold` is therefore the factual level. Movements promised by unsettled Commitments belong to feasibility, which introduces them as hypothetical additions to this level rather than as accumulated facts.

### Snapshots

A snapshot is a checkpoint of `fold`, never of `view`.

Resuming means folding the remaining knowledge onto a checkpoint and then interpreting at whichever `T` is asked. A checkpoint that had baked in a deadline evaluation would carry an interpretation valid only for the instant it was taken.

```text
fold(knowledge up to K2)  =  fold( checkpoint at K1 , knowledge between K1 and K2 )

P at any T                =  view( that accumulation , T )
```

A checkpoint carries the knowledge coordinate it was taken at. Because a head is content-addressed, the chain identified by a checkpoint is verifiable: the delta is the walk from the current head back to it.

---

## Derived Conditions

Operational state is never canonical.

It emerges from the interaction between Commitments, Events, dependencies, Constraints and time.

A Projection derives complementary operational conditions for individual Commitments and identifies feasibility conflicts over the Commitment graph.

Commitment conditions may be evaluated along several independent axes, such as:

- **Settlement**
  - **Unsettled** — no admitted Event settles the Commitment.
  - **Fulfilled** — an admitted Event fulfills the Commitment.
  - **Cancelled** — an admitted Event cancels the Commitment.

- **Dependency wait** — whether any dependency is still unsettled.

  This axis reports only that narrow, negative fact, and deliberately does not name it as a condition of the Commitment. A dependency settled either way, fulfilled *or* cancelled, stops the waiting; so a Commitment can be waiting on nothing and still be impossible to realize. Calling that state *available* or *ready* would assert what the dependencies alone cannot establish, and a consumer acting on the label would schedule work this engine already knows cannot be done. Whether a terminal dependency leaves its dependent realizable is feasibility's question — a different dimension, not a harder computation over the same one.

- **Timeliness**
  - An unsettled Commitment may remain within its deadline or become **Breached** after that deadline elapses.

These conditions are complementary rather than mutually exclusive.

A Commitment may, for example, be simultaneously Unsettled, waiting on a dependency and Breached.

These names describe projected consequences rather than Kernel entities.

The exact public representation of projected conditions may evolve without changing the underlying ontology.

---

## Settlement

Settlement is derived from Events.

A Commitment is fulfilled or cancelled only when an applicable Event produces the corresponding effect defined by its Statement.

```text
Commitment
    │
    ▼
Applicable Event
    │
    ├── Fulfill
    └── Cancel
```

The Canon guarantees that a Commitment cannot be settled more than once in canonical history.

Projection interprets the admitted settlement.

It does not enforce settlement admission.

---

## Dependencies

Commitment dependencies form a directed acyclic graph.

A dependency is a requirement, and it carries a definite meaning rather than a merely temporal one. It states two things about the Commitment holding it:

- it is **executable** only while no dependency is still pending;
- it is **realizable** only if every dependency — all of them, and only them — is fulfilled.

The two are separate questions over the same edge, and a dependency that reaches a terminal outcome answers them differently. It stops the waiting either way, so a Commitment whose dependencies were all cancelled is executable in the sense that it awaits nothing. But cancellation is not fulfilment, so that Commitment can no longer be realized: **a cancelled dependency renders its dependent infeasible.**

This is why the dependency axis reports only the waiting and leaves realizability to feasibility. Both derive from the same edge; they are not the same judgment.

Infeasibility travels along the graph. A Commitment made infeasible by a cancelled dependency can never be fulfilled, so anything depending on *it* cannot be realized either, and the consequence follows the dependency path to its end. Reaching the immediate conclusion needs only the neighbouring edge; carrying it downstream is what needs the graph.

Projection does not alter dependency relationships.

It derives their operational effects.

---

## Constraints

Constraints define the operational boundaries within which Resources may evolve.

A quantifiable Resource is affected by the Action of each Commitment reaching it: the Effect gives the direction and the Commitment's value gives the magnitude.

Projection accumulates the factual movements into a level and evaluates that level, together with the hypothetical movements a feasibility hypothesis introduces, against the Constraint.

### Where a level begins

The engine defines no opening level for a Resource.

An initial level, where one exists, is expressed as operational knowledge like any other: the first Commitments over that Resource and the Events that settle them. The application asserts them.

This keeps the level entirely derived from accumulated knowledge, and adds nothing to the ontology.

---

## Feasibility

Assertions may be structurally valid and canonically admitted while the future they collectively describe remains impossible to realize.

Feasibility is derived by `view`. It evaluates whether the selected Commitment graph admits a valid completion, given the settlements known at the selected head.

```text
feasible(C, H(K), hypothesis)
```

Feasibility uses K only to resolve the settlements known at that instant; unlike the full Projection, it does not use an effective time.

Violating a deadline is a breach, not an infeasibility: a Commitment past its deadline remains realizable, only late. No judgment about deadlines therefore enters feasibility, and the instant being consulted drops out of its inputs.

A verdict consequently depends only on the graph and the known settlements. It changes as the head advances, never as the consulted time moves.

### The hypothesis is explicit

A verdict is meaningless without the hypothesis that produced it, so the hypothesis is an input and it labels the result.

```text
final_state
→ every unsettled Commitment is realized, in no particular order
→ checks the level once every movement has landed

on_due_date_net
→ every unsettled Commitment is realized on the day it is due,
  a settled one where it was observed
→ checks the level once each date's movements have landed

on_due_date_in_any_order
→ the same assumption
→ checks every level any arrangement *within* a date can produce
```

The two `on_due_date` readings share one assumption and differ in what they ask of it. What separates them is the ambiguity that lives inside a single date: movements sharing one are genuinely unordered, so an excursion that opens and closes within a date is visible to one reading and not the other.

No hypothesis is the truth about the graph. Each answers a bounded question, and the verdicts are asymmetric.

```text
final_state violated        → infeasible under every realization
final_state satisfied       → nothing proven about the realizations

on_due_date violated        → the punctual realization does not hold
on_due_date_net satisfied   → the level closes each date within bounds
on_due_date_in_any_order
  satisfied                 → no arrangement consistent with the due dates breaches
```

`final_state` detects impossibility, never establishes possibility. A valid sum says nothing about the path to it: under `0 <= level <= 50`, the movements `+60` and `-20` accumulate to a valid `40`, yet one order exceeds the upper bound and the other falls below the lower. No realization exists at all.

`on_due_date` tests the latest punctual realization, not every punctual one. Its failure means that schedule cannot happen — actionable as replanning — not that the graph is impossible. A Commitment realized before its deadline may rescue a plan the hypothesis rejects.

Neither `on_due_date` reading is the weaker one, and reading `net` as an approximation of `in_any_order` misses what it is for. Only `in_any_order` can promise that no admissible arrangement breaches. But a Constraint that means a closing balance — stock at inventory, a position at end of day — is asking exactly what `net` answers, and an excursion that opens and closes inside one date never violates it. `net` also always answers, which `in_any_order` cannot promise: see *Deadlines that coincide* below.

A further question is stronger than all of these, and none of them answers it:

```text
eventually
→ Does there exist any dependency-respecting realization whose
  every intermediate level is valid?
```

It is deliberately not offered. Answering it means searching over orders rather than fixing one, and what it buys is a verdict of *not doomed* — which coordinates nothing the named hypotheses do not already coordinate better, at a cost none of them pay. It is stated here so that the bounded hypotheses are not mistaken for it, and so that its absence reads as a decision rather than an omission.

### Why a fixed hypothesis removes the search

Whether a Resource stays within its Constraint *throughout* its evolution depends on the order in which Commitments are realized. Dependencies establish only a partial order, so asking whether *some* admissible order stays within bounds means quantifying over exponentially many orders.

Under `on_due_date` the order stops being an unknown and becomes data.

```text
settled Commitments     → positioned by the instant they occurred     (fact)
unsettled Commitments   → positioned by their due date                (hypothesis)
cancelled Commitments   → contribute no movement
```

The sequence is derived and the levels are accumulated along it. Both readings check every level the sequence produces between dates; they differ only in what they judge of the movements a single date carries. Neither searches for an order — the order is data.

The opening level is not judged. A Resource before anything moved it is where the model starts rather than a state a Commitment produced, and introducing that judgment under one hypothesis would have the hypotheses disagree about something none of them asks.

The hypothesis constrains only what remains unsettled. History pins everything it has already touched.

### Deadlines that coincide

Commitments sharing a due date are not ordered by the hypothesis, so their relative order is open again — within that group only.

Order within a group is free only where precedence does not fix it. A movement may land once every dependency of it sharing that date already has, so what can have landed so far is any subset of the group **closed under those dependencies** — and the levels the group can produce are the sums of exactly those subsets. `on_due_date_in_any_order` judges every one of them, which is exact for *any* Constraint, including one whose forbidden values sit between its extremes and which a check of the extremes alone would miss.

Judging every subset instead, closed or not, would contradict the section above: a dependency the hypothesis has just been shown to respect would then be evaluated against arrangements that violate it. A credit of 10 and a debit of 10 falling on one date, the debit waiting on the credit, can only go `0 → 10 → 0`; the trajectory through `-10` belongs to an order the graph forbids, and reporting it would be a conflict against something that is not a completion of the graph.

Exactness costs enumeration, and enumeration is exponential in the group. Past a bound the group is refused rather than approximated: a verdict that skipped an arrangement would read as clean for a graph that breaches, and a false clean verdict is worse than an admitted inability to answer. A model itemized finely enough — a movement per invoice line, all due at month end — reaches that bound easily, which is why `on_due_date_net` exists rather than being a lesser version of the same thing. It enumerates nothing and therefore never refuses.

### Inverted deadlines

A dependency requires its dependent to settle after it. A due date only bounds when each of them is expected.

The two can disagree: a Commitment may depend on another whose due date is later than its own.

Such a graph is not invalid, and the knowledge describing it is admitted like any other. It remains realizable, because a dependency may be fulfilled well before its deadline. What it can never be is realized *punctually*: no arrangement satisfies either `on_due_date` reading while respecting the dependency.

Projection reports the hypothesis as unrealizable over that dependency path.

The plan is named as defective where it is interpreted, without refusing the knowledge that describes it.

### What a conflict identifies

Feasibility may depend on the combined consequences of several Commitments.

A Projection may therefore identify an infeasible set, dependency path or affected Resource without attributing the conflict to a single Assertion.

Projected infeasibility does not mean that any Assertion should not have been constructed or admitted.

The Assertions remain valid operational knowledge.

Infeasibility means that their combined promised consequences do not admit a consistent realization under the settlements known at the selected head and the hypothesis under which they were evaluated.

As new Events are admitted, the known settlements change. A previously feasible Commitment graph may therefore become infeasible, and a previously infeasible graph may become feasible.

---

## Scenarios

A Projection operates over a *selected* Commitment graph. It does not decide how that selection is formed.

```text
Projection = the graph a Scenario selects + Event history
```

Every Scenario projects identically, and no Scenario is privileged by these semantics.

Projection does establish one boundary, because it follows from the ontology rather than from any Scenario policy: **the Event history is always factual.** A Scenario reasons about intentions. It cannot introduce an observation, and it cannot retract one.

How a Scenario composes its graph — which Commitments it may introduce, omit or replace, and what a dependency on an omitted Commitment implies — is defined by the Scenario layer.

---

## Merge

Merging a Scenario into canonical history asks whether the intentions it introduces can join the canonical graph without producing an unrealizable operational future.

Projection supplies the material for that judgment: a feasibility verdict over the combined graph, labelled with the hypothesis that produced it.

It does not decide which hypothesis a merge must ask, nor what a failing verdict should prevent. Those are policy, and they belong to the Merge layer.

---

## Non-Retroactive Knowledge

Every Projection is valid only relative to the knowledge from which it was produced, following the head-relative validity the Canon establishes for any reader of the history.

Newly admitted knowledge may change the current interpretation of previous Assertions. It does not change what was known earlier.

```text
Knowledge at K1 → Projection P1
Knowledge at K2 → Projection P2
```

`P1` remains a valid interpretation of the knowledge available at `K1`. It is not a valid representation of the knowledge available at `K2`.

This preserves both properties:

- operational interpretation evolves as knowledge grows;
- historical knowledge remains reconstructible.

New projections may derive different consequences without revising canonical history.

---

## Immutability

A Projection is an immutable result.

It does not mutate Commitments, Events, Resources, Constraints or canonical history.

When any input changes, a new Projection is produced.

```text
Knowledge K1 + Time T1 → Projection P1
Knowledge K2 + Time T1 → Projection P2
Knowledge K2 + Time T2 → Projection P3
```

No Projection updates another Projection.

Each result remains attributable to the context from which it was derived.

---

## Non-Responsibilities

Projection is **not** responsible for:

- Constructing Kernel entities.
- Validating Assertion structure.
- Admitting Assertions into canonical history.
- Modifying canonical knowledge.
- Creating hypothetical Events.
- Persisting projected state as canonical truth.
- Composing the Commitment graph a Scenario selects.
- Deciding which feasibility hypothesis a merge must satisfy.
- Resolving how conflicting Scenarios are merged.
- Orchestrating application workflows.

Those responsibilities belong to the Axiom, Canon, Scenario, Merge, Engine or application layers.

> _Projection does not interpret the business. It derives the consequences of the operational model defined by the application._

---

## Example

```text
Selected Commitment Graph
            │
            ├───────────────┐
            │               │
            ▼               ▼
       Dependencies     Constraints
            │               │
            └───────┬───────┘
                    │
    Event history as known at K
                    │
                    ▼
               Projection
                    │
                    ▼
       Derived Operational View
```

The resulting view may contain Commitment conditions, dependency availability, temporal breaches, resource feasibility and operational conflicts.

None of these derived values become canonical Assertions.

---

## Principles

Projection follows a small number of principles.

- State is derived, never stored.
- Projection is determined by admitted knowledge, not by submission order.
- Projection validity is relative to the knowledge it was produced from.
- Knowledge time and operational time are distinct.
- Accumulation is factual and resumable; interpretation is not accumulated.
- Every statement about the future is labelled by the hypothesis that produced it.
- A bounded hypothesis is never mistaken for the truth about the graph.
- New knowledge changes interpretation without rewriting history.
- Operational conflicts are projected rather than rejected retroactively.
- Valid knowledge may produce infeasible operational consequences.
- Commitments are interpreted, never mutated.
- Events remain facts and cannot be hypothetical.
- Complexity emerges from the Commitment graph rather than additional ontology.
