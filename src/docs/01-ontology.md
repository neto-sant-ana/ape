# Ontology

## Introduction

The Assertion Projection Engine does not model industries, organizations or business processes.

It models the minimal set of concepts required to represent operational coordination independently of any particular domain.

This set is intentionally small in favor of a greater stability.

Rather than introducing new primitives for every operational scenario, APE encourages expressing complexity through composition.

The ontology defines the vocabulary of the engine.

Every assertion, projection and operational model is ultimately expressed in terms of these concepts.

---

# Agent

An **Agent** is an operational entity capable of assuming responsibilities and being held accountable for commitments.

An Agent may represent an individual, an organization or any other entity that can legitimately participate in operational coordination.

The ontology is agnostic about the nature of an Agent.

Its only concern is whether that entity can assume operational responsibilities.

---

# Role

A **Role** represents a responsibility that may be assumed by eligible Agents.

Roles describe operational expectations rather than identities.

Multiple Agents may be eligible for the same Role, and the same Agent may assume different Roles in different contexts.

Roles allow Statements to remain reusable without referencing specific Agents.

---

# Resource

A **Resource** is anything whose operational state may be affected by an Action.

Resources may be quantifiable, such as inventory, capacity or balance, or discrete, such as contracts, projects or documents.

The ontology does not distinguish resources by industry.

Only by how they are affected by Actions.

---

# Constraint

A **Constraint** defines the operational boundaries within which reality may evolve.

Constraints represent invariant conditions that remain true regardless of individual Commitments or Events.

While Commitments express intended operational evolution, Constraints define the limits within which that evolution remains feasible.

Operational validity emerges from two complementary mechanisms:

- Dependency relationships govern discrete resources.
- Constraints govern quantifiable resources.

---

# Action

An **Action** describes the semantic operation performed over exactly one Resource.

Actions are intentionally atomic.

Each Action affects one Resource, and each Statement contains exactly one Action.

More complex operational behavior emerges from composing multiple Statements and Commitments rather than expanding the Action itself.

---

# Statement

A **Statement** is a reusable operational proposition.

It defines:

- which Action is being performed (and therefore which Resource it affects);
- which Roles perform that Action;
- which Roles benefit from that Action;
- which kinds of Events may settle or cancel its future Commitments.

A Statement describes operational semantics.

It does not describe a particular execution.

---

# Commitment

A **Commitment** is a concrete instantiation of a Statement.

It binds abstract operational semantics to a specific operational context by assigning accountable Agents, executors, beneficiaries, deadlines and dependencies.

A Commitment represents intended operational evolution.

It exists before execution and coordinates future observations.

---

# Event

An **Event** represents an observed fact of operational relevance.

Events never describe intentions.

They describe observations.

An Event may fulfill or cancel a Commitment according to the semantics defined by its Statement.

Through dependency relationships, this outcome may indirectly affect other Commitments.

Events do not directly store operational state.

They contribute new knowledge from which state may later be projected.

---

# Assertion

The term _Assertion_, the **A** in **A**PE, refers to the immutable knowledge objects the engine manages.

The ontology defines two kinds of assertions:

- _Commitments_, representing intended operational reality.
- _Events_, representing observed operational reality.

Together they form the complete body of operational knowledge represented by the engine.

Assertions are therefore a conceptual category rather than an ontological primitive.

---

# Relationships

The ontology intentionally contains very few primitives.

Their relationships can be summarized as follows.

```text
Agent
    │
assumes
    ▼
Role
    │
participates in
    ▼
Statement  ──performs──▶  Action  ──affects──▶  Resource
    │
instantiated as
    ▼
Commitment
    │
affected by
    ▼
Event
```

Every coordination model built upon APE ultimately reduces to compositions of these concepts.

No additional domain-specific entities are required by the kernel itself.
