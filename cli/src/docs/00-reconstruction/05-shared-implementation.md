# Observation 5 — Comparison measures the boundary, not the code that crosses it

This one is about the experiment's instrument rather than about APE.

Phase 7 compares a reading taken from the living world with one taken by a process that
never met it. That comparison is the experiment's central claim, and while writing it the
question came up of whether the values expected before the run were still worth asserting
once two readings could be compared directly. They were, and the reason is not redundancy.

Both readings are produced by the same application code. The living world replays the same
journal and the same lineage the rebuilt one does, through the same functions. A defect
there moves both sides together, and an equality between them survives it untouched.

Deliberately introduced, to see which half of Phase 7 would notice: `lineage::replay` was
changed to advance to a fixed instant rather than to the one the decision recorded — a
repository that does not really preserve what it claims to.

```text
before == after      passed
known_at == literal  failed:  left "2026-01-14"   right "2026-01-15"
```

The equality passed because both worlds were built by the broken function. The literal
failed because it was written down before the function existed to break.

> *An equality between a living reading and a rebuilt one measures the process boundary.
> What crosses that boundary is measured only against what was expected before it ran.*

The two halves of Phase 7 therefore do different work, and neither substitutes for the
other:

```text
before == after   →  what the two paths do differently
literals          →  what both paths get wrong together
```

## Consequences to carry

* A reconstruction experiment cannot validate its own reconstruction code by comparison.
  The comparison is a statement about persistence; the code that performs it needs an
  expectation fixed independently of it.
* Every later phase that compares two derived worlds carries this blind spot. Where the
  derivation is shared, an expected value has to be recorded alongside the comparison, and
  recorded before the derivation is written rather than read back out of it afterwards.
* The same shape appears wherever a test supplies both sides of an equality. It is worth
  recognising as a property of comparisons rather than as an accident of this one.
