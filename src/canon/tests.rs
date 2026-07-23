use std::collections::{BTreeMap, BTreeSet};

use super::{AppendOutcome, CanonError, Canonical, CanonicalHistory};

use crate::kernel::entities::{
    AgentId, Commitment, CommitmentId, CommitmentInput, EligibilityAssignment,
    EligibilityAssignmentId, EligibilityAssignmentInput, Event, EventId, EventInput,
    ResourceInstanceId, RoleId, StatementId,
};

use crate::kernel::value_objects::{ActionValue, Assignment, Date, Observation, Term};

/// The reference in-memory canonical history. It enforces only the atomic
/// conditions — id absence and the event chain head — never the admission rules.
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

    fn append_commitment(
        &mut self,
        commitment: Canonical<Commitment>,
    ) -> Result<AppendOutcome, CanonError> {
        let id = commitment.assertion().id();
        if self.commitments.contains_key(&id) {
            return Ok(AppendOutcome::AlreadyPresent);
        }
        self.commitments.insert(id, commitment);
        Ok(AppendOutcome::Admitted)
    }

    fn append_eligibility(
        &mut self,
        eligibility: Canonical<EligibilityAssignment>,
    ) -> Result<AppendOutcome, CanonError> {
        let id = eligibility.assertion().id();
        if self.eligibility.contains_key(&id) {
            return Ok(AppendOutcome::AlreadyPresent);
        }
        self.eligibility.insert(id, eligibility);
        Ok(AppendOutcome::Admitted)
    }

    fn append_event(&mut self, event: Canonical<Event>) -> Result<AppendOutcome, CanonError> {
        let id = event.assertion().id();
        if self.events.contains_key(&id) {
            return Ok(AppendOutcome::AlreadyPresent);
        }

        let expected = *event.assertion().previous_event();
        if self.head != expected {
            return Err(CanonError::UnexpectedHead {
                expected,
                found: self.head,
            });
        }

        self.events.insert(id, event);
        self.head = Some(id);
        Ok(AppendOutcome::Admitted)
    }
}

fn date(y: i32, m: u8, d: u8) -> Date {
    Date::from_ymd(y, m, d).unwrap()
}

// The store never inspects entity contents beyond their id and chain link, so the
// factory data only needs to be valid and distinct — `tag` varies the id.
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
        occurred_at: date(2025, 1, 1),
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
fn append_commitment_is_idempotent_by_id() {
    let mut history = MemoryHistory::default();
    let record = Canonical::new(commitment(1), date(2026, 7, 1));

    assert_eq!(
        history.append_commitment(record.clone()).unwrap(),
        AppendOutcome::Admitted
    );
    assert_eq!(
        history.append_commitment(record).unwrap(),
        AppendOutcome::AlreadyPresent
    );
}

#[test]
fn append_eligibility_is_idempotent_by_id() {
    let mut history = MemoryHistory::default();
    let record = Canonical::new(eligibility(1), date(2026, 7, 1));

    assert_eq!(
        history.append_eligibility(record.clone()).unwrap(),
        AppendOutcome::Admitted
    );
    assert_eq!(
        history.append_eligibility(record).unwrap(),
        AppendOutcome::AlreadyPresent
    );
}

#[test]
fn events_extend_a_single_chain_and_advance_the_head() {
    let mut history = MemoryHistory::default();
    let commitment = commitment(1);
    assert_eq!(history.head(), None);

    let genesis = Canonical::new(event(commitment.id(), None, "Signed"), date(2026, 7, 1));
    let genesis_id = genesis.assertion().id();
    assert_eq!(
        history.append_event(genesis).unwrap(),
        AppendOutcome::Admitted
    );
    assert_eq!(history.head(), Some(genesis_id));

    let next = Canonical::new(
        event(commitment.id(), Some(genesis_id), "Paid"),
        date(2026, 7, 2),
    );
    let next_id = next.assertion().id();
    assert_eq!(history.append_event(next).unwrap(), AppendOutcome::Admitted);
    assert_eq!(history.head(), Some(next_id));
}

#[test]
fn rejects_a_second_genesis_event() {
    let mut history = MemoryHistory::default();
    let commitment = commitment(1);

    history
        .append_event(Canonical::new(
            event(commitment.id(), None, "Signed"),
            date(2026, 7, 1),
        ))
        .unwrap();

    // A different genesis (distinct observation, still previous_event == None) must
    // not fork the chain: the head is no longer empty.
    let second_genesis = Canonical::new(event(commitment.id(), None, "Paid"), date(2026, 7, 2));
    assert!(matches!(
        history.append_event(second_genesis),
        Err(CanonError::UnexpectedHead {
            expected: None,
            found: Some(_),
        })
    ));
}

#[test]
fn rejects_an_event_extending_a_stale_head() {
    let mut history = MemoryHistory::default();
    let commitment = commitment(1);

    let genesis = Canonical::new(event(commitment.id(), None, "Signed"), date(2026, 7, 1));
    let genesis_id = genesis.assertion().id();
    history.append_event(genesis).unwrap();

    // An event built against a head that never was — the CAS reports what it
    // expected against the real head, signalling a rebuild.
    let alien = EventId::from([9u8; 32]);
    let stale = Canonical::new(event(commitment.id(), Some(alien), "Paid"), date(2026, 7, 2));
    assert!(matches!(
        history.append_event(stale),
        Err(CanonError::UnexpectedHead { expected, found })
            if expected == Some(alien) && found == Some(genesis_id)
    ));
}

#[test]
fn re_appending_the_same_event_is_idempotent() {
    let mut history = MemoryHistory::default();
    let commitment = commitment(1);
    let genesis = Canonical::new(event(commitment.id(), None, "Signed"), date(2026, 7, 1));

    assert_eq!(
        history.append_event(genesis.clone()).unwrap(),
        AppendOutcome::Admitted
    );
    assert_eq!(
        history.append_event(genesis).unwrap(),
        AppendOutcome::AlreadyPresent
    );
}
