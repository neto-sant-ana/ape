# Observation 3 — What survives of the loser is not a property of the loss

C5 predicted that contention loses a whole party's line — its admissions and its decisions together —
and that the repository reconstructs over what is left. Both hold, in all six orderings. What C5 did
not predict is that the *second* half of the answer is not one answer.

```text
the two prepares were separated by a turn    the loser's whole state is the
                                             previous generation, intact

the two prepares collided                    the previous generation is the base,
                                             and the loser's state is nowhere
```

Same visible outcome — one writer's state live, the other's line gone from it, both writers told they
succeeded. Different residue, and nothing in the repository says which of the two happened.

## The generation that keeps the loser was built for something else

The atomicity experiment gave the repository two generations so that *the repository before an
interrupted write* would survive. In the serialized ordering that previous state happens to be the
losing writer's whole repository, so the only place a lost writer's work exists is a facility built for
a different failure — and it is the *previous write*, not the *losing writer*, that the design keeps.
Where the prepares collided, the previous write was the base, and the loser has nowhere to be.

So a recovery is available in some orderings and not others, for reasons that have nothing to do with
recovery.

## What the record says about a party whose line is gone

Phase 5 asked, for every ordering, and measured both halves positively rather than by absence.

```text
decided_by(loser)      empty
decided_by(winner)     one world
the loser's agent      still in canonical knowledge — it was admitted in the base
the loser's plan       not in the journal, and not among the commitments
```

The party exists and nothing says it ever decided anything. That is not a new shape: the coordination
experiment recorded that `decided_by` cannot tell *not this party's* from *unclaimed*, as the cost of
an optional field. This is the same unanswerable query arriving from the other side — a party that
decided and was overwritten is indistinguishable from a party that decided nothing — and here the
cause is a write that does not compare rather than a field that may be absent.

Stated as the query that has no answer:

> **Ask a repository what a party decided and an empty answer means two things.** Nothing, or nothing
> any more. A record whose parties are knowledge and whose claims are not can say that a party exists;
> it cannot say that a party's line was ever there.

## What it does not say

That the loser should be recoverable, or that a repository owes a lost writer anything. It says that
the answer today depends on an ordering the record does not preserve, which makes *recoverable* not a
property of the repository at all.
