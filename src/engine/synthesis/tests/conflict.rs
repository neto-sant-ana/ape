//! Every invariant a transfer can break, and the two that look alike.

use std::collections::BTreeSet;

use super::{Fixture, d3, ids, introducing, omitting};

use crate::engine::synthesis::conflict::conflicts;
use crate::engine::synthesis::{
    ApplicabilityConflict, CandidateSelection, IntentionalDifference, ResolvedTransfer,
};
use crate::engine::thesis::Thesis;

fn found(
    knowledge: &Fixture,
    base: &Thesis,
    source: &Thesis,
    target: &Thesis,
) -> Vec<ApplicabilityConflict> {
    let difference = IntentionalDifference::between(base, source);
    let transfer = ResolvedTransfer::resolving(&difference, target);
    let candidate = CandidateSelection::deriving(&transfer, target);

    conflicts(knowledge, &transfer, &candidate, target).unwrap()
}

#[test]
fn an_applicable_transfer_breaks_nothing() {
    let mut knowledge = Fixture::default();
    let shared = knowledge.commit((3, 31), BTreeSet::new());
    let added = knowledge.commit((4, 30), BTreeSet::new());
    let elsewhere = knowledge.commit((5, 31), BTreeSet::new());

    let base = knowledge.genesis(&[shared]);
    let source = base.fork(&knowledge, introducing(&[added])).unwrap();
    let target = base.fork(&knowledge, introducing(&[elsewhere])).unwrap();

    assert!(found(&knowledge, &base, &source, &target).is_empty());
}

#[test]
fn removing_what_the_targets_history_froze_is_refused() {
    let mut knowledge = Fixture::default();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let kept = knowledge.commit((4, 30), BTreeSet::new());

    let base = knowledge.genesis(&[settled, kept]);
    let source = base.fork(&knowledge, omitting(&[settled])).unwrap();

    knowledge.settle(settled);
    let advanced = base.advance(&knowledge, knowledge.cut()).unwrap();
    let target = advanced.thesis();

    assert_eq!(
        found(&knowledge, &base, &source, target),
        vec![ApplicabilityConflict::HistoricalFreezing {
            commitment: settled
        }],
    );
}

/// A commitment admitted after the Target's cut exists canonically and was still not
/// knowledge in the world the Target recognizes.
#[test]
fn introducing_what_the_target_could_not_have_known_is_refused() {
    let mut knowledge = Fixture::default();
    let shared = knowledge.commit((3, 31), BTreeSet::new());
    let base = knowledge.genesis(&[shared]);

    let later = knowledge.commit_recorded_at(d3(), (6, 30), BTreeSet::new());

    let advanced = base.advance(&knowledge, knowledge.cut_at(d3())).unwrap();
    let source = advanced
        .thesis()
        .fork(&knowledge, introducing(&[later]))
        .unwrap();

    let conflicts = found(&knowledge, &base, &source, &base);

    assert!(
        matches!(
            conflicts.as_slice(),
            [ApplicabilityConflict::HistoricalUnavailability { commitment, .. }]
                if *commitment == later
        ),
        "the Source knew it; the world the Target recognizes did not",
    );
}

/// Closure can only break in the combination, never inside either side: a Thesis whose fork
/// left a dependency dangling is refused where it is built. So the dependent here comes from
/// the Target, which added it while the Source was dropping what it needs — the divergence
/// this layer exists to notice.
#[test]
fn removing_a_dependency_the_target_came_to_need_is_a_breakage() {
    let mut knowledge = Fixture::default();
    let dependency = knowledge.commit((3, 31), BTreeSet::new());
    let dependent = knowledge.commit((4, 30), ids(&[dependency]));

    let base = knowledge.genesis(&[dependency]);
    let source = base.fork(&knowledge, omitting(&[dependency])).unwrap();
    let target = base.fork(&knowledge, introducing(&[dependent])).unwrap();

    assert_eq!(
        found(&knowledge, &base, &source, &target),
        vec![ApplicabilityConflict::DependencyBreakage {
            dependent,
            missing_dependency: dependency,
        }],
        "the Target held it, so the transfer broke what was there",
    );
}

/// The dependency was never in the Target: the introduction arrives without support that
/// world never had.
#[test]
fn introducing_without_a_dependency_the_target_never_had_is_missing() {
    let mut knowledge = Fixture::default();
    let dependency = knowledge.commit((3, 31), BTreeSet::new());
    let dependent = knowledge.commit((4, 30), ids(&[dependency]));

    let base = knowledge.genesis(&[dependency]);
    let source = base.fork(&knowledge, introducing(&[dependent])).unwrap();
    let target = base.fork(&knowledge, omitting(&[dependency])).unwrap();

    assert_eq!(
        found(&knowledge, &base, &source, &target),
        vec![ApplicabilityConflict::MissingDependency {
            commitment: dependent,
            dependency,
        }],
        "the Target does not hold it, so nothing was broken — it was never there",
    );
}

/// Both categories in one transfer, which is what makes the discrimination worth having.
#[test]
fn the_two_dependency_conflicts_are_told_apart_by_the_target() {
    let mut knowledge = Fixture::default();
    let held = knowledge.commit((3, 31), BTreeSet::new());
    let broken = knowledge.commit((4, 30), ids(&[held]));
    let absent = knowledge.commit((5, 31), BTreeSet::new());
    let unsupported = knowledge.commit((6, 30), ids(&[absent]));

    let base = knowledge.genesis(&[held, absent]);
    let source = base
        .fork(&knowledge, omitting(&[held]))
        .unwrap()
        .fork(&knowledge, introducing(&[unsupported]))
        .unwrap();
    let target = base
        .fork(&knowledge, introducing(&[broken]))
        .unwrap()
        .fork(&knowledge, omitting(&[absent]))
        .unwrap();

    let conflicts = found(&knowledge, &base, &source, &target);

    assert!(
        conflicts.contains(&ApplicabilityConflict::DependencyBreakage {
            dependent: broken,
            missing_dependency: held,
        }),
        "held was in the Target and the transfer removed it",
    );
    assert!(
        conflicts.contains(&ApplicabilityConflict::MissingDependency {
            commitment: unsupported,
            dependency: absent,
        }),
        "absent was already out of the Target, so its dependent arrives unsupported",
    );
}

/// Detection is ordered, because reports are compared and cached.
#[test]
fn conflicts_come_back_in_a_stable_order() {
    let mut knowledge = Fixture::default();
    let dependency = knowledge.commit((3, 31), BTreeSet::new());
    let dependent = knowledge.commit((4, 30), ids(&[dependency]));
    let other = knowledge.commit((5, 31), ids(&[dependency]));

    let base = knowledge.genesis(&[dependency]);
    let source = base.fork(&knowledge, omitting(&[dependency])).unwrap();
    let target = base
        .fork(&knowledge, introducing(&[dependent, other]))
        .unwrap();

    let once = found(&knowledge, &base, &source, &target);
    let again = found(&knowledge, &base, &source, &target);

    assert_eq!(once, again);
    assert_eq!(once.len(), 2, "both dependents are reported");
}
