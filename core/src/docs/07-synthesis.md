# Synthesis

## Introduction

A Thesis defines one complete and historically closed continuation of intended operational evolution.

Different Theses may preserve the same known reality while selecting different Commitments for their open futures.

As those selections evolve independently, applications eventually need to determine whether the membership changes expressed by one Thesis may be applied to another.

> _**Synthesis** determines whether an intentional difference may be applied to a Target Thesis without violating an invariant known to APE._

Its responsibility is not to determine what any Agent meant by changing a selection.

Its responsibility is not to decide whether the resulting plan is desirable.

Its responsibility is to derive a Source difference, evaluate its application to a Target selection and report every known invariant that prevents that application.

The result of Synthesis is an immutable **Applicability Report**.

---

## Purpose

Synthesis provides a deterministic analysis of intentional transfer.

It allows applications to:

- derive the membership difference introduced between a Base and Source Thesis;
- distinguish intentional membership changes from Commitments imposed by historical advancement;
- evaluate those changes against a Target Thesis;
- resolve which changes would have an effective impact on the Target;
- verify that the resulting candidate preserves the invariants known to APE;
- describe every objective conflict preventing the transfer;
- prepare the construction of another Thesis without constructing it itself.

Synthesis evaluates whether a transfer may be applied.

It does not evaluate whether the transfer should be applied.

---

## Scope of Judgment

APE represents operational intentions through Commitments selected by Theses.

It can determine:

- whether a Commitment belongs to a selection;
- whether it was available at a Knowledge Cut;
- whether it belongs to the frozen causal past;
- whether its dependencies remain present;
- whether the resulting selection is historically closed;
- whether a candidate can be interpreted under the Target cut.

APE cannot determine:

- why an Agent omitted a Commitment;
- whether one Commitment was intended to replace another;
- whether two independently introduced Commitments are competing alternatives;
- whether their coexistence preserves a planner’s purpose;
- which possible plan is preferable;
- which plan an Agent would endorse.

These judgments require knowledge that is not contained in the operational graph.

> _Synthesis evaluates membership changes. It does not judge the intention those changes express._

The absence of an Applicability Conflict is therefore not an endorsement of the candidate.

It means only that no invariant evaluated by Synthesis prevents its construction.

---

## Three-Way Analysis

Synthesis is fundamentally a three-way analysis.

```text
Base Thesis
Source Thesis
Target Thesis
```

The Base and Source determine the membership difference being transferred.

The Target provides the selected world against which that difference is evaluated.

```text
Δsource = intentional_difference(Base, Source)

applicability =
    evaluate_application(Δsource, Target)
```

A direct comparison between Source and Target is insufficient.

Without a Base, the analysis cannot distinguish:

- Commitments inherited by Source;
- Commitments omitted by Source;
- Commitments introduced by Source;
- Commitments already absent from the common context;
- differences caused by historical advancement;
- differences that preceded the Source evolution being transferred.

The Base establishes the intentional context from which the Source difference is derived.

---

## Base Thesis

The Base Thesis defines the selection against which the Source difference is measured.

Conceptually:

```text
Base
├── membership preserved by Source
├── membership omitted by Source
└── membership introduced by Source
```

The Base must be a common ancestor of the Source and the Target.

```text
coherent_base(Base, Source, Target) iff

Base ∈ ancestors(Source)

and

Base ∈ ancestors(Target)
```

A Base cannot be invented by comparing unrelated Commitment sets.

Two selections may happen to contain similar Commitments without sharing an intentional lineage.

Ancestry to the Source is what earns the right to read an absence as a decision. Without it, `omitted` means only *not present*, and Synthesis would attribute an intention to a Thesis that never chose anything.

Ancestry to the Target is what earns the right to apply that decision. An omission is measured over `Open(Base)`, and applying it to a Target that never passed through the Base would remove a Commitment the Target selected on its own. Membership alone cannot distinguish a Commitment held by inheritance from one held by independent decision, and unlike a textual patch — which carries its context and fails loudly where that context is absent — a membership removal would apply silently.

Transferring Commitments between unrelated lineages remains available, and it is a fork: the Thesis layer introduces them over the Target directly. What such an operation cannot claim is that its omissions carry intention, which is the whole of what Synthesis adds.

Synthesis does not compute the Base. The declared Base need not be the nearest common ancestor — choosing an earlier one transfers a longer evolution, and that choice belongs to the caller.

---

## Source Thesis

The Source Thesis contains the membership evolution to be transferred.

Synthesis does not transfer the complete Source selection.

It derives the difference between Base and Source.

```text
Source Difference
├── omitted membership
└── introduced membership
```

Commitments remain immutable.

Synthesis does not describe an omission and an introduction as a replacement.

For example:

```text
Base:
    { C1 }

Source:
    { C2 }
```

Synthesis knows only:

```text
omit C1
introduce C2
```

It does not infer:

```text
C2 replaces C1
```

Such a relationship may be represented by application metadata or understood by the responsible Agent.

It does not belong to Synthesis semantics.

---

## Target Thesis

The Target Thesis defines the selected world onto which the Source difference is evaluated.

The Target may have evolved independently from the Base.

It may:

- preserve Commitments omitted by Source;
- omit Commitments preserved by Source;
- introduce additional Commitments;
- already contain Commitments introduced by Source;
- already omit Commitments omitted by Source;
- recognize a different Knowledge Cut;
- contain Commitments imposed by later canonical history.

Synthesis does not overwrite the Target with the Source.

It applies the Source membership difference over the Target selection and verifies whether the resulting candidate preserves known invariants.

---

## Intentional Difference

The Source difference contains only membership evolution attributable to the path from Base to Source.

Conceptually:

```text
IntentionalDifference {
    omitted
    introduced
}
```

A Commitment selected by both Base and Source requires no transfer.

```text
preserved =
    Commitments(Base)
    ∩ Commitments(Source)
```

An omission is an open Commitment selected by Base and absent from Source.

```text
omitted =
    Open(Base)
    − Commitments(Source)
```

An introduction is an open Commitment selected by Source and absent from Base.

```text
introduced =
    Open(Source)
    − Commitments(Base)
```

These are membership relations.

They carry no implicit correspondence between omitted and introduced Commitments.

```text
omitted { C1 }
introduced { C2 }
```

does not mean:

```text
C2 supersedes C1
```

It means only that Source no longer selects `C1` and now selects `C2`.

---

## Historical Difference Is Not Intentional Difference

Theses may recognize different Knowledge Cuts.

When canonical history advances, a child Thesis may acquire Commitments that were absent from its parent because the later Event chain makes them historically unavoidable.

These Commitments are imposed by history.

They are not intentional introductions.

```text
Source Selection
=
Source Open Selection
∪ Source Frozen Causal Past
```

Only changes to the open intentional selection participate in the transferable Source difference.

A Commitment introduced into the Source selection solely because it entered the frozen causal past is not a Source decision.

Likewise, a Commitment that became frozen cannot be interpreted as intentionally removable.

Synthesis must therefore distinguish:

```text
Membership change caused by fork
→ intentional difference

Membership change caused by advancement
→ historical imposition
```

Historical facts cannot be transferred as though they were planning decisions.

---

## Independent Source and Target Cuts

Source and Target remain anchored to their own Knowledge Cuts.

```text
Source: Tsource@Ks
Target: Ttarget@Kt
```

Synthesis never reinterprets Source under `Kt`.

It never reinterprets Target under `Ks`.

The Source difference is derived from the Source lineage under its own historical context.

Its attempted application is then evaluated within the Target historical context.

```text
derive difference under Source ancestry
                │
                ▼
evaluate transfer under Target cut
```

Each Thesis remains inseparable from the Knowledge Cut it recognizes.

---

## Target Historical Context

Any candidate derived from applying the Source difference belongs to the Target historical context.

```text
Source: Tsource@Ks
Target: Ttarget@Kt

Candidate context: Kt
```

The Target Knowledge Cut determines:

- the Event chain that must be recognized;
- the frozen causal past that must be preserved;
- the Commitments historically available for selection;
- the exact epistemic context in which the candidate may be interpreted.

A Source Commitment unavailable at the Target cut cannot enter the candidate selection.

It remains part of the requested Source difference and is reported as an Applicability Conflict.

```text
Source difference:
    introduce Cnew

recorded_at(Cnew) > Kt.known_at

→ HistoricalUnavailability(Cnew)
```

The report preserves the attempted introduction so that an application or Agent may later:

- abandon it;
- advance the Target to another Knowledge Cut;
- construct a different Commitment;
- create another Thesis under a context in which the Commitment is available.

Synthesis reports the failed transfer.

It does not prescribe the later response.

---

## Candidate Selection

To evaluate applicability, Synthesis derives an ephemeral candidate selection.

```text
CandidateSelection =
    apply(SourceDifference, TargetSelection)
```

The candidate is the mechanical result of applying effective Source membership changes to the Target.

Conceptually:

```text
CandidateSelection =
    TargetSelection
    − EffectiveOmissions
    ∪ EffectiveIntroductions
```

The candidate inherits the Target partition.

```text
Frozen(Candidate) = Frozen(Target)

Open(Candidate) =
    Open(Target)
    − EffectiveOmissions
    ∪ EffectiveIntroductions
```

This holds by construction rather than by verification. `Frozen(Kt)` is contained in the Target because a Thesis is historically closed; no effective omission may touch it, since removing a frozen Commitment is refused as a conflict; and an effective introduction is by definition absent from the Target, so it cannot be frozen there and enters the open selection.

Historical closure of the candidate therefore needs no separate check. A Commitment already frozen in the Target is never an introduction — it is already selected, which makes it idempotent.

The candidate is not a Thesis.

It has no:

- Thesis identity;
- parent;
- ancestry position;
- mutable reference;
- persistence requirement;
- independent canonical meaning.

Its purpose is analysis.

A real Thesis exists only if an application later requests its construction through the Thesis layer.

---

## Resolved Transfer

The Source difference is expressed relative to the Base.

Before evaluation, it is resolved against the Target.

Conceptually:

```text
ResolvedTransfer {
    remove
    introduce
}
```

The resolved transfer contains only effective membership changes.

If Source omits a Commitment already absent from Target, no removal is required.

```text
Source omits C
Target already omits C

→ no effective removal
```

If Source introduces a Commitment already selected by Target, no introduction is required.

```text
Source introduces C
Target already selects C

→ no effective introduction
```

These cases are idempotent.

Neither is a conflict on its own.

A transfer whose changes are *all* idempotent is another matter: it resolves to nothing, and no operation remains to be evaluated against the Target. That is not a conflict either — it is the `AlreadyApplied` status.

The resolved transfer therefore represents the exact operation evaluated against the Target selection.

---

## Applicability

Applicability asks:

> Can the membership difference derived from Source be applied to Target without violating an invariant known to APE?

Applicability presupposes a coherent Base. Where there is none the question is not answered negatively — it is not asked, and no report is produced.

An applicable transfer must:

- preserve the Target Knowledge Cut;
- preserve the Target frozen causal past;
- introduce only Commitments available at the Target cut;
- preserve complete dependency closure;
- resolve to at least one effective membership change;
- preserve every structural invariant required by Thesis;
- contain no Applicability Conflict.

Applicability does not ask:

- whether the candidate is desirable;
- whether its Commitments express a coherent strategy;
- whether independently introduced Commitments were intended as alternatives;
- whether the responsible Agents approve the result;
- whether the candidate is operationally feasible;
- whether an application should construct it.

```text
applicable
≠ feasible
≠ desired
≠ approved
≠ recommended
≠ constructed
```

Applicable means only:

> No invariant evaluated by Synthesis prevents the resolved transfer from being applied.

---

## Refusal Is Not Conflict

Some requests produce no analysis at all, and they are refused rather than reported.

```text
Refused
→ no report exists

Conflicted
→ a report exists, and it names what prevents the transfer
```

An **incoherent Base** is the case that matters, because it is the one that could be mistaken for a conflict. Every conflict names an invariant the resulting world would break; an incoherent Base breaks none. It means there is no intentional difference to speak of: without ancestry to the Source an absence is not a decision, and without ancestry to the Target a decision is not that world's to receive.

Deriving a difference over such a Base is arithmetic without meaning, and everything downstream inherits it — the conflicts a candidate would then show are noise attributed to a transfer nobody could have intended. So the Base is established first, and the operation ends there.

Theses the archive cannot resolve are refused for the same reason: a Thesis it does not hold has no lineage to establish, and a Commitment canonical history does not hold cannot have its dependencies read.

---

## Applicability Conflict

An Applicability Conflict exists only when applying the Source difference would violate an invariant known to APE.

```text
ApplicabilityConflict
→ evidence of a known invariant violation
```

A difference of opinion, preference or planning intent is not an Applicability Conflict unless the operational model expresses it through an invariant that APE can evaluate.

For example:

```text
Source introduces C2
Target already contains C3
```

is not a conflict merely because a planner may regard `C2` and `C3` as alternatives.

If both may coexist in a valid selected graph, the transfer is applicable.

Whether they should coexist belongs to the planner.

> _APE can determine whether Commitments may coexist in a valid selected graph. It cannot determine whether their coexistence represents what any Agent intended._

---

## Applicability Report

Synthesis produces an immutable **Applicability Report**.

Conceptually:

```text
ApplicabilityReport {
    base
    source
    target
    source_difference
    status
}
```

The status carries the conclusion, and each conclusion carries only what it is made of:

```text
Applicable { resolved_transfer, candidate_selection }
→ at least one effective membership change, and no conflict

AlreadyApplied
→ the difference is valid and entirely idempotent

Conflicted { attempted_transfer, conflicts }
→ a known invariant prevents the transfer
```

Reading the conclusion from the shape rather than from an empty collection keeps two statements true at once: **a conflict always names a violated invariant**, and **an applicable result always carries an effective change**. Neither a conflicted report without conflicts nor an applicable one with them can be represented.

`AlreadyApplied` carries no transfer, because nothing remains to apply, and no candidate, because the candidate would be the Target. It is not a failure and not a violation: the Target already contains the difference. Whether anything should be constructed afterwards is not Synthesis's concern — nor is the fact that deriving a Thesis from an unchanged selection is refused by the Thesis layer, which is that layer's invariant over an operation Synthesis never performs.

`Conflicted` carries no candidate either. Where an invariant prevented the transfer, no candidate was validly derived.

The report records the complete context and analysis from which its conclusion was derived.

The candidate may be represented directly, by a deterministic identity or by another equivalent description. The representation is an implementation choice, and its meaning is the exact candidate selection evaluated by the report.

### Reproducibility

Synthesis is deterministic.

```text
synthesize(Base, Source, Target)
    → ApplicabilityReport
```

Given the same three Theses, it produces the same source difference and the same status with the same content.

A report is not an entity. Nothing refers to it, it takes part in no graph, and Synthesis does not persist it. Like the reports Hermeneia derives, it carries the coordinates that produced it and is obtained again by asking again — it needs no identity of its own.

That reproduction holds permanently, and the reason is the guarantee the Canon provides. Every question the report asks — availability at `Kt`, frozen membership, dependency resolution — is answered below cuts that are fixed, on Theses that are immutable. Because recording is monotonic across admission, nothing may later appear below those cuts. The three Theses therefore determine the report for good, and without that guarantee two reports over the same three could disagree over time.

---

## Applicable Report

A report is applicable when its status is `Applicable`: an effective transfer, and no invariant against it.

```text
applicable(report) iff

report.status = Applicable { .. }
```

An applicable report means that its resolved transfer may be used as input to the construction of another Thesis derived from the Target.

```text
Target
+
report.resolved_transfer
        │
        ▼
Thesis construction
```

The report does not prove that:

- the Thesis was constructed;
- the candidate was approved;
- the candidate is feasible;
- the candidate expresses the desired intention;
- any mutable reference was moved.

It proves only that the evaluated transfer violates no invariant checked by Synthesis.

---

## Conflicted Report

A report is conflicted when one or more known invariants prevent the Source difference from being applied.

```text
conflicted(report) iff

report.status = Conflicted { .. }
```

A conflicted report does not produce a valid partial transfer.

It preserves:

- the requested Source difference;
- the transfer that was attempted;
- the exact invariants that prevented application;
- the Commitments or historical structures involved.

The application or responsible Agent may use this information when deciding what to do next.

Synthesis does not determine that response.

---

## Applicability Conflicts

The exact public vocabulary may evolve, but every conflict must identify an objective invariant violation.

### Historical Freezing

The Source difference attempts to remove a Commitment belonging to the Target frozen causal past.

```text
C ∈ Frozen(Kt)

SourceDifference.omitted contains C
```

The Target Event history has made `C` historically unavoidable.

Removing it would rewrite the causal meaning of observed facts.

```text
HistoricalFreezing {
    commitment: C
}
```

### Historical Unavailability

The Source difference introduces a Commitment that was not available at the Target Knowledge Cut.

```text
recorded_at(C) > Kt.known_at
```

The Commitment may exist canonically now, but it was not knowledge in the world recognized by Target.

```text
HistoricalUnavailability {
    commitment: C
    target_cut: Kt
}
```

The introduction remains visible in the report.

It simply cannot enter this candidate.

### Dependency Breakage

Applying an omission would leave a selected Commitment without one of its required dependencies.

```text
Dependent selected
Dependency removed
```

A Thesis denotes a complete Commitment graph.

It cannot retain a Commitment while removing an identity required by its structure.

```text
DependencyBreakage {
    dependent
    missing_dependency
}
```

### Missing Dependency

A Source introduction may depend on Commitments absent from the Target and not introduced by the resolved transfer.

```text
introduced C
dependency D absent from candidate
```

The candidate would not contain the complete graph required by `C`.

```text
MissingDependency {
    commitment: C
    dependency: D
}
```

Both conflicts report the same broken invariant — dependency closure — and they are told apart by the origin of the absence rather than by precedence:

```text
dependency absent from the candidate, and present in the Target
→ DependencyBreakage

dependency absent from the candidate, and never present
→ MissingDependency
```

An introduction whose dependency the same transfer removes is therefore a breakage: something that existed was broken. The classification is decided by the candidate, not by the order in which the conflicts were found.

### No Generic Category

There is deliberately no catch-all conflict for a candidate that fails some other invariant.

Every invariant Synthesis presses today is named specifically, so a generic one would have no producer — an unconstructible case in the vocabulary, which is the same defect as a check that can never fail. It would also invite the opposite problem in time: a residual category is where poorly modelled invariants accumulate, each one reported without the planner learning which rule was broken.

The taxonomy is expected to grow. A new invariant arrives with the name that describes it, alongside the check that finds it.

---

## What Is Not a Conflict

Synthesis does not report a conflict merely because Source and Target changed the same Base membership.

Consider:

```text
Base:
    { C1 }

Source:
    { C2 }

Target:
    { C3 }
```

The Source difference is:

```text
omit C1
introduce C2
```

The Target already omits `C1`.

Applying the Source difference produces:

```text
Candidate:
    { C2, C3 }
```

If:

- `C2` is available at the Target cut;
- `C2` and `C3` retain complete dependency closure;
- no frozen Commitment is removed;
- the candidate is historically closed;
- no other known invariant is violated;

then the transfer is applicable.

Synthesis does not infer that:

- `C2` replaces `C1`;
- `C3` replaces `C1`;
- `C2` and `C3` are alternatives;
- selecting both is undesirable;
- either lineage should prevail.

Those conclusions are not present in the graph.

The planner who created or understands the operational intentions remains responsible for judging them.

---

## Application-Level Semantics

Applications may maintain additional semantics outside APE.

For example, an application may record:

```text
C2 supersedes C1
```

or:

```text
C2 and C3 belong to the same planning choice
```

Such information may support:

- user interfaces;
- approval workflows;
- revision histories;
- planning guidance;
- warnings;
- automatic resolution policies;
- domain-specific comparisons.

These semantics do not change Synthesis.

From the perspective of APE, Theses continue to select immutable Commitments.

The application may judge those selections using richer domain knowledge.

APE does not require that knowledge to become part of its ontology.

---

## Applicability and Feasibility

Applicability and feasibility are orthogonal judgments.

```text
Applicability
→ Can this membership difference be applied without violating
  a known Synthesis invariant?

Feasibility
→ What operational future does the candidate graph permit
  under a named hypothesis?
```

A transfer may be applicable while producing an infeasible candidate.

```text
ApplicabilityReport:
    status = Applicable

FeasibilityReport:
    violated under hypothesis H
```

This is not contradictory.

Synthesis evaluates the construction of a selected world.

Hermeneia evaluates the operational consequences of that world.

Likewise, a transfer blocked by an Applicability Conflict might have described Commitments that were feasible in another historical context.

The conflict concerns the transfer into this Target, not the isolated operational effects of the Commitments.

---

## Relationship with Feasibility

Feasibility is central to operational coordination.

Applications will commonly want a feasibility verdict before deciding what to do with a report.

The candidate is not the object they evaluate. It exists for the applicability analysis, and it is deliberately not a world: a selection and a chain handed over as separate arguments can always be mismatched, and the answer would look ordinary. A Thesis is what pairs them inseparably, which is why interpretation is offered over a Thesis and never over a bare selection.

```text
ApplicabilityReport
        │
        ▼
Application constructs a Thesis from the Target
        │
        ▼
Hermeneia interprets that Thesis
        │
        ▼
FeasibilityReport
```

Constructing a Thesis obliges nothing further — no persistence, no mutable reference moved — so an application may construct, interpret and discard.

An application may also wrap the whole sequence in one operation of its own. The following is a sketch of such an application, not an operation APE provides:

```text
# application level

analyze(Base, Source, Target, hypotheses):
    report = synthesize(Base, Source, Target)
    if report is conflicted:
        return report

    candidate = thesis_layer.derive(Target, report.resolved_transfer)
    verdicts  = [hermeneia.feasibility(candidate, h) for h in hypotheses]

    return report, verdicts
```

Synthesis offers no such operation. Composing the two analyses is an application concern, and an operation returning both verdicts would place a feasibility judgment inside a layer whose entire product is the Applicability Report.

The Applicability Report remains an evaluation of membership transfer.

Each Feasibility Report remains an evaluation under an explicit hypothesis.

Application policy may require, for example:

```text
Applicability conflicts must be empty.

final_state must not be violated.

on_due_date_net violations require confirmation.
```

These are application decisions.

Synthesis itself does not impose them.

---

## Applicable but Infeasible

A candidate may preserve every Synthesis invariant and still describe an operationally infeasible future.

This does not make the transfer inapplicable.

It means only that Hermeneia found a feasibility conflict under the requested hypothesis.

There may be legitimate reasons to preserve or construct such a Thesis:

- representing a plan received from another Agent;
- documenting an existing planning contradiction;
- comparing defective alternatives;
- inspecting which Constraints are violated;
- reconstructing a historical decision;
- evaluating the same selection under other hypotheses.

A feasibility verdict is always labelled by its hypothesis.

Failure under one hypothesis does not necessarily prove impossibility under every realization.

> _An applicable transfer may produce an infeasible candidate. Application policy decides what follows._

---

## Constructing Another Thesis

Synthesis does not construct a Thesis.

When an Applicability Report is clean, the application may request that the Thesis layer create a child of the Target using the resolved transfer.

```text
Applicability Report
        │
        ▼
Application decision
        │
        ▼
Thesis construction
        │
        ▼
New Thesis
```

The Thesis layer remains responsible for:

- parent assignment;
- immutable identity;
- exact Knowledge Cut;
- frozen and open partitioning;
- complete selection;
- historical closure;
- ancestry;
- final validation.

Synthesis must not provide an alternative construction path.

Even an applicable report is evidence produced by analysis, not a substitute for Thesis validation.

---

## No Prescribed Result

Synthesis has no concept of a reconciled result.

After receiving a report, an application or Agent may:

- construct the mechanically derived candidate;
- ignore the report;
- advance the Target;
- introduce different Commitments;
- preserve the Target unchanged;
- create another Thesis unrelated to the candidate;
- request another Synthesis.

Any later Thesis is a separate operational decision.

It is not the semantic product of Synthesis.

> _Synthesis does not prescribe the world that should follow from its report._

The only product of Synthesis is the Applicability Report.

---

## Fast-Forward

Some transfers contain no effective Target divergence from the Base membership relevant to the Source difference.

```text
Target membership = Base membership

apply difference(Base, Source) to Target
→ Source-equivalent selection
```

This is conceptually similar to a fast-forward.

However, Source and a later constructed Thesis may still differ in:

- Knowledge Cut;
- parent;
- ancestry;
- frozen membership;
- identity.

```text
same open selection
different Knowledge Cut
→ different Thesis
```

Fast-forward describes a simple transfer.

It does not collapse Thesis identity or historical meaning.

---

## Immutability

An Applicability Report is immutable.

Its conclusion remains attributable to:

- the exact Base Thesis;
- the exact Source Thesis;
- the exact Target Thesis;
- the canonical knowledge required to resolve them.

If any input changes, another report must be produced.

```text
synthesize(B, S, T1) → R1

synthesize(B, S, T2) → R2
```

No report is updated in place.

A report derived from one Target cannot be treated as valid for another.

---

## Historical Validity

A report is valid only for the exact Theses from which it was derived.

An earlier report does not automatically apply after Target advancement.

```text
Target T@K
Target advanced to T'@K'

report over T@K
≠ report over T'@K'
```

The later Target may contain:

- newly frozen Commitments;
- new historical impositions;
- newly available Commitments;
- a different open selection.

The Source difference must be evaluated again.

Earlier reports remain historically reconstructible because their input Theses are immutable.

They do not become current reports for later worlds.

---

## Relationship with Thesis

Thesis defines immutable intended worlds.

Synthesis evaluates the transfer of membership changes between them.

```text
Thesis
→ complete historically closed selection

Synthesis
→ applicability analysis over Thesis differences
```

Synthesis depends on Thesis semantics for:

- ancestry;
- Knowledge Cuts;
- frozen causal past;
- open intentional future;
- historical advancement;
- complete Commitment selection.

It does not redefine them.

Ancestry is also a read dependency, not only a semantic one. Establishing that a Base is a common ancestor means walking from the Source and from the Target through their parents until it is reached or the lineage ends, which requires resolving a Thesis by identity.

```text
Synthesis reads:
├── canonical knowledge  → the Canon
└── Theses by identity   → the Thesis layer
```

Synthesis therefore cannot verify a Base against Theses an application holds only in memory or names only by convention. Where a Thesis cannot be resolved, its ancestry cannot be established, and the transfer is refused rather than assumed.

Source and Target remain valid independently of the Applicability Report.

A conflicted report invalidates neither Thesis.

---

## Relationship with Hermeneia

Hermeneia derives the operational consequences of a selected Commitment graph.

Synthesis derives an ephemeral candidate selection.

Hermeneia may evaluate that candidate when requested.

```text
Synthesis
→ derives candidate membership

Hermeneia
→ derives candidate consequences
```

Synthesis does not:

- calculate Resource levels;
- derive Commitment conditions;
- evaluate deadlines;
- define feasibility hypotheses;
- search possible realizations;
- produce feasibility verdicts.

Those responsibilities remain with Hermeneia.

---

## Relationship with the Canon

Synthesis reads canonical knowledge.

It does not admit assertions.

The Canon determines:

- which Commitments exist canonically;
- when they became available;
- which Events belong to the factual chain;
- the recording metadata required by Knowledge Cuts.

Synthesis uses this knowledge to verify:

- availability at the Target cut;
- frozen causal membership;
- dependency resolution;
- historical closure;
- stable identity.

It does not modify canonical history.

---

## Relationship with the Axiom

Synthesis does not construct Commitments.

If an application or Agent decides to express another intention after reading a report, any new Commitment is constructed through the Axiom and admitted through the Canon.

```text
Planner decision
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
Thesis construction
```

That later decision lies outside Synthesis.

Synthesis never reports that a particular new intention is required.

It reports only the invariants preventing the attempted transfer.

---

## Determinism

Synthesis must be deterministic.

Given the same Base, Source, Target and canonical knowledge, it must produce the same:

- Source difference;
- resolved transfer;
- candidate selection;
- Applicability Conflicts;
- Applicability Report.

```text
synthesize(B, S, T) = R
```

Conflict ordering must also be deterministic when exposed publicly.

The report must not depend on:

- hash-map iteration order;
- persistence retrieval order;
- mutable application references;
- the order in which equivalent inputs were presented.

Determinism makes reports reproducible, comparable and cacheable.

---

## Responsibilities

Synthesis is responsible for:

- establishing the Base before anything is derived, and refusing where there is none;
- deriving the Source difference from Base and Source;
- separating intentional membership changes from historical imposition;
- resolving effective changes against the Target;
- preserving the Target Knowledge Cut;
- preserving the Target frozen causal past;
- validating Commitment availability at the Target cut;
- preserving complete dependency closure;
- deriving an ephemeral candidate selection;
- identifying invariant violations caused by the transfer;
- returning an immutable deterministic Applicability Report.

---

## Non-Responsibilities

Synthesis is **not** responsible for:

- interpreting why an Agent changed a Thesis;
- determining whether one Commitment replaces another;
- inferring competing or complementary intentions;
- judging planner preference;
- deciding whether a candidate is desirable;
- constructing Commitments;
- admitting assertions;
- creating hypothetical Events;
- modifying any Thesis;
- constructing the candidate as a Thesis;
- moving mutable references such as `main`, which belongs to the application;
- evaluating feasibility directly;
- selecting feasibility hypotheses;
- rejecting infeasible knowledge;
- choosing application policy;
- resolving domain-specific conflicts;
- prescribing a later Thesis;
- orchestrating user interaction;
- persisting reports.

---

## Example: Clean Application

Assume:

```text
Base:
    open { C1, C2 }

Source:
    open { C1, C3 }

Target:
    open { C1, C2, C4 }
```

The Source difference is:

```text
omit C2
introduce C3
```

The resolved transfer is:

```text
remove C2
introduce C3
```

The candidate is:

```text
open { C1, C3, C4 }
```

Assuming:

- `C2` is not frozen;
- `C3` is available at the Target cut;
- all dependencies remain selected;
- the candidate is historically closed;

the report is applicable.

```text
ApplicabilityReport {
    status: Applicable { .. }
}
```

The report does not determine whether `{ C1, C3, C4 }` is desirable or feasible.

---

## Example: Transfer Already Applied

Assume:

```text
Base:
    open { C1 }

Source:
    open { C1, C2 }

Target:
    open { C1, C2, C3 }
```

The Source introduces `C2`.

The Target already selects `C2`.

```text
ResolvedTransfer {
    remove: ∅
    introduce: ∅
}
```

Nothing is left to apply.

```text
ApplicabilityReport {
    status: AlreadyApplied
}
```

No invariant was violated, so this is not a conflict; and no effective change remains, so it is not an applicable transfer either. It is the report of a world that already contains the difference, which is why applying the same difference twice is safe.

An idempotent *part* of a transfer is different: where other changes remain effective, the status is `Applicable` and the redundant ones simply resolve to nothing.

---

## Example: Independent Introductions

Assume:

```text
Base:
    open { C1 }

Source:
    open { C2 }

Target:
    open { C3 }
```

The Source difference is:

```text
omit C1
introduce C2
```

The Target already omits `C1`.

The resolved transfer is:

```text
introduce C2
```

The candidate is:

```text
open { C2, C3 }
```

Synthesis does not infer that `C2` and `C3` are competing revisions.

If the candidate preserves every known invariant, the report is applicable.

Whether both Commitments should remain selected belongs to the planner.

---

## Example: Historical Freezing

Assume Source omits `C1`.

Later, the Target recognizes an Event chain that makes `C1` part of its frozen causal past.

```text
C1 ∈ Frozen(Target Cut)
```

The omission cannot be applied.

```text
ApplicabilityConflict:
    HistoricalFreezing(C1)
```

The report preserves the attempted omission but does not produce a valid candidate without `C1`.

---

## Example: Historical Unavailability

Assume Source introduces `C2`.

```text
recorded_at(C2) > Target.known_at
```

`C2` was not available in the world recognized by Target.

```text
ApplicabilityConflict:
    HistoricalUnavailability(C2)
```

The attempted introduction remains visible in the report.

An application may later advance the Target or make another planning decision.

Synthesis prescribes neither.

---

## Example: Dependency Breakage

Assume:

```text
Target selects:
    C1
    C2 depends on C1
```

The Source difference omits `C1` but preserves no change removing `C2`.

Applying the difference would produce:

```text
Candidate:
    C2 selected
    C1 absent
```

The candidate does not contain a complete Commitment graph.

```text
ApplicabilityConflict:
    DependencyBreakage {
        dependent: C2
        missing_dependency: C1
    }
```

---

## Example: Applicable but Infeasible

Assume Source introduces `C2`.

The transfer preserves every Synthesis invariant.

```text
ApplicabilityReport:
    status = Applicable
```

Hermeneia then evaluates the candidate and finds that its combined Resource movements violate a Constraint under `final_state`.

```text
FeasibilityReport:
    final_state = violated
```

The transfer remains applicable.

The candidate is infeasible under that hypothesis.

Application policy decides whether another Thesis should be constructed.

---

## Principles

Synthesis follows a small number of principles.

- Intentional difference is derived through three-way analysis.
- A Base is a common ancestor, and lineage is what makes an absence a decision.
- Intentional difference is expressed as membership change.
- Historical difference is not automatically intentional difference.
- Omission and introduction do not imply replacement.
- Source and Target remain anchored to their own Knowledge Cuts.
- A candidate belongs to the Target historical context.
- A conflict exists only when a known invariant is violated.
- A missing precondition is refused, never reported as a conflict.
- Applicable, already applied and conflicted are three outcomes, not two.
- Applicability and feasibility are orthogonal, and composing them is the application's.
- Applicable does not mean desirable, approved or correct.
- A clean report does not judge planner intention.
- Synthesis reports what prevents transfer.
- Synthesis does not prescribe what should follow.
- The Applicability Report is the only product of Synthesis.
- Thesis construction remains the responsibility of Thesis.
- Events remain factual and cannot be synthesized.
- Canonical history is read, never rewritten.
- Deterministic reports make intentional transfer reproducible.

---

## Summary

A Thesis preserves one complete and historically closed intended world.

Synthesis derives the intentional membership difference between a Base and Source Thesis and evaluates its application to a Target Thesis.

It produces an immutable Applicability Report: the Source difference, and a status saying what became of it.

```text
Applicable      → an effective transfer, and the candidate it produces
AlreadyApplied  → the Target already contains the difference
Conflicted      → the invariants that prevent it
```

An applicable report means that no Synthesis invariant prevents the transfer.

It does not mean that the candidate is desirable, feasible, approved or faithful to an Agent’s purpose.

A conflicted report identifies objective reasons why the transfer cannot be applied within the Target historical context.

Synthesis does not infer replacement.

It does not judge intentions.

It does not create a Thesis.

It does not prescribe a later world.

```text
Thesis preserves an intended world.

Synthesis evaluates membership transfer.

Applicability Report identifies invariant violations.

Hermeneia derives operational consequences.

The planner judges intention.
```
