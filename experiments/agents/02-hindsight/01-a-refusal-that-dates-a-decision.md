# Observation 1 — A refusal is the only thing that dates a decision

The auditor was asked when each decision was taken and answered that the graph does not say —
with one exception it found by trying to reconstruct a cut and being refused:

```text
head 5664085b… precedes the cut … which its instant addresses
```

The genesis holds a cut whose instant is 2026-01-06 and whose head is the Event that settled
the opening, recorded on the 2nd. Today that instant addresses the cancellation, recorded on
the 6th. Neither public path rebuilds the original: `KnowledgeCut::at` resolves the later head,
and `KnowledgeCut::within` refuses a head belonging to an earlier instant.

A Thesis holding that cut therefore could not have been manufactured after the cancellation was
recorded. Its cut dates it, and nothing was recorded in order to date it.

---

## What was already known from the engine

All of the mechanism, and none of the use.

A cut is two coordinates, and the layer says why: an instant alone would admit a Commitment
recorded after the head, so the head is resolved from the instant rather than supplied beside
it. Naming a head directly is a refinement *within one instant* and is refused otherwise —
because a cut that held a current instant beside an old head would express retraction, which
the layer states a cut must not be able to say.

The rules exist to prevent a world from setting aside facts it already knew. That they also
make a backdated world unconstructible is a consequence nobody wrote them for.

---

## The consequence of applying it

An audit of an autonomous agent wants to know when a decision was taken, and the graph carries
no provenance at all: an Event has no author, a Thesis has no author, and the Thesis layer
states that attesting when a cut was declared is not its responsibility.

What it has instead is content. A cut is a position in the sequence, its head is part of the
Thesis's identity, and the engine refuses to construct positions that would misrepresent
history. So the ordering evidence is not a record — it is the absence of any way to have
produced that value later.

> *Through the current public boundary, a Thesis can be shown to predate knowledge recorded
> after it, without provenance having been recorded, because the cut it holds is no longer
> constructible.*

---

## The limit, so it is not overclaimed

This works only where knowledge moved *within* an instant.

A day that nothing was recorded after still addresses the head it addressed then, so a cut
naming such a day is constructible at any later time and dates nothing. The auditor said as
much: *"Nothing comparable exists for the other decisions."*

The evidence is therefore opportunistic. It is strongest exactly where it is most needed —
inside a busy instant, where several things happened and the order matters — and absent where
the day was quiet. It is not a substitute for provenance, and an application that needs to know
who decided and when still has to record that itself.

Both halves are asserted in `tests/hindsight.rs`: that the genesis's cut cannot be rebuilt, and
that a cut on a quiet day still can.

---

## Smallest reproducing case

`the_genesis_cut_cannot_be_rebuilt_today`, and beside it
`a_cut_on_a_day_nothing_followed_is_still_constructible`.

The second exists because the first, alone, would read as a general claim about tamper
evidence. It is not one.
