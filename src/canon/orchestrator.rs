//! The `Canon` — the canonical admission layer.
//!
//! It is the single entry point through which knowledge becomes history: it
//! delegates structural validation to the Axiom, enriches the assertion with
//! canonical metadata (`recorded_at`, via [`Canonical`]), and admits it through
//! the mechanical [`CanonicalHistory`] primitives.

use super::{CanonError, Canonical, CanonicalHistory};

use crate::kernel::axiom::Axiom;

use crate::kernel::entities::{
    ActionId, ActionInput, AgentId, AgentInput, CommitmentId, CommitmentInput,
    EligibilityAssignmentId, EligibilityAssignmentInput, EventId, EventInput, ResourceId,
    ResourceInput, ResourceInstanceId, ResourceInstanceInput, RoleId, RoleInput, StatementId,
    StatementInput,
};

use crate::kernel::value_objects::{Date, Observation};

pub struct EventSubmission {
    pub commitment_id: CommitmentId,
    pub observation: Observation,
    pub occurred_at: Date,
}

pub struct Canon<H: CanonicalHistory> {
    history: H,
}
impl<H: CanonicalHistory> Canon<H> {
    pub fn new(history: H) -> Self {
        Self { history }
    }

    pub fn history(&self) -> &H {
        &self.history
    }

    canonical_admission! {
        admit_role(RoleInput) -> RoleId { admit_role, put_role },
        admit_agent(AgentInput) -> AgentId { admit_agent, put_agent },
        admit_resource(ResourceInput) -> ResourceId { admit_resource, put_resource },
        admit_resource_instance(ResourceInstanceInput) -> ResourceInstanceId {
            admit_resource_instance, put_resource_instance
        },
        admit_action(ActionInput) -> ActionId { admit_action, put_action },
        admit_statement(StatementInput) -> StatementId { admit_statement, put_statement },
        admit_commitment(CommitmentInput) -> CommitmentId { admit_commitment, put_commitment },
        admit_eligibility(EligibilityAssignmentInput) -> EligibilityAssignmentId {
            admit_eligibility_assignment, put_eligibility
        },
    }

    pub fn admit_event(
        &mut self,
        submission: EventSubmission,
        recorded_at: Date,
    ) -> Result<EventId, CanonError> {
        if let Some(settled) = self.history.event_of(submission.commitment_id) {
            if settled.observation() == &submission.observation
                && *settled.occurred_at() == submission.occurred_at
            {
                return Ok(settled.id());
            }

            return Err(CanonError::CommitmentAlreadySettled(submission.commitment_id));
        }

        let previous = self.history.head();

        let event = Axiom::new(&self.history).admit_event(EventInput {
            commitment_id: submission.commitment_id,
            observation: submission.observation,
            previous_event: previous,
            occurred_at: submission.occurred_at,
        })?;

        let id = event.id();

        self.history.put_event(Canonical::new(event, recorded_at)?);
        self.history.advance_head(previous, id)?;

        Ok(id)
    }
}
