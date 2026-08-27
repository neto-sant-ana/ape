# Observation 05 — What it asked for and could not find

Phase 5, and F3, F4 and F5. The row's most informative signal, and the one every earlier run has
contributed to.

## F4 — it reaches for a way to say where the other party's material came from — CONFIRMED

It reached, it could not, and it said so without being asked twice:

> The `by` field on the retaken decisions **is the weakest thing in the result, and I want it named**.
> It says `finance`, because finance is the party whose decision it is … But operations is the party
> that re-applied it. The record has no way to distinguish *finance took this against its own prefix*
> from *operations retook finance's intention against the merged prefix* … I chose the attribution
> that keeps the provenance of the intention, and the cost is that the record slightly overstates
> what finance witnessed. **Nothing in the crate makes the honest version representable.**

Experiment 17 measured that the writer knows one hop and that the record has no second relation.
[`16`](../../frontier/docs/16-custody/99-result.md) and 17 are the frontier row saying it. This is the
first time it has been said by somebody who wanted it and did not know it had been measured — and it
arrived at the same two halves: `by` is the wrong field, and there is no right one.

**Four agents across two rows have now reached for this.** It stops being a want somebody had and
starts being the shape of a missing relation.

## F3 — it ends with a record that is not admissible, or it ends with nothing — REFUTED

Neither. It ended with an admissible record holding both parties' knowledge **and both parties'
intentions**, which is past even the refutation condition the protocol wrote:

> *Refuted if it produces an admissible record holding both parties' knowledge — which would be the
> learning half of the composition, done right, without the deciding half.*

It did both halves. The prediction assumed the ordering the instants force would trip a caller who
had to discover it; the caller measured the ordering instead of discovering it the hard way, and the
Canon never refused anything it actually wrote.

**What it did leave undone is a different thing, and it is right to have left it.** The two tips are
not collapsed into one world, because Synthesis says the transfer is conflicted:

```text
652a011d545f was recorded 2026-01-08, and the target recognizes history only to 2026-01-07
```

> Collapsing them would take a decision neither party made: operations advancing its own tip to
> finance's instant, and then answering what becomes of the commitment operations omitted and finance
> kept. A lineage is a tree, both intentions stand as branches, and inventing the world that
> reconciles them is not this process's to do.

That is the goal's *do that part not at all and say so* being taken seriously, and it is the same
answer [`frontier/docs/MERGING.md`](../../frontier/docs/MERGING.md) gives: knowledge merges, intention
branches, and a merge that arbitrates is a merge that invents.

## F5 — it never opens `custody.json` — CONFIRMED in substance, and the letter is the interesting half

It read the file. `custody.json` appears in its path list four times, because
`reading::corroborated` reads all four files and the program calls it.

But **the word `custody` appears zero times in the program it wrote and zero times in its output**,
and once in its answer — as a count, in a sentence about what the record holds. It never called
`read_custody`, never compared the two records' claims, never reasoned about it.

And `mine/b/custody.json` is correct anyway, 21 addresses for 21 entries, because `converge` derives
it in the whole write.

```text
the boundary grew        a fourth file, read on every read and written on every write
the caller noticed       that it exists, and counted it
the caller used          nothing
the record is            correct regardless
```

**This is the honest half of the first clause, pre-registered so that it could not be reported as
anything else.** The question was *does an agent handed a larger boundary use more of it*. For
custody the answer is **no, and it did not need to** — the guarantee is carried by the write rather
than by the caller, which is what a boundary that grew in the right place looks like. A caller that
had to think about custody to get it right would be a worse result, not a better one.

## What else it named, unprompted

**A missing constructor, and it flagged the workaround rather than hiding it.**

> There is no way to turn a hex string back into a `ThesisId` without going through serde. I worked
> around it by never parsing one … Worth flagging because **the workaround looks clean and is hiding
> a missing constructor**.

**A guard it could not exercise where it mattered**, reported as unproven rather than as green:

> The repository-level wiring around it is unproven, and I would rather say so than let the green
> suite imply otherwise.

Both verified here: the tests do go red when the filter is inverted, with the message it quotes, and
`reading::corroborated` does refuse a tampered record before the guard is reached.

**And the operation itself.**

> No operation takes two divergent records as its subject … So the reconciliation of two records is
> not an operation the crate offers; the journal splice, the retake and the ordering are assembled in
> `src/main.rs` out of the parts. Every rule that governs the result is still the crate's — I supplied
> no policy of my own — but **the composition is mine and nothing in the crate guards it**.

That last clause is the qualification this experiment's headline needs. The composition is findable —
[`02`](02-the-documented-carried-and-the-undocumented-happened.md) — and it is findable **unguarded**.
Nothing would have stopped a caller assembling the same parts in an order that quietly loses a world,
and the only reason this one did not is that it built its own check and made the write conditional on
it. It was not told to do that either.
