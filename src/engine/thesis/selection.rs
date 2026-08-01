//! `Selection` — the complete Commitment graph a Thesis denotes, split by what may still change.
//!
//! ```text
//! frozen  ← what the recognized cut made unavoidable
//! open    ← what a fork may still preserve, omit or replace
//! ```
//!
//! The two halves are disjoint by construction rather than by agreement: what history imposed
//! is not revisable, so a commitment offered as open while already frozen is simply frozen.
//! Refusing it would be wrong — selecting what the cut already imposed changes nothing — and
//! leaving it in both would make the partition a claim instead of a fact.
//!
//! Nothing here exposes the collection underneath. A caller reads the selection by iterating
//! it, asking whether an id belongs, or resolving it whole; the concrete structure is one
//! decision in one place, which is what lets it change without changing what a Thesis means.
//!
//! Both halves are materialized: a Thesis carries every id it selects rather than a delta over
//! its parent, so a lineage holds O(n) ids per Thesis and reads n records to build one, and
//! nothing resolves a selection by walking ancestry. That is what buys an invariant checked in
//! a single place, an identity derived from what is present, and a world interpretable without
//! consulting anything but itself.
//!
//! The semantics admit a persistent representation — structural sharing, a delta over ancestry,
//! periodic materialization — because identity is derived from the *resolved* selection and not
//! from its encoding. Sharing the storage alone would not remove the cost per derivation
//! though: the whole selection is materialized to be hashed whatever lies underneath. Removing
//! both costs at once means hashing hierarchically, where an unchanged region contributes its
//! own hash and only the path to a change is recomputed. Neither belongs here before a Thesis
//! is stored somewhere.
//!
//! Whenever that arrives, the hierarchy has to be addressed by key rather than by position. A
//! root over ordered ids is deterministic but not prunable: a leaf's place depends on every id
//! below it, so a single insertion rewrites nearly all of them. Keyed by the bits of a
//! `CommitmentId` — uniform already, being a hash — an insertion touches one path, and the two
//! halves keep roots of their own, so an unchanged past proves itself apart from an unchanged
//! intention.
//!
//! Such a root would define the identity and the serialized storage form would not. That
//! divergence is worth stating before it exists: deriving identity from the encoding is what
//! makes the encoding impossible to change afterwards.

use std::collections::BTreeSet;

use crate::kernel::entities::CommitmentId;

define_value_object! {
    pub struct Selection {
        frozen: BTreeSet<CommitmentId>,
        open: BTreeSet<CommitmentId>,
    }
}
impl Selection {
    /// Split `open` against `frozen`, keeping the halves disjoint.
    pub(super) fn partitioned(
        frozen: BTreeSet<CommitmentId>,
        open: BTreeSet<CommitmentId>,
    ) -> Self {
        let open = open.difference(&frozen).copied().collect();

        Self { frozen, open }
    }

    /// The commitments the recognized cut made unavoidable.
    pub fn frozen(&self) -> impl Iterator<Item = CommitmentId> + '_ {
        self.frozen.iter().copied()
    }

    /// The commitments a fork may still revise.
    pub fn open(&self) -> impl Iterator<Item = CommitmentId> + '_ {
        self.open.iter().copied()
    }

    /// The complete graph, in no particular order: a Thesis is a set, and every reader of one
    /// answers the same whichever way it is walked.
    pub fn resolved(&self) -> impl Iterator<Item = CommitmentId> + '_ {
        self.frozen.iter().chain(self.open.iter()).copied()
    }

    pub fn contains(&self, commitment: CommitmentId) -> bool {
        self.frozen.contains(&commitment) || self.open.contains(&commitment)
    }

    pub fn is_frozen(&self, commitment: CommitmentId) -> bool {
        self.frozen.contains(&commitment)
    }

    pub fn len(&self) -> usize {
        self.frozen.len() + self.open.len()
    }

    pub fn is_empty(&self) -> bool {
        self.frozen.is_empty() && self.open.is_empty()
    }
}
