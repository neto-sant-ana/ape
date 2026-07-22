//! Why an assertion was refused admission.

use crate::kernel::entities::{
    ActionId, AgentId, CommitmentId, EventId, IdentityError, ResourceId, ResourceInstanceId,
    RoleId, StatementId,
};

#[derive(Debug, ::thiserror::Error)]
pub enum AxiomError {
    #[error(transparent)]
    Identity(#[from] IdentityError),

    #[error("referenced role does not exist: {0}")]
    UnknownRole(RoleId),

    #[error("referenced agent does not exist: {0}")]
    UnknownAgent(AgentId),

    #[error("referenced resource does not exist: {0}")]
    UnknownResource(ResourceId),

    #[error("referenced resource instance does not exist: {0}")]
    UnknownResourceInstance(ResourceInstanceId),

    #[error("referenced action does not exist: {0}")]
    UnknownAction(ActionId),

    #[error("referenced statement does not exist: {0}")]
    UnknownStatement(StatementId),

    #[error("referenced commitment does not exist: {0}")]
    UnknownCommitment(CommitmentId),

    #[error("referenced event does not exist: {0}")]
    UnknownEvent(EventId),

    #[error("action kind does not match the target resource kind")]
    ActionResourceKindMismatch,

    #[error("resource instance {found} is not an instance of the statement's resource {expected}")]
    ResourceInstanceMismatch {
        expected: ResourceId,
        found: ResourceId,
    },

    #[error("action value does not match the statement action's kind")]
    ActionValueMismatch,

    #[error("executor {0} is not eligible for any of the statement's actor roles")]
    IneligibleExecutor(AgentId),

    #[error("beneficiary {0} is not eligible for any of the statement's recipient roles")]
    IneligibleBeneficiary(AgentId),

    #[error("a superseding commitment must reference the same statement as the one it supersedes")]
    SupersedeStatementMismatch,

    #[error("observation is not recognized by the commitment's statement settlement")]
    ObservationNotSettling,
}
