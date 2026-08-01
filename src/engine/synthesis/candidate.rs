//! `CandidateSelection` — the world a transfer would produce, derived to be judged.
//!
//! ```text
//! Frozen(Candidate) = Frozen(Target)
//! Open(Candidate)   = Open(Target) − remove ∪ introduce
//! ```
//!
//! It is not a `Selection` and not a Thesis. A Thesis pairs a selection with the cut it
//! recognizes and carries an identity derived from both; a candidate is an argument about
//! what would follow, held only long enough to be checked. Giving it the type of a world
//! would invite storing one, and constructing a Thesis belongs to the Thesis layer.
//!
//! **The frozen half is carried over whole**, and that is what keeps the candidate honest.
//! Historical closure needs no verification here: `Frozen(Kt)` is in the Target because a
//! Thesis is historically closed, and an effective introduction is by definition absent from
//! the Target, so it cannot be frozen there and lands in the open future. Neither fact is
//! checked, because neither can fail — checking them would be validating the unvalidatable.
//!
//! What *can* be asked for and cannot happen is removing a frozen commitment. That request
//! is simply not expressible here: the frozen half does not consult the removals. The
//! candidate therefore never quietly drops a fact, and refusing such a transfer stays a
//! conflict reported over the result rather than a silence buried in it.

use std::collections::BTreeSet;

use super::ResolvedTransfer;

use crate::engine::thesis::Thesis;

use crate::kernel::entities::CommitmentId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateSelection {
    frozen: BTreeSet<CommitmentId>,
    open: BTreeSet<CommitmentId>,
}
impl CandidateSelection {
    pub(super) fn deriving(transfer: &ResolvedTransfer, target: &Thesis) -> Self {
        let removed: BTreeSet<CommitmentId> = transfer.remove().collect();

        Self {
            frozen: target.selection().frozen().collect(),
            open: target
                .selection()
                .open()
                .filter(|id| !removed.contains(id))
                .chain(transfer.introduce())
                .collect(),
        }
    }

    /// The Target's frozen past, which a transfer does not reach.
    pub fn frozen(&self) -> impl Iterator<Item = CommitmentId> + '_ {
        self.frozen.iter().copied()
    }

    pub fn open(&self) -> impl Iterator<Item = CommitmentId> + '_ {
        self.open.iter().copied()
    }

    /// Everything the candidate selects, in either half.
    pub fn resolved(&self) -> impl Iterator<Item = CommitmentId> + '_ {
        self.frozen().chain(self.open())
    }

    pub fn contains(&self, commitment: CommitmentId) -> bool {
        self.frozen.contains(&commitment) || self.open.contains(&commitment)
    }
}
