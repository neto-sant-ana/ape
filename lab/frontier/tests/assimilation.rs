//! Experiment 15 — Assimilation. Phases against `lab/frontier/docs/15-assimilation/00-protocol.md`.
//!
//! The question: can a record take another's material as its own — learning what it did not know and
//! deciding what it did not decide — without either record claiming a past that was not its? And is
//! what that produces a candidate, or an instruction to become the other record?
//!
//! *Take* is settled before any phase runs: a record takes another's material when what it ends up
//! holding is admissible on its own terms — every entry at an instant it can honestly claim, every
//! decision witnessed by a prefix that actually stood here — and nothing in it asserts anything about
//! the other record's past. Not `copy`, not `merge`, not `agree`.

use std::collections::BTreeSet;

use ape_cli::error::ConvergeError;
use ape_cli::journal::{Admission, EntryId};
use ape_cli::{converge, reading};

use ape_frontier::subject::assimilation::{
    self, ALONE_ENTRIES, BASE_ENTRIES, CROSSES, DIVERGES_AT, FAMILIES, HERE_DECIDES, HERE_ENTRIES,
    LACKING, LACKING_FROM_EVERYTHING, LACKING_FROM_NOTHING, RETAKEN, SHARED_AFTER, TAKEN_ON,
    THERE_DECIDES, THERE_ENTRIES, UNBRANCHED_ENTRIES,
};

/// A repository path no other process shares, emptied before it is used.
fn scratch(named: &str) -> ape_cli::repository::Repository {
    let path = std::env::temp_dir()
        .join(format!("ape-assimilation-{}", std::process::id()))
        .join(named);
    let _ = std::fs::remove_dir_all(&path);

    ape_cli::repository::Repository::open(path)
}

/// Every address a journal produces, as the set two journals are compared by.
fn held(journal: &[Admission]) -> BTreeSet<EntryId> {
    assimilation::addresses(journal)
        .expect("the journal admits")
        .into_iter()
        .collect()
}

/// What each world of a rebuilt record answers, settled then intended.
fn answers(
    files: &assimilation::Files,
    account: ape::kernel::entities::ResourceInstanceId,
) -> Vec<(i128, i128)> {
    let rebuilt = assimilation::rebuilt(files).expect("the record rebuilds");

    rebuilt
        .lineage
        .decided()
        .iter()
        .map(|world| {
            assimilation::answered(rebuilt.canon.history(), world, account)
                .expect("the world reads")
        })
        .collect()
}

// ---------------------------------------------------------------------------------------------
// Phase 0 — What two records can do today
// ---------------------------------------------------------------------------------------------

/// The arrangement holds what it says it holds, and every literal is read once.
#[test]
fn the_arrangement_is_what_the_subject_says_it_is() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    for (label, journal, entries) in [
        ("here", &arrangement.here.journal, HERE_ENTRIES),
        ("there", &arrangement.there.journal, THERE_ENTRIES),
        (
            "holding everything",
            &arrangement.holding_everything.journal,
            THERE_ENTRIES,
        ),
        (
            "holding nothing",
            &arrangement.holding_nothing.journal,
            ALONE_ENTRIES,
        ),
        (
            "unbranched",
            &arrangement.unbranched.journal,
            UNBRANCHED_ENTRIES,
        ),
    ] {
        assert_eq!(journal.len(), entries, "{label}");
    }

    assert_eq!(arrangement.shared.len(), BASE_ENTRIES, "the base");
}

/// The two records share the base by content, and two more entries after they diverged.
///
/// Derived by intersecting addresses rather than arranged, which is what makes the overlap the
/// collision experiment's twinning happening after the split instead of a list somebody wrote.
#[test]
fn the_two_records_share_the_base_and_two_entries_after_it() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    let common: BTreeSet<_> = held(&arrangement.here.journal)
        .intersection(&held(&arrangement.there.journal))
        .cloned()
        .collect();

    assert_eq!(
        common.len(),
        BASE_ENTRIES + SHARED_AFTER,
        "the base, and what both admitted after it"
    );
    assert!(
        arrangement
            .shared
            .iter()
            .all(|entry| common.contains(entry)),
        "and the base is inside it"
    );
}

/// Each record answers what the arrangement says, and no two worlds answer alike.
///
/// The falsifiability condition of everything after: a retaken decision that answered what somebody
/// already answered would let a later phase report *the intention crossed* about a world that was
/// already there.
#[test]
fn each_record_answers_what_the_arrangement_names() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    assert_eq!(
        answers(&arrangement.here, arrangement.account),
        vec![HERE_DECIDES],
        "here decides once"
    );
    assert_eq!(
        answers(&arrangement.there, arrangement.account),
        THERE_DECIDES.to_vec(),
        "there decides twice: a genesis, then a fork that introduces"
    );

    let decided: BTreeSet<_> = [HERE_DECIDES].into_iter().chain(THERE_DECIDES).collect();

    assert_eq!(decided.len(), 3, "and no two of them answer alike");
}

/// What the only operation between two records does today, and what it does not.
///
/// Read before anything is built, so that what a report adds is weighed against something measured.
#[test]
fn what_two_records_can_do_today() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    let here = scratch("today-here");
    let there = scratch("today-there");

    assimilation::write_whole(&here, &arrangement.here).expect("a whole write");
    assimilation::write_whole(&there, &arrangement.there).expect("a whole write");

    let theirs = reading::corroborated(&there).expect("the other record reads on its own");

    // The merge refuses whole, leaving this record exactly as it was — and it refuses **before** the
    // overlap is exhausted. The two records have `SHARED_AFTER` entries in common past the base and
    // did not admit them adjacently, so a comparison by position stops at the first disagreement and
    // never reaches the second thing they share.
    assert!(
        matches!(
            converge::converge(&here, &theirs),
            Err(ConvergeError::Diverged { position, .. }) if position == DIVERGES_AT
        ),
        "converge refuses at the first position the two sequences differ"
    );
    assert_eq!(
        answers(&arrangement.here, arrangement.account),
        vec![HERE_DECIDES],
        "and this record is untouched"
    );

    // And the one thing a second repository is answerable to says no: the two decided no world in
    // common, which is 09's boundary read from the side where it is empty.
    let rebuilt = assimilation::rebuilt(&arrangement.there).expect("the other record rebuilds");

    for world in rebuilt.lineage.decided() {
        assert!(
            !converge::holds(&here, world.id()).expect("the question is answerable"),
            "no world of theirs is one of ours"
        );
    }
}

// ---------------------------------------------------------------------------------------------
// Phase 1 — What one record can learn, and at whose instant
// ---------------------------------------------------------------------------------------------

/// What one record lacks of another is derived, and it covers every family.
///
/// Ten entries and nine families, because the record being shown committed twice. The families are
/// counted from the entries rather than asserted beside them, so an arrangement that lost a family
/// says which one.
#[test]
fn what_one_record_lacks_of_the_other_covers_every_family() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    let lacking = assimilation::lacking(&arrangement.here.journal, &arrangement.there.journal)
        .expect("both journals admit");

    assert_eq!(lacking.len(), LACKING, "what here does not hold");

    let families: BTreeSet<_> = lacking
        .iter()
        .map(|held| assimilation::family(&held.entry))
        .collect();
    assert_eq!(families.len(), FAMILIES, "one of each: {families:?}");
}

/// And the two degenerate cases are producible, which is what lets the real one be named.
#[test]
fn both_degenerate_records_are_producible() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    for (label, journal, expected) in [
        (
            "holding everything",
            &arrangement.holding_everything.journal,
            LACKING_FROM_EVERYTHING,
        ),
        (
            "holding nothing",
            &arrangement.holding_nothing.journal,
            LACKING_FROM_NOTHING,
        ),
    ] {
        let lacking =
            assimilation::lacking(journal, &arrangement.there.journal).expect("both admit");

        assert_eq!(lacking.len(), expected, "{label}");
    }
}

/// **S1.** The entries a record lacks admit into it at its own recording instant.
///
/// The whole of what *taking* means, measured: the content crosses and the instant does not. What is
/// read back is the instant this record wrote, not the one it was shown — an entry admitted with the
/// other's instant would be a copy, and this record would be claiming a past it did not have.
#[test]
fn the_entries_a_record_lacks_admit_at_its_own_instant() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    let lacking = assimilation::lacking(&arrangement.here.journal, &arrangement.there.journal)
        .expect("both journals admit");
    let learned = assimilation::taken(&lacking, TAKEN_ON);

    let mut journal = arrangement.here.journal.clone();
    journal.extend(learned.iter().cloned());

    let addresses = assimilation::addresses(&journal).expect("the extended journal admits");

    assert_eq!(
        addresses.len(),
        HERE_ENTRIES + LACKING,
        "every one of them admitted"
    );
    assert!(
        journal[HERE_ENTRIES..]
            .iter()
            .all(|entry| entry.recorded_at() == assimilation::day(TAKEN_ON)),
        "and every one at this record's own instant"
    );

    let shown: BTreeSet<_> = lacking
        .iter()
        .map(|held| held.entry.recorded_at().to_owned())
        .collect();

    assert!(
        !shown.contains(&assimilation::day(TAKEN_ON)),
        "which is not an instant the other record ever wrote"
    );
}

/// Copying the other record's instants is not merely dishonest here — it is inadmissible.
///
/// Found by mutating the instrument, which is the only reason it is known: taking with the instants
/// they were shown with is refused by the Canon itself, `RecordedOutOfOrder`, because recording is
/// monotonic and this record's watermark is already past their days.
///
/// **And the guarantee is one-sided**, which is the half that must not be overclaimed. What the Canon
/// forbids is back-dating, not copying: an instant at or after the receiver's watermark is admissible
/// whoever wrote it. So a record that is **behind** the one it is shown could copy, and nothing in the
/// engine would stop it. Both directions are measured.
#[test]
fn copying_their_instants_is_refused_only_where_this_record_is_ahead() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    let lacking = assimilation::lacking(&arrangement.here.journal, &arrangement.there.journal)
        .expect("both journals admit");

    let extended = |entries: Vec<Admission>| {
        let mut journal = arrangement.here.journal.clone();
        journal.extend(entries);
        assimilation::addresses(&journal)
    };

    let copied: Vec<Admission> = lacking.iter().map(|held| held.entry.clone()).collect();

    assert!(
        matches!(
            extended(copied),
            Err(ape_cli::error::JournalError::Canon(
                ape::canon::CanonError::RecordedOutOfOrder { .. }
            ))
        ),
        "their instants precede what this record has already recorded through"
    );

    // The other side of it: their last day is this record's own last day, and an instant that does
    // not go backwards is admissible however it was arrived at. Nothing here is checking honesty.
    let level = assimilation::taken(&lacking, assimilation::TAIL_SETTLED_ON);

    assert!(
        extended(level).is_ok(),
        "an instant at this record's watermark admits, whoever wrote it"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 2 — What one record can decide with another's intention
// ---------------------------------------------------------------------------------------------

/// The instants a retaken decision is asked to name: this record's own, and the original's.
fn retake(known_at: u8) -> assimilation::Retake {
    assimilation::Retake {
        learned_at: TAKEN_ON,
        known_at,
    }
}

/// **S2.** A foreign intention becomes a decision this record takes, and the witness it writes is
/// true.
///
/// Measured on the **written record**: the whole thing is put on disk and read back through the
/// application's own reader, which is what checks a witness against the prefix that actually stood. A
/// phase that read its own construction would be checking the construction.
#[test]
fn a_foreign_intention_becomes_a_decision_this_record_takes() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    let retaken = assimilation::retaking(&arrangement.here, &arrangement.there, retake(TAKEN_ON))
        .expect("the intention is retakeable");

    assert_eq!(retaken.learn.len(), LACKING, "what it had to learn");
    assert_eq!(
        retaken.decide.len(),
        arrangement.there.lineage.len(),
        "and one decision for each of theirs"
    );

    let files = assimilation::having_taken(&arrangement.here, &retaken)
        .expect("the receiving record rebuilds");
    let repository = scratch("retaken");

    assimilation::write_whole(&repository, &files).expect("a whole write");

    let readings =
        reading::reconstruct(&repository, arrangement.account, &assimilation::asked_at())
            .expect("the record reads, which is the witness being checked against what stood");

    assert_eq!(readings.len(), 1 + RETAKEN.len(), "its own, then theirs");
    assert_eq!(
        readings[1..]
            .iter()
            .map(|reading| reading.level)
            .collect::<Vec<_>>(),
        RETAKEN
            .iter()
            .map(|(settled, _)| *settled)
            .collect::<Vec<_>>(),
        "and the retaken worlds answer what the arrangement names"
    );
}

/// And the witness it writes is this record's prefix, entry for entry.
///
/// The severe condition, stated as the thing it would take to fail: a candidate that needed a witness
/// naming what stood **there** would be experiment 12 with a report in front of it.
#[test]
fn the_witness_it_writes_is_its_own_prefix() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    let retaken = assimilation::retaking(&arrangement.here, &arrangement.there, retake(TAKEN_ON))
        .expect("the intention is retakeable");
    let files = assimilation::having_taken(&arrangement.here, &retaken)
        .expect("the receiving record rebuilds");

    let mine = held(&files.journal);
    let theirs = held(&arrangement.there.journal);

    for taken in &retaken.decide {
        assert!(
            taken.witness.iter().all(|entry| mine.contains(entry)),
            "every entry the witness names is one this record admitted"
        );
        assert_eq!(
            taken.witness.len(),
            files.journal.len(),
            "and it names all of them, which is the prefix that stood when it was taken"
        );
        assert!(
            taken.witness.iter().any(|entry| !theirs.contains(entry)),
            "so it is not the other record's prefix wearing this one's name"
        );
    }
}

/// A retaken genesis cannot keep the instant the original named.
///
/// Not a prediction — derived while the subject was being built, before anything ran, and measured
/// here. A record learns a foreign fact when it is shown it, so the fact is not selectable at an
/// instant before that: `ensure_selectable` refuses a commitment recorded after the cut. So a genesis
/// crosses only by being re-dated, and what it recognizes is not what the original recognized.
#[test]
fn a_retaken_genesis_cannot_keep_the_instant_the_original_named() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    let refusal = assimilation::retaking(
        &arrangement.here,
        &arrangement.there,
        retake(assimilation::DECIDED_AT),
    );

    assert!(
        matches!(
            refusal,
            Err(ape_cli::error::SubjectError::Lineage(
                ape_cli::error::LineageError::Thesis(
                    ape::engine::thesis::ThesisError::CommitmentNotKnownAtCut { .. }
                )
            ))
        ),
        "the commitment it names was recorded after the instant it names"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 3 — What crosses by identity
// ---------------------------------------------------------------------------------------------

/// **S4.** Eight of the nine families keep the address they had; the Event does not.
///
/// Closed over all nine, and the eight that are predicted not to move are what makes the table say
/// anything: a table whose every row moved could not have told a chain from a set.
#[test]
fn eight_families_cross_by_identity_and_the_event_does_not() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    let measured = crossing(&arrangement.here);

    assert_eq!(
        measured,
        CROSSES.to_vec(),
        "into a record whose chain left the base's"
    );
}

/// And into a record whose chain never left the base's, the Event crosses too.
///
/// Which is what says the Event row is about the **chain** and not about Events: the same entry, the
/// same receiver in every other respect, and the address survives.
#[test]
fn into_an_unbranched_record_the_event_crosses_too() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    let measured = crossing(&arrangement.unbranched);
    let expected: Vec<_> = CROSSES
        .into_iter()
        .map(|(family, _)| (family, true))
        .collect();

    assert_eq!(measured, expected, "every family, including the Event");
}

// ---------------------------------------------------------------------------------------------
// Phase 4 — Candidate or instruction
// ---------------------------------------------------------------------------------------------

/// What the report says to each of the three receivers: how much to learn, how much to decide.
///
/// The two degenerate rows are here on purpose. A report whose answer is always the first or always
/// the last has said *take nothing* or *take everything*, and the middle row is only a candidate
/// because the other two exist to tell it from.
const REPORTS: [(&str, usize, usize); 3] = [
    ("holding everything", LACKING_FROM_EVERYTHING, 2),
    ("here", LACKING, 2),
    ("holding nothing", LACKING_FROM_NOTHING, 2),
];

#[test]
fn what_the_report_says_to_each_of_the_three_receivers() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    for (label, learn, decide) in REPORTS {
        let into = match label {
            "holding everything" => &arrangement.holding_everything,
            "here" => &arrangement.here,
            "holding nothing" => &arrangement.holding_nothing,
            other => panic!("the table names a receiver the phase does not build: {other}"),
        };

        let retaken = assimilation::retaking(into, &arrangement.there, retake(TAKEN_ON))
            .expect("the intention is retakeable");

        assert_eq!(
            (label, retaken.learn.len(), retaken.decide.len()),
            (label, learn, decide),
            "{label}"
        );
    }
}

/// **S3 is half confirmed and half refuted.** What to learn is a candidate; what to decide is not.
///
/// The first column of the table above is neither empty nor everything for the receiver that genuinely
/// diverged, and is both for the two that did not — so *learn* discriminates. The second column is the
/// same number in every row, and it is the whole of the other record's lineage: **the report always
/// says take all of their decisions.**
///
/// The reason is not an accident of the arrangement, and the phase below is what says so.
#[test]
fn what_to_learn_discriminates_and_what_to_decide_does_not() {
    let learning: BTreeSet<_> = REPORTS.iter().map(|(_, learn, _)| *learn).collect();
    let deciding: BTreeSet<_> = REPORTS.iter().map(|(_, _, decide)| *decide).collect();

    assert_eq!(
        learning.len(),
        REPORTS.len(),
        "three receivers, three answers"
    );
    assert_eq!(deciding.len(), 1, "and one answer about deciding");
}

/// A **retaken** world is never a world the other record decided, so the report can never say
/// *already held*.
///
/// Which is why the second column cannot discriminate. Synthesis has an `AlreadyApplied` status
/// because its Source and Target sit in one canonical history and can arrive at one world. Retaking
/// cannot: a world's identity is derived from its cut, a cut is resolved from an instant, and the
/// instant a receiver may claim is never the instant the original claimed. **Every decision is always
/// left to take, because none of them can already have been taken.**
#[test]
fn a_retaken_world_is_never_a_world_the_other_record_decided() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");
    let theirs = worlds(&arrangement.there);

    for (label, ..) in REPORTS {
        let into = match label {
            "holding everything" => &arrangement.holding_everything,
            "here" => &arrangement.here,
            _ => &arrangement.holding_nothing,
        };

        let retaken = assimilation::retaking(into, &arrangement.there, retake(TAKEN_ON))
            .expect("the intention is retakeable");
        let taken = retaken.decide.len();
        let files =
            assimilation::having_taken(into, &retaken).expect("the receiving record rebuilds");

        let ours = decided(&files);
        let crossed: BTreeSet<_> = ours[ours.len() - taken..].iter().copied().collect();

        assert!(
            crossed.is_disjoint(&theirs),
            "{label}: not one retaken world is one of theirs"
        );
    }
}

/// And a record that already held the knowledge arrives at their world **without retaking anything**.
///
/// Found by the phase above failing where it was expected to pass, and it is the more useful half.
/// `holding everything` admitted the same content and decided at the same instant, so its own genesis
/// produces the other record's first world by identity — with no operation between them, which is
/// experiment 09.
///
/// So *already held* is reachable, and never by the road this experiment is about: it is a coincidence
/// of two decisions, not an outcome of one crossing.
#[test]
fn a_record_that_already_held_the_knowledge_arrives_there_without_retaking() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    let theirs = worlds(&arrangement.there);
    let already = worlds(&arrangement.holding_everything);
    let ours = worlds(&arrangement.here);

    assert!(
        !already.is_disjoint(&theirs),
        "the record holding the same knowledge decided one of their worlds on its own"
    );
    assert!(
        ours.is_disjoint(&theirs),
        "and the record that diverged decided none of them"
    );
}

/// Every world a record's lineage decided, by identity, in the order it decided them.
///
/// Ordered rather than a set, because one phase needs the **last** few — the ones a retaking added —
/// and a set has no last.
fn decided(files: &assimilation::Files) -> Vec<ape::engine::thesis::ThesisId> {
    assimilation::rebuilt(files)
        .expect("the record rebuilds")
        .lineage
        .decided()
        .iter()
        .map(|world| world.id())
        .collect()
}

/// The same, as the set two lineages are compared by.
fn worlds(files: &assimilation::Files) -> BTreeSet<ape::engine::thesis::ThesisId> {
    decided(files).into_iter().collect()
}

// ---------------------------------------------------------------------------------------------
// Phase 5 — What does not survive
// ---------------------------------------------------------------------------------------------

/// Every field a decision holds, and whether the original's survives being retaken.
///
/// Closed over what a `Taken` is, so *the provenance is lost* is a statement about the whole record
/// rather than about the field somebody thought of.
const SURVIVES: [(&str, bool); 4] = [
    ("the intention", true),
    ("the coordinate", false),
    ("the witness", false),
    ("the decider", false),
];

#[test]
fn what_survives_being_retaken_and_what_does_not() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    let retaken = assimilation::retaking(&arrangement.here, &arrangement.there, retake(TAKEN_ON))
        .expect("the intention is retakeable");

    for (theirs, ours) in arrangement.there.lineage.iter().zip(&retaken.decide) {
        let measured = [
            (
                "the intention",
                intended(&theirs.decision) == intended(&ours.decision),
            ),
            ("the coordinate", theirs.after == ours.after),
            ("the witness", theirs.witness == ours.witness),
            ("the decider", theirs.by == ours.by),
        ];

        assert_eq!(measured.to_vec(), SURVIVES.to_vec());
    }
}

/// What a decision is about, apart from where and when it was taken.
fn intended(decision: &ape_cli::lineage::Decision) -> Vec<String> {
    use ape_cli::lineage::Decision;

    match decision {
        Decision::Genesis { selection, .. } => selection.iter().map(|id| id.to_string()).collect(),
        Decision::Advance { .. } => Vec::new(),
        Decision::Fork {
            omitted,
            introduced,
            ..
        } => omitted
            .iter()
            .chain(introduced)
            .map(|id| id.to_string())
            .collect(),
    }
}

/// **S5 refuted, and the reason is worse than the prediction.**
///
/// The prediction was that the receiving record cannot say the other is why. It can: an `AgentId` is
/// content-addressed, the receiver learns the agent along with everything else, and a decision
/// claiming it is accepted — `attributed` checks only that the agent was known at the coordinate, and
/// after learning it was.
///
/// What the record cannot do is make it **true**. `by` is the party that *took* the decision, and the
/// party that took this one is this record. So the one field that crosses does not mean what it would
/// have to mean, and a record that filled it in would be writing a claim of exactly the kind
/// experiment 05 measured nothing can check.
#[test]
fn the_decider_is_the_one_field_that_could_cross_and_it_would_be_false() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    let retaken = assimilation::retaking(&arrangement.here, &arrangement.there, retake(TAKEN_ON))
        .expect("the intention is retakeable");

    let claiming: Vec<_> = retaken
        .decide
        .iter()
        .map(|taken| ape_cli::lineage::Taken {
            by: Some(arrangement.claimed_there),
            ..taken.clone()
        })
        .collect();

    let files = assimilation::having_taken(
        &arrangement.here,
        &assimilation::Retaken {
            learn: retaken.learn.clone(),
            decide: claiming,
        },
    )
    .expect("the receiving record rebuilds");

    let repository = scratch("claiming-theirs");
    assimilation::write_whole(&repository, &files).expect("a whole write");

    assert!(
        reading::corroborated(&repository).is_ok(),
        "nothing refuses a decision claiming the other record's party"
    );
    assert!(
        retaken.decide.iter().all(|taken| taken.by.is_none()),
        "and what the candidate writes claims nobody, because the party that took it is this one"
    );
}

// ---------------------------------------------------------------------------------------------
// Phase 6 — The three guards
// ---------------------------------------------------------------------------------------------

/// Which guard answered, where one did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Weighed {
    Coordinate,
    Witness,
    Worlds,
    Nothing,
}

/// Every state this experiment produced, put to all three guards. Closed.
const GUARDS: [(&str, Weighed); 6] = [
    ("here as written", Weighed::Nothing),
    ("there as written", Weighed::Nothing),
    ("here having taken", Weighed::Nothing),
    ("holding everything, having taken", Weighed::Nothing),
    ("holding nothing, having taken", Weighed::Nothing),
    ("here having taken, claiming their party", Weighed::Nothing),
];

#[test]
fn every_state_is_put_to_the_three_guards() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    let taking = |into: &assimilation::Files, claiming: bool| -> assimilation::Files {
        let retaken = assimilation::retaking(into, &arrangement.there, retake(TAKEN_ON))
            .expect("the intention is retakeable");

        let decide = match claiming {
            false => retaken.decide,
            true => retaken
                .decide
                .iter()
                .map(|taken| ape_cli::lineage::Taken {
                    by: Some(arrangement.claimed_there),
                    ..taken.clone()
                })
                .collect(),
        };

        assimilation::having_taken(
            into,
            &assimilation::Retaken {
                learn: retaken.learn,
                decide,
            },
        )
        .expect("the receiving record rebuilds")
    };

    for (state, expected) in GUARDS {
        let repository = scratch(&format!("guard-{}", state.replace([' ', ','], "-")));

        match state {
            "here as written" => assimilation::write_whole(&repository, &arrangement.here),
            "there as written" => assimilation::write_whole(&repository, &arrangement.there),
            "here having taken" => {
                assimilation::write_whole(&repository, &taking(&arrangement.here, false))
            }
            "holding everything, having taken" => assimilation::write_whole(
                &repository,
                &taking(&arrangement.holding_everything, false),
            ),
            "holding nothing, having taken" => {
                assimilation::write_whole(&repository, &taking(&arrangement.holding_nothing, false))
            }
            "here having taken, claiming their party" => {
                assimilation::write_whole(&repository, &taking(&arrangement.here, true))
            }
            other => panic!("the table names a state the phase does not build: {other}"),
        }
        .expect("a whole write");

        let weighed =
            match reading::reconstruct(&repository, arrangement.account, &assimilation::asked_at())
            {
                Ok(_) => Weighed::Nothing,
                Err(ape_cli::error::ReadingError::Journal(_)) => Weighed::Coordinate,
                Err(ape_cli::error::ReadingError::Lineage(
                    ape_cli::error::LineageError::Journal(_),
                )) => Weighed::Coordinate,
                Err(ape_cli::error::ReadingError::Lineage(_)) => Weighed::Witness,
                Err(_) => Weighed::Worlds,
            };

        assert_eq!((state, weighed), (state, expected), "{state}");
    }
}

// ---------------------------------------------------------------------------------------------
// Phase 7 — Weighing it against the condition
// ---------------------------------------------------------------------------------------------

/// The report is composable from what the application already offers, and this rebuilds it to say so.
///
/// The first half of the condition is *the report answers something no existing operation answers —
/// not more conveniently, at all*, and it has to be measured rather than argued. So this recomputes
/// the harder half of the report using **only** `ape_cli`, in a file that is a consumer of it, and
/// weighs the result against what the subject's instrument produced.
///
/// The crate boundary is what makes it proof rather than illustration: the laboratory depends on the
/// application and the application never depends back, so nothing reachable here is anything an
/// application could not reach.
#[test]
fn the_report_is_composable_from_what_the_application_already_offers() {
    use ape::canon::Canon;
    use ape_cli::history::ResidentHistory;
    use ape_cli::journal;

    let arrangement = assimilation::arranged().expect("the subject is admissible");

    let here = scratch("composable-here");
    let there = scratch("composable-there");

    assimilation::write_whole(&here, &arrangement.here).expect("a whole write");
    assimilation::write_whole(&there, &arrangement.there).expect("a whole write");

    // Everything below is the application's public surface and nothing else.
    let ours = reading::corroborated(&here).expect("this record reads");
    let theirs = reading::corroborated(&there).expect("the other record reads");

    let held: BTreeSet<_> = ours.admitted.entries.iter().cloned().collect();

    let mut aside = Canon::new(ResidentHistory::new());
    let addressed = journal::replay(&mut aside, &theirs.journal).expect("their journal admits");

    let composed: Vec<_> = theirs
        .journal
        .iter()
        .zip(&addressed.entries)
        .filter(|(_, address)| !held.contains(address))
        .map(|(entry, _)| entry.clone())
        .collect();

    let instrumented = assimilation::lacking(&arrangement.here.journal, &arrangement.there.journal)
        .expect("both journals admit");

    assert_eq!(
        composed.len(),
        instrumented.len(),
        "a caller reaches the same answer with what it already has"
    );
    assert_eq!(composed.len(), LACKING);
}

/// And the operation a caller would reach for now says how much was takeable.
///
/// Which is the one thing here that was a **defect** rather than an absence. `converge` is what an
/// application calls when it is handed another record, and it answered `Diverged` at a position and
/// nothing else — a caller reading that concludes the two are incompatible, and what was measured is
/// that ten of the other record's entries and both of its decisions could have been taken.
///
/// **The repair landed afterwards**, together with experiment 13's Request 1, because the queue held
/// the two as one item and said they were to be answered together or not at all. This phase asserted
/// the position and could not assert the number it was written to point at; it can now, and it is the
/// same `LACKING` the retake below reaches by a different route. The published result stands against
/// the commit it was taken at.
///
/// What the refusal says is the **knowledge** half only. This experiment measured that what is left to
/// decide is always the whole of the other lineage, so a count of decisions in a refusal would be a
/// constant, and the second assertion below is where that number belongs.
#[test]
fn the_operation_a_caller_reaches_for_says_how_much_was_takeable() {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    let here = scratch("takeable-here");
    let there = scratch("takeable-there");

    assimilation::write_whole(&here, &arrangement.here).expect("a whole write");
    assimilation::write_whole(&there, &arrangement.there).expect("a whole write");

    let theirs = reading::corroborated(&there).expect("the other record reads");

    assert!(
        matches!(
            converge::converge(&here, &theirs),
            Err(ConvergeError::Diverged { position, shared, .. })
                if position == DIVERGES_AT && shared == BASE_ENTRIES + SHARED_AFTER
        ),
        "the merge says where two sequences part, and how much of them is one body of knowledge"
    );

    let retaken = assimilation::retaking(&arrangement.here, &arrangement.there, retake(TAKEN_ON))
        .expect("and yet the material is takeable");

    assert_eq!((retaken.learn.len(), retaken.decide.len()), (LACKING, 2));
}

/// For each family, whether the address an entry had survives being admitted into this record.
fn crossing(into: &assimilation::Files) -> Vec<(&'static str, bool)> {
    let arrangement = assimilation::arranged().expect("the subject is admissible");

    let lacking = assimilation::lacking(&into.journal, &arrangement.there.journal)
        .expect("both journals admit");
    let learned = assimilation::taken(&lacking, TAKEN_ON);

    let mut journal = into.journal.clone();
    journal.extend(learned);

    let produced = assimilation::addresses(&journal).expect("the extended journal admits");
    let mut crossed: Vec<(&'static str, bool)> = Vec::new();

    for (position, held) in lacking.iter().enumerate() {
        let family = assimilation::family(&held.entry);
        let here = &produced[into.journal.len() + position];

        // A family that arrives twice is one row, and it may not disagree with itself — otherwise
        // the row would be reporting whichever of the two the loop saw last.
        match crossed.iter_mut().find(|(named, _)| *named == family) {
            Some((_, survived)) => assert_eq!(
                *survived,
                here == &held.there,
                "{family} arrives twice and crosses both times or neither"
            ),
            None => crossed.push((family, here == &held.there)),
        }
    }

    crossed
}
