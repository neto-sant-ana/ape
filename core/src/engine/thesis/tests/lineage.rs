//! What descending from a Thesis means, and where the walk stops.

use std::collections::BTreeSet;

use super::{Fixture, d1, introducing};

use crate::engine::thesis::{
    InMemoryArchive, Thesis, ThesisArchive, ThesisError, ThesisId, descends_from,
};

/// Archive a lineage and hand back the archive, so a test reads as the shape it built.
fn archived(theses: &[&Thesis]) -> InMemoryArchive {
    let mut archive = InMemoryArchive::default();

    for thesis in theses {
        archive.put_thesis((*thesis).clone()).unwrap();
    }

    archive
}

#[test]
fn a_thesis_descends_from_itself() {
    let mut knowledge = Fixture::new();
    let one = knowledge.commit((3, 31), BTreeSet::new());
    let genesis = knowledge.genesis(knowledge.cut(d1()), &[one]);

    let archive = archived(&[&genesis]);

    assert!(
        descends_from(&archive, genesis.id(), genesis.id()).unwrap(),
        "the relation is reflexive, which is what lets a Base equal its Target",
    );
}

#[test]
fn a_child_descends_from_every_ancestor_and_from_no_descendant() {
    let mut knowledge = Fixture::new();
    let one = knowledge.commit((3, 31), BTreeSet::new());
    let two = knowledge.commit((4, 30), BTreeSet::new());
    let three = knowledge.commit((5, 31), BTreeSet::new());

    let genesis = knowledge.genesis(knowledge.cut(d1()), &[one]);
    let middle = genesis.fork(&knowledge, introducing(&[two])).unwrap();
    let last = middle.fork(&knowledge, introducing(&[three])).unwrap();

    let archive = archived(&[&genesis, &middle, &last]);

    assert!(descends_from(&archive, last.id(), middle.id()).unwrap());
    assert!(
        descends_from(&archive, last.id(), genesis.id()).unwrap(),
        "the walk reaches beyond the immediate parent",
    );

    assert!(
        !descends_from(&archive, genesis.id(), last.id()).unwrap(),
        "ancestry runs one way; a parent does not descend from its child",
    );
}

/// The shape Synthesis meets when two Theses evolved apart: a shared ancestor, and neither
/// branch descending from the other.
#[test]
fn diverging_branches_share_an_ancestor_without_descending_from_each_other() {
    let mut knowledge = Fixture::new();
    let one = knowledge.commit((3, 31), BTreeSet::new());
    let two = knowledge.commit((4, 30), BTreeSet::new());
    let three = knowledge.commit((5, 31), BTreeSet::new());

    let root = knowledge.genesis(knowledge.cut(d1()), &[one]);
    let left = root.fork(&knowledge, introducing(&[two])).unwrap();
    let right = root.fork(&knowledge, introducing(&[three])).unwrap();

    let archive = archived(&[&root, &left, &right]);

    assert!(descends_from(&archive, left.id(), root.id()).unwrap());
    assert!(descends_from(&archive, right.id(), root.id()).unwrap());

    assert!(!descends_from(&archive, left.id(), right.id()).unwrap());
    assert!(
        !descends_from(&archive, right.id(), left.id()).unwrap(),
        "siblings are not ancestors of one another",
    );
}

/// And the shape that makes a Base incoherent: no walk from one lineage reaches the other.
#[test]
fn unrelated_lineages_descend_from_nothing_in_common() {
    let mut knowledge = Fixture::new();
    let one = knowledge.commit((3, 31), BTreeSet::new());
    let two = knowledge.commit((4, 30), BTreeSet::new());
    let three = knowledge.commit((5, 31), BTreeSet::new());

    let here = knowledge.genesis(knowledge.cut(d1()), &[one]);
    let elsewhere = knowledge.genesis(knowledge.cut(d1()), &[two]);
    let descendant = elsewhere.fork(&knowledge, introducing(&[three])).unwrap();

    let archive = archived(&[&here, &elsewhere, &descendant]);

    assert!(!descends_from(&archive, descendant.id(), here.id()).unwrap());
    assert!(!descends_from(&archive, here.id(), elsewhere.id()).unwrap());
}

/// An ancestor the archive does not hold is a `false`, not a failure: nothing it holds
/// could descend from it, since no child is stored before its parent.
#[test]
fn an_absent_ancestor_is_answered_rather_than_refused() {
    let mut knowledge = Fixture::new();
    let one = knowledge.commit((3, 31), BTreeSet::new());
    let genesis = knowledge.genesis(knowledge.cut(d1()), &[one]);

    let archive = archived(&[&genesis]);

    assert!(!descends_from(&archive, genesis.id(), ThesisId::from([9; 32])).unwrap());
}

/// The starting Thesis is different: without it there is no walk to report on.
#[test]
fn an_absent_starting_thesis_is_refused() {
    let archive = InMemoryArchive::default();
    let absent = ThesisId::from([9; 32]);

    assert!(matches!(
        descends_from(&archive, absent, absent),
        Err(ThesisError::UnknownThesis(id)) if id == absent
    ));
}
