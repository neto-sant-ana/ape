//! What a Source decided, and what merely happened to it.

use std::collections::BTreeSet;

use super::{Fixture, ids, introducing, omitting};

use crate::engine::synthesis::IntentionalDifference;

/// The difference is built from the pair, so a Thesis compared with itself decided nothing.
#[test]
fn a_thesis_differs_in_nothing_from_itself() {
    let mut knowledge = Fixture::default();
    let one = knowledge.commit((3, 31), BTreeSet::new());
    let base = knowledge.genesis(&[one]);

    let difference = IntentionalDifference::between(&base, &base);

    assert!(difference.is_empty());
}

#[test]
fn what_both_select_is_not_transferred() {
    let mut knowledge = Fixture::default();
    let kept = knowledge.commit((3, 31), BTreeSet::new());
    let added = knowledge.commit((4, 30), BTreeSet::new());

    let base = knowledge.genesis(&[kept]);
    let source = base.fork(&knowledge, introducing(&[added])).unwrap();

    let difference = IntentionalDifference::between(&base, &source);

    assert_eq!(
        difference.introduced().collect::<BTreeSet<_>>(),
        ids(&[added])
    );
    assert_eq!(
        difference.omitted().count(),
        0,
        "what the Source kept is not a decision to transfer",
    );
}

#[test]
fn dropping_an_open_commitment_is_an_omission() {
    let mut knowledge = Fixture::default();
    let dropped = knowledge.commit((3, 31), BTreeSet::new());
    let kept = knowledge.commit((4, 30), BTreeSet::new());

    let base = knowledge.genesis(&[dropped, kept]);
    let source = base.fork(&knowledge, omitting(&[dropped])).unwrap();

    let difference = IntentionalDifference::between(&base, &source);

    assert_eq!(
        difference.omitted().collect::<BTreeSet<_>>(),
        ids(&[dropped])
    );
    assert_eq!(difference.introduced().count(), 0);
}

/// The asymmetry, on the omission side: `Commitments(Source)` rather than `Open(Source)`.
///
/// A commitment the Source carries as frozen has left its open future without anyone
/// dropping it — an Event settled it, and the Source is obliged to hold it. Reading that as
/// an omission would transfer a removal nobody decided, and the Target would be asked to
/// drop a commitment its own history may have settled too.
#[test]
fn a_commitment_frozen_in_the_source_is_not_an_omission() {
    let mut knowledge = Fixture::default();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let other = knowledge.commit((4, 30), BTreeSet::new());

    let base = knowledge.genesis(&[settled, other]);

    knowledge.settle(settled);
    let source = base.advance(&knowledge, knowledge.cut()).unwrap();
    let source = source.thesis();

    assert!(
        source.selection().is_frozen(settled),
        "the fixture must actually freeze it, or this proves nothing",
    );

    let difference = IntentionalDifference::between(&base, source);

    assert!(
        difference.is_empty(),
        "history moving a commitment into the frozen past is not a decision",
    );
}

/// The asymmetry, on the introduction side: `Open(Source)` rather than `Commitments(Source)`.
///
/// An advancement adds to the Source whatever the newly recognized Events made unavoidable.
/// Those commitments are absent from the Base and present in the Source, so a whole-selection
/// comparison would call them introductions — transferring a fact as though it were a plan.
#[test]
fn a_commitment_imposed_by_advancement_is_not_an_introduction() {
    let mut knowledge = Fixture::default();
    let selected = knowledge.commit((3, 31), BTreeSet::new());
    let imposed = knowledge.commit((4, 30), BTreeSet::new());

    let base = knowledge.genesis(&[selected]);

    knowledge.settle(imposed);
    let advancement = base.advance(&knowledge, knowledge.cut()).unwrap();
    let source = advancement.thesis();

    assert_eq!(
        advancement.imposed().collect::<BTreeSet<_>>(),
        ids(&[imposed]),
        "the fixture must actually impose it, or this proves nothing",
    );

    let difference = IntentionalDifference::between(&base, source);

    assert!(
        difference.is_empty(),
        "what history imposed on the Source was never the Source's decision",
    );
}

/// An omission and an introduction in one Source are two facts, not one replacement.
#[test]
fn omission_and_introduction_carry_no_correspondence() {
    let mut knowledge = Fixture::default();
    let dropped = knowledge.commit((3, 31), BTreeSet::new());
    let added = knowledge.commit((4, 30), BTreeSet::new());

    let base = knowledge.genesis(&[dropped]);
    let source = base
        .fork(
            &knowledge,
            super::ForkInput {
                omitted: ids(&[dropped]),
                introduced: ids(&[added]),
            },
        )
        .unwrap();

    let difference = IntentionalDifference::between(&base, &source);

    assert_eq!(
        difference.omitted().collect::<BTreeSet<_>>(),
        ids(&[dropped])
    );
    assert_eq!(
        difference.introduced().collect::<BTreeSet<_>>(),
        ids(&[added])
    );
}
