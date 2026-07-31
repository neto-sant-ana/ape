# Canon

## Introduction

The Axiom guarantees that Assertions are structurally consistent.

Structural consistency alone, however, does not guarantee that a new Assertion may safely become part of the canonical operational history.

> _The **Canon** is responsible for preserving the integrity of the canonical history._

**Axiom determines whether an Assertion may exist. Canon determines whether it may become history.**

Its responsibility is not to define operational semantics or execute projections.

Its responsibility is to ensure that every Assertion admitted into the canonical history preserves the invariants of that history.

---

## Purpose

The Canon acts as the canonical admission layer of the Assertion Projection Engine.

Applications submit structurally valid Assertions produced by the Axiom.

The Canon validates historical invariants, enriches Assertions with canonical metadata when necessary, and admits them into the canonical history.

Once admitted, Assertions become part of the immutable operational record.

---

## Responsibilities

The Canon is responsible for:

* Validating invariants that depend on the existing canonical history.
* Enriching Assertions with canonical metadata.
* Preserving the integrity of the event history.
* Preventing duplicate admission of equivalent Assertions.
* Admitting Assertions atomically into the canonical history.

---

## Non-Responsibilities

The Canon is **not** responsible for:

* Validating Assertion structure.
* Constructing Kernel entities.
* Executing business logic.
* Producing projections.
* Resolving conflicts between Theses.
* Defining how canonical history is persisted.

Those responsibilities belong to the Axiom, the Engine or the application itself.

---

## Historical Consistency

The Canon validates properties that emerge only from accumulated operational knowledge.

Unlike the Axiom, which validates Assertions in isolation, the Canon validates Assertions in the context of the existing canonical history.

Typical historical invariants include:

* preserving a single continuous event chain;
* ensuring the uniqueness of the initial Event;
* validating historical references;
* guaranteeing idempotent admission;
* keeping the recording of Assertions monotonic across admission.

The exact set of invariants may evolve without changing the role of the Canon.

---

## Head-Relative Validity

> _A history validation holds only while the observed head remains the same._

A validation against the canonical history is not absolute: it holds only as of the head it observed.

The instant the head advances, a conclusion drawn against the prior head may be stale (a commitment seen as unsettled may now be settled, a reference seen as absent may now exist).

The Canon therefore admits atomically against the observed head: if the head has moved, admission is refused and any subsequent attempt must be re-evaluated against the new head.

The same discipline governs any reader of the history; a projection is valid only as of the head it was computed from.

---

## Canonical Metadata

Some information does not belong to the semantic meaning of an Assertion, but to its admission into canonical history.

The Canon is responsible for assigning such metadata before persistence.

Examples include recording timestamps and other history-specific metadata required to preserve canonical integrity.

---

## Monotonic Recording

Recording time is what makes canonical knowledge addressable by an instant, and that addressing is only meaningful while the knowledge an instant selects cannot change.

The Canon therefore keeps recording **monotonic across admission**: no Assertion may be recorded before knowledge already admitted.

Without it, an Assertion admitted today carrying a recording instant from last year would silently join the knowledge of that period. Nothing in the history would have been rewritten, and yet the past it reports would have changed.

The guarantee spans every family of Assertion rather than the event chain alone. A Commitment never enters the chain, so back-dating one is invisible to any guarantee the chain could offer — and it is precisely a Commitment appearing retroactively that rewrites the past without touching a single Event.

Spanning every family also settles the chain as a special case: an Event cannot be recorded before the Event it extends, since that one was admitted earlier.

What readers of the history derive from this guarantee is defined by the layers that interpret it.

---

## Persistence

The Canon does not persist Assertions directly.

Instead, it depends on an application-provided abstraction representing the canonical history.

Whether that history is implemented using object storage, relational databases, embedded databases or any other persistence mechanism is entirely outside the scope of the engine.

This separation keeps the Canon independent from infrastructure while guaranteeing consistent historical admission.

---

## Example

```text
Application
      │
      ▼
    Axiom
      │
      ▼
Structurally valid Assertion
      │
      ▼
    Canon
      │
      ▼
Validate historical invariants
      │
Enrich canonical metadata
      │
Atomically admit into history
      │
      ▼
Canonical History
```
