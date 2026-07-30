//! Why a Thesis could not be derived.
//!
//! Every variant reports a request that could not produce an interpretable world, never an
//! operational judgment. A continuation that contradicts what history imposed is not
//! refused here.
//!
//! The refusals separate by who got it wrong:
//! `FrozenPastOmitted` and `DanglingDependency`
//! Answer the caller: one names a commitment history made unavoidable, the other a
//! selection that names a dependency it does not select.
//! 
//! `AlreadyAtHead` and `HeadDoesNotDescend`
//! Answer an advancement that is not one.
//! 
//! `UnknownCommitment` and `UnknownEvent`
//! Report a reference canonical history does not hold, and each is a single refusal
//! whether the assertion was never admitted or the reference is simply wrong.
//!
//! `OmittedAndIntroduced`
//! Is the one refusal of a request that is merely incoherent.

use crate::kernel::entities::{CommitmentId, EventId, IdentityError};

#[derive(Debug, ::thiserror::Error)]
pub enum ThesisError {
    #[error("commitment {0} is selected but absent from canonical history")]
    UnknownCommitment(CommitmentId),

    #[error("event {0} is named as a head but absent from canonical history")]
    UnknownEvent(EventId),

    #[error("commitment {0} belongs to the frozen causal past and may not be omitted")]
    FrozenPastOmitted(CommitmentId),

    #[error("commitment {dependent} depends on {dependency}, which the selection omits")]
    DanglingDependency {
        dependent: CommitmentId,
        dependency: CommitmentId,
    },

    #[error("commitment {0} is both omitted and introduced")]
    OmittedAndIntroduced(CommitmentId),

    #[error("event {target} does not descend from the recognized head {parent:?}")]
    HeadDoesNotDescend {
        parent: Option<EventId>,
        target: EventId,
    },

    #[error("event {0} is already the recognized head; advancement requires a later one")]
    AlreadyAtHead(EventId),

    #[error(transparent)]
    Identity(#[from] IdentityError),
}
