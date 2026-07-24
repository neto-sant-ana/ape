//! The conformance suite for [`CanonicalHistory`] adapters.
//!
//! A storage adapter proves it honors the contract by running [`verify`] against a
//! fresh instance in its own test suite:
//!
//! ```ignore
//! #[test]
//! fn my_history_conforms() {
//!     ape::canon::conformance::verify(MyHistory::default);
//! }
//! ```

use std::collections::BTreeSet;

use crate::canon::{AppendOutcome, Canonical, CanonicalHistory};

use crate::kernel::entities::{
    AgentId, Commitment, CommitmentId, CommitmentInput, EligibilityAssignment,
    EligibilityAssignmentInput, Event, EventId, EventInput, ResourceInstanceId, RoleId,
    StatementId,
};

use crate::kernel::value_objects::{ActionValue, Assignment, Date, Observation, Term};

pub fn verify<H: CanonicalHistory>(instance: impl Fn() -> H) {
    commitment_put_is_idempotent(instance());
    eligibility_put_is_idempotent(instance());
    event_put_is_idempotent(instance());
    append_event_is_atomic_compare_and_append(instance());
    event_of_returns_the_settling_event(instance());
    canonical_reads_expose_the_stored_record(instance());
}

pub fn commitment_put_is_idempotent<H: CanonicalHistory>(mut history: H) {
    let record = sample_commitment(1);

    assert_eq!(
        history.put_commitment(record.clone()),
        AppendOutcome::Admitted,
    );

    assert_eq!(
        history.put_commitment(record),
        AppendOutcome::AlreadyPresent,
    );
}

pub fn eligibility_put_is_idempotent<H: CanonicalHistory>(mut history: H) {
    let record = sample_eligibility(1);

    assert_eq!(
        history.put_eligibility(record.clone()),
        AppendOutcome::Admitted,
    );

    assert_eq!(
        history.put_eligibility(record),
        AppendOutcome::AlreadyPresent,
    );
}

pub fn event_put_is_idempotent<H: CanonicalHistory>(mut history: H) {
    let record = sample_event(CommitmentId::from([1; 32]), None, "Signed");

    assert!(matches!(
        history.append_event(record.clone()),
        Ok(AppendOutcome::Admitted)
    ));

    assert!(matches!(
        history.append_event(record),
        Ok(AppendOutcome::AlreadyPresent)
    ));
}

pub fn append_event_is_atomic_compare_and_append<H: CanonicalHistory>(mut history: H) {
    assert_eq!(history.head(), None, "an empty history has no head");

    let genesis = sample_event(CommitmentId::from([1; 32]), None, "Signed");
    let genesis_id = genesis.assertion().id();
    history.append_event(genesis).unwrap();
    assert_eq!(history.head(), Some(genesis_id));

    // An event built against the empty chain is stale now. The append is refused
    // and leaves no trace: not the head, not the commitment index, not the by-id
    // read — persisting and linking were one indivisible step.
    let stale_commitment = CommitmentId::from([2; 32]);
    let stale = sample_event(stale_commitment, None, "Paid");
    let stale_id = stale.assertion().id();
    assert!(history.append_event(stale).is_err(), "a stale head must be refused");
    assert_eq!(history.head(), Some(genesis_id));
    assert!(history.event_of(stale_commitment).is_none());
    assert!(history.event(stale_id).is_none());

    // Extending from the current head succeeds and moves it.
    let next = sample_event(CommitmentId::from([3; 32]), Some(genesis_id), "Signed");
    let next_id = next.assertion().id();
    history.append_event(next).unwrap();
    assert_eq!(history.head(), Some(next_id));
}

pub fn canonical_reads_expose_the_stored_record<H: CanonicalHistory>(mut history: H) {
    let commitment = sample_commitment(1);
    let commitment_id = commitment.assertion().id();
    let commitment_recorded_at = *commitment.recorded_at();
    history.put_commitment(commitment);

    let stored = history
        .canonical_commitment(commitment_id)
        .expect("a stored commitment is readable as a canonical record");
    assert_eq!(stored.assertion().id(), commitment_id);
    assert_eq!(*stored.recorded_at(), commitment_recorded_at);

    let event = sample_event(CommitmentId::from([9; 32]), None, "Signed");
    let event_id = event.assertion().id();
    let event_recorded_at = *event.recorded_at();
    history.append_event(event).unwrap();

    let stored = history
        .canonical_event(event_id)
        .expect("a stored event is readable as a canonical record");
    assert_eq!(*stored.recorded_at(), event_recorded_at);
}

pub fn event_of_returns_the_settling_event<H: CanonicalHistory>(mut history: H) {
    let commitment = CommitmentId::from([1; 32]);

    assert!(
        history.event_of(commitment).is_none(),
        "no event before one is put",
    );

    let event = sample_event(commitment, None, "Signed");
    let event_id = event.assertion().id();

    history.append_event(event).unwrap();

    assert_eq!(history.event_of(commitment).map(|e| e.id()), Some(event_id));

    assert!(
        history.event_of(CommitmentId::from([2; 32])).is_none(),
        "an unsettled commitment has no event",
    );
}

// ---------------------------------------------------------------------------
// Sample assertions: valid and distinct by `tag`.
// ---------------------------------------------------------------------------

fn date(y: i32, m: u8, d: u8) -> Date {
    Date::from_ymd(y, m, d).unwrap()
}

fn sample_commitment(tag: u8) -> Canonical<Commitment> {
    let commitment = Commitment::create(CommitmentInput {
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
    .unwrap();

    Canonical::new(commitment, date(2026, 7, 1)).unwrap()
}

fn sample_eligibility(tag: u8) -> Canonical<EligibilityAssignment> {
    let eligibility = EligibilityAssignment::create(EligibilityAssignmentInput {
        agent: AgentId::from([tag; 32]),
        roles: BTreeSet::from([RoleId::from([tag; 32])]),
        effective_from: date(2025, 1, 1),
    })
    .unwrap();

    Canonical::new(eligibility, date(2026, 7, 1)).unwrap()
}

fn sample_event(
    commitment: CommitmentId,
    previous: Option<EventId>,
    observation: &str,
) -> Canonical<Event> {
    let event = Event::create(EventInput {
        commitment_id: commitment,
        observation: Observation::new(observation).unwrap(),
        previous_event: previous,
        occurred_at: date(2026, 6, 1),
    })
    .unwrap();

    Canonical::new(event, date(2026, 7, 1)).unwrap()
}
