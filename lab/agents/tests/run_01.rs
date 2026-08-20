//! Phase 6 of run 01 — whether the reading taken at the moment of decision survives the
//! knowledge that arrived after it.
//!
//! Migrated to the repository substrate, every assertion keeping its value and its wording. The
//! sequence is shorter than it was, because folding a prefix now leaves both things the
//! comparison needs in one place: the world as it was decided, and a canon that has since moved.

use ape::engine::hermeneia::{Conflict, Hypothesis};
use ape::engine::thesis::{GenesisInput, Interpretation, KnowledgeCut, Thesis};

use ape_agents::hindsight::{self, Built, Step};
use ape_agents::world;

const EVERY_HYPOTHESIS: [Hypothesis; 3] = [
    Hypothesis::FinalState,
    Hypothesis::OnDueDateNet,
    Hypothesis::OnDueDateInAnyOrder,
];

/// The intention the house wanted, and the world it was read in.
fn at_the_decision() -> Built {
    hindsight::replay(&hindsight::scenario()[..2])
}

/// The same, plus the knowledge that would exonerate it.
fn after_it_moved() -> Built {
    hindsight::replay(&hindsight::scenario()[..4])
}

fn conflicts(built: &Built, thesis: &Thesis, hypothesis: Hypothesis) -> Vec<Conflict> {
    Interpretation::of(thesis, built.canon.history())
        .expect("the Thesis is interpretable")
        .feasibility_under(hypothesis)
        .expect("feasibility is derivable")
        .conflicts()
        .to_vec()
}

fn out_of_bounds(built: &Built) -> Vec<Conflict> {
    vec![Conflict::OutOfBounds {
        instance: built.world.account,
        level: -20,
    }]
}

/// Phase 3 — what the decision was read as, at the moment it was taken.
#[test]
fn phase_3_records_how_the_decision_read_when_it_was_taken() {
    let built = at_the_decision();
    let decided = built.world_at(built.current());

    assert_eq!(decided.cut().known_at(), &world::on(6));
    assert_eq!(decided.selection().len(), 2);

    for hypothesis in EVERY_HYPOTHESIS {
        assert_eq!(
            conflicts(&built, &decided, hypothesis),
            out_of_bounds(&built),
            "spending 120 against 100 leaves the account below its floor under {hypothesis:?}"
        );
    }
}

/// Phase 6 — the same Thesis, after the knowledge that would exonerate it.
#[test]
fn phase_6_the_decision_reads_the_same_after_knowledge_moved() {
    let before = at_the_decision();
    let taken_under = before
        .world_at(before.current())
        .cut()
        .event_head()
        .expect("the opening had settled");

    let after = after_it_moved();
    let decided = after.world_at(after.worlds[0]);

    assert_eq!(
        decided.cut().event_head(),
        Some(taken_under),
        "the cut still recognizes the chain it recognized, not the one that grew past it"
    );

    assert_eq!(decided.cut().known_at(), &world::on(6));
    assert_eq!(decided.selection().len(), 2);
    assert!(decided.selection().contains(after.intentions[0]));

    for hypothesis in EVERY_HYPOTHESIS {
        assert_eq!(
            conflicts(&after, &decided, hypothesis),
            out_of_bounds(&after),
            "the reading taken at the moment of decision did not move when knowledge did"
        );
    }
}

/// The contrast that keeps the test above from being a tautology.
#[test]
fn the_same_commitment_reads_innocent_under_a_later_cut() {
    let after = after_it_moved();
    let history = after.canon.history();

    let hindsight = Thesis::genesis(
        history,
        GenesisInput {
            cut: KnowledgeCut::at(history, world::on(6)),
            selection: [after.intentions[0]].into(),
        },
    )
    .expect("the priority slot is still selectable");

    for hypothesis in EVERY_HYPOTHESIS {
        assert!(
            conflicts(&after, &hindsight, hypothesis).is_empty(),
            "a cancelled commitment moves no level, so nothing is found under {hypothesis:?}"
        );
    }
}

/// A step the previous substrate could not take: the world the house undertook, and the fact
/// that dropping the refused intention is no longer available once it settled.
#[test]
fn the_refused_intention_can_no_longer_be_dropped() {
    let undertaken = hindsight::replay(&hindsight::scenario()[..6]);
    let current = undertaken.world_at(undertaken.current());

    assert!(
        conflicts(&undertaken, &current, Hypothesis::FinalState).is_empty(),
        "the world the house undertook conflicts with nothing"
    );

    let droppable = hindsight::replay(&[
        Step::Intend {
            magnitude: 120,
            incoming: false,
            due: 8,
            recorded_at: 6,
        },
        Step::Open {
            known_at: 6,
            select: vec![0],
        },
    ]);

    assert!(
        droppable
            .world_at(droppable.current())
            .selection()
            .open()
            .count()
            == 1,
        "before the withdrawal, the intention is open and a fork could drop it"
    );

    assert!(
        current.selection().is_frozen(undertaken.intentions[0]),
        "afterwards it is frozen into the world, and no fork can reach it"
    );
}
