# Kernel

## Introduction

The APE Kernel is the minimal implementation of the ontology.

Its purpose is not to solve operational problems directly, but to provide a stable foundation upon which operational models, projection engines and applications can be built.

The kernel intentionally favors stability over completeness.

Rather than introducing specialized concepts for every use case, it provides a small set of composable primitives whose semantics remain stable over time.

Every implementation decision follows this principle.

---

# Stable Ontology

The kernel implements only concepts that are expected to remain valid regardless of operational domain.

Business-specific concepts such as orders, invoices, projects or contracts are intentionally excluded.

They are compositions built upon the kernel rather than primitives of the kernel itself.

A stable ontology is more valuable than a comprehensive one.

---

# Statements are Reusable

A Statement defines operational semantics.

It describes which Action is performed — and thereby which Resource it affects — who performs it, who benefits from it, and which Events may settle future Commitments.

Statements intentionally contain no execution-specific information.

Deadlines, accountable Agents, executors, dependencies and quantitative values belong to Commitments.

This separation allows the same Statement to be instantiated many times without duplicating operational knowledge.

---

# Commitments Bind Operational Context

A Commitment transforms a reusable Statement into a concrete operational intention.

It binds abstract semantics to a specific context by assigning accountable Agents, executors, beneficiaries, deadlines, dependencies and Action values.

This distinction separates operational knowledge from operational planning.

Statements describe what a commitment means.

Commitments describe where and when that meaning applies.

---

# Actions are Atomic

Every Action affects exactly one Resource.

Consequently, every Statement contains exactly one Action.

This restriction is intentional.

Composite operations emerge from coordinating multiple Commitments rather than embedding multiple Actions into a single Statement.

Atomic Actions simplify reasoning, dependency analysis and future projection.

---

# Events Describe Facts

Events represent observed operational facts.

Unlike Commitments, Events do not instantiate Statements.

They simply record observations capable of affecting existing Commitments according to the semantics already defined by their Statements.

Events remain intentionally small.

Their meaning is derived from the Commitments they affect rather than duplicated within the Event itself.

---

# State is Never Stored

Operational state is never considered canonical.

States such as Open, Fulfilled, Blocked or Cancelled are projections derived from Commitments, Events, Constraints and dependency relationships.

The kernel therefore stores knowledge rather than state.

Every state may be reconstructed from accumulated assertions.

---

# Constraints Define Feasibility

Commitments express intended operational evolution.

Constraints define the operational boundaries within which that evolution remains feasible.

The kernel treats Constraints as part of operational reality rather than projection logic.

Projection evaluates them.

It does not define them.

---

# Validation Preserves Invariants

Invalid operational models must not be constructible.

Invariants that depend only on an entity's own data are enforced by that entity, or its value objects, at construction.

Invariants that span several entities are enforced by a dedicated mechanism of knowledge admission, the single gateway through which entities are created: it resolves the referenced objects, validates semantic consistency, and only then emits the entity.

Either way, invalid coordination models fail at build time rather than during projection.

---

# Identity over References

Kernel entities relate to one another through stable identities.

The knowledge admission may temporarily require references in order to validate semantic consistency.

Once validation succeeds, only identities are retained.

This separation keeps the operational graph independent from implementation details while allowing rich validation during construction.

---

# Composition Defines Complexity

The kernel intentionally avoids introducing specialized primitives for increasingly complex operational scenarios.

Instead, complexity emerges from composing stable concepts.

Operational sophistication should result from richer graphs rather than richer ontologies.

This principle keeps the kernel small while allowing operational models to grow without changing its foundations.

---

# Design Principles

The implementation of the kernel follows a small number of architectural principles.

* Stable ontology over complete ontology.
* Composition over specialization.
* Identity over references.
* Knowledge over state.
* Atomic Actions over composite operations.
* Validation at construction time.
* Projection derives meaning rather than storing it.

These principles guide the evolution of the kernel and should remain stable independently of programming language or implementation details.
