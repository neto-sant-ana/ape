//! The canonical history repository: the append side of the Canon.
//!
//! An adapter provides durable storage and the atomic ordering primitives; it holds
//! no admission *rules*. Those live once in the Canon, which composes these
//! primitives.
//!
//! - `put_*` is a content-addressed put-if-absent: the id is the hash of the
//!   content, so it is idempotent by construction and a re-put is a no-op.
//!
//! - `append_event` is an atomic compare-and-append: it persists the event,
//!   indexes it by commitment and advances the single event-chain head only
//!   if the event still extends the current head.
//!
//! Every admission is additionally guarded by a monotonic recording watermark: an
//! assertion may not be recorded before knowledge already admitted.

use super::{CanonError, Canonical};

use crate::kernel::axiom::Knowledge;

use crate::kernel::entities::{
    Action, Agent, Commitment, CommitmentId, EligibilityAssignment, Event, EventId, Resource,
    ResourceInstance, Role, Statement,
};

use crate::kernel::value_objects::Date;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppendOutcome {
    Admitted,
    AlreadyPresent,
}

pub trait CanonicalHistory: Knowledge {
    fn head(&self) -> Option<EventId>;

    /// The instant through which this history is recorded: the greatest `recorded_at`
    /// admitted so far, or `None` while the history is empty.
    fn recorded_through(&self) -> Option<Date>;

    fn event_of(&self, commitment: CommitmentId) -> Option<Event>;

    /// The canonical records — the assertion together with its `recorded_at` —
    /// behind the bare-entity reads inherited from `Knowledge`. A projection reads
    /// these to answer questions as of a recording instant; the Axiom, validating
    /// structure only, needs just the bare entity.
    fn canonical_commitment(&self, id: CommitmentId) -> Option<Canonical<Commitment>>;
    fn canonical_event(&self, id: EventId) -> Option<Canonical<Event>>;

    /// Put the record if its id is absent, refusing with
    /// [`CanonError::RecordedOutOfOrder`] a recording instant that precedes
    /// [`CanonicalHistory::recorded_through`].
    fn put_role(&mut self, role: Canonical<Role>) -> Result<AppendOutcome, CanonError>;
    fn put_agent(&mut self, agent: Canonical<Agent>) -> Result<AppendOutcome, CanonError>;
    fn put_resource(&mut self, resource: Canonical<Resource>) -> Result<AppendOutcome, CanonError>;
    fn put_resource_instance(
        &mut self,
        instance: Canonical<ResourceInstance>,
    ) -> Result<AppendOutcome, CanonError>;
    fn put_action(&mut self, action: Canonical<Action>) -> Result<AppendOutcome, CanonError>;
    fn put_statement(
        &mut self,
        statement: Canonical<Statement>,
    ) -> Result<AppendOutcome, CanonError>;

    fn put_commitment(
        &mut self,
        commitment: Canonical<Commitment>,
    ) -> Result<AppendOutcome, CanonError>;
    fn put_eligibility(
        &mut self,
        eligibility: Canonical<EligibilityAssignment>,
    ) -> Result<AppendOutcome, CanonError>;

    /// Append `event` to the chain atomically: persisting it, indexing it by
    /// commitment, and advancing the head happen together, and only while the head
    /// still equals the event's `previous_event`. A stale head is refused with
    /// [`CanonError::UnexpectedHead`], leaving no trace.
    ///
    /// The watermark is checked before the head, because the two refusals call for
    /// opposite responses: a stale head invites a fresh admission, while a back-dated
    /// instant would have that admission fail identically forever.
    ///
    /// A refusal is not the adapter's to retry, nor the caller's to paper over by
    /// re-stamping the event and resubmitting here. Recovery is a fresh admission through
    /// the Canon, which re-runs the settle-once check.
    fn append_event(&mut self, event: Canonical<Event>) -> Result<AppendOutcome, CanonError>;
}
