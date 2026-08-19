# Observation 3 — The record rolls back what something witnesses the length of

A4 predicted that in every partial state, including the refused ones, the repository that existed
before the write is gone. It is **refuted**. In two of the six states it comes back byte for byte.

Phase 3 puts it back using rules the format supplies, applied **blind** — without being told which file
the commit had replaced, because that is the only reader there is:

```text
the worlds file is derived                 recompute it from the two files that produce it
the lineage appends, and the worlds file
  records how many worlds the previous
  one produced                             truncate a lineage longer than the record beside it
```

There is no third rule, and the absence is the measurement. Nothing on disk records how long the
journal was.

```text
the commit's files    refused?          the previous repository
────────────────────────────────────────────────────────────────
journal               no                lost
lineage               at the entry      put back, byte for byte
worlds                at the length     put back, byte for byte
journal + lineage     at the length     lost
journal + worlds      at the length     lost
lineage + worlds      at the entry      lost
```

## One rule, and the journal fails it

A replaced file comes back where **what survived determines it**:

* the **worlds file** is derived from the other two, so it comes back whenever both survive;
* the **lineage** appends, and its previous length is recorded in the worlds file — and nowhere else.
  So it comes back where the worlds file survived, and the state that replaced both loses it;
* the **journal** appends, and nothing records its length. It never comes back.

What a replaced journal costs is exactly the tail no decision witnessed. The witness of the last
surviving decision pins 15 of the 16 entries the previous journal held, and the sixteenth is the one
nothing decided about:

```text
witnessed by the last surviving decision   15
the tail no decision witnessed              1
                                           ──
the previous journal                       16
```

So the witness is a **floor** and not a length, and nothing on disk says whether the floor is the
length. A reader cannot tell a previous journal that ended at its last witnessed entry from one that
held two more.

Which is the durability face of what exploration measured about pruning: *nothing referring to an
admission is why no reader can find it*. Here it is why no reader can attribute it to a state.

## Being refused and being safe are independent

Not weaker than the protocol expected, and not stronger — unrelated. Three of the four cells are
occupied:

```text
                  put back   lost
refused                  2      3
reconstructs             0      1
```

The one empty cell is the reassuring one: no partial state both reconstructs and keeps what was there.
So a refusal is worth exactly what corroboration promised — a reader that cannot be misled — and it
carries no information at all about the record. Two of the five refusals sit on a repository that can
be restored whole; three sit on one that cannot; and the state nobody is warned about is lossy.

## The record contains the rollback, and not the reason to perform it

The two rules above are available and are not safe to apply blind, and that is the sharp end of this.

A lineage written before its journal is refused by `the journal holds no entry <appended>`. So is a
repository whose journal was **tampered** — an entry a standing decision names, removed. The refusal is
the same refusal. In the first case truncating the lineage restores the previous repository; in the
second it destroys a legitimate decision and calls it recovery.

> The repository holds enough to reconstruct the state before an interrupted write, and nothing that
> says an interruption is what happened.

## What this answers, and it was an open question

> *What would a repository have to keep in order to be rolled back, and is keeping it compatible with
> nothing derived is persisted unless something compares it?*

It already keeps some, and what it keeps is **precisely the derived witness corroboration keeps**. The
worlds file exists because a reader needs two representations to weigh; the length it carries is the
only rollback information in the repository, and it was put there for a reader.

So the two are not in tension and never were. The amount of rollback a repository has is the amount of
derived witness it compares — rollback is a **byproduct of corroboration**, not a competing
requirement, and a repository that persisted less derived material would have less of it.

The corollary runs the other way too, and it is the one that bears on a remedy: the journal is the only
file nothing compares by length, because the journal is the only file that is a **source**. Adding a
length witness for it would be persisting a derived value that something compares, which the rule
permits — and it would buy rollback for the file that has none.

## What it does not say

That the previous repository is worth restoring, or that an application should try. Phase 5 records what
an application would need; nothing here proposes a shape.

And nothing about `fsync`. Every file measured here was written whole by a call that returned.
