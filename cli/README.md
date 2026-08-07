# APE CLI

APE CLI is an executable laboratory for the Assertion Projection Engine.

It demonstrates how an application can persist, reconstruct, inspect, and operate over APE semantics outside the engine’s in-memory environment.

Its purpose is architectural.

---

# Purpose

APE defines a deterministic model for operational evolution.

It introduces a set of core semantics:

* commitments;
* events;
* canonical history;
* projections;
* theses;
* synthesis;
* historical knowledge.

These concepts define *what it means for a system to evolve consistently over time*.

However, a library alone does not answer how those semantics behave when placed inside an application boundary.

APE CLI exists to explore that boundary.

It turns abstract semantics into an executable form.

---

# Application Boundary

The CLI is not a product interface.

It is a structural environment where APE is exercised as a dependency of an application.

It emphasizes clarity of interaction over convenience of use.

This distinction is intentional:

* APE defines meaning.
* Applications define interaction with that meaning.
* CLI exposes the interaction layer explicitly.

```text
APE
→ defines operational semantics

Application
→ defines interaction with those semantics

APE CLI
→ makes that interaction explicit and inspectable
```

---

# Persistence Model

APE assumes that operational knowledge can outlive process execution.

APE CLI explores this assumption through a repository model, directly inspired by Git’s object and reference system.

Like Git, the repository separates identity from interpretation:

* immutable objects preserve identity;
* mutable references express navigation through that identity.

```text
immutable objects
        ↓
persistent identity

mutable references
        ↓
current operational context
```

This separation allows a system to:

* preserve history without mutation;
* reconstruct meaning from stored state;
* evolve interpretation without rewriting origin;
* maintain stable identity across changing viewpoints.

In Git, this manifests as content-addressed objects and movable references such as branches and HEAD. APE CLI adopts the same conceptual structure, but applies it to operational knowledge rather than source code history.

The physical storage format is not part of APE semantics.

It is an application-level concern.

---

# Reconstruction Principle

A central question in APE is not how data is stored, but how meaning is recovered.

APE CLI treats reconstruction as a first-class operation:

> A system must be able to terminate and later re-establish the same operational meaning from persisted state.

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
   ↓
equivalent semantics
```

This property is what distinguishes structural persistence from simple data storage.

---

# Responsibility of the Application Layer

APE CLI owns concerns that arise at the boundary between semantics and execution:

* persistence orchestration;
* repository lifecycle;
* object addressing strategy;
* reference management;
* command interpretation;
* human-readable representation;
* import and export flows;
* application-level policies.

These responsibilities are intentionally kept outside the engine.

They exist to test where abstraction boundaries naturally emerge.

---

# Non-Goals

APE CLI is not intended to become:

* a domain-specific product;
* a workflow system;
* a general-purpose database;
* a distributed platform;
* an autonomous agent system;
* a canonical storage specification for APE.

Its role is to remain minimal enough to expose the engine clearly.

---

# Dependency Direction

The architecture enforces a strict one-way dependency:

```text
ape-cli
   ↓
  ape
```

The engine must remain independent of application concerns.

This ensures that:

* semantics remain stable;
* applications can evolve freely;
* implementation details do not leak into the model.

---

# Evolution Principle

APE CLI does not begin with abstractions.

It begins with concrete application behavior.

Abstractions are introduced only when repeated structural pressure makes them unavoidable.

This leads to a simple principle:

> Let structure emerge from use.

---

# Core Idea

APE defines what operational knowledge is.

APE CLI demonstrates how an application can exist on top of it without distorting its meaning.
