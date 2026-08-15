//! The setup experiment 02 audits, checked before an auditor is asked anything.
//!
//! Criteria 1 to 3 of the protocol are mechanical, and they are the part that must hold for
//! the audit to be measuring anything at all: the obligation has to be genuinely out of reach
//! of the cut the decision was taken under, the decision has to read the same with it in
//! history, and the world that contains it has to genuinely conflict.
//!
//! The last test is not a criterion. It walks the lineage from the single entry point an
//! auditor is given, and records what that walk can reach — which decides whether the audit
//! is possible at all, and is therefore worth knowing before asking anyone to perform it.

use ape::engine::hermeneia::{Conflict, Hypothesis};
use ape::engine::thesis::{
    GenesisInput, Interpretation, KnowledgeCut, Thesis, ThesisLookup,
};
use ape::kernel::entities::CommitmentId;

use ape_agents::hindsight::{self, Scenario, audited_at, decided_at};
use ape_agents::world;

fn conflicts(scenario: &Scenario, thesis: &Thesis) -> Vec<Conflict> {
    Interpretation::of(thesis, scenario.graph.canon.history())
        .expect("the Thesis is interpretable")
        .feasibility_under(Hypothesis::FinalState)
        .expect("feasibility is derivable")
        .conflicts()
        .to_vec()
}

fn resolve(scenario: &Scenario, id: ape::engine::thesis::ThesisId) -> Thesis {
    scenario
        .graph
        .archive
        .thesis(id)
        .expect("the archive holds what was stored in it")
}

// The harness knows the plot, and says so plainly. What the sequence withholds it withholds
// from the auditor, not from the checks that keep the auditor honest.
fn priority(scenario: &Scenario) -> CommitmentId {
    scenario.intentions[0]
}

fn standard(scenario: &Scenario) -> CommitmentId {
    scenario.intentions[1]
}

fn obligation(scenario: &Scenario) -> CommitmentId {
    scenario.intentions[2]
}

/// Criterion 1 — the obligation cannot be selected at the instant the decision was taken.
///
/// It was recorded after that instant, so a cut there does not reach it. This is the property
/// the whole experiment rests on, and asserting it is cheaper than trusting it.
#[test]
fn the_obligation_is_out_of_reach_of_the_decision_instant() {
    let scenario = hindsight::build();

    let refused = Thesis::genesis(
        scenario.graph.canon.history(),
        GenesisInput {
            cut: KnowledgeCut::at(scenario.graph.canon.history(), decided_at()),
            selection: [obligation(&scenario)].into(),
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
    let scenario = hindsight::build();
    let decision = resolve(&scenario, scenario.decision());

    assert_eq!(decision.cut().known_at(), &decided_at());
    assert!(!decision.selection().contains(obligation(&scenario)));

    assert!(
        conflicts(&scenario, &decision).is_empty(),
        "spending 30 of 100 conflicts with nothing under what could be known"
    );
}

/// Criterion 3 — and the world that knows about the obligation genuinely does not.
///
/// Without this the comparison would be between two readings that agree, and agreement
/// proves nothing about hindsight.
#[test]
fn the_world_that_knows_the_obligation_is_short() {
    let scenario = hindsight::build();
    let current = resolve(&scenario, scenario.graph.current);

    assert_eq!(current.cut().known_at(), &audited_at());
    assert!(current.selection().contains(obligation(&scenario)));

    assert_eq!(
        conflicts(&scenario, &current),
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
    let scenario = hindsight::build();

    let mut reached: Vec<CommitmentId> = Vec::new();
    let mut visited = 0;
    let mut next = Some(scenario.graph.current);

    while let Some(id) = next {
        let thesis = resolve(&scenario, id);
        visited += 1;
        reached.extend(thesis.selection().resolved());
        next = *thesis.parent();
    }

    // Five, not three. Recognizing new knowledge is a node of its own, so each of the two
    // decisions in this lineage is preceded by the advancement that made it possible.
    assert_eq!(visited, 5, "two forks, two advancements, and the genesis");

    assert!(
        reached.contains(&standard(&scenario)),
        "the intention taken is reached"
    );
    assert!(
        reached.contains(&priority(&scenario)),
        "the intention abandoned is reached, through the ancestor that selected it"
    );
    assert!(
        reached.contains(&obligation(&scenario)),
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
