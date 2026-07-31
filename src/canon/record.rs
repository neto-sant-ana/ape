//! `Canonical<T>` — a kernel assertion wrapped with the metadata of its admission
//! into canonical history.
//!
//! `recorded_at` is when the knowledge entered the system, distinct from an
//! assertion's own time (`occurred_at`, `committed_at`).

use super::CanonError;

use crate::kernel::entities::{
    Action, Agent, Commitment, EligibilityAssignment, Event, Resource, ResourceInstance, Role,
    Statement,
};

use crate::kernel::value_objects::Date;

/// The earliest instant at which an assertion could have been recorded — the lower
/// bound its `recorded_at` must not precede.
///
/// An `Event` was observed (`occurred_at`) and a `Commitment` was decided
/// (`committed_at`), so recording cannot predate that instant. Definitional
/// entities and eligibility (a forward declaration whose `effective_from` may be
/// future) impose no lower bound.
///
/// This bound is the assertion's own, and having none is not a licence to record at
/// any instant: admission keeps recording monotonic, so nothing enters before
/// knowledge already admitted — including the entities it references, which were
/// admitted earlier and therefore recorded no later than it.
pub trait RecordableAfter {
    fn recordable_after(&self) -> Option<&Date>;
}

impl RecordableAfter for Commitment {
    fn recordable_after(&self) -> Option<&Date> {
        Some(self.term().committed_at())
    }
}
impl RecordableAfter for Event {
    fn recordable_after(&self) -> Option<&Date> {
        Some(self.occurred_at())
    }
}
recordable_unanchored!(
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
impl<T: RecordableAfter> Canonical<T> {
    pub(crate) fn new(assertion: T, recorded_at: Date) -> Result<Self, CanonError> {
        if let Some(earliest) = assertion.recordable_after()
            && !earliest.up_to(&recorded_at)
        {
            return Err(CanonError::RecordedTooEarly {
                earliest: *earliest,
                recorded_at,
            });
        }

        Ok(Self {
            assertion,
            recorded_at,
        })
    }
}
