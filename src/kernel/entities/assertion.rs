//! Assertions admitted as knowledge about operational coordination.
//!
//! A `Commitment` is a proposed execution of a statement, completed with an
//! `assignment`, a `due_date`, an optional commitment it `supersedes`, its
//! `action_value`, and the other commitments it depends on (`dependencies`). Its
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
//! `previous_event` in the chain, and carries the `occurrence` pairing when it
//! happened with when it was recorded.
//! 
//! An `EligibilityAssignment` asserts that an agent may assume a role.

use std::collections::BTreeSet;

use crate::kernel::entities::{AgentId, ResourceInstanceId, RoleId, StatementId};

use crate::kernel::value_objects::{ActionValue, Assignment, Date, Observation, Occurrence};

define_id!(CommitmentId);
define_entity! {
    pub struct Commitment(CommitmentId) via CommitmentInput {
        assignment: Assignment,
        statement: StatementId,
        resource: ResourceInstanceId,
        due_date: Date,
        supersedes: Option<CommitmentId>,
        action_value: ActionValue,
        recorded_at: Date,
        dependencies: BTreeSet<CommitmentId>,
    }
}

define_id!(EventId);
define_entity! {
    pub struct Event(EventId) via EventInput {
        commitment_id: CommitmentId,
        observation: Observation,
        previous_event: Option<EventId>,
        occurrence: Occurrence,
    }
}

define_id!(EligibilityAssignmentId);
define_entity! {
    pub struct EligibilityAssignment(EligibilityAssignmentId) via EligibilityAssignmentInput {
        agent: AgentId,
        role: RoleId,
    }
}
