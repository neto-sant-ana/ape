# Exploration

## Abstract

To find out what an intention would lead to, the intention has to become canonical knowledge first.
Canonical knowledge cannot be removed. So an application that weighs ten candidates has admitted ten
propositions it never intended, and one that weighs ten thousand has admitted ten thousand.

> *What must an application do when history grows for reasons nobody intended?*

Two things this sequence has deferred meet here, and they are one question seen from two sides.
**Cost** has been deferred by six protocols because it had no subject: seven worlds and twenty entries
cost nothing, so measuring them would produce a number nobody could act on. **Abandoned siblings**
have been left unmodelled by six protocols, each time deliberately and each time noted — and the
provenance experiment measured a cost against them for the first time.

Exploration is the subject both were waiting for. It is the first usage pattern in which the record
grows for reasons nobody intended, and the first in which the alternatives considered and not taken
outnumber the ones that were.

---

## Question

**Can a record be bounded and auditable at the same time, under exploration?**

The provenance experiment stated one half of the answer as a consequence:

> *the only worlds a repository could ever prune are the ones nothing points at, and provenance
> points at exactly the ones a planner would want to drop.*

And the divergence experiment left the matching half open, in as many words:

> *whether auditability depends on an application forking rather than discarding remains unanswered.*

Put together, an application has three arrangements available and no fourth:

```text
A  ephemeral    fork in memory, interpret, drop the world
                journal grows      lineage does not      the deliberation is unrecoverable

B  recorded     each candidate is a decision
                journal grows      lineage grows         nothing prunes what a claim points at

C  pruned       record, then drop what nothing points at
                available only where nothing points at it
```

**A is the default, and nobody chooses it.** It is what happens when an application interprets a
candidate the cheap way, and it is the worst pair of the three: the proposition is in the journal
forever, and whether it was ever weighed is nowhere. You cannot prune what you cannot identify, and
you cannot reconstruct what you did not record.

So this is not a cost curve. It is a **choice forced by scale**, whose default option is the one no
reader can audit and no application can bound.

---

## Hypothesis

```text
monotonicity  exploration is monotonic, and a monotonic record still answers
              everything it answered before

exclusivity   bounding the record and auditing the deliberation are exclusive
              under exploration, and the default arrangement is neither
```

The first is a precondition. If exploration changes what the record answers, there is a correctness
problem in front of the cost conversation and the cost conversation waits.

The second is the experiment. If it holds, *comparison* is the one verb of construct-compare-interpret
that the substrate charges for, the charge falls on whichever of bounding and auditing an application
gives up, and the claim that planning, audit and exploration share one representation has a price with
a name.

---

## Pre-registered predictions

Six, written before anything runs, each with the observation that would refute it. This is stricter
than the five protocols before it, which stated a hypothesis and derived criteria from it — the
addition is deliberate, because this experiment produces **numbers**, and a number is the easiest
result to read backwards into whatever it turned out to be.

**E1 — Nothing derived changes.**

*Prediction:* every projection, reading, feasibility report and applicability report that was true
before exploration is identical after it, **by value**. A commitment admitted and selected by no world
enters none of them.

*Refuted if* an unselected admission moves any derived answer.

**E2 — Exploring leaves an indelible proposition and, by default, no deliberation.**

*Prediction:* arrangement A leaves the candidate's commitment in the journal permanently and leaves no
recoverable evidence that any world considered it. The two halves fail in opposite directions:
unprunable because unidentifiable, unauditable because unrecorded.

*Refuted if* an ephemeral evaluation leaves a recoverable trace of the deliberation, or leaves no
trace of the proposition.

**E3 — The record's dominant term is discarded work, and the witness is the driver.**

*Prediction:* under arrangement B the lineage grows as the product of candidates explored and
decisions taken, because each decision witnesses every entry that stood before it. The journal grows
linearly; the lineage does not.

*Refuted if* the lineage grows linearly in decisions.

**E4 — Repetition is free in canonical knowledge and not in the journal.**

*Prediction:* exploring the same candidate twice produces **one** canonical record and **two** journal
records. So canonical history grows in distinct candidates and the file grows in attempts, and the
mitigation an application would reach for — revisit ground rather than enumerate fresh ground — is
free in the history and not in what has to be stored, read and replayed.

*Refuted if* a repeat adds a canonical record, or if it adds no journal record.

> This prediction was **reshaped from the proposal**, which predicted that a repeat adds no entry at
> all and would be refuted if one did. Measured before writing this, on the current tree: a journal
> of 15 grows to 16 and then to 17, producing 17 entry addresses of which 16 are distinct, and the
> repository still reconstructs. Idempotence is a property of canonical knowledge — `put_commitment`
> answering `AlreadyPresent` — and the journal is a sequence of admission records, which nothing
> deduplicates. Stating it the original way would have refuted the wrong claim.

**E5 — Exploratory worlds are leaves, so one file can be pruned and the other cannot.**

*Prediction:* a candidate recorded as a decision is a leaf, and dropping it from the lineage **and its
witness from the worlds** leaves everything remaining reconstructible and corroborated. The journal
cannot be pruned at all. So pruning recovers one file and leaves the other whole.

*Refuted if* dropping a leaf breaks reconstruction, or if the journal admits pruning too.

> The proposal asked whether any invariant requires the lineage to be closed under descendancy, which
> would change E5's shape. **There is none, and the opposite closure exists.** `ResidentArchive::
> put_thesis` refuses a child whose parent is absent — ancestors are required, descendants are not —
> and `reading::corroborate` compares the worlds the decisions produce against the worlds recorded,
> position by position, after comparing their lengths. So pruning consistently in both files passes,
> and pruning a *middle* decision is refused by name. Already measured, in the tree:
> `coordination::a_lost_decision_is_invisible_exactly_where_nothing_refers_to_it`.
>
> One refinement the proposal did not have. Whether the surviving witnesses still **name** what was
> pruned depends on the arrangement: a decision taken *after* the candidates were admitted witnesses
> every one of them, and a lineage whose exploratory decisions are its last ones leaves, after
> pruning, only decisions that predate every candidate and name none of them. Both are valid
> repositories and they have opposite audit consequences, so Phase 4 measures **both** rather than
> assuming which one an application produces.

**E6 — Considered-and-rejected and never-considered are the same silence.**

*Prediction:* given the repository after exploration, *which propositions were ever intended* is
answerable exactly, by walking the archive; *which were considered* is not answerable at all. Under
arrangement A there is nothing to walk, under C the evidence was pruned, under B it is there and
unbounded.

*Refuted if* a query separates a rejected candidate from an incidental admission.

**E3, E5 and E6 together are the experiment.** Each alone is a measurement. Together they say whether
a record can be bounded and auditable at once, and the prediction is that it cannot, and that the
arrangement an application falls into by default is neither.

---

## What is carried forward, and is not available as a finding

Usable in reasoning, and not reportable as a result:

* **An intention must be admitted before its realizability can be examined.** An `Interpretation` is
  taken of a `Thesis` over canonical knowledge, and a `Thesis` selects `CommitmentId`s that have to
  resolve. This is the premise of the question rather than an answer to it.
* **Knowledge appends, and the port offers no other verb.** `CanonicalHistory` has `put_*` and
  `append_event`. No remove, no retract, no discard.
* **A Canon refuses an admission dated before its `recorded_through`.**
* **A repeated value is one canonical record**, because identity is content.
* **Every decision carries the entries that stood when it was taken**, and the sequence witness is not
  redundant — three experiments established that, the last by making it load-bearing for append-only.
* **A claim keeps a world alive**, and the only prunable worlds are the ones nothing points at.
* **There is no `Feasible`**; feasibility reports findings under a hypothesis the caller names.
* **Neither where an intention came from nor who decided it fits on a world.** Both are properties of
  a decision.

Before any sentence here is allowed to be a finding, it names the file it would be new to. That check
has failed three times in one experiment, each failure one layer more foundational than the last, and
none was caught by asking whether the fact was in a docstring.

---

## Motivation

A third usage is being considered — APE as the substrate something reasons over by weighing many
worlds — and it explores at a scale planning and audit never do.

```text
planning      weighs a few worlds and keeps them
audit         weighs none and reconstructs
exploration   weighs many and keeps almost none
```

The Canon exists to prevent the third. That is not a defect: it is the property every result in this
sequence rests on. But it means the claim that *planning, audit and exploration share one
representation* is a claim about one representation over substrates with opposite retention
economics, and the third has never been measured.

Exploration is named as the third rather than any particular reason for exploring. Something that
learns is one thing that would explore at this scale; so is a solver, a search, an operator asking
*what if* a hundred times. What the substrate meets is the weighing, and it meets it identically
whatever the weighing is for — so naming a purpose here would attach the result to one consumer of it.

**This experiment runs before any work that would close the gap.** An application that gives an
explorer a throwaway history, a scope it may forget at, or any other way to forget ends the question
of what exploration cost while it could not be forgotten. Run first, the finding becomes the reason
that work exists. Run second, it is a casualty of it.

---

## Experimental Boundary

This experiment exercises one application exploring candidate worlds mechanically, over one
repository, in three arrangements.

It includes:

* candidates enumerated rather than judged, admitted, interpreted under a named hypothesis, and
  scored by a fixed objective;
* the three arrangements, over one budget fixed in advance;
* pruning what nothing points at, and measuring what survives;
* the corroboration discipline as it stands, applied to every arrangement.

It deliberately excludes:

* **any reason for exploring.** No reward, no evaluator, no policy, no gradient, and equally no
  solver strategy and no heuristic. The objective is a fixed rule applied unchanged; the moment
  anything adapts, what is measured is the thing that adapts;
* **a judging explorer.** The collision is a property of the substrate under repeated exploration, and
  a planner, a solver, a rule engine or a loop would meet it identically. Exploration is performed
  mechanically so that the substrate's behaviour is not confounded with an explorer's;
* **generated Events.** An explorer that wants to be told its decision aged badly needs somebody to
  manufacture the surprise, and whoever manufactures it chooses its distribution. That is the
  sharpest dependency exploration-at-scale has on something outside APE;
* **simulated time.** Candidates recorded now and due later advance the watermark to now, once,
  however many there are — checked before this protocol was written, against an earlier draft that
  predicted otherwise. A loop recording at simulated future instants reopens it, and whoever chooses
  the clock chooses whether the watermark overtakes reality;
* **snapshots**, and any other remedy. A remedy is not measured by the experiment that motivates it;
* **benchmarking.** *Slow* is not a finding. What is reported is **which term dominates** and whether
  any term is driven by work nobody intended. `O(entries)` per read is known by inspection and is not
  a result; a dominant term made of discarded work is one, because it changes the design question;
* parallelism, atomic commit, authenticity, fractional magnitudes, mutable named references.

Those concerns may become later experiments.

They must not influence the structure introduced here unless this experiment itself requires them.

---

## Experimental Subject

**A subject of this experiment's own**, for the reason the five before it give — the proposal reused
the earlier world, and a concluded experiment keeps its own unchanged.

What this one must express, and no previous subject could:

```text
a floor that binds     so that some candidates are refused and some are not
a settled opening      so that something is frozen before exploration begins
many candidates        enumerated, with a budget fixed in advance
one objective          a fixed rule, entirely outside the engine
```

```text
cash ∈ [0, 1000]
account            the instance every intention moves
opening   +100     received, and settled by an Event
```

And the objective, fixed in code before anything runs:

> *spend as much as the account admits, and never break the floor.*

The engine contributes exactly three things to evaluating a candidate, and naming them is how nothing
else is smuggled in: **signed movements**, **feasibility findings under a named hypothesis**, and
**applicability conflicts**. Which movements count, and what *better* means, belongs to the objective
and to nobody else.

The settled opening is new. Every previous subject had nothing frozen, so this is the first
arrangement in which a cut partitions a selection into frozen and open for a reason other than
convention.

Everything else stays as thin as before: integers, one quantifiable resource, no dependencies unless
the procedure demands them.

---

## Initial State

```text
repository   the world admitted, the opening settled, one world decided
candidates   none admitted
budget       fixed, recorded, and read by every arrangement
```

Nothing is inherited from the previous experiments except their conclusions and their code.

---

## Procedure

### Phase 0 — The world, the objective, the budget

Written and recorded before anything explores, so that no measurement is taken over a number chosen
after seeing the curve.

---

### Phase 1 — Explore ephemerally

Arrangement A. Each candidate admitted, forked in memory, interpreted under a named hypothesis,
scored, and dropped.

Measured after each: journal entries, lineage bytes, worlds recorded, and the recording watermark.

---

### Phase 2 — What the record still answers

E1, by value and not by absence of error. Every reading taken before Phase 1 is re-derived and
compared field by field.

---

### Phase 3 — Explore, recording each candidate

Arrangement B, from the same starting repository, with the same candidates in the same order, so that
the two arrangements differ in one thing.

E3 and E4 measured against the budget, with the predicted shape written down before the numbers are
read.

---

### Phase 4 — Prune the leaves

Arrangement C. Drop the exploratory decisions and their witnesses, rebuild, and measure what
survives — in **both** the arrangement where a decision follows exploration and the one where the
exploratory decisions are last, because E5's second half differs between them.

---

### Phase 5 — Ask the repository what it can say

E6, mechanically and as a test: the query that answers *which propositions were ever intended*, and
the absence of any query that answers *which were considered*. A closed set of what each arrangement
can and cannot report.

This is a departure from the proposal, which put an agent here. Every phase in this sequence is a test
because a comparison has to fail loudly, and an agent's reading cannot fail loudly. What an agent
recovers is a question about presentation rather than about the record — worth asking, and recorded
separately as an observation that **is not a success criterion**.

---

### Phase 6 — Name what an application would need, and build none of it

Requests, verbatim, in the vocabulary of the need rather than of a solution.

---

## Success Criteria

1. Exploration is expressible through existing primitives, with none added and none repurposed.
2. Candidates are evaluated by the engine deriving consequences, never by the harness asserting them.
3. The objective and the hypothesis live outside the engine, and the record shows neither.
4. Everything the record answered before exploration, it answers identically after — by value.
5. The three arrangements are measured against one budget fixed in advance, and every literal is
   written before the run.
6. What each arrangement can and cannot report is a closed set, checked by a test.
7. Nothing is added to the APE engine.
8. The six earlier experiments' conclusions stand, or the change is recorded as a result of this one.

Criteria 1 to 4 are the arrangement holding. **Criterion 5 with 6 is the experiment.**

---

## Failure Conditions

The severe one:

> If exploring candidate worlds requires adding to the kernel anything resembling `Trial`, `Discard`,
> `Retract`, `Scratch`, `Ephemeral`, `Reward` or `Score`, or requires a history that can forget, then
> the claim that planning, audit and exploration share one representation has met a real friction.

`Candidate` is deliberately absent from that list. `CandidateSelection` already exists in Synthesis
and means something else — the world a transfer would produce, held only long enough to be judged.
Reusing the word for an exploratory intention would be ontology growth by **repurposing**, which is
the dangerous kind because it compiles.

And the ordinary ones:

* a projection that changes because of something nobody intended;
* a record after exploration that cannot be reconstructed;
* exploration that cannot be performed without the harness asserting a consequence the engine should
  derive;
* a cost that is a cost of the harness's way of exploring rather than of exploring, with no way to
  tell the two apart;
* a measurement whose literals were written after the numbers were seen.

**A refuted E1 stops the experiment** rather than narrowing it. The other five can be refuted and
still leave a complete result.

---

## Variables Deliberately Left Open

### Whether the journal should deduplicate

E4 predicts that a repeat costs a journal record and no knowledge. Whether an application ought to
drop the duplicate on the way in is not decided here. It would be the first place the CLI's journal
stops being a faithful log of what was supplied, and that is a trade this experiment measures rather
than makes.

### What an exploratory decision is, if anything

Arrangement B records each candidate as an ordinary `Fork`. Whether an application wants a decision
that says *I considered this* as distinct from *I intend this* is exactly the ontology growth the
severe failure condition guards against — so the procedure does not reach for one, and if the result
needs one, that need is the finding.

### Abandoned siblings

Deferred by six protocols. This is the first arrangement in which they are the majority of the record
rather than an exception in it, and the first with a measured cost on both sides of the choice.

### Cost, as an absolute

Deferred by six protocols and still not answered here. What this measures is **which term dominates**
and what it is made of. How long a replay takes on a real history is a different question, and it
needs a history nobody has.

---

## Methodological Constraint

This experiment follows one implementation rule:

> *Do not introduce an abstraction intended to solve an experiment that has not yet been performed.*

Structure may be introduced only when required by the current experimental procedure.

The previous experiments' conclusions are not revised here. Where this one finds something that would
have changed them, it is recorded as a finding of this experiment, against the implementation as it
then stood.

And the editorial rule the last three experiments needed: before recording anything as a finding, ask
whether it is written in a docstring in `core/`, **and** whether the behaviour being described would
be better the other way. A defined behaviour reported as friction sends the next reader looking for a
repair nobody needs.

One rule specific to this one, because it produces numbers: **every literal is written before the
run, and a wrong prediction is corrected in the open rather than adjusted.** A number written after
the fact is not a measurement.

---

## Expected Pressure Points

### The watermark moves, and something might read it

E1 says nothing derived changes. The one thing that provably does change is `recorded_through`, and
the question is whether any derived answer depends on it. `ApplicabilityConflict::
HistoricalUnavailability` compares a commitment's `recorded_at` against a Target's `known_at`, so a
report is the place to look — and an unselected candidate is in no difference, which is the reason to
expect E1 to hold rather than an argument that it does.

### The witness is quadratic and was chosen on purpose

Three experiments concluded the sequence witness is load-bearing, the last of them because it is what
makes knowledge append-only. E3 predicts it is also the dominant term. Both can be true, and if they
are, the experiment reports a cost against a guard it is not proposing to remove.

### Pruning is a repository operation with no code

Nothing in the application prunes. Phase 4 does it by writing the two files, which is the same
technique the corroboration experiment used to tamper — and the difference between tampering and
pruning is intent, not mechanism. Worth stating, because it means the repository cannot tell them
apart either.

### The default arrangement is the one nobody writes down

Arrangement A is what an application does by accident. It has no code to inspect and no record to
read, so measuring it means measuring an absence — and an absence is the easiest thing to report as
whatever the author expected.

---

## Observations

Each observation is recorded as its own numbered document beside this one, as the experiment produces
it.

Record facts rather than decisions retroactively presented as inevitable.

Useful observations include:

* terms that dominate, and what the work in them was for;
* places where a guard's cost and its necessity are both real;
* what a repository cannot distinguish, stated as the query that has no answer;
* mitigations that turn out to be free in one representation and not in another.

Where possible, record the smallest reproducing case.

---

## Open Questions

* What does a leaner witness give up, and is the thing it gives up one of the two it currently buys?
* Is there a representation of *considered* that a reader can bound without an application choosing
  what to forget?
* Does the scope an application may forget at belong to the application or to the repository?
* What manufactures the surprises an explorer needs in order to be told it was wrong, and who chooses
  their distribution?

These are candidates for later experiments.

They are not requirements for this one.

---

## Experimental Principle

Reconstruction asked whether meaning survives.

Divergence asked whether deliberation survives.

Corroboration asked whether the record can be trusted without being believed.

Convergence asked whether two lines of deliberation can still reach each other.

Provenance asked whether a record that can be rebuilt perfectly is thereby understood, and answered
that it does not need to be.

Coordination asked whether a record built for one mind can hold two, and answered that the second one
gets a name and not a proof.

```text
This one asks what a record owes to the
worlds nobody chose — and whether it can
afford to owe them anything.
```

The claim under test is that APE is a representation for operational reasoning, and that planning,
audit and exploration are one mechanism at three moments. The first two are measured and they hold.
What this asks of the third is mechanical: **comparison requires construction, construction is
admission, and admission is irreversible.**

Whatever the exploring is *for* comes after that and is somebody else's question.

If exploration is monotonic and the monotonic record still answers everything it answered before,
the representation really is common and the cost is arithmetic — payable, and an application's
problem.

If bounding the record and auditing the deliberation turn out to be exclusive, then the beautiful
claim has a price, and the experiment's job is to say whose it is.
