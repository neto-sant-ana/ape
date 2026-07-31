//! What a knowledge cut addresses, and what it refuses to be made of.

use std::collections::BTreeSet;

use super::{Fixture, KnowledgeCut, d1, d2, d3, frozen_of, ids, open_of};

use crate::engine::thesis::ThesisError;

use crate::kernel::entities::EventId;

/// The chain a cut recognizes is resolved from its instant, so the two coordinates always describe
/// one moment. An instant with nothing recorded by it addresses no Event at all.
#[test]
fn an_instant_addresses_the_chain_that_was_current_at_it() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());

    assert_eq!(KnowledgeCut::at(&knowledge, d1()).event_head(), None);

    let head = knowledge.settle(settled);

    assert_eq!(KnowledgeCut::at(&knowledge, d1()).event_head(), None);
    assert_eq!(KnowledgeCut::at(&knowledge, d2()).event_head(), Some(head));
    assert_eq!(KnowledgeCut::at(&knowledge, d3()).event_head(), Some(head));
}

/// Setting aside a fact already recorded by the instant is what a resolved cut cannot express, and
/// naming a head directly does not become the way around it.
#[test]
fn a_head_preceding_the_instants_group_is_refused() {
    let mut knowledge = Fixture::new();
    let first = knowledge.commit((3, 31), BTreeSet::new());
    let second = knowledge.commit((4, 30), BTreeSet::new());
    // Recorded the day before the instant the cut is taken at, so it is not of its group.
    let earlier = knowledge.settle_recorded_at(super::date(2026, 2, 4), first);
    let addressed = knowledge.settle(second);

    let refusal = KnowledgeCut::within(&knowledge, d2(), earlier);

    assert!(matches!(
        refusal,
        Err(ThesisError::HeadPrecedesCut { named, addressed: cut })
            if named == earlier && cut == Some(addressed)
    ));
}

/// Within one instant the group is addressable, because a day is only as fine as the recording
/// instant is. That is the finer selection, and the only one.
#[test]
fn a_head_within_the_instants_group_is_a_finer_cut() {
    let mut knowledge = Fixture::new();
    let first = knowledge.commit((3, 31), BTreeSet::new());
    let second = knowledge.commit((4, 30), BTreeSet::new());
    let earlier = knowledge.settle(first);
    let later = knowledge.settle(second);

    assert_eq!(KnowledgeCut::at(&knowledge, d2()).event_head(), Some(later));

    let cut = knowledge.cut_within(d2(), earlier);
    let thesis = knowledge.genesis(cut, &[]);

    assert_eq!(frozen_of(&thesis), ids(&[first]));
    assert_eq!(thesis.cut().event_head(), Some(earlier));
}

/// A finer cut refines the group of its own instant, and an instant with nothing recorded at it has
/// none. Refining the last group *before* it would combine intentions known at the instant with a
/// factual history that omits Events recorded before them — retraction, reached by naming a head
/// that is legitimately of the group the instant resolves to.
#[test]
fn a_finer_cut_cannot_reopen_an_event_group_from_an_earlier_instant() {
    let mut knowledge = Fixture::new();
    let first = knowledge.commit((3, 31), BTreeSet::new());
    let second = knowledge.commit((4, 30), BTreeSet::new());

    let earlier = knowledge.settle(first);
    let addressed = knowledge.settle(second);

    // Nothing is recorded at d3, so it resolves to the group of d2 — which is already the past.
    let refusal = KnowledgeCut::within(&knowledge, d3(), earlier);

    assert!(
        matches!(
            refusal,
            Err(ThesisError::NoEventGroupAtCut { known_at, addressed: cut, addressed_at })
                if known_at == d3() && cut == Some(addressed) && addressed_at == Some(d2())
        ),
        "d3 addresses no group of its own, so there is nothing to refine within it",
    );
}

/// Sharing the instant is not belonging to the chain. An Event of another reach of history can be
/// recorded on the same day as the head an instant addresses, and naming it would recognize a past
/// that never led there — refused where the cut is built, not left for a later operation to notice.
#[test]
fn a_detached_event_of_the_same_instant_is_not_a_valid_finer_cut() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let elsewhere = knowledge.commit((6, 30), BTreeSet::new());
    let addressed = knowledge.settle(settled);
    let detached = knowledge.detached(elsewhere);

    let refusal = KnowledgeCut::within(&knowledge, d2(), detached);

    assert!(matches!(
        refusal,
        Err(ThesisError::HeadDoesNotBelongToCut { named, addressed: cut })
            if named == detached && cut == addressed
    ));
}

#[test]
fn a_head_absent_from_history_is_refused() {
    let knowledge = Fixture::new();
    let absent = EventId::from([9; 32]);

    let refusal = KnowledgeCut::within(&knowledge, d2(), absent);

    assert!(matches!(refusal, Err(ThesisError::UnknownEvent(id)) if id == absent));
}

#[test]
fn a_head_recorded_after_the_instant_is_refused() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let head = knowledge.settle(settled);

    let refusal = KnowledgeCut::within(&knowledge, d1(), head);

    assert!(matches!(
        refusal,
        Err(ThesisError::EventNotKnownAtCut { event, recorded_at, known_at })
            if event == head && recorded_at == d2() && known_at == d1()
    ));
}

/// The instant is part of what a Thesis means, so it alone separates two identities. Same parent,
/// same partition, same head, different cut: a different claim about what was knowable.
#[test]
fn the_instant_alone_changes_identity() {
    let mut knowledge = Fixture::new();
    let selection = knowledge.commit((3, 31), BTreeSet::new());

    let earlier = knowledge.genesis(knowledge.cut(d1()), &[selection]);
    let later = knowledge.genesis(knowledge.cut(d3()), &[selection]);

    assert_eq!(earlier.parent(), later.parent());
    assert_eq!(earlier.cut().event_head(), later.cut().event_head());
    assert_eq!(frozen_of(&earlier), frozen_of(&later));
    assert_eq!(open_of(&earlier), open_of(&later));
    assert_ne!(earlier.id(), later.id());
}
