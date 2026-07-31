//! Shared test harness for the Axiom, and the test modules built on it.
//!
//! `Store` is an in-memory [`Knowledge`] the Axiom reads through; alongside it live
//! the `add_*` seeders and the `discrete_graph` fixture, reused across the test
//! modules — one per concern the Axiom validates.

mod commitment;
mod eligibility;
mod event;
mod structure;

use std::collections::{BTreeMap, BTreeSet};

use super::{Axiom, AxiomError, Knowledge};

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
struct Store {
    roles: BTreeMap<RoleId, Role>,
    agents: BTreeMap<AgentId, Agent>,
    resources: BTreeMap<ResourceId, Resource>,
    instances: BTreeMap<ResourceInstanceId, ResourceInstance>,
    actions: BTreeMap<ActionId, Action>,
    statements: BTreeMap<StatementId, Statement>,
    commitments: BTreeMap<CommitmentId, Commitment>,
    events: BTreeMap<EventId, Event>,
    eligibility: BTreeMap<AgentId, BTreeMap<EligibilityAssignmentId, EligibilityAssignment>>,
}
impl Knowledge for Store {
    fn role(&self, id: RoleId) -> Option<Role> {
        self.roles.get(&id).cloned()
    }
    fn agent(&self, id: AgentId) -> Option<Agent> {
        self.agents.get(&id).cloned()
    }
    fn resource(&self, id: ResourceId) -> Option<Resource> {
        self.resources.get(&id).cloned()
    }
    fn resource_instance(&self, id: ResourceInstanceId) -> Option<ResourceInstance> {
        self.instances.get(&id).cloned()
    }
    fn action(&self, id: ActionId) -> Option<Action> {
        self.actions.get(&id).cloned()
    }
    fn statement(&self, id: StatementId) -> Option<Statement> {
        self.statements.get(&id).cloned()
    }
    fn commitment(&self, id: CommitmentId) -> Option<Commitment> {
        self.commitments.get(&id).cloned()
    }
    fn event(&self, id: EventId) -> Option<Event> {
        self.events.get(&id).cloned()
    }
    fn eligibilities_of(&self, agent: AgentId) -> Vec<EligibilityAssignment> {
        self.eligibility
            .get(&agent)
            .into_iter()
            .flat_map(|by_id| by_id.values().cloned())
            .collect()
    }
}
impl Store {
    fn add_role(&mut self, r: Role) -> RoleId {
        let id = r.id();
        self.roles.insert(id, r);
        id
    }
    fn add_agent(&mut self, a: Agent) -> AgentId {
        let id = a.id();
        self.agents.insert(id, a);
        id
    }
    fn add_resource(&mut self, r: Resource) -> ResourceId {
        let id = r.id();
        self.resources.insert(id, r);
        id
    }
    fn add_instance(&mut self, i: ResourceInstance) -> ResourceInstanceId {
        let id = i.id();
        self.instances.insert(id, i);
        id
    }
    fn add_action(&mut self, a: Action) -> ActionId {
        let id = a.id();
        self.actions.insert(id, a);
        id
    }
    fn add_statement(&mut self, s: Statement) -> StatementId {
        let id = s.id();
        self.statements.insert(id, s);
        id
    }
    fn add_commitment(&mut self, c: Commitment) -> CommitmentId {
        let id = c.id();
        self.commitments.insert(id, c);
        id
    }
    fn add_eligibility(&mut self, ea: EligibilityAssignment) {
        self.eligibility
            .entry(*ea.agent())
            .or_default()
            .insert(ea.id(), ea);
    }
}

fn ident(value: &str) -> Identifier {
    Identifier::new(value).unwrap()
}
fn obs(name: &str) -> Observation {
    Observation::new(name).unwrap()
}
fn date(y: i32, m: u8, dd: u8) -> Date {
    Date::from_ymd(y, m, dd).unwrap()
}

struct Fixture {
    store: Store,
    actor_role: RoleId,
    accountable: AgentId,
    executor: AgentId,
    beneficiary: AgentId,
    instance: ResourceInstanceId,
    statement: StatementId,
}
fn discrete_graph() -> Fixture {
    let mut store = Store::default();

    let actor_role = store.add_role(Role::create(RoleInput { label: ident("actor") }).unwrap());
    let recipient_role =
        store.add_role(Role::create(RoleInput { label: ident("recipient") }).unwrap());

    let accountable = store.add_agent(
        Agent::create(AgentInput {
            label: ident("accountable"),
            kind: AgentKind::Company,
        })
        .unwrap(),
    );
    let executor = store.add_agent(
        Agent::create(AgentInput {
            label: ident("executor"),
            kind: AgentKind::Individual,
        })
        .unwrap(),
    );
    let beneficiary = store.add_agent(
        Agent::create(AgentInput {
            label: ident("beneficiary"),
            kind: AgentKind::Company,
        })
        .unwrap(),
    );

    store.add_eligibility(
        EligibilityAssignment::create(EligibilityAssignmentInput {
            agent: executor,
            roles: BTreeSet::from([actor_role]),
            effective_from: date(2025, 1, 1),
        })
        .unwrap(),
    );
    store.add_eligibility(
        EligibilityAssignment::create(EligibilityAssignmentInput {
            agent: beneficiary,
            roles: BTreeSet::from([recipient_role]),
            effective_from: date(2025, 1, 1),
        })
        .unwrap(),
    );

    let resource = store.add_resource(
        Resource::create(ResourceInput {
            label: ident("resource"),
            kind: ResourceKind::Discrete,
        })
        .unwrap(),
    );
    let instance = store.add_instance(
        ResourceInstance::create(ResourceInstanceInput {
            label: ident("instance"),
            resource,
        })
        .unwrap(),
    );
    let action = store.add_action(
        Action::create(ActionInput {
            verb: ident("sign"),
            kind: ActionKind::Discrete,
            resource,
        })
        .unwrap(),
    );

    let statement = store.add_statement(
        Statement::create(StatementInput {
            participants: Participants::new([actor_role], [recipient_role]).unwrap(),
            action,
            settlement: Settlement::new([obs("Signed")], [obs("Cancelled")]).unwrap(),
        })
        .unwrap(),
    );

    Fixture {
        store,
        actor_role,
        accountable,
        executor,
        beneficiary,
        instance,
        statement,
    }
}
fn commitment_input(f: &Fixture) -> CommitmentInput {
    CommitmentInput {
        assignment: Assignment::new(f.accountable, [f.executor], [f.beneficiary]).unwrap(),
        statement: f.statement,
        resource: f.instance,
        term: Term::new(date(2026, 1, 1), date(2026, 12, 31)).unwrap(),
        action_value: ActionValue::none(),
        dependencies: BTreeSet::new(),
    }
}
fn commit(f: &Fixture) -> Result<Commitment, AxiomError> {
    Axiom::new(&f.store).emit_commitment(commitment_input(f))
}
