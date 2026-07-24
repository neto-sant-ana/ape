//! `Canonical<T>` — a kernel assertion wrapped with the metadata of its admission
//! into canonical history.
//!
//! `recorded_at` is when the knowledge entered the system, distinct from an
//! assertion's own factual time (`occurred_at`, `committed_at`).

use super::CanonError;

use crate::kernel::entities::{
    Action, Agent, Commitment, EligibilityAssignment, Event, Resource, ResourceInstance, Role,
    Statement,
};

use crate::kernel::value_objects::Date;

pub trait FactualPast {
    fn factual_past(&self) -> Option<&Date>;
}

impl FactualPast for Commitment {
    fn factual_past(&self) -> Option<&Date> {
        Some(self.term().committed_at())
    }
}
impl FactualPast for Event {
    fn factual_past(&self) -> Option<&Date> {
        Some(self.occurred_at())
    }
}
no_factual_past!(
    Role,
    Agent,
    Resource,
    ResourceInstance,
    Action,
    Statement,
    EligibilityAssignment,
);

#[derive(Debug, Clone)]
pub struct Canonical<T> {
    assertion: T,
    recorded_at: Date,
}
impl<T> Canonical<T> {
    pub fn assertion(&self) -> &T {
        &self.assertion
    }

    pub fn recorded_at(&self) -> &Date {
        &self.recorded_at
    }
}
impl<T: FactualPast> Canonical<T> {
    pub(crate) fn new(assertion: T, recorded_at: Date) -> Result<Self, CanonError> {
        if let Some(fact) = assertion.factual_past()
            && !fact.up_to(&recorded_at)
        {
            return Err(CanonError::RecordedBeforeFact {
                fact: *fact,
                recorded_at,
            });
        }

        Ok(Self {
            assertion,
            recorded_at,
        })
    }
}
