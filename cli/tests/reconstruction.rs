//! The reconstruction experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, because what the procedure asks for is a
//! comparison and a comparison has to fail loudly. The observations a phase is told to
//! preserve are asserted here, so a later phase that reproduces them differently is a
//! failure rather than a footnote.

use ape::canon::{Canon, CanonicalHistory, EventSubmission};
use ape::engine::hermeneia::{Hypothesis, Outcome, Timeliness};
use ape::engine::thesis::{GenesisInput, Interpretation, KnowledgeCut, Thesis};
use ape::kernel::value_objects::Date;

use ape_cli::history::ResidentHistory;
use ape_cli::level;
use ape_cli::subject::{self, Subject};

fn day(day: u8) -> Date {
    Date::from_ymd(2026, 1, day).expect("a real date in January 2026")
}

/// The world as Phase 1 leaves it: the subject admitted, and a Genesis Thesis selecting it
/// at a cut taken before any Event exists.
fn constructed() -> (Canon<ResidentHistory>, Subject, Thesis) {
    let mut canon = Canon::new(ResidentHistory::new());
    let subject = subject::construct(&mut canon).expect("the subject is admissible");

    let thesis = Thesis::genesis(
        canon.history(),
        GenesisInput {
            cut: KnowledgeCut::at(canon.history(), day(10)),
            selection: [subject.commitment].into(),
        },
    )
    .expect("a genesis selecting an admitted commitment");

    (canon, subject, thesis)
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

    // Nothing has settled, so nothing has moved the warehouse. This is the derived
    // consequence on the level, and it is recorded because reconstruction must reproduce
    // it — not because the engine stores it anywhere.
    assert_eq!(
        level::settled(canon.history(), &within, subject.instance).expect("the world reads whole"),
        0.0,
        "an unsettled commitment has moved nothing"
    );

    // The intended movement fits the resource's bounds, so nothing about the intention is
    // infeasible.
    let feasibility = interpretation
        .feasibility_under(Hypothesis::FinalState)
        .expect("feasibility is derivable without an effective time");

    assert!(
        feasibility.conflicts().is_empty(),
        "a single increase of 10 within 0..100 conflicts with nothing, found {:?}",
        feasibility.conflicts()
    );
    assert_eq!(feasibility.event_head(), None);
}

/// Phase 2 — Observe.
///
/// An Event settles the commitment, and the intended world is advanced only as far as the
/// existing semantics require in order to recognize it. The interpretation that results is
/// the reference observation reconstruction is measured against.
#[test]
fn phase_2_observe() {
    let (mut canon, subject, genesis) = constructed();

    let event = canon
        .admit_event(
            EventSubmission {
                commitment_id: subject.commitment,
                observation: subject.fulfilling.clone(),
                occurred_at: day(12),
            },
            day(12),
        )
        .expect("an observation the statement can settle with");

    assert_eq!(
        canon.history().head(),
        Some(event),
        "the canonical chain now ends at the settling Event"
    );

    // The Genesis Thesis does not see it, and re-reading it proves the cut is doing the
    // work rather than the projection. A Thesis holds the instant it was taken at; an
    // Event admitted afterwards is not knowledge it withheld, it is knowledge it predates.
    let behind = Interpretation::of(&genesis, canon.history())
        .expect("a Thesis stays interpretable after history moves");

    assert_eq!(
        behind
            .conditions_at(&day(15))
            .expect("conditions project at an instant")
            .condition(subject.commitment)
            .expect("the selected commitment has a condition")
            .outcome(),
        &Outcome::Unsettled,
        "a cut taken before the Event cannot recognize it"
    );

    // So recognizing it requires advancing, which is what the boundary's exclusion list
    // permits when the procedure itself demands it. Advancement is how a Thesis takes on
    // later history, and it produces a new Thesis rather than mutating this one.
    let advancement = genesis
        .advance(canon.history(), KnowledgeCut::at(canon.history(), day(15)))
        .expect("a later cut the genesis can advance to");

    assert_eq!(
        advancement.imposed_count(),
        0,
        "history imposed nothing the genesis had not already selected"
    );

    let advanced = advancement.into_thesis();

    assert_ne!(
        advanced.id(),
        genesis.id(),
        "recognizing later history produces a different Thesis"
    );
    assert_eq!(advanced.parent(), &Some(genesis.id()));
    assert_eq!(advanced.cut().event_head(), Some(event));
    assert_eq!(advanced.cut().known_at(), &day(15));

    // Settled, and therefore no longer under deadline pressure: timeliness is absent once
    // an outcome is known, because a due date stops applying to what is already settled.
    let interpretation =
        Interpretation::of(&advanced, canon.history()).expect("the advanced Thesis interprets");

    let projected = interpretation
        .conditions_at(&day(15))
        .expect("conditions project at an instant");

    let condition = projected
        .condition(subject.commitment)
        .expect("the selected commitment has a condition");

    assert_eq!(condition.outcome(), &Outcome::Fulfilled);
    assert_eq!(condition.timeliness(), None);
    assert_eq!(projected.event_head(), Some(event));

    // The settlement moved the warehouse. Against Phase 1's zero, this is the observation
    // that distinguishes a world where the Event happened from one where it had not.
    assert_eq!(
        level::settled(canon.history(), &projected, subject.instance)
            .expect("the world reads whole"),
        10.0,
        "the fulfilled increase of 10 has landed"
    );

    // Projected past the due date, a fulfilled commitment stays fulfilled. Settlement is a
    // fact the passage of time does not revisit.
    assert_eq!(
        interpretation
            .conditions_at(&day(25))
            .expect("conditions project at any instant")
            .condition(subject.commitment)
            .expect("the selected commitment has a condition")
            .outcome(),
        &Outcome::Fulfilled
    );

    let feasibility = interpretation
        .feasibility_under(Hypothesis::FinalState)
        .expect("feasibility is derivable without an effective time");

    assert!(
        feasibility.conflicts().is_empty(),
        "the settled increase of 10 stays within 0..100, found {:?}",
        feasibility.conflicts()
    );
    assert_eq!(
        feasibility.event_head(),
        Some(event),
        "the report is derived from the chain that contains the Event"
    );
}
