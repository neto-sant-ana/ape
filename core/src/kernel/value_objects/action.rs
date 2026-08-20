//! Value objects describing what an action does to a resource.
//!
//! - `ActionKind` — whether the action targets a `Discrete` resource or a
//!   `Quantifiable` one, the latter carrying the `Effect` it has on the level.
//!
//! - `Effect` — the direction of a quantifiable action: `Increase` or `Decrease`.
//!
//! - `ActionValue` — the magnitude a quantifiable action moves the level by
//!   (`Value`), or `None` for a discrete action.
//!
//! # The magnitude is a count, and the engine does not know of what
//!
//! A level is a **sum** of movements and nothing here ever multiplies or divides one, so an integer
//! count is exact and associative where a binary float is neither — and the two hypotheses a caller
//! may ask under fold in different orders, which is a way for two answers over identical knowledge to
//! disagree by arithmetic alone.
//!
//! What the count counts is **not here**. Cents, whole items, thirds of an hour, pallets of forty
//! eight: the engine adds and compares, and which unit an application means is that application's,
//! because a resource is the axis its own movements move along and there is no second one to
//! reconcile. Naming the unit here — as a decimal scale, or as anything else — would make every
//! application share one approach to units, and the whole point of a minimal ontology is that they
//! need not.

use serde::Serialize;

define_value_object! {
    pub enum ActionKind {
        Discrete,
        Quantifiable(Effect),
    }
}

define_value_object! {
    pub enum Effect {
        Increase,
        Decrease,
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
enum ActionValueKind {
    None,
    Value(u128),
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ActionValue(ActionValueKind);
impl ActionValue {
    pub fn none() -> Self {
        Self(ActionValueKind::None)
    }

    /// A magnitude, which is unsigned: the direction is the [`Effect`]'s and cannot be given here.
    ///
    /// Two refusals went away and neither by being checked more carefully. `NonFinite` left with the
    /// float — a count has no infinity and no NaN — and a *negative* magnitude stopped being
    /// something a caller can write down at all. What is left is zero, which is the one value the
    /// type still admits and the ontology does not.
    pub fn value(magnitude: u128) -> Result<Self, ActionValueError> {
        if magnitude == 0 {
            return Err(ActionValueError::Zero);
        }

        Ok(Self(ActionValueKind::Value(magnitude)))
    }

    pub fn as_value(&self) -> Option<u128> {
        match &self.0 {
            ActionValueKind::None => None,
            ActionValueKind::Value(magnitude) => Some(*magnitude),
        }
    }
}

define_error! {
    pub enum ActionValueError {
        Zero => "action value must be a magnitude, and zero is not one",
    }
}

#[cfg(test)]
mod tests {
    use super::{ActionValue, ActionValueError};

    #[test]
    fn none_carries_no_value() {
        assert_eq!(ActionValue::none().as_value(), None);
    }

    #[test]
    fn value_accepts_a_positive_count() {
        assert_eq!(ActionValue::value(25).unwrap().as_value(), Some(25));
    }

    /// Zero is the whole of what is left to refuse.
    ///
    /// This test used to have a second half, for a negative magnitude. It is gone because the case is
    /// gone: `ActionValue::value(-1)` no longer compiles, so there is nothing left here to assert
    /// about it. A shrinking test is what moving an invariant into a type looks like.
    #[test]
    fn value_rejects_zero() {
        assert!(matches!(ActionValue::value(0), Err(ActionValueError::Zero)));
    }
}
