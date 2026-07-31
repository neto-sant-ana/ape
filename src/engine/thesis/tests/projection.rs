//! That a derived Thesis is interpretable at the head its cut recognizes, and only there.

use std::collections::BTreeSet;

use super::{Fixture, ForkInput, d1, d2, ids, introducing};

use crate::engine::hermeneia::{Accumulation, Outcome, ProjectionError};
use crate::engine::thesis::Interpretation;

use crate::kernel::value_objects::Date;

fn at(month: u8, day: u8) -> Date {
    Date::from_ymd(2026, month, day).unwrap()
}

#[test]
fn every_derivation_interprets_the_chain_its_cut_recognizes() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let open = knowledge.commit((6, 30), BTreeSet::new());
    let introduced = knowledge.commit((9, 30), BTreeSet::new());

    let genesis = knowledge.genesis(knowledge.cut(d1(), None), &[settled, open]);
    let conditions = Interpretation::of(&genesis, &knowledge)
        .unwrap()
        .conditions_at(&at(4, 1))
        .unwrap();
    assert_eq!(
        conditions.condition(settled).unwrap().outcome(),
        &Outcome::Unsettled
    );

    let fork = genesis
        .fork(&knowledge, introducing(&[introduced]))
        .unwrap();
    assert!(Interpretation::of(&fork, &knowledge).is_ok());

    let head = knowledge.settle(settled);
    let advanced = fork
        .advance(&knowledge, knowledge.cut(d2(), Some(head)))
        .unwrap()
        .into_thesis();

    let conditions = Interpretation::of(&advanced, &knowledge)
        .unwrap()
        .conditions_at(&at(4, 1))
        .unwrap();

    assert_eq!(conditions.event_head(), Some(head));
    assert_eq!(
        conditions.condition(settled).unwrap().outcome(),
        &Outcome::Fulfilled
    );
}

#[test]
fn a_thesis_interprets_a_settlement_it_never_intended() {
    let mut knowledge = Fixture::new();
    let replaced = knowledge.commit((3, 31), BTreeSet::new());
    let replacement = knowledge.commit((4, 30), BTreeSet::new());

    let genesis = knowledge.genesis(knowledge.cut(d1(), None), &[replaced]);
    let revised = genesis
        .fork(
            &knowledge,
            ForkInput {
                omitted: ids(&[replaced]),
                introduced: ids(&[replacement]),
            },
        )
        .unwrap();

    let head = knowledge.settle(replaced);
    let advanced = revised
        .advance(&knowledge, knowledge.cut(d2(), Some(head)))
        .unwrap()
        .into_thesis();

    let conditions = Interpretation::of(&advanced, &knowledge)
        .unwrap()
        .conditions_at(&at(4, 1))
        .unwrap();

    assert_eq!(
        conditions.condition(replaced).unwrap().outcome(),
        &Outcome::Fulfilled
    );
    assert_eq!(
        conditions.condition(replacement).unwrap().outcome(),
        &Outcome::Unsettled
    );
}

#[test]
fn the_ancestry_answers_what_was_intended_earlier() {
    let mut knowledge = Fixture::new();
    let commitment = knowledge.commit((3, 31), BTreeSet::new());

    let before = knowledge.genesis(knowledge.cut(d1(), None), &[commitment]);
    let head = knowledge.settle(commitment);
    let after = before
        .advance(&knowledge, knowledge.cut(d2(), Some(head)))
        .unwrap()
        .into_thesis();

    let earlier = Interpretation::of(&before, &knowledge)
        .unwrap()
        .conditions_at(&at(3, 1))
        .unwrap();
    let later = Interpretation::of(&after, &knowledge)
        .unwrap()
        .conditions_at(&at(3, 1))
        .unwrap();

    assert_eq!(
        earlier.condition(commitment).unwrap().outcome(),
        &Outcome::Unsettled
    );
    assert_eq!(
        later.condition(commitment).unwrap().outcome(),
        &Outcome::Fulfilled
    );
}

/// The chain the cut recognizes is resolved from the cut, so an interpretation of a Thesis
/// recognizing no Event folds no Event — whatever canonical history has recorded since.
#[test]
fn a_thesis_recognizing_no_event_interprets_none() {
    let mut knowledge = Fixture::new();
    let commitment = knowledge.commit((3, 31), BTreeSet::new());

    let thesis = knowledge.genesis(knowledge.cut(d1(), None), &[commitment]);
    knowledge.settle(commitment);

    let conditions = Interpretation::of(&thesis, &knowledge)
        .unwrap()
        .conditions_at(&at(3, 1))
        .unwrap();

    assert_eq!(conditions.event_head(), None);
    assert_eq!(
        conditions.condition(commitment).unwrap().outcome(),
        &Outcome::Unsettled
    );
}

/// The boundary from below: a bound accumulation refuses an event past the head it recognizes.
#[test]
fn an_event_beyond_the_recognized_head_is_refused() {
    let mut knowledge = Fixture::new();
    let commitment = knowledge.commit((3, 31), BTreeSet::new());
    let head = knowledge.settle(commitment);

    let mut accumulation = Accumulation::recognizing(None);
    let refusal = accumulation.absorb(
        &knowledge,
        &[commitment],
        &knowledge.chain_through(Some(head)),
    );

    assert!(matches!(
        refusal,
        Err(ProjectionError::EventBeyondRecognizedHead { event, recognized })
            if event == head && recognized.is_none()
    ));
}

/// And from above: a chain that stops short looks complete from the inside, so the refusal
/// comes when the question is asked.
#[test]
fn a_chain_short_of_the_recognized_head_interprets_nothing() {
    let mut knowledge = Fixture::new();
    let first = knowledge.commit((3, 31), BTreeSet::new());
    let second = knowledge.commit((4, 30), BTreeSet::new());
    let reached = knowledge.settle(first);
    let recognized = knowledge.settle(second);

    let mut accumulation = Accumulation::recognizing(Some(recognized));
    accumulation
        .absorb(
            &knowledge,
            &[first, second],
            &knowledge.chain_through(Some(reached)),
        )
        .unwrap();

    assert!(matches!(
        accumulation.conditions_at(&at(4, 1)),
        Err(ProjectionError::RecognizedChainIncomplete { reached: stopped, recognized: expected })
            if stopped == Some(reached) && expected == Some(recognized)
    ));
}
