# APE — Assertion Projection Engine

> **Bringing Git's philosophy to operational evolution.**

APE (Assertion Projection Engine) is an open-source coordination engine inspired by the architectural principles that made Git one of the most successful distributed systems.

Git transformed software engineering by modeling software evolution as a graph of immutable commits.

APE applies the same philosophy to operational evolution.

Instead of modeling workflows, processes or state machines, APE models operational knowledge as immutable Assertions: Commitments, which assert an intended reality, and Events, which assert an observed one. From these facts, operational state can be projected, analyzed, combined and evolved over time.

The goal is not to replace ERPs, CRMs or workflow systems.

The goal is to establish a canonical coordination engine that any operational software can build upon.

---

# Why?

Most operational systems model **processes** as the primary abstraction.

APE models **assertions**.

A process describes _how_ work should flow.

An assertion describes _what_ is the case, or _what_ is expected to become true.

Rather than embedding business rules inside workflows and state transitions, APE represents operational knowledge as immutable assertions connected by explicit relationships.

From those assertions, their dependencies and the passage of time, every operational state becomes a projection instead of a stored truth.

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

# Repository

The workspace holds two crates.

**[`ape`](core)** — the engine, in [`core/`](core). The ontology, the canonical history that accumulates it, and the interpretation built over that history. It executes no business logic, stores no operational state and orchestrates no workflow.

**[`ape-cli`](cli)** — an executable laboratory, in [`cli/`](cli). APE exercised as a dependency of an application, so the boundary between semantics and use can be inspected rather than assumed.

The dependency runs one way, and never back.

```text
ape-cli
   ↓
  ape
```

Storage format, addressing strategy, command vocabulary and presentation are decisions of the application. None of them may become a decision of the engine.

---

# Documentation

The design documents the implementation answers to live in [`core/src/docs`](core/src/docs), ordered as they were written: philosophy, ontology, then one per layer.

They are rendered as part of the crate documentation, so the reasoning travels with the signatures.

```sh
cargo doc --open -p ape   # the engine, its layers and the documents
cargo test --workspace    # the suites
```

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

---

# License

APE is licensed under the Apache License 2.0.

See the [LICENSE](LICENSE) file for details.
