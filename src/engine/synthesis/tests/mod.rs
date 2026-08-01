//! Test harness for Synthesis, and the test modules built on it.
//!
//! What Synthesis reasons about is membership, so the fixture holds commitments, the Events
//! that freeze them, and nothing else — no statements, actions or agents, because deriving a
//! difference never resolves one. An Event here settles a commitment by existing; whether it
//! fulfils or cancels belongs to interpretation, which is not this layer's question.
//!
//! ```text
//! 2026-01-05  D1, when commitments are recorded
//! 2026-02-05  D2, when settling events are recorded
//! ```

mod base;
mod difference;

use std::collections::{BTreeMap, BTreeSet};

use crate::canon::{Canonical, CanonicalKnowledge};

use crate::engine::thesis::{ForkInput, GenesisInput, KnowledgeCut, Thesis};

use crate::kernel::entities::{
    AgentId, Commitment, CommitmentId, CommitmentInput, Event, EventId, EventInput,
    ResourceInstanceId, StatementId,
};

use crate::kernel::value_objects::{ActionValue, Assignment, Date, Observation, Term};

fn date(year: i32, month: u8, day: u8) -> Date {
    Date::from_ymd(year, month, day).unwrap()
}

/// When commitments are recorded.
fn d1() -> Date {
    date(2026, 1, 5)
}

/// When settling events are recorded, and the instant every cut here is taken at.
fn d2() -> Date {
    date(2026, 2, 5)
}

#[derive(Default)]
struct Fixture {
    commitments: BTreeMap<CommitmentId, Canonical<Commitment>>,
    events: BTreeMap<EventId, Canonical<Event>>,
    head: Option<EventId>,
}
impl CanonicalKnowledge for Fixture {
    fn canonical_commitment(&self, id: CommitmentId) -> Option<Canonical<Commitment>> {
        self.commitments.get(&id).cloned()
    }

    fn canonical_event(&self, id: EventId) -> Option<Canonical<Event>> {
        self.events.get(&id).cloned()
    }

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
impl Fixture {
    /// A commitment due on `due`, waiting on `dependencies`, recorded at [`d1`].
    fn commit(&mut self, due: (u8, u8), dependencies: BTreeSet<CommitmentId>) -> CommitmentId {
        let (month, day) = due;

        let commitment = Commitment::create(CommitmentInput {
            assignment: Assignment::new(
                AgentId::from([1; 32]),
                [AgentId::from([2; 32])],
                [AgentId::from([3; 32])],
            )
            .unwrap(),
            statement: StatementId::from([1; 32]),
            resource: ResourceInstanceId::from([1; 32]),
            term: Term::new(date(2026, 1, 1), date(2026, month, day)).unwrap(),
            action_value: ActionValue::none(),
            dependencies,
        })
        .unwrap();

        let id = commitment.id();
        self.commitments
            .insert(id, Canonical::new(commitment, d1()).unwrap());

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
        self.events.insert(id, Canonical::new(event, d2()).unwrap());
        self.head = Some(id);

        id
    }

    /// The cut [`d2`] addresses, resolved against whatever has been recorded so far.
    fn cut(&self) -> KnowledgeCut {
        KnowledgeCut::at(self, d2())
    }

    fn genesis(&self, selection: &[CommitmentId]) -> Thesis {
        Thesis::genesis(
            self,
            GenesisInput {
                cut: self.cut(),
                selection: selection.iter().copied().collect(),
            },
        )
        .unwrap()
    }
}

fn introducing(introduced: &[CommitmentId]) -> ForkInput {
    ForkInput {
        omitted: BTreeSet::new(),
        introduced: introduced.iter().copied().collect(),
    }
}

fn omitting(omitted: &[CommitmentId]) -> ForkInput {
    ForkInput {
        omitted: omitted.iter().copied().collect(),
        introduced: BTreeSet::new(),
    }
}

fn ids(of: &[CommitmentId]) -> BTreeSet<CommitmentId> {
    of.iter().copied().collect()
}
