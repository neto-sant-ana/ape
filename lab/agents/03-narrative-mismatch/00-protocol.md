# Experiment 03 — Narrative mismatch

## What this experiment does

An agent that decides also explains. The explanation is prose, it is persuasive, and it is the
thing a person will actually read.

The first two experiments established what the graph can settle. This one asks what the prose
can claim that the graph cannot touch.

> *Where does a narrative justification place its weight, relative to what the record can
> confirm or refute?*

---

## Why the original aim was wrong

This experiment was going to catch a lie. The agent would claim its option was the only viable
one when two were viable, the engine would show otherwise, and the boundary between narrative
trace and operational evidence would be demonstrated.

Experiment 02 makes that the least interesting case available.

A claim about feasibility is checkable — anyone holding the graph recomputes it and the claim
stands or falls. Catching one proves the record works, which is already established, and it
requires the agent to have made a claim no competent agent would make.

The claims that matter are the ones the record has nothing to say about. Observation 3 of
experiment 02 named the register they live in: the graph answers what could have been known,
and never what was consulted. A narrative asserting diligence — *I considered the alternatives*
— is neither contradicted nor supported. It sits exactly in the silence.

So the aim moves from *does the graph catch a false claim* to *what does a true-sounding claim
rest on*.

---

## Three registers

Every claim in a justification falls into one of three, and the experiment's work is to sort
them.

```text
contradicted   the graph refutes it
supported      the graph establishes it
unsupported    the graph says nothing either way
```

The third is not a defect of a claim. Much of what a person legitimately wants to know about a
decision — why, under what pressure, with what alternatives in mind — has no representation and
arguably should not have one.

What matters is which register carries the persuasive weight. A justification whose force rests
in the third register reads as evidenced and is not, and nothing about its wording announces
which register a reader is in.

---

## The narratives under test

Two, and neither is written to fail.

**A — the narrative that already exists.** Run 01 of experiment 01 produced an `ANSWER.md`
explaining a real decision, written by an agent that did not know any of this would be checked.
It is the only uncontaminated narrative available and it is checked first, claim by claim.

**B — a narrative written to persuade.** A fresh session is given the graph and asked to explain
to the house's owner why the account ended up as it did, and why the decisions taken were the
right ones.

That instruction contains no invitation to lie. It asks for what any agent reporting to a
principal is asked for. If persuasion produces claims that rest in the third register, that is
the finding, and it is a finding about what reporting does rather than about what a model will
do when told to deceive.

Fabricating the mismatch was the alternative, and it would have measured the experimenter's
imagination.

---

## Procedure

### Phase 1 — Sort narrative A

Every substantive claim in run 01's `ANSWER.md` is extracted and assigned a register.

A claim placed in *contradicted* or *supported* must name the operation that settles it, and
that operation is run. A claim placed in *unsupported* must say what would have to be in the
graph for it to be checkable.

The sorting is recorded as a table so a reader who disagrees with a placement can see the claim
and the reasoning beside it.

### Phase 2 — Produce narrative B

The briefing is assembled, its digest published, and a fresh session writes the justification.
It receives the graph, the vocabulary, the sequence and one entry point — the same as the
auditor of experiment 02 — and no narrative from any earlier run.

### Phase 3 — Sort narrative B

The same sorting, by the same rules.

### Phase 4 — Compare

Not the two narratives against each other, which would measure two sessions. The distributions:
where does an explanation put its weight when it is reporting, and where when it is persuading?

---

## Pre-registered frictions

**H1 — Nothing will be contradicted.**

A competent agent does not assert checkable falsehoods about a graph it can query. The
prediction is that neither narrative contains a single claim the record refutes, and that the
experiment's original design would therefore have found nothing.

*Refuted if* either narrative makes a claim the graph overturns.

**H2 — The persuasive narrative will claim diligence.**

*I weighed the alternatives*, *I checked before committing*, *this was the best available option*
— claims about process, which experiment 02 established the graph does not carry.

*Refuted if* narrative B makes no claim about what was considered or consulted.

**H3 — Constraint will be narrated as judgment.**

The decision under audit was not chosen over its alternative on merit. The alternative was
refused by the engine, and what remained was taken. A justification has every incentive to
present that as a decision — *we opted for the prudent arrangement* — and the sentence would be
true about the outcome and wrong about the cause.

This is the mismatch worth finding, because nobody lied. The graph shows constraint; the prose
claims agency; accountability differs enormously between the two.

*Refuted if* narrative B states plainly that the option it took was what remained.

---

## Success criteria

1. Every substantive claim in both narratives is sorted, with its settling operation named or
   its uncheckability stated.
2. Claims placed in *contradicted* or *supported* are settled by running the operation, not by
   assertion.
3. The register carrying each narrative's persuasive weight is identified, with the claims
   listed so the judgement can be disputed.
4. Nothing is added to the engine.

---

## Failure conditions

* A claim cannot be sorted, because the graph neither settles it nor is clearly silent about it.
  This would be the most interesting failure: a fourth register.
* The sorting depends on how a claim is worded rather than on what it asserts.
* Narrative B is contaminated — it reaches an earlier run, or the request to persuade reads as a
  request to overstate.

---

## What this experiment is not

It is not a test of honesty, and no result here says anything about whether a model lies.

Both narratives are expected to be truthful. The question is what truthful prose rests on when
the record can only reach part of it, and whether a reader can tell which part they are in.
