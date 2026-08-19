//! The divergence experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, for the reason the reconstruction experiment
//! gives: a comparison has to fail loudly. What is different here is how many worlds are
//! under comparison — a lineage rather than a world — so a phase records what it leaves
//! behind for *every* world it produced.

use std::collections::BTreeSet;

use ape::canon::{Canon, CanonicalHistory};
use ape::engine::hermeneia::{Conflict, Hypothesis, Outcome, Timeliness};
use ape::engine::thesis::{Interpretation, KnowledgeCut};
use ape::kernel::value_objects::Date;

use ape_cli::history::ResidentHistory;
use ape_cli::journal;
use ape_cli::level;
use ape_cli::lineage::{self, Decision, Lineage, Taken};
use ape_cli::reading::{self, ConflictRecord, OutcomeRecord, Reading, WorldRecord};
use ape_cli::repository::Repository;
use ape_cli_lab::subject::divergence::{self, Begun, Reasoned};

/// The instant every world is interpreted at, from Phase 5 on.
///
/// It sits past the deadline the first two commitments carry and inside the one the
/// alternative carries, so a comparison of conditions has something to disagree about beyond
/// settlement.
const EFFECTIVE: &str = "2026-01-25";

/// A repository path no other process shares.
///
/// The process id is part of it because two runs of this laboratory once wrote to the same
/// path and read each other's repositories back. A candidate repair measured against another
/// candidate's file is not evidence about either.
fn scratch(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir()
        .join(format!("ape-divergence-{}", std::process::id()))
        .join(name);
    let _ = std::fs::remove_dir_all(&path);
    path
}

fn day(day: u8) -> Date {
    Date::from_ymd(2026, 1, day).expect("a real date in January 2026")
}

/// The world as Phase 1 leaves it: the subject admitted, and a Genesis Thesis selecting both
/// commitments at a cut taken before any Event exists.
///
/// The Thesis comes from applying a decision rather than from a call written here. That is a
/// finding of the previous experiment rather than a preference: what a later process gets is
/// the decision, so the decision is what every phase must go through.
fn constructed() -> Begun {
    divergence::begun().expect("the subject is admissible")
}

/// Run the procedure through Phase 3.
///
/// The sequence lives beside the subject rather than here, because the order in which
/// admissions and decisions interleave *is* the subject. A harness holding its own copy of it
/// would be able to drift from the arrangement it claims to observe.
fn reasoned() -> Reasoned {
    divergence::reasoned().expect("the arrangement holds")
}

/// Phase 1 — Construct.
///
/// The subject is admitted and a Genesis Thesis selects both commitments, at a cut no Event
/// has reached. The world it denotes is one the resource's own bounds refuse, and that
/// refusal is the point: a verdict can be reproduced, whereas the previous experiment's
/// feasibility comparison could only compare two absences.
#[test]
fn phase_1_construct() {
    let Begun {
        canon,
        subject,
        lineage,
        ..
    } = constructed();
    let thesis = lineage.decided().last().expect("the genesis");

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
        thesis.selection().open().collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.inflow, subject.overspend]),
        "both commitments are open"
    );

    let interpretation =
        Interpretation::of(thesis, canon.history()).expect("the Thesis is interpretable");

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

/// Phase 2 — Observe.
///
/// An Event cancels the overspend, recorded within the instant the genesis names and after
/// the genesis was decided. The world is then advanced to recognize it, and what the
/// advancement produces is recorded alongside what history imposed.
///
/// The phase also records what the arrangement costs, because it is already visible here and
/// no process has died: the instant the genesis decided at no longer resolves to the cut it
/// resolved to then.
#[test]
fn phase_2_observe() {
    let Begun {
        mut canon,
        subject,
        decisions,
        lineage,
    } = constructed();
    let genesis = lineage.decided().last().expect("the genesis").clone();

    journal::replay(&mut canon, &[divergence::cancellation(subject.overspend)])
        .expect("an observation the statement can cancel with");

    let event = canon
        .history()
        .head()
        .expect("the cancelling Event is now the head");

    // The world the application decided is unmoved. A cut is a value and holds the head it
    // resolved, so an Event recorded *within its own instant* is no more visible to it than
    // one recorded later — sharing the instant was never what let a cut recognize an Event.
    let held = Interpretation::of(&genesis, canon.history())
        .expect("a Thesis stays interpretable after history moves");

    assert_eq!(
        held.conditions_at(&day(10))
            .expect("conditions project at an instant")
            .condition(subject.overspend)
            .expect("the selected commitment has a condition")
            .outcome(),
        &Outcome::Unsettled,
        "the world that was decided does not learn"
    );

    assert_eq!(
        held.feasibility_under(Hypothesis::FinalState)
            .expect("feasibility is derivable")
            .conflicts(),
        [Conflict::OutOfBounds {
            instance: subject.instance,
            level: -70.0,
        }],
        "and stays refused, because what refused it has not changed"
    );

    // What did change is the instant. The decision names `2026-01-10`, and that instant now
    // resolves a different cut than the one it resolved when the decision was taken.
    assert_eq!(
        KnowledgeCut::at(canon.history(), day(10)).event_head(),
        Some(event),
        "the instant the genesis named has since acquired a head"
    );

    // So applying the same decision again builds a different world. Nothing has been
    // persisted and no process has died: re-deriving a decision against knowledge that moved
    // is sufficient on its own.
    let mut rederived = Lineage::new();
    lineage::decide(canon.history(), &mut rederived, &decisions[0].decision)
        .expect("the genesis decision still applies");
    let rederived = rederived
        .decided()
        .last()
        .expect("the world it produced")
        .clone();

    assert_ne!(
        rederived.id(),
        genesis.id(),
        "the same decision produced a different world"
    );
    assert_eq!(
        rederived.selection().frozen().collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.overspend]),
        "what was open when the decision was taken comes back unavoidable"
    );
    assert_eq!(
        rederived.selection().open().collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.inflow]),
    );
    assert!(
        Interpretation::of(&rederived, canon.history())
            .expect("the rederived world interprets")
            .feasibility_under(Hypothesis::FinalState)
            .expect("feasibility is derivable")
            .conflicts()
            .is_empty(),
        "and the world that was refused comes back unrefused"
    );

    // The advancement, decided the way an application decides: against the lineage it holds,
    // at the moment it takes it.
    let mut lineage = lineage;
    let imposed = lineage::decide(
        canon.history(),
        &mut lineage,
        &divergence::advancement(genesis.id()),
    )
    .expect("a later cut over the same intention");

    // Recorded rather than established. History settled nothing the genesis had not already
    // selected, so this subject cannot make an imposition happen — hard-wiring the report to
    // empty passes here. What losing it would cost is a question this experiment leaves open.
    assert!(imposed.is_empty(), "nothing was imposed, found {imposed:?}");

    let advanced = lineage.decided().last().expect("the advanced world");

    assert_eq!(advanced.parent(), &Some(genesis.id()));
    assert_eq!(advanced.cut().known_at(), &day(15));
    assert_eq!(advanced.cut().event_head(), Some(event));
    assert_eq!(
        advanced.selection().frozen().collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.overspend]),
        "recognizing the cancellation makes the overspend unavoidable"
    );
    assert_eq!(
        advanced.selection().open().collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.inflow]),
    );

    let interpretation =
        Interpretation::of(advanced, canon.history()).expect("the advanced Thesis interprets");

    let projected = interpretation
        .conditions_at(&day(15))
        .expect("conditions project at an instant");

    let cancelled = projected
        .condition(subject.overspend)
        .expect("the selected commitment has a condition");

    assert_eq!(cancelled.outcome(), &Outcome::Cancelled);
    assert_eq!(cancelled.timeliness(), None);

    let waiting = projected
        .condition(subject.inflow)
        .expect("the selected commitment has a condition");

    assert_eq!(waiting.outcome(), &Outcome::Unsettled);
    assert_eq!(waiting.timeliness(), Some(&Timeliness::WithinDeadline));

    // Nothing was fulfilled, so the account still holds nothing. A cancellation settles a
    // commitment without moving a level, which is what makes the refusal disappear without
    // anything having landed.
    assert_eq!(
        level::settled(canon.history(), &projected, subject.instance)
            .expect("the world reads whole"),
        0.0,
    );

    assert!(
        interpretation
            .feasibility_under(Hypothesis::FinalState)
            .expect("feasibility is derivable")
            .conflicts()
            .is_empty(),
        "an inflow of 50 alone stays within 0..100"
    );
}

/// Phase 3 — Diverge.
///
/// An affordable outflow is admitted, and a fork of the advanced world introduces it. What
/// the fork adds to the lineage is a selection reached by choice: an advancement recognizes
/// what history did, and this is the application answering it.
///
/// Three worlds have now been reasoned about. Each is a result, and Phase 8 asks for each.
#[test]
fn phase_3_diverge() {
    let Reasoned {
        canon,
        subject,
        alternative,
        lineage,
        ..
    } = reasoned();

    let lineage = lineage.decided();
    assert_eq!(lineage.len(), 3, "genesis, advancement, fork");

    let (advanced, forked) = (&lineage[1], &lineage[2]);

    // The affordable outflow was admitted after the advanced world was decided, and that
    // world stays decided without it. Knowledge is not intention.
    assert!(
        !advanced.selection().contains(alternative),
        "admitting a commitment does not enter it into a world"
    );

    // A fork moves one axis. The cut is the parent's — the same instant and the same head —
    // so whatever differs between the two worlds is intention rather than knowledge.
    assert_eq!(forked.parent(), &Some(advanced.id()));
    assert_eq!(forked.cut(), advanced.cut());

    assert_eq!(
        forked.selection().frozen().collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.overspend]),
        "a fork inherits its parent's frozen past unchanged"
    );
    assert_eq!(
        forked.selection().open().collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.inflow, alternative]),
        "and revises only what was open"
    );

    let interpretation =
        Interpretation::of(forked, canon.history()).expect("the forked world interprets");

    let projected = interpretation
        .conditions_at(&day(15))
        .expect("conditions project at an instant");

    for (label, id) in [("inflow", subject.inflow), ("alternative", alternative)] {
        let condition = projected
            .condition(id)
            .expect("a selected commitment has a condition");

        assert_eq!(condition.outcome(), &Outcome::Unsettled, "{label}");
        assert_eq!(
            condition.timeliness(),
            Some(&Timeliness::WithinDeadline),
            "{label}"
        );
    }

    assert_eq!(
        projected
            .condition(subject.overspend)
            .expect("the frozen commitment has a condition")
            .outcome(),
        &Outcome::Cancelled,
        "the fork carries its parent's settled past"
    );

    assert_eq!(
        level::settled(canon.history(), &projected, subject.instance)
            .expect("the world reads whole"),
        0.0,
        "choosing an intention lands nothing",
    );

    // 50 in, 30 out, and the cancelled overspend moving nothing: the account ends at 20. The
    // world the application forked to is one its own bounds admit, which is what the fork was
    // for.
    assert!(
        interpretation
            .feasibility_under(Hypothesis::FinalState)
            .expect("feasibility is derivable")
            .conflicts()
            .is_empty(),
        "an inflow of 50 against an outflow of 30 stays within 0..100"
    );
}

/// Phase 4 — Persist.
///
/// Only what a later process cannot derive is written down, which for a lineage of three
/// worlds is still two sequences: what became known, and what was decided. Neither holds a
/// world.
///
/// The discipline is the previous experiment's and is not relaxed here. What this phase adds
/// is a third kind of datum to hold it against — a fork's request — and a fourth the rule
/// could never have asked for: the entry each decision was taken after.
///
/// That is the answer to Observation 2, and the observation stands. "Nothing derived is
/// persisted" is a rule about what is written and says nothing about what is missing; the
/// coordinate was found by a world failing to come back, not by auditing the fields that
/// were there. Both audits are asserted below, and the second is the closed field set.
#[test]
fn phase_4_persist() {
    let Reasoned {
        canon,
        journal,
        decisions,
        lineage,
        subject,
        alternative,
    } = reasoned();

    let repository = Repository::open(scratch("phase-4"));
    repository
        .write_journal(&journal)
        .expect("the repository is writable");
    repository
        .write_lineage(&decisions)
        .expect("the repository is writable");

    let read = repository.read_journal().expect("the journal reads back");
    assert_eq!(
        read.len(),
        journal.len(),
        "every admission was kept, and none was invented"
    );

    let read = repository.read_lineage().expect("the lineage reads back");
    assert_eq!(read.len(), 3, "the genesis, the advancement and the fork");

    let written = std::fs::read_to_string(repository.journal_path()).expect("the file is there")
        + &std::fs::read_to_string(repository.lineage_path()).expect("the file is there");

    // Each of these is derived from what sits above it, and storing one would keep an answer
    // beside the question it comes from. `frozen` and `open` join the list this experiment
    // inherited, because a partition is a function of a cut and not a fact about a decision.
    for derived in [
        "level",
        "outcome",
        "fulfilled",
        "cancelled",
        "timeliness",
        "breached",
        "condition",
        "feasib",
        "conflict",
        "thesis",
        "previous_event",
        "head",
        "frozen",
        "open",
        "imposed",
    ] {
        assert!(
            !written.to_lowercase().contains(derived),
            "the repository holds {derived:?}, which is derived rather than supplied"
        );
    }

    // A world identity does appear now, and only where a decision extends one. The change is
    // a result of the convergence experiment: a lineage that may branch has to say which world
    // each decision is about, and the reference is weighed on every read by having to resolve
    // against a world the earlier decisions produce.
    //
    // The world *nothing* extends is still absent, and that is the half this phase was always
    // asking about — a decision records what it asked for, never what it got.
    let lineage = lineage.decided();

    for (position, extended) in [(0, true), (1, true), (2, false)] {
        assert_eq!(
            written.contains(&lineage[position].id().to_string()),
            extended,
            "world {position} is referenced by a later decision: {extended}"
        );
    }

    // And what must be there. Each answers the phase's question with a reconstruction that
    // becomes impossible without it.
    for (datum, instant) in [
        ("the commitments were recorded", "2026-01-05"),
        ("the cancellation was recorded", "2026-01-10"),
        ("the alternative was recorded", "2026-01-11"),
        ("the advancement was decided", "2026-01-15"),
    ] {
        assert!(written.contains(instant), "{datum}");
    }

    assert!(
        written.contains(&subject.inflow.to_string())
            && written.contains(&subject.overspend.to_string()),
        "the genesis records the selection it proposed"
    );
    assert!(
        written.contains(&alternative.to_string()),
        "the fork records the commitment it asked to introduce"
    );

    // The coordinate, and the only datum here that nothing else in either file already says.
    // The cancelling Event's identity appears in no admission — an Event's record names the
    // commitment it settles, and the identity is derived by admitting it — so a lineage
    // holding it is a lineage saying *when* the advancement was taken. Without it, that
    // decision resolves against a journal that has since grown.
    let cancellation = canon
        .history()
        .head()
        .expect("the cancelling Event is the chain");

    assert!(
        written.contains(&cancellation.to_string()),
        "the advancement records the entry it was taken after"
    );

    // The whole of what a decision records, named field by field. The phase's question is
    // asked of every datum, so the set has to be closed rather than sampled.
    //
    // `after` is what the experiment added, and it answers the question the same way the
    // recording instants do: without it the journal knows the cancellation was recorded on
    // the tenth, the lineage knows the genesis was decided on the tenth, and nothing in
    // either says which came first. It is a reference rather than a derivation — the entry
    // it addresses is re-derived from content on every replay, and an address that named
    // nothing would be refused instead of believed.
    //
    // `extends` belongs to the convergence experiment and appears here because a decision has
    // one shape everywhere. In this arrangement it names the world decided last, which order
    // already said — so it buys nothing *here*, and this phase's own rule would drop it. What
    // it buys is a lineage that branches, which this subject cannot arrange; recording that
    // honestly is what keeps the set closed rather than approximately closed.
    let recorded: Vec<BTreeSet<String>> = serde_json::from_str::<Vec<serde_json::Value>>(
        &std::fs::read_to_string(repository.lineage_path()).expect("the file is there"),
    )
    .expect("the lineage is a list of objects")
    .iter()
    .map(|decision| {
        decision
            .as_object()
            .expect("a decision is an object")
            .keys()
            .cloned()
            .collect()
    })
    .collect();

    assert_eq!(
        recorded,
        [
            BTreeSet::from([
                "decides".to_owned(),
                "known_at".into(),
                "selection".into(),
                "after".into(),
                "witness".into(),
            ]),
            BTreeSet::from([
                "decides".to_owned(),
                "known_at".into(),
                "extends".into(),
                "after".into(),
                "witness".into(),
            ]),
            BTreeSet::from([
                "decides".to_owned(),
                "omitted".into(),
                "introduced".into(),
                "extends".into(),
                "after".into(),
                "witness".into(),
            ]),
        ],
        "a decision records an instant, an intention, which world it extends, and where in \
         the sequence it was taken"
    );

    // The other audit, which the rule above cannot perform: the coordinate has to *address*
    // something. It is the one datum in either file that can dangle — an instant is read as
    // an instant and a selection names commitments the engine refuses to select if absent,
    // whereas an address naming nothing is a hex string like any other.
    //
    // So it is derived rather than compared to a copy: the journal is replayed, and what it
    // produces is the closed set of entries a decision may follow.
    let addressable = journal::replay(
        &mut Canon::new(ResidentHistory::new()),
        &repository.read_journal().expect("the journal reads back"),
    )
    .expect("the journal admits")
    .entries;

    for (position, taken) in repository
        .read_lineage()
        .expect("the lineage reads back")
        .iter()
        .enumerate()
    {
        assert!(
            addressable.contains(&taken.after),
            "decision {position} follows {}, which the journal never admits",
            taken.after
        );
    }
}

/// Phases 5 to 7 — Terminate, Reload, Reconstruct.
///
/// The original process is dead, and here that is literal: the lineage is rebuilt by the
/// `ape-cli` binary in an operating-system process of its own, which shares no memory with
/// the one that reasoned. What it is given is a repository path, a resource instance and an
/// instant — nothing the original process computed can reach it, because none of it can
/// cross.
///
/// What these phases establish is narrower than agreement, and has to come first: a process
/// that shared nothing produced *three* worlds. Whether they are the three that were reasoned
/// about is Phase 8.
#[test]
fn phase_5_terminate() {
    let reasoned = reasoned();
    let repository = Repository::open(scratch("phase-5"));
    persist(&repository, &reasoned);

    let dead = &ape_cli_lab::binary();

    let refused =
        rebuild_in_fresh_process(dead, &scratch("phase-5-absent"), reasoned.subject.instance);

    assert!(
        !refused.status.success(),
        "a fresh process with no repository must not produce a world"
    );

    let survived = rebuild_in_fresh_process(dead, repository.root(), reasoned.subject.instance);

    assert!(
        survived.status.success(),
        "the fresh process failed: {}",
        String::from_utf8_lossy(&survived.stderr)
    );

    let rebuilt: Vec<Reading> =
        serde_json::from_slice(&survived.stdout).expect("a lineage came back");

    assert_eq!(
        rebuilt.len(),
        3,
        "the repository yields every world it holds decisions for, not only the last"
    );

    // Ancestry survived as a chain rather than as three unrelated worlds. This says nothing
    // yet about whether they are the right three.
    assert_eq!(
        rebuilt[0].thesis_parent, None,
        "the lineage begins at a genesis"
    );
    assert_eq!(
        rebuilt[1].thesis_parent.as_deref(),
        Some(rebuilt[0].thesis.as_str())
    );
    assert_eq!(
        rebuilt[2].thesis_parent.as_deref(),
        Some(rebuilt[1].thesis.as_str())
    );

    assert_eq!(rebuilt[2].effective_at, EFFECTIVE);
}

/// Phase 8 — Compare.
///
/// Each reconstructed world against the one it reproduces. Every coordinate the protocol
/// names is asserted by name, so a divergence says which one moved rather than that something
/// did, and then the readings are compared whole — a coordinate nobody thought to list is
/// exactly the kind a list misses.
///
/// Three worlds, not one. Observation 3 established that a Thesis is identified by its
/// ancestry too, so the genesis carries its descendants: the identity of the third world is
/// evidence about the first, and comparing only the tip would have measured nothing.
///
/// The literals written down before the run are asserted separately, and they answer a
/// different question. Equality compares two readings produced by one implementation and
/// survives that implementation drifting; the literals do not.
#[test]
fn phase_8_compare() {
    let reasoned = reasoned();
    let repository = Repository::open(scratch("phase-8"));
    persist(&repository, &reasoned);

    let effective = Date::parse(EFFECTIVE).expect("the effective instant is a date");

    let before = reading::all(
        reasoned.canon.history(),
        reasoned.lineage.decided(),
        reasoned.subject.instance,
        &effective,
    )
    .expect("the living lineage reads");

    let rebuilt = rebuild_in_fresh_process(
        &ape_cli_lab::binary(),
        repository.root(),
        reasoned.subject.instance,
    );

    // Asked before the output is parsed, so that a fresh process which refused says why. A
    // reconstruction that fails outright is a different finding from one that disagrees, and
    // reading empty stdout as malformed JSON would report neither.
    assert!(
        rebuilt.status.success(),
        "the fresh process failed: {}",
        String::from_utf8_lossy(&rebuilt.stderr)
    );

    let after: Vec<Reading> = serde_json::from_slice(&rebuilt.stdout).expect("a lineage came back");

    assert_eq!(after.len(), before.len(), "three worlds either way");

    // Every world, coordinate by coordinate. Identity is asserted first and separately
    // because it is the only one that reports ancestry: two worlds can agree about
    // everything they say and still be different worlds.
    for (position, (before, after)) in before.iter().zip(&after).enumerate() {
        assert_eq!(
            before.thesis, after.thesis,
            "world {position} comes back under the identity it had"
        );
        assert_eq!(
            before.thesis_parent, after.thesis_parent,
            "world {position} descends from the world it descended from"
        );
        assert_eq!(
            (&before.known_at, &before.event_head),
            (&after.known_at, &after.event_head),
            "world {position} recognizes the same instant and the same head"
        );
        assert_eq!(
            (&before.frozen, &before.open),
            (&after.frozen, &after.open),
            "world {position} partitions the same commitments into the same halves"
        );
        assert_eq!(
            before.conditions, after.conditions,
            "world {position} projects the same conditions"
        );
        assert_eq!(
            before.level, after.level,
            "world {position} folds to the same level"
        );
        assert_eq!(
            before.conflicts, after.conflicts,
            "world {position} reaches the same verdict"
        );
        assert_eq!(
            before, after,
            "world {position} agrees in every coordinate, including the ones not named above"
        );
    }

    // The genesis, written down before the run. It is the world the arrangement aimed at: an
    // Event was recorded within the instant it names, after it was decided, and it comes back
    // at the empty chain it was decided against rather than at the one that instant addresses
    // now.
    let (genesis, overspend) = (&after[0], reasoned.subject.overspend.to_string());

    assert_eq!(genesis.thesis_parent, None, "a genesis descends from none");
    assert_eq!(genesis.known_at, "2026-01-10");
    assert_eq!(
        genesis.event_head, None,
        "the instant addresses a head now, and the world decided at it does not"
    );
    assert_ne!(
        genesis.event_head, genesis.canonical_head,
        "canonical history moved past it, which is the whole arrangement"
    );

    assert!(
        genesis.frozen.is_empty(),
        "nothing had settled, so nothing is unavoidable"
    );
    assert_eq!(
        genesis.open,
        BTreeSet::from([
            reasoned.subject.inflow.to_string(),
            reasoned.subject.overspend.to_string(),
        ]),
        "both commitments are still open to a fork"
    );
    assert_eq!(
        genesis.conditions[&overspend].outcome,
        OutcomeRecord::Unsettled,
        "the reconstruction does not learn an Event the decision could not have known"
    );
    assert_eq!(genesis.level, 0.0, "nothing was ever fulfilled");
    assert_eq!(
        genesis.conflicts,
        [ConflictRecord::OutOfBounds {
            instance: reasoned.subject.instance.to_string(),
            level: -70.0,
        }],
        "the refused world comes back refused, by the same instance at the same level"
    );

    // The advancement: the one world whose cut is supposed to have moved.
    let advanced = &after[1];

    assert_eq!(
        advanced.thesis_parent.as_deref(),
        Some(genesis.thesis.as_str())
    );
    assert_eq!(advanced.known_at, "2026-01-15");
    assert_eq!(
        advanced.event_head, advanced.canonical_head,
        "recognizing later history means recognizing the cancellation"
    );
    assert_eq!(
        advanced.frozen,
        BTreeSet::from([reasoned.subject.overspend.to_string()]),
        "which makes the overspend unavoidable"
    );
    assert_eq!(
        advanced.open,
        BTreeSet::from([reasoned.subject.inflow.to_string()])
    );
    assert!(
        advanced.conflicts.is_empty(),
        "and a cancelled commitment moves no level, so the refusal is gone"
    );

    // The fork: the same cut, and what the repository says it asked for. A fork's request is
    // an outcome rather than a transition, so it is read from the decision rather than
    // inferred by comparing two selections.
    let forked = &after[2];

    let Taken {
        decision:
            Decision::Fork {
                omitted,
                introduced,
                ..
            },
        ..
    } = &repository.read_lineage().expect("the lineage reads back")[2]
    else {
        panic!("the third decision is the fork");
    };

    assert!(omitted.is_empty(), "the fork withdrew no intention");
    assert_eq!(
        introduced,
        &BTreeSet::from([reasoned.alternative]),
        "and asked for exactly the affordable outflow"
    );

    assert_eq!(
        forked.thesis_parent.as_deref(),
        Some(advanced.thesis.as_str())
    );
    assert_eq!(
        (&forked.known_at, &forked.event_head),
        (&advanced.known_at, &advanced.event_head),
        "a fork inherits its parent's cut"
    );
    assert_eq!(
        forked.frozen, advanced.frozen,
        "and its parent's frozen past unchanged"
    );
    assert_eq!(
        forked.open,
        BTreeSet::from([
            reasoned.subject.inflow.to_string(),
            reasoned.alternative.to_string(),
        ]),
        "revising only what was open, by what the decision introduced"
    );
    assert_eq!(forked.level, 0.0, "choosing an intention lands nothing");
    assert_eq!(forked.effective_at, EFFECTIVE);
    assert!(
        forked.conflicts.is_empty(),
        "and the account the application forked to ends inside its bounds"
    );
}

/// Leave the reasoned lineage on disk, exactly as Phase 4 does.
fn persist(repository: &Repository, reasoned: &Reasoned) {
    repository
        .write_journal(&reasoned.journal)
        .expect("the repository is writable");
    repository
        .write_lineage(&reasoned.decisions)
        .expect("the repository is writable");
    repository
        .write_worlds(
            &reasoned
                .lineage
                .decided()
                .iter()
                .map(WorldRecord::of)
                .collect::<Vec<_>>(),
        )
        .expect("the repository is writable");
}

fn rebuild_in_fresh_process(
    binary: &std::path::Path,
    repository: &std::path::Path,
    instance: ape::kernel::entities::ResourceInstanceId,
) -> std::process::Output {
    std::process::Command::new(binary)
        .arg(repository)
        .arg(instance.to_string())
        .arg(EFFECTIVE)
        .output()
        .expect("the binary runs")
}
