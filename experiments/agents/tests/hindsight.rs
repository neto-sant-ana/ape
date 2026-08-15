//! The setup experiment 02 audits, checked before an auditor is asked anything.
//!
//! Criteria 1 to 3 of the protocol are mechanical, and they are the part that must hold for
//! the audit to be measuring anything at all: the obligation has to be genuinely out of reach
//! of the cut the decision was taken under, the decision has to read the same with it in
//! history, and the world that contains it has to genuinely conflict.
//!
//! The plot lives here and not in `hindsight.rs`, because that file is handed to the auditor.
//! What the sequence withholds it withholds from the auditor, never from the checks that keep
//! the auditor honest.

use ape::engine::hermeneia::{Conflict, Hypothesis};
use ape::engine::thesis::{GenesisInput, Interpretation, KnowledgeCut, Thesis, ThesisId, ThesisLookup};
use ape::kernel::entities::CommitmentId;

use ape_agents::hindsight::{self, Replay, january};
use ape_agents::world;

fn decided_at() -> ape::kernel::value_objects::Date {
    january(6)
}

fn audited_at() -> ape::kernel::value_objects::Date {
    january(12)
}

fn priority(replay: &Replay) -> CommitmentId {
    replay.intentions[0]
}

fn standard(replay: &Replay) -> CommitmentId {
    replay.intentions[1]
}

fn obligation(replay: &Replay) -> CommitmentId {
    replay.intentions[2]
}

/// The world the decision under audit was taken in: the third opened, counting the two that
/// only carried knowledge forward.
fn decision(replay: &Replay) -> ThesisId {
    replay.worlds[2]
}

fn conflicts(replay: &Replay, thesis: &Thesis) -> Vec<Conflict> {
    Interpretation::of(thesis, replay.graph.canon.history())
        .expect("the Thesis is interpretable")
        .feasibility_under(Hypothesis::FinalState)
        .expect("feasibility is derivable")
        .conflicts()
        .to_vec()
}

fn resolve(replay: &Replay, id: ThesisId) -> Thesis {
    replay
        .graph
        .archive
        .thesis(id)
        .expect("the archive holds what was stored in it")
}

/// Criterion 1 — the obligation cannot be selected at the instant the decision was taken.
///
/// It was recorded after that instant, so a cut there does not reach it. This is the property
/// the whole experiment rests on, and asserting it is cheaper than trusting it.
#[test]
fn the_obligation_is_out_of_reach_of_the_decision_instant() {
    let replay = hindsight::build();

    let refused = Thesis::genesis(
        replay.graph.canon.history(),
        GenesisInput {
            cut: KnowledgeCut::at(replay.graph.canon.history(), decided_at()),
            selection: [obligation(&replay)].into(),
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
    let replay = hindsight::build();
    let decision = resolve(&replay, decision(&replay));

    assert_eq!(decision.cut().known_at(), &decided_at());
    assert!(!decision.selection().contains(obligation(&replay)));

    assert!(
        conflicts(&replay, &decision).is_empty(),
        "spending 30 of 100 conflicts with nothing under what could be known"
    );
}

/// Criterion 3 — and the world that knows the obligation genuinely does not.
///
/// Without this the comparison would be between two readings that agree, and agreement
/// proves nothing about hindsight.
#[test]
fn the_world_that_knows_the_obligation_is_short() {
    let replay = hindsight::build();
    let current = resolve(&replay, replay.graph.current);

    assert_eq!(current.cut().known_at(), &audited_at());
    assert!(current.selection().contains(obligation(&replay)));

    assert_eq!(
        conflicts(&replay, &current),
        vec![Conflict::OutOfBounds {
            instance: world::build().account,
            level: -20.0,
        }],
        "100 in, 30 spent and 90 owed leaves the account 20 below its floor"
    );
}

/// What one entry point reaches.
///
/// The engine's reads are by identity: nothing enumerates commitments and nothing enumerates
/// theses. So an auditor's reach is exactly the transitive closure of walking `parent` from
/// the thesis it is handed, and reading the selections it finds on the way.
///
/// This records that the walk finds both candidates — the one taken and the one abandoned —
/// which is what makes an audit of the deliberation conceivable rather than hopeless.
#[test]
fn walking_the_lineage_reaches_both_candidates() {
    let replay = hindsight::build();

    let mut reached: Vec<CommitmentId> = Vec::new();
    let mut visited = 0;
    let mut next = Some(replay.graph.current);

    while let Some(id) = next {
        let thesis = resolve(&replay, id);
        visited += 1;
        reached.extend(thesis.selection().resolved());
        next = *thesis.parent();
    }

    // Five, not three. Recognizing new knowledge is a node of its own, so each of the two
    // decisions in this lineage is preceded by the advancement that made it possible.
    assert_eq!(visited, 5, "two forks, two advancements, and the genesis");

    assert!(
        reached.contains(&standard(&replay)),
        "the intention taken is reached"
    );
    assert!(
        reached.contains(&priority(&replay)),
        "the intention abandoned is reached, through the ancestor that selected it"
    );
    assert!(
        reached.contains(&obligation(&replay)),
        "the knowledge that made the decision look bad is reached"
    );
}

/// The world is a function of the sequence.
///
/// This is what makes handing an auditor the sequence equivalent to handing it the world,
/// rather than a convenience of the harness: two replays of the same steps agree on every
/// identity, and identities here are derived from content.
#[test]
fn the_same_sequence_produces_the_same_worlds() {
    let once = hindsight::replay(&hindsight::scenario());
    let twice = hindsight::replay(&hindsight::scenario());

    assert_eq!(once.graph.current, twice.graph.current);
    assert_eq!(once.worlds, twice.worlds);
    assert_eq!(once.intentions, twice.intentions);
}
