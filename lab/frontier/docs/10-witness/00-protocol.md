# Witness

## Abstract

A persisted decision is a `Taken`: the decision, the entry that was most recent when it was applied, and
the **set of every entry that had been admitted**. The last of those is the witness, and it is the only
part of a repository that is neither an input nor a derivation of one — it is a claim about a moment.

Four experiments have now reached the same sentence from four directions, and three of them reached it in
one afternoon without noticing they had.

```text
06 exploration   as COST — 18,226 of 23,553 lineage bytes are witness, and the same
                 thirteen decisions taken later cost 21% more for having enumerated
                 before judging. What a decision costs is driven by something the
                 decision is not about

09 collision     as CAPABILITY, three times — the union of two repositories' knowledge
                 is admissible and is refused; knowledge cannot be taken without the
                 lineage that witnessed it; and one decision taken twice is two records,
                 so agreement is representable only as repetition
```

The mechanism is two lines, and they are the whole of the subject:

```rust
if let Some(unexpected) = offered.difference(&witnessed).next() {   // UnwitnessedKnowledge
if let Some(missing)    = witnessed.difference(&offered).next() {   // WitnessedKnowledgeAbsent
```

Two directions, so the witness is a test of **set equality** and not of containment. A decision is
therefore a fact about *exactly* the prefix that stood — and nothing in the record says which part of
that prefix the decision was about.

---

## Question

**Can a record say what a decision depends on, rather than what stood when it was taken — and keep what
the witness buys?**

The second half is not a caveat. Three experiments concluded the witness is load-bearing, and the last of
them because it is what makes knowledge **append-only in a checkable way**: every standing decision names
the entries that stood, so a journal whose contents moved makes those decisions disagree with it. That is
the refusal convergence declined to call redundant and three experiments have used since.

So this is not *may the witness be weaker*. It is whether **dependence** — the knowledge a decision's
world is actually a function of — is a claim the record can hold in its place, and what is lost if it is.

---

## Hypothesis

```text
dependence is narrower     the entries a decision's world is a function of are a strict
                           subset of the entries that stood, and derivable from the record
                           without asking the writer

and the difference is      the entries a decision does NOT depend on are exactly the ones
what the witness is for    whose disappearance the witness exists to refuse — so narrowing
                           it buys three capabilities and sells the guarantee
```

The two are in tension by construction, and the tension is the experiment. If the second holds, the
answer is **no** and the finding is why — that a witness is broad *because* the broad part is the
load-bearing part, and that *what I depended on* and *what was there* are two different facts a record
needs both of.

**A refusal is a first-class outcome here**, and the protocol says so before running because the pressure
runs the other way: four arrivals wanting the same thing is exactly the situation in which an experiment
talks itself into a remedy.

---

## Pre-registered predictions

Five, written before anything runs, each with the observation that would refute it.

**W1 — Dependence is narrower than presence, and derivable.**

*Prediction:* for every decision in the arrangement, the set of entries its world is a function of is a
**strict** subset of the entries that stood. And it is derivable from what is already on disk: an entry is
addressed by `EntryId::of(id)` — the identity admitting it produced — so from a world's selection and its
cut, the entries reached are a closure over identities rather than a guess about positions.

*Refuted if* the two sets coincide for any decision, or if the closure needs something the record does not
hold.

**W2 — The difference grows with the journal, and the dependence does not.**

*Prediction:* the broad witness grows with everything admitted before the decision; the dependence set
grows only with what the decision names. So the gap is not a constant overhead — a decision taken later
in the same history has a larger witness and the **same** dependence.

*Refuted if* the dependence set grows with the journal too, in which case the cost half of this question
dissolves.

**W3 — Dependence recorded in its place admits all three blocked cases.**

*Prediction:* with dependence compared instead of presence, and nothing else changed: the union of two
twinned repositories rebuilds; a repository can take another's journal while keeping its own lineage;
and two records of one decision become **one** record, so the duplicate world of the collision
experiment never arises.

*Refuted if* any of the three is still refused, which would mean the witness was not what blocked it.

**W4 — And it stops refusing something the broad witness refuses.**

*Prediction:* knowledge that was present when a decision was taken, was not depended on, and is later
**removed** is refused today by name — `WitnessedKnowledgeAbsent` — and under dependence is met with
silence. So the trade is not free, and this is the half a remedy would have to pay for.

*Refuted if* nothing the broad witness refuses becomes undetectable, in which case the witness has been
carrying a guarantee it does not provide.

**W5 — The readmission diagnosis survives, or the narrowing costs a named cause.**

*Prediction:* `ReadmittedEntryIsAmbiguous` exists because a witness can outrun a replay for a reason
worth naming, and it was earned by relocating a diagnosis to the layer that held the fact. Under
dependence it must still be reachable.

*Refuted if* it cannot be produced, in which case narrowing the witness costs a refusal that names a real
cause — which is the shape the authenticity candidate warns about.

**W3 with W4 is the experiment.** Each alone is half a trade. Together they say what a narrower witness
buys and what it sells, in the same arrangement and the same units.

---

## What is carried forward, and is not available as a finding

Usable in reasoning, and not reportable as a result:

* **The witness is a set, and reordering is already invisible to it.** Two entries swapped inside a
  prefix change neither the set nor the entry the decision names, which the divergence experiment
  measured as *a coordinate that is wrong but well-formed is not detectable from the record*.
* **A record can drop what nothing points at, and a reader can find only what something points at** —
  one predicate, read from two sides.
* **A world is identified by its parent, its cut and its selection**, all content-addressed, and is blind
  to every admission except the ones its selection names and the Events its cut resolves against.
* **A cut resolves against the Event chain that stood**, so knowledge inserted before a decision may move
  the world it produces.
* **A party that cannot converge writes nothing**, and a rebuild admits the journal in step with the
  lineage rather than wholly before it.
* **An entry is addressed by the identity admitting it produced**, which is what makes the same knowledge
  the same entry in two repositories that never met.

Before any sentence here is allowed to be a finding, ask what would have to be false for it to be false.
A founding premise makes it a corollary; a documented design decision makes it a reading; an
implementation fact whose composition is unstated makes it a finding.

---

## Motivation

The candidate is held in [`candidates/05-witness.md`](../../../candidates/05-witness.md), and the reason
to run it now rather than next is that the previous experiment did the reduction. Three requests that
would have entered the queue as three items are one, and the fold was found by review rather than by
sweeping — so the question arrives already knowing what change would answer it and which line refuses it.

It also goes **before authenticity**, which is older and has been named by eight results. That is a
deliberate ordering and the reason is the ontology's test: coordination is representable without a
signature, so authenticity's remedy belongs to an application and its likely result is a **boundary**.
Boundaries keep. This one has a measured mechanism, a measured cost, and three consequences that one
change closes — and if the answer is *no*, it closes four arrivals with a reason instead of leaving them
to arrive a fifth time.

**This runs before any remedy**, for the reason four protocols have now given: work that closes a gap
makes the gap unmeasurable.

---

## Experimental Boundary

This experiment exercises the witness as a **claim** — what it says, what it refuses, and what a narrower
claim would say and refuse in its place.

It includes:

* deriving, for every decision, the set of entries its world is a function of;
* comparing that set against the witness the record holds, decision by decision;
* the same comparison for the same decisions taken at two different points in one history, which is
  where the cost half lives;
* applying a **dependence** comparison, in the laboratory, to the three cases the collision experiment
  measured as refused;
* and the mirror: every state the broad witness refuses, checked against what a dependence comparison
  makes of it.

It deliberately excludes:

* **changing the application to make any of it pass.** The narrow comparison is an instrument in the
  laboratory. If it earns its way in, that is Part B, and it happens after the trade is measured — the
  same rule that kept `converge` out of the coordination experiment's Part A;
* **the remedy's shape.** A derived summary, a narrower set, a checkable claim of insensitivity, a second
  file — naming one before Part A would be choosing the answer;
* **removing the witness.** Not asked for by any of the four arrivals, and refused here in advance: three
  experiments concluded it is what makes knowledge append-only, and an experiment that removed it would be
  measuring a different record;
* **benchmarking.** The cost half is measured in **bytes and entries**, which is what the exploration
  experiment measured and what makes the two comparable. No timing;
* **two repositories as somebody else's**, which is authenticity and is deferred on purpose;
* **the engine.** A witness is the application's record of the application's own act. Nothing here asks
  the ontology to know it exists.

Those concerns may become later experiments. They must not influence the structure introduced here unless
this experiment itself requires them.

---

## Experimental Subject

**A subject of this experiment's own**, for the reason the ten before it give.

What it must express, and no previous subject needed:

```text
a long tail of knowledge      most of what is admitted must be knowledge no decision
no decision is about          depends on, or the gap this measures is a rounding error

one decision, two positions   the same decision taken early and taken late in the same
                              history, so that W2 is a comparison rather than an argument

a decision that depends on    at least one where an Event is on the chain the cut resolves
an Event                      to, so the dependence set is not merely a selection

removal, as a case            knowledge present at a decision, not depended on, and taken
                              away afterwards — which is W4's whole measurement

and the three refused cases   reachable, because W3 is about the collision experiment's
                              outcomes and this arrangement has to produce them
```

Everything else stays as thin as the others: integers, one quantifiable resource, no dependencies unless
the procedure demands them.

The instrument is a **dependence comparison written in the laboratory**: given a journal and a decision,
derive the entries the decision's world is a function of and compare those instead of the whole prefix.
It sits beside the phases, not inside the application, and the phases run both comparisons over the same
repositories so that every difference is a difference of the claim rather than of the arrangement.

---

## Initial State

```text
one repository     written whole by the application, with more knowledge in it than any
                   of its decisions is about, and read once to establish what it answers
two comparisons    the one the record uses, and the one this experiment writes
```

Nothing is inherited from the previous experiments except their conclusions and their code.

---

## Procedure

### Phase 0 — What the repository answers, and what each decision witnesses

Read once. For every decision: the entries that stood, the entry it names, and the world it produced.
Every later phase compares against these.

---

### Phase 1 — Derive what each decision depends on

W1. The closure, decision by decision, and the two sets side by side. What is reported is the
**difference** — which entries are in the witness and not in the dependence — because that difference is
the whole of what the rest of the experiment trades.

If the closure needs something the record does not hold, that is the result and the experiment stops
here.

---

### Phase 2 — The same decision, early and late

W2. One decision taken at two points in one history, and the two sets measured at each. The prediction is
that one grows and the other does not; the measurement is in entries and in bytes, so that it is
comparable to the exploration experiment's own.

---

### Phase 3 — Dependence in its place, over the three refused cases

W3. The collision experiment's three outcomes, reproduced and then run through the dependence comparison:
the union of two twinned repositories; a journal taken without its lineage; and two records of one
decision. What each becomes, by value, and whether the world identities are the ones the record already
held.

---

### Phase 4 — What stops being refused

W4, and it is the half that decides the answer. Every state the broad witness refuses, produced and put
through both comparisons:

```text
an entry added to a prefix, not depended on
an entry removed from a prefix, not depended on
an entry removed from a prefix, DEPENDED on
a readmission
```

Reported as a table, because the claim is about which of them survive and a phase that measured one
would be reporting a sample as a closed set.

---

### Phase 5 — Name the trade, in one sentence per side

Not a recommendation. What a narrower witness buys and what it sells, each stated as the query that gains
an answer and the query that loses one.

---

### Phase 6 — Name what an application would need, and build none of it

Requests, verbatim, in the vocabulary of the need rather than of a solution.

---

### Phase 7 — Part B, if it was earned

A repair, whose shape is decided by Phases 1 to 5 and named nowhere above. It stays only if it meets the
criterion the coordination experiment set and three experiments have inherited:

> **It removes a state a reader can be misled by, and what the repair replaces survives.**

And one condition of its own, because this is the first experiment whose remedy would weaken a guarantee
rather than add one:

> **A repair may not remove a refusal without putting the fact it protected somewhere.** If *this
> knowledge was present when I decided* stops being checkable, the honest report says which reader loses
> what — and a remedy that trades a silent loss for a capability is the shape three experiments have
> measured as the worst kind.

---

## Success Criteria

1. The dependence set is **derived** from the record, not declared by the arrangement, and the derivation
   is shown to use nothing the record does not hold.
2. Both comparisons are run over the same repositories, so every difference is a difference of the claim.
3. The cost half is measured in entries and bytes, comparable to the exploration experiment's numbers.
4. What stops being refused is stated as a closed table, not as an example.
5. The three cases the collision experiment measured as refused are reproduced before they are re-run,
   so a change in outcome is attributable.
6. Nothing is added to the APE engine.
7. Part B is built only against both criteria above, and its absence is a reportable result.
8. The nine earlier experiments' conclusions stand, or the change is recorded as a result of this one —
   and *the witness is load-bearing*, concluded by three of them, is the one at risk, so it is stated
   either way.

**Criteria 1 and 4 are the experiment.** The rest is the arrangement holding.

---

## Failure Conditions

The severe one:

> If dependence cannot be derived without the writer's help — if the record cannot say, from what is on
> disk, which knowledge a decision was a function of — then the witness is not broad by choice and this
> question has an answer nobody has to like. That is a finding, and a large one.

Its neighbour, which is this experiment's own:

> If a narrower witness turns out to admit a state a reader cannot tell from a legitimate one, the remedy
> is refused **and the state is reported**, because it belongs to the veracity candidate and would be its
> first reachable case from the writing side.

And the ordinary ones:

* a dependence set the arrangement supplies rather than derives;
* an outcome reported from the absence of an error rather than from what the record holds;
* a repair chosen before Phase 4 and justified afterwards;
* reporting the broad witness as *wasteful* when it is measured to buy a guarantee;
* four arrivals treated as four reasons to say yes. They are one question asked four times, and the
  number of times a want is expressed is not evidence of a need.

**A refuted W3 is the cheapest interesting result.** If dependence does not admit the three cases, then
the witness was not what blocked them and the collision experiment's requests need a different question —
which is worth knowing before anybody builds anything.

---

## Variables Deliberately Left Open

### Whether *what was present* deserves its own record

If both facts matter — what I depended on, and what was there — then a record might hold both rather than
choosing. Whether that is one file or two, and whether the second is derivable, is not asked here.

### Pruning

Exploration's question, and this one moves the line it sits on: what may be dropped is what nothing points
at, and this changes what points at things. The interaction is named and not measured.

### Cost as an absolute

Deferred by ten experiments. This one measures a **ratio** in the same units exploration used, which is
the nearest anything has come, and still not a number for a real history.

### Two repositories as somebody else's

Authenticity, deferred deliberately and now next in line.

---

## Methodological Constraint

> *Do not introduce an abstraction intended to solve an experiment that has not yet been performed.*

Structure may be introduced only when required by the current experimental procedure. The previous
experiments' conclusions are not revised here; where this one finds something that would have changed
them, it is recorded as a finding of this experiment, against the implementation as it then stood.

And the rules the last four experiments earned, which apply here without amendment. Before recording
anything as a finding, ask what would have to be false for it to be false. Every literal is written before
the run, and a wrong prediction is corrected in the open rather than adjusted. A prediction's
justification must quantify over as much as its claim does. A friction is evidence of a want, not of a
need.

One more, and it is this experiment's own: **a phase must not be satisfied by an arrangement it could not
have failed.** Phase 1 can only report a difference if the arrangement admits knowledge no decision is
about, and the subject is required to say how much — because an arrangement whose journal is all
dependence would report *the sets coincide* and prove nothing.

---

## Expected Pressure Points

### Four arrivals is a wish, not a warrant

The strongest force in this experiment is that everybody already wants the answer to be yes. Three of the
four arrivals are requests, and a request is somebody reaching for something. The ontology's own test
exists for exactly this shape, and it applies here even though the witness is the application's rather
than the engine's.

### The broad witness will look wasteful before it looks load-bearing

18,226 of 23,553 bytes is a striking number and it is not an argument. What those bytes buy is that
knowledge present at a decision cannot be quietly removed afterwards — which is a guarantee about
**history** rather than about worlds, and no world identity would notice its loss. Phase 4 exists to keep
that from being discovered after a remedy.

### *Dependence* is a word that will be asked to stretch

A world is a function of its parent, its cut and its selection. The closure over those is well defined.
The temptation is to extend *dependence* to mean *whatever seemed relevant* — a commitment the decider
looked at and did not select, knowledge that motivated a choice. None of that is in the record, and the
provenance experiment already measured that a record which can be rebuilt perfectly is not thereby
understood.

### A negative result closes four arrivals and will feel like a failure

If the answer is that the two facts are both needed, this experiment ends with no capability and one
sentence. That sentence would be worth more than three features, because it is the reason those three
requests are not coming back.

---

## Observations

Each observation is recorded as its own numbered document beside this one, as the experiment produces it.
Record facts rather than decisions retroactively presented as inevitable.

Useful observations include: what a decision is a function of and what it merely followed; where a
guarantee turns out to live in the difference between two sets; which refusals a narrower claim keeps;
and what the record cannot distinguish, stated as the query that has no answer.

Where possible, record the smallest reproducing case.

---

## Open Questions

* Is *this knowledge was present when I decided* a fact a record owes anybody, or an artifact of how the
  witness was implemented?
* Can insensitivity be **claimed and checked**, rather than derived — and if it can, who is entitled to
  claim it?
* If both facts are needed, is the second one derivable from the first plus the journal, or does it have
  to be written?

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

```text
This one asks whether a decision is about
everything it happened to come after.
```

Ten experiments have treated the witness as furniture — the thing that makes corroboration possible, cited
and never examined. Four of them have since asked for something it forbids. If the answer is that it is
broad because it has to be, then the record has been paying for a guarantee on purpose and nobody had
written down which one. And if it is broad by accident, three capabilities have been waiting on two lines.
