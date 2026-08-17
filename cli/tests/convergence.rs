//! The convergence experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, for the reason the reconstruction experiment
//! gives: a comparison has to fail loudly. What is new here is that the worlds under
//! comparison no longer form a line, so a phase says which world it is talking about rather
//! than counting from the end.

use std::collections::BTreeSet;

use ape::engine::thesis::{ForkInput, ThesisError, descends_from};
use ape::kernel::value_objects::Date;

use ape_cli::lineage::{self, Lineage};
use ape_cli::reading::{self, OutcomeRecord, TimelinessRecord, WorldRecord};
use ape_cli::subject::convergence::{self, Branched, Diverged};

/// The instant every world is interpreted at.
///
/// It sits past the deadline the funding and the equipment carry and inside the one the
/// inventory carries, so the two lines of thinking differ in their conditions as well as in
/// what they select.
const EFFECTIVE: &str = "2026-01-22";

fn effective() -> Date {
    Date::parse(EFFECTIVE).expect("a real date")
}

/// The arrangement at Phase 1, and the arrangement whole.
///
/// Both live beside the subject rather than here, because the order in which admissions and
/// decisions interleave *is* the subject. Nothing in this harness calls `Thesis::fork` either:
/// what a later process gets is the decision, so the decision is what every phase goes through.
fn branched() -> Branched {
    convergence::branched().expect("the subject is admissible")
}

fn diverged() -> Diverged {
    convergence::diverged().expect("the arrangement holds")
}

/// Phase 1 — Branch.
///
/// The subject is admitted, a common ancestor is decided over it, and two decisions extend
/// that same ancestor. Neither sibling descends from the other, both descend from one world,
/// and both are read at an instant that tells them apart.
#[test]
fn phase_1_branch() {
    let arrangement = branched();
    let subject = &arrangement.subject;

    let ancestor = arrangement.ancestor();
    assert_eq!(ancestor.parent(), &None, "the ancestor begins the lineage");
    assert_eq!(
        ancestor.selection().open().collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.funding]),
        "the ancestor decides only that the account is funded"
    );
    assert_eq!(
        ancestor.selection().frozen().count(),
        0,
        "an empty chain makes nothing unavoidable"
    );

    // Both siblings extend the ancestor and inherit its cut. The second half is what makes
    // them siblings rather than two worlds that merely resemble each other: a fork moves
    // intention and never the cut, so anything they disagree about was decided.
    for (label, sibling) in [
        ("equipping", arrangement.equipping()),
        ("stocking", arrangement.stocking()),
    ] {
        assert_eq!(
            sibling.parent(),
            &Some(ancestor.id()),
            "{label} extends the ancestor"
        );
        assert_eq!(
            sibling.cut(),
            ancestor.cut(),
            "{label} inherits the ancestor's cut"
        );
        assert_eq!(
            sibling.selection().frozen().count(),
            0,
            "{label} freezes nothing"
        );
    }

    assert_eq!(
        arrangement
            .equipping()
            .selection()
            .open()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.funding, subject.equipment])
    );
    assert_eq!(
        arrangement
            .stocking()
            .selection()
            .open()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.funding, subject.inventory])
    );
    assert_ne!(
        arrangement.equipping().id(),
        arrangement.stocking().id(),
        "two intentions, two worlds"
    );

    // Interpreted, and recorded. Neither line of thinking is refused by the account's own
    // bounds — 60 − 30 and 60 − 40 both land inside them — so what separates them is what
    // they intend rather than whether they are possible at all.
    let readings = reading::all(
        arrangement.canon.history(),
        arrangement.lineage.decided(),
        subject.instance,
        &effective(),
    )
    .expect("every world reads");

    assert_eq!(readings.len(), 3);

    for (position, label) in [(1, "equipping"), (2, "stocking")] {
        let reading = &readings[position];

        assert_eq!(
            reading.thesis_parent,
            Some(ancestor.id().to_string()),
            "{label}"
        );
        assert_eq!(reading.known_at, "2026-01-10", "{label}");
        assert_eq!(reading.event_head, None, "{label}");
        assert!(reading.frozen.is_empty(), "{label}");
        assert_eq!(reading.level, 0.0, "{label}: nothing has settled");
        assert!(
            reading.conflicts.is_empty(),
            "{label}: the account carries it"
        );

        let funding = reading
            .conditions
            .get(&subject.funding.to_string())
            .expect("{label}: the funding is selected in both");

        assert_eq!(funding.outcome, OutcomeRecord::Unsettled, "{label}");
        assert_eq!(
            funding.timeliness,
            Some(TimelinessRecord::Breached),
            "{label}: the funding is due before the instant read at"
        );
    }

    // What each line of thinking holds that the other does not, and how the two differ at the
    // instant read at. The inventory is not yet due; the equipment is.
    let equipping = &readings[1].conditions;
    let stocking = &readings[2].conditions;

    assert_eq!(
        equipping[&subject.equipment.to_string()].timeliness,
        Some(TimelinessRecord::Breached)
    );
    assert!(!equipping.contains_key(&subject.inventory.to_string()));

    assert_eq!(
        stocking[&subject.inventory.to_string()].timeliness,
        Some(TimelinessRecord::WithinDeadline)
    );
    assert!(!stocking.contains_key(&subject.equipment.to_string()));
}

/// Phase 2 — Diverge in two directions.
///
/// Each line takes a decision of its own, so neither is a prefix of the other and the
/// difference between them is two steps rather than one. What separates the two decisions is
/// the arrangement's second half: one line could decide, and the other had to advance first.
#[test]
fn phase_2_diverge() {
    let arrangement = diverged();
    let subject = &arrangement.subject;

    assert_eq!(
        arrangement.lineage.decided().len(),
        6,
        "the ancestor, two forks, and one more decision on each side"
    );

    // The inventory line's second decision. `M` was recorded on the fourth and the ancestor's
    // cut is the tenth, so it was already knowledge nobody had selected — an intention is free
    // to take it up with no history moving at all.
    let maintaining = arrangement.maintaining();

    assert_eq!(
        maintaining.parent(),
        &Some(arrangement.stocking().id()),
        "the inventory line's second decision extends its first"
    );
    assert_eq!(
        maintaining.cut(),
        arrangement.ancestor().cut(),
        "a fork inherits its parent's cut, and this line has never advanced"
    );
    assert_eq!(
        maintaining.selection().open().collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.funding, subject.inventory, subject.maintenance]),
        "the inventory line holds the funding, the inventory and the maintenance"
    );

    // The equipment line's, which took two decisions to reach because `N` did not exist when
    // the line forked. Advancing changes the cut and no intention; forking changes the
    // intention and no cut. Keeping them apart is what lets each ancestry edge say which
    // happened.
    let advanced = arrangement.advanced();

    assert_eq!(
        advanced.parent(),
        &Some(arrangement.equipping().id()),
        "the equipment line advances from where it forked"
    );
    assert_eq!(advanced.cut().known_at().to_iso(), "2026-01-16");
    assert_eq!(
        advanced.cut().event_head(),
        None,
        "nothing has been observed, so a later instant resolves the same empty chain"
    );
    assert_eq!(
        advanced.selection().open().collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.funding, subject.equipment]),
        "advancing recognized knowledge and decided nothing"
    );

    let provisioning = arrangement.provisioning();

    assert_eq!(
        provisioning.parent(),
        &Some(advanced.id()),
        "and forks from what it advanced to, not from where it forked before"
    );
    assert_eq!(
        provisioning.cut(),
        advanced.cut(),
        "a fork moves no cut, so the knowledge it reasons over is the one it advanced to"
    );
    assert_eq!(
        provisioning.selection().open().collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.funding, subject.equipment, arrangement.contingency]),
        "the equipment line holds the funding, the equipment and the contingency"
    );

    // The asymmetry is the engine's and not the arrangement's politeness. A world at the
    // ancestor's cut cannot select the contingency at all, so the inventory line could not have
    // taken it without advancing either.
    let refused = arrangement.stocking().fork(
        arrangement.canon.history(),
        ForkInput {
            omitted: [].into(),
            introduced: [arrangement.contingency].into(),
        },
    );

    assert!(
        matches!(
            refused,
            Err(ThesisError::CommitmentNotKnownAtCut { commitment, .. })
                if commitment == arrangement.contingency
        ),
        "a fork cannot select what its cut cannot see, found {refused:?}"
    );

    // Both tips descend from the ancestor, which makes it an *admissible* Base and not the
    // right one — Synthesis accepts any common ancestor, and which one is asked about is part of
    // the question rather than a fact to be discovered.
    //
    // Here it happens to be the only one, so this arrangement can exercise a Base and cannot
    // exercise choosing between two.
    let archive = arrangement.lineage.archive();
    let (equipment_line, inventory_line) = (provisioning.id(), maintaining.id());

    for (label, tip) in [
        ("the equipment line", equipment_line),
        ("the inventory line", inventory_line),
    ] {
        assert!(
            descends_from(archive, tip, arrangement.ancestor().id()).expect("ancestry walks"),
            "{label} descends from the ancestor"
        );
    }

    for (label, tip, other) in [
        ("the equipment line", equipment_line, arrangement.stocking()),
        (
            "the inventory line",
            inventory_line,
            arrangement.equipping(),
        ),
    ] {
        assert!(
            !descends_from(archive, tip, other.id()).expect("ancestry walks"),
            "{label} passed through nothing the other decided"
        );
    }

    // Which is the phase's own requirement rather than a statement about the Base: two lines
    // that diverged, with the difference between them more than one step on either side.

    assert!(
        !descends_from(archive, equipment_line, inventory_line).expect("ancestry walks")
            && !descends_from(archive, inventory_line, equipment_line).expect("ancestry walks"),
        "neither line is a prefix of the other"
    );

    // How many Bases a transfer between the two tips could be measured against, counted rather
    // than argued. It is the boundary of what this subject can say: a Base can be exercised
    // here, and choosing between two cannot.
    let admissible: Vec<_> = arrangement
        .lineage
        .decided()
        .iter()
        .filter(|world| {
            [equipment_line, inventory_line]
                .iter()
                .all(|tip| descends_from(archive, *tip, world.id()).expect("ancestry walks"))
        })
        .map(|world| world.id())
        .collect();

    assert_eq!(
        admissible,
        [arrangement.ancestor().id()],
        "exactly one world is an ancestor of both tips"
    );

    // Each line is within the account's bounds on its own. Nothing here says what a world
    // holding both would be: that is what a transfer produces, and the next phase asks for it
    // rather than arranging it.
    let readings = reading::all(
        arrangement.canon.history(),
        arrangement.lineage.decided(),
        subject.instance,
        &effective(),
    )
    .expect("every world reads");

    for (position, label) in [(3, "the inventory line"), (5, "the equipment line")] {
        assert!(
            readings[position].conflicts.is_empty(),
            "{label} is feasible on its own, found {:?}",
            readings[position].conflicts
        );
        assert_eq!(
            readings[position].level, 0.0,
            "{label}: nothing has settled"
        );
    }

    assert_eq!(
        readings[5].known_at, "2026-01-16",
        "and the two lines no longer recognize the same knowledge"
    );
    assert_eq!(readings[3].known_at, "2026-01-10");
}

/// Two forks of one world, and two forks in a row, are different arrangements — and the
/// record now says which.
///
/// This is the measurement Phase 1 was arranged to make, kept as a guard because the whole
/// experiment turns on it. Before a decision could name what it extends, both arrangements
/// were written down identically, and reading them back produced only the second: a world
/// holding both lines of thinking, parented on one of them, that nobody decided.
#[test]
fn a_decision_extending_a_sibling_is_not_the_same_decision() {
    let arrangement = branched();
    let subject = &arrangement.subject;
    let canon = &arrangement.canon;

    let mut chained = Lineage::new();

    lineage::decide(
        canon.history(),
        &mut chained,
        &convergence::genesis(subject.funding),
    )
    .expect("the same ancestor");

    let ancestor = chained.decided()[0].id();

    lineage::decide(
        canon.history(),
        &mut chained,
        &convergence::equipping(ancestor, subject.equipment),
    )
    .expect("the same first fork");

    let sibling = chained.decided()[1].id();

    lineage::decide(
        canon.history(),
        &mut chained,
        &convergence::stocking(sibling, subject.inventory),
    )
    .expect("the second fork, extending the first");

    let merged = WorldRecord::of(&chained.decided()[2]);
    let intended = WorldRecord::of(arrangement.stocking());

    assert_eq!(
        merged.disagreement(&intended),
        Some("what it still proposes"),
        "the chained world holds what the sibling introduced"
    );
    assert_eq!(
        merged.open,
        BTreeSet::from([
            subject.funding.to_string(),
            subject.equipment.to_string(),
            subject.inventory.to_string(),
        ]),
        "both lines of thinking at once, which is neither of them"
    );
    assert_eq!(
        merged.thesis_parent,
        Some(chained.decided()[1].id().to_string()),
        "parented on the sibling rather than on the ancestor they share"
    );

    // And the ancestor is one world in both arrangements, which is what makes the two
    // comparable at all.
    assert_eq!(
        chained.decided()[0].id(),
        arrangement.ancestor().id(),
        "the same decision produces the same world"
    );
}
