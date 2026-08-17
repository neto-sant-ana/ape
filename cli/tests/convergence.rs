//! The convergence experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, for the reason the reconstruction experiment
//! gives: a comparison has to fail loudly. What is new here is that the worlds under
//! comparison no longer form a line, so a phase says which world it is talking about rather
//! than counting from the end.

use std::collections::BTreeSet;

use ape::engine::synthesis::synthesize;
use ape::engine::thesis::{ForkInput, ThesisError, descends_from};
use ape::kernel::value_objects::Date;

use ape_cli::lineage::{self, Lineage};
use ape_cli::reading::{self, OutcomeRecord, TimelinessRecord, WorldRecord};
use ape_cli::repository::Repository;
use ape_cli::subject::convergence::{self, Branched, Diverged};
use ape_cli::transfer::{
    Applicability, CandidateRecord, ConflictRecord, StatusRecord, TransferRecord,
};

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

/// The reference Phase 3 records: one report in each direction, from a living process.
struct Reference {
    /// The equipment line's intention, asked about in the inventory line.
    into_inventory: Applicability,
    /// And the reverse, which is a different question rather than the same one backwards.
    into_equipment: Applicability,
}

/// Ask the engine both questions, against the archive the decisions filled.
///
/// The Base is the ancestor because it is the only world both lines descend from, which Phase 2
/// counted. Where a subject offered more than one, this choice would be part of the question
/// and would need saying — Synthesis verifies a Base and never searches for one.
fn synthesized(arrangement: &Diverged) -> Reference {
    let archive = arrangement.lineage.archive();
    let knowledge = arrangement.canon.history();
    let base = arrangement.ancestor().id();

    let (equipment_line, inventory_line) = (
        arrangement.provisioning().id(),
        arrangement.maintaining().id(),
    );

    let ask = |source, target| {
        Applicability::of(
            &synthesize(archive, knowledge, base, source, target).expect("the Base is coherent"),
        )
    };

    Reference {
        into_inventory: ask(equipment_line, inventory_line),
        into_equipment: ask(inventory_line, equipment_line),
    }
}

/// Phase 3 — Synthesize.
///
/// What one line of thinking's intention would be in the other, asked in both directions and
/// recorded whole. This is the reference; everything after it is a claim about reproducing it.
///
/// The two answers differ, and not by symmetry. Each line decided over the knowledge it had, and
/// only one of them had advanced — so an intention built on what arrived later has nowhere to go
/// in a world that has not recognized it, while the intention built on knowledge both lines
/// always had moves without objection.
#[test]
fn phase_3_synthesize() {
    let arrangement = diverged();
    let subject = &arrangement.subject;
    let reference = synthesized(&arrangement);

    let (funding, equipment, inventory, maintenance, contingency) = (
        subject.funding.to_string(),
        subject.equipment.to_string(),
        subject.inventory.to_string(),
        subject.maintenance.to_string(),
        arrangement.contingency.to_string(),
    );

    // Both reports are measured over the same three worlds, named the same way. A report that
    // agreed about everything else and disagreed here would be an answer to another question.
    for (label, report) in [
        ("into the inventory line", &reference.into_inventory),
        ("into the equipment line", &reference.into_equipment),
    ] {
        assert_eq!(
            report.base,
            arrangement.ancestor().id().to_string(),
            "{label} is measured against the world both lines came from"
        );
        assert!(
            report.omitted.is_empty(),
            "{label}: neither line withdrew anything the ancestor proposed"
        );
    }

    assert_eq!(
        reference.into_inventory.source,
        arrangement.provisioning().id().to_string()
    );
    assert_eq!(
        reference.into_inventory.target,
        arrangement.maintaining().id().to_string()
    );

    // How the three cuts stand, which is what decides which direction can refuse.
    //
    // A fork copies its parent's cut and an advance may not regress the instant, so a Base is
    // never later than something that descends from it. That is not checked anywhere and does
    // not need to be: it follows from the two derivations, and coherence of the Base already
    // implies it.
    //
    // What is unordered is Source against Target — two branches, and nothing relates them. Here
    // one is strictly later, and that ordering is the whole of why one direction conflicts.
    let (base_at, equipment_at, inventory_at) = (
        arrangement.ancestor().cut().known_at(),
        arrangement.provisioning().cut().known_at(),
        arrangement.maintaining().cut().known_at(),
    );

    assert!(
        base_at <= equipment_at && base_at <= inventory_at,
        "descending from the Base cannot move a cut backwards"
    );
    assert!(
        inventory_at < equipment_at,
        "and only here do the two lines stand in an order at all"
    );

    // Asking the equipment line's intention in the inventory line. What it decided is the
    // equipment and the contingency; the contingency was recorded on the twelfth, and the
    // inventory line recognizes the tenth.
    assert_eq!(
        reference.into_inventory.introduced,
        BTreeSet::from([equipment.clone(), contingency.clone()]),
        "the equipment line decided two things the ancestor had not"
    );
    assert_eq!(
        reference.into_inventory.status,
        StatusRecord::Conflicted {
            attempted: TransferRecord {
                remove: BTreeSet::new(),
                introduce: BTreeSet::from([equipment.clone(), contingency.clone()]),
            },
            conflicts: vec![ConflictRecord::HistoricalUnavailability {
                commitment: contingency.clone(),
                recorded_at: "2026-01-12".into(),
                known_at: "2026-01-10".into(),
            }],
        },
        "an intention built on knowledge the other line has not recognized cannot move to it"
    );

    // The reverse, which is a different question and not the same one backwards. A difference is
    // measured between the Base and the Source and never consults the Target at all, so swapping
    // the two roles changes what is being measured before anything judges whether it lands.
    assert_eq!(
        reference.into_equipment.introduced,
        BTreeSet::from([inventory.clone(), maintenance.clone()])
    );
    assert!(
        reference
            .into_inventory
            .introduced
            .is_disjoint(&reference.into_equipment.introduced),
        "the two directions have no commitment in common, so neither is the other's inverse"
    );
    assert_eq!(
        reference.into_equipment.status,
        StatusRecord::Applicable {
            transfer: TransferRecord {
                remove: BTreeSet::new(),
                introduce: BTreeSet::from([inventory.clone(), maintenance.clone()]),
            },
            candidate: CandidateRecord {
                frozen: BTreeSet::new(),
                open: BTreeSet::from([
                    funding.clone(),
                    equipment.clone(),
                    contingency.clone(),
                    inventory.clone(),
                    maintenance.clone(),
                ]),
            },
        },
        "and moves without objection"
    );

    // A transfer is applicable and the world it proposes is not feasible, which are different
    // questions with different owners. Synthesis judges what a Target may hold; the account's
    // own bounds judge what it can carry, and 60 − 30 − 25 − 40 − 15 is outside them.
    //
    // Nothing here refuses that world. It is not a world yet, and whether it becomes one is a
    // decision Phase 7 takes with this report in hand rather than instead of it.
    assert!(
        reference.into_equipment.status != StatusRecord::AlreadyApplied,
        "the Target does not already contain the difference"
    );

    // The cause of the refusal, isolated rather than inferred. Asked about the same line of
    // thinking, in the same target, one step earlier — before it advanced — the same transfer
    // is applicable. So what the inventory line cannot take is not the equipment line's
    // intention; it is the half of that intention built over knowledge it has not recognized.
    let earlier = Applicability::of(
        &synthesize(
            arrangement.lineage.archive(),
            arrangement.canon.history(),
            arrangement.ancestor().id(),
            arrangement.equipping().id(),
            arrangement.maintaining().id(),
        )
        .expect("the Base is coherent"),
    );

    assert_eq!(
        earlier.introduced,
        BTreeSet::from([equipment.clone()]),
        "one step earlier the equipment line had decided one thing"
    );
    assert_eq!(
        earlier.status,
        StatusRecord::Applicable {
            transfer: TransferRecord {
                remove: BTreeSet::new(),
                introduce: BTreeSet::from([equipment.clone()]),
            },
            candidate: CandidateRecord {
                frozen: BTreeSet::new(),
                open: BTreeSet::from([funding, equipment, inventory, maintenance]),
            },
        },
        "and that much of it moves, which leaves the contingency as the whole of the refusal"
    );
}

/// A repository path no other process shares.
///
/// The process id is part of it because two runs of this laboratory once wrote to the same path
/// and read each other's repositories back.
fn scratch(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir()
        .join(format!("ape-convergence-{}", std::process::id()))
        .join(name);
    let _ = std::fs::remove_dir_all(&path);
    path
}

/// Leave the arrangement on disk: the journal, the decisions, and the worlds they produced.
fn persist(repository: &Repository, arrangement: &Diverged) {
    repository
        .write_journal(&arrangement.journal)
        .expect("the repository is writable");
    repository
        .write_lineage(&arrangement.decisions)
        .expect("the repository is writable");
    repository
        .write_worlds(
            &arrangement
                .lineage
                .decided()
                .iter()
                .map(WorldRecord::of)
                .collect::<Vec<_>>(),
        )
        .expect("the repository is writable");
}

/// Phase 4 — Persist.
///
/// Both of the corroboration experiment's questions, asked of every datum: what becomes
/// impossible without it, and what compares it on every read.
///
/// The phase's own question is the report, and the answer is that it is not written. Not because
/// derived values are forbidden — that rule was retired — but because a report is a function of
/// three worlds and canonical knowledge, and every one of those is already witnessed. A copy of
/// it would re-detect what the world records detect and nothing else, which is exactly the
/// liability the replacement rule names.
///
/// What is *not* derivable is which three worlds were asked about. That is a choice, and this
/// repository holds no transfer to have chosen it for: Phase 3 asked a question, and asking is
/// not deciding. Whether a transfer that was **taken** must record its Base is Phase 7's, and
/// nothing here answers it early.
#[test]
fn phase_4_persist() {
    let arrangement = diverged();
    let repository = Repository::open(scratch("phase-4"));
    persist(&repository, &arrangement);

    assert_eq!(
        repository
            .read_journal()
            .expect("the journal reads back")
            .len(),
        arrangement.journal.len(),
        "every admission was kept, and none was invented"
    );
    assert_eq!(
        repository
            .read_lineage()
            .expect("the lineage reads back")
            .len(),
        6
    );

    // The worlds, and the shape they carry. A record naming its parent is what makes the
    // branching survive: two of them name the same one, which no sequence could express and no
    // reader could infer from order.
    let worlds = repository.read_worlds().expect("the worlds read back");

    assert_eq!(worlds.len(), 6);
    assert_eq!(
        worlds[1].thesis_parent, worlds[2].thesis_parent,
        "the two lines of thinking record the world they share"
    );
    assert_eq!(
        worlds[1].thesis_parent,
        Some(worlds[0].thesis.clone()),
        "and it is the one the first decision produced"
    );

    let written = std::fs::read_to_string(repository.journal_path()).expect("the file is there")
        + &std::fs::read_to_string(repository.lineage_path()).expect("the file is there");

    // What must not be there, inherited from the two experiments that asked it before. `frozen`
    // and `open` live in the worlds file now and are audited by being compared rather than by
    // being absent, which is what the corroboration experiment changed.
    for derived in [
        "level",
        "outcome",
        "fulfilled",
        "cancelled",
        "timeliness",
        "breached",
        "condition",
        "feasib",
        "previous_event",
        "head",
        "imposed",
    ] {
        assert!(
            !written.to_lowercase().contains(derived),
            "the repository holds {derived:?}, which is derived rather than supplied"
        );
    }

    // And what this experiment adds to that list: every coordinate of a report. A transfer was
    // asked about and none was taken, so nothing here should carry a trace of the question.
    for coordinate in [
        "status",
        "applicable",
        "conflicted",
        "already-applied",
        "candidate",
        "attempted",
        "unavailability",
        "freezing",
        "\"base\"",
        "\"source\"",
        "\"target\"",
        "remove",
    ] {
        assert!(
            !written.to_lowercase().contains(coordinate),
            "the repository holds {coordinate:?}, which belongs to a report nobody acted on"
        );
    }

    // The sharpest form of the same statement. A recorded transfer would have to name the two
    // worlds it moves between, and those are the only two in this lineage that nothing extends —
    // so their absence from the two files that hold intention is the evidence.
    //
    // Absent *there* and present in the worlds file, which is what makes the absence a fact
    // rather than a property of where the check happened to look. An audit that proves nothing
    // is missing by reading the wrong bytes is the failure this pairing exists to rule out.
    let witnessed = std::fs::read_to_string(repository.worlds_path()).expect("the file is there");

    for (label, tip) in [
        ("the equipment line", arrangement.provisioning()),
        ("the inventory line", arrangement.maintaining()),
    ] {
        assert!(
            !written.contains(&tip.id().to_string()),
            "{label} is named by no intention, so no transfer between the two was recorded"
        );
        assert!(
            witnessed.contains(&tip.id().to_string()),
            "{label} is recorded as a world, so the absence above is about where it is not"
        );
    }

    // The worlds each decision extends are named, which is Phase 1's repair seen from the file.
    for (position, label) in [
        (0, "the ancestor"),
        (1, "the equipment line's first decision"),
        (2, "the inventory line's first"),
        (4, "the world the equipment line advanced to"),
    ] {
        assert!(
            written.contains(&arrangement.lineage.decided()[position].id().to_string()),
            "{label} is extended by a later decision, so the file names it"
        );
    }

    // The whole of what a decision records, named field by field, for each of the six. The set
    // is closed rather than sampled because the phase's question is asked of every datum.
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

    let placed = ["after", "witness", "decides"].map(str::to_owned);
    let genesis = BTreeSet::from_iter(
        placed
            .iter()
            .cloned()
            .chain(["known_at".into(), "selection".into()]),
    );
    let fork = BTreeSet::from_iter(placed.iter().cloned().chain([
        "extends".into(),
        "omitted".into(),
        "introduced".into(),
    ]));
    let advance = BTreeSet::from_iter(
        placed
            .iter()
            .cloned()
            .chain(["extends".into(), "known_at".into()]),
    );

    assert_eq!(
        recorded,
        [
            genesis,
            fork.clone(),
            fork.clone(),
            fork.clone(),
            advance,
            fork
        ],
        "a decision records an intention, which world it extends, and where it was taken"
    );
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
