//! Experiment 16 — Custody. Phases against `lab/frontier/docs/16-custody/00-protocol.md`.
//!
//! The question: does a record's one claim about its own history cover everything the record holds,
//! or does it stop at the last decision — and is what lies past it lost without consequence?
//!
//! *Covers* is settled before any phase runs: **a claim covers an entry when a record that lost it,
//! or gained one beside it, is refused because of that claim.** Not *mentions*, not *is reachable
//! by replay*, and not *changes an answer* — what a claim covers and what an answer depends on are
//! different sets, which is the trap this whole experiment is built to walk around.

use std::collections::BTreeSet;

use ape_cli::error::{LineageError, ReadingError};
use ape_cli::journal::{Admission, EntryId};
use ape_cli::lineage::Taken;
use ape_cli::reading::{self, WorldRecord};

use ape_frontier::subject::custody::{
    self, DECIDED, DECIDED_AGAIN, DECISIONS, ENTRIES, Moved, PREFIX, REACHED, STATES, TAIL,
    UNREACHED,
};

/// A repository path no other process shares, emptied before it is used.
fn scratch(named: &str) -> ape_cli::repository::Repository {
    let path = std::env::temp_dir()
        .join(format!("ape-custody-{}", std::process::id()))
        .join(named);
    let _ = std::fs::remove_dir_all(&path);

    ape_cli::repository::Repository::open(path)
}

/// The record as it stood when this experiment was written: three files, and no claim about itself.
///
/// Which is the shape the question is about, and it is not a museum piece — it is what every
/// repository written before Phase 6 looks like, including the four under
/// `lab/agents/04-multiagent` that were written by parties nobody can re-run. The three files are
/// written one at a time on purpose: a whole write now derives a fourth, and a phase measuring the
/// three guards must not be handed the guard it exists to say was missing.
fn unclaimed(named: &str, journal: &[Admission]) -> ape_cli::repository::Repository {
    let arrangement = custody::arranged().expect("the subject is admissible");
    let repository = scratch(named);

    repository.write_journal(journal).expect("one file");
    repository
        .write_lineage(&arrangement.files.lineage)
        .expect("one file");
    repository
        .write_worlds(&arrangement.files.worlds)
        .expect("one file");

    repository
}

/// The same record after a whole write, with one file edited afterwards from outside.
///
/// `write_journal` is public because five experiments need a record somebody else touched, and the
/// record's defence against one is corroboration. What Phase 6 measures is whether that defence now
/// reaches the half of the journal no witness does.
fn edited(named: &str, journal: &[Admission]) -> ape_cli::repository::Repository {
    let arrangement = custody::arranged().expect("the subject is admissible");
    let repository = scratch(named);

    custody::write_whole(&repository, &arrangement.files).expect("a whole write");
    repository.write_journal(journal).expect("one file, edited");

    repository
}

/// The last world of a record, as the file that witnesses it describes one.
fn last(files: &custody::Files) -> WorldRecord {
    files
        .worlds
        .last()
        .expect("the record decided something")
        .clone()
}

// ---------------------------------------------------------------------------------------------
// Phase 0 — What claims the tail today
// ---------------------------------------------------------------------------------------------

/// The arrangement holds what it says it holds, and every literal is read once.
#[test]
fn the_arrangement_is_what_the_subject_says_it_is() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    assert_eq!(arrangement.files.journal.len(), ENTRIES, "the journal");
    assert_eq!(arrangement.files.lineage.len(), DECISIONS, "the lineage");
    assert_eq!(arrangement.files.worlds.len(), DECISIONS, "the worlds");

    let tail = custody::tail(&arrangement.files).expect("the tail is derivable");

    assert_eq!(
        tail.len(),
        TAIL,
        "what the record holds past its coordinate"
    );
    assert_eq!(
        arrangement.files.journal.len() - tail.len(),
        PREFIX,
        "and what it holds before it"
    );

    let repository = scratch("as-written");
    custody::write_whole(&repository, &arrangement.files).expect("a whole write");

    let read = reading::reconstruct(&repository, arrangement.account, &custody::asked_at())
        .expect("the record reads");

    assert_eq!(read.len(), DECISIONS);
    assert_eq!(
        (read[0].level, read[0].level),
        DECIDED,
        "and answers what the arrangement said it would"
    );
}

/// The last decision's witness names the prefix entire and not one entry of the tail.
///
/// The experiment's first sentence, measured rather than argued. Both sets are derived from the same
/// journal by different routes: the witness is what the record wrote down, and the tail is what the
/// journal offers past the coordinate that witness ends at.
#[test]
fn the_last_decisions_witness_names_the_prefix_and_none_of_the_tail() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    let held = custody::addresses(&arrangement.files.journal).expect("the journal admits");
    let witness = &arrangement
        .files
        .lineage
        .last()
        .expect("the record decided something")
        .witness;

    let prefix: BTreeSet<&EntryId> = held[..PREFIX].iter().collect();
    let tail: BTreeSet<&EntryId> = held[PREFIX..].iter().collect();
    let witnessed: BTreeSet<&EntryId> = witness.iter().collect();

    // Named rather than compared whole: two sets of twenty-two addresses printed side by side is a
    // reader's problem, and what a phase owes is the entry that differs.
    if let Some(missing) = prefix.difference(&witnessed).next() {
        panic!("the witness does not name {missing}, which stood before the coordinate");
    }

    if let Some(beyond) = witnessed.difference(&prefix).next() {
        panic!("the witness names {beyond}, which is not in the prefix");
    }

    if let Some(claimed) = witnessed.intersection(&tail).next() {
        panic!("the witness names {claimed}, which the record holds past its coordinate");
    }

    assert_eq!((witnessed.len(), tail.len()), (PREFIX, TAIL));
}

/// Nor does any world the record recorded mention anything the tail holds.
///
/// The second of the three files, read for the same negative. A `worlds.json` names identities —
/// a thesis, its parent, the chain it recognizes, what it froze and what it proposes — and the
/// question is whether any of them is a tail entry. None is, and that is the whole of what the third
/// guard has to say about the tail.
#[test]
fn no_world_the_record_recorded_mentions_anything_the_tail_holds() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    let named = custody::named(&arrangement.files.worlds);
    let tail = [
        arrangement.outflow.to_string(),
        arrangement.filings[0].to_string(),
        arrangement.filings[1].to_string(),
    ];

    for identity in &tail {
        assert!(
            !named.contains(identity),
            "the worlds file names {identity}, which the tail holds"
        );
    }

    assert!(
        named.contains(&arrangement.fund.to_string()),
        "and it does name what the prefix froze, so the reading is not vacuous"
    );
}

/// And the prefix holds an entry of the same class as the tail's unreached half.
///
/// The control, and the comparison the experiment turns on. The `observer` role is referred to by
/// nothing and no world is a function of it — exactly like a filing. The difference between the two
/// is not what they are; it is which side of the coordinate they sit on.
#[test]
fn the_prefix_holds_an_entry_of_the_same_class_as_the_tails_unreached_half() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    let named = custody::named(&arrangement.files.worlds);

    assert!(
        !named.contains(&arrangement.unreferenced.to_string()),
        "no world is a function of it"
    );
    assert!(
        custody::addresses(&arrangement.moved(Moved::OneFromThePrefix)).is_ok(),
        "and nothing admitted after it refers to it, so a journal without it still admits"
    );
    assert!(
        arrangement
            .files
            .lineage
            .last()
            .expect("the record decided something")
            .witness
            .contains(&arrangement.unreferenced),
        "and the witness names it anyway, because it stood"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 1 — Losing it, and gaining beside it
// ---------------------------------------------------------------------------------------------

/// Every move past the last coordinate reads, and the world comes back unchanged.
///
/// U1 and U2 together, because a claim that caught one direction and not the other would be a
/// different shape and the phase has to be able to say so.
#[test]
fn a_journal_moved_past_the_last_coordinate_still_reads() {
    let arrangement = custody::arranged().expect("the subject is admissible");
    let original = last(&arrangement.files);

    for state in STATES {
        if state == Moved::OneFromThePrefix {
            continue;
        }

        let journal = arrangement.moved(state);
        let repository = unclaimed(
            &format!("reads-{}", state.named().replace(' ', "-")),
            &journal,
        );

        // The phase asserts that nothing refuses, and an untouched record refuses nothing either —
        // so the edit is read back off the disk before the reading is taken. Without this the whole
        // of Phase 1 is satisfied by an arrangement it could not have failed.
        assert_eq!(
            repository
                .read_journal()
                .expect("the journal is there")
                .len(),
            journal.len(),
            "{} is the record the phase reads",
            state.named()
        );

        let read = reading::reconstruct(&repository, arrangement.account, &custody::asked_at())
            .unwrap_or_else(|refusal| panic!("{}: {refusal}", state.named()));

        assert_eq!(read.len(), DECISIONS, "{}", state.named());
        assert_eq!(
            (read[0].level, read[0].level),
            DECIDED,
            "{} answers what it answered",
            state.named()
        );
        assert_eq!(
            read[0].thesis,
            original.thesis,
            "{} produces the world it produced",
            state.named()
        );
    }
}

/// And the same move before the coordinate is refused, naming the entry.
///
/// The red half of the phase, and it is a row of the table rather than an aside: the entry removed
/// here is of the same class as the two the phase above removed without consequence.
#[test]
fn the_same_move_before_the_coordinate_is_refused() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    let repository = unclaimed(
        "refused-prefix",
        &arrangement.moved(Moved::OneFromThePrefix),
    );

    let refusal = reading::reconstruct(&repository, arrangement.account, &custody::asked_at())
        .expect_err("the prefix is covered");

    assert!(
        matches!(
            refusal,
            ReadingError::Lineage(LineageError::WitnessedKnowledgeAbsent { ref entry })
                if entry == &arrangement.unreferenced
        ),
        "the witness names the entry that went missing, and it is the one removed: {refusal}"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 2 — What the next decision stands on
// ---------------------------------------------------------------------------------------------

/// For each state: how long the journal is, whether deciding again reaches the world the whole
/// record's later decision reaches, and what that world answers on the account.
///
/// Pre-registered. The fourth column is the trap the protocol names — two rows produce a **different
/// world** and the second of them answers the **same numbers**, because the entry it gained moves a
/// resource no world about cash weighs.
#[rustfmt::skip]
const STOOD: [(Moved, usize, bool, (i128, i128)); 7] = [
    (Moved::Nothing,                      ENTRIES,            true,  DECIDED_AGAIN),
    (Moved::TheWholeTail,                 PREFIX,             false, DECIDED),
    (Moved::WhatNothingReaches,           PREFIX + REACHED,   true,  DECIDED_AGAIN),
    (Moved::WhatSomethingReaches,         PREFIX + UNREACHED, false, DECIDED),
    (Moved::OneMoreNothingReaches,        ENTRIES + 1,        true,  DECIDED_AGAIN),
    (Moved::OneMoreSomethingReaches,      ENTRIES + 1,        false, DECIDED_AGAIN),
    (Moved::OneMoreUnsettledOnTheAccount, ENTRIES + 1,        true,  DECIDED_AGAIN),
];

/// What a later decision stands on moves with the tail, and it is a legitimate record either way.
///
/// U3, and it is the experiment. Every one of these records reads, decides, and writes a whole
/// repository nothing refuses — and two of them stand on a prefix that is missing entries their
/// author never noticed losing.
#[test]
fn what_a_later_decision_stands_on_moves_with_the_tail() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    let whole = custody::deciding_again(&arrangement.files.lineage, &arrangement.files.journal)
        .expect("the record decides again");
    let reference = last(&whole);

    for (state, entries, same, answers) in STOOD {
        let journal = arrangement.moved(state);
        assert_eq!(journal.len(), entries, "{}", state.named());

        let files = custody::deciding_again(&arrangement.files.lineage, &journal)
            .unwrap_or_else(|refusal| panic!("{}: {refusal}", state.named()));

        let world = last(&files);
        assert_eq!(
            (state.named(), world.thesis == reference.thesis),
            (state.named(), same),
            "{} and the world the whole record reaches",
            state.named()
        );

        let repository = scratch(&format!("again-{}", state.named().replace(' ', "-")));
        custody::write_whole(&repository, &files).expect("a whole write");

        let read = reading::reconstruct(&repository, arrangement.account, &custody::asked_at())
            .unwrap_or_else(|refusal| panic!("{}: {refusal}", state.named()));

        let latest = read.last().expect("the later decision");
        assert_eq!(
            (state.named(), (latest.level, latest.level)),
            (state.named(), answers),
            "{} answers",
            state.named()
        );
    }
}

/// And a record that lost an entry from its prefix cannot decide again at all.
///
/// Which is the difference stated as a capability rather than as a refusal: the covered half of a
/// journal cannot move without the record noticing, so nothing can be built on top of the move.
#[test]
fn a_record_that_lost_an_entry_from_its_prefix_cannot_decide_again() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    let refusal = custody::deciding_again(
        &arrangement.files.lineage,
        &arrangement.moved(Moved::OneFromThePrefix),
    )
    .expect_err("the prefix is covered");

    assert!(
        matches!(
            refusal,
            ape_cli::error::SubjectError::Lineage(LineageError::WitnessedKnowledgeAbsent {
                ref entry
            }) if entry == &arrangement.unreferenced
        ),
        "and the refusal is the witness's, naming the entry: {refusal}"
    );
}

/// A tail entry is lost alone only where nothing admitted after it refers to it.
///
/// Per entry, derived rather than declared: each of the four is removed by itself and the record is
/// asked to decide again. Three outcomes, and the first is not one the protocol predicted — an entry
/// of the tail is protected after all, by the entry that names it rather than by any claim.
#[test]
fn a_tail_entry_is_lost_alone_only_where_nothing_refers_to_it() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    let whole = custody::deciding_again(&arrangement.files.lineage, &arrangement.files.journal)
        .expect("the record decides again");
    let reference = last(&whole);

    let outcomes: Vec<&'static str> = (PREFIX..ENTRIES)
        .map(|position| {
            let journal = custody::without(&arrangement.files.journal, position);

            match custody::deciding_again(&arrangement.files.lineage, &journal) {
                Err(_) => "the journal does not admit",
                Ok(files) if last(&files).thesis == reference.thesis => "nothing moved",
                Ok(_) => "a different world",
            }
        })
        .collect();

    assert_eq!(
        outcomes,
        [
            "the journal does not admit",
            "a different world",
            "nothing moved",
            "nothing moved",
        ],
        "each entry of the tail, removed by itself"
    );
}

/// What makes the tail's unreached half unreached is that no Event names it, not the resource.
///
/// Recorded as a finding rather than as a prediction: the arrangement puts the filings on a second
/// resource so that a *gained* Event can move a world without moving a number, and reading that as
/// the reason they are unreached would be a claim about the instrument the instrument does not make.
/// Measured by admitting one more outflow on the very resource every world here is about, and
/// leaving it unsettled — the later decision reaches it exactly as much as it reaches a filing,
/// which is not at all.
#[test]
fn the_unreached_half_is_unreached_because_no_event_names_it() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    let whole = custody::deciding_again(&arrangement.files.lineage, &arrangement.files.journal)
        .expect("the record decides again");

    let gained = custody::deciding_again(
        &arrangement.files.lineage,
        &arrangement.moved(Moved::OneMoreUnsettledOnTheAccount),
    )
    .expect("the record decides again");

    assert_eq!(
        last(&gained).thesis,
        last(&whole).thesis,
        "an unsettled commitment on the account moves the later world no more than a filing does"
    );
}

/// But a fork does reach it, so losing the unreached half costs a decision the record cannot take.
///
/// U3's second half, and the phase that settles whether the tail's quiet half is inert. An advance
/// recognizes what history settled and never selects an open commitment; a fork names one outright.
/// The two records here produce the **same** later world by identity, and only one of them can go on
/// to fork it — which is what *the loss changes what a later decision stands on* looks like when the
/// number did not move.
#[test]
fn a_fork_reaches_the_unreached_half_and_a_record_that_lost_it_cannot_take_one() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    let whole = custody::deciding_again(&arrangement.files.lineage, &arrangement.files.journal)
        .expect("the record decides again");
    let lost = custody::deciding_again(
        &arrangement.files.lineage,
        &arrangement.moved(Moved::WhatNothingReaches),
    )
    .expect("the truncated record decides again");

    assert_eq!(
        last(&whole).thesis,
        last(&lost).thesis,
        "the two stand on the same world, which is what makes the comparison controlled"
    );

    let forked = custody::introducing(&whole, arrangement.filings[0])
        .expect("the whole record can still propose what it holds");

    assert!(
        last(&forked)
            .open
            .contains(&arrangement.filings[0].to_string()),
        "and the world it reaches proposes it"
    );

    let refusal = custody::introducing(&lost, arrangement.filings[0])
        .expect_err("the truncated record cannot propose what it no longer holds");

    assert!(
        format!("{refusal}").contains(&arrangement.filings[0].to_string()),
        "the refusal names the commitment: {refusal}"
    );
    assert!(
        !format!("{refusal}").contains("journal"),
        "and says nothing about the journal being short, which is the fact: {refusal}"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 3 — Whether a witness could reach it
// ---------------------------------------------------------------------------------------------

/// A witness widened to name a tail entry is refused by the record itself.
///
/// U4, and the phase answers it by what a witness is written **from** rather than by failing to think
/// of a wider one. `Taken::now` takes its witness from the replay standing at the moment the decision
/// is taken, so a witness naming more than that is not a wider claim — it is a claim about knowledge
/// the record cannot show stood, and `corroborate` says which entry.
#[test]
fn a_witness_widened_to_name_a_tail_entry_is_refused() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    let held = custody::addresses(&arrangement.files.journal).expect("the journal admits");
    let first_of_the_tail = held[PREFIX].clone();

    let taken = arrangement
        .files
        .lineage
        .last()
        .expect("the record decided something");

    let mut witness = taken.witness.clone();
    witness.insert(first_of_the_tail.clone());

    let widened = vec![Taken {
        witness,
        ..taken.clone()
    }];

    let files = custody::Files {
        journal: arrangement.files.journal.clone(),
        lineage: widened,
        worlds: arrangement.files.worlds.clone(),
    };

    let repository = scratch("widened");
    custody::write_whole(&repository, &files).expect("a whole write");

    let refusal = reading::reconstruct(&repository, arrangement.account, &custody::asked_at())
        .expect_err("a witness may not outrun its coordinate");

    assert!(
        matches!(
            refusal,
            ReadingError::Lineage(LineageError::WitnessedKnowledgeAbsent { ref entry })
                if entry == &first_of_the_tail
        ),
        "and it names the tail entry the witness reached for: {refusal}"
    );
}

/// So a witness that covers the tail is a decision taken after it — and then there is a new tail.
///
/// The finding U4 was reaching for, and it is stronger than the prediction: the tail is not a corner
/// of a record's life, it is a **fixed point** of it. Deciding again empties it, and one admission
/// refills it.
#[test]
fn a_witness_that_covers_the_tail_is_a_decision_taken_after_it() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    assert_eq!(
        custody::tail(&arrangement.files)
            .expect("the tail is derivable")
            .len(),
        TAIL,
        "before deciding again"
    );

    let again = custody::deciding_again(&arrangement.files.lineage, &arrangement.files.journal)
        .expect("the record decides again");

    assert_eq!(
        custody::tail(&again).expect("the tail is derivable").len(),
        0,
        "the decision that covers the tail is one taken after it"
    );

    let refilled = custody::Files {
        journal: arrangement.moved(Moved::OneMoreNothingReaches),
        ..custody::kept(&again.journal, &again.lineage).expect("the record keeps")
    };

    assert_eq!(
        custody::tail(&refilled)
            .expect("the tail is derivable")
            .len(),
        1,
        "and one admission afterwards opens a new one"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 4 — Reading the three files for a claim about the record
// ---------------------------------------------------------------------------------------------

/// Every key any of the three files writes, derived from the files rather than from the types.
///
/// Closed, and asserted against what the record produces so a family that stopped being written
/// would be reported rather than quietly dropped from the reading.
#[rustfmt::skip]
const KEYS: [(&str, &[&str]); 3] = [
    ("journal.json", &[
        "accountable", "action", "actors", "admits", "agent", "beneficiaries", "cancels",
        "commitment", "committed_at", "dependencies", "due_date", "effective_from", "executors",
        "fulfills", "kind", "label", "magnitude", "observation", "occurred_at", "recipients",
        "recorded_at", "resource", "roles", "statement", "verb",
    ]),
    ("lineage.json", &["after", "decides", "known_at", "selection", "witness"]),
    ("worlds.json",  &["event_head", "frozen", "known_at", "open", "thesis", "thesis_parent"]),
];

/// Each of the three files is a sequence, and every claim in it is about one item of that sequence.
///
/// U5, as the closed list the protocol asks for. There is no top-level object in any of the three,
/// so there is nowhere for a statement whose subject is the record to be written — which also means
/// each file's length is a property of the file, and a truncated file agrees with itself about it.
#[test]
fn every_claim_the_three_files_make_is_about_one_entry_one_decision_or_one_world() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    let repository = scratch("three-files");
    custody::write_whole(&repository, &arrangement.files).expect("a whole write");

    let paths = [
        ("journal.json", repository.journal_path(), ENTRIES),
        ("lineage.json", repository.lineage_path(), DECISIONS),
        ("worlds.json", repository.worlds_path(), DECISIONS),
    ];

    for ((name, path, items), (named, expected)) in paths.into_iter().zip(KEYS) {
        assert_eq!(name, named, "the tables are in step");

        let encoded = std::fs::read_to_string(&path).expect("the file is there");
        let value: serde_json::Value = serde_json::from_str(&encoded).expect("the file is JSON");

        let elements = value
            .as_array()
            .unwrap_or_else(|| panic!("{name} is not a sequence, so it has a place for a claim"));

        assert_eq!(elements.len(), items, "{name} holds one entry per item");

        let mut keys: BTreeSet<&str> = BTreeSet::new();

        for element in elements {
            let object = element
                .as_object()
                .unwrap_or_else(|| panic!("{name} holds something that is not an item"));

            keys.extend(object.keys().map(String::as_str));
        }

        assert_eq!(
            keys,
            expected.iter().copied().collect::<BTreeSet<&str>>(),
            "{name}"
        );
    }
}

// ---------------------------------------------------------------------------------------------
// Phase 5 — The three guards
// ---------------------------------------------------------------------------------------------

/// Which guard answered, where one did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Weighed {
    Coordinate,
    Witness,
    Worlds,
    Custody,
    Nothing,
}

/// What answered, for one repository.
fn weighing(
    repository: &ape_cli::repository::Repository,
    account: ape::kernel::entities::ResourceInstanceId,
) -> Weighed {
    match reading::reconstruct(repository, account, &custody::asked_at()) {
        Ok(_) => Weighed::Nothing,
        Err(ReadingError::Journal(_)) => Weighed::Coordinate,
        Err(ReadingError::Lineage(LineageError::Journal(_))) => Weighed::Coordinate,
        Err(ReadingError::Lineage(_)) => Weighed::Witness,
        Err(ReadingError::UnheldKnowledge { .. } | ReadingError::HeldKnowledgeAbsent { .. }) => {
            Weighed::Custody
        }
        Err(_) => Weighed::Worlds,
    }
}

/// Every state this experiment produced, put to all three guards. Closed.
///
/// Seven of the eight are journals a record's own reader accepts without a word. The eighth differs
/// from two of them only in **where** the entry that went missing sat.
const GUARDS: [(Moved, Weighed); 8] = [
    (Moved::Nothing, Weighed::Nothing),
    (Moved::TheWholeTail, Weighed::Nothing),
    (Moved::WhatNothingReaches, Weighed::Nothing),
    (Moved::WhatSomethingReaches, Weighed::Nothing),
    (Moved::OneMoreNothingReaches, Weighed::Nothing),
    (Moved::OneMoreSomethingReaches, Weighed::Nothing),
    (Moved::OneMoreUnsettledOnTheAccount, Weighed::Nothing),
    (Moved::OneFromThePrefix, Weighed::Witness),
];

#[test]
fn every_state_is_put_to_the_three_guards() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    for (state, expected) in GUARDS {
        let repository = unclaimed(
            &format!("guard-{}", state.named().replace(' ', "-")),
            &arrangement.moved(state),
        );

        assert_eq!(
            repository.read_custody().expect("readable"),
            None,
            "{} is measured against a record that claims nothing about itself",
            state.named()
        );

        assert_eq!(
            (state.named(), weighing(&repository, arrangement.account)),
            (state.named(), expected),
            "{}",
            state.named()
        );
    }
}

// ---------------------------------------------------------------------------------------------
// Phase 6 — Part B, and the same eight states put to a record that claims what it holds
// ---------------------------------------------------------------------------------------------

/// The same table against a record written whole after this experiment. Closed, and it is the point.
///
/// Every state the three guards accepted is now answered, and the one they already answered is still
/// answered by them — the witness runs first, because a claim about a decision is more specific than
/// a claim about the record and sends the reader somewhere narrower.
const IN_CUSTODY: [(Moved, Weighed); 8] = [
    (Moved::Nothing, Weighed::Nothing),
    (Moved::TheWholeTail, Weighed::Custody),
    (Moved::WhatNothingReaches, Weighed::Custody),
    (Moved::WhatSomethingReaches, Weighed::Custody),
    (Moved::OneMoreNothingReaches, Weighed::Custody),
    (Moved::OneMoreSomethingReaches, Weighed::Custody),
    (Moved::OneMoreUnsettledOnTheAccount, Weighed::Custody),
    (Moved::OneFromThePrefix, Weighed::Witness),
];

#[test]
fn a_record_that_claims_what_it_holds_answers_for_the_tail() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    for (state, expected) in IN_CUSTODY {
        let repository = edited(
            &format!("custody-{}", state.named().replace([' ', ','], "-")),
            &arrangement.moved(state),
        );

        assert_eq!(
            repository
                .read_custody()
                .expect("readable")
                .map(|held| held.len()),
            Some(ENTRIES),
            "{} is measured against a record that claims what it wrote",
            state.named()
        );

        assert_eq!(
            (state.named(), weighing(&repository, arrangement.account)),
            (state.named(), expected),
            "{}",
            state.named()
        );
    }
}

/// And the refusal names the entry, on both sides.
///
/// A count would have said *the journal is the wrong length*, which sends a reader back to the bytes.
/// The two are kept apart because they send a reader to opposite places: one to a journal that grew,
/// one to a journal that was cut.
#[test]
fn the_refusal_names_the_entry_that_was_lost_or_gained() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    let held = custody::addresses(&arrangement.files.journal).expect("the journal admits");

    let lost = edited("names-lost", &arrangement.moved(Moved::TheWholeTail));
    let refusal = reading::reconstruct(&lost, arrangement.account, &custody::asked_at())
        .expect_err("the record answers for its tail");

    assert!(
        matches!(
            refusal,
            ReadingError::HeldKnowledgeAbsent { ref entry } if held[PREFIX..].contains(entry)
        ),
        "the entry named is one the tail held: {refusal}"
    );

    let gained = edited(
        "names-gained",
        &arrangement.moved(Moved::OneMoreNothingReaches),
    );
    let refusal = reading::reconstruct(&gained, arrangement.account, &custody::asked_at())
        .expect_err("the record answers for what it did not write");

    let arrived = custody::addresses(&arrangement.moved(Moved::OneMoreNothingReaches))
        .expect("the journal admits");

    assert!(
        matches!(
            refusal,
            ReadingError::UnheldKnowledge { ref entry } if entry == arrived.last().expect("one more")
        ),
        "the entry named is the one that arrived: {refusal}"
    );
}

/// A repository that makes no such claim is read exactly as it was, which four committed ones need.
///
/// Not a courtesy. `lab/agents/04-multiagent/run-*/repo` hold three files each and were written by
/// parties nobody can re-run, so a required fourth file would strand them — which is the item
/// experiment 14 queued, met here for the first time by a change that could have caused it.
#[test]
fn a_record_that_claims_nothing_about_itself_is_read_as_it_was() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    let repository = unclaimed("no-claim", &arrangement.moved(Moved::TheWholeTail));

    assert_eq!(repository.read_custody().expect("readable"), None);
    assert!(
        reading::reconstruct(&repository, arrangement.account, &custody::asked_at()).is_ok(),
        "absent is not empty, and a record that says nothing is not a record that says it holds nothing"
    );

    let claiming_nothing = unclaimed("claims-empty", &arrangement.files.journal);
    claiming_nothing
        .write_custody(&[])
        .expect("one file, written");

    assert!(
        matches!(
            reading::reconstruct(&claiming_nothing, arrangement.account, &custody::asked_at()),
            Err(ReadingError::UnheldKnowledge { .. })
        ),
        "and one that says it holds nothing is refused by the first entry the journal offers"
    );
}

/// And a generation one writer prepared, with another's journal put over it, is refused by it.
///
/// The state experiment 08 enumerates, reached the way 08 reaches it: prepare, then write one file
/// into the prepared generation, then read the generation where it is. 08 measured that the mixtures
/// which reconstruct silently are exactly the ones whose two journals stand in extension, and
/// reported the finer grain as a door the atomicity repair left open. The claim narrows it — the
/// prepared generation carries a claim about the journal the second write replaced.
///
/// It is measured here rather than left as a sentence in 08's suite, because 08's arrangement is
/// pinned to the shape it met and a claim about a change belongs to the change.
#[test]
fn a_generation_prepared_by_one_write_refuses_another_writes_journal() {
    let arrangement = custody::arranged().expect("the subject is admissible");

    let repository = scratch("mixture");
    let staged = repository
        .prepare(ape_cli::repository::RepositoryInput {
            journal: &arrangement.files.journal,
            lineage: &arrangement.files.lineage,
            worlds: &arrangement.files.worlds,
            designations: &[],
        })
        .expect("preparable");

    let into = ape_cli::repository::Repository::open(staged.generation());
    into.write_journal(&arrangement.moved(Moved::TheWholeTail))
        .expect("one file, over another writer's");

    assert!(
        matches!(
            reading::corroborated(&into),
            Err(ReadingError::HeldKnowledgeAbsent { .. })
        ),
        "the prepared claim is about the journal the second write replaced"
    );

    assert!(
        matches!(
            staged.turn(),
            Err(ape_cli::error::RepositoryError::Contended { .. })
        ),
        "and the turn still refuses to publish it, which is experiment 08's repair and not this one"
    );
}

/// For the four repositories nobody can re-run, measured rather than reasoned about.
#[test]
fn the_committed_repositories_are_read_unchanged() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../agents/04-multiagent")
        .canonicalize()
        .expect("the agents row is there");

    let mut found = 0;

    for entry in std::fs::read_dir(&root).expect("readable") {
        let repository = entry.expect("readable").path().join("repo");

        if !repository.join("journal.json").exists() {
            continue;
        }

        found += 1;

        let opened = ape_cli::repository::Repository::open(&repository);

        assert_eq!(
            opened.read_custody().expect("readable"),
            None,
            "{} was written before the claim existed",
            repository.display()
        );
        assert!(
            reading::corroborated(&opened).is_ok(),
            "{} still rebuilds",
            repository.display()
        );
    }

    assert_eq!(found, 4, "the four repositories nobody can re-run");
}
