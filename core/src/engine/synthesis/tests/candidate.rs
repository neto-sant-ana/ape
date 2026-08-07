//! The world a transfer would produce, and the half it cannot reach.

use std::collections::BTreeSet;

use super::{Fixture, ForkInput, ids, introducing, omitting};

use crate::engine::synthesis::{CandidateSelection, IntentionalDifference, ResolvedTransfer};
use crate::engine::thesis::Thesis;

fn candidate(base: &Thesis, source: &Thesis, target: &Thesis) -> CandidateSelection {
    let difference = IntentionalDifference::between(base, source);
    let transfer = ResolvedTransfer::resolving(&difference, target);

    CandidateSelection::deriving(&transfer, target)
}

#[test]
fn the_candidate_is_the_target_with_the_transfer_applied() {
    let mut knowledge = Fixture::default();
    let dropped = knowledge.commit((3, 31), BTreeSet::new());
    let added = knowledge.commit((4, 30), BTreeSet::new());
    let target_only = knowledge.commit((5, 31), BTreeSet::new());

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
    let target = base.fork(&knowledge, introducing(&[target_only])).unwrap();

    let candidate = candidate(&base, &source, &target);

    assert_eq!(
        candidate.open().collect::<BTreeSet<_>>(),
        ids(&[added, target_only]),
        "what the Target kept stays, the omission goes, the introduction arrives",
    );
    assert!(!candidate.contains(dropped));
}

#[test]
fn an_empty_transfer_leaves_the_target_as_it_was() {
    let mut knowledge = Fixture::default();
    let shared = knowledge.commit((3, 31), BTreeSet::new());
    let added = knowledge.commit((4, 30), BTreeSet::new());

    let base = knowledge.genesis(&[shared]);
    let source = base.fork(&knowledge, introducing(&[added])).unwrap();
    let target = base.fork(&knowledge, introducing(&[added])).unwrap();

    let candidate = candidate(&base, &source, &target);

    assert_eq!(
        candidate.resolved().collect::<BTreeSet<_>>(),
        target.selection().resolved().collect::<BTreeSet<_>>(),
    );
}

/// The frozen half belongs to the Target's history, and a transfer is about intention.
#[test]
fn the_candidate_inherits_the_targets_frozen_past() {
    let mut knowledge = Fixture::default();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let open = knowledge.commit((4, 30), BTreeSet::new());
    let added = knowledge.commit((5, 31), BTreeSet::new());

    let base = knowledge.genesis(&[settled, open]);
    let source = base.fork(&knowledge, introducing(&[added])).unwrap();

    knowledge.settle(settled);
    let advanced = base.advance(&knowledge, knowledge.cut()).unwrap();
    let target = advanced.thesis();

    assert!(
        target.selection().is_frozen(settled),
        "the fixture must actually freeze it, or this proves nothing",
    );

    let candidate = candidate(&base, &source, target);

    assert_eq!(
        candidate.frozen().collect::<BTreeSet<_>>(),
        target.selection().frozen().collect::<BTreeSet<_>>(),
    );
}

/// An effective introduction is absent from the Target by definition, so it can only land in
/// the open future — there is no path by which it arrives already frozen.
#[test]
fn an_introduction_lands_in_the_open_future() {
    let mut knowledge = Fixture::default();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let added = knowledge.commit((4, 30), BTreeSet::new());

    let base = knowledge.genesis(&[settled]);
    let source = base.fork(&knowledge, introducing(&[added])).unwrap();

    knowledge.settle(settled);
    let advanced = base.advance(&knowledge, knowledge.cut()).unwrap();
    let target = advanced.thesis();

    let candidate = candidate(&base, &source, target);

    assert!(candidate.open().any(|id| id == added));
    assert!(
        !candidate.frozen().any(|id| id == added),
        "nothing a transfer introduces is part of the Target's history",
    );
}

/// Asking to remove a frozen commitment cannot succeed quietly.
///
/// The frozen half never consults the removals, so the candidate keeps the fact and the
/// transfer's request stays visible for a conflict to name. A candidate that dropped it would
/// report a world in which an observed settlement no longer has its commitment.
#[test]
fn removing_a_frozen_commitment_is_not_expressible_in_the_candidate() {
    let mut knowledge = Fixture::default();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let kept = knowledge.commit((4, 30), BTreeSet::new());

    let base = knowledge.genesis(&[settled, kept]);
    let source = base.fork(&knowledge, omitting(&[settled])).unwrap();

    knowledge.settle(settled);
    let advanced = base.advance(&knowledge, knowledge.cut()).unwrap();
    let target = advanced.thesis();

    assert!(target.selection().is_frozen(settled));

    let difference = IntentionalDifference::between(&base, &source);
    let transfer = ResolvedTransfer::resolving(&difference, target);

    assert_eq!(
        transfer.remove().collect::<BTreeSet<_>>(),
        ids(&[settled]),
        "the transfer does ask for it",
    );

    let candidate = CandidateSelection::deriving(&transfer, target);

    assert!(
        candidate.contains(settled),
        "and the candidate keeps it, so the refusal is a conflict rather than a silence",
    );
}
