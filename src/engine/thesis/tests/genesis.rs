//! What a lineage may begin as.

use std::collections::BTreeSet;

use super::{Fixture, GenesisInput, Thesis, ids};

use crate::engine::thesis::ThesisError;

use crate::kernel::entities::EventId;

#[test]
fn an_empty_history_freezes_nothing() {
    let mut knowledge = Fixture::new();
    let a = knowledge.commit((3, 31), BTreeSet::new());
    let b = knowledge.commit((6, 30), BTreeSet::new());

    let thesis = knowledge.genesis(None, &[a, b]);

    assert!(thesis.frozen().is_empty());
    assert_eq!(thesis.open(), &ids(&[a, b]));
    assert_eq!(thesis.head(), &None);
    assert_eq!(thesis.parent(), &None);
}

#[test]
fn a_genesis_at_an_advanced_head_absorbs_what_history_settled() {
    let mut knowledge = Fixture::new();
    let settled = knowledge.commit((3, 31), BTreeSet::new());
    let intended = knowledge.commit((6, 30), BTreeSet::new());
    let head = knowledge.settle(settled);

    let thesis = knowledge.genesis(Some(head), &[intended]);

    assert_eq!(thesis.frozen(), &ids(&[settled]));
    assert_eq!(thesis.open(), &ids(&[intended]));
}

#[test]
fn the_ancestors_of_a_settled_commitment_are_frozen_with_it() {
    let mut knowledge = Fixture::new();
    let root = knowledge.commit((3, 31), BTreeSet::new());
    let settled = knowledge.commit((6, 30), ids(&[root]));
    let head = knowledge.settle(settled);

    let thesis = knowledge.genesis(Some(head), &[]);

    assert_eq!(thesis.frozen(), &ids(&[root, settled]));
    assert!(thesis.open().is_empty());
}

#[test]
fn a_selection_omitting_a_dependency_is_refused() {
    let mut knowledge = Fixture::new();
    let required = knowledge.commit((3, 31), BTreeSet::new());
    let waiting = knowledge.commit((6, 30), ids(&[required]));

    let refusal = Thesis::genesis(
        &knowledge,
        GenesisInput {
            head: None,
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
fn an_unadmitted_commitment_is_refused() {
    let knowledge = Fixture::new();
    let absent = super::CommitmentId::from([9; 32]);

    let refusal = Thesis::genesis(
        &knowledge,
        GenesisInput {
            head: None,
            selection: ids(&[absent]),
        },
    );

    assert!(matches!(refusal, Err(ThesisError::UnknownCommitment(id)) if id == absent));
}

#[test]
fn a_head_absent_from_history_is_refused() {
    let knowledge = Fixture::new();
    let absent = EventId::from([9; 32]);

    let refusal = Thesis::genesis(
        &knowledge,
        GenesisInput {
            head: Some(absent),
            selection: BTreeSet::new(),
        },
    );

    assert!(matches!(refusal, Err(ThesisError::UnknownEvent(id)) if id == absent));
}
