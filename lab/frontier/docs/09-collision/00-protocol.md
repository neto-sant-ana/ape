# Collision

## Abstract

Every experiment in this row has met through **one** repository. Divergence and convergence put two
lines in one; coordination put two writers on one; contention put two writers in one generation of one.
The medium was always shared, and every question about reaching each other was a question about what
that medium held.

This one removes the medium.

Two repositories, each whole, each holding a lineage nothing outside it refers to. Neither was copied
from the other, neither has read the other, and nothing in either one names anything in the other. The
question every previous experiment could take for granted is the only thing left:

> *Can two repositories meet, when nothing in either of them refers to the other?*

It has been open since convergence, and named by every result since. Five of them say *untouched*.

---

## Question

**What must two repositories already have in common in order to meet, and is any of it a thing either
one was told?**

The distributed system this engine is shaped after answers with an object: a merge-base, a commit both
sides can name, found by walking two histories until they touch. APE has no such walk and no such
object. What it has instead is that **every identity is derived from content** — an entry, a
commitment, a world.

So there are two possible answers and they are not variations of each other:

```text
a common ancestor is a thing a record HAS      → two repositories can only meet if one of
                                                 them descends from the other, or something
                                                 outside both holds what they share

a common ancestor is a thing a record IS       → two repositories that admitted the same
                                                 knowledge already hold the same identities,
                                                 and the ancestor is derived rather than found
```

If the second holds, this row has been carrying a distributed capability for nine experiments without
exercising it once. If the first holds, *two repositories meeting* is not a question about the record
at all and never was.

---

## Hypothesis

```text
knowledge does not meet     two journals that diverge are refused by name, and the operation
                            that refuses them is the only one the application has

intention meets for free    a world's identity is derived from its parent, its cut and its
                            selection, all of them content-addressed — so two repositories
                            agree about a world without either being asked

and the second is           the cut resolves against the Event chain that stood, so agreeing
conditional on the first    about a world requires having agreed about the knowledge under it
```

The third line is the experiment. The two halves are in tension by construction: the free agreement
depends on exactly the thing the refusal is about. Either that tension collapses — meeting reduces to
*did they admit the same things*, and content addressing answers it — or it does not, and there is a
case where two repositories agree about a world over knowledge that is not the same. That case is
[`candidates/01-veracity`](../../../candidates/01-veracity.md)'s shape, arriving somewhere neither of
its protocols was looking.

---

## Pre-registered predictions

Five, written before anything runs, each with the observation that would refute it.

**C1 — Knowledge does not meet, and the refusal measures what was shared.**

*Prediction:* the only operation the application has for putting two lines together is `converge`, and
its comparison is against the journal on disk. Two repositories are, by construction, journals that
diverge — neither read the other. So a meeting is refused by name, `Diverged`, and the **position**
names how many entries the two happened to share: at the first entry that differs for two that share a
base, and at **0** for two founded independently.

*Refuted if* a meeting is not refused, or if the position does not correspond to the shared prefix.

**C2 — A shared base is shared by content, and nothing copied it.**

*Prediction:* two repositories founded from the same admissions hold **byte-identical entry
identities**, without either being cloned and without any operation being run between them. An
`EntryId` is derived from what admitting produced, so sameness is a property of what was said rather
than of where it came from.

*Refuted if* two repositories founded identically hold different entry identities, or if producing a
shared base requires copying one repository into the other.

> If C2 holds, the merge-base is not missing. It is derived, and it was derived by the reconstruction
> experiment's first observation without anybody noticing what it would later be for.

**C3 — Intention cannot meet before knowledge does.**

*Prediction:* a `Thesis` is identified by `(parent, cut, selection)`. A `KnowledgeCut` is
`(known_at, event_head)` and the head is **resolved against the Event chain that stood**, so two
repositories whose Events differ resolve different cuts for the same instant and produce different
worlds from the same decision. Agreement about a world therefore implies agreement about the knowledge
beneath it, and the free merge is conditional on what C1 refuses.

*Refuted if* two repositories agree about a world whose knowledge they do not agree about — which is
the interesting refutation and is measured, not argued.

**C4 — A world's identity does not pin the journal it came from.**

*Prediction:* what a world's identity is derived from is its parent, its cut and its selection —
**not** its journal. So two repositories can hold the same world over journals that differ, provided
the difference is in entries the world neither selects nor cuts at. The identity is honest about what
it names and says nothing about the rest.

*Refuted if* every difference between two journals changes every world they produce.

**C5 — The application can express a meeting only by making one of them a party.**

*Prediction:* `converge(repository, held)` takes a repository and a **working copy**, and a repository
read back is exactly a working copy — so `converge(a, corroborated(b))` is the whole of what the
application can say. Which makes a meeting **asymmetric**: on success `a` holds both lines and `b` is
untouched and does not know. There is no operation whose subject is two repositories.

*Refuted if* the application has a symmetric form, or if the asymmetric one turns out to be
sufficient for a reason this experiment can state.

**C1 with C3 is the experiment.** Each alone is a measurement. Together they say whether *two
repositories meeting* is a question the record can answer at all, or a question that dissolves into
*did they admit the same things*.

---

## What is carried forward, and is not available as a finding

Usable in reasoning, and not reportable as a result:

* **An identity is derived from content**, for every entity the engine has.
* **Knowledge appends**, and a journal whose earlier entries moved makes standing decisions disagree
  with it — refused by name, at the entry where the two disagree.
* **Intention merges.** Two decisions cannot contradict one another, so the union of two parties'
  decisions is a lineage in the same sense either party's was.
* **The merge compares against the repository, not against what a party read**, and it re-reads at the
  moment of writing.
* **A party that cannot converge writes nothing.**
* **A `Thesis` does not deserialize.** An archive is never opened, only rebuilt.
* **A transfer is asked *of* a record and arrives from outside it**, which is convergence's finding
  and the nearest thing this row has to an operation between two records.
* **A cut is not reproducible from its date alone**, where something was recorded at that instant
  after the decision was taken.

Before any sentence here is allowed to be a finding, ask what would have to be false for it to be
false. A founding premise makes it a corollary; a documented design decision makes it a reading; an
implementation fact whose composition is unstated makes it a finding.

---

## Motivation

The candidate has been inherited five times and acted on never, which is the failure a queue exists to
stop. But the reason to run it **now** rather than next is that the previous experiment supplied the
prediction.

Contention measured that the merge accepts **extension** and refuses **divergence**, and named the
coordinate of the refusal. Two repositories are divergent by construction: neither read the other, so
neither journal extends the other. That makes C1 a consequence of a published result rather than a
suspicion — and it means the operation the application has for putting two lines together **refuses
the shape two repositories always have**, which is a sentence somebody should have to either confirm
or take back.

And the queue found a second reason. The remaining half of
[`candidates/02-scale`](../../../candidates/02-scale.md) turns on what a reader does with two
repositories that disagree about a unit, so that candidate cannot be settled before this one runs. It
is the only item in the queue that another item waits on.

**This runs before any remedy**, for the reason the exploration protocol gave and two experiments have
reused: work that closes a gap makes the gap unmeasurable.

---

## Experimental Boundary

This experiment exercises two whole repositories, founded independently in one process, and the
operations the application already has.

It includes:

* three relations two repositories can stand in — **disjoint**, **shared base with divergent tails**,
  and **one extending the other** — and what each makes of a meeting;
* whether a shared base is shared without anything copying it, measured by identity;
* whether two repositories produce the same world from the same decision, and under what condition;
* what a world's identity does and does not pin;
* what the application can express, stated as the operation that exists and its asymmetry.

It deliberately excludes:

* **a network, a protocol, a transport.** Two directories in one process, and the phase decides which
  is read. A measurement that needed a socket would be measuring a socket;
* **a clone operation.** Nothing in the application copies a repository, and adding one would answer
  C2 by construction — the arrangement founds both from the same admissions instead, which is the
  whole point of C2;
* **threads.** Contention settled that contention is an order of calls; nothing here needs less than
  that;
* **the remedy's mechanism.** A fetch, a symmetric merge, a reference to another repository, an
  authority — naming one before Part A would be choosing the answer;
* **more than two repositories.** Two is the smallest number that can meet;
* **authenticity**, unchanged and gathered as a candidate of its own — and it will be tempting here,
  because a second repository is the first thing in nine experiments that could be *somebody else's*;
* **the engine.** Storage is the application's decision, and so is what a second repository is.

Those concerns may become later experiments. They must not influence the structure introduced here
unless this experiment itself requires them.

---

## Experimental Subject

**A subject of this experiment's own**, for the reason the nine before it give.

What it must express, and no previous subject needed:

```text
two foundings, from one       both repositories are built by the same construction, so that
statement of the subject      a shared base is shared BECAUSE the admissions were the same
                              and not because the arrangement copied anything

three relations, on demand    disjoint, shared-base-divergent, and extending — the same two
                              repositories in three configurations, so that what is reported
                              about a meeting is reported about all three

one decision, taken twice     the same decision taken independently in both, so that C3 has
                              something to compare: two worlds that should be one identity

and one difference a world    knowledge admitted in one and not the other, which the world
cannot see                    neither selects nor cuts at — C4's whole case
```

Everything else stays as thin as the others: integers, one quantifiable resource, no dependencies
unless the procedure demands them.

The instrument is **which repository is read as a working copy**, and nothing else. Where a phase needs
a relation the application cannot produce, it founds the two repositories differently rather than
editing one — because a repository edited from outside is a tampering, and five experiments have that
instrument for a different question.

**One consequence of the instrument, stated because it is a trap.** `converge` **writes** on success.
So a probe is not read-only, and a phase that reused a repository across two probes would be measuring
the first probe. Every phase founds what it reads.

---

## Initial State

```text
two repositories   each whole, each written by the application, each read once to establish
                   what it answers alone
nothing between    no reference, no copy, no operation run — and nothing in either one that
                   could name the other if it wanted to
```

Nothing is inherited from the previous experiments except their conclusions and their code.

---

## Procedure

### Phase 0 — What each repository answers alone

Read both, and record both. Every later phase compares against these, so a result that is one
repository's state is distinguishable from the other's, from their union, and from something neither
holds.

---

### Phase 1 — The probe, in three relations

C1 and C5. `converge(a, corroborated(b))`, for each of the three relations, and for both directions —
because C5 says the operation is asymmetric and an asymmetry is measured by running it both ways.

What is recorded is the refusal and its **coordinate**, or the merge and what it produced; and in every
case what happened to the repository that was *not* the subject.

---

### Phase 2 — The base nobody copied

C2. Two repositories founded from the same admissions, and the entry identities compared by value. Then
the same comparison after each has gone on to admit something of its own, because a shared prefix that
stops being shared when either side grows would be a shared prefix in name only.

Measured by identity rather than by bytes, deliberately: two files could differ in encoding and still
hold the same knowledge, and it is the knowledge that has to be the same for a meeting to mean
anything.

---

### Phase 3 — One decision, taken twice

C3. The same decision taken independently in both repositories, and the two worlds compared **by
identity**. Then again with the Event chains made to differ, which is the condition the cut resolves
against — so that what is reported is not *they agree* but *they agree exactly when this is true*.

---

### Phase 4 — What a world's identity does not pin

C4. Knowledge admitted in one repository and not the other, chosen so that no world selects it and no
cut names it. Whether the worlds stay identical, and what that means for a reader holding one world and
two journals.

If they stay identical, enumerate what kinds of difference are invisible to a world's identity. The
claim is about which differences, so a phase that measured one would be reporting a sample as a
closed set.

---

### Phase 5 — What the application can express, and what it cannot

C5, stated positively. Not *there is no operation* — but which sentence about two repositories the
application can say, which it cannot, and what a caller would have to do to get the missing one out of
what exists.

---

### Phase 6 — Name what an application would need, and build none of it

Requests, verbatim, in the vocabulary of the need rather than of a solution.

---

### Phase 7 — Part B, if it was earned

A repair, whose shape is decided by Phases 1 to 5 and named nowhere above. It stays only if it meets
the criterion the coordination experiment set for its own second half, and the two after it inherited:

> **It removes a state a reader can be misled by, and what the repair replaces survives.** A repair
> that only removes states already refused by name has removed nothing a reader needed, and the honest
> report is that the remedy nobody measured a need for was not built.

And one condition of its own, because this is the first experiment whose remedy could import a whole
architecture:

> **A repair may not give one repository authority over another.** If the only way two records meet is
> that one of them is declared the real one, then this record is centralized and the honest report says
> so — the remedy would be a finding about what the format cannot be, not a feature.

---

## Success Criteria

1. Every measurement is a call over two directories in one process, and none depends on a network.
2. Both repositories are founded from one statement of the subject, so a shared base is shared by
   content and shown to be.
3. What each outcome holds is compared **by value** against each repository alone and against their
   union, never by absence of error.
4. Where a meeting is refused, the coordinate is named and related to what the two shared.
5. The three relations are exercised in both directions, and the asymmetry is reported as a
   measurement rather than as a remark.
6. What a world's identity pins is stated as a closed set of differences, not as an example.
7. Nothing is added to the APE engine.
8. Part B is built only against both criteria above, and its absence is a reportable result.
9. The eight earlier experiments' conclusions stand, or the change is recorded as a result of this one
   — and coordination's *intention merges* is the one at risk, so it is stated either way.

**Criteria 4 and 6 are the experiment.** The rest is the arrangement holding.

---

## Failure Conditions

The severe one, and it is new:

> If two repositories can only meet by one of them being declared authoritative, then this record is
> **centralized**, and every sentence in this row about a distributed architecture is a sentence about
> an aspiration. That is a finding rather than a remedy, and it would be the most valuable one the row
> has produced.

Its neighbour, inherited:

> If meeting requires the engine to know that knowledge lives in repositories — a handle, an origin, a
> name for *elsewhere* — then *storage is the application's decision* has met a real friction.

And the ordinary ones:

* a measurement that needs a network, a clone, or a thread to reproduce;
* an outcome reported from the absence of an error rather than from what the record holds;
* a repair chosen before Phase 3 and justified afterwards;
* reporting `Diverged` as a defect when it is measured to refuse the shape it was built to refuse;
* treating the asymmetry of `converge` as an implementation detail rather than as the answer to C5;
* asserting that two worlds are the same because they were built from the same decision, rather than
  because their identities compare equal.

**A refuted C3 is the interesting refutation.** If two repositories can agree about a world whose
knowledge they do not agree about, then a world identity is a claim the record cannot check, and this
experiment has found the veracity candidate's first reachable case.

---

## Variables Deliberately Left Open

### Whether a second repository is somebody else's

The first thing in nine experiments that could be. Every question about trust follows from it and none
of them is asked here — the second repository in this arrangement is the same author's, on the same
disk, and that is a choice made to keep this measurable rather than a claim that it is the interesting
case.

### What a repository owes another one it has met

Nothing here records that a meeting happened. Whether it should is downstream of whether one can.

### More than two

Two is the smallest number that can meet. Whether the third is free is not asked.

### Cost

Deferred by nine experiments, and now with a term this one would add: a meeting rebuilds both lineages
in memory before writing either.

---

## Methodological Constraint

> *Do not introduce an abstraction intended to solve an experiment that has not yet been performed.*

Structure may be introduced only when required by the current experimental procedure. The previous
experiments' conclusions are not revised here; where this one finds something that would have changed
them, it is recorded as a finding of this experiment, against the implementation as it then stood.

And the rules the last three experiments earned, which apply here without amendment. Before recording
anything as a finding, ask what would have to be false for it to be false. Every literal is written
before the run, and a wrong prediction is corrected in the open rather than adjusted. A prediction's
justification must quantify over as much as its claim does.

One more, and this experiment is where it bites hardest: **a friction is evidence of a want, not of a
need.** Two repositories that cannot meet will produce a very clear wish for a primitive that names
*elsewhere*. The ontology's test is whether operational coordination can be represented without it, and
the answer is not decided by how badly the arrangement wants it.

---

## Expected Pressure Points

### The word *meeting* has a shape borrowed from somewhere else

Git is the acknowledged influence, and it answers this with a merge-base object and a fetch. The
pressure is to measure whether APE *has* those, which is a question about Git. What this asks is what
two APE repositories already share, which may be more or less than a merge-base and is not the same
question.

### `Diverged` will refuse almost everything, and that is not a result on its own

Contention already established that the merge refuses divergence and that doing so is correct. Reporting
it again as a defect would be reporting a measurement as a complaint. What is new here is the
**coordinate** — that the position of the refusal is the size of the shared history, which makes a
refusal into a measurement of what two repositories have in common.

### Content addressing makes agreement look free, and free is suspicious

If two repositories agree about a world without being asked, the temptation is to report a distributed
merge. What has to be checked first is C3: whether the agreement is about the same knowledge, or
whether the identities merely coincide. The second is not a merge, it is the veracity candidate.

### One repository is untouched, and that will read as a bug

`converge` writes into its subject and leaves the working copy alone. The asymmetry is not an oversight
to be repaired inside a phase — it is C5, and the temptation is to build the symmetric form in order to
measure it, which would end the question.

---

## Observations

Each observation is recorded as its own numbered document beside this one, as the experiment produces
it. Record facts rather than decisions retroactively presented as inevitable.

Useful observations include: what two repositories share without being told; where a refusal's
coordinate is a measurement; which differences a world's identity is blind to; what the application can
say about two repositories and what it cannot; and what the record cannot distinguish, stated as the
query that has no answer.

Where possible, record the smallest reproducing case.

---

## Open Questions

* If a common ancestor is derived rather than found, does anything still need it to be named?
* Can a repository state that it has met another, using only what a repository holds?
* Two repositories that agree about a world and disagree about the journal under it — which of them is
  a reader supposed to believe, and does the question have an answer?

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

Contention asked whether a promise made to one writer is a promise at all when there are two.

```text
This one asks whether two records that have
never met were already the same record.
```

Nine experiments have measured what one repository can carry. If content addressing means two of them
already share whatever they said the same way, then everything this row built for one record was
distributed the whole time and nobody checked. And if it does not, the row has been describing a
distributed architecture it does not have — which is worth finding out before another nine.
