//! Admission for actions (see `entities/action.rs`).

use super::{Axiom, AxiomError, Knowledge};

use crate::kernel::entities::{Action, ActionInput};

use crate::kernel::value_objects::{ActionKind, ResourceKind};

impl<'k, K: Knowledge> Axiom<'k, K> {
    pub fn admit_action(&self, input: ActionInput) -> Result<Action, AxiomError> {
        let target = self
            .knowledge
            .resource(input.resource)
            .ok_or(AxiomError::UnknownResource(input.resource))?;

        let compatible = matches!(
            (&input.kind, &target.kind),
            (ActionKind::Discrete, ResourceKind::Discrete)
                | (ActionKind::Quantifiable(_), ResourceKind::Quantifiable(_))
        );

        if !compatible {
            return Err(AxiomError::ActionResourceKindMismatch);
        }

        Ok(Action::create(input)?)
    }
}
