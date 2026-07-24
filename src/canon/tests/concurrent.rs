//! A thread-safe reference `CanonicalHistory`, sibling to [`MemoryHistory`].
//!
//! Where `MemoryHistory` is the single-thread fake behind the semantic tests, this
//! is its concurrent counterpart: a `Clone` handle over an `Arc<Mutex<_>>` whose
//! clones share one store. It exists to drive the contention the sequential fake
//! cannot — the conformance thread-safety suite and the Canon's settle-once race.
//!
//! Each `CanonicalHistory` write is `&mut self`, honored by every thread owning its
//! own clone; the real mutation serializes on the shared lock. `append_event` holds
//! that lock across its whole compare-and-append, so the step is indivisible — yet
//! the lock is released *between* calls, so the head can still move between a
//! `Canon`'s `head()` read and its `append_event`. That gap is exactly what the race
//! exercises.

use std::collections::BTreeMap;
use std::sync::{Arc, Barrier, Mutex};

use super::*;

use crate::canon::EventSubmission;

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
struct SharedHistory {
    shelf: Arc<Mutex<Shelf>>,
}
impl Knowledge for SharedHistory {
    fn role(&self, id: RoleId) -> Option<Role> {
        self.shelf.lock().unwrap().roles.get(&id).map(|r| r.assertion().clone())
    }
    fn agent(&self, id: AgentId) -> Option<Agent> {
        self.shelf.lock().unwrap().agents.get(&id).map(|a| a.assertion().clone())
    }
    fn resource(&self, id: ResourceId) -> Option<Resource> {
        self.shelf.lock().unwrap().resources.get(&id).map(|r| r.assertion().clone())
    }
    fn resource_instance(&self, id: ResourceInstanceId) -> Option<ResourceInstance> {
        self.shelf.lock().unwrap().instances.get(&id).map(|i| i.assertion().clone())
    }
    fn action(&self, id: ActionId) -> Option<Action> {
        self.shelf.lock().unwrap().actions.get(&id).map(|a| a.assertion().clone())
    }
    fn statement(&self, id: StatementId) -> Option<Statement> {
        self.shelf.lock().unwrap().statements.get(&id).map(|s| s.assertion().clone())
    }
    fn commitment(&self, id: CommitmentId) -> Option<Commitment> {
        self.shelf.lock().unwrap().commitments.get(&id).map(|c| c.assertion().clone())
    }
    fn event(&self, id: EventId) -> Option<Event> {
        self.shelf.lock().unwrap().events.get(&id).map(|e| e.assertion().clone())
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
impl CanonicalHistory for SharedHistory {
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
            return Err(CanonError::UnexpectedHead { expected, found: shelf.head });
        }

        let commitment = *event.assertion().commitment_id();
        shelf.events.insert(id, event);
        shelf.events_by_commitment.insert(commitment, id);
        shelf.head = Some(id);

        Ok(AppendOutcome::Admitted)
    }
}

#[test]
fn the_shared_history_conforms() {
    crate::canon::conformance::verify(SharedHistory::default);
    crate::canon::conformance::verify_thread_safe(SharedHistory::default());
}

/// Two threads race to settle the same commitment with different, mutually
/// exclusive observations. Settle-once must hold every time: exactly one event
/// settles the commitment, the conflicting one is refused — even when the loser's
/// append lost the head race and had to recover by re-admitting (which re-runs the
/// settle-once check against the now-settled commitment). The barrier and the repeat
/// make the head race, and thus the re-admission path, actually occur.
#[test]
fn concurrent_settlement_of_one_commitment_settles_it_once() {
    for _ in 0..64 {
        let shared = SharedHistory::default();
        let commitment = seed_commitment(&mut Canon::new(shared.clone()));

        let barrier = Arc::new(Barrier::new(2));
        let observations = ["Signed", "Cancelled"];

        let racers: Vec<_> = observations
            .into_iter()
            .map(|observation| {
                let mut canon = Canon::new(shared.clone());
                let barrier = Arc::clone(&barrier);
                std::thread::spawn(move || {
                    let submission = || EventSubmission {
                        commitment_id: commitment,
                        observation: Observation::new(observation).unwrap(),
                        occurred_at: date(2026, 6, 1),
                    };

                    barrier.wait();

                    let mut outcome = canon.admit_event(submission(), date(2026, 7, 1));
                    let mut retries = 0;
                    while matches!(outcome, Err(CanonError::UnexpectedHead { .. })) && retries < 8 {
                        outcome = canon.admit_event(submission(), date(2026, 7, 1));
                        retries += 1;
                    }
                    outcome
                })
            })
            .collect();

        let outcomes: Vec<_> = racers.into_iter().map(|r| r.join().unwrap()).collect();

        let settled = outcomes.iter().filter(|o| o.is_ok()).count();
        let refused = outcomes
            .iter()
            .filter(|o| matches!(o, Err(CanonError::CommitmentAlreadySettled(_))))
            .count();
        assert_eq!(settled, 1, "exactly one event settles the commitment");
        assert_eq!(refused, 1, "the conflicting settlement is refused, even after re-admission");

        let winner = outcomes.iter().find_map(|o| o.as_ref().ok().copied()).unwrap();
        assert_eq!(shared.head(), Some(winner), "the head is the settling event");
        assert_eq!(
            shared.event_of(commitment).map(|e| e.id()),
            Some(winner),
            "the commitment is settled by exactly that event",
        );
    }
}
