# Observation 7 — The record answers what it cannot act on

C5 predicted that the application can express a meeting only by making one repository a party, and that
the meeting is therefore asymmetric. That is true of **acting** and false of **asking**, and the second
half was not predicted at all.

## What acting looks like, and it is asymmetric

`converge(repository, held)` takes a repository and a working copy — and a repository read back **is** a
working copy, which is the whole of what the application can say about two of them. Measured, over the
one relation that succeeds:

```text
met(left, right)     left holds both lines afterwards       right is untouched, by value
met(right, left)     right holds both lines afterwards      left is untouched, by value
```

The outcome is the same value whichever is the subject; what differs is who ends up holding it. So the
asymmetry is not in the answer, it is in the **effect** — one of them gains a meeting and the other
cannot tell one happened.

## And asking is symmetric, and already there

`converge::holds(repository, world)` asks whether a repository holds a world, by identity. The
coordination experiment added it so a party could find out whether what it decided survived — a question
about one repository and its own past.

Pointed at another repository's world identity, it answers what two repositories agree about:

```text
relation      what each holds of the other's worlds      changed anything?
──────────────────────────────────────────────────────────────────────────
Disjoint      0                                          no
Shared        1                                          no
Extending     1                                          no
```

Read-only, symmetric, no meeting required, neither repository told, and the same number in both
directions. Nothing was built for this. It is the operation the record already had, pointed one repository
to the left of where it was aimed.

## The finding

> **The record can say what two repositories agree about, and cannot put it together.** The question is
> symmetric, cheap and already implemented; the operation that would act on the answer refuses, at the
> first entry the two journals do not share.

Both halves are measured over the same two repositories in the same test: `holds` says they share the
base world, and `converge` refuses them at entry 13.

That is not a missing feature reported as a gap. It is the specific shape of what is missing, which is
narrower than *there is no distributed merge*: what is absent is not knowledge of the agreement and not a
way to find it. It is an operation whose subject is the agreement.

## What it does not say

Which operation. Naming one is Part B's business if Part B is earned, and the protocol excluded naming a
mechanism before the phases ran. What Phase 6 records is the need, in the vocabulary of the need.
