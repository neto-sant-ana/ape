# Observation 2 — Only a file addressed by its prefix can be replaced in silence

A3 predicted that writing the lineage before the journal turns the silent case into a refused one, and
that nothing else about the application changes. Phase 4 measured it, and enumerated rather than
sampled: three files admit six orders, each order has two points at which an interruption is a mixed
state, and the twelve schedules leave **six** distinct states — every mixture of the two repositories'
files, each reached by exactly two of the six orders.

```text
the commit's files    what a fresh process makes of it                reached by
─────────────────────────────────────────────────────────────────────────────────
journal               reconstructs, answering 2 worlds                2 of 6 orders
lineage               refused: the journal holds no entry <appended>  2
worlds                refused: 2 worlds derived, 3 recorded           2
journal + lineage     refused: 3 worlds derived, 2 recorded           2
journal + worlds       refused: 2 worlds derived, 3 recorded          2
lineage + worlds      refused: the journal holds no entry <appended>  2
```

One state of six reconstructs, and it is the one where the journal is the only file the commit
replaced. Which is reachable only where the journal is written first — so under the two orders that
begin with it, and under no other.

```text
journal, lineage, worlds   silent   ·   refused at the length      ← the application's order
lineage, journal, worlds   refused at the entry   ·   at the length
```

A3 is **confirmed**, and the protocol's own note about it holds: two lines reorder a write, a rename is
a mechanism, and this is not atomicity. It changes *which* partial state is reachable, not whether one
is.

## The general form, which is not about the journal

The interesting half is not that the journal is first. It is why replacing it satisfies every
reference that survived.

```text
the lineage addresses the journal   by an entry, and by a set of entries — a PREFIX
the worlds file answers to the lineage   by its LENGTH
```

Knowledge appends, and every standing decision names the entries that stood when it was taken. So a
journal that grew still holds, unmoved, every address and every witnessed entry the old decisions
name: `replay_through` walks to the same entry, the witness sets compare equal, and the extra entries
are admitted after the last decision like any other unwitnessed tail.

A lineage that grew satisfies nothing, because what reads it counts it. A worlds file that grew
satisfies nothing, for the same reason in the other direction — and the same refusal carries both, with
the two fields saying which way round it was.

Stated so that it is about the relation rather than about this repository:

> **A file can be replaced in silence exactly where its consumers address a prefix of it. A file whose
> consumers address the whole of it cannot.**

The journal is the only file here in the first position, and it is there for a reason nothing about
durability decided: it is the only one that is a **source**. Sources append; the two derived files are
compared entire.

## What it does not say

That the order should change. Reversing it removes the silent state from *this* application's reach
and puts a refusal where a reader used to be told nothing — and Observation 3 measures what that
refusal is worth, which is not the same question.
