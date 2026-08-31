# Observation 1 — The journal takes it, and loses the return

The arrangement is one record: eighteen entries, four worlds, two parties, and a plan that moves
three times over two distinct worlds.

```text
journal      18 entries
lineage       4 decisions   a genesis that claims nobody, and three forks
worlds        4             the shared ancestor and one fork per plan
parties       2             planner takes two forks, steward takes one

DESIGNATED   W₁   W₂   W₁    three moves, and the third is the first again
```

Three forks rather than two because with two, *moving back* and *moving to the other one* are the
same act. Both parties are admitted before anything decides, which is where this differs from
[`coordination`](../05-coordination/00-protocol.md)'s arrangement — the trap of a decision attributed
to a party that did not yet exist belongs to `17-imputation`, and reproducing it here would make
every phase carry a refusal it is not measuring.

## Phase 1 — A, attempted

The journal is a closed enum of the nine kernel entities and none of them is a claim about a world.
So an application that wants a designation in there has exactly one move available: **put the world's
identity in a field that takes free text.** That is the attempt, and it is the one an application
would actually reach for.

```text
Admission::Role { label: <ThesisId>, recorded_at: … }
```

**The Canon does not refuse it.** It is admitted, and what the record then holds is a `Role` whose
label happens to be a world — addressed by an `EntryId` of its own, which is not the world's identity
and resolves to nothing.

**And nothing weighs the target.** The same attempt aimed at sixty-four zeroes — an identity no
lineage here ever produced — is admitted, written whole, and read back. Corroboration passes it. A
reader gets a plan pointing at nothing, and no layer between the caller and the file said so.

That is P1 **refuted in its letter**, and the protocol's fourth failure condition produced rather
than avoided: *a designation that names a world the record does not hold, admitted without refusal.*
A is disqualified, and not for the reason the prediction gave. It is disqualified for the worse one.

### The control, because an acceptance proves nothing on its own

`is_ok()` is only a measurement if this path can produce an `Err`. The same admission aimed at an
entity the record does not hold — a `ResourceInstance` naming an unadmitted `Resource` — **is
refused**, by name.

So the admitting layer does check references. What it checks is references to **admitted** things,
and a `ThesisId` is derived and never admitted — which is `17-imputation`'s sentence arriving exactly
where this experiment said it would, one phase in.

## And then the measurement nobody predicted

The guard for this phase was written to assert that three moves leave **three** addresses, because
the recording instants differ where the designations do not. It measured **two**, and went red saying
so.

```text
journal entries after three moves    21      18 + 3
addresses in the tail                 3
DISTINCT addresses in the tail        2      ← the prediction said 3
tail[0] == tail[2]                 true
distinct addresses in the whole      20      where the journal holds 21
```

The reason is the fact the whole project is built on: **an entry's identity is derived from its
content, and no identity carries a recording instant.** The third move is `W₁` again, so it is the
first move's entry — byte for byte, address for address.

So the journal, which is a sequence, holds three. Everything the record *checks itself against*,
which is a set, holds two:

```text
journal.json     3 entries          the moves are all there, in order
custody.json     2 addresses        membership, per `16-custody`
Taken::witness   2 addresses        membership, per experiment 10
```

**A record that lost the return agrees with itself about what it holds.** The custody experiment's
finding, one level over and with a sharper edge: there it was the tail past the last decision that
nothing claimed; here it is an entry inside the claimed region that the claim cannot count twice.

The last form of it is the one worth keeping. A decision taken after the plan returned:

```text
witness    20 addresses      two of the three moves
after      tail[0]           the entry where the plan FIRST was, not where it is
```

Its coordinate addresses the departure. There is no arrangement of content-addressed entries in which
it does not, because the return **is** the departure.

## What did not break, and it is worth having measured

Admitting a designation is not admitting an Event, so it does not move the chain a cut resolves
against. Every world witnessed before it reads back with **no coordinate moved** — checked field by
field through [`WorldRecord::disagreement`], not by identity alone.

Which matters because it rules out the wrong finding. If A were disqualified by moving the worlds,
this experiment would be about the cut, and it is not.

## Where this leaves the three homes

A is measured and out. It fails twice, and the second failure is the one that constrains what comes
next: **whatever holds a designation has to be ordered, and it has to be checked against
`worlds.json`.** A set cannot hold a plan that returns, and the admitting layer cannot weigh a
`ThesisId`.

Both of those are properties of B and C rather than of A, so P5 — which asked whether a *bare
pointer* can answer *what was the plan on the twelfth* — is now a question this row has to answer
before it builds anything, rather than after.
