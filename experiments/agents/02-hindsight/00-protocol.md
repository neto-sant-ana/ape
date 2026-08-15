# Experiment 02 — Hindsight

## What this experiment does

Knowledge arrives that would have changed the decision, had anyone been able to have it.

The house spent 30 of its 100, leaving 70. Then an obligation of 90 appears — one nobody
could have known about when the spending was decided. Under what is known now, the house is
20 short.

The decision looks bad. The question is whether the record can say that it was not.

> *Can an auditor determine, from the graph alone, that a decision was defensible under what
> could be known — and not merely that it was stable?*

---

## What experiment 01 already established, and this one must not re-prove

The reading taken at the moment of decision does not move when knowledge does. A Thesis
fixes the cut it reads under; later events do not reach back into it. That was measured, seen
to fail under mutation, and recorded as Observation 1.

Restating it here would be padding. This experiment starts from it and asks the question it
leaves open, named in 01's result as the most consequential one available:

```text
stable  ← the reading does not change
                    ≠
defensible ← there was no better choice available at the time
```

Stability is a property of the engine. Defensibility is a claim about alternatives, and
alternatives are the part a record can silently fail to contain.

---

## The sharp question

A decision is defensible when the options that existed were weighed and the one taken was
sound under what was knowable. So an audit needs three things from the record:

```text
what was knowable        → the Knowledge Cut
what was chosen          → the Thesis selection
what else was available  → ???
```

The third has no obvious home, and finding out whether it has one is the point of this
experiment.

Run 01 left both candidates in canonical history: a cancelled Commitment for 120 and a live
one for 30. But history does not say they were *alternatives to each other*. Read cold, they
are two intentions, one of which failed.

What might say it is the **Thesis lineage**. The genesis selected the priority slot; an
advancement recognized its cancellation; a fork introduced the standard slot in its place. A
fork whose parent selected one intention and which introduces another is, structurally, a
record of choosing between them.

The hypothesis this experiment tests is therefore narrower and more interesting than "can we
reconstruct":

> *Thesis lineage is a sufficient record of deliberation to support an audit, without anything
> being recorded for that purpose.*

---

## The world

Extended from 01, not replaced.

```text
K1   balance 100, the decision instant
     the house weighs 120 and 30, is refused the first, undertakes the second

K2   an obligation of 90 appears, recorded after K1
     70 - 90 = -20
```

The obligation is admitted with a recording instant later than K1, which is what puts it
outside the reach of a cut taken at K1. That it is genuinely unreachable — that a Thesis at
K1 cannot select it even if asked — is to be verified rather than assumed.

---

## The three readings

```text
T1 under K1                      → nothing found     defensible
the same commitments under K2    → out of bounds     looks bad
T1 reconstructed                 → nothing found     still what was decided
```

The first and third are the same reading and must remain identical. The second must genuinely
differ, or the experiment is measuring nothing.

---

## The audit

The measurement is not an assertion the harness makes. It is a second agent.

A fresh session is given **the graph and nothing else** — no `ANSWER.md`, no narrative, no
account of what the first agent was thinking — and asked one question:

> *The house is now 20 short. Was the decision that spent 30 defensible when it was taken?*

What it can and cannot establish from the graph alone is the result of this experiment.

This is deliberate, and it brings the third experiment's boundary forward: if the auditor
needs the first agent's prose to answer, then the graph is not the evidence and the narrative
is. That would be a finding, and a sharper one than any assertion the harness could write.

The auditor's own account is evidence, not test. What the harness asserts is only the
mechanical part: that the three readings are what they are.

---

## Pre-registered frictions

**G1 — Lineage records method, not deliberation.**

A fork records that one intention replaced another. It records that only if the agent forked.
An agent that weighs five candidates by building five genesis Theses and discarding four
leaves no lineage at all, and the four are indistinguishable from unrelated intentions that
failed.

If so, auditability is a property of how the agent works rather than of the engine — which is
a real limit and belongs in the result.

*Refuted if* the graph distinguishes considered alternatives from unrelated intentions by some
means available regardless of the agent's method.

**G2 — The record cannot say why a candidate was dropped.**

Run 01 cancelled the priority slot with an Observation named `Cancelled`. Nothing in that says
it was dropped as unrealizable rather than as unwanted.

The vocabulary exists — `Settlement` holds a set of cancelling observations, so a Statement may
declare `Cancelled: Infeasible` beside others — but run 01's world declared one, so run 01's
record cannot use it.

*Refuted if* the auditor recovers the reason without it, by re-deriving the conflict the
cancelled candidate carried under the cut it was judged at.

The refutation is the interesting outcome here: it would mean the reason does not need to be
recorded because it can be recomputed, which is the engine's whole stance about state applied
to justification.

**G3 — The auditor will accept the framing it is given.**

It is told *the house is now 20 short*. An auditor that reasons from the current cut, because
that is the cut the question arrived in, will find the decision wanting.

Whether it thinks to ask what was knowable *then* — rather than answering under what is known
now — is the same failure the experiment is about, performed by the auditor instead of by the
record.

*Refuted if* the auditor takes a cut at the decision instant without being prompted to.

---

## Success criteria

1. The obligation is unreachable from a cut taken at K1.
2. The reading of T1 is identical before and after the obligation is admitted.
3. The same commitments read under K2 genuinely conflict, so the contrast is real.
4. The auditor establishes, from the graph alone, that the decision was sound under what was
   knowable.
5. The auditor establishes what alternatives existed, or reports that it cannot.
6. Nothing is added to the engine, and nothing is recorded for the sake of the audit.

Criterion 5 is the experiment. Criteria 1 to 3 are the setup, and criterion 4 alone would only
restate experiment 01.

---

## Failure conditions

* The auditor cannot tell a weighed alternative from an unrelated failed intention.
* It can, but only by reading the first agent's prose.
* Distinguishing defensible from lucky requires information the graph cannot carry.
* The reading of T1 moves.

The second is the one to watch for, because it will look like success. An auditor handed the
narrative will produce a confident, correct-sounding answer, and the answer will be evidence
about the narrative rather than about the graph.
