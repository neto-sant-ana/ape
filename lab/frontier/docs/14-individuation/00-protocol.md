# Individuation

## Abstract

Experiment 13 measured that a reference is fixed by one value and that the value is unverifiable. This
asks the question it left standing: **where should that value be written down?**

```text
11 veracity        no identity carries a recording instant, so two journals can be equal entry
                   for entry and not be the same journal. Closed the one state it found with a
                   comparison in `converge`

12 commensurability  a union of two records is ordered by `recorded_at` whether or not an address
                   carries it, and that ordering is what decides which record loses a decision

13 indexicality    a coordinate and a witness leave two bodies of knowledge standing; adding the
                   instant each witnessed entry was recorded at leaves one. And nothing derives a
                   recording instant, so a completed pin is a claim nobody can weigh
```

Each of the three added the instant somewhere else — at a comparison, at an ordering, at a decision.
The proposal on the queue is to say it **once, at the address**, and let the guards that already exist
carry it.

`identity` appears 463 times across the laboratory, the application and the engine's documents, which
makes it unusable as a handle for the same reason `coordinate` was. **Individuation** is the principle
by which one thing is counted as one and told from another, it appears nowhere, and it is the question
rather than the method: is a journal entry individuated by what it says, or by what it says and when it
was learned?

The predictions are lettered **N**, because `I` belongs to the experiment this one came out of.

---

## Question

**Should an address say when its entry was recorded — and can it, without costing two records the
ability to recognize each other?**

The second clause is the experiment, and it is not a caution attached to a good idea. An address is the
**only** place two records founded apart meet: experiment 09 measured that they share the history they
said the same way, and what they share is a prefix of equal addresses. Two records that learned one
fact on two days would share none.

So the buy and the cost land at the same boundary, and the question is whether that is a trade or an
artefact of one arrangement.

---

## What *the same entry* means here, decided before any phase runs

The three experiments before this one each settled a word first, and each found the definition doing
more work than the phases. The word here is **the same entry**, and it has an operational meaning
already — the Canon's, not this laboratory's:

> **Two admissions are the same entry when admitting either produces the same knowledge, and admitting
> both produces the knowledge of admitting one.**

That is `AppendOutcome::AlreadyPresent`, and idempotent readmission is what identity buys today.

Three weaker things it deliberately does **not** mean:

```text
written the same way   an encoding question. The record already refuses to depend on it —
                       a `WorldRecord` is computed over accessors for exactly this reason

about the same fact    two parties observing one event are about one fact, and 11 measured
                       that they may have learned it on different days. Under today's rule
                       they are one entry; the proposal is that they are two

reached the same way   a coordinate resolves to the first occurrence of an address. That is
                       about a journal's shape, not about what an entry is
```

## The five consequences, which are the instrument

*Same entry* is not a feeling; it is a set of behaviours, and this is all of them. Every phase reads
this table, and the finding is what each cell becomes.

```text
1  idempotence            readmitting is a no-op — `AlreadyPresent`
2  witness membership     which entries a prefix has, as `corroborate` compares them
3  coordinate resolution  where `replay_through` stops
4  journal comparison     `converge::appended`, position by position
5  cross-record meeting   the prefix two records founded apart share
```

The space is closed and it is closed by derivation rather than by listing: these are the five places in
`cli/src` that an `EntryId` is compared. A sixth would be a change to the application, and the phase
that reads the table checks the count.

---

## Hypothesis

```text
the pin completes with no       a witness is already a set of addresses, so an address that
field added                     says *when* leaves experiment 13's I3 satisfied by what the
                                record already writes down, and `Taken` unchanged

and the same change is what     an address is how two records recognize a shared history,
stops two records recognizing   and two records that learned one fact on two days would
each other                      share no address at all
```

If both hold, the address is where the instant does the most work and the most damage, and the trade is
total. If the first holds and the second does not, the row has an answer it has wanted since experiment
01 for the price of a struct field.

---

## Pre-registered predictions

Five, written before anything runs.

**N1 — a composite address completes the pin at the witness, with no field added to `Taken`.**

*Prediction:* experiment 13's stage table, re-run against the new shape, shows the witnessed stage
leaving one body of knowledge where it left two.

*Refuted if* it still leaves more than one — or if the **coordinate alone** already determines, which
would mean the arrangement, and not the change, did the work.

**N2 — and it makes `RecordedDifferently` unreachable.**

*Prediction:* experiment 11's Part B was a second comparison in `appended`, added because the first one
compares addresses and an address could not see a recording instant. With one that can, no state
reaches the second.

*Refuted if* a state reaches it. And a prediction inside the prediction, because it is a cost rather
than a saving: what refuses instead is `Diverged`, which names **two addresses that differ** where
`RecordedDifferently` named **one entry and two instants** — so the trade includes a worse diagnosis,
which is 13's Request 1 arriving as a price rather than as a want.

**N3 — it closes the readmission ambiguity only where the two occurrences were learned on different
days.**

*Prediction:* `ReadmittedEntryIsAmbiguous` exists because a journal can admit one address twice and a
witness is a set. Two occurrences recorded on two days become two addresses; two recorded on one day
stay one.

*Refuted if* it closes entirely, or not at all. Stated as a partial on purpose: a prediction that
claimed the whole would be claiming more than the change can do.

**N4 — and two records founded apart stop sharing a journal, while whether they agree about a world is
left exactly as it was.**

*Prediction:* experiment 09's finding splits. The half `converge` uses — a shared prefix of equal
addresses — dies for any two records that learned anything on different days. The half `holds` answers
— agreement about a world, by `ThesisId` — is untouched **by this change**, because a `ThesisId` is
derived from a cut and a selection and neither carries a recording instant.

*Refuted if* either half fails. **This is the cost, and N1 with N4 is the experiment.**

**N5 — because an address is the only place two records meet, and a world is not.**

*Prediction:* stated as the reason so it can be wrong separately. `converge::holds` is the only
operation in the application a second repository is answerable to, and it takes an identity rather than
a journal. Everything else that crosses between two records crosses as an address.

*Refuted if* something else two records share turns out to be affected, or if the world-level agreement
moves.

---

## What is carried forward, and is not available as a finding

* **No identity carries a recording instant** — 11.
* **Two records founded apart agree about a world exactly where they agreed about the knowledge under
  it, and are refused by the journal compared entire** — 09.
* **A union is ordered by `recorded_at`, and that ordering decides which record loses a decision** —
  12.
* **The pin determines the reference at the stage that carries recording instants and not before** —
  13.
* **A recording instant is derived from nothing, so no record weighs it — not the receiver, and not
  the record it came from** — 13. *A composite address does not change this, and a phase reporting it
  as a finding would be reporting the thing that is carried forward.*
* **Changing what decides an identity does not make an older repository disagree; it makes it stop
  replaying** — `core/src/kernel/entities/identification.rs`.

Before any sentence here is allowed to be a finding, ask what would have to be false for it to be
false.

---

## Motivation

It goes now because both axes the queue names agree on it, for the second time running, and because it
is the last item on the board that would change what a type in the record **is**.

**By ripeness** it has two arrivals, 11 and 13, which is fewer than three other items have. **By what
it costs if deferred** it is first, and by a distance: everything built on the present `EntryId` becomes
rework, and the row's ripest item — *saying two lines agree*, named by five results — sits directly on
top of it, because a union is ordered by the very value this proposes to move.

And it goes now because for the first time an item arrives with its **subject already built**.
Experiment 13's arrangement is exactly this experiment's case: two journals equal address for address
with one instant different, a reordering, an insertion, and a record founded apart.

---

## Experimental Boundary

This experiment asks where a recording instant should be written down, and what moving it there costs.

It includes:

* measuring the five consequences under the present shape, re-measured here rather than inherited;
* **building the composite address in `cli/src`**, on this branch, because the buy cannot be measured
  without it;
* measuring the same five under the new shape, as one table with two columns;
* the blast radius, counted rather than estimated: which suites stop compiling, which stop reading, and
  what a refusal is called afterwards;
* the decision, and a revert if the table says revert.

It deliberately excludes:

* **the engine.** A composite `EntryId` is the application's — `EntryId` is declared in
  `cli/src/journal.rs` and built out of an entity id. The kernel keeps content-addressing, and an
  experiment that reached `define_entity!` would be a different one;
* **a migration, a version tag, a tolerant reader.** Those are designs for a problem the phases have
  not measured yet, and naming one here would pre-decide the answer;
* **whether a recording instant can be trusted.** 13 measured that it cannot be checked by anybody.
  Moving it does not change that, and a phase suggesting otherwise is wrong;
* **more than two records, the network, a transport.** Excluded throughout the row.

---

## Experimental Subject

**Experiment 13's, extended by one entry, and the extension is load-bearing.**

Reusing a concluded experiment's *arrangement* is not reusing its subject file: this experiment builds
its own, and 13's stays where it is. What is inherited is the shape, and the shape already produces
four of the five cases.

What must be added:

```text
a second instant that varies,   13 varied the recording instant of the entry the coordinate
and it is NOT the coordinate's  names. Under a composite address the coordinate alone would
                                then determine the reference, and N1 would be confirmed by
                                the arrangement rather than by the change
```

That is the one place this subject could make the experiment un-failable, and it is why N1 names it as
a refutation condition rather than leaving it to be noticed.

What it must also express, and 13's already does:

```text
two records founded apart       same content, and therefore the same addresses today. What
with the same content           they share is what N4 says the change takes away

a readmission                   one address admitted twice, on one day and on two, because
                                N3 predicts the change separates only the second

a reordering and an insertion   the two candidates that are about neither instants nor
                                addresses, kept so the table has rows the change should not move
```

---

## Initial State

```text
the present shape          `EntryId` is the hex of an entity id
five consequences          measured under it, as literals
the arrangement            13's, plus the second varying instant and the two founded apart
the committed records      `lab/agents/04-multiagent/run-*/repo`, read by a live suite
```

---

## Procedure

### Phase 0 — The five consequences under the present shape

Read once, as literals, before anything is built. Every one of them is a behaviour some earlier result
already reported; every one is re-measured here, because a table with an inherited column is a table
whose columns were not taken the same way.

### Phase 1 — The subject, and the second varying instant

The arrangement, and the assertion that the coordinate alone does **not** determine the reference under
either shape. Without it N1 cannot fail.

### Phase 2 — What the change would touch, counted before it is made

Every place in `cli/src` an `EntryId` is compared, constructed or serialized, and every place in the
workspace one is read from a file. Counted, named, and written down — so that what the build actually
touches can be weighed against what was predicted, and a surprise is visible as a surprise.

### Phase 3 — Building it

The composite address, in `cli/src`. This is the experiment's Part B and it is in the middle rather
than at the end, because the buy cannot be measured without it.

**A revert is an outcome of this experiment, not a failure of it**, and saying so before the build is
what keeps Phase 7 from arguing past its own table.

### Phase 4 — The same five consequences under the new shape

One table, two columns. N1, N2, N3, N5.

### Phase 5 — The blast radius, counted

What stopped compiling, what stopped reading, and what a refusal is called now. Including the thing
Phase 2 will have found: this workspace holds **committed repositories** — a concluded agents-row
experiment's published artefacts, read as data by a live suite — and a change to what an address is
makes them stop replaying rather than disagree.

### Phase 6 — Two records founded apart

N4, both halves. What `converge` does, and what `holds` says, before and after.

### Phase 7 — The decision

Keep or revert, decided by the table and against the condition below. Whichever it is, the reason is
the table rather than a preference, and the losing side is written down.

### Phase 8 — What the answer leaves for the queue

Requests, verbatim, in the vocabulary of the need — and, if the answer is revert, where the instant
goes instead, since three experiments have now put it somewhere different.

---

## The condition on keeping it

Every Part B in this row has been a repair that had to earn its place. This one is not a repair: the
change **is** the subject, so the condition is inverted and written before anything runs.

> **It is kept only if the table shows the buy without the cost, or shows the cost being one nothing is
> relying on.** A buy that is real and a cost that is real is not a reason to keep it — it is a trade,
> and a trade is reported rather than taken.

And its neighbour, which is this experiment's own:

> **A cost that lands on a published result is not a cost this experiment may pay.** The agents row's
> committed repositories belong to a concluded experiment, and `lab/README.md` says a concluded
> experiment keeps its own. If the change makes them unreadable, keeping it means either breaking a
> published result's runnability or editing a subject that is not this experiment's — and both are
> refused, whatever the rest of the table says.

---

## Success Criteria

1. *The same entry* is defined before any phase runs, in the Canon's terms rather than this
   laboratory's, and the three weaker readings are each produced rather than asserted.
2. The five consequences are **derived** as the places an `EntryId` is compared, not listed — and the
   count is checked.
3. Both columns of the table are measured the same way, with neither inherited from an earlier result.
4. The arrangement cannot confirm N1 by itself, and the phase that says so runs before the build.
5. The blast radius is **counted**, including what is read from a file, and weighed against Phase 2's
   prediction.
6. Every literal is written before the run.
7. Nothing is added to the APE engine.
8. The decision cites the table, and a revert is reported as a result with the same weight as a keep.
9. The thirteen earlier conclusions stand, or the change is recorded as a result of this one.
   Experiment 09's is the one at risk, and N4 says how.

**Criteria 4 and 8 are the experiment.**

---

## Failure Conditions

The severe one, and it is not about the change being wrong:

> If the change is kept and N4 holds, then the row bought a portable decision by closing the item five
> results have named. *Saying two lines agree* needs two records to share a prefix, and this would end
> that for any two that learned anything on different days. That is reported as a **result about the
> queue**, and every item that depends on two records meeting is listed.

Its neighbour:

> If the change makes `lab/agents/04-multiagent/run-*/repo` unreadable, then this workspace holds
> records written by one version of a type and read by another — and `cli/src/lineage.rs` says in as
> many words that it does not. That note was written about `Taken.by` and it was right when it was
> written; a finding here would be that it stopped being right, and where.

And the ordinary ones:

* the buy measured against an arrangement that could have produced it without the change;
* a column inherited from 09, 11 or 13 rather than re-measured;
* *the cost is acceptable* reported where what was measured is *nothing in this workspace pays it*;
* a migration invented mid-phase to make the blast radius smaller, which is designing for a problem
  before measuring it;
* keeping the change because it is built, which is what putting the build in the middle risks and why
  the condition above is written first.

**A revert is a real outcome and possibly the likeliest.** Three results put the instant in three
different places and none of them chose the address; that is either three near-misses or three correct
decisions, and this is the experiment that says which.

---

## Variables Deliberately Left Open

### Whether an old record should still be readable

A tolerant reader, a version tag, a migration. Designs for a problem the phases have not measured, and
the one place this protocol most wants to jump ahead.

### Where the instant goes if the address is the wrong place

Phase 8, and only if the answer is revert.

### Whether two records should be able to meet at all

The row has assumed it. N4 measures the price of not being able to, and does not decide it.

### Cost

Deferred by fourteen experiments.

---

## Methodological Constraint

> *Do not introduce an abstraction intended to solve an experiment that has not yet been performed.*

The previous experiments' conclusions are not revised here; where this one finds something that would
have changed them, it is recorded as a finding of this experiment, against the implementation as it
then stood.

And the rules the last eight experiments earned, which apply without amendment. Before recording
anything as a finding, ask what would have to be false for it to be false. Every literal is written
before the run, and a wrong prediction is corrected in the open rather than adjusted. A prediction's
justification must quantify over as much as its claim does. A friction is evidence of a want, not of a
need. A phase must not be satisfied by an arrangement it could not have failed. A negative result must
name the space it searched. A capability several results asked for is still a want. A guard whose two
halves read the same derived value must assert the mutation before it reads.

---

## Expected Pressure Points

### The build in the middle will want to be kept

Nothing in this row has built first and judged after, and the reason is exactly this. Code that exists
argues for itself: the diff is done, the suite is green, and reverting looks like waste. The condition
on keeping it is written above the procedure for that reason, and Phase 7 cites the table or it cites
nothing.

### *The cost is acceptable* will be reached by measuring only this workspace

Nothing outside this repository reads an APE repository, so every compatibility cost measurable here is
a cost this laboratory pays to itself. That makes the measurement easy and the conclusion narrow, and
the honest report says *nothing in this workspace pays it* rather than *it is free*.

### The blast radius will be counted where it is cheap to count

Compilation errors are countable and land in a minute. What is read from a file is countable and does
not, because a suite that reads a committed repository fails at runtime and only if it is run. Phase 2
counts both before the build precisely so that the second is not discovered by its absence.

### N4 is the prediction most likely to be argued into a technicality

*They still agree about a world* is true and is not the half that matters — `converge` is what an
application calls, and `converge` compares journals. A phase that reported the surviving half as though
it answered the question would be reporting the half nothing uses.

---

## Observations

Each observation is recorded as its own numbered document beside this one, as the experiment produces
it. Record facts rather than decisions retroactively presented as inevitable.

Where possible, record the smallest reproducing case.

---

## Open Questions

* Is *when I learned this* a property of the fact, or of my having it? The record currently says the
  second, by putting it in an envelope rather than in an identity.
* If two records cannot share a prefix, is there anything else they could share that is not a world?
* Does a repository owe its own past readability, and is that a property of the record or of the
  laboratory that keeps it?

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

```text
This one asks whether two records that learned
the same thing on different days learned one thing.
```

Experiment 09 answered *yes* and made it the ground two records stand on. Experiment 11 found the state
where that answer is false and closed it with a comparison. Experiment 13 found that the value the
comparison is about is the one thing a reference needs and nothing can check. This asks whether the
answer should have been *no* all along — and if it should, what the row loses by saying so.
