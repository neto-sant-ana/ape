# APE — Assertion Projection Engine

> **Bringing Git's philosophy to operational evolution.**

APE (Assertion Projection Engine) is an open-source coordination engine inspired by the architectural principles that made Git one of the most successful distributed systems.

Git transformed software engineering by modeling software evolution as a graph of immutable commits.

APE applies the same philosophy to operational evolution.

Instead of modeling workflows, processes or state machines, APE models operational assertions, commitments and events. From these facts, operational state can be projected, analyzed, merged and evolved over time.

The goal is not to replace ERPs, CRMs or workflow systems.

The goal is to establish a canonical coordination engine that any operational software can build upon.

---

# Why?

Most operational systems models **processes** as the primary abstraction.

APE models **assertions**.

A process describes _how_ work should flow.

An assertion describes _what_ is expected to become true.

Rather than embedding business rules inside workflows and state transitions, APE represents operational knowledge as immutable assertions connected by explicit relationships.

From these assertions, dependencies and observed events, every operational state becomes a projection instead of a stored truth.

This distinction allows planning, execution and historical analysis to coexist without duplicating business logic.

---

# Inspiration

Git demonstrated that software evolution could be represented as a graph of immutable facts instead of mutable files.

APE explores the same idea for operational coordination.

Git coordinates software evolution.

APE coordinates operational evolution.

Both systems are founded on the same architectural principles:

- immutable facts
- explicit relationships
- graph-based evolution
- derived state
- composition over specialization

The domains are different.

The philosophy is intentionally shared.

---

# Core Philosophy

APE is intentionally small.

The engine avoids domain-specific concepts of industries, departments or business processes, keeping the kernel stable, deterministic and extensible.

Instead, it defines an ontology of a few universal concepts capable of expressing coordination in many operational domains.

Complex behavior emerges from composing simple concepts rather than introducing increasingly specialized abstractions.

---

# Core Concepts

The kernel is centered around the following concepts:

- **Agent** — an operational entity capable of assuming roles and being held accountable for commitments.
- **Role** — a responsibility that may be fulfilled by eligible agents.
- **Resource** — something affected by an action.
- **Action** — a semantic operation performed over exactly one resource.
- **Statement** — a reusable operational proposition describing who performs which action for whom.
- **Commitment** — a concrete instantiation of a Statement, binding agents, deadlines and dependencies.
- **Event** — an observed operational fact that affects a commitment.
- **Projection** — the result produced by the engine after evaluating commitments, events and dependency graphs.

Everything else is composition.

---

# Design Principles

APE follows a few fundamental rules.

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

Operational states such as **Open**, **Blocked**, **Fulfilled**, **Cancelled** or **Breached** are not manually maintained.

They emerge from the relationships between commitments, dependency graphs, observed events and time.

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

# Vision

Git demonstrated that a stable, immutable graph of objects could coordinate software development at a global scale.

APE explores the same philosophy for operational coordination because it often benefits from the same principles:

- immutable facts
- explicit relationships
- derived state
- composition over specialization
- distributed evolution

Git became the language of software collaboration.

APE aims to become the language of operational coordination.

# License

APE is licensed under the Apache License 2.0.

See the [LICENSE](LICENSE) file for details.
