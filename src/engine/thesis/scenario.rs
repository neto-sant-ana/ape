//! `Thesis` — one complete, historically closed continuation of intended evolution.
//!
//! A Thesis is not an assertion about operational reality; it is a selection over
//! assertions. Nothing it holds is new knowledge: every commitment it names was admitted
//! through the Axiom and the Canon, and every Event it recognizes belongs to the one shared
//! chain. What it adds is a boundary of commitments, at a factual cut with an associated identity.
//!
//! Two invariants make the boundary meaningful, and together they are all this layer
//! enforces:
//!
//! - **historically closed** — every commitment settled in the chain up to the recognized
//!   head is selected.
//! - **dependency closed** — every dependency of a selected commitment is selected.
//!
//! The head is part of what a Thesis means, not of where it came from. That is what makes
//! projectability intrinsic: `T@H` is projectable at `H`, decidable from `T` alone.
//!
//! Three derivations, each moving exactly one axis:
//!
//! ```text
//! genesis                  → no parent, a chosen head
//! fork(parent, Δintent)    → the parent's head, a different intention
//! advance(parent, H')      → the parent's intention, a strictly later head
//! ```
//!
//! Keeping them disjoint is what lets an ancestry edge say why it exists. A commitment absent
//! from a parent was either introduced by a decision or imposed by history, and nothing
//! downstream could tell the two apart if one operation could do both.
//!
//! The selection is held as the frozen/open partition rather than as one set. That split is a
//! function of the head, so it is derived exactly once and inherited unchanged by every fork.

use std::collections::BTreeSet;

use super::frozen::{ensure_closed, settled_at, settled_between, with_ancestors};
use super::{Advancement, ThesisError};

use crate::kernel::axiom::Knowledge;

use crate::kernel::entities::{CommitmentId, EventId};

define_id!(ThesisId);
define_entity! {
    pub struct Thesis(ThesisId) via ThesisInput {
        parent: Option<ThesisId>,
        head: Option<EventId>,
        frozen: BTreeSet<CommitmentId>,
        open: BTreeSet<CommitmentId>,
    }
}

/// The selection is a proposal: whatever the head makes unavoidable is added to it, which is
/// what allows a genesis at a head canonical history has already reached.
pub struct GenesisInput {
    pub head: Option<EventId>,
    pub selection: BTreeSet<CommitmentId>,
}

/// `omitted` names commitments of the parent's open future;
/// `introduced` names any admitted commitment.
pub struct ForkInput {
    pub omitted: BTreeSet<CommitmentId>,
    pub introduced: BTreeSet<CommitmentId>,
}

impl Thesis {
    pub fn genesis<K: Knowledge>(knowledge: &K, input: GenesisInput) -> Result<Self, ThesisError> {
        let frozen = with_ancestors(knowledge, settled_at(knowledge, input.head)?)?;
        let open = input.selection.difference(&frozen).copied().collect();

        Self::assemble(
            knowledge,
            ThesisInput {
                parent: None,
                head: input.head,
                frozen,
                open,
            },
        )
    }

    /// An introduced commitment that is already frozen is dropped rather than added, which
    /// keeps the two halves of the partition disjoint; selecting what history already
    /// imposed changes nothing.
    pub fn fork<K: Knowledge>(&self, knowledge: &K, input: ForkInput) -> Result<Self, ThesisError> {
        if let Some(both) = input.omitted.intersection(&input.introduced).next() {
            return Err(ThesisError::OmittedAndIntroduced(*both));
        }

        if let Some(unavoidable) = input.omitted.intersection(&self.frozen).next() {
            return Err(ThesisError::FrozenPastOmitted(*unavoidable));
        }

        let mut open: BTreeSet<CommitmentId> =
            self.open.difference(&input.omitted).copied().collect();
        open.extend(input.introduced.difference(&self.frozen).copied());

        Self::assemble(
            knowledge,
            ThesisInput {
                parent: Some(self.id()),
                head: self.head,
                frozen: self.frozen.clone(),
                open,
            },
        )
    }

    /// Strictness is what keeps the two derivations disjoint: advancing to the head already
    /// recognized would be a fork that changed nothing but its parent, and the ancestry edge
    /// would stop naming an axis.
    pub fn advance<K: Knowledge>(
        &self,
        knowledge: &K,
        target: EventId,
    ) -> Result<Advancement, ThesisError> {
        if self.head == Some(target) {
            return Err(ThesisError::AlreadyAtHead(target));
        }

        let mut frozen = self.frozen.clone();
        frozen.extend(with_ancestors(
            knowledge,
            settled_between(knowledge, self.head, target)?,
        )?);

        let imposed: BTreeSet<CommitmentId> = frozen
            .difference(&self.frozen)
            .filter(|id| !self.open.contains(id))
            .copied()
            .collect();

        let open = self.open.difference(&frozen).copied().collect();

        let thesis = Self::assemble(
            knowledge,
            ThesisInput {
                parent: Some(self.id()),
                head: Some(target),
                frozen,
                open,
            },
        )?;

        Ok(Advancement::new(thesis, imposed))
    }

    pub fn selection(&self) -> Vec<CommitmentId> {
        self.frozen.union(&self.open).copied().collect()
    }

    /// `ThesisInput` doubles as the request: the fields the identity is derived from are the
    /// ones the invariants are checked against, so there is no second list of fields to keep
    /// in step.
    fn assemble<K: Knowledge>(knowledge: &K, input: ThesisInput) -> Result<Self, ThesisError> {
        let selection = input.frozen.union(&input.open).copied().collect();
        ensure_closed(knowledge, &selection)?;

        Ok(Self::create(input)?)
    }
}
