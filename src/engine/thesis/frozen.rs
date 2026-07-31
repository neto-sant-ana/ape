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

use super::{KnowledgeCut, ThesisError};

use crate::canon::CanonicalKnowledge;

use crate::kernel::entities::{Commitment, CommitmentId, Event, EventId};

/// Every commitment settled by the chain reachable from `head`.
pub(super) fn settled_at<K: CanonicalKnowledge>(
    knowledge: &K,
    head: Option<EventId>,
) -> Result<BTreeSet<CommitmentId>, ThesisError> {
    match head {
        None => Ok(BTreeSet::new()),
        Some(head) => settled_between(knowledge, None, head),
    }
}

/// Every commitment settled after `origin` and through `target`.
///
/// The walk runs backwards from `target`, so it reads only the segment it reports on.
/// Reaching the start of the chain without meeting `origin` is what proves `target` never
/// descended from it. A `target` equal to `origin` settles nothing, which is the advancement
/// that recognizes later knowledge without observing an Event.
pub(super) fn settled_between<K: CanonicalKnowledge>(
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

        let event = read_event(knowledge, id)?;

        settled.insert(*event.commitment_id());
        cursor = *event.previous_event();
    }

    Ok(settled)
}

/// `seeds` closed upwards over dependencies.
pub(super) fn with_ancestors<K: CanonicalKnowledge>(
    knowledge: &K,
    seeds: BTreeSet<CommitmentId>,
) -> Result<BTreeSet<CommitmentId>, ThesisError> {
    let mut pending: Vec<CommitmentId> = seeds.iter().copied().collect();
    let mut closed = seeds;

    while let Some(id) = pending.pop() {
        let commitment = read_commitment(knowledge, id)?;

        for dependency in commitment.dependencies() {
            if closed.insert(*dependency) {
                pending.push(*dependency);
            }
        }
    }

    Ok(closed)
}

/// Refuse a selection that is not a world the cut can interpret.
///
/// One read per commitment answers both questions, because the canonical record carries the
/// dependencies and the recording instant together:
///
/// - a dependency outside the selection leaves an edge Hermeneia cannot resolve, since it
///   resolves dependencies out of the selection it was handed;
/// - a commitment recorded after the cut was not knowledge there, and selecting it would
///   claim an intention that could not yet have been formed.
///
/// The second refusal also covers a case no caller asked for: the causal ancestors pulled in
/// by the frozen past. An Event recognized by the cut settling a Commitment recorded after it
/// means the cut and the history disagree, and the answer is to refuse rather than to
/// compensate.
pub(super) fn ensure_selectable<K: CanonicalKnowledge>(
    knowledge: &K,
    selection: &BTreeSet<CommitmentId>,
    cut: &KnowledgeCut,
) -> Result<(), ThesisError> {
    for id in selection {
        let record = knowledge
            .canonical_commitment(*id)
            .ok_or(ThesisError::UnknownCommitment(*id))?;

        if !record.recorded_at().up_to(cut.known_at()) {
            return Err(ThesisError::CommitmentNotKnownAtCut {
                commitment: *id,
                recorded_at: *record.recorded_at(),
                known_at: *cut.known_at(),
            });
        }

        for dependency in record.assertion().dependencies() {
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

fn read_commitment<K: CanonicalKnowledge>(
    knowledge: &K,
    id: CommitmentId,
) -> Result<Commitment, ThesisError> {
    knowledge
        .canonical_commitment(id)
        .map(|record| record.assertion().clone())
        .ok_or(ThesisError::UnknownCommitment(id))
}

fn read_event<K: CanonicalKnowledge>(knowledge: &K, id: EventId) -> Result<Event, ThesisError> {
    knowledge
        .canonical_event(id)
        .map(|record| record.assertion().clone())
        .ok_or(ThesisError::UnknownEvent(id))
}
