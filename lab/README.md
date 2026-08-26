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

**[`agents/`](agents)** — whether an autonomous LLM agent can express, evaluate and later
reconstruct an operational decision through APE without extending its ontology, and whether several
of them can coordinate through it without the engine inferring what any of them meant.
