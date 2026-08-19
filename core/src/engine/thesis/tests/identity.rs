//! A Thesis's identity, pinned: a known world against a known identity.
//!
//! The kernel pins its nine families beside `IdentityError`, where what decides an identity is
//! written down. This is the tenth, and it is here rather than there because a `Selection` is
//! assembled by a constructor visible only inside this module — reaching it from the kernel would
//! mean widening that to satisfy a test.
//!
//! # It pins more than the Thesis, and that is deliberate
//!
//! The commitments a world selects are themselves derived, so a red here can come from
//! `Commitment`'s declaration as easily as from `Thesis`'s. The kernel's table is what tells the two
//! apart: if it is green and this is red, the change is in `Thesis`, its `KnowledgeCut` or its
//! `Selection`. If both are red, it is the macro, the encoder, or a family below.
//!
//! Two worlds rather than one, because `parent: Option<ThesisId>` is one of the encodings a
//! derivation depends on, and a genesis exercises only the `None` side of it.
//!
//! These literals are a pin and not a prediction, for the reason the kernel's copy states.

use std::collections::BTreeSet;

use super::{Fixture, d1, introducing};

#[test]
fn a_genesis_identity_is_unchanged() {
    let mut knowledge = Fixture::new();
    let one = knowledge.commit((3, 31), BTreeSet::new());
    let two = knowledge.commit((6, 30), BTreeSet::new());

    let genesis = knowledge.genesis(knowledge.cut(d1()), &[one, two]);

    assert_eq!(
        genesis.id().to_string(),
        "478822f7b34f07dd43f138e512b2b98efc2e494a57d8b63717297d270274150c",
        "a Thesis identity moved. If that was deliberate, update this literal and say what it \
         breaks: a repository names the world a decision extends, so one written before the change \
         resolves that name to nothing"
    );
    assert_eq!(genesis.parent(), &None, "the None side of the parent");
}

#[test]
fn a_fork_identity_is_unchanged() {
    let mut knowledge = Fixture::new();
    let one = knowledge.commit((3, 31), BTreeSet::new());
    let two = knowledge.commit((6, 30), BTreeSet::new());

    let genesis = knowledge.genesis(knowledge.cut(d1()), &[one]);
    let fork = genesis.fork(&knowledge, introducing(&[two])).unwrap();

    assert_eq!(
        fork.id().to_string(),
        "f3cd48f71a0ecaa53bb05d4fc5c18a6e8b05efaa93de30d1c56bdfe1b2994084",
        "a Thesis identity moved, on the branch where a parent is named"
    );
    assert!(fork.parent().is_some(), "the Some side of the parent");
}
