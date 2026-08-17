# Observation 10 — Every check is about the reference, and none is about the attribution

The corroboration experiment asks two questions of anything a repository holds. Phase 5 answers both,
and the second answer is the one the protocol said would be uncomfortable.

## What compares it, on every read

Something does, and it is more than expected. The check runs inside reconstruction and refuses two
things:

```text
by = <a commitment's identity>              DeciderNotKnown   the identity names no agent
by = <a party admitted after the decision>  DeciderNotKnown   nobody had admitted it yet
```

One refusal for both, because at the coordinate a decision was taken at they are the same thing: an
identity that is not among the agents the replay had produced.

The second half is the interesting one, and it is not existence — it is **timing**. The check asks
the replay's prefix rather than the whole journal, so a decision cannot be attributed to a party that
came later. Moving the check to after the full replay accepts exactly that case, which is how the
distinction was measured rather than asserted.

That is the third thing built on the coordinate a decision carries — after resolving the cut and
witnessing the prefix — and it did not need anything new.

## And where the checking stops

Swap the two parties' claims. The repository reconstructs, corroborates, and says the opposite of
what happened:

```text
planner's line ← attributed to the steward
steward's line ← attributed to the planner
                                              →  Ok, three worlds
```

Every check passes because every check is about the **reference**: that the identity names a real
party, known at the right time. Nothing is about the **attribution**, and nothing can be. The only
witness to who took a decision is the party that wrote the record, and no amount of internal
agreement reaches a fact that left no other trace.

> The record can check that the party it names exists. It cannot check that the party it names
> decided.

Which is precisely the boundary the protocol predicted, stated as a measurement. It is not a gap to
be closed by a better check — a signature and a key are what close it, and those are the
authenticity question the corroboration experiment named, measured and excluded.

## What becomes impossible if this is not preserved

One thing, and it survives scrutiny: **a process with no memory cannot tell whose line is whose.**

A party that was there knows its own decisions, so it can compute the other's by subtraction. A
party that terminated cannot, and neither can a third reader. Since surviving the process is the
whole point of the repository — the reconstruction experiment's question — the decider is the only
thing that makes *whose* survive alongside *what*.

Measured: a fresh read answers `worlds decided by the steward → { equipping }`, and Phase 4's
measurement stops holding.

```text
Phase 4   two parties and one party are one repository
Phase 5   they are no longer the same repository
```

The gain is real and it is exactly as large as the hypothesis said: naming a party lets a party be
*referred to*. Nothing more.

## The cost, measured as provenance measured its own

Phase 2 established that deciding what is already decided adds nothing — one record, one world,
because identity is derived from content. A decider is not derived from content, so that stops being
true:

```text
two parties deciding the same fork  →  one world, two records, recorded twice
```

Deduplicating by decision instead of by record collapses it back to one, which is how the cause was
isolated: the cost comes from the party being part of what a record *is*.

And the honest reading cuts both ways. Two records of one world is duplication, and it is also the
agreement record Phase 3 said was missing — for the case where two parties decide the *same* fork,
and only that case. The mutual adoption Phase 3 measured still produces two different worlds, because
a parent is part of an identity.

So the decider fills a corner of the gap Phase 3 found and not the interesting part of it.

## A coupling nobody asked for

Part A's canonical order reads what the decisions say. A decision now says who claims it, so the
order takes the party into account — the position of a decision in the file moved when it gained one.

Still canonical, still independent of arrival order, so nothing Part A concluded breaks. But it is
worth naming: recording *who* reordered the record of *what*, and a linearization chosen from
decision content will absorb anything added to decision content.
