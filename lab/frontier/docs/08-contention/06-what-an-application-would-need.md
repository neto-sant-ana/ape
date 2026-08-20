# Observation 6 — What an application would need, and none of it is built

Phase 6. Requests, verbatim, in the vocabulary of the need rather than of a solution — because naming a
mechanism here would be choosing Part B's answer before the criterion is applied to it. The protocol's
exclusions already name four shapes; none of them appears below.

---

## Request 1 — *"Do not publish a generation I did not write."*

Observations 2 and 5. Two writers that prepare before either turns share a generation, and then a turn
is a claim about a name whose contents are nobody's business. It publishes the other writer's state
(Observation 2), a mixture of both (Observation 4), or a state that was live two commits ago
(Observation 5) — and returns `Ok` in every case.

**Not** *stop two writers from writing*. The request is that a writer's own commit be about its own
bytes. Whether two writers may proceed at all is Request 4.

---

## Request 2 — *"Tell me whether what I am about to read is the newest thing anybody committed."*

Observation 5, and it is the reader's half of Request 1. A repository that moved backwards answers
cleanly, so the question a reader would want to ask has no form: nothing on disk is a clock, a count,
or an order.

**Not** *timestamp the commit*. A time would be a second representation of order and would have to be
weighed against the first one — which is the pattern this repository already uses for worlds and
already refuses to trust one-sided.

---

## Request 3 — *"Say whether a party's line was ever here."*

Observation 3. `decided_by` answers *nothing* for a party that decided and was overwritten, and
*nothing* for a party that never decided, and a reader cannot tell those apart. The party itself is
canonical knowledge and survives every loss; what it did is a claim and survives none of them.

**Not** an audit log, and not attribution of writes. The request is narrower: that the record be able to
distinguish an empty answer from a lost one.

---

## Request 4 — *"Let a repository say whether two writers are welcome."*

The variable the protocol left open, arriving as a need. Six orderings, twenty-four calls, and nothing
in the record has an opinion about any of it — an application that intends one writer cannot say so, and
one that intends several cannot find out what it is promising them.

**Not** a lock. An application may answer *one writer, and here is how you know* just as coherently as
it may answer *several* — what is missing is anywhere to write the answer down.

---

## Request 5 — *"Let a writer that lost find out, cheaply."*

Observations 2 and 3. A refused party has a recovery path the coordination experiment established, and
it works: read again, decide again, converge. A party that was *not* refused has nothing to recover
from, because as far as it knows it committed. The asymmetry is the request: the loss is knowable at the
moment of writing and unknowable afterwards.

**Not** retry, and not a queue. Being told is the request; what to do about it is the writer's.

---

## What is not requested

**A lock, a lease, a compare-and-swap on the pointer, or a single writer by construction.** The
protocol excluded naming any of them before Part A, and none of the five requests above needs one in
order to be stated. Part B chooses among them, or declines, against the two criteria — including the
one this experiment added, which asks what a repair that serializes writers costs a writer that waits.

**Anything outside the record.** No request above asks for a file nothing refers to, a service, or a
clock. That matters: the protocol named *a repair that needs something outside the record* as a finding
rather than a remedy, and a set of requests that had quietly assumed one would have prejudged it.

**Anything from the engine.** Every request is about files and about who wrote them. Nothing here asks
the ontology to know that knowledge lives in a directory, which is the severe failure condition the
protocol named and did not meet.
