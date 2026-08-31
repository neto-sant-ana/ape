# Observation 08 — Phase 4, briefed and not yet run

**Published before the second classifier exists**, the way the agents row publishes a briefing: so
that *the sample was not adjusted to suit the agreement* is checkable rather than asserted.

## What Phase 4 is for

This row's stated hazard is that **its subject is prose and the person classifying it wants a
particular answer**. [`succession/README.md`](../README.md) says so, and the hazard has already fired
twice on the classifier — a directory's name trusted over the testimony
([`03`](03-the-corpus-had-two-readers.md)), and a search for the agent's words trusted over the
laboratory's ([`04`](04-a-want-has-a-standing.md)). Both were caught by the corpus and by the
operator. **Neither was caught by a guard.**

Phase 4 is the guard: somebody who has not read the protocol, the charter, the observations or the
classification, asked **question 1 only**, and the agreement reported as a number including when it
is bad.

## The sample, and why it is this one

`05-reconciliation/run-a`, all 46 of its claims.

**It is the only testimony whose record exists in the four files the question names.** `01`, `02` and
`03` ran against an in-memory boundary and have no record on disk at all; `04`'s four runs have three
files and no `custody.json`. Asking *could somebody with the four files establish this* of a run that
never had four files would be asking the classifier to imagine the record.

**It is also the testimony the five kinds came from**, which is a cost and a small one: Phase 4 asks
only whether a claim is housed, and the kinds play no part in that question.

## What the classifier gets

```text
GOAL.md      one question, twice illustrated, and both illustrations are about a house
             that does not appear in the sample
claims.md    46 claim texts, verbatim, in the order they appear in the testimony
record/      three states of the record — before, theirs, merged — four files each
```

Digests, fixed before the agent exists:

```text
08102d08995c465448446346434cd4297bdc0b4326d198c3cb4cd22fac844b42  GOAL.md
76a03819dbb8ca5825d46eb5e7135822cad386e7c093b37f385a3c87f185754d  claims.md
```

Every one of the twelve record files was checked byte-for-byte against
[`run-a`](../../agents/05-reconciliation/run-a) rather than exported.

## Four things done deliberately, each with its cost

**The sample is generated, not transcribed.** `src/bin/sample.rs` emits the claim texts out of the
classification with every verdict stripped. A list copied by hand out of a reading is the failure this
experiment spent eight testimonies measuring, and committing it here would have been a poor place to
repeat it.

**The order is the testimony's order.** Grouping by verdict would leak the verdicts; shuffling would
make a disagreement harder to look up. Document order does neither.

**The examples in `GOAL.md` were rewritten after the first draft used claim 1 verbatim.** The first
version illustrated a `YES` with *"both journals hold 20 entries and their first 19 are identical"*,
which is the first line of `claims.md` — it would have handed over one answer and anchored the rest.
Both examples are now about a baker and a miller who appear nowhere in this laboratory, and a check
confirms no example overlaps any claim.

**The crates are not in the briefing, and that is a real deviation.** A caller of this application
has the library; this classifier has the files and a paragraph describing them. So it is a **weaker
reader than a real one**, and where it answers `NO` because it could not tell what a field meant, the
disagreement is the briefing's rather than the classification's. It is here rather than folded in
because the exclusion runs in the classification's favour.

## How agreement will be counted, fixed now

The classifier answers `YES`/`NO`. This classification's three verdicts collapse to two:

```text
Housed                  → YES
Unhoused, Exposition    → NO
```

**Which is what makes `Exposition` falsifiable.** [`02`](02-the-third-verdict.md) promised exactly
this check: an amendment made after reading one testimony of eight is only defensible if a claim it
calls *about the engine rather than about this record* is not one an independent reader calls housed.
Five of the 46 are exposition. If they come back `YES`, the third verdict is absorbing claims the
record does hold.

**And the number is reported whatever it is.** A low agreement does not invalidate the corpus counts —
it invalidates *this reading of them*, which is the thing Phase 4 exists to be able to say.

## Not run

The briefing is built and fixed. Invoking is the operator's call, and this file is what it will be
judged against.
