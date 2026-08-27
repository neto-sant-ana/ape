# Custody

## Abstract

Experiment 10 established the sentence this row has leaned on ever since:

> The witness is the **only** claim a repository holds whose subject is its history, and what it exists
> for is a change no world is a function of — knowledge that arrived, was never depended on, and left.

The claim is written by a decision, out of the prefix that stood when it was taken. So it covers the
journal up to the last coordinate.

**What comes after the last coordinate is in no witness.** A record admits entries and does not decide
again; nothing names them, nothing depends on them, and no world is a function of them. Which is
exactly the class experiment 10 named — arriving at the half of the journal 10 did not look at.

```text
journal    [ ................ prefix ................ ][ ... tail ... ]
                                                      ^
                                          the last decision's coordinate

witness    every entry here, by address                 nothing
```

Experiment 07 asked for a claim about the journal's extent and was refused it as a substitute for
atomicity — which it was, and 07's own repair made a torn write unreachable through the application.
This is the part that survived that refusal, and it is not about tearing: `write_journal` is public and
five experiments need it, so a record edited from outside is a shape the application already expects
and defends against by corroboration. The question is whether corroboration reaches the tail.

Chosen by measuring vocabulary load first. `extent` is the queue's own word for this and appears 14
times, which makes it the question rather than a handle for it; `unwitnessed` appears **49** times,
which is the concept saturated in a *different* sense and is probably why nobody noticed this one.
**Custody** is 0, and it names the relation the question is about: what a record holds and answers for.

The predictions are lettered **U**, because `C` belongs to four earlier experiments.

---

## Question

**Does a record's one claim about its own history cover everything the record holds — or does it stop
at the last decision, and is what lies past it lost without consequence?**

The second clause is the experiment. A gap that changes no answer is a curiosity; a gap that changes
what the **next** decision stands on is a defect, and the difference is measurable rather than
arguable.

---

## What *covers* means here, decided before any phase runs

The four experiments before this one each settled a word first. The word here is **covers**.

> **A claim covers an entry when a record that lost it, or gained one beside it, is refused because of
> that claim.**

Refused *because of that claim* is load-bearing. Three weaker readings it deliberately excludes:

```text
mentions          the entry is in some collection the record writes down. A `worlds.json`
                  mentions commitment identities and refuses nothing about the journal

is reachable      replay walks past it. Replay walks past everything, which is why a
                  journal is replayed at all

changes an        experiment 12's insertion changed no world and was refused anyway, and
answer            experiment 13 measured a change that moved an answer and was not. What
                  a claim covers is not what an answer depends on
```

The third is the one that matters here, and it points both ways: this experiment is about entries no
answer depends on, and *no answer moved* is therefore not evidence that nothing was lost.

---

## Hypothesis

```text
the claim stops at the last       a witness is written by a decision, out of what stood when it
coordinate                        was taken. There is no decision after the last one, so there
                                  is no claim about what came after it

and what is lost there is not     the next decision taken in that record stands on the prefix
inert                             that is actually there. A record that silently forgot what it
                                  learned decides differently, later, and nothing said so
```

If both hold, experiment 10's class is half-covered and the uncovered half has consequences. If the
first holds and the second does not, the gap is real and costs nothing, which is a result and closes
experiment 07's surviving request rather than answering it.

---

## Pre-registered predictions

Five, written before anything runs.

**U1 — a journal truncated past its last coordinate reads, and answers what it answered.**

*Prediction:* every guard passes. `replay_through` reaches every coordinate, `corroborate` compares
prefixes that are unchanged, and `worlds.json` matches because no world is a function of the tail.

*Refuted if* anything refuses, in which case experiment 10's claim is whole and this item is dead.

**U2 — and a journal *extended* past it reads too, so the gap is two-sided.**

*Prediction:* stated separately because a claim that caught additions and not losses would be a
different shape. Entries appended after the last coordinate are in no witness either.

*Refuted if* one direction is caught and the other is not.

**U3 — and what is lost is not inert: the next decision taken there stands on a different prefix.**

*Prediction:* a decision taken in the truncated record is witnessed by a prefix that is missing
entries, produces a different world where the tail reached anything it selects, and is a legitimate
record either way. **This is the experiment**, and it is what separates a gap from a defect.

*Refuted if* a later decision cannot tell the two records apart.

**U4 — a witness cannot be made to cover the tail.**

*Prediction:* not for want of trying but by what a witness is. It is written **by a decision**, from
the prefix that stood; the tail is by definition what stood after the last one. Widening a witness
would mean a decision claiming knowledge that did not exist when it was taken.

*Refuted if* some arrangement of the existing shape covers it.

**U5 — so what would cover it is a claim whose subject is the record rather than a decision, and the
record holds no such thing.**

*Prediction:* the three files are a journal, a lineage of decisions, and the worlds those produced.
Every claim in them is made by a decision or about a world. A claim about the record's own extent has
no author among them.

*Refuted if* one of the three already makes a claim of that kind.

**U1 with U3 is the experiment.**

---

## What is carried forward, and is not available as a finding

* **The witness is the only claim whose subject is the record's history**, and it exists for a change
  no world is a function of — 10.
* **A whole write is all-or-nothing against a process that stops**, and a torn journal is unreachable
  through the application — 07.
* **Writing one file is public and five experiments need it**, so a record edited from outside is a
  shape the application expects — `cli/src/repository.rs`.
* **What a cut recognizes is a negative, and no closure holds a negative** — 12.
* **Knowledge that arrived, was never depended on, and left is what the witness forbids** — 10. *That
  it forbids it in the prefix is not a finding; that it does not forbid it in the tail is what this
  experiment is about.*

Before any sentence here is allowed to be a finding, ask what would have to be false for it to be
false.

---

## Motivation

It goes now because it is one of two remaining faces of the question the agents row is parked on, and
because it is the older of the two: experiment 07 asked for it and every result since has left it
where it was.

**By ripeness** it has one arrival and that is honest — nothing has re-asked for it, because nothing
has met it. Which is the shape of a gap rather than a friction, and the reason a count is the wrong
instrument here.

**By what it costs if deferred** it is small: it adds rather than changes, so nothing built on the
present shape becomes rework. That is also why it is a reasonable thing to run while a row is parked.

---

## Experimental Boundary

This experiment asks whether a record's claim about its own history reaches the whole of it.

It includes:

* the tail, measured: what is there, what claims it, and what happens when it moves;
* both directions, loss and gain, because a one-sided claim is a different shape;
* what a later decision in the moved record stands on;
* the three files, read for a claim whose subject is the record;
* every state put to the three guards, as a closed table.

It deliberately excludes:

* **the engine.** Nothing here is about what the Canon admits;
* **atomicity.** Experiment 07 settled the write, and a torn journal is not what this is about;
* **a shape for the claim.** What would cover the tail is Part B, decided by the phases, and naming
  one here is what the methodological constraint forbids;
* **more than one record.** Two records meeting is the row's other thread and it is not this.

---

## Experimental Subject

**A subject of this experiment's own.**

What it must express:

```text
a tail that is not empty     entries admitted after the last decision, and more than one,
                             because a tail of one cannot be partially lost

a tail something later       at least one entry the tail holds that a subsequent decision
would reach                  would select or recognize — without it, U3 measures nothing

a tail nothing later         and at least one it would not, so the phase can say which half
would reach                  of the tail has consequences and which does not

answers that differ          every state answers a number the arrangement wrote down, and
                             the truncated record's later decision answers a different one
```

The second and third together are where a subject would make U3 true or false by construction. A tail
that is entirely reachable makes every loss consequential; a tail that is entirely unreachable makes
none of them. Both are producible here, and the phase names which it is looking at.

---

## Initial State

```text
one record        a journal whose last entries came after its last decision
three guards      unchanged and consulted throughout
one later         taken after the tail has moved, which is the only thing that can tell
decision          two of these records apart
```

---

## Procedure

### Phase 0 — What claims the tail today

Read once. Which entries the last decision's witness names, which it does not, and what each of the
three files says about the difference. Every answer as a literal.

### Phase 1 — Losing it, and gaining beside it

U1 and U2. The truncated record and the extended one, both written whole and read through the
application's own reader.

### Phase 2 — What the next decision stands on

U3. A decision taken in each of the three records, and what it is witnessed by and what it answers.

### Phase 3 — Whether a witness could reach it

U4. Not by proposing a wider one, but by asking what a witness is written from and what stood when.

### Phase 4 — Reading the three files for a claim about the record

U5, as a closed list: every claim each file makes, and whose subject each one is.

### Phase 5 — The three guards

Every state to the coordinate, the witness and the worlds file, and to the reading.

### Phase 6 — Part B, if it was earned

A claim whose shape is decided by Phases 0 to 5 and named nowhere above.

---

## The condition on building it

This adds a claim to the record, and the row's rule aimed at additions applies:

> **A capability several results asked for is still a want.**

Only one result asked for this, which is fewer rather than better — a single arrival is an anecdote by
the same standard. So Part B is refused unless:

> **The phases show the loss changing what a later decision stands on**, and show it against a record
> that is legitimate and reads. A gap that costs nothing is a finding and not a reason to write a file.

---

## Success Criteria

1. *Covers* is defined before any phase runs, and the three weaker readings are each produced.
2. The tail is measured against **all three** files, not only the witness.
3. Both directions are measured, and a one-sided result is reported as one-sided.
4. The subject produces a tail with reachable and unreachable halves, and the phase names which half a
   measurement is about.
5. U4 is answered by what a witness is written from rather than by failing to think of a wider one.
6. Every literal is written before the run.
7. Nothing is added to the APE engine.
8. Part B is built only against the condition above, and its absence is a reportable result.
9. The fifteen earlier conclusions stand, or the change is recorded as a result of this one.
   Experiment 10's is the one at risk, and U3 is how.

**Criteria 3 and 4 are the experiment.**

---

## Failure Conditions

The severe one:

> If nothing refuses the truncated record **and** a later decision cannot tell it from the original,
> then the tail is inert and experiment 07's surviving request is answered *no*. That is a result, and
> it retires an item that has been on the board since 07 rather than leaving it to be re-asked.

Its neighbour, and the one that would be worse:

> If something **does** refuse the truncated record, then experiment 10's sentence is whole and this
> protocol was written against a gap that is not there. The finding would be where the refusal comes
> from, and every observation above would be a mis-reading rather than a measurement.

And the ordinary ones:

* a tail built entirely of entries a later decision reaches, which makes U3 true by construction;
* *the witness cannot cover it* reported where what was measured is *this witness does not*;
* a loss reported as consequential because a number moved, when the number moved for another reason;
* a Part B built because a record ought to know its own size, which is coherence and not evidence.

---

## Variables Deliberately Left Open

### Where such a claim would live

A fourth file, a field on the worlds file, a header on the journal. All are shapes and the phases have
not measured which the finding needs.

### Whether a record should be able to forget

A tail nobody reasoned about may be exactly what a record is allowed to drop. This measures what
dropping it costs and does not decide whether it should be allowed.

### Two records

The other thread, and not this one.

### Cost

Deferred by sixteen experiments.

---

## Methodological Constraint

> *Do not introduce an abstraction intended to solve an experiment that has not yet been performed.*

The previous experiments' conclusions are not revised here; where this one finds something that would
have changed them, it is recorded as a finding of this experiment, against the implementation as it
then stood.

And the rules the last ten experiments earned, which apply without amendment. Before recording anything
as a finding, ask what would have to be false for it to be false. Every literal is written before the
run, and a wrong prediction is corrected in the open rather than adjusted. A prediction's justification
must quantify over as much as its claim does. A friction is evidence of a want, not of a need. A phase
must not be satisfied by an arrangement it could not have failed. A negative result must name the space
it searched. A capability several results asked for is still a want. A guard whose two halves read the
same derived value must assert the mutation before it reads. And a measurement taken with an unchecked
instrument is a false conclusion wearing the clothes of evidence.

---

## Expected Pressure Points

### *No answer moved* will be read as *nothing was lost*

It is the whole trap, and experiment 12 already walked into its mirror image: an insertion that changed
no world was refused, and a change that moved an answer was not. What a claim covers and what an answer
depends on are different sets, and the definition above exists to keep them apart.

### The tail will look like an edge case until the next decision is taken

A record sitting still after its last decision is the ordinary state of a record, not a corner of one.
Every repository in this laboratory is in it right now.

### U4 will be tempting to answer by design

*A wider witness would fix it* is a sentence about a shape, and the prediction is about what a witness
**is**. The phase answers by reading what writes one, and if a wider one is possible the finding is that
— not a proposal for it.

### Only one result asked for this

Which the condition above turns into a higher bar rather than a lower one. The honest outcome of a
single-arrival item is often that it was right to leave it.

---

## Observations

Each observation is recorded as its own numbered document beside this one, as the experiment produces
it. Record facts rather than decisions retroactively presented as inevitable.

Where possible, record the smallest reproducing case.

---

## Open Questions

* Is a record's extent a fact about the record or about the writing of it?
* If the tail is droppable, is a record that drops it the same record?
* Does anything outside this laboratory ever read a repository twice?

---

## Experimental Principle

Reconstruction asked whether meaning survives.

Divergence asked whether deliberation survives.

Corroboration asked whether the record can be trusted without being believed.

Convergence asked whether two lines of deliberation can still reach each other.

Provenance asked whether a record that can be rebuilt perfectly is thereby understood.

Coordination asked whether a record built for one mind can hold two.

Exploration asked what a record owes the worlds nobody chose.

Atomicity asked who the record's promises are to.

Contention asked whether a promise made to one writer is a promise at all when there are two.

Collision asked whether two records that have never met were already the same record.

Witness asked whether a decision is about everything it happened to come after.

Veracity asked whether a record that agrees with itself has thereby told the truth.

Commensurability asked whether two records that disagree have a common measure anyway.

Indexicality asked whether a decision can say where it stood to somebody who was not standing there.

Individuation asked whether two records that learned the same thing on different days learned one thing.

Assimilation asked whether a record can take what another knows without claiming to have known it.

```text
This one asks whether a record's word about its own
past reaches the part of it nobody has reasoned about.
```

Experiment 10 found the record's single history-shaped claim and said what it was for. This asks
whether it reaches the whole of what it is for, and the answer is either that a sentence eleven results
have leaned on is narrower than it reads, or that the part it does not reach was never worth reaching.
