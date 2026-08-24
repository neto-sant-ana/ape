# Veracity

## Abstract

Corroboration proves that a repository agrees with itself. Eleven experiments have leaned on that, and
none of them has asked what it is worth.

Two protocols asked in almost the same words and neither answered:

```text
07 atomicity   Is there a partial state that is neither refused nor silent — one that
               reconstructs and answers something FALSE?

08 contention  Is there an interleaving whose result is neither party's state and yet
               reconstructs, corroborates, and answers something FALSE?
```

Both carried the question as open, both excluded it from their own boundary, and both went unanswered
for the same reason: each measured what **its own mechanism** leaves rather than searching for this.

And one experiment already found an instance, by an instrument no phase of it was supposed to use.
Experiment 01 tampered with two repositories in one place each, and both produced a world every check
accepted — the overspend frozen, a refusal gone, conflicts one to zero. It filed the finding under
*what the result does not cover*, and stated the boundary in a form that is not about that subject:

```text
malformed record  →  detectable      (an address that is not there, a count past the end)
false record      →  undetectable    (an address that is there and is the wrong one)
```

It also named the move that would close it, and was not permitted to make it:

> A coordinate that named the *state* of knowledge rather than a position in it would be
> checkable: a hash over the admitted prefix disagrees with a reordered journal, and a
> reconstruction can compute it. That value is derived, and this experiment is not permitted to
> persist a derived value — a constraint published by another experiment and not this one's to
> relax.

**That move was made four experiments later, for an unrelated reason.** `Taken::witness` is a
representation of the state of knowledge rather than of a position, and it arrived in the coordination
experiment because two parties needed to disagree about a prefix. Nobody has gone back to ask whether
it closed what experiment 01 left open.

---

## Question

**Is there a state this record can reach *by accident* that passes every check it has, is nobody's
forgery, and answers a question falsely?**

The load-bearing words are **by accident**. A record that agrees with itself and is wrong because
somebody edited it is the authenticity candidate, and the answer there has to come from outside the
record. This asks whether the same state is reachable by a process that stopped, by two writers
interleaving, by an admission landing in an order nobody intended — in which case there is nothing
outside to appeal to, because nobody lied.

---

## What *false* means here, decided before any phase runs

Neither earlier protocol defined it, and both used it. That is this experiment's first obligation,
because a reading is derived from what the record holds and is therefore always true **of** what the
record holds. Without a definition, the search has no target and *found nothing* means nothing.

> **A record answers falsely when it answers a question differently from the record the same events
> would have produced, and nothing in it refuses.**

The anchor is the laboratory, which constructs the events and therefore holds the faithful record. So
falsity is a **comparison of two records' answers**, not a judgement about truth — and the questions
compared are the ones the application already answers: the level an instance holds, the identity of a
world, and a feasibility verdict.

Three things are deliberately **not** false, and saying so is half the definition:

```text
incomplete            a record that knows less answers less. That is a shorter history and
                      every answer is true of it

differently arranged  06 measured a repository that weighed twelve candidates and one that
                      never explored, byte-identical. Both readings are true of what is
                      there; the record simply does not pin which history produced it

refused               a refusal is the record working, however unhelpful the message
```

And one hard case, which the definition has to survive because the contention experiment already met
it: where **two** writers acted, there is no single faithful record. The faithful record is then each
writer's own, and the question is whether the accidental state answers something **neither** of them
would. That is exactly how experiment 08 read its closest case, and this definition inherits it rather
than replacing it.

---

## Hypothesis

```text
falsity needs a claim the record   every answer is derived from what the record holds, and
cannot check                       every derived value is weighed against a stored one. Two
                                   things escape: the COORDINATE, which is a claim about a
                                   position, and `recorded_at`, which is a claim about an
                                   instant and belongs to no identity

and the record has since grown     the witness is the closing move experiment 01 named and
a check for one of them            was refused — so the coordinate may already be shut, and
                                   the hunt moves to the one that is not
```

The two lines point at opposite answers, and that is the experiment. If the witness closes the
coordinate and no accident can reach the instant, then this record is safe **by construction rather
than by luck**, and eleven experiments' worth of leaning on corroboration is justified for a reason
nobody had written down. If the instant is reachable, corroboration promises less than every result
since 02 has assumed.

**A negative result is the expected one, and it is worth as much as a positive one** — provided the
space it searched is named. That is this experiment's own methodological constraint, below.

---

## Pre-registered predictions

Five, written before anything runs, each with the observation that would refute it.

**V1 — The record has exactly two claims nothing checks.**

*Prediction:* an audit of every field a repository holds — what it is, and what it is weighed against —
finds that all but two are either content, a reference, or a derivation compared with a stored copy. The
two exceptions are `Taken::after` and every `recorded_at`.

*Refuted if* there is a third, or if either turns out to be checked after all.

**V2 — The witness closes the coordinate.**

*Prediction:* experiment 01's first tamper — repoint `after` at another address the journal holds —
is refused today, because the prefix that address resolves to has different members from the set the
decision witnessed. The move that experiment named as out of scope was made in experiment 05 for an
unrelated reason.

*Refuted if* the tampered repository still reads without complaint.

**V3 — The recording instant is not closed, and an answer depends on it.**

*Prediction:* `recorded_at` is carried in the envelope and not in the input an identity is derived
from, so changing it leaves **every address in the record intact** — the journal, the witness, the
coordinate and the entry ids all agree with themselves. And it is what `head_as_of` resolves a cut
against and what `ensure_selectable` refuses a commitment by, so a changed instant can change a world.

*Refuted if* changing it moves an address, or if no answer the application gives depends on it.

**V4 — And no accident this application has produces one.**

*Prediction:* the generators are an interrupted write, two writers interleaving, a readmission, a merge,
and any composition of them. Each copies values a writer wrote; none computes a recording instant or a
coordinate of its own. So every state they leave answers something true of some writer's record.

*Refuted if* one of them does not — which is the severe condition, and the largest finding in the
programme if it happens.

**V5 — Because an accident can lose a write or mix two, and cannot invent a value.**

*Prediction:* stated as the reason rather than as the result, so that it can be wrong separately. Every
field an accidental state holds was copied from something some writer wrote; falsity needs a value
nobody wrote.

*Refuted if* a composition produces a field no writer produced — a coordinate resolving to a position
neither writer was at, or a prefix neither admitted.

**V3 with V4 is the experiment.** V3 says where a false record would have to live; V4 says whether
anything but a hand can put one there. Either alone is half: an unchecked field nothing can reach is
not a hazard, and a search with no target is not a search.

---

## What is carried forward, and is not available as a finding

Usable in reasoning, and not reportable as a result:

* **A coordinate is a claim about the past and the record holds nothing to check it against** — 01,
  which is this experiment's premise rather than its finding.
* **A witness is a set**, so two entries swapped inside a prefix change neither its membership nor the
  entry a decision names.
* **An entry is addressed by the identity admitting it produced**, and an Event's identity contains its
  predecessor — so Event order is baked into every Event id after the first.
* **A cut resolves against the Event chain that stood**, and the head is `head_as_of(known_at)`.
* **A repository holds three files, and reading one weighs the third against what the first two
  produce** — 10, and the reason every state here goes to three guards rather than one.
* **A world's identity is blind to everything but its selection and its cut.**
* **A merge writes its own `worlds.json`**, so the third guard does not weigh anything a merge produced.

Before any sentence here is allowed to be a finding, ask what would have to be false for it to be false.

---

## Motivation

The material is in [`candidates/01-veracity.md`](../../../candidates/01-veracity.md), and the reason to
run it now rather than next is that experiment 10 supplied the thing 07 and 08 lacked. Neither of them
had a **method**: both asked whether a state was false and both answered by inspecting the one state
their mechanism produced. Experiment 10's Phase 4 is the apparatus — enumerate the states, put each one
to **every** guard the record has, and report the closed table. What it measured is not what is wanted
here; how it measured is.

It goes **before authenticity**, and this time the ordering is not a preference. That candidate's own
material now argues its outcome: of the five faces it gathered, four answer *no* to *is the
distinguishing fact in the record?*, so the remedy is a key or an anchor outside and the likely result
is a boundary. Boundaries keep. This one is the only queued question whose answer could **withdraw**
something eleven experiments have been building on.

**And it runs before any remedy**, for the reason five protocols have now given: work that closes a gap
makes the gap unmeasurable. Experiment 01 already named a remedy — a coordinate over the state of
knowledge — and part of it exists. Measuring what it covers has to happen before anybody extends it.

---

## Experimental Boundary

This experiment searches for a **state**, and the search is the whole of it.

It includes:

* constructing a faithful record and the answers it gives, as the anchor everything is compared to;
* **calibration by tampering** — producing a false record on purpose, to know what the target looks
  like, clearly labelled and load-bearing for nothing;
* an audit of every claim the record holds and what weighs it;
* the accident generators, enumerated before the search and searched one at a time;
* compositions across generators, which is where neither 07 nor 08 looked;
* every state put to all three guards **and** to the reading, as a closed table.

It deliberately excludes:

* **tampering as evidence.** A hand editing a file is the authenticity candidate's instrument. It
  appears here only as calibration, never as a result, and no finding may rest on it;
* **a remedy.** If the search finds a state, naming what would close it is Part B and happens after;
* **the network, a transport, more than two writers.** Excluded throughout the row and still excluded;
* **the engine.** Whether a record tells the truth is a question about a record. Nothing here asks the
  ontology to know that repositories exist;
* **benchmarking.** No timing.

Those concerns may become later experiments. They must not influence the structure introduced here
unless this experiment itself requires them.

---

## Experimental Subject

**A subject of this experiment's own**, for the reason the eleven before it give.

What it must express, and no previous subject needed:

```text
an answer that depends on      at least one commitment whose recording instant decides
a recording instant            whether a cut can select it, and at least one Event whose
                               instant decides whether a cut resolves to it. Without both,
                               V3 has nothing to move

two writers with a real        two parties whose journals differ in something a world is a
disagreement                   function of, so that a mixture of them can be about
                               something rather than about a filing nobody selected

a faithful record, held        the arrangement knows what happened, because falsity is
outside the record             defined as disagreeing with the record those events would
                               have produced — and the record cannot hold that

an interruption point that     the atomicity subject's partial states, in an arrangement
matters                        where the previous generation's worlds ANSWER DIFFERENTLY
                               from the new one's
```

The last is where a subject could quietly make the experiment un-failable. If the two generations
answer the same number, an interrupted write leaves a state that is true by coincidence, and the phase
would report *nothing false* about an arrangement that could not have produced anything false. So the
levels of every generation, every party and every mixture are written down as literals before the run,
and they are required to differ.

---

## Initial State

```text
one faithful record       written whole, read once, and its answers recorded as the anchor
the events that made it   held by the arrangement, which is the only place they can be held
three guards              the coordinate against the journal, the witness against the prefix,
                          and the worlds file against what the decisions produce
```

Nothing is inherited from the previous experiments except their conclusions and their code.

---

## Procedure

### Phase 0 — The faithful record, and what it answers

Read once. Every level, every world identity, every feasibility verdict, recorded as literals. This is
the anchor, and every later phase is a comparison against it.

---

### Phase 1 — Calibration: what a false record looks like

**Not a finding. An instrument.**

Reproduce experiment 01's two tampers against the record as it now stands, and add the one V3 names —
a changed recording instant. For each: which guard refuses, if any, and whether the answer moved.

This exists so the search knows its target. A phase that hunted for *false* without first producing one
on purpose would not be able to tell an empty result from a blind one.

---

### Phase 2 — The audit: what the record checks, and what it does not

V1, V2 and V3. Every field a repository holds, in a table: what it is, what weighs it, and what happens
if it moves. The two exceptions the hypothesis names have to survive being looked for, and a third
would refute V1.

---

### Phase 3 — The generators, named before the search

V4's space, written down before anything is run, because a negative result is only as good as the
enumeration behind it. Each generator with the mechanism that produces it and the states it can leave.

---

### Phase 4 — The search, one generator at a time

Every state each generator leaves, put to all three guards and to the reading, as a closed table per
generator. Where a state reproduces one experiment 07 or 08 already measured, it is reproduced by value
first, so that a change in outcome is attributable.

---

### Phase 5 — Compositions

Where neither earlier experiment looked. An interruption during a contended write; a readmission inside
an interrupted one; a merge over a journal that a stopped write left. Composition is the only reason to
expect a state a single mechanism cannot leave.

---

### Phase 6 — The space, and what was not searched

The verdict on coverage, stated as a boundary rather than as a conclusion: which generators were
exhausted, which were sampled, and which were not reached. A search that found nothing says so here or
it says nothing.

---

### Phase 7 — Name what an application would need, and build none of it

Requests, verbatim, in the vocabulary of the need rather than of a solution.

---

### Phase 8 — Part B, if it was earned

A repair, whose shape is decided by Phases 1 to 6 and named nowhere above. It stays only if it meets the
criterion the coordination experiment set and five experiments have inherited:

> **It removes a state a reader can be misled by, and what the repair replaces survives.**

This experiment is unusual in that a positive result would meet that criterion by definition — a state
that passes every check and answers falsely **is** a state a reader can be misled by. So the discipline
runs the other way here, and the condition of its own is about scope rather than about worth:

> **A repair may close only the state that was found.** A remedy for the class the state belongs to is
> a remedy for cases nobody measured, and this is the experiment least able to afford that: the whole
> value of a negative result is that the space was named, and a repair aimed at the space would make
> the next search unmeasurable.

---

## Success Criteria

1. *False* is defined before any phase runs, and the definition excludes **incomplete**, **differently
   arranged**, and **refused** — with an instance of each produced, so the exclusions are measured
   rather than asserted.
2. The faithful record is constructed by the arrangement, so falsity is a comparison of answers and
   never a judgement.
3. Tampering appears only as calibration, is labelled as such wherever it appears, and no finding rests
   on it.
4. The generators are enumerated **before** the search, and Phase 6 says which were exhausted.
5. Every state goes to all three guards and to the reading, reported as a closed table.
6. Every literal — levels, identities, verdicts — is written before the run.
7. Nothing is added to the APE engine.
8. Part B is built only against both criteria above, and its absence is a reportable result.
9. The ten earlier experiments' conclusions stand, or the change is recorded as a result of this one.
   *Corroboration proves a repository agrees with itself* is the one at risk, and what it is worth is
   the thing being measured — so it is stated either way.

**Criteria 1 and 4 are the experiment.** The rest is the arrangement holding.

---

## Failure Conditions

The severe one, and it is inverted from every experiment before it — here the severe outcome is
**finding something**:

> If an accident produces a record that passes every check and answers falsely, then corroboration
> promises less than every result since experiment 02 has assumed, and the finding is larger than this
> experiment. It is reported as a failure of a **premise**, not as a success of a search, and every
> conclusion that leaned on it is listed.

Its neighbour, which is this experiment's own:

> If *false* cannot be told from *differently arranged* even with the arrangement holding the events,
> then the question is malformed and the finding is why. That is a real possibility and it is named
> here so that discovering it is a result rather than a retreat.

And the ordinary ones:

* a state produced by tampering and reported as an accident;
* a negative result whose space is not named — indistinguishable from not having looked;
* an arrangement whose generations, parties or mixtures answer the same number, making *nothing false*
  a property of the subject;
* a repair aimed at the class rather than at the state;
* reporting *the record is safe* when what was measured is *these generators did not reach it*.

**A refuted V3 is the cheapest interesting result.** If no answer depends on a recording instant, then
the one unchecked claim left is inert, and the record is closed for a reason simpler than anybody
expected.

---

## Variables Deliberately Left Open

### What would close the recording instant

Experiment 01 named the shape of the coordinate's closure and was refused it. If the instant turns out
to be the live one, its closure has a shape too — and naming it before Phase 6 would be choosing the
answer.

### More than two writers

Excluded throughout the row.

### Whether a record can say it might be false

A record that could report its own uncertainty is a different design and is not asked here.

### Cost

Deferred by eleven experiments.

---

## Methodological Constraint

> *Do not introduce an abstraction intended to solve an experiment that has not yet been performed.*

Structure may be introduced only when required by the current experimental procedure. The previous
experiments' conclusions are not revised here; where this one finds something that would have changed
them, it is recorded as a finding of this experiment, against the implementation as it then stood.

And the rules the last five experiments earned, which apply here without amendment. Before recording
anything as a finding, ask what would have to be false for it to be false. Every literal is written
before the run, and a wrong prediction is corrected in the open rather than adjusted. A prediction's
justification must quantify over as much as its claim does. A friction is evidence of a want, not of a
need. A phase must not be satisfied by an arrangement it could not have failed.

One more, and it is this experiment's own, because this is the first one whose likely result is that
nothing is there:

> **A negative result must name the space it searched.** A search that finds nothing is
> indistinguishable from a search that did not look, unless the generators are enumerated before the run
> and the report says which were exhausted, which were sampled, and which were not reached.

---

## Expected Pressure Points

### The instrument that finds it is the instrument that is forbidden

Tampering works. It found the instance in experiment 01 and it will find one again in Phase 1. The
temptation is to let a calibration slide into evidence — to produce a state by hand, notice that no
guard refuses, and report it as what an accident could do. Every earlier finding of this shape came
from a hand, and the whole of what this experiment adds is the word *accident*.

### A negative result will feel like nothing happened

Eleven experiments have produced capabilities, refusals and tables. This one may produce a sentence:
*every state these five generators reach answers something true.* That sentence is the justification
for everything built on corroboration since experiment 02, and it has never been said.

### The subject can make the answer true by accident

If the two generations of a repository answer the same number, an interrupted write cannot be false and
the phase reports a property of the arrangement. This is the same hazard experiment 10 met at Phase 1
and answered by writing the tail down as a literal. Here it is answered by requiring every generation,
party and mixture to answer **differently**, before the run.

### *Accident* will be asked to stretch

An operator running the application in a way nobody intended is not an accident of the record. A disk
that returns old bytes is not one either — that is durability, which is queued separately. The
generators are the mechanisms the application itself has, and a state produced by anything else belongs
to a different question.

---

## Observations

Each observation is recorded as its own numbered document beside this one, as the experiment produces
it. Record facts rather than decisions retroactively presented as inevitable.

Useful observations include: what a guard covers that nobody had noticed; where an answer turns out to
depend on a value nothing checks; which generators can reach which fields; and what the record cannot
distinguish, stated as the query that has no answer.

Where possible, record the smallest reproducing case.

---

## Open Questions

* Is *the record was written by a process telling the truth about when it decided* an assumption a
  record can shed, or the boundary of what a record is?
* If the recording instant is the live claim, does anything derive from it that a second representation
  could be compared against?
* Does a merged record — which writes its own worlds file — lose a guard the other generators face?

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

Collision asked whether two records that have never met were already the same record.

Witness asked whether a decision is about everything it happened to come after.

```text
This one asks whether a record that agrees
with itself has thereby told the truth.
```

Corroboration was built in experiment 02 and has been the floor under every result since. Nobody has
asked what it stands on. If the answer is that this record cannot be made false except by a hand, then
eleven experiments have been right for a reason none of them wrote down — and the reason is worth more
than the reassurance. If the answer is that it can, then the floor has a hole in it, and every result
that stood on it says so.
