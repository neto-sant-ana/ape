//! Primitive-level tests: the dumb port (put-if-absent by id, head CAS).

use super::*;

#[test]
fn put_commitment_is_idempotent_by_id() {
    let mut history = MemoryHistory::default();
    let record = Canonical::new(commitment(1), date(2026, 7, 1)).unwrap();

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
    let record = Canonical::new(eligibility(1), date(2026, 7, 1)).unwrap();

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
    let record =
        Canonical::new(event(commitment(1).id(), None, "Signed"), date(2026, 7, 1)).unwrap();

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
    history.put_event(Canonical::new(genesis, date(2026, 7, 1)).unwrap());
    history.advance_head(None, genesis_id).unwrap();

    let orphan = event(commitment.id(), None, "Paid");
    let orphan_id = orphan.id();
    assert_eq!(
        history.put_event(Canonical::new(orphan, date(2026, 7, 2)).unwrap()),
        AppendOutcome::Admitted
    );
    assert!(history.advance_head(None, orphan_id).is_err());
    assert_eq!(history.head(), Some(genesis_id));
}
