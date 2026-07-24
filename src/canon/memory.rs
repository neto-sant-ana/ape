//! `InMemoryHistory` — a thread-safe, in-memory reference [`CanonicalHistory`].
//!
//! Available under `#[cfg(test)]` and behind the `reference` feature. It is the
//! reference implementation of the canonical-history port: a `Clone` handle over an
//! `Arc<Mutex<_>>` whose clones share one store.
//!
//! It is **not durable**: the history lives only as long as the process. It suits
//! tests, prototyping, and single-process use — never a deployment that must survive
//! a restart. A durable deployment supplies its own adapter (file, database, ...);

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use super::{AppendOutcome, CanonError, Canonical, CanonicalHistory};

use crate::kernel::axiom::Knowledge;

use crate::kernel::entities::{
    Action, ActionId, Agent, AgentId, Commitment, CommitmentId, EligibilityAssignment,
    EligibilityAssignmentId, Event, EventId, Resource, ResourceId, ResourceInstance,
    ResourceInstanceId, Role, RoleId, Statement, StatementId,
};

#[derive(Default)]
struct Shelf {
    roles: BTreeMap<RoleId, Canonical<Role>>,
    agents: BTreeMap<AgentId, Canonical<Agent>>,
    resources: BTreeMap<ResourceId, Canonical<Resource>>,
    instances: BTreeMap<ResourceInstanceId, Canonical<ResourceInstance>>,
    actions: BTreeMap<ActionId, Canonical<Action>>,
    statements: BTreeMap<StatementId, Canonical<Statement>>,
    commitments: BTreeMap<CommitmentId, Canonical<Commitment>>,
    eligibility: BTreeMap<EligibilityAssignmentId, Canonical<EligibilityAssignment>>,
    events: BTreeMap<EventId, Canonical<Event>>,
    events_by_commitment: BTreeMap<CommitmentId, EventId>,
    head: Option<EventId>,
}

#[derive(Clone, Default)]
pub struct InMemoryHistory {
    shelf: Arc<Mutex<Shelf>>,
}
impl Knowledge for InMemoryHistory {
    fn role(&self, id: RoleId) -> Option<Role> {
        self.shelf
            .lock()
            .unwrap()
            .roles
            .get(&id)
            .map(|r| r.assertion().clone())
    }
    fn agent(&self, id: AgentId) -> Option<Agent> {
        self.shelf
            .lock()
            .unwrap()
            .agents
            .get(&id)
            .map(|a| a.assertion().clone())
    }
    fn resource(&self, id: ResourceId) -> Option<Resource> {
        self.shelf
            .lock()
            .unwrap()
            .resources
            .get(&id)
            .map(|r| r.assertion().clone())
    }
    fn resource_instance(&self, id: ResourceInstanceId) -> Option<ResourceInstance> {
        self.shelf
            .lock()
            .unwrap()
            .instances
            .get(&id)
            .map(|i| i.assertion().clone())
    }
    fn action(&self, id: ActionId) -> Option<Action> {
        self.shelf
            .lock()
            .unwrap()
            .actions
            .get(&id)
            .map(|a| a.assertion().clone())
    }
    fn statement(&self, id: StatementId) -> Option<Statement> {
        self.shelf
            .lock()
            .unwrap()
            .statements
            .get(&id)
            .map(|s| s.assertion().clone())
    }
    fn commitment(&self, id: CommitmentId) -> Option<Commitment> {
        self.shelf
            .lock()
            .unwrap()
            .commitments
            .get(&id)
            .map(|c| c.assertion().clone())
    }
    fn event(&self, id: EventId) -> Option<Event> {
        self.shelf
            .lock()
            .unwrap()
            .events
            .get(&id)
            .map(|e| e.assertion().clone())
    }
    fn eligibilities_of(&self, agent: AgentId) -> Vec<EligibilityAssignment> {
        self.shelf
            .lock()
            .unwrap()
            .eligibility
            .values()
            .map(|e| e.assertion())
            .filter(|e| *e.agent() == agent)
            .cloned()
            .collect()
    }
}
impl CanonicalHistory for InMemoryHistory {
    fn head(&self) -> Option<EventId> {
        self.shelf.lock().unwrap().head
    }

    fn event_of(&self, commitment: CommitmentId) -> Option<Event> {
        let shelf = self.shelf.lock().unwrap();
        shelf
            .events_by_commitment
            .get(&commitment)
            .and_then(|id| shelf.events.get(id))
            .map(|e| e.assertion().clone())
    }

    fn canonical_commitment(&self, id: CommitmentId) -> Option<Canonical<Commitment>> {
        self.shelf.lock().unwrap().commitments.get(&id).cloned()
    }
    fn canonical_event(&self, id: EventId) -> Option<Canonical<Event>> {
        self.shelf.lock().unwrap().events.get(&id).cloned()
    }

    fn put_role(&mut self, role: Canonical<Role>) -> AppendOutcome {
        let id = role.assertion().id();
        put_if_absent(&mut self.shelf.lock().unwrap().roles, id, role)
    }
    fn put_agent(&mut self, agent: Canonical<Agent>) -> AppendOutcome {
        let id = agent.assertion().id();
        put_if_absent(&mut self.shelf.lock().unwrap().agents, id, agent)
    }
    fn put_resource(&mut self, resource: Canonical<Resource>) -> AppendOutcome {
        let id = resource.assertion().id();
        put_if_absent(&mut self.shelf.lock().unwrap().resources, id, resource)
    }
    fn put_resource_instance(&mut self, instance: Canonical<ResourceInstance>) -> AppendOutcome {
        let id = instance.assertion().id();
        put_if_absent(&mut self.shelf.lock().unwrap().instances, id, instance)
    }
    fn put_action(&mut self, action: Canonical<Action>) -> AppendOutcome {
        let id = action.assertion().id();
        put_if_absent(&mut self.shelf.lock().unwrap().actions, id, action)
    }
    fn put_statement(&mut self, statement: Canonical<Statement>) -> AppendOutcome {
        let id = statement.assertion().id();
        put_if_absent(&mut self.shelf.lock().unwrap().statements, id, statement)
    }
    fn put_commitment(&mut self, commitment: Canonical<Commitment>) -> AppendOutcome {
        let id = commitment.assertion().id();
        put_if_absent(&mut self.shelf.lock().unwrap().commitments, id, commitment)
    }
    fn put_eligibility(&mut self, eligibility: Canonical<EligibilityAssignment>) -> AppendOutcome {
        let id = eligibility.assertion().id();
        put_if_absent(&mut self.shelf.lock().unwrap().eligibility, id, eligibility)
    }

    fn append_event(&mut self, event: Canonical<Event>) -> Result<AppendOutcome, CanonError> {
        let mut shelf = self.shelf.lock().unwrap();

        let id = event.assertion().id();
        if shelf.events.contains_key(&id) {
            return Ok(AppendOutcome::AlreadyPresent);
        }

        let expected = *event.assertion().previous_event();
        if shelf.head != expected {
            return Err(CanonError::UnexpectedHead {
                expected,
                found: shelf.head,
            });
        }

        let commitment = *event.assertion().commitment_id();
        shelf.events.insert(id, event);
        shelf.events_by_commitment.insert(commitment, id);
        shelf.head = Some(id);

        Ok(AppendOutcome::Admitted)
    }
}

fn put_if_absent<K: Ord, V>(map: &mut BTreeMap<K, V>, key: K, value: V) -> AppendOutcome {
    use std::collections::btree_map::Entry;

    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            AppendOutcome::Admitted
        }
        Entry::Occupied(_) => AppendOutcome::AlreadyPresent,
    }
}
