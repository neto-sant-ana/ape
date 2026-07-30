//! What recognizing later history does to an intention.

use std::collections::BTreeSet;

use super::{Fixture, ForkInput, ids, omitting};

use crate::engine::thesis::ThesisError;

#[test]
fn advancing_to_the_recognized_head_is_refused() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let head = knowledge.settle(settled);
    let thesis = knowledge.genesis(Some(head), &[]);

    let refusal = thesis.advance(&knowledge, head);

    assert!(matches!(refusal, Err(ThesisError::AlreadyAtHead(id)) if id == head));
}

#[test]
fn advancing_to_a_head_of_another_reach_of_history_is_refused() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let elsewhere = knowledge.commit((6, 30), BTreeSet::new());
    let head = knowledge.settle(settled);
    let detached = knowledge.detached(elsewhere);
    let thesis = knowledge.genesis(Some(head), &[]);

    let refusal = thesis.advance(&knowledge, detached);

    assert!(matches!(
        refusal,
        Err(ThesisError::HeadDoesNotDescend { parent, target })
            if parent == Some(head) && target == detached
    ));
}

#[test]
fn a_selected_commitment_moves_from_the_open_future_into_the_frozen_past() {
    let mut knowledge = Fixture::new();
    let intended = knowledge.commit((3, 31), BTreeSet::new());
    let thesis = knowledge.genesis(None, &[intended]);
    let head = knowledge.settle(intended);

    let advancement = thesis.advance(&knowledge, head).unwrap();
    let advanced = advancement.thesis();

    assert!(advancement.imposed().is_empty());
    assert_eq!(advanced.frozen(), &ids(&[intended]));
    assert!(advanced.open().is_empty());
    assert_eq!(advanced.head(), &Some(head));
}

#[test]
fn what_history_settled_outside_the_parent_is_reported_as_imposed() {
    let mut knowledge = Fixture::new();
    let replaced = knowledge.commit((3, 31), BTreeSet::new());
    let replacement = knowledge.commit((4, 30), BTreeSet::new());

    let thesis = knowledge.genesis(None, &[replaced]);
    let revised = thesis
        .fork(
            &knowledge,
            ForkInput {
                omitted: ids(&[replaced]),
                introduced: ids(&[replacement]),
            },
        )
        .unwrap();

    let head = knowledge.settle(replaced);
    let advancement = revised.advance(&knowledge, head).unwrap();

    assert_eq!(advancement.imposed(), &ids(&[replaced]));
    assert_eq!(advancement.thesis().frozen(), &ids(&[replaced]));
    assert_eq!(advancement.thesis().open(), &ids(&[replacement]));
}

#[test]
fn an_imposed_commitment_arrives_with_its_ancestors() {
    let mut knowledge = Fixture::new();
    let root = knowledge.commit((3, 31), BTreeSet::new());
    let settled = knowledge.commit((6, 30), ids(&[root]));
    let thesis = knowledge.genesis(None, &[]);

    let head = knowledge.settle(settled);
    let advancement = thesis.advance(&knowledge, head).unwrap();

    assert_eq!(advancement.imposed(), &ids(&[root, settled]));
}

#[test]
fn advancement_spans_every_event_of_the_segment() {
    let mut knowledge = Fixture::new();
    let first = knowledge.commit((3, 31), BTreeSet::new());
    let second = knowledge.commit((4, 30), BTreeSet::new());
    let third = knowledge.commit((5, 31), BTreeSet::new());
    let thesis = knowledge.genesis(None, &[]);

    knowledge.settle(first);
    knowledge.settle(second);
    let head = knowledge.settle(third);

    let advancement = thesis.advance(&knowledge, head).unwrap();

    assert_eq!(advancement.imposed(), &ids(&[first, second, third]));
}

#[test]
fn what_advancement_froze_can_no_longer_be_omitted() {
    let mut knowledge = Fixture::new();
    let intended = knowledge.commit((3, 31), BTreeSet::new());
    let thesis = knowledge.genesis(None, &[intended]);

    assert!(thesis.fork(&knowledge, omitting(&[intended])).is_ok());

    let head = knowledge.settle(intended);
    let advanced = thesis.advance(&knowledge, head).unwrap().into_thesis();

    let refusal = advanced.fork(&knowledge, omitting(&[intended]));

    assert!(matches!(
        refusal,
        Err(ThesisError::FrozenPastOmitted(id)) if id == intended
    ));
}
