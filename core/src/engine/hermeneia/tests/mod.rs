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
mod continuity;
mod levels;
mod punctuality;
mod refusal;
mod resumption;
mod sequence;

struct Ledger {
    credit: StatementId,
    debit: StatementId,
    instance: ResourceInstanceId,
}

use std::collections::{BTreeMap, BTreeSet};

use super::Accumulation;

use crate::kernel::axiom::Knowledge;

use crate::kernel::entities::{
    Action, ActionId, ActionInput, Agent, AgentId, Commitment, CommitmentId, CommitmentInput,
    EligibilityAssignment, Event, EventId, EventInput, Resource, ResourceId, ResourceInput,
    ResourceInstance, ResourceInstanceId, ResourceInstanceInput, Role, RoleId, Statement,
    StatementId, StatementInput,
};

use crate::kernel::value_objects::{
    ActionKind, ActionValue, Assignment, Constraint, Date, Effect, Identifier, Observation,
    Participants, ResourceKind, Settlement, Term,
};

fn date(y: i32, m: u8, d: u8) -> Date {
    Date::from_ymd(y, m, d).unwrap()
}
fn obs(name: &str) -> Observation {
    Observation::new(name).unwrap()
}
fn ident(value: &str) -> Identifier {
    Identifier::new(value).unwrap()
}

/// Only the reads a projection performs are answered; the graph a commitment reaches.
/// Roles, agents, eligibility and the chain stay deliberately absent, so a
/// projection that grew a dependency on the admission path would fail here.
#[derive(Default)]
struct Fixture {
    commitments: BTreeMap<CommitmentId, Commitment>,
    statements: BTreeMap<StatementId, Statement>,
    actions: BTreeMap<ActionId, Action>,
    instances: BTreeMap<ResourceInstanceId, ResourceInstance>,
    resources: BTreeMap<ResourceId, Resource>,
}
impl Knowledge for Fixture {
    fn commitment(&self, id: CommitmentId) -> Option<Commitment> {
        self.commitments.get(&id).cloned()
    }
    fn statement(&self, id: StatementId) -> Option<Statement> {
        self.statements.get(&id).cloned()
    }
    fn action(&self, id: ActionId) -> Option<Action> {
        self.actions.get(&id).cloned()
    }
    fn resource_instance(&self, id: ResourceInstanceId) -> Option<ResourceInstance> {
        self.instances.get(&id).cloned()
    }
    fn resource(&self, id: ResourceId) -> Option<Resource> {
        self.resources.get(&id).cloned()
    }

    fn role(&self, _: RoleId) -> Option<Role> {
        None
    }
    fn agent(&self, _: AgentId) -> Option<Agent> {
        None
    }
    fn event(&self, _: EventId) -> Option<Event> {
        None
    }
    fn eligibilities_of(&self, _: AgentId) -> Vec<EligibilityAssignment> {
        Vec::new()
    }
}
impl Fixture {
    fn discrete(&mut self) -> StatementId {
        let resource = Resource::create(ResourceInput {
            label: ident("archive"),
            kind: ResourceKind::Discrete,
        })
        .unwrap();
        let action = Action::create(ActionInput {
            verb: ident("file"),
            kind: ActionKind::Discrete,
            resource: resource.id(),
        })
        .unwrap();

        let statement = self.statement(action.id());
        self.resources.insert(resource.id(), resource);
        self.actions.insert(action.id(), action);

        statement
    }

    fn quantifiable(
        &mut self,
        effect: Effect,
        constraint: Constraint,
    ) -> (StatementId, ResourceInstanceId) {
        let resource = Resource::create(ResourceInput {
            label: ident("stock"),
            kind: ResourceKind::Quantifiable(constraint),
        })
        .unwrap();
        let instance = ResourceInstance::create(ResourceInstanceInput {
            label: ident("warehouse"),
            resource: resource.id(),
        })
        .unwrap();
        let action = Action::create(ActionInput {
            verb: ident("move"),
            kind: ActionKind::Quantifiable(effect),
            resource: resource.id(),
        })
        .unwrap();

        let statement = self.statement(action.id());
        let instance_id = instance.id();

        self.resources.insert(resource.id(), resource);
        self.instances.insert(instance_id, instance);
        self.actions.insert(action.id(), action);

        (statement, instance_id)
    }

    /// One bounded level reachable through two actions, one raising it and one lowering it, so
    /// a sequence over the same instance can move in both directions.
    fn ledger(&mut self, constraint: Constraint) -> Ledger {
        let resource = Resource::create(ResourceInput {
            label: ident("account"),
            kind: ResourceKind::Quantifiable(constraint),
        })
        .unwrap();
        let instance = ResourceInstance::create(ResourceInstanceInput {
            label: ident("balance"),
            resource: resource.id(),
        })
        .unwrap();

        let mut action = |verb: &str, effect: Effect| {
            let action = Action::create(ActionInput {
                verb: ident(verb),
                kind: ActionKind::Quantifiable(effect),
                resource: resource.id(),
            })
            .unwrap();

            let id = action.id();
            self.actions.insert(id, action);

            self.statement(id)
        };

        let credit = action("credit", Effect::Increase);
        let debit = action("debit", Effect::Decrease);
        let instance_id = instance.id();

        self.resources.insert(resource.id(), resource);
        self.instances.insert(instance_id, instance);

        Ledger {
            credit,
            debit,
            instance: instance_id,
        }
    }

    fn statement(&mut self, action: ActionId) -> StatementId {
        let statement = Statement::create(StatementInput {
            participants: Participants::new([RoleId::from([1; 32])], [RoleId::from([2; 32])])
                .unwrap(),
            action,
            settlement: Settlement::new([obs("Delivered")], [obs("Cancelled")]).unwrap(),
        })
        .unwrap();

        let id = statement.id();
        self.statements.insert(id, statement);

        id
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

fn commit(
    knowledge: &mut Fixture,
    statement: StatementId,
    instance: ResourceInstanceId,
    value: ActionValue,
    due: Date,
    dependencies: BTreeSet<CommitmentId>,
) -> CommitmentId {
    let commitment = Commitment::create(CommitmentInput {
        assignment: Assignment::new(
            AgentId::from([1; 32]),
            [AgentId::from([2; 32])],
            [AgentId::from([3; 32])],
        )
        .unwrap(),
        statement,
        resource: instance,
        term: Term::new(date(2026, 1, 1), due).unwrap(),
        action_value: value,
        dependencies,
    })
    .unwrap();

    let id = commitment.id();
    knowledge.commitments.insert(id, commitment);

    id
}

fn guide() -> Guide {
    let mut knowledge = Fixture::default();
    let statement = knowledge.discrete();
    let instance = ResourceInstanceId::from([1; 32]);

    let mut chain = |due: Date, dependencies: BTreeSet<CommitmentId>| {
        commit(
            &mut knowledge,
            statement,
            instance,
            ActionValue::none(),
            due,
            dependencies,
        )
    };

    let a = chain(date(2026, 3, 31), BTreeSet::new());
    let b = chain(date(2026, 6, 30), BTreeSet::from([a]));
    let c = chain(date(2026, 9, 30), BTreeSet::from([b]));
    let d = chain(date(2026, 12, 31), BTreeSet::from([c]));

    Guide {
        knowledge,
        a,
        b,
        c,
        d,
    }
}

struct Basket {
    knowledge: Fixture,
    instance: ResourceInstanceId,
    ids: Vec<CommitmentId>,
}
impl Basket {
    fn accumulate(&self, events: &[Event]) -> Accumulation {
        let mut accumulation = Accumulation::default();
        accumulation
            .absorb(&self.knowledge, &self.ids, events)
            .unwrap();
        accumulation
    }
}

/// Four commitments each moving one `constraint`-bounded level by 40 in the direction of
/// `effect`, so the bound alone decides whether they fit. Only the last depends on the first,
/// which keeps the dependency half silent for the level arithmetic while still allowing one
/// commitment to be doomed on demand.
fn basket(effect: Effect, constraint: Constraint) -> Basket {
    let mut knowledge = Fixture::default();
    let (statement, instance) = knowledge.quantifiable(effect, constraint);

    let mut ids: Vec<CommitmentId> = Vec::new();
    for month in 3..6 {
        let id = commit(
            &mut knowledge,
            statement,
            instance,
            ActionValue::value(40).unwrap(),
            date(2026, month, 28),
            BTreeSet::new(),
        );
        ids.push(id);
    }

    let last = commit(
        &mut knowledge,
        statement,
        instance,
        ActionValue::value(40).unwrap(),
        date(2026, 6, 28),
        BTreeSet::from([ids[0]]),
    );
    ids.push(last);

    Basket {
        knowledge,
        instance,
        ids,
    }
}

fn chain<'a>(settlements: impl IntoIterator<Item = (CommitmentId, &'a str)>) -> Vec<Event> {
    let mut previous = None;

    settlements
        .into_iter()
        .map(|(commitment, observation)| {
            let event = Event::create(EventInput {
                commitment_id: commitment,
                observation: obs(observation),
                previous_event: previous,
                occurred_at: date(2026, 2, 1),
            })
            .unwrap();

            previous = Some(event.id());

            event
        })
        .collect()
}

/// One settling event, opening a chain of its own.
fn settles(commitment: CommitmentId, observation: &str) -> Event {
    Event::create(EventInput {
        commitment_id: commitment,
        observation: obs(observation),
        previous_event: None,
        occurred_at: date(2026, 2, 1),
    })
    .unwrap()
}
