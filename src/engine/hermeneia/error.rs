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
//! `ObservationNotSettling` and `SettledMoreThanOnce` describe states the Canon already
//! prevents. They are kept because the code has to branch somewhere, and a named refusal
//! beats a silently wrong answer.

use crate::kernel::entities::{CommitmentId, EventId, StatementId};

#[derive(Debug, ::thiserror::Error)]
pub enum ProjectionError {
    #[error("commitment {0} is named but not in the selection")]
    UnknownCommitment(CommitmentId),

    #[error("commitment {commitment} names statement {statement}, which is absent")]
    UnknownStatement {
        commitment: CommitmentId,
        statement: StatementId,
    },

    #[error(
        "event {event} carries an observation its commitment's statement neither fulfills nor cancels"
    )]
    ObservationNotSettling { event: EventId },

    #[error("commitment {0} is settled by more than one event")]
    SettledMoreThanOnce(CommitmentId),
}
