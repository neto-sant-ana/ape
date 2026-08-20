# Observation 1 — The guard compares against the repository, not against the reading

C4 was wrong in both halves, and wrong the same way: it had the two orderings exchanged.

```text
                                        predicted        measured
A read  A write  B read  B write        refused          not refused
A read  B read   A write B write        not refused      refused, at entry 15
```

Phase 1 ran the first and nothing refused: the second party read *after* the first wrote, so its
journal **extends** what it found rather than diverging from it, and the merge put both lines in. Phase
2 ran the second and the refusal is there, by name and with the coordinate — `Diverged { position: 15 }`,
which is the entry each party added on top of the fifteen the base holds.

The prediction's justification names its own error:

> because the comparison is against the journal each party read and both read the same one

`converge` re-reads the repository at the moment of writing. What it compares is what is **there now**
against what the party **holds** — never against what the party read, which it does not keep. So the
window the prediction imagined, between the reading and the write, is not a window the comparison looks
through; it looks at the write itself.

Read that way, the guard's reach is not narrower than its docstring claims. It is exactly what the
docstring says, and the sentence is about the ordering C4 said it could not see:

> The journal a party read is not the one it is writing on top of.

## Which makes the application's own path safe, and that is measurable

Six orderings of two parties' read and converge, with the refused ones doing what the refusal asks —
reading again and deciding again. All six end holding **both** lines, and the refusal occurs in exactly
one place: where both parties read before either wrote.

The mechanism is a `converge` being one call. Two of them cannot interleave without a thread, so the
re-read at the top of the second one is always after the first one's write. A stale reading therefore
costs a party an attempt, and no party's line.

## What the compare-and-append is actually for, which is not safety

Removing the comparison does not let a divergent party's write through. Mutated so that it never
fires, the merge fails later anyway, at `the journal holds no entry <…>` — because the merged journal
is one party's and the other party's decision names an entry that is not in it, and *a party that
cannot converge writes nothing* leaves the repository as it was.

So what the coordination experiment's guard buys is not that the bad write is stopped. It is that the
refusal arrives **early and legible**: at the write rather than in the middle of a rebuild, naming the
position where the two journals disagree and the two entries that disagree there, rather than naming
an entry a reader would have to go and look for.

That is worth having and it is a different claim from the one that module's own note makes. Both
comparisons are load-bearing; only one of them is what keeps the repository intact.

## What it does not say

That contention is answered. Everything above is about parties that **converge**. A writer that puts
its own whole state back without merging has no comparison at all — Observation 2 measures what that
leaves, and Observation 3 what is left of whoever lost.
