//! The `OnDueDate` hypothesis: every level along the punctual realization, not only the last.

use super::*;
use crate::engine::hermeneia::{Conflict, Hypothesis, ProjectionError};

struct Ledgered {
    knowledge: Fixture,
    ledger: Ledger,
    ids: Vec<CommitmentId>,
}
impl Ledgered {
    fn accumulate(&self, events: &[Event]) -> Accumulation {
        let mut accumulation = Accumulation::default();
        accumulation
            .absorb(&self.knowledge, &self.ids, events)
            .unwrap();
        accumulation
    }

    fn under(&self, hypothesis: Hypothesis, events: &[Event]) -> Vec<Conflict> {
        self.accumulate(events).conflicts(hypothesis).unwrap()
    }
}

/// `moves` is `(statement-picking closure, magnitude, due date)`, instantiated in order.
fn ledgered(ceiling: f64, moves: &[(bool, f64, Date)]) -> Ledgered {
    let mut knowledge = Fixture::default();
    let ledger = knowledge.ledger(Constraint::between(0.0, ceiling).unwrap());

    let ids = moves
        .iter()
        .map(|(credit, magnitude, due)| {
            let statement = if *credit { ledger.credit } else { ledger.debit };
            commit(
                &mut knowledge,
                statement,
                ledger.instance,
                ActionValue::value(*magnitude).unwrap(),
                *due,
                BTreeSet::new(),
            )
        })
        .collect();

    Ledgered {
        knowledge,
        ledger,
        ids,
    }
}

#[test]
fn a_sequence_that_ends_within_bounds_can_still_breach_along_the_way() {
    let l = ledgered(
        50.0,
        &[(true, 60.0, date(2026, 3, 31)), (false, 20.0, date(2026, 4, 30))],
    );

    assert!(
        l.under(Hypothesis::FinalState, &[]).is_empty(),
        "the level they end at is within bounds",
    );

    assert_eq!(
        l.under(Hypothesis::OnDueDate, &[]),
        vec![Conflict::OutOfBounds {
            instance: l.ledger.instance,
            level: 60.0,
        }],
        "the level they pass through is not",
    );
}

#[test]
fn a_settled_movement_lands_where_it_was_observed() {
    let l = ledgered(
        100.0,
        &[(true, 60.0, date(2026, 3, 31)), (false, 20.0, date(2026, 4, 30))],
    );

    assert!(
        l.under(Hypothesis::OnDueDate, &[]).is_empty(),
        "credit then debit: 60 then 40, both within bounds",
    );

    let observed_early = Event::create(EventInput {
        commitment_id: l.ids[1],
        observation: obs("Delivered"),
        previous_event: None,
        occurred_at: date(2026, 2, 1),
    })
    .unwrap();

    assert_eq!(
        l.under(Hypothesis::OnDueDate, &[observed_early]),
        vec![Conflict::OutOfBounds {
            instance: l.ledger.instance,
            level: -20.0,
        }],
        "observed in February, the debit lands before the credit and takes the level below zero",
    );
}

/// Movements sharing an instant are unordered among themselves, so the verdict must hold for
/// every arrangement of them.
#[test]
fn simultaneous_movements_are_judged_by_every_arrangement() {
    let due = date(2026, 3, 31);
    let l = ledgered(100.0, &[(true, 60.0, due), (false, 20.0, due)]);

    assert!(
        l.under(Hypothesis::FinalState, &[]).is_empty(),
        "they end at 40",
    );

    assert_eq!(
        l.under(Hypothesis::OnDueDate, &[]),
        vec![Conflict::OutOfBounds {
            instance: l.ledger.instance,
            level: -20.0,
        }],
        "debit first reaches -20, so the group is not safe however the credit is ordered",
    );
}

#[test]
fn ordering_the_same_movements_removes_the_breach() {
    let l = ledgered(
        100.0,
        &[(true, 60.0, date(2026, 3, 30)), (false, 20.0, date(2026, 3, 31))],
    );

    assert!(l.under(Hypothesis::OnDueDate, &[]).is_empty());
}

#[test]
fn a_cancelled_movement_never_enters_the_sequence() {
    let due = date(2026, 3, 31);
    let l = ledgered(100.0, &[(true, 60.0, due), (false, 20.0, due)]);

    assert!(
        l.under(Hypothesis::OnDueDate, &[settles(l.ids[1], "Cancelled")])
            .is_empty(),
        "with the debit cancelled the credit stands alone, and 60 is within bounds",
    );
}

/// Deciding a group means asking whether every arrangement of it holds, and the levels its
/// arrangements can produce are the sums of its subsets. Past a point that stops being worth
/// enumerating, and the group is refused rather than approximated.
#[test]
fn a_group_too_large_to_decide_is_refused_rather_than_approximated() {
    let due = date(2026, 3, 31);
    let moves: Vec<_> = (1..=17)
        .map(|n| (true, f64::from(n), due))
        .collect();
    let l = ledgered(1000.0, &moves);

    assert!(matches!(
        l.accumulate(&[]).conflicts(Hypothesis::OnDueDate),
        Err(ProjectionError::TooManySimultaneousMovements { count: 17, .. })
    ));
}
