# Observation 4 — The same decision costs more for having waited

W2, and it is a comparison rather than an argument because it is **one decision**.

`Side::restated` is the second decision written a second time at the end of the same history: the same
`Decision` value, so the same world by identity, over a longer prefix. The only thing that differs
between the two records is what had been admitted by the time each was written, and none of it is what
the decision is about.

```text
                          witnessed     depended on     world
─────────────────────────────────────────────────────────────
taken after its plan            24              17      the same ThesisId
taken at the end of history     29              17      the same ThesisId
```

Five entries of growth — the four filings and the plan the decision is not about — for a record of the
same fact. The gap is not a constant overhead a format could amortize: it is everything admitted in
between, so it is a function of the journal and not of the decision.

## One lineage over, in two currencies

```text
                addresses     lineage.json
────────────────────────────────────────────
witnessed              76        6,489 bytes
depended on            48        4,417 bytes   68%
```

Both measured over the same three decisions, with only the `witness` field substituted — so the
comparison is of the claim and not of an encoding somebody redesigned in between. No timing was taken,
for the eleventh time.

Put beside the exploration experiment's own numbers, in the same units it used:

```text
06 exploration     18,226 of 23,553 lineage bytes are witness, and thirteen decisions
                   taken later cost 21% more for having enumerated before judging

10 witness         the same three decisions cost 32% less when each claims what its
                   world is a function of; and one decision, unchanged, witnesses five
                   more entries for having been written later
```

The two agree about the shape and about nothing else. Exploration measured the cost of *enumerating*
before judging; this measures the cost of *witnessing* whatever was there. They are the same
observation from two sides of one field.

## What it does not say

That the bytes are the reason to change anything. Observation 7 is where that gets decided, and the
number is the weakest thing in this experiment — it is a property of one tail. The load-bearing half of
W2 is the column that **does not move**: seventeen at both positions, because a world is a function of
its cut and its selection and a later position changes neither.
