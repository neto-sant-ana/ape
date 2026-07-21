//! Agents and the roles they can take.
//!
//! - `Role` — a named capability an agent can be assigned to.
//! 
//! - `Agent` — a company or individual (`kind`) with the set of roles it is
//!   eligible to assume (`eligibility`).
//!
//! An agent may take different roles in different statements.

use crate::kernel::value_objects::{AgentKind, Eligibility};

define_id!(RoleId);
define_entity! {
    pub struct Role(RoleId){}
}

define_id!(AgentId);
define_entity! {
    pub struct Agent(AgentId) {
        kind: AgentKind,

        #[serde(alias = "no_hash")]
        eligibility: Eligibility,
    }
}
