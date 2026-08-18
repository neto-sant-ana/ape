# The substrate changes

The three experiments ran over the engine's in-memory reference adapters. They will run again over
the CLI's repository.

This document exists because a re-run whose protocol is written after the first results are known
is not a re-run — it is a confirmation. So what the change obliges is written down before any of it
moves, and the predictions are recorded where they can be wrong.

---

## Why the substrate was wrong

Not because in-memory was a shortcut. Because it was making one result better than it is.

Experiment 02 concluded that the counterfactual audit is *the half that cannot be falsified by the
party being audited*. That held, over an adapter with no durable record — there was nothing to
forge. The CLI's corroboration experiment then measured what happens when there is: a **consistent
forgery**, where a repository is edited and every derived value recomputed from what was written,
is not refused, and a different lineage comes back with the refusal that mattered simply gone.

An experiment cannot claim unfalsifiability against a substrate that offers no way to falsify. That
is the whole reason for the move, and it is a correction rather than an upgrade.

---

## What the substrate now is

Six concluded CLI experiments, and the parts this experiment stands on:

```text
journal      the admission sequence, entries addressable by the identity each produced
lineage      a tree of decisions — Genesis, Advance, Fork — each carrying `extends`
Taken        a decision, plus where it was taken:
               after     the journal entry that stood at that moment
               witness   the whole prefix, as a set, so a reader can disagree
               by        the party that took it, optional, an AgentId
archive      ThesisArchive over the repository
reading      reconstruction, comparing every persisted derived value on every read
```

Two of these answer requests this experiment made and did not take: a decision's position in the
admission sequence, and `Fork`. Both were built by the experiment that needed them, on its own
terms, and the position was solved better than the shape proposed here — as a reference to an entry
rather than an index into a sequence.

---

## What must be re-verified before anything runs

**The engine moved.** `AgentKind` was removed from the kernel; an `Agent` is a label. The
pre-registration's *Known before the run* section describes a boundary that no longer exists in
that detail, and it is re-checked against the pinned engine rather than edited from the news.

**F1 was not refuted. It was malformed.** The prediction asked whether the deciding program has an
honest `AgentKind`. The program is not in the graph at all, and the kind carried no semantic weight
for the entities that are. A prediction that was answering the wrong question is worth more in the
record than one quietly deleted, and it is corrected where it stands rather than removed.

**The pin doubles.** Every run from here records the engine commit *and* the CLI commit.

---

## Predicted, before the re-run

Five, each with what would refute it.

**S1 — Unfalsifiability narrows.** Experiment 02's claim becomes: not falsifiable by *selective*
editing, falsifiable by *wholesale recomputation*, absent an anchor outside the repository. Near
certain, because it is already measured on the other side.

*Refuted if* the repository refuses a consistent forgery of the agents' own scenario.

**S2 — The lower bound becomes a bound.** The auditor recorded that its set of alternatives was a
lower bound, because nothing enumerates. A journal enumerates every admission.

*Refuted if* something the auditor needs still cannot be enumerated.

**S3 — An inference becomes a read.** Observation 2 of experiment 02 has an auditor deriving
advance-from-fork by comparing consecutive states. A persisted `Decision` names the operation.

*Refuted if* the auditor still prefers to infer, or if the two disagree.

**S4 — A silence becomes checkable.** The justification claimed an expectation *was never written
down*, sorted as unsupported partly because nothing enumerates. With a journal it is decidable.

*Refuted if* the claim, restated against the journal, is still out of reach.

**S5 — The fourth register may exist after all.** Experiment 03 pre-registered a claim that can
neither be settled nor be clearly outside reach as its most interesting possible failure, and did
not find one. `by: Option<AgentId>` may be one: the CLI measured that a recorded party is checkable
as a **reference** — it names an agent admitted and known at the decision's coordinate — and
uncheckable as an **attribution**, because nothing says the named party is who wrote it.

A claim resting on that field is therefore partly settleable and partly not, in a way neither of the
three registers describes. If an agent claims authorship and the record can confirm the name while
being silent about the person, the sorting has a case it was not built for.

*Refuted if* such a claim sorts cleanly into supported or unsupported once the check is run.

S5 is the one worth running the re-run for. The rest are bookkeeping on results already understood.

---

## What the earlier runs now mean

They are pinned records of a different boundary, and they are not rewritten.

Where a result narrows, the narrowing is a new result with its own reasoning, published beside the
old one. Where a prediction was malformed, the correction is appended and the original stays. A
refuted result never becomes a confirmed one because the architecture changed underneath it; the
original failure belongs to the record and a later experiment may test the revised architecture.

That rule was inherited from the reconstruction experiment before the first agent ran. This is the
first time it costs anything.

---

## What the re-run must not do

**It must not change the CLI to make itself succeed.** If the harness needs something the
repository does not offer, that need is handed over as a finding and the experiment waits or
records the gap. The precedent exists: the coordinate this experiment needed was found here, handed
over, refuted in its naive form there, and repaired on that experiment's own terms.

**It must not reuse a briefing.** The agents receive a different world — a repository, enumeration,
a lineage that names its operations, and a field for who decided. That is a different briefing, so
the runs are new runs with new digests, not the old ones re-scored.

**It must not read the old runs to the new agents.** Same isolation as before, and now with a
larger surface to leak through: the repository on disk is readable, and it must contain only the
scenario.
