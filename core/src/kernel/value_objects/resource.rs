//! How a resource is classified and how its quantifiable values are constrained.
//!
//! - `ResourceKind` — a resource is either `Discrete` (not measurable by a numeric
//!   state: a contract, document, order) or `Quantifiable` (measurable: stock,
//!   balance, capacity, hours). A quantifiable resource carries the `Constraint`
//!   that bounds its valid values; a discrete one is constrained instead by its
//!   commitment dependency chain.
//!
//! - `Constraint` — an opaque predicate (`equal`, `between`, ...) over counts of whatever unit the
//!   resource's movements are counted in. What that unit is belongs to the application, for the
//!   reason [`super::ActionValue`] gives.
//!
//! The bounds are counts and so is every level weighed against them, which leaves one thing to
//! validate — a range must not be inverted — and takes two away. `NonFinite` is gone from bounds and
//! from levels alike, so six of the seven constructors cannot fail and no longer pretend to, and
//! `check` is a comparison again rather than something that can refuse.
//!
//! The predicates are a closed vocabulary, and that is structural rather than a preference: the
//! engine has to *evaluate* a bound in order to report a conflict, and an identity derived from
//! content cannot be derived from a closure.

use serde::Serialize;

define_value_object! {
    pub enum ResourceKind {
        Discrete,
        Quantifiable(Constraint),
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
enum ConstraintKind {
    Equal(i128),
    NotEqual(i128),
    GreaterThan(i128),
    GreaterThanOrEqual(i128),
    LessThan(i128),
    LessThanOrEqual(i128),
    Between {
        lower_bound: i128,
        upper_bound: i128,
    },
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Constraint(ConstraintKind);
impl Constraint {
    pub fn equal(bound: i128) -> Self {
        Self(ConstraintKind::Equal(bound))
    }

    pub fn not_equal(bound: i128) -> Self {
        Self(ConstraintKind::NotEqual(bound))
    }

    pub fn greater_than(bound: i128) -> Self {
        Self(ConstraintKind::GreaterThan(bound))
    }

    pub fn greater_than_or_equal(bound: i128) -> Self {
        Self(ConstraintKind::GreaterThanOrEqual(bound))
    }

    pub fn less_than(bound: i128) -> Self {
        Self(ConstraintKind::LessThan(bound))
    }

    pub fn less_than_or_equal(bound: i128) -> Self {
        Self(ConstraintKind::LessThanOrEqual(bound))
    }

    /// The only constructor that can still fail, because it is the only one with two bounds to
    /// contradict each other.
    pub fn between(lower_bound: i128, upper_bound: i128) -> Result<Self, ConstraintError> {
        if lower_bound > upper_bound {
            return Err(ConstraintError::InvertedRange);
        }

        Ok(Self(ConstraintKind::Between {
            lower_bound,
            upper_bound,
        }))
    }

    pub fn check(&self, value: i128) -> bool {
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
        InvertedRange => "range lower bound must not exceed the upper bound",
    }
}

#[cfg(test)]
mod tests {
    use super::{Constraint, ConstraintError};

    #[test]
    fn accepts_a_range_and_a_degenerate_one() {
        assert!(Constraint::between(1, 5).is_ok());
        assert!(Constraint::between(5, 5).is_ok());
    }

    #[test]
    fn rejects_inverted_range() {
        assert!(matches!(
            Constraint::between(5, 1),
            Err(ConstraintError::InvertedRange)
        ));
    }

    #[test]
    fn check_evaluates_bounds() {
        let c = Constraint::between(0, 10).unwrap();

        assert!(c.check(0) && c.check(5) && c.check(10));
        assert!(!c.check(-1) && !c.check(11));
    }

    /// Every predicate, on its own bound, so a strict one is not mistaken for a closed one.
    ///
    /// Six of the seven constructors changed shape here, and only `between` had a test before.
    #[test]
    fn each_predicate_answers_at_its_bound() {
        assert!(!Constraint::greater_than(10).check(10));
        assert!(Constraint::greater_than_or_equal(10).check(10));
        assert!(!Constraint::less_than(10).check(10));
        assert!(Constraint::less_than_or_equal(10).check(10));
        assert!(Constraint::equal(10).check(10));
        assert!(!Constraint::not_equal(10).check(10));
    }
}
