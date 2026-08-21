# Observation 2 — Derivable, and not by following references

W1's second half was *derivable from the record without asking the writer*. It is — and the derivation
is not the one the protocol imagined when it said the closure would be *over identities rather than a
guess about positions*.

## Two graphs, and the second is where it gets interesting

A world is produced from its cut and its selection, so the engine reads the Event chain to the head and
every commitment named, closed over dependencies. That is a set of commitments and Events, and it is
entirely a walk over identities.

But **an entry needs entries.** An admission resolves its references through what was admitted before
it, so a commitment that is reached drags its statement, its instance, its agents. A dependence set that
could not be replayed would not be a dependence set, so the closure has to be the second graph closed
over the first.

And in the second graph there is an edge that no field records.

## Nothing points at an eligibility

An agent is committed for only while an eligibility says so. `Axiom::emit_commitment` calls
`require_eligible` for every executor against the statement's actors and for every beneficiary against
its recipients, at the commitment's own `committed_at`. The commitment does not name which eligibility
let it exist — there is no field, and there could not usefully be one, since what is in effect at an
instant is a fact about the sequence of assignments rather than about the commitment.

So a closure that followed references only would drop it, and would look perfectly well formed. Measured
by taking the edge away:

```text
the closure of the late world                       17 entries
the same, with the eligibility rule not restated    15 entries, and the journal
                                                    no longer admits:
                                                    "agent … is not eligible for role"
```

The refusal is the engine's own, one layer below where the closure was computed, which is the only
reason the mistake would ever surface. A record written from that closure would be a repository that
cannot be read — a fault, loudly, rather than a quiet wrong answer. That is the good case; the point is
that nothing about the closure itself said anything was missing.

Stated so that it is about any record and not about this one:

> **A dependence set is derived by restating the rules that admitted the knowledge, not by walking the
> references it holds.** Every rule that consults knowledge by *query* rather than by *reference* adds
> an edge no pointer records, and a closure is only as trustworthy as its fidelity to those rules.

## What this costs the answer

W1 stands, and it is narrower than it looked. *Derivable* is true, and it is not *cheap in the sense of
following pointers*: the closure has to know the engine's admission rules, so it belongs beside the
engine or is a second copy of them. This experiment's instrument is a second copy, in the laboratory,
and it is a second copy on purpose — [the laboratory's rules](../../../README.md) forbid moving it into the
application before the trade is measured.

Had the answer come out *yes*, this would have been the first thing Part B had to place, and it is a
cost the candidate never priced: a closure inside the application is a **second representation of the
Axiom's admission conditions**, and two representations of one rule are two places for it to be true.
The answer did not come out yes, so the question stays in the laboratory — but it is the first thing any
later remedy has to answer for.

## The smallest reproducing case

One commitment, one agent, one eligibility. Drop the eligibility from the journal and keep everything
the commitment names; the journal stops admitting, and nothing the commitment holds said the
eligibility was there.
