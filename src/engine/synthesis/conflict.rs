//! What an invariant known to APE says about a transfer.
//!
//! A conflict is evidence, never an opinion: each one names a rule the resulting world would
//! break. A planner regarding two commitments as alternatives is not a conflict, because the
//! graph cannot tell competition from coexistence — and a report that guessed would be
//! judging intention.
//!
//! Two of them report the same broken invariant, dependency closure, and they are told apart
//! by the **origin of the absence** rather than by an order of precedence:
//!
//! ```text
//! dependency absent from the candidate, and present in the Target
//! → DependencyBreakage
//!
//! dependency absent from the candidate, and never present
//! → MissingDependency
//! ```
//!
//! So an introduction whose dependency the same transfer removes is a breakage — something
//! that existed was broken — and the classification is decided by the candidate rather than
//! by which check ran first.
//!
//! Detection is ordered by category and then by commitment id, because a report is compared
//! and cached, and an order that followed iteration would make equal analyses look different.

use std::collections::BTreeSet;

use super::{CandidateSelection, ResolvedTransfer};

use crate::canon::CanonicalKnowledge;

use crate::engine::thesis::{Thesis, ThesisError};

use crate::kernel::entities::CommitmentId;

use crate::kernel::value_objects::Date;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicabilityConflict {
    /// The transfer asks to remove a commitment the Target's history made unavoidable.
    HistoricalFreezing { commitment: CommitmentId },

    /// The transfer introduces a commitment that was not knowledge at the Target's cut.
    HistoricalUnavailability {
        commitment: CommitmentId,
        recorded_at: Date,
        known_at: Date,
    },

    /// A removal would leave a selected commitment without a dependency the Target held.
    DependencyBreakage {
        dependent: CommitmentId,
        missing_dependency: CommitmentId,
    },

    /// An introduction arrives without a dependency the candidate never had.
    MissingDependency {
        commitment: CommitmentId,
        dependency: CommitmentId,
    },
}

pub(super) fn conflicts<K: CanonicalKnowledge>(
    knowledge: &K,
    transfer: &ResolvedTransfer,
    candidate: &CandidateSelection,
    target: &Thesis,
) -> Result<Vec<ApplicabilityConflict>, ThesisError> {
    let mut found = Vec::new();

    found.extend(historical_freezing(transfer, target));
    found.extend(historical_unavailability(knowledge, transfer, target)?);
    found.extend(broken_closure(knowledge, candidate, target)?);

    Ok(found)
}

/// The Target's history made these unavoidable, and a transfer reasons about intention.
fn historical_freezing(transfer: &ResolvedTransfer, target: &Thesis) -> Vec<ApplicabilityConflict> {
    transfer
        .remove()
        .filter(|id| target.selection().is_frozen(*id))
        .map(|commitment| ApplicabilityConflict::HistoricalFreezing { commitment })
        .collect()
}

/// A commitment may exist canonically now and still not have been knowledge in the world the
/// Target recognizes. Introducing it there would select what that world could not have known.
fn historical_unavailability<K: CanonicalKnowledge>(
    knowledge: &K,
    transfer: &ResolvedTransfer,
    target: &Thesis,
) -> Result<Vec<ApplicabilityConflict>, ThesisError> {
    let known_at = *target.cut().known_at();
    let mut found = Vec::new();

    for commitment in transfer.introduce() {
        let record = knowledge
            .canonical_commitment(commitment)
            .ok_or(ThesisError::UnknownCommitment(commitment))?;

        if !record.recorded_at().up_to(&known_at) {
            found.push(ApplicabilityConflict::HistoricalUnavailability {
                commitment,
                recorded_at: *record.recorded_at(),
                known_at,
            });
        }
    }

    Ok(found)
}

/// A Thesis denotes a complete graph, so the candidate may not select a commitment while
/// leaving out an identity its structure requires.
fn broken_closure<K: CanonicalKnowledge>(
    knowledge: &K,
    candidate: &CandidateSelection,
    target: &Thesis,
) -> Result<Vec<ApplicabilityConflict>, ThesisError> {
    let selected: BTreeSet<CommitmentId> = candidate.resolved().collect();
    let mut found = Vec::new();

    for commitment in &selected {
        let record = knowledge
            .canonical_commitment(*commitment)
            .ok_or(ThesisError::UnknownCommitment(*commitment))?;

        for dependency in record.assertion().dependencies() {
            if selected.contains(dependency) {
                continue;
            }

            found.push(if target.selection().contains(*dependency) {
                ApplicabilityConflict::DependencyBreakage {
                    dependent: *commitment,
                    missing_dependency: *dependency,
                }
            } else {
                ApplicabilityConflict::MissingDependency {
                    commitment: *commitment,
                    dependency: *dependency,
                }
            });
        }
    }

    Ok(found)
}
