//! Concurrency tests for the Canon over the reference [`InMemoryHistory`] adapter.

use std::sync::{Arc, Barrier};

use super::*;

use crate::canon::{CanonError, EventSubmission, InMemoryHistory};

#[test]
fn the_in_memory_history_conforms() {
    crate::canon::conformance::verify(InMemoryHistory::default);
    crate::canon::conformance::verify_thread_safe(InMemoryHistory::default());
}

/// Two threads race to settle the same commitment with different, mutually
/// exclusive observations. Settle-once must hold every time: exactly one event
/// settles the commitment, the conflicting one is refused.
#[test]
fn concurrent_settlement_of_one_commitment_settles_it_once() {
    for _ in 0..64 {
        let shared = InMemoryHistory::default();
        let commitment = seed_commitment(&mut Canon::new(shared.clone()));

        let barrier = Arc::new(Barrier::new(2));
        let observations = ["Signed", "Cancelled"];

        let racers: Vec<_> = observations
            .into_iter()
            .map(|observation| {
                let mut canon = Canon::new(shared.clone());
                let barrier = Arc::clone(&barrier);
                std::thread::spawn(move || {
                    let submission = || EventSubmission {
                        commitment_id: commitment,
                        observation: Observation::new(observation).unwrap(),
                        occurred_at: date(2026, 6, 1),
                    };

                    barrier.wait();

                    let mut outcome = canon.admit_event(submission(), date(2026, 7, 1));
                    let mut retries = 0;
                    while matches!(outcome, Err(CanonError::UnexpectedHead { .. })) && retries < 8 {
                        outcome = canon.admit_event(submission(), date(2026, 7, 1));
                        retries += 1;
                    }
                    outcome
                })
            })
            .collect();

        let outcomes: Vec<_> = racers.into_iter().map(|r| r.join().unwrap()).collect();

        let settled = outcomes.iter().filter(|o| o.is_ok()).count();
        let refused = outcomes
            .iter()
            .filter(|o| matches!(o, Err(CanonError::CommitmentAlreadySettled(_))))
            .count();
        assert_eq!(settled, 1, "exactly one event settles the commitment");
        assert_eq!(
            refused, 1,
            "the conflicting settlement is refused, even after re-admission"
        );

        let winner = outcomes
            .iter()
            .find_map(|o| o.as_ref().ok().copied())
            .unwrap();
        assert_eq!(
            shared.head(),
            Some(winner),
            "the head is the settling event"
        );
        assert_eq!(
            shared.event_of(commitment).map(|e| e.id()),
            Some(winner),
            "the commitment is settled by exactly that event",
        );
    }
}
