//! Shared test harness for the Canon, and the test modules built on it.
//!
//! [`MemoryHistory`] is the reference in-memory canonical history: the two faces
//! of one repository. `Knowledge` answers the Axiom's lookups (returning the
//! assertion inside each record); the [`CanonicalHistory`] primitives are a dumb
//! put-if-absent and a dumb compare-and-swap. Alongside it live the standalone
//! factories and the seeded [`graph`], reused across the test modules.

mod admission;
mod envelope;

use std::collections::{BTreeMap, BTreeSet};

use super::{AppendOutcome, Canon, CanonError, Canonical, CanonicalHistory};

use crate::kernel::axiom::Knowledge;

use crate::kernel::entities::{
    Action, ActionId, ActionInput, Agent, AgentId, AgentInput, Commitment, CommitmentId,
    CommitmentInput, EligibilityAssignment, EligibilityAssignmentId, EligibilityAssignmentInput,
    Event, EventId, EventInput, Resource, ResourceId, ResourceInput, ResourceInstance,
    ResourceInstanceId, ResourceInstanceInput, Role, RoleId, RoleInput, Statement, StatementId,
    StatementInput,
};

use crate::kernel::value_objects::{
    ActionKind, ActionValue, AgentKind, Assignment, Date, Identifier, Observation, Participants,
    ResourceKind, Settlement, Term,
};

#[derive(Default)]
struct MemoryHistory {
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
impl Knowledge for MemoryHistory {
    fn role(&self, id: RoleId) -> Option<&Role> {
        self.roles.get(&id).map(|r| r.assertion())
    }
    fn agent(&self, id: AgentId) -> Option<&Agent> {
        self.agents.get(&id).map(|a| a.assertion())
    }
    fn resource(&self, id: ResourceId) -> Option<&Resource> {
        self.resources.get(&id).map(|r| r.assertion())
    }
    fn resource_instance(&self, id: ResourceInstanceId) -> Option<&ResourceInstance> {
        self.instances.get(&id).map(|i| i.assertion())
    }
    fn action(&self, id: ActionId) -> Option<&Action> {
        self.actions.get(&id).map(|a| a.assertion())
    }
    fn statement(&self, id: StatementId) -> Option<&Statement> {
        self.statements.get(&id).map(|s| s.assertion())
    }
    fn commitment(&self, id: CommitmentId) -> Option<&Commitment> {
        self.commitments.get(&id).map(|c| c.assertion())
    }
    fn event(&self, id: EventId) -> Option<&Event> {
        self.events.get(&id).map(|e| e.assertion())
    }
    fn eligibilities_of(&self, agent: AgentId) -> impl Iterator<Item = &EligibilityAssignment> {
        self.eligibility
            .values()
            .map(|e| e.assertion())
            .filter(move |e| *e.agent() == agent)
    }
}
impl CanonicalHistory for MemoryHistory {
    fn head(&self) -> Option<EventId> {
        self.head
    }

    fn event_of(&self, commitment: CommitmentId) -> Option<&Event> {
        self.events_by_commitment
            .get(&commitment)
            .and_then(|id| self.events.get(id))
            .map(|e| e.assertion())
    }

    fn canonical_commitment(&self, id: CommitmentId) -> Option<&Canonical<Commitment>> {
        self.commitments.get(&id)
    }
    fn canonical_event(&self, id: EventId) -> Option<&Canonical<Event>> {
        self.events.get(&id)
    }

    fn put_role(&mut self, role: Canonical<Role>) -> AppendOutcome {
        put_if_absent(&mut self.roles, role.assertion().id(), role)
    }
    fn put_agent(&mut self, agent: Canonical<Agent>) -> AppendOutcome {
        put_if_absent(&mut self.agents, agent.assertion().id(), agent)
    }
    fn put_resource(&mut self, resource: Canonical<Resource>) -> AppendOutcome {
        put_if_absent(&mut self.resources, resource.assertion().id(), resource)
    }
    fn put_resource_instance(&mut self, instance: Canonical<ResourceInstance>) -> AppendOutcome {
        put_if_absent(&mut self.instances, instance.assertion().id(), instance)
    }
    fn put_action(&mut self, action: Canonical<Action>) -> AppendOutcome {
        put_if_absent(&mut self.actions, action.assertion().id(), action)
    }
    fn put_statement(&mut self, statement: Canonical<Statement>) -> AppendOutcome {
        put_if_absent(&mut self.statements, statement.assertion().id(), statement)
    }
    fn put_commitment(&mut self, commitment: Canonical<Commitment>) -> AppendOutcome {
        put_if_absent(
            &mut self.commitments,
            commitment.assertion().id(),
            commitment,
        )
    }
    fn put_eligibility(&mut self, eligibility: Canonical<EligibilityAssignment>) -> AppendOutcome {
        put_if_absent(
            &mut self.eligibility,
            eligibility.assertion().id(),
            eligibility,
        )
    }
    fn append_event(&mut self, event: Canonical<Event>) -> Result<AppendOutcome, CanonError> {
        let id = event.assertion().id();
        if self.events.contains_key(&id) {
            return Ok(AppendOutcome::AlreadyPresent);
        }

        let expected = *event.assertion().previous_event();
        if self.head != expected {
            return Err(CanonError::UnexpectedHead {
                expected,
                found: self.head,
            });
        }

        // Persist, index by commitment, and advance the head as one indivisible
        // step: a refused append (above) has already returned, leaving no trace.
        let commitment = *event.assertion().commitment_id();
        self.events.insert(id, event);
        self.events_by_commitment.insert(commitment, id);
        self.head = Some(id);

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

fn date(y: i32, m: u8, d: u8) -> Date {
    Date::from_ymd(y, m, d).unwrap()
}
fn ident(value: &str) -> Identifier {
    Identifier::new(value).unwrap()
}
fn obs(name: &str) -> Observation {
    Observation::new(name).unwrap()
}

// ---------------------------------------------------------------------------
// Standalone factories for the primitive and envelope tests
// ---------------------------------------------------------------------------
fn commitment(tag: u8) -> Commitment {
    Commitment::create(CommitmentInput {
        assignment: Assignment::new(
            AgentId::from([tag; 32]),
            [AgentId::from([tag; 32])],
            [AgentId::from([tag; 32])],
        )
        .unwrap(),
        statement: StatementId::from([tag; 32]),
        resource: ResourceInstanceId::from([tag; 32]),
        term: Term::new(date(2026, 1, 1), date(2026, 12, 31)).unwrap(),
        supersedes: None,
        action_value: ActionValue::none(),
        dependencies: BTreeSet::new(),
    })
    .unwrap()
}

fn eligibility(tag: u8) -> EligibilityAssignment {
    EligibilityAssignment::create(EligibilityAssignmentInput {
        agent: AgentId::from([tag; 32]),
        roles: BTreeSet::from([RoleId::from([tag; 32])]),
        effective_from: date(2025, 1, 1),
    })
    .unwrap()
}

fn event(commitment: CommitmentId, previous: Option<EventId>, observation: &str) -> Event {
    Event::create(EventInput {
        commitment_id: commitment,
        observation: Observation::new(observation).unwrap(),
        previous_event: previous,
        occurred_at: date(2026, 6, 1),
    })
    .unwrap()
}

// ---------------------------------------------------------------------------
// A valid graph, seeded entirely through the Canon, for the orchestrator tests.
// ---------------------------------------------------------------------------

struct Graph {
    canon: Canon<MemoryHistory>,
    accountable: AgentId,
    executor: AgentId,
    beneficiary: AgentId,
    actor_role: RoleId,
    instance: ResourceInstanceId,
    statement: StatementId,
}
fn graph() -> Graph {
    let mut canon = Canon::new(MemoryHistory::default());
    let rec = date(2025, 1, 1);

    let actor_role = canon
        .admit_role(
            RoleInput {
                label: ident("actor"),
            },
            rec,
        )
        .unwrap();
    let recipient_role = canon
        .admit_role(
            RoleInput {
                label: ident("recipient"),
            },
            rec,
        )
        .unwrap();

    let accountable = canon
        .admit_agent(
            AgentInput {
                label: ident("accountable"),
                kind: AgentKind::Company,
            },
            rec,
        )
        .unwrap();
    let executor = canon
        .admit_agent(
            AgentInput {
                label: ident("executor"),
                kind: AgentKind::Individual,
            },
            rec,
        )
        .unwrap();
    let beneficiary = canon
        .admit_agent(
            AgentInput {
                label: ident("beneficiary"),
                kind: AgentKind::Company,
            },
            rec,
        )
        .unwrap();

    canon
        .admit_eligibility(
            EligibilityAssignmentInput {
                agent: executor,
                roles: BTreeSet::from([actor_role]),
                effective_from: date(2025, 1, 1),
            },
            rec,
        )
        .unwrap();
    canon
        .admit_eligibility(
            EligibilityAssignmentInput {
                agent: beneficiary,
                roles: BTreeSet::from([recipient_role]),
                effective_from: date(2025, 1, 1),
            },
            rec,
        )
        .unwrap();

    let resource = canon
        .admit_resource(
            ResourceInput {
                label: ident("resource"),
                kind: ResourceKind::Discrete,
            },
            rec,
        )
        .unwrap();
    let instance = canon
        .admit_resource_instance(
            ResourceInstanceInput {
                label: ident("instance"),
                resource,
            },
            rec,
        )
        .unwrap();
    let action = canon
        .admit_action(
            ActionInput {
                verb: ident("sign"),
                kind: ActionKind::Discrete,
                resource,
            },
            rec,
        )
        .unwrap();
    let statement = canon
        .admit_statement(
            StatementInput {
                participants: Participants::new([actor_role], [recipient_role]).unwrap(),
                action,
                settlement: Settlement::new([obs("Signed")], [obs("Cancelled")]).unwrap(),
            },
            rec,
        )
        .unwrap();

    Graph {
        canon,
        accountable,
        executor,
        beneficiary,
        actor_role,
        instance,
        statement,
    }
}
fn commitment_input(g: &Graph) -> CommitmentInput {
    CommitmentInput {
        assignment: Assignment::new(g.accountable, [g.executor], [g.beneficiary]).unwrap(),
        statement: g.statement,
        resource: g.instance,
        term: Term::new(date(2026, 1, 1), date(2026, 12, 31)).unwrap(),
        supersedes: None,
        action_value: ActionValue::none(),
        dependencies: BTreeSet::new(),
    }
}

#[test]
fn the_reference_history_conforms() {
    super::conformance::verify(MemoryHistory::default);
}
