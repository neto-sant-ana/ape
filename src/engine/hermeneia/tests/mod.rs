//! Test harness for the interpretation layer, and the test modules built on it.
//!
//! The fixture is a bare [`Knowledge`] holding commitments and one statement, built
//! without the Canon, the Axiom, roles, agents or eligibility. That is not a shortcut: a
//! projection resolves ids and derives consequences, so if it needed an admission path to
//! be exercised, it would be reading something it has no business reading.
//!
//! The guide graph is a chain `a → b → c → d`, each depending on the one before, with
//! staggered deadlines so timeliness can be moved independently of settlement.

mod chain;
mod resumption;

use std::collections::{BTreeMap, BTreeSet};

use super::Accumulation;

use crate::kernel::axiom::Knowledge;

use crate::kernel::entities::{
    Action, ActionId, Agent, AgentId, Commitment, CommitmentId, CommitmentInput,
    EligibilityAssignment, Event, EventId, EventInput, Resource, ResourceId, ResourceInstance,
    ResourceInstanceId, Role, RoleId, Statement, StatementId, StatementInput,
};

use crate::kernel::value_objects::{
    ActionValue, Assignment, Date, Observation, Participants, Settlement, Term,
};

fn date(y: i32, m: u8, d: u8) -> Date {
    Date::from_ymd(y, m, d).unwrap()
}
fn obs(name: &str) -> Observation {
    Observation::new(name).unwrap()
}

/// Only the two reads a projection performs are answered; the rest of the ontology is
/// deliberately absent, so a projection that grew a dependency on it would fail here.
#[derive(Default)]
struct Fixture {
    commitments: BTreeMap<CommitmentId, Commitment>,
    statements: BTreeMap<StatementId, Statement>,
}
impl Knowledge for Fixture {
    fn commitment(&self, id: CommitmentId) -> Option<Commitment> {
        self.commitments.get(&id).cloned()
    }
    fn statement(&self, id: StatementId) -> Option<Statement> {
        self.statements.get(&id).cloned()
    }

    fn role(&self, _: RoleId) -> Option<Role> {
        None
    }
    fn agent(&self, _: AgentId) -> Option<Agent> {
        None
    }
    fn resource(&self, _: ResourceId) -> Option<Resource> {
        None
    }
    fn resource_instance(&self, _: ResourceInstanceId) -> Option<ResourceInstance> {
        None
    }
    fn action(&self, _: ActionId) -> Option<Action> {
        None
    }
    fn event(&self, _: EventId) -> Option<Event> {
        None
    }
    fn eligibilities_of(&self, _: AgentId) -> Vec<EligibilityAssignment> {
        Vec::new()
    }
}

struct Guide {
    knowledge: Fixture,
    a: CommitmentId,
    b: CommitmentId,
    c: CommitmentId,
    d: CommitmentId,
}
impl Guide {
    fn selection(&self) -> Vec<CommitmentId> {
        vec![self.a, self.b, self.c, self.d]
    }

    fn accumulate(&self, events: &[Event]) -> Accumulation {
        let mut accumulation = Accumulation::default();
        accumulation
            .absorb(&self.knowledge, &self.selection(), events)
            .unwrap();
        accumulation
    }
}

fn guide() -> Guide {
    let mut knowledge = Fixture::default();

    let statement = Statement::create(StatementInput {
        participants: Participants::new([RoleId::from([1; 32])], [RoleId::from([2; 32])]).unwrap(),
        action: ActionId::from([1; 32]),
        settlement: Settlement::new([obs("Delivered")], [obs("Cancelled")]).unwrap(),
    })
    .unwrap();
    let statement_id = statement.id();
    knowledge.statements.insert(statement_id, statement);

    let mut commit = |due: Date, dependencies: BTreeSet<CommitmentId>| {
        let commitment = Commitment::create(CommitmentInput {
            assignment: Assignment::new(
                AgentId::from([1; 32]),
                [AgentId::from([2; 32])],
                [AgentId::from([3; 32])],
            )
            .unwrap(),
            statement: statement_id,
            resource: ResourceInstanceId::from([1; 32]),
            term: Term::new(date(2026, 1, 1), due).unwrap(),
            action_value: ActionValue::none(),
            dependencies,
        })
        .unwrap();

        let id = commitment.id();
        knowledge.commitments.insert(id, commitment);
        id
    };

    let a = commit(date(2026, 3, 31), BTreeSet::new());
    let b = commit(date(2026, 6, 30), BTreeSet::from([a]));
    let c = commit(date(2026, 9, 30), BTreeSet::from([b]));
    let d = commit(date(2026, 12, 31), BTreeSet::from([c]));

    Guide {
        knowledge,
        a,
        b,
        c,
        d,
    }
}

fn settles(commitment: CommitmentId, observation: &str) -> Event {
    Event::create(EventInput {
        commitment_id: commitment,
        observation: obs(observation),
        previous_event: None,
        occurred_at: date(2026, 2, 1),
    })
    .unwrap()
}
