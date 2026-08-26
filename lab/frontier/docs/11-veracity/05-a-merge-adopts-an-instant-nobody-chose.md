# Observation 5 — The search. Four generators leave nothing false, and the fifth does

Every state, put to all three guards and to the reading, one generator at a time.

## The four that leave nothing false

**An interrupted write** leaves the previous generation, live and whole. Measured against both
generations: before the turn a reader reads `[(120, 420)]`, after it `[(120, 420), (120, 370),
(120, 370), (120, 450)]`, and the turn is the only thing that changes what a reader reads. That is
**incomplete**.

**A mixture** — all eight combinations of the two generations' three files, enumerated rather than
sampled:

```text
files taken from the newer generation        outcome

none                                          answers generation one
journal                                       answers generation one
lineage                                       refused
worlds                                        refused
journal + lineage                             refused
journal + worlds                              refused
lineage + worlds                              refused
all three                                     answers generation two
```

Three answer, and the phase asserts of each that it answers *some whole generation's* answers. The
one that is not a whole state is the newer **journal** under the older lineage and worlds: a record
that knows more than anything in it has decided about. That is the state experiment 07 named and it
is still **incomplete** — the extra knowledge sits past every coordinate, so no world is a function
of it. The prediction written before the run said two would answer; three do, and the third is 07's
own finding, which is a correction of the prediction rather than of the earlier result.

**A readmission** takes two shapes and neither is false. Behind every standing decision, it changes
no answer — a decision's witness is a **set**, and re-adding a member the set already holds changes
no set. With something genuinely new between the two occurrences and a decision taken after the
second, it is refused by name: `ReadmittedEntryIsAmbiguous`. Which is worth recording as the other
half of experiment 10's finding: that refusal is reachable through an accident, and the silence 10
measured is what happens when the readmission adds nothing.

**An interleaving without a merge** loses a turn rather than an answer. Two writers that both prepare
before either turns choose the same generation, and the turn compares against what it wrote, so the
overwritten writer is refused with `Contended` and what a reader reads is one writer's whole state.

## The fifth

Two parties read one base. Each admits **the same Event** — same commitment, same observation, same
instant of occurrence — and records it on the day it learned of it. Each decides at day 10, which
lies between the two days. Each then puts back through `converge`.

Nothing is edited by hand. No file is written except through the application's own path.

```text
                                    ledger's world      counterparty's world

ledger's own record                     (120, 420)              —
counterparty's own record                   —                 (0, 120)

merged, ledger converging last          (120, 420)            (120, 120)
merged, counterparty converging last      (0, 420)            (0, 120)
```

Read the two merged rows against the two above them. **Each merge moves exactly one party's world,
and it is the party that converged first** — the one that is no longer there. And it moves to a pair
neither party's own record holds, which is the definition's hard two-writer case met exactly: not
one writer's answer imposed on the other, but an answer **neither** writer would give.

The number that moves is what has **settled**. A world that had settled 120 settles 0, or the other
way round, in a record that passes every check it has.

## Why nothing refuses, in three parts

**`converge` compares by address.** `appended` weighs the two journals entry by entry as `EntryId`s,
and its own note says two entries that agree are the same knowledge *however either side spelled it*.
The recording instant is not spelling. It is what a cut resolves its Event head against, and no
identity contains one — so the two journals are equal, fifteen entries for fifteen entries, and are
not the same journal.

Mutating that comparison to see the instant produces the shape of the hole drawn in a message:

```text
Diverged { position: 14, expected: EntryId("7a948a39…"), found: EntryId("7a948a39…") }
```

The same address on both sides. The refusal cannot name what disagrees, because what disagrees is not
addressable.

**The witness is a set of addresses.** The arriving decision's `after` resolves, its witness matches
the prefix member for member, and both are *true* — the entry is that entry and the prefix is that
prefix. Which is the sharpest way to say what happened:

> `Taken` records **where** a decision was taken and **what stood there**, and both survive the merge
> intact while the answer moves. The two things the record keeps in order to say *this decision was
> taken against this knowledge* are both true and both insufficient, because *this knowledge* is
> identified by address, and an address does not carry when it was learned.

**A merge writes its own `worlds.json`.** Observation 3 established that the one thing weighing a
recording instant is `event_head`, written down in that file and recompared on every read. A merge
derives it from the rebuild it just performed, so the comparison is a derivation against itself. The
guard that would have caught this is the one the operation disarms.

## V5 is refuted, and precisely

V5 said falsity needs a value nobody wrote, and offered it as the *reason* rather than the result so
that it could be wrong on its own. It is.

**Every field of the merged record was written by somebody.** The journal is one party's, entire. Each
decision is the party's own, verbatim — its coordinate, its witness, its selection, its instant, its
claim of who decided. Nothing was invented and nothing was interpolated.

> What nobody wrote is the **pairing**. Falsity here does not need an invented value; it needs an
> invented combination — one writer's journal under another writer's decision.

Which is why an audit of fields could not have found it, and why V1's question was the wrong shape.

## Composition adds nothing, and that is a result

A merge over a journal a stopped write left reaches the same state, by the same route, and the
abandoned generation contributes nothing: the interruption only chooses which journal is on the other
side of the disagreement. Recorded because the protocol expected compositions to be where the
interesting state lived, and here the single mechanism was enough.
