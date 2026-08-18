//! The coordination experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, for the reason the reconstruction experiment
//! gives: a comparison has to fail loudly. What is different here is that the first phase
//! **expects a defect**, and says so in advance — a phase that discovers what it went looking for
//! is not making a finding, so what is measured is how the defect presents rather than that it is
//! there.

use std::collections::BTreeSet;

use ape::engine::synthesis::SynthesisError;
use ape::engine::thesis::{ThesisError, ThesisId, descends_from};
use ape::kernel::entities::AgentId;

use ape_cli::converge;
use ape_cli::error::{ConvergeError, JournalError, LineageError, ReadingError, TransferError};
use ape_cli::lineage::Decision;
use ape_cli::reading;
use ape_cli::repository::Repository;
use ape_cli::subject::coordination::{self, Founded};
use ape_cli::transfer::{self, StatusRecord};

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

/// Two parties, each with a line of its own in the repository.
///
/// The state Phase 2 ends in, which is where reaching each other can begin.
fn converged(name: &str) -> (Repository, Founded, ThesisId, ThesisId) {
    let (repository, arrangement) = founded(name);
    let subject = &arrangement.subject;
    let shared = arrangement.shared().id();

    let mut one = coordination::read(&repository).expect("reconstructs");
    let mut other = coordination::read(&repository).expect("reconstructs");

    let staffing = coordination::decide(&mut one, coordination::also(shared, subject.hiring))
        .expect("one party plans");
    let equipping = coordination::decide(&mut other, coordination::also(shared, subject.equipment))
        .expect("and so does the other");

    converge::converge(&repository, &one).expect("converges");
    converge::converge(&repository, &other).expect("converges");

    (repository, arrangement, staffing, equipping)
}

/// The instant every world is interpreted at, past every deadline the subject carries.
const EFFECTIVE: &str = "2026-01-28";

/// Phase 3 — Reach each other.
///
/// One party takes up an intention out of the other's line, through the machinery convergence built
/// and provenance left alone. What the phase is for is what a transfer between *parties* needs that
/// a transfer between lines did not.
#[test]
fn phase_3_reach_each_other() {
    let (repository, arrangement, staffing, equipping) = converged("phase-3");
    let subject = &arrangement.subject;
    let shared = arrangement.shared().id();

    // The question is asked of the repository, by a party that read it and named what it found.
    let report = transfer::reconstruct(&repository, shared, staffing, equipping)
        .expect("the repository answers");

    let StatusRecord::Applicable {
        transfer: asked, ..
    } = &report.status
    else {
        panic!("expected an applicable transfer, found {:?}", report.status);
    };

    assert!(asked.remove.is_empty(), "nothing is withdrawn");
    assert_eq!(
        asked.introduce,
        BTreeSet::from([subject.hiring.to_string()]),
        "the other party's plan, and only what the Target does not already hold"
    );

    // The report names three worlds and no party. Asserted as a closed set, because a field nothing
    // looks at is how an unnecessary one survives.
    let fields: BTreeSet<String> = serde_json::from_value::<serde_json::Map<_, _>>(
        serde_json::to_value(&report).expect("a report serializes"),
    )
    .expect("an object")
    .keys()
    .cloned()
    .collect();

    assert_eq!(
        fields,
        BTreeSet::from(
            [
                "base",
                "source",
                "target",
                "omitted",
                "introduced",
                "status"
            ]
            .map(str::to_owned)
        ),
        "a transfer between parties asks for nothing a transfer between lines did not"
    );

    // And the party takes it up, which is an ordinary fork of its own line.
    let mut adopter = coordination::read(&repository).expect("reconstructs");
    let adopted =
        coordination::adopt(&mut adopter, shared, staffing, equipping).expect("takes it up");

    converge::converge(&repository, &adopter).expect("converges");

    let readings = reading::reconstruct(
        &repository,
        subject.instance,
        &ape::kernel::value_objects::Date::parse(EFFECTIVE).expect("a real date"),
    )
    .expect("the repository reconstructs");

    assert_eq!(readings.len(), 4);

    let world = readings
        .iter()
        .find(|world| world.thesis == adopted.to_string())
        .expect("the adopted world is there");

    assert_eq!(
        world.open,
        BTreeSet::from([
            subject.budget.to_string(),
            subject.hiring.to_string(),
            subject.equipment.to_string(),
        ]),
        "both parties' plans, in one world"
    );
    assert_eq!(
        world.thesis_parent,
        Some(equipping.to_string()),
        "on the adopting party's branch, and not on the donor's"
    );
    // The account admits both plans together — 50 − 20 − 15 leaves 15, inside its bounds — and the
    // settled level is nothing, because no Event has settled anything. Feasibility is about what
    // the world proposes; the level is about what history has closed.
    assert!(
        world.conflicts.is_empty(),
        "both plans in one world, and the account admits it: {:?}",
        world.conflicts
    );
    assert_eq!(world.level, 0.0, "and nothing in this subject has settled");

    // The donor's line is untouched, which is the whole of what a party gave up by donating.
    assert!(
        converge::holds(&repository, staffing).expect("reconstructs"),
        "the world the intention came out of is still there"
    );
}

/// What a transfer between parties needs that a transfer between lines did not.
///
/// A convergence experiment's writer held both lines because it decided both. A party holds only
/// what it read, so naming another party's world is possible exactly when that party has converged.
#[test]
fn a_party_cannot_reach_a_line_that_has_not_arrived() {
    let (repository, arrangement) = founded("unarrived");
    let subject = &arrangement.subject;
    let shared = arrangement.shared().id();

    let mut donor = coordination::read(&repository).expect("reconstructs");
    let mut adopter = coordination::read(&repository).expect("reconstructs");

    let staffing = coordination::decide(&mut donor, coordination::also(shared, subject.hiring))
        .expect("one party plans");
    let equipping =
        coordination::decide(&mut adopter, coordination::also(shared, subject.equipment))
            .expect("and so does the other");

    // Only the adopter converged. The donor is still thinking, so its line is nowhere.
    converge::converge(&repository, &adopter).expect("converges");

    match transfer::reconstruct(&repository, shared, staffing, equipping) {
        Err(TransferError::Synthesis(SynthesisError::Thesis(ThesisError::UnknownThesis(
            thesis,
        )))) => {
            assert_eq!(thesis, staffing, "the world that has not arrived is named");
        }
        Err(other) => panic!("refused, and for another reason: {other:?}"),
        Ok(_) => panic!("expected a Source nothing holds, and the repository answered"),
    }

    // And once the donor converges, the same question has an answer. Nothing about the transfer
    // changed; what changed is that both lines are in one repository.
    converge::converge(&repository, &donor).expect("converges");

    transfer::reconstruct(&repository, shared, staffing, equipping)
        .expect("the same question, now answerable");
}

/// Reaching each other does not join two lines, and this is the cost of that.
///
/// Convergence concluded that Source and Target are roles rather than sides, and that the asymmetry
/// is a gain. With two parties the gain has a price: both adopting from the other produces **two**
/// worlds that select exactly the same commitments, and nothing in the repository says they are one
/// plan.
#[test]
fn reaching_each_other_does_not_join_two_lines() {
    let (repository, arrangement, staffing, equipping) = converged("mutual");
    let shared = arrangement.shared().id();

    let mut one = coordination::read(&repository).expect("reconstructs");
    let mut other = coordination::read(&repository).expect("reconstructs");

    let into_staffing =
        coordination::adopt(&mut one, shared, equipping, staffing).expect("one direction");
    let into_equipping =
        coordination::adopt(&mut other, shared, staffing, equipping).expect("and the other");

    converge::converge(&repository, &one).expect("converges");
    converge::converge(&repository, &other).expect("converges");

    assert_eq!(
        repository.read_lineage().expect("readable").len(),
        5,
        "two parties, two lines, and two ways of holding both plans"
    );

    let worlds = repository.read_worlds().expect("readable");
    let of = |id: ThesisId| {
        worlds
            .iter()
            .find(|world| world.thesis == id.to_string())
            .expect("the world is recorded")
            .clone()
    };

    let (here, there) = (of(into_staffing), of(into_equipping));

    assert_ne!(here.thesis, there.thesis, "two worlds");
    assert_eq!(
        here.open, there.open,
        "selecting exactly the same commitments"
    );
    assert_eq!(here.frozen, there.frozen);
    assert_eq!(here.known_at, there.known_at);
    assert_ne!(
        here.thesis_parent, there.thesis_parent,
        "and differing in nothing but whose branch they are on"
    );

    // Neither is downstream of the other, so no later decision can prefer one by extending it.
    let rebuilt = reading::corroborated(&repository).expect("reconstructs");
    let archive = rebuilt.lineage.archive();

    assert!(
        !descends_from(archive, into_staffing, into_equipping).expect("ancestry walks")
            && !descends_from(archive, into_equipping, into_staffing).expect("ancestry walks"),
        "two tips, and no relation between them"
    );
}

/// The keys of one record, as it is written down.
fn keys<T: serde::Serialize>(record: &T) -> BTreeSet<String> {
    serde_json::from_value::<serde_json::Map<String, serde_json::Value>>(
        serde_json::to_value(record).expect("a record serializes"),
    )
    .expect("an object")
    .keys()
    .cloned()
    .collect()
}

/// Every identity a decision itself mentions, which is not every identity it carries.
///
/// `after` and `witness` are addresses of knowledge, and knowledge is admitted by nobody in
/// particular. What this collects is the identities the *intention* names.
fn mentioned(decision: &Decision) -> BTreeSet<String> {
    match decision {
        Decision::Genesis { selection, .. } => selection.iter().map(|id| id.to_string()).collect(),
        Decision::Advance { extends, .. } => BTreeSet::from([extends.to_string()]),
        Decision::Fork {
            extends,
            omitted,
            introduced,
        } => std::iter::once(extends.to_string())
            .chain(omitted.iter().chain(introduced).map(|id| id.to_string()))
            .collect(),
    }
}

/// Phase 4 — Establish that it is not derivable.
///
/// Provenance's unanswerable question was about a world, and worlds are derived, so a search could
/// at least be attempted — it found too many answers rather than none. This one is about a party,
/// and the phase's job is to establish by asking that there is not even a candidate.
#[test]
fn phase_4_ask_who_decided() {
    let (repository, arrangement, _, _) = converged("phase-4");

    let decisions = repository.read_lineage().expect("readable");
    let worlds = repository.read_worlds().expect("readable");

    assert_eq!(decisions.len(), 3);

    // Every field of every decision, as a closed set per variant. Nothing here is a party, and an
    // added field would be named rather than tolerated.
    assert_eq!(
        keys(&decisions[0]),
        BTreeSet::from(["decides", "known_at", "selection", "after", "witness"].map(str::to_owned)),
        "the genesis"
    );
    for taken in &decisions[1..] {
        assert_eq!(
            keys(taken),
            BTreeSet::from(
                [
                    "decides",
                    "extends",
                    "omitted",
                    "introduced",
                    "after",
                    "witness"
                ]
                .map(str::to_owned)
            ),
            "each fork"
        );
    }
    assert_eq!(
        keys(&worlds[0]),
        BTreeSet::from(
            [
                "thesis",
                "thesis_parent",
                "known_at",
                "event_head",
                "frozen",
                "open"
            ]
            .map(str::to_owned)
        ),
        "and every world"
    );

    // The population exists. This is what makes the question worth asking rather than empty: the
    // repository knows agents, by name, and knows what each of them is a party to.
    let agents: BTreeSet<String> = arrangement
        .subject
        .admitted
        .agents
        .iter()
        .map(|id| id.to_string())
        .collect();

    assert_eq!(agents.len(), 2, "a payer and a payee");

    let parties: BTreeSet<String> = repository
        .read_journal()
        .expect("readable")
        .iter()
        .filter_map(|entry| match entry {
            ape_cli::journal::Admission::Commitment {
                accountable,
                executors,
                beneficiaries,
                ..
            } => Some(
                std::iter::once(accountable.to_string())
                    .chain(
                        executors
                            .iter()
                            .chain(beneficiaries)
                            .map(|id| id.to_string()),
                    )
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        })
        .flatten()
        .collect();

    assert_eq!(
        parties, agents,
        "and both of them are parties to commitments"
    );

    // And no decision names one. Derived from the journal rather than from a list written here, so
    // that an agent added to the subject is searched for too.
    for taken in &decisions {
        assert!(
            mentioned(&taken.decision).is_disjoint(&agents),
            "a decision names an agent: {:?}",
            mentioned(&taken.decision).intersection(&agents).next()
        );
    }

    // The nuance that keeps this honest: the agents *are* in the lineage file, inside the witness.
    // Which says they were known when the decision was taken, and never that they took it.
    assert!(
        agents.iter().all(|agent| decisions[0]
            .witness
            .iter()
            .any(|entry| entry.to_string() == *agent)),
        "the record says an agent was known, and that is a different statement"
    );
}

/// Two parties and one party are one repository.
///
/// The measurement that makes non-derivability a demonstration rather than an inventory. Provenance
/// established the same shape for a transfer carried versus a decision taken alone; here it is the
/// number of minds, which is a larger thing to be unable to see.
#[test]
fn two_parties_and_one_party_are_one_repository() {
    let (apart, arrangement, _, _) = converged("two-parties");

    let together = {
        let (repository, arrangement) = founded("one-party");
        let subject = &arrangement.subject;
        let shared = arrangement.shared().id();

        let mut alone = coordination::read(&repository).expect("reconstructs");

        coordination::decide(&mut alone, coordination::also(shared, subject.hiring))
            .expect("one mind, deciding both");
        coordination::decide(&mut alone, coordination::also(shared, subject.equipment))
            .expect("one after the other");

        converge::converge(&repository, &alone).expect("converges once");

        repository
    };

    assert_eq!(
        differing(&bytes(&apart), &bytes(&together)),
        None,
        "two parties interleaving and one party thinking twice produce the same repository"
    );

    // So there is no function from a repository to how many minds wrote it, and therefore none to
    // which of them wrote what. Not an ambiguous answer — no candidate at all.
    let _ = arrangement;
}

/// Part A's repair removed the last accidental trace of who wrote what.
///
/// Before it, the surviving decision was the last writer's, which Phase 1 measured. Position in the
/// file weakly encoded arrival, and arrival weakly encoded party. The canonical order is derived
/// from content, so it encodes neither.
#[test]
fn the_repair_erased_the_only_accidental_signal() {
    let (repository, arrangement, staffing, _) = converged("erased");

    let worlds = repository.read_worlds().expect("readable");
    let at = worlds
        .iter()
        .position(|world| world.thesis == staffing.to_string())
        .expect("the first party's world is recorded");

    // Phase 2 already measured that either order produces one repository. So whichever position the
    // first party's world holds, it holds it because of what the decision says — not because that
    // party went first, which the other order would have contradicted.
    let reversed = {
        let (repository, arrangement) = founded("erased-reversed");
        let subject = &arrangement.subject;
        let shared = arrangement.shared().id();

        let mut one = coordination::read(&repository).expect("reconstructs");
        let mut other = coordination::read(&repository).expect("reconstructs");

        coordination::decide(&mut one, coordination::also(shared, subject.hiring)).expect("plans");
        coordination::decide(&mut other, coordination::also(shared, subject.equipment))
            .expect("plans");

        converge::converge(&repository, &other).expect("the other party first, this time");
        converge::converge(&repository, &one).expect("converges");

        repository
            .read_worlds()
            .expect("readable")
            .iter()
            .position(|world| world.thesis == staffing.to_string())
            .expect("the same world, recorded")
    };

    assert_eq!(
        at, reversed,
        "the same position whichever party wrote first, so position says nothing about who"
    );

    let _ = arrangement;
}

/// Two parties who exist in the record, each with a line, each decision claimed.
///
/// The parties are admitted before they are named, because a party is knowledge and an attribution
/// is a claim. The genesis stays unattributed, as every experiment before this one wrote it — so the
/// arrangement holds the realistic shape of an optional field rather than a tidy one.
fn claimed(name: &str) -> (Repository, Founded, ThesisId, ThesisId, [AgentId; 2]) {
    let (repository, arrangement) = founded(name);
    let subject = &arrangement.subject;
    let shared = arrangement.shared().id();

    let mut one = coordination::read(&repository).expect("reconstructs");
    coordination::admit(&mut one, subject.planner.clone()).expect("a party is admitted");
    coordination::admit(&mut one, subject.steward.clone()).expect("and so is the other");

    let planner = one.admitted.agents[2];
    let steward = one.admitted.agents[3];

    let staffing = coordination::decided(
        &mut one,
        planner,
        coordination::also(shared, subject.hiring),
    )
    .expect("one party plans, and says so");

    converge::converge(&repository, &one).expect("converges");

    let mut other = coordination::read(&repository).expect("reconstructs");
    let equipping = coordination::decided(
        &mut other,
        steward,
        coordination::also(shared, subject.equipment),
    )
    .expect("and so does the other");

    converge::converge(&repository, &other).expect("converges");

    (
        repository,
        arrangement,
        staffing,
        equipping,
        [planner, steward],
    )
}

/// Phase 5 — Record it, and separate addressing from proof.
///
/// The corroboration experiment's two questions, answered against a record that holds a decider.
/// The second has a known and uncomfortable answer, and the phase's job is to state where the
/// checking stops rather than to soften it.
#[test]
fn phase_5_record_a_decider() {
    let (repository, arrangement, staffing, equipping, [planner, steward]) = claimed("phase-5");

    assert_eq!(
        repository.read_journal().expect("readable").len(),
        FOUNDED + 2,
        "two parties, admitted as knowledge"
    );

    let decisions = repository.read_lineage().expect("readable");

    assert_eq!(decisions.len(), 3);

    // What the record now says, and what it still does not. The genesis names nobody, because the
    // field is optional and had to be — so a reader cannot rely on the answer being there.
    assert_eq!(decisions[0].by, None, "the genesis claims no author");
    assert_eq!(
        keys(&decisions[0]),
        BTreeSet::from(["decides", "known_at", "selection", "after", "witness"].map(str::to_owned)),
        "and does not carry the field at all"
    );

    let attributed: BTreeSet<Option<AgentId>> =
        decisions[1..].iter().map(|taken| taken.by).collect();

    assert_eq!(
        attributed,
        BTreeSet::from([Some(planner), Some(steward)]),
        "and the two forks each name a party"
    );
    assert_eq!(
        keys(&decisions[1]),
        BTreeSet::from(
            [
                "decides",
                "extends",
                "omitted",
                "introduced",
                "after",
                "witness",
                "by"
            ]
            .map(str::to_owned)
        ),
        "one field, and it is the whole of what Part B adds"
    );

    // **What becomes impossible if this is not preserved.** A fresh process, holding no memory of
    // either session, is asked whose line each world is. With the record it answers.
    let worlds = repository.read_worlds().expect("readable");
    let by = |party: AgentId| -> BTreeSet<String> {
        decisions
            .iter()
            .zip(&worlds)
            .filter(|(taken, _)| taken.by == Some(party))
            .map(|(_, world)| world.thesis.clone())
            .collect()
    };

    assert_eq!(by(planner), BTreeSet::from([staffing.to_string()]));
    assert_eq!(by(steward), BTreeSet::from([equipping.to_string()]));

    // And Phase 4's measurement stops holding, which is the gain stated as a measurement: two
    // parties and one party are no longer one repository.
    let alone = {
        let (repository, arrangement) = founded("phase-5-alone");
        let subject = &arrangement.subject;
        let shared = arrangement.shared().id();

        let mut alone = coordination::read(&repository).expect("reconstructs");
        coordination::admit(&mut alone, subject.planner.clone()).expect("admitted");
        coordination::admit(&mut alone, subject.steward.clone()).expect("admitted");

        let one = alone.admitted.agents[2];

        coordination::decided(&mut alone, one, coordination::also(shared, subject.hiring))
            .expect("one mind");
        coordination::decided(
            &mut alone,
            one,
            coordination::also(shared, subject.equipment),
        )
        .expect("deciding both");

        converge::converge(&repository, &alone).expect("converges");

        repository
    };

    assert!(
        differing(&bytes(&repository), &bytes(&alone)).is_some(),
        "one party deciding both is now a different repository"
    );

    let _ = arrangement;
}

/// Everything the record can check about a party is about the reference.
///
/// Two refusals, and they are one check: the identity has to name an agent, and the agent has to
/// have been known at the coordinate the decision was taken at. Both come from asking the replay
/// rather than the whole journal.
#[test]
fn a_decider_is_checked_against_the_knowledge_that_stood() {
    let (repository, arrangement, _, _, [planner, _]) = claimed("checked");
    let subject = &arrangement.subject;

    // An identity that names no agent. A commitment's, so that it is an identity the repository
    // genuinely holds — a random one would be refused for being absent rather than for not being
    // an agent.
    let mut forged = repository.read_lineage().expect("readable");
    forged[1].by = Some(AgentId::from(*subject.hiring.as_ref()));
    repository.write_lineage(&forged).expect("writable");

    match reading::corroborated(&repository) {
        Err(ReadingError::Lineage(LineageError::DeciderNotKnown { agent })) => {
            assert_eq!(agent.to_string(), subject.hiring.to_string());
        }
        Err(other) => panic!("refused, and for another reason: {other:?}"),
        Ok(_) => panic!("expected a decider that is not an agent, and the repository read clean"),
    }

    // And a party that exists, attributed to a decision taken before it was admitted. Same
    // refusal, because at that coordinate the two are the same thing.
    let mut ahead = repository.read_lineage().expect("readable");
    ahead[1].by = None;
    ahead[0].by = Some(planner);
    repository.write_lineage(&ahead).expect("writable");

    match reading::corroborated(&repository) {
        Err(ReadingError::Lineage(LineageError::DeciderNotKnown { agent })) => {
            assert_eq!(
                agent, planner,
                "the party is real, and had not been admitted when the genesis was decided"
            );
        }
        Err(other) => panic!("refused, and for another reason: {other:?}"),
        Ok(_) => panic!("expected a party that did not exist yet, and the repository read clean"),
    }
}

/// And nothing is about the attribution. This is where the checking stops.
///
/// Swapping the two parties' claims produces a repository that reconstructs, corroborates, and says
/// the opposite of what happened. Every check passes, because every check is about the reference.
#[test]
fn a_false_attribution_is_not_refused() {
    let (repository, _, _, _, [planner, steward]) = claimed("false");

    let mut swapped = repository.read_lineage().expect("readable");

    // Located by party rather than by position, because the canonical order now takes the claim into
    // account — a decision's place in the file moved when it gained a party, which is a coupling
    // worth naming: Part A's linearization read what the decisions say, and this adds who says it.
    let at = |party: AgentId| {
        swapped
            .iter()
            .position(|taken| taken.by == Some(party))
            .expect("the party's decision is recorded")
    };

    let (here, there) = (at(planner), at(steward));

    swapped[here].by = Some(steward);
    swapped[there].by = Some(planner);

    repository.write_lineage(&swapped).expect("writable");

    let rebuilt = reading::corroborated(&repository).expect("the lie reconstructs");

    assert_eq!(rebuilt.lineage.decided().len(), 3);
    assert_eq!(
        rebuilt.decisions[here].by,
        Some(steward),
        "and the record now attributes each line to the other party"
    );
}

/// What the claim couples, measured as provenance measured its own.
///
/// Phase 2 established that deciding what is already decided adds nothing — one record and one
/// world, because identity is derived from content. A decider is not derived from content, so that
/// stops being true: two parties agreeing produce two records of one world.
#[test]
fn a_decider_makes_agreement_look_like_duplication() {
    let (repository, arrangement, staffing, _, [_, steward]) = claimed("duplication");
    let subject = &arrangement.subject;
    let shared = arrangement.shared().id();

    assert_eq!(repository.read_lineage().expect("readable").len(), 3);

    let mut agreeing = coordination::read(&repository).expect("reconstructs");
    let reached = coordination::decided(
        &mut agreeing,
        steward,
        coordination::also(shared, subject.hiring),
    )
    .expect("the other party decides the same plan");

    converge::converge(&repository, &agreeing).expect("converges");

    assert_eq!(reached, staffing, "one world");
    assert_eq!(
        repository.read_lineage().expect("readable").len(),
        4,
        "and two records of it"
    );

    let worlds = repository.read_worlds().expect("readable");
    let held = worlds
        .iter()
        .filter(|world| world.thesis == staffing.to_string())
        .count();

    assert_eq!(held, 2, "recorded twice, once per party that claims it");

    // Which cuts both ways, and the honest reading is that this is the agreement record Phase 3
    // said was missing — for the case where two parties decide the *same* fork, and only that case.
    // The mutual adoption Phase 3 measured still produces two different worlds, because a parent is
    // part of an identity and the two parties are on different branches.
}

/// Ask an operating-system process of its own, given the repository and nothing else.
fn asked(repository: &std::path::Path, question: &[&str]) -> std::process::Output {
    std::process::Command::new(std::path::Path::new(env!("CARGO_BIN_EXE_ape-cli")))
        .arg(repository)
        .args(question)
        .output()
        .expect("the binary runs")
}

/// The answer, or the reason the process refused to give one.
fn answered<T: serde::de::DeserializeOwned>(output: &std::process::Output) -> T {
    assert!(
        output.status.success(),
        "the fresh process failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    serde_json::from_slice(&output.stdout).expect("an answer came back")
}

/// Phase 6 — Terminate, rebuild, compare.
///
/// The processes that decided are gone, and here that is literal: what answers shares no memory with
/// them. Everything the two parties recorded has to arrive through the repository or not at all.
#[test]
fn phase_6_terminate() {
    let (repository, arrangement, staffing, equipping, [planner, steward]) = claimed("phase-6");
    let subject = &arrangement.subject;

    // Nothing, given nothing.
    assert!(
        !asked(
            &scratch("phase-6-absent"),
            &[&subject.instance.to_string(), EFFECTIVE]
        )
        .status
        .success(),
        "a fresh process with no repository must not produce a world"
    );

    // The worlds, against literals.
    let readings: Vec<reading::Reading> = answered(&asked(
        repository.root(),
        &[&subject.instance.to_string(), EFFECTIVE],
    ));

    assert_eq!(readings.len(), 3);
    assert_eq!(
        readings[0].thesis,
        arrangement.shared().id().to_string(),
        "the shared ancestor, before anything that extends it"
    );
    assert_eq!(
        readings[1..]
            .iter()
            .map(|world| world.thesis.clone())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([equipping.to_string(), staffing.to_string()]),
        "and one line per party"
    );

    // The two lines are compared as a set on purpose. The linearization promises determinism and
    // ancestry — the same decisions produce the same file, and a parent precedes its children — and
    // it promises nothing about the order between siblings, which falls out of identity bytes.
    //
    // Written first as an ordered comparison, which is over-specification: it passed, and then
    // failed when a field was removed from `Agent` and every identity moved. The assertion was
    // claiming a guarantee the record does not make.
    for world in &readings[1..] {
        assert_eq!(
            world.thesis_parent,
            Some(readings[0].thesis.clone()),
            "each line extends the ancestor that precedes it"
        );
    }

    // What the order is **not** is arrival: the planner converged first, and Phase 4 measured why
    // position cannot be read as who.

    // And whose each line is, which is the only thing Part B added and the only thing here that
    // could not have been asked before.
    let claims = |party: AgentId| -> BTreeSet<String> {
        answered(&asked(repository.root(), &["decided", &party.to_string()]))
    };

    assert_eq!(claims(planner), BTreeSet::from([staffing.to_string()]));
    assert_eq!(claims(steward), BTreeSet::from([equipping.to_string()]));

    // A party the repository does not hold claims nothing, and is not refused. Which is consistent
    // and worth stating: the check that refuses an unknown decider is about what is *written*, and
    // asking about somebody who never decided is a question with an empty answer rather than an
    // error.
    let stranger = AgentId::from(*subject.hiring.as_ref());

    assert!(claims(stranger).is_empty());

    // What the answers do not cover, measured. Every world minus every claimed world is not empty:
    // the genesis is claimed by nobody, and no query says so.
    let claimed: BTreeSet<String> = claims(planner).union(&claims(steward)).cloned().collect();
    let all: BTreeSet<String> = readings.iter().map(|world| world.thesis.clone()).collect();

    assert_eq!(
        all.difference(&claimed).collect::<Vec<_>>(),
        vec![&arrangement.shared().id().to_string()],
        "one world belongs to nobody, and a reader cannot tell that from not-this-party's"
    );
}

/// There is no room on a world for who decided it, and it is the same reason provenance found.
///
/// A reading is per world; an attribution is per decision; and one world can be produced by two
/// decisions. So the answer cannot travel on the thing a reader reads — measured through the process
/// boundary, where a per-world record is all there is.
#[test]
fn a_decider_does_not_fit_on_a_world() {
    let (repository, arrangement, staffing, equipping, [planner, steward]) = claimed("no-room");
    let subject = &arrangement.subject;
    let shared = arrangement.shared().id();

    let mut agreeing = coordination::read(&repository).expect("reconstructs");
    coordination::decided(
        &mut agreeing,
        steward,
        coordination::also(shared, subject.hiring),
    )
    .expect("the other party decides the same plan");

    converge::converge(&repository, &agreeing).expect("converges");

    let claims = |party: AgentId| -> BTreeSet<String> {
        answered(&asked(repository.root(), &["decided", &party.to_string()]))
    };

    // One world, claimed by two parties. No field on a world could hold that.
    assert_eq!(claims(planner), BTreeSet::from([staffing.to_string()]));
    assert_eq!(
        claims(steward),
        BTreeSet::from([staffing.to_string(), equipping.to_string()]),
        "and the same world is claimed by both"
    );

    // Which surfaces at the boundary as a duplicate: the reading form walks decisions, so a world
    // two decisions produced is read twice.
    let readings: Vec<reading::Reading> = answered(&asked(
        repository.root(),
        &[&subject.instance.to_string(), EFFECTIVE],
    ));

    assert_eq!(readings.len(), 4, "four decisions");
    assert_eq!(
        readings
            .iter()
            .map(|world| world.thesis.clone())
            .collect::<BTreeSet<_>>()
            .len(),
        3,
        "and three worlds"
    );
}
