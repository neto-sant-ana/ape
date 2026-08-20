//! Resource levels against their constraints, under the `FinalState` hypothesis.

use super::*;
use crate::engine::hermeneia::{Conflict, Hypothesis};

fn conflicts(accumulation: &Accumulation) -> Vec<Conflict> {
    accumulation
        .feasibility_under(Hypothesis::FinalState)
        .unwrap()
        .conflicts()
        .to_vec()
}

fn bounded(ceiling: i128) -> Constraint {
    Constraint::between(0, ceiling).unwrap()
}

#[test]
fn a_decreasing_effect_moves_the_level_the_other_way() {
    let b = basket(Effect::Decrease, bounded(100));

    assert_eq!(
        conflicts(&b.accumulate(&[])),
        vec![Conflict::OutOfBounds {
            instance: b.instance,
            level: -160,
        }],
        "four decreases of 40 fall below the floor of zero, not above the ceiling",
    );
}

#[test]
fn movements_that_together_leave_the_bounds_are_reported() {
    let b = basket(Effect::Increase, bounded(100));

    assert_eq!(
        conflicts(&b.accumulate(&[])),
        vec![Conflict::OutOfBounds {
            instance: b.instance,
            level: 160,
        }],
    );
}

#[test]
fn movements_that_together_fit_report_nothing() {
    let b = basket(Effect::Increase, bounded(200));

    assert!(conflicts(&b.accumulate(&[])).is_empty());
}

/// The two refusals that came in with the count, exercised rather than hoped for.
///
/// Both are unreachable with any plausible magnitude, and both would otherwise be guards nobody had
/// ever seen answer. What each protects is the same thing from two sides: a number that wrapped would
/// be weighed against a bound as though it were a level.
mod counts_that_do_not_fit {
    use super::*;
    use crate::engine::hermeneia::HermeneiaError;

    /// One magnitude, unsigned, too large to become a signed movement.
    ///
    /// This is the price of an unsigned magnitude, and the only case that pays it.
    #[test]
    fn a_magnitude_past_the_signed_range_cannot_move_a_level() {
        let mut knowledge = Fixture::default();
        let (statement, instance) = knowledge.quantifiable(Effect::Increase, bounded(100));

        let unmovable = commit(
            &mut knowledge,
            statement,
            instance,
            ActionValue::value(u128::MAX).unwrap(),
            date(2026, 3, 28),
            BTreeSet::new(),
        );

        let mut accumulation = Accumulation::default();

        assert!(matches!(
            accumulation.absorb(&knowledge, &[unmovable], &[]),
            Err(HermeneiaError::MagnitudeUnmovable { commitment }) if commitment == unmovable
        ));
    }

    /// Two magnitudes that each fit and together do not.
    #[test]
    fn a_sum_past_the_signed_range_is_refused_rather_than_wrapped() {
        let mut knowledge = Fixture::default();
        let (statement, instance) = knowledge.quantifiable(Effect::Increase, bounded(100));

        let half = i128::MAX as u128 / 2 + 1;
        let mut ids = Vec::new();

        for month in 3..5 {
            ids.push(commit(
                &mut knowledge,
                statement,
                instance,
                ActionValue::value(half).unwrap(),
                date(2026, month, 28),
                BTreeSet::new(),
            ));
        }

        let mut accumulation = Accumulation::default();
        accumulation
            .absorb(&knowledge, &ids, &[])
            .expect("each magnitude moves a level on its own");

        assert!(matches!(
            accumulation.feasibility_under(Hypothesis::FinalState),
            Err(HermeneiaError::LevelOutOfRange { instance: reported }) if reported == instance
        ));
    }
}

#[test]
fn a_cancelled_movement_never_lands() {
    let b = basket(Effect::Increase, bounded(150));

    assert_eq!(
        conflicts(&b.accumulate(&[])),
        vec![Conflict::OutOfBounds {
            instance: b.instance,
            level: 160,
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
    let b = basket(Effect::Increase, bounded(100));

    assert_eq!(
        conflicts(&b.accumulate(&[settles(b.ids[1], "Delivered")])),
        vec![Conflict::OutOfBounds {
            instance: b.instance,
            level: 160,
        }],
    );
}

#[test]
fn an_open_commitment_behind_a_cancelled_dependency_precedes_any_level() {
    let b = basket(Effect::Increase, bounded(100));
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
