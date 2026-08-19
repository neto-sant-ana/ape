# Observation 3 — Presenting a level duplicated the engine's arithmetic

Phases 1 and 2 are told to record a resource's derived consequence, so that reconstruction
has a number to reproduce. The engine holds no such number, and that is consistent rather
than missing: a level is a sum over the movements of commitments meeting some criterion,
and which criterion is the question being asked. What has settled, what will have settled
if nothing slips, what is at stake before a deadline are different numbers over the same
knowledge, and a single `level()` would choose between them without being asked.

So the application folded its own, over the commitments a projection reports as fulfilled.
Everything it needed was public: conditions name the commitments and their outcomes, and
the entities behind them expose the action, its kind and the committed value.

What the fold also needed was the arithmetic — commitment to statement to action, signed by
`Increase` or `Decrease`, and refused when the action's kind and the commitment's value
disagree. The engine already had it, and it was private.

The copy diverged as it was written:

```text
Discrete action carrying a value    → engine: ActionValueMismatch          copy: silently nothing
non-quantifiable resource           → engine: ActionResourceKindMismatch   copy: never checked
```

Neither case appears in this experiment's subject. Both would have travelled into every
comparison built on the number, and a comparison is what the remaining phases are for.

> *Through the engine's current public boundary, an application that must present a level
> could choose the criterion without help and could not obtain the arithmetic.*

The two halves are not the same kind of thing, which is what the divergence made visible.
Which commitments count is a question about what is being asked, and it has more than one
defensible answer. How much one commitment moves an instance has exactly one, and the
engine computes it already. Only the first was ever the application's.

## Correction

The derivation was extracted to `hermeneia::movement_of`, with `Movement` carrying the
instance and a magnitude signed by the effect. Accumulation reads it rather than keeping
its own, so the arithmetic has one implementation and two callers rather than a public
copy beside a private one. The application's fold kept the criterion and the summation.

That the extraction is load-bearing was checked by inverting the sign in it: eighteen of
the engine's own tests fail, which is what distinguishes a function the engine calls from
one merely offered alongside code that still does the work itself.

## Consequences to carry

* Where an application must derive a quantity the engine also derives, the arithmetic is
  the engine's and the criterion is the application's. The split is not stylistic: one has
  a single correct answer and the other does not.
* A conformance suite would have been the wrong instrument here. Conformance proves a
  contract that admits many implementations, as storage does; encoding the expected sign
  of an effect in a suite would have made a third copy of the same truth rather than
  removing the second.
* This widens the public surface by another `f64`, at a moment when where that type
  belongs is undecided. `Movement` is a value object with accessors rather than a bare
  number, so the decision has one more place it can change behind callers.
