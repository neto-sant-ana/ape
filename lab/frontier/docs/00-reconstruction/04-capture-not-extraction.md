# Observation 4 — Persistence is capture, not extraction

Phase 3 began by asking what to write down, and assumed the answer could be read off the
world the application had just built. It cannot.

A `Constraint` offers constructors and `check`, and nothing that returns its bounds. An
admitted `Resource` therefore does not yield the constraint it was made with, and no
sequence of public calls recovers it. The same holds wherever a value object was built from
inputs the entity no longer describes.

So nothing the repository writes is extracted from knowledge. Every record captures the
input **as it was supplied**, at the moment of admission.

## Why the boundary is shaped that way

The engine states that operational state is projected rather than stored. The application
boundary shows that the claim reaches further than a commitment's condition: the body of
known entities at an instant is not stored either. It is what folding the admission
sequence up to that instant produces.

Read that way, this and Observation 1 are one statement from opposite ends:

```text
reconstruction = replay    (not hydration)
persistence    = capture   (not extraction)
```

If state is never stored, the only durable thing is the sequence that produces it. An
application writes that sequence going in and replays it coming out, and neither direction
handles an object. An entity that could be serialized and read back would be a second
source for something the sequence already determines — which is the situation the engine
declines to create, at the cost of an application not being able to write one down either.

It also explains a coordinate the experiment had used without examining. `KnowledgeCut` is
resolved from an instant rather than pointing at a stored world: a cut is a position in the
sequence, not a reference to a snapshot of it.

## What this settled for the experiment

The subject stopped being a sequence of calls and became a journal that construction
replays. There is now one description of the world rather than two that could disagree,
and the path Phase 1 exercises is the path Phase 6 must reproduce.

## Consequences to carry

* A repository built on this boundary is a log of admissions. It is not an object store,
  and the engine offers nothing that would let it become one.
* Snapshots and indexes remain available and remain derived. A snapshot is a memoised fold,
  and its correctness is checkable by folding again — which is exactly what disqualifies it
  as a source of truth and qualifies it as a cache. The open question of when repeated
  reconstruction justifies one is unchanged, and it is not this experiment's to answer.
* The cost this imposes is the one Observation 1 measured: opening a repository is the
  length of its history. Whether that cost is worth paying is a question for a later
  experiment, and it should be paid in full here first.
