# Observation 09 — Phase 4 ran, and the guard caught the classifier

**36 of 46, 78%.** And the check that was built to falsify this experiment's own amendment fired.

```text
this reading      19 housed        of 46
second reading    17 established   of 46
agreement         36               78%
```

The two readings were compared by [`tests/agreement.rs`](../tests/agreement.rs) against the collapse
fixed in [`08`](08-the-second-classifier.md) before the run — `Housed → YES`, `Unhoused` and
`Exposition → NO` — with the classifier's answers held as data so the number is computed rather than
tallied by the reader being checked.

## Where they part, and it is not evenly

Ten claims. **Six this reading called housed and the classifier did not; four the other way.**

### The six it would not grant, and four of them are one shape

```text
 5   the by→party mapping … and the program derives this rather than assuming it
 9   the merged journal is operations' 20 then finance's 1 … that order is not a preference
12   finance's records claim a prefix … the program prints which entry each one trips on
 7   the refusal names position 19 and adds that the two share 19 entries
```

Each pairs a record-fact the files fully establish with a clause about **what the crate does**, and
the classifier applied a rule it stated up front: *an unestablishable conjunct sinks the claim*. It
flagged 5, 9 and 12 as difficult and said outright that grading on the sentence's principal assertion
would flip all three.

**That is a disagreement about where a claim ends, not about the record**, and it is the sharpest
methodological thing Phase 4 produced. This reading classified by a sentence's operative content; the
classifier classified by its weakest conjunct. Neither is wrong and the two give different numbers —
which means the housed/unhoused count carries a granularity choice that
[`00-protocol.md`](00-protocol.md) never specified.

The other two are `24` and `25`, both `Derived(Applicability)`: this reading counts *computable from
the files* as housed, and the classifier drew the line at *the operation was asked and answered*.
Given the briefing said "you do not have to run anything — decide whether the files contain what it
would take", that is a fair reading of an instruction that was not sharp enough.

### The four it granted and this reading did not

`28`, `30`, `32` and `33`. Three are claims about what the merged record does **not** carry — the lost
witnesses, whose witnesses the merged ones are, and the absence of any field marking a re-application
— and the classifier's position is consistent and, on reflection, better than this one's:

> **An absence the files exhibit is established by the files.** `merged/lineage.json` shows `by` on
> two decisions and no field of any kind for a re-application; that is not a gap in the record, it is
> a fact the record displays.

This reading called those `Loss`, `Qualification` and `Want`. **Both readings can be right at once**,
and the difference is what a claim is *about*: the classifier read them as descriptions of a record it
was holding, and this reading as requests aimed at a boundary. Recorded, and not reconciled — a
reconciliation now would be the checked party rewriting the check.

## The one that was a defect, and the guard is what found it

**Claim 33 is an extraction error of this reading's**, and [`02`](02-the-third-verdict.md) named this
exact test as what would catch `Exposition` absorbing something. It caught it, and then the diagnosis
went the other way.

The testimony reads:

> *`transfer::applied` says the same thing about its own case ("the record says which commitments were
> introduced and never that another line of thinking is why")*

**This reading stored only the inner quotation.** Stripped of the frame, it is a bare claim about the
record — and the classifier called it established, correctly. With the frame it is the crate quoting
itself, which is what `Exposition` is for.

So the category did not absorb a claim the record holds. **A mis-cut quotation did**, and no guard in
this experiment could have found that: `every_claim_is_quoted_from_the_testimony_it_names` passes on
an inner quotation, because an inner quotation *is* in the file.

**The text is left exactly as the classifier judged it.** Amending a claim after somebody has answered
it is not a correction, it is a different sample. The check is pinned to `[33]` instead, so a new
absorption fails and this one stays named.

## What it does to the result

**Nothing to the two-set finding**, which rests on the unclassified rate across seven testimonies and
not on the housed/unhoused line. Nothing to `Recorded`-versus-`Tracked`. And nothing to the counts as
counts — 78% is agreement about a question with a granularity nobody had specified.

**It does bound P1.** *About half of what an agent says is already in the record* was already the
honest reading, and two independent readers landing at 19 and 17 of 46 — with ten of the differences
being where a sentence ends — says that number is soft by a few points in either direction and no
sharper.

## And the hazard fired three times, on the classifier, and was caught three ways

```text
03   a directory's name trusted over the testimony      caught by the corpus
04   a search for the agent's words over the lab's      caught by the operator
09   a quotation cut inside its own frame               caught by the guard
```

That is what Phase 4 was for, and it is the first of the three that a *mechanism* found. The row
opened saying its subject is prose and prose is classified by somebody with an interest in the
outcome — [`succession/README.md`](../README.md) — and the answer it offered was a second classifier.
The answer worked.
