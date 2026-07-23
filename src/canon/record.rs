//! `Canonical<T>` — a kernel assertion wrapped with the metadata of its admission
//! into canonical history.
//!
//! `recorded_at` is when the knowledge entered the system, distinct from an
//! assertion's own factual time (`occurred_at`, `committed_at`).

use crate::kernel::value_objects::Date;

#[derive(Debug, Clone)]
pub struct Canonical<T> {
    assertion: T,
    recorded_at: Date,
}
impl<T> Canonical<T> {
    pub fn new(assertion: T, recorded_at: Date) -> Self {
        Self {
            assertion,
            recorded_at,
        }
    }

    pub fn assertion(&self) -> &T {
        &self.assertion
    }

    pub fn recorded_at(&self) -> &Date {
        &self.recorded_at
    }
}
