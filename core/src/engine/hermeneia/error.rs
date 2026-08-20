//! Why knowledge could not be accumulated or interpreted.
//!
//! Every variant reports input that does not hold together, never an operational
//! judgment: an infeasible graph, a breached deadline and a commitment waiting on a
//! dependency are derived results, not errors.
//!
//! The selection is the world. A commitment outside it does not exist for this
//! projection, whether it was never admitted or a Thesis left it out, so naming one —
//! as a selection root, as a dependency, or as what an event settles — is the single
//! `UnknownCommitment`.
//!
//! `DisjointEventChain` is what keeps resumption a guarantee rather than a convention.
//! `previous_event` is part of an event's hashed identity, so a batch of events proves its own
//! contiguity and where it attaches: an accumulation can refuse a batch that belongs to another
//! history, or that skips events, without consulting anything outside what it was handed.
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
pub enum HermeneiaError {
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

    /// A magnitude too large to become a signed movement.
    ///
    /// The price of an unsigned magnitude, and the whole of it: a negative one stopped being
    /// writable, and in exchange the very largest ones stop being movable. Unreachable with any
    /// plausible magnitude, and refused rather than wrapped — a wrapped movement would be a level of
    /// the wrong sign, which is worse than no answer.
    #[error("commitment {commitment} carries a magnitude too large to move a level")]
    MagnitudeUnmovable { commitment: CommitmentId },

    /// Movements that sum past what a count can hold.
    ///
    /// Unreachable with any plausible magnitude, and refused rather than hoped: wrapping is the one
    /// arithmetic outcome that is silently wrong, because a wrapped total would be compared against a
    /// bound as though it were a level.
    #[error("the movements on resource instance {instance} sum beyond what a count can hold")]
    LevelOutOfRange { instance: ResourceInstanceId },

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

    #[error("event {event} extends {carried:?} but the chain absorbed so far ends at {absorbed:?}")]
    DisjointEventChain {
        event: EventId,
        absorbed: Option<EventId>,
        carried: Option<EventId>,
    },

    #[error("event {event} lies beyond {recognized:?}, the head this accumulation recognizes")]
    EventBeyondRecognizedHead {
        event: EventId,
        recognized: Option<EventId>,
    },

    #[error(
        "the chain absorbed so far ends at {reached:?} and {recognized:?} is recognized; nothing is interpretable until it is reached"
    )]
    RecognizedChainIncomplete {
        reached: Option<EventId>,
        recognized: Option<EventId>,
    },
}
