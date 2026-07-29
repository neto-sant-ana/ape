//! Resource levels against their constraints, under the `FinalState` hypothesis.

use super::*;
use crate::engine::hermeneia::{Conflict, Hypothesis};

fn conflicts(accumulation: &Accumulation) -> Vec<Conflict> {
    accumulation.conflicts(Hypothesis::FinalState).unwrap()
}

fn bounded(ceiling: f64) -> Constraint {
    Constraint::between(0.0, ceiling).unwrap()
}

#[test]
fn a_decreasing_effect_moves_the_level_the_other_way() {
    let b = basket(Effect::Decrease, bounded(100.0));

    assert_eq!(
        conflicts(&b.accumulate(&[])),
        vec![Conflict::OutOfBounds {
            instance: b.instance,
            level: -160.0,
        }],
        "four decreases of 40 fall below the floor of zero, not above the ceiling",
    );
}

#[test]
fn movements_that_together_leave_the_bounds_are_reported() {
    let b = basket(Effect::Increase, bounded(100.0));

    assert_eq!(
        conflicts(&b.accumulate(&[])),
        vec![Conflict::OutOfBounds {
            instance: b.instance,
            level: 160.0,
        }],
    );
}

#[test]
fn movements_that_together_fit_report_nothing() {
    let b = basket(Effect::Increase, bounded(200.0));

    assert!(conflicts(&b.accumulate(&[])).is_empty());
}

#[test]
fn a_cancelled_movement_never_lands() {
    let b = basket(Effect::Increase, bounded(150.0));

    assert_eq!(
        conflicts(&b.accumulate(&[])),
        vec![Conflict::OutOfBounds {
            instance: b.instance,
            level: 160.0,
        }],
        "all four together overshoot the ceiling",
    );

    assert!(
        conflicts(&b.accumulate(&[settles(b.ids[1], "Cancelled")])).is_empty(),
        "with one cancelled, the three that remain fit",
    );
}

#[test]
fn a_fulfilled_movement_counts_like_an_assumed_one() {
    let b = basket(Effect::Increase, bounded(100.0));

    assert_eq!(
        conflicts(&b.accumulate(&[settles(b.ids[1], "Delivered")])),
        vec![Conflict::OutOfBounds {
            instance: b.instance,
            level: 160.0,
        }],
    );
}

#[test]
fn an_open_commitment_behind_a_cancelled_dependency_precedes_any_level() {
    let b = basket(Effect::Increase, bounded(100.0));
    let found = conflicts(&b.accumulate(&[settles(b.ids[0], "Cancelled")]));

    assert_eq!(
        found,
        vec![Conflict::Unrealizable(b.ids[3])],
        "only the doomed dependent — not the cancelled commitment, and no level, \
         even though the movements that remain would overshoot the ceiling",
    );
}

#[test]
fn a_discrete_commitment_moves_no_level() {
    let g = guide();

    assert!(conflicts(&g.accumulate(&[])).is_empty());
    assert!(conflicts(&g.accumulate(&[settles(g.a, "Delivered")])).is_empty());
}
