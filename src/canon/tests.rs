use std::collections::BTreeMap;
use std::collections::BTreeSet;

use super::{AppendOutcome, CanonError, Canonical, CanonicalHistory};

use crate::kernel::entities::{
    AgentId, Commitment, CommitmentId, CommitmentInput, EligibilityAssignment,
    EligibilityAssignmentId, EligibilityAssignmentInput, Event, EventId, EventInput,
    ResourceInstanceId, RoleId, StatementId,
};

use crate::kernel::value_objects::{ActionValue, Assignment, Date, Observation, Term};

/// The reference in-memory canonical history. It holds no admission rules — only
/// durable storage and one compare-and-swap.
#[derive(Default)]
struct MemoryHistory {
    commitments: BTreeMap<CommitmentId, Canonical<Commitment>>,
    eligibility: BTreeMap<EligibilityAssignmentId, Canonical<EligibilityAssignment>>,
    events: BTreeMap<EventId, Canonical<Event>>,
    head: Option<EventId>,
}
impl CanonicalHistory for MemoryHistory {
    fn head(&self) -> Option<EventId> {
        self.head
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

    fn advance_head(
        &mut self,
        expected: Option<EventId>,
        new: EventId,
    ) -> Result<(), CanonError> {
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

#[test]
fn put_commitment_is_idempotent_by_id() {
    let mut history = MemoryHistory::default();
    let record = Canonical::new(commitment(1), date(2026, 7, 1));

    assert_eq!(
        history.put_commitment(record.clone()),
        AppendOutcome::Admitted
    );

    assert_eq!(
        history.put_commitment(record),
        AppendOutcome::AlreadyPresent
    );
}

#[test]
fn put_eligibility_is_idempotent_by_id() {
    let mut history = MemoryHistory::default();
    let record = Canonical::new(eligibility(1), date(2026, 7, 1));

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
    let record = Canonical::new(event(commitment(1).id(), None, "Signed"), date(2026, 7, 1));

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

    history.put_event(Canonical::new(genesis, date(2026, 7, 1)));
    history.advance_head(None, genesis_id).unwrap();

    let orphan = event(commitment.id(), None, "Paid");
    let orphan_id = orphan.id();

    assert_eq!(
        history.put_event(Canonical::new(orphan, date(2026, 7, 2))),
        AppendOutcome::Admitted
    );

    assert!(history.advance_head(None, orphan_id).is_err());
    assert_eq!(history.head(), Some(genesis_id));
}
