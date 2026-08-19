# Observation 5 — What an application would need, and none of it is built

Phase 5. Requests, verbatim, in the vocabulary of the need rather than of a solution. An experiment
that named a mechanism here would be choosing Part B's answer before the criterion is applied to it.

Each is written as the sentence a reader or a writer would say, followed by what in the phases above
put it there and what it is **not**.

---

## Request 1 — *"Tell me whether the repository I am reading is the one its writer meant to leave."*

Observation 1. One partial state reconstructs, answers for every world the previous repository
answered, and is byte-identical to a legitimate commit. So the question has no answer, and it is the
only one of the six that a reader would want to ask and cannot.

**Not** *tell me the repository is valid*, which it already does, six tampered repositories out of six.
This is a question about a **writer's intention**, and nothing in the record is about that.

---

## Request 2 — *"Let me put back what was there before a write that did not finish."*

Observation 3. Two of the six states can be put back, using two rules the format already supplies, and
four cannot. In three of the four the reason is the same: the file that appends has no witness of the
length it had.

**Not** a history of repositories, and not versioning. The request is for **one** previous state, and
for it to be reachable from what is on disk.

---

## Request 3 — *"Say what a record was written beside, and not only what it was taken after."*

Observation 3, and it is the narrowest of these. A decision records the entries that stood when it was
taken, which pins a prefix and says nothing about the end. Fifteen entries of sixteen are pinned by the
last surviving decision, and the sixteenth cannot be attributed to either repository.

**Not** *record the journal's length* — that is a shape, and it belongs to Part B if Part B earns it.
What is asked for is that a file's **extent at the moment of writing** be something a later reader can
weigh, in whatever form.

---

## Request 4 — *"Refuse to leave a state nobody meant to leave."*

Observations 1 and 2 together. Five of the six states are refused on being read, which protects a
reader and, measured, says nothing about the record. The request is for the states not to be reachable,
which is a different promise from any this repository makes: every promise measured in seven
experiments is to whoever reads, and this one is to whoever writes.

**Not** durability against power loss. Whether bytes handed to the operating system reach the platter
is excluded, and nothing here measured it.

---

## Request 5 — *"Tell an interruption from a tampering, and from a pruning."*

Observation 1, and it is **not this experiment's to answer**. Four intents now share one mechanism and
the record distinguishes none of them. It is the authenticity candidate, gathered in
[`candidates/00-authenticity.md`](../../../candidates/00-authenticity.md), appearing for the sixth time
and named here only so that it is not counted as something this experiment left undone.

---

## What is not requested

**A rename, a lock, a write-ahead file, or one file holding three.** All four are shapes, three of them
were named in the protocol's exclusions, and the fourth is the variable it left open. Part B chooses
among them or declines to, against the criterion the coordination experiment set — and the choice is
made after these requests rather than before.

**Anything from the engine.** Every request above is about files. Nothing here asks the ontology to know
that knowledge lives in one, which is the severe failure condition the protocol named and did not meet.
