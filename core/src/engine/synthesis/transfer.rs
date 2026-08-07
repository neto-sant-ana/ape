//! `ResolvedTransfer` — the Source difference, minus what the Target already satisfies.
//!
//! A difference is expressed relative to the Base. Before it can be evaluated it is resolved
//! against the Target, because a decision the Target already reflects asks nothing of it.
//!
//! ```text
//! remove    = omitted    ∩ Commitments(Target)
//! introduce = introduced − Commitments(Target)
//! ```
//!
//! Both sides read the Target whole, and that is not an oversight of the partition the
//! difference was careful about. The two ask different questions. A difference asks what was
//! *decided*, and only an open commitment can be — so it reads `Open` on the side being
//! judged. A resolved transfer asks what is *left to do*, and presence is presence: a
//! commitment the Target holds is there whether history put it there or a planner did.
//!
//! So a commitment already frozen in the Target is not introduced — it is already selected,
//! which makes the introduction idempotent rather than impossible. And an omission of
//! something frozen in the Target does resolve to a removal: the transfer asks for it, and
//! whether it may happen is a judgment made afterwards, over this result. Deciding it here
//! would hide a refusal inside a resolution.
//!
//! An empty resolved transfer is not a failure. It means the Target already contains the
//! difference, which is a conclusion of its own and reported as such.

use std::collections::BTreeSet;

use super::IntentionalDifference;

use crate::engine::thesis::Thesis;

use crate::kernel::entities::CommitmentId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedTransfer {
    remove: BTreeSet<CommitmentId>,
    introduce: BTreeSet<CommitmentId>,
}
impl ResolvedTransfer {
    pub(super) fn resolving(difference: &IntentionalDifference, target: &Thesis) -> Self {
        Self {
            remove: difference
                .omitted()
                .filter(|id| target.selection().contains(*id))
                .collect(),
            introduce: difference
                .introduced()
                .filter(|id| !target.selection().contains(*id))
                .collect(),
        }
    }

    /// Omitted by the Source and still held by the Target.
    pub fn remove(&self) -> impl Iterator<Item = CommitmentId> + '_ {
        self.remove.iter().copied()
    }

    /// Introduced by the Source and absent from the Target.
    pub fn introduce(&self) -> impl Iterator<Item = CommitmentId> + '_ {
        self.introduce.iter().copied()
    }

    /// Whether the Target already satisfies every change the difference asked for.
    pub fn is_empty(&self) -> bool {
        self.remove.is_empty() && self.introduce.is_empty()
    }
}
