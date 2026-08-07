# ape

> **The engine.**

APE models operational knowledge as immutable assertions about operational reality.

A **Commitment** asserts intended reality; an **Event** asserts observed reality.

From those facts, operational state can be projected, analyzed, combined and evolved over time.

This crate is the engine itself: the ontology, the canonical history that accumulates it, and the interpretation built over that history.

It is a library, and it is meant to be depended upon rather than run.

For what APE is and why it exists, see the [project README](https://github.com/neto-sant-ana/ape).

---

# Layers

Each layer rests on the one before it and never reaches back.

```text
engine
   ↓ interprets
canon
   ↓ accumulates
kernel
```

**`kernel`** — the ontology. Entities and value objects that cannot be constructed in an invalid state, and `kernel::axiom`, the single gateway through which knowledge is admitted once its cross-entity invariants hold.

**`canon`** — the canonical history. What became known, in the order it became known, appended under compare-and-append.

**`engine`** — interpretation of that history. Conditions and feasibility (`engine::hermeneia`), scenarios and their lineage (`engine::thesis`), and the transfer of intent between them (`engine::synthesis`).

---

# Ontology

The ontology is eight primitives: **Agent**, **Role**, **Resource**, **Constraint**, **Action**, **Statement**, **Commitment** and **Event**.

**Assertion** is a category over them rather than a ninth primitive, and the engine reserves the capital. Every immutable knowledge object is an *assertion*; an **Assertion** is a Commitment or an Event, the two kinds that claim a factual instant.

Everything else is composition.

---

# Design Principles

## Declarative by Design

The kernel describes operational semantics.

It never executes business logic.

It never stores operational state.

It never orchestrates workflows.

Those responsibilities belong to higher-level layers.

---

## Atomic Coordination

Every Statement defines exactly one Action.

Every Action affects exactly one Resource.

Complex operational behavior is obtained through composition of multiple commitments rather than larger statements.

---

## Immutable Facts

Operational history is never rewritten.

Events describe observations.

Commitments describe decisions.

Operational knowledge evolves by accumulating facts rather than mutating state.

---

## Derived State

Operational conditions are never manually maintained.

They emerge from the relationships between commitments, dependency graphs, observed events and time.

They are also not a single state.

Settlement — **Unsettled**, **Fulfilled**, **Cancelled** — and timeliness — **WithinDeadline**, **Breached** — are independent axes, because a commitment can be unsettled, waiting on a dependency and breached at once.

Collapsing them into one enumeration would force an order of precedence the domain does not have.

---

## Stable Ontology

APE intentionally provides a minimal ontology.

Rather than continuously introducing new concepts, the engine encourages expressing complexity through combinations of existing primitives.

Its purpose is not to model every business domain directly, but to provide a stable coordination language from which many operational models can be constructed.

---

# Construction Model

Entities cannot be created in an invalid state.

Invariants that depend only on an entity's own data are validated by the entity, or its value objects, at construction.

Invariants that span several entities are validated by a dedicated mechanism of knowledge admission, the single entry point through which entities are created, which resolves the needed references, checks semantic consistency, and only then emits the entity holding stable identities.

Validation stays separate from instantiation, keeping the construction path predictable and composable.

---

# Storage

Where knowledge is kept is not a decision of this crate.

Persistence is expressed as two ports, `canon::CanonicalHistory` and `engine::thesis::ThesisArchive`, which an adapter implements over whatever medium it likes.

Two features serve adapters, and neither is enabled by default:

**`conformance`** — the suites that prove an adapter honors a port. An adapter hands a constructor to `verify` from its own test suite, and the suite drives the contract against a fresh instance.

**`reference`** — in-memory implementations of both ports. They exist so that the contract has a demonstration and so that this crate can test itself. They are not meant to back an application.

---

# Documentation

The design documents the implementation answers to live in [`src/docs`](src/docs), ordered as they were written.

They are rendered as part of the crate documentation, so `cargo doc --open` carries the reasoning and not only the signatures.

---

# License

APE is licensed under the Apache License 2.0.

See the [LICENSE](https://github.com/neto-sant-ana/ape/blob/main/LICENSE) file for details.
