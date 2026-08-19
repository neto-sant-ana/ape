# Result

**Confirmed**, for the stated experimental boundary — and confirmed only after the form the
experiment began with was refuted.

An application reasoned about three worlds, terminated completely, and a fresh
operating-system process rebuilt all three from the repository alone. Each comes back with the
identity, the cut, the partition, the conditions, the level and the verdict of the world it
reproduces, including the genesis's refusal.

```text
                    reasoned                    rebuilt
genesis             53e2b385…                =  53e2b385…
  cut               2026-01-10, head none    =  same
  partition         frozen {}, open {A,B}    =  same
  feasibility       OutOfBounds −70          =  OutOfBounds −70
advancement         d8b9242b…                =  d8b9242b…
fork                37a5fbf0…                =  37a5fbf0…
  partition         frozen {B}, open {A,C}   =  same
```

The hypothesis holds as stated. What a decision must record is its **position in the sequence
of admissions**, and nothing further about a Thesis is kept.

## Against each criterion

1. **Every world in the lineage is reconstructed.** The binary prints the lineage, not its
   tip. Which world an application ended at turned out to be the easy half.
2. **Each reconstructed world has the identity of the world it reproduces.** Two of the three
   are reproduced in every other coordinate as well, and would have passed a comparison that
   walked coordinates alone. Identity is what caught them (Observation 3).
3. **Each cut addresses the same instant and the same head.** Including the genesis, whose
   instant addresses a different head once the cancellation is in the journal — which is the
   whole of what the repair had to fix.
4. **Each selection is partitioned identically.** The quieter half of the refutation, and the
   one the previous experiment's harness could not have caught; it does now.
5. **Conditions are equivalent at the same effective instant.** Every commitment of every
   world, at 2026-01-25 — past two deadlines and inside a third.
6. **The refused world is reproduced as refused**, by `OutOfBounds` naming the same instance
   at the same level. This is the criterion the previous experiment could only compare as two
   absences, and the reason this subject contests its bounds.
7. **The fork's introduced and omitted commitments are those the decision recorded.** Read
   back from the repository rather than inferred from two selections — though for this subject
   the two agree, so the distinction is exercised only in the record, not in a divergence.
8. **No derived value is persisted.** Phase 4 keeps the substring scan, pins the closed field
   set of every decision, and adds the completeness audit of Observation 4.
9. **No previously computed projection or identity is required.** The genesis's cut resolves
   an empty chain from a journal that contains the Event, which is only possible because the
   coordinate says how much of that journal to admit first.
10. **No persistence-specific concern entered the engine.** `core/` is untouched by this
    experiment. The one interface change is in the application: it stopped discarding
    identities the engine was already returning.

## How the result was reached

The naive form was implemented, refuted, and the refutation recorded before any repair
existed (Observations 1 to 3). Four candidate repairs were then implemented independently, in
isolated worktrees, each blind to the others and each held to this protocol:

```text
count of admissions        reproduces all three   dominated: an offset re-points in silence
reference to an entry      reproduces all three   kept
one interleaved sequence   reproduces all three   viable; ordered by a different criterion
ordering discipline        reproduces one         refuted (Observation 5)
```

The reference was kept over the interleaved sequence on a criterion outside this experiment:
a `ThesisArchive` resolves by identity, and a repository whose entries are addressable by
identity is the one that lets the next experiment ask what a decision still records once the
decided world is stored. Both forms are sound for the question asked here.

## What the result does not cover

**A coordinate that is well-formed and false is not detectable from the record** (Observation
6). Both surviving repairs reproduce the diverged world, silently, from a repository edited in
one place. The criteria above do not ask for this and it is not a failure against them; it is
the boundary of what "reconstructible" was shown to mean.

**The road not taken was not measured.** `ThesisArchive` exists for storing decided worlds and
would close the divergence by not re-deriving at all. This experiment declined it on a rule
published by another, which makes the comparison between the two an experiment rather than an
argument, and it has not been run.

**Three paths are unexercised by this subject**, each named where it lives: the trailing
admission after the last decision, both branches of a decision that follows an entry already
admitted, and `Advancement::imposed`, which this subject cannot make non-empty.

**Concurrency is untouched.** One writer, one process, one append order. Whether a position in
a sequence survives a repository written to by more than one process is the open question it
was.

**Abandoned siblings are still not modelled**, and nothing in this experiment needed them. A
lineage records the worlds that were kept; whether auditability depends on an application
forking rather than discarding remains unanswered.

---

# Architectural Consequences

## Established for the CLI

* **A persisted decision is a decision *taken*.** An intention plus the entry the journal
  had reached when it was taken. The two are one record, because a decision filed under a
  coordinate it did not have is a lineage that reads back as a different lineage.
* **The two sequences are read together, never one after the other.** `lineage::replay` is
  sound only where nothing was admitted within an instant a decision names, and says so; a
  repository is read through `lineage::rebuild`.
* **A journal entry is addressable by the identity it produced.** The application had been
  discarding those identities. This is what made a reference available as a coordinate, and
  it is the idiom a `ThesisArchive` already speaks.

## Pressure recorded, not acted on

* **"Nothing derived" audits presence, never absence** (Observation 2). One instrument for the
  inverse now exists — every address a decision names must be in the journal — and
  Observation 6 measures how far it does not reach.
* **The writer is trusted and unverified** (Observation 6). Closing it means a coordinate
  naming the *state* of knowledge rather than a position in it, which is a derived value and
  therefore another experiment's to permit.
* **Deciding about the present is what makes this hard** (Observation 5). An application that
  only reasons about closed days needs no coordinate and cannot select what was admitted
  today. The cost of reproducibility was paid in the record precisely so it would not be paid
  there.

## Method

* **A guard written before the run is the only one that survives a defect both sides share.**
  Three independent implementations reproduced this: negating the reported level passes
  reading-against-reading equality and is caught by the literal alone. The previous experiment
  observed it once; this one measured it three times, blind.
* **A concluded experiment's subject does not move; its harness does.** The reconstruction
  subject is untouched. Its harness followed the shared infrastructure twice, and both times
  every assertion kept its value and its wording.
* **Tampering with the repository is an instrument no phase of this protocol has.** Every
  comparison here weighs a rebuilt world against a remembered one, and a reconstruction has no
  remembered world. Observation 6 exists because the repository was edited by hand.

## Candidates for later experiments

* **`ThesisArchive`** — what a decision still records once the decided world is stored. The
  question this experiment refused to answer, now with a repository that speaks the archive's
  idiom.
* **A verifiable coordinate** — a value that ties a decision to the state of knowledge rather
  than to a position in it, and what permitting a derived value costs.
* **Synthesis across process death**, which needs decisions to travel between lineages rather
  than worlds.
* **Cost**, unmeasured and now larger: reconstruction admits the journal in step with the
  lineage rather than in one pass.
