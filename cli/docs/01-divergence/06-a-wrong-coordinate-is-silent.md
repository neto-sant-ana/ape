# Observation 6 — A coordinate that is wrong but well-formed is not detectable from the record

Four candidate repairs were implemented independently, each against this subject, each blind
to the others. Three reproduced the lineage. The two strongest were then given the test the
success criteria do not ask for:

> *Tamper with the repository — not the code — and see whether a reader holding only that
> repository can tell.*

That is the situation reconstruction is actually in. There is no living world to compare
against; that is the entire premise.

---

## The two tampers

Both repositories were built by their own harness, verified, and then edited by hand in one
place.

**A coordinate addressed by identity.** The genesis records the entry it followed. It was
repointed at the cancelling Event — an address that exists, that the journal contains, and
whose prefix still admits both commitments the genesis selects. Every check the repair has
passes: the address is well-formed, present, and in the completeness audit's derived set.

**A coordinate carried by order.** The repository is one log and the position is the medium.
Two adjacent entries were swapped, so the genesis follows the cancellation instead of
preceding it. The file remains well-formed and the fold accepts it without complaint.

```text
intact       → 53e2b385…   head none   frozen 0   conflicts 1
tampered     → b5e38526…   head 7906cb…  frozen 1   conflicts 0    (both, identically)
```

Both produce the same world, and it is the diverged world Observation 1 measured — the
overspend frozen, the refusal at −70 gone, the cascade carried into the advancement and the
fork. Neither repository refuses. Neither reports anything.

---

## Why neither can

A coordinate is a claim about the past, and the record holds nothing to check it against. What
a repository *can* verify is internal: that an address exists, that a count is within bounds,
that a sequence does not go backwards. All of those pass for a coordinate that is well-formed
and false.

The distinction that survives is smaller than it first appears:

```text
malformed record  →  detectable      (an address that is not there, a count past the end)
false record      →  undetectable    (an address that is there and is the wrong one)
```

This narrows a claim made earlier in this experiment. The identity-addressed coordinate was
preferred partly because it "can audit the inverse question" — and it can, for absence. It
cannot for falsehood, and falsehood is the failure that matters here.

---

## What this does and does not say about the discipline that was rejected

Observation 5 rejected an ordering discipline because correctness rested on a rule that left
no trace. The repair that was kept does leave a trace, and this observation shows the trace is
unverifiable. The difference is real and it is narrower than the rejection implied: the
discipline leaves nothing to check, the repair leaves something that checks only itself.

What the repair buys over the discipline is not detectability. It is that the ordinary case
works at all — a disciplined application cannot reason about the day it is living in, and this
one can.

---

## What would close it, and why it is not here

A coordinate that named the *state* of knowledge rather than a position in it would be
checkable: a hash over the admitted prefix disagrees with a reordered journal, and a
reconstruction can compute it. That value is derived, and this experiment is not permitted to
persist a derived value — a constraint published by another experiment and not this one's to
relax.

So the closing move is available, named, and out of scope. Recording it here is the point:
the next experiment to touch this has the shape of the answer and the reason it was refused.

---

## Consequences to carry

* Reconstruction is only as trustworthy as the writer. Every candidate here, including the one
  kept, assumes the repository was written by a process that was telling the truth about when
  it decided. Nothing verifies it.
* The failure classes that reach this in ordinary operation are not tampering: a journal
  edited by hand, a log assembled after the fact, two writers appending without a shared
  order. The protocol lists the last as an open question; the first two are not yet listed
  anywhere.
* A test that compares a rebuilt world against a remembered one cannot find this, and a
  reconstruction has no remembered world. The instrument that found it was tampering with the
  repository directly, which no phase of this protocol does.
