# Thesis

## Introduction

The canonical history records how operational knowledge became available.

Operational intention, however, does not evolve through a single unavoidable path.

Organizations continuously compare alternatives, revise unsettled plans and explore different continuations of the same known reality.

> _A **Thesis** is an immutable and historically closed context of intended operational evolution._

Every Thesis:

- selects a complete Commitment graph;
- recognizes exactly one canonical Knowledge Cut;
- preserves every factual and causal consequence established by that cut.

Its responsibility is not to invent an alternative observed history.

Its responsibility is to define one possible continuation of intended reality from a historically valid body of knowledge.

A Thesis enables operational exploration without permitting anachronism or reinterpretation of observed facts.

---

## Purpose

A Thesis provides an isolated context in which operational intention may evolve.

It allows applications to:

- preserve an established body of known facts;
- retain, omit or replace intentions that remain open;
- introduce Commitments that were available at the recognized cut;
- evaluate alternative operational continuations;
- preserve the historical context in which decisions were made;
- reconstruct what was intended under earlier knowledge;
- support future comparison and combination of operational continuations.

A Thesis contains no hypothetical Events.

Events describe observed reality and belong exclusively to canonical history.

Theses differ only in how intended reality continues from the knowledge they recognize.

---

## Shared History, Alternative Intentions

All Theses derive from the same canonical history.

They do not create parallel factual timelines.

```text
Canonical knowledge

Commitments admitted over time
Events admitted into one chain
              │
              ▼
        Knowledge Cut K
              │
      ┌───────┼───────┐
      ▼       ▼       ▼
  Thesis A Thesis B Thesis C
```

The Knowledge Cut defines what was historically knowable to the Thesis.

Every Thesis recognizing the same cut accepts the same factual past and the same epistemic horizon.

It may select a different intended future.

It may not select a different observed past.

---

## Knowledge Cut

A **Knowledge Cut** identifies one historically valid cut of canonical knowledge.

```text
Knowledge Cut K
├── known_at
└── event_head
```

Its two coordinates delimit different families of Assertions.

```text
known_at
→ bounds which Commitments were available for selection

event_head
→ bounds which Events belong to the recognized factual chain
```

A Commitment does not belong to the Event chain.

Therefore, an Event Head alone cannot identify the complete body of knowledge recognized by a Thesis.

A Thesis must be anchored not only to what had happened, but also to what could be known.

> _A Thesis is anchored not merely to what had happened, but to what could be known._

---

## Historical Cut

A Knowledge Cut represents a real historical cut.

Its coordinates are not selected independently.

For a given `known_at`, the canonical history resolves the latest Event recorded no later than that instant.

```text
head_as_of(known_at) → addressed Event Head
```

The ordinary Knowledge Cut for that instant is:

```text
K =
    {
        known_at,
        event_head: head_as_of(known_at)
    }
```

This guarantees that the Thesis does not selectively ignore Events already known at that cut.

A cut does not represent:

> Commitments known at one instant combined with an arbitrarily older factual history.

It represents one coherent historical body of knowledge.

---

## Fine-Grained Cuts

Canonical recording time currently has civil-day resolution.

Several Events may therefore share the same `recorded_at`.

At that resolution, the ordinary cut for a date resolves to the latest Event of that date in canonical chain order.

Applications may also address a finer cut within the same recording instant.

```text
E1 ──▶ E2 ──▶ E3

recorded_at(E1) =
recorded_at(E2) =
recorded_at(E3) = D
```

Possible cuts include:

```text
K1 = (D, E1)
K2 = (D, E2)
K3 = (D, E3)
```

A named Event is a valid finer cut only when:

1. it has the same `recorded_at` as the head addressed by the instant;
2. it lies on the canonical chain ending at that addressed head.

```text
valid_within(D, E) iff

recorded_at(E) = recorded_at(head_as_of(D))

and

E lies on the chain to head_as_of(D)
```

The first condition ensures that the finer cut remains within the same addressable instant.

The second ensures that the Event is a real prefix of that historical cut rather than a merely contemporaneous Event from another reach.

An incoherent Knowledge Cut is not constructed.

---

## Recording Resolution

The precision of a Knowledge Cut is limited by the canonical recording-time type.

With the current `Date` value object:

> Knowledge recorded on the same civil day belongs to the same coarse temporal address.

Commitments admitted on the same date cannot be ordered by `known_at` alone.

Events retain finer ordering through their canonical chain.

A fine-grained Event cut can therefore distinguish prefixes within one day, while Commitment availability remains bounded only at daily resolution.

The model prevents anachronism between addressable recording dates.

It does not claim intraday precision for Commitment admission.

---

## Recognized Knowledge Cut

Every Thesis recognizes exactly one Knowledge Cut.

```text
Thesis T
└── Recognized Knowledge Cut K
    ├── known_at
    └── event_head
```

The cut defines:

- which Commitments were available to be selected;
- the complete Event chain recognized by the Thesis;
- the Commitments that became historically unavoidable;
- the exact knowledge context under which the Thesis may be interpreted.

A Thesis may be interpreted only under its own cut.

```text
interpret(T@K, K') → permitted iff K' = K
```

Both directions of mismatch are invalid.

```text
K' later than K
→ the Thesis may omit knowledge unavailable at its creation

K' earlier than K
→ the Thesis may expose Assertions not yet knowable at that cut
```

Hermeneia cannot repair either mismatch by filtering or extending the selection.

> _The selection is the world._

If another Knowledge Cut must be interpreted, another Thesis must be used.

---

## Canonical Availability

Every Commitment selected by a Thesis must have been canonically admitted no later than its recognized cut.

```text
recorded_at(C) ≤ K.known_at
```

This rule applies to:

- Commitments selected by a Genesis Thesis;
- Commitments introduced during a fork;
- Commitments incorporated into the frozen causal past during advancement.

A Commitment that exists in the current repository but was admitted after the Thesis cut is not available to that Thesis.

```text
recorded_at(Cnew) > K.known_at

fork(T@K, +Cnew) → invalid
```

The relevant question is not:

> Does this Commitment exist now?

It is:

> Was this Commitment knowledge at the cut recognized by the Thesis?

---

## Provenance Boundary

A Thesis validates knowledge membership relative to its declared historical cut.

It does not attest when an application physically created the Thesis.

Preventing applications from declaring future-dated cuts is a provenance policy.

An application with access to the current canonical horizon may impose:

```text
known_at ≤ current recorded horizon
```

That policy does not belong to Thesis semantics.

The Thesis layer is responsible for preventing Assertions later than the declared cut from entering the selected world.

It is not responsible for proving when the application actually made the decision.

---

## Parent Thesis

Every non-genesis Thesis identifies the Thesis from which it was derived.

```text
Parent Thesis
     │
     ▼
Child Thesis
```

The parent supplies the intentional context available for continuation.

It determines:

- the selected Commitment graph from which the child derives;
- the open intentions available for preservation, omission or replacement;
- the intentional ancestry of the child.

Parenthood does not determine unavoidable reality.

The recognized Knowledge Cut does.

> _The Knowledge Cut provides what a Thesis must recognize._

> _The Parent Thesis provides what it may continue._

---

## Genesis Thesis

A Genesis Thesis has no parent.

It establishes the first selected Commitment graph of a Thesis lineage at a canonical Knowledge Cut.

```text
Genesis Thesis
├── Parent Thesis: None
├── Recognized Knowledge Cut: K
└── Selected Commitment Graph
```

A Genesis Thesis may begin at any historical cut.

It does not need to precede the first Event.

```text
Canonical history

E1 ──▶ E2 ──▶ E3 ──▶ H5

Genesis Thesis at K5
```

It must already include the complete frozen causal past required by its cut.

```text
Genesis@K =
    Frozen(K)
    ∪ Initial Open Selection
```

If no Event has been observed by the cut:

```text
K.event_head = None
Frozen(K) = ∅
```

A Genesis Thesis is not a different ontological kind.

It differs only by having no parent.

---

## Fork

Forking creates a new Thesis from a parent without changing its Knowledge Cut.

```text
fork(T@K, Δintent) → T'@K
```

The complete cut is inherited:

```text
child.known_at   = parent.known_at
child.event_head = parent.event_head
```

A fork changes intention under exactly the same historical knowledge.

It may:

- preserve open Commitments;
- omit open Commitments;
- replace open Commitments with newly constructed Commitments;
- introduce additional Commitments available at the cut.

It may not:

- remove the frozen causal past;
- introduce a Commitment recorded after the cut;
- change the recognized Event chain;
- advance the knowledge horizon.

A fork is an intentional transition.

It never represents growth of knowledge.

---

## Fork Boundary

A fork separates:

- the factual and causal past that cannot be reinterpreted;
- the unsettled intentional future that may still be revised.

This boundary is causal, not merely temporal.

```text
Frozen(K)
→ cannot be omitted or rewritten

Open(T)
→ may be preserved, omitted or replaced
```

The complete origin of a fork is therefore:

```text
Fork origin
├── Parent Thesis
└── Parent Knowledge Cut
```

---

## Advancement

Canonical knowledge may grow after a Thesis has been created.

Growth may occur through:

- new Commitments;
- new Events;
- both.

```text
K ──▶ K'
```

The existing Thesis remains unchanged.

```text
T@K remains T@K
```

To obtain a Thesis under the later cut, the application derives a new Thesis through advancement.

```text
advance(T@K, K') → Advancement
```

Advancement changes knowledge while preserving intention.

```text
fork
→ same knowledge, different intention

advance
→ later knowledge, preserved intention
```

---

## Advancement over the Complete Cut

Advancement is defined over the complete Knowledge Cut, not over `known_at` or `event_head` independently.

A target cut is later when neither coordinate regresses and at least one advances.

Conceptually:

```text
K < K' iff

K.known_at ≤ K'.known_at

and

K.event_head ≤chain K'.event_head

and

at least one relation is strict
```

This is a product order.

It is not a lexicographic order.

A later `known_at` does not compensate for a regressed Event Head.

```text
(D2, E5) → (D3, E3)

invalid when E3 precedes E5
```

Likewise, the same `known_at` may still admit a valid advancement when the factual refinement advances.

```text
(D3, E1) → (D3, E2)

valid when E1 precedes E2
```

---

## Valid Advancement Forms

### Knowledge Horizon Advances, Event Head Remains

```text
(D2, E1) → (D3, E1)
```

This occurs when new Commitments become known but no new Event is observed.

The child recognizes a later Commitment horizon under the same factual chain.

### Event Refinement Advances Within the Same Instant

```text
(D3, E1) → (D3, E2)
```

This occurs when both Events belong to the same recording date but `E2` follows `E1` in the canonical chain.

### Both Coordinates Advance

```text
(D2, E1) → (D3, E4)
```

New Commitments and new Events became known.

### Same Cut

```text
(D3, E2) → (D3, E2)
```

Invalid.

No coordinate advanced.

### Factual Regression

```text
(D3, E2) → (D4, E1)
```

Invalid.

The recording horizon advanced, but the factual chain regressed.

### Temporal Regression

```text
(D3, E2) → (D2, E2)
```

Invalid.

The Commitment horizon regressed.

---

## Advancement Semantics

Advancement must:

- ensure the target recording instant does not precede the parent instant;
- ensure the target Event Head is reachable from the parent Event Head;
- require at least one coordinate of the cut to advance;
- preserve the parent’s selected open future;
- incorporate the frozen causal past required by the target cut;
- produce a new immutable Thesis;
- report Commitments imposed by the advanced history;
- expose rather than repair consequences introduced by new facts.

Conceptually:

```text
advance(T@K, K') →
    Advancement {
        thesis: T'@K',
        imposed: Frozen(K') − Commitments(T)
    }
```

The resulting Thesis is:

```text
T'@K' =
    Frozen(K')
    ∪ Preserved Parent Open Selection
```

Advancement does not automatically select every Commitment admitted between the two cuts.

Newly available but unsettled Commitments merely become eligible for a later fork.

```text
advance(T@K1, K2) → T'@K2

fork(T'@K2, +Cnew) → T''@K2
```

This preserves the separation between knowledge and intention.

---

## Historical Imposition

Canonical history may require an advanced Thesis to include Commitments absent from its parent.

This occurs when an Event in the newly recognized chain settles a Commitment not selected by the parent.

```text
Parent Thesis T@K:
    C2 omitted
    C2' selected
```

Later canonical history settles `C2`.

```text
advance(T@K, K') →
    Child Thesis containing:
        C2 fulfilled
        C2' open
```

The Thesis layer cannot infer whether `C2'` was intended to replace, complement or merely coexist with `C2`.

Hermeneia may also find no conflict unless their combined effects violate a Constraint.

Advancement therefore reports historical imposition explicitly.

```text
Imposed(T, K') =
    Frozen(K')
    − Commitments(T)
```

This report does not claim:

- that a replacement existed;
- that the child is infeasible;
- that either Commitment should be removed;
- that automatic repair is possible.

It states only:

> Canonical history required Commitments that were absent from the parent Thesis.

---

## Settled Past

A Commitment is settled at a Knowledge Cut when an Event reachable from the cut’s Event Head fulfills or cancels it.

```text
Settled(K) =
    Commitments settled by Events reachable from K.event_head
```

Once settled, its identity and structure belong to the established operational past.

Every Thesis recognizing that cut must preserve it unchanged.

A settled Commitment may not be:

- removed;
- replaced;
- assigned a different Statement;
- assigned different participants;
- assigned different dependencies;
- assigned a different Action value;
- otherwise reinterpreted.

A Thesis may explore what follows from a fact.

It may not explore a world in which that fact meant something else.

---

## Frozen Causal Past

Preserving settled Commitments alone is insufficient.

A settled Commitment may depend on other Commitments whose own settlement has not been observed.

Removing those dependencies would retrospectively alter the causal structure in which the settled Commitment existed.

```text
Frozen(K) =
    Settled(K)
    ∪ Ancestors(Settled(K))
```

The frozen causal past contains:

- every Commitment settled in the Event chain recognized by `K`;
- every transitive dependency required by those Commitments.

It may include Commitments that never belonged to the parent Thesis but became historically unavoidable through shared Events.

Every Commitment in `Frozen(K)` must appear structurally unchanged in every Thesis recognizing `K`.

```text
Frozen(K) ⊆ Commitments(Thesis@K)
```

The factual past is preserved together with the causal graph that gives it meaning.

---

## Historical Closure

A Thesis is historically closed when its selected Commitment graph contains the complete frozen causal past required by its Knowledge Cut.

```text
HistoricallyClosed(T@K) iff

Frozen(K) ⊆ Commitments(T)
```

Historical closure is required for interpretation.

Hermeneia absorbs the canonical Event chain contiguously.

Every Event must resolve the Commitment it settles from the Thesis selection.

Events cannot be filtered by Thesis membership.

Consequently, a Thesis missing any Commitment required by its recognized Event chain is not interpretable.

Historical closure is not feasibility.

A historically closed Thesis may still contain:

- conflicting future Commitments;
- impossible dependency outcomes;
- violated quantitative Constraints;
- contradictions already established by observed facts.

Historical closure determines whether the selected world can be interpreted.

Hermeneia determines its consequences.

---

## Open Future

A Thesis partitions its selected Commitment graph into:

```text
Selection(T@K)
├── Frozen(K)
└── Open(T@K)
```

The frozen set is determined by the recognized Event chain.

The open set contains selected Commitments that do not belong to the frozen causal past.

During a fork, open Commitments may be preserved, omitted or replaced.

During advancement, the parent open selection is preserved unless a Commitment becomes frozen under the later Event chain.

```text
ChildOpen =
    ParentOpen
    − Frozen(K')
```

The sources of the two partitions are distinct.

```text
Frozen causal past
← canonical Event history

Open intentional future
← Thesis selection
```

---

## Commitment Replacement

Commitments are immutable Assertions.

Replacing an open Commitment does not modify it.

A fork selects a newly constructed Commitment instead.

```text
Parent Thesis:
    C1, C2, C3

Child Thesis:
    C1, C2', C3
```

`C2'` is a new Commitment.

`C2` remains unchanged and continues to belong to every Thesis that selects it.

No mutable or hypothetical Commitment type is required.

---

## Complete Commitment Graph

A Thesis denotes a complete selected Commitment graph.

It is not semantically a delta.

```text
resolve(Thesis) → Complete Commitment Graph
```

An implementation may represent the graph using:

- complete materialization;
- structural sharing;
- parent references;
- additions and removals;
- persistent collections;
- hierarchical content-addressed structures;
- cached resolution;
- periodic materialization.

These strategies do not change Thesis semantics.

The logical model is complete regardless of its physical representation.

---

## Current Materialized Representation

The current implementation stores the complete Commitment selection as materialized sets of identifiers.

Conceptually:

```text
Selection
├── Frozen Commitment IDs
└── Open Commitment IDs
```

This representation intentionally favors:

- local validation of every Thesis;
- simple membership;
- direct deterministic hashing;
- transparent debugging;
- independence from parent correctness during assembly.

Its costs are also explicit.

For a selection of size `n`, each derivation may require:

- rebuilding or cloning `O(n)` identifiers;
- reading every selected Commitment through the canonical port;
- validating the complete resolved selection;
- hashing the complete selection.

With an in-memory adapter, these costs may be negligible.

With durable storage, canonical reads are expected to dominate the cost more than copying identifiers.

The current representation is a first implementation choice, not a semantic requirement.

---

## Structural Sharing

Structural sharing remains a valid future implementation strategy.

A Thesis may eventually use a persistent, content-addressed selection structure in which unchanged regions are shared.

```text
Selection root
├── changed path
└── shared immutable subtrees
```

A simple parent-plus-delta representation would reduce storage duplication but would not necessarily reduce derivation cost.

If each Thesis must still:

- resolve the complete selection;
- validate every Commitment;
- hash the full flat set;

then CPU and canonical reads remain `O(n)`.

Reducing both storage and derivation cost requires a hierarchical hash structure, such as a persistent Merkle tree, where unchanged subtrees contribute their existing hashes.

That evolution would also require an explicit decision about validation strategy.

```text
complete validation
→ preserves local proof
→ remains O(n)

incremental validation by induction
→ validates only changed regions
→ relies on parent validity
```

The current implementation deliberately preserves complete local validation.

Structural sharing is permitted by the semantics but is not implemented in version `0.1`.

Frequent Thesis creation in the current implementation may therefore duplicate the materialized selection proportionally.

---

## Immutability

A Thesis is immutable.

Once created, the following never change:

- its parent Thesis;
- its Knowledge Cut;
- its frozen selection;
- its open selection;
- its identity.

Intentional revision produces a fork.

```text
T1@K ──fork──▶ T2@K
```

Recognition of later knowledge produces advancement.

```text
T2@K ──advance──▶ T3@K'
```

Applications may present these operations as an editable experience.

The underlying model remains append-only.

Mutable references may move between Theses.

A Thesis itself never moves.

---

## Ancestry

Every non-genesis Thesis identifies its parent.

Ancestry records two independent forms of evolution.

```text
T0@K0
 └── T1@K0    intentional fork
      └── T2@K1    knowledge advancement
           ├── T3@K1    intentional fork
           └── T4@K1    alternative fork
```

### Intentional Fork

```text
Cut(Parent) = Cut(Child)
```

The selected Commitment graph changes under the same known world.

### Knowledge Advancement

```text
Cut(Parent) < Cut(Child)
```

The child recognizes a later historical cut while preserving the parent intention.

A child may contain Commitments absent from its parent for two reasons:

- they were introduced intentionally by a fork;
- they became historically required by advancement.

These causes must not be conflated.

---

## The Time Machine of Intention

A later Thesis is never interpreted under an earlier Knowledge Cut.

Doing so could expose Commitments not yet knowable at that cut.

Retrospective questions are answered through Thesis ancestry.

```text
T0@K0
 └── T1@K1
      └── T2@K2
           └── T3@K3
```

To answer:

> What was intended under `K1`?

The application uses:

```text
T1@K1
```

It does not reinterpret:

```text
T3@K3 under K1
```

The Event chain preserves the history of observed facts.

The Thesis chain preserves the history of intended worlds.

> _The Thesis chain is the time machine of intention._

Together they reconstruct:

- what could be known;
- what had been observed;
- what was intended;
- what operational consequences followed.

---

## Main as Convention

APE defines no privileged kind of Thesis.

A main line is a mutable application reference.

```text
main ──▶ Thesis
```

Other references may identify other Theses.

```text
main          ──▶ T4
factory-plan  ──▶ T7
alternative   ──▶ T9
```

References may move.

Theses remain immutable.

Canonical knowledge may advance while a reference continues to point to an older Thesis.

```text
canonical cut: K ──▶ K'
main:          T@K
```

The reference remains legitimate but historically behind.

The application may later perform:

```text
advance(T@K, K') → T'@K'

main ──▶ T'
```

Advancement cadence belongs to application policy.

---

## Historical Alignment

A Thesis may recognize a Knowledge Cut older than the current canonical cut.

```text
main ──▶ T@K

current canonical cut = K'
K < K'
```

This does not invalidate the Thesis.

It means the reference is historically behind.

```text
Aligned:
    Thesis Cut = Current Canonical Cut

Behind:
    Thesis Cut < Current Canonical Cut
```

Historical alignment is derived, never stored as mutable Thesis state.

A behind Thesis remains interpretable under its own cut.

It must be advanced before it can represent the later body of knowledge.

---

## Relationship with the Axiom

Every Commitment selected by a Thesis is an ordinary Commitment.

There is no hypothetical Commitment type.

New Commitments are constructed through the Axiom and must satisfy the same structural invariants as every other Commitment.

The Axiom determines whether a Commitment may exist.

The Thesis determines whether and where that Commitment participates in an intended world.

```text
Application
     │
     ▼
   Axiom
     │
     ▼
Structurally valid Commitment
     │
     ▼
   Canon
     │
     ▼
Canonical Commitment
     │
     ▼
   Thesis
```

A Commitment does not change nature when selected by different Theses.

Only membership changes.

---

## Relationship with the Canon

The Canon preserves admitted assertions and their canonical metadata.

It provides read-only access to the two Assertion families required by Thesis:

```text
Canonical knowledge
├── canonical Commitment records
└── canonical Event records
```

The Thesis layer depends only on this read-only port.

It does not require:

- admission operations;
- compare-and-set;
- mutable history access;
- unrelated ontological families.

The Canon determines:

- which Commitments were canonically admitted;
- when they were recorded;
- which Events were observed;
- when they were recorded;
- the order of the Event chain;
- the Event Head as of a recording instant.

A Thesis determines:

- which available open Commitments compose one intended continuation;
- which complete Commitment graph constitutes its world;
- which canonical Knowledge Cut it recognizes.

Canonical admission and Thesis membership remain distinct properties.

A Commitment may be canonical without belonging to every Thesis.

Once its settlement enters a Thesis’s recognized Event chain, it becomes part of that Thesis’s frozen causal past.

---

## Relationship with Hermeneia

A Thesis does not store operational state.

It supplies Hermeneia with:

- its complete selected Commitment graph;
- the exact Event chain recognized by its Knowledge Cut.

```text
Interpretation =
    Thesis selection
    + Thesis recognized Event chain
    + Effective Time
```

The public interpretation boundary derives both selection and chain from the same Thesis.

The application does not freely combine:

```text
selection from Thesis A
+
Event chain from cut B
```

This makes the exact-cut boundary enforceable by construction.

The same Thesis may still produce different projections at different effective times.

```text
project(T@K, τ1) ≠ project(T@K, τ2)
```

Its epistemic and factual cut remains fixed.

Only operational time changes.

A Thesis may be historically closed and structurally valid while Hermeneia projects it as infeasible.

Thesis preserves an interpretable proposition.

Hermeneia derives its consequences.

---

## Interpretation Boundary

A Thesis is interpreted only under its recognized Knowledge Cut.

```text
interpret(T@K)
```

The interpretation layer obtains:

```text
selection = resolve(T)

event chain = chain through K.event_head
```

No external Event Head is accepted.

This prevents:

- interpreting a Thesis against later Events;
- interpreting it against an earlier factual chain;
- pairing its selection with another historical reach;
- filtering factual history to fit the selected graph.

The selection and factual chain form one inseparable world.

---

## Future Combination Boundary

Operations transferring intentional changes between Theses at different cuts must preserve the same boundary.

A source Thesis is never directly reinterpreted under the target cut.

Instead, the operation constructs a candidate Thesis historically closed at the target cut.

```text
source: Tsource@K
target: Ttarget@K'

candidate: Tc@K'
```

Hermeneia evaluates the candidate.

```text
interpret(Tc@K')
```

The source remains anchored to its own cut.

Detailed comparison and combination semantics belong to another layer.

---

## Identity

A Thesis identity derives only from its immutable meaning.

It includes:

- its parent Thesis, when present;
- its recognized `known_at`;
- its recognized Event Head;
- its complete selected Commitment graph or an equivalent deterministic root;
- the distinction between frozen and open membership.

Conceptually:

```text
Identity(T) =
    hash(
        parent,
        known_at,
        event_head,
        frozen,
        open
    )
```

Therefore:

```text
same selection
same Event Head
different known_at
→ different Thesis
```

The Theses represent the same intention under different knowledge cuts.

That epistemic distinction is part of their meaning.

Mutable references and human-readable names do not belong to identity.

---

## Responsibilities

A Thesis is responsible for:

- identifying its parent, when present;
- recognizing exactly one historical Knowledge Cut;
- selecting only Commitments available at that cut;
- preserving the frozen causal past required by the cut;
- defining a complete selected Commitment graph;
- permitting intentional divergence only over the open future;
- advancing knowledge without inventing intention;
- preserving immutable ancestry;
- exposing Commitments imposed by historical advancement;
- remaining interpretable only under its own cut;
- providing a stable input for Hermeneia and future combination operations.

---

## Non-Responsibilities

A Thesis is **not** responsible for:

- constructing Commitment structure;
- admitting assertions into canonical history;
- creating hypothetical Events;
- modifying canonical history;
- proving when an application physically created a cut;
- enforcing the current canonical recording horizon;
- automatically following new canonical knowledge;
- defining application advancement cadence;
- storing projected operational state;
- determining feasibility;
- interpreting why a Commitment had previously been omitted;
- repairing contradictions imposed by facts;
- defining mutable branch references;
- deciding which Thesis is main;
- defining combination policy;
- resolving combination conflicts;
- requiring a particular storage representation;
- requiring structural sharing.

---

## Example: Fork under the Same Cut

Consider:

```text
T@K = {
    frozen: { C1 }
    open:   { C2, C3 }
}
```

Two forks may be created:

```text
Tx@K = {
    frozen: { C1 }
    open:   { C2, Cx }
}

Ty@K = {
    frozen: { C1 }
    open:   { C3, Cy }
}
```

Both recognize exactly the same body of knowledge.

They differ only in intended continuation.

```text
        ┌──▶ Tx@K
T@K ────┤
        └──▶ Ty@K
```

A Commitment recorded after `K.known_at` cannot be introduced by either fork.

---

## Example: Advancement without New Events

Assume:

```text
K1 = {
    known_at: D1
    event_head: E10
}

K2 = {
    known_at: D2
    event_head: E10
}
```

New Commitments were admitted between `D1` and `D2`, but no Event was observed.

```text
advance(T@K1, K2) → T'@K2
```

The result preserves:

- the same frozen causal past;
- the same open selection;
- the same Event Head.

It receives a different identity because the Knowledge Cut changed.

Newly admitted Commitments are not selected automatically.

They become available for a subsequent fork.

```text
fork(T'@K2, +Cnew) → T''@K2
```

---

## Example: Fine-Grained Advancement within One Day

Assume:

```text
E1 ──▶ E2

recorded_at(E1) = D3
recorded_at(E2) = D3
```

Two valid cuts exist:

```text
K1 = (D3, E1)
K2 = (D3, E2)
```

Then:

```text
advance(T@K1, K2)
```

is valid.

The recording date did not change, but the recognized factual prefix advanced.

Without this operation, a Thesis created at a finer cut could never reach the complete chain of that instant through ancestry.

---

## Example: Invalid Detached Fine Cut

Assume:

```text
canonical chain:
E1 ──▶ E2

detached Event:
Dx
```

and:

```text
recorded_at(E2) = recorded_at(Dx)
```

`Dx` is not a valid finer cut merely because it is contemporary.

```text
within(D, Dx) → invalid
```

It does not lie on the canonical chain ending at the head addressed by `D`.

The incoherent cut is rejected during construction.

---

## Example: Historical Imposition

Assume:

```text
T@K = {
    C1,
    C2'
}
```

Later canonical history fulfills `C2`.

Advancement produces:

```text
advance(T@K, K') →
    Advancement {
        thesis: T'@K',
        imposed: { C2 }
    }
```

The resulting Thesis contains:

```text
C1
C2 fulfilled
C2' open
```

The Thesis layer does not declare a conflict or remove `C2'`.

It exposes that canonical history imposed `C2`.

---

## Example: Retrospective Reconstruction

Consider:

```text
T0@K0
 └── T1@K1
      └── T2@K2
           └── T3@K3
```

To answer:

> What was intended under `K1`?

Use:

```text
interpret(T1@K1)
```

Do not reinterpret:

```text
T3@K3 under K1
```

`T3` may contain Commitments unavailable at `K1`.

The correct historical world already exists in the Thesis ancestry.

---

## Summary

A Thesis represents one complete and historically closed continuation of intended operational evolution.

It selects a complete Commitment graph.

It recognizes one real canonical Knowledge Cut composed of:

- the recording instant through which Commitments were available;
- the canonical Event prefix known at that cut.

It may be forked into alternative intentions under exactly the same knowledge.

It may be advanced to a later cut when neither coordinate regresses and at least one advances.

It preserves every settled Commitment and every causal dependency required to interpret observed history.

It reports Commitments imposed by advancement without interpreting or repairing intention.

It may be interpreted only under its exact Knowledge Cut.

Earlier intended reality is reconstructed through Thesis ancestry, never by projecting a later Thesis against older knowledge.

Its current implementation materializes the complete selection and validates it uniformly.

Alternative persistent or structurally shared representations remain compatible with its semantics.

```text
Events preserve the history of observed reality.

Knowledge Cuts preserve what could be known.

Theses preserve the history of intended reality.
```
