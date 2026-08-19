# Observation 1 — There is no fact that says a write did not finish

Phase 1 interrupts after the journal. The repository reconstructs, it answers every world Phase 0
answered, and it holds the entry the interrupted commit admitted:

```text
                            entries   decisions   worlds   watermark
before                           16           2        2   2026-01-05
interrupted after the journal     17           2        2   2026-01-06
the commit, finished              17           3        3   2026-01-06
```

The two surviving worlds are compared against Phase 0 as whole values — reading and intended level
together — and they agree. What is missing is the third: the world that intends the rest of the
account, at 10 where the two before it intend 100 and 70. So the loss is an intention, and it is a
number rather than a count.

A2 called this the silent case, and the protocol asked for silence to be measured positively rather
than by failing to find an error. The positive form available here is stronger than *the repository
agrees with itself about a smaller world*.

## The measurement

A writer that admitted the same outflow and **decided nothing about it** finishes every write it
begins, and lands the same three files:

```text
write_journal(after)          write_journal(after)
     ── interrupted ──        write_lineage(before)
                             write_worlds(before)

               byte-identical, in all three files
```

Both directories are read off disk and compared as bytes. They are equal.

## Why this is not "corroboration is too weak"

The repository holds two derived witnesses and compares both on every read, and it refuses six
tampered repositories out of six. It does not *miss* this one. There is nothing here to miss: the
interrupted state and a legitimate commit differ in no byte, so no comparison over their contents can
separate them — not the sequence witness, not the worlds file, and not a third witness added later.

That is A5's prediction arriving with a reason rather than as an argument. *Atomicity's whole value is
the case corroboration cannot see* is true because the case is not a gap in the checking; it is the
**absence of a fact**. Every remedy on the reader's side of the boundary is bounded by what the record
contains, and the record contains no distinction between a commit that stopped and a commit that
stopped meaning to.

## The fourth intent, and it is not this experiment's

Corroboration reported that the repository cannot tell pruning from tampering because the difference
is intent. Exploration sharpened it past its own wording: there is nothing to tell apart, since a
pruned repository and one that never explored are the same bytes.

An interruption now joins that set — four intents, one mechanism, and the record distinguishes none of
them. Which is the authenticity candidate appearing again, and the protocol excluded it from this
experiment before it arrived.

## What it does not say

Nothing about APE. No kernel or engine type is involved: three `fs::write` calls in some order, and a
reader that replays what it finds.

And nothing yet about how much was lost. Every surviving world answers identically, and *what the
previous repository loses* is Observation 3's measurement rather than this one's.
