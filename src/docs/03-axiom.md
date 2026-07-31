# Axiom

## Introduction

The Kernel defines the ontology of the Assertion Projection Engine.

However, entities alone cannot guarantee that newly introduced knowledge is consistent with the existing knowledge graph.

> _The **Axiom** is the single entry point through which new knowledge enters the Kernel._

Its responsibility is not to execute operations or produce projections.

Its responsibility is to ensure that every new Assertion is structurally consistent before it is emitted.

---

## Purpose

The Axiom acts as the system call interface of the Kernel.

Applications never instantiate Kernel entities directly.

Instead, they request the creation of new knowledge through the Axiom, which resolves references, validates cross-entity invariants, and produces immutable Kernel entities.

Once emitted, Assertions can never be modified.

---

## Responsibilities

The Axiom is responsible for:

* Resolving entity references into immutable identifiers.
* Validating cross-entity invariants.
* Ensuring referenced entities exist.
* Constructing immutable Kernel entities.
* Rejecting structurally inconsistent Assertions.

---

## Non-Responsibilities

The Axiom is **not** responsible for:

* Executing business logic.
* Producing projections.
* Evaluating operational consequences.
* Resolving conflicts between Theses.
* Performing merges.
* Persisting data.

Those responsibilities belong to higher layers.

---

## Consistency

The Axiom validates the consistency of Assertions themselves.

It does **not** validate whether an Assertion produces a desirable or consistent projection.

An Assertion may be structurally valid while leading to conflicting interpretations within a particular Thesis.

Such conflicts are derived by Hermeneia, never by the Kernel.

---

## Knowledge Emission

Only Assertions emitted by the Axiom can become operational knowledge.

Whether an emitted Assertion enters canonical history is decided by the Canon.

Once emitted, they are immutable.

The context in which an Assertion is used (the Thesis that selects it, whichever one an application treats as main) is defined by higher layers.

Knowledge evolves exclusively through the addition of new Assertions.

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
Emit immutable Assertion
      │
      ▼
    Canon  ← decides whether it may become history
```
