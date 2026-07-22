//! Admission for the agent family (see `entities/agent.rs`).

use super::{Axiom, AxiomError, Knowledge};

use crate::kernel::entities::{Agent, AgentInput, Role, RoleInput};

impl<'k, K: Knowledge> Axiom<'k, K> {
    pub fn admit_role(&self, input: RoleInput) -> Result<Role, AxiomError> {
        Ok(Role::create(input)?)
    }

    pub fn admit_agent(&self, input: AgentInput) -> Result<Agent, AxiomError> {
        for role in input.eligibility.roles() {
            if self.knowledge.role(*role).is_none() {
                return Err(AxiomError::UnknownRole(*role));
            }
        }

        Ok(Agent::create(input)?)
    }
}
