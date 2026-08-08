//! The reconstruction experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, because what the procedure asks for is a
//! comparison and a comparison has to fail loudly. The observations a phase is told to
//! preserve are asserted here, so a later phase that reproduces them differently is a
//! failure rather than a footnote.

use ape::canon::Canon;
use ape::engine::hermeneia::{Hypothesis, Outcome, Timeliness};
use ape::engine::thesis::{GenesisInput, Interpretation, KnowledgeCut, Thesis};
use ape::kernel::value_objects::Date;

use ape_cli::history::ResidentHistory;
use ape_cli::subject;

fn day(day: u8) -> Date {
    Date::from_ymd(2026, 1, day).expect("a real date in January 2026")
}

/// Phase 1 — Construct.
///
/// The subject is admitted, a Genesis Thesis recognizes it, and the world is interpreted
/// before any settling Event exists. What this records is the reference the later phases
/// are measured against, so every value asserted here is one reconstruction must reproduce.
#[test]
fn phase_1_construct() {
    let mut canon = Canon::new(ResidentHistory::new());
    let subject = subject::construct(&mut canon).expect("the subject is admissible");

    // The cut is taken after the commitment was recorded and before it is due. With no
    // Event admitted, the chain it resolves is empty — which is itself something
    // reconstruction must reproduce rather than approximate.
    let cut = KnowledgeCut::at(canon.history(), day(10));
    assert_eq!(cut.event_head(), None, "no Event has been admitted yet");
    assert_eq!(cut.known_at(), &day(10));

    let thesis = Thesis::genesis(
        canon.history(),
        GenesisInput {
            cut,
            selection: [subject.commitment].into(),
        },
    )
    .expect("a genesis selecting an admitted commitment");

    assert!(
        thesis.selection().contains(subject.commitment),
        "the Thesis selects the commitment it was given"
    );

    let interpretation =
        Interpretation::of(&thesis, canon.history()).expect("the Thesis is interpretable");

    // Within the deadline, the commitment is unsettled and waiting on nothing: the only
    // thing standing between it and a settlement is an Event that has not happened.
    let within = interpretation
        .conditions_at(&day(10))
        .expect("conditions project at an instant");

    let condition = within
        .condition(subject.commitment)
        .expect("the selected commitment has a condition");

    assert_eq!(condition.outcome(), &Outcome::Unsettled);
    assert!(!condition.has_pending_dependencies());
    assert!(!condition.has_unfulfillable_dependencies());
    assert_eq!(condition.timeliness(), Some(&Timeliness::WithinDeadline));
    assert_eq!(within.event_head(), None);

    // The same interpretation projected past the due date. Settlement has not moved,
    // because no Event settled it; timeliness has, because the deadline passed. The two
    // axes are independent, and reading them from one interpretation is what shows it.
    let overdue = interpretation
        .conditions_at(&day(25))
        .expect("conditions project at any instant");

    let condition = overdue
        .condition(subject.commitment)
        .expect("the selected commitment has a condition");

    assert_eq!(condition.outcome(), &Outcome::Unsettled);
    assert_eq!(condition.timeliness(), Some(&Timeliness::Breached));

    // The intended movement fits the resource's bounds, so nothing about the intention is
    // infeasible. This is the factual consequence the subject exists to expose.
    let feasibility = interpretation
        .feasibility_under(Hypothesis::FinalState)
        .expect("feasibility is derivable without an effective time");

    assert!(
        feasibility.conflicts().is_empty(),
        "a single increase of 10 within 0..100 conflicts with nothing"
    );
    assert_eq!(feasibility.event_head(), None);
}
