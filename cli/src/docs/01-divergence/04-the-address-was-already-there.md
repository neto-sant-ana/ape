# Observation 4 — The coordinate is a reference to knowledge, and the address already existed

The repair needed a decision to name the point in the sequence of admissions at which it was
taken. Two forms were available: a count of entries, or a reference to the last entry.

The reference was ruled out on a diagnosis that was wrong.

---

## The diagnosis, and what it missed

Journal entries looked unaddressable. `journal::replay` hands back the identities of the
things it created, and two admissions appeared to create nothing:

```text
Eligibility  → the journal recorded it and named nothing
Event        → the journal recorded it and named nothing
```

Both do produce an identity, and the engine returns it. `Canon::admit_eligibility` yields an
`EligibilityAssignmentId` and `Canon::admit_event` yields an `EventId`. The application was
discarding them at the call site.

So the boundary was sufficient the whole time and the application had narrowed it. This is
recorded because the wrong reading was not idle — it was written down, argued from, and used
to eliminate a candidate before it was tried.

> *An interface the application throws away reads exactly like an interface that was never
> offered.*

---

## What the coordinate is, once it exists

`EntryId` is the hex of the identity an admission produced, and a decision records the entry
that was the journal's most recent one when it was taken:

```json
{ "decides": "genesis", "known_at": "2026-01-10",
  "selection": ["592c7fc6…", "d4694991…"],
  "after":     "d4694991…" }
```

Two things follow, and both matter to the constraint this experiment inherited.

**It is a reference, not a cached derivation.** The reconstruction experiment drew that line
for the ids already in the journal — an identity written down so one record can name another
is not an answer stored beside its question, because replay re-derives it from content. The
coordinate carries a second load on the same line, and the same argument holds: everything
derived from it — the cut, the partition, the identity of the world — is still recomputed.

**It is not the resolved head.** For the genesis the coordinate is a commitment while the head
is `None`; for the fork it is a commitment while the head is an Event. The two coincide only
where a decision happens to follow an Event, which the advancement does. So recording it does
not smuggle in the instant-plus-head form the cut boundary refuses — the head is still derived
by `KnowledgeCut::at`, from the knowledge the coordinate selects.

---

## The audit it makes possible

Observation 2 recorded that the "nothing derived" discipline interrogates the data present and
never asks whether anything is missing, and that no method here asked the inverse. An address
admits one:

```text
replay the journal    →  the set of entries that exist
every `after`         →  must be in it
```

That is the first check in this repository that fails on something *absent* rather than on
something wrongly present. Its reach is narrow — Observation 6 measures exactly how narrow —
but the inverse question now has at least one instrument.

---

## Consequences to carry

* An identity the application discards is indistinguishable from one the engine does not
  offer. Before concluding that a boundary is missing something, check what the call site is
  dropping.
* The reference/derivation line drawn by the reconstruction experiment carries weight it was
  not originally load-tested for. It now decides whether a coordinate is admissible, and it
  held.
* The address is stable under reordering, splitting or re-encoding the journal; the *replay*
  is not, and never was. What survives is the ability to say the file changed, not the ability
  to reconstruct from the changed one.
