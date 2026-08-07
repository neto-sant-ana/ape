//! What is left to do once the Target is taken into account.

use std::collections::BTreeSet;

use super::{Fixture, ForkInput, ids, introducing, omitting};

use crate::engine::synthesis::{IntentionalDifference, ResolvedTransfer};
use crate::engine::thesis::Thesis;

fn resolved(base: &Thesis, source: &Thesis, target: &Thesis) -> ResolvedTransfer {
    ResolvedTransfer::resolving(&IntentionalDifference::between(base, source), target)
}

#[test]
fn an_effective_change_is_carried_through() {
    let mut knowledge = Fixture::default();
    let dropped = knowledge.commit((3, 31), BTreeSet::new());
    let added = knowledge.commit((4, 30), BTreeSet::new());
    let elsewhere = knowledge.commit((5, 31), BTreeSet::new());

    let base = knowledge.genesis(&[dropped]);
    let source = base
        .fork(
            &knowledge,
            ForkInput {
                omitted: ids(&[dropped]),
                introduced: ids(&[added]),
            },
        )
        .unwrap();
    let target = base.fork(&knowledge, introducing(&[elsewhere])).unwrap();

    let transfer = resolved(&base, &source, &target);

    assert_eq!(transfer.remove().collect::<BTreeSet<_>>(), ids(&[dropped]));
    assert_eq!(transfer.introduce().collect::<BTreeSet<_>>(), ids(&[added]));
}

#[test]
fn omitting_what_the_target_already_dropped_requires_no_removal() {
    let mut knowledge = Fixture::default();
    let dropped = knowledge.commit((3, 31), BTreeSet::new());
    let kept = knowledge.commit((4, 30), BTreeSet::new());

    let base = knowledge.genesis(&[dropped, kept]);
    let source = base.fork(&knowledge, omitting(&[dropped])).unwrap();
    let target = base.fork(&knowledge, omitting(&[dropped])).unwrap();

    assert!(
        resolved(&base, &source, &target).is_empty(),
        "both reached the same outcome, so nothing is left to apply",
    );
}

#[test]
fn introducing_what_the_target_already_selects_requires_no_introduction() {
    let mut knowledge = Fixture::default();
    let shared = knowledge.commit((3, 31), BTreeSet::new());
    let added = knowledge.commit((4, 30), BTreeSet::new());

    let base = knowledge.genesis(&[shared]);
    let source = base.fork(&knowledge, introducing(&[added])).unwrap();
    let target = base.fork(&knowledge, introducing(&[added])).unwrap();

    assert!(resolved(&base, &source, &target).is_empty());
}

/// Presence is presence: the Target holding a commitment as frozen satisfies an introduction
/// just as an open one does. History put it there rather than a planner, and the transfer
/// asked for it to be there.
#[test]
fn introducing_what_the_target_froze_requires_no_introduction() {
    let mut knowledge = Fixture::default();
    let shared = knowledge.commit((3, 31), BTreeSet::new());
    let added = knowledge.commit((4, 30), BTreeSet::new());

    let base = knowledge.genesis(&[shared]);
    let source = base.fork(&knowledge, introducing(&[added])).unwrap();

    knowledge.settle(added);
    let target = base.advance(&knowledge, knowledge.cut()).unwrap();
    let target = target.thesis();

    assert!(
        target.selection().is_frozen(added),
        "the fixture must actually freeze it in the Target, or this proves nothing",
    );

    assert!(
        resolved(&base, &source, target).is_empty(),
        "a commitment the Target already holds is not introduced, however it got there",
    );
}

/// The mirror case, and it resolves the other way: an omission of something the Target froze
/// *is* an effective removal. Whether it may happen is judged later, over this result.
#[test]
fn omitting_what_the_target_froze_still_resolves_to_a_removal() {
    let mut knowledge = Fixture::default();
    let dropped = knowledge.commit((3, 31), BTreeSet::new());
    let kept = knowledge.commit((4, 30), BTreeSet::new());

    let base = knowledge.genesis(&[dropped, kept]);
    let source = base.fork(&knowledge, omitting(&[dropped])).unwrap();

    knowledge.settle(dropped);
    let target = base.advance(&knowledge, knowledge.cut()).unwrap();
    let target = target.thesis();

    assert!(
        target.selection().is_frozen(dropped),
        "the fixture must actually freeze it in the Target, or this proves nothing",
    );

    let transfer = resolved(&base, &source, target);

    assert_eq!(
        transfer.remove().collect::<BTreeSet<_>>(),
        ids(&[dropped]),
        "resolving states what the transfer asks for; refusing it is a separate judgment",
    );
}

/// Idempotence is per change, not per transfer: what remains effective still applies.
#[test]
fn a_partly_idempotent_transfer_keeps_what_is_left() {
    let mut knowledge = Fixture::default();
    let shared = knowledge.commit((3, 31), BTreeSet::new());
    let both_added = knowledge.commit((4, 30), BTreeSet::new());
    let only_source_added = knowledge.commit((5, 31), BTreeSet::new());

    let base = knowledge.genesis(&[shared]);
    let source = base
        .fork(&knowledge, introducing(&[both_added, only_source_added]))
        .unwrap();
    let target = base.fork(&knowledge, introducing(&[both_added])).unwrap();

    let transfer = resolved(&base, &source, &target);

    assert!(!transfer.is_empty());
    assert_eq!(
        transfer.introduce().collect::<BTreeSet<_>>(),
        ids(&[only_source_added]),
    );
}
