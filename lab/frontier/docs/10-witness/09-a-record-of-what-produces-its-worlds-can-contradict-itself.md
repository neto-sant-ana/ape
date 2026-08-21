# Observation 9 — A record holding only what produces its worlds *can* contradict itself

A sentence in the application's own source, falsified by measurement, and the correction is worth more
than the sentence was.

`cli/src/lineage.rs` says of the witness:

> A repository holding only what produces its worlds cannot contradict itself, which is another way of
> saying it cannot notice anything.

Read as an argument for the witness, it says: the reason to record what stood is that a record of only
the inputs to its worlds would have nothing to check against anything. That is false, and the
instrument of this experiment is the counter-example.

## The dependence claim is falsifiable, in both directions

A dependence set is a claim, and it is weighed the way `corroborate` weighs the witness — twice, in
opposite directions:

```text
the world reaches an entry the record did not claim      Unrecorded(entry)
the record claims an entry the world does not reach      NotDepended(entry)
```

Both are reachable and both were produced. Handed the **broad** witness as if it were a dependence
claim, the comparison refuses `NotDepended`, naming an entry the world does not depend on — which is
also the cheapest demonstration that the two claims are not interchangeable. Handed a record short of
its own world by one Event, it refuses `Unrecorded`, naming the entry.

The second is the more instructive of the two. Under-claiming is normally caught earlier — the entry is
not admitted at all and the world cannot be produced — and it reaches the comparison only because
another decision's claim carried the entry. So the check is not decoration: a dependence set is weighed
against the world it produced, not against the journal it was written beside.

## What the sentence should say

Two things, and the second is Observation 7's:

**A record of only what produces its worlds contradicts itself wherever a claim disagrees with a
derivation.** The closure is a function of the world, and the world is produced from the coordinate and
the journal, so nothing has to be believed: the record states, and the read derives, and the two are
compared. That is the same structure the witness has — `after` written a second time so a reader can
compare — applied to a narrower claim.

**What it cannot notice is a change no world is a function of.** That is the real content of the
original sentence, and it is a much stronger thing to say than *cannot notice anything*, because it
names the class exactly and it is the class the witness exists for.

## Why the wrong version was persuasive

Because it is nearly right, and because it argues for the correct conclusion. The witness *is*
load-bearing; three experiments said so and this one agrees. The sentence reached that conclusion
through a claim about what a record can check, when the true reason is a claim about what a world can
see. A reader who accepted it would have believed the witness was the record's only comparison, and
would have been surprised by `reading::corroborated` — which weighs the worlds file against what the
decisions produce, and which is the guard Observation 7 had to put every mutation to.

The docstring is corrected in this experiment's second half. It is the only change to the application,
and it is not Part B: nothing was repaired, and one sentence stopped being false.
