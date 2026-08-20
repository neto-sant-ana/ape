//! The setup experiment 02 audits, checked before an auditor is asked anything.
//!
//! Migrated to the repository substrate. Every assertion keeps the value and the wording it had
//! over the in-memory adapters; where one moved it would be a finding, and none did.
//!
//! The plot lives here and not in `hindsight.rs`, because that file is handed to the auditor.

use ape::engine::hermeneia::{Conflict, Hypothesis};
use ape::engine::thesis::{GenesisInput, Interpretation, KnowledgeCut, Thesis};
use ape::kernel::entities::CommitmentId;

use ape_agents::hindsight::{self, Built};
use ape_agents::world;

fn conflicts(built: &Built, thesis: &Thesis) -> Vec<Conflict> {
    Interpretation::of(thesis, built.canon.history())
        .expect("the Thesis is interpretable")
        .feasibility_under(Hypothesis::FinalState)
        .expect("feasibility is derivable")
        .conflicts()
        .to_vec()
}

fn priority(built: &Built) -> CommitmentId {
    built.intentions[0]
}

fn standard(built: &Built) -> CommitmentId {
    built.intentions[1]
}

fn obligation(built: &Built) -> CommitmentId {
    built.intentions[2]
}

/// The world the decision under audit was taken in: the third opened, counting the two that
/// only carried knowledge forward.
fn decision(built: &Built) -> Thesis {
    built.world_at(built.worlds[2])
}

/// Criterion 1 — the obligation cannot be selected at the instant the decision was taken.
#[test]
fn the_obligation_is_out_of_reach_of_the_decision_instant() {
    let built = hindsight::build();

    let refused = Thesis::genesis(
        built.canon.history(),
        GenesisInput {
            cut: KnowledgeCut::at(built.canon.history(), world::on(6)),
            selection: [obligation(&built)].into(),
        },
    );

    assert!(
        refused.is_err(),
        "a cut at the decision instant cannot select knowledge recorded after it"
    );
}

/// Criterion 2 — the decision reads the same with the obligation sitting in history.
#[test]
fn the_decision_still_reads_clean() {
    let built = hindsight::build();
    let decision = decision(&built);

    assert_eq!(decision.cut().known_at(), &world::on(6));
    assert!(!decision.selection().contains(obligation(&built)));

    assert!(
        conflicts(&built, &decision).is_empty(),
        "spending 30 of 100 conflicts with nothing under what could be known"
    );
}

/// Criterion 3 — and the world that knows the obligation genuinely does not.
#[test]
fn the_world_that_knows_the_obligation_is_short() {
    let built = hindsight::build();
    let current = built.world_at(built.current());

    assert_eq!(current.cut().known_at(), &world::on(12));
    assert!(current.selection().contains(obligation(&built)));

    assert_eq!(
        conflicts(&built, &current),
        vec![Conflict::OutOfBounds {
            instance: built.world.account,
            level: -20,
        }],
        "100 in, 30 spent and 90 owed leaves the account 20 below its floor"
    );
}

/// What one entry point reaches, by walking `parent` and reading the selections found on the way.
#[test]
fn walking_the_lineage_reaches_both_candidates() {
    let built = hindsight::build();

    let mut reached: Vec<CommitmentId> = Vec::new();
    let mut visited = 0;
    let mut next = Some(built.current());

    while let Some(id) = next {
        let thesis = built.world_at(id);
        visited += 1;
        reached.extend(thesis.selection().resolved());
        next = *thesis.parent();
    }

    // Five, not three. Recognizing new knowledge is a node of its own, so each of the two
    // decisions in this lineage is preceded by the advancement that made it possible.
    assert_eq!(visited, 5, "two forks, two advancements, and the genesis");

    assert!(
        reached.contains(&standard(&built)),
        "the intention taken is reached"
    );
    assert!(
        reached.contains(&priority(&built)),
        "the intention abandoned is reached, through the ancestor that selected it"
    );
    assert!(
        reached.contains(&obligation(&built)),
        "the knowledge that made the decision look bad is reached"
    );
}

/// The genesis's cut cannot be rebuilt today, and that refusal is evidence.
#[test]
fn the_genesis_cut_cannot_be_rebuilt_today() {
    let built = hindsight::build();
    let history = built.canon.history();

    let genesis = built.world_at(built.worlds[0]);
    let taken_under = genesis.cut().event_head().expect("the opening had settled");

    assert_eq!(genesis.cut().known_at(), &world::on(6));

    assert_ne!(
        KnowledgeCut::at(history, world::on(6)).event_head(),
        Some(taken_under),
        "the instant now addresses the cancellation, not what the genesis was taken under"
    );

    assert!(
        KnowledgeCut::within(history, world::on(6), taken_under).is_err(),
        "a head from an earlier instant cannot be named as a finer cut within a later one"
    );
}

/// The limit of that evidence, stated so it is not overclaimed.
#[test]
fn a_cut_on_a_day_nothing_followed_is_still_constructible() {
    let built = hindsight::build();
    let genesis = built.world_at(built.worlds[0]);
    let taken_under = genesis.cut().event_head().expect("the opening had settled");

    assert_eq!(
        KnowledgeCut::at(built.canon.history(), world::on(5)).event_head(),
        Some(taken_under),
        "a quiet day still addresses the head it addressed then"
    );
}

/// The world is a function of the sequence.
#[test]
fn the_same_sequence_produces_the_same_worlds() {
    let once = hindsight::replay(&hindsight::scenario());
    let twice = hindsight::replay(&hindsight::scenario());

    assert_eq!(once.current(), twice.current());
    assert_eq!(once.worlds, twice.worlds);
    assert_eq!(once.intentions, twice.intentions);
}

/// New with the substrate: the two records agree, and the coordinate is what makes them able to.
///
/// Every decision names the journal entry that stood when it was taken, and its witness is that
/// same prefix said a second way. This is the property the in-memory adapters had no way to
/// carry, and the reason a claim of unfalsifiability had nothing to be tested against.
#[test]
fn every_decision_names_a_journal_entry_that_exists() {
    let built = hindsight::build();

    let entries: Vec<_> = built.world.admitted.entries.clone();
    assert!(!built.taken.is_empty(), "the sequence takes decisions");

    for taken in &built.taken {
        assert!(
            built.witnessed(&taken.after),
            "a decision names an entry the journal does not hold"
        );
        assert!(
            taken.witness.contains(&taken.after),
            "the prefix a decision witnesses ends at the entry it names"
        );
        assert_eq!(
            taken.by,
            Some(built.world.house),
            "every decision here is claimed by the house"
        );
    }

    assert!(
        entries.len() >= 5,
        "the scan ran against a journal, not against an empty one"
    );
}
