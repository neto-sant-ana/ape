//! `Thesis` — one complete, historically closed continuation of intended evolution.
//!
//! A Thesis is not an assertion about operational reality; it is a selection over
//! assertions. Nothing it holds is new knowledge: every commitment it names was admitted
//! through the Axiom and the Canon, and every Event it recognizes belongs to the one shared
//! chain. What it adds is a boundary of commitments, at a knowledge cut with an associated identity.
//!
//! Three invariants make the boundary meaningful, and together they are all this layer
//! enforces:
//!
//! - **historically closed** — every commitment settled in the chain up to the recognized
//!   head is selected.
//! - **dependency closed** — every dependency of a selected commitment is selected.
//! - **free of anachronism** — every selected commitment was recorded no later than the cut.
//!
//! The cut is part of what a Thesis means, not of where it came from. That is what makes
//! projectability intrinsic: `T@K` is projectable at the head `K` recognizes, decidable from
//! `T` alone.
//!
//! Three derivations, each moving exactly one axis:
//!
//! ```text
//! genesis                  → no parent, a declared cut
//! fork(parent, Δintent)    → the parent's cut, a different intention
//! advance(parent, K')      → the parent's intention, a strictly later cut
//! ```
//!
//! Keeping them disjoint is what lets an ancestry edge say why it exists. A commitment absent
//! from a parent was either introduced by a decision or imposed by history, and nothing
//! downstream could tell the two apart if one operation could do both.
//!
//! > _Advance changes what could be known, without deciding what should be intended._
//!
//! An advancement therefore never adds an intention. A commitment admitted between the two
//! cuts and not settled does not enter: it merely becomes eligible for a fork afterwards,
//! which is why planning runs `advance` then `fork` rather than one operation doing both.
//!
//! The selection is held as the frozen/open partition rather than as one set. That split is a
//! function of the cut, so it is derived exactly once and inherited unchanged by every fork.

use std::collections::BTreeSet;

use super::frozen::{ensure_selectable, settled_at, settled_between, with_ancestors};
use super::{Advancement, KnowledgeCut, ThesisError};

use crate::canon::CanonicalKnowledge;

use crate::kernel::entities::CommitmentId;

define_id!(ThesisId);
define_entity! {
    pub struct Thesis(ThesisId) via ThesisInput {
        parent: Option<ThesisId>,
        cut: KnowledgeCut,
        frozen: BTreeSet<CommitmentId>,
        open: BTreeSet<CommitmentId>,
    }
}

/// The selection is a proposal: whatever the cut makes unavoidable is added to it, which is
/// what allows a genesis at a cut canonical history has already reached.
pub struct GenesisInput {
    pub cut: KnowledgeCut,
    pub selection: BTreeSet<CommitmentId>,
}

/// `omitted` names commitments of the parent's open future;
/// `introduced` names any commitment admitted by the parent's cut.
pub struct ForkInput {
    pub omitted: BTreeSet<CommitmentId>,
    pub introduced: BTreeSet<CommitmentId>,
}

impl Thesis {
    pub fn genesis<K: CanonicalKnowledge>(
        knowledge: &K,
        input: GenesisInput,
    ) -> Result<Self, ThesisError> {
        let frozen = with_ancestors(knowledge, settled_at(knowledge, input.cut.event_head())?)?;
        let open = input.selection.difference(&frozen).copied().collect();

        Self::assemble(
            knowledge,
            ThesisInput {
                parent: None,
                cut: input.cut,
                frozen,
                open,
            },
        )
    }

    /// An introduced commitment that is already frozen is dropped rather than added, which
    /// keeps the two halves of the partition disjoint; selecting what history already
    /// imposed changes nothing.
    pub fn fork<K: CanonicalKnowledge>(
        &self,
        knowledge: &K,
        input: ForkInput,
    ) -> Result<Self, ThesisError> {
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
                cut: self.cut.clone(),
                frozen: self.frozen.clone(),
                open,
            },
        )
    }

    /// The same intention under a strictly later cut.
    ///
    /// Strictness lives on `known_at`, not on the head. Two cuts sharing a head are a genuine
    /// advancement — commitments were admitted and no Event was observed — while advancing to
    /// the cut already recognized would be a fork that changed nothing but its parent, and the
    /// ancestry edge would stop naming an axis.
    ///
    /// The head may hold or move forward. Giving it back is refused: a cut recognizing no
    /// Event where its parent recognized one un-knows a fact, which is not a way of knowing
    /// more.
    pub fn advance<K: CanonicalKnowledge>(
        &self,
        knowledge: &K,
        target: KnowledgeCut,
    ) -> Result<Advancement, ThesisError> {
        if target.known_at().up_to(self.cut.known_at()) {
            return Err(ThesisError::CutNotLater {
                parent: *self.cut.known_at(),
                target: *target.known_at(),
            });
        }

        let settled = match (self.cut.event_head(), target.event_head()) {
            (Some(parent), None) => return Err(ThesisError::HeadWithdrawn { parent }),
            (_, None) => BTreeSet::new(),
            (parent, Some(head)) => settled_between(knowledge, parent, head)?,
        };

        let mut frozen = self.frozen.clone();
        frozen.extend(with_ancestors(knowledge, settled)?);

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
                cut: target,
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
    fn assemble<K: CanonicalKnowledge>(
        knowledge: &K,
        input: ThesisInput,
    ) -> Result<Self, ThesisError> {
        let selection = input.frozen.union(&input.open).copied().collect();
        ensure_selectable(knowledge, &selection, &input.cut)?;

        Ok(Self::create(input)?)
    }
}
