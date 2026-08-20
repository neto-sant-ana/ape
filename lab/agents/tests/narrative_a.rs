//! Phase 1 of experiment 03 — the checkable claims of narrative A, settled by running them.
//!
//! Migrated to the repository substrate. Every claim keeps the verdict it had, including the one
//! that is contradicted: the sorting does not move because the substrate changed, and if it had,
//! that would be the finding.

use ape::engine::hermeneia::{Conflict, Hypothesis, Outcome, movement_of};
use ape::engine::thesis::{ForkInput, Interpretation, Thesis};
use ape::kernel::axiom::Knowledge;

use ape_agents::hindsight::{self, Built, Step};
use ape_agents::world;

const EVERY_HYPOTHESIS: [Hypothesis; 3] = [
    Hypothesis::FinalState,
    Hypothesis::OnDueDateNet,
    Hypothesis::OnDueDateInAnyOrder,
];

/// Everything up to the first intention being opened as a world.
const WANTED: usize = 2;
/// Everything up to the world the house undertook.
const UNDERTAKEN: usize = 6;

fn upto(steps: usize) -> Built {
    hindsight::replay(&hindsight::scenario()[..steps])
}

fn current(built: &Built) -> Thesis {
    built.world_at(built.current())
}

fn conflicts(built: &Built, thesis: &Thesis, hypothesis: Hypothesis) -> Vec<Conflict> {
    Interpretation::of(thesis, built.canon.history())
        .expect("interpretable")
        .feasibility_under(hypothesis)
        .expect("derivable")
        .conflicts()
        .to_vec()
}

fn out_of_bounds(built: &Built, level: i128) -> Vec<Conflict> {
    vec![Conflict::OutOfBounds {
        instance: built.world.account,
        level,
    }]
}

/// > "It cannot be carried out ... every hypothesis the engine offers agrees on it."
#[test]
fn claim_the_wanted_arrangement_is_refused_under_every_hypothesis() {
    let built = upto(WANTED);
    let wanted = current(&built);

    for hypothesis in EVERY_HYPOTHESIS {
        assert_eq!(
            conflicts(&built, &wanted, hypothesis),
            out_of_bounds(&built, -20),
            "under {hypothesis:?}"
        );
    }
}

/// > "the standard slot, which leaves the account at 70 and against which nothing was found."
#[test]
fn claim_the_undertaken_arrangement_leaves_seventy_and_conflicts_with_nothing() {
    let built = upto(UNDERTAKEN);
    let undertaken = current(&built);
    let history = built.canon.history();

    for hypothesis in EVERY_HYPOTHESIS {
        assert!(conflicts(&built, &undertaken, hypothesis).is_empty());
    }

    let projected = Interpretation::of(&undertaken, history)
        .expect("interpretable")
        .conditions_at(&world::on(6))
        .expect("conditions project");

    let level: i128 = projected
        .conditions()
        .iter()
        .filter(|(_, condition)| condition.outcome() != &Outcome::Cancelled)
        .filter_map(|(id, _)| {
            let commitment = history.commitment(*id)?;
            movement_of(history, &commitment).ok().flatten()
        })
        .map(|movement| movement.magnitude())
        .sum();

    assert_eq!(
        level, 70,
        "100 received, 30 intended, the refused one cancelled"
    );
}

/// > "only one of them is selected by the world the house ends up in."
///
/// **Contradicted**, and the test asserts the contradiction rather than the claim.
#[test]
fn claim_only_one_candidate_is_selected_is_contradicted() {
    let built = upto(UNDERTAKEN);
    let undertaken = current(&built);
    let history = built.canon.history();

    let (wanted, taken) = (built.intentions[0], built.intentions[1]);

    assert!(history.commitment(wanted).is_some());
    assert!(history.commitment(taken).is_some());

    assert!(
        undertaken.selection().contains(wanted) && undertaken.selection().contains(taken),
        "the world selects both candidates, against what the narrative says"
    );

    assert!(undertaken.selection().is_frozen(wanted));
    assert!(!undertaken.selection().is_frozen(taken));
}

/// > "The money exists only because the cut recognizes the event that made it."
///
/// Supported in substance; the illustration offered for it is not constructible, and would show
/// nothing if it were.
#[test]
fn claim_the_money_is_a_movement_of_something_selected_not_a_stored_balance() {
    let built = upto(UNDERTAKEN);
    let history = built.canon.history();
    let undertaken = current(&built);

    assert!(
        undertaken.selection().is_frozen(built.world.opening),
        "the receipt is in the world because the cut recognizes the event that settled it"
    );

    let commitment = history.commitment(built.world.opening).expect("admitted");
    let movement = movement_of(history, &commitment)
        .expect("derivable")
        .expect("a quantifiable action moves a level");

    assert_eq!(movement.magnitude(), 100);

    let unsettled = upto(WANTED);
    assert_eq!(
        conflicts(&unsettled, &current(&unsettled), Hypothesis::FinalState),
        out_of_bounds(&unsettled, -20),
        "an unsettled intention moves the level exactly as a settled one does"
    );
}

/// > "Cancelling is what removes an intention's movement."
#[test]
fn claim_cancelling_is_what_removes_the_movement() {
    let steps: Vec<Step> = hindsight::scenario()[..UNDERTAKEN]
        .iter()
        .filter(|step| !matches!(step, Step::Cancel { .. } | Step::Carry { .. }))
        .cloned()
        .collect();

    let built = hindsight::replay(&steps);

    assert_eq!(
        conflicts(&built, &current(&built), Hypothesis::FinalState),
        out_of_bounds(&built, -50),
        "100 in, 120 and 30 out, if neither is cancelled"
    );
}

/// > "the omit-in-a-fork route was available up to the moment the event was admitted."
#[test]
fn claim_omitting_was_available_until_the_cancellation_was_admitted() {
    let built = upto(3);

    let omitted = current(&built).fork(
        built.canon.history(),
        ForkInput {
            omitted: [built.intentions[0]].into(),
            introduced: [built.intentions[1]].into(),
        },
    );

    assert!(
        omitted.is_ok(),
        "before the cancellation, the wanted intention is still open and can be dropped"
    );

    assert!(
        conflicts(&built, &omitted.unwrap(), Hypothesis::FinalState).is_empty(),
        "and dropping it leaves a world that conflicts with nothing"
    );
}

/// > "Any penalty above 70 makes the standard path also end below zero."
#[test]
fn claim_a_penalty_above_seventy_would_have_sunk_the_other_path_too() {
    let mut steps: Vec<Step> = hindsight::scenario()[..UNDERTAKEN].to_vec();
    steps.push(Step::Intend {
        magnitude: 71,
        incoming: false,
        due: 20,
        recorded_at: 6,
    });
    steps.push(Step::Add {
        omit: vec![],
        introduce: vec![2],
    });

    let built = hindsight::replay(&steps);

    assert_eq!(
        conflicts(&built, &current(&built), Hypothesis::FinalState),
        out_of_bounds(&built, -1),
        "70 left, 71 owed"
    );
}
