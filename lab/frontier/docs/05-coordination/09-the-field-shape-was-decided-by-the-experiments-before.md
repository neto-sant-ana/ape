# Observation 9 — The field's shape was decided by the experiments before it

Part B set out to record a decider and found that the first decision about it was not Part B's to
make.

## Optional, and not because optional is nicer

Four concluded experiments construct decisions of their own, and their subjects say why that matters:

> *A concluded experiment keeps its own unchanged: a published result whose subject moved underneath
> it is a result nobody can run again.*

A mandatory decider would have forced reconstruction, divergence, corroboration, convergence and
provenance to name parties they never had. So the field is `Option<AgentId>`, and the constraint
came from outside the phase that wanted it.

The same constraint applied one level down, and it was found by getting it wrong first. Adding the
party as a parameter to the subject's `decide` rewrote every call in Phases 1 through 4 — the
diff was 136 lines across measurements that had already been recorded. Reverted, and replaced by a
second entry point: `decide` writes no party, `decided` writes one.

```text
Part A   decide     the decisions Phases 1–4 measured, unchanged
Part B   decided    the same, with a party written down
```

Which leaves the arrangement holding a repository where **some decisions name a party and some do
not** — the genesis names nobody, because that is how every experiment before this one wrote it.
That is the realistic shape of an optional field rather than a tidy one, and it is the shape a reader
actually has to cope with: the answer may simply not be there.

## An identity, not a label

The decider is an `AgentId`, and the reason is the corroboration experiment's rule. A label resolves
against nothing; an identity resolves against the knowledge that stood when the decision was taken.
Choosing the weaker representation would have made the second question unanswerable by construction
instead of unanswerable on the merits.

Which settles, for this experiment and provisionally, the variable the protocol left open. The two
notions of agent **share a population and not a purpose**: an `AgentId` is a party to a commitment
in the kernel and a party that decides here, and in this arrangement the two sets are disjoint —
`customer` and `merchant` are parties to commitments and decide nothing; `planner` and `steward`
decide and are parties to nothing.

Sharing the type permits an overlap without assuming one. A separate population would have forbidden
the ordinary case where a merchant plans its own spending.

## A party is knowledge; the attribution is a claim

The two parties are admitted through the journal, like every other fact, before any decision can name
them. That is not ceremony — it is what makes half the record checkable:

```text
the party exists              knowledge, admitted, append-only
the party decided this        a claim, written by the party that benefits from it
```

The journal grows by two entries and Part A's measurements do not move, because a decider is not a
new kind of thing. It is an agent, and the repository already knew how to hold agents.
