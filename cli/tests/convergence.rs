//! The convergence experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, for the reason the reconstruction experiment
//! gives: a comparison has to fail loudly. What is new here is that the worlds under
//! comparison no longer form a line, so a phase says which world it is talking about rather
//! than counting from the end.

use std::collections::BTreeSet;

use ape::canon::Canon;
use ape::engine::thesis::Thesis;
use ape::kernel::value_objects::Date;

use ape_cli::history::ResidentHistory;
use ape_cli::lineage::{self, Lineage};
use ape_cli::reading::{self, OutcomeRecord, TimelinessRecord, WorldRecord};
use ape_cli::subject::convergence;

/// The instant every world is interpreted at.
///
/// It sits past the deadline the funding and the equipment carry and inside the one the
/// inventory carries, so the two lines of thinking differ in their conditions as well as in
/// what they select.
const EFFECTIVE: &str = "2026-01-22";

fn effective() -> Date {
    Date::parse(EFFECTIVE).expect("a real date")
}

/// What Phase 1 arranges: one ancestor, and two worlds that extend it.
struct Branched {
    canon: Canon<ResidentHistory>,
    subject: convergence::Constructed,
    lineage: Lineage,
}

impl Branched {
    fn ancestor(&self) -> &Thesis {
        &self.lineage.decided()[0]
    }

    fn equipping(&self) -> &Thesis {
        &self.lineage.decided()[1]
    }

    fn stocking(&self) -> &Thesis {
        &self.lineage.decided()[2]
    }
}

/// Admit the subject and take the three decisions, each through the decision record.
///
/// Nothing here calls `Thesis::fork` directly. That is the discipline the divergence
/// experiment established — what a later process gets is the decision, so the decision is what
/// every phase must go through — and it is also what made this arrangement impossible to state
/// until a decision could name the world it extends.
fn branched() -> Branched {
    let mut canon = Canon::new(ResidentHistory::new());
    let subject = convergence::construct(&mut canon).expect("the subject is admissible");

    let mut lineage = Lineage::new();

    lineage::decide(
        canon.history(),
        &mut lineage,
        &convergence::genesis(subject.funding),
    )
    .expect("the common ancestor is decidable");

    let ancestor = lineage.decided()[0].id();

    lineage::decide(
        canon.history(),
        &mut lineage,
        &convergence::equipping(ancestor, subject.equipment),
    )
    .expect("one line of thinking");

    lineage::decide(
        canon.history(),
        &mut lineage,
        &convergence::stocking(ancestor, subject.inventory),
    )
    .expect("the other");

    Branched {
        canon,
        subject,
        lineage,
    }
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
