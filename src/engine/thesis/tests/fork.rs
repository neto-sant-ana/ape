//! What may diverge under a knowledge cut that may not.

use std::collections::BTreeSet;

use super::{
    Fixture, ForkInput, d1, d2, d3, frozen_of, ids, introducing, omitting, open_of, resolved_of,
};

use crate::engine::thesis::ThesisError;

#[test]
fn an_open_commitment_may_be_omitted() {
    let mut knowledge = Fixture::new();
    let kept = knowledge.commit((3, 31), BTreeSet::new());
    let dropped = knowledge.commit((6, 30), BTreeSet::new());
    let parent = knowledge.genesis(knowledge.cut(d1(), None), &[kept, dropped]);

    let fork = parent.fork(&knowledge, omitting(&[dropped])).unwrap();

    assert_eq!(open_of(&fork), ids(&[kept]));
    assert_eq!(resolved_of(&fork), ids(&[kept]));
}

#[test]
fn the_frozen_past_may_not_be_omitted() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let head = knowledge.settle(settled);
    let parent = knowledge.genesis(knowledge.cut(d2(), Some(head)), &[]);

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
    let parent = knowledge.genesis(knowledge.cut(d1(), None), &[required, waiting]);

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
    let parent = knowledge.genesis(knowledge.cut(d1(), None), &[replaced]);

    let fork = parent
        .fork(
            &knowledge,
            ForkInput {
                omitted: ids(&[replaced]),
                introduced: ids(&[replacement]),
            },
        )
        .unwrap();

    assert_eq!(open_of(&fork), ids(&[replacement]));
}

#[test]
fn omitting_and_introducing_the_same_commitment_is_refused() {
    let mut knowledge = Fixture::new();
    let contested = knowledge.commit((3, 31), BTreeSet::new());
    let parent = knowledge.genesis(knowledge.cut(d1(), None), &[contested]);

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
    let parent = knowledge.genesis(knowledge.cut(d1(), None), &[]);

    let refusal = parent.fork(&knowledge, introducing(&[introduced]));

    assert!(matches!(
        refusal,
        Err(ThesisError::DanglingDependency { dependent, dependency })
            if dependent == introduced && dependency == required
    ));
}

/// A fork holds its parent's cut, so it may only introduce what that cut already knew. An
/// intention formed later belongs to a Thesis that recognizes a later cut.
#[test]
fn introducing_a_commitment_recorded_after_the_cut_is_refused() {
    let mut knowledge = Fixture::new();
    let later = knowledge.commit_recorded_at(d3(), (6, 30), BTreeSet::new());
    let parent = knowledge.genesis(knowledge.cut(d1(), None), &[]);

    let refusal = parent.fork(&knowledge, introducing(&[later]));

    assert!(matches!(
        refusal,
        Err(ThesisError::CommitmentNotKnownAtCut { commitment, recorded_at, known_at })
            if commitment == later && recorded_at == d3() && known_at == d1()
    ));
}

#[test]
fn a_fork_that_would_select_what_its_parent_selects_is_refused() {
    let mut knowledge = Fixture::new();
    let kept = knowledge.commit((3, 31), BTreeSet::new());
    let absent = knowledge.commit((6, 30), BTreeSet::new());
    let parent = knowledge.genesis(knowledge.cut(d1(), None), &[kept]);

    for request in [
        ForkInput {
            omitted: BTreeSet::new(),
            introduced: BTreeSet::new(),
        },
        omitting(&[absent]),
        introducing(&[kept]),
        ForkInput {
            omitted: ids(&[absent]),
            introduced: ids(&[kept]),
        },
    ] {
        assert!(matches!(
            parent.fork(&knowledge, request),
            Err(ThesisError::SelectionUnchanged)
        ));
    }
}

/// A request may carry redundancy and still be a fork: what is refused is a request whose
/// outcome is the parent, not a request that says more than it needed to.
#[test]
fn omitting_something_absent_alongside_something_open_is_a_fork() {
    let mut knowledge = Fixture::new();
    let kept = knowledge.commit((3, 31), BTreeSet::new());
    let dropped = knowledge.commit((6, 30), BTreeSet::new());
    let never_selected = knowledge.commit((9, 30), BTreeSet::new());
    let parent = knowledge.genesis(knowledge.cut(d1(), None), &[kept, dropped]);

    let fork = parent
        .fork(&knowledge, omitting(&[dropped, never_selected]))
        .unwrap();

    assert_eq!(resolved_of(&fork), ids(&[kept]));
}

#[test]
fn a_fork_inherits_the_cut_and_the_frozen_region() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let open = knowledge.commit((6, 30), BTreeSet::new());
    let head = knowledge.settle(settled);
    let parent = knowledge.genesis(knowledge.cut(d2(), Some(head)), &[open]);

    let fork = parent.fork(&knowledge, omitting(&[open])).unwrap();

    assert_eq!(fork.cut(), parent.cut());
    assert_eq!(frozen_of(&fork), frozen_of(&parent));
    assert_eq!(fork.parent(), &Some(parent.id()));
}

/// Selecting what the cut already imposed adds nothing to the open future, so it stays out of
/// the half a later fork may revise. It is stated alongside a real introduction, since on its
/// own it would change nothing at all.
#[test]
fn introducing_an_already_frozen_commitment_leaves_the_partition_disjoint() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let fresh = knowledge.commit((6, 30), BTreeSet::new());
    let head = knowledge.settle(settled);
    let parent = knowledge.genesis(knowledge.cut(d2(), Some(head)), &[]);

    let fork = parent
        .fork(&knowledge, introducing(&[settled, fresh]))
        .unwrap();

    assert_eq!(frozen_of(&fork), ids(&[settled]));
    assert_eq!(open_of(&fork), ids(&[fresh]));

    // Counted once each, which only holds while the halves stay disjoint.
    assert_eq!(fork.selection().len(), 2);
}

#[test]
fn identity_follows_meaning_and_not_the_order_it_was_asked_in() {
    let mut knowledge = Fixture::new();
    let first = knowledge.commit((3, 31), BTreeSet::new());
    let second = knowledge.commit((6, 30), BTreeSet::new());
    let parent = knowledge.genesis(knowledge.cut(d1(), None), &[]);

    let one = parent
        .fork(&knowledge, introducing(&[first, second]))
        .unwrap();
    let other = parent
        .fork(&knowledge, introducing(&[second, first]))
        .unwrap();

    assert_eq!(one.id(), other.id());
}

/// Two Theses may select the same graph at the same cut and still be different Theses, because
/// where an intention came from is part of what it is.
#[test]
fn a_different_parent_yields_a_different_identity() {
    let mut knowledge = Fixture::new();
    let inherited = knowledge.commit((3, 31), BTreeSet::new());
    let added = knowledge.commit((6, 30), BTreeSet::new());

    let parent = knowledge.genesis(knowledge.cut(d1(), None), &[inherited]);
    let sibling = knowledge.genesis(knowledge.cut(d1(), None), &[]);

    let from_parent = parent.fork(&knowledge, introducing(&[added])).unwrap();
    let from_sibling = sibling
        .fork(&knowledge, introducing(&[inherited, added]))
        .unwrap();

    assert_eq!(resolved_of(&from_parent), resolved_of(&from_sibling));
    assert_eq!(from_parent.cut(), from_sibling.cut());
    assert_ne!(from_parent.id(), from_sibling.id());
}
