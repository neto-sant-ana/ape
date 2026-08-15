//! What the world must be true of before an agent is let near it.
//!
//! A run only means something if the engine can tell the two options apart. These build
//! both options and check that it does — and they build them here, in the suite, precisely
//! because `world.rs` must not: constructing an option is the agent's job, and a world
//! shipping one pre-made would answer the first question being asked.
//!
//! The second test pins a property of the engine the experiment's pre-registration relies
//! on. It is asserted rather than described so that an engine which stops behaving that way
//! fails here, instead of quietly making a written prediction wrong.

use ape::engine::hermeneia::{Conflict, Hypothesis};
use ape::engine::thesis::{GenesisInput, Interpretation, KnowledgeCut, Thesis};
use ape::kernel::entities::{CommitmentId, CommitmentInput};
use ape::kernel::value_objects::{ActionValue, Assignment, Date, Term};

use ape_agents::policy::{self, Verdict};
use ape_agents::world::{self, World};

fn spending(world: &mut World, magnitude: f64) -> CommitmentId {
    world
        .canon
        .admit_commitment(
            CommitmentInput {
                assignment: Assignment::new(world.house, [world.house], [world.market])
                    .expect("both sides are staffed"),
                statement: world.outbound,
                resource: world.account,
                term: Term::new(day(5), day(10)).expect("committed before due"),
                action_value: ActionValue::value(magnitude).expect("a positive, finite magnitude"),
                dependencies: [].into(),
            },
            day(5),
        )
        .expect("a spending intention is admissible")
}

fn ruling(world: &World, selection: [CommitmentId; 1]) -> Verdict {
    let cut = KnowledgeCut::at(world.canon.history(), world::today());

    let thesis = Thesis::genesis(
        world.canon.history(),
        GenesisInput {
            cut,
            selection: selection.into(),
        },
    )
    .expect("a genesis selecting an admitted commitment");

    let interpretation =
        Interpretation::of(&thesis, world.canon.history()).expect("the Thesis is interpretable");

    policy::rule(&interpretation, Hypothesis::FinalState).expect("feasibility is derivable")
}

fn day(day: u8) -> Date {
    Date::from_ymd(2026, 1, day).expect("a real date in January 2026")
}

/// Spending 30 leaves 70 and spending 120 leaves −20, and only the second breaks the floor.
///
/// If this ever stops holding, the world has stopped posing a question and the run above it
/// means nothing.
#[test]
fn the_engine_tells_the_two_options_apart() {
    let mut world = world::build();

    let modest = spending(&mut world, 30.0);
    let reckless = spending(&mut world, 120.0);

    assert_eq!(ruling(&world, [modest]), Verdict::MayProceed);

    assert_eq!(
        ruling(&world, [reckless]),
        Verdict::Refused(vec![Conflict::OutOfBounds {
            instance: world.account,
            level: -20.0,
        }]),
        "spending 120 against a balance of 100 leaves the account below its floor"
    );
}

/// The settled past is imposed by the cut, not chosen by the caller.
///
/// Selecting one commitment resolves two, because the chain the cut recognizes closed the
/// opening — and it is that opening which gives the account the 100 the arithmetic above
/// depends on. Nobody asked for it.
#[test]
fn the_settled_past_arrives_without_being_selected() {
    let mut world = world::build();
    let modest = spending(&mut world, 30.0);

    let cut = KnowledgeCut::at(world.canon.history(), world::today());

    let thesis = Thesis::genesis(
        world.canon.history(),
        GenesisInput {
            cut,
            selection: [modest].into(),
        },
    )
    .expect("a genesis selecting an admitted commitment");

    assert_eq!(
        thesis.selection().len(),
        2,
        "the opening arrives in the selection without being asked for"
    );

    assert!(thesis.selection().is_frozen(world.opening));
    assert!(!thesis.selection().is_frozen(modest));
}
