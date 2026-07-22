//! Admission for the assertion family (see `entities/assertion.rs`).

use super::{Axiom, AxiomError, Knowledge};

use crate::kernel::entities::{Commitment, CommitmentInput, Event, EventInput};

use crate::kernel::value_objects::ActionKind;

impl<'k, K: Knowledge> Axiom<'k, K> {
    pub fn admit_commitment(&self, input: CommitmentInput) -> Result<Commitment, AxiomError> {
        let stmt = self
            .knowledge
            .statement(input.statement)
            .ok_or(AxiomError::UnknownStatement(input.statement))?;

        let instance = self
            .knowledge
            .resource_instance(input.resource)
            .ok_or(AxiomError::UnknownResourceInstance(input.resource))?;

        let action = self
            .knowledge
            .action(stmt.action)
            .ok_or(AxiomError::UnknownAction(stmt.action))?;

        if instance.resource != action.resource {
            return Err(AxiomError::ResourceInstanceMismatch {
                expected: action.resource,
                found: instance.resource,
            });
        }

        let value_matches_kind = match &action.kind {
            ActionKind::Discrete => input.action_value.as_value().is_none(),
            ActionKind::Quantifiable(_) => input.action_value.as_value().is_some(),
        };

        if !value_matches_kind {
            return Err(AxiomError::ActionValueMismatch);
        }

        if self.knowledge.agent(input.assignment.accountable()).is_none() {
            return Err(AxiomError::UnknownAgent(input.assignment.accountable()));
        }

        for executor in input.assignment.executors() {
            let agent = self
                .knowledge
                .agent(*executor)
                .ok_or(AxiomError::UnknownAgent(*executor))?;

            if !agent.eligibility.can_assume_any(stmt.participants.actors()) {
                return Err(AxiomError::IneligibleExecutor(*executor));
            }
        }

        for beneficiary in input.assignment.beneficiaries() {
            let agent = self
                .knowledge
                .agent(*beneficiary)
                .ok_or(AxiomError::UnknownAgent(*beneficiary))?;

            if !agent
                .eligibility
                .can_assume_any(stmt.participants.recipients())
            {
                return Err(AxiomError::IneligibleBeneficiary(*beneficiary));
            }
        }

        if let Some(superseded_id) = input.supersedes {
            let superseded = self
                .knowledge
                .commitment(superseded_id)
                .ok_or(AxiomError::UnknownCommitment(superseded_id))?;

            if superseded.statement != input.statement {
                return Err(AxiomError::SupersedeStatementMismatch);
            }
        }

        for dependency in &input.dependencies {
            if self.knowledge.commitment(*dependency).is_none() {
                return Err(AxiomError::UnknownCommitment(*dependency));
            }
        }

        Ok(Commitment::create(input)?)
    }

    pub fn admit_event(&self, input: EventInput) -> Result<Event, AxiomError> {
        let commitment = self
            .knowledge
            .commitment(input.commitment_id)
            .ok_or(AxiomError::UnknownCommitment(input.commitment_id))?;

        let stmt = self
            .knowledge
            .statement(commitment.statement)
            .ok_or(AxiomError::UnknownStatement(commitment.statement))?;

        if !(stmt.settlement.can_settle(&input.observation)
            || stmt.settlement.can_cancel(&input.observation))
        {
            return Err(AxiomError::ObservationNotSettling);
        }

        if let Some(previous) = input.previous_event
            && self.knowledge.event(previous).is_none()
        {
            return Err(AxiomError::UnknownEvent(previous));
        }

        Ok(Event::create(input)?)
    }
}
