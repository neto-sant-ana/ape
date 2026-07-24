//! The reference in-memory canonical history: the two faces of one repository.
//! `Knowledge` answers the Axiom's lookups (returning the assertion inside each
//! record); the [`CanonicalHistory`] primitives are a dumb put-if-absent and a
//! dumb compare-and-swap.

use std::collections::{BTreeMap, BTreeSet};

use super::{AppendOutcome, Canon, CanonError, Canonical, CanonicalHistory, EventSubmission};

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
        put_if_absent(&mut self.commitments, commitment.assertion().id(), commitment)
    }
    fn put_eligibility(&mut self, eligibility: Canonical<EligibilityAssignment>) -> AppendOutcome {
        put_if_absent(&mut self.eligibility, eligibility.assertion().id(), eligibility)
    }
    fn put_event(&mut self, event: Canonical<Event>) -> AppendOutcome {
        put_if_absent(&mut self.events, event.assertion().id(), event)
    }

    fn advance_head(&mut self, expected: Option<EventId>, new: EventId) -> Result<(), CanonError> {
        if self.head != expected {
            return Err(CanonError::UnexpectedHead {
                expected,
                found: self.head,
            });
        }
        self.head = Some(new);
        Ok(())
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

    let actor_role = canon.admit_role(RoleInput { label: ident("actor") }, rec).unwrap();
    let recipient_role = canon
        .admit_role(RoleInput { label: ident("recipient") }, rec)
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

// ---------------------------------------------------------------------------
// Primitive-level tests
// ---------------------------------------------------------------------------

#[test]
fn put_commitment_is_idempotent_by_id() {
    let mut history = MemoryHistory::default();
    let record = Canonical::new(commitment(1), date(2026, 7, 1)).unwrap();

    assert_eq!(
        history.put_commitment(record.clone()),
        AppendOutcome::Admitted
    );
    assert_eq!(history.put_commitment(record), AppendOutcome::AlreadyPresent);
}

#[test]
fn put_eligibility_is_idempotent_by_id() {
    let mut history = MemoryHistory::default();
    let record = Canonical::new(eligibility(1), date(2026, 7, 1)).unwrap();

    assert_eq!(
        history.put_eligibility(record.clone()),
        AppendOutcome::Admitted
    );
    assert_eq!(
        history.put_eligibility(record),
        AppendOutcome::AlreadyPresent
    );
}

#[test]
fn put_event_is_idempotent_by_id() {
    let mut history = MemoryHistory::default();
    let record = Canonical::new(event(commitment(1).id(), None, "Signed"), date(2026, 7, 1)).unwrap();

    assert_eq!(history.put_event(record.clone()), AppendOutcome::Admitted);
    assert_eq!(history.put_event(record), AppendOutcome::AlreadyPresent);
}

#[test]
fn advance_head_extends_from_the_expected_head() {
    let mut history = MemoryHistory::default();
    let commitment = commitment(1);
    assert_eq!(history.head(), None);

    let genesis = event(commitment.id(), None, "Signed").id();
    assert!(history.advance_head(None, genesis).is_ok());
    assert_eq!(history.head(), Some(genesis));

    let next = event(commitment.id(), Some(genesis), "Paid").id();
    assert!(history.advance_head(Some(genesis), next).is_ok());
    assert_eq!(history.head(), Some(next));
}

#[test]
fn advance_head_rejects_a_stale_expected() {
    let mut history = MemoryHistory::default();
    let commitment = commitment(1);
    let genesis = event(commitment.id(), None, "Signed").id();
    history.advance_head(None, genesis).unwrap();

    let other = event(commitment.id(), None, "Paid").id();
    assert!(matches!(
        history.advance_head(None, other),
        Err(CanonError::UnexpectedHead {
            expected: None,
            found: Some(found),
        }) if found == genesis
    ));

    let alien = EventId::from([9u8; 32]);
    assert!(matches!(
        history.advance_head(Some(alien), other),
        Err(CanonError::UnexpectedHead { expected, found })
            if expected == Some(alien) && found == Some(genesis)
    ));
}

#[test]
fn a_stored_event_left_unlinked_is_a_harmless_dangling_object() {
    let mut history = MemoryHistory::default();
    let commitment = commitment(1);

    let genesis = event(commitment.id(), None, "Signed");
    let genesis_id = genesis.id();
    history.put_event(Canonical::new(genesis, date(2026, 7, 1)).unwrap());
    history.advance_head(None, genesis_id).unwrap();

    let orphan = event(commitment.id(), None, "Paid");
    let orphan_id = orphan.id();
    assert_eq!(
        history.put_event(Canonical::new(orphan, date(2026, 7, 2)).unwrap()),
        AppendOutcome::Admitted
    );
    assert!(history.advance_head(None, orphan_id).is_err());
    assert_eq!(history.head(), Some(genesis_id));
}

// ---------------------------------------------------------------------------
// Envelope-level tests
// ---------------------------------------------------------------------------

#[test]
fn an_event_recorded_before_it_occurred_is_rejected() {
    let observed = event(commitment(1).id(), None, "Signed"); // occurs 2026-06-01

    assert!(matches!(
        Canonical::new(observed, date(2026, 5, 31)),
        Err(CanonError::RecordedBeforeFact { .. })
    ));
}

#[test]
fn a_commitment_recorded_before_it_was_committed_is_rejected() {
    assert!(matches!(
        Canonical::new(commitment(1), date(2025, 12, 31)), // committed 2026-01-01
        Err(CanonError::RecordedBeforeFact { .. })
    ));
}

#[test]
fn an_eligibility_may_be_recorded_before_it_takes_effect() {
    assert!(Canonical::new(eligibility(1), date(2020, 1, 1)).is_ok());
}

// ---------------------------------------------------------------------------
// Orchestrator-level tests
// ---------------------------------------------------------------------------

#[test]
fn admits_a_valid_commitment_and_is_idempotent() {
    let g = graph();
    let input = commitment_input(&g);

    let mut canon = g.canon;
    let first = canon.admit_commitment(input.clone(), date(2026, 2, 1)).unwrap();
    let again = canon.admit_commitment(input, date(2026, 3, 1)).unwrap();

    assert_eq!(first, again);
}

#[test]
fn propagates_a_structural_rejection_from_the_axiom() {
    let g = graph();

    let input = CommitmentInput {
        assignment: Assignment::new(AgentId::from([99u8; 32]), [g.executor], [g.beneficiary])
            .unwrap(),
        ..commitment_input(&g)
    };

    let mut canon = g.canon;

    assert!(matches!(
        canon.admit_commitment(input, date(2026, 2, 1)),
        Err(CanonError::Axiom(_))
    ));
}

#[test]
fn admits_an_eligibility_declared_effective_in_the_future() {
    let g = graph();
    let agent = g.accountable;
    let role = g.actor_role;
    let mut canon = g.canon;

    assert!(
        canon
            .admit_eligibility(
                EligibilityAssignmentInput {
                    agent,
                    roles: BTreeSet::from([role]),
                    effective_from: date(2030, 1, 1),
                },
                date(2026, 1, 1),
            )
            .is_ok()
    );
}

#[test]
fn admits_events_extending_the_chain() {
    let g = graph();
    let input = commitment_input(&g);
    let mut canon = g.canon;
    let commitment_id = canon.admit_commitment(input, date(2026, 2, 1)).unwrap();

    assert_eq!(canon.history().head(), None);

    let first = canon
        .admit_event(
            EventSubmission {
                commitment_id,
                observation: obs("Signed"),
                occurred_at: date(2026, 6, 1),
            },
            date(2026, 6, 2),
        )
        .unwrap();

    assert_eq!(canon.history().head(), Some(first));

    let second = canon
        .admit_event(
            EventSubmission {
                commitment_id,
                observation: obs("Cancelled"),
                occurred_at: date(2026, 7, 1),
            },
            date(2026, 7, 2),
        )
        .unwrap();

    assert_eq!(canon.history().head(), Some(second));
    assert_ne!(first, second);
}
