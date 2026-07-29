//! Why knowledge could not be accumulated or interpreted.
//!
//! Every variant reports input that does not hold together, never an operational
//! judgment: an infeasible graph, a breached deadline and a blocked commitment are
//! derived results, not errors.
//!
//! The selection is the world. A commitment outside it does not exist for this
//! projection, whether it was never admitted or a Scenario left it out, so naming one —
//! as a selection root, as a dependency, or as what an event settles — is the single
//! `UnknownCommitment`.
//!
//! `ObservationNotSettling`, `SettledMoreThanOnce`, `ActionValueMismatch` and
//! `ActionResourceKindMismatch` describe states the Axiom and the Canon already prevent. They
//! are kept because the code has to branch somewhere, and a named refusal beats a silently
//! wrong answer.

use crate::kernel::entities::{
    ActionId, CommitmentId, EventId, ResourceId, ResourceInstanceId, StatementId,
};

use crate::kernel::value_objects::Date;

#[derive(Debug, ::thiserror::Error)]
pub enum ProjectionError {
    #[error("commitment {0} is named but not in the selection")]
    UnknownCommitment(CommitmentId),

    #[error("commitment {commitment} names statement {statement}, which is absent")]
    UnknownStatement {
        commitment: CommitmentId,
        statement: StatementId,
    },

    #[error("statement {statement} names action {action}, which is absent")]
    UnknownAction {
        statement: StatementId,
        action: ActionId,
    },

    #[error("commitment {commitment} names resource instance {instance}, which is absent")]
    UnknownResourceInstance {
        commitment: CommitmentId,
        instance: ResourceInstanceId,
    },

    #[error("resource instance {instance} names resource {resource}, which is absent")]
    UnknownResource {
        instance: ResourceInstanceId,
        resource: ResourceId,
    },

    #[error("commitment {0} carries a value its action's kind does not admit")]
    ActionValueMismatch(CommitmentId),

    #[error("commitment {0} moves a level on a resource that declares no bounds")]
    ActionResourceKindMismatch(CommitmentId),

    #[error(
        "{count} movements share {position:?} on resource instance {instance}, too many to decide every arrangement of"
    )]
    TooManySimultaneousMovements {
        instance: ResourceInstanceId,
        position: Date,
        count: usize,
    },

    #[error(
        "event {event} carries an observation its commitment's statement neither fulfills nor cancels"
    )]
    ObservationNotSettling { event: EventId },

    #[error("commitment {0} is settled by more than one event")]
    SettledMoreThanOnce(CommitmentId),
}
