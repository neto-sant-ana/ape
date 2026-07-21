//! How a resource is classified and how its quantifiable values are constrained.
//!
//! - `ResourceKind` — a resource is either `Discrete` (not measurable by a numeric
//!   state: a contract, document, order) or `Quantifiable` (measurable: stock,
//!   balance, capacity, hours). A quantifiable resource carries the `Constraint`
//!   that bounds its valid values; a discrete one is constrained instead by its
//!   commitment dependency chain.
//!
//! - `Constraint` — an opaque numeric predicate (`equal`, `between`, ...) built only
//!   through the validating constructors below, so every constraint has finite
//!   bounds and ordered ranges by construction.

use serde::Serialize;

define_value_object! {
    pub enum ResourceKind {
        Discrete,
        Quantifiable(Constraint),
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
enum ConstraintKind {
    Equal(f64),
    NotEqual(f64),
    GreaterThan(f64),
    GreaterThanOrEqual(f64),
    LessThan(f64),
    LessThanOrEqual(f64),
    Between { lower_bound: f64, upper_bound: f64 },
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Constraint(ConstraintKind);
impl Constraint {
    pub fn equal(bound: f64) -> Result<Self, ConstraintError> {
        Self::new(ConstraintKind::Equal(bound))
    }

    pub fn not_equal(bound: f64) -> Result<Self, ConstraintError> {
        Self::new(ConstraintKind::NotEqual(bound))
    }

    pub fn greater_than(bound: f64) -> Result<Self, ConstraintError> {
        Self::new(ConstraintKind::GreaterThan(bound))
    }

    pub fn greater_than_or_equal(bound: f64) -> Result<Self, ConstraintError> {
        Self::new(ConstraintKind::GreaterThanOrEqual(bound))
    }

    pub fn less_than(bound: f64) -> Result<Self, ConstraintError> {
        Self::new(ConstraintKind::LessThan(bound))
    }

    pub fn less_than_or_equal(bound: f64) -> Result<Self, ConstraintError> {
        Self::new(ConstraintKind::LessThanOrEqual(bound))
    }

    pub fn between(lower_bound: f64, upper_bound: f64) -> Result<Self, ConstraintError> {
        Self::new(ConstraintKind::Between {
            lower_bound,
            upper_bound,
        })
    }

    fn new(constraint_kind: ConstraintKind) -> Result<Self, ConstraintError> {
        let constraint = Self(constraint_kind);

        constraint.validate()?;

        Ok(constraint)
    }

    fn validate(&self) -> Result<(), ConstraintError> {
        let finite = |bound: f64| {
            if bound.is_finite() {
                Ok(())
            } else {
                Err(ConstraintError::NonFinite)
            }
        };

        match &self.0 {
            ConstraintKind::Equal(bound)
            | ConstraintKind::NotEqual(bound)
            | ConstraintKind::GreaterThan(bound)
            | ConstraintKind::GreaterThanOrEqual(bound)
            | ConstraintKind::LessThan(bound)
            | ConstraintKind::LessThanOrEqual(bound) => finite(*bound),
            ConstraintKind::Between {
                lower_bound,
                upper_bound,
            } => {
                finite(*lower_bound)?;
                finite(*upper_bound)?;

                if lower_bound > upper_bound {
                    return Err(ConstraintError::InvertedRange);
                }

                Ok(())
            }
        }
    }

    pub fn check(&self, value: f64) -> bool {
        match &self.0 {
            ConstraintKind::Equal(bound) => value == *bound,
            ConstraintKind::NotEqual(bound) => value != *bound,
            ConstraintKind::GreaterThan(bound) => value > *bound,
            ConstraintKind::GreaterThanOrEqual(bound) => value >= *bound,
            ConstraintKind::LessThan(bound) => value < *bound,
            ConstraintKind::LessThanOrEqual(bound) => value <= *bound,
            ConstraintKind::Between {
                lower_bound,
                upper_bound,
            } => *lower_bound <= value && value <= *upper_bound,
        }
    }
}

define_error! {
    pub enum ConstraintError {
        NonFinite => "constraint bounds must be finite numbers",
        InvertedRange => "range lower bound must not exceed the upper bound",
    }
}

#[cfg(test)]
mod tests {
    use super::{Constraint, ConstraintError};

    #[test]
    fn accepts_valid() {
        assert!(Constraint::equal(1.0).is_ok());
        assert!(Constraint::between(1.0, 5.0).is_ok());
        assert!(Constraint::between(5.0, 5.0).is_ok());
    }

    #[test]
    fn rejects_inverted_range() {
        assert!(matches!(
            Constraint::between(5.0, 1.0),
            Err(ConstraintError::InvertedRange)
        ));
    }

    #[test]
    fn rejects_non_finite() {
        assert!(matches!(
            Constraint::equal(f64::NAN),
            Err(ConstraintError::NonFinite)
        ));

        assert!(matches!(
            Constraint::greater_than(f64::INFINITY),
            Err(ConstraintError::NonFinite)
        ));

        assert!(matches!(
            Constraint::between(0.0, f64::NAN),
            Err(ConstraintError::NonFinite)
        ));
    }

    #[test]
    fn check_evaluates_bounds() {
        let c = Constraint::between(0.0, 10.0).unwrap();

        assert!(c.check(0.0) && c.check(5.0) && c.check(10.0));
        assert!(!c.check(-1.0) && !c.check(11.0));
    }
}
