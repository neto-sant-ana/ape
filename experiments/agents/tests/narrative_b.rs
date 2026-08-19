//! Phase 3 of experiment 03 — the checkable claims of narrative B, settled by running them.
//!
//! Migrated to the repository substrate, every claim keeping its verdict. The four remedies are
//! still rebuilt from scratch rather than taken on trust, and they are now expressed as sequences
//! rather than as direct admissions, which is the same construction said in the record's terms.

use ape::canon::{CanonicalHistory, CanonicalKnowledge};
use ape::engine::hermeneia::{Conflict, Hypothesis, Outcome, Timeliness, movement_of};
use ape::engine::thesis::{ForkInput, Interpretation, KnowledgeCut, Thesis};
use ape::kernel::axiom::Knowledge;

use ape_agents::hindsight::{self, Built, Step};
use ape_agents::world;

const EVERY_HYPOTHESIS: [Hypothesis; 3] = [
    Hypothesis::FinalState,
    Hypothesis::OnDueDateNet,
    Hypothesis::OnDueDateInAnyOrder,
];

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

fn out_of_bounds(built: &Built, level: f64) -> Vec<Conflict> {
    vec![Conflict::OutOfBounds {
        instance: built.world.account,
        level,
    }]
}

fn lineage(built: &Built) -> Vec<Thesis> {
    let mut walked = Vec::new();
    let mut cursor = Some(built.current());

    while let Some(id) = cursor {
        let thesis = built.world_at(id);
        cursor = *thesis.parent();
        walked.push(thesis);
    }

    walked.reverse();
    walked
}

/// > "the account is at -20 ... All three give the same number."
#[test]
fn claim_the_breach_is_minus_twenty_under_all_three_ways_of_asking() {
    let built = hindsight::build();
    let world = current(&built);

    for hypothesis in EVERY_HYPOTHESIS {
        assert_eq!(
            conflicts(&built, &world, hypothesis),
            out_of_bounds(&built, -20.0)
        );
    }
}

/// > "There is 100 in the account ... Nothing is late."
#[test]
fn claim_the_hundred_is_intact_and_nothing_is_late() {
    let built = hindsight::build();
    let history = built.canon.history();

    let projected = Interpretation::of(&current(&built), history)
        .expect("interpretable")
        .conditions_at(&world::on(12))
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

    assert_eq!(
        settled, 100.0,
        "one receipt has landed and nothing has left"
    );

    for condition in projected.conditions().values() {
        assert_ne!(condition.timeliness(), Some(&Timeliness::Breached));
    }
}

/// > "the only two facts ever recorded against it."
#[test]
fn claim_exactly_two_facts_were_ever_recorded() {
    let built = hindsight::build();
    let history = built.canon.history();

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
/// **Supported.** The measure the narrative used is the right one for this sentence.
#[test]
fn claim_nothing_entered_the_plan_that_was_not_chosen() {
    let built = hindsight::build();

    for pair in lineage(&built).windows(2) {
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
/// **Not established by the measurement offered for it.** Something the house could do, it could
/// no longer do, and no imposed set reports it.
#[test]
fn claim_nothing_was_forced_is_not_what_the_empty_imposed_set_shows() {
    let before = hindsight::replay(&hindsight::scenario()[..3]);

    assert!(
        current(&before)
            .fork(
                before.canon.history(),
                ForkInput {
                    omitted: [before.intentions[0]].into(),
                    introduced: [].into(),
                },
            )
            .is_ok(),
        "before the withdrawal was recorded, the intention could be dropped"
    );

    let after = hindsight::build();

    assert!(
        current(&after)
            .fork(
                after.canon.history(),
                ForkInput {
                    omitted: [after.intentions[0]].into(),
                    introduced: [].into(),
                },
            )
            .is_err(),
        "afterwards the same option is gone, and no imposed set reports it"
    );
}

/// > "Had I brought the plan forward on the 9th, you would have had this conversation three days
/// > earlier."
#[test]
fn claim_the_warning_was_available_three_days_earlier() {
    let built = hindsight::build();
    let history = built.canon.history();

    let dated_the_sixth = built.world_at(built.worlds[2]);

    assert!(
        dated_the_sixth
            .fork(
                history,
                ForkInput {
                    omitted: [].into(),
                    introduced: [built.intentions[2]].into(),
                },
            )
            .is_err(),
        "a plan dated the 6th cannot select knowledge recorded on the 9th"
    );

    let on_the_ninth = dated_the_sixth
        .advance(history, KnowledgeCut::at(history, world::on(9)))
        .expect("the 9th recognizes knowledge the 6th did not")
        .into_thesis()
        .fork(
            history,
            ForkInput {
                omitted: [].into(),
                introduced: [built.intentions[2]].into(),
            },
        )
        .expect("brought forward, the plan takes the 90");

    assert_eq!(
        conflicts(&built, &on_the_ninth, Hypothesis::FinalState),
        out_of_bounds(&built, -20.0),
        "the same warning, three days earlier"
    );
}

/// > "All four clear the floor under all three ways of asking."
#[test]
fn claim_all_four_ways_out_clear_the_floor() {
    let exits: [(&str, Vec<Step>); 4] = [
        (
            "drop the 90",
            vec![Step::Add {
                omit: vec![2],
                introduce: vec![],
            }],
        ),
        (
            "drop the 30",
            vec![Step::Add {
                omit: vec![1],
                introduce: vec![],
            }],
        ),
        (
            "secure 20 in",
            vec![
                Step::Intend {
                    magnitude: 20.0,
                    incoming: true,
                    due: 19,
                    recorded_at: 12,
                },
                Step::Add {
                    omit: vec![],
                    introduce: vec![3],
                },
            ],
        ),
        (
            "reduce 90 to 70",
            vec![
                Step::Intend {
                    magnitude: 70.0,
                    incoming: false,
                    due: 20,
                    recorded_at: 12,
                },
                Step::Add {
                    omit: vec![2],
                    introduce: vec![3],
                },
            ],
        ),
    ];

    for (exit, tail) in exits {
        let mut steps = hindsight::scenario();
        steps.extend(tail);

        let built = hindsight::replay(&steps);

        for hypothesis in EVERY_HYPOTHESIS {
            assert!(
                conflicts(&built, &current(&built), hypothesis).is_empty(),
                "{exit} still breaches under {hypothesis:?}"
            );
        }
    }
}

/// > "20 is the minimum that works."
#[test]
fn claim_twenty_is_the_minimum_that_works() {
    let mut steps = hindsight::scenario();
    steps.extend([
        Step::Intend {
            magnitude: 19.0,
            incoming: true,
            due: 19,
            recorded_at: 12,
        },
        Step::Add {
            omit: vec![],
            introduce: vec![3],
        },
    ]);

    let built = hindsight::replay(&steps);

    assert_eq!(
        conflicts(&built, &current(&built), Hypothesis::FinalState),
        out_of_bounds(&built, -1.0),
        "19 leaves the account one short, so 20 is the least that clears"
    );
}
