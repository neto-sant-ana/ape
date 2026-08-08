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

/// The body of canonically admitted knowledge, read as records rather than bare entities.
pub trait CanonicalKnowledge {
    fn canonical_commitment(&self, id: CommitmentId) -> Option<Canonical<Commitment>>;
    fn canonical_event(&self, id: EventId) -> Option<Canonical<Event>>;

    /// The head as of `at`: the latest Event recorded no later than that instant, or `None`
    /// while none had been.
    ///
    /// This is what lets a reader address a whole knowledge cut — the Commitments recorded by an
    /// instant *and* the chain that was current at it — without asking what is current now. A
    /// reader names an instant it already holds and learns nothing of an Event admitted after it,
    /// which is the difference between a Thesis that fell behind and one that set aside facts it
    /// knew.
    ///
    /// The answer is well defined because recording is monotonic across admission: `recorded_at`
    /// never decreases along the chain, so the Events recorded no later than `at` are a prefix of
    /// it and this is that prefix's last. Where several share the instant, it is the last of them
    /// in chain order — the coarser cut an instant addresses, which naming a head directly
    /// refines.
    fn head_as_of(&self, at: &Date) -> Option<EventId>;
}

pub trait CanonicalHistory: Knowledge + CanonicalKnowledge {
    fn head(&self) -> Option<EventId>;

    /// The instant through which this history is recorded: the greatest `recorded_at`
    /// admitted so far, or `None` while the history is empty.
    fn recorded_through(&self) -> Option<Date>;

    fn event_of(&self, commitment: CommitmentId) -> Option<Event>;

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
    /// Checking the watermark is not advancing it. A refused append leaves
    /// [`CanonicalHistory::recorded_through`] where it was, because a watermark moved by a
    /// write that never happened is a trace like any other — and one that would refuse a
    /// later record the history should have taken.
    ///
    /// A refusal is not the adapter's to retry, nor the caller's to paper over by
    /// re-stamping the event and resubmitting here. Recovery is a fresh admission through
    /// the Canon, which re-runs the settle-once check.
    fn append_event(&mut self, event: Canonical<Event>) -> Result<AppendOutcome, CanonError>;
}
