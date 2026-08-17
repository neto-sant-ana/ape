//! Agents, the roles they can take, and when they may take them.
//!
//! - `Role` — a named capability an agent can be assigned to.
//!
//! - `Agent` — a party, distinguished by its `label`.
//!
//! - `EligibilityAssignment` — the full set of `roles` an agent may assume, effective
//!   from `effective_from`. It carries a time of its own because eligibility changes
//!   while the agent stays the same: holding it on the `Agent` would either break the
//!   immutability of an emitted entity or force a new `Agent` per change, for what is
//!   one agent throughout. The roles in force at a given moment are those of the latest
//!   assignment whose `effective_from` does not exceed it, and an empty set withdraws
//!   the agent from every role.
//!
//! Its date may lie in the future, so unlike a `Commitment` or an `Event` it is a
//! forward declaration rather than a claim about a factual instant — which is why the
//! Canon imposes no lower bound on when it may be recorded.

use std::collections::BTreeSet;

use crate::kernel::value_objects::{Date, Identifier};

define_id!(RoleId);
define_entity! {
    pub struct Role(RoleId) via RoleInput {
        label: Identifier,
    }
}

define_id!(AgentId);
define_entity! {
    pub struct Agent(AgentId) via AgentInput {
        label: Identifier,
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
