//! Value objects describing what an action does to a resource.
//!
//! - `ActionKind` — whether the action targets a `Discrete` resource or a
//!   `Quantifiable` one, the latter carrying the `Effect` it has on the level.
//!
//! - `Effect` — the direction of a quantifiable action: `Increase` or `Decrease`.
//!
//! - `ActionValue` — the magnitude a quantifiable action moves the level by
//!   (`Value`), or `None` for a discrete action.

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
    Value(f64),
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ActionValue(ActionValueKind);
impl ActionValue {
    pub fn none() -> Self {
        Self(ActionValueKind::None)
    }

    pub fn value(magnitude: f64) -> Result<Self, ActionValueError> {
        if !magnitude.is_finite() {
            return Err(ActionValueError::NonFinite);
        }

        if magnitude <= 0.0 {
            return Err(ActionValueError::NonPositive);
        }

        Ok(Self(ActionValueKind::Value(magnitude)))
    }

    pub fn as_value(&self) -> Option<f64> {
        match &self.0 {
            ActionValueKind::None => None,
            ActionValueKind::Value(magnitude) => Some(*magnitude),
        }
    }
}

define_error! {
    pub enum ActionValueError {
        NonFinite => "action value must be a finite number",
        NonPositive => "action value must be a positive magnitude (direction is given by the effect)",
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
    fn value_accepts_positive_finite() {
        assert_eq!(ActionValue::value(2.5).unwrap().as_value(), Some(2.5));
    }

    #[test]
    fn value_rejects_non_positive() {
        assert!(matches!(
            ActionValue::value(0.0),
            Err(ActionValueError::NonPositive)
        ));

        assert!(matches!(
            ActionValue::value(-1.0),
            Err(ActionValueError::NonPositive)
        ));
    }

    #[test]
    fn value_rejects_non_finite() {
        assert!(matches!(
            ActionValue::value(f64::NAN),
            Err(ActionValueError::NonFinite)
        ));

        assert!(matches!(
            ActionValue::value(f64::INFINITY),
            Err(ActionValueError::NonFinite)
        ));
    }
}
