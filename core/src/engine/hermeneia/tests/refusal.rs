//! A refused absorption changes nothing.
//!
//! Resolution reads and can fail; recording only writes.

use super::*;
use crate::engine::hermeneia::{HermeneiaError, Outcome};

#[test]
fn a_failed_absorption_does_not_change_the_accumulation() {
    let g = guide();
    let events = chain([
        (g.a, "Delivered"),
        (g.b, "Delivered"),
        (g.c, "Unrecognised"),
    ]);

    let mut accumulation = Accumulation::default();
    accumulation
        .absorb(&g.knowledge, &g.selection(), &events[..1])
        .unwrap();
    let before = accumulation.conditions_at(&date(2026, 7, 1)).unwrap();

    let mut attempted = accumulation.clone();
    let refused = attempted.absorb(&g.knowledge, &g.selection(), &events[1..]);

    assert!(matches!(
        refused,
        Err(HermeneiaError::ObservationNotSettling { .. })
    ));

    assert_eq!(
        attempted
            .conditions_at(&date(2026, 7, 1))
            .unwrap()
            .conditions(),
        before.conditions(),
        "the settlement of b, resolved before the failure, must not have been recorded either",
    );
}

#[test]
fn a_duplicate_settlement_does_not_replace_the_original() {
    let g = guide();
    let events = chain([(g.a, "Delivered"), (g.a, "Cancelled")]);

    let mut accumulation = Accumulation::default();
    accumulation
        .absorb(&g.knowledge, &g.selection(), &events[..1])
        .unwrap();

    let refused = accumulation.absorb(&g.knowledge, &[], &events[1..]);

    assert!(matches!(
        refused,
        Err(HermeneiaError::SettledMoreThanOnce(twice)) if twice == g.a
    ));

    assert_eq!(
        accumulation
            .conditions_at(&date(2026, 7, 1))
            .unwrap()
            .condition(g.a)
            .unwrap()
            .outcome(),
        &Outcome::Fulfilled,
        "the original settlement stands; the refused one never landed",
    );
}

#[test]
fn an_invalid_event_leaves_no_new_selection_behind() {
    let g = guide();
    let mut accumulation = Accumulation::default();
    accumulation.absorb(&g.knowledge, &[g.a], &[]).unwrap();

    let refused = accumulation.absorb(&g.knowledge, &[g.b], &[settles(g.c, "Delivered")]);

    assert!(matches!(
        refused,
        Err(HermeneiaError::UnknownCommitment(missing)) if missing == g.c
    ));

    let projection = accumulation.conditions_at(&date(2026, 2, 1)).unwrap();
    assert_eq!(
        projection.conditions().len(),
        1,
        "b was resolved before the event failed, and must not have joined the selection",
    );
    assert!(projection.condition(g.a).is_some());
    assert!(projection.condition(g.b).is_none());
}
