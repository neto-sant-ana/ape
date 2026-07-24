//! Why an assertion was refused admission into canonical history.

use crate::kernel::axiom::AxiomError;

use crate::kernel::entities::{AgentId, CommitmentId, EventId};

use crate::kernel::value_objects::Date;

#[derive(Debug, ::thiserror::Error)]
pub enum CanonError {
    #[error(transparent)]
    Axiom(#[from] AxiomError),

    #[error("event chain head moved: the event extends {expected:?} but the head is {found:?}")]
    UnexpectedHead {
        expected: Option<EventId>,
        found: Option<EventId>,
    },

    #[error("commitment {0} is already settled by an event")]
    CommitmentAlreadySettled(CommitmentId),

    #[error("agent {agent} already has a different eligibility effective from {effective_from:?}")]
    ConflictingEligibility { agent: AgentId, effective_from: Date },

    #[error(
        "recorded_at {recorded_at:?} precedes {earliest:?}, the earliest this assertion could be recorded"
    )]
    RecordedTooEarly { earliest: Date, recorded_at: Date },
}
