//! What the world must be true of before an agent is let near it.
//!
//! A run only means something if the engine can tell the two options apart. These build both
//! options and check that it does — and they build them here, in the suite, precisely because
//! the world must not: constructing an option is the agent's job.
//!
//! Migrated to the repository substrate with every assertion's value and wording intact, which
//! is the discipline the CLI's divergence experiment established: a concluded experiment's
//! subject does not move, its harness does.

use ape::engine::hermeneia::{Conflict, Hypothesis};
use ape::engine::thesis::Interpretation;

use ape_agents::hindsight::{self, Built, Step};
use ape_agents::policy::{self, Verdict};

fn ruling(built: &Built) -> Verdict {
    let world = built.world_at(built.current());

    let interpretation =
        Interpretation::of(&world, built.canon.history()).expect("the Thesis is interpretable");

    policy::rule(&interpretation, Hypothesis::FinalState).expect("feasibility is derivable")
}

fn spending(magnitude: u128) -> Built {
    hindsight::replay(&[
        Step::Intend {
            magnitude,
            incoming: false,
            due: 10,
            recorded_at: 6,
        },
        Step::Open {
            known_at: 6,
            select: vec![0],
        },
    ])
}

/// Spending 30 leaves 70 and spending 120 leaves −20, and only the second breaks the floor.
#[test]
fn the_engine_tells_the_two_options_apart() {
    assert_eq!(ruling(&spending(30)), Verdict::MayProceed);

    let reckless = spending(120);

    assert_eq!(
        ruling(&reckless),
        Verdict::Refused(vec![Conflict::OutOfBounds {
            instance: reckless.world.account,
            level: -20,
        }]),
        "spending 120 against a balance of 100 leaves the account below its floor"
    );
}

/// The settled past is imposed by the cut, not chosen by the caller.
#[test]
fn the_settled_past_arrives_without_being_selected() {
    let built = spending(30);
    let world = built.world_at(built.current());

    assert_eq!(
        world.selection().len(),
        2,
        "the opening arrives in the selection without being asked for"
    );

    assert!(world.selection().is_frozen(built.world.opening));
    assert!(!world.selection().is_frozen(built.intentions[0]));
}
