//! A punctual hypothesis can contradict the dependencies it is asked about.
//!
//! A dependency must settle before its dependent. Positioning each commitment by its own date can
//! place a dependent ahead of what it waits on, and the levels such a sequence produces describe a
//! realization that cannot occur.

use super::*;
use crate::engine::hermeneia::{Conflict, Hypothesis};

struct Ordered {
    knowledge: Fixture,
    first: CommitmentId,
    second: CommitmentId,
}
impl Ordered {
    fn under(&self, hypothesis: Hypothesis, events: &[Event]) -> Vec<Conflict> {
        let mut accumulation = Accumulation::default();
        accumulation
            .absorb(&self.knowledge, &[self.first, self.second], events)
            .unwrap();
        accumulation
            .feasibility_under(hypothesis)
            .unwrap()
            .conflicts()
            .to_vec()
    }
}

fn ordered(first_due: Date, second_due: Date) -> Ordered {
    let mut knowledge = Fixture::default();
    let ledger = knowledge.ledger(Constraint::between(0.0, 100.0).unwrap());

    let first = commit(
        &mut knowledge,
        ledger.credit,
        ledger.instance,
        ActionValue::value(10.0).unwrap(),
        first_due,
        BTreeSet::new(),
    );
    let second = commit(
        &mut knowledge,
        ledger.credit,
        ledger.instance,
        ActionValue::value(10.0).unwrap(),
        second_due,
        BTreeSet::from([first]),
    );

    Ordered {
        knowledge,
        first,
        second,
    }
}

#[test]
fn a_dependency_due_after_its_dependent_cannot_be_realized_punctually() {
    let o = ordered(date(2026, 4, 30), date(2026, 3, 31));

    let violation = vec![Conflict::PunctualDependencyViolation {
        dependency: o.first,
        dependent: o.second,
    }];

    assert_eq!(o.under(Hypothesis::OnDueDateNet, &[]), violation);
    assert_eq!(o.under(Hypothesis::OnDueDateInAnyOrder, &[]), violation);

    assert!(
        o.under(Hypothesis::FinalState, &[]).is_empty(),
        "quantifying over no order, there is no order to contradict",
    );
}

#[test]
fn the_contradiction_is_reported_even_though_every_level_fits() {
    let o = ordered(date(2026, 4, 30), date(2026, 3, 31));

    let found = o.under(Hypothesis::OnDueDateInAnyOrder, &[]);

    assert!(
        !found
            .iter()
            .any(|conflict| matches!(conflict, Conflict::OutOfBounds { .. })),
        "10 then 20 is comfortably inside the ceiling; nothing about levels is wrong",
    );
    assert_eq!(found.len(), 1);
}

#[test]
fn ordering_the_deadlines_correctly_leaves_nothing_to_report() {
    let o = ordered(date(2026, 3, 31), date(2026, 4, 30));

    assert!(o.under(Hypothesis::OnDueDateInAnyOrder, &[]).is_empty());
}

/// A settled dependency is positioned by the fact, so observing it *after* its dependent's
/// deadline refutes that dependent's punctuality just as an inverted deadline would.
#[test]
fn a_dependency_observed_after_its_dependents_deadline_violates_it_too() {
    let o = ordered(date(2026, 3, 31), date(2026, 4, 30));

    let observed_late = Event::create(EventInput {
        commitment_id: o.first,
        observation: obs("Delivered"),
        previous_event: None,
        occurred_at: date(2026, 5, 31),
    })
    .unwrap();

    assert_eq!(
        o.under(Hypothesis::OnDueDateInAnyOrder, &[observed_late]),
        vec![Conflict::PunctualDependencyViolation {
            dependency: o.first,
            dependent: o.second,
        }],
        "the dependency landed in May; the dependent's April deadline can no longer be met",
    );
}

/// The mirror of the case above, and the reason an inverted deadline is not an invalidity: a
/// dependency fulfilled *ahead* of its deadline dissolves the inversion.
#[test]
fn a_dependency_fulfilled_early_resolves_an_inverted_deadline() {
    let o = ordered(date(2026, 4, 30), date(2026, 3, 31));

    assert!(
        !o.under(Hypothesis::OnDueDateInAnyOrder, &[]).is_empty(),
        "on the deadlines alone the dependent cannot be punctual",
    );

    assert!(
        o.under(
            Hypothesis::OnDueDateInAnyOrder,
            &[settles(o.first, "Delivered")]
        )
        .is_empty(),
        "fulfilled in February, the dependency no longer stands in its dependent's way",
    );
}

#[test]
fn a_fulfilled_dependent_is_not_declared_unrealizable_punctually() {
    let o = ordered(date(2026, 4, 30), date(2026, 3, 31));

    assert!(
        o.under(
            Hypothesis::OnDueDateInAnyOrder,
            &[settles(o.second, "Delivered")]
        )
        .is_empty(),
        "the dependent happened; only its still-open dependency remains, and that is punctual",
    );
}

/// A cancelled dependency does not produce an ordering conflict, because its dependent is doomed
/// outright and that is reported first.
#[test]
fn a_cancelled_dependency_is_reported_as_doom_rather_than_disorder() {
    let o = ordered(date(2026, 4, 30), date(2026, 3, 31));

    assert_eq!(
        o.under(
            Hypothesis::OnDueDateInAnyOrder,
            &[settles(o.first, "Cancelled")]
        ),
        vec![Conflict::Unrealizable(o.second)],
    );
}
