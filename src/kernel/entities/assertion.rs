//! Assertions — immutable knowledge about operational coordination.
//!
//! A `Commitment` is a proposed execution of a statement, completed with an
//! `assignment`, a `term` (when it was committed and when it is due), its
//! `action_value`, and the other commitments it depends on (`dependencies`). Its
//! condition is not stored but derived from those relations and the events, along
//! complementary axes rather than as one exclusive state:
//!
//! - settlement — `Unsettled` until an event settles it per its statement, and
//!   `Fulfilled` or `Cancelled` once one does;
//! - dependencies — whether any is still pending, and whether any can never be
//!   fulfilled, which leaves this one unrealizable;
//! - timeliness — `Breached` once its due date elapses while it is unsettled.
//!
//! One commitment may be unsettled, waiting on a dependency and breached at once,
//! which is why no single state is named here. Nor is it ever invalid: a commitment
//! whose promised effects cannot be realized remains knowledge, and the
//! impossibility is a verdict derived over the graph rather than a property of the
//! assertion.
//!
//! An `Event` is a factual execution of coordination relevance; it settles or
//! cancels a commitment per that commitment's statement, links to the
//! `previous_event` in the chain, and records when it `occurred_at`.
//! 
//! An `EligibilityAssignment` declares the full set of `roles` an agent may
//! assume, effective from `effective_from`. Unlike an `Event`, it is not an
//! observation but a decision, so its date may lie in the future; the roles an
//! agent may assume at a given moment are those carried by the latest assignment
//! whose `effective_from` does not exceed that moment. An empty set withdraws the
//! agent from every role.

use std::collections::BTreeSet;

use crate::kernel::entities::{AgentId, ResourceInstanceId, RoleId, StatementId};

use crate::kernel::value_objects::{ActionValue, Assignment, Date, Observation, Term};

define_id!(CommitmentId);
define_entity! {
    pub struct Commitment(CommitmentId) via CommitmentInput {
        assignment: Assignment,
        statement: StatementId,
        resource: ResourceInstanceId,
        term: Term,
        action_value: ActionValue,
        dependencies: BTreeSet<CommitmentId>,
    }
}

define_id!(EventId);
define_entity! {
    pub struct Event(EventId) via EventInput {
        commitment_id: CommitmentId,
        observation: Observation,
        previous_event: Option<EventId>,
        occurred_at: Date,
    }
}

define_id!(EligibilityAssignmentId);
define_entity! {
    pub struct EligibilityAssignment(EligibilityAssignmentId) via EligibilityAssignmentInput {
        agent: AgentId,
        roles: BTreeSet<RoleId>,
        effective_from: Date,
    }
}
