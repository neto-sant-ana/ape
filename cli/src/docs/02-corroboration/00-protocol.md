# Corroboration

## Abstract

The reconstruction experiment established that operational meaning survives process death.
The divergence experiment established that a lineage of alternative worlds survives it too,
once a decision records where in the sequence of admissions it was taken.

Both were established against a repository assumed to be telling the truth.

The divergence experiment then measured what happens when it is not. A repository edited in
one place — an address repointed at an entry that exists and is the wrong one — rebuilds a
different lineage and says nothing. Every check either candidate repair had passed.

> *Can a repository detect that it is wrong?*

---

## Question

Both previous experiments obeyed one inherited rule: nothing derived is persisted. A stored
identity, a stored head, a stored verdict — each is an answer kept beside the question it
comes from, free to disagree with it.

That freedom is the whole of the objection, and it is also the whole of the value.

```text
a copy that can disagree, never compared   →  a liability
a copy that can disagree, always compared  →  a witness
```

A record that holds only inputs cannot contradict itself, which is another way of saying it
cannot detect anything. Detection needs two representations of one fact and something that
compares them.

So the question has two halves, and they are the same half:

- a **stored world** corroborates the world a decision re-derives;
- a **value over the admitted prefix** corroborates the coordinate a decision records.

Both are derived. Both are forbidden by the rule as it stands. This experiment is where that
rule is revised or upheld, and either outcome is a result.

---

## Hypothesis

A derived value persisted **and compared on every read** converts a class of silent wrong
answers into refusals, and costs nothing that the derivation did not already cost.

Three claims follow, each separately refutable:

```text
a stored Thesis        →  makes a re-derivation that differs detectable
a prefix witness       →  makes a false coordinate detectable
neither supersedes     →  the decisions stay, because a lineage records deliberation
                          and a set of stored worlds records only outcomes
```

The third is the one this experiment is least sure of. If the stored worlds turn out to make
the decisions redundant, that is a finding, and a larger one than the first two.

---

## Motivation

An application that cannot tell a faithful reconstruction from a corrupted one is an
application whose record has to be believed rather than checked. For a coordination engine
that is the wrong posture: the reason knowledge is content-addressed and history is a chain is
so that agreement can be established rather than trusted.

The gap is therefore not decorative. Everything APE says about a world is derived, and the
divergence experiment showed the derivation can be steered by one edited field, invisibly.

There is a second reason, and it is about the layer above. Synthesis moves intent between
lineages, and it decides whether a Base is a common ancestor by walking stored ancestry. A
walk over records nobody can check is a conclusion nobody can check. Whatever this experiment
settles, Synthesis inherits.

---

## Experimental Boundary

This experiment exercises the smallest repository capable of catching itself.

It includes:

* the `ThesisArchive` port, and the first adapter this laboratory writes for it;
* a derived value over the sequence of admissions, in whatever form the procedure requires;
* comparison of each persisted derived value against its derivation, as part of reading rather
  than as part of testing;
* deliberate corruption of a persisted repository, which no phase of any previous protocol
  performs;
* a measurement of what the decisions still record once the worlds are stored;
* a measurement of what corroboration does **not** catch.

It deliberately excludes:

* **authenticity.** Corroboration detects a record that disagrees with itself. A record edited
  consistently — every derived value recomputed to match — is not a disagreement, and catching
  it needs a signature and a key that this laboratory does not have. Phase 7 measures that
  boundary rather than assuming it.
* Synthesis, and any transfer of intent between lineages;
* concurrent writers;
* snapshots, indexes, and any measurement of what replay costs;
* fractional magnitudes;
* mutable named references;
* production-oriented user experience.

Those concerns may become later experiments.

They must not influence the structure introduced here unless this experiment itself requires
them.

---

## Experimental Subject

**The divergence subject, unchanged.**

It already produces what this experiment needs: three worlds, one of them refused by its own
bounds, and a genesis whose cut is fragile enough that a single edited field steers the whole
lineage. Reusing it means the corruption measured here is a corruption of a repository this
laboratory has already declared correct.

It is not modified. A concluded experiment's subject is the reproducer of a published result,
and the arrangement that makes this one able to fail is the same arrangement.

A subject of this experiment's own is introduced **only** if a corruption the procedure
requires cannot be expressed against that one — and if it is, the reason is recorded as a
finding about what the divergence subject could not reach.

Three paths the divergence experiment named as unreachable are the likely candidates: an
admission recorded after the last decision, a decision that follows an entry already admitted,
and an advancement that imposes.

---

## Initial State

The experiment begins from a repository the divergence experiment would accept.

```text
repository = the lineage of three worlds, intact
archive    = empty
witness    = absent
```

Nothing is assumed about it beyond what that experiment established. Phase 1 re-establishes it
rather than inheriting it, because a baseline taken on faith is the thing this experiment is
about.

---

## Procedure

### Phase 1 — Construct

Build the divergence lineage and persist it, exactly as that experiment leaves it.

Reconstruct it in a fresh process, and record the three worlds.

This is the baseline, and it is measured rather than assumed. Every later phase is a claim
about a difference from it.

---

### Phase 2 — Corrupt

Edit the persisted repository — not the code — in ways that are well-formed and false, and
record what each produces.

At minimum:

```text
a coordinate repointed at an entry that exists and is the wrong one
a journal entry reordered
a journal entry removed
a decision's intention altered
```

Every one of these is expected to produce a wrong world in silence, or to fail for a reason
that names the harness rather than the record. That expectation is the starting measurement,
not a step to be hurried through: what the later phases claim is that specific entries in this
table change.

Record the table. It is the instrument for the rest of the experiment.

---

### Phase 3 — Witness the sequence

Introduce a derived value over the admissions a decision was taken after, persisted beside the
coordinate and compared whenever the repository is read.

Re-run every corruption of Phase 2.

Record which became refusals, which did not, and what each refusal names. A refusal that says
only "the repository is invalid" has answered half the question — the reader needs to know
which datum disagrees with which.

---

### Phase 4 — Archive the worlds

Store each decided world through `ThesisArchive`, and write an adapter that passes the port's
conformance suite.

Reading a world now has two sources: the stored record, and the re-derivation from decisions.
Compare them on every read.

Re-run every corruption of Phase 2, and record the table again.

The conformance suite is not optional and not a formality. It is the first time this
laboratory implements that port, and the suite already pins the two coordinates the divergence
experiment watched move.

---

### Phase 5 — Subtract

With the worlds stored, remove parts of the decision record and find what stops being
reconstructible.

```text
remove the coordinate      →  ?
remove the instant         →  ?
remove the intention       →  ?
remove the decisions       →  ?
```

Answer by demonstration rather than by argument. A field that can be removed with nothing
lost is a field the repository should not hold, and the third claim of the hypothesis is
decided here.

Whatever survives, state the closed set — and for each member, what becomes impossible without
it.

---

### Phase 6 — Terminate, reconstruct and compare

Terminate the process. Rebuild in an operating-system process of its own, given the repository
and nothing else.

Every world of the lineage comes back, as the divergence experiment requires, and the
comparison between the stored world and the re-derived one happens **inside** reconstruction
rather than inside a test. A derived value checked only by the harness is a derived value that
is not checked.

---

### Phase 7 — Forge

Corrupt the repository consistently: edit it, and recompute every derived value so that
nothing disagrees with anything.

Confirm that it reconstructs without complaint.

This is the boundary of the result, and it is measured so that the result cannot be
overclaimed. A record that checks itself proves internal agreement and nothing about who wrote
it.

---

## Success Criteria

The hypothesis is confirmed for the experimental boundary when all of the following hold:

1. Phase 1's baseline is reproduced from a fresh process, unchanged from the divergence
   experiment's result.
2. Each corruption of Phase 2 either becomes a refusal or is named, in the result, as one that
   does not.
3. Every refusal names which two representations disagree, and about what.
4. **Every persisted derived value is compared on every read.** A derived value written and
   not checked is the liability the old rule feared, and its presence is a failure of this
   experiment rather than a shortcut within it.
5. The `ThesisArchive` adapter passes the port's conformance suite.
6. Reconstruction still derives every world. A world that can only be read back is a world
   whose derivation is no longer tested.
7. What a decision still records is stated as a closed set, and anything removed in Phase 5 is
   removed by demonstration.
8. The consistent forgery of Phase 7 is undetected, and the result says so plainly.
9. No persistence-specific concern is introduced into the APE engine. Using `ThesisArchive` is
   using the boundary; changing it is not.
10. The reconstruction and divergence experiments' conclusions stand, or the change is recorded
    as a result of this experiment rather than as an adjustment to theirs.

---

## Failure Conditions

The hypothesis is refuted, or narrowed, if:

* a corruption that produced a wrong world in Phase 2 still produces one after Phases 3 and 4;
* a derived value is persisted and consulted only by the harness;
* the stored world becomes the only source, so that reconstruction stops being a derivation and
  becomes a read;
* detecting a corruption requires the engine to understand repository layout or ordering;
* the decisions turn out to be recoverable from the stored worlds, which would mean this
  repository never recorded deliberation in the first place.

The last is not a defeat. It is the strongest finding this experiment could produce, and it
would reach back into what the divergence experiment concluded a lineage is for.

A failed experiment is a valid result. The implementation must not hide failure by weakening a
corruption until the comparison agrees.

---

## Variables Deliberately Left Open

### The form of the witness

Whether it is a hash of one entry, a chain over entries, or a value over the whole admitted
prefix is not decided here. What is required is that it disagree with a sequence that is not
the one the decision was taken against.

### What the archive is keyed by

`ThesisArchive` resolves by identity. Whether an application also needs to reach a world by
anything else is a question about naming, and naming is still not this laboratory's subject.

### Whether decisions survive Phase 5

Stated as a hypothesis and answered by subtraction, not assumed in either direction.

---

## Methodological Constraint

This experiment follows one implementation rule:

> *Do not introduce an abstraction intended to solve an experiment that has not yet been
> performed.*

Structure may be introduced only when required by the current experimental procedure.

The previous experiments' conclusions are not revised here. Where this one finds something
that would have changed them, it is recorded as a finding of this experiment, against the
implementation as it then stood.

---

## Expected Pressure Points

### A witness over an encoding makes the encoding load-bearing

The repository is JSON because a laboratory whose repository cannot be read by eye hides half
of what it is for. A value computed over those bytes ties the record's validity to a format
chosen for legibility. The engine's own `Selection` documentation states the general form of
this: deriving identity from an encoding is what makes the encoding impossible to change
afterwards.

### Two derived values with different lifetimes

A Thesis is content-addressed and final. A witness over a prefix grows as the journal grows.
Whether they are one mechanism or two is not obvious from here.

### The archive and the journal answer to different authorities

Knowledge is admitted; worlds are decided. The two have been in separate files for that
reason, and a third store enters the same question rather than settling it.

### Corroboration that is never exercised

A comparison that always agrees is indistinguishable from a comparison that is not performed.
Phase 2's table exists so that each check is shown failing at least once, on a corruption
chosen to reach it.

### Reading becomes more expensive

Every read now derives *and* compares. The protocol excludes measurement, so this experiment
records that the cost changed and says nothing about how much.

---

## Observations

Each observation is recorded as its own numbered document beside this one, as the experiment
produces it.

Record facts rather than decisions retroactively presented as inevitable.

Useful observations include:

* corruptions that resisted detection, and why;
* derived values that turned out to corroborate nothing;
* comparisons that could not be placed inside reconstruction;
* places where storing a world made a derivation stop being exercised;
* assumptions that held only because a subject never arranged for them to be tested.

Where possible, record the smallest reproducing case.

---

## Open Questions

The experiment intentionally leaves several questions unanswered.

Among them:

* What signs a record, and who holds the key?
* Does a witness survive a repository written to by more than one process?
* If a stored world and a derivation disagree, which one is wrong?
* What does a repository do with a world it can no longer derive?
* Does an archive change what a lineage costs to walk?

These are candidates for later experiments.

They are not requirements for this one.

---

## Experimental Principle

Reconstruction asked whether meaning survives.

Divergence asked whether deliberation survives.

```text
This one asks whether the record can be trusted
without being believed.
```

The experiment will determine what a repository must hold twice for that to be true.
