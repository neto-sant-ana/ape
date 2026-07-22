//! Admission for statements (see `entities/statement.rs`).

use super::{Axiom, AxiomError, Knowledge};

use crate::kernel::entities::{Statement, StatementInput};

impl<'k, K: Knowledge> Axiom<'k, K> {
    pub fn admit_statement(&self, input: StatementInput) -> Result<Statement, AxiomError> {
        if self.knowledge.action(input.action).is_none() {
            return Err(AxiomError::UnknownAction(input.action));
        }

        for role in input
            .participants
            .actors()
            .iter()
            .chain(input.participants.recipients())
        {
            if self.knowledge.role(*role).is_none() {
                return Err(AxiomError::UnknownRole(*role));
            }
        }

        Ok(Statement::create(input)?)
    }
}
