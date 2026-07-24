//! Shared test harness for the Canon, and the test modules built on it.
//!
//! The tests run against the reference [`InMemoryHistory`] adapter (in `canon::memory`).
//! Here live the standalone factories and the seeded [`graph`] / [`seed_commitment`],
//! reused across the test modules.

mod admission;
mod concurrent;
mod envelope;

use std::collections::BTreeSet;

use super::{Canon, CanonicalHistory, InMemoryHistory};

use crate::kernel::entities::{
    ActionInput, AgentId, AgentInput, Commitment, CommitmentId, CommitmentInput,
    EligibilityAssignment, EligibilityAssignmentInput, Event, EventId, EventInput, ResourceInput,
    ResourceInstanceId, ResourceInstanceInput, RoleId, RoleInput, StatementId, StatementInput,
};

use crate::kernel::value_objects::{
    ActionKind, ActionValue, AgentKind, Assignment, Date, Identifier, Observation, Participants,
    ResourceKind, Settlement, Term,
};

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
    canon: Canon<InMemoryHistory>,
    accountable: AgentId,
    executor: AgentId,
    beneficiary: AgentId,
    actor_role: RoleId,
    instance: ResourceInstanceId,
    statement: StatementId,
}
fn graph() -> Graph {
    let mut canon = Canon::new(InMemoryHistory::default());
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

/// Seed a full valid graph through `canon` and admit one commitment, returning its id.
fn seed_commitment<H: CanonicalHistory>(canon: &mut Canon<H>) -> CommitmentId {
    let rec = date(2026, 7, 1);

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

    canon
        .admit_commitment(
            CommitmentInput {
                assignment: Assignment::new(accountable, [executor], [beneficiary]).unwrap(),
                statement,
                resource: instance,
                term: Term::new(date(2026, 1, 1), date(2026, 12, 31)).unwrap(),
                supersedes: None,
                action_value: ActionValue::none(),
                dependencies: BTreeSet::new(),
            },
            rec,
        )
        .unwrap()
}
