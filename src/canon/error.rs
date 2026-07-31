//! Why an assertion was refused admission into canonical history.

use crate::kernel::axiom::AxiomError;

use crate::kernel::entities::{AgentId, CommitmentId, EventId};

use crate::kernel::value_objects::Date;

#[derive(Debug, ::thiserror::Error)]
pub enum CanonError {
    #[error(transparent)]
    Axiom(#[from] AxiomError),

    /// The event was built against a head that has since moved. Recovery is a fresh
    /// admission through the Canon — which re-reads the head and re-runs settle-once.
    #[error("event chain head moved: the event extends {expected:?} but the head is {found:?}")]
    UnexpectedHead {
        expected: Option<EventId>,
        found: Option<EventId>,
    },

    #[error("commitment {0} is already settled by an event")]
    CommitmentAlreadySettled(CommitmentId),

    #[error("agent {agent} already has a different eligibility effective from {effective_from:?}")]
    ConflictingEligibility {
        agent: AgentId,
        effective_from: Date,
    },

    #[error(
        "recorded_at {recorded_at:?} precedes {earliest:?}, the earliest this assertion could be recorded"
    )]
    RecordedTooEarly { earliest: Date, recorded_at: Date },

    /// The assertion would be back-dated into knowledge already admitted, which would
    /// change what a past interpretation reads.
    #[error(
        "recorded_at {recorded_at:?} precedes {recorded_through:?}, through which history is already recorded"
    )]
    RecordedOutOfOrder {
        recorded_at: Date,
        recorded_through: Date,
    },
}
