//! That a derived Thesis is interpretable at the head its cut recognizes, whatever derived it.

use std::collections::BTreeSet;

use super::{Fixture, ForkInput, d1, d2, ids, introducing};

use crate::engine::hermeneia::{Accumulation, Outcome};
use crate::engine::thesis::Thesis;

use crate::kernel::value_objects::Date;

fn project(knowledge: &Fixture, thesis: &Thesis) -> Accumulation {
    let head = thesis.cut().event_head();
    let mut accumulation = Accumulation::default();

    accumulation
        .absorb(
            knowledge,
            &thesis.selection(),
            &knowledge.chain_through(head),
        )
        .unwrap();

    assert_eq!(accumulation.event_head(), head);

    accumulation
}

#[test]
fn every_derivation_absorbs_the_chain_its_cut_recognizes() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let open = knowledge.commit((6, 30), BTreeSet::new());
    let introduced = knowledge.commit((9, 30), BTreeSet::new());

    let genesis = knowledge.genesis(knowledge.cut(d1(), None), &[settled, open]);
    project(&knowledge, &genesis);

    let fork = genesis
        .fork(&knowledge, introducing(&[introduced]))
        .unwrap();
    project(&knowledge, &fork);

    let head = knowledge.settle(settled);
    let advanced = fork
        .advance(&knowledge, knowledge.cut(d2(), Some(head)))
        .unwrap()
        .into_thesis();
    let accumulation = project(&knowledge, &advanced);

    let conditions = accumulation
        .conditions_at(&Date::from_ymd(2026, 4, 1).unwrap())
        .unwrap();

    assert_eq!(
        conditions.condition(settled).unwrap().outcome(),
        &Outcome::Fulfilled
    );
}

#[test]
fn a_thesis_absorbs_a_settlement_it_never_intended() {
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

    let accumulation = project(&knowledge, &advanced);
    let conditions = accumulation
        .conditions_at(&Date::from_ymd(2026, 4, 1).unwrap())
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

    let at = Date::from_ymd(2026, 3, 1).unwrap();

    let earlier = project(&knowledge, &before).conditions_at(&at).unwrap();
    let later = project(&knowledge, &after).conditions_at(&at).unwrap();

    assert_eq!(
        earlier.condition(commitment).unwrap().outcome(),
        &Outcome::Unsettled
    );
    assert_eq!(
        later.condition(commitment).unwrap().outcome(),
        &Outcome::Fulfilled
    );
}
