//! What may diverge under a factual past that may not.

use std::collections::BTreeSet;

use super::{Fixture, ForkInput, ids, introducing, omitting, selected};

use crate::engine::thesis::ThesisError;

#[test]
fn an_open_commitment_may_be_omitted() {
    let mut knowledge = Fixture::new();
    let kept = knowledge.commit((3, 31), BTreeSet::new());
    let dropped = knowledge.commit((6, 30), BTreeSet::new());
    let parent = knowledge.genesis(None, &[kept, dropped]);

    let fork = parent.fork(&knowledge, omitting(&[dropped])).unwrap();

    assert_eq!(fork.open(), &ids(&[kept]));
    assert_eq!(fork.selection(), selected(&[kept]));
}

#[test]
fn the_frozen_past_may_not_be_omitted() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let head = knowledge.settle(settled);
    let parent = knowledge.genesis(Some(head), &[]);

    let refusal = parent.fork(&knowledge, omitting(&[settled]));

    assert!(matches!(
        refusal,
        Err(ThesisError::FrozenPastOmitted(id)) if id == settled
    ));
}

#[test]
fn omitting_what_a_retained_commitment_depends_on_is_refused() {
    let mut knowledge = Fixture::new();
    let required = knowledge.commit((3, 31), BTreeSet::new());
    let waiting = knowledge.commit((6, 30), ids(&[required]));
    let parent = knowledge.genesis(None, &[required, waiting]);

    let refusal = parent.fork(&knowledge, omitting(&[required]));

    assert!(matches!(
        refusal,
        Err(ThesisError::DanglingDependency { dependent, dependency })
            if dependent == waiting && dependency == required
    ));
}

#[test]
fn a_replacement_omits_and_introduces_together() {
    let mut knowledge = Fixture::new();
    let replaced = knowledge.commit((3, 31), BTreeSet::new());
    let replacement = knowledge.commit((4, 30), BTreeSet::new());
    let parent = knowledge.genesis(None, &[replaced]);

    let fork = parent
        .fork(
            &knowledge,
            ForkInput {
                omitted: ids(&[replaced]),
                introduced: ids(&[replacement]),
            },
        )
        .unwrap();

    assert_eq!(fork.open(), &ids(&[replacement]));
}

#[test]
fn omitting_and_introducing_the_same_commitment_is_refused() {
    let mut knowledge = Fixture::new();
    let contested = knowledge.commit((3, 31), BTreeSet::new());
    let parent = knowledge.genesis(None, &[contested]);

    let refusal = parent.fork(
        &knowledge,
        ForkInput {
            omitted: ids(&[contested]),
            introduced: ids(&[contested]),
        },
    );

    assert!(matches!(
        refusal,
        Err(ThesisError::OmittedAndIntroduced(id)) if id == contested
    ));
}

#[test]
fn introducing_a_commitment_whose_dependency_is_unselected_is_refused() {
    let mut knowledge = Fixture::new();
    let required = knowledge.commit((3, 31), BTreeSet::new());
    let introduced = knowledge.commit((6, 30), ids(&[required]));
    let parent = knowledge.genesis(None, &[]);

    let refusal = parent.fork(&knowledge, introducing(&[introduced]));

    assert!(matches!(
        refusal,
        Err(ThesisError::DanglingDependency { dependent, dependency })
            if dependent == introduced && dependency == required
    ));
}

#[test]
fn a_fork_inherits_the_head_and_the_frozen_region() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let open = knowledge.commit((6, 30), BTreeSet::new());
    let head = knowledge.settle(settled);
    let parent = knowledge.genesis(Some(head), &[open]);

    let fork = parent.fork(&knowledge, omitting(&[open])).unwrap();

    assert_eq!(fork.head(), &Some(head));
    assert_eq!(fork.frozen(), parent.frozen());
    assert_eq!(fork.parent(), &Some(parent.id()));
}

#[test]
fn introducing_an_already_frozen_commitment_leaves_the_partition_disjoint() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let head = knowledge.settle(settled);
    let parent = knowledge.genesis(Some(head), &[]);

    let fork = parent.fork(&knowledge, introducing(&[settled])).unwrap();

    assert_eq!(fork.frozen(), &ids(&[settled]));
    assert!(fork.open().is_empty());
    assert_eq!(fork.selection(), selected(&[settled]));
}

#[test]
fn identity_follows_meaning_and_not_the_order_it_was_asked_in() {
    let mut knowledge = Fixture::new();
    let first = knowledge.commit((3, 31), BTreeSet::new());
    let second = knowledge.commit((6, 30), BTreeSet::new());
    let parent = knowledge.genesis(None, &[]);

    let one = parent
        .fork(&knowledge, introducing(&[first, second]))
        .unwrap();
    let other = parent
        .fork(&knowledge, introducing(&[second, first]))
        .unwrap();

    assert_eq!(one.id(), other.id());
}

#[test]
fn a_different_parent_yields_a_different_identity() {
    let mut knowledge = Fixture::new();
    let selected = knowledge.commit((3, 31), BTreeSet::new());
    let parent = knowledge.genesis(None, &[selected]);
    let sibling = knowledge.genesis(None, &[]);

    let from_parent = parent.fork(&knowledge, omitting(&[])).unwrap();
    let from_sibling = sibling.fork(&knowledge, introducing(&[selected])).unwrap();

    assert_eq!(from_parent.selection(), from_sibling.selection());
    assert_ne!(from_parent.id(), from_sibling.id());
}
