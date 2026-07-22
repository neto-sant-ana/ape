//! Admission for the agent family (see `entities/agent.rs`).

use super::{Axiom, AxiomError, Knowledge};

use crate::kernel::entities::{Agent, AgentInput, Role, RoleInput};

impl<'k, K: Knowledge> Axiom<'k, K> {
    pub fn admit_role(&self, input: RoleInput) -> Result<Role, AxiomError> {
        Ok(Role::create(input)?)
    }

    pub fn admit_agent(&self, input: AgentInput) -> Result<Agent, AxiomError> {
        Ok(Agent::create(input)?)
    }
}
