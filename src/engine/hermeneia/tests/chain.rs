//! The conditions derived over the guide chain `a → b → c → d`.

use super::*;
use crate::engine::hermeneia::{Outcome, ProjectionError, Timeliness};

#[test]
fn an_unsettled_dependency_is_pending_for_its_dependent() {
    let g = guide();
    let p = g.accumulate(&[]).view(&date(2026, 2, 1)).unwrap();

    assert!(
        !p.condition(g.a).unwrap().has_pending_dependencies(),
        "a depends on nothing, so nothing is pending for it",
    );
    assert!(p.condition(g.b).unwrap().has_pending_dependencies());
    assert!(p.condition(g.c).unwrap().has_pending_dependencies());

    for id in g.selection() {
        assert_eq!(p.condition(id).unwrap().outcome(), &Outcome::Unsettled);
    }
}

#[test]
fn fulfilling_a_dependency_clears_it_for_its_dependent_but_not_the_next() {
    let g = guide();
    let p = g
        .accumulate(&[settles(g.a, "Delivered")])
        .view(&date(2026, 2, 1))
        .unwrap();

    assert_eq!(p.condition(g.a).unwrap().outcome(), &Outcome::Fulfilled);
    assert!(!p.condition(g.b).unwrap().has_pending_dependencies());
    assert!(
        p.condition(g.c).unwrap().has_pending_dependencies(),
        "c waits on b, which is still unsettled",
    );
}

/// The deliberate boundary: a cancelled dependency stops being pending, and this layer
/// says nothing more about `b` than that.
///
/// Whether `b` can still be fulfilled at all is feasibility's question. Reporting the
/// cleared wait as a positive condition of `b` — Available, Ready — would answer that
/// second question too, and answer it wrongly.
#[test]
fn cancelling_a_dependency_also_clears_it_for_its_dependent() {
    let g = guide();
    let p = g
        .accumulate(&[settles(g.a, "Cancelled")])
        .view(&date(2026, 2, 1))
        .unwrap();

    assert_eq!(p.condition(g.a).unwrap().outcome(), &Outcome::Cancelled);
    assert!(
        !p.condition(g.b).unwrap().has_pending_dependencies(),
        "a cancelled dependency has an outcome, so b is no longer waiting on it; \
         whether b remains realizable is evaluated separately",
    );
}

#[test]
fn a_deadline_is_breached_only_once_it_has_elapsed() {
    let g = guide();
    let accumulation = g.accumulate(&[]);

    let on_the_due_date = accumulation.view(&date(2026, 3, 31)).unwrap();
    assert_eq!(
        on_the_due_date.condition(g.a).unwrap().timeliness(),
        Some(&Timeliness::WithinDeadline),
        "the due date itself is still within the deadline, not past it",
    );

    let the_day_after = accumulation.view(&date(2026, 4, 1)).unwrap();
    assert_eq!(
        the_day_after.condition(g.a).unwrap().timeliness(),
        Some(&Timeliness::Breached),
        "the day after the due date is past it",
    );
    assert_eq!(
        the_day_after.condition(g.b).unwrap().timeliness(),
        Some(&Timeliness::WithinDeadline),
        "b's own deadline is still ahead",
    );
}

#[test]
fn settling_ends_the_timeliness_question() {
    let g = guide();
    let p = g
        .accumulate(&[settles(g.a, "Delivered")])
        .view(&date(2027, 1, 1))
        .unwrap();

    assert_eq!(
        p.condition(g.a).unwrap().timeliness(),
        None,
        "a deadline stops applying once the outcome is known, however late the reading",
    );
    assert_eq!(
        p.condition(g.b).unwrap().timeliness(),
        Some(&Timeliness::Breached),
    );
}

/// Moving only the effective time moves only timeliness. Knowledge time and operational
/// time are separate dimensions, and the accumulation holds nothing that depends on the
/// latter.
#[test]
fn the_same_knowledge_at_two_instants_differs_only_in_timeliness() {
    let g = guide();
    let accumulation = g.accumulate(&[settles(g.a, "Delivered")]);

    let early = accumulation.view(&date(2026, 2, 1)).unwrap();
    let late = accumulation.view(&date(2026, 7, 1)).unwrap();

    for id in g.selection() {
        let (early, late) = (early.condition(id).unwrap(), late.condition(id).unwrap());

        assert_eq!(early.outcome(), late.outcome());
        assert_eq!(
            early.has_pending_dependencies(),
            late.has_pending_dependencies()
        );
    }

    assert_ne!(
        early.condition(g.b).unwrap().timeliness(),
        late.condition(g.b).unwrap().timeliness(),
    );
}

#[test]
fn a_selection_missing_a_dependency_cannot_be_interpreted() {
    let g = guide();
    let mut accumulation = Accumulation::default();
    accumulation.absorb(&g.knowledge, &[g.b, g.c], &[]).unwrap();

    let refused = accumulation.view(&date(2026, 2, 1));

    assert!(matches!(
        refused,
        Err(ProjectionError::UnknownCommitment(missing)) if missing == g.a
    ));
}

#[test]
fn an_event_settling_an_unselected_commitment_is_refused() {
    let g = guide();
    let mut accumulation = Accumulation::default();

    let refused = accumulation.absorb(&g.knowledge, &[g.b], &[settles(g.a, "Delivered")]);

    assert!(matches!(
        refused,
        Err(ProjectionError::UnknownCommitment(missing)) if missing == g.a
    ));
}

#[test]
fn one_commitment_cannot_be_settled_twice() {
    let g = guide();
    let mut accumulation = Accumulation::default();

    let refused = accumulation.absorb(
        &g.knowledge,
        &g.selection(),
        &[settles(g.a, "Delivered"), settles(g.a, "Cancelled")],
    );

    assert!(matches!(
        refused,
        Err(ProjectionError::SettledMoreThanOnce(twice)) if twice == g.a
    ));
}

#[test]
fn a_cancelled_dependency_makes_the_path_below_it_unrealizable() {
    let g = guide();
    let p = g
        .accumulate(&[settles(g.a, "Cancelled")])
        .view(&date(2026, 2, 1))
        .unwrap();

    assert!(
        !p.condition(g.a).unwrap().has_unfulfillable_dependencies(),
        "a depends on nothing; that a is itself unfulfillable is a different statement",
    );
    assert!(p.condition(g.b).unwrap().has_unfulfillable_dependencies());
    assert!(
        p.condition(g.c).unwrap().has_unfulfillable_dependencies(),
        "c sits behind an unsettled b that can never be fulfilled",
    );
    assert!(
        p.condition(g.d).unwrap().has_unfulfillable_dependencies(),
        "d is far enough down the path that only a genuinely transitive rule reaches it",
    );
}

#[test]
fn a_fulfilled_dependency_leaves_the_path_realizable() {
    let g = guide();
    let p = g
        .accumulate(&[settles(g.a, "Delivered")])
        .view(&date(2026, 2, 1))
        .unwrap();

    for id in g.selection() {
        assert!(!p.condition(id).unwrap().has_unfulfillable_dependencies());
    }
}

/// Propagation stops at an accomplished fact.
///
/// The Canon admits an event fulfilling `b` even though `a` was cancelled — it checks
/// settle-once, the observation and the chain, never the dependencies, and refusing an
/// observation would be refusing what happened. So `b` is fulfilled, and `c`, which
/// required `b`, has its requirement met: nothing downstream inherits the contradiction.
///
/// That `b` is fulfilled *and* carries an unfulfillable dependency is the record of
/// reality having outrun the plan. It is read here, never repaired.
#[test]
fn unrealizability_does_not_travel_past_a_fulfilled_commitment() {
    let g = guide();
    let p = g
        .accumulate(&[settles(g.a, "Cancelled"), settles(g.b, "Delivered")])
        .view(&date(2026, 2, 1))
        .unwrap();

    let b = p.condition(g.b).unwrap();
    assert_eq!(b.outcome(), &Outcome::Fulfilled);
    assert!(
        b.has_unfulfillable_dependencies(),
        "the violation stays legible on b",
    );

    assert!(
        !p.condition(g.c).unwrap().has_unfulfillable_dependencies(),
        "c required b, and b was fulfilled — whatever a did",
    );
}
