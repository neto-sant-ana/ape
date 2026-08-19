# Reconstruction

## Abstract

APE defines operational meaning independently of application infrastructure.

The engine can construct assertions, preserve canonical history, interpret operational knowledge, maintain alternative intended worlds and analyze intentional transfer.

Those semantics have so far been exercised primarily within a live process.

This experiment asks whether they survive its death.

> *Can an application terminate and later reconstruct the same operational meaning exclusively from persisted information?*

The experiment deliberately begins without prescribing a repository structure, representation or object-addressing scheme.

Its purpose is to discover the minimum information an application must preserve so that APE semantics can be reconstructed rather than merely approximated.

---

## Question

Can operational meaning produced through APE be reconstructed after process termination without preserving process-local state?

More precisely:

Given a body of canonical knowledge and an intended world derived from it, can an application:

```text
construct
    ↓
persist
    ↓
terminate
    ↓
reload
    ↓
reconstruct
```

and obtain results semantically equivalent to those obtained before termination?

The experiment is concerned with meaning rather than representation.

Byte-for-byte equality of application storage is irrelevant.

Semantic equivalence is the property under investigation.

---

## Hypothesis

APE semantics are reconstructible from durable knowledge alone.

If the engine's abstraction boundaries are sufficient, an application should not need to preserve opaque runtime state in order to recover:

* canonical knowledge;
* the canonical Event chain;
* a Thesis and its Knowledge Cut;
* projected Commitment conditions;
* feasibility results derived under the same hypothesis.

Conceptually:

```text
meaning before termination
        =
meaning after reconstruction
```

provided that both executions are supplied with equivalent persisted knowledge and the same explicit interpretation inputs.

No process-local cache, object graph or previously computed projection should be required for correctness.

---

## Motivation

Persistence and reconstruction are different properties.

An application may successfully serialize every object it holds and still fail to reconstruct the meaning those objects previously produced.

Such a system has preserved data.

It has not necessarily preserved operational knowledge.

APE CLI therefore treats process termination as an experimental boundary.

Anything required after that boundary must either:

1. be recoverable from persisted information; or
2. be deterministically derived again.

Anything required for semantic correctness that exists only in process memory identifies architectural pressure.

That pressure may reveal:

* missing application-level persistence;
* an accidental dependency on runtime state;
* an insufficient public read boundary in APE;
* information whose ownership between engine and application is unclear.

The experiment exists to expose these cases.

---

## Experimental Boundary

This experiment exercises the smallest path capable of distinguishing persisted data from reconstructible operational meaning.

It includes:

* construction of a minimal operational model;
* canonical admission of Commitments;
* canonical admission of Events;
* construction of a Genesis Thesis;
* interpretation of that Thesis;
* durable application-level persistence;
* complete process termination;
* reconstruction in a fresh process;
* repetition of the same interpretation;
* comparison of semantic results.

It deliberately excludes:

* alternative Thesis branches;
* Thesis advancement;
* Synthesis;
* mutable named references;
* distributed storage;
* remote synchronization;
* concurrent processes;
* storage optimization;
* hierarchical hashes;
* Merkle structures;
* caching strategies;
* snapshots;
* production-oriented user experience.

Those concerns may become later experiments.

They must not influence the structure introduced here unless this experiment itself requires them.

---

## Experimental Subject

The subject should be the smallest operational graph capable of exercising both intended and observed reality.

It must contain enough knowledge to produce a non-trivial projection after an Event is admitted.

Conceptually:

```text
Agent
  │
Role
  │
Statement ──▶ Action ──▶ Resource
  │
Commitment
  │
Event
```

The exact domain represented by these assertions is irrelevant.

The subject exists only to provide a complete semantic path through APE.

A simple quantifiable Resource is preferred because it allows the reconstructed interpretation to expose both Commitment conditions and a derived factual consequence.

The experiment should not introduce additional domain concepts merely to make the example realistic.

---

## Initial State

The experiment begins with no persisted application state.

```text
repository = empty
process    = fresh
```

The application then constructs the minimum knowledge required by the experimental subject through the ordinary APE boundaries.

No engine entity should be created through a persistence-specific construction path.

Persisted and non-persisted execution must exercise the same public semantics.

---

## Procedure

### Phase 1 — Construct

Create the minimal vocabulary required by the operational subject.

Construct at least one Commitment representing intended operational evolution.

Admit that Commitment into canonical knowledge.

Create a Genesis Thesis selecting the Commitment at the appropriate Knowledge Cut.

Interpret the Thesis before any settlement Event exists.

Record the resulting semantic observations.

At minimum, preserve for comparison:

```text
Thesis identity
Knowledge Cut
Commitment settlement condition
dependency wait
timeliness condition
factual Resource consequence
```

where each item is applicable to the chosen subject.

---

### Phase 2 — Observe

Construct an Event capable of settling the Commitment.

Admit the Event through the Canon.

Advance or derive the intended world only where required by the existing APE semantics to recognize the new canonical knowledge.

Interpret the resulting Thesis.

Record again:

```text
canonical Event Head
Thesis identity
Knowledge Cut
Commitment settlement condition
factual Resource consequence
```

The result of this interpretation becomes the reference observation for reconstruction.

---

### Phase 3 — Persist

Persist only information the application determines is necessary to reconstruct the experiment.

The first implementation must not begin from a predetermined storage ontology.

Instead, persistence structure should be introduced only in response to information that must survive termination.

For every persisted datum, the implementation should be able to answer:

> What semantic reconstruction becomes impossible if this datum is not preserved?

Information that cannot answer this question should be treated with suspicion.

Derived state should not be persisted merely to simplify reconstruction.

---

### Phase 4 — Terminate

Terminate the process completely.

No in-memory object may survive into the reconstruction phase.

The experiment must not preserve correctness through:

* static process state;
* shared in-memory adapters;
* test fixtures reused across phases;
* cached projections;
* previously instantiated Theses;
* hidden initialization data unavailable from persisted state.

For the purposes of the experiment:

> *The original process is dead.*

---

### Phase 5 — Reload

Start a fresh process.

The new process receives only:

* the persisted application repository;
* the APE engine;
* the explicit inputs required to request the same interpretation.

Reconstruct whatever application-level adapters are necessary from persisted information.

No semantic result from the original process is supplied as input.

---

### Phase 6 — Reconstruct

Resolve the same canonical knowledge from durable state.

Resolve the Thesis by its persisted identity or by whatever application-level addressing mechanism the experiment discovers to be necessary.

Interpret the reconstructed Thesis using the same effective time and, where applicable, the same feasibility hypothesis used before termination.

The reconstruction path must use ordinary APE semantics.

It must not deserialize a previously computed projection and call that reconstruction.

---

### Phase 7 — Compare

Compare the original and reconstructed observations.

The experiment is concerned with semantic equality.

At minimum:

```text
canonical head before
    =
canonical head after

Thesis identity before
    =
Thesis identity after

Knowledge Cut before
    =
Knowledge Cut after

projected conditions before
    =
projected conditions after

factual Resource consequence before
    =
factual Resource consequence after
```

If feasibility is exercised:

```text
feasibility(H) before
    =
feasibility(H) after
```

for the same explicit hypothesis `H`.

Internal allocation, retrieval order, filesystem layout and serialized byte representation are not part of the comparison.

---

## Reconstruction Invariant

The experiment proposes the following application-level invariant:

> *Process lifetime must not participate in operational meaning.*

For equivalent durable knowledge and equivalent explicit interpretation inputs:

```text
interpret(
    reconstruct(persist(state))
)
=
interpret(state)
```

This notation is conceptual.

`state` does not mean a mutable APE state object.

It denotes the complete application context required to reproduce the operational world under examination.

The experiment exists partly to discover what belongs in that context.

---

## Success Criteria

The hypothesis is confirmed for the experimental boundary when all of the following hold:

1. The original process can terminate completely.
2. A fresh process can reconstruct canonical knowledge exclusively from persisted information.
3. The reconstructed Event chain has the same canonical identity and ordering.
4. The reconstructed Thesis has the same identity and Knowledge Cut.
5. Interpretation under the same effective time produces equivalent Commitment conditions.
6. Any factual Resource consequence is reproduced.
7. Any feasibility verdict requested under the same hypothesis is reproduced.
8. No previously computed projection is required.
9. No process-local application state is required for semantic correctness.
10. No persistence-specific concern must be introduced into the APE engine for reconstruction to succeed.

The final criterion is essential.

A successful reconstruction that requires changing the engine to understand the CLI's storage model would falsify the intended application boundary even if the resulting values happened to match.

---

## Failure Conditions

The hypothesis is refuted, or at least narrowed, if reconstruction requires information that:

* existed only in the terminated process;
* cannot be derived from canonical knowledge and persisted Thesis information;
* is semantically necessary but has no available durable representation;
* requires reconstructing an engine object through a path different from its normal semantics;
* requires the engine to understand repository layout or representation;
* causes identical semantic inputs to produce different identities or interpretation results.

A failed experiment is a valid result.

The implementation should not hide failure by persisting additional derived state until the outputs happen to match.

The reason reconstruction failed is more valuable than a passing test obtained by storing the answer.

---

## Variables Deliberately Left Open

The following are intentionally not specified before the experiment:

### Repository Layout

No directory structure is prescribed.

### Repository Representation

No application-level encoding format is prescribed.

### Object Addressing

Content-addressing may be useful, but the experiment does not assume that every persisted application object must be content-addressed.

### Mutable References

The experiment does not yet require names such as `main`, `HEAD` or branches.

A Thesis identity may be addressed directly if that is sufficient.

### Storage Granularity

The experiment does not determine whether application persistence should mirror engine entity boundaries.

### Indexes

No secondary lookup structure should be introduced unless reconstruction demonstrates a concrete need for one.

### Optimization

Repeated reads, duplicated representation or linear reconstruction are acceptable.

The first question is whether reconstruction is correct, not whether it is fast.

---

## Methodological Constraint

This experiment follows one implementation rule:

> *Do not introduce an abstraction intended to solve an experiment that has not yet been performed.*

Structure may be introduced only when required by the current experimental procedure.

If several later experiments exert the same pressure, that repetition may justify an abstraction.

Until then, the duplication is evidence.

Premature abstraction would erase it.

---

## Expected Pressure Points

The experiment is expected to pressure several boundaries without assuming their outcome.

### Canonical Read Model

Reconstruction may reveal which forms of canonical lookup an application genuinely requires after process death.

### Event Chain Reconstruction

The application must recover the factual chain without depending on insertion order held only in memory.

### Thesis Resolution

A Thesis is immutable and content-derived.

The experiment should reveal what durable information is necessary to resolve one without treating its previous in-memory representation as canonical.

### Knowledge Cut Recovery

The application must preserve enough information to recover the exact epistemic and factual context recognized by the Thesis.

### Derived State

Any temptation to persist settlement, timeliness, feasibility or Resource levels should be examined as a possible violation of reconstruction by derivation.

### Canonical Representation

The experiment does not prescribe how canonical APE representations are framed, organized or stored by the repository. Where the engine already defines a deterministic representation required by object identity, the CLI must preserve rather than redefine it.

The experiment may discover application-level metadata or envelopes required around canonical representations. Those are repository concerns and must remain distinguishable from the representation that gives APE objects their identity.

Canonical hashing format and public durable interchange format may coincide, but are not conceptually the same promise.

These are expected areas of pressure.

They are not conclusions.

---

## Observations

Each observation is recorded as its own numbered document beside this one, as the
experiment produces it.

Record facts rather than decisions retroactively presented as inevitable.

Useful observations include:

* information unexpectedly required for reconstruction;
* information initially persisted but later found derivable;
* awkward or excessive engine reads;
* public interfaces that proved sufficient;
* public interfaces that created pressure;
* assumptions that existed only because tests previously shared memory;
* representation choices that remained purely application-local;
* accidental coupling between storage and semantics.

Where possible, record the smallest reproducing case.

---

## Open Questions

The first experiment intentionally leaves several questions unanswered.

Among them:

* What should provide human-readable names for persisted identities?
* Which mutable references does an operational application actually need?
* How should multiple Theses be navigated?
* What information is necessary to reconstruct ancestry efficiently?
* Should canonical objects and Thesis objects share an addressing mechanism?
* When does repeated reconstruction justify snapshots or indexes?
* What happens when Source, Base and Target must all survive process death for Synthesis?
* Which repository structures emerge repeatedly enough to deserve stable abstraction?

These questions are candidates for later experiments.

They are not requirements for this one.

---

## Experimental Principle

APE CLI exists to exercise the engine without protecting it from application reality.

This first experiment therefore begins with the most basic durability claim:

```text
If operational meaning is structural,
process death should not erase it.
```

The experiment will determine what an application must preserve for that statement to remain true.
