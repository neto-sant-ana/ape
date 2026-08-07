//! `IntentionalDifference` — what a Source decided, told apart from what history imposed.
//!
//! A Thesis selects a complete graph, but only part of it was ever chosen. The frozen past
//! grows on its own as the recognized chain advances, so comparing two selections whole
//! would read historical imposition as intention.
//!
//! ```text
//! omitted    = Open(Base)   − Commitments(Source)
//! introduced = Open(Source) − Commitments(Base)
//! ```
//!
//! The two sides of each subtraction are deliberately different, and neither may be made to
//! match the other. `Open` on the left is what could have been decided: a commitment already
//! frozen in the Base was never the Source's to drop, and a commitment frozen in the Source
//! entered because an Event settled it, not because anyone chose it. `Commitments` on the
//! right is what the other side holds *in any partition*: a commitment the Source carries as
//! frozen is not omitted merely for having left its open future, and one the Base already
//! held is not introduced for being open in the Source.
//!
//! Nothing here pairs an omission with an introduction. Commitments are immutable, so
//! replacing one is dropping one and selecting another, and which of them replaces which is
//! knowledge the graph does not carry.

use std::collections::BTreeSet;

use crate::engine::thesis::Thesis;

use crate::kernel::entities::CommitmentId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntentionalDifference {
    omitted: BTreeSet<CommitmentId>,
    introduced: BTreeSet<CommitmentId>,
}
impl IntentionalDifference {
    pub(super) fn between(base: &Thesis, source: &Thesis) -> Self {
        let held_by_base: BTreeSet<CommitmentId> = base.selection().resolved().collect();
        let held_by_source: BTreeSet<CommitmentId> = source.selection().resolved().collect();

        Self {
            omitted: base
                .selection()
                .open()
                .filter(|id| !held_by_source.contains(id))
                .collect(),
            introduced: source
                .selection()
                .open()
                .filter(|id| !held_by_base.contains(id))
                .collect(),
        }
    }

    /// Open in the Base and absent from the Source: a decision to drop.
    pub fn omitted(&self) -> impl Iterator<Item = CommitmentId> + '_ {
        self.omitted.iter().copied()
    }

    /// Open in the Source and absent from the Base: a decision to add.
    pub fn introduced(&self) -> impl Iterator<Item = CommitmentId> + '_ {
        self.introduced.iter().copied()
    }

    /// Whether the Source decided nothing the Base had not.
    pub fn is_empty(&self) -> bool {
        self.omitted.is_empty() && self.introduced.is_empty()
    }
}
