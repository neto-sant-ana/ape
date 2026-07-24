//! Envelope-level tests: the recorded_at invariant, by construction.

use super::*;

#[test]
fn an_event_recorded_before_it_occurred_is_rejected() {
    let observed = event(commitment(1).id(), None, "Signed"); // occurs 2026-06-01

    assert!(matches!(
        Canonical::new(observed, date(2026, 5, 31)),
        Err(CanonError::RecordedTooEarly { .. })
    ));
}

#[test]
fn a_commitment_recorded_before_it_was_committed_is_rejected() {
    assert!(matches!(
        Canonical::new(commitment(1), date(2025, 12, 31)), // committed 2026-01-01
        Err(CanonError::RecordedTooEarly { .. })
    ));
}

#[test]
fn an_eligibility_may_be_recorded_before_it_takes_effect() {
    assert!(Canonical::new(eligibility(1), date(2020, 1, 1)).is_ok());
}
