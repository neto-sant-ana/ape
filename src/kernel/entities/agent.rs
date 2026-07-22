//! Agents and the roles they can take.
//!
//! - `Role` — a named capability an agent can be assigned to.
//!
//! - `Agent` — a company or individual (`kind`).

use crate::kernel::value_objects::{AgentKind, Identifier};

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
        kind: AgentKind,
    }
}
