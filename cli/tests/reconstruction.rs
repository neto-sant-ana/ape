//! The reconstruction experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, because what the procedure asks for is a
//! comparison and a comparison has to fail loudly. The observations a phase is told to
//! preserve are asserted here, so a later phase that reproduces them differently is a
//! failure rather than a footnote.

use ape::canon::{Canon, CanonicalHistory};
use ape::engine::hermeneia::{Hypothesis, Outcome, Timeliness};
use ape::engine::thesis::{GenesisInput, Interpretation, KnowledgeCut, Thesis};
use ape::kernel::entities::{CommitmentId, ResourceInstanceId};
use ape::kernel::value_objects::Date;

use ape_cli::history::ResidentHistory;
use ape_cli::journal;
use ape_cli::level;
use ape_cli::lineage::{self, Decision};
use ape_cli::reading::{OutcomeRecord, Reading};
use ape_cli::repository::Repository;
use ape_cli::subject::{self, Constructed};

fn day(day: u8) -> Date {
    Date::from_ymd(2026, 1, day).expect("a real date in January 2026")
}

/// The world as Phase 1 leaves it: the subject admitted, and a Genesis Thesis selecting it
/// at a cut taken before any Event exists.
///
/// The Thesis comes from replaying a decision rather than from a call written here, for the
/// same reason the subject comes from a journal: what a later process gets is the decision,
/// so the decision is what every phase must go through.
fn constructed() -> (Canon<ResidentHistory>, Constructed, Vec<Decision>, Thesis) {
    let mut canon = Canon::new(ResidentHistory::new());
    let subject = subject::construct(&mut canon).expect("the subject is admissible");

    let decisions = vec![subject::genesis(subject.commitment)];
    let lineage =
        lineage::replay(canon.history(), &decisions).expect("a genesis over admitted knowledge");

    let thesis = lineage.into_iter().next_back().expect("one Thesis");

    (canon, subject, decisions, thesis)
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
    let (mut canon, subject, decisions, genesis) = constructed();

    // The settlement is a journal entry like every other admission, so the world stays
    // described in one place rather than in a journal plus a call this test made.
    journal::replay(&mut canon, &[subject::settlement(subject.commitment)])
        .expect("an observation the statement can settle with");

    let event = canon
        .history()
        .head()
        .expect("the settling Event is now the head");

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
    // permits when the procedure itself demands it. It is a decision like the genesis was,
    // and it goes into the lineage rather than being taken here.
    let mut decisions = decisions;
    decisions.push(subject::advancement());

    let lineage = lineage::replay(canon.history(), &decisions)
        .expect("a lineage the canonical knowledge supports");

    assert_eq!(lineage.len(), 2, "the genesis, and the world that succeeded it");

    let advanced = lineage.into_iter().next_back().expect("the advanced Thesis");

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

/// Phase 3 — Persist.
///
/// Only what a later process cannot derive is written down. For every datum the procedure
/// asks what reconstruction becomes impossible without it, and the journal is what survives
/// that question: an admission's fields are the input the engine derives everything else
/// from, and its recording instant is assigned rather than derived, so it exists after the
/// process only because it was kept.
#[test]
fn phase_3_persist() {
    let (mut canon, subject, decisions, genesis) = constructed();

    let mut journal = subject.journal;
    let settlement = subject::settlement(subject.commitment);
    journal::replay(&mut canon, std::slice::from_ref(&settlement)).expect("the Event admits");
    journal.push(settlement);

    // The lineage the experiment reached: the world it first reasoned about, and the
    // recognition of the Event. Both are decisions, and neither is recoverable from
    // canonical knowledge — history says what became known, not which of it a world selects.
    let mut decisions = decisions;
    decisions.push(subject::advancement());

    let repository = Repository::open(scratch("phase-3"));
    repository
        .write_journal(&journal)
        .expect("the repository is writable");
    repository
        .write_lineage(&decisions)
        .expect("the repository is writable");

    // The journal survives its own encoding. This is not yet reconstruction — no process
    // has died — but a repository that cannot return what it was handed would fail the
    // later phases for a reason that has nothing to do with APE.
    let read = repository.read_journal().expect("the journal reads back");

    assert_eq!(
        read.len(),
        journal.len(),
        "every admission was kept, and none was invented"
    );

    let written = std::fs::read_to_string(repository.journal_path()).expect("the file is there");

    // What must not be there. Each of these is derived from the journal above it, and
    // storing one would put an answer beside the question it comes from, where the two can
    // drift apart without anything noticing.
    for derived in [
        "level",
        "outcome",
        "fulfilled",
        "unsettled",
        "timeliness",
        "breached",
        "condition",
        "feasib",
        "conflict",
        "thesis",
        "previous_event",
        "head",
    ] {
        assert!(
            !written.to_lowercase().contains(derived),
            "the repository holds {derived:?}, which is derived rather than supplied"
        );
    }

    // And what must: the recording instants, which nothing recomputes.
    assert!(written.contains("2026-01-05"), "the commitment's instant");
    assert!(written.contains("2026-01-12"), "the Event's instant");

    // The lineage survives the same question. A `ThesisId` is derived from the selection a
    // cut resolves, so writing one down would keep an answer beside the question it comes
    // from — and the decisions are what a fresh process needs to ask it again.
    let read = repository.read_lineage().expect("the lineage reads back");
    assert_eq!(read.len(), decisions.len());

    let written = std::fs::read_to_string(repository.lineage_path()).expect("the file is there");

    let reached = lineage::replay(canon.history(), &decisions).expect("the lineage rebuilds");

    for thesis in [&genesis, reached.last().expect("the advanced Thesis")] {
        assert!(
            !written.contains(&thesis.id().to_string()),
            "the repository holds a derived Thesis identity"
        );
    }
    assert!(written.contains("2026-01-10"), "the genesis instant");
    assert!(written.contains("2026-01-15"), "the advancement instant");
}

/// Phase 4 — Terminate.
///
/// The original process is dead, and here that is literal rather than agreed: the world is
/// rebuilt by the `ape-cli` binary, in an operating-system process of its own, which shares
/// no memory with the one that built it. None of the things the procedure forbids can carry
/// correctness across that boundary, because none of them can cross it at all.
///
/// What the child is given is a repository path and the explicit inputs needed to ask for
/// the same interpretation. That the repository is doing the work rather than something
/// ambient is what the second half of this test establishes: without it, the same command
/// on the same machine fails.
#[test]
fn phase_4_terminate() {
    let repository = Repository::open(scratch("phase-4"));
    let (commitment, instance) = persisted(&repository);

    let dead = std::path::Path::new(env!("CARGO_BIN_EXE_ape-cli"));

    let refused = reconstruct_in_fresh_process(dead, &scratch("phase-4-absent"), commitment, instance);

    assert!(
        !refused.status.success(),
        "a fresh process with no repository must not produce a world"
    );

    let survived = reconstruct_in_fresh_process(dead, repository.root(), commitment, instance);

    assert!(
        survived.status.success(),
        "the fresh process failed: {}",
        String::from_utf8_lossy(&survived.stderr)
    );

    let reading: Reading = serde_json::from_slice(&survived.stdout).expect("a reading came back");

    // Phase 7 is what weighs these against the living world. What Phase 4 establishes is
    // narrower and has to come first: a process that shared nothing produced a world at all.
    assert_eq!(reading.effective_at, "2026-01-15");
    assert_eq!(reading.outcome, OutcomeRecord::Fulfilled);
}

/// Build the world through Phases 1 to 3 and leave it on disk.
fn persisted(repository: &Repository) -> (CommitmentId, ResourceInstanceId) {
    let (mut canon, subject, decisions, _) = constructed();

    let mut journal = subject.journal;
    let settlement = subject::settlement(subject.commitment);
    journal::replay(&mut canon, std::slice::from_ref(&settlement)).expect("the Event admits");
    journal.push(settlement);

    let mut decisions = decisions;
    decisions.push(subject::advancement());

    repository.write_journal(&journal).expect("writable");
    repository.write_lineage(&decisions).expect("writable");

    (subject.commitment, subject.instance)
}

fn reconstruct_in_fresh_process(
    binary: &std::path::Path,
    repository: &std::path::Path,
    commitment: CommitmentId,
    instance: ResourceInstanceId,
) -> std::process::Output {
    std::process::Command::new(binary)
        .arg(repository)
        .arg(commitment.to_string())
        .arg(instance.to_string())
        .arg("2026-01-15")
        .output()
        .expect("the binary runs")
}

fn scratch(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join("ape-reconstruction").join(name);
    let _ = std::fs::remove_dir_all(&path);
    path
}
