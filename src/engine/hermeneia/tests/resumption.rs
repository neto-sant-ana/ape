//! Folding in pieces must equal folding at once — the property that lets a checkpoint
//! stand in for the knowledge behind it.

use super::*;
use crate::engine::hermeneia::Hypothesis;

#[test]
fn folding_in_pieces_equals_folding_at_once() {
    let g = guide();

    let events = chain([(g.a, "Delivered"), (g.b, "Cancelled")]);

    let at_once = g.accumulate(&events).conditions_at(&date(2026, 7, 1)).unwrap();

    let mut piecewise = Accumulation::default();
    piecewise
        .absorb(&g.knowledge, &[g.a], &events[..1])
        .unwrap();
    piecewise
        .absorb(&g.knowledge, &[g.b, g.c, g.d], &events[1..])
        .unwrap();
    let piecewise = piecewise.conditions_at(&date(2026, 7, 1)).unwrap();

    assert_eq!(at_once.conditions(), piecewise.conditions());
}

#[test]
fn the_order_of_the_selection_does_not_change_the_result() {
    let g = guide();
    let events = chain([(g.a, "Cancelled")]);

    let mut forwards = Accumulation::default();
    forwards
        .absorb(&g.knowledge, &[g.a, g.b, g.c, g.d], &events)
        .unwrap();

    let mut backwards = Accumulation::default();
    backwards
        .absorb(&g.knowledge, &[g.d, g.c, g.b, g.a], &events)
        .unwrap();

    assert_eq!(
        forwards.conditions_at(&date(2026, 7, 1)).unwrap().conditions(),
        backwards
            .conditions_at(&date(2026, 7, 1))
            .unwrap()
            .conditions(),
    );

    let one_way = forwards
        .feasibility_under(Hypothesis::OnDueDateInAnyOrder)
        .unwrap();
    let other_way = backwards
        .feasibility_under(Hypothesis::OnDueDateInAnyOrder)
        .unwrap();

    assert!(
        !one_way.conflicts().is_empty(),
        "the comparison has to have something to compare",
    );
    assert_eq!(one_way, other_way);
}

/// An accumulation is a value: interpreting it does not consume or change it, so the same
/// checkpoint answers for any effective time, in any order, as many times as asked.
#[test]
fn interpreting_a_checkpoint_leaves_it_reusable() {
    let g = guide();
    let accumulation = g.accumulate(&[settles(g.a, "Delivered")]);

    let late = accumulation.conditions_at(&date(2026, 7, 1)).unwrap();
    let early = accumulation.conditions_at(&date(2026, 2, 1)).unwrap();
    let late_again = accumulation.conditions_at(&date(2026, 7, 1)).unwrap();

    assert_eq!(late.conditions(), late_again.conditions());
    assert_ne!(early.conditions(), late.conditions());
}
