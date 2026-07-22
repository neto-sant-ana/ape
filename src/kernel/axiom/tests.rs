use std::collections::{BTreeMap, BTreeSet};

use super::{Axiom, AxiomError, Knowledge};

use crate::kernel::entities::{
    Action, ActionId, ActionInput, Agent, AgentId, AgentInput, Commitment, CommitmentId,
    CommitmentInput, EligibilityAssignment, EligibilityAssignmentInput, Event, EventId, EventInput,
    Resource, ResourceId, ResourceInput, ResourceInstance, ResourceInstanceId,
    ResourceInstanceInput, Role, RoleId, RoleInput, Statement, StatementId, StatementInput,
};

use crate::kernel::value_objects::{
    ActionKind, ActionValue, AgentKind, Assignment, Date, Effect, Identifier, Observation,
    Participants, ResourceKind, Settlement,
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
    eligibility: BTreeMap<(AgentId, RoleId), EligibilityAssignment>,
}
impl Knowledge for Store {
    fn role(&self, id: RoleId) -> Option<&Role> {
        self.roles.get(&id)
    }
    fn agent(&self, id: AgentId) -> Option<&Agent> {
        self.agents.get(&id)
    }
    fn resource(&self, id: ResourceId) -> Option<&Resource> {
        self.resources.get(&id)
    }
    fn resource_instance(&self, id: ResourceInstanceId) -> Option<&ResourceInstance> {
        self.instances.get(&id)
    }
    fn action(&self, id: ActionId) -> Option<&Action> {
        self.actions.get(&id)
    }
    fn statement(&self, id: StatementId) -> Option<&Statement> {
        self.statements.get(&id)
    }
    fn commitment(&self, id: CommitmentId) -> Option<&Commitment> {
        self.commitments.get(&id)
    }
    fn event(&self, id: EventId) -> Option<&Event> {
        self.events.get(&id)
    }
    fn eligibility(&self, agent: AgentId, role: RoleId) -> Option<&EligibilityAssignment> {
        self.eligibility.get(&(agent, role))
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
        self.eligibility.insert((*ea.agent(), *ea.role()), ea);
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
    recipient_role: RoleId,
    accountable: AgentId,
    executor: AgentId,
    beneficiary: AgentId,
    resource: ResourceId,
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
            role: actor_role,
        })
        .unwrap(),
    );
    store.add_eligibility(
        EligibilityAssignment::create(EligibilityAssignmentInput {
            agent: beneficiary,
            role: recipient_role,
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
        recipient_role,
        accountable,
        executor,
        beneficiary,
        resource,
        instance,
        statement,
    }
}
fn commitment_input(f: &Fixture) -> CommitmentInput {
    CommitmentInput {
        assignment: Assignment::new(f.accountable, [f.executor], [f.beneficiary]).unwrap(),
        statement: f.statement,
        resource: f.instance,
        due_date: date(2026, 12, 31),
        supersedes: None,
        action_value: ActionValue::none(),
        dependencies: BTreeSet::new(),
    }
}
fn commit(f: &Fixture) -> Result<Commitment, AxiomError> {
    Axiom::new(&f.store).admit_commitment(commitment_input(f))
}

#[test]
fn admits_a_consistent_discrete_commitment() {
    let f = discrete_graph();
    assert!(commit(&f).is_ok());
}

#[test]
fn rejects_action_on_missing_resource() {
    let store = Store::default();
    let axiom = Axiom::new(&store);

    assert!(matches!(
        axiom.admit_action(ActionInput {
            verb: ident("sign"),
            kind: ActionKind::Discrete,
            resource: ResourceId::from([9u8; 32]),
        }),
        Err(AxiomError::UnknownResource(_))
    ));
}

#[test]
fn rejects_action_kind_not_matching_resource_kind() {
    let mut store = Store::default();

    let resource = store.add_resource(
        Resource::create(ResourceInput {
            label: ident("resource"),
            kind: ResourceKind::Discrete,
        })
        .unwrap(),
    );

    let axiom = Axiom::new(&store);

    assert!(matches!(
        axiom.admit_action(ActionInput {
            verb: ident("increase"),
            kind: ActionKind::Quantifiable(Effect::Increase),
            resource,
        }),
        Err(AxiomError::ActionResourceKindMismatch)
    ));
}

#[test]
fn rejects_statement_referencing_unknown_action() {
    let mut store = Store::default();
    let role = store.add_role(Role::create(RoleInput { label: ident("role") }).unwrap());
    let axiom = Axiom::new(&store);

    assert!(matches!(
        axiom.admit_statement(StatementInput {
            participants: Participants::new([role], [role]).unwrap(),
            action: ActionId::from([3u8; 32]),
            settlement: Settlement::new([obs("Signed")], [obs("Cancelled")]).unwrap(),
        }),
        Err(AxiomError::UnknownAction(_))
    ));
}

#[test]
fn admits_a_valid_eligibility_assignment() {
    let mut store = Store::default();
    let role = store.add_role(Role::create(RoleInput { label: ident("role") }).unwrap());
    let agent = store.add_agent(
        Agent::create(AgentInput {
            label: ident("agent"),
            kind: AgentKind::Company,
        })
        .unwrap(),
    );
    let axiom = Axiom::new(&store);

    assert!(
        axiom
            .admit_eligibility_assignment(EligibilityAssignmentInput { agent, role })
            .is_ok()
    );
}

#[test]
fn rejects_eligibility_assignment_for_unknown_agent() {
    let mut store = Store::default();
    let role = store.add_role(Role::create(RoleInput { label: ident("role") }).unwrap());
    let axiom = Axiom::new(&store);

    assert!(matches!(
        axiom.admit_eligibility_assignment(EligibilityAssignmentInput {
            agent: AgentId::from([9u8; 32]),
            role,
        }),
        Err(AxiomError::UnknownAgent(_))
    ));
}

#[test]
fn rejects_eligibility_assignment_for_unknown_role() {
    let mut store = Store::default();
    let agent = store.add_agent(
        Agent::create(AgentInput {
            label: ident("agent"),
            kind: AgentKind::Company,
        })
        .unwrap(),
    );
    let axiom = Axiom::new(&store);

    assert!(matches!(
        axiom.admit_eligibility_assignment(EligibilityAssignmentInput {
            agent,
            role: RoleId::from([9u8; 32]),
        }),
        Err(AxiomError::UnknownRole(_))
    ));
}

#[test]
fn rejects_commitment_with_instance_of_another_resource() {
    let mut f = discrete_graph();

    let other = f.store.add_resource(
        Resource::create(ResourceInput {
            label: ident("other-resource"),
            kind: ResourceKind::Discrete,
        })
        .unwrap(),
    );
    let alien = f.store.add_instance(
        ResourceInstance::create(ResourceInstanceInput {
            label: ident("alien-instance"),
            resource: other,
        })
        .unwrap(),
    );

    f.instance = alien;

    assert!(matches!(
        commit(&f),
        Err(AxiomError::ResourceInstanceMismatch { .. })
    ));
}

#[test]
fn rejects_discrete_commitment_carrying_a_value() {
    let f = discrete_graph();
    let mut input = commitment_input(&f);

    input.action_value = ActionValue::value(5.0).unwrap();

    let result = Axiom::new(&f.store).admit_commitment(input);

    assert!(matches!(result, Err(AxiomError::ActionValueMismatch)));
}

#[test]
fn rejects_executor_without_eligibility_for_an_actor_role() {
    let mut f = discrete_graph();

    let bad = f.store.add_agent(
        Agent::create(AgentInput {
            label: ident("bad-executor"),
            kind: AgentKind::Individual,
        })
        .unwrap(),
    );

    f.executor = bad;

    assert!(matches!(
        commit(&f),
        Err(AxiomError::AgentNotEligibleForRole(_))
    ));
}

#[test]
fn accountable_needs_no_role_only_existence() {
    let mut f = discrete_graph();

    let bystander = f.store.add_agent(
        Agent::create(AgentInput {
            label: ident("bystander"),
            kind: AgentKind::Company,
        })
        .unwrap(),
    );

    f.accountable = bystander;

    assert!(commit(&f).is_ok());
}

#[test]
fn rejects_supersede_of_a_different_statement() {
    let mut f = discrete_graph();

    let action = f.store.add_action(
        Action::create(ActionInput {
            verb: ident("other-action"),
            kind: ActionKind::Discrete,
            resource: f.resource,
        })
        .unwrap(),
    );

    let other_statement = f.store.add_statement(
        Statement::create(StatementInput {
            participants: Participants::new([f.actor_role], [f.recipient_role]).unwrap(),
            action,
            settlement: Settlement::new([obs("Signed")], [obs("Cancelled")]).unwrap(),
        })
        .unwrap(),
    );

    let superseded_input = CommitmentInput {
        statement: other_statement,
        ..commitment_input(&f)
    };

    let superseded = f
        .store
        .add_commitment(Axiom::new(&f.store).admit_commitment(superseded_input).unwrap());

    let result = Axiom::new(&f.store).admit_commitment(CommitmentInput {
        supersedes: Some(superseded),
        ..commitment_input(&f)
    });

    assert!(matches!(result, Err(AxiomError::SupersedeStatementMismatch)));
}

#[test]
fn rejects_commitment_with_unknown_dependency() {
    let f = discrete_graph();

    let result = Axiom::new(&f.store).admit_commitment(CommitmentInput {
        dependencies: BTreeSet::from([CommitmentId::from([1u8; 32])]),
        ..commitment_input(&f)
    });

    assert!(matches!(result, Err(AxiomError::UnknownCommitment(_))));
}

#[test]
fn admits_a_recognized_event_and_rejects_an_unrecognized_one() {
    let mut f = discrete_graph();
    let commitment = f.store.add_commitment(commit(&f).unwrap());
    let axiom = Axiom::new(&f.store);

    assert!(
        axiom
            .admit_event(EventInput {
                commitment_id: commitment,
                observation: obs("Signed"),
                previous_event: None,
                occurred_at: date(2026, 6, 1),
            })
            .is_ok()
    );

    assert!(matches!(
        axiom.admit_event(EventInput {
            commitment_id: commitment,
            observation: obs("Unrelated"),
            previous_event: None,
            occurred_at: date(2026, 6, 1),
        }),
        Err(AxiomError::ObservationNotSettling)
    ));
}
