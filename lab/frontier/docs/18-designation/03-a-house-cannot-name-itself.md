# Observation 3 — A house cannot name itself, and the log is `Taken`'s shape

The protocol's discriminator was three-way: is *which world is live* a fact about the house, about a
record, or about a party? It is two-way, and the measurement that collapses it is one line long.

## Two records founded alike are one record

Two repositories, two directories, built from the same subject. Every file compared, byte for byte —
with both sides required to be present first, because two absent files compare equal.

```text
journal.json   lineage.json   worlds.json   custody.json      identical, all four
```

**A record carries no name, no identity of its own, and nothing above it.** So a claim *about this
record* and a claim *about the house* have the same subject and the same silence: there is nothing a
record could say to tell itself from another. They are not two homes. They are one home, and it is
the home a claim has when nobody is named.

Which leaves exactly one axis, and it is one the application already has:

> *The party that took it, **where anything says**. Optional because a decision that claims nobody is
> the ordinary case rather than a legacy one. An application reasoning alone has no party to name.*
> — [`Taken::by`](../../../../cli/src/lineage.rs)

So B and C are the same answer, which is P3 — arriving from a direction the prediction did not take.
Not *because a designation is per party*, but because **the unqualified case and the per-party case
are one field with one of them absent.**

## What was built, and every field is a phase

```rust
struct Designated { plan: ThesisId, after: EntryId, by: Option<AgentId> }
```

Three fields, and each one is something a phase measured missing. It is `Taken`'s shape, and that is
a finding rather than a convenience: a designation and a decision are both claims *about a world, at
a coordinate, by somebody*, and the record already knew how to hold one of those.

```text
plan     which world              checked against worlds.json      Phase 2
after    where in the journal     checked against the replay       Phase 5
by       whose plan               checked against the replay       Phase 3
```

**Three references, three ways to be wrong, and the refusal names which.** Phase 2's file could check
one, because it had one. A reader told only that a log is bad has to go and find out how.

**It is `after` and not a date.** A recording instant is the one value nothing derives —
`13-indexicality` — so a designation carrying one would be a claim no receiver can weigh, which is
the class `17-imputation` closed. What the log answers is *what was the plan when the record knew
this much*, and that is checkable. It is also the project's own way of asking about time, which
`Taken` settled before this experiment existed.

**And it is a sequence, ordered by position.** Phase 1's finding arrives here as a constraint on the
remedy: an `EntryId` is content-derived, so two moves with no admission between them carry the
**same** coordinate. `after` orders the log against knowledge; the file's own order is the only thing
that separates two moves at one coordinate. Measured, with three entries at one address.

## What it answers

**P5, in the coordinate the record can check.** Three moves at three coordinates, and the log says
what the plan was at each. Before the first move it answers *nothing* — which is not the same as
answering a world, and the guard asks for the difference.

**P3, with the unqualified row present.** Two parties hold two plans in one record and neither is
arbitrated, which is `converge`'s own treatment of two decisions, one file over. And the third row
carries no party: it is the record's own, the answer for a reader who is no party, and the row
Phase 3 measured a per-party file has nowhere to put.

**Criterion 2**, which is the protocol's own: a log is additive and optional, so a record with none
makes no claim — custody's tolerance, for custody's reason.

## What the red pass found, and it was a real defect

Mutating `plan_at` to read the log forwards instead of backwards left
`phase_6_two_moves_at_one_coordinate` **green**. The arrangement is why: `DESIGNATED` is `W₁ W₂ W₁`,
so the first and last entries name the same world and a guard about *order* could not see order.

The guard now uses three distinct worlds and carries the assertion that they are distinct. The
property that makes this arrangement right for Phase 1 — the return — is the one thing that hides
what Phase 6 measures, and the guard had to be told so.

Two mutations, two reds, and the second only existed because the first was tried.
