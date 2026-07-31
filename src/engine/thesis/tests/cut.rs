//! What a declared knowledge cut admits, and what it refuses.

use std::collections::BTreeSet;

use super::{Fixture, KnowledgeCut, d1, d2, d3, frozen_of, ids, open_of};

use crate::engine::thesis::ThesisError;

use crate::kernel::entities::EventId;

#[test]
fn a_head_absent_from_history_is_refused() {
    let knowledge = Fixture::new();
    let absent = EventId::from([9; 32]);

    let refusal = KnowledgeCut::declare(&knowledge, d2(), Some(absent));

    assert!(matches!(refusal, Err(ThesisError::UnknownEvent(id)) if id == absent));
}

#[test]
fn a_head_recorded_after_the_instant_is_refused() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let head = knowledge.settle(settled);

    let refusal = KnowledgeCut::declare(&knowledge, d1(), Some(head));

    assert!(matches!(
        refusal,
        Err(ThesisError::EventNotKnownAtCut { event, recorded_at, known_at })
            if event == head && recorded_at == d2() && known_at == d1()
    ));
}

/// What `declare` cannot yet tell apart, recorded so the limit is visible rather than implied:
/// a cut is proved to recognize nothing recorded after its instant, and *not* proved to
/// recognize everything recorded by it. Settling the second needs the latest Event recorded no
/// later than the instant, which this port cannot ask for.
#[test]
fn a_cut_is_not_proved_to_recognize_every_event_known_at_its_instant() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let intended = knowledge.commit((6, 30), BTreeSet::new());
    knowledge.settle(settled);

    let cut = KnowledgeCut::declare(&knowledge, d3(), None).unwrap();
    let thesis = knowledge.genesis(cut, &[intended]);

    assert!(frozen_of(&thesis).is_empty());
    assert_eq!(thesis.cut().event_head(), None);
}

/// The instant is part of what a Thesis means, so it alone separates two identities. Same
/// parent, same partition, same head, different cut: a different claim about what was knowable.
#[test]
fn the_instant_alone_changes_identity() {
    let mut knowledge = Fixture::new();
    let selection = knowledge.commit((3, 31), BTreeSet::new());

    let earlier = knowledge.genesis(knowledge.cut(d1(), None), &[selection]);
    let later = knowledge.genesis(knowledge.cut(d3(), None), &[selection]);

    assert_eq!(earlier.parent(), later.parent());
    assert_eq!(frozen_of(&earlier), frozen_of(&later));
    assert_eq!(open_of(&earlier), open_of(&later));
    assert_ne!(earlier.id(), later.id());
}

/// Addressing a head directly stays the finer selection: a cut may recognize an earlier head
/// than the latest one recorded by its instant.
#[test]
fn a_cut_may_recognize_a_head_earlier_than_the_latest_recorded() {
    let mut knowledge = Fixture::new();
    let first = knowledge.commit((3, 31), BTreeSet::new());
    let second = knowledge.commit((4, 30), BTreeSet::new());
    let earlier = knowledge.settle(first);
    knowledge.settle(second);

    let cut = KnowledgeCut::declare(&knowledge, d3(), Some(earlier)).unwrap();
    let thesis = knowledge.genesis(cut, &[]);

    assert_eq!(frozen_of(&thesis), ids(&[first]));
    assert_eq!(thesis.cut().event_head(), Some(earlier));
}
