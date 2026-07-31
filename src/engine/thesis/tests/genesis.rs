//! What a lineage may begin as.

use std::collections::BTreeSet;

use super::{Fixture, GenesisInput, Thesis, d1, d2, d3, frozen_of, ids, open_of};

use crate::engine::thesis::ThesisError;

use crate::kernel::entities::CommitmentId;

#[test]
fn an_empty_history_freezes_nothing() {
    let mut knowledge = Fixture::new();
    let a = knowledge.commit((3, 31), BTreeSet::new());
    let b = knowledge.commit((6, 30), BTreeSet::new());

    let thesis = knowledge.genesis(knowledge.cut(d1(), None), &[a, b]);

    assert!(frozen_of(&thesis).is_empty());
    assert_eq!(open_of(&thesis), ids(&[a, b]));
    assert_eq!(thesis.cut().known_at(), &d1());
    assert_eq!(thesis.parent(), &None);
}

#[test]
fn a_genesis_at_an_advanced_head_absorbs_what_history_settled() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let intended = knowledge.commit((6, 30), BTreeSet::new());
    let head = knowledge.settle(settled);

    let thesis = knowledge.genesis(knowledge.cut(d2(), Some(head)), &[intended]);

    assert_eq!(frozen_of(&thesis), ids(&[settled]));
    assert_eq!(open_of(&thesis), ids(&[intended]));
}

#[test]
fn the_ancestors_of_a_settled_commitment_are_frozen_with_it() {
    let mut knowledge = Fixture::new();
    let root = knowledge.commit((3, 31), BTreeSet::new());
    let settled = knowledge.commit((6, 30), ids(&[root]));
    let head = knowledge.settle(settled);

    let thesis = knowledge.genesis(knowledge.cut(d2(), Some(head)), &[]);

    assert_eq!(frozen_of(&thesis), ids(&[root, settled]));
    assert!(open_of(&thesis).is_empty());
}

#[test]
fn a_selection_omitting_a_dependency_is_refused() {
    let mut knowledge = Fixture::new();
    let required = knowledge.commit((3, 31), BTreeSet::new());
    let waiting = knowledge.commit((6, 30), ids(&[required]));

    let refusal = Thesis::genesis(
        &knowledge,
        GenesisInput {
            cut: knowledge.cut(d1(), None),
            selection: ids(&[waiting]),
        },
    );

    assert!(matches!(
        refusal,
        Err(ThesisError::DanglingDependency { dependent, dependency })
            if dependent == waiting && dependency == required
    ));
}

#[test]
fn a_commitment_recorded_after_the_cut_is_refused() {
    let mut knowledge = Fixture::new();
    let later = knowledge.commit_recorded_at(d3(), (6, 30), BTreeSet::new());

    let refusal = Thesis::genesis(
        &knowledge,
        GenesisInput {
            cut: knowledge.cut(d1(), None),
            selection: ids(&[later]),
        },
    );

    assert!(matches!(
        refusal,
        Err(ThesisError::CommitmentNotKnownAtCut { commitment, recorded_at, known_at })
            if commitment == later && recorded_at == d3() && known_at == d1()
    ));
}

#[test]
fn an_unadmitted_commitment_is_refused() {
    let knowledge = Fixture::new();
    let absent = CommitmentId::from([9; 32]);

    let refusal = Thesis::genesis(
        &knowledge,
        GenesisInput {
            cut: knowledge.cut(d1(), None),
            selection: ids(&[absent]),
        },
    );

    assert!(matches!(refusal, Err(ThesisError::UnknownCommitment(id)) if id == absent));
}
