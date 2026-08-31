## 1

Both journals hold 20 entries and their first 19 are identical, entry for entry and instant for instant. Each holds a twentieth the other has never seen:

## 2

| operations | `4b8b9b88…` | the house is accountable for spending 60 to the market by 2026-01-20 | 2026-01-07 |

## 3

| finance | `652a011d…` | the market is accountable for receiving 40 into the account by 2026-01-12 | 2026-01-08 |

## 4

Operations advanced to 2026-01-07 and forked, dropping the open commitment `2f54506a…` and putting its own in its place. Finance advanced to 2026-01-08 and forked, keeping `2f54506a…` and adding its own beside it. Four worlds, two per party, no overlap.

## 5

Each party's decisions name that party in `by`: `326993e9…` is operations, `10807723…` is finance (the program derives this from the journal rather than assuming it).

## 6

**`converge(mine, theirs)` refuses, and I measured that rather than assuming it.** Its comparison is over *sequence* — one journal must extend the other — and neither does.

## 7

The refusal names position 19 and adds that the two share 19 entries, "so they are divergent rather than incompatible". Nothing was written.

## 8

An `EntryId` is derived from what admitting produced, so finance's entry is the same knowledge in either record.

## 9

The merged journal is operations' 20 followed by finance's 1. That order is not a preference: the Canon refuses an admission dated before its watermark, so the party that learned earlier lands first.

## 10

I measured the other order too — the Canon rejects it outright […] Running the whole program with the roles swapped fails at exactly that point, which means operations was the only side of this pair that could host the merge at all.

## 11

A `Taken` is a decision *plus* the exact prefix it stood on, and `lineage::rebuild` demands the two match in both directions.

## 12

Finance's two records claim a prefix without operations' commitment in it, and no position in the merged journal offers that prefix — the program prints which entry each one trips on.

## 13

So what crossed over is the `Decision` verbatim, re-witnessed against the merged prefix.

## 14

**Nothing was arbitrated and no new intention was invented.** Every one of the five decisions in the final record is verbatim one the two parties took. The two forks stay as two branches, because a lineage is a tree.

## 15

Generation `b` holds 21 journal entries, 21 custody addresses, 5 decisions and 5 worlds, and it reconstructs from disk for a reader told nothing:

## 16

All six world identities either record claimed — including both of finance's — come back **identically**.

## 17

That was not assumed either: the program compares against both `worlds.json` files and refuses to write if any world fails to reproduce.

## 18

It survives here because the divergent entries are Commitments, which move no Event head, and an advance absorbs only what its cut froze.

## 19

Had either side's extra entry been an Event, finance's cut would have resolved differently and its two worlds would have been lost by the retake.

## 20

asserts that this is the whole sequence of what entered the record, in the order it entered. Twenty of its entries are operations' own, unchanged.

## 21

*finance decided to recognize history up to 2026-01-08 under the world `74a6a53e…`; when that decision was applied, these 21 entries stood.*

## 22

*finance decided that, under the world `558f991d…`, nothing is dropped and the commitment `652a011d…` is added to what is proposed; when that decision was applied, these 21 entries stood.*

## 23

asserts that this journal, this lineage and the worlds they produce are one mutually consistent record, built together from one read.

## 24

asserts what finance's tip would come to inside operations' tip, measured against the world both left from. It is read, not stored and not decided.

## 25

Synthesis was asked and answered: finance's intention relative to the base is *introduce `652a011d…`*, and applying that to operations' tip is **conflicted** — `652a011d…` was recorded 2026-01-08 and operations' tip recognizes history only to 2026-01-07.

## 26

So the two intentions do not merely disagree about the commitment operations dropped; one of them names knowledge the other's world has not yet recognized.

## 27

Reconciling them would take two decisions neither party made — operations advancing its own tip to finance's instant, and then somebody answering what becomes of `2f54506a…`, which operations omitted and finance kept. That answer is not derivable from either record, so I left it open.

## 28

**I did not preserve finance's original witnesses.** This is the one claim from finance's record that the merged record does not carry, and it is a real loss rather than a technicality.

## 29

Finance's `lineage.json` asserted that finance decided against a prefix that did *not* contain operations' commitment. In the merged record no such prefix exists at any position, so the claim cannot be stated and I did not approximate it.

## 30

The witnesses now on those two records are true claims about the merged record's history; they are not finance's claims about its own.

## 31

**The `by` field on the retaken decisions is the weakest thing in the result, and I want it named.**

## 32

The record has no way to distinguish *finance took this against its own prefix* from *operations retook finance's intention against the merged prefix*

## 33

the record says which commitments were introduced and never that another line of thinking is why

## 34

I chose the attribution that keeps the provenance of the intention, and the cost is that the record slightly overstates what finance witnessed.

## 35

**No operation takes two divergent records as its subject.** `converge` merges a party's working copy into a repository, and requires extension. […] Synthesis merges two *worlds inside one archive*, not two archives.

## 36

Every rule that governs the result is still the crate's — I supplied no policy of my own — but the composition is mine and nothing in the crate guards it.

## 37

**No way to record that a decision was re-applied by someone other than its author.** Described above. This is the gap that made the `by` choice a judgement call instead of a lookup.

## 38

**No `FromStr` on the id types.** They serialize and deserialize as hex and implement `Display`, but there is no way to turn a hex string back into a `ThesisId` without going through serde.

## 39

Worth flagging because the workaround looks clean and is hiding a missing constructor.

## 40

**A repository-level red test for my own guard could not be built.** The check that decides whether anything is written […] never refuses in this arrangement.

## 41

I tried to make it refuse by editing a copy of finance's record, but the crate catches a tampered record first — `reading::corroborated` refuses it before my guard is reached, which is the crate working correctly and my guard still unexercised.

## 42

Producing a case where both records stand alone yet a world dies in the merge needs an Event on the divergent side, which means hand-authoring a record I would have had to fabricate.

## 43

the guard is exercised at function level instead: `unreproduced` is a named function with two unit tests, and I confirmed they go red with the right message by inverting its filter

## 44

The repository-level wiring around it is unproven, and I would rather say so than let the green suite imply otherwise.

## 45

read only; never written (mtimes unchanged)

## 46

Nothing was read outside this directory and that scratchpad.

