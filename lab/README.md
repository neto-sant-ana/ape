# Laboratory

This directory holds the questions. `core/` is the engine and `cli/` is the reference application;
neither of them asks anything — they are what the answers were built into.

`ape` defines operational meaning. An experiment exists to apply exactly one pressure to it and
report what happened.

There are two rows, and they differ in **what they are the subject of**:

```text
frontier/   what must an application do to carry that meaning across a process?
agents/     does the meaning survive a caller the engine was not designed for?
```

Named after where the pressure is applied, which is why neither is named after a crate. `frontier/`
studies the join between what the engine means and what an application must carry; the reference
application is its instrument, not its subject.

Both depend on `ape-cli` and neither is depended on by it. What differs is what each may ask of it,
and the difference is not a matter of trust:

* **`frontier/` produces the obligations the application implements.** A finding here is *meant* to
  become application behaviour — that is what the row is for. It becomes one by a reviewed change that
  records which experiment earned it, and `cli/tests/pedigree.rs` in the application refuses a claim
  whose experiment did not reach the verdict it cites.
* **`agents/` measures the application as a subject.** A need it finds is recorded as a **request**,
  verbatim, and handed over — never built there. An experiment that repaired what it was measuring
  would be measuring its own repair.

---

# Rules

Every experiment here obeys the same constraints.

**The engine is the subject, not the material.**

An experiment may not change `ape` to make itself succeed. If it needs a change, that need
*is* the result, and it belongs in the record before any code is written.

**Dependencies run one way, and never back.**

```text
experiment
    ↓
 ape-cli      ← the reference implementation, where an experiment needs one
    ↓
   ape
```

The CLI never depends on an experiment. An experiment may depend on the CLI, and this rule said
otherwise until the agents experiment had run three times.

It said *never sideways*, which was right while the CLI was a sibling asking its own question
and had nothing an experiment could stand on. Six concluded experiments later it is not a
sibling — it is the reference application, and the agents experiment measured a weaker boundary
than the real one by declining it. One of its published results held only because an in-memory
adapter has no writable record to forge.

The change is recorded rather than quietly made, and what the rule was protecting is kept:

* **An experiment may not change the CLI to make itself succeed**, exactly as it may not change
  the engine. If it needs something, that need is the result, and it is handed over rather than
  taken. This has happened once already, and worked.
* **Independent failure becomes a pinning discipline.** A run records the commit of every layer
  beneath it. A CLI change that moves an experiment's result must read as a consumer breaking,
  never as the result having always been different.

**A friction is evidence of a want, not of a need.**

The rule above says an experiment may not change the engine to make itself succeed, and the agents
row's constraint says to let the friction show what is missing. Neither of them says a friction is a
mandate, and the difference decides whether a finding may touch the ontology at all.

The test is the ontology's own, in [`../core/src/docs/01-ontology.md`](../core/src/docs/01-ontology.md):
*a concept enters only if operational coordination cannot be represented without it.* Not whether a
caller wanted it named, and not how many callers wanted it — which is exactly the shape the evidence
here arrives in, because a friction is by construction somebody reaching for something.

It has already been applied once, retroactively. Three agents reached for a way to say *acting for* —
that one Agent decides on another's behalf — and all three said the nearest primitive was not the same
claim. Read as a friction, that is three independent witnesses to a gap. Read against the test, it is
three agents who **were** agents and wanted to say whose behalf they acted on: coordination between
commitments and events is representable without it, so it belongs to an application. The record keeps
the friction, and the ontology does not grow.

**A need is handed over, and the experiment waits for it.**

An experiment that finds the application missing something does not build it. The need goes to the
branch that owns the application, and where the need blocks, the experiment stops until that work
lands.

A command running a feasibility report after an applicability one was the example this rule was
written for, and it is now the example of the third guard below rather than of the rule: it was held
back until the fourth experiment had run, because an application that runs the second report for its
caller ends any question about whether the caller would have asked. The measurement exists, pinned to
the version it met, so the work is no longer blocked by it. It is still the application's, and no
experiment writes it here.

Three things keep the rule from being either ceremony or an excuse.

*What counts as blocking.* Whether the experiment can still measure what it set out to measure. A
floor-only resource constraint was not expressible in the journal, and the world was restated as a
range whose ceiling nothing reaches — with every number shown unchanged, which is what made
restating it legitimate instead of convenient. A restatement is available exactly as far as it can be
*shown* not to move the measurement, and the showing is the price.

*The pressure runs one way.* Waiting is expensive and a workaround is cheap, so the temptation is
always to find the need non-blocking. That is the same move as reclassifying an obligation downward
until it evaporates, and the guard against it is the paragraph above: demonstrate that nothing moved,
or wait. Asserting it is not demonstrating it.

*And handing over can destroy the measurement.* Work that closes a gap makes the gap unmeasurable —
an application that runs feasibility for its caller ends any question about whether the caller would
have asked. So an experiment whose subject *is* a gap runs before the work that fills it, its result
pins the version it met, and the finding becomes the reason the work exists rather than a casualty of
it.

This has happened once already, in both directions. A finding about a decision's coordinate went from
here to the branch that owned it, was refuted there in its first form, and was repaired on that
experiment's terms — and the repair came back as a substrate this one now stands on.

**A concluded experiment is pinned, not frozen.**

An improvement to `ape` or `ape-cli` that a later experiment earns is not blocked by an earlier
experiment having been run. Three different things are preserved under that one word, and only one of
them can veto anything:

```text
a published claim        "23,553 bytes of lineage", "six of six refused"
                         kept true by the commit it was taken against
                         → never a veto. The obligation is to record the change

a runnable arrangement   a subject and its suite, which `cargo test` runs today
                         kept honest by the compiler and by red
                         → not a veto, a bill — and it must read as a consumer
                           breaking, never as the result having always been different

an irreproducible        a record that exists because something happened once
artifact                 → the only real veto
```

There are two of the third kind, both in `agents/`: the repositories two LLM parties wrote in
`04-multiagent`, read as data by that row's suites. Nobody can re-run the parties that produced them.

And even that veto dissolves rather than standing. A **recorded migration, with the file as its author
wrote it kept beside the migrated one**, translates a record whose content does not change —
re-encoding is not re-running, and what must survive is what the parties chose.

*What is not preservation, and is mistaken for it.* Whether a change forces a concluded arrangement to
express something the experiment was built to leave unexpressed — a decision that claims nobody, a
selection with an empty half. That is a design question about the change, not a claim by the past, and
it decides more cases than the veto does.

**Why this rule is written down.** One field's shape was justified by the fear instead of by the
design. `Taken::by` is optional, and the reason recorded beside it was that four concluded experiments
hold repositories whose decisions name nobody. They hold none: every suite in `frontier/` builds its
repository from its own subject, on every run, in a temporary directory. The conclusion was right for a
different reason — a decision that claims nobody is the ordinary case — and the reason taught the wrong
rule, which is how it came to be offered as a veto over a later experiment's remedy. Found while
deciding Part B of `07-atomicity`, and the sweep that followed found no second decision made that way.

**A failed experiment is a result.**

The reason something did not work is worth more than a passing test obtained by relaxing
the question until it passed.

---

# Record

Each experiment is a directory holding its own protocol, its numbered observations as it
produces them, and its result.

The protocol is written before the experiment runs, and is not edited afterwards to match
what happened. Observations are appended. The result is written last.

Directories appear as their experiment begins. An experiment that has been planned but not
started has no directory — its absence is the honest signal that nothing has been observed
yet.

A **candidate** is different from a planned experiment, and has a file rather than a directory. It is
what the findings have accumulated for a question **nothing else is holding** — gathered so that it is
not five times inherited and never held. A candidate names no shape and predicts nothing; the protocol
is written when its experiment begins, with these in hand.

*Nothing else is holding it* is the test, and it was **ownership by neither row** until the queue was
first swept. Four of the five candidates turned out to be owned by a row and dropped by it anyway —
inherited by result after result, or decided in conversation and written down nowhere. Ownership was
never what made a question get lost; being nobody's to hold was.

They live in [`candidates/`](candidates), numbered as experiments are.

* **[`00-authenticity`](candidates/00-authenticity.md)** — what signs a record, and who holds the key.
  Every result since corroboration names it and each says *unchanged*.
* **[`01-veracity`](candidates/01-veracity.md)** — whether a record that agrees with itself thereby
  says something true. Asked identically by two protocols, answered by neither, and measured once
  before either asked. **Answered by experiment 11: no.**
* **[`02-scale`](candidates/02-scale.md)** — where a unit lives, in an engine that deliberately will
  not name one.
* **[`03-bounds`](candidates/03-bounds.md)** — a floor with no ceiling cannot be stated, so the
  workaround admits a bound that is false.
* **[`04-training`](candidates/04-training.md)** — whether this structure is a harness for training
  decision reasoning. The only one that asks what a record is *for* rather than what it must hold.
* **[`05-witness`](candidates/05-witness.md)** — whether a record can say what a decision *depends on*
  rather than what happened to have been admitted. Four arrivals, one of them asking it as cost and three
  of them, in one experiment, looking like three separate candidates.

And [`QUEUE.md`](QUEUE.md) is the whole of what can be worked on: every open item, where it was named,
how many results named it, and which kind it is. It holds no question of its own and restates no
result — what it holds is the count and the classification, because a candidate inherited and never held
is the failure a candidate file fixes one item at a time, and the queue is that failure one level up.

---

# Index

**[`frontier/`](frontier)** — what an application must do to persist, reconstruct and reason over APE's
meaning outside the engine's memory. Its concluded experiments keep one directory each in
[`frontier/docs/`](frontier/docs), and their subjects and phases are the `ape-frontier` crate.

One document there is not an experiment: [`MERGING.md`](frontier/docs/MERGING.md), the assembled answer
to whether two records can be merged. **A question seven experiments each answered a piece of had
nowhere to be answered**, so it kept being asked — and a numbered directory is the wrong shape for an
answer nobody ran a protocol to get. It restates none of them and points at all of them, which is the
same discipline [`QUEUE.md`](QUEUE.md) keeps one level up. If a second assembled answer arrives, it
goes beside that one.

**[`agents/`](agents)** — whether an autonomous LLM agent can express, evaluate and later
reconstruct an operational decision through APE without extending its ontology, and whether several
of them can coordinate through it without the engine inferring what any of them meant.

---

# The agents row is on stand-by, and that is a decision

Its last experiment concluded on 17/08/2026 and nothing has moved since. **That is deliberate, and a
row that is parked reads exactly like a row that was abandoned** — which is why it is written here
rather than left to be inferred from the dates.

## Why

**Because the substrate has already moved under it once, and the cost is on the record.** The row's own
[re-run debt](agents/00-question/05-the-re-runs-that-did-not-happen.md) is three caveats that the CLI's
repository made liftable and nobody lifted. Its experiments 01 to 03 are results of an in-memory
boundary, pinned at engine `db3f965`; 04 is a result against the repository. Running more agent
experiments while the frontier row keeps changing the application would produce more debt of exactly
that kind, and the debt document already says a re-run *"would make three sentences more precise"* —
which is not worth paying for twice.

**And the frontier row was not finished changing the application.** What each experiment added to
`cli/src` is the table under *Corrected twice* below — one set of facts serving two arguments, and
therefore held in one place. It was written out twice until experiment 17, and by then the two copies
had begun to disagree about how many rows there were.

That is the reason as it stood when the row was parked. It no longer holds; see *What resumes it*.

## What resumes it

> **When the coordinate question is answered** — the shape changes and settles, or it is refused with
> a published reason.

*What a decision records about where it was taken* is one question with four faces on the board, each
named by a concluded result: a coordinate that is not a position (01, 05, 11, 12), a witness of the
journal's extent (07), recording that a decision was carried (12), and whether an address should say
when the entry was recorded (11). They are the four items that would change what a `Taken` or an
`EntryId` **is**.

**One of the four is answered, and the condition asks for the question.** Experiment 13 refused a
repair to `Taken` with a published reason — a completed reference costs one recording instant per
witnessed entry, and a recording instant is the one value in a repository that nothing derives, so
what a completed pin buys is a claim no receiver can weigh. That is the ripest face and the most
expensive to defer, and it is settled: `Taken` does not grow a pin.

**Two of the four are answered.** Experiment 14 ran the fourth — *whether an address should say when
the entry was recorded* — built it, measured it, and reverted it. An address must stay a function of
the entity it names, and a dated one is not: for seven of the nine kinds the port does not hand the
instant over, though the adapter holds it. So completing that change is a port widening — and an
obligation every adapter would newly owe — which needs the engine, and its own boundary excluded one.

**Three of the four are answered.** Experiment 16 ran *a witness of the journal's extent* (07),
answered it **yes**, and built it: `custody.json`, the addresses the journal comes to, derived by the
whole write and compared on every read. It changes what a repository **is** and leaves `Taken` and
`EntryId` untouched, which is why it could run while the row was parked — a fourth file adds a claim
beside the two that already exist rather than moving either of them.

That is also the face that produced the sharpest reading of the shape: the uncovered region is a
**fixed point** of a record's life, not a lag. Covering a tail is an operation that opens a new one.

**All four are answered, and the condition is met.** Experiment 17 ran the last of them — *recording
that a decision was carried* (12) — and **closed** it rather than answering it. Naming where the
intention came from does not work, because an author and a relay are byte-identical in all four files,
so the name does not distinguish the source from somebody the source had it from and the writer has no
way of telling which it is saying. A flag does not work either, because a record that took and a record
that never met anybody are also byte-identical. What survives is a request for a **vocabulary** — the
record has a name for a party and a name for a world and none for a record — and that is not a change
to `Taken`.

```text
13   a coordinate that is not a position          refused, with a published reason
14   an address that says when it was recorded    built, measured, reverted
16   a witness of the journal's extent            answered YES, and built
17   recording that a decision was carried        closed, with a published reason
```

> `Taken` and `EntryId` are unchanged by all four. **The shape settled**, and the row's own condition
> is what says so rather than anybody's judgement that enough time has passed.

**What did change under the row while it was parked**, and it is one thing: experiment 16 added a
fourth file to the whole write. A new agents run writes `custody.json`; the four repositories nobody
can re-run do not have one and were measured rebuilding unchanged. That is additive, and it is the
whole of the substrate movement this stand-by was protecting against.

**Resuming is the operator's call, not this file's.** What this section owed was to say when the
condition was met, and it is met.

**Corrected twice in six days, and both times for the same reason** — the condition kept measuring
something adjacent to what it is for.

It first read *two consecutive frontier experiments that add no behaviour to `cli/src`, the second of
them after the operation that comparison needs*. The clause went first: experiment 12 found that a
partial meeting computes and its result is not a repository, so there is no operation to wait for.

Then the rest went, and this is the part worth keeping. Counting quiet experiments measures the wrong
thing, and front-loading the large ones makes it worse rather than better:

```text
07 08   the generation machinery and the turn's comparison    the substrate as it now stands
09      declined its remedy                                  no behaviour
10      declined its Part B — and the refusal was the finding no behaviour
11      one comparison in `converge`                          ~37 lines
12      declined its Part B                                   no behaviour
13      refused its Part B on the measurement                  no behaviour
14      built its change, measured it, and reverted it         no behaviour
15      refused its Part B on both halves                      no behaviour
16      built its Part B — a fourth file in the whole write    ~90 lines
17      refused its Part B on two of three conditions          no behaviour
```

**The largest experiments produced the least code.** The bigger the change an experiment weighs, the
likelier its measurement declines to justify it — so a counter of quiet experiments fires *earliest*
exactly while the shape is least settled. Experiment 12 was the largest of the row and moved that
counter to one of two by building nothing; 13 would have moved it to two of two, and 13 is the
experiment that settled the ripest face of the question this condition is about.

What invalidates an agents-row measurement is not the application having changed recently. It is the
**shape** changing, because an agents subject compiles against `cli` and is measured through it. So
the condition is about the shape, which is stricter today and cheaper to reach: one experiment rather
than two silences.

**The honest limit:** no shape condition is proof against a fifth face arriving. This one is about the
faces already named by concluded results, which is the recorded cut — without it the condition reads
*never*, which is the defect the first version had.

**And a second limit, found by running into it rather than by foreseeing it.** The condition is about
what a `Taken` or an `EntryId` **is**. Experiment 16 changed neither and changed the repository: a
fourth file, `custody.json`, in every whole write. So it ran while the row was parked, legitimately by
the letter — and the letter and the reason are not the same size, because the reason given is that *an
agents subject compiles against `cli` and is measured through it*, which a fourth file also touches.

What makes it harmless here is measured rather than assumed: the file is **optional on read**, and all
four of `lab/agents/04-multiagent/run-*/repo` were measured rebuilding unchanged, with that suite's
seven tests green. An additive change to the repository is not what invalidates an agents measurement;
a change to what a value **means** is. But nothing in the condition says so, and this is the first time
anything has tested the difference — recorded here, and left as a decision rather than amended,
because a condition that the thing it constrains gets to relax is not a condition.

Not *the queue is empty*, which never happens: experiment 11 alone added three rows to it. The
condition is about the application's surface, because that is what an agents-row subject compiles
against and what its measurements are taken through.

## What is not parked

**The guard that would refuse the row's work.** `cli/tests/pedigree.rs` resolves a citation's verdict
out of a result document, and it read the frontier row alone — so nothing the agents row earns could
be cited by the application, and the divergence in how the two rows wrote a verdict was nobody's
error. Fixed while the row is parked, which is the right time: it is a defect in a frontier-row guard
rather than agents-row work, and doing it now means nobody meets it mid-experiment.

**Saying what the row's state is**, which is this section. A stand-by that is not written down is
indistinguishable from a stall, and the point of parking a row is that somebody else can rely on it
staying parked.
