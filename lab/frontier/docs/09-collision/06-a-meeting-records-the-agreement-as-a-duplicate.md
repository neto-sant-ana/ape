# Observation 6 — A meeting records the agreement as a duplicate

The one relation that is not refused is `Extending`, and what it produces is worth measuring for a
reason that has nothing to do with extension.

```text
before          left  14 entries, 2 worlds        right  15 entries, 2 worlds
after           15 entries, 4 decisions, 4 worlds, intending 300, 300, 260, 230
```

Four decisions produce **three** distinct worlds. The base is answered for twice.

## Why, and it is not a bug in the merge

Both sides decided the base, and Observation 3 established that they decided *the same world* — one
identity. But what a repository stores is not a world, it is a `Taken`: the decision, the entry it was
taken after, and the set of entries that stood when it was. Those differ, because the two sides took the
same decision at different points in their own journals.

```text
left's founding      after entry 12, witnessing 13 entries
right's founding     after entry 13, witnessing 14 entries      the same world, a different record
```

The merge unions **records of decisions**, and holds each once by comparing records. Two records that
are not equal are both kept — correctly, since neither is redundant as a record — and the world they
both produce is stored once in the archive and pushed twice into the sequence. So the repository ends up
answering for one world in two places, and `worlds.json` has two identical entries.

It reconstructs. It corroborates. The derived count and the recorded count agree, because both are four.

## The finding, and it is about the two files rather than this merge

> **A repository holds records of decisions, and a decision is not its world.** Two records of one world
> are two facts — *this was decided here* and *this was decided there* — and only the second file knows
> the difference.

Which is the same shape as the exploration experiment's finding one layer along: what a record keeps is
the deliberation, not the conclusion. Here it means agreement between two repositories is representable
only as **repetition**, because the record has no way to say *and the other one decided this too*.

## Is it wrong?

Not obviously, and the honest answer is that it depends on a question nobody has asked. A reader
counting worlds gets four and there are three; a reader asking which worlds exist gets three, because
the archive holds identities. So the duplicate is visible exactly where the count is taken from the
sequence and invisible where it is taken from the archive — and the repository contains both.

What would settle it is knowing whether *two parties decided the same world* is a fact the record is
supposed to hold. If it is, this is the record holding it, clumsily. If it is not, this is a derived
file with a duplicate in it.

## What it does not say

That the merge should deduplicate by world. That would discard the coordinate one of the two decisions
was taken at, which is the only thing pinning it to a journal position — and the divergence experiment
established that a decision filed under a coordinate it did not have is a lineage that reads back as a
different lineage.
