# Divergence

## Abstract

The reconstruction experiment established that operational meaning survives process death.

It established it for one world.

An application that reasons about alternatives does not hold one world. It holds a lineage: a world it first considered, the worlds that recognized later history, and the worlds it forked to consider something else. Every one of them was reasoned about, and every one of them is part of what the application decided.

> *Can an application reconstruct every world it considered, when knowledge kept moving between the decisions that produced them?*

The previous experiment never asked. Its subject admitted nothing during the instants its decisions were taken, so the question of what a decision is taken *against* never arose.

This one arranges for it to arise.

---

## Question

A decision is persisted as an instant and, for a genesis, a proposed selection. A cut is resolved from that instant against canonical knowledge.

Resolved against *which* knowledge?

```text
at the time of deciding     →  what was known then
at the time of replaying    →  everything the journal holds
```

Those are the same body only while nothing was admitted after the decision and within the instant it names. Where something was, the instant addresses a later head, and the world that comes back is not the world that was reasoned about.

The experiment therefore asks two things that are one thing:

- can a lineage of alternative worlds be reconstructed at all;
- and what must a decision record so that the world it produced comes back rather than a world that merely shares its date.

---

## Hypothesis

Every world an application considered is reconstructible from durable knowledge and durable decisions alone.

What a decision must record for that to hold is its **position in the sequence of admissions**, and not merely the instant it names.

```text
a decision is a coordinate
├── which instant is being recognized
└── which knowledge that instant is being resolved against
```

The reconstruction experiment persisted the first half and re-derived the second against a sequence that had already moved. The hypothesis is that recording both is sufficient, and that nothing further about a Thesis needs to be kept.

It may be refuted in more than one way. Position may be insufficient — a fork's selection may fail to reproduce for reasons that have nothing to do with cuts. Or it may be unnecessary, if some ordering discipline the application can impose makes the date sufficient after all.

---

## Motivation

A Thesis exists so that an operational world can be reasoned about without being committed to. Forking exists so that more than one can be. If only the last world survives a process, the whole of that machinery is reachable only within a single execution, and an application built on APE would be one that cannot close.

There is a second reason, and it is about the record rather than the mechanism.

What an application decided is not only which world it kept. It is also which worlds it considered and set aside. A lineage that reconstructs only its tip preserves the outcome and discards the deliberation — and the deliberation is what makes the outcome answerable to anyone.

The reconstruction experiment closed with the observation that persistence is capture rather than extraction, and that a `KnowledgeCut` is a position in a sequence rather than a pointer into a stored world. This experiment is where that second half stops being an aside.

---

## Experimental Boundary

This experiment exercises the smallest lineage capable of distinguishing a world that was reasoned about from a world that shares its date.

It includes:

* admissions interleaved with decisions, rather than admitted wholly before them;
* an Event recorded **within the instant a decision names, after that decision was taken**;
* a genesis whose world is refused by its own resource bounds;
* an advancement that recognizes the Event;
* a fork that introduces a commitment the parent did not select;
* durable persistence of admissions and decisions;
* complete process termination;
* reconstruction of **every** world in the lineage, not only its tip;
* comparison of each reconstructed world against the one it reproduces.

It deliberately excludes:

* Synthesis, and any transfer of intent between lineages;
* forks taken and abandoned, which the lineage does not model;
* mutable named references;
* concurrent processes;
* snapshots, indexes and any measurement of what replay costs;
* fractional magnitudes;
* production-oriented user experience.

Those concerns may become later experiments.

They must not influence the structure introduced here unless this experiment itself requires them.

---

## Experimental Subject

The subject must produce a world that is refused, a fact that changes whether it is refused, and an alternative that was chosen rather than imposed.

A single quantifiable resource carries all three, because bounds are what a world can be refused by and a fork is how an application answers that refusal.

```text
cash  ∈ [0, 100]

receive  ──▶ Increase
spend    ──▶ Decrease

A  receive 50    admitted day 5
B  spend   120   admitted day 5      → A and B together leave cash at −70
C  spend   30    admitted day 11     → A and C together leave cash at 20
```

Two statements over one resource, so that the commitments move a level in opposite directions. Both magnitudes and both bounds are integers, so that what is measured is reconstruction and not the associativity of floating-point addition.

The cancellation of `B` is an Event, and **it is recorded within the instant the genesis names, after the genesis was decided**. That is the whole of the arrangement this experiment adds, and everything it expects to find follows from it.

The exact domain is irrelevant. The subject exists to provide a lineage in which a world's identity depends on what was known when it was decided.

---

## Initial State

The experiment begins with no persisted application state.

```text
repository = empty
process    = fresh
```

Knowledge and decisions are then produced in the order written below, and that order is itself part of the subject rather than a detail of the harness.

---

## Procedure

### Phase 1 — Construct

Admit the vocabulary, and admit `A` and `B`.

Decide a genesis at an instant later than their recording, selecting both.

Interpret it, and record:

```text
Thesis identity
Knowledge Cut          → an instant, and an empty chain
selection              → what is frozen, and what is open
commitment conditions
derived level
feasibility verdict    → a refusal, naming the level and the instance
```

The verdict is the point of the phase. A world refused by its own bounds is a world whose reproduction cannot be satisfied by an absence, which is what the previous experiment's feasibility comparison had to settle for.

---

### Phase 2 — Observe

Admit an Event cancelling `B`, **recorded at the instant the genesis names**, and therefore after the genesis was decided.

Nothing about this is irregular. Knowledge arrives when it arrives, and an application does not stop deciding while a day is still in progress.

Decide an advancement to a later instant.

Interpret the world it produces, and record the same coordinates, plus what history imposed.

A cancelled commitment moves no level, so the world that was refused is expected to become a world that is not. Whether that is what the engine reports is for the run to say.

---

### Phase 3 — Diverge

Admit `C`.

Fork the advanced world, introducing `C`.

A fork changes what is selected and inherits its parent's cut, so what this phase adds is a selection reached by choice rather than by history. Interpret it, and record the same coordinates.

At this point three worlds have been reasoned about. Each one is a result, and each one must come back.

---

### Phase 4 — Persist

Persist only what a later process cannot derive.

For every persisted datum, the implementation should be able to answer:

> What semantic reconstruction becomes impossible if this datum is not preserved?

Nothing derived may be written. No identity, no cut resolved into a head, no selection partitioned into frozen and open, no condition, no level, no verdict. Each of those is recomputed, and a repository holding one would keep an answer beside the question it comes from.

That constraint is not this experiment's to relax. It is a published result of another, and changing it is a result rather than an adjustment.

---

### Phase 5 — Terminate

Terminate the process completely.

Reconstruction runs in an operating-system process of its own, so that nothing forbidden can carry correctness across the boundary rather than merely being avoided.

> *The original process is dead.*

---

### Phase 6 — Reload

Start a fresh process.

It receives the repository, the engine, and the explicit inputs required to request the same interpretations. It receives no value the original process computed.

---

### Phase 7 — Reconstruct

Rebuild the lineage from what the repository holds.

Every world, in the order they were decided. A reconstruction that produces only the world the application ended at has answered a smaller question than the one asked.

The reconstruction path must use ordinary APE semantics. A world resolved by reading back an identity would prove nothing about whether that identity is still derivable.

---

### Phase 8 — Compare

Compare each reconstructed world against the one it reproduces.

```text
for every world in the lineage

identity        before = after
Knowledge Cut   before = after     ← instant and head
selection       before = after     ← frozen and open, as partitioned
conditions      before = after
derived level   before = after
feasibility     before = after     ← including the refusal, and what it names
```

And, for the fork specifically, that what it introduced and omitted is what was decided rather than what a comparison of two selections would suggest.

Two comparisons are made, and they answer different questions. Equality between a living reading and a rebuilt one measures the process boundary; it survives a defect in code both sides share. Values written down before the run measure that code. Neither substitutes for the other.

---

## Success Criteria

The hypothesis is confirmed for the experimental boundary when all of the following hold:

1. Every world in the lineage is reconstructed, and not only its tip.
2. Each reconstructed world has the identity of the world it reproduces.
3. Each reconstructed Knowledge Cut addresses the same instant **and the same head** — including the genesis, whose instant addresses a different head once the Event is in the journal.
4. Each selection is partitioned into the same frozen and open commitments.
5. Interpretation of each world at the same effective instant produces equivalent conditions.
6. The refused world is reproduced as refused, with the same conflict naming the same instance and the same level.
7. The fork's introduced and omitted commitments are those the decision recorded.
8. No derived value is persisted.
9. No previously computed projection or identity is required.
10. No persistence-specific concern is introduced into the APE engine for reconstruction to succeed.

---

## Failure Conditions

The hypothesis is refuted, or narrowed, if reconstruction requires information that:

* existed only in the terminated process;
* cannot be derived from persisted admissions and persisted decisions;
* requires reconstructing a Thesis through a path different from its normal semantics;
* requires the engine to understand repository layout or ordering.

And specifically, if:

* a world comes back with a different identity because its cut resolved against knowledge admitted after the decision that took it;
* a selection comes back partitioned differently, so that what was open is frozen or the reverse;
* a refused world comes back unrefused, or refused by a different conflict.

The last is the one the arrangement of the subject is designed to provoke. A refutation there is the expected result of the naive form and is a finding rather than a defeat. What the experiment is for is establishing what makes it stop.

A failed experiment is a valid result. The implementation must not hide failure by persisting derived state until the outputs agree.

---

## Variables Deliberately Left Open

### The form of the coordinate

Whether a decision records a position, or whether admissions and decisions become one interleaved sequence, is not decided here. Both preserve the reason the two are kept apart — what became known is not revisable, and which world is being considered is a choice that may be made again.

### Abandoned siblings

The lineage models the worlds that were kept. A fork taken and discarded is not written down, and this experiment does not introduce it.

If that absence makes something unanswerable, the consequence is that auditability depends on how an application works rather than on what it records — which is a result to be stated, not a gap to be filled ahead of it.

### Addressing

Nothing here requires a world to be named. Positions in a sequence are how the lineage is walked, and whether an application needs more is a later question.

### Ordering discipline

Whether an application could avoid the whole problem by refusing to admit knowledge within an instant a decision names is not assumed either way. It would be a constraint on how an application may behave, which is a different kind of answer from one about what a repository records.

---

## Methodological Constraint

This experiment follows one implementation rule:

> *Do not introduce an abstraction intended to solve an experiment that has not yet been performed.*

Structure may be introduced only when required by the current experimental procedure.

The reconstruction experiment's conclusions are not revised here. Where this one finds something that would have changed them, it is recorded as a finding of this experiment, against the implementation as it then stood.

---

## Expected Pressure Points

The experiment is expected to pressure several boundaries without assuming their outcome.

### The decision coordinate

A cut resolved at replay against a fully replayed journal is resolved against more knowledge than the decision had. This is where the subject aims.

### The frozen and open partition

A selection is partitioned by what a cut made unavoidable. If a cut comes back different, the partition comes back different, and a world may reconstruct with the right commitments in the wrong halves. That would be a quieter failure than a changed identity.

### Fork inputs

`omitted` and `introduced` state an outcome rather than a transition, and the engine tolerates redundancy in them. Whether what a repository records is the request or the result is a distinction that has not yet had to be made.

### Ordering between the two sequences

Admissions and decisions are two files. Replaying one entirely and then the other is what makes a cut resolve against the wrong knowledge. Whether the order between them is payload is the question underneath this whole experiment.

### Interpretation of worlds that are not the tip

The previous experiment interpreted one world. Nothing yet establishes that an older world stays interpretable once history has moved past it — only that it stays *readable*.

These are expected areas of pressure.

They are not conclusions.

---

## Observations

Each observation is recorded as its own numbered document beside this one, as the experiment produces it.

Record facts rather than decisions retroactively presented as inevitable.

Useful observations include:

* information unexpectedly required for reconstruction;
* information initially persisted but later found derivable;
* public interfaces that proved sufficient;
* public interfaces that created pressure;
* assumptions that held only because a subject never arranged for them to be tested;
* accidental coupling between the order things were written and the meaning they carry.

Where possible, record the smallest reproducing case.

---

## Open Questions

The experiment intentionally leaves several questions unanswered.

Among them:

* What should a world be called, when an application holds several?
* Does anything need the worlds a lineage did not keep?
* When several lineages exist, what relates them?
* Does a position in a sequence survive a repository that is written to by more than one process?
* What does a fork cost to reconstruct, when a lineage is long?

These are candidates for later experiments.

They are not requirements for this one.

---

## Experimental Principle

The reconstruction experiment asked whether meaning survives.

This one asks whether *deliberation* does.

```text
If a world was reasoned about,
it should still be there to be reasoned about again.
```

The experiment will determine what an application must record for that to remain true.
