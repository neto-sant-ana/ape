# Observation 2 — The quiet loss and the loud one

The loss was predicted, so finding it is not the observation. How it presents is, and it presents
two ways depending on where the other party's write lands — with the **worse** of the two being the
one that says nothing.

## The measurement

Two parties read the founded repository, which holds one world. Each decides a fork of it. Each
writes.

```text
both read            lineage.json = [ shared ]                         1
one decides          + staffing                                        holds 2
the other decides    + equipping                                       holds 2
one writes           lineage.json = [ shared, staffing ]               2
the other writes     lineage.json = [ shared, equipping ]              2   ← three were taken
```

The staffing world is gone. Not damaged — **absent**, along with its witness, and nothing in the
repository is a reference to it. So:

```text
reading::corroborated(&repository)   →   Ok, two worlds
```

It reconstructs. It corroborates. Every coordinate agrees with every other coordinate. A reader has
no way to ask the question that would find the hole, because the hole is not a hole: it is a
smaller, coherent repository.

## And the interleaving that leaves evidence

A party's write is not one write. It is three files, and the whole-file semantics apply to each of
them separately. So a party that lands *between* another's writes leaves this:

```text
one writes lineage        [ shared, staffing ]
other writes lineage      [ shared, equipping ]
other writes worlds       [ shared, equipping ]
one writes worlds         [ shared, staffing ]
```

```text
WorldDisagrees { position: 1, coordinate: "what it still proposes" }
```

Refused, by name, at the coordinate that moved.

## Which is the finding, and it is not about corroboration being weak

The same decision is lost in both interleavings. Only the second one is detected — and the reason is
not a gap in the check but what the check *is*.

Corroboration compares two representations of one fact and refuses them when they disagree. A write
that lands between them moves one representation and not the other, so it disagrees. A write that
lands **before both** moves both, consistently, and there is nothing left to disagree with.

> A tear is not a worse repository than a lost update. It is a louder one.

That reverses the intuition a reader brings to it. The interleaving that looks like corruption is
the recoverable one — you know something happened, and you know where. The interleaving that looks
clean destroyed the same intention and left no trace of having done so.

## What it means for the repair

Phase 2 cannot be a stronger comparison between what a repository holds and what it produced. That
comparison is already exhaustive on the axis it works: it caught the tear immediately, at the right
field, without being asked. It cannot reach a lost update because both sides of every comparison
were written by the same hand.

Whatever the repair is, it has to happen at the moment of writing — a writer has to be able to find
out that what it read is not what is there. Which is the shape the Canon already has one layer down,
and Phase 2 is where that gets tested rather than inherited.
