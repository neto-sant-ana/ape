//! Assertions admitted as knowledge about operational coordination.
//!
//! A `Commitment` is a proposed execution of a statement, completed with an
//! `assignment`, a `term` (when it was committed and when it is due), an
//! optional commitment it `supersedes`, its `action_value`, and the other
//! commitments it depends on (`dependencies`). Its
//! state is not stored but derived from those relations and the events:
//!
//! - `Open` — valid, on schedule, not blocked by a dependency.
//! - `Breached` — valid, off schedule, not blocked.
//! - `Blocked` — valid, blocked by a dependency's execution.
//! - `Fulfilled` — valid, all of its scope completed.
//! - `Cancelled` — valid, cancelled by an event.
//! - `Superseded` — invalid, replaced by a newer valid commitment.
//! - `Invalid` — invalidated by a dependency or a system constraint.
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
        supersedes: Option<CommitmentId>,
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
