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

Every experiment here obeys the same three constraints.

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

---

# Index

**[`frontier/`](frontier)** — what an application must do to persist, reconstruct and reason over APE's
meaning outside the engine's memory. Seven concluded experiments, whose record is in
[`frontier/docs/`](frontier/docs) and whose subjects and phases are the `ape-frontier` crate.

**[`agents/`](agents)** — whether an autonomous LLM agent can express, evaluate and later
reconstruct an operational decision through APE without extending its ontology, and whether several
of them can coordinate through it without the engine inferring what any of them meant.
