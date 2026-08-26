# Observation 8 — Part B: one comparison, scoped by the addressing rather than by judgement

The protocol's own condition on a repair here runs the opposite way from every experiment before it,
because a positive result meets the inherited criterion by definition — a state that passes every
check and answers falsely *is* a state a reader can be misled by. So the condition is about scope:

> A repair may close only the state that was found. A remedy for the class the state belongs to is a
> remedy for cases nobody measured.

## What was built

`converge`'s `appended` gains a second comparison over the shared prefix, and a refusal of its own:

```text
the journal disagrees about entry 14, 7a948a39…: this party recorded it at
2026-01-11 and 2026-01-03 is there
```

Separate from `Diverged` because the address is the **same**, and a refusal naming two identical
identities sends a reader looking for a difference that is not there. That was measured: mutating the
old comparison to see the instant produced `Diverged { expected: EntryId("7a948a39…"), found:
EntryId("7a948a39…") }`, which is a message that cannot say what disagrees.

## Why the scope is exact, and it is not a judgement call

Every field of an admission except the recording instant is in the identity admitting produces. So
two journals that agree by `EntryId` can differ in the recording instant **and in nothing else** —
any other difference has already shown up as a different address, one comparison earlier.

Comparing the recording instant therefore closes exactly the state that was found. Not the class
*two records disagree*, which `Diverged` already covers; not the class *a record was edited*, which is
authenticity's; and not the class *a recording instant is unchecked*, which would want a digest and
would refuse the differently-arranged record of Observation 1 as well.

## Why a refusal and not a choice

The merge has two other options and both are the disease.

Keeping the converging party's instant is what it does today, and it moves the other party's world.
Keeping the **earlier** one is not better for being principled: it moves the converging party's world
instead, silently, in the same way. And keeping both is not available — the record has no
representation for having been told twice, which is Request 3 and a design question rather than a
repair.

Refusing is the only answer that invents nothing, and it is the answer `converge` already gives for
the case it can see. The recovery is the one every refusal here offers and the coordination experiment
established: read again, decide again, converge.

## Against the criterion

**It removes a state a reader can be misled by.** Both orderings, measured: the application refuses at
entry 14, by name, and a refused merge leaves the repository exactly as it was — the phase asserts that
what is on disk afterwards is still the first party's whole record.

**And what the repair replaces survives.** Two shapes measured, and they are what every earlier
experiment's merges are made of: a party that appends knowledge the other does not have, and a party
that admits nothing and only decides against the base it read. Both still converge, and both lines are
in the lineage afterwards. Nothing that was correct before stops working, because what stops going
through is the one case that produced a wrong answer.

## What the repair does not close

**Request 2.** A merge still writes its own `worlds.json` and still discards the worlds the arriving
record recorded. This repair means the instant cannot differ; it does not mean a merge could not change
a standing decision's world for some other reason. Nothing here found one, and Observation 6 says
where it stopped looking.

**The two unmeasured merge states of Observation 6** — a Commitment's instant rather than an Event's,
and a disagreement no cut straddles. The repair happens to refuse both, because it compares the field
rather than the consequence. That is wider than what was measured, and it is the one place this repair
exceeds its own scope: it is reported here rather than defended, and the reason it is acceptable is
that the comparison has no narrower form. There is no way to compare *an instant that matters* without
first deciding which ones do, which is a derivation, and a derivation is what got the record here.

## How the finding stays measured after the repair

The measurement in Observation 5 is a state the application no longer produces, and a suite that
measured it only through `converge` would have lost it. So the merge is reimplemented in the
laboratory — the same three steps minus the comparison this adds — and the phase that measures the
falsity runs against that.

Its agreement with the application was asserted rather than assumed, and asserted **before** anything
was repaired: for both orderings, `converge` reached the state the instrument reaches. The numbers in
Observation 5 are the application's, measured. The phase that made that assertion is now the phase
that measures the refusal.
