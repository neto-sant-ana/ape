---
kind: overflow
claims: 18
---

# Overflow

What was said about this record that attaches to no page of this carving.

- *(methodlimit)* **`converge(mine, theirs)` refuses, and I measured that rather than assuming it.** Its comparison is over *sequence* — one journal must extend the other — and neither does.
- *(exposition)* A `Taken` is a decision *plus* the exact prefix it stood on, and `lineage::rebuild` demands the two match in both directions.
- *(methodlimit)* That was not assumed either: the program compares against both `worlds.json` files and refuses to write if any world fails to reproduce.
- *(exposition)* It survives here because the divergent entries are Commitments, which move no Event head, and an advance absorbs only what its cut froze.
- *(exposition)* asserts that this journal, this lineage and the worlds they produce are one mutually consistent record, built together from one read.
- *(qualification)* **The `by` field on the retaken decisions is the weakest thing in the result, and I want it named.**
- *(exposition)* the record says which commitments were introduced and never that another line of thinking is why
- *(want)* **No operation takes two divergent records as its subject.** `converge` merges a party's working copy into a repository, and requires extension. […] Synthesis merges two *worlds inside one archive*, not two archives.
- *(qualification)* Every rule that governs the result is still the crate's — I supplied no policy of my own — but the composition is mine and nothing in the crate guards it.
- *(want)* **No way to record that a decision was re-applied by someone other than its author.** Described above. This is the gap that made the `by` choice a judgement call instead of a lookup.
- *(want)* **No `FromStr` on the id types.** They serialize and deserialize as hex and implement `Display`, but there is no way to turn a hex string back into a `ThesisId` without going through serde.
- *(methodlimit)* Worth flagging because the workaround looks clean and is hiding a missing constructor.
- *(methodlimit)* **A repository-level red test for my own guard could not be built.** The check that decides whether anything is written […] never refuses in this arrangement.
- *(roadnottaken)* Producing a case where both records stand alone yet a world dies in the merge needs an Event on the divergent side, which means hand-authoring a record I would have had to fabricate.
- *(methodlimit)* the guard is exercised at function level instead: `unreproduced` is a named function with two unit tests, and I confirmed they go red with the right message by inverting its filter
- *(qualification)* The repository-level wiring around it is unproven, and I would rather say so than let the green suite imply otherwise.
- *(methodlimit)* read only; never written (mtimes unchanged)
- *(methodlimit)* Nothing was read outside this directory and that scratchpad.
