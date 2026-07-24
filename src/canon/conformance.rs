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
    EligibilityAssignmentInput, Event, EventId, EventInput, ResourceInstanceId, RoleId, StatementId,
};

use crate::kernel::value_objects::{ActionValue, Assignment, Date, Observation, Term};

pub fn verify<H: CanonicalHistory>(instance: impl Fn() -> H) {
    commitment_put_is_idempotent(instance());
    eligibility_put_is_idempotent(instance());
    event_put_is_idempotent(instance());
    advance_head_is_compare_and_swap(instance());
    a_stored_event_left_unlinked_is_harmless(instance());
    event_of_returns_the_settling_event(instance());
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

    assert_eq!(history.put_event(record.clone()), AppendOutcome::Admitted);

    assert_eq!(history.put_event(record), AppendOutcome::AlreadyPresent);
}

pub fn advance_head_is_compare_and_swap<H: CanonicalHistory>(mut history: H) {
    let commitment = CommitmentId::from([1; 32]);

    assert_eq!(history.head(), None, "a instance history has no head");

    let genesis = sample_event(commitment, None, "Signed");
    let genesis_id = genesis.assertion().id();

    history.put_event(genesis);

    history
        .advance_head(None, genesis_id)
        .expect("genesis extends the empty chain");

    assert_eq!(history.head(), Some(genesis_id));

    let rival = sample_event(commitment, None, "Paid").assertion().id();

    assert!(
        history.advance_head(None, rival).is_err(),
        "a stale `expected` must be refused",
    );

    assert_eq!(history.head(), Some(genesis_id));

    let next = sample_event(commitment, Some(genesis_id), "Paid");
    let next_id = next.assertion().id();

    history.put_event(next);

    history
        .advance_head(Some(genesis_id), next_id)
        .expect("extends the current head");

    assert_eq!(history.head(), Some(next_id));
}

pub fn a_stored_event_left_unlinked_is_harmless<H: CanonicalHistory>(mut history: H) {
    let commitment = CommitmentId::from([1; 32]);
    let genesis = sample_event(commitment, None, "Signed");
    let genesis_id = genesis.assertion().id();

    history.put_event(genesis);
    history.advance_head(None, genesis_id).unwrap();

    let orphan = sample_event(commitment, None, "Paid");
    let orphan_id = orphan.assertion().id();

    assert_eq!(history.put_event(orphan), AppendOutcome::Admitted);
    assert!(history.advance_head(None, orphan_id).is_err());
    assert_eq!(history.head(), Some(genesis_id));
}

pub fn event_of_returns_the_settling_event<H: CanonicalHistory>(mut history: H) {
    let commitment = CommitmentId::from([1; 32]);

    assert!(
        history.event_of(commitment).is_none(),
        "no event before one is put",
    );

    let event = sample_event(commitment, None, "Signed");
    let event_id = event.assertion().id();

    history.put_event(event);

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
