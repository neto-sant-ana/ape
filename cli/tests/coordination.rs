//! The coordination experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, for the reason the reconstruction experiment
//! gives: a comparison has to fail loudly. What is different here is that the first phase
//! **expects a defect**, and says so in advance — a phase that discovers what it went looking for
//! is not making a finding, so what is measured is how the defect presents rather than that it is
//! there.

use std::collections::BTreeSet;

use ape_cli::converge;
use ape_cli::error::{ConvergeError, JournalError, LineageError, ReadingError};
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

/// Every byte of a repository, so that two of them can be compared whole.
///
/// Each file arrives under its own name, because a comparison that fails has to say which file
/// stopped agreeing. Three files concatenated report a difference by printing all of them.
fn bytes(repository: &Repository) -> Vec<(&'static str, String)> {
    [
        ("journal.json", repository.journal_path()),
        ("lineage.json", repository.lineage_path()),
        ("worlds.json", repository.worlds_path()),
    ]
    .into_iter()
    .map(|(name, path)| (name, std::fs::read_to_string(path).expect("readable")))
    .collect()
}

/// The name of the first file two repositories disagree about, and what disagrees within it.
fn differing(one: &[(&'static str, String)], other: &[(&'static str, String)]) -> Option<String> {
    one.iter()
        .zip(other)
        .find(|((_, here), (_, there))| here != there)
        .map(|((name, here), (_, there))| {
            let at = here
                .lines()
                .zip(there.lines())
                .position(|(here, there)| here != there);

            match at {
                Some(line) => format!("{name} differs at line {}", line + 1),
                None => format!("{name} differs in length"),
            }
        })
}

/// Phase 2 — Converge.
///
/// The same two parties, and the same interleaving. What changes is that putting a working copy
/// back is a merge that must reconstruct, rather than a write that replaces.
#[test]
fn phase_2_converge() {
    let (repository, arrangement) = founded("phase-2");
    let subject = &arrangement.subject;

    let mut one = coordination::read(&repository).expect("reconstructs");
    let mut other = coordination::read(&repository).expect("reconstructs");

    let shared = arrangement.shared().id();

    let staffing = coordination::decide(&mut one, coordination::also(shared, subject.hiring))
        .expect("one party plans");
    let equipping = coordination::decide(&mut other, coordination::also(shared, subject.equipment))
        .expect("and so does the other");

    converge::converge(&repository, &one).expect("the first party converges");

    assert_eq!(
        repository.read_lineage().expect("readable").len(),
        2,
        "one world was founded and one was decided"
    );

    converge::converge(&repository, &other).expect("and so does the second");

    // Three decisions were taken and three are there. Neither party lost the other.
    assert_eq!(repository.read_lineage().expect("readable").len(), 3);
    assert!(
        converge::holds(&repository, staffing).expect("reconstructs"),
        "the first party's world is there"
    );
    assert!(
        converge::holds(&repository, equipping).expect("reconstructs"),
        "and so is the second party's"
    );

    // And the repository still says what it produced, which is the check Phase 1 showed a lost
    // update passes trivially. Here it has something to check.
    let rebuilt = reading::corroborated(&repository).expect("the merged repository reconstructs");

    assert_eq!(rebuilt.lineage.decided().len(), 3);
    assert_eq!(
        repository.read_journal().expect("readable").len(),
        FOUNDED,
        "neither party learned anything, so the journal did not move"
    );
}

/// The property that matters more than the repair: **order must not survive into the result.**
///
/// A merge that appended in arrival order would remove the loss and leave the arrival order in the
/// repository, which is a lock in a merge's clothes. So the two parties converge in one order in
/// one repository and the other order in another, and the two repositories are compared whole.
#[test]
fn the_same_decisions_in_either_order_are_one_repository() {
    let run = |name: &str, reversed: bool| -> Vec<(&'static str, String)> {
        let (repository, arrangement) = founded(name);
        let subject = &arrangement.subject;
        let shared = arrangement.shared().id();

        let mut one = coordination::read(&repository).expect("reconstructs");
        let mut other = coordination::read(&repository).expect("reconstructs");

        coordination::decide(&mut one, coordination::also(shared, subject.hiring)).expect("plans");
        coordination::decide(&mut other, coordination::also(shared, subject.equipment))
            .expect("plans");

        let order = if reversed {
            [&other, &one]
        } else {
            [&one, &other]
        };

        for party in order {
            converge::converge(&repository, party).expect("converges");
        }

        bytes(&repository)
    };

    let forwards = run("order-forwards", false);
    let backwards = run("order-backwards", true);

    assert_eq!(
        differing(&forwards, &backwards),
        None,
        "the same two decisions, and one repository"
    );

    // And the repository is not merely identical to itself: it holds three decisions in an order
    // neither party chose, which is what a linearization of something that is not a line looks
    // like from outside.
    assert_eq!(
        serde_json::from_str::<Vec<serde_json::Value>>(&forwards[1].1)
            .expect("the lineage parses")
            .len(),
        3
    );
}

/// Two parties on different branches never conflicted, and this is what that means.
///
/// Nothing is arbitrated, nothing is rewritten, and no party's decision is altered by the other's
/// arriving. What contended in Phase 1 was the file, and never the content.
#[test]
fn two_branches_are_not_a_conflict() {
    let (repository, arrangement) = founded("branches");
    let subject = &arrangement.subject;
    let shared = arrangement.shared().id();

    let mut one = coordination::read(&repository).expect("reconstructs");
    let mut other = coordination::read(&repository).expect("reconstructs");

    coordination::decide(&mut one, coordination::also(shared, subject.hiring)).expect("plans");
    coordination::decide(&mut other, coordination::also(shared, subject.equipment)).expect("plans");

    converge::converge(&repository, &one).expect("converges");
    converge::converge(&repository, &other).expect("converges");

    let recorded = repository.read_lineage().expect("readable");

    // Each party's decision is in there exactly as that party took it.
    for held in [&one.decisions[1], &other.decisions[1]] {
        assert!(
            recorded.contains(held),
            "a party's decision survived unaltered"
        );
    }

    // Both worlds extend the shared ancestor, and neither extends the other. There was no version
    // of this in which one of them had to give way.
    let worlds = repository.read_worlds().expect("readable");
    let parents: BTreeSet<_> = worlds[1..]
        .iter()
        .map(|world| world.thesis_parent.clone())
        .collect();

    assert_eq!(
        parents,
        BTreeSet::from([Some(arrangement.shared().id().to_string())]),
        "two branches of one world"
    );

    // And the case that looks like a conflict is not one either: two parties deciding the *same*
    // thing produce one decision and one world, because identity is derived from content.
    let mut third = coordination::read(&repository).expect("reconstructs");
    coordination::decide(&mut third, coordination::also(shared, subject.hiring))
        .expect("the same plan, decided again by somebody else");

    converge::converge(&repository, &third).expect("converges");

    assert_eq!(
        repository.read_lineage().expect("readable").len(),
        3,
        "deciding what is already decided adds nothing"
    );
}

/// A party whose knowledge moved is refused by name, and leaves the repository as it was.
///
/// This is the other half of what the Canon promises a writer who lost, and here it is a whole
/// merge rather than one append: a refusal that had written part of it would be the tear Phase 1
/// measured, produced by the code meant to prevent it.
#[test]
fn a_party_whose_knowledge_moved_writes_nothing() {
    let (repository, _) = founded("diverged");
    let subject = coordination::founded()
        .expect("the arrangement holds")
        .subject;

    let mut one = coordination::read(&repository).expect("reconstructs");
    let mut other = coordination::read(&repository).expect("reconstructs");

    coordination::admit(&mut one, subject.grant.clone()).expect("one party learns something");
    coordination::admit(&mut other, subject.rebate.clone()).expect("the other, something else");

    converge::converge(&repository, &one).expect("the first party converges");

    assert_eq!(
        repository.read_journal().expect("readable").len(),
        FOUNDED + 1
    );

    let before = bytes(&repository);

    match converge::converge(&repository, &other) {
        Err(ConvergeError::Diverged { position, .. }) => {
            assert_eq!(
                position, FOUNDED,
                "at the entry the two parties disagree on"
            );
        }
        Err(other) => panic!("refused, and for another reason: {other:?}"),
        Ok(_) => panic!("expected a party writing on a journal that moved, and it converged"),
    }

    assert_eq!(
        differing(&before, &bytes(&repository)),
        None,
        "and it left nothing behind"
    );

    // The refusal is not the end of it. The party reads again and admits again — knowledge is not
    // revisable, so learning the same thing on top of more history is the same fact.
    let mut again = coordination::read(&repository).expect("reconstructs");
    coordination::admit(&mut again, subject.rebate).expect("admits again");

    converge::converge(&repository, &again).expect("and converges");

    assert_eq!(
        repository.read_journal().expect("readable").len(),
        FOUNDED + 2,
        "both parties' knowledge is there"
    );
}

/// Why knowledge appends rather than merging, measured rather than inherited.
///
/// A merge that ordered the journal by anything other than arrival would move entries a standing
/// decision was taken against — and the repository already refuses that, through the sequence
/// witness convergence declined to call redundant. So the append-only rule is not borrowed from
/// the Canon: it is what the record on disk requires.
#[test]
fn a_journal_reordered_makes_a_standing_decision_disagree() {
    let (repository, arrangement) = founded("reordered");
    let subject = coordination::founded()
        .expect("the arrangement holds")
        .subject;

    let mut one = coordination::read(&repository).expect("reconstructs");
    coordination::admit(&mut one, subject.grant).expect("learns something");

    let grant = *one.admitted.commitments.last().expect("just admitted");

    coordination::decide(
        &mut one,
        coordination::also(arrangement.shared().id(), grant),
    )
    .expect("and decides against it");

    converge::converge(&repository, &one).expect("converges");

    // What a merge sorting by `recorded_at` would have produced: the other party's knowledge, of
    // the same day, ahead of what this decision was taken against.
    let mut sorted = repository.read_journal().expect("readable");
    let learned = sorted.pop().expect("the grant");
    sorted.push(subject.rebate);
    sorted.push(learned);

    repository.write_journal(&sorted).expect("writable");

    match reading::corroborated(&repository) {
        Err(ReadingError::Lineage(LineageError::UnwitnessedKnowledge { .. })) => {}
        Err(other) => panic!("refused, and for another reason: {other:?}"),
        Ok(_) => panic!("expected a standing decision to disagree, and the repository read clean"),
    }
}
