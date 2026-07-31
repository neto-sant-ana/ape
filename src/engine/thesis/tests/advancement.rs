//! What recognizing later knowledge does to an intention.

use std::collections::BTreeSet;

use super::{
    Fixture, ForkInput, d1, d2, d3, frozen_of, ids, imposed_of, introducing, omitting, open_of,
    resolved_of,
};

use crate::engine::thesis::ThesisError;

#[test]
fn advancing_to_the_recognized_cut_is_refused() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    knowledge.settle(settled);
    let thesis = knowledge.genesis(knowledge.cut(d2()), &[]);

    let refusal = thesis.advance(&knowledge, knowledge.cut(d2()));

    assert!(matches!(
        refusal,
        Err(ThesisError::CutNotLater { parent, target, .. }) if parent == d2() && target == d2()
    ));
}

/// Two cuts of one instant are ordered by their heads, so a finer cut advances to the one its own
/// group refines. Without it a Thesis born at a finer cut could never reach the instant's whole
/// chain by ancestry, and the lineage would have to start over.
#[test]
fn a_finer_cut_may_advance_within_the_same_instant() {
    let mut knowledge = Fixture::new();
    let first = knowledge.commit((3, 31), BTreeSet::new());
    let second = knowledge.commit((4, 30), BTreeSet::new());
    let earlier = knowledge.settle(first);
    let later = knowledge.settle(second);

    let parent = knowledge.genesis(knowledge.cut_within(d3(), earlier), &[]);
    assert_eq!(frozen_of(&parent), ids(&[first]));

    let advancement = parent.advance(&knowledge, knowledge.cut(d3())).unwrap();
    let advanced = advancement.thesis();

    assert_eq!(advanced.cut().known_at(), parent.cut().known_at());
    assert_eq!(advanced.cut().event_head(), Some(later));
    assert_eq!(imposed_of(&advancement), ids(&[second]));
    assert_eq!(frozen_of(advanced), ids(&[first, second]));
}

#[test]
fn an_earlier_instant_is_refused() {
    let mut knowledge = Fixture::new();
    let intended = knowledge.commit((3, 31), BTreeSet::new());
    let thesis = knowledge.genesis(knowledge.cut(d3()), &[intended]);

    let refusal = thesis.advance(&knowledge, knowledge.cut(d1()));

    assert!(matches!(
        refusal,
        Err(ThesisError::CutNotLater { parent, target, .. }) if parent == d3() && target == d1()
    ));
}

/// The walk that resolves a segment reads from the target back to the head already
/// recognized, and no further. What the parent had frozen is not settled again, so it is not
/// reported as imposed — and the frozen region it produces is the one a genesis at the same
/// cut would compute from the whole chain.
#[test]
fn advancing_the_head_reads_the_new_segment_only() {
    let mut knowledge = Fixture::new();
    let first = knowledge.commit((3, 31), BTreeSet::new());
    let second = knowledge.commit((4, 30), BTreeSet::new());
    let third = knowledge.commit((5, 31), BTreeSet::new());
    let kept = knowledge.commit((9, 30), BTreeSet::new());

    knowledge.settle(first);
    let parent = knowledge.genesis(knowledge.cut(d2()), &[kept]);

    knowledge.settle(second);
    knowledge.settle(third);

    let advancement = parent.advance(&knowledge, knowledge.cut(d3())).unwrap();
    let advanced = advancement.thesis();

    assert_eq!(imposed_of(&advancement), ids(&[second, third]));
    assert_eq!(frozen_of(advanced), ids(&[first, second, third]));
    assert_eq!(open_of(advanced), ids(&[kept]));

    let recomputed = knowledge.genesis(knowledge.cut(d3()), &[kept]);
    assert_eq!(frozen_of(advanced), frozen_of(&recomputed));
}

#[test]
fn a_head_preceding_the_recognized_one_is_refused() {
    let mut knowledge = Fixture::new();
    let first = knowledge.commit((3, 31), BTreeSet::new());
    let second = knowledge.commit((4, 30), BTreeSet::new());
    let earlier = knowledge.settle(first);
    let recognized = knowledge.settle(second);
    let thesis = knowledge.genesis(knowledge.cut(d2()), &[]);

    let refusal = thesis.advance(&knowledge, knowledge.cut_within(d3(), earlier));

    assert!(matches!(
        refusal,
        Err(ThesisError::HeadDoesNotDescend { parent, target })
            if parent == Some(recognized) && target == earlier
    ));
}

/// The advancement in which knowledge grew without anything being observed. The head holding is
/// not a choice the caller made but what the later instant resolved to, which is the whole point
/// of resolving it: a head that did not move *means* no Event was recorded in between.
#[test]
fn knowledge_may_advance_while_the_head_holds() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let intended = knowledge.commit((6, 30), BTreeSet::new());
    let head = knowledge.settle(settled);
    let parent = knowledge.genesis(knowledge.cut(d2()), &[intended]);

    let advancement = parent.advance(&knowledge, knowledge.cut(d3())).unwrap();
    let advanced = advancement.thesis();

    assert_eq!(advancement.imposed_count(), 0);
    assert_eq!(frozen_of(advanced), frozen_of(&parent));
    assert_eq!(open_of(advanced), open_of(&parent));
    assert_eq!(resolved_of(advanced), resolved_of(&parent));
    assert_eq!(advanced.cut().event_head(), Some(head));
    assert_eq!(advanced.cut().known_at(), &d3());
    assert_ne!(advanced.id(), parent.id());
}

#[test]
fn a_selected_commitment_moves_from_the_open_future_into_the_frozen_past() {
    let mut knowledge = Fixture::new();
    let intended = knowledge.commit((3, 31), BTreeSet::new());
    let thesis = knowledge.genesis(knowledge.cut(d1()), &[intended]);
    knowledge.settle(intended);

    let advancement = thesis.advance(&knowledge, knowledge.cut(d2())).unwrap();
    let advanced = advancement.thesis();

    assert_eq!(advancement.imposed_count(), 0);
    assert_eq!(frozen_of(advanced), ids(&[intended]));
    assert!(open_of(advanced).is_empty());
    assert_eq!(advanced.selection().len(), thesis.selection().len());
}

#[test]
fn what_history_settled_outside_the_parent_is_reported_as_imposed() {
    let mut knowledge = Fixture::new();
    let replaced = knowledge.commit((3, 31), BTreeSet::new());
    let replacement = knowledge.commit((4, 30), BTreeSet::new());

    let thesis = knowledge.genesis(knowledge.cut(d1()), &[replaced]);
    let revised = thesis
        .fork(
            &knowledge,
            ForkInput {
                omitted: ids(&[replaced]),
                introduced: ids(&[replacement]),
            },
        )
        .unwrap();

    knowledge.settle(replaced);
    let advancement = revised.advance(&knowledge, knowledge.cut(d2())).unwrap();

    assert_eq!(imposed_of(&advancement), ids(&[replaced]));
    assert_eq!(frozen_of(advancement.thesis()), ids(&[replaced]));
    assert_eq!(open_of(advancement.thesis()), ids(&[replacement]));
}

#[test]
fn an_imposed_commitment_arrives_with_its_ancestors() {
    let mut knowledge = Fixture::new();
    let root = knowledge.commit((3, 31), BTreeSet::new());
    let settled = knowledge.commit((6, 30), ids(&[root]));
    let thesis = knowledge.genesis(knowledge.cut(d1()), &[]);

    knowledge.settle(settled);
    let advancement = thesis.advance(&knowledge, knowledge.cut(d2())).unwrap();

    assert_eq!(imposed_of(&advancement), ids(&[root, settled]));
}

/// Every way a commitment can reach the frozen past of a child, in one advancement, against the
/// formula the layer promises: `Imposed = Frozen(H') − Commitments(T)`.
///
/// ```text
/// parent   frozen {a}          already unavoidable
///          open   {b}          selected, and settled by the new segment
/// child    frozen {a,b,c,d}    c settled under another continuation, d its ancestor
///          imposed      {c,d}  only what the parent never selected
/// ```
#[test]
fn imposition_is_what_the_parent_never_selected() {
    let mut knowledge = Fixture::new();
    let a = knowledge.commit((3, 31), BTreeSet::new());
    let b = knowledge.commit((4, 30), BTreeSet::new());
    let d = knowledge.commit((5, 31), BTreeSet::new());
    let c = knowledge.commit((6, 30), ids(&[d]));

    knowledge.settle(a);
    let parent = knowledge.genesis(knowledge.cut(d2()), &[b]);

    assert_eq!(frozen_of(&parent), ids(&[a]));
    assert_eq!(open_of(&parent), ids(&[b]));

    knowledge.settle(b);
    knowledge.settle(c);

    let advancement = parent.advance(&knowledge, knowledge.cut(d3())).unwrap();
    let advanced = advancement.thesis();

    assert_eq!(frozen_of(advanced), ids(&[a, b, c, d]));
    assert!(open_of(advanced).is_empty());
    assert_eq!(imposed_of(&advancement), ids(&[c, d]));
}

#[test]
fn advancement_spans_every_event_of_the_segment() {
    let mut knowledge = Fixture::new();
    let first = knowledge.commit((3, 31), BTreeSet::new());
    let second = knowledge.commit((4, 30), BTreeSet::new());
    let third = knowledge.commit((5, 31), BTreeSet::new());
    let thesis = knowledge.genesis(knowledge.cut(d1()), &[]);

    knowledge.settle(first);
    knowledge.settle(second);
    knowledge.settle(third);

    let advancement = thesis.advance(&knowledge, knowledge.cut(d2())).unwrap();

    assert_eq!(imposed_of(&advancement), ids(&[first, second, third]));
}

#[test]
fn what_advancement_froze_can_no_longer_be_omitted() {
    let mut knowledge = Fixture::new();
    let intended = knowledge.commit((3, 31), BTreeSet::new());
    let thesis = knowledge.genesis(knowledge.cut(d1()), &[intended]);

    assert!(thesis.fork(&knowledge, omitting(&[intended])).is_ok());

    knowledge.settle(intended);
    let advanced = thesis
        .advance(&knowledge, knowledge.cut(d2()))
        .unwrap()
        .into_thesis();

    let refusal = advanced.fork(&knowledge, omitting(&[intended]));

    assert!(matches!(
        refusal,
        Err(ThesisError::FrozenPastOmitted(id)) if id == intended
    ));
}

/// An advancement recognizes knowledge; it does not adopt intention. A commitment admitted
/// between the two cuts and left unsettled does not enter — it only becomes eligible.
#[test]
fn a_commitment_admitted_between_cuts_enters_by_decision_and_not_by_advancement() {
    let mut knowledge = Fixture::new();
    let intended = knowledge.commit((3, 31), BTreeSet::new());
    let thesis = knowledge.genesis(knowledge.cut(d1()), &[intended]);

    let admitted_later = knowledge.commit_recorded_at(d3(), (9, 30), BTreeSet::new());

    let advanced = thesis
        .advance(&knowledge, knowledge.cut(d3()))
        .unwrap()
        .into_thesis();

    assert_eq!(resolved_of(&advanced), ids(&[intended]));

    let decided = advanced
        .fork(&knowledge, introducing(&[admitted_later]))
        .unwrap();

    assert_eq!(resolved_of(&decided), ids(&[intended, admitted_later]));

    // Which is why the advancement was needed: the same decision under the earlier cut claims
    // an intention that could not yet have been formed.
    assert!(matches!(
        thesis.fork(&knowledge, introducing(&[admitted_later])),
        Err(ThesisError::CommitmentNotKnownAtCut { commitment, .. }) if commitment == admitted_later
    ));
}

/// The cycle the layer exists to support: knowledge grows, then intention changes, each in a
/// Thesis of its own.
#[test]
fn knowledge_grows_then_intention_changes() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let kept = knowledge.commit((6, 30), BTreeSet::new());

    let first = knowledge.genesis(knowledge.cut(d1()), &[settled, kept]);
    knowledge.settle(settled);

    let second = first
        .advance(&knowledge, knowledge.cut(d2()))
        .unwrap()
        .into_thesis();

    let third = second.fork(&knowledge, omitting(&[kept])).unwrap();

    assert_eq!(second.parent(), &Some(first.id()));
    assert_eq!(third.parent(), &Some(second.id()));

    assert_eq!(second.cut().known_at(), &d2());
    assert_eq!(third.cut(), second.cut());

    assert_eq!(frozen_of(&third), ids(&[settled]));
    assert!(open_of(&third).is_empty());
}
