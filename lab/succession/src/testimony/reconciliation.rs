//! `05-reconciliation/run-a` — the last testimony, and **the one the five kinds were derived from**.
//!
//! # It cannot test what it was pre-registered against, and that is this file's main finding
//!
//! [`07`](../../00-testimony/07-the-line-is-whether-the-reader-must-choose.md) predicted, before this
//! file was opened, that a run written for somebody who will *check* would land under 3 unclassified
//! claims. It landed at **zero**.
//!
//! That confirms nothing. The five kinds were derived from this testimony and from no other, so every
//! claim in it fitting one of them is close to a tautology — and the pre-registration should have said
//! so instead of treating this as the split's last test. It was written in good faith and it was a
//! near-vacuous test, which is worth more as a recorded mistake than as a confirmation.
//!
//! **The split rests on the seven testimonies this file did not produce**, where it is 2 of 110
//! against 15 of 39.
//!
//! # What it does contribute
//!
//! Twenty-two unhoused claims, and the fact that the categories fit it is the *premise* rather than
//! the result. Two of them are the corpus's sharpest H4 material — a want the record cannot represent
//! at all, and the crate's own sentence about what a fork never says.

use crate::classification::{Carrier, Claim, Derived, File, Kind, Standing, Verdict};
use crate::corpus::Run;

const fn housed(text: &'static str, carrier: Carrier) -> Claim {
    Claim {
        run: Run::Reconciliation,
        text,
        verdict: Verdict::Housed(carrier),
        standing: None,
    }
}

const fn unhoused(text: &'static str, kind: Kind) -> Claim {
    Claim {
        run: Run::Reconciliation,
        text,
        verdict: Verdict::Unhoused(Some(kind)),
        standing: None,
    }
}

const fn want(text: &'static str, standing: Standing) -> Claim {
    Claim {
        run: Run::Reconciliation,
        text,
        verdict: Verdict::Unhoused(Some(Kind::Want)),
        standing: Some(standing),
    }
}

const fn loss(text: &'static str, standing: Standing) -> Claim {
    Claim {
        run: Run::Reconciliation,
        text,
        verdict: Verdict::Unhoused(Some(Kind::Loss)),
        standing: Some(standing),
    }
}

const fn exposition(text: &'static str) -> Claim {
    Claim {
        run: Run::Reconciliation,
        text,
        verdict: Verdict::Exposition,
        standing: None,
    }
}

const COMMITMENT: Carrier = Carrier::Entity("Commitment");
const WORLDS: Carrier = Carrier::File(File::Worlds);
const LINEAGE: Carrier = Carrier::File(File::Lineage);
const JOURNAL: Carrier = Carrier::File(File::Journal);
const CUSTODY: Carrier = Carrier::File(File::Custody);
const APPLICABILITY: Carrier = Carrier::Derived(Derived::Applicability);

const IN_THE_QUEUE: Standing = Standing::Tracked("lab/QUEUE.md");

pub const CLAIMS: &[Claim] = &[
    housed(
        "Both journals hold 20 entries and their first 19 are identical, entry for entry and instant \
         for instant. Each holds a twentieth the other has never seen:",
        JOURNAL,
    ),
    housed(
        "| operations | `4b8b9b88…` | the house is accountable for spending 60 to the market by \
         2026-01-20 | 2026-01-07 |",
        COMMITMENT,
    ),
    housed(
        "| finance | `652a011d…` | the market is accountable for receiving 40 into the account by \
         2026-01-12 | 2026-01-08 |",
        COMMITMENT,
    ),
    housed(
        "Operations advanced to 2026-01-07 and forked, dropping the open commitment `2f54506a…` and \
         putting its own in its place. Finance advanced to 2026-01-08 and forked, keeping `2f54506a…` \
         and adding its own beside it. Four worlds, two per party, no overlap.",
        WORLDS,
    ),
    housed(
        "Each party's decisions name that party in `by`: `326993e9…` is operations, `10807723…` is \
         finance (the program derives this from the journal rather than assuming it).",
        LINEAGE,
    ),
    // ---- What I did --------------------------------------------------------------------------
    unhoused(
        "**`converge(mine, theirs)` refuses, and I measured that rather than assuming it.** Its \
         comparison is over *sequence* — one journal must extend the other — and neither does.",
        Kind::MethodLimit,
    ),
    housed(
        "The refusal names position 19 and adds that the two share 19 entries, \"so they are divergent \
         rather than incompatible\". Nothing was written.",
        JOURNAL,
    ),
    exposition(
        "An `EntryId` is derived from what admitting produced, so finance's entry is the same \
         knowledge in either record.",
    ),
    housed(
        "The merged journal is operations' 20 followed by finance's 1. That order is not a preference: \
         the Canon refuses an admission dated before its watermark, so the party that learned earlier \
         lands first.",
        JOURNAL,
    ),
    unhoused(
        "I measured the other order too — the Canon rejects it outright […] Running the whole program \
         with the roles swapped fails at exactly that point, which means operations was the only side \
         of this pair that could host the merge at all.",
        Kind::MethodLimit,
    ),
    exposition(
        "A `Taken` is a decision *plus* the exact prefix it stood on, and `lineage::rebuild` demands \
         the two match in both directions.",
    ),
    housed(
        "Finance's two records claim a prefix without operations' commitment in it, and no position in \
         the merged journal offers that prefix — the program prints which entry each one trips on.",
        LINEAGE,
    ),
    housed(
        "So what crossed over is the `Decision` verbatim, re-witnessed against the merged prefix.",
        LINEAGE,
    ),
    housed(
        "**Nothing was arbitrated and no new intention was invented.** Every one of the five decisions \
         in the final record is verbatim one the two parties took. The two forks stay as two branches, \
         because a lineage is a tree.",
        LINEAGE,
    ),
    // ---- What mine/ holds at the end ---------------------------------------------------------
    housed(
        "Generation `b` holds 21 journal entries, 21 custody addresses, 5 decisions and 5 worlds, and \
         it reconstructs from disk for a reader told nothing:",
        CUSTODY,
    ),
    housed(
        "All six world identities either record claimed — including both of finance's — come back \
         **identically**.",
        WORLDS,
    ),
    unhoused(
        "That was not assumed either: the program compares against both `worlds.json` files and \
         refuses to write if any world fails to reproduce.",
        Kind::MethodLimit,
    ),
    exposition(
        "It survives here because the divergent entries are Commitments, which move no Event head, and \
         an advance absorbs only what its cut froze.",
    ),
    unhoused(
        "Had either side's extra entry been an Event, finance's cut would have resolved differently \
         and its two worlds would have been lost by the retake.",
        Kind::Qualification,
    ),
    // ---- For every object I construct, what it asserts ---------------------------------------
    housed(
        "asserts that this is the whole sequence of what entered the record, in the order it entered. \
         Twenty of its entries are operations' own, unchanged.",
        JOURNAL,
    ),
    housed(
        "*finance decided to recognize history up to 2026-01-08 under the world `74a6a53e…`; when that \
         decision was applied, these 21 entries stood.*",
        LINEAGE,
    ),
    housed(
        "*finance decided that, under the world `558f991d…`, nothing is dropped and the commitment \
         `652a011d…` is added to what is proposed; when that decision was applied, these 21 entries \
         stood.*",
        LINEAGE,
    ),
    exposition(
        "asserts that this journal, this lineage and the worlds they produce are one mutually \
         consistent record, built together from one read.",
    ),
    housed(
        "asserts what finance's tip would come to inside operations' tip, measured against the world \
         both left from. It is read, not stored and not decided.",
        APPLICABILITY,
    ),
    // ---- What I did not do, and why ----------------------------------------------------------
    housed(
        "Synthesis was asked and answered: finance's intention relative to the base is *introduce \
         `652a011d…`*, and applying that to operations' tip is **conflicted** — `652a011d…` was \
         recorded 2026-01-08 and operations' tip recognizes history only to 2026-01-07.",
        APPLICABILITY,
    ),
    housed(
        "So the two intentions do not merely disagree about the commitment operations dropped; one of \
         them names knowledge the other's world has not yet recognized.",
        APPLICABILITY,
    ),
    unhoused(
        "Reconciling them would take two decisions neither party made — operations advancing its own \
         tip to finance's instant, and then somebody answering what becomes of `2f54506a…`, which \
         operations omitted and finance kept. That answer is not derivable from either record, so I \
         left it open.",
        Kind::RoadNotTaken,
    ),
    // The mechanism is in 15's result — "the witness it writes is true" — but as a property that
    // holds, never as a cost. This is the first time the loss is named.
    loss(
        "**I did not preserve finance's original witnesses.** This is the one claim from finance's \
         record that the merged record does not carry, and it is a real loss rather than a \
         technicality.",
        Standing::Recorded("lab/frontier/docs/15-assimilation/99-result.md"),
    ),
    housed(
        "Finance's `lineage.json` asserted that finance decided against a prefix that did *not* \
         contain operations' commitment. In the merged record no such prefix exists at any position, \
         so the claim cannot be stated and I did not approximate it.",
        LINEAGE,
    ),
    unhoused(
        "The witnesses now on those two records are true claims about the merged record's history; \
         they are not finance's claims about its own.",
        Kind::Qualification,
    ),
    unhoused(
        "**The `by` field on the retaken decisions is the weakest thing in the result, and I want it \
         named.**",
        Kind::Qualification,
    ),
    want(
        "The record has no way to distinguish *finance took this against its own prefix* from \
         *operations retook finance's intention against the merged prefix*",
        IN_THE_QUEUE,
    ),
    exposition(
        "the record says which commitments were introduced and never that another line of thinking is \
         why",
    ),
    unhoused(
        "I chose the attribution that keeps the provenance of the intention, and the cost is that the \
         record slightly overstates what finance witnessed.",
        Kind::RoadNotTaken,
    ),
    // ---- What I needed and could not find ----------------------------------------------------
    want(
        "**No operation takes two divergent records as its subject.** `converge` merges a party's \
         working copy into a repository, and requires extension. […] Synthesis merges two *worlds \
         inside one archive*, not two archives.",
        IN_THE_QUEUE,
    ),
    unhoused(
        "Every rule that governs the result is still the crate's — I supplied no policy of my own — \
         but the composition is mine and nothing in the crate guards it.",
        Kind::Qualification,
    ),
    want(
        "**No way to record that a decision was re-applied by someone other than its author.** \
         Described above. This is the gap that made the `by` choice a judgement call instead of a \
         lookup.",
        IN_THE_QUEUE,
    ),
    want(
        "**No `FromStr` on the id types.** They serialize and deserialize as hex and implement \
         `Display`, but there is no way to turn a hex string back into a `ThesisId` without going \
         through serde.",
        Standing::Untracked,
    ),
    unhoused(
        "Worth flagging because the workaround looks clean and is hiding a missing constructor.",
        Kind::MethodLimit,
    ),
    unhoused(
        "**A repository-level red test for my own guard could not be built.** The check that decides \
         whether anything is written […] never refuses in this arrangement.",
        Kind::MethodLimit,
    ),
    unhoused(
        "I tried to make it refuse by editing a copy of finance's record, but the crate catches a \
         tampered record first — `reading::corroborated` refuses it before my guard is reached, which \
         is the crate working correctly and my guard still unexercised.",
        Kind::MethodLimit,
    ),
    unhoused(
        "Producing a case where both records stand alone yet a world dies in the merge needs an Event \
         on the divergent side, which means hand-authoring a record I would have had to fabricate.",
        Kind::RoadNotTaken,
    ),
    unhoused(
        "the guard is exercised at function level instead: `unreproduced` is a named function with two \
         unit tests, and I confirmed they go red with the right message by inverting its filter",
        Kind::MethodLimit,
    ),
    unhoused(
        "The repository-level wiring around it is unproven, and I would rather say so than let the \
         green suite imply otherwise.",
        Kind::Qualification,
    ),
    // ---- Every path I read ---------------------------------------------------------------------
    unhoused(
        "read only; never written (mtimes unchanged)",
        Kind::MethodLimit,
    ),
    unhoused(
        "Nothing was read outside this directory and that scratchpad.",
        Kind::MethodLimit,
    ),
];
