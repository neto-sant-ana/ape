//! `ApplicabilityReport` — the whole product of Synthesis.
//!
//! The conclusion is read from the shape rather than from an empty collection, which keeps
//! two statements true at once: **a conflict always names a violated invariant**, and **an
//! applicable result always carries an effective change**. Neither a conflicted report
//! without conflicts nor an applicable one with them can be represented.
//!
//! ```text
//! Applicable     { transfer, candidate }  → an effective change, and nothing against it
//! AlreadyApplied                          → the Target already contains the difference
//! Conflicted     { attempted, conflicts } → a known invariant prevents it
//! ```
//!
//! `AlreadyApplied` carries no transfer, because nothing remains to apply, and no candidate,
//! because the candidate would be the Target. It is neither a failure nor a violation. That a
//! Thesis derived from an unchanged selection would be refused is the Thesis layer's
//! invariant over an operation Synthesis never performs, and reporting it as a conflict here
//! would name a rule this transfer did not break.
//!
//! A report is not an entity. Nothing refers to it, it takes part in no graph, and Synthesis
//! does not persist it; it carries the coordinates that produced it and is obtained again by
//! asking again. That reproduction holds permanently because every question it asks is
//! answered below cuts that are fixed, on Theses that are immutable, and recording is
//! monotonic across admission — so nothing may later appear beneath those cuts.

use super::{ApplicabilityConflict, CandidateSelection, IntentionalDifference, ResolvedTransfer};

use crate::engine::thesis::ThesisId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicabilityStatus {
    Applicable {
        transfer: ResolvedTransfer,
        candidate: CandidateSelection,
    },
    AlreadyApplied,
    Conflicted {
        attempted: ResolvedTransfer,
        conflicts: Vec<ApplicabilityConflict>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicabilityReport {
    base: ThesisId,
    source: ThesisId,
    target: ThesisId,
    difference: IntentionalDifference,
    status: ApplicabilityStatus,
}
impl ApplicabilityReport {
    pub(super) fn new(
        base: ThesisId,
        source: ThesisId,
        target: ThesisId,
        difference: IntentionalDifference,
        status: ApplicabilityStatus,
    ) -> Self {
        Self {
            base,
            source,
            target,
            difference,
            status,
        }
    }

    pub fn base(&self) -> ThesisId {
        self.base
    }

    pub fn source(&self) -> ThesisId {
        self.source
    }

    pub fn target(&self) -> ThesisId {
        self.target
    }

    /// What the Source decided relative to the Base, whatever became of it.
    pub fn difference(&self) -> &IntentionalDifference {
        &self.difference
    }

    pub fn status(&self) -> &ApplicabilityStatus {
        &self.status
    }

    /// Whether the transfer may be used to construct another Thesis derived from the Target.
    pub fn is_applicable(&self) -> bool {
        matches!(self.status, ApplicabilityStatus::Applicable { .. })
    }

    /// The invariants that prevented the transfer, empty unless the report is conflicted.
    pub fn conflicts(&self) -> &[ApplicabilityConflict] {
        match &self.status {
            ApplicabilityStatus::Conflicted { conflicts, .. } => conflicts,
            _ => &[],
        }
    }
}
