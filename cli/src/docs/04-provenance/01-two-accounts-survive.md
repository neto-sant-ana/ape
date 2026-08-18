# Observation 1 — Two accounts survive, and that is less than it sounds

The subject the protocol asked for exists.

```text
                   ancestor { funding }                        40
        ┌──────────────┬───────────────────┐
  narrow { funding, tooling }              │
              broad { funding, tooling, expansion }   −5  ← the account refuses it
                                  receiving { funding, expansion, grant }   35
                                          │
                              adopting { funding, tooling, expansion, grant }   20
```

`narrow` wants tooling. `broad` wants tooling **and** an expansion. Neither withdraws anything, so
as intentions over the ancestor they differ in what they *introduce*:

```text
from narrow   introduced { tooling }
from broad    introduced { tooling, expansion }
```

The receiving line had already decided on that expansion for its own reasons. A resolved transfer
drops what the Target already holds, because introducing it again asks nothing — so both plans ask
the receiving line for **one identical change**, and the world that results carries no trace of
which was consulted.

With the Target pinned by what the record says the decision extends, four Base-and-Source pairs
produce that world, and the degenerate-exclusion rule convergence proposed — and never tested —
removes exactly the two whose Source is the result read backwards. **Two survive.**

## What that establishes, and what it does not

It establishes that **search does not recover** which line an intention came from. That is
criterion 1 and it is measured.

It does not establish that anyone needs to know, and the reason is worth stating carefully because
it is not a property of this arrangement:

> Two Sources that explain one world are, for the purposes of what was transferred, **the same
> transfer in essence**. That is not a coincidence to be engineered away — it is what makes them
> rival accounts at all.

Any pair of Sources that produces one world produces one resolved transfer, so they agree about
everything that reached the Target. A reader who asks "which of the two?" is asking which of two
indistinguishable donors gave an identical gift. The question may have no content, and Phase 2 is
where that is settled rather than assumed.

The protocol wrote a refuted necessity as the most likely outcome. This is the shape it would
take.

## The first draft was weaker, and why

The subject was first built so that the two Sources differed in what they **omitted**: one kept a
lease, the other withdrew it, and the receiving line had withdrawn it already. Measured, it
produced the same count.

It was replaced because the disagreement was dismissible. A withdrawal the Target had already made
asks for nothing, so the two Sources differed by an act with no effect — and an ambiguity between
a request and a no-op is a weak ambiguity. Here both plans asked for something real, and what makes
them coincide is the Target's own state rather than one of them having done nothing.

The mechanism is the same in both — a resolved transfer discards what the Target already reflects,
in either half — and it is documented by the engine and depended upon. What changed is only which
half carries the disagreement, and therefore how easily an objection lands.

## The candidate consequence, and the objection it will have to answer

`broad` is refused by the account's own bounds at −5. `narrow` and the world they both explain are
feasible. So the two candidate Sources do not have the same standing, and the question becomes:

> Did this world take its intention from a plan the account refuses?

That has a consequence outside the world's own contents, which is what the tautology above lacks.

Phase 2 has to weigh it against the obvious reply, and the reply is strong: **what travelled is a
commitment identity, and an identity carries no origin.** The tooling is the same canonical
commitment either way. `broad` is infeasible as a *combination*, and the combination did not
travel — so it is not obvious that anything about `broad` reaches `adopting` at all.

Recorded here as the live question rather than answered, and the answer is not assumed to be yes.

## A correction to the previous experiment

Convergence's Observation 8 reports three transfers producing one world, counted with the Target
left open. That counts transfers producing the same **selection**.

A parent is part of an identity, so a fork of a different Target produces a different world holding
exactly the same commitments — measured here: a transfer into `narrow` reaches `adopting`'s
selection and cannot reach `adopting`.

Pinned, convergence's own arrangement leaves two explanations with one degenerate, so the rule it
proposed would have left exactly one. Its conclusion that provenance is not recoverable by search
did not hold of its own subject. Nothing else it concluded moves — reproduction never needed the
Base either way — and it is recorded here rather than corrected there.
