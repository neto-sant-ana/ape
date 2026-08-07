//! What a Base must be to the two Theses it stands between.

use std::collections::BTreeSet;

use super::{Fixture, introducing};

use crate::engine::synthesis::base::coherent_base;
use crate::engine::thesis::{InMemoryArchive, Thesis, ThesisArchive, ThesisError, ThesisId};

fn archived(theses: &[&Thesis]) -> InMemoryArchive {
    let mut archive = InMemoryArchive::default();

    for thesis in theses {
        archive.put_thesis((*thesis).clone()).unwrap();
    }

    archive
}

/// The shape the operation exists for: two branches that grew apart from one world.
#[test]
fn a_common_ancestor_of_both_is_coherent() {
    let mut knowledge = Fixture::default();
    let shared = knowledge.commit((3, 31), BTreeSet::new());
    let here = knowledge.commit((4, 30), BTreeSet::new());
    let there = knowledge.commit((5, 31), BTreeSet::new());

    let base = knowledge.genesis(&[shared]);
    let source = base.fork(&knowledge, introducing(&[here])).unwrap();
    let target = base.fork(&knowledge, introducing(&[there])).unwrap();

    let archive = archived(&[&base, &source, &target]);

    assert!(coherent_base(&archive, base.id(), source.id(), target.id()).unwrap());
}

/// A Base the Target never passed through is the case that makes an omission remove
/// what the Target decided for itself.
#[test]
fn a_base_the_target_does_not_descend_from_is_incoherent() {
    let mut knowledge = Fixture::default();
    let shared = knowledge.commit((3, 31), BTreeSet::new());
    let here = knowledge.commit((4, 30), BTreeSet::new());
    let elsewhere = knowledge.commit((5, 31), BTreeSet::new());

    let base = knowledge.genesis(&[shared]);
    let source = base.fork(&knowledge, introducing(&[here])).unwrap();
    let unrelated = knowledge.genesis(&[elsewhere]);

    let archive = archived(&[&base, &source, &unrelated]);

    assert!(
        !coherent_base(&archive, base.id(), source.id(), unrelated.id()).unwrap(),
        "a difference measured over the Base may not be applied to a lineage that never held it",
    );
}

#[test]
fn a_base_the_source_does_not_descend_from_is_incoherent() {
    let mut knowledge = Fixture::default();
    let shared = knowledge.commit((3, 31), BTreeSet::new());
    let there = knowledge.commit((4, 30), BTreeSet::new());
    let elsewhere = knowledge.commit((5, 31), BTreeSet::new());

    let base = knowledge.genesis(&[shared]);
    let target = base.fork(&knowledge, introducing(&[there])).unwrap();
    let unrelated = knowledge.genesis(&[elsewhere]);

    let archive = archived(&[&base, &target, &unrelated]);

    assert!(
        !coherent_base(&archive, base.id(), unrelated.id(), target.id()).unwrap(),
        "an absence in a Thesis that never descended from the Base is not a decision",
    );
}

/// Reflexivity is what keeps the degenerate cases out of this refusal: they are answers of
/// their own — a fast-forward, and a transfer already applied — reached further along.
#[test]
fn a_base_equal_to_either_side_stays_coherent() {
    let mut knowledge = Fixture::default();
    let shared = knowledge.commit((3, 31), BTreeSet::new());
    let added = knowledge.commit((4, 30), BTreeSet::new());

    let base = knowledge.genesis(&[shared]);
    let descendant = base.fork(&knowledge, introducing(&[added])).unwrap();

    let archive = archived(&[&base, &descendant]);

    assert!(
        coherent_base(&archive, base.id(), descendant.id(), base.id()).unwrap(),
        "Base equal to Target is a fast-forward, not an incoherent Base",
    );
    assert!(
        coherent_base(&archive, base.id(), base.id(), descendant.id()).unwrap(),
        "Base equal to Source leaves an empty difference, not an incoherent Base",
    );
}

/// A Base outside the archive is an answer rather than a failure: nothing it holds descends
/// from a record it does not have.
#[test]
fn a_base_absent_from_the_archive_is_incoherent() {
    let mut knowledge = Fixture::default();
    let shared = knowledge.commit((3, 31), BTreeSet::new());
    let added = knowledge.commit((4, 30), BTreeSet::new());

    let source = knowledge.genesis(&[shared]);
    let target = source.fork(&knowledge, introducing(&[added])).unwrap();

    let archive = archived(&[&source, &target]);

    assert!(!coherent_base(&archive, ThesisId::from([9; 32]), source.id(), target.id()).unwrap());
}

/// A Source outside the archive is different: there is no lineage to walk at all.
#[test]
fn a_source_absent_from_the_archive_is_refused() {
    let mut knowledge = Fixture::default();
    let shared = knowledge.commit((3, 31), BTreeSet::new());
    let base = knowledge.genesis(&[shared]);

    let archive = archived(&[&base]);
    let absent = ThesisId::from([9; 32]);

    assert!(matches!(
        coherent_base(&archive, base.id(), absent, base.id()),
        Err(ThesisError::UnknownThesis(id)) if id == absent
    ));
}
