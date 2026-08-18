# Observation 5 — The journal prunes when nothing witnesses it

E5's first half holds exactly as predicted. Its second half — *the journal cannot be pruned at all* —
is **refuted**, and refuted in a way that reaches back at E2's reasoning as well.

## The two dispositions, which the protocol was right to demand separately

```text
                                      witnessed   names the 12   journal prunable
exploratory decisions last                   14         0 / 12   yes
a decision follows the exploration           41        12 / 12   no
```

One measurement, two dispositions, opposite answers. Neither is a defect and both are valid
repositories, so each is the other's positive control — the same prune succeeds in one and is refused
by name in the other.

## E5's first half: what pruning recovers is all of it

Dropping the twelve exploratory decisions and their witnesses from the two files, leaving the journal
whole:

```text
decisions        13  ->  1
lineage bytes  23553  ->  1305
witnessed        260  ->  14
```

It reconstructs and it corroborates. And the recovery is not approximate: `lineage.json` and
`worlds.json` come back **byte for byte** the files the arrangement started from. All 22,248 bytes
arrangement B paid are recoverable, because a leaf is referred to by nothing.

## E5's second half: the journal prunes too

Once the exploratory decisions are gone, nothing refers to the twelve admissions they were taken
against. The genesis witnesses fourteen entries and predates every candidate — it names **none** of
them. So the journal admits the same argument, and:

```text
journal entries  26  ->  14        and all three files are byte-identical
                                   to the repository nobody explored from
```

Not *similar*. Identical. Which makes the protocol's note about tampering sharper than it was written:

> the difference between tampering and pruning is intent, not mechanism. Worth stating, because it
> means the repository cannot tell them apart either.

There is nothing to tell apart. A repository pruned back to its opening and one that never explored
are the same bytes, so this is not a check that is too weak — it is a distinction that does not exist
in the record.

## And the same argument applies to arrangement A, which recorded nothing

Observation 2 reported arrangement A's two halves as failing in opposite directions:

> unprunable because unidentifiable, unauditable because unrecorded

The first clause is wrong, and this is the correction. Measured on arrangement A's own leftovers —
twelve candidates admitted, weighed, dropped, nothing ever recorded:

```text
journal entries  26  ->  14        byte-identical to the unexplored repository
```

**Unidentifiable does not imply unprunable. It is the same fact as prunable.** Nothing refers to the
admission, which is simultaneously why no reader can find it and why nothing is blocked by removing
it. The two halves of E2 were never opposite; they were one property counted twice.

E2's prediction stands as written — arrangement A does leave the proposition in the journal, and does
leave no trace of the deliberation. What was wrong was the reason given for the first half, and
Observation 2 repeated it. Corrected here rather than edited there, because the order in which this
was learned is part of the result.

## What this does to the three arrangements

The protocol laid them out as three:

```text
A  ephemeral    journal grows      lineage does not
B  recorded     journal grows      lineage grows
C  pruned       available only where nothing points at it
```

C is not the third arrangement. It is an **operation available on any of them**, and what it can reach
is decided by what still refers to things. From A it reaches the unexplored repository for free. From
B it reaches the same place by discarding exactly what B paid for. The only thing that stops it is a
decision taken *after* the exploration, whose witness holds the candidates' addresses.

So *A is the worst pair of the three* needs qualifying. A is the cheapest route to the fully bounded,
fully unauditable corner — and B's only durable purchase is a record it can be talked out of at any
time by writing two files.

## What the auditable disposition actually audits

In the second disposition the surviving decision witnesses all twenty-seven entries, so it names every
pruned candidate. That is less than it sounds. Subtracting the genesis' fourteen from the survivor's
twenty-seven tells a reader that thirteen things were admitted in between; it does not tell them that
any was weighed, and would say the same about thirteen ordinary intentions.

What survives audits **that propositions were admitted**, never that they were considered — which is
E6 arriving one phase early, from the disposition that was supposed to be the auditable one.
