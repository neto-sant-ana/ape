//! The fixture answers only the reads a derivation performs: a commitment record and an
//! Event record. Roles, agents, eligibility and resource instances stay deliberately absent,
//! so a derivation that grew a dependency on anything beyond the graph and the chain would
//! fail here.
//!
//! It implements both ports on purpose. A Thesis reads [`CanonicalKnowledge`], because it
//! needs the recording instant a bare entity does not carry; Hermeneia reads [`Knowledge`],
//! and the projection suite drives it over the same fixture. A real adapter answers both the
//! same way.
//!
//! `settle` appends to the chain the same way the Canon does, so a walk backwards from
//! any head reaches exactly the history that head recognizes.
//!
//! Recording instants are explicit throughout, because they are what the cut is made of. The
//! calendar the suites share:
//!
//! ```text
//! 2026-01-01  committed_at of every commitment
//! 2026-01-05  D1, when commitments are recorded by default
//! 2026-02-01  occurred_at of every settling event
//! 2026-02-05  D2, when events are recorded by default
//! 2026-03-05  D3, a later cut with nothing recorded at it
//! ```

mod advancement;
mod archive;
mod cut;
mod fork;
mod genesis;
mod lineage;
mod projection;

use std::collections::{BTreeMap, BTreeSet};

use super::{Advancement, ForkInput, GenesisInput, KnowledgeCut, Thesis};

use crate::canon::{Canonical, CanonicalKnowledge};

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

/// When commitments are recorded unless a test says otherwise.
fn d1() -> Date {
    date(2026, 1, 5)
}
/// When settling events are recorded unless a test says otherwise.
fn d2() -> Date {
    date(2026, 2, 5)
}
/// A cut later than both, with nothing recorded at it.
fn d3() -> Date {
    date(2026, 3, 5)
}

struct Fixture {
    commitments: BTreeMap<CommitmentId, Canonical<Commitment>>,
    statements: BTreeMap<StatementId, Statement>,
    actions: BTreeMap<ActionId, Action>,
    events: BTreeMap<EventId, Canonical<Event>>,
    statement: StatementId,
    head: Option<EventId>,
}
impl CanonicalKnowledge for Fixture {
    fn canonical_commitment(&self, id: CommitmentId) -> Option<Canonical<Commitment>> {
        self.commitments.get(&id).cloned()
    }
    fn canonical_event(&self, id: EventId) -> Option<Canonical<Event>> {
        self.events.get(&id).cloned()
    }

    /// Walked back from the head, as the reference adapter does: recording never decreases along
    /// the chain, so the first Event recorded no later than `at` is the latest one.
    fn head_as_of(&self, at: &Date) -> Option<EventId> {
        let mut cursor = self.head;

        while let Some(id) = cursor {
            let record = self.events.get(&id)?;

            if record.recorded_at().up_to(at) {
                return Some(id);
            }

            cursor = *record.assertion().previous_event();
        }

        None
    }
}
impl Knowledge for Fixture {
    fn commitment(&self, id: CommitmentId) -> Option<Commitment> {
        self.commitments
            .get(&id)
            .map(|record| record.assertion().clone())
    }
    fn event(&self, id: EventId) -> Option<Event> {
        self.events
            .get(&id)
            .map(|record| record.assertion().clone())
    }
    fn statement(&self, id: StatementId) -> Option<Statement> {
        self.statements.get(&id).cloned()
    }
    fn action(&self, id: ActionId) -> Option<Action> {
        self.actions.get(&id).cloned()
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

    /// A commitment due on `due`, waiting on `dependencies`, recorded at [`d1`].
    fn commit(&mut self, due: (u8, u8), dependencies: BTreeSet<CommitmentId>) -> CommitmentId {
        self.commit_recorded_at(d1(), due, dependencies)
    }

    /// The same, admitted into canonical history at a stated instant.
    fn commit_recorded_at(
        &mut self,
        recorded_at: Date,
        due: (u8, u8),
        dependencies: BTreeSet<CommitmentId>,
    ) -> CommitmentId {
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
        self.commitments
            .insert(id, Canonical::new(commitment, recorded_at).unwrap());

        id
    }

    /// Settle `commitment`, extending the chain and advancing the head.
    fn settle(&mut self, commitment: CommitmentId) -> EventId {
        self.settle_recorded_at(d2(), commitment)
    }

    /// The same, admitted into canonical history at a stated instant.
    fn settle_recorded_at(&mut self, recorded_at: Date, commitment: CommitmentId) -> EventId {
        let event = Event::create(EventInput {
            commitment_id: commitment,
            observation: Observation::new("Delivered").unwrap(),
            previous_event: self.head,
            occurred_at: date(2026, 2, 1),
        })
        .unwrap();

        let id = event.id();
        self.events
            .insert(id, Canonical::new(event, recorded_at).unwrap());
        self.head = Some(id);

        id
    }

    /// An Event opening a chain of its own: a head no Thesis descends from.
    fn detached(&mut self, commitment: CommitmentId) -> EventId {
        let event = Event::create(EventInput {
            commitment_id: commitment,
            observation: Observation::new("Delivered").unwrap(),
            previous_event: None,
            occurred_at: date(2026, 2, 1),
        })
        .unwrap();

        let id = event.id();
        self.events.insert(id, Canonical::new(event, d2()).unwrap());

        id
    }

    /// An Event whose predecessor was never admitted: the head of a chain with a missing link.
    fn severed(&mut self, commitment: CommitmentId) -> EventId {
        let event = Event::create(EventInput {
            commitment_id: commitment,
            observation: Observation::new("Delivered").unwrap(),
            previous_event: Some(EventId::from([9; 32])),
            occurred_at: date(2026, 2, 1),
        })
        .unwrap();

        let id = event.id();
        self.events.insert(id, Canonical::new(event, d2()).unwrap());
        self.head = Some(id);

        id
    }

    /// The chain a head recognizes, in the order Hermeneia absorbs it.
    fn chain_through(&self, head: Option<EventId>) -> Vec<Event> {
        let mut chain = Vec::new();
        let mut cursor = head;

        while let Some(id) = cursor {
            let event = self.events.get(&id).unwrap().assertion().clone();
            cursor = *event.previous_event();
            chain.push(event);
        }

        chain.reverse();
        chain
    }

    /// The cut the instant addresses, resolved against whatever has been recorded so far.
    fn cut(&self, known_at: Date) -> KnowledgeCut {
        KnowledgeCut::at(self, known_at)
    }

    /// A finer cut, naming a head within the instant's group.
    fn cut_within(&self, known_at: Date, event_head: EventId) -> KnowledgeCut {
        KnowledgeCut::within(self, known_at, event_head).unwrap()
    }

    fn genesis(&self, cut: KnowledgeCut, selection: &[CommitmentId]) -> Thesis {
        Thesis::genesis(
            self,
            GenesisInput {
                cut,
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

/// The halves of a selection, gathered so a test can state them as sets.
fn frozen_of(thesis: &Thesis) -> BTreeSet<CommitmentId> {
    thesis.selection().frozen().collect()
}
fn open_of(thesis: &Thesis) -> BTreeSet<CommitmentId> {
    thesis.selection().open().collect()
}
fn resolved_of(thesis: &Thesis) -> BTreeSet<CommitmentId> {
    thesis.selection().resolved().collect()
}
fn imposed_of(advancement: &Advancement) -> BTreeSet<CommitmentId> {
    advancement.imposed().collect()
}
