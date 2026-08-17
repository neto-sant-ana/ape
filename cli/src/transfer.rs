//! What Synthesis said, in a form the experiment can compare.
//!
//! An `ApplicabilityReport` is the engine's whole answer about moving one line of thinking's
//! intention into another. It is not an entity, it is not stored, and it derives no
//! serialization — so a process asking for one twice is what makes it comparable at all, and
//! what crosses a boundary is this record rather than the report.
//!
//! Every field is application-owned, for the reason the journal and the reading already met:
//! nothing the engine produces can be read back from bytes. The shape mirrors the engine's
//! exactly, field for field, so that a coordinate the report distinguishes is a coordinate this
//! distinguishes.
//!
//! # The fourth coordinate
//!
//! A report is derived from a Base, a Source, a Target and canonical knowledge. Three of those
//! are worlds a repository already produces; the Base is a **choice**. Synthesis accepts any
//! common ancestor of the two and verifies rather than searches — a Base equal to the Target is
//! a fast-forward, one equal to the Source leaves an empty difference, and both are answers —
//! so which world a difference is measured against is part of the question being asked.
//!
//! That is why the record carries all three identities and not only the two it moves between.
//! Whether a repository must carry them too is a different question, and this module does not
//! answer it.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use ape::engine::synthesis::{
    ApplicabilityConflict, ApplicabilityReport, ApplicabilityStatus, ResolvedTransfer, synthesize,
};
use ape::engine::thesis::ThesisId;

use crate::error::TransferError;
use crate::lineage::Decision;
use crate::reading;
use crate::repository::Repository;

/// A rule the resulting world would break, naming what it is about.
///
/// The naming is the point, as it is for a feasibility conflict: a count says a transfer was
/// refused, and this says which commitment and against which instant, so a reproduction that
/// refuses for another reason is not mistaken for the same verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "conflict", rename_all = "kebab-case")]
pub enum ConflictRecord {
    HistoricalFreezing {
        commitment: String,
    },
    HistoricalUnavailability {
        commitment: String,
        recorded_at: String,
        known_at: String,
    },
    DependencyBreakage {
        dependent: String,
        missing_dependency: String,
    },
    MissingDependency {
        commitment: String,
        dependency: String,
    },
}

/// What a transfer asks of the Target, once what the Target already satisfies is taken out.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferRecord {
    pub remove: BTreeSet<String>,
    pub introduce: BTreeSet<String>,
}

/// The world a transfer would produce, held only long enough to be judged.
///
/// Deliberately not a [`crate::reading::WorldRecord`]. That one carries an identity and a cut,
/// and a candidate has neither: it is the argument for what would follow, not the world that
/// would follow, and giving it the shape of a world would invite reading it as one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidateRecord {
    pub frozen: BTreeSet<String>,
    pub open: BTreeSet<String>,
}

/// The conclusion, read from the shape rather than from an empty collection.
///
/// The three cases are kept apart here as the engine keeps them apart, so that neither a
/// conflicted result without conflicts nor an applicable one carrying them can be written down.
/// Flattening them into a status plus two optional collections would make both representable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum StatusRecord {
    Applicable {
        transfer: TransferRecord,
        candidate: CandidateRecord,
    },
    AlreadyApplied,
    Conflicted {
        attempted: TransferRecord,
        conflicts: Vec<ConflictRecord>,
    },
}

/// One report, as the experiment compares it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Applicability {
    pub base: String,
    pub source: String,
    pub target: String,
    /// What the Source decided relative to the Base, whatever became of it.
    ///
    /// Kept beside the status rather than folded into it, because the two say different things.
    /// A difference is what was decided; a transfer is what is left to do about it, and a
    /// report where the second is empty still has the first to explain why.
    pub omitted: BTreeSet<String>,
    pub introduced: BTreeSet<String>,
    pub status: StatusRecord,
}

impl Applicability {
    pub fn of(report: &ApplicabilityReport) -> Self {
        Self {
            base: report.base().to_string(),
            source: report.source().to_string(),
            target: report.target().to_string(),
            omitted: report.difference().omitted().map(hex).collect(),
            introduced: report.difference().introduced().map(hex).collect(),
            status: match report.status() {
                ApplicabilityStatus::Applicable {
                    transfer,
                    candidate,
                } => StatusRecord::Applicable {
                    transfer: transferred(transfer),
                    candidate: CandidateRecord {
                        frozen: candidate.frozen().map(hex).collect(),
                        open: candidate.open().map(hex).collect(),
                    },
                },

                ApplicabilityStatus::AlreadyApplied => StatusRecord::AlreadyApplied,

                ApplicabilityStatus::Conflicted {
                    attempted,
                    conflicts,
                } => StatusRecord::Conflicted {
                    attempted: transferred(attempted),
                    conflicts: conflicts.iter().map(conflicted).collect(),
                },
            },
        }
    }
}

fn transferred(transfer: &ResolvedTransfer) -> TransferRecord {
    TransferRecord {
        remove: transfer.remove().map(hex).collect(),
        introduce: transfer.introduce().map(hex).collect(),
    }
}

fn conflicted(conflict: &ApplicabilityConflict) -> ConflictRecord {
    match conflict {
        ApplicabilityConflict::HistoricalFreezing { commitment } => {
            ConflictRecord::HistoricalFreezing {
                commitment: commitment.to_string(),
            }
        }

        ApplicabilityConflict::HistoricalUnavailability {
            commitment,
            recorded_at,
            known_at,
        } => ConflictRecord::HistoricalUnavailability {
            commitment: commitment.to_string(),
            recorded_at: recorded_at.to_iso(),
            known_at: known_at.to_iso(),
        },

        ApplicabilityConflict::DependencyBreakage {
            dependent,
            missing_dependency,
        } => ConflictRecord::DependencyBreakage {
            dependent: dependent.to_string(),
            missing_dependency: missing_dependency.to_string(),
        },

        ApplicabilityConflict::MissingDependency {
            commitment,
            dependency,
        } => ConflictRecord::MissingDependency {
            commitment: commitment.to_string(),
            dependency: dependency.to_string(),
        },
    }
}

fn hex(id: impl std::fmt::Display) -> String {
    id.to_string()
}

/// Carry a transfer into its Target as an ordinary decision.
///
/// A resolved transfer states what is left to do — remove these, introduce those — and a fork
/// states an outcome by the same two halves. So this is a translation and not a construction:
/// the engine builds nothing from a report, and turning one into a world is the application's
/// decision, taken through the same path as every other decision it has ever taken.
///
/// What the decision does **not** carry is where the intention came from. The Base and the Source
/// produced it and do not survive into it, so the record says which commitments were introduced
/// and never that another line of thinking is why.
pub fn applied(target: ThesisId, transfer: &ResolvedTransfer) -> Decision {
    Decision::Fork {
        extends: target,
        omitted: transfer.remove().collect(),
        introduced: transfer.introduce().collect(),
    }
}

/// Rebuild a repository and ask it what one world's intention would be in another.
///
/// The three identities arrive from outside, and they have to. Reconstructing a *world* needs
/// the repository alone; reconstructing a *report* needs the repository and a question, and the
/// question is not in there — Phase 4 established that a query nobody acted on is not part of
/// the record.
///
/// Where the identities come from is not a gap. They are content-addressed, so a caller obtains
/// them by reading the same repository and naming what it found. A question is asked *about* a
/// record; it is not stored in one.
pub fn reconstruct(
    repository: &Repository,
    base: ThesisId,
    source: ThesisId,
    target: ThesisId,
) -> Result<Applicability, TransferError> {
    let (canon, lineage) = reading::corroborated(repository)?;

    let report = synthesize(lineage.archive(), canon.history(), base, source, target)?;

    Ok(Applicability::of(&report))
}
