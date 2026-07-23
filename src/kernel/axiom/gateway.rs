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
    Action, ActionId, Agent, AgentId, Commitment, CommitmentId, EligibilityAssignment, Event,
    EventId, Resource, ResourceId, ResourceInstance, ResourceInstanceId, Role, RoleId, Statement,
    StatementId,
};

use crate::kernel::value_objects::Date;

pub trait Knowledge {
    fn role(&self, id: RoleId) -> Option<&Role>;
    fn agent(&self, id: AgentId) -> Option<&Agent>;
    fn resource(&self, id: ResourceId) -> Option<&Resource>;
    fn resource_instance(&self, id: ResourceInstanceId) -> Option<&ResourceInstance>;
    fn action(&self, id: ActionId) -> Option<&Action>;
    fn statement(&self, id: StatementId) -> Option<&Statement>;
    fn commitment(&self, id: CommitmentId) -> Option<&Commitment>;
    fn event(&self, id: EventId) -> Option<&Event>;

    /// Every eligibility assignment recorded for `agent`, in any order.
    fn eligibilities_of(&self, agent: AgentId) -> impl Iterator<Item = &EligibilityAssignment>;

    /// The assignment in effect for `agent` as of `at`: the latest one whose
    /// `effective_from` does not exceed `at`. The boundary rule lives here, once,
    /// so no adapter can diverge on it. Uniqueness of `(agent, effective_from)` is
    /// the canonical history's responsibility, not the Axiom's.
    fn eligibility_at(&self, agent: AgentId, at: &Date) -> Option<&EligibilityAssignment> {
        self.eligibilities_of(agent)
            .filter(|e| e.effective_from().up_to(at))
            .max_by_key(|e| *e.effective_from())
    }
}

pub struct Axiom<'k, K: Knowledge> {
    pub(super) knowledge: &'k K,
}
impl<'k, K: Knowledge> Axiom<'k, K> {
    pub fn new(knowledge: &'k K) -> Self {
        Self { knowledge }
    }
}
