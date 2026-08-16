//! The divergence experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, for the reason the reconstruction experiment
//! gives: a comparison has to fail loudly. What is different here is how many worlds are
//! under comparison — a lineage rather than a world — so a phase records what it leaves
//! behind for *every* world it produced.

use std::collections::BTreeSet;

use ape::canon::{Canon, CanonicalHistory};
use ape::engine::hermeneia::{Conflict, Hypothesis, Outcome, Timeliness};
use ape::engine::thesis::{Interpretation, KnowledgeCut, Thesis};
use ape::kernel::value_objects::Date;

use ape_cli::history::ResidentHistory;
use ape_cli::journal;
use ape_cli::level;
use ape_cli::lineage::{self, Decision};
use ape_cli::subject::divergence::{self, Constructed};

fn day(day: u8) -> Date {
    Date::from_ymd(2026, 1, day).expect("a real date in January 2026")
}

/// The world as Phase 1 leaves it: the subject admitted, and a Genesis Thesis selecting both
/// commitments at a cut taken before any Event exists.
///
/// The Thesis comes from applying a decision rather than from a call written here. That is a
/// finding of the previous experiment rather than a preference: what a later process gets is
/// the decision, so the decision is what every phase must go through.
///
/// The lineage is carried on rather than the world alone, because an application holds the
/// worlds it decided. Re-deriving them is what a reconstruction does, and this experiment is
/// about whether the two agree.
fn constructed() -> (
    Canon<ResidentHistory>,
    Constructed,
    Vec<Decision>,
    Vec<Thesis>,
) {
    let mut canon = Canon::new(ResidentHistory::new());
    let subject = divergence::construct(&mut canon).expect("the subject is admissible");

    let decisions = vec![divergence::genesis(subject.inflow, subject.overspend)];

    let mut lineage = Vec::new();
    lineage::decide(canon.history(), &mut lineage, &decisions[0])
        .expect("a genesis over admitted knowledge");

    (canon, subject, decisions, lineage)
}

/// Phase 1 — Construct.
///
/// The subject is admitted and a Genesis Thesis selects both commitments, at a cut no Event
/// has reached. The world it denotes is one the resource's own bounds refuse, and that
/// refusal is the point: a verdict can be reproduced, whereas the previous experiment's
/// feasibility comparison could only compare two absences.
#[test]
fn phase_1_construct() {
    let (canon, subject, _, lineage) = constructed();
    let thesis = lineage.last().expect("the genesis");

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
    let (mut canon, subject, decisions, lineage) = constructed();
    let genesis = lineage.last().expect("the genesis").clone();

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
    let mut rederived = Vec::new();
    lineage::decide(canon.history(), &mut rederived, &decisions[0])
        .expect("the genesis decision still applies");
    let rederived = rederived.pop().expect("the world it produced");

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
    let imposed = lineage::decide(canon.history(), &mut lineage, &divergence::advancement())
        .expect("a later cut over the same intention");

    // Recorded rather than established. History settled nothing the genesis had not already
    // selected, so this subject cannot make an imposition happen — hard-wiring the report to
    // empty passes here. What losing it would cost is a question this experiment leaves open.
    assert!(imposed.is_empty(), "nothing was imposed, found {imposed:?}");

    let advanced = lineage.last().expect("the advanced world");

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
    let (mut canon, subject, decisions, lineage) = constructed();
    let mut lineage = lineage;
    let mut decisions = decisions;

    journal::replay(&mut canon, &[divergence::cancellation(subject.overspend)])
        .expect("the cancellation admits");

    decisions.push(divergence::advancement());
    lineage::decide(canon.history(), &mut lineage, &decisions[1]).expect("the world advances");

    let advanced = lineage.last().expect("the advanced world").clone();

    // Admitted after the cancellation and before the advanced cut, so a fork may introduce
    // it. Admitting it selects nothing: knowledge is not intention.
    let admitted = journal::replay(&mut canon, &[divergence::alternative(&subject)])
        .expect("an affordable outflow admits");
    let alternative = admitted.commitments[0];

    assert!(
        !advanced.selection().contains(alternative),
        "admitting a commitment does not enter it into a world"
    );

    decisions.push(divergence::fork(alternative));
    let imposed = lineage::decide(canon.history(), &mut lineage, &decisions[2])
        .expect("a fork over the parent's cut");

    assert!(imposed.is_empty(), "a fork recognizes no new history");
    assert_eq!(lineage.len(), 3, "genesis, advancement, fork");

    let forked = lineage.last().expect("the forked world");

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
