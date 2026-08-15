//! `Movement` — how much a commitment moves a quantifiable resource, and in which direction.
//!
//! The arithmetic is small and it is the engine's, because it is semantics rather than
//! presentation: an `Increase` adds and a `Decrease` subtracts, a `Discrete` action moves
//! no level at all, and the three ways those can disagree with a commitment's value or its
//! resource are errors rather than silences.
//!
//! It is public because an application cannot avoid needing it. A level is a sum over
//! movements meeting some criterion, and *which* criterion is the question being asked —
//! what has settled, what will have settled, what is at stake before a deadline are
//! different numbers over the same knowledge. Choosing between them is the application's;
//! knowing what one commitment contributes is not, and an application left to work it out
//! from `ActionKind` and `ActionValue` would be keeping a second copy of this file that
//! nothing compares against the first.
//!
//! Feasibility needs the bounds alongside the movement, so it reads
//! [`bounded_movement_of`]; everything else reads [`movement_of`], which is the same
//! derivation with the bounds dropped.

use crate::engine::hermeneia::HermeneiaError;

use crate::kernel::axiom::Knowledge;

use crate::kernel::entities::{Commitment, ResourceInstanceId};

use crate::kernel::value_objects::{ActionKind, Constraint, Effect, ResourceKind};

/// The signed level a commitment moves on one resource instance.
#[derive(Debug, Clone, PartialEq)]
pub struct Movement {
    instance: ResourceInstanceId,
    magnitude: f64,
}

impl Movement {
    pub fn instance(&self) -> ResourceInstanceId {
        self.instance
    }

    /// Positive for an increase, negative for a decrease.
    pub fn magnitude(&self) -> f64 {
        self.magnitude
    }
}

/// How much `commitment` moves its resource instance, or nothing when it moves no level.
pub fn movement_of<K: Knowledge>(
    knowledge: &K,
    commitment: &Commitment,
) -> Result<Option<Movement>, HermeneiaError> {
    Ok(bounded_movement_of(knowledge, commitment)?.map(|(movement, _)| movement))
}

/// The level a commitment moves, together with the bounds that level answers to.
pub(crate) fn bounded_movement_of<K: Knowledge>(
    knowledge: &K,
    commitment: &Commitment,
) -> Result<Option<(Movement, Constraint)>, HermeneiaError> {
    let statement_id = *commitment.statement();
    let statement = knowledge
        .statement(statement_id)
        .ok_or(HermeneiaError::UnknownStatement {
            commitment: commitment.id(),
            statement: statement_id,
        })?;

    let action_id = *statement.action();
    let action = knowledge
        .action(action_id)
        .ok_or(HermeneiaError::UnknownAction {
            statement: statement_id,
            action: action_id,
        })?;

    let effect = match (action.kind(), commitment.action_value().as_value()) {
        (ActionKind::Discrete, None) => return Ok(None),
        (ActionKind::Quantifiable(effect), Some(magnitude)) => (effect, magnitude),
        _ => return Err(HermeneiaError::ActionValueMismatch(commitment.id())),
    };

    let instance_id = *commitment.resource();
    let instance = knowledge.resource_instance(instance_id).ok_or(
        HermeneiaError::UnknownResourceInstance {
            commitment: commitment.id(),
            instance: instance_id,
        },
    )?;

    let resource_id = *instance.resource();
    let resource = knowledge
        .resource(resource_id)
        .ok_or(HermeneiaError::UnknownResource {
            instance: instance_id,
            resource: resource_id,
        })?;

    let ResourceKind::Quantifiable(constraint) = resource.kind() else {
        return Err(HermeneiaError::ActionResourceKindMismatch(commitment.id()));
    };

    let (effect, magnitude) = effect;

    Ok(Some((
        Movement {
            instance: instance_id,
            magnitude: match effect {
                Effect::Increase => magnitude,
                Effect::Decrease => -magnitude,
            },
        },
        constraint.clone(),
    )))
}
