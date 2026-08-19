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
//!
//! Earned by: 00-reconstruction (Confirmed), 02-corroboration (Confirmed)

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use ape::canon::{Canon, CanonicalHistory};
use ape::engine::hermeneia::{Conflict, Hypothesis, Outcome, Timeliness};
use ape::engine::thesis::{Interpretation, Thesis};
use ape::kernel::entities::{AgentId, ResourceInstanceId};
use ape::kernel::value_objects::Date;

use crate::error::ReadingError;
use crate::history::ResidentHistory;
use crate::journal::{Admission, Replayed};
use crate::level;
use crate::lineage::{self, Lineage, Taken};
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

/// What a world *is*, before anything is asked of it.
///
/// This is the second thing a repository writes down twice. The first — the entries a
/// decision was taken against — corroborates the coordinate; this one corroborates the world
/// the coordinate and the intention produce together, which is the only place an altered
/// intention shows.
///
/// It is the application's vocabulary rather than the engine's serialized form, for the reason
/// the journal already gives: none of the engine's types can be read back from bytes, so a
/// record written in them would be a record only its writer can use. Here that argument has a
/// second edge — a witness computed over an encoding makes the encoding load-bearing, and this
/// one is computed over accessors instead.
///
/// A `Thesis` cannot be deserialized, so what crosses a process boundary is never a world. It
/// is this: enough to say *that* a world came back different, and which coordinate of it did.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldRecord {
    pub thesis: String,
    pub thesis_parent: Option<String>,
    pub known_at: String,
    pub event_head: Option<String>,
    pub frozen: BTreeSet<String>,
    pub open: BTreeSet<String>,
}

impl WorldRecord {
    pub fn of(thesis: &Thesis) -> Self {
        Self {
            thesis: thesis.id().to_string(),
            thesis_parent: thesis.parent().map(|id| id.to_string()),
            known_at: thesis.cut().known_at().to_iso(),
            event_head: thesis.cut().event_head().map(|id| id.to_string()),
            frozen: thesis
                .selection()
                .frozen()
                .map(|id| id.to_string())
                .collect(),
            open: thesis.selection().open().map(|id| id.to_string()).collect(),
        }
    }

    /// The coordinate that disagrees, or nothing where the two describe one world.
    ///
    /// Named rather than counted, and named field by field: a reader told only that a world
    /// came back different has to go and find out how.
    pub fn disagreement(&self, other: &Self) -> Option<&'static str> {
        // Identity is weighed last because it is derived from the rest, so it differs
        // whenever anything does and would mask what actually moved. Reaching it means every
        // named coordinate agreed and the identity did not — which is not a corrupted
        // repository but a changed derivation, and worth saying so.
        [
            ("the instant it recognizes", self.known_at != other.known_at),
            (
                "the chain it recognizes",
                self.event_head != other.event_head,
            ),
            ("what history made unavoidable", self.frozen != other.frozen),
            ("what it still proposes", self.open != other.open),
            ("ancestry", self.thesis_parent != other.thesis_parent),
            ("identity alone", self.thesis != other.thesis),
        ]
        .into_iter()
        .find_map(|(coordinate, differs)| differs.then_some(coordinate))
    }
}

/// One world, as the experiment compares it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reading {
    /// The head of the Event chain, which is the coordinate a cut resolves against.
    ///
    /// Not *the end of canonical history*, which is what this said before Phase 2 of the
    /// exploration experiment looked at it. A Commitment admitted moves canonical history and does
    /// not move this. The two coincide in an arrangement whose admissions are Events, and come apart
    /// in one that admits twelve Commitments and expects every derived answer to hold still — where
    /// the old wording invited reading an unchanged head as an unchanged history.
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
///
/// The two sequences are replayed together rather than one after the other. Which is not a
/// detail of this function: it is what the repository now records, and reading it any other
/// way resolves a decision against knowledge it was not taken against.
pub fn reconstruct(
    repository: &Repository,
    instance: ResourceInstanceId,
    effective_at: &Date,
) -> Result<Vec<Reading>, ReadingError> {
    let Corroborated { canon, lineage, .. } = corroborated(repository)?;

    all(canon.history(), lineage.decided(), instance, effective_at)
}

/// A repository rebuilt, and weighed against what it says it produced.
///
/// A reader wants one of these fields. A writer wants all of them, and that is why they arrive
/// together rather than by asking twice: knowledge to resolve a cut against, the worlds already
/// decided, how far into the journal the replay reached — which is the coordinate a new decision
/// has to be able to name — and the two sequences as they were read, which is what a writer
/// extends and writes back.
///
/// One read of each file, whole. A procedure that opened the journal again to get at its records
/// would be reading a file that may have moved between the two opens, which is the hazard this
/// value exists to keep out of everything downstream of it.
pub struct Corroborated {
    pub canon: Canon<ResidentHistory>,
    pub lineage: Lineage,
    pub admitted: Replayed,
    pub journal: Vec<Admission>,
    pub decisions: Vec<Taken>,
}

/// The worlds a party's decisions produced, by identity, where anything says who decided.
///
/// This is the whole of what recording a decider buys, and it is deliberately identities rather than
/// readings. An identity is what a transfer is asked about — convergence established that a question
/// is asked *of* a record and arrives from outside it — so what a party needs in order to be referred
/// to is a name it can hand back, and everything else about those worlds is already answerable.
///
/// It cannot answer for a decision that claims nobody, and says nothing about there being one. A
/// reader cannot tell *not this party's* from *unclaimed*, which is what an optional field costs.
pub fn decided_by(
    repository: &Repository,
    party: AgentId,
) -> Result<BTreeSet<String>, ReadingError> {
    let Corroborated {
        lineage, decisions, ..
    } = corroborated(repository)?;

    Ok(decisions
        .iter()
        .zip(lineage.decided())
        .filter(|(taken, _)| taken.by == Some(party))
        .map(|(_, world)| world.id().to_string())
        .collect())
}

/// Rebuild what a repository holds, and weigh it against what the repository says it produced.
///
/// This is reconstruction itself, with nothing yet asked of the result. It is separate because
/// three things now need it and they need exactly it: reading the worlds, asking what one world's
/// intention would be in another, and extending the lineage. A second copy of the procedure would
/// be a second place for the order between the two files to drift.
pub fn corroborated(repository: &Repository) -> Result<Corroborated, ReadingError> {
    let mut canon = Canon::new(ResidentHistory::new());

    let journal = repository.read_journal()?;
    let decisions = repository.read_lineage()?;
    let worlds = repository.read_worlds()?;

    let (lineage, admitted) = lineage::rebuild(&mut canon, &journal, &decisions)?;

    corroborate(lineage.decided(), &worlds)?;

    Ok(Corroborated {
        canon,
        lineage,
        admitted,
        journal,
        decisions,
    })
}

/// Weigh the worlds a repository says it decided against the worlds its decisions produce.
///
/// The stored side cannot become a `Thesis` again — the engine derives `Serialize` and not
/// `Deserialize` — so this is a comparison and never a fallback. A repository whose decisions
/// no longer produce the worlds it recorded is refused rather than silently believed on
/// either side.
fn corroborate(lineage: &[Thesis], recorded: &[WorldRecord]) -> Result<(), ReadingError> {
    if lineage.len() != recorded.len() {
        return Err(ReadingError::LineageLengthDisagrees {
            derived: lineage.len(),
            recorded: recorded.len(),
        });
    }

    for (position, (thesis, recorded)) in lineage.iter().zip(recorded).enumerate() {
        let derived = WorldRecord::of(thesis);

        if let Some(coordinate) = derived.disagreement(recorded) {
            return Err(ReadingError::WorldDisagrees {
                position,
                coordinate,
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The two field lists of this module describe one thing, and neither derives the other.
    ///
    /// They are adjacent so that anyone editing one sees the other, and this reads both to say
    /// which coordinate stopped agreeing. A `Reading` that reported a world differently from
    /// the record that witnesses it would make a repository refuse itself.
    #[test]
    fn a_reading_reports_the_world_its_witness_records() {
        let (canon, subject, _, lineage) = {
            let begun = crate::subject::divergence::begun().expect("the subject is admissible");
            (begun.canon, begun.subject, begun.decisions, begun.lineage)
        };

        let thesis = lineage.decided().last().expect("the genesis");
        let record = WorldRecord::of(thesis);

        let reading = of(
            canon.history(),
            thesis,
            subject.instance,
            &Date::parse("2026-01-10").expect("a date"),
        )
        .expect("the world reads");

        assert_eq!(reading.thesis, record.thesis, "identity");
        assert_eq!(reading.thesis_parent, record.thesis_parent, "ancestry");
        assert_eq!(reading.known_at, record.known_at, "instant");
        assert_eq!(reading.event_head, record.event_head, "chain");
        assert_eq!(reading.frozen, record.frozen, "frozen");
        assert_eq!(reading.open, record.open, "open");
    }
}
