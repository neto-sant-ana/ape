# Observation 5 — A turn can publish a state that was already replaced

Everything measured so far misleads a **writer**: a party is told it succeeded when its line was
overwritten. This one misleads a **reader**, which is a different thing and is what a repository is for.

The smallest reproducing case, five calls:

```text
1   writer A prepares                    three files, in the generation nothing reads
2   writer B commits                     alone — and into the generation A prepared
3   writer B commits again               having read, so this one holds BOTH lines
4   writer A turns                       Ok
5   a reader reads                       the state step 2 left
```

Step 3 is the repository at its best: both parties' knowledge, all three worlds, nothing lost. Step 4
undoes it with one `rename`, and the writer that did so had written none of the bytes it published —
they are B's, from two commits ago, and B's later commit is in the generation the pointer just stopped
naming.

It reconstructs. It corroborates. There is no fact in it that says it is a rollback.

## Why this is outside Phase 3's closed set, and why the set is still closed

Phase 3 enumerated every interleaving of two writers with **two operations each**, and that set is
closed and complete. This takes five operations, because it needs one writer to commit twice — the
second commit is what makes the published state *superseded* rather than merely *another writer's*.

So the closure claim holds as stated and not absolutely, and finding this is what says so. A closed set
of orderings is closed over the operations it counts.

## What it adds that Observation 2 did not

Observation 2's collision loses a line that had never been published. A reader who never saw it cannot
be misled about it — the loss is the writer's. Here the state being replaced was **live**: something
could have read it, quoted a world out of it, decided against it. And what replaces it is not damage,
which is the whole difficulty: a reader has no reason to look twice at a repository that answers
cleanly.

Stated as the property:

> **A pointer that can be turned by a stale handle is a repository that can move backwards.** Time in
> this record is a sequence of commits, and a turn is the only operation that can put an earlier one
> back — silently, cheaply, and by a writer who thought it was committing.

This is the state a reader can be misled by, which is the criterion the coordination experiment set for
a second half and the atomicity experiment inherited. It is what earns Part B.
