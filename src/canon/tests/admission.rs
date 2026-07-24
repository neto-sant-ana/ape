//! Orchestrator-level tests: admission composes Axiom + envelope + primitives.

use super::*;
use crate::canon::{CanonError, EventSubmission};

#[test]
fn admits_a_valid_commitment_and_is_idempotent() {
    let g = graph();
    let input = commitment_input(&g.seeded);

    let mut canon = g.canon;
    let first = canon
        .admit_commitment(input.clone(), date(2026, 2, 1))
        .unwrap();
    let again = canon.admit_commitment(input, date(2026, 3, 1)).unwrap();

    assert_eq!(first, again);
}

#[test]
fn propagates_a_structural_rejection_from_the_axiom() {
    let g = graph();

    let input = CommitmentInput {
        assignment: Assignment::new(AgentId::from([99u8; 32]), [g.seeded.executor], [g.seeded.beneficiary])
            .unwrap(),
        ..commitment_input(&g.seeded)
    };

    let mut canon = g.canon;

    assert!(matches!(
        canon.admit_commitment(input, date(2026, 2, 1)),
        Err(CanonError::Axiom(_))
    ));
}

#[test]
fn admits_an_eligibility_declared_effective_in_the_future() {
    let g = graph();
    let agent = g.seeded.accountable;
    let role = g.seeded.actor_role;
    let mut canon = g.canon;

    assert!(
        canon
            .admit_eligibility(
                EligibilityAssignmentInput {
                    agent,
                    roles: BTreeSet::from([role]),
                    effective_from: date(2030, 1, 1),
                },
                date(2026, 1, 1),
            )
            .is_ok()
    );
}

#[test]
fn re_declaring_the_same_eligibility_is_idempotent() {
    let g = graph();
    let agent = g.seeded.accountable;
    let role = g.seeded.actor_role;
    let mut canon = g.canon;

    let first = canon
        .admit_eligibility(
            EligibilityAssignmentInput {
                agent,
                roles: BTreeSet::from([role]),
                effective_from: date(2025, 1, 1),
            },
            date(2025, 1, 1),
        )
        .unwrap();

    let again = canon
        .admit_eligibility(
            EligibilityAssignmentInput {
                agent,
                roles: BTreeSet::from([role]),
                effective_from: date(2025, 1, 1),
            },
            date(2025, 6, 1),
        )
        .unwrap();

    assert_eq!(first, again);
}

#[test]
fn rejects_a_conflicting_eligibility_at_the_same_instant() {
    let g = graph();
    let agent = g.seeded.accountable;
    let role = g.seeded.actor_role;
    let mut canon = g.canon;

    canon
        .admit_eligibility(
            EligibilityAssignmentInput {
                agent,
                roles: BTreeSet::from([role]),
                effective_from: date(2025, 1, 1),
            },
            date(2025, 1, 1),
        )
        .unwrap();

    assert!(matches!(
        canon.admit_eligibility(
            EligibilityAssignmentInput {
                agent,
                roles: BTreeSet::new(),
                effective_from: date(2025, 1, 1),
            },
            date(2025, 1, 1),
        ),
        Err(CanonError::ConflictingEligibility { .. })
    ));
}

#[test]
fn admits_a_later_eligibility_for_the_same_agent() {
    let g = graph();
    let agent = g.seeded.accountable;
    let role = g.seeded.actor_role;
    let mut canon = g.canon;

    canon
        .admit_eligibility(
            EligibilityAssignmentInput {
                agent,
                roles: BTreeSet::from([role]),
                effective_from: date(2025, 1, 1),
            },
            date(2025, 1, 1),
        )
        .unwrap();

    assert!(
        canon
            .admit_eligibility(
                EligibilityAssignmentInput {
                    agent,
                    roles: BTreeSet::new(),
                    effective_from: date(2026, 1, 1),
                },
                date(2026, 1, 1),
            )
            .is_ok()
    );
}

#[test]
fn admits_events_extending_the_chain() {
    let g = graph();
    let first_input = commitment_input(&g.seeded);
    let second_input = CommitmentInput {
        term: Term::new(date(2026, 2, 1), date(2027, 1, 1)).unwrap(),
        ..commitment_input(&g.seeded)
    };
    let mut canon = g.canon;
    let first_commitment = canon
        .admit_commitment(first_input, date(2026, 2, 1))
        .unwrap();
    let second_commitment = canon
        .admit_commitment(second_input, date(2026, 3, 1))
        .unwrap();
    assert_ne!(first_commitment, second_commitment);

    assert_eq!(canon.history().head(), None);

    let first = canon
        .admit_event(
            EventSubmission {
                commitment_id: first_commitment,
                observation: obs("Signed"),
                occurred_at: date(2026, 6, 1),
            },
            date(2026, 6, 2),
        )
        .unwrap();
    assert_eq!(canon.history().head(), Some(first));

    let second = canon
        .admit_event(
            EventSubmission {
                commitment_id: second_commitment,
                observation: obs("Signed"),
                occurred_at: date(2026, 7, 1),
            },
            date(2026, 7, 2),
        )
        .unwrap();
    assert_eq!(canon.history().head(), Some(second));
    assert_ne!(first, second);
}

#[test]
fn re_admitting_the_same_event_fact_is_idempotent() {
    let g = graph();
    let input = commitment_input(&g.seeded);
    let mut canon = g.canon;
    let commitment_id = canon.admit_commitment(input, date(2026, 2, 1)).unwrap();

    let first = canon
        .admit_event(
            EventSubmission {
                commitment_id,
                observation: obs("Signed"),
                occurred_at: date(2026, 6, 1),
            },
            date(2026, 6, 2),
        )
        .unwrap();
    let head_after = canon.history().head();

    let again = canon
        .admit_event(
            EventSubmission {
                commitment_id,
                observation: obs("Signed"),
                occurred_at: date(2026, 6, 1),
            },
            date(2026, 6, 3),
        )
        .unwrap();
    assert_eq!(first, again);
    assert_eq!(canon.history().head(), head_after);
}

#[test]
fn rejects_a_conflicting_settlement_of_an_already_settled_commitment() {
    let g = graph();
    let input = commitment_input(&g.seeded);
    let mut canon = g.canon;
    let commitment_id = canon.admit_commitment(input, date(2026, 2, 1)).unwrap();

    canon
        .admit_event(
            EventSubmission {
                commitment_id,
                observation: obs("Signed"),
                occurred_at: date(2026, 6, 1),
            },
            date(2026, 6, 2),
        )
        .unwrap();

    assert!(matches!(
        canon.admit_event(
            EventSubmission {
                commitment_id,
                observation: obs("Cancelled"),
                occurred_at: date(2026, 7, 1),
            },
            date(2026, 7, 2),
        ),
        Err(CanonError::CommitmentAlreadySettled(_))
    ));
}
