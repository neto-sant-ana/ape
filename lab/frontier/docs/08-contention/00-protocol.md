# Contention

## Abstract

A write is now whole. Three files land in the generation the pointer does not name, and one `rename`
makes them the one it does — so a process that stops leaves the repository it found, byte for byte.
The atomicity experiment measured that and named what it left open in the same breath:

> **Concurrency.** Two writers preparing at once would target the same generation, and nothing here
> refuses it. Excluded, and now the nearest unanswered question after this one.

This asks it, and it asks it about a promise that is already in the application in writing. The
coordination experiment built a compare-and-append so that a party could not put back what it read
over a journal that had moved, and said so:

> The journal a party read is not the one it is writing on top of. […] The party re-reads and admits
> again.

> *When two writers each read the same repository and each finish, which promise holds — and what is
> left of the one that did not?*

---

## Question

**Is a repository's write a compare-and-swap, or a compare followed by a swap?**

The difference is the whole experiment. `converge` reads the repository, computes a merge, and writes
it — three steps, and the refusal it can raise is decided at the first. So it refuses a journal that
moved **before** it read, and this asks whether it refuses one that moves **while it is thinking**.

Two orderings of the same two writers, and the sequence's own arithmetic says they are not the same:

```text
A read   A write   B read   B write        B reads what A left
A read   B read    A write  B write        neither reads what the other left
```

The first is the case coordination built its guard for. The second is not, and nothing in seven
experiments has asked what it leaves.

---

## Hypothesis

```text
divergence      the two orderings do not agree: one is refused by name, and one
                lets both writers finish

exposure        a whole write is atomic against a process that stops and not against
                another writer, and the pointer turning makes the loss arrive as a
                success
```

The first is the measurement. The second is the claim, and if it holds then *atomic commit* — the
thing the sequence has just finished building — protects exactly one of the two ways a repository can
be written over, and the record cannot tell a writer which one it got.

---

## Pre-registered predictions

Five, written before anything runs, each with the observation that would refute it.

**C1 — Two writers that both read before either writes choose the same generation.**

*Prediction:* `Repository::prepare` chooses its target by reading the pointer, so two writers holding
the same reading of the pointer prepare into the same generation. The second overwrites the first,
whole, and neither call refuses.

*Refuted if* the two prepares land in different generations, or if either refuses.

**C2 — The last turn wins, and the loser is told it succeeded.**

*Prediction:* in the interleaving `A prepares, B prepares, A turns, B turns`, the live generation
holds **B's three files entire** — not a mixture — and both writers' `write_whole` returned `Ok`. What
is lost is everything A appended and everything A decided.

*Refuted if* the surviving state is a mixture at this granularity, or if either write refuses, or if
any of A's admissions survive.

**C3 — A finer interleaving reopens the six states atomicity closed.**

*Prediction:* the application's own write is one call, so a mixture is not reachable through it. Reached
by writing the pending generation's three files one at a time from two writers — the instrument
atomicity already used — the outcomes are the **same six states** Phase 4 of that experiment
enumerated, and the turn presents whichever one as a finished commit.

*Refuted if* any mixture is refused *because* it is a mixture, rather than for the reasons the
atomicity experiment already named for it.

> If C3 holds it is not a new failure. It is the old set of failures arriving through a door the
> repair did not close, and saying so is the whole of what it is worth.

**C4 — The compare-and-append catches one ordering and not the other.**

*Prediction:* under `A read, A write, B read, B write`, B is refused by name — `Diverged`, at the first
entry where the two journals differ. Under `A read, B read, A write, B write`, neither is refused,
because the comparison is against the journal each party read and both read the same one.

*Refuted if* the guard refuses in the second ordering, or fails to refuse in the first.

**C5 — What contention loses is coarser than what an interruption lost.**

*Prediction:* the atomicity experiment's silent state lost an **intention** and kept the knowledge —
the journal grew, the decision vanished. This loses a whole party's line: its admissions and its
decisions together, and the repository reconstructs and corroborates over what is left.

*Refuted if* any of the loser's admissions survive, or if the surviving repository is refused.

**C4 with C2 is the experiment.** Each alone is a measurement. Together they say whether the guard the
application has is the guard the application needs.

---

## What is carried forward, and is not available as a finding

Usable in reasoning, and not reportable as a result:

* **A repository holds two generations and a pointer**, and a whole write puts three files in the one
  the pointer does not name and then turns it.
* **`prepare` and `turn` are two operations**, and the seam is public because an interruption is a
  prefix.
* **A repository with no pointer is its own live generation.**
* **Writing one file is still possible**, and nothing in the application does it.
* **A party that cannot converge writes nothing**, because the merge is rebuilt in memory first.
* **The journal a party read is not the one it writes on top of** is refused by name, at the entry
  where the two disagree.
* **Knowledge appends and a decision addresses it** by the entry that stood when it was taken.
* **Intention merges.** Two decisions cannot contradict one another, so the union of two parties'
  decisions is a lineage in the same sense either party's was.

Before any sentence here is allowed to be a finding, ask what would have to be false for it to be
false. A founding premise makes it a corollary; a documented design decision makes it a reading; an
implementation fact whose composition is unstated makes it a finding.

---

## Motivation

The coordination experiment measured a lost decision and built the repair for it at the moment of
writing, "where a party can still find out that what it read is not what is there". That sentence is
the hypothesis this experiment is here to test, because it is true of one ordering and unexamined for
the other.

And the atomicity experiment has just narrowed what *else* could be wrong. Before it, a torn
repository had two possible authors: a process that stopped, and a second writer. It closed the first
and named the second. So this is not a new suspicion; it is the residue of a result, and running it is
how a sequence finds out whether its last answer was the whole one.

**This runs before the remedy**, for the reason the exploration protocol gave and atomicity reused:
work that closes a gap makes the gap unmeasurable. A write that cannot be overwritten ends any
question about what an overwrite costs.

---

## Experimental Boundary

This experiment exercises two writers over one repository, their operations interleaved
deterministically.

It includes:

* every interleaving of two writers' `prepare` and `turn` that a single sequence of calls can express;
* what a fresh reader makes of each result, by value and not by absence of error;
* whether the compare-and-append refuses, and at which coordinate;
* the finer interleaving that writes the pending generation one file at a time;
* what survives of the losing party's line.

It deliberately excludes:

* **threads, and any measurement that needs one.** Contention here is an **order of operations**, and
  an order is a value — the same move that let coordination produce a lost decision by interleaving
  rather than by racing, and let atomicity produce an interruption as a prefix. A measurement that
  needed a scheduler would be reproducible nowhere. What this therefore does *not* claim is that any
  particular ordering is likely; it claims what each one leaves;
* **the remedy's mechanism.** A lock, a lease, a compare-and-swap on the pointer, a single writer by
  construction — naming one before Part A would be choosing the answer;
* **more than two writers.** Two is the smallest number that can contend, and nothing here suggests
  the third is free;
* **`fsync`, power loss, and a partial `rename`.** Excluded by atomicity and untouched;
* **authenticity**, unchanged and gathered as a candidate of its own;
* **the engine.** Storage is the application's decision, and so is who may write.

Those concerns may become later experiments. They must not influence the structure introduced here
unless this experiment itself requires them.

---

## Experimental Subject

**A subject of this experiment's own**, for the reason the seven before it give.

What it must express, and no previous subject needed:

```text
one base, read twice        both writers begin from the same reading, which is the
                            condition the whole question is about

two commits worth telling   each party appends knowledge and takes a decision, and the
   apart                    two are distinguishable by value — so a lost line is named
                            rather than counted

each legitimate alone       either party's commit, applied to the base by itself, produces
                            a repository that reconstructs and corroborates. What is lost
                            must be lost for being overwritten and not for being wrong
```

Everything else stays as thin as the others: integers, one quantifiable resource, no dependencies
unless the procedure demands them.

The instrument is **the order of the calls**, and nothing else. Two `Repository` handles on one
directory, in one process, and the phase decides the sequence. Where a phase needs a finer grain than
the application offers, it writes the pending generation's files directly — which is what atomicity's
instrument did, and what that experiment reported as the door left open.

---

## Initial State

```text
base         one repository, written whole by the application, and read once to
             establish what it answers
writers      two, each holding that reading and neither having written
```

Nothing is inherited from the previous experiments except their conclusions and their code.

---

## Procedure

### Phase 0 — What the base answers, and what each party's commit would answer alone

Read once, and record all three. Every later phase compares against them, so a result that is one
party's whole state is distinguishable from a mixture, and both are distinguishable from the base.

---

### Phase 1 — Serialized: A writes, then B reads and writes

The ordering coordination built its guard for. Which refusal, at which coordinate, in the
application's own words — and what B does next, since the guard's own docstring says the party
re-reads and admits again.

---

### Phase 2 — Interleaved: both read, then both write

C1, C2 and C4's second half. Which generation each prepare targets, what the live generation holds
after both turns, what each call returned, and what a fresh reader makes of the result.

Measured **by value** against Phase 0: is it the base, is it A's state, is it B's, or is it none of
them?

---

### Phase 3 — Every interleaving the two operations admit

Two writers with two operations each is a closed set of orderings. Enumerated rather than sampled,
with the outcome of each — so what is reported about *an* interleaving is reported about all of them.

---

### Phase 4 — The finer grain, and whether it is a new failure

C3. The pending generation written one file at a time from two writers, and the results classified
against the six states Phase 4 of the atomicity experiment enumerated. If they are the same six, say
so and say that they are not new.

---

### Phase 5 — What is left of the losing party

C5. For each outcome, what survives of the line that did not win: its admissions, its decisions, and
whether anything in the repository says it existed. The answer may be *nothing*, and if it is, that is
the result rather than a gap.

---

### Phase 6 — Name what an application would need, and build none of it

Requests, verbatim, in the vocabulary of the need rather than of a solution.

---

### Phase 7 — Part B, if it was earned

A repair, whose shape is decided by Phases 1 to 5 and named nowhere above. It stays only if it meets
the criterion the coordination experiment set for its own second half, and atomicity inherited:

> **It removes a state a reader can be misled by, and what the repair replaces survives.** A repair
> that only removes states already refused by name has removed nothing a reader needed, and the honest
> report is that the remedy nobody measured a need for was not built.

And one condition of its own, because this is the first experiment whose remedy could be a lock:

> **A repair that serializes writers must say what it costs a writer that waits, and what happens to
> one that does not come back.** A guard that can be held forever is a guard that can stop the
> application, which is a different failure from the one it prevents.

---

## Success Criteria

1. Every interleaving is produced by an order of calls, and no measurement depends on a scheduler.
2. What each outcome holds is compared **by value** against the base and against each party's whole
   state, never by absence of error.
3. Where the compare-and-append refuses, the coordinate is named; where it does not, the reason it
   does not is stated in terms of what it compares rather than as a gap.
4. What survives of the losing party is stated for every outcome, including when the answer is
   nothing.
5. The set of interleavings is closed, and shown to be closed.
6. Nothing is added to the APE engine.
7. Part B is built only against both criteria above, and its absence is a reportable result.
8. The seven earlier experiments' conclusions stand, or the change is recorded as a result of this one
   — and atomicity's is the one at risk, so it is stated either way.

**Criteria 3 and 8 are the experiment.** The rest is the arrangement holding.

---

## Failure Conditions

The severe one:

> If making a write a compare-and-swap requires the engine to know that knowledge lives in files — a
> handle, a lock, an ordering the ontology has an opinion about — then *storage is the application's
> decision* has met a real friction.

And its neighbour, which is new here:

> If it requires something **outside the record** — a lock file nothing in the repository refers to, a
> service, a clock — then the record cannot defend itself against a second writer, and that is a
> finding rather than a remedy.

And the ordinary ones:

* a measurement that needs a thread to reproduce;
* an outcome reported from the absence of an error rather than from what the record holds;
* a repair chosen before Phase 2 and justified afterwards;
* reporting the compare-and-append as *broken* when it is measured to refuse the ordering it was built
  for;
* treating an interleaving as unlikely instead of saying what it leaves.

**A refuted C4 is the interesting refutation.** If the guard refuses both orderings, then the
application was already safe against a second writer and this experiment is a measurement of a
promise nobody had checked — which is worth having and is a duller result.

---

## Variables Deliberately Left Open

### Whether two writers are one repository's business at all

An application may answer contention by admitting one writer and saying so. That is a coherent answer
and this experiment does not prefer against it — but it is an answer somebody has to give, and the
record currently gives none.

### More than two

Two is the smallest number that can contend. Whether the third is free is not asked.

### What a writer that waits is owed

Opened by the second criterion on Part B, and closed by it only if Part B is built.

### Two repositories meeting

Open since convergence, and untouched.

### Cost

Deferred by eight experiments. A repair that serializes writers would give the term its first
observable price, and that is the nearest this sequence has come to needing a number.

---

## Methodological Constraint

> *Do not introduce an abstraction intended to solve an experiment that has not yet been performed.*

Structure may be introduced only when required by the current experimental procedure. The previous
experiments' conclusions are not revised here; where this one finds something that would have changed
them, it is recorded as a finding of this experiment, against the implementation as it then stood.

And the two rules the last experiments needed. Before recording anything as a finding, ask what would
have to be false for it to be false. Every literal is written before the run, and a wrong prediction is
corrected in the open rather than adjusted.

One more, earned by the last experiment and written down here because it caught a real defect: **a
prediction's justification must quantify over as much as its claim does.** A4 of the atomicity protocol
claimed something about every partial state and argued it from one write order, which is why its
refutation was an entanglement rather than a surprise.

---

## Expected Pressure Points

### "Contention" invites threads, and threads would end the measurement

Every experiment in this sequence is reproducible because it never raced. An interleaving is an order
of calls; a race is a hope about a scheduler. The pressure is to demonstrate the *likelihood* of the
bad ordering, and that is not this experiment's claim: it says what each ordering leaves, and leaves
likelihood to whoever runs an application.

### The guard is not broken, and reporting it as broken would be wrong

`Diverged` refuses the ordering it was built for. What this measures is its **scope** — that its
comparison is against the journal a party read, so it cannot see a journal that moves after the read
and before the write. Coordination reported a repair; this reports where the repair ends.

### The repair the sequence just built is the thing under suspicion

Atomicity's result says a whole write is all-or-nothing against a process that stops. That is true and
this cannot make it false. What it can do is show the sentence is narrower than it sounds — and the
temptation runs both ways: to soften the earlier result, or to defend it. The wording that survives has
to name *which* writer it is atomic against.

### A lost write looks exactly like a successful one

Both parties get `Ok`. There is no state to inspect that says otherwise, which is the same shape
atomicity found for its silent case and the reason that case needed measuring positively. The positive
form here is that the surviving repository **reconstructs and corroborates**: it is not damaged, it is
one party's world with the other's work absent.

---

## Observations

Each observation is recorded as its own numbered document beside this one, as the experiment produces
it. Record facts rather than decisions retroactively presented as inevitable.

Useful observations include: which orderings a guard reaches and which it does not; what a losing
party can find out and from where; where an outcome is one party's whole state rather than a mixture;
and what the record cannot distinguish, stated as the query that has no answer.

Where possible, record the smallest reproducing case.

---

## Open Questions

* Can a repository refuse a second writer using only what the repository holds, or does it take
  something outside it?
* A writer that is refused re-reads and decides again — is the decision it took still the decision it
  would take, and does anything say?
* Is there an interleaving whose result is neither party's state and yet reconstructs, corroborates,
  and answers something **false**?

These are candidates for later experiments. They are not requirements for this one.

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

```text
This one asks whether a promise made to one
writer is a promise at all when there are two.
```

The last experiment made a write whole against everything except another writer, and said so in the
same result. If a compare and a swap are two steps here, then the repository has been keeping a
promise it can only keep alone — and the sequence will have found that out by asking the question its
own answer left behind.
