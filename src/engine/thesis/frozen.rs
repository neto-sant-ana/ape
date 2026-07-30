//! The frozen causal past: what a recognized Event Head makes unavoidable.
//!
//! ```text
//! Frozen(H) = Settled(H) ∪ Ancestors(Settled(H))
//! ```
//!
//! This is the only place in the layer that reads the Event chain, and both walks below are
//! the same walk. What an advancement needs is the settlement between two heads; settlement
//! at a head is that walk with the start of the chain as its origin.

use std::collections::BTreeSet;

use super::ThesisError;

use crate::kernel::axiom::Knowledge;

use crate::kernel::entities::{CommitmentId, EventId};

pub(super) fn settled_at<K: Knowledge>(
    knowledge: &K,
    head: Option<EventId>,
) -> Result<BTreeSet<CommitmentId>, ThesisError> {
    match head {
        None => Ok(BTreeSet::new()),
        Some(head) => settled_between(knowledge, None, head),
    }
}

/// The walk runs backwards from `target`, so it reads only the segment it reports on.
/// Reaching the start of the chain without meeting `origin` is what proves `target` never
/// descended from it.
pub(super) fn settled_between<K: Knowledge>(
    knowledge: &K,
    origin: Option<EventId>,
    target: EventId,
) -> Result<BTreeSet<CommitmentId>, ThesisError> {
    let mut settled = BTreeSet::new();
    let mut cursor = Some(target);

    while cursor != origin {
        let Some(id) = cursor else {
            return Err(ThesisError::HeadDoesNotDescend {
                parent: origin,
                target,
            });
        };

        let event = knowledge.event(id).ok_or(ThesisError::UnknownEvent(id))?;

        settled.insert(*event.commitment_id());
        cursor = *event.previous_event();
    }

    Ok(settled)
}

/// `seeds` closed upwards over dependencies.
pub(super) fn with_ancestors<K: Knowledge>(
    knowledge: &K,
    seeds: BTreeSet<CommitmentId>,
) -> Result<BTreeSet<CommitmentId>, ThesisError> {
    let mut pending: Vec<CommitmentId> = seeds.iter().copied().collect();
    let mut closed = seeds;

    while let Some(id) = pending.pop() {
        let commitment = knowledge
            .commitment(id)
            .ok_or(ThesisError::UnknownCommitment(id))?;

        for dependency in commitment.dependencies() {
            if closed.insert(*dependency) {
                pending.push(*dependency);
            }
        }
    }

    Ok(closed)
}

/// This is what makes a Thesis projectable by construction. Hermeneia resolves a
/// commitment's dependencies out of the selection it was handed, so a dangling edge is not
/// an odd graph.
pub(super) fn ensure_closed<K: Knowledge>(
    knowledge: &K,
    selection: &BTreeSet<CommitmentId>,
) -> Result<(), ThesisError> {
    for id in selection {
        let commitment = knowledge
            .commitment(*id)
            .ok_or(ThesisError::UnknownCommitment(*id))?;

        for dependency in commitment.dependencies() {
            if !selection.contains(dependency) {
                return Err(ThesisError::DanglingDependency {
                    dependent: *id,
                    dependency: *dependency,
                });
            }
        }
    }

    Ok(())
}
