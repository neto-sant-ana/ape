# Observation 5 — Knowledge appends because a decision points at it

The protocol left open whether the journal and the lineage deserve the same repair, and warned that
applying the Canon's compare-and-append to intention would be inheriting a shape rather than
choosing one. Measured, they get **different** repairs — and the reason is not the one the question
suggested.

```text
intention    merges     a union, canonically ordered
knowledge    appends    refused when what a party read is not what is there
```

The tempting explanation is that knowledge is not revisable and intention is. That is true and it is
not what forces this. What forces it is on disk.

## A journal reordered makes a standing decision disagree

Every decision carries the set of entries that stood when it was taken. So consider what a merge
that ordered the journal by `recorded_at` would produce when two parties learn different things on
the same day, and one of them has already decided against what it learned:

```text
[ …founded…, grant ]              one party's journal, with a decision taken against the grant
[ …founded…, rebate, grant ]      the same entries, ordered by the day they were recorded
```

```text
UnwitnessedKnowledge { entry: rebate }
```

The repository refuses it, and refuses it for the right reason: the decision was taken against
fifteen entries and the journal now offers sixteen before reaching the one it names. Nothing about
the decision changed. Nothing about the grant changed. **The prefix changed**, and the prefix is
what the decision recorded.

## So append-only is not borrowed from the Canon

It looked like an inheritance and it is a requirement. Reordering the journal is not merely
disagreeable to knowledge-as-a-chain — it invalidates every decision whose witnessed prefix moved,
which is every decision at or after the insertion point.

```text
knowledge may only be appended, because a decision addresses it by what stood before it
```

The refusal a converging party gets is therefore the same fact, caught one step earlier and by name:

```text
Diverged { position: 15, expected: <this party's entry>, found: <what is there> }
```

Shaped after `UnexpectedHead` deliberately, and now justified rather than copied. A party that
diverges re-reads and admits again — knowledge is not revisable, so learning the same thing on top
of more history is the same fact, and it lands with both parties' knowledge present.

## The third use of a witness that was nearly dropped

The convergence experiment considered the sequence witness redundant with the address it duplicates,
and concluded it was not. Phase 1 found a second confirmation: with the address blinded, the witness
is the only thing that notices missing knowledge.

This is the third, and it is a different kind. Here the witness is not a *check* that happens to
catch something — it is what makes the append-only rule true. A repository holding only each
decision's address would tolerate a reordered journal silently, and would answer with worlds nobody
decided.

## And a party that cannot converge leaves nothing

The merged repository is rebuilt in memory before any of the three files is written, so a refusal
leaves the repository byte-identical to what it was — compared file by file, not asserted. That is
the other half of what the Canon promises a writer who lost, and here it matters more: a refusal
that had written part of a merge would be the tear Phase 1 measured, produced by the code that
exists to prevent it.
