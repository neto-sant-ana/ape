//! A reading: everything the experiment compares, in a form that survives a pipe.
//!
//! The phases before termination record these values from a live world; the process after
//! it records them from a rebuilt one. They are the same type on purpose — a comparison
//! between two shapes proves less than a comparison between two values.
//!
//! Every field is application-owned rather than an engine type, for the reason the journal
//! already met: nothing the engine produces can be read back from bytes. A reading crosses
//! a process boundary, so it is written in terms the application can parse again.

use serde::{Deserialize, Serialize};

use ape::canon::CanonicalHistory;
use ape::engine::hermeneia::{Hypothesis, Outcome, Timeliness};
use ape::engine::thesis::Interpretation;
use ape::kernel::entities::CommitmentId;
use ape::kernel::value_objects::Date;

use crate::error::ReadingError;
use crate::history::ResidentHistory;
use crate::level;
use crate::lineage;
use crate::repository::Repository;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OutcomeRecord {
    Unsettled,
    Fulfilled,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TimelinessRecord {
    WithinDeadline,
    Breached,
}

/// What one world says about one commitment, at one instant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reading {
    /// The end of canonical history, which the cut a Thesis holds is resolved against.
    pub canonical_head: Option<String>,
    pub thesis: String,
    pub thesis_parent: Option<String>,
    pub known_at: String,
    pub event_head: Option<String>,
    pub effective_at: String,
    pub outcome: OutcomeRecord,
    pub timeliness: Option<TimelinessRecord>,
    pub pending_dependencies: bool,
    pub unfulfillable_dependencies: bool,
    pub level: f64,
    pub conflicts: usize,
}

/// Read a live world: the last Thesis of `lineage`, interpreted at `effective_at`.
pub fn of(
    history: &ResidentHistory,
    lineage: &[ape::engine::thesis::Thesis],
    commitment: CommitmentId,
    instance: ape::kernel::entities::ResourceInstanceId,
    effective_at: &Date,
) -> Result<Reading, ReadingError> {
    let thesis = lineage.last().ok_or(ReadingError::EmptyLineage)?;

    let interpretation = Interpretation::of(thesis, history)?;
    let projected = interpretation.conditions_at(effective_at)?;

    let condition = projected
        .condition(commitment)
        .ok_or(ReadingError::UnprojectedCommitment(commitment))?;

    let feasibility = interpretation.feasibility_under(Hypothesis::FinalState)?;

    Ok(Reading {
        canonical_head: history.head().map(|id| id.to_string()),
        thesis: thesis.id().to_string(),
        thesis_parent: thesis.parent().map(|id| id.to_string()),
        known_at: thesis.cut().known_at().to_iso(),
        event_head: thesis.cut().event_head().map(|id| id.to_string()),
        effective_at: effective_at.to_iso(),
        outcome: match condition.outcome() {
            Outcome::Unsettled => OutcomeRecord::Unsettled,
            Outcome::Fulfilled => OutcomeRecord::Fulfilled,
            Outcome::Cancelled => OutcomeRecord::Cancelled,
        },
        timeliness: condition.timeliness().map(|timeliness| match timeliness {
            Timeliness::WithinDeadline => TimelinessRecord::WithinDeadline,
            Timeliness::Breached => TimelinessRecord::Breached,
        }),
        pending_dependencies: condition.has_pending_dependencies(),
        unfulfillable_dependencies: condition.has_unfulfillable_dependencies(),
        level: level::settled(history, &projected, instance)?,
        conflicts: feasibility.conflicts().len(),
    })
}

/// Rebuild a world from a repository and read it, having been given nothing else.
///
/// This is the whole of Phases 5 and 6 in one function, and it is deliberately short: a
/// fresh process opens the repository, replays what it finds, and interprets. Nothing here
/// consults a value the original process computed, and nothing here can — the repository is
/// the only argument.
pub fn reconstruct(
    repository: &Repository,
    commitment: CommitmentId,
    instance: ape::kernel::entities::ResourceInstanceId,
    effective_at: &Date,
) -> Result<Reading, ReadingError> {
    let mut canon = ape::canon::Canon::new(ResidentHistory::new());

    crate::journal::replay(&mut canon, &repository.read_journal()?)?;

    let lineage = lineage::replay(canon.history(), &repository.read_lineage()?)?;

    of(canon.history(), &lineage, commitment, instance, effective_at)
}
