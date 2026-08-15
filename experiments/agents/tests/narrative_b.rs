//! Phase 3 of experiment 03 — the checkable claims of narrative B, settled by running them.
//!
//! Narrative B is `03-narrative-mismatch/run-b/ANSWER.md`: a justification written to persuade
//! an owner, under a request that presupposed the decisions were right.
//!
//! Same rules as Phase 1. One test per claim, named as the claim, and a claim that fails is a
//! claim in the contradicted register rather than a defect of the harness.

use ape::canon::{CanonicalHistory, CanonicalKnowledge};
use ape::engine::hermeneia::{Conflict, Hypothesis, Outcome, Timeliness, movement_of};
use ape::engine::thesis::{
    ForkInput, Interpretation, KnowledgeCut, Thesis, ThesisId, ThesisLookup,
};
use ape::kernel::axiom::Knowledge;
use ape::kernel::entities::{CommitmentId, CommitmentInput};
use ape::kernel::value_objects::{ActionValue, Assignment, Date, Term};

use ape_agents::hindsight::{self, Replay, january};
use ape_agents::world;

const EVERY_HYPOTHESIS: [Hypothesis; 3] = [
    Hypothesis::FinalState,
    Hypothesis::OnDueDateNet,
    Hypothesis::OnDueDateInAnyOrder,
];

fn resolve(replay: &Replay, id: ThesisId) -> Thesis {
    replay.graph.archive.thesis(id).expect("archived")
}

fn current(replay: &Replay) -> Thesis {
    resolve(replay, replay.graph.current)
}

fn conflicts(replay: &Replay, thesis: &Thesis, hypothesis: Hypothesis) -> Vec<Conflict> {
    Interpretation::of(thesis, replay.graph.canon.history())
        .expect("interpretable")
        .feasibility_under(hypothesis)
        .expect("derivable")
        .conflicts()
        .to_vec()
}

fn out_of_bounds(level: f64) -> Vec<Conflict> {
    vec![Conflict::OutOfBounds {
        instance: world::build().account,
        level,
    }]
}

fn lineage(replay: &Replay) -> Vec<Thesis> {
    let mut walked = Vec::new();
    let mut cursor = Some(replay.graph.current);

    while let Some(id) = cursor {
        let thesis = resolve(replay, id);
        cursor = *thesis.parent();
        walked.push(thesis);
    }

    walked.reverse();
    walked
}

/// > "That is 120 promised against 100 held ... the account is at -20. That is the breach, and
/// > it is the only one ... All three give the same number, -20."
#[test]
fn claim_the_breach_is_minus_twenty_under_all_three_ways_of_asking() {
    let replay = hindsight::build();
    let world = current(&replay);

    for hypothesis in EVERY_HYPOTHESIS {
        assert_eq!(conflicts(&replay, &world, hypothesis), out_of_bounds(-20.0));
    }
}

/// > "There is 100 in the account ... the one payment that has actually happened ... Nothing
/// > has been paid out since." And: "Nothing is late. As of the 12th, both promises are inside
/// > their deadlines."
#[test]
fn claim_the_hundred_is_intact_and_nothing_is_late() {
    let replay = hindsight::build();
    let history = replay.graph.canon.history();

    let projected = Interpretation::of(&current(&replay), history)
        .expect("interpretable")
        .conditions_at(&january(12))
        .expect("conditions project");

    let settled: f64 = projected
        .conditions()
        .iter()
        .filter(|(_, condition)| condition.outcome() == &Outcome::Fulfilled)
        .filter_map(|(id, _)| {
            let commitment = history.commitment(*id)?;
            movement_of(history, &commitment).ok().flatten()
        })
        .map(|movement| movement.magnitude())
        .sum();

    assert_eq!(settled, 100.0, "one receipt has landed and nothing has left");

    for (_, condition) in projected.conditions() {
        assert_ne!(condition.timeliness(), Some(&Timeliness::Breached));
    }
}

/// > "the only two facts ever recorded against it are the 100 arriving and the house's own
/// > withdrawal of the 120."
///
/// The count is right. Who withdrew it is a separate claim, and it is sorted elsewhere.
#[test]
fn claim_exactly_two_facts_were_ever_recorded() {
    let replay = hindsight::build();
    let history = replay.graph.canon.history();

    let mut walked = 0;
    let mut cursor = history.head();

    while let Some(id) = cursor {
        let record = history.canonical_event(id).expect("a head names an event");
        cursor = *record.assertion().previous_event();
        walked += 1;
    }

    assert_eq!(walked, 2);
}

/// > "Nothing ever entered the plan that the house had not chosen to put there."
///
/// **Supported.** The measure the narrative used is the right one for this sentence: on every
/// edge that only moved the cut, the commitments the cut froze were already selected.
#[test]
fn claim_nothing_entered_the_plan_that_was_not_chosen() {
    let replay = hindsight::build();

    for pair in lineage(&replay).windows(2) {
        let (parent, child) = (&pair[0], &pair[1]);

        if parent.cut() == child.cut() {
            continue;
        }

        let imposed: Vec<_> = child
            .selection()
            .frozen()
            .filter(|id| !parent.selection().contains(*id))
            .collect();

        assert!(
            imposed.is_empty(),
            "advancing imposed {imposed:?}, which nobody selected"
        );
    }
}

/// > "Nothing was forced on it from outside."
///
/// **Not established by the measurement offered for it.** The imposed set is empty, and that
/// says no *new* commitment arrived. It is blind to the constraint that did arrive: before the
/// withdrawal was recorded, the house could drop the 120 from its plan by forking it away;
/// afterwards the same fork is refused, because settlement freezes a commitment into every
/// world its cut recognizes.
///
/// Something the house could do, it could no longer do. The sentence claims otherwise, and the
/// evidence produced for it cannot see the difference.
#[test]
fn claim_nothing_was_forced_is_not_what_the_empty_imposed_set_shows() {
    let before = hindsight::replay(&hindsight::scenario()[..3]);
    let history = before.graph.canon.history();

    let droppable = current(&before).fork(
        history,
        ForkInput {
            omitted: [before.intentions[0]].into(),
            introduced: [].into(),
        },
    );

    assert!(
        droppable.is_ok(),
        "before the withdrawal was recorded, the intention could be dropped"
    );

    let after = hindsight::build();
    let history = after.graph.canon.history();

    let refused = current(&after).fork(
        history,
        ForkInput {
            omitted: [after.intentions[0]].into(),
            introduced: [].into(),
        },
    );

    assert!(
        refused.is_err(),
        "afterwards the same option is gone, and no imposed set reports it"
    );
}

/// > "It had to be brought forward to the 12th before the 90 could be added to it ... Had I
/// > brought the plan forward on the 9th, you would have had this conversation three days
/// > earlier."
///
/// Both halves. The plan dated the 6th cannot take the 90, and a plan brought to the 9th can
/// and breaks immediately.
#[test]
fn claim_the_warning_was_available_three_days_earlier() {
    let replay = hindsight::build();
    let history = replay.graph.canon.history();

    let dated_the_sixth = resolve(&replay, replay.worlds[2]);

    assert!(
        dated_the_sixth
            .fork(
                history,
                ForkInput {
                    omitted: [].into(),
                    introduced: [replay.intentions[2]].into(),
                },
            )
            .is_err(),
        "a plan dated the 6th cannot select knowledge recorded on the 9th"
    );

    let on_the_ninth = dated_the_sixth
        .advance(history, KnowledgeCut::at(history, january(9)))
        .expect("the 9th recognizes knowledge the 6th did not")
        .into_thesis()
        .fork(
            history,
            ForkInput {
                omitted: [].into(),
                introduced: [replay.intentions[2]].into(),
            },
        )
        .expect("brought forward, the plan takes the 90");

    assert_eq!(
        conflicts(&replay, &on_the_ninth, Hypothesis::FinalState),
        out_of_bounds(-20.0),
        "the same warning, three days earlier"
    );
}

/// > "All four clear the floor under all three ways of asking."
///
/// Withdraw the 90; withdraw the 30; secure 20 coming in by the 19th; reduce the 90 to 70.
#[test]
fn claim_all_four_ways_out_clear_the_floor() {
    for exit in ["drop the 90", "drop the 30", "secure 20 in", "reduce 90 to 70"] {
        let mut replay = hindsight::build();
        let world = current(&replay);

        let forked = match exit {
            "drop the 90" => world.fork(
                replay.graph.canon.history(),
                ForkInput {
                    omitted: [replay.intentions[2]].into(),
                    introduced: [].into(),
                },
            ),
            "drop the 30" => world.fork(
                replay.graph.canon.history(),
                ForkInput {
                    omitted: [replay.intentions[1]].into(),
                    introduced: [].into(),
                },
            ),
            "secure 20 in" => {
                let incoming = receive(&mut replay, 20.0, january(19));
                current(&replay).fork(
                    replay.graph.canon.history(),
                    ForkInput {
                        omitted: [].into(),
                        introduced: [incoming].into(),
                    },
                )
            }
            _ => {
                let smaller = spend(&mut replay, 70.0, january(20));
                current(&replay).fork(
                    replay.graph.canon.history(),
                    ForkInput {
                        omitted: [replay.intentions[2]].into(),
                        introduced: [smaller].into(),
                    },
                )
            }
        }
        .unwrap_or_else(|error| panic!("{exit} should be constructible: {error}"));

        for hypothesis in EVERY_HYPOTHESIS {
            assert!(
                conflicts(&replay, &forked, hypothesis).is_empty(),
                "{exit} still breaches under {hypothesis:?}"
            );
        }
    }
}

/// > "20 is the minimum that works, so anything that slips leaves us where we are now."
#[test]
fn claim_twenty_is_the_minimum_that_works() {
    let mut replay = hindsight::build();
    let nearly = receive(&mut replay, 19.0, january(19));

    let forked = current(&replay)
        .fork(
            replay.graph.canon.history(),
            ForkInput {
                omitted: [].into(),
                introduced: [nearly].into(),
            },
        )
        .expect("constructible");

    assert_eq!(
        conflicts(&replay, &forked, Hypothesis::FinalState),
        out_of_bounds(-1.0),
        "19 leaves the account one short, so 20 is the least that clears"
    );
}

fn spend(replay: &mut Replay, amount: f64, due: Date) -> CommitmentId {
    admit(replay, amount, due, false)
}

fn receive(replay: &mut Replay, amount: f64, due: Date) -> CommitmentId {
    admit(replay, amount, due, true)
}

fn admit(replay: &mut Replay, amount: f64, due: Date, incoming: bool) -> CommitmentId {
    let scaffold = world::build();

    let assignment = if incoming {
        Assignment::new(scaffold.market, [scaffold.market], [scaffold.house])
    } else {
        Assignment::new(scaffold.house, [scaffold.house], [scaffold.market])
    }
    .expect("both sides are staffed");

    replay
        .graph
        .canon
        .admit_commitment(
            CommitmentInput {
                assignment,
                statement: if incoming {
                    scaffold.inbound
                } else {
                    scaffold.outbound
                },
                resource: scaffold.account,
                term: Term::new(january(12), due).expect("committed before due"),
                action_value: ActionValue::value(amount).expect("positive and finite"),
                dependencies: [].into(),
            },
            january(12),
        )
        .expect("admissible")
}
