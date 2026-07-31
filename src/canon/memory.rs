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

use super::{AppendOutcome, CanonError, Canonical, CanonicalHistory, CanonicalKnowledge};

use crate::kernel::axiom::Knowledge;

use crate::kernel::entities::{
    Action, ActionId, Agent, AgentId, Commitment, CommitmentId, EligibilityAssignment,
    EligibilityAssignmentId, Event, EventId, Resource, ResourceId, ResourceInstance,
    ResourceInstanceId, Role, RoleId, Statement, StatementId,
};

use crate::kernel::value_objects::Date;

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
    recorded_through: Option<Date>,
}
impl Shelf {
    /// Checking and advancing the watermark are separate on purpose: an admission may
    /// still be refused after this passes (a stale head), and a refusal must leave no
    /// trace — a watermark moved by a write that never happened is a trace.
    fn check_recording(&self, recorded_at: Date) -> Result<(), CanonError> {
        match self.recorded_through {
            Some(through) if !through.up_to(&recorded_at) => Err(CanonError::RecordedOutOfOrder {
                recorded_at,
                recorded_through: through,
            }),
            _ => Ok(()),
        }
    }

    fn advance_recording(&mut self, recorded_at: Date) {
        self.recorded_through = Some(recorded_at);
    }
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
impl CanonicalKnowledge for InMemoryHistory {
    fn canonical_commitment(&self, id: CommitmentId) -> Option<Canonical<Commitment>> {
        self.shelf.lock().unwrap().commitments.get(&id).cloned()
    }
    fn canonical_event(&self, id: EventId) -> Option<Canonical<Event>> {
        self.shelf.lock().unwrap().events.get(&id).cloned()
    }

    /// Walked backwards from the current head, which is sound because recording never decreases
    /// along the chain: the first Event recorded no later than `at` is the latest one. A durable
    /// adapter with an index on `recorded_at` may answer without walking.
    fn head_as_of(&self, at: &Date) -> Option<EventId> {
        let shelf = self.shelf.lock().unwrap();
        let mut cursor = shelf.head;

        while let Some(id) = cursor {
            let record = shelf.events.get(&id)?;

            if record.recorded_at().up_to(at) {
                return Some(id);
            }

            cursor = *record.assertion().previous_event();
        }

        None
    }
}
impl CanonicalHistory for InMemoryHistory {
    fn head(&self) -> Option<EventId> {
        self.shelf.lock().unwrap().head
    }

    fn recorded_through(&self) -> Option<Date> {
        self.shelf.lock().unwrap().recorded_through
    }

    fn event_of(&self, commitment: CommitmentId) -> Option<Event> {
        let shelf = self.shelf.lock().unwrap();
        shelf
            .events_by_commitment
            .get(&commitment)
            .and_then(|id| shelf.events.get(id))
            .map(|e| e.assertion().clone())
    }

    fn put_role(&mut self, role: Canonical<Role>) -> Result<AppendOutcome, CanonError> {
        let id = role.assertion().id();
        self.put(|shelf| &mut shelf.roles, id, role)
    }
    fn put_agent(&mut self, agent: Canonical<Agent>) -> Result<AppendOutcome, CanonError> {
        let id = agent.assertion().id();
        self.put(|shelf| &mut shelf.agents, id, agent)
    }
    fn put_resource(&mut self, resource: Canonical<Resource>) -> Result<AppendOutcome, CanonError> {
        let id = resource.assertion().id();
        self.put(|shelf| &mut shelf.resources, id, resource)
    }
    fn put_resource_instance(
        &mut self,
        instance: Canonical<ResourceInstance>,
    ) -> Result<AppendOutcome, CanonError> {
        let id = instance.assertion().id();
        self.put(|shelf| &mut shelf.instances, id, instance)
    }
    fn put_action(&mut self, action: Canonical<Action>) -> Result<AppendOutcome, CanonError> {
        let id = action.assertion().id();
        self.put(|shelf| &mut shelf.actions, id, action)
    }
    fn put_statement(
        &mut self,
        statement: Canonical<Statement>,
    ) -> Result<AppendOutcome, CanonError> {
        let id = statement.assertion().id();
        self.put(|shelf| &mut shelf.statements, id, statement)
    }
    fn put_commitment(
        &mut self,
        commitment: Canonical<Commitment>,
    ) -> Result<AppendOutcome, CanonError> {
        let id = commitment.assertion().id();
        self.put(|shelf| &mut shelf.commitments, id, commitment)
    }
    fn put_eligibility(
        &mut self,
        eligibility: Canonical<EligibilityAssignment>,
    ) -> Result<AppendOutcome, CanonError> {
        let id = eligibility.assertion().id();
        self.put(|shelf| &mut shelf.eligibility, id, eligibility)
    }

    fn append_event(&mut self, event: Canonical<Event>) -> Result<AppendOutcome, CanonError> {
        let mut shelf = self.shelf.lock().unwrap();

        let id = event.assertion().id();
        if shelf.events.contains_key(&id) {
            return Ok(AppendOutcome::AlreadyPresent);
        }

        shelf.check_recording(*event.recorded_at())?;

        let expected = *event.assertion().previous_event();
        if shelf.head != expected {
            return Err(CanonError::UnexpectedHead {
                expected,
                found: shelf.head,
            });
        }

        let commitment = *event.assertion().commitment_id();
        let recorded_at = *event.recorded_at();
        shelf.events.insert(id, event);
        shelf.events_by_commitment.insert(commitment, id);
        shelf.head = Some(id);
        shelf.advance_recording(recorded_at);

        Ok(AppendOutcome::Admitted)
    }
}
impl InMemoryHistory {
    /// Put-if-absent under the lock, guarded by the recording watermark.
    fn put<Id: Ord, T>(
        &mut self,
        shelf: impl Fn(&mut Shelf) -> &mut BTreeMap<Id, Canonical<T>>,
        id: Id,
        record: Canonical<T>,
    ) -> Result<AppendOutcome, CanonError> {
        let mut guard = self.shelf.lock().unwrap();

        if shelf(&mut guard).contains_key(&id) {
            return Ok(AppendOutcome::AlreadyPresent);
        }

        let recorded_at = *record.recorded_at();
        guard.check_recording(recorded_at)?;

        shelf(&mut guard).insert(id, record);
        guard.advance_recording(recorded_at);

        Ok(AppendOutcome::Admitted)
    }
}
