//! The fixture answers only the reads a derivation performs: a commitment and its
//! dependencies, and an Event of the chain. Roles, agents, eligibility and resource
//! instances stay deliberately absent, so a derivation that grew a dependency on anything
//! beyond the graph and the chain would fail here.
//!
//! `settle` appends to the chain the same way the Canon does, so a walk backwards from
//! any head reaches exactly the history that head recognizes.

mod advancement;
mod fork;
mod genesis;
mod projection;

use std::collections::{BTreeMap, BTreeSet};

use super::{ForkInput, GenesisInput, Thesis};

use crate::kernel::axiom::Knowledge;

use crate::kernel::entities::{
    Action, ActionId, ActionInput, Agent, AgentId, Commitment, CommitmentId, CommitmentInput,
    EligibilityAssignment, Event, EventId, EventInput, Resource, ResourceId, ResourceInput,
    ResourceInstance, ResourceInstanceId, Role, RoleId, Statement, StatementId, StatementInput,
};

use crate::kernel::value_objects::{
    ActionKind, ActionValue, Assignment, Date, Identifier, Observation, Participants, ResourceKind,
    Settlement, Term,
};

fn date(y: i32, m: u8, d: u8) -> Date {
    Date::from_ymd(y, m, d).unwrap()
}
fn ident(value: &str) -> Identifier {
    Identifier::new(value).unwrap()
}

struct Fixture {
    commitments: BTreeMap<CommitmentId, Commitment>,
    statements: BTreeMap<StatementId, Statement>,
    actions: BTreeMap<ActionId, Action>,
    events: BTreeMap<EventId, Event>,
    statement: StatementId,
    head: Option<EventId>,
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
    fn event(&self, id: EventId) -> Option<Event> {
        self.events.get(&id).cloned()
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
    fn eligibilities_of(&self, _: AgentId) -> Vec<EligibilityAssignment> {
        Vec::new()
    }
}
impl Fixture {
    fn new() -> Self {
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
        let statement = Statement::create(StatementInput {
            participants: Participants::new([RoleId::from([1; 32])], [RoleId::from([2; 32])])
                .unwrap(),
            action: action.id(),
            settlement: Settlement::new(
                [Observation::new("Delivered").unwrap()],
                [Observation::new("Cancelled").unwrap()],
            )
            .unwrap(),
        })
        .unwrap();

        Self {
            commitments: BTreeMap::new(),
            statements: BTreeMap::from([(statement.id(), statement.clone())]),
            actions: BTreeMap::from([(action.id(), action)]),
            events: BTreeMap::new(),
            statement: statement.id(),
            head: None,
        }
    }

    /// A commitment due on `due`, waiting on `dependencies`.
    fn commit(&mut self, due: (u8, u8), dependencies: BTreeSet<CommitmentId>) -> CommitmentId {
        let (month, day) = due;

        let commitment = Commitment::create(CommitmentInput {
            assignment: Assignment::new(
                AgentId::from([1; 32]),
                [AgentId::from([2; 32])],
                [AgentId::from([3; 32])],
            )
            .unwrap(),
            statement: self.statement,
            resource: ResourceInstanceId::from([1; 32]),
            term: Term::new(date(2026, 1, 1), date(2026, month, day)).unwrap(),
            action_value: ActionValue::none(),
            dependencies,
        })
        .unwrap();

        let id = commitment.id();
        self.commitments.insert(id, commitment);

        id
    }

    /// Settle `commitment`, extending the chain and advancing the head.
    fn settle(&mut self, commitment: CommitmentId) -> EventId {
        let event = Event::create(EventInput {
            commitment_id: commitment,
            observation: Observation::new("Delivered").unwrap(),
            previous_event: self.head,
            occurred_at: date(2026, 2, 1),
        })
        .unwrap();

        let id = event.id();
        self.events.insert(id, event);
        self.head = Some(id);

        id
    }

    /// An Event opening a chain of its own, admitted nowhere: a head no Thesis descends from.
    fn detached(&mut self, commitment: CommitmentId) -> EventId {
        let event = Event::create(EventInput {
            commitment_id: commitment,
            observation: Observation::new("Delivered").unwrap(),
            previous_event: None,
            occurred_at: date(2026, 3, 1),
        })
        .unwrap();

        let id = event.id();
        self.events.insert(id, event);

        id
    }

    /// The chain a head recognizes, in the order Hermeneia absorbs it.
    fn chain_through(&self, head: Option<EventId>) -> Vec<Event> {
        let mut chain = Vec::new();
        let mut cursor = head;

        while let Some(id) = cursor {
            let event = self.events.get(&id).unwrap().clone();
            cursor = *event.previous_event();
            chain.push(event);
        }

        chain.reverse();
        chain
    }

    fn genesis(&self, head: Option<EventId>, selection: &[CommitmentId]) -> Thesis {
        Thesis::genesis(
            self,
            GenesisInput {
                head,
                selection: selection.iter().copied().collect(),
            },
        )
        .unwrap()
    }
}

fn omitting(omitted: &[CommitmentId]) -> ForkInput {
    ForkInput {
        omitted: omitted.iter().copied().collect(),
        introduced: BTreeSet::new(),
    }
}

fn introducing(introduced: &[CommitmentId]) -> ForkInput {
    ForkInput {
        omitted: BTreeSet::new(),
        introduced: introduced.iter().copied().collect(),
    }
}

fn ids(commitments: &[CommitmentId]) -> BTreeSet<CommitmentId> {
    commitments.iter().copied().collect()
}

/// The same commitments in the shape [`Thesis::selection`] returns them.
fn selected(commitments: &[CommitmentId]) -> Vec<CommitmentId> {
    ids(commitments).into_iter().collect()
}
