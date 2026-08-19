# Observation 1 — Order is not a name

Three experiments recorded a lineage as a sequence, and every decision in all three extended
the one before it. That was never written down as an assumption. It did not have to be: with
one line of thinking, position and ancestry say the same thing, and nothing distinguishes a
record that means *the world before this one* from one that means *this particular world*.

The convergence subject decides two forks of one ancestor. That is where the two stop
agreeing.

## What was measured

Before anything was changed, the arrangement was built twice: once with the engine directly,
which has always been able to express it — `Thesis::fork` takes the world it extends — and
once through the decision record, which could not.

Written down as three decisions in the order they were taken, the second fork came back
extending the first:

```text
decided                          recorded, then read back
─────────────────────────        ─────────────────────────
ancestor    { A }                ancestor    { A }
equipping   { A, L }             equipping   { A, L }
stocking    { A, R }             ???         { A, L, R }
                                             parent: equipping
```

The third world holds both lines of thinking, is parented on one of them, and nobody decided
it. Both halves were predicted in writing before the run and both held: the disagreement is
in *what it still proposes*, and the parent is the sibling rather than the ancestor.

The gap is in this laboratory, not in APE. The engine's ancestry is a tree and always was.
What flattened it was the application's own record of a decision.

## What it cost

A decision now names the world it extends:

```text
Genesis  { known_at, selection }
Advance  { extends, known_at }
Fork     { extends, omitted, introduced }
```

`extends` is a `ThesisId`, which is derived — and writing a derived value down is the thing
the corroboration experiment made conditional rather than forbidden. It qualifies on the same
grounds `after` did, and by the same reading: it is an **instruction**, not a witness. A reader
derives from it. It cannot be dropped for redundancy, and it does not need a witness of its own.

That it is checkable comes for free rather than by design. An identity is derived from the
content of the world it addresses, so a repository whose earlier decisions no longer produce
the world a later one names cannot resolve it, and says which one it could not find.

Resolving requires holding decided worlds by identity, which is what a `ThesisArchive` is.
Nothing in this laboratory needed one until now, and the reason is the same fact stated
differently: a lineage that only grows forward can find the world it means by looking at the
end of a list.

## The honest measurement of what it buys

With `extends` ignored — the fork resolving the world decided last, which is the old
behaviour — the reconstruction, divergence and corroboration suites stay **entirely green**,
and only the convergence suite goes red. Sixteen tests are indifferent to the field; two
depend on it.

That is not an argument against it. It is the precise statement of what it is for: `extends`
buys nothing in a lineage that is a line, and a lineage that is a line is a tree that never
branched. It also means the earlier experiments' subjects cannot be used to defend it, and
they are not asked to.

## What this does not settle

Whether `extends` should have been an identity at all. The protocol left three candidates
open — position, identity, or the shape of the record — and identity was taken for the reason
the divergence experiment gave about journal entries: an offset addresses a place, and a place
survives nothing. That argument is inherited rather than re-made here, and a later phase may
find it insufficient.
