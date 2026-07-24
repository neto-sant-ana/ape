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

use super::{CanonError, Canonical};

use crate::kernel::axiom::Knowledge;

use crate::kernel::entities::{
    Action, Agent, Commitment, CommitmentId, EligibilityAssignment, Event, EventId, Resource,
    ResourceInstance, Role, Statement,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppendOutcome {
    Admitted,
    AlreadyPresent,
}

pub trait CanonicalHistory: Knowledge {
    fn head(&self) -> Option<EventId>;

    fn event_of(&self, commitment: CommitmentId) -> Option<&Event>;

    /// The canonical records — the assertion together with its `recorded_at` —
    /// behind the bare-entity reads inherited from `Knowledge`. A projection reads
    /// these to answer questions as of a recording instant; the Axiom, validating
    /// structure only, needs just the bare entity.
    fn canonical_commitment(&self, id: CommitmentId) -> Option<&Canonical<Commitment>>;
    fn canonical_event(&self, id: EventId) -> Option<&Canonical<Event>>;

    fn put_role(&mut self, role: Canonical<Role>) -> AppendOutcome;
    fn put_agent(&mut self, agent: Canonical<Agent>) -> AppendOutcome;
    fn put_resource(&mut self, resource: Canonical<Resource>) -> AppendOutcome;
    fn put_resource_instance(&mut self, instance: Canonical<ResourceInstance>) -> AppendOutcome;
    fn put_action(&mut self, action: Canonical<Action>) -> AppendOutcome;
    fn put_statement(&mut self, statement: Canonical<Statement>) -> AppendOutcome;

    fn put_commitment(&mut self, commitment: Canonical<Commitment>) -> AppendOutcome;
    fn put_eligibility(&mut self, eligibility: Canonical<EligibilityAssignment>) -> AppendOutcome;

    /// Append `event` to the chain atomically: persisting it, indexing it by
    /// commitment, and advancing the head happen together, and only while the head
    /// still equals the event's `previous_event`. A stale head is refused with
    /// [`CanonError::UnexpectedHead`], leaving no trace.
    ///
    /// A refusal is not the adapter's to retry, nor the caller's to paper over by
    /// re-stamping the event and resubmitting here. Recovery is a fresh admission through
    /// the Canon, which re-runs the settle-once check.
    fn append_event(&mut self, event: Canonical<Event>) -> Result<AppendOutcome, CanonError>;
}
