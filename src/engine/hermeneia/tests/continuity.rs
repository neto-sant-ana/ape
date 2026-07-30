//! Resumption is a guarantee, not a convention.
//!
//! `previous_event` belongs to an event's hashed identity, so a batch proves its own contiguity
//! and where it attaches. An accumulation can therefore refuse a batch that belongs to another
//! history, or that skips events.

use super::*;
use crate::engine::hermeneia::{Hypothesis, ProjectionError};

#[test]
fn absorbing_a_segment_advances_the_accumulations_coordinate() {
    let g = guide();
    let events = chain([(g.a, "Delivered"), (g.b, "Cancelled")]);

    let mut accumulation = Accumulation::default();
    assert_eq!(
        accumulation.event_head(),
        None,
        "nothing absorbed, nowhere on the chain",
    );

    accumulation
        .absorb(&g.knowledge, &g.selection(), &events[..1])
        .unwrap();
    assert_eq!(accumulation.event_head(), Some(events[0].id()));

    accumulation
        .absorb(&g.knowledge, &[], &events[1..])
        .unwrap();
    assert_eq!(accumulation.event_head(), Some(events[1].id()));
}

/// The two carry different coordinates, and the difference is the point: conditions answer to an
/// effective time, a feasibility report cannot, because feasibility does not take one.
#[test]
fn a_result_carries_the_context_that_produced_it() {
    let g = guide();
    let events = chain([(g.a, "Delivered")]);
    let mut accumulation = Accumulation::default();
    accumulation
        .absorb(&g.knowledge, &g.selection(), &events)
        .unwrap();

    let conditions = accumulation.conditions_at(&date(2026, 7, 1)).unwrap();
    assert_eq!(conditions.effective_at(), &date(2026, 7, 1));
    assert_eq!(conditions.event_head(), Some(events[0].id()));
    assert_eq!(
        conditions.conditions().len(),
        g.selection().len(),
        "keyed by the selection they were derived from",
    );

    let report = accumulation
        .feasibility_under(Hypothesis::OnDueDateNet)
        .unwrap();
    assert_eq!(report.hypothesis(), Hypothesis::OnDueDateNet);
    assert_eq!(report.event_head(), Some(events[0].id()));
}

#[test]
fn two_hypotheses_over_one_accumulation_stay_told_apart() {
    let g = guide();
    let accumulation = g.accumulate(&[]);

    let net = accumulation
        .feasibility_under(Hypothesis::OnDueDateNet)
        .unwrap();
    let final_state = accumulation
        .feasibility_under(Hypothesis::FinalState)
        .unwrap();

    assert_eq!(net.hypothesis(), Hypothesis::OnDueDateNet);
    assert_eq!(final_state.hypothesis(), Hypothesis::FinalState);
}

#[test]
fn a_segment_from_another_history_is_refused() {
    let g = guide();
    let ours = chain([(g.a, "Delivered")]);
    let theirs = chain([(g.c, "Delivered"), (g.b, "Cancelled")]);

    let mut accumulation = Accumulation::default();
    accumulation
        .absorb(&g.knowledge, &g.selection(), &ours)
        .unwrap();

    let refused = accumulation.absorb(&g.knowledge, &[], &theirs);

    assert!(matches!(
        refused,
        Err(ProjectionError::DisjointEventChain { carried: None, .. }),
    ));
    assert_eq!(
        accumulation.event_head(),
        Some(ours[0].id()),
        "a refused segment moves the coordinate no more than it records knowledge",
    );
}

#[test]
fn a_segment_that_skips_an_event_is_refused() {
    let g = guide();
    let events = chain([(g.a, "Delivered"), (g.b, "Cancelled"), (g.c, "Delivered")]);

    let mut accumulation = Accumulation::default();
    accumulation
        .absorb(&g.knowledge, &g.selection(), &events[..1])
        .unwrap();

    let refused = accumulation.absorb(&g.knowledge, &[], &events[2..]);

    assert!(matches!(
        refused,
        Err(ProjectionError::DisjointEventChain { .. })
    ));
}

#[test]
fn a_fresh_accumulation_cannot_begin_mid_chain() {
    let g = guide();
    let events = chain([(g.a, "Delivered"), (g.b, "Cancelled")]);

    let mut accumulation = Accumulation::default();
    let refused = accumulation.absorb(&g.knowledge, &g.selection(), &events[1..]);

    assert!(matches!(
        refused,
        Err(ProjectionError::DisjointEventChain { absorbed: None, .. }),
    ));
}

#[test]
fn a_segment_already_absorbed_cannot_be_absorbed_again() {
    let g = guide();
    let events = chain([(g.a, "Delivered")]);

    let mut accumulation = Accumulation::default();
    accumulation
        .absorb(&g.knowledge, &g.selection(), &events)
        .unwrap();

    assert!(matches!(
        accumulation.absorb(&g.knowledge, &[], &events),
        Err(ProjectionError::DisjointEventChain { .. })
    ));
}
