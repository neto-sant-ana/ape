# Observation 4 — A cut is not reproducible from its date

This one belongs to two experiments at once, and it was found by asking a question about
neither: would running these experiments over the CLI's finished repository change what they
concluded?

Mostly not. In one place it would break them, and the break is worth more than the survey that
found it.

---

## The finding

The CLI persists a lineage as the decisions that produced it — the same discipline as its
journal, and for the same reason its Observation 4 gives: a stored `ThesisId` would be an
answer kept beside the question it comes from.

The decision forms carry an instant:

```rust
Genesis { known_at, selection }
Advance { known_at }
```

and replay resolves each cut with `KnowledgeCut::at(knowledge, known_at)`, against knowledge
that has already been replayed in full.

That reproduces the original cut only while nothing was recorded **within** the deciding
instant after the decision was taken. Where something was, the instant now addresses a later
head, and the rebuilt Thesis is a different world.

Measured, on the sequence experiment 02 audits — a genesis taken on the 6th, and a
cancellation recorded on the 6th after it:

```text
same identity?      false
original verdict :  [OutOfBounds { level: -20.0 }]
rebuilt  verdict :  []
original open      : 1
rebuilt  open      : 0
```

The intention arrives already frozen as cancelled, a cancelled commitment moves no level, and
the world that was refused reads as a world with nothing wrong.

That is the innocence of hindsight, reintroduced — the exact property experiment 02 measured
the engine preventing, lost on the way through storage rather than through interpretation.

---

## Why it is the other face of Observation 1

Observation 1 of this experiment records that the genesis's cut **cannot be rebuilt today**, and
treats the refusal as evidence: a Thesis holding that cut could not have been manufactured
later.

The same fact, read from the persistence side, says a later process **cannot rebuild it either**
— including the process that is honestly trying to.

One property, two consequences, and they pull in opposite directions. What makes a backdated
world unconstructible is what makes a legitimately old world unreconstructible from its date.

---

## Naming the head does not fix it

The obvious repair is to record the head beside the instant and rebuild with
`KnowledgeCut::within`. It does not work, and the reason is the engine being right.

`within` refuses a head recorded at an earlier instant than the one the date addresses —
`HeadPrecedesCut`. In the case above the genesis's head was recorded on the 2nd and the instant
now addresses the 6th, so the refinement is refused.

Correctly. A cut holding the 6th beside a head from the 2nd, *constructed on the 12th*, is
indistinguishable from retraction — a world claiming to know a day while setting aside what was
recorded in it. The engine cannot tell that from a cut legitimately taken before those records
existed, and refuses both.

So the boundary does not merely fail to help here. It forecloses the repair that treats a cut as
data to be restored, and leaves only the repair that treats it as a position to be arrived at.

---

## The repair the boundary leaves

A decision's cut is reproducible only when the knowledge it resolves against is the knowledge
that existed at that point in the sequence. So the admissions and the decisions cannot be two
sequences replayed one after the other; their **order relative to each other** is load-bearing.

Two shapes satisfy that, and the CLI's reason for keeping the files apart survives both — that
what became known is not revisable while which world is being reasoned about is a choice that
may be made again:

* one interleaved sequence, folded once; or
* two files, where each decision records its position in the admission sequence.

The second keeps the separation the CLI argued for and adds the missing half of a coordinate.
Which is what the finding really is: the CLI's own Observation 4 established that a
`KnowledgeCut` is a position in the sequence rather than a pointer into a snapshot, and the
persisted decision records only the *date* half of that position, re-deriving the other half
against a sequence that has since moved.

---

## What this does not say

**The CLI's experiment is sound.** Its subject never records anything within the instant a
decision was taken at, so its Phase 6 reproduces and its result stands. The limitation is
latent, and the sequence that exposes it is not one that experiment had any reason to write.

**Nothing here is an engine change.** `KnowledgeCut::within` exists and is refusing correctly.
What is missing is application-level, in what a repository writes down.

**And this experiment did not solve it on purpose.** Our own scenario is a single interleaved
sequence of admissions and decisions, folded in order, and it reproduces every identity — which
is asserted rather than assumed. That shape was chosen so a briefing would carry no narrative,
not because anyone had seen this coming. It happens to be the first of the two repairs above.

---

## Smallest reproducing case

`scratchpad/probe/tests/replay_by_instant.rs`, kept outside the repository because it belongs
to no experiment here: a genesis, an event recorded within the same instant after it, and the
same `Genesis { known_at, selection }` rebuilt against the finished history.

Everything else this survey found leaves the three results intact, and two of them improve — a
journal enumerates commitments, so the *lower bound* on alternatives becomes a bound; and a
persisted `Decision` names its operation, so what Observation 2 has an auditor inferring is
read instead.

---

## What happened after it was handed over

Everything above was written before the finding left this experiment, and it is filed unchanged
except for this section. The repair it proposed was hypothetical when proposed; it is not now, and
recording only the prediction would hide the more useful half.

The CLI took the finding, **implemented the naive form, refuted it, and recorded the refutation
before any repair existed.** Then four candidate repairs were built in isolated worktrees, each
blind to the others, and held to one protocol:

```text
count of admissions        reproduces all three   dominated: an offset re-points in silence
reference to an entry      reproduces all three   kept
one interleaved sequence   reproduces all three   viable; ordered by a different criterion
ordering discipline        reproduces one         refuted
```

Two things in that are worth more than the prediction being right.

**Both of the shapes named here survived, and the choice between them was made on a criterion this
experiment could not have supplied.** A `ThesisArchive` resolves by identity, so a repository whose
entries are addressable by identity is the one that lets the *next* question be asked. The repair
kept is a reference to an entry rather than a position in a sequence — which is the same coordinate
with a better address, and is not what was proposed here.

**The third candidate is the shape this experiment already had.** Our scenario is one interleaved
sequence of admissions and decisions, folded once, and it was chosen so a briefing would carry no
narrative. It reproduces every identity, and it was measured to do so there too. So the form this
experiment arrived at for an editorial reason turns out to be one of two sound answers to a
durability question it was not asking — which is worth stating precisely because it is a
coincidence, and reading it as insight would be flattering the wrong thing.

The handoff itself is the durable part: a finding, measured here on a scenario the other experiment
had no reason to write, was refused in its first form there and repaired on that experiment's own
terms. Neither branch changed the other's subject.
