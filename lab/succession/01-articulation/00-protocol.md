# Experiment 01 — Articulation

## What the operator's provocation removed from this experiment

It was going to compare representations. It cannot, and the reason took two sentences:

> *"Frontmatter+wikilink resolve motivação bem." Sim, assim como um bd relacional com coluna varchar
> para why, assim como um json com campo why, assim como qualquer estrutura de dados que absorva
> dados e metadados. […] Assim como uma sequência de páginas html se referenciado via anchor tag.*

**A question whose answer is always yes is not a question.** Every structure that holds a value holds
a `why`, and markdown-with-wikilinks, HTML-with-anchors and any other hypertext are the same artifact
wearing different syntax. [`CHARTER.md`](../../CHARTER.md) H4 asks *in what representation*, and that
half is now answered by inspection: **the format is free.**

And a second thing is removed, which is a confound this laboratory would have caught late:

> Comparing markdown against SQL against JSON, with an LLM as the reader, measures **what the reader
> was trained on**. Text wins before the experiment runs.

So the substrate is held constant — everything below is markdown, everything is readable with `cat`,
and choosing markdown over HTML is declared arbitrary rather than argued.

## What is left, and it is not free

> **Given hypertext, what is a page and what does a link mean?**
>
> The format is free. The carving is not: a hypertext in which everything is a node and every edge is
> the same edge is unreadable at twenty-one entries, and being markdown does not save it.

## The question

> *Does the unit a record is carved into change what a reader who was not there can establish from
> it — and at what cost?*

## Why this is answerable now and was not before

[`00-testimony`](../00-testimony/99-result.md) supplies both halves of the instrument, and neither was
invented for this experiment.

**The question set.** Its 46 claims about one record are exactly *the things somebody said about a
decision*, and each already carries a verdict on whether the record establishes it.

**The baseline, measured twice by two readers who did not confer.**

```text
the record as four JSON files, and no prose
  19 of 46 establishable     this laboratory's reading
  17 of 46                   an independent classifier, Phase 4
```

That is what a reader can get today. **Every arrangement below is measured against 19 and 17**, and a
number inside that range is no result at all.

## The three articulations

Same content, same substrate, same reader. Only the carving differs.

```text
A  FLAT          one document. The record as prose, and all the reasoning, in reading order.
                 Hypertext with no carving — the null against which a carving has to earn its
                 keep

B  PER ENTITY    one page per Commitment, Agent, Event, Thesis. Frontmatter holds the
                 relations, the body holds what attaches to that thing. The operator's
                 original analogy, built

C  PER DECISION  one page per `Taken`. The decision is the unit; entities are linked and not
                 owned. Reasoning lives with the deciding rather than with the decided
```

**C exists because of a correction made while writing this protocol** — and the correction was itself
corrected, by the operator, before the protocol was committed. Both passes are kept, because the
second says something about how the first went wrong.

`00-testimony` reported four shapes as *unanchored*, having tested each against an **entity**. The
first correction tested them against a **decision** and split them two and two. The operator then
observed that the two left over *"me parecem informações muito mais vagas que as 3 primeiras"* — and
counting the seventeen claims shows the grouping was wrong in **both** directions:

```text
motivation              1    an entity
accountability          2    a decision that was taken
own reasoning           1    a decision that was taken

specific imperatives    8    "fork 1cd9af, introducing 06b94, omitting nothing"
                             "ask finance to confirm that 791528 is theirs"
                             "tell finance to advance its cut to 2026-01-07"

general norms           3    "going forward, a decision should join the plan on the day"
evaluations             2    "the system doing what it was built to do"
```

**The eight are the most precise claims in the whole set** — hex identities, dates, named operations —
and every one had been filed as *recommendation, anchored to nothing existing*. They are the opposite
of vague.

**The operator's vagueness picks out five of seventeen**, and what separates them is not the anchor
but the **logical type**: a norm is an imperative and has no truth value; an evaluation is an appraisal
against a standard nobody stated. The other twelve are assertions about what happened.

And inside the eight there is a division that matters more than the original one:

```text
"fork 1cd9af, introducing 06b94"     a whole Decision, in the engine's own vocabulary,
                                     simply NOT TAKEN
"ask finance to confirm"             a question to a party
"record the provenance elsewhere"    an instruction about record-keeping — this is H4
```

**Only two of the eight are cleanly of the first kind, and two is not a finding.** But the shape is
worth naming before the run rather than after: the record has a place for a decision that **was**
taken and none for one that is **proposed**. That is not a missing home for prose. It is a missing
state, and no carving reaches it.

## Pre-registered predictions

**P1 — every carving beats flat, and B and C beat it by different claims.** A reader of A can
establish what a reader of the JSON can, plus whatever the prose states outright, and no more:
finding a thing in one long document is not the same as the document relating it.

*Refuted if* A matches B and C, which would say the carving buys nothing and the prose is the whole of
the gain — the strongest possible result for *just write it down anywhere*, and it must stay reachable.

**P2 — B wins on motivation and C wins on accountability and own reasoning**, by the anchors above.

*This is the experiment.* Refuted if either wins both, which would say one unit serves and the
correction above was wrong.

**The whole of P2 rests on four claims of four hundred and twenty-three** — one motivation, two
accountability, one own reasoning. That is a thin base and it is stated here rather than discovered in
the reading: if the four split cleanly the prediction holds on a sample too small to carry it, and the
right conclusion is *worth building on and not yet measured*.

**P3 — the five vague ones are established by none of the three, and the eight specific ones are
established by all three.** Norms and evaluations have no truth value to establish; the eight name
identities that every carving holds, so any carving reaches them and none of them discriminates.

*Rewritten after the operator's observation, and the earlier version is worth keeping visible:* it
predicted that *recommendation and evaluation* would be reached by no carving, which lumped the eight
precise imperatives in with the five vague ones. Had it run unamended, the eight would have come back
established and P3 would have read as refuted — **for the wrong reason**, and the wrong reason would
have been invisible.

*Refuted if* the eight discriminate between carvings after all, which would say that naming an
identity is not the same as being findable, and that a carving can lose a claim it holds.

**P3b was written, refuted by the operator before the protocol was committed, and is kept here as the
error it was.**

It read: *none of the three has a place for a decision that is proposed rather than taken.* The
operator's objection, and it is decisive:

> *Thesis e Synthesis foram criados justamente para permitirem o estudo do espaço de possibilidades
> viáveis […] para uma tese toda decisão é tomada, mas tomar a decisão numa tese é o ato de propor.*

**A fork is a proposal.** The core says so in its first line — *organizations continuously compare
alternatives, revise unsettled plans and explore different continuations* — and
[`06-exploration`](../../frontier/docs/06-exploration/99-result.md) measured that **eleven of thirteen**
decisions in a record are roads not taken. The tree of Theses **is** the space of proposals. There is
no missing state.

**Where the error came from is worth more than the error.** `run-reader` left its proposal in prose
because it was a *reader and was instructed not to write* — not because the record could not hold it.
This reading took an agent's permissions for a property of the boundary.

**And the real asymmetry is the inverse.** The record holds proposals in abundance; what it does not
hold is which one was **adopted** — `02-hindsight` measured exactly that, and called the live world
*application state outside the graph*.

> **Amended 31/08, before the run and after `frontier/18-designation`.** The sentence above is kept
> because it is what sent that experiment; it is no longer true of the application, and the
> correction is in [the section below](#what-18-designation-changed-and-what-it-did-not).

So what survived of P3b is two items **already in the queue**: *a decision that says it weighed rather
than meant*, and *on-behalf-of between agents*. This reading re-derived tracked items by a wrong route
and presented them as new — **one day after building the `Standing` field to stop precisely that**.
The discipline applies to claims this laboratory makes, and not only to claims it classifies. Recorded
in `01-the-standing-applies-to-us.md`.

**P4 — C costs least to read and B costs most.** Cost is measured, not estimated: bytes opened and
pages traversed per question answered. B has one page per entity and a record of twenty-one entries
has many entities; C has five decisions.

*Refuted if* B is cheaper, which would say the entity graph is the shorter path even when it is the
wider one. **This is the half [`CHARTER.md`](../../CHARTER.md) says H4 and H1 meet through**, and
nobody has measured a cost in this laboratory yet.

**P5 — the two readers do not want the same thing, so H5's *indiscriminately* fails on at least one
carving.** An agent will answer from whatever page it is on; a person will need to know where to
start.

*The honest limit, stated before the run:* the human side is **n = 1**, the operator, one reading,
recorded as one reading. It is evidence and it is not a measurement, and no conclusion rests on it
alone.

## What 18-designation changed, and what it did not

This protocol was written on 31/08 and **held back** on one sentence: a carving cannot produce a
reference the record does not have, and whether a page can say *this one is the plan* was not this
experiment's to decide. [`frontier/18-designation`](../../frontier/docs/18-designation/99-result.md)
decided it, Confirmed, the same day. Everything below is written before any agent is invoked.

**A record can now say which of its worlds it means, and it is a log rather than a pointer.** Ordered
by position, coordinate-checked against the journal, attributable where anything says. So the third
paragraph of P3b is superseded: what the record does not hold is no longer *which one was adopted*.

```text
was      the record holds proposals in abundance and not which was adopted
is       the record holds proposals in abundance, and a per-party ordered log of
         which was the plan, at which coordinate — `cli/src/designation.rs`
```

**And it changes nothing about the material this experiment reads**, which is the part that had to be
checked rather than assumed. The generator's source is
[`agents/05-reconciliation/run-a`](../../agents/05-reconciliation), written by parties nobody can
re-run, and it is a **four-file record with no designation log**. Nothing rewrites it — `lab/README.md`
names those repositories as the one real veto — so:

* **The baseline stands.** *19 of 46 and 17 of 46* were measured against a record of four JSON files,
  and the record this experiment carves is that record. The number is not stale; it is about a
  fixture that has not moved.
* **No carving gets a plan to carve.** Whatever a page could say about *what the plan was* is
  unavailable to all three equally, so it discriminates between none of them.

**What it does introduce is a confound, and naming it before the run is the whole point of naming
it.** 18 found that a designation is [`Taken`]'s shape — *a claim about a world, at a coordinate, by
somebody* — so the record's two claim-carrying files now have one shape, and that shape is the
**decision**. Carving C is per decision.

That is an argument for C which arrives from outside this experiment and is not evidence for it. If C
wins, the reading must say which of the two it won by: the 46 questions, or the fact that the
laboratory spent the previous day deciding the record's newest claim is decision-shaped. **A result
for C that cannot separate those is a result about the order the experiments ran in.**

## Method

**Build all three from the same source.** A generator in `ape-succession` reads
`agents/05-reconciliation/run-a` and emits three directories. Written once, so no carving can be
quietly favoured by a better hand — and derived, so the content cannot drift between them.

**The reasoning that goes in comes from the classification**, not from a fresh reading: the 27
non-housed claims are placed by their kind and their anchor, mechanically. Where a claim has no
anchor under a carving, it goes to that carving's overflow — and **the size of the overflow is
itself a result**.

**One agent per carving**, none of which has read this protocol, each asked the same question set and
each reporting what it opened. Three agents, one question set, one measured baseline.

**Nothing is hand-tuned between runs.** If a carving is improved after seeing its result, the
experiment is over and says so.

## Success criteria

1. The three carvings are generated from one source, and the generator is committed.
2. Every agent answers the same 46 questions and reports what it opened.
3. The four numbers — A, B, C and the baseline — are reported together, including where a carving
   loses to the baseline.
4. All five predictions answered, including *no*.
5. Cost is measured rather than argued.
6. Nothing added to the engine, and nothing to the application.
7. Where a carving is worse, that is reported as its result and not as a fault of the build.

## Failure conditions

* **A carving improved after its run.** The generator is fixed before the first agent is invoked.
* **The overflow quietly absorbing what will not fit**, unreported. Its size is a result.
* **Cost estimated instead of measured**, which is what has kept it unmeasured for eighteen
  experiments.
* **The human reading treated as a measurement.** It is n = 1 and says so.
* **Markdown argued for.** It is arbitrary among equivalents and the protocol says so at the top;
  finding it good would be finding the reader is an LLM.
* **C winning without the confound being separated.** See the section on `18-designation`: an
  argument for the decision as the unit already exists, it came from another row, and a reading that
  cannot tell it apart from these 46 questions is reporting the order the experiments ran in.

## Excluded

* **Any comparison of substrates.** Settled by inspection, above.
* **A database, an index, a query language.** The question is the carving, not the retrieval.
* **The engine.** As always.

## The hypotheses this bears on

**H4's second half**, reduced by the operator's provocation from *which representation* to *which
carving* — which is the part that was ever a question.

**And H5, which turns out to be the same experiment.** *Navigable and intelligible to human and
autonomous readers indiscriminately* is not a property to check after a representation is chosen: it
is the only thing left that distinguishes one hypertext from another, once the format is free. P5 is
where it is answered, and it is the weakest of the five.
