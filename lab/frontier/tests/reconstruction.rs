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
use ape_cli::lineage::{self, Decision, Taken};
use ape_cli::reading::{self, OutcomeRecord, Reading, WorldRecord};
use ape_cli::repository::Repository;
use ape_frontier::subject::reconstruction::{self as subject, Constructed};

fn day(day: u8) -> Date {
    Date::from_ymd(2026, 1, day).expect("a real date in January 2026")
}

/// The prefix a decision taken now stands on: what the subject admitted, and what has been
/// admitted since.
///
/// The two arrive separately here because this experiment admits its settlement outside the
/// subject, which was harmless while a decision recorded only where it fell. A decision now
/// carries the knowledge it was taken against, so the halves have to be put back together.
fn prefix(subject: &Constructed, since: &journal::Replayed) -> journal::Replayed {
    let mut admitted = subject.admitted.clone();
    admitted.entries.extend(since.entries.iter().cloned());
    admitted
}

/// The decisions alone, with the coordinate that places each of them dropped.
///
/// Dropping it is what [`lineage::replay`] does, and doing it at the call site is so that it
/// is visible. This subject admits nothing within an instant one of its decisions names, so
/// the two forms agree here — which is the condition, and the reason the divergence
/// experiment needed a subject of its own to find that out.
fn intentions(lineage: &[Taken]) -> Vec<Decision> {
    lineage.iter().map(|taken| taken.decision.clone()).collect()
}

/// The world as Phase 1 leaves it: the subject admitted, and a Genesis Thesis selecting it
/// at a cut taken before any Event exists.
///
/// The Thesis comes from replaying a decision rather than from a call written here, for the
/// same reason the subject comes from a journal: what a later process gets is the decision,
/// so the decision is what every phase must go through.
fn constructed() -> (Canon<ResidentHistory>, Constructed, Vec<Taken>, Thesis) {
    let mut canon = Canon::new(ResidentHistory::new());
    let subject = subject::construct(&mut canon).expect("the subject is admissible");

    // The commitment is the last thing the subject admits, so it is the entry the genesis is
    // taken after — and the whole of what it was admitted alongside is what witnesses it.
    let decisions = vec![
        Taken::now(subject::genesis(subject.commitment), &subject.admitted)
            .expect("the subject admitted something"),
    ];
    let lineage = lineage::replay(canon.history(), &intentions(&decisions))
        .expect("a genesis over admitted knowledge");

    let thesis = lineage.decided().last().expect("one Thesis").clone();

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
    let admitted = journal::replay(&mut canon, &[subject::settlement(subject.commitment)])
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
    decisions.push(
        Taken::now(
            subject::advancement(genesis.id()),
            &prefix(&subject, &admitted),
        )
        .expect("the Event was admitted"),
    );

    let lineage = lineage::replay(canon.history(), &intentions(&decisions))
        .expect("a lineage the canonical knowledge supports");

    assert_eq!(
        lineage.decided().len(),
        2,
        "the genesis, and the world that succeeded it"
    );

    let advanced = lineage.decided().last().expect("the advanced Thesis");

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
        Interpretation::of(advanced, canon.history()).expect("the advanced Thesis interprets");

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

    let mut journal = subject.journal.clone();
    let settlement = subject::settlement(subject.commitment);
    let admitted =
        journal::replay(&mut canon, std::slice::from_ref(&settlement)).expect("the Event admits");
    journal.push(settlement);

    // The lineage the experiment reached: the world it first reasoned about, and the
    // recognition of the Event. Both are decisions, and neither is recoverable from
    // canonical knowledge — history says what became known, not which of it a world selects.
    let mut decisions = decisions;
    decisions.push(
        Taken::now(
            subject::advancement(genesis.id()),
            &prefix(&subject, &admitted),
        )
        .expect("the Event was admitted"),
    );

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

    // The lineage no longer survives that question whole, and the change is a result of the
    // convergence experiment rather than of this one. A decision now names the world it
    // extends, so a `ThesisId` does appear — as a reference, weighed on every read by having
    // to resolve against a world the earlier decisions produce.
    //
    // What a decision still never records is the world it *produced*. That is the half that
    // would be an answer kept beside its question, and nothing reads a world back either way:
    // a `Thesis` does not deserialize.
    let read = repository.read_lineage().expect("the lineage reads back");
    assert_eq!(read.len(), decisions.len());

    let written = std::fs::read_to_string(repository.lineage_path()).expect("the file is there");

    let reached =
        lineage::replay(canon.history(), &intentions(&decisions)).expect("the lineage rebuilds");

    assert!(
        written.contains(&genesis.id().to_string()),
        "the advancement names the world it extends"
    );
    assert!(
        !written.contains(
            &reached
                .decided()
                .last()
                .expect("the advanced")
                .id()
                .to_string()
        ),
        "the repository holds the world a decision produced"
    );
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
    let living = persisted(&repository);
    let (commitment, instance) = (living.commitment, living.instance);

    let dead = &ape_frontier::binary();

    let refused = reconstruct_in_fresh_process(dead, &scratch("phase-4-absent"), instance);

    assert!(
        !refused.status.success(),
        "a fresh process with no repository must not produce a world"
    );

    let survived = reconstruct_in_fresh_process(dead, repository.root(), instance);

    assert!(
        survived.status.success(),
        "the fresh process failed: {}",
        String::from_utf8_lossy(&survived.stderr)
    );

    let rebuilt: Vec<Reading> =
        serde_json::from_slice(&survived.stdout).expect("a lineage came back");
    let reading = rebuilt.last().expect("the world the lineage reached");

    // Phase 7 is what weighs these against the living world. What Phase 4 establishes is
    // narrower and has to come first: a process that shared nothing produced a world at all.
    assert_eq!(reading.effective_at, "2026-01-15");
    assert_eq!(
        reading.conditions[&commitment.to_string()].outcome,
        OutcomeRecord::Fulfilled
    );
}

/// Phase 7 — Compare.
///
/// Two readings of one world: one taken while the process that built it was alive, one
/// taken by a process that never met it. The protocol names six coordinates, and each is
/// asserted by name so that a divergence says which one moved rather than that something
/// did. The whole reading is then compared as well, because a coordinate the protocol did
/// not think to name is exactly the kind that would slip through a list.
///
/// The literals are asserted too, and they answer a different question. The equality above
/// proves reconstruction; it would keep proving it if both sides drifted together. The
/// literals are what the experiment expected before it ran, and they are the reason a
/// change of meaning cannot pass as a success.
#[test]
fn phase_7_compare() {
    let repository = Repository::open(scratch("phase-7"));
    let living = persisted(&repository);

    let rebuilt =
        reconstruct_in_fresh_process(&ape_frontier::binary(), repository.root(), living.instance);

    assert!(
        rebuilt.status.success(),
        "the fresh process failed: {}",
        String::from_utf8_lossy(&rebuilt.stderr)
    );

    let rebuilt: Vec<Reading> =
        serde_json::from_slice(&rebuilt.stdout).expect("a lineage came back");
    let after = rebuilt
        .last()
        .expect("the world the lineage reached")
        .clone();
    let before = living.reading;

    assert_eq!(
        before.canonical_head, after.canonical_head,
        "the canonical chain ends where it ended"
    );
    assert_eq!(
        before.thesis, after.thesis,
        "the Thesis has the identity it had, derived again rather than remembered"
    );
    assert_eq!(
        (&before.known_at, &before.event_head),
        (&after.known_at, &after.event_head),
        "the Knowledge Cut addresses the same moment"
    );
    assert_eq!(
        before.conditions, after.conditions,
        "the projected condition is the same condition"
    );
    assert_eq!(
        before.level, after.level,
        "the derived consequence on the resource is reproduced"
    );
    assert_eq!(
        before.conflicts, after.conflicts,
        "feasibility under the same hypothesis reaches the same verdict"
    );

    assert_eq!(
        before, after,
        "the readings agree in every coordinate, including the ones not named above"
    );

    // What the experiment expected before it ran, and the half that does the work when a
    // defect is in code both sides share. Equality compares two readings produced by one
    // implementation, so it survives that implementation moving; these do not, because they
    // were written down before it ran.
    let condition = &after.conditions[&living.commitment.to_string()];

    assert_eq!(
        condition.outcome,
        OutcomeRecord::Fulfilled,
        "the Event settled the commitment"
    );
    assert_eq!(
        condition.timeliness, None,
        "a settled commitment is under no deadline"
    );
    assert_eq!(after.level, 10.0, "the increase of 10 landed");
    assert!(
        after.conflicts.is_empty(),
        "the world stays within the resource bounds"
    );

    // The partition, which this experiment never asserted and which equality alone cannot
    // defend: both readings come from one implementation, so a `reading` that reported the
    // two halves the wrong way round would agree with itself. Added while the divergence
    // experiment was measuring exactly that.
    assert_eq!(
        after.frozen,
        std::collections::BTreeSet::from([living.commitment.to_string()]),
        "the settled commitment is what history made unavoidable"
    );
    assert!(
        after.open.is_empty(),
        "and nothing is left for a fork to revise"
    );
    assert_eq!(
        after.known_at, "2026-01-15",
        "the world recognizes history up to the instant the advancement decided"
    );
    assert_eq!(
        after.effective_at, "2026-01-15",
        "the interpretation was asked for at the instant requested"
    );
    assert!(after.event_head.is_some(), "the cut recognizes the Event");
    assert_eq!(
        after.event_head, after.canonical_head,
        "the world recognizes the whole chain"
    );
    assert!(
        after.thesis_parent.is_some(),
        "the world reached is a child of the genesis"
    );
}

/// The world Phases 1 to 3 produce: what it says about itself, and what it left on disk.
struct Living {
    commitment: CommitmentId,
    instance: ResourceInstanceId,
    reading: Reading,
}

/// Build the world through Phases 1 to 3, read it while it is alive, and leave it on disk.
fn persisted(repository: &Repository) -> Living {
    let (mut canon, subject, decisions, genesis) = constructed();

    let mut journal = subject.journal.clone();
    let settlement = subject::settlement(subject.commitment);
    let admitted =
        journal::replay(&mut canon, std::slice::from_ref(&settlement)).expect("the Event admits");
    journal.push(settlement);

    let mut decisions = decisions;
    decisions.push(
        Taken::now(
            subject::advancement(genesis.id()),
            &prefix(&subject, &admitted),
        )
        .expect("the Event was admitted"),
    );

    let lineage =
        lineage::replay(canon.history(), &intentions(&decisions)).expect("the lineage holds");

    let reading = reading::of(
        canon.history(),
        lineage
            .decided()
            .last()
            .expect("the world the lineage reached"),
        subject.instance,
        &day(15),
    )
    .expect("the living world reads");

    repository.write_journal(&journal).expect("writable");
    repository.write_lineage(&decisions).expect("writable");
    repository
        .write_worlds(
            &lineage
                .decided()
                .iter()
                .map(WorldRecord::of)
                .collect::<Vec<_>>(),
        )
        .expect("writable");

    Living {
        commitment: subject.commitment,
        instance: subject.instance,
        reading,
    }
}

fn reconstruct_in_fresh_process(
    binary: &std::path::Path,
    repository: &std::path::Path,
    instance: ResourceInstanceId,
) -> std::process::Output {
    std::process::Command::new(binary)
        .arg(repository)
        .arg(instance.to_string())
        .arg("2026-01-15")
        .output()
        .expect("the binary runs")
}

/// A repository path no other process shares. See the divergence harness for why.
fn scratch(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir()
        .join(format!("ape-reconstruction-{}", std::process::id()))
        .join(name);
    let _ = std::fs::remove_dir_all(&path);
    path
}
