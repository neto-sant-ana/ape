# Observation 3 — The reference runs one way

The protocol asked whether the journal loses knowledge the same way the lineage loses intention,
because the two answer to different authorities and may deserve different repairs. Measured, the
answer splits:

```text
mechanism      identical — both files are written whole by whoever writes last
being found    different — and the difference is not about the files
```

## What makes a loss detectable

A missing record is found out when something else **refers** to it. The repository has exactly two
such references, and both point the same direction.

**A decision names the world it extends.** Convergence established that, and it is why a lineage is
a tree rather than a line. So a decision that is gone is named by whatever descended from it:

```text
[ shared, staffing, deepening ]     staffing removed, with its witness
→ ExtendsUnknownWorld { thesis: staffing }
```

**A decision names the point in the journal it was taken at.** So knowledge that is gone is named by
whatever was decided against it:

```text
journal reverted to what the other party held, one decision left standing on the grant
→ UnknownEntry(c685214990…)
```

Both refusals arrive with the identity in them. Neither says the repository is invalid.

## And what makes one undetectable

Remove the **leaf** instead — the last decision, which nothing extends — together with its witness:

```text
[ shared, staffing ]
→ Ok, two worlds
```

Which is the whole of it:

> Nothing refers to a decision that nothing extends. A second party's line is always a leaf.

The arrangement in Phase 2 is not an unlucky case of the loss. It is the exact case the record
cannot see, and it is the *normal* one — two parties reasoning in parallel each hold a tip, and a
tip is by definition what nothing has descended from yet.

The asymmetry follows from that and not from anything about knowledge versus intention: **the
lineage refers to the journal, and nothing refers to the lineage.** Knowledge lost alone is caught
by the decision left standing on it. Intention lost alone is caught by nothing, because there is no
outer file whose records point inward.

## A note the mutations produced

Blinding each reference in turn — making `replay_through` admit everything rather than refuse an
unknown address, then making the archive answer a lookup with any world it happened to hold — showed
that the two detectable cases are each covered **twice**, by references that were built for
different reasons and in different experiments.

```text
knowledge gone    address refuses first; blinded, the witness refuses    WitnessedKnowledgeAbsent
world gone        archive refuses first; blinded, the world record does  WorldDisagrees
```

Convergence concluded that the sequence witness is not redundant with the address it duplicates.
This is a second confirmation of the same thing, from the other side: with the address blinded, the
witness is the only thing left that knows.

None of that redundancy reaches the leaf. Two independent guards on each of two references, and
zero on the thing this experiment is about.
