# Observation 7 — What an application would need, in the vocabulary of the need

Requests, verbatim, before any of them is turned into a shape. One of them was built and is
Observation 8; the rest are not, and are written here so that the next experiment inherits the need
rather than the solution.

**Request 1 — to be told that two journals are not the same journal.**
Two parties that learn one fact on different days write journals that agree entry for entry. The
record's only comparison over journals is over addresses, and an address is a function of everything
an admission carries **except** when it was recorded. So the one field that decides a cut is the one
field the comparison cannot see. *Built. Observation 8.*

**Request 2 — to know whether a decision still means what it meant.**
A decision's world is derived on every read, and nothing compares the world it produces now against
the world it produced when it was taken. `worlds.json` does exactly that — for a record that writes
it. A merge writes its own, so the one operation that can change a standing decision's world is the
one operation exempt from the check that would notice. What is wanted is not a second file; it is that
a merge weigh the arriving decisions against the worlds the arriving record recorded, which it already
reads and currently discards.

**Request 3 — to say that a record was told something twice, on two days.**
Refusing is what the repair does and it is not what a reader wants. Two parties recording the same
fact on different days is ordinary, and the honest answer is not *your journals disagree* but *this
entry was learned on day 3 by one of you and day 11 by the other*. The record has no representation
for that, and inventing one is a design question rather than a repair.

**Request 4 — an attribution that is worth checking, or none.**
Observation 3 measured that `by` can be changed to any agent the coordinate knows and every answer
holds. It is not this experiment's hazard because no generator reaches it, and it is a field the
record carries and cannot stand behind. Either something outside the record answers for it — which is
the authenticity candidate — or the record should say less.

## What is deliberately not a request

**A hash over the journal.** Experiment 01 wanted one and this experiment is the reason not to reach
for it now: a digest over the encoded prefix would have caught the merge, and it would also have
refused the *differently arranged* record of Observation 1, which answers every question identically.
A guard that refuses a record for its spelling is a guard that makes the encoding load-bearing —
the objection `WorldRecord` already answers for a witness. The comparison in Observation 8 is narrower
on purpose: it weighs the one field an address omits, and nothing else.

**A total order across two writers.** The record has no clock it can trust and this experiment did not
find that it needs one. What it found is that two records can disagree about an instant without either
being wrong.
