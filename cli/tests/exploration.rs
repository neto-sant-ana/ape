//! The exploration experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, for the reason the reconstruction experiment gives:
//! a comparison has to fail loudly. What is different here is that the phases produce **numbers**,
//! so every literal one of them compares against is written before it runs — and a prediction that
//! turns out wrong is corrected in the open rather than adjusted into agreement.

use std::collections::BTreeSet;

use ape::canon::CanonicalHistory;
use ape::kernel::value_objects::Date;

use ape_cli::reading;
use ape_cli::repository::Repository;
use ape_cli::subject::exploration::{
    self, ADMISSIBLE, BUDGET, CANDIDATES, Founded, Judged, OPENED, OPENING,
};

/// A repository path no other process shares.
fn scratch(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir()
        .join(format!("ape-exploration-{}", std::process::id()))
        .join(name);
    let _ = std::fs::remove_dir_all(&path);
    path
}

/// The starting repository, and what the arrangement refers to.
fn founded(name: &str) -> (Repository, Founded) {
    let repository = Repository::open(scratch(name));
    let founded = exploration::founded().expect("the arrangement holds");

    exploration::found(&repository, &founded).expect("writable");

    (repository, founded)
}

/// Phase 0 — the world, the objective and the budget, recorded before anything explores.
///
/// Nothing here is a finding. It is the state every later measurement is taken against, written down
/// so that a number read in Phase 3 can be read against a starting point nobody adjusted.
#[test]
fn phase_0_the_world_the_objective_and_the_budget() {
    let (repository, arrangement) = founded("phase-0");
    let subject = &arrangement.subject;

    // The world. Fourteen admissions, of which the last is the Event that settles the opening.
    assert_eq!(repository.read_journal().expect("readable").len(), OPENED);
    assert_eq!(
        subject.admitted.commitments.len(),
        1,
        "the opening, and no candidate"
    );

    let rebuilt = reading::corroborated(&repository).expect("the repository reconstructs");

    assert_eq!(
        rebuilt.canon.history().recorded_through(),
        Some(Date::parse("2026-01-03").expect("a date")),
        "the watermark stands where the Event was recorded"
    );

    // The settled opening: frozen by a cut that recognizes the Event, and nothing yet proposed.
    let world = arrangement.opening();

    assert_eq!(
        world.selection().frozen().collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.opening]),
        "the opening is unavoidable"
    );
    assert_eq!(
        world.selection().open().count(),
        0,
        "and nothing is proposed about spending it"
    );

    let opened = reading::of(
        rebuilt.canon.history(),
        world,
        subject.instance,
        &exploration::asked_at(),
    )
    .expect("the world reads");

    assert_eq!(opened.level, OPENING, "the opening landed");
    assert!(opened.conflicts.is_empty(), "and nothing is wrong with it");

    // The budget. These read the enumeration against itself and prove nothing about the engine:
    // how many candidates it actually refuses is Phase 1's measurement, and this is the prediction
    // it will be read against. The array's own type is what fixes the count at BUDGET.
    assert_eq!(
        CANDIDATES
            .iter()
            .map(|value| *value as u64)
            .collect::<BTreeSet<_>>()
            .len(),
        BUDGET,
        "twelve distinct candidates, so a repeat is something a phase does on purpose"
    );
    assert_eq!(
        CANDIDATES.iter().filter(|spend| **spend <= OPENING).count(),
        ADMISSIBLE,
        "ten of the twelve fit inside the opening"
    );

    // The objective, on the only world there is: nothing spent, and the opening still there.
    assert_eq!(
        exploration::judge(rebuilt.canon.history(), world, subject.instance)
            .expect("the objective reads the world"),
        Judged::Admissible { level: OPENING },
        "spending nothing is admissible, and it is the worst admissible score"
    );
}

/// The objective is a rule and not a state: asked twice about one world, it answers the same.
///
/// This is what the experimental boundary excludes, checked rather than promised. An objective that
/// adapted between two readings would make every later number a measurement of the objective, and
/// the phase that discovered it would have no way to tell.
#[test]
fn the_objective_does_not_move_between_two_readings() {
    let (repository, arrangement) = founded("objective");

    let one = exploration::read(&repository).expect("reconstructs");
    let other = exploration::read(&repository).expect("and reconstructs again");

    let judged = |working: &ape_cli::reading::Corroborated| {
        exploration::judge(
            working.canon.history(),
            &working.lineage.decided()[0],
            arrangement.subject.instance,
        )
        .expect("the objective reads the world")
    };

    assert_eq!(judged(&one), judged(&other));
}
