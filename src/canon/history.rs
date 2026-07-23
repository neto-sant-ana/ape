//! The canonical history repository: the append side of the Canon.
//!
//! An adapter provides durable storage and one atomic compare-and-swap; it holds
//! no admission *rules*. Those live once in the Canon, which composes these
//! primitives.
//!
//! - `put_*` is a content-addressed put-if-absent: the id is the hash of the
//!   content, so it is idempotent by construction and a re-put is a no-op.
//! 
//! - `advance_head` is a compare-and-swap on the single mutable pointer of the
//!   event chain.

use crate::kernel::entities::{Commitment, EligibilityAssignment, Event, EventId};

use super::{CanonError, Canonical};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppendOutcome {
    Admitted,
    AlreadyPresent,
}

pub trait CanonicalHistory {
    fn head(&self) -> Option<EventId>;
    fn put_commitment(&mut self, commitment: Canonical<Commitment>) -> AppendOutcome;
    fn put_eligibility(&mut self, eligibility: Canonical<EligibilityAssignment>) -> AppendOutcome;
    fn put_event(&mut self, event: Canonical<Event>) -> AppendOutcome;

    /// Compare-and-swap the chain head: set it to `new` only while it still equals
    /// `expected`. `expected == None` requires the chain to be empty.
    fn advance_head(&mut self, expected: Option<EventId>, new: EventId)
    -> Result<(), CanonError>;
}
