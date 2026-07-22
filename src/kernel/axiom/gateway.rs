//! The Axiom — the single gateway through which new knowledge enters the kernel.
//!
//! Applications never instantiate kernel entities directly,
//! they request admission through the Axiom, passing the same
//! input struct `create` consumes. The Axiom resolves the referenced entities,
//! validates the cross-entity invariants that a single entity's constructor
//! cannot see, and only then emits the immutable entity — or rejects a
//! structurally inconsistent assertion.
//!
//! The Axiom does not persist. It reads existing knowledge through the read-only
//! [`Knowledge`] trait that higher layers implement.

use crate::kernel::entities::{
    Action, ActionId, Agent, AgentId, Commitment, CommitmentId, Event, EventId, Resource,
    ResourceId, ResourceInstance, ResourceInstanceId, Role, RoleId, Statement, StatementId,
};

pub trait Knowledge {
    fn role(&self, id: RoleId) -> Option<&Role>;
    fn agent(&self, id: AgentId) -> Option<&Agent>;
    fn resource(&self, id: ResourceId) -> Option<&Resource>;
    fn resource_instance(&self, id: ResourceInstanceId) -> Option<&ResourceInstance>;
    fn action(&self, id: ActionId) -> Option<&Action>;
    fn statement(&self, id: StatementId) -> Option<&Statement>;
    fn commitment(&self, id: CommitmentId) -> Option<&Commitment>;
    fn event(&self, id: EventId) -> Option<&Event>;
}

pub struct Axiom<'k, K: Knowledge> {
    pub(super) knowledge: &'k K,
}
impl<'k, K: Knowledge> Axiom<'k, K> {
    pub fn new(knowledge: &'k K) -> Self {
        Self { knowledge }
    }
}
