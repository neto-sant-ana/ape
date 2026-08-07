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
//!
//! An adapter that additionally claims to be thread-safe proves it with
//! [`verify_thread_safe`], which drives concurrent contention against the atomic
//! compare-and-append. A thread-safe adapter is a `Clone` handle whose clones share
//! one backing store; the suite drives one clone per thread.

use std::collections::BTreeSet;
use std::sync::{Arc, Barrier};

use crate::canon::{
    AppendOutcome, CanonError, Canonical, CanonicalHistory, record::RecordableAfter,
};

use crate::kernel::entities::{
    Agent, AgentId, AgentInput, Commitment, CommitmentId, CommitmentInput, EligibilityAssignment,
    EligibilityAssignmentInput, Event, EventId, EventInput, ResourceInstanceId, Role, RoleId,
    RoleInput, StatementId,
};

use crate::kernel::value_objects::{
    ActionValue, AgentKind, Assignment, Date, Identifier, Observation, Term,
};

pub fn verify<H: CanonicalHistory>(instance: impl Fn() -> H) {
    commitment_put_is_idempotent(instance());
    eligibility_put_is_idempotent(instance());
    event_put_is_idempotent(instance());
    append_event_is_atomic_compare_and_append(instance());
    event_of_returns_the_settling_event(instance());
    canonical_reads_expose_the_stored_record(instance());
    recording_is_monotonic(instance());
    append_event_refuses_a_back_dated_record(instance());
    recording_is_shared_across_families(instance());
    head_as_of_resolves_the_cut(instance());
}

/// The head an instant addresses, which an adapter may answer however it stores the chain.
///
/// This is the one read whose answer is a *rule* rather than a lookup, so it is the one an
/// adapter can implement plausibly and wrongly. Four questions pin it: before anything was
/// recorded there is no head; at an instant, the head is the last Event recorded by it; after
/// everything, it is the last Event of all; and where several Events share the instant, it is the
/// last of them in chain order rather than the first.
pub fn head_as_of_resolves_the_cut<H: CanonicalHistory>(mut history: H) {
    let before = date(2026, 6, 30);
    let first_day = date(2026, 7, 1);
    let second_day = date(2026, 7, 2);
    let after = date(2026, 7, 3);

    assert_eq!(
        history.head_as_of(&after),
        None,
        "an empty history has no head at any instant"
    );

    let genesis = sample_event(CommitmentId::from([1; 32]), None, "Signed");
    let genesis_id = genesis.assertion().id();
    history.append_event(genesis).unwrap();

    // Sharing the recording instant of the event before it: the cut resolves to the later one.
    let same_day = sample_event(CommitmentId::from([2; 32]), Some(genesis_id), "Signed");
    let same_day_id = same_day.assertion().id();
    history.append_event(same_day).unwrap();

    let later = rerecorded(
        &sample_event(CommitmentId::from([3; 32]), Some(same_day_id), "Signed"),
        second_day,
    );
    let later_id = later.assertion().id();
    history.append_event(later).unwrap();

    assert_eq!(
        history.head_as_of(&before),
        None,
        "no Event was recorded by an instant preceding every record"
    );
    assert_eq!(
        history.head_as_of(&first_day),
        Some(same_day_id),
        "an instant shared by several Events resolves to the last of them in chain order"
    );
    assert_eq!(
        history.head_as_of(&second_day),
        Some(later_id),
        "the head at an instant is the last Event recorded by it"
    );
    assert_eq!(
        history.head_as_of(&after),
        Some(later_id),
        "an instant later than every record resolves to the whole chain"
    );
    assert_eq!(
        history.head_as_of(&after),
        history.head(),
        "and agrees with the current head"
    );
}

/// One watermark governs every family, the chain included.
///
/// An adapter that gave each family its own transaction — a table per entity kind,
/// each with its own watermark — would satisfy every other case in this suite while
/// letting a commitment be back-dated behind a role. The guarantee is global or it is
/// nothing, so this walks both directions: a put is checked against an instant a
/// different family established, and against one the chain established.
pub fn recording_is_shared_across_families<H: CanonicalHistory>(mut history: H) {
    history
        .put_role(rerecorded(&sample_role(1), date(2026, 7, 10)))
        .unwrap();
    assert_eq!(history.recorded_through(), Some(date(2026, 7, 10)));

    let commitment = rerecorded(&sample_commitment(1), date(2026, 7, 9));
    let commitment_id = commitment.assertion().id();
    assert!(
        matches!(
            history.put_commitment(commitment),
            Err(CanonError::RecordedOutOfOrder { .. })
        ),
        "a commitment cannot be back-dated behind a role",
    );
    assert!(history.commitment(commitment_id).is_none());

    let event = rerecorded(
        &sample_event(CommitmentId::from([1; 32]), None, "Signed"),
        date(2026, 7, 9),
    );
    let event_id = event.assertion().id();
    assert!(
        matches!(
            history.append_event(event),
            Err(CanonError::RecordedOutOfOrder { .. })
        ),
        "the chain answers to the same watermark as the puts",
    );
    assert!(history.event(event_id).is_none());
    assert_eq!(history.head(), None);

    history
        .append_event(rerecorded(
            &sample_event(CommitmentId::from([1; 32]), None, "Signed"),
            date(2026, 7, 11),
        ))
        .unwrap();
    assert_eq!(history.recorded_through(), Some(date(2026, 7, 11)));

    let agent = rerecorded(&sample_agent(1), date(2026, 7, 10));
    let agent_id = agent.assertion().id();
    assert!(
        matches!(
            history.put_agent(agent),
            Err(CanonError::RecordedOutOfOrder { .. })
        ),
        "an agent cannot be back-dated behind an admitted event",
    );
    assert!(history.agent(agent_id).is_none());
}

pub fn commitment_put_is_idempotent<H: CanonicalHistory>(mut history: H) {
    let record = sample_commitment(1);

    assert!(matches!(
        history.put_commitment(record.clone()),
        Ok(AppendOutcome::Admitted)
    ));

    assert!(matches!(
        history.put_commitment(record),
        Ok(AppendOutcome::AlreadyPresent)
    ));
}

pub fn eligibility_put_is_idempotent<H: CanonicalHistory>(mut history: H) {
    let record = sample_eligibility(1);

    assert!(matches!(
        history.put_eligibility(record.clone()),
        Ok(AppendOutcome::Admitted)
    ));

    assert!(matches!(
        history.put_eligibility(record),
        Ok(AppendOutcome::AlreadyPresent)
    ));
}

/// Recording is monotonic: an assertion may not be admitted before knowledge already
/// admitted, which is what makes a past interpretation reproducible.
/// Monotonic means non-decreasing, so admitting at the watermark itself is allowed.
pub fn recording_is_monotonic<H: CanonicalHistory>(mut history: H) {
    assert_eq!(
        history.recorded_through(),
        None,
        "an empty history is recorded through no instant",
    );

    let admitted = rerecorded(&sample_commitment(1), date(2026, 7, 10));
    history.put_commitment(admitted.clone()).unwrap();
    assert_eq!(history.recorded_through(), Some(date(2026, 7, 10)));

    let back_dated = rerecorded(&sample_eligibility(1), date(2026, 7, 9));
    assert!(
        matches!(
            history.put_eligibility(back_dated.clone()),
            Err(CanonError::RecordedOutOfOrder { .. })
        ),
        "a record predating the watermark is refused",
    );
    assert!(
        history
            .eligibilities_of(*back_dated.assertion().agent())
            .is_empty(),
        "a refused put persists nothing",
    );
    assert_eq!(
        history.recorded_through(),
        Some(date(2026, 7, 10)),
        "a refused put does not move the watermark",
    );

    let same_instant = rerecorded(&sample_eligibility(2), date(2026, 7, 10));
    assert!(
        matches!(
            history.put_eligibility(same_instant),
            Ok(AppendOutcome::Admitted)
        ),
        "monotonic is non-decreasing: the watermark itself is admissible",
    );

    let re_put = rerecorded(&admitted, date(2026, 7, 1));
    assert!(
        matches!(
            history.put_commitment(re_put),
            Ok(AppendOutcome::AlreadyPresent)
        ),
        "presence is settled before the watermark, so a re-put stays a no-op",
    );
}

/// The chain obeys the watermark too, and reports it *before* a stale head,
/// so the caller can distinct a possible new admission from an ever failure.
pub fn append_event_refuses_a_back_dated_record<H: CanonicalHistory>(mut history: H) {
    let genesis = rerecorded(
        &sample_event(CommitmentId::from([1; 32]), None, "Signed"),
        date(2026, 7, 10),
    );
    let genesis_id = genesis.assertion().id();
    history.append_event(genesis).unwrap();

    let back_dated = rerecorded(
        &sample_event(CommitmentId::from([2; 32]), Some(genesis_id), "Signed"),
        date(2026, 7, 9),
    );
    let back_dated_id = back_dated.assertion().id();
    assert!(
        matches!(
            history.append_event(back_dated),
            Err(CanonError::RecordedOutOfOrder { .. })
        ),
        "an event predating the watermark is refused",
    );
    assert!(
        history.event(back_dated_id).is_none(),
        "a refused append persists no event",
    );
    assert_eq!(history.head(), Some(genesis_id), "the head does not move");

    // Stale head *and* back-dated: the unrecoverable condition is the one reported.
    let both = rerecorded(
        &sample_event(CommitmentId::from([3; 32]), None, "Signed"),
        date(2026, 7, 9),
    );
    assert!(
        matches!(
            history.append_event(both),
            Err(CanonError::RecordedOutOfOrder { .. })
        ),
        "a back-dated instant outranks a stale head, which retrying could not fix",
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
    assert!(
        history.append_event(stale).is_err(),
        "a stale head must be refused"
    );
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
    history.put_commitment(commitment).unwrap();

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

/// The opt-in contention suite for an adapter that declares itself thread-safe.
///
/// `instance` must build a `Clone` handle whose clones share one backing store; each
/// thread drives its own clone. It is a factory rather than a single handle because
/// the races below are run repeatedly against a fresh history, so that both
/// interleavings are actually exercised.
pub fn verify_thread_safe<H>(instance: impl Fn() -> H)
where
    H: CanonicalHistory + Clone + Send + 'static,
{
    append_event_serializes_under_contention(instance());
    recording_serializes_across_families(&instance);
}

type Attempt = (Option<Date>, Result<AppendOutcome, CanonError>);

type Racer<H> = Box<dyn FnOnce(H) -> Attempt + Send>;

/// Enough rounds for a violating interleaving to surface. A concurrency test samples
/// the schedule, so this detects a broken adapter rather than proving a correct one.
const ROUNDS: u16 = 256;

fn racer<H, Admit>(recorded_at: Date, admit: Admit) -> (Date, Racer<H>)
where
    H: CanonicalHistory + Clone + Send + 'static,
    Admit: FnOnce(&mut H) -> Result<AppendOutcome, CanonError> + Send + 'static,
{
    (
        recorded_at,
        Box::new(move |mut history: H| {
            let observed = history.recorded_through();
            (observed, admit(&mut history))
        }),
    )
}

/// Admissions across every family — several `put_*` and one append — race from the
/// same empty watermark, each carrying a different instant, the later ones ahead of the
/// earlier ones.
///
/// Whatever order the adapter serializes them into, two properties must survive:
///
/// - the watermark ends at the latest instant it admitted, never below it;
/// - no record was admitted by a thread that had already seen the watermark pass it.
pub fn recording_serializes_across_families<H>(instance: &impl Fn() -> H)
where
    H: CanonicalHistory + Clone + Send + 'static,
{
    for _ in 0..ROUNDS {
        let history = instance();

        let (latest, earliest) = (date(2026, 7, 10), date(2026, 7, 6));
        let racers = vec![
            racer(latest, |h: &mut H| {
                h.put_role(rerecorded(&sample_role(1), date(2026, 7, 10)))
            }),
            racer(date(2026, 7, 9), |h: &mut H| {
                h.put_agent(rerecorded(&sample_agent(1), date(2026, 7, 9)))
            }),
            racer(date(2026, 7, 8), |h: &mut H| {
                h.put_commitment(rerecorded(&sample_commitment(1), date(2026, 7, 8)))
            }),
            racer(date(2026, 7, 7), |h: &mut H| {
                h.append_event(rerecorded(
                    &sample_event(CommitmentId::from([7; 32]), None, "Signed"),
                    date(2026, 7, 7),
                ))
            }),
            racer(earliest, |h: &mut H| {
                h.put_eligibility(rerecorded(&sample_eligibility(1), date(2026, 7, 6)))
            }),
        ];

        let gate = Arc::new(Barrier::new(racers.len()));
        let running: Vec<_> = racers
            .into_iter()
            .map(|(recorded_at, admit)| {
                let handle = history.clone();
                let gate = Arc::clone(&gate);
                std::thread::spawn(move || {
                    gate.wait();
                    (recorded_at, admit(handle))
                })
            })
            .collect();

        let attempts: Vec<_> = running.into_iter().map(|r| r.join().unwrap()).collect();

        let mut admitted_through = None;
        for (recorded_at, (observed, outcome)) in &attempts {
            match outcome {
                Ok(AppendOutcome::Admitted) => {
                    if let Some(observed) = observed {
                        assert!(
                            observed.up_to(recorded_at),
                            "admitted at {recorded_at:?} after observing a watermark of {observed:?}",
                        );
                    }
                    admitted_through = admitted_through.max(Some(*recorded_at));
                }
                Err(CanonError::RecordedOutOfOrder { .. }) => {}
                other => panic!("unexpected outcome at {recorded_at:?}: {other:?}"),
            }
        }

        assert_eq!(
            history.recorded_through(),
            admitted_through,
            "the watermark ends at the latest instant admitted, never below it",
        );
        assert_eq!(
            history.recorded_through(),
            Some(latest),
            "the latest record cannot be refused: nothing here can outrun it",
        );
        assert!(
            earliest.up_to(&latest),
            "the racers must span instants for the race to mean anything",
        );
    }
}

/// Many threads race to extend the same head; the compare-and-append must serialize
/// them: exactly one advances the head, and every loser is refused with
/// [`CanonError::UnexpectedHead`] leaving no trace — no event by id, no commitment
/// index entry. This is what proves the append is atomic under real threads, not
/// merely a sequential check-then-write with a window a second thread can slip into.
pub fn append_event_serializes_under_contention<H>(history: H)
where
    H: CanonicalHistory + Clone + Send + 'static,
{
    let genesis = sample_event(CommitmentId::from([0; 32]), None, "Signed");
    let genesis_id = genesis.assertion().id();
    history.clone().append_event(genesis).unwrap();

    const CONTENDERS: u8 = 8;
    let racers: Vec<_> = (1..=CONTENDERS)
        .map(|tag| {
            let mut adapter = history.clone();
            std::thread::spawn(move || {
                let commitment = CommitmentId::from([tag; 32]);
                let event = sample_event(commitment, Some(genesis_id), "Signed");
                let event_id = event.assertion().id();
                (commitment, event_id, adapter.append_event(event))
            })
        })
        .collect();

    let outcomes: Vec<_> = racers.into_iter().map(|r| r.join().unwrap()).collect();

    let winners = outcomes
        .iter()
        .filter(|(_, _, outcome)| matches!(outcome, Ok(AppendOutcome::Admitted)))
        .count();
    assert_eq!(winners, 1, "exactly one racer advances the head");

    for (commitment, event_id, outcome) in &outcomes {
        match outcome {
            Ok(AppendOutcome::Admitted) => {
                assert_eq!(
                    history.head(),
                    Some(*event_id),
                    "the head is the sole winner's event"
                );
            }
            Err(CanonError::UnexpectedHead { .. }) => {
                assert!(
                    history.event(*event_id).is_none(),
                    "a refused append persists no event",
                );
                assert!(
                    history.event_of(*commitment).is_none(),
                    "a refused append indexes no commitment",
                );
            }
            other => panic!("unexpected outcome under contention: {other:?}"),
        }
    }
}

// ---------------------------------------------------------------------------
// Sample assertions: valid and distinct by `tag`.
// ---------------------------------------------------------------------------

fn date(y: i32, m: u8, d: u8) -> Date {
    Date::from_ymd(y, m, d).unwrap()
}

/// The same assertion — and therefore the same content-addressed id — carrying a
/// different recording instant, which is what the watermark cases vary.
fn rerecorded<T: Clone + RecordableAfter>(record: &Canonical<T>, at: Date) -> Canonical<T> {
    Canonical::new(record.assertion().clone(), at).unwrap()
}

fn sample_role(tag: u8) -> Canonical<Role> {
    let role = Role::create(RoleInput {
        label: Identifier::new(format!("role-{tag}")).unwrap(),
    })
    .unwrap();

    Canonical::new(role, date(2026, 7, 1)).unwrap()
}

fn sample_agent(tag: u8) -> Canonical<Agent> {
    let agent = Agent::create(AgentInput {
        label: Identifier::new(format!("agent-{tag}")).unwrap(),
        kind: AgentKind::Company,
    })
    .unwrap();

    Canonical::new(agent, date(2026, 7, 1)).unwrap()
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
