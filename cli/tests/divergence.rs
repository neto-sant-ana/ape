//! The divergence experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, for the reason the reconstruction experiment
//! gives: a comparison has to fail loudly. What is different here is how many worlds are
//! under comparison — a lineage rather than a world — so a phase records what it leaves
//! behind for *every* world it produced.

use ape::canon::Canon;
use ape::engine::hermeneia::{Conflict, Hypothesis, Outcome, Timeliness};
use ape::engine::thesis::{Interpretation, Thesis};
use ape::kernel::value_objects::Date;

use ape_cli::history::ResidentHistory;
use ape_cli::level;
use ape_cli::lineage::{self, Decision};
use ape_cli::subject::divergence::{self, Constructed};

fn day(day: u8) -> Date {
    Date::from_ymd(2026, 1, day).expect("a real date in January 2026")
}

/// The world as Phase 1 leaves it: the subject admitted, and a Genesis Thesis selecting both
/// commitments at a cut taken before any Event exists.
///
/// The Thesis comes from replaying a decision rather than from a call written here. That is
/// a finding of the previous experiment rather than a preference: what a later process gets
/// is the decision, so the decision is what every phase must go through.
fn constructed() -> (Canon<ResidentHistory>, Constructed, Vec<Decision>, Thesis) {
    let mut canon = Canon::new(ResidentHistory::new());
    let subject = divergence::construct(&mut canon).expect("the subject is admissible");

    let decisions = vec![divergence::genesis(subject.inflow, subject.overspend)];
    let lineage =
        lineage::replay(canon.history(), &decisions).expect("a genesis over admitted knowledge");

    let thesis = lineage.into_iter().next_back().expect("one Thesis");

    (canon, subject, decisions, thesis)
}

/// Phase 1 — Construct.
///
/// The subject is admitted and a Genesis Thesis selects both commitments, at a cut no Event
/// has reached. The world it denotes is one the resource's own bounds refuse, and that
/// refusal is the point: a verdict can be reproduced, whereas the previous experiment's
/// feasibility comparison could only compare two absences.
#[test]
fn phase_1_construct() {
    let (canon, subject, _, thesis) = constructed();

    // The cut names an instant the day is not finished with, and resolves an empty chain
    // because nothing has been recorded within it yet. Both halves are recorded: the second
    // is what the rest of the experiment is about.
    assert_eq!(thesis.cut().known_at(), &day(10));
    assert_eq!(
        thesis.cut().event_head(),
        None,
        "no Event has been recorded yet"
    );

    // Nothing has settled, so the cut imposes nothing and the whole selection is open — a
    // fork could still revise either commitment. The partition is asserted rather than the
    // membership, because it is the partition a later reconstruction can get wrong while
    // still selecting the right graph.
    assert_eq!(
        thesis.selection().frozen().count(),
        0,
        "an empty chain makes nothing unavoidable"
    );
    assert_eq!(
        thesis.selection().open().collect::<Vec<_>>(),
        [subject.inflow, subject.overspend]
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>(),
        "both commitments are open"
    );

    let interpretation =
        Interpretation::of(&thesis, canon.history()).expect("the Thesis is interpretable");

    let projected = interpretation
        .conditions_at(&day(10))
        .expect("conditions project at an instant");

    for (label, id) in [("inflow", subject.inflow), ("overspend", subject.overspend)] {
        let condition = projected
            .condition(id)
            .expect("a selected commitment has a condition");

        assert_eq!(condition.outcome(), &Outcome::Unsettled, "{label}");
        assert_eq!(
            condition.timeliness(),
            Some(&Timeliness::WithinDeadline),
            "{label}"
        );
        assert!(!condition.has_pending_dependencies(), "{label}");
        assert!(!condition.has_unfulfillable_dependencies(), "{label}");
    }

    assert_eq!(projected.event_head(), None);

    // Nothing is fulfilled, so nothing has moved the account. The level is the application's
    // fold over what a projection reports, and it counts what landed rather than what is
    // intended — which is why a world about to be declared infeasible still reads as zero.
    assert_eq!(
        level::settled(canon.history(), &projected, subject.instance)
            .expect("the world reads whole"),
        0.0,
        "an unsettled commitment has moved nothing"
    );

    // The verdict. Assuming every unsettled commitment is realized, the account ends at
    // −70, which the resource's constraint does not admit. This is a refusal produced by
    // the subject's own bounds, and reproducing it is what Phase 8 will ask for.
    let feasibility = interpretation
        .feasibility_under(Hypothesis::FinalState)
        .expect("feasibility is derivable without an effective time");

    assert_eq!(
        feasibility.conflicts(),
        [Conflict::OutOfBounds {
            instance: subject.instance,
            level: -70.0,
        }],
        "receiving 50 and spending 120 leaves the account outside 0..100"
    );
    assert_eq!(feasibility.event_head(), None);
}
