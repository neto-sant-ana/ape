# Axiom

## Introduction

The Kernel defines the ontology of the Assertion Projection Engine.

However, entities alone cannot guarantee that newly introduced knowledge is consistent with the existing knowledge graph.

> _The **Axiom** is the single entry point through which new knowledge enters the Kernel._

Its responsibility is not to execute operations or produce projections.

Its responsibility is to ensure that every new assertion is structurally consistent before it is emitted.

---

## Purpose

The Axiom acts as the system call interface of the Kernel.

Applications never instantiate Kernel entities directly.

Instead, they request the creation of new knowledge through the Axiom, which resolves references, validates cross-entity invariants, and produces immutable Kernel entities.

Once emitted, assertions can never be modified.

---

## Responsibilities

The Axiom is responsible for:

* Resolving entity references into immutable identifiers.
* Validating cross-entity invariants.
* Ensuring referenced entities exist.
* Constructing immutable Kernel entities.
* Rejecting structurally inconsistent assertions.

---

## Non-Responsibilities

The Axiom is **not** responsible for:

* Executing business logic.
* Producing projections.
* Evaluating operational consequences.
* Resolving conflicts between Theses.
* Transferring intentions between Theses.
* Persisting data.

Those responsibilities belong to higher layers.

---

## Consistency

The Axiom validates the consistency of assertions themselves.

It does **not** validate whether an Assertion produces a desirable or consistent projection.

An Assertion may be structurally valid while leading to conflicting interpretations within a particular Thesis.

Such conflicts are derived by Hermeneia, never by the Kernel.

---

## Knowledge Emission

Only assertions emitted by the Axiom can become operational knowledge.

Whether an emitted assertion enters canonical history is decided by the Canon.

Once emitted, they are immutable.

The context in which an Assertion is used (the Thesis that selects it, whichever one an application treats as main) is defined by higher layers.

Knowledge evolves exclusively through the addition of new assertions.

---

## Example

```text
Application
      │
      ▼
    Axiom  ← reads existing Knowledge
      │
      ▼
Resolve references
      │
      ▼
Validate cross-entity invariants
      │
      ▼
Emit immutable assertion
      │
      ▼
    Canon  ← decides whether it may become history
```
