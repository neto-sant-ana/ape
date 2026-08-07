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
//! projectability intrinsic: `T@K` is projectable only under `K` — its factual chain fixed by
//! `K.event_head`, its selectable Commitments bounded by `K.known_at` — and that is decidable
//! from `T` alone.
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
//! [`Selection`] owns it, together with what representing it costs.

use std::collections::BTreeSet;

use super::frozen::{ensure_selectable, settled_at, settled_between, with_ancestors};
use super::{Advancement, KnowledgeCut, Selection, ThesisError};

use crate::canon::CanonicalKnowledge;

use crate::kernel::entities::CommitmentId;

define_id!(ThesisId);
define_entity! {
    pub struct Thesis(ThesisId) via ThesisInput {
        parent: Option<ThesisId>,
        cut: KnowledgeCut,
        selection: Selection,
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

        Self::assemble(
            knowledge,
            ThesisInput {
                parent: None,
                cut: input.cut,
                selection: Selection::partitioned(frozen, input.selection),
            },
        )
    }

    /// A fork must change what is selected.
    ///
    /// Its cut is its parent's by construction, so a fork that selects the same graph would be
    /// an ancestry edge carrying neither a decision nor an observation — and a Thesis is what a
    /// change in selection or in recognized cut produces. Pinning a world under a name is what
    /// a reference is for, and a reference costs no Thesis.
    ///
    /// Redundancy within the request is tolerated, because omission and introduction state an
    /// outcome rather than a transition. Omitting what the parent never selected is silence: the
    /// outcome asked for already holds. Introducing what is already selected is the same. What
    /// is refused is an outcome that *cannot* hold — omitting a frozen commitment — and a
    /// request whose outcome is the parent itself.
    pub fn fork<K: CanonicalKnowledge>(
        &self,
        knowledge: &K,
        input: ForkInput,
    ) -> Result<Self, ThesisError> {
        if let Some(both) = input.omitted.intersection(&input.introduced).next() {
            return Err(ThesisError::OmittedAndIntroduced(*both));
        }

        if let Some(unavoidable) = input
            .omitted
            .iter()
            .find(|id| self.selection.is_frozen(**id))
        {
            return Err(ThesisError::FrozenPastOmitted(*unavoidable));
        }

        let open: BTreeSet<CommitmentId> = self
            .selection
            .open()
            .filter(|id| !input.omitted.contains(id))
            .chain(input.introduced.iter().copied())
            .collect();

        let selection = Selection::partitioned(self.selection.frozen().collect(), open);

        if selection == self.selection {
            return Err(ThesisError::SelectionUnchanged);
        }

        Self::assemble(
            knowledge,
            ThesisInput {
                parent: Some(self.id()),
                cut: self.cut.clone(),
                selection,
            },
        )
    }

    /// The same intention under a later cut.
    ///
    /// Later is a property of the whole cut, not of its instant alone. Neither coordinate may
    /// regress and at least one must advance, which admits both advancements that exist: the
    /// instant moving while the head holds — Commitments were admitted and no Event observed — and
    /// the head moving within one instant, where a finer cut is refined by the Events of its own
    /// group. Advancing to the cut already recognized is refused, since it would be a fork that
    /// changed nothing but its parent.
    ///
    /// Only the instant is compared here. That the head does not regress is proved by the walk
    /// that resolves the segment, which refuses a target the recognized head does not reach —
    /// there is no second comparison to keep in step with it.
    pub fn advance<K: CanonicalKnowledge>(
        &self,
        knowledge: &K,
        target: KnowledgeCut,
    ) -> Result<Advancement, ThesisError> {
        let same_instant = target.known_at() == self.cut.known_at();
        let regressed = target.known_at() < self.cut.known_at();

        if regressed || (same_instant && target.event_head() == self.cut.event_head()) {
            return Err(ThesisError::CutNotLater {
                parent: *self.cut.known_at(),
                parent_head: self.cut.event_head(),
                target: *target.known_at(),
                target_head: target.event_head(),
            });
        }

        let settled = match (self.cut.event_head(), target.event_head()) {
            (Some(parent), None) => return Err(ThesisError::HeadWithdrawn { parent }),
            (_, None) => BTreeSet::new(),
            (parent, Some(head)) => settled_between(knowledge, parent, head)?,
        };

        let frozen: BTreeSet<CommitmentId> = self
            .selection
            .frozen()
            .chain(with_ancestors(knowledge, settled)?)
            .collect();

        // Frozen(H') − Commitments(T): what the cut required and the parent never selected,
        // which is why a commitment merely moving from open to frozen is not imposed.
        let imposed: BTreeSet<CommitmentId> = frozen
            .iter()
            .filter(|id| !self.selection.contains(**id))
            .copied()
            .collect();

        let thesis = Self::assemble(
            knowledge,
            ThesisInput {
                parent: Some(self.id()),
                cut: target,
                selection: Selection::partitioned(frozen, self.selection.open().collect()),
            },
        )?;

        Ok(Advancement::new(thesis, imposed))
    }

    /// `ThesisInput` doubles as the request: the fields the identity is derived from are the
    /// ones the invariants are checked against, so there is no second list of fields to keep
    /// in step.
    fn assemble<K: CanonicalKnowledge>(
        knowledge: &K,
        input: ThesisInput,
    ) -> Result<Self, ThesisError> {
        ensure_selectable(knowledge, &input.selection, &input.cut)?;

        Ok(Self::create(input)?)
    }
}
