//! The coordination experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, for the reason the reconstruction experiment
//! gives: a comparison has to fail loudly. What is different here is that the first phase
//! **expects a defect**, and says so in advance — a phase that discovers what it went looking for
//! is not making a finding, so what is measured is how the defect presents rather than that it is
//! there.

use std::collections::BTreeSet;

use ape_cli::error::{JournalError, LineageError, ReadingError};
use ape_cli::lineage::Decision;
use ape_cli::reading;
use ape_cli::repository::Repository;
use ape_cli::subject::coordination::{self, Founded};

/// A repository path no other process shares.
fn scratch(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir()
        .join(format!("ape-coordination-{}", std::process::id()))
        .join(name);
    let _ = std::fs::remove_dir_all(&path);
    path
}

/// A founded repository, and what the arrangement refers to.
fn founded(name: &str) -> (Repository, Founded) {
    let repository = Repository::open(scratch(name));
    let founded = coordination::founded().expect("the arrangement holds");

    coordination::found(&repository, &founded).expect("writable");

    (repository, founded)
}

/// The entries the founded journal holds, before any party admits anything.
const FOUNDED: usize = 15;

/// Phase 1 — Lose a decision.
///
/// Two parties read one repository, each decides, and each writes. The loss is predicted; what is
/// not predicted is how it presents, and that is what this measures.
#[test]
fn phase_1_lose_a_decision() {
    let (repository, arrangement) = founded("phase-1");
    let subject = &arrangement.subject;

    assert_eq!(
        repository.read_lineage().expect("readable").len(),
        1,
        "one world, decided before either party looked"
    );
    assert_eq!(repository.read_journal().expect("readable").len(), FOUNDED);

    // Both parties read, before either has written. Nothing about this is simultaneous.
    let mut one = coordination::read(&repository).expect("the founded repository reconstructs");
    let mut other = coordination::read(&repository).expect("and reconstructs again");

    let shared = arrangement.shared().id();

    let staffing = coordination::decide(&mut one, coordination::also(shared, subject.hiring))
        .expect("one party plans");
    let equipping = coordination::decide(&mut other, coordination::also(shared, subject.equipment))
        .expect("and so does the other");

    assert_ne!(staffing, equipping, "two worlds, and two decisions");

    coordination::write(&repository, &one).expect("writable");
    coordination::write(&repository, &other).expect("writable");

    // The loss, on a repository. Three decisions were taken and two are there.
    let recorded = repository.read_lineage().expect("readable");

    assert_eq!(recorded.len(), 2, "three decisions were taken");

    let Decision::Fork { introduced, .. } = &recorded[1].decision else {
        panic!("the surviving decision is a fork, found {:?}", recorded[1]);
    };
    assert_eq!(
        introduced,
        &BTreeSet::from([subject.equipment]),
        "and the one that survived is the last one written"
    );

    // Which is not the whole of it. The repository that lost a decision is **valid**: it
    // reconstructs, it corroborates, and nothing in it is a reference to what is gone.
    let rebuilt = reading::corroborated(&repository).expect("the repository reconstructs");

    assert_eq!(rebuilt.lineage.decided().len(), 2);
    assert!(
        !repository
            .read_worlds()
            .expect("readable")
            .iter()
            .any(|world| world.thesis == staffing.to_string()),
        "the lost world is nowhere, and nothing says it ever was"
    );
}

/// The journal loses knowledge by the same mechanism, and this measures that rather than assuming
/// it: two parties admit different knowledge, both write whole, and one admission is gone.
#[test]
fn the_journal_loses_by_the_same_mechanism() {
    let (repository, _) = founded("journal");
    let subject = coordination::founded()
        .expect("the arrangement holds")
        .subject;

    let mut one = coordination::read(&repository).expect("reconstructs");
    let mut other = coordination::read(&repository).expect("reconstructs");

    coordination::admit(&mut one, subject.grant).expect("one party learns something");
    coordination::admit(&mut other, subject.rebate).expect("the other learns something else");

    assert_eq!(one.journal.len(), FOUNDED + 1);
    assert_eq!(other.journal.len(), FOUNDED + 1);

    coordination::write(&repository, &one).expect("writable");
    coordination::write(&repository, &other).expect("writable");

    assert_eq!(
        repository.read_journal().expect("readable").len(),
        FOUNDED + 1,
        "two admissions were made and one is there"
    );
}

/// The interleaving that leaves evidence, and it is the same loss.
///
/// A party's write is not one write. It is three files, and a party that lands between another's
/// makes the repository disagree with itself — which the corroboration experiment's discipline
/// refuses. The decision is lost either way. Only this way says so.
#[test]
fn a_write_that_lands_between_another_is_refused() {
    let (repository, arrangement) = founded("interleaved");
    let subject = &arrangement.subject;

    let mut one = coordination::read(&repository).expect("reconstructs");
    let mut other = coordination::read(&repository).expect("reconstructs");

    let shared = arrangement.shared().id();

    coordination::decide(&mut one, coordination::also(shared, subject.hiring)).expect("plans");
    coordination::decide(&mut other, coordination::also(shared, subject.equipment))
        .expect("plans too");

    // Interleaved *within* a party's write rather than between two of them.
    repository.write_lineage(&one.decisions).expect("writable");
    repository
        .write_lineage(&other.decisions)
        .expect("writable");
    repository
        .write_worlds(&coordination::worlds(&other.lineage))
        .expect("writable");
    repository
        .write_worlds(&coordination::worlds(&one.lineage))
        .expect("writable");

    match reading::corroborated(&repository) {
        Err(ReadingError::WorldDisagrees {
            position,
            coordinate,
        }) => {
            assert_eq!(position, 1);
            assert_eq!(coordinate, "what it still proposes");
        }
        Err(other) => panic!("refused, and for another reason: {other:?}"),
        Ok(_) => panic!("expected a repository that disagrees with itself, and it read clean"),
    }
}

/// Why the loss is silent, and it is not that the record is thin.
///
/// A missing decision is found out when something **refers** to it, and exactly two things can:
/// a later decision that extends it, and the journal coordinate it was taken at. A leaf has
/// neither — and a second party's line is always a leaf.
#[test]
fn a_lost_decision_is_invisible_exactly_where_nothing_refers_to_it() {
    let (repository, arrangement) = founded("references");
    let subject = &arrangement.subject;

    let mut party = coordination::read(&repository).expect("reconstructs");

    let shared = arrangement.shared().id();
    let staffing = coordination::decide(&mut party, coordination::also(shared, subject.hiring))
        .expect("plans");
    coordination::decide(&mut party, coordination::also(staffing, subject.equipment))
        .expect("and goes on from there");

    coordination::write(&repository, &party).expect("writable");

    let decisions = repository.read_lineage().expect("readable");
    let worlds = repository.read_worlds().expect("readable");

    assert_eq!(decisions.len(), 3);

    // Drop the middle one — the world the third decision extends.
    let dropped = |at: usize| {
        let mut kept = decisions.clone();
        let mut witnessed = worlds.clone();
        kept.remove(at);
        witnessed.remove(at);
        repository.write_lineage(&kept).expect("writable");
        repository.write_worlds(&witnessed).expect("writable");
    };

    dropped(1);

    match reading::corroborated(&repository) {
        Err(ReadingError::Lineage(LineageError::ExtendsUnknownWorld { thesis })) => {
            assert_eq!(thesis, staffing, "the decision that extends it says which");
        }
        Err(other) => panic!("refused, and for another reason: {other:?}"),
        Ok(_) => panic!("expected a decision naming a world that is gone, and it read clean"),
    }

    // And drop the leaf instead. Nothing extends it, so nothing notices.
    dropped(2);

    let rebuilt = reading::corroborated(&repository).expect("a repository missing its leaf reads");

    assert_eq!(rebuilt.lineage.decided().len(), 2);
}

/// The other reference, and it runs one way: the lineage names the journal, and nothing names
/// the lineage.
///
/// So knowledge lost under a decision that stood is named, and that is the asymmetry the protocol
/// asked about. The two files lose by one mechanism and are found out by different ones.
#[test]
fn knowledge_lost_under_a_decision_that_stood_is_named() {
    let (repository, arrangement) = founded("witness");
    let subject = &arrangement.subject;

    let founding = repository.read_journal().expect("readable");

    let mut party = coordination::read(&repository).expect("reconstructs");

    coordination::admit(&mut party, subject.grant.clone()).expect("learns something");

    let grant = *party.admitted.commitments.last().expect("just admitted");

    coordination::decide(
        &mut party,
        coordination::also(arrangement.shared().id(), grant),
    )
    .expect("and decides on it");

    coordination::write(&repository, &party).expect("writable");

    // Another party, holding the journal as it was, writes it back.
    repository.write_journal(&founding).expect("writable");

    match reading::corroborated(&repository) {
        Err(ReadingError::Lineage(LineageError::Journal(JournalError::UnknownEntry(entry)))) => {
            assert_eq!(
                entry.to_string(),
                grant.to_string(),
                "the coordinate names the entry the journal no longer holds"
            );
        }
        Err(other) => panic!("refused, and for another reason: {other:?}"),
        Ok(_) => panic!("expected a coordinate naming knowledge that is gone, and it read clean"),
    }
}
