# Canon

## Introduction

The Axiom guarantees that Assertions are structurally consistent.

Structural consistency alone, however, does not guarantee that a new Assertion may safely become part of the canonical operational history.

> *The ****Canon**** is responsible for preserving the integrity of the canonical history.*

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
* Resolving scenario conflicts.
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
* guaranteeing idempotent admission.

The exact set of invariants may evolve without changing the role of the Canon.

---

## Head-Relative Validity

> _A history validation only remais valid while the observed head remains the same._

A validation against the canonical history is not absolute: it holds only as of the head it observed.

The instant the head advances, a conclusion drawn against the prior head may be stale (a commitment seen as unsettled may now be settled, a reference seen as absent may now exist).

The Canon therefore admits atomically against the observed head: if the head has moved, admission is refused and any subsequent attempt mut be re-evaluated against the new head.

The same discipline governs any reader of the history; a projection is valid
only as of the head it was computed from.

---

## Canonical Metadata

Some information does not belong to the semantic meaning of an Assertion, but to its admission into canonical history.

The Canon is responsible for assigning such metadata before persistence.

Examples include recording timestamps and other history-specific metadata required to preserve canonical integrity.

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
