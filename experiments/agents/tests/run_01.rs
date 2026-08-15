//! Phase 6 of run 01 — whether the reading taken at the moment of decision survives the
//! knowledge that arrived after it.
//!
//! The sequence below reproduces what the agent did, in `run-01/main.rs`: it formed the
//! intention it wanted, was told the account could not carry it, and cancelled it. That
//! cancellation is what makes this a real test rather than a tautology — under knowledge as
//! it stands now, the priority slot conflicts with nothing, because a cancelled commitment
//! moves no level.
//!
//! So there are two honest readings of the same commitment, and the experiment's whole claim
//! is that they do not contaminate each other:
//!
//! ```text
//! read under the cut it was decided under → out of bounds at -20
//! read under the cut current afterwards   → nothing found
//! ```
//!
//! The first must not become the second. A record in which the decision quietly acquires the
//! innocence of hindsight would be worth nothing for saying what was decided and why.
//!
//! The agent's own file is the record and is not edited. This is the harness performing the
//! comparison the protocol asks for, over the same sequence.

use ape::canon::EventSubmission;
use ape::engine::hermeneia::{Conflict, Hypothesis};
use ape::engine::thesis::{GenesisInput, Interpretation, KnowledgeCut, Thesis};
use ape::kernel::entities::{CommitmentId, CommitmentInput, EventId};
use ape::kernel::value_objects::{ActionValue, Assignment, Date, Term};

use ape_agents::world::{self, World, cancelling, today};

const PRIORITY_COST: f64 = 120.0;
const STANDARD_COST: f64 = 30.0;

/// The world at the instant the decision was taken, and the Thesis that denotes it.
struct Decision {
    world: World,
    priority: CommitmentId,
    thesis: Thesis,
    head: Option<EventId>,
}

fn decide() -> Decision {
    let mut world = world::build();

    let priority = admit(&mut world, PRIORITY_COST, january(8));

    let cut = KnowledgeCut::at(world.canon.history(), today());
    let head = cut.event_head();

    let thesis = Thesis::genesis(
        world.canon.history(),
        GenesisInput {
            cut,
            selection: [priority].into(),
        },
    )
    .expect("the priority slot is selectable at today's cut");

    Decision {
        world,
        priority,
        thesis,
        head,
    }
}

/// What the world learned after the decision: a standard slot, and the fact that the
/// priority slot will not happen.
fn then_knowledge_moves(decision: &mut Decision) {
    admit(&mut decision.world, STANDARD_COST, january(12));

    decision
        .world
        .canon
        .admit_event(
            EventSubmission {
                commitment_id: decision.priority,
                observation: cancelling(),
                occurred_at: today(),
            },
            today(),
        )
        .expect("an unrealizable intention may be cancelled");
}

fn conflicts(world: &World, thesis: &Thesis, hypothesis: Hypothesis) -> Vec<Conflict> {
    let interpretation =
        Interpretation::of(thesis, world.canon.history()).expect("the Thesis is interpretable");

    interpretation
        .feasibility_under(hypothesis)
        .expect("feasibility is derivable")
        .conflicts()
        .to_vec()
}

const EVERY_HYPOTHESIS: [Hypothesis; 3] = [
    Hypothesis::FinalState,
    Hypothesis::OnDueDateNet,
    Hypothesis::OnDueDateInAnyOrder,
];

/// Phase 3 — what the decision was read as, at the moment it was taken.
///
/// This is the reference the next test is measured against, so every value asserted here is
/// one that must reappear unchanged.
#[test]
fn phase_3_records_how_the_decision_read_when_it_was_taken() {
    let decision = decide();

    assert_eq!(decision.thesis.cut().known_at(), &today());
    assert_eq!(decision.thesis.selection().len(), 2);

    for hypothesis in EVERY_HYPOTHESIS {
        assert_eq!(
            conflicts(&decision.world, &decision.thesis, hypothesis),
            vec![Conflict::OutOfBounds {
                instance: decision.world.account,
                level: -20.0,
            }],
            "spending 120 against 100 leaves the account below its floor under {hypothesis:?}"
        );
    }
}

/// Phase 6 — the same Thesis, after the knowledge that would exonerate it.
#[test]
fn phase_6_the_decision_reads_the_same_after_knowledge_moved() {
    let mut decision = decide();
    let before: Vec<_> = EVERY_HYPOTHESIS
        .map(|hypothesis| conflicts(&decision.world, &decision.thesis, hypothesis))
        .into();

    then_knowledge_moves(&mut decision);

    assert_eq!(
        decision.thesis.cut().event_head(),
        decision.head,
        "the cut still recognizes the chain it recognized, not the one that grew past it"
    );

    assert_eq!(decision.thesis.cut().known_at(), &today());
    assert_eq!(decision.thesis.selection().len(), 2);
    assert!(decision.thesis.selection().contains(decision.priority));

    let after: Vec<_> = EVERY_HYPOTHESIS
        .map(|hypothesis| conflicts(&decision.world, &decision.thesis, hypothesis))
        .into();

    assert_eq!(
        before, after,
        "the reading taken at the moment of decision did not move when knowledge did"
    );
}

/// The contrast that keeps the test above from being a tautology.
///
/// Under a cut taken now, the very same commitment conflicts with nothing — it was
/// cancelled, and a cancelled commitment moves no level. Both readings are correct, and they
/// answer different questions.
#[test]
fn the_same_commitment_reads_innocent_under_a_later_cut() {
    let mut decision = decide();
    then_knowledge_moves(&mut decision);

    let cut = KnowledgeCut::at(decision.world.canon.history(), today());

    assert_ne!(
        cut.event_head(),
        decision.head,
        "knowledge moved, so a cut taken now recognizes a longer chain"
    );

    let hindsight = Thesis::genesis(
        decision.world.canon.history(),
        GenesisInput {
            cut,
            selection: [decision.priority].into(),
        },
    )
    .expect("the priority slot is still selectable");

    for hypothesis in EVERY_HYPOTHESIS {
        assert!(
            conflicts(&decision.world, &hindsight, hypothesis).is_empty(),
            "a cancelled commitment moves no level, so nothing is found under {hypothesis:?}"
        );
    }
}

fn admit(world: &mut World, amount: f64, due: Date) -> CommitmentId {
    world
        .canon
        .admit_commitment(
            CommitmentInput {
                assignment: Assignment::new(world.house, [world.house], [world.market])
                    .expect("both sides are staffed"),
                statement: world.outbound,
                resource: world.account,
                term: Term::new(today(), due).expect("committed before due"),
                action_value: ActionValue::value(amount).expect("a positive, finite magnitude"),
                dependencies: [].into(),
            },
            today(),
        )
        .expect("the house may commit to spend")
}

fn january(day: u8) -> Date {
    Date::from_ymd(2026, 1, day).expect("a real date in January 2026")
}
