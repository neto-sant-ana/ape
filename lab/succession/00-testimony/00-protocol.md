# Experiment 00 — Testimony

## What this experiment does

Eight agents, across five experiments and eighteen months of engine changes, each wrote an `ANSWER.md`
saying what it did and why. **13,382 words of prose about decisions the record holds.**

Every one of them was written before this hypothesis existed. This experiment reads them and asks, of
every claim: **does the record already hold this?**

> *What did eight agents say that the record cannot hold — and is what is left over a **bounded set of
> kinds**, or is it prose?*

## Why the answer decides the shape of everything after it

[`CHARTER.md`](../../CHARTER.md) H4 asks whether the record can hold the material that reaches no
primitive, *and in what representation*. **This experiment refuses the second half and answers the
first**, because a representation cannot be chosen before the thing to be represented is known — and
the analogy that first described H4 (a structured header holding the entities, a body holding the rest)
already presumes an answer to this question.

```text
if the leftover sorts into a bounded, recurring set of kinds
    → it has fields. A header can hold it, and H4's next experiment is a comparison
      of representations with a known workload

if the leftover is idiosyncratic to each run
    → it has no fields, and H4's answer is a body of prose attached to an entity, which
      is a different design and a much weaker claim about what the record can carry
```

Both are real outcomes. The second is the one that makes H4 smaller, and it must stay reachable.

## What it is not

**Not a measurement of the agents.** The prose is evidence about the *record's* coverage. An agent
saying something the record cannot hold is not an agent doing badly.

**Not the training experiment.** H3 asks whether this material is a signal for teaching decision
reasoning. This asks only what the material *is*.

**Not a proposal.** Nothing is designed here, and no representation is named. The deliverable is a
requirement.

**Not a re-reading of the experiments' results.** The `99-result.md` documents are this laboratory's
reading of what happened. The corpus here is the **agents' own words**, which is a different thing and
the only part of the archive written by somebody who did not know what it would be used for.

## Why this corpus is unusually good evidence, and it is the point

H4 is marked **emergent** in the charter — thought of after the work, which puts it in a weaker
epistemic position than H1 and H2, and the charter says so on purpose.

**This corpus is the strongest available correction for exactly that weakness.** Every word of it was
written before the hypothesis existed, by eight sessions that could not have shaped it to fit:

```text
01-single-agent/run-01          1617     in-memory boundary, engine db3f965
02-hindsight/run-01             2377
03-narrative-mismatch/run-b     1469
04-multiagent/run-a             1617     the repository boundary
04-multiagent/run-b             1698
04-multiagent/run-b-prime       1595
04-multiagent/run-reader        1136     the only one that reads rather than writes
05-reconciliation/run-a         1873     the fourth file, and an enforced barrier
                                -----
                                13382
```

**Two boundaries and one reader**, which matters: if a kind appears only under the in-memory boundary
it is a property of that boundary, and if it appears only in writers it is a property of writing.

## What earlier results established, and is not measured again

* **The prose exists and the record has no place for it.**
  [`candidates/04-training.md`](../../candidates/04-training.md) recorded it in one line — *every run
  produced a decision expressible through the primitives and a justification in prose that the record
  has no place for*. This experiment does not re-establish that. It asks **what** the prose is.
* **A record's majority is roads not taken.** `06-exploration` measured eleven of thirteen decisions
  unchosen, and that the one thing the record cannot say about any of them is which was **weighed**
  rather than **meant**.
* **A stated reason can contradict the derivation.** `03-narrative-mismatch` sorted prose against the
  derived record and found it wrong in specific ways. That the two can disagree is settled; here a
  disagreement is one possible **kind**, not the finding.

## Method

### The unit

**A claim**: one assertion about the world, about the record, or about the run, stated in the
`ANSWER.md` and able to be true or false on its own. Section headings, restatements of the task, and
pure narration ("then I ran the program") are not claims and are counted as skipped rather than
silently dropped, so that coverage is measurable.

### The two questions asked of every claim

```text
1  HOUSED?    could this claim be read off the record by somebody with the four files and
              no prose? If yes, it names WHICH primitive or derived value carries it

2  IF NOT     what kind of claim is it?
```

**Question 1 is the falsifiable half and question 2 is the useful half.** A housed claim is prose the
record made redundant; the leftover is H4's subject.

### Where the categories come from, and why not from the whole corpus

**The candidate kinds are derived from `05-reconciliation` alone** — the one run already read in full —
and then tested against the seven that did not produce them. Deriving categories from the whole corpus
and then reporting that the corpus fits them measures nothing.

From that one run, the candidate set:

```text
road-not-taken     an alternative weighed and rejected, with the reason
                   "I did not collapse the two tips into one world, because…"

want               something reached for at the boundary and not found
                   "nothing in the crate makes the honest version representable"

qualification      a confidence or weakness attached to a claim the record DOES hold
                   "the `by` field is the weakest thing in the result"

loss               a statement that the record cannot say something
                   "finance's original witnesses are genuinely lost"

method-limit       a limit of how the work was verified, not of the record
                   "the repository-level wiring around it is unproven"
```

**Five, and none of them is about the domain.** Every one is a claim *about the record or about the
work* rather than about the house, the account or the money — which is either the sharpest thing in
this experiment or an artefact of the one run they came from. Seven runs decide which.

A claim that fits none of the five is recorded as **unclassified with its text**, never forced into the
nearest box. The count of unclassified claims is a result: many of them is the *idiosyncratic prose*
outcome arriving.

### The guard, so that this is a measurement and not an essay

The classification is committed as data, and a suite derives from the sources what it must satisfy:

* **Coverage.** Every `ANSWER.md` under `lab/agents` is classified. The count is **read from the
  filesystem**, not written down — a sweep that found none would otherwise agree with everything.
* **Every housed verdict names a carrier, and the carrier exists.** A claim marked housed must name a
  kernel entity or a named derived value, and the suite **derives the legal names from
  `core/src/kernel/entities/`** rather than holding a copy. A classification citing something that is
  not in the kernel fails, naming the orphan.
* **Every unhoused claim carries its verbatim text**, so that a reader can disagree with the
  classification without re-reading the corpus.
* **The mutation.** Re-labelling one housed claim to cite a carrier the kernel does not have must go
  red naming that carrier; deleting one `ANSWER.md` from the classification must go red naming that
  file.

## Pre-registered predictions

**P1 — most of the corpus is housed.** More than half of the claims restate what the record already
carries, because the goal asked each agent to say what it did and the record says what it did.

*Refuted if* the majority is unhoused, which would mean the record carries less of a decision than
eighteen frontier experiments have assumed.

**P2 — the leftover is bounded, and the five kinds survive contact with seven runs they did not come
from.** Fewer than one in five unhoused claims lands unclassified.

*This is the experiment.* Refuted if unclassified is a large share, which is the *idiosyncratic prose*
outcome and makes H4's representation a document body rather than a set of fields.

**P3 — no unhoused kind is about the domain.** Every one is about the record, the work, or the
boundary; none is an operational fact about the house that the primitives could not express.

*Refuted if* domain facts appear, which would be a much heavier finding: the ontology would be missing
something operational rather than missing a home for reasoning.

**P4 — `road-not-taken` is the largest unhoused kind**, because `06-exploration` measured that
abandoned siblings are the majority of a record and that what the record cannot say about them is
whether they were weighed.

*Refuted if* it is rare, which would say the roads not taken are in the record and it is only their
*reasons* that are not — a narrower requirement, and a better one.

**P5 — `run-reader` is different from the seven writers.** The only run that read rather than wrote
should produce fewer roads-not-taken and more losses.

*Pre-registered because the corpus permits a control and it would be waste not to use it.* If it looks
like the writers, the kinds are properties of the record rather than of what the agent was doing.

## Procedure

### Phase 0 — Fix the corpus

The eight files, their digests, and their word counts, committed before any of them is classified.

### Phase 1 — Derive the categories from one run

Already done above, from `05-reconciliation`, and committed in this protocol before the other seven are
read. It is not edited afterwards.

### Phase 2 — Classify

Every claim in the eight files, as data. Verbatim text for every unhoused claim.

### Phase 3 — The guard

Written against the classification, mutated, and seen red for the named reason.

### Phase 4 — The second classifier

**A subagent that has not read this protocol or the charter** re-classifies a sample of one file
against question 1 only — *is this claim readable off the record?* — with the four files and no prose.
Agreement is reported as a number, including when it is bad.

*This is the row's answer to its own hazard*, and it is the cheapest honest instrument available: the
person classifying wants H4 to be true, and no amount of care substitutes for a reader who does not.

### Phase 5 — Answer the five predictions

From what the classification says, including the ones answered *no*.

## Success criteria

1. All eight files classified, coverage derived from the filesystem rather than asserted.
2. Every housed verdict names a carrier that the kernel actually has.
3. Every unhoused claim quoted verbatim, so the classification is disputable without the corpus.
4. The five predictions answered, including *no*.
5. The second classifier's agreement reported as a number.
6. Nothing added to the engine or the application, and no representation named.
7. The requirement stated as a requirement, not as an obligation.

**Criteria 3 and 5 are what make this an experiment** rather than a well-argued reading.

## Failure conditions

* **Categories widened during classification to absorb what did not fit.** The five are fixed by this
  document. What does not fit is unclassified, and a large unclassified count is a *result*.
* **The corpus read for confirmation.** If the reading begins with `05-reconciliation`, whose kinds are
  already known to fit, the order does the arguing. The seven are classified first.
* **Housed used to mean *could be reconstructed with enough work*.** The test is a reader with the four
  files and no prose, not a reader with the crate and a week.
* **The second classifier briefed on the hypothesis**, which would make agreement meaningless.

## Excluded

* **Any representation.** Named nowhere, compared nowhere.
* **Cost.** It is H4's next question and it needs this one's workload first.
* **The engine.** As always.
* **Whether the agents were right.** A false claim is classified by kind like any other.

## The hypothesis this bears on

H4, first half. And it is the first experiment in the laboratory whose material was produced by
somebody who could not have known what it would be used for — which is worth naming, because
[`CHARTER.md`](../../CHARTER.md) records H4 as emergent and this is the evidence that the corpus was
not arranged to suit it.
