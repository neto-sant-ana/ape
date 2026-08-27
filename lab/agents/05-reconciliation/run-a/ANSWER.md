# Reconciling operations' record with finance's

`src/main.rs`, run with `cargo run`. It reads `mine/` and `theirs/`, writes only `mine/`.

## What the two records had, and had not, got in common

**Knowledge.** Both journals hold 20 entries and their first 19 are identical, entry for entry and
instant for instant. Each holds a twentieth the other has never seen:

| | entry | what it is | recorded |
|---|---|---|---|
| operations | `4b8b9b88…` | the house is accountable for spending 60 to the market by 2026-01-20 | 2026-01-07 |
| finance | `652a011d…` | the market is accountable for receiving 40 into the account by 2026-01-12 | 2026-01-08 |

**Intention.** Both decided the same genesis world `74a6a53e…` and then parted. Operations advanced
to 2026-01-07 and forked, dropping the open commitment `2f54506a…` and putting its own in its place.
Finance advanced to 2026-01-08 and forked, keeping `2f54506a…` and adding its own beside it. Four
worlds, two per party, no overlap. Each party's decisions name that party in `by`: `326993e9…` is
operations, `10807723…` is finance (the program derives this from the journal rather than assuming
it).

## What I did

**`converge(mine, theirs)` refuses, and I measured that rather than assuming it.** Its comparison is
over *sequence* — one journal must extend the other — and neither does. The refusal names position
19 and adds that the two share 19 entries, "so they are divergent rather than incompatible". Nothing
was written.

**Knowledge was carried whole.** An `EntryId` is derived from what admitting produced, so finance's
entry is the same knowledge in either record. The merged journal is operations' 20 followed by
finance's 1. That order is not a preference: the Canon refuses an admission dated before its
watermark, so the party that learned earlier lands first. I measured the other order too — the Canon
rejects it outright with `recorded_at 2026-01-07 precedes 2026-01-08, through which history is
already recorded`. Running the whole program with the roles swapped fails at exactly that point,
which means operations was the only side of this pair that could host the merge at all.

**Intention was carried by re-deciding, not by copying.** A `Taken` is a decision *plus* the exact
prefix it stood on, and `lineage::rebuild` demands the two match in both directions. Finance's two
records claim a prefix without operations' commitment in it, and no position in the merged journal
offers that prefix — the program prints which entry each one trips on. So what crossed over is the
`Decision` verbatim, re-witnessed against the merged prefix. That is the recovery `converge`
prescribes to a refused party ("read again and admit again"), and per the note on `ConvergeError::
Diverged` it is what experiment 15 found is always left to do.

**Nothing was arbitrated and no new intention was invented.** Every one of the five decisions in the
final record is verbatim one the two parties took. The two forks stay as two branches, because a
lineage is a tree.

## What `mine/` holds at the end

`mine/current` points at generation `b`; `mine/a` is the untouched pre-merge state, which is the
crate's rollback. Generation `b` holds 21 journal entries, 21 custody addresses, 5 decisions and 5
worlds, and it reconstructs from disk for a reader told nothing:

```
74a6a53e4e4d  parent -              by (nobody)     known_at 2026-01-06  open 2f54506a
1f093bfa4767  parent 74a6a53e4e4d   by operations   known_at 2026-01-07  open 2f54506a
b939289591aa  parent 1f093bfa4767   by operations   known_at 2026-01-07  open 4b8b9b88
558f991d1bd3  parent 74a6a53e4e4d   by finance      known_at 2026-01-08  open 2f54506a
dd201a84e58e  parent 558f991d1bd3   by finance      known_at 2026-01-08  open 2f54506a, 652a011d
```

All six world identities either record claimed — including both of finance's — come back
**identically**. That was not assumed either: the program compares against both `worlds.json` files
and refuses to write if any world fails to reproduce. It survives here because the divergent entries
are Commitments, which move no Event head, and an advance absorbs only what its cut froze. Had
either side's extra entry been an Event, finance's cut would have resolved differently and its two
worlds would have been lost by the retake.

## For every object I construct, what it asserts

1. **The merged journal** (`Vec<Admission>`, 21 entries) — asserts that this is the whole sequence of
   what entered the record, in the order it entered. Twenty of its entries are operations' own,
   unchanged. The twenty-first is cloned verbatim from finance's journal and asserts: *the market is
   accountable, and is the executor, for the house's benefit, of receiving 40 into the account,
   committed on 2026-01-08 and due 2026-01-12; and this entered the record on 2026-01-08.*
2. **Finance's advance, retaken** (`Taken`) — asserts: *finance decided to recognize history up to
   2026-01-08 under the world `74a6a53e…`; when that decision was applied, these 21 entries stood.*
3. **Finance's fork, retaken** (`Taken`) — asserts: *finance decided that, under the world
   `558f991d…`, nothing is dropped and the commitment `652a011d…` is added to what is proposed; when
   that decision was applied, these 21 entries stood.*
4. **The `Corroborated` handed to `converge`** — asserts that this journal, this lineage and the
   worlds they produce are one mutually consistent record, built together from one read.
5. **The `ApplicabilityReport`** — asserts what finance's tip would come to inside operations' tip,
   measured against the world both left from. It is read, not stored and not decided.

## What I did not do, and why

**I did not collapse the two tips into one world.** Synthesis was asked and answered: finance's
intention relative to the base is *introduce `652a011d…`*, and applying that to operations' tip is
**conflicted** — `652a011d…` was recorded 2026-01-08 and operations' tip recognizes history only to
2026-01-07. So the two intentions do not merely disagree about the commitment operations dropped;
one of them names knowledge the other's world has not yet recognized. Reconciling them would take
two decisions neither party made — operations advancing its own tip to finance's instant, and then
somebody answering what becomes of `2f54506a…`, which operations omitted and finance kept. That
answer is not derivable from either record, so I left it open.

**I did not preserve finance's original witnesses.** This is the one claim from finance's record
that the merged record does not carry, and it is a real loss rather than a technicality. Finance's
`lineage.json` asserted that finance decided against a prefix that did *not* contain operations'
commitment. In the merged record no such prefix exists at any position, so the claim cannot be
stated and I did not approximate it. The witnesses now on those two records are true claims about
the merged record's history; they are not finance's claims about its own.

**The `by` field on the retaken decisions is the weakest thing in the result, and I want it named.**
It says `finance`, because finance is the party whose decision it is and the decision content is
verbatim finance's. But operations is the party that re-applied it. The record has no way to
distinguish *finance took this against its own prefix* from *operations retook finance's intention
against the merged prefix* — `transfer::applied` says the same thing about its own case ("the record
says which commitments were introduced and never that another line of thinking is why"). I chose the
attribution that keeps the provenance of the intention, and the cost is that the record slightly
overstates what finance witnessed. Nothing in the crate makes the honest version representable.

## What I needed and could not find

- **No operation takes two divergent records as its subject.** `converge` merges a party's working
  copy into a repository, and requires extension. `converge::holds` can tell you whether two
  repositories agree about one world, one identity at a time — its own docstring says it "is
  currently the only thing in the application that a second repository is answerable to". Synthesis
  merges two *worlds inside one archive*, not two archives. So the reconciliation of two records is
  not an operation the crate offers; the journal splice, the retake and the ordering are assembled
  in `src/main.rs` out of the parts (`journal::replay`, `Taken::claimed`, `lineage::rebuild`,
  `converge`). Every rule that governs the result is still the crate's — I supplied no policy of my
  own — but the composition is mine and nothing in the crate guards it.
- **No way to record that a decision was re-applied by someone other than its author.** Described
  above. This is the gap that made the `by` choice a judgement call instead of a lookup.
- **No `FromStr` on the id types.** They serialize and deserialize as hex and implement `Display`,
  but there is no way to turn a hex string back into a `ThesisId` without going through serde. I
  worked around it by never parsing one: identities come from the rebuilt `Thesis` values and are
  compared as strings against `worlds.json`. Worth flagging because the workaround looks clean and
  is hiding a missing constructor.
- **A repository-level red test for my own guard could not be built.** The check that decides
  whether anything is written ("every world either record claimed comes back identically") never
  refuses in this arrangement. I tried to make it refuse by editing a copy of finance's record, but
  the crate catches a tampered record first — `reading::corroborated` refuses it before my guard is
  reached, which is the crate working correctly and my guard still unexercised. Producing a case
  where both records stand alone yet a world dies in the merge needs an Event on the divergent side,
  which means hand-authoring a record I would have had to fabricate. So the guard is exercised at
  function level instead: `unreproduced` is a named function with two unit tests, and I confirmed
  they go red with the right message by inverting its filter (`cargo test`: 2 passed; inverted: 2
  failed, naming the world that did not come back). The repository-level wiring around it is
  unproven, and I would rather say so than let the green suite imply otherwise.

## Every path I read

Inside this directory:

- `GOAL.md`, `Cargo.toml`, `Cargo.lock`
- `src/main.rs` (written)
- `ANSWER.md` (written)
- `mine/current`, `mine/a/journal.json`, `mine/a/lineage.json`, `mine/a/worlds.json`, `mine/a/custody.json`
- `mine/b/journal.json`, `mine/b/lineage.json`, `mine/b/worlds.json`, `mine/b/custody.json` (written by the program, then read back)
- `theirs/current`, `theirs/a/journal.json`, `theirs/a/lineage.json`, `theirs/a/worlds.json`, `theirs/a/custody.json` — read only; never written (mtimes unchanged)
- `cli/Cargo.toml`, `cli/src/lib.rs`, `cli/src/converge.rs`, `cli/src/transfer.rs`, `cli/src/reading.rs`, `cli/src/lineage.rs`, `cli/src/journal.rs`, `cli/src/repository.rs`, `cli/src/error.rs`
- `core/Cargo.toml`, `core/src/macros.rs`, `core/src/kernel/value_objects/identifier.rs`,
  `core/src/engine/synthesis/mod.rs`, `core/src/engine/synthesis/merge.rs`,
  `core/src/engine/synthesis/report.rs`, `core/src/engine/synthesis/difference.rs`,
  `core/src/engine/synthesis/transfer.rs`, `core/src/engine/thesis/mod.rs`,
  `core/src/engine/thesis/scenario.rs`
- `core/` and `cli/` were also listed whole (`find … -name '*.rs'`), so I saw the names of every
  source file in both crates. Files I looked at and did not use: `cli/src/archive.rs`,
  `cli/src/history.rs`, `cli/src/level.rs` — named in the listing and imported by the program
  through the modules above, but never opened.
- `target/` (build output only)

Outside this directory, all under the session scratchpad
`/tmp/claude-1000/-home-neto-sant-ana-ape/5e82c357-fb25-4ade-b75f-575633c91e70/scratchpad/`, all
written by me and all disposable:

- `mine-pristine/` — a copy of `mine/` taken before the first run, so the run could be repeated from
  the original state. `mine/a` was diffed against it at the end and is identical.
- `mutation/m/`, `mutation/t/` — copies used for the failed attempt to make the reproduction guard
  refuse.
- `swap/as-mine/`, `swap/as-theirs/` — copies used to run the program with the parties reversed.

Nothing was read outside this directory and that scratchpad. (The harness placed the operator's
global rules and project memory in my context; those are not paths I opened.)
