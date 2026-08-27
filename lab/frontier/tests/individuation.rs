//! Experiment 14 — Individuation. Phases against `lab/frontier/docs/14-individuation/00-protocol.md`.
//!
//! The question: should an address say when its entry was recorded — and can it, without costing two
//! records the ability to recognize each other?
//!
//! *The same entry* is settled before any phase runs, and in the Canon's terms rather than this
//! laboratory's: two admissions are the same entry when admitting either produces the same knowledge,
//! and admitting both produces the knowledge of admitting one. That is `AppendOutcome::AlreadyPresent`.
//!
//! **Every phase here runs against two shapes**, and which one it is in is derived from an address
//! rather than from a build flag — [`individuation::composite`]. A suite deciding its own column with
//! a `#[cfg]` would be a suite that cannot be run against the other one.

use std::collections::BTreeSet;

use ape_cli::journal::{Admission, EntryId};

use ape_frontier::subject::individuation::{
    self, CANDIDATE_ENTRIES, CANDIDATES, ENTRIES, INSERTED_ENTRIES, READMITTED_ENTRIES, SITES,
    WHEN_EARLY, WHEN_LATE,
};

/// A repository path no other process shares, emptied before it is used.
fn scratch(named: &str) -> ape_cli::repository::Repository {
    let path = std::env::temp_dir()
        .join(format!("ape-individuation-{}", std::process::id()))
        .join(named);
    let _ = std::fs::remove_dir_all(&path);

    ape_cli::repository::Repository::open(path)
}

/// Every address a journal produces, as the set two journals are intersected by.
fn held(journal: &[Admission]) -> BTreeSet<EntryId> {
    individuation::addresses(journal)
        .expect("the journal admits")
        .into_iter()
        .collect()
}

// ---------------------------------------------------------------------------------------------
// Phase 0 — What *the same entry* means, and where the application asks
// ---------------------------------------------------------------------------------------------

/// The places an address is compared are read out of the application, and there are eight.
///
/// Derived rather than listed, which is the difference between a closed space and a confident one: a
/// function that starts comparing addresses turns this red and names itself, and the count is what
/// says the scan ran at all.
///
/// The scan is lexical, and its one limit is named here rather than left to be discovered. It finds a
/// function by the tokens in its **signature and body** — an address type, or a field that holds one,
/// together with a comparison — so `converge::appended` is found because its body binds `entries` and
/// not because the line `found != expected` says anything about an address. A comparison in a function
/// that mentions neither would be missed, and nothing here can rule that out.
///
/// # It read the body alone, and that missed a real site
///
/// Recorded rather than quietly fixed. Experiment 16 added `reading::held`, which takes two slices of
/// addresses and compares them — and its body names neither an address type nor a field holding one,
/// because the addresses arrive as **parameters**. The scan found it only while the signature happened
/// to be wrapped across lines, and stopped finding it the moment `rustfmt` fitted the signature onto
/// one. A guard that depends on where a formatter breaks a line is a guard that reads the wrong
/// source, so the declaring line is now part of what is scanned.
///
/// The five it reported are unchanged by the repair — checked, and the reason it matters is that a
/// derived guard whose derivation moves is worth less than the list it replaced.
#[test]
fn the_places_an_address_is_compared_are_derived_and_there_are_eight() {
    const ADDRESSED: [&str; 4] = ["EntryId", ".entries", ".witness", ".after"];
    const COMPARED: [&str; 5] = ["==", "!=", ".difference(", ".contains(", ".position("];

    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cli/src")
        .canonicalize()
        .expect("the application is beside the laboratory");

    let mut found = Vec::new();
    let mut files = 0;

    for entry in std::fs::read_dir(&source).expect("the application's sources are readable") {
        let path = entry.expect("a directory entry").path();

        if path.extension().is_none_or(|kind| kind != "rs") {
            continue;
        }
        files += 1;

        let text = std::fs::read_to_string(&path).expect("a source file is readable");
        let name = path
            .file_name()
            .expect("a file has a name")
            .to_string_lossy()
            .to_string();

        let mut current: Option<String> = None;
        let mut body = String::new();

        let close = |current: &mut Option<String>, body: &mut String, found: &mut Vec<String>| {
            if let Some(function) = current.take()
                && ADDRESSED.iter().any(|token| body.contains(token))
                && COMPARED.iter().any(|token| body.contains(token))
            {
                found.push(format!("{}::{function}", name.trim_end_matches(".rs")));
            }
            body.clear();
        };

        for line in text.lines() {
            match declared(line) {
                Some(function) => {
                    close(&mut current, &mut body, &mut found);
                    current = Some(function);
                    // The declaring line is part of what is scanned: a function whose addresses
                    // arrive as parameters names them nowhere else. See the note above.
                    body.push_str(line);
                }
                None => body.push_str(line),
            }
        }
        close(&mut current, &mut body, &mut found);
    }

    assert!(files >= 10, "the scan read {files} files; it did not break");

    let mut named = SITES.to_vec();
    found.sort();
    named.sort();

    assert_eq!(found, named, "the places an address is compared");
}

/// The name of the function a line declares, where it declares one.
fn declared(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let rest = trimmed
        .strip_prefix("pub ")
        .or_else(|| trimmed.strip_prefix("pub(crate) "))
        .unwrap_or(trimmed)
        .strip_prefix("fn ")?;

    Some(
        rest.chars()
            .take_while(|letter| letter.is_alphanumeric() || *letter == '_')
            .collect(),
    )
}

// ---------------------------------------------------------------------------------------------
// Phase 1 — The subject, and the second varying instant
// ---------------------------------------------------------------------------------------------

/// The arrangement holds what it says it holds, and every literal is read once.
#[test]
fn the_arrangement_is_what_the_subject_says_it_is() {
    let arrangement = individuation::arranged().expect("the subject is admissible");

    assert_eq!(arrangement.here.journal.len(), ENTRIES, "the record");
    assert_eq!(
        arrangement.later.journal.len(),
        ENTRIES,
        "the one a day behind"
    );
    assert_eq!(
        arrangement.readmitted.len(),
        READMITTED_ENTRIES,
        "the journal that admits one entry twice"
    );

    for (candidate, (label, entries)) in arrangement
        .candidates
        .iter()
        .zip(CANDIDATES.into_iter().zip(CANDIDATE_ENTRIES))
    {
        assert_eq!(candidate.label, label, "the candidates are in order");
        assert_eq!(candidate.journal.len(), entries, "{label}");
    }

    assert_eq!(
        arrangement.candidate("inserted-early").journal.len(),
        INSERTED_ENTRIES
    );
}

/// Each candidate answers the number the arrangement wrote down for it.
///
/// Under both shapes: whether a candidate can be resolved at all is a phase below, and this is only
/// about what it says where it can. The candidate whose coordinate the change excludes is read
/// against its own coordinate, because a phase that could not read it would have no number here.
#[test]
fn each_candidate_answers_the_number_the_arrangement_names() {
    let arrangement = individuation::arranged().expect("the subject is admissible");

    for candidate in &arrangement.candidates {
        let after = individuation::addresses(&candidate.journal)
            .expect("the candidate admits")
            .pop()
            .expect("and is not empty");

        let resolved = individuation::resolve(
            &candidate.journal,
            &after,
            &arrangement.taken.decision,
            arrangement.instance,
        )
        .expect("every candidate resolves against its own coordinate");

        assert_eq!(resolved.answers, candidate.answers, "{}", candidate.label);
    }

    assert_ne!(WHEN_EARLY, WHEN_LATE, "the two answers are two answers");
}

/// Two instants vary, and only one of them is the coordinate's.
///
/// The one thing this subject adds to experiment 13's, and the reason the phase that measures the
/// witness can fail. Without it a coordinate that said *when* would determine the reference by itself,
/// and the arrangement rather than the change would have done the work.
#[test]
fn a_second_instant_varies_and_it_is_not_the_coordinates() {
    let arrangement = individuation::arranged().expect("the subject is admissible");

    let tail = |journal: &[Admission]| -> Vec<String> {
        journal
            .iter()
            .map(|entry| entry.recorded_at().to_owned())
            .collect()
    };

    let (here, late, noticed) = (
        tail(&arrangement.here.journal),
        tail(&arrangement.candidate("late").journal),
        tail(&arrangement.candidate("early-noticed-late").journal),
    );

    let differing = |one: &[String], other: &[String]| -> Vec<usize> {
        (0..one.len()).filter(|at| one[*at] != other[*at]).collect()
    };

    assert_eq!(
        differing(&here, &late),
        vec![ENTRIES - 1],
        "`late` differs at the entry the coordinate names, and nowhere else"
    );
    assert_eq!(
        differing(&here, &noticed),
        vec![ENTRIES - 2],
        "`early-noticed-late` differs at an entry the coordinate does not name"
    );

    let after = &arrangement.taken.after;
    let coordinate = |journal: &[Admission]| {
        individuation::addresses(journal)
            .expect("the journal admits")
            .pop()
            .expect("and is not empty")
    };

    assert_eq!(
        coordinate(&arrangement.candidate("early-noticed-late").journal),
        *after,
        "so the coordinate alone cannot tell that candidate from the record, under either shape"
    );
}

/// The two records founded apart admitted the same content, and their worlds are one world.
///
/// The ground experiment 09 measured, re-read here rather than inherited. What they hold in common is
/// the next phase's subject, and it is the number this change is predicted to move.
#[test]
fn the_two_records_founded_apart_decided_one_world() {
    let arrangement = individuation::arranged().expect("the subject is admissible");

    assert_eq!(
        arrangement.here.worlds.len(),
        arrangement.later.worlds.len(),
        "one decision each"
    );
    assert_eq!(
        arrangement.here.worlds[0].thesis, arrangement.later.worlds[0].thesis,
        "and one world, by identity, with nothing copied between them"
    );

    let here = scratch("here");
    let later = scratch("later");

    individuation::write_whole(&here, &arrangement.here).expect("a whole write");
    individuation::write_whole(&later, &arrangement.later).expect("a whole write");

    for (repository, instance, named) in [
        (&here, arrangement.instance, "here"),
        (&later, arrangement.later_instance, "later"),
    ] {
        let readings =
            ape_cli::reading::reconstruct(repository, instance, &individuation::asked_at())
                .expect("the record reads");

        assert_eq!(readings[0].level, WHEN_EARLY.0, "{named}");
    }
}

// ---------------------------------------------------------------------------------------------
// Phase 2 — What the change would touch, counted before it is made
// ---------------------------------------------------------------------------------------------

/// What two journals hold in common, under the shape that is there.
///
/// Written as a pair per row so that both columns are pre-registered and the run reads one of them.
/// The first row is two records that learned everything a day apart; the second is two that disagree
/// about a single instant. Between them they say whether the change is proportional to the
/// disagreement or total.
const IN_COMMON: [(&str, usize, usize); 2] = [
    ("here and later", ENTRIES, 0),
    ("here and late", ENTRIES, ENTRIES - 1),
];

#[test]
fn what_two_journals_hold_in_common() {
    let arrangement = individuation::arranged().expect("the subject is admissible");
    let composite = individuation::composite();

    for (pair, by_content, by_composite) in IN_COMMON {
        let other = match pair {
            "here and later" => &arrangement.later.journal,
            "here and late" => &arrangement.candidate("late").journal,
            other => panic!("the table names a pair the phase does not build: {other}"),
        };

        let shared = held(&arrangement.here.journal)
            .intersection(&held(other))
            .count();

        assert_eq!(
            shared,
            match composite {
                true => by_composite,
                false => by_content,
            },
            "{pair}"
        );
    }
}

/// Repositories committed to this workspace, and the source files that read one.
///
/// Counted before the build rather than estimated, because a compilation error lands in a minute and
/// a repository that stops replaying lands only when somebody runs the suite that reads it. These are
/// a concluded agents-row experiment's published artefacts — its published result is what they are
/// evidence for — and `lab/README.md` says a concluded experiment keeps its own.
const COMMITTED: (usize, usize) = (4, 2);

#[test]
fn the_workspace_holds_records_on_disk_that_live_code_reads() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace is above the laboratory");

    let mut repositories = Vec::new();
    let mut walk = vec![root.join("lab")];

    while let Some(directory) = walk.pop() {
        for entry in std::fs::read_dir(&directory).expect("a directory is readable") {
            let path = entry.expect("a directory entry").path();

            if !path.is_dir() {
                continue;
            }
            if path.join("lineage.json").is_file() {
                repositories.push(path.clone());
            }
            walk.push(path);
        }
    }

    let mut readers = 0;
    let mut scanned = 0;

    for source in ["lab/agents/tests", "lab/agents/src"] {
        for entry in std::fs::read_dir(root.join(source)).expect("a source directory is readable") {
            let path = entry.expect("a directory entry").path();

            if path.extension().is_none_or(|kind| kind != "rs") {
                continue;
            }
            scanned += 1;

            if std::fs::read_to_string(&path)
                .expect("a source file is readable")
                .contains("04-multiagent")
            {
                readers += 1;
            }
        }
    }

    assert!(
        scanned >= 4,
        "the scan read {scanned} files; it did not break"
    );
    assert_eq!(
        (repositories.len(), readers),
        COMMITTED,
        "committed repositories, and files that name where they are"
    );

    // The one that decides whether they are a cost: their addresses are bare, so a shape that writes
    // anything else writes something these cannot be read against.
    let lineage = std::fs::read_to_string(repositories[0].join("lineage.json"))
        .expect("a committed lineage is readable");
    let recorded: Vec<serde_json::Value> =
        serde_json::from_str(&lineage).expect("and is a lineage");

    let after = recorded[0]["after"]
        .as_str()
        .expect("it names a coordinate");

    assert_eq!(after.len(), 64, "a committed address, as it was written");
}

// ---------------------------------------------------------------------------------------------
// Phases 0 and 4 — the five sites, one probe each, under whichever shape is there
// ---------------------------------------------------------------------------------------------

/// What a site answered, in terms coarse enough to be one word and specific enough to be wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Answered {
    /// It read, and the number beside it is what it said.
    Read(i128),
    /// The coordinate reached no entry of that name.
    Unfound,
    /// The prefix offered is not the one the decision names.
    Unwitnessed,
    /// The coordinate resolves to an entry the journal admits more than once.
    Ambiguous,
    /// Two journals hold different entries at one position.
    Diverged,
    /// Two journals hold one entry at one position and disagree about when it was learned.
    RecordedDifferently,
}

/// One probe per comparison site, pre-registered under both shapes.
///
/// The rows are the five sites the scan derived, in the order the application reaches them, and each
/// probe is the smallest state that gets that far — a state refused earlier would be measuring the
/// site above it. `converge::ordered` is here because a site the change does **not** move is part of
/// the measurement: a table whose every row moved would be a table that could not have said which
/// rows the change is about.
const PROBES: [(&str, Answered, Answered); 6] = [
    (
        "journal::replay_through",
        Answered::Read(WHEN_LATE.0),
        Answered::Unfound,
    ),
    (
        "lineage::corroborate",
        Answered::Read(WHEN_EARLY.0),
        Answered::Unwitnessed,
    ),
    (
        "lineage::diagnosed",
        Answered::Ambiguous,
        Answered::Read(WHEN_EARLY.0),
    ),
    (
        "lineage::diagnosed, on one day",
        Answered::Ambiguous,
        Answered::Ambiguous,
    ),
    (
        "converge::appended",
        Answered::RecordedDifferently,
        Answered::Diverged,
    ),
    (
        "converge::ordered",
        Answered::Read(WHEN_EARLY.0),
        Answered::Read(WHEN_EARLY.0),
    ),
];

#[test]
fn each_site_answers_what_the_shape_that_is_there_says_it_does() {
    let arrangement = individuation::arranged().expect("the subject is admissible");
    let composite = individuation::composite();

    for (site, by_content, by_composite) in PROBES {
        let answered = match site {
            "journal::replay_through" => probe(
                &arrangement.candidate("late").journal,
                &arrangement.taken,
                &arrangement,
            ),
            "lineage::corroborate" => probe(
                &arrangement.candidate("early-noticed-late").journal,
                &arrangement.taken,
                &arrangement,
            ),
            "lineage::diagnosed" => {
                let taken = individuation::decided_over(&arrangement.readmitted)
                    .expect("a decision is takeable over it");
                probe(&arrangement.readmitted, &taken, &arrangement)
            }
            "lineage::diagnosed, on one day" => {
                let taken = individuation::decided_over(&arrangement.readmitted_at_once)
                    .expect("a decision is takeable over it");
                probe(&arrangement.readmitted_at_once, &taken, &arrangement)
            }
            "converge::appended" => converged(&arrangement),
            "converge::ordered" => extended(&arrangement),
            other => panic!("the table names a site the phase does not probe: {other}"),
        };

        assert_eq!(
            answered,
            match composite {
                true => by_composite,
                false => by_content,
            },
            "{site}"
        );
    }
}

/// Write a journal and a decision as a repository, read it back, and say what happened.
///
/// Through the application's own reader rather than through the laboratory's, because what a site
/// answers is a property of the record and not of a call this suite chose to make.
fn probe(
    journal: &[Admission],
    taken: &ape_cli::lineage::Taken,
    arrangement: &individuation::Arranged,
) -> Answered {
    use ape_cli::error::{JournalError, LineageError, ReadingError};

    let repository = scratch(&format!("probe-{}", journal.len()));
    let files =
        individuation::kept(journal, std::slice::from_ref(taken)).unwrap_or(individuation::Files {
            journal: journal.to_vec(),
            lineage: vec![taken.clone()],
            worlds: arrangement.here.worlds.clone(),
        });

    individuation::write_whole(&repository, &files).expect("a whole write");

    match ape_cli::reading::reconstruct(
        &repository,
        arrangement.instance,
        &individuation::asked_at(),
    ) {
        Ok(readings) => Answered::Read(readings[0].level),
        Err(ReadingError::Lineage(LineageError::Journal(JournalError::UnknownEntry(_)))) => {
            Answered::Unfound
        }
        Err(ReadingError::Lineage(LineageError::ReadmittedEntryIsAmbiguous { .. })) => {
            Answered::Ambiguous
        }
        Err(ReadingError::Lineage(
            LineageError::UnwitnessedKnowledge { .. }
            | LineageError::WitnessedKnowledgeAbsent { .. },
        )) => Answered::Unwitnessed,
        Err(other) => panic!("an unclassified refusal — {other}"),
    }
}

/// Put the record founded a day later to this one, through the only operation that takes a journal.
///
/// The two records founded apart, and they have to be the pair: both are legitimate repositories that
/// read on their own, so what refuses is the **merge** rather than either record. A pair where one
/// side cannot be read would be measuring the coordinate again, which is the row above.
fn converged(arrangement: &individuation::Arranged) -> Answered {
    use ape_cli::error::ConvergeError;

    let ours = scratch("converge-ours");
    let theirs = scratch("converge-theirs");

    individuation::write_whole(&ours, &arrangement.here).expect("a whole write");
    individuation::write_whole(&theirs, &arrangement.later).expect("a whole write");

    let held = ape_cli::reading::corroborated(&theirs).expect("the other record reads on its own");

    match ape_cli::converge::converge(&ours, &held) {
        Ok(_) => Answered::Read(WHEN_EARLY.0),
        Err(ConvergeError::RecordedDifferently { .. }) => Answered::RecordedDifferently,
        Err(ConvergeError::Diverged { .. }) => Answered::Diverged,
        Err(other) => panic!("an unclassified refusal — {other}"),
    }
}

/// Two parties on one journal, one of them a decision ahead, merged.
///
/// The site the change is not predicted to touch. Both hold the same journal, so every address agrees
/// under either shape, and what `ordered` does is put two decisions in one sequence.
fn extended(arrangement: &individuation::Arranged) -> Answered {
    use ape_cli::lineage::Decision;

    let ours = scratch("ordered-ours");
    let theirs = scratch("ordered-theirs");

    individuation::write_whole(&ours, &arrangement.here).expect("a whole write");

    let decided = individuation::resolve(
        &arrangement.here.journal,
        &arrangement.taken.after,
        &arrangement.taken.decision,
        arrangement.instance,
    )
    .expect("the record's own decision resolves");

    let advanced = ape_cli::lineage::Taken {
        decision: Decision::Advance {
            extends: decided.thesis.id(),
            known_at: individuation::day(individuation::KNOWN_AT + 1),
        },
        ..arrangement.taken.clone()
    };
    let held = individuation::kept(
        &arrangement.here.journal,
        &[arrangement.taken.clone(), advanced],
    )
    .expect("the party's own record rebuilds");

    individuation::write_whole(&theirs, &held).expect("a whole write");

    let read = ape_cli::reading::corroborated(&theirs).expect("the party's record reads");

    match ape_cli::converge::converge(&ours, &read) {
        Ok(merged) => Answered::Read(
            individuation::answers(
                merged.canon.history(),
                &merged.lineage.decided()[0],
                arrangement.instance,
            )
            .expect("the merged record reads")
            .0,
        ),
        Err(other) => panic!("the merge was expected to succeed — {other}"),
    }
}

/// What each stage of a pin still admits, under the shape that is there.
///
/// **N1.** Both columns are in the subject, written before the build. The prediction is the second
/// row: the witness determines where today the instants have to.
#[test]
fn each_stage_of_the_pin_admits_what_the_shape_says_it_does() {
    let arrangement = individuation::arranged().expect("the subject is admissible");

    for (stage, expected) in individuation::Stage::ALL
        .into_iter()
        .zip(individuation::stages())
    {
        let pin = arrangement.pin(stage).expect("the pin reads");

        let admitted: Vec<_> = arrangement
            .candidates
            .iter()
            .filter(|candidate| {
                pin.satisfied_by(
                    &candidate.journal,
                    &arrangement.taken.decision,
                    arrangement.instance,
                )
            })
            .collect();

        let bodies: BTreeSet<_> = admitted
            .iter()
            .map(|candidate| {
                individuation::knowledge(&candidate.journal, &arrangement.taken.after)
                    .expect("an admitted candidate resolves its coordinate")
            })
            .collect();

        let worlds: BTreeSet<_> = admitted
            .iter()
            .map(|candidate| {
                individuation::resolve(
                    &candidate.journal,
                    &arrangement.taken.after,
                    &arrangement.taken.decision,
                    arrangement.instance,
                )
                .expect("an admitted candidate produces a world")
                .thesis
                .id()
                .to_string()
            })
            .collect();

        assert_eq!(
            (stage.label(), admitted.len(), bodies.len(), worlds.len()),
            expected,
            "{}: candidates admitted, bodies of knowledge, worlds",
            stage.label()
        );
    }
}
