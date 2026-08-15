//! Phase 1 of experiment 03 — the checkable claims of narrative A, settled by running them.
//!
//! Narrative A is `01-single-agent/run-01/ANSWER.md`, written by an agent that did not know it
//! would ever be checked. Each test below is one claim from it, named as the claim, and
//! passing means the graph settles it in the narrative's favour.
//!
//! A claim that fails here is not a defect of the harness. It is a claim in the *contradicted*
//! register, and finding one would refute this experiment's first prediction.
//!
//! The world is addressed by prefixes of the recorded sequence, so each claim is checked
//! against the world as it stood when the claim was made rather than against the end of it.

use ape::engine::hermeneia::{Conflict, Hypothesis, Outcome, movement_of};
use ape::engine::thesis::{ForkInput, Interpretation, Thesis};
use ape::kernel::axiom::Knowledge;

use ape_agents::hindsight::{self, Replay, Step, january};
use ape_agents::world;

/// Everything up to and including the first intention being opened as a world.
const WANTED: usize = 2;
/// Everything up to and including the world the house undertook.
const UNDERTAKEN: usize = 6;

fn upto(steps: usize) -> Replay {
    hindsight::replay(&hindsight::scenario()[..steps])
}

fn conflicts(replay: &Replay, thesis: &Thesis, hypothesis: Hypothesis) -> Vec<Conflict> {
    Interpretation::of(thesis, replay.graph.canon.history())
        .expect("the Thesis is interpretable")
        .feasibility_under(hypothesis)
        .expect("feasibility is derivable")
        .conflicts()
        .to_vec()
}

fn current(replay: &Replay) -> Thesis {
    use ape::engine::thesis::ThesisLookup;

    replay
        .graph
        .archive
        .thesis(replay.graph.current)
        .expect("the archive holds the world it reports as current")
}

const EVERY_HYPOTHESIS: [Hypothesis; 3] = [
    Hypothesis::FinalState,
    Hypothesis::OnDueDateNet,
    Hypothesis::OnDueDateInAnyOrder,
];

/// > "It cannot be carried out. The account holds 100 ... and spending 120 puts it at −20 ...
/// > every hypothesis the engine offers agrees on it."
#[test]
fn claim_the_wanted_arrangement_is_refused_under_every_hypothesis() {
    let replay = upto(WANTED);
    let wanted = current(&replay);

    for hypothesis in EVERY_HYPOTHESIS {
        assert_eq!(
            conflicts(&replay, &wanted, hypothesis),
            vec![Conflict::OutOfBounds {
                instance: world::build().account,
                level: -20.0,
            }],
            "under {hypothesis:?}"
        );
    }
}

/// > "the standard slot, which leaves the account at 70 and against which nothing was found."
///
/// Both halves are checked, and the 70 is folded from the engine's own arithmetic rather than
/// from the narrative's.
#[test]
fn claim_the_undertaken_arrangement_leaves_seventy_and_conflicts_with_nothing() {
    let replay = upto(UNDERTAKEN);
    let undertaken = current(&replay);
    let history = replay.graph.canon.history();

    for hypothesis in EVERY_HYPOTHESIS {
        assert!(conflicts(&replay, &undertaken, hypothesis).is_empty());
    }

    let projected = Interpretation::of(&undertaken, history)
        .expect("interpretable")
        .conditions_at(&january(6))
        .expect("conditions project");

    let level: f64 = projected
        .conditions()
        .iter()
        .filter(|(_, condition)| condition.outcome() != &Outcome::Cancelled)
        .filter_map(|(id, _)| {
            let commitment = history.commitment(*id)?;
            movement_of(history, &commitment).ok().flatten()
        })
        .map(|movement| movement.magnitude())
        .sum();

    assert_eq!(level, 70.0, "100 received, 30 intended, the refused one cancelled");
}

/// > "The two candidate commitments coexist in canonical history, and only one of them is
/// > selected by the world the house ends up in."
///
/// **Contradicted.** The first half holds. The second does not: cancelling the refused
/// candidate settled it, and what a cut settles it makes unavoidable, so that world selects
/// both — the refused one frozen, the taken one open.
///
/// The claim reads as a description of the engine's division between knowledge and world, and
/// the division is real. What the narrative got wrong is which side its own cancellation put
/// the candidate on: it did not remove the intention from the world, it froze it there without
/// a movement.
///
/// This test asserts the contradiction rather than the claim, because the contradiction is the
/// finding. It refutes this experiment's first prediction.
#[test]
fn claim_only_one_candidate_is_selected_is_contradicted() {
    let replay = upto(UNDERTAKEN);
    let undertaken = current(&replay);
    let history = replay.graph.canon.history();

    let wanted = replay.intentions[0];
    let taken = replay.intentions[1];

    assert!(history.commitment(wanted).is_some());
    assert!(history.commitment(taken).is_some());

    assert!(
        undertaken.selection().contains(wanted) && undertaken.selection().contains(taken),
        "the world selects both candidates, against what the narrative says"
    );

    assert!(undertaken.selection().is_frozen(wanted));
    assert!(!undertaken.selection().is_frozen(taken));
}

/// > "The money exists only because the cut recognizes the event that made it. ... A cut taken
/// > before that event would make every spend look out of bounds."
///
/// The substance holds: the 100 is a movement of a selected commitment and not a stored
/// balance. The illustration does not, and both halves are recorded.
///
/// A cut before the settling event cannot be built here at all — the spending was recorded
/// after it, and the recording watermark refuses knowledge back-dated into an instant already
/// closed. So the counterfactual the narrative offers as proof cannot be instantiated in the
/// world it describes.
///
/// And it would not show what it claims if it could. Under `FinalState` an unsettled
/// commitment contributes its movement too, so what the event supplies is not the money but
/// the *automatic* selection of what carries it: settlement freezes the opening into every
/// world the cut recognizes, and without it the opening would merely have to be selected on
/// purpose.
#[test]
fn claim_the_money_is_a_movement_of_something_selected_not_a_stored_balance() {
    let replay = upto(UNDERTAKEN);
    let history = replay.graph.canon.history();
    let undertaken = current(&replay);

    let opening = world::build().opening;

    assert!(
        undertaken.selection().is_frozen(opening),
        "the receipt is in the world because the cut recognizes the event that settled it"
    );

    let commitment = history.commitment(opening).expect("admitted");
    let movement = movement_of(history, &commitment)
        .expect("derivable")
        .expect("a quantifiable action moves a level");

    assert_eq!(movement.magnitude(), 100.0);

    // The overreach, made concrete: nothing about the money depends on settlement. A world
    // that selects an unsettled spend of the same size reads the same way.
    let mut steps: Vec<Step> = hindsight::scenario()[..1].to_vec();
    steps.push(Step::Open {
        known_at: january(6),
        select: vec![0],
    });

    let unsettled = hindsight::replay(&steps);
    assert_eq!(
        conflicts(&unsettled, &current(&unsettled), Hypothesis::FinalState),
        vec![Conflict::OutOfBounds {
            instance: world::build().account,
            level: -20.0,
        }],
        "an unsettled intention moves the level exactly as a settled one does"
    );
}

/// > "Cancelling is what removes an intention's movement; omitting is what removes the
/// > intention."
///
/// Checked by not cancelling: the same two intentions, both live, sink the account.
#[test]
fn claim_cancelling_is_what_removes_the_movement() {
    // The same world without the cancellation. The advancement goes with it: carrying a world
    // to a cut it already recognizes is refused, and with nothing observed there is nothing to
    // recognize.
    let steps: Vec<Step> = hindsight::scenario()[..UNDERTAKEN]
        .iter()
        .filter(|step| !matches!(step, Step::Cancel { .. } | Step::Carry { .. }))
        .cloned()
        .collect();

    let replay = hindsight::replay(&steps);
    let both = current(&replay);

    assert_eq!(
        conflicts(&replay, &both, Hypothesis::FinalState),
        vec![Conflict::OutOfBounds {
            instance: world::build().account,
            level: -50.0,
        }],
        "100 in, 120 and 30 out, if neither is cancelled"
    );
}

/// > "the omit-in-a-fork route was available up to the moment the event was admitted."
#[test]
fn claim_omitting_was_available_until_the_cancellation_was_admitted() {
    let replay = upto(3);
    let history = replay.graph.canon.history();
    let opened = current(&replay);

    let omitted = opened.fork(
        history,
        ForkInput {
            omitted: [replay.intentions[0]].into(),
            introduced: [replay.intentions[1]].into(),
        },
    );

    assert!(
        omitted.is_ok(),
        "before the cancellation, the wanted intention is still open and can be dropped"
    );

    assert!(
        conflicts(&replay, &omitted.unwrap(), Hypothesis::FinalState).is_empty(),
        "and dropping it leaves a world that conflicts with nothing"
    );
}

/// > "Any penalty above 70 makes the standard path also end below zero."
#[test]
fn claim_a_penalty_above_seventy_would_have_sunk_the_other_path_too() {
    let mut steps: Vec<Step> = hindsight::scenario()[..UNDERTAKEN].to_vec();
    steps.push(Step::Intend {
        amount: 71.0,
        due: january(20),
        recorded_at: january(6),
    });
    steps.push(Step::Add { introduce: vec![2] });

    let replay = hindsight::replay(&steps);
    let with_penalty = current(&replay);

    assert_eq!(
        conflicts(&replay, &with_penalty, Hypothesis::FinalState),
        vec![Conflict::OutOfBounds {
            instance: world::build().account,
            level: -1.0,
        }],
        "70 left, 71 owed"
    );
}
