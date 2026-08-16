//! A reading: everything the experiment compares, in a form that survives a pipe.
//!
//! The phases before termination record these values from a live world; the process after
//! it records them from a rebuilt one. They are the same type on purpose — a comparison
//! between two shapes proves less than a comparison between two values.
//!
//! Every field is application-owned rather than an engine type, for the reason the journal
//! already met: nothing the engine produces can be read back from bytes. A reading crosses
//! a process boundary, so it is written in terms the application can parse again.
//!
//! A reading covers one world whole rather than one commitment within it. The divergence
//! experiment compares worlds that agree on which commitments they select and disagree on
//! how those are partitioned, so a record naming a single commitment would report two worlds
//! as the same world.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use ape::canon::CanonicalHistory;
use ape::engine::hermeneia::{Conflict, Hypothesis, Outcome, Timeliness};
use ape::engine::thesis::{Interpretation, Thesis};
use ape::kernel::entities::ResourceInstanceId;
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
pub struct ConditionRecord {
    pub outcome: OutcomeRecord,
    pub timeliness: Option<TimelinessRecord>,
    pub pending_dependencies: bool,
    pub unfulfillable_dependencies: bool,
}

/// A finding feasibility reported, naming what it is about.
///
/// The naming is the point. A count says a world was refused; this says by which resource
/// and at which level, so a reproduction that refuses for another reason is not mistaken for
/// the same verdict.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "conflict", rename_all = "kebab-case")]
pub enum ConflictRecord {
    Unrealizable {
        commitment: String,
    },
    PunctualDependencyViolation {
        dependency: String,
        dependent: String,
    },
    OutOfBounds {
        instance: String,
        level: f64,
    },
}

/// One world, as the experiment compares it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reading {
    /// The end of canonical history, which the cut a Thesis holds is resolved against.
    pub canonical_head: Option<String>,
    pub thesis: String,
    pub thesis_parent: Option<String>,
    pub known_at: String,
    pub event_head: Option<String>,
    pub effective_at: String,
    pub frozen: BTreeSet<String>,
    pub open: BTreeSet<String>,
    pub conditions: BTreeMap<String, ConditionRecord>,
    pub level: f64,
    pub conflicts: Vec<ConflictRecord>,
}

/// Read one world, interpreted at `effective_at`.
pub fn of(
    history: &ResidentHistory,
    thesis: &Thesis,
    instance: ResourceInstanceId,
    effective_at: &Date,
) -> Result<Reading, ReadingError> {
    let interpretation = Interpretation::of(thesis, history)?;
    let projected = interpretation.conditions_at(effective_at)?;
    let feasibility = interpretation.feasibility_under(Hypothesis::FinalState)?;

    Ok(Reading {
        canonical_head: history.head().map(|id| id.to_string()),
        thesis: thesis.id().to_string(),
        thesis_parent: thesis.parent().map(|id| id.to_string()),
        known_at: thesis.cut().known_at().to_iso(),
        event_head: thesis.cut().event_head().map(|id| id.to_string()),
        effective_at: effective_at.to_iso(),
        frozen: thesis
            .selection()
            .frozen()
            .map(|id| id.to_string())
            .collect(),
        open: thesis.selection().open().map(|id| id.to_string()).collect(),
        conditions: projected
            .conditions()
            .iter()
            .map(|(id, condition)| {
                (
                    id.to_string(),
                    ConditionRecord {
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
                    },
                )
            })
            .collect(),
        level: level::settled(history, &projected, instance)?,
        conflicts: feasibility
            .conflicts()
            .iter()
            .map(|conflict| match conflict {
                Conflict::Unrealizable(commitment) => ConflictRecord::Unrealizable {
                    commitment: commitment.to_string(),
                },
                Conflict::PunctualDependencyViolation {
                    dependency,
                    dependent,
                } => ConflictRecord::PunctualDependencyViolation {
                    dependency: dependency.to_string(),
                    dependent: dependent.to_string(),
                },
                Conflict::OutOfBounds { instance, level } => ConflictRecord::OutOfBounds {
                    instance: instance.to_string(),
                    level: *level,
                },
            })
            .collect(),
    })
}

/// Read every world of a lineage, oldest first.
///
/// Every one of them was reasoned about, so every one of them is a result. A reading of the
/// tip alone answers a smaller question than the one the experiment asks.
pub fn all(
    history: &ResidentHistory,
    lineage: &[Thesis],
    instance: ResourceInstanceId,
    effective_at: &Date,
) -> Result<Vec<Reading>, ReadingError> {
    if lineage.is_empty() {
        return Err(ReadingError::EmptyLineage);
    }

    lineage
        .iter()
        .map(|thesis| of(history, thesis, instance, effective_at))
        .collect()
}

/// Rebuild a lineage from a repository and read it, having been given nothing else.
///
/// This is the whole of Phases 6 and 7 in one function, and it is deliberately short: a
/// fresh process opens the repository, replays what it finds, and interprets. Nothing here
/// consults a value the original process computed, and nothing here can — the repository is
/// the only source.
pub fn reconstruct(
    repository: &Repository,
    instance: ResourceInstanceId,
    effective_at: &Date,
) -> Result<Vec<Reading>, ReadingError> {
    let mut canon = ape::canon::Canon::new(ResidentHistory::new());

    crate::journal::replay(&mut canon, &repository.read_journal()?)?;

    let lineage = lineage::replay(canon.history(), &repository.read_lineage()?)?;

    all(canon.history(), &lineage, instance, effective_at)
}
