//! Folding in pieces must equal folding at once — the property that lets a checkpoint
//! stand in for the knowledge behind it.

use super::*;

#[test]
fn folding_in_pieces_equals_folding_at_once() {
    let g = guide();

    let at_once = g
        .accumulate(&[settles(g.a, "Delivered"), settles(g.b, "Cancelled")])
        .view(&date(2026, 7, 1))
        .unwrap();

    let mut piecewise = Accumulation::default();
    piecewise
        .absorb(&g.knowledge, &[g.a], &[settles(g.a, "Delivered")])
        .unwrap();
    piecewise
        .absorb(&g.knowledge, &[g.b, g.c], &[settles(g.b, "Cancelled")])
        .unwrap();
    let piecewise = piecewise.view(&date(2026, 7, 1)).unwrap();

    assert_eq!(at_once.conditions(), piecewise.conditions());
}

/// An accumulation is a value: interpreting it does not consume or change it, so the same
/// checkpoint answers for any effective time, in any order, as many times as asked.
#[test]
fn interpreting_a_checkpoint_leaves_it_reusable() {
    let g = guide();
    let accumulation = g.accumulate(&[settles(g.a, "Delivered")]);

    let late = accumulation.view(&date(2026, 7, 1)).unwrap();
    let early = accumulation.view(&date(2026, 2, 1)).unwrap();
    let late_again = accumulation.view(&date(2026, 7, 1)).unwrap();

    assert_eq!(late.conditions(), late_again.conditions());
    assert_ne!(early.conditions(), late.conditions());
}
