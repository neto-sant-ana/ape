//! The exploration experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, for the reason the reconstruction experiment gives:
//! a comparison has to fail loudly. What is different here is that the phases produce **numbers**,
//! so every literal one of them compares against is written before it runs — and a prediction that
//! turns out wrong is corrected in the open rather than adjusted into agreement.

use std::collections::BTreeSet;

use ape::canon::CanonicalHistory;
use ape::engine::hermeneia::Conflict;
use ape::engine::thesis::ThesisId;
use ape::kernel::entities::{CommitmentId, ResourceInstanceId};
use ape::kernel::value_objects::Date;

use ape_cli::error::{JournalError, LineageError, ReadingError};
use ape_cli::journal::EntryId;
use ape_cli::lineage::Taken;
use ape_cli::reading::{self, Corroborated, Reading, WorldRecord};
use ape_cli::repository::Repository;
use ape_cli::subject::exploration::{
    self, ADMISSIBLE, BEST, BUDGET, CANDIDATES, Constructed, Founded, Judged, OPENED, OPENING,
    REFUSED,
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

/// What arrangement B's record holds once the budget is spent, written before the run.
///
/// Thirteen decisions and thirteen worlds — the opening plus one per candidate. The number that
/// matters is the third: every decision witnesses every entry that stood when it was taken, so the
/// genesis witnesses 14 and each fork one more than the last, and the total is 14+15+…+26. It is a
/// literal, because a prediction computed from the arithmetic it predicts cannot be wrong.
///
/// `LINEAR` is what that total would be if a decision's record did not grow with the journal:
/// thirteen decisions witnessing fourteen entries each. The gap between the two is E3.
/// `BATCHED` is the same budget with the same decisions taken at the end instead of one at a time:
/// twelve decisions witnessing all twenty-six entries, plus the genesis. Predicted here because the
/// witness's contents are pinned exactly by corroboration, so the only thing a phase can vary — and
/// the only thing worth measuring — is *when* a decision is taken relative to what is admitted.
const RECORDED: usize = 1 + BUDGET;
const WITNESSED: usize = 260;
const LINEAR: usize = 182;
const WITNESSED_BY_EXPLORING: usize = 246;
const BATCHED: usize = 326;

const _: () = assert!(LINEAR == RECORDED * OPENED);
const _: () = assert!(WITNESSED_BY_EXPLORING == WITNESSED - OPENED);

/// What arrangement B's record holds, counted rather than sized.
///
/// Counts and not bytes, because a byte count is a property of the encoding and the protocol asks
/// which term dominates rather than how large a file is. The bytes are reported beside these and
/// predicted by nothing.
#[derive(Debug, PartialEq)]
struct Counted {
    entries: usize,
    decisions: usize,
    worlds: usize,
}

fn counted(repository: &Repository) -> Counted {
    Counted {
        entries: repository.read_journal().expect("readable").len(),
        decisions: repository.read_lineage().expect("readable").len(),
        worlds: repository.read_worlds().expect("readable").len(),
    }
}

/// Phase 3 — explore recording each candidate.
///
/// Arrangement B, from the same starting repository, with the same twelve candidates in the same
/// order, so that the two arrangements differ in one thing. E3 is the shape of what the record then
/// holds, and it is written above before the numbers are read.
#[test]
fn phase_3_explore_recording_each_candidate() {
    let (repository, arrangement) = founded("phase-3");
    let subject = &arrangement.subject;
    let opening = arrangement.opening().id();

    let mut working = exploration::read(&repository).expect("reconstructs");

    for (nth, spend) in CANDIDATES.iter().enumerate() {
        exploration::admit(&mut working, subject.candidate(*spend)).expect("admissible");

        let candidate = *last_commitment(&mut working);

        exploration::decide(&mut working, exploration::spending(opening, candidate))
            .expect("the candidate is decidable");
        exploration::judge(
            working.canon.history(),
            working.lineage.decided().last().expect("just decided"),
            subject.instance,
        )
        .expect("and scores");

        exploration::write(&repository, &working).expect("writable");

        assert_eq!(
            counted(&repository),
            Counted {
                entries: OPENED + nth + 1,
                decisions: 2 + nth,
                worlds: 2 + nth,
            },
            "after recording candidate {spend}"
        );
    }

    // E3. The journal grew by one per candidate; the witness grew by one *more* each time.
    let recorded = repository.read_lineage().expect("readable");

    assert_eq!(recorded.len(), RECORDED);

    for (position, taken) in recorded.iter().enumerate() {
        assert_eq!(
            taken.witness.len(),
            OPENED + position,
            "decision {position} witnesses every entry that stood when it was taken"
        );
    }

    let witnessed: usize = recorded.iter().map(|taken| taken.witness.len()).sum();

    assert_eq!(witnessed, WITNESSED);
    assert_eq!(
        counted(&repository).entries - OPENED,
        BUDGET,
        "twelve candidates cost twelve journal entries"
    );
    assert_eq!(
        witnessed - OPENED,
        WITNESSED_BY_EXPLORING,
        "and 246 witnessed entries, where a record that did not grow with the journal would cost 168"
    );
    assert!(
        witnessed > LINEAR,
        "the lineage does not grow linearly in decisions"
    );

    // The same twelve candidates and the same thirteen decisions, deciding at the end rather than as
    // it goes. Which is what an application does when it enumerates first and judges after, and it
    // is the more expensive of the two: every decision then witnesses the whole journal.
    let (batched, arrangement) = founded("batched");
    let subject = &arrangement.subject;
    let opening = arrangement.opening().id();

    let mut deferring = exploration::read(&batched).expect("reconstructs");
    let mut candidates = Vec::new();

    for spend in CANDIDATES {
        exploration::admit(&mut deferring, subject.candidate(spend)).expect("admissible");
        candidates.push(*last_commitment(&mut deferring));
    }
    for candidate in candidates {
        exploration::decide(&mut deferring, exploration::spending(opening, candidate))
            .expect("decidable");
    }
    exploration::write(&batched, &deferring).expect("writable");

    assert_eq!(
        counted(&batched),
        Counted {
            entries: OPENED + BUDGET,
            decisions: RECORDED,
            worlds: RECORDED,
        },
        "the same record by every count the protocol asks for"
    );
    assert_eq!(
        batched
            .read_lineage()
            .expect("readable")
            .iter()
            .map(|taken| taken.witness.len())
            .sum::<usize>(),
        BATCHED,
        "and 25% more witnessed entries, for enumerating before judging"
    );
}

/// The two arrangements differ in one thing, checked rather than arranged.
///
/// Every candidate is weighed twice over two repositories — dropped in one, recorded in the other —
/// and both the world it produces and what the objective makes of it come back identical. Which is
/// what lets Phase 1's numbers and Phase 3's be read as two prices for one deliberation, rather than
/// as two deliberations.
#[test]
fn recording_a_candidate_does_not_change_what_it_is_worth() {
    let explored = |name: &str, record: bool| {
        let (repository, arrangement) = founded(name);
        let subject = &arrangement.subject;
        let opening = arrangement.opening().id();

        let mut working = exploration::read(&repository).expect("reconstructs");
        let mut weighed = Vec::new();

        for spend in CANDIDATES {
            exploration::admit(&mut working, subject.candidate(spend)).expect("admissible");

            let candidate = *last_commitment(&mut working);
            let proposal = exploration::spending(opening, candidate);

            let world = if record {
                exploration::decide(&mut working, proposal).expect("decidable");
                working
                    .lineage
                    .decided()
                    .last()
                    .expect("just decided")
                    .clone()
            } else {
                exploration::considered(&working, &proposal).expect("a world")
            };

            weighed.push((
                world.id(),
                exploration::judge(working.canon.history(), &world, subject.instance)
                    .expect("scored"),
            ));
        }

        weighed
    };

    let dropped = explored("differ-dropped", false);
    let kept = explored("differ-kept", true);

    for ((spend, dropped), kept) in CANDIDATES.iter().zip(&dropped).zip(&kept) {
        assert_eq!(
            dropped, kept,
            "candidate {spend}, weighed and dropped against weighed and recorded"
        );
    }
    assert_eq!(dropped.len(), kept.len());
}

/// E4's first clause — a repeat costs a journal record and no knowledge.
///
/// Which is the mitigation an application reaches for when the record grows: revisit ground rather
/// than enumerate fresh ground. It is free in canonical history, where identity is content, and not
/// free in a sequence of admission records, which nothing deduplicates.
#[test]
fn a_repeat_costs_a_journal_record_and_no_knowledge() {
    let (repository, arrangement) = founded("repeat");
    let subject = &arrangement.subject;

    let mut working = exploration::read(&repository).expect("reconstructs");
    let candidate = subject.candidate(CANDIDATES[0]);

    exploration::admit(&mut working, candidate.clone()).expect("admissible");
    let first = *last_commitment(&mut working);

    exploration::admit(&mut working, candidate).expect("readmissible");
    let again = *last_commitment(&mut working);

    assert_eq!(working.journal.len(), OPENED + 2, "two journal records");
    assert_eq!(
        first, again,
        "and one canonical record, because identity is content"
    );
    assert_eq!(
        working
            .admitted
            .entries
            .iter()
            .collect::<BTreeSet<_>>()
            .len(),
        OPENED + 1,
        "so sixteen records address fifteen entries"
    );
}

/// E4's second clause — a repeat can cost readability, and what it costs depends on the interleaving.
///
/// Three arrangements of one journal, and the difference between them is only what falls between the
/// two occurrences and where the decision lands. Predicted before the run, from composing three
/// things the laboratory already held: `Taken::after` is an address, `replay_through` resolves an
/// address to its **first** occurrence, and corroboration weighs sets.
///
/// In every leg the **write succeeds**. Nothing at the moment of writing says the repository has
/// stopped being readable.
#[test]
fn a_readmission_costs_readability_when_something_was_learned_between() {
    let readmitting = |name: &str, learn_between: bool, decide_between: bool| {
        let (repository, arrangement) = founded(name);
        let subject = &arrangement.subject;
        let opening = arrangement.opening().id();

        let mut working = exploration::read(&repository).expect("reconstructs");

        let candidate = subject.candidate(CANDIDATES[0]);
        exploration::admit(&mut working, candidate.clone()).expect("admissible");
        let repeated = *last_commitment(&mut working);

        let mut learned = None;
        if learn_between {
            exploration::admit(&mut working, subject.candidate(CANDIDATES[1])).expect("admissible");
            learned = Some(*last_commitment(&mut working));
        }

        if decide_between {
            exploration::decide(
                &mut working,
                exploration::spending(opening, learned.expect("something was learned")),
            )
            .expect("decidable");
        }

        exploration::admit(&mut working, candidate).expect("readmissible");
        exploration::decide(&mut working, exploration::spending(opening, repeated))
            .expect("decidable");

        exploration::write(&repository, &working).expect("the write succeeds in every leg");

        let distinct = working
            .admitted
            .entries
            .iter()
            .collect::<BTreeSet<_>>()
            .len();

        (
            repository,
            working.journal.len(),
            distinct,
            repeated,
            learned,
        )
    };

    // Adjacent, decision after the repeat. Sixteen records, fifteen addresses, and it reconstructs —
    // not because nothing is wrong, but because a set cannot tell fifteen-of-sixteen from
    // fifteen-of-fifteen.
    let (repository, records, distinct, ..) = readmitting("adjacent", false, false);

    assert_eq!((records, distinct), (OPENED + 2, OPENED + 1));
    assert_eq!(
        reading::corroborated(&repository)
            .expect("the adjacent readmission reconstructs")
            .lineage
            .decided()
            .len(),
        2
    );

    // Separated, decision after the repeat. Refused — and named at the readmission rather than at
    // the innocent entry learned in between.
    let (repository, records, distinct, repeated, learned) = readmitting("separated", true, false);

    assert_eq!((records, distinct), (OPENED + 3, OPENED + 2));
    match reading::corroborated(&repository).map(|_| ()) {
        Err(ReadingError::Lineage(LineageError::ReadmittedEntryIsAmbiguous {
            readmitted,
            entry,
        })) => {
            assert_eq!(
                readmitted,
                EntryId::of(repeated),
                "the address that repeats"
            );
            assert_eq!(
                entry,
                EntryId::of(learned.expect("something was learned")),
                "and the entry it left unadmitted, which is not the one at fault"
            );
        }
        other => panic!("expected an ambiguous readmission, found {other:?}"),
    }

    // Separated, decision taken between the two occurrences. Refused by the replay instead, which
    // can see the repetition and says so.
    let (repository, records, distinct, repeated, _) = readmitting("between", true, true);

    assert_eq!((records, distinct), (OPENED + 3, OPENED + 2));
    match reading::corroborated(&repository).map(|_| ()) {
        Err(ReadingError::Lineage(LineageError::Journal(JournalError::EntryAlreadyPassed(
            entry,
        )))) => assert_eq!(entry, EntryId::of(repeated)),
        other => panic!("expected an entry already passed, found {other:?}"),
    }
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

/// Weigh and record the whole budget, returning the candidate commitments in order.
fn explore_recording(
    working: &mut Corroborated,
    subject: &Constructed,
    opening: ThesisId,
) -> Vec<CommitmentId> {
    CANDIDATES
        .iter()
        .map(|spend| {
            exploration::admit(working, subject.candidate(*spend)).expect("admissible");

            let candidate = *last_commitment(working);
            exploration::decide(working, exploration::spending(opening, candidate))
                .expect("decidable");

            candidate
        })
        .collect()
}

/// Arrangement C: drop the decisions `keep` rejects, and their witnesses, leaving the journal whole.
///
/// There is no code in the application that prunes, so this writes the two files — which is the same
/// technique the corroboration experiment used to tamper. The difference between tampering and pruning
/// is intent rather than mechanism, which means the repository cannot tell them apart either.
fn prune(repository: &Repository, keep: impl Fn(usize) -> bool) {
    let decisions: Vec<Taken> = repository
        .read_lineage()
        .expect("readable")
        .into_iter()
        .enumerate()
        .filter_map(|(at, taken)| keep(at).then_some(taken))
        .collect();

    let worlds: Vec<WorldRecord> = repository
        .read_worlds()
        .expect("readable")
        .into_iter()
        .enumerate()
        .filter_map(|(at, world)| keep(at).then_some(world))
        .collect();

    repository.write_lineage(&decisions).expect("writable");
    repository.write_worlds(&worlds).expect("writable");
}

/// Drop the entries the exploration added, keeping everything admitted before and after it.
fn prune_journal(repository: &Repository) {
    let mut journal = repository.read_journal().expect("readable");

    journal.drain(OPENED..OPENED + BUDGET);

    repository.write_journal(&journal).expect("writable");
}

/// How many of `candidates` any surviving witness still names.
fn named(repository: &Repository, candidates: &[CommitmentId]) -> usize {
    let witnessed: BTreeSet<EntryId> = repository
        .read_lineage()
        .expect("readable")
        .into_iter()
        .flat_map(|taken| taken.witness)
        .collect();

    candidates
        .iter()
        .filter(|id| witnessed.contains(&EntryId::of(**id)))
        .count()
}

fn witnessed_entries(repository: &Repository) -> usize {
    repository
        .read_lineage()
        .expect("readable")
        .iter()
        .map(|taken| taken.witness.len())
        .sum()
}

/// The whole of a repository, as three strings, so that two of them can be compared for identity.
fn bytes(repository: &Repository) -> (String, String, String) {
    let read = |path: std::path::PathBuf| std::fs::read_to_string(path).expect("readable");

    (
        read(repository.journal_path()),
        read(repository.lineage_path()),
        read(repository.worlds_path()),
    )
}

/// A magnitude no candidate has, for the intention that follows the exploration.
const INTENDED: f64 = 5.0;

/// What survives in the disposition where a decision follows the exploration: the genesis witnessing
/// 14, and the intention witnessing all 27. Written before the run.
const WITNESSED_AFTER: usize = 41;

/// Phase 4 — prune the leaves.
///
/// Arrangement C, in both dispositions the protocol's E5 note said to measure rather than assume: a
/// lineage whose exploratory decisions are its last ones, and one where a decision follows the
/// exploration. They differ in what a surviving witness names — and, predicted here before the run,
/// in whether the journal can be pruned at all, which is the half of E5 that says it cannot.
#[test]
fn phase_4_prune_the_leaves() {
    // ---------- the exploratory decisions are the last ones ----------
    let (last, arrangement) = founded("phase-4-last");
    let opened = bytes(&last);

    let mut working = exploration::read(&last).expect("reconstructs");
    let candidates = explore_recording(
        &mut working,
        &arrangement.subject,
        arrangement.opening().id(),
    );
    exploration::write(&last, &working).expect("writable");

    prune(&last, |at| at == 0);

    assert_eq!(
        counted(&last),
        Counted {
            entries: OPENED + BUDGET,
            decisions: 1,
            worlds: 1,
        },
        "twelve leaves dropped and the journal untouched"
    );
    reading::corroborated(&last).expect("the pruned repository reconstructs and corroborates");

    // E5's first half: what pruning recovers is the whole of what recording cost, byte for byte.
    let (_, lineage, worlds) = bytes(&last);

    assert_eq!(
        lineage, opened.1,
        "the lineage this arrangement started from"
    );
    assert_eq!(worlds, opened.2);
    assert_eq!(witnessed_entries(&last), OPENED);
    assert_eq!(
        named(&last, &candidates),
        0,
        "and the genesis predates every candidate, so nothing names one"
    );

    // E5's second half. Nothing refers to the twelve admissions any more.
    prune_journal(&last);

    reading::corroborated(&last).expect("the journal-pruned repository reconstructs too");
    assert_eq!(
        bytes(&last),
        opened,
        "byte for byte the repository nobody explored from"
    );

    // ---------- a decision follows the exploration ----------
    let (after, arrangement) = founded("phase-4-after");
    let subject = &arrangement.subject;
    let opening = arrangement.opening().id();

    let mut working = exploration::read(&after).expect("reconstructs");
    let candidates = explore_recording(&mut working, subject, opening);

    exploration::admit(&mut working, subject.candidate(INTENDED)).expect("admissible");
    let intention = *last_commitment(&mut working);
    exploration::decide(&mut working, exploration::spending(opening, intention))
        .expect("decidable");
    exploration::write(&after, &working).expect("writable");

    prune(&after, |at| at == 0 || at == RECORDED);

    assert_eq!(
        counted(&after),
        Counted {
            entries: OPENED + BUDGET + 1,
            decisions: 2,
            worlds: 2,
        }
    );
    reading::corroborated(&after).expect("this pruned repository reconstructs as well");

    // The opposite audit consequence: the surviving decision witnessed every entry that stood when it
    // was taken, so it names all twelve of the candidates that are gone.
    assert_eq!(
        witnessed_entries(&after),
        WITNESSED_AFTER,
        "the genesis witnessing 14 and a decision taken after the exploration witnessing all 27"
    );
    assert_eq!(
        named(&after, &candidates),
        BUDGET,
        "every pruned candidate is still named by what survived"
    );

    // And the same journal prune, refused. One measurement, two dispositions, opposite answers.
    prune_journal(&after);

    match reading::corroborated(&after).map(|_| ()) {
        Err(ReadingError::Lineage(LineageError::WitnessedKnowledgeAbsent { entry })) => {
            assert!(
                candidates.iter().any(|id| EntryId::of(*id) == entry),
                "refused at a candidate the surviving decision witnesses"
            )
        }
        other => panic!("expected the surviving witness to refuse it, found {other:?}"),
    }

    // ---------- the same argument, applied to arrangement A ----------
    // Observation 2 reported that arrangement A leaves its propositions in the journal permanently,
    // *because* nothing identifies them. Nothing was ever recorded here, so there are no leaves to
    // drop — and that reasoning is the thing being measured rather than a premise being used.
    let (ephemeral, arrangement) = founded("phase-4-ephemeral");
    let unexplored = bytes(&ephemeral);
    let subject = &arrangement.subject;
    let opening = arrangement.opening().id();

    let mut working = exploration::read(&ephemeral).expect("reconstructs");

    for spend in CANDIDATES {
        exploration::admit(&mut working, subject.candidate(spend)).expect("admissible");

        let candidate = *last_commitment(&mut working);
        exploration::considered(&working, &exploration::spending(opening, candidate))
            .expect("weighed, and dropped");
    }
    exploration::write(&ephemeral, &working).expect("writable");

    assert_eq!(counted(&ephemeral).entries, OPENED + BUDGET);

    prune_journal(&ephemeral);

    reading::corroborated(&ephemeral).expect("arrangement A's leftovers prune as well");
    assert_eq!(
        bytes(&ephemeral),
        unexplored,
        "so the indelible proposition was indelible only while something referred to it"
    );
}
