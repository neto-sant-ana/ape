# Atomicity

## Abstract

A repository is three files, written one after another. Corroboration refuses a repository whose
witnesses disagree, so a reader is protected from misreading one — and the coordination experiment said
plainly what that leaves open:

> What is measured is that a torn repository is **refused**, not that it cannot occur. An atomic
> commit — a rename, a lock — is a question this experiment is arranged to be able to ask and does not
> answer.

It named atomic commit as the nearest unanswered question. This asks the question that has to come
first, because the answer decides whether the remedy is the one that was named.

> *When a writer is interrupted, what does the record lose — and does it say so?*

---

## Question

**Is being refused the same as being safe?**

Six experiments have measured refusals and treated each one as a success, and they were right about
what they measured: a reader that cannot be misled is the whole of what corroboration promises. But a
refusal is a statement about the **reader**. Nothing in this sequence has yet asked what it says about
the **record**.

Three files, written in order. Interrupt between any two and the repository on disk is a prefix of a
commit nobody finished:

```text
journal written, lineage and worlds not
journal and lineage written, worlds not
nothing written
```

The last is not a case — a write that never began leaves the repository as it was. The first two are,
and this experiment is about whether they behave alike.

---

## Hypothesis

```text
divergence      the partial states do not agree with each other: one is refused by name,
                and one reconstructs

exposure        being refused protects the reader and not the record, so every partial
                state costs the repository that existed before the write
```

The first is the measurement. The second is the claim, and if it holds, *atomic commit* stops being a
durability nicety and becomes the only thing standing between an interruption and a repository nobody
can read or roll back.

---

## Pre-registered predictions

Five, written before anything runs, each with the observation that would refute it. Kept from the
exploration protocol because this one also produces outcomes that are easy to read backwards into
whatever they turned out to be.

**A1 — A truncated commit is three cases, and they do not agree.**

*Prediction:* the journal written alone **reconstructs**, because nothing refers to the lineage and a
journal holding entries no decision witnesses is admitted after the last decision like any other. The
lineage written without the journal it addresses is **refused by name**, at the entry a decision was
taken after. The lineage written without its worlds is **refused by name**, at the length.

*Refuted if* the three behave alike, or if the silent one is refused.

**A2 — The silent case is the one that loses intention.**

*Prediction:* the only partial state that reconstructs is the one where decisions were lost and
knowledge kept. Which is the coordination experiment's asymmetry — *nothing refers to a decision that
nothing extends* — arriving as a **durability** property rather than a concurrency one.

*Refuted if* a silent case loses knowledge, or if losing intention is refused.

**A3 — The order the three files are written in decides which case an interruption produces.**

*Prediction:* writing the lineage **before** the journal turns the silent case into a refused one, and
nothing else about the application changes. So the current order is not neutral: it is the order whose
interruption is undetectable.

*Refuted if* reversing the order leaves the same set of reachable outcomes.

> If A3 holds it is the cheapest finding in this sequence, and it is **not** atomicity. Two lines
> reorder a write; a rename is a mechanism. An experiment that reached for the named remedy without
> measuring this first would have paid for the expensive answer to a question the cheap one closes.

**A4 — Being refused is not being safe.**

*Prediction:* in every partial state, including the two that are refused, the repository that existed
before the write is **gone** — the files are written whole, so the previous journal is overwritten
before the lineage is. A refusal therefore reports that the repository cannot be read, and offers
nothing to read instead.

*Refuted if* any partial state leaves something the previous repository can be recovered from.

**A5 — Atomicity's whole value is the case corroboration cannot see.**

*Prediction:* an all-or-nothing commit removes all three partial states. Two of them were already
refused, so removing them buys a reader nothing it did not have; what it buys is the third. And it
does not touch A4 — a commit that is atomic against a reader is not thereby recoverable, unless what
it replaces is kept.

*Refuted if* removing a refused state changes what a reader can answer.

**A2 with A4 is the experiment.** Each alone is a measurement. Together they say whether the record
protects itself or only its readers.

---

## What is carried forward, and is not available as a finding

Usable in reasoning, and not reportable as a result:

* **A repository holds two derived witnesses and compares both on every read**, and refuses six
  tampered repositories out of six.
* **A refusal names a coordinate**, because half a refusal sends a reader back to the bytes.
* **Each file is written whole.** Nothing appends; a write replaces.
* **Knowledge appends and a decision addresses it** by the entry that stood when it was taken, so a
  journal that lost entries makes standing decisions disagree with it.
* **Nothing refers to a decision that nothing extends.** The lineage refers to the journal and nothing
  refers to the lineage.
* **A party that cannot converge writes nothing**, because the merge is rebuilt in memory first.
* **Pruning is writing the files**, and a pruned repository is byte-identical to one that never
  explored.

Before any sentence here is allowed to be a finding, ask what would have to be false for it to be
false. A founding premise makes it a corollary; a documented design decision makes it a reading; an
implementation fact whose composition is unstated makes it a finding.

---

## Motivation

Every experiment in this sequence has been a reader's experiment. Reconstruction asked whether a reader
recovers meaning; corroboration, whether a reader can tell a repository is wrong; coordination, whether
a reader sees both parties. The repository has been the thing under test and the reader has been the
one served.

A writer that is interrupted is the first arrangement in which the repository is the thing at risk. And
it is the one an operator meets: a process killed, a disk full, a container stopped between two
`fs::write` calls. None of that needs concurrency, which is why it can be asked now.

**This runs before the remedy** for the reason the exploration protocol gave: work that closes a gap
makes the gap unmeasurable. An application that writes atomically ends any question about what an
interruption costs, and the finding becomes the reason that work exists rather than a casualty of it.

---

## Experimental Boundary

This experiment exercises one application writing one repository, interrupted between files.

It includes:

* every prefix of the write sequence, produced deterministically;
* what a fresh process makes of each, by value and not by absence of error;
* the same prefixes under a reversed write order;
* what survives of the repository that existed before the write.

It deliberately excludes:

* **concurrency.** An interruption is a **prefix** of a write sequence, and a prefix is deterministic —
  the same insight that let the coordination experiment produce a lost decision by interleaving rather
  than by racing. Nothing here needs two threads, and a measurement that did would be reproducible
  nowhere;
* **power loss and `fsync`.** Whether bytes handed to the operating system reach the platter is the
  operating system's question. What is measured here is what happens when a *program* stops between two
  writes it intended to make;
* **the remedy's mechanism.** A rename, a lock, a write-ahead file, a single file holding all three —
  naming one before Part A would be choosing the answer. Part B builds a repair only if Part A shows
  which state needs removing, and the shape follows from that;
* **authenticity**, unchanged since corroboration and now gathered as a candidate of its own;
* **the engine.** Storage is the application's decision, and an interruption is a property of files.

Those concerns may become later experiments. They must not influence the structure introduced here
unless this experiment itself requires them.

---

## Experimental Subject

**A subject of this experiment's own**, for the reason the six before it give.

What it must express, and no previous subject needed:

```text
all three files non-trivial   a journal with entries no decision witnesses, and more than
                              one decision, so a truncated lineage is a different length
                              rather than an empty one
a repository worth losing     a state that a reader can name, so that "the previous
                              repository is gone" is a measurement and not a shrug
```

Everything else stays as thin as the others: integers, one quantifiable resource, no dependencies
unless the procedure demands them.

The instrument is **not** part of the subject. Interruption is performed by writing the files the
application would write, stopping after each — the same technique corroboration used to tamper and
exploration used to prune. The difference between tampering, pruning and being interrupted is intent,
and the repository cannot tell any of the three apart. That is worth stating before it is measured.

---

## Initial State

```text
repository   written whole, and read once to establish what it answers
prefixes     none produced
```

Nothing is inherited from the previous experiments except their conclusions and their code.

---

## Procedure

### Phase 0 — What the whole repository answers

Read once, and record it. Every later phase compares against this, so a partial state that answers
*differently* is distinguishable from one that answers *less*.

---

### Phase 1 — Interrupt after the journal

The first prefix. Measured by value: does it reconstruct, and if it does, what does it answer that
Phase 0 did not — or fail to answer that Phase 0 did?

A2 says this is the silent one. Silence is measured **positively**: not by failing to find an error,
but by naming what the reading now omits and showing the repository agrees with itself about it.

---

### Phase 2 — Interrupt after the lineage

The second prefix, and the one A1 says is refused. Which refusal, at which coordinate, in the
application's own words.

---

### Phase 3 — What is left of what was there

A4. For each prefix, what remains of the repository Phase 0 read: what is recoverable, from what, and
by whom. The answer may be *nothing*, and if it is, that is the result rather than a gap.

---

### Phase 4 — The same prefixes, written in the other order

A3. The lineage before the journal, and nothing else changed. Which outcomes become reachable and which
stop being.

---

### Phase 5 — Name what an application would need, and build none of it

Requests, verbatim, in the vocabulary of the need rather than of a solution.

---

### Phase 6 — Part B, if it was earned

A repair, whose shape is decided by Phases 1 to 4 and named nowhere above. It stays only if it meets
the criterion the coordination experiment set for its own second half, restated here before it can be
argued with:

> **It removes a state a reader can be misled by, and the repository before an interrupted write
> survives.** A repair that only removes states already refused by name has removed nothing a reader
> needed, and the honest report is that the remedy nobody measured a need for was not built.

---

## Success Criteria

1. Every prefix of the write sequence is produced deterministically, and no measurement depends on a
   race.
2. What each prefix answers is compared **by value** against the whole repository, never by absence of
   error.
3. The silent case, if there is one, is measured positively — by what the record says, not by what a
   reader failed to find.
4. What survives of the previous repository is stated for every prefix, including when the answer is
   nothing.
5. The write order is a measured variable rather than a detail, and the outcomes reachable under each
   order are a closed set.
6. Nothing is added to the APE engine.
7. Part B is built only against the criterion above, and its absence is a reportable result.
8. The six earlier experiments' conclusions stand, or the change is recorded as a result of this one.

**Criteria 3 and 4 are the experiment.** The rest is the arrangement holding.

---

## Failure Conditions

The severe one:

> If making a commit atomic requires the engine to know that knowledge lives in files — a flush, a
> transaction, a handle, an ordering the ontology has an opinion about — then *storage is the
> application's decision* has met a real friction.

And the ordinary ones:

* a measurement that needs a race to reproduce;
* a silent case reported from the absence of an error rather than from what the record says;
* a repair chosen before Phase 1 and justified afterwards;
* a claim that a refusal is safety, without saying what it is safe *for*;
* treating the write order as an implementation detail after Phase 4 measured that it decides the
  outcome.

**A refuted A1 does not stop the experiment** — three cases behaving alike is a result, and a duller
one. A refuted A4 is the interesting refutation: it would mean the record protects itself and nobody
had noticed.

---

## Variables Deliberately Left Open

### Whether the three files should be one

A single file holding journal, lineage and worlds makes a write atomic by construction on most
filesystems, and gives up reading the repository by eye — which the repository module chose
deliberately and called a decision about this experiment rather than about APE. Measured here, not
decided.

### What a repository owes a writer

Every promise in this sequence is to a reader. Whether a record owes anything to the process writing it
is the question this experiment opens and does not close.

### Two repositories meeting

Open since convergence, and untouched.

### Cost

Deferred by seven experiments, and now with a shape: exploration measured which term dominates. How
long a replay takes on a real history still needs a history nobody has.

---

## Methodological Constraint

> *Do not introduce an abstraction intended to solve an experiment that has not yet been performed.*

Structure may be introduced only when required by the current experimental procedure. The previous
experiments' conclusions are not revised here; where this one finds something that would have changed
them, it is recorded as a finding of this experiment, against the implementation as it then stood.

And the two rules the last experiments needed. Before recording anything as a finding, ask what would
have to be false for it to be false — a founding premise makes it a corollary, a documented decision
makes it a reading. Every literal is written before the run, and a wrong prediction is corrected in the
open rather than adjusted.

---

## Expected Pressure Points

### "Refused" reads as "safe", and this experiment is about the gap between them

Six results report refusals as successes. They were right, and the wording is load-bearing: a refusal
is a promise to a reader. Reporting it as a promise to the record is the mistake this experiment exists
to make impossible, and the pressure runs the other way too — concluding that a refusal is *worthless*
because it does not save the repository would be as wrong.

### The silent case is an absence again

Exploration met this and the answer was to measure positively. Here the positive form is that the
repository **agrees with itself** about a smaller world: it reconstructs, it corroborates, and the
reading it produces omits something Phase 0 can name.

### The write order looks like a fix and is not

If A3 holds, reversing two lines converts silence into refusal. That is a real improvement and it is
not atomicity: it changes *which* partial state is reachable, not whether one is. Presenting it as the
answer would close the question the remedy was named for.

### The instrument is indistinguishable from tampering and from pruning

Writing a prefix of the files is what corroboration did to tamper and exploration did to prune. Three
intents, one mechanism, and the repository cannot tell them apart — which is the authenticity candidate
appearing for the sixth time, and is not this experiment's to answer.

---

## Observations

Each observation is recorded as its own numbered document beside this one, as the experiment produces
it. Record facts rather than decisions retroactively presented as inevitable.

Useful observations include: what a partial state answers that a whole one does not; what a refusal
protects and what it does not; where an order decides an outcome; and what a repository cannot
distinguish, stated as the query that has no answer.

Where possible, record the smallest reproducing case.

---

## Open Questions

* What would a repository have to keep in order to be rolled back, and is keeping it compatible with
  *nothing derived is persisted unless something compares it*?
* Does a writer's interruption belong to the application or to the repository format?
* Is there a partial state that is neither refused nor silent — one that reconstructs and answers
  something **false**?

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

```text
This one asks who the record's promises are
to — and what is left of it when the writer
does not finish.
```

Every promise measured so far has been to a reader, and every one of them held. What this asks is
whether a record that protects its readers perfectly protects **itself** at all.

If it does, atomic commit is a convenience and the sequence can say so.

If it does not, then six experiments' worth of refusals have been guarding the wrong side of the door,
and the cheapest thing that would have helped is two lines in the order the files are written.
