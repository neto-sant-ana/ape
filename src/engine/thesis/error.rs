//! Why a Thesis could not be derived, or interpreted.
//!
//! Every variant reports a request that could not produce an interpretable world, never an
//! operational judgment. A continuation that contradicts what history imposed is not
//! refused here.
//!
//! `Projection` carries what Hermeneia refused while folding a recognized chain. Deriving and
//! interpreting share one error because one operation does both: an interpretation resolves
//! the chain its cut recognizes and then folds it, and either half may refuse.
//!
//! The refusals separate by who got it wrong:
//! `FrozenPastOmitted` and `DanglingDependency`
//! Answer the caller: one names a commitment history made unavoidable, the other a
//! selection that names a dependency it does not select.
//!
//! `CutNotLater`, `HeadDoesNotDescend` and `HeadWithdrawn`
//! Answer an advancement that is not one: one that recognizes no later knowledge, one
//! reaching a head of another reach of history, one giving a head back.
//!
//! `CommitmentNotKnownAtCut` and `EventNotKnownAtCut`
//! Answer anachronism — an assertion recorded after the cut that claims to recognize it.
//! A cut refuses the head it cannot have known; a selection refuses the commitment.
//!
//! `UnknownCommitment` and `UnknownEvent`
//! Report a reference canonical history does not hold, and each is a single refusal
//! whether the assertion was never admitted or the reference is simply wrong.
//!
//! `OmittedAndIntroduced` and `SelectionUnchanged`
//! Answer a request that states nothing: one both drops and selects the same commitment, the
//! other leaves the world exactly as its parent had it. A Thesis is produced by a change in
//! what is selected or in what is recognized, and an edge that carries neither would record a
//! decision the model cannot read back.

use crate::engine::hermeneia::ProjectionError;

use crate::kernel::entities::{CommitmentId, EventId, IdentityError};

use crate::kernel::value_objects::Date;

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

    #[error("the fork selects exactly what its parent selects; nothing would distinguish them")]
    SelectionUnchanged,

    #[error("event {target} does not descend from the recognized head {parent:?}")]
    HeadDoesNotDescend {
        parent: Option<EventId>,
        target: EventId,
    },

    #[error("the recognized head {parent} may not be given back by an advancement")]
    HeadWithdrawn { parent: EventId },

    #[error(
        "a cut known through {target:?} recognizes no knowledge later than {parent:?}; advancement requires a strictly later instant"
    )]
    CutNotLater { parent: Date, target: Date },

    #[error(
        "commitment {commitment} was not knowledge at {known_at:?}, having been recorded at {recorded_at:?}"
    )]
    CommitmentNotKnownAtCut {
        commitment: CommitmentId,
        recorded_at: Date,
        known_at: Date,
    },

    #[error(
        "event {event} was not knowledge at {known_at:?}, having been recorded at {recorded_at:?}"
    )]
    EventNotKnownAtCut {
        event: EventId,
        recorded_at: Date,
        known_at: Date,
    },

    #[error(transparent)]
    Identity(#[from] IdentityError),

    #[error(transparent)]
    Projection(#[from] ProjectionError),
}
