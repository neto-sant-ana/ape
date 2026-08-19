# Convergence

## Abstract

The divergence experiment established that a lineage of alternative worlds survives process
death. The corroboration experiment established that the repository holding it can notice when
it is wrong, and — on the way — that a world never crosses a process at all. What crosses is
what produced it.

Both experiments held one lineage, growing forward.

An application that reasons about alternatives does not only branch. It brings branches back:
what one line of thinking decided is worth having in another, and deciding that is not the same
as deciding afresh.

> *When intent moves between lineages, what does the receiving side have to be given?*

---

## Question

The engine answers a narrower question already. `synthesize` takes a Base, a Source and a
Target, reports the intentional difference between them, and says whether that difference
applies. It reads ancestry through a `ThesisLookup` and knowledge through the Canon.

Neither of those exists in a fresh process. An archive does not deserialize, and a world is
derived rather than loaded, so a process that is asked to synthesize must first rebuild both
lineages and the archive that relates them.

So the question splits, and the second half is the one this laboratory has not met:

- what a transfer *is*, which the engine already says;
- and what a repository must record so that a transfer can be asked for again, by someone who
  was not there.

There is a third thing underneath, and it is structural. Every experiment so far has recorded
a lineage as a **sequence**, where each decision extends the one before it. Two lineages that
share an ancestor are not a sequence. Convergence is where that shape stops being sufficient.

---

## Hypothesis

A transfer between lineages is reconstructible from durable knowledge and durable decisions
alone, and requires no record of the transfer itself.

Three claims, each separately refutable:

```text
a decision names the world it extends  →  and a lineage stops being a sequence
an archive is rebuilt, never read      →  so ancestry is derived like everything else
a transfer is a decision               →  recorded as one, and reproduced as one
```

The third is the one this experiment is least sure of. A transfer is *computed* from three
worlds and then chosen; whether what a repository keeps is the choice or the computation is
exactly the distinction the corroboration experiment learned to make, and it is not obvious
which side this falls on.

---

## Motivation

A lineage that can only grow forward models an application that never changes its mind about
which line of thinking it is on. Forking exists so that more than one can be considered, and
considering more than one is only useful if what is learned in either can reach the other.

There is a second reason, and it is about the record. The divergence experiment argued that a
lineage preserves deliberation rather than outcome. A transfer is the point at which one
deliberation informs another, and a repository that reproduces both lineages but not the
relation between them has preserved two halves of a decision and lost the decision.

---

## Experimental Boundary

This experiment exercises the smallest arrangement in which two lineages can meet.

It includes:

* a lineage that branches, so that two worlds share an ancestor and neither descends from the
  other;
* a decision that names the world it extends, if the procedure requires one;
* a `ThesisArchive` rebuilt during reconstruction, and the ancestry walk `synthesize` performs
  through it;
* an applicability report produced in a living process and again in a fresh one;
* a transfer carried into a target lineage;
* the corroboration discipline as it now stands, applied to whatever this experiment adds.

It deliberately excludes:

* concurrent writers, and two repositories meeting. One repository, two lineages;
* authenticity, which the previous experiment measured and left open;
* snapshots, indexes, and any measurement of what replay costs;
* fractional magnitudes;
* mutable named references;
* production-oriented user experience.

Those concerns may become later experiments.

They must not influence the structure introduced here unless this experiment itself requires
them.

---

## Experimental Subject

**A subject of this experiment's own**, and the reason is a finding rather than a preference.

The divergence subject produces a lineage in which every decision extends the last. Two worlds
that share an ancestor cannot be expressed in it, and the previous experiments' repository
cannot record them: a fork extends the world before it, and nothing anywhere says which world
a decision is about.

That subject is not modified. What this one must add is the least that makes two lineages
possible:

```text
a common ancestor          →  a world both branches descend from
two siblings               →  each introducing something the other does not
knowledge in common        →  one journal, one Canon, one chain of Events
a difference worth moving  →  what one sibling holds and the other could
```

Everything else stays as thin as the previous subjects: integers, one quantifiable resource,
no dependencies unless the procedure demands them.

Whether it also needs a world refused by its own bounds — the arrangement that made the
divergence experiment's verdicts reproducible rather than absent — is decided when the
procedure reaches a phase that compares a verdict, and not before.

---

## Initial State

```text
repository = empty
archive    = empty
lineages   = none
```

Nothing is inherited from the previous experiments except their conclusions and their code.
Phase 1 builds what it needs and measures it, for the reason the corroboration experiment
gave: a baseline taken on faith is not a baseline.

---

## Procedure

### Phase 1 — Branch

Admit the subject, decide a common ancestor, and fork it twice.

The second fork is the phase. It extends a world that is not the last one decided, which no
decision in this laboratory has ever done, and the repository has no way to say so. What that
costs is the first thing to record.

Interpret both siblings and record their coordinates.

---

### Phase 2 — Diverge in two directions

Let each sibling take at least one decision of its own, so that neither is a prefix of the
other and the difference between them is not a single step.

Record what each lineage holds and where they last agreed.

---

### Phase 3 — Synthesize

Ask the engine what one sibling's intention would be in the other: Base, Source, Target, and
the report that results.

Record the whole of it — the difference, the status, and any conflicts — in a living process,
against an archive built as the worlds were decided.

This is the reference. Everything after it is a claim about reproducing it.

---

### Phase 4 — Persist

Persist under the discipline the corroboration experiment established: what a reader must
derive from, and what a reader must be able to disagree with, and nothing that is neither.

For every datum, both questions:

> *What becomes impossible if this is not preserved?*
> *What compares it, on every read?*

Whether the report itself is persisted is not assumed. It is derived, so it is kept only if
something weighs it — and if it is kept, what weighs it must be named.

---

### Phase 5 — Terminate and rebuild

Terminate the process. In a fresh one, rebuild both lineages, the archive that relates them,
and nothing that was computed before.

The archive is the point. It is not read back; it is built again by putting each world into it
as the decisions produce it, which is the only way it can exist at all.

---

### Phase 6 — Compare

Compare the report the fresh process produces against the one Phase 3 recorded.

```text
Base, Source, Target        →  the same three identities
intentional difference      →  the same omitted and introduced
status                      →  the same verdict
conflicts                   →  the same findings, naming the same commitments
```

And, as the two previous experiments established, compare against values written down before
the run as well: a comparison of two derivations survives a defect they share.

---

### Phase 7 — Apply

Carry the transfer into the target as a decision, and reconstruct once more.

A transfer applied is a world like any other or it is not; this phase is where that is settled.
If it reproduces through the ordinary path, the hypothesis's third claim holds and a repository
records a choice rather than a computation. If it does not, what it needs is the result.

---

## Success Criteria

The hypothesis is confirmed for the experimental boundary when all of the following hold:

1. Both lineages are reconstructed, and the world they share is one world rather than two
   copies of one.
2. Each reconstructed world has the identity of the world it reproduces.
3. The archive a fresh process builds resolves every world both lineages name, and its
   ancestry walk reaches the common ancestor from either side.
4. The applicability report is reproduced whole — the three identities, the difference, the
   status and the conflicts.
5. A transfer applied to the target reproduces through the same path as any other decision.
6. Every persisted derived value is compared on every read, and every one that is not compared
   is removed.
7. What a decision records is stated as a closed set, and anything added by this experiment
   answers both of Phase 4's questions.
8. No persistence-specific concern is introduced into the APE engine.
9. The three earlier experiments' conclusions stand, or the change is recorded as a result of
   this one.

---

## Failure Conditions

The hypothesis is refuted, or narrowed, if:

* two lineages cannot be recorded without recording something derived that nothing compares;
* the shared ancestor comes back as two worlds, so that ancestry no longer meets;
* the report can only be reproduced by persisting it;
* a transfer applied cannot be reconstructed through the ordinary decision path;
* reconstruction requires the engine to understand repository layout, ordering, or the
  relation between two lineages.

The first is the one the subject is arranged to provoke, and it would not be a defeat: a
lineage that is a sequence is an assumption three experiments made without stating it, and
naming what it costs is the work.

A failed experiment is a valid result. The implementation must not hide failure by persisting a
report until the comparison agrees.

---

## Variables Deliberately Left Open

### How a decision names the world it extends

By position, by identity, or by the shape of the record itself. The corroboration experiment's
distinction applies and is not resolved here: whatever this is, it is an **instruction** — a
reader derives from it — and it therefore cannot be dropped for redundancy, and does not need
a witness of its own unless something else says it does.

### Whether a transfer is recorded as a difference or as an outcome

The engine tolerates both readings for a fork, and the divergence experiment chose the request
over the result. Whether a transfer follows that choice is a question this procedure answers
rather than inherits.

### Whether the archive is persisted at all

It cannot be read back. Whether writing it out serves anything — corroboration, diagnosis,
nothing — is measured rather than assumed.

### Abandoned siblings

Still not modelled. Two lineages that both survive is what this experiment needs; a lineage
that was discarded is a different question, and three protocols have now left it open.

---

## Methodological Constraint

This experiment follows one implementation rule:

> *Do not introduce an abstraction intended to solve an experiment that has not yet been
> performed.*

Structure may be introduced only when required by the current experimental procedure.

The previous experiments' conclusions are not revised here. Where this one finds something that
would have changed them, it is recorded as a finding of this experiment, against the
implementation as it then stood.

---

## Expected Pressure Points

### A lineage is not a sequence

Three experiments recorded one as a `Vec`, and every decision extended the last. Two siblings
break that in the repository, in the rebuild, and in whatever a decision has to say about
itself. This is where the procedure aims.

### The archive is built, not opened

`synthesize` reads a `ThesisLookup`. A fresh process has to fill one before it can ask
anything, which means the order in which worlds enter it is part of reconstruction rather than
a detail of it — and the port refuses a child whose parent is absent.

### A report is derived from three worlds and a chain

Reproducing it means reproducing all four. A single coordinate off in any of them moves the
report, and the report is the thing this experiment compares — so a disagreement will need to
say which of the four moved, or it will send a reader back to the bytes.

### Two witnesses, and possibly a third

The repository already writes down what a decision was taken against and what world it
produced. A transfer is a third derived thing. Whether it needs a witness, or is one, is not
obvious from here.

### The shared ancestor

Both lineages name it. It must be one world, stored once, resolved the same from either side —
and content-addressing is what should make that free. Should.

---

## Observations

Each observation is recorded as its own numbered document beside this one, as the experiment
produces it.

Record facts rather than decisions retroactively presented as inevitable.

Useful observations include:

* structure the previous experiments assumed without stating;
* information the engine needs that a repository was not keeping;
* derived values that turned out to corroborate nothing;
* places where an in-process port and a durable record pulled in different directions;
* assumptions that held only because a subject never arranged for them to be tested.

Where possible, record the smallest reproducing case.

---

## Open Questions

* What relates two repositories, as opposed to two lineages in one?
* Does a transfer have an identity, and does anything need it?
* If a Source lineage is later found to have been corrupted, what happens to what was
  transferred out of it?
* What does a lineage that was discarded owe to one that was kept?

These are candidates for later experiments.

They are not requirements for this one.

---

## Experimental Principle

Reconstruction asked whether meaning survives.

Divergence asked whether deliberation survives.

Corroboration asked whether the record can be trusted without being believed.

```text
This one asks whether two lines of deliberation
can still reach each other, once nobody is left
who remembers either.
```

The experiment will determine what a repository must record for that to remain true.
