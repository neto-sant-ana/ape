# Candidate — Scale

**This is not a protocol.** A protocol is written when its experiment begins, with the previous
findings in hand, and is not edited afterwards to match what happened. This is the material that would
be in front of whoever writes it.

It exists because the decision that produced it was taken outside the record. When the engine's
magnitudes stopped being binary floats, the decision was that **the engine adds and compares, and what
the count counts is the application's — to arrive via an experiment**. The engine half was built and
says so. The other half was named in conversation and written nowhere, which is how a decision becomes
a thing nobody remembers having made.

---

## The foundational question comes first, and it settles half of this

> *A concept enters the ontology only if operational coordination cannot be represented without it.*

That is [`01-ontology.md`](../../core/src/docs/01-ontology.md), and applied here it answers before the
candidate begins. A resource is the axis its own movements move along and **there is no second axis to
reconcile**, so nothing the engine does — adding, comparing against a bound — is unable to proceed for
want of a unit. No conversion exists to get wrong. Coordination is representable without it.

So a unit does not enter the ontology, and the two placements that would have put it there are closed
rather than open:

```text
a unit on the Resource            a new field on an existing input — the ontology's own
a unit on the ResourceInstance    definition of an extension, and it is not necessary
```

What remains is an application's, which is where the engine already said it goes. This candidate is
therefore **not about the engine**, and its first job is to stop reading like it is.

---

## The question that is left

> *An application must say what it is counting somewhere. Is that somewhere in the record?*

A magnitude is `u128` and a bound is `i128`. Nothing in the engine multiplies or divides one, so an
integer count is exact and associative, and the engine's own note says what is deliberately missing:

> What the count counts is **not here**. Cents, whole items, thirds of an hour, pallets of forty
> eight: the engine adds and compares, and which unit an application means is that application's,
> because a resource is the axis its own movements move along and there is no second one to reconcile.
> Naming the unit here — as a decimal scale, or as anything else — would make every application share
> one approach to units, and the whole point of a minimal ontology is that they need not.

And it was answered once already by removing an answer: a `Scale` type was drafted and deleted for
reproducing, one layer up, the industrial premise that had chosen the float in the first place.

So the question is what an application must do instead, and whether *composition* is a real answer or
a place where the work was left.

---

## What the application currently does, and it is one application's choice

A count is written as a **decimal string** in the journal, attached to the fields rather than to a
newtype. Two reasons, and both are about the file rather than about the domain: serde cannot read a
128-bit integer out of an internally tagged enum, and a JSON *number* is a double to most things that
read JSON — so the exactness the engine gained would have stopped at the file.

That is a format decision and it is settled. It says nothing about the unit. Every value in both
laboratory rows is a whole unit of something unnamed, in every one of nine experiments, which means
**no arrangement has ever needed the question answered** — and that is the honest reason it is still
open rather than an oversight.

---

## What each result contributes

**00 — the hazard, found first and worked around by discipline.** The reconstruction subject's own
docstring said, from the first experiment, that feasibility accumulates in `f64` where addition is not
associative, and that integers keep the experiment measuring reconstruction rather than float
determinism. Every subject in both rows did the same thing independently, for seven experiments,
without ever raising it to the engine.

**00's own candidate, superseded.** *Fractional magnitudes, which would answer whether reconstruction
is order-stable where `f64` accumulation is not associative.* That question is closed by the
representation — the accumulation is exact now — and what is left of it is exactly this candidate: not
whether fractions are stable, but where the fraction's denominator is written down.

**And the measurement that made the change safe** is also what leaves this unmeasured: the two
hypotheses a caller may ask fold in different orders, one by `CommitmentId` and one by due date, so
under `f64` two answers over identical knowledge could disagree by arithmetic alone. Fixed. The unit
question has no equivalent demonstration, and an experiment would have to build one.

---

## Two things the application may do, and only one of them is in the record

The ontology's note allows both: an application may **compose** the primitives or **wrap** them in
types of its own. A `Money` holding a magnitude and a unit is the obvious wrapper, and it is legitimate
— but it is the application's vocabulary and does not enter the record. So it answers the question for
the process that holds it and for nobody else, which is the whole reason this candidate cannot stop
there: the frontier row exists because a later process reads what the first one wrote.

What *is* in the record is the other one. A `Resource` is admitted with a **label**. *cash* is a label;
so is *cash, in cents*. An application that wants the unit in the record can put it there today,
without a primitive and without the engine learning anything — which is what *express complexity
through composition* means when it is not a slogan.

And that placement has consequences the record already enforces, which is the argument for it: a label
is knowledge, so it is admitted, immutable, and part of the resource's identity. Changing the unit
produces a **different resource**, and everything that referred to the old one still refers to the old
one. That is the engine's answer to every question of this kind, arrived at by doing nothing.

So what is genuinely open is smaller than it looked, and it is three questions:

```text
must an application say it?     nothing forces a unit into the label, and nothing notices
                                its absence. Nine arrangements said nothing and nine
                                arrangements were right, because every value was a whole
                                unit of something unnamed

what does a reader do with
two records that disagree?      two repositories meeting — the candidate open since
                                convergence — with one saying cents and one saying units.
                                This is the first concrete thing that question has to
                                reconcile, and it is not a number, it is a name

is a label enough to be
checked against?                a unit in a label is a string, and nothing derives a
                                contradiction from it. The record can hold the claim and
                                cannot weigh it, which is the shape 01-veracity is about.
                                A wrapper does not help here — it makes the unit checkable
                                inside one process, and the mismatch this fears is between
                                a writer and a reader that never met
```

The third is why this may not be an experiment of its own. If a unit in the record is a claim nothing
can check, then *scale* is a naming convention with a candidate file, and the honest result is to write
the convention down in the application and say so. What would make it an experiment is the second
question — and that one arrives on its own the first time two repositories meet.
