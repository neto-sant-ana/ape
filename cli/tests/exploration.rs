//! The exploration experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, for the reason the reconstruction experiment gives:
//! a comparison has to fail loudly. What is different here is that the phases produce **numbers**,
//! so every literal one of them compares against is written before it runs — and a prediction that
//! turns out wrong is corrected in the open rather than adjusted into agreement.

use std::collections::BTreeSet;

use ape::canon::CanonicalHistory;
use ape::engine::hermeneia::Conflict;
use ape::kernel::entities::{CommitmentId, ResourceInstanceId};
use ape::kernel::value_objects::Date;

use ape_cli::reading::{self, Reading};
use ape_cli::repository::Repository;
use ape_cli::subject::exploration::{
    self, ADMISSIBLE, BEST, BUDGET, CANDIDATES, Founded, Judged, OPENED, OPENING, REFUSED,
};
use ape_cli::transfer;

/// A repository path no other process shares.
fn scratch(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir()
        .join(format!("ape-exploration-{}", std::process::id()))
        .join(name);
    let _ = std::fs::remove_dir_all(&path);
    path
}

/// What a repository costs, read off the files rather than off the process that wrote them.
///
/// The four the protocol asks for, and compared as one value: a phase that asserted them field by
/// field would pass while quietly leaving one of them unmeasured.
#[derive(Debug, PartialEq)]
struct Measured {
    entries: usize,
    lineage_bytes: u64,
    worlds: usize,
    watermark: Option<String>,
}

fn measured(repository: &Repository) -> Measured {
    Measured {
        entries: repository.read_journal().expect("readable").len(),
        lineage_bytes: std::fs::metadata(repository.lineage_path())
            .expect("the lineage is on disk")
            .len(),
        worlds: repository.read_worlds().expect("readable").len(),
        watermark: reading::corroborated(repository)
            .expect("the repository reconstructs")
            .canon
            .history()
            .recorded_through()
            .map(|at| at.to_iso()),
    }
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

/// Phase 1 — explore ephemerally: admit, fork in memory, interpret, score, drop the world.
///
/// Arrangement A, and every number it compares against is written here before it runs. The journal
/// grows by one per candidate; the lineage does not grow at all; one world stays recorded; and the
/// watermark advances **once** for twelve candidates, because they are all recorded at one instant.
///
/// What the objective finds is the first real measurement of the division Phase 0 only read off its
/// own literals.
#[test]
fn phase_1_explore_ephemerally() {
    let (repository, arrangement) = founded("phase-1");
    let subject = &arrangement.subject;
    let opening = arrangement.opening().id();

    let founding = measured(&repository);

    assert_eq!(
        founding,
        Measured {
            entries: OPENED,
            lineage_bytes: founding.lineage_bytes,
            worlds: 1,
            watermark: Some("2026-01-03".to_owned()),
        }
    );

    let mut working = exploration::read(&repository).expect("reconstructs");
    let mut weighed = Vec::new();

    for (nth, spend) in CANDIDATES.iter().enumerate() {
        exploration::admit(&mut working, subject.candidate(*spend)).expect("admissible");

        let candidate = *working
            .admitted
            .commitments
            .last()
            .expect("the candidate was just admitted");

        let world = exploration::considered(&working, &exploration::spending(opening, candidate))
            .expect("the candidate makes a world");

        let judged = exploration::judge(working.canon.history(), &world, subject.instance)
            .expect("the objective reads it");

        exploration::write(&repository, &working).expect("writable");

        assert_eq!(
            measured(&repository),
            Measured {
                entries: OPENED + nth + 1,
                lineage_bytes: founding.lineage_bytes,
                worlds: 1,
                watermark: Some("2026-01-04".to_owned()),
            },
            "after weighing and dropping candidate {spend}"
        );

        weighed.push((world.id(), judged));
    }

    // What the objective made of the twelve, measured this time.
    assert_eq!(
        weighed
            .iter()
            .filter(|(_, judged)| judged.level().is_some())
            .count(),
        ADMISSIBLE,
        "candidates the engine found nothing against"
    );
    assert_eq!(
        weighed
            .iter()
            .filter(|(_, judged)| matches!(judged, Judged::Refused(_)))
            .count(),
        REFUSED,
        "candidates the floor refused"
    );

    // And *why* the two were refused, by what the engine found rather than by which two they are.
    assert_eq!(
        weighed
            .iter()
            .filter_map(|(_, judged)| match judged {
                Judged::Refused(conflicts) => Some(conflicts.clone()),
                Judged::Admissible { .. } => None,
            })
            .collect::<Vec<_>>(),
        vec![
            vec![Conflict::OutOfBounds {
                instance: subject.instance,
                level: -10.0
            }],
            vec![Conflict::OutOfBounds {
                instance: subject.instance,
                level: -20.0
            }],
        ],
        "the floor refused them, one by ten and one by twenty"
    );

    assert_eq!(
        exploration::best(&weighed).expect("something was admissible"),
        (weighed[ADMISSIBLE - 1].0, BEST),
        "the objective's answer is the candidate that spends the opening exactly"
    );

    // E2, both halves, read off the repository a later process would open.
    assert_eq!(
        measured(&repository),
        Measured {
            entries: OPENED + BUDGET,
            lineage_bytes: founding.lineage_bytes,
            worlds: 1,
            watermark: Some("2026-01-04".to_owned()),
        }
    );
    assert_eq!(repository.read_lineage().expect("readable").len(), 1);
    assert!(
        !weighed.iter().any(|(world, _)| {
            repository
                .read_worlds()
                .expect("readable")
                .iter()
                .any(|record| record.thesis == world.to_string())
        }),
        "not one of the twelve worlds is anywhere in the repository"
    );
}

/// The other half of E2, and it is measured rather than assumed absent.
///
/// *Unidentifiable* is a claim about what the journal says, so it is checked by reading what the
/// journal says: every commitment admission it holds — the settled opening and all twelve weighed
/// candidates alike — carries the same field names. One shape across thirteen entries is the
/// positive form of "nothing marks a candidate", and asserting the absence of a marker would have
/// been the author's expectation rather than a measurement.
#[test]
fn an_ephemeral_candidate_is_shaped_exactly_like_an_intention() {
    let (repository, arrangement) = founded("shape");
    let subject = &arrangement.subject;

    let mut working = exploration::read(&repository).expect("reconstructs");

    for spend in CANDIDATES {
        exploration::admit(&mut working, subject.candidate(spend)).expect("admissible");
    }
    exploration::write(&repository, &working).expect("writable");

    let encoded: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(repository.journal_path()).expect("readable"),
    )
    .expect("the journal is JSON");

    let shapes: BTreeSet<Vec<String>> = encoded
        .as_array()
        .expect("the journal is a sequence")
        .iter()
        .filter_map(|entry| entry.as_object())
        .filter(|entry| entry.get("admits") == Some(&serde_json::json!("commitment")))
        .map(|entry| entry.keys().cloned().collect())
        .collect();

    assert_eq!(
        encoded
            .as_array()
            .expect("a sequence")
            .iter()
            .filter(|entry| entry.get("admits") == Some(&serde_json::json!("commitment")))
            .count(),
        BUDGET + 1,
        "twelve candidates and the opening"
    );
    assert_eq!(
        shapes.len(),
        1,
        "and one shape between them, so nothing in the journal says which were only weighed"
    );
}

/// The instants a projection is taken at, so that no answer is compared at one moment only.
const INSTANTS: [&str; 3] = ["2026-01-01", "2026-01-10", "2026-01-20"];

/// Phase 2 — what the record still answers, by value.
///
/// E1, and the one phase whose refutation stops the experiment rather than narrowing it. Every
/// derived answer the repository could give before exploration is taken again after twelve candidates
/// have been weighed and dropped, and compared as a **whole value**: a derived `PartialEq` *is* the
/// field-by-field comparison the protocol asks for, and it is the only form that keeps covering a
/// field added after this was written.
///
/// Four kinds of answer, because E1 names four — a projection at three instants, a reading of every
/// world, a feasibility report, and an applicability report. The last is derived from three worlds,
/// so this phase records two intentions of its own. They are minted by the very method that mints a
/// candidate, because Observation 2 measured that the two are the same admission.
#[test]
fn phase_2_what_the_record_still_answers() {
    let (repository, arrangement) = founded("phase-2");
    let subject = &arrangement.subject;
    let opening = arrangement.opening().id();

    let mut working = exploration::read(&repository).expect("reconstructs");

    let intending = |working: &mut _, spend| {
        exploration::admit(working, subject.intention(spend)).expect("admissible");
        let commitment = *last_commitment(working);

        exploration::decide(working, exploration::spending(opening, commitment)).expect("decidable")
    };

    let source = intending(&mut working, 5.0);
    let target = intending(&mut working, 15.0);

    exploration::write(&repository, &working).expect("writable");

    let intended = measured(&repository);

    assert_eq!(
        intended,
        Measured {
            entries: OPENED + 2,
            lineage_bytes: intended.lineage_bytes,
            worlds: 3,
            watermark: Some("2026-01-03".to_owned()),
        },
        "two intentions recorded at the instant the Event was, so the watermark has not moved yet"
    );

    let before_readings = readings(&repository, subject.instance);
    let before_judgments = judgments(&repository, subject.instance);
    let before_report =
        transfer::reconstruct(&repository, opening, source, target).expect("a report is derivable");

    for spend in CANDIDATES {
        exploration::admit(&mut working, subject.candidate(spend)).expect("admissible");

        let candidate = *last_commitment(&mut working);
        let world = exploration::considered(&working, &exploration::spending(opening, candidate))
            .expect("the candidate makes a world");

        exploration::judge(working.canon.history(), &world, subject.instance).expect("and scores");
    }
    exploration::write(&repository, &working).expect("writable");

    assert_eq!(
        measured(&repository),
        Measured {
            entries: OPENED + 2 + BUDGET,
            lineage_bytes: intended.lineage_bytes,
            worlds: 3,
            watermark: Some("2026-01-04".to_owned()),
        },
        "twelve propositions arrived, the watermark moved with them, and nothing about the worlds did"
    );

    // E1's premise, checked rather than assumed: fifteen commitments are known and three are
    // selected. A phase whose candidates had quietly ended up in a world would be measuring that
    // selected knowledge is stable, which nobody doubted.
    let held = exploration::read(&repository).expect("reconstructs");

    assert_eq!(held.admitted.commitments.len(), 1 + 2 + BUDGET);
    assert_eq!(
        held.lineage
            .decided()
            .iter()
            .flat_map(|world| world.selection().resolved())
            .collect::<BTreeSet<_>>()
            .len(),
        3,
        "the opening and the two intentions, and no candidate"
    );

    // E1 itself. Compared one world at one instant at a time, because a single assertion over all
    // nine prints all eighteen readings and names neither the instant nor the world that moved.
    for ((at, before), after) in INSTANTS
        .iter()
        .zip(&before_readings)
        .zip(readings(&repository, subject.instance))
    {
        for (position, (before, after)) in before.iter().zip(&after).enumerate() {
            assert_eq!(
                before, after,
                "world {position} read at {at}, before exploring and after"
            );
        }
        assert_eq!(before.len(), after.len(), "worlds readable at {at}");
    }
    assert_eq!(
        judgments(&repository, subject.instance),
        before_judgments,
        "and what the objective makes of every world"
    );
    assert_eq!(
        transfer::reconstruct(&repository, opening, source, target).expect("still derivable"),
        before_report,
        "and the applicability report the pressure point pointed at"
    );
}

fn last_commitment(working: &mut ape_cli::reading::Corroborated) -> &CommitmentId {
    working
        .admitted
        .commitments
        .last()
        .expect("something was just admitted")
}

/// Every world read at every instant, from the repository rather than from a live process.
fn readings(repository: &Repository, instance: ResourceInstanceId) -> Vec<Vec<Reading>> {
    INSTANTS
        .iter()
        .map(|at| {
            reading::reconstruct(repository, instance, &Date::parse(at).expect("a date"))
                .expect("the repository reads")
        })
        .collect()
}

/// What the objective makes of every world the repository holds.
fn judgments(repository: &Repository, instance: ResourceInstanceId) -> Vec<Judged> {
    let held = exploration::read(repository).expect("reconstructs");

    held.lineage
        .decided()
        .iter()
        .map(|world| {
            exploration::judge(held.canon.history(), world, instance)
                .expect("the objective reads it")
        })
        .collect()
}
