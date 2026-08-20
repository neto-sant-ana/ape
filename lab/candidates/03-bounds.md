# Candidate — Bounds

**This is not a protocol.** A protocol is written when its experiment begins, with the previous
findings in hand, and is not edited afterwards to match what happened. This is the material that would
be in front of whoever writes it.

It is the narrowest candidate here and the only one where the **engine is ahead of the application**
rather than behind it. It exists because the agents row handed it over twice, in the same words both
times, and nothing was holding it.

---

## The question

> *A resource with a floor and no ceiling cannot be stated in the record. What is a workaround worth,
> when the workaround writes down a bound that does not exist?*

The engine has seven constraint kinds. `greater_than_or_equal` is one of them, and six of the seven
cannot fail to construct.

```text
engine          Equal  NotEqual  GreaterThan  GreaterThanOrEqual
                LessThan  LessThanOrEqual  Between

record format   Discrete  Between
```

So an account that may never go below zero and has no upper limit is admitted as a range whose ceiling
nothing reaches. It works. Every arrangement in both rows does it, and the agents row said so twice:
*a floor-only resource constraint is still not expressible, still restated as a range whose ceiling
nothing reaches, and still the earlier experiment's handover.*

---

## Why it is not merely an omission

Two reasons, and the second is the one worth an experiment.

**It puts a number in the journal that is not a fact.** `CEILING = 1000` in an arrangement whose
account has no ceiling is an admitted, immutable, content-addressed assertion about the world, and it
is false. Nothing refuses it, nothing derives a contradiction from it, and it changes the resource's
identity — so two applications that pick different fictional ceilings for the same real resource have
two resources. That is the [`01-veracity`](01-veracity.md) shape arriving from a direction neither of
those protocols was looking: not a record made false by an accident, but a record made false by a
**workaround the format required**.

**And it is a live invariant, not a decoration.** A feasibility verdict is `OutOfBounds` against
whichever bound the record holds. A fictional ceiling is a real refusal waiting to happen: whichever
number was picked, an operation that legitimately crosses it is refused for a reason that does not
exist in the world being modelled. No arrangement has been large enough to reach one, which is why this
has cost nothing so far.

---

## What each result contributes

**Both agents-row handovers**, which are the only places it is named. The first stated it and the
fourth restated it verbatim, which is the signal: two experiments apart, unchanged, and neither could
act on it because an experiment may not extend the application to make itself succeed.

**The `i128` change touched it and left it alone**, deliberately. `ConstraintKind` moved from `f64` to
`i128` and `NonFinite` disappeared; `ConstraintError` went from two variants to one, and the one that
survived is `InvertedRange` — which exists **only for `Between`**. So the record format exposes the one
kind that can be constructed wrongly and hides the six that cannot.

---

## What is not decided here

Whether the record format should carry all seven kinds, a subset chosen for a reason, or a different
shape entirely — a floor and an optional ceiling, say, which is not one of the engine's seven and would
be the application deciding how to *say* a constraint rather than which constraints exist.

And the honest question of whether this needs an experiment at all. It may be a decision rather than a
measurement: the engine's answer is already there, the gap is in one enum in one file, and what an
experiment would add is a subject that **reaches the fictional bound** and shows what the refusal looks
like from a reader's side. That subject does not exist and would be small.

Which is the argument for and against in one sentence. Against: it is an afternoon's work behind a
question nobody has been hurt by. For: the same was true of the float, for nine experiments, until
somebody measured the predicate flipping.
