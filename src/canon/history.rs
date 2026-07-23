//! The canonical history repository: the append side of the Canon.
//!
//! `Knowledge` (in the kernel) is the read side of the same repository; this is
//! the write side. Reads are advisory, or load-bearing for construction; the
//! guarantee that preserves history lives in the atomic append conditions here.

use crate::kernel::entities::{Commitment, EligibilityAssignment, Event, EventId};

use super::{CanonError, Canonical};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppendOutcome {
    Admitted,
    AlreadyPresent,
}

pub trait CanonicalHistory {
    fn head(&self) -> Option<EventId>;
    fn append_commitment(
        &mut self,
        commitment: Canonical<Commitment>,
    ) -> Result<AppendOutcome, CanonError>;
    fn append_eligibility(
        &mut self,
        eligibility: Canonical<EligibilityAssignment>,
    ) -> Result<AppendOutcome, CanonError>;
    fn append_event(&mut self, event: Canonical<Event>) -> Result<AppendOutcome, CanonError>;
}
