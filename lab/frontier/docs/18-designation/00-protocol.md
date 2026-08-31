# Experiment 18 — Designation

## Where this came from, and it is not the queue

**The operator's analogy, followed until it broke a `✓`.** The project takes its architecture from
Git, and the reading that opens this experiment is three sentences long:

> *É como se abrir uma tese é criar uma branch e testar/propor. […] Mas dentro daquela branch a base
> de código é o que é. Enquanto main é só uma referência ao que seria "o código principal".*

A Thesis is a branch and opening one is proposing — [`06-exploration`](../06-exploration/99-result.md)
measured that **eleven of thirteen** decisions in a record are roads not taken, so the space of
proposals is already there and complete. **Nothing is missing from a world.** What is missing is the
reference beside them.

The core delegates exactly that, in a list of what Synthesis does not do:

> *moving mutable references such as `main`, **which belongs to the application***
> — [`07-synthesis.md:1341`](../../../../core/src/docs/07-synthesis.md)

[`CHARTER.md`](../../../CHARTER.md) had it marked concretized, on the strength of `cli`'s `current`.
`current`'s own docstring says what it is — *the file naming the **generation** a reader reads*,
alternating two directories with an atomic rename. That is Git's pointer to the live object store. It
is not a ref to a commit. **The mark is corrected and H1 fell from ~40% to ~30%.**

And the consequence was measured two experiments before anybody named the cause:

> *The only thing privileging W4 is that `hindsight::build()` handed it over as the world the house is
> in now, **which is application state outside the graph**.* — `agents/02-hindsight`

## The question

> *A record's identity is derived from its content, and a reference is a thing that changes. Can a
> record say which of its worlds it means — and where does that claim live?*

## What makes it hard, and it is one sentence from experiment 17

Whatever holds a designation must name a `ThesisId`. And 17 measured what a `ThesisId` is:

> *a **world** (`ThesisId`, derived and never admitted, so it resolves nowhere but where it was
> produced)*

**A designation names something the Canon has never seen.** Every other claim in a journal names
content the Canon admitted and can therefore check; this one names a value the lineage *produces*. So
the question is not only *where does the reference live* but **what can check it there**.

## The three candidate homes, and each has a precedent in this row

```text
A  A JOURNAL ENTRY      shared content. Two records that disagree about which world is
                        principal disagree in the journal, and `converge` refuses them.
                        Precedent: everything else in the record

B  A FIFTH FILE         a record's own claim about itself, optional on read, checked
                        against `worlds.json` the way custody is checked against the
                        journal. Precedent: 16-custody, and it is three weeks old

C  PER PARTY            a designation carries a `by`, like a decision. Two parties of one
                        house hold different plans and neither is wrong. Precedent:
                        `Taken.by` and `decided_by`, which already make the lineage
                        answerable per party
```

**The discriminator is a question about the domain, not about the code:** is *which world is live* a
fact about the house, or a fact about a record, or a fact about a party?

Git answers it C-shaped — `main` is per-clone, and two clones disagreeing is the normal state rather
than a conflict. That is an argument and not a measurement, and this experiment exists because the
analogy has now been wrong once.

## Pre-registered predictions

**P1 — A is refused, and by the Canon rather than by taste.** An admission naming a `ThesisId` names
something never admitted, so nothing at the admitting layer can check the target exists. The record
would hold a claim it cannot verify, which is the shape `11-veracity` measured and closed.

*Refuted if* it admits cleanly, which would say a journal can carry a reference to a derived value —
and would make every other derived value a candidate for the same treatment.

**P2 — B works, and the check is custody's, one layer over.** `custody.json` says which entries a
record holds and is compared against the journal on read. A designation says which world a record
means and is compared against `worlds.json` on read. Same shape, same optionality, same failure mode
when absent.

*Refuted if* the comparison cannot be made — which would mean `worlds.json` does not hold what a
designation would have to point at.

**P3 — it is per party, so B and C are the same answer and not two.** A designation carries a `by`,
the file holds one per party, and the house's own plan is the designation of whichever party the
reader is. `Taken.by` already established that the lineage is answerable per party.

*This is the experiment.* Refuted if one designation per record is enough — which would say a
repository has one plan, and would contradict `04-multiagent`, where two parties held two tips of one
record on purpose.

**P4 — two records with different designations converge without refusal.** A designation is not
knowledge, and [`MERGING.md`](../MERGING.md) settled that the application merges two lines of
deliberation over one line of knowledge. A disagreement about which world is principal is
deliberation.

*Refuted if* `converge` refuses them, which would make a designation knowledge after all and put P1
back on the table.

**P5 — a designation without a history cannot answer *what was the plan on the 12th*.** The lineage
holds every world that was decided and the order decisions were taken in; it does not hold which was
principal when. So a bare pointer loses the question `agents/02-hindsight` was built to ask about
knowledge and has never been able to ask about intention.

*Refuted if* the answer is derivable from the lineage — which would mean the plan's history was there
all along and only the pointer was missing.

## Procedure

**Phase 0 — the world.** A record with several decided worlds, at least two parties, and a plan that
moves: designated, moved, moved back. Extended from the arrangement the row already uses rather than
built to suit the question.

**Phase 1 — A, measured rather than argued.** Attempt the journal entry. P1 says the Canon refuses;
whatever it does is recorded as a literal.

**Phase 2 — B, built.** A designation file, derived on write and compared on read, following custody's
shape exactly — including optional-on-read, so that the records already committed to this repository
keep rebuilding unchanged.

**Phase 3 — C, by putting two parties' plans in one record** and asking whether one designation could
have served.

**Phase 4 — the merge.** Two records, two designations, `converge`.

**Phase 5 — the history question**, which is P5, and which decides whether B is a pointer or a log.

## Success criteria

1. Every one of A, B and C is measured, including the ones that fail.
2. The four repositories in `lab/agents/04-multiagent/run-*/repo` and the three in
   `05-reconciliation/run-a` rebuild unchanged — a designation is additive or it is refused.
3. All five predictions answered, including *no*.
4. What is built, if anything, is an **obligation** with the experiment that earned it recorded in the
   module that carries it.
5. Nothing added to the engine. The core delegated this in writing; taking it back would be answering
   a different question.

## Failure conditions

* **A designed rather than measured.** P1 says the Canon refuses; if the attempt is not made because
  the refusal is obvious, the experiment has assumed its own first result.
* **B built before A is measured**, which would make the order of the phases the argument.
* **The Git analogy treated as evidence.** It opened this experiment and it has already been wrong
  once — about `current`.
* **A designation that names a world the record does not hold**, admitted without refusal. That is the
  one thing worse than no designation at all.

## Excluded

* **The application's command surface.** Whether `ape-cli` grows a `checkout` is not this question.
* **What a reader does with a designation.** That is `succession/01-articulation`, which is deferred
  until this concludes and which is why this experiment goes first.
* **The engine.**

## What it bears on

**H1**, and it is the ✗ that was a ✓ — the most direct thing in the laboratory: a gap the core
delegated in writing, to the application, which the application never filled.

**And `succession/01-articulation` waits on it.** That experiment measures how a record should be
carved for a reader who was not there, and a carving cannot produce a reference the record does not
have. Whether the pages can say *this one is the plan* is decided here, not there.
