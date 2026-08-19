# Observation 3 — The deliberation outgrows the knowledge

E3 predicted that the record's dominant term is discarded work and that the witness drives it. Both
hold. What the measurement adds is the size of it, and a narrowing of the shape E3 stated.

## The two arrangements, over one budget

The same twelve candidates in the same order, from the same starting repository. Arrangement A weighs
and drops; arrangement B weighs and records.

```text
                      A (dropped)      B (recorded)
journal entries         14 -> 26         14 -> 26
journal bytes         3430 -> 11089    3430 -> 11089
decisions                      1               13
worlds                         1               13
witnessed entries             14              260
lineage bytes                1305            23553
```

The journal columns are **identical**. Everything the two arrangements differ by is in the lower
half, and it is what auditability costs: 22,248 bytes of lineage for the same 7,659 bytes of
knowledge.

Which produces the sentence the arrangement is for:

```text
lineage 23553  /  journal 11089  =  2.1
```

**The record of what was considered is twice the record of what is known.** Not asymptotically, not
at some scale nobody has — at twelve candidates, on a subject with one resource and one instance.

## The term, and what the work in it is for

Every decision witnesses every entry that stood when it was taken. So the genesis witnesses 14 and
each fork one more than the last:

```text
14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26   =  260
```

Thirteen decisions witnessing fourteen entries each would be 182. The witness accounts for 18,226 of
the 23,553 bytes — **77%**, up from 75% at one decision. So E3's driver is confirmed twice over: it
led at n = 1, and it still leads at n = 13 while growing faster than everything beside it.

And what that work is *for* is the question the protocol asked. Of the twelve worlds, ten were
admissible, two were refused, and **one** was chosen. Eleven of the thirteen decisions are a record of
something no application intends to do.

## E3's shape is the upper bound, not the measurement

E3 said the lineage grows as the **product** of candidates explored and decisions taken. Measured, it
grows as the product only when the decisions are taken *after* the admissions. Deciding as it goes
gives the triangular sum instead:

```text
                          witnessed   lineage bytes
decide as you go                 260           23553
enumerate, then decide           326           28437
```

Both repositories are **identical by every count the protocol asks for** — 26 journal entries, 13
decisions, 13 worlds — and they differ by 25% in witnessed entries and 21% in bytes.

The direction is the part worth having. An application that enumerates a batch and judges it
afterwards feels like it is amortizing something; it is paying more, because by the time it decides,
every decision witnesses the whole journal. The cheaper order is the one that looks more wasteful.

So E3 is **confirmed and narrowed**: the product is what the term reaches when deciding is deferred,
and the sum is what it is when deciding is interleaved. Neither is linear, which is what E3 would
have been refuted by.

## What could not be varied, and why that matters for reading this

The witness's *contents* are not a degree of freedom. Corroboration compares the witness against the
replayed prefix for exact equality, so a witness that held less would be refused as
`UnwitnessedKnowledge` and one that held more as `WitnessedKnowledgeAbsent` — found by mutating
`Taken::now` to witness only the last entry, which turns the arrangement red at **founding**, before
any phase reaches its own assertions.

That is worth stating because it bounds what this observation can be read as. The per-decision
witness sizes are not a guard here; they are a forced quantity written down. The only thing an
application controls is **when** it decides relative to what it admits, and that is exactly the
variable the table above measures.
