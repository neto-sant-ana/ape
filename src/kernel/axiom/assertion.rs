//! Admission for the assertion family (see `entities/assertion.rs`).

use std::collections::BTreeSet;

use super::{Axiom, AxiomError, Knowledge};

use crate::kernel::entities::{
    AgentId, Commitment, CommitmentInput, EligibilityAssignment, EligibilityAssignmentInput, Event,
    EventInput, RoleId,
};

use crate::kernel::value_objects::{ActionKind, Date};

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
            .action(*stmt.action())
            .ok_or(AxiomError::UnknownAction(*stmt.action()))?;

        if instance.resource() != action.resource() {
            return Err(AxiomError::ResourceInstanceMismatch {
                expected: *action.resource(),
                found: *instance.resource(),
            });
        }

        let value_matches_kind = match action.kind() {
            ActionKind::Discrete => input.action_value.as_value().is_none(),
            ActionKind::Quantifiable(_) => input.action_value.as_value().is_some(),
        };

        if !value_matches_kind {
            return Err(AxiomError::ActionValueMismatch);
        }

        if self.knowledge.agent(input.assignment.accountable()).is_none() {
            return Err(AxiomError::UnknownAgent(input.assignment.accountable()));
        }

        let committed_at = input.term.committed_at();

        for executor in input.assignment.executors() {
            self.require_eligible(*executor, stmt.participants().actors(), committed_at)?;
        }

        for beneficiary in input.assignment.beneficiaries() {
            self.require_eligible(*beneficiary, stmt.participants().recipients(), committed_at)?;
        }

        if let Some(superseded_id) = input.supersedes {
            let superseded = self
                .knowledge
                .commitment(superseded_id)
                .ok_or(AxiomError::UnknownCommitment(superseded_id))?;

            if *superseded.statement() != input.statement {
                return Err(AxiomError::SupersedeStatementMismatch);
            }

            if *superseded.resource() != input.resource {
                return Err(AxiomError::SupersedeResourceInstanceMismatch);
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
            .statement(*commitment.statement())
            .ok_or(AxiomError::UnknownStatement(*commitment.statement()))?;

        if !(stmt.settlement().can_settle(&input.observation)
            || stmt.settlement().can_cancel(&input.observation))
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

    pub fn admit_eligibility_assignment(
        &self,
        input: EligibilityAssignmentInput,
    ) -> Result<EligibilityAssignment, AxiomError> {
        if self.knowledge.agent(input.agent).is_none() {
            return Err(AxiomError::UnknownAgent(input.agent));
        }

        for role in &input.roles {
            if self.knowledge.role(*role).is_none() {
                return Err(AxiomError::UnknownRole(*role));
            }
        }

        // An empty role set is admissible on purpose: it is how an agent is
        // withdrawn from every role as of `occurred_at`, without inventing a
        // fictitious role. So there is no non-empty requirement here.
        Ok(EligibilityAssignment::create(input)?)
    }

    fn require_eligible(
        &self,
        agent: AgentId,
        roles: &BTreeSet<RoleId>,
        at: &Date,
    ) -> Result<(), AxiomError> {
        if self.knowledge.agent(agent).is_none() {
            return Err(AxiomError::UnknownAgent(agent));
        }

        let eligible = self
            .knowledge
            .eligibility_at(agent, at)
            .is_some_and(|assignment| !assignment.roles().is_disjoint(roles));

        if eligible {
            Ok(())
        } else {
            Err(AxiomError::AgentNotEligibleForRole(agent))
        }
    }
}
