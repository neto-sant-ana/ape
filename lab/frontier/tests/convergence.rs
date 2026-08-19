//! The convergence experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, for the reason the reconstruction experiment
//! gives: a comparison has to fail loudly. What is new here is that the worlds under
//! comparison no longer form a line, so a phase says which world it is talking about rather
//! than counting from the end.

use std::collections::BTreeSet;

use ape::engine::synthesis::synthesize;
use ape::engine::thesis::{ForkInput, ThesisError, ThesisId, descends_from};
use ape::kernel::value_objects::Date;

use ape_cli::journal::EntryId;
use ape_cli::lineage::{self, Lineage, Taken};
use ape_cli::reading::{self, OutcomeRecord, Reading, TimelinessRecord, WorldRecord};
use ape_cli::repository::Repository;
use ape_cli::transfer::{
    Applicability, CandidateRecord, ConflictRecord, StatusRecord, TransferRecord,
};
use ape_frontier::subject::convergence::{self, Branched, Diverged};

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

/// Rebuild in an operating-system process of its own, given the repository and nothing else.
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

/// Phase 5 — Terminate and rebuild.
///
/// The process that decided the six worlds is gone, and here that is literal: the rebuild
/// happens in an operating-system process that shares no memory with it. Nothing it computed can
/// cross, because none of it can cross at all.
///
/// The archive is the point of the phase and is invisible in the result, which is the whole of
/// what makes it interesting. It is not read back — a `Thesis` does not deserialize — so it is
/// filled again by putting each world into it as the decisions produce it. Its absence would not
/// show up as a missing archive; it would show up as a decision naming a world nobody can find,
/// and the rebuild succeeding at all is what says it was there.
#[test]
fn phase_5_terminate() {
    let arrangement = diverged();
    let repository = Repository::open(scratch("phase-5"));
    persist(&repository, &arrangement);

    let dead = &ape_frontier::binary();

    let refused = rebuild_in_fresh_process(
        dead,
        &scratch("phase-5-absent"),
        arrangement.subject.instance,
    );

    assert!(
        !refused.status.success(),
        "a fresh process with no repository must not produce a world"
    );

    let survived = rebuild_in_fresh_process(dead, repository.root(), arrangement.subject.instance);

    assert!(
        survived.status.success(),
        "the fresh process failed: {}",
        String::from_utf8_lossy(&survived.stderr)
    );

    let rebuilt: Vec<Reading> =
        serde_json::from_slice(&survived.stdout).expect("a lineage came back");

    assert_eq!(
        rebuilt.len(),
        6,
        "the repository yields every world it holds decisions for"
    );

    // The shape came back, which is the first thing a sequence could have lost. Two worlds name
    // one parent, and the world they name is one world rather than two copies of one — the
    // archive holds it under a single identity because the identity is its content.
    assert_eq!(
        rebuilt[0].thesis_parent, None,
        "the lineage begins at a genesis"
    );
    assert_eq!(
        rebuilt[1].thesis_parent, rebuilt[2].thesis_parent,
        "the two lines of thinking came back sharing an ancestor"
    );
    assert_eq!(
        rebuilt[1].thesis_parent.as_deref(),
        Some(rebuilt[0].thesis.as_str()),
        "and it is the world the first decision produced"
    );

    // Each line's second decision found the world it extends, across a process boundary and
    // through an archive that was rebuilt rather than opened.
    assert_eq!(
        rebuilt[3].thesis_parent.as_deref(),
        Some(rebuilt[2].thesis.as_str()),
        "the inventory line's second decision extends its first"
    );
    assert_eq!(
        rebuilt[4].thesis_parent.as_deref(),
        Some(rebuilt[1].thesis.as_str()),
        "the equipment line advanced from where it forked"
    );
    assert_eq!(
        rebuilt[5].thesis_parent.as_deref(),
        Some(rebuilt[4].thesis.as_str()),
        "and forked from what it advanced to"
    );

    // And the two lines came back recognizing different knowledge, which is what made the two
    // directions of Phase 3 different questions.
    assert_eq!(rebuilt[3].known_at, "2026-01-10");
    assert_eq!(rebuilt[5].known_at, "2026-01-16");
    assert_eq!(rebuilt[5].effective_at, EFFECTIVE);

    // That the fresh process resolves `extends` through an archive it built, rather than reading
    // past it, is not visible in a result that succeeded. So one decision is repointed at a
    // world that **exists** — the other line's first — and the rebuild is asked again.
    //
    // Breaking the archive in code proves the same thing and proves it in the wrong place: the
    // living process and the fresh one run one code path, so a mutation there never reaches a
    // process boundary. Moving the coordinate in the repository is what isolates this side of it.
    let repointed = Repository::open(scratch("phase-5-repointed"));
    persist(&repointed, &arrangement);

    let mut decisions = arrangement.decisions.clone();
    match &mut decisions[3].decision {
        lineage::Decision::Fork { extends, .. } => *extends = arrangement.equipping().id(),
        other => panic!("the fourth decision is a fork, found {other:?}"),
    }
    repointed.write_lineage(&decisions).expect("writable");

    let refused_repointed =
        rebuild_in_fresh_process(dead, repointed.root(), arrangement.subject.instance);
    let complaint = String::from_utf8_lossy(&refused_repointed.stderr);

    assert!(
        !refused_repointed.status.success(),
        "a decision extending the other line is a different lineage: {}",
        String::from_utf8_lossy(&refused_repointed.stdout)
    );
    assert!(
        complaint.contains("world 3 disagrees"),
        "and the world it produced is what says so, naming which: {complaint}"
    );
}

/// Ask a fresh process what one world's intention would be in another.
fn transfer_in_fresh_process(
    binary: &std::path::Path,
    repository: &std::path::Path,
    base: &str,
    source: &str,
    target: &str,
) -> std::process::Output {
    std::process::Command::new(binary)
        .arg(repository)
        .arg("transfer")
        .arg(base)
        .arg(source)
        .arg(target)
        .output()
        .expect("the binary runs")
}

/// Phase 6 — Compare.
///
/// The report a fresh process produces against the one Phase 3 recorded, whole, in both
/// directions. And against literals written down before the run, because a comparison of two
/// derivations survives a defect the two share.
///
/// The three identities are handed to the fresh process rather than found by it. That is Phase
/// 4's decision arriving as an interface: reconstructing a world needs the repository alone, and
/// reconstructing a report needs the repository and a question.
#[test]
fn phase_6_compare() {
    let arrangement = diverged();
    let repository = Repository::open(scratch("phase-6"));
    persist(&repository, &arrangement);

    let living = synthesized(&arrangement);
    let dead = &ape_frontier::binary();

    let ask = |source: &str, target: &str| {
        let produced = transfer_in_fresh_process(
            dead,
            repository.root(),
            &arrangement.ancestor().id().to_string(),
            source,
            target,
        );

        assert!(
            produced.status.success(),
            "the fresh process failed: {}",
            String::from_utf8_lossy(&produced.stderr)
        );

        serde_json::from_slice::<Applicability>(&produced.stdout).expect("a report came back")
    };

    let (equipment_line, inventory_line) = (
        arrangement.provisioning().id().to_string(),
        arrangement.maintaining().id().to_string(),
    );

    let into_inventory = ask(&equipment_line, &inventory_line);
    let into_equipment = ask(&inventory_line, &equipment_line);

    // Whole, in both directions. Every coordinate the protocol names is inside this comparison —
    // the three identities, the difference, the status and the conflicts — and so is any
    // coordinate nobody thought to list, which is the kind a list misses.
    assert_eq!(
        into_inventory, living.into_inventory,
        "the refused direction came back the same report"
    );
    assert_eq!(
        into_equipment, living.into_equipment,
        "and so did the one that carries"
    );

    // Against literals instead, which answers a different question. Equality above compares two
    // derivations produced by one implementation and survives that implementation drifting;
    // these do not.
    assert_eq!(
        into_inventory.status,
        StatusRecord::Conflicted {
            attempted: TransferRecord {
                remove: BTreeSet::new(),
                introduce: BTreeSet::from([
                    arrangement.subject.equipment.to_string(),
                    arrangement.contingency.to_string(),
                ]),
            },
            conflicts: vec![ConflictRecord::HistoricalUnavailability {
                commitment: arrangement.contingency.to_string(),
                recorded_at: "2026-01-12".into(),
                known_at: "2026-01-10".into(),
            }],
        }
    );

    match &into_equipment.status {
        StatusRecord::Applicable {
            transfer,
            candidate,
        } => {
            assert_eq!(transfer.remove.len(), 0);
            assert_eq!(transfer.introduce.len(), 2);
            assert_eq!(candidate.frozen.len(), 0);
            assert_eq!(
                candidate.open.len(),
                5,
                "the funding, both lines' decisions, and the contingency"
            );
        }
        other => panic!("the carrying direction is applicable, found {other:?}"),
    }

    // And the ancestry walk, which nothing else in this phase would show. A report at all means
    // the rebuilt archive reached the Base from both tips; a Base that only one of them descends
    // from has to be refused, or the walk was never performed.
    let incoherent = transfer_in_fresh_process(
        dead,
        repository.root(),
        &arrangement.equipping().id().to_string(),
        &equipment_line,
        &inventory_line,
    );

    assert!(
        !incoherent.status.success(),
        "a Base only the Source descends from is not a Base: {}",
        String::from_utf8_lossy(&incoherent.stdout)
    );
    assert!(
        String::from_utf8_lossy(&incoherent.stderr).contains("is not a common ancestor of"),
        "and the refusal says so: {}",
        String::from_utf8_lossy(&incoherent.stderr)
    );
}

/// Phase 7 — Apply.
///
/// The transfer Synthesis found applicable is carried into its Target as a decision, and the
/// repository is read once more. This is where the hypothesis's least certain claim is settled:
/// if a transfer applied reproduces through the ordinary path, a repository records a **choice**;
/// if it does not, it has to record a computation.
#[test]
fn phase_7_apply() {
    let arrangement = convergence::reconciled().expect("the arrangement holds");
    let subject = &arrangement.subject;

    assert_eq!(
        arrangement.lineage.decided().len(),
        7,
        "one more world, and no new kind of thing"
    );

    // The world the transfer produced, against the candidate the report predicted. If those
    // disagreed the report would be evidence about a world nobody can build.
    let reconciling = arrangement.reconciling();
    let carried = Applicability::of(&arrangement.carried);

    let StatusRecord::Applicable { candidate, .. } = &carried.status else {
        panic!(
            "the carried transfer is applicable, found {:?}",
            carried.status
        );
    };

    assert_eq!(
        reconciling
            .selection()
            .open()
            .map(|id| id.to_string())
            .collect::<BTreeSet<_>>(),
        candidate.open,
        "the world a fork produces is the candidate the report described"
    );
    assert_eq!(
        reconciling
            .selection()
            .frozen()
            .map(|id| id.to_string())
            .collect::<BTreeSet<_>>(),
        candidate.frozen
    );

    // And it is an ordinary child of its Target, at its Target's cut. A transfer moves intention
    // and no knowledge, so nothing about the world says it came from anywhere but a fork.
    assert_eq!(
        reconciling.parent(),
        &Some(arrangement.lineage.decided()[5].id()),
        "the applied world extends the Target"
    );
    assert_eq!(
        reconciling.cut().known_at().to_iso(),
        "2026-01-16",
        "and recognizes exactly what the Target recognized"
    );

    // The record it left is a fork like the four before it. No field was added, and the closed
    // set Phase 4 pinned is unchanged.
    let repository = Repository::open(scratch("phase-7"));
    repository
        .write_journal(&arrangement.journal)
        .expect("writable");
    repository
        .write_lineage(&arrangement.decisions)
        .expect("writable");
    repository
        .write_worlds(
            &arrangement
                .lineage
                .decided()
                .iter()
                .map(WorldRecord::of)
                .collect::<Vec<_>>(),
        )
        .expect("writable");

    let written = std::fs::read_to_string(repository.lineage_path()).expect("the file is there");
    let recorded: Vec<serde_json::Value> =
        serde_json::from_str(&written).expect("the lineage is a list");

    assert_eq!(recorded.len(), 7);
    assert_eq!(
        recorded[6]["decides"], "fork",
        "a transfer applied is recorded as a fork"
    );
    assert_eq!(
        recorded[6]
            .as_object()
            .expect("an object")
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "decides".to_owned(),
            "extends".into(),
            "omitted".into(),
            "introduced".into(),
            "after".into(),
            "witness".into(),
        ]),
        "and records nothing a fork does not"
    );

    // Reproduced through the same path as every other decision, in a process of its own.
    let dead = &ape_frontier::binary();
    let survived = rebuild_in_fresh_process(dead, repository.root(), subject.instance);

    assert!(
        survived.status.success(),
        "the fresh process failed: {}",
        String::from_utf8_lossy(&survived.stderr)
    );

    let rebuilt: Vec<Reading> =
        serde_json::from_slice(&survived.stdout).expect("a lineage came back");

    assert_eq!(rebuilt.len(), 7);
    assert_eq!(
        rebuilt[6].thesis,
        reconciling.id().to_string(),
        "the applied world came back as the world it was"
    );

    // The verdict it carries, which is the reason to have applied anything at all. Each line was
    // within the account's bounds alone; holding both is not. Synthesis said the transfer was
    // applicable and it was right — applicability is about what a Target may hold, and the
    // account's own bounds are a different judgement with a different owner.
    assert_eq!(
        rebuilt[6].conflicts,
        vec![ape_cli::reading::ConflictRecord::OutOfBounds {
            instance: subject.instance.to_string(),
            level: -50.0,
        }],
        "60 − 30 − 25 − 40 − 15 is outside 0..100"
    );
    for position in [3, 5] {
        assert!(
            rebuilt[position].conflicts.is_empty(),
            "and neither line was refused on its own"
        );
    }

    // What the record no longer says, counted rather than argued.
    //
    // The decision names what it introduced and not that another line of thinking is why. So:
    // given only the repository, how many transfers would produce exactly this world? If one,
    // the provenance is recoverable by search and recording the Base buys nothing. If more, the
    // applied world does not identify the transfer that produced it, and no amount of looking
    // will make it.
    let reading::Corroborated { canon, lineage, .. } =
        reading::corroborated(&repository).expect("the repository rebuilds");
    let worlds: Vec<_> = lineage.decided().iter().map(|world| world.id()).collect();
    let base = worlds[0];

    let produced: BTreeSet<String> = reconciling
        .selection()
        .open()
        .map(|id| id.to_string())
        .collect();

    let explanations: Vec<(usize, usize)> = worlds
        .iter()
        .enumerate()
        .flat_map(|(from, source)| {
            worlds
                .iter()
                .enumerate()
                .map(move |(into, target)| (from, into, *source, *target))
        })
        .filter_map(|(from, into, source, target)| {
            let report = synthesize(lineage.archive(), canon.history(), base, source, target)
                .expect("the ancestor is a coherent Base for every pair");

            match Applicability::of(&report).status {
                StatusRecord::Applicable { candidate, .. } if candidate.open == produced => {
                    Some((from, into))
                }
                _ => None,
            }
        })
        .collect();

    // Three, and the count was written down as two before the run. The one missed is the applied
    // world transferred into what the equipment line had advanced to — which reaches the same
    // selection by a different Target, and is the reason a guess is not a measurement here.
    assert_eq!(
        explanations,
        [(3, 5), (6, 4), (6, 5)],
        "three transfers produce this world, and the record says which was taken by not saying"
    );
}

/// One case of the audit below: what it is called, what it moves, and what must be said about it.
type Tamper<T> = (&'static str, Box<dyn Fn(&mut T)>, &'static str);

/// Every derived value this repository holds, tampered one at a time.
///
/// The sixth success criterion says each of them is compared on every read, and every one that is
/// not is removed. Half of that was verified by reasoning in Phases 4 and 5 — the report is not
/// written, the world records and the witnesses are — and reasoning is what this laboratory
/// spends its time refusing to accept from itself.
///
/// So the list is closed and walked. Each case moves one value in a repository that is otherwise
/// intact, and the read must refuse **and name what disagreed**: a refusal that says the
/// repository is invalid sends a reader back to the bytes.
///
/// The check happens inside `reading::corroborated`, which is reconstruction itself and not a
/// harness. A comparison a test performs is a comparison production does not.
#[test]
fn every_persisted_derivation_is_weighed() {
    let arrangement = convergence::reconciled().expect("the arrangement holds");

    let intact = Repository::open(scratch("weighed-intact"));
    persist_reconciled(&intact, &arrangement);

    assert!(
        reading::corroborated(&intact).is_ok(),
        "the baseline reads, or every refusal below is about something else"
    );

    let worlds = intact.read_worlds().expect("the worlds read back");
    let decisions = intact.read_lineage().expect("the lineage reads back");

    // The world record, coordinate by coordinate. `disagreement` weighs them in a fixed order and
    // an earlier match hides a later one, so every coordinate is moved on its own to prove none
    // of them is unreachable — identity in particular, which is weighed last.
    let world_cases: Vec<Tamper<Vec<WorldRecord>>> = vec![
        (
            "the instant a world recognizes",
            Box::new(|worlds: &mut Vec<WorldRecord>| worlds[6].known_at = "2026-01-17".into()),
            "world 6 disagrees with what was recorded, in the instant it recognizes",
        ),
        (
            "the chain a world recognizes",
            Box::new(|worlds: &mut Vec<WorldRecord>| worlds[6].event_head = Some("00".repeat(32))),
            "world 6 disagrees with what was recorded, in the chain it recognizes",
        ),
        (
            "what history made unavoidable",
            Box::new(|worlds: &mut Vec<WorldRecord>| {
                let moved = worlds[6].open.iter().next().cloned().expect("an open one");
                worlds[6].frozen.insert(moved);
            }),
            "world 6 disagrees with what was recorded, in what history made unavoidable",
        ),
        (
            "what a world still proposes",
            Box::new(|worlds: &mut Vec<WorldRecord>| {
                let dropped = worlds[6].open.iter().next().cloned().expect("an open one");
                worlds[6].open.remove(&dropped);
            }),
            "world 6 disagrees with what was recorded, in what it still proposes",
        ),
        (
            "ancestry",
            Box::new(|worlds: &mut Vec<WorldRecord>| {
                worlds[6].thesis_parent = Some(worlds[3].thesis.clone())
            }),
            "world 6 disagrees with what was recorded, in ancestry",
        ),
        (
            "identity alone",
            Box::new(|worlds: &mut Vec<WorldRecord>| worlds[6].thesis = "00".repeat(32)),
            "world 6 disagrees with what was recorded, in identity alone",
        ),
        (
            "how many worlds there are",
            Box::new(|worlds: &mut Vec<WorldRecord>| {
                worlds.pop();
            }),
            "the decisions produce 7 worlds, and 6 were recorded",
        ),
    ];

    for (label, tamper, complaint) in world_cases {
        let moved = Repository::open(scratch(&format!("weighed-{}", label.replace(' ', "-"))));
        persist_reconciled(&moved, &arrangement);

        let mut tampered = worlds.clone();
        tamper(&mut tampered);
        moved.write_worlds(&tampered).expect("writable");

        let refusal = reading::corroborated(&moved)
            .err()
            .unwrap_or_else(|| panic!("{label} moved and nothing noticed"));

        assert_eq!(refusal.to_string(), complaint, "{label}");
    }

    // The decision record's two derived halves. Neither is a world, and neither is caught by the
    // world records: a witness disagrees before any world is built, and a reference that resolves
    // to nothing never builds one.
    let decision_cases: Vec<Tamper<Vec<Taken>>> = vec![
        (
            "an entry dropped from a witness",
            Box::new(|decisions: &mut Vec<Taken>| {
                let dropped = decisions[3]
                    .witness
                    .iter()
                    .next()
                    .cloned()
                    .expect("a witnessed entry");
                decisions[3].witness.remove(&dropped);
            }),
            "was admitted, and the decision was not taken against it",
        ),
        (
            "an entry invented in a witness",
            Box::new(|decisions: &mut Vec<Taken>| {
                decisions[3]
                    .witness
                    .insert(EntryId::of(ThesisId::from([0; 32])));
            }),
            "which the journal does not offer",
        ),
        (
            "a reference to a world nothing produces",
            Box::new(
                |decisions: &mut Vec<Taken>| match &mut decisions[6].decision {
                    lineage::Decision::Fork { extends, .. } => *extends = ThesisId::from([0; 32]),
                    other => panic!("the last decision is a fork, found {other:?}"),
                },
            ),
            "which the lineage does not hold",
        ),
        (
            "a coordinate moved along the journal",
            Box::new(|decisions: &mut Vec<Taken>| decisions[0].after = decisions[6].after.clone()),
            "was admitted, and the decision was not taken against it",
        ),
        // The case the corroboration experiment declared its subject could not build.
        //
        // The inventory line's second decision is moved forward past the contingency's admission.
        // It selects what it names and inherits a cut at the tenth, and the contingency is
        // neither selected by it nor an Event — so the world it produces is *identical*, and
        // every world record still agrees. Only the witness sees the decision has been relocated.
        (
            "a decision relocated where no world changes",
            Box::new(|decisions: &mut Vec<Taken>| decisions[3].after = decisions[4].after.clone()),
            "was admitted, and the decision was not taken against it",
        ),
    ];

    for (label, tamper, fragment) in decision_cases {
        let moved = Repository::open(scratch(&format!("weighed-{}", label.replace(' ', "-"))));
        persist_reconciled(&moved, &arrangement);

        let mut tampered = decisions.clone();
        tamper(&mut tampered);
        moved.write_lineage(&tampered).expect("writable");

        let refusal = reading::corroborated(&moved)
            .err()
            .unwrap_or_else(|| panic!("{label} and nothing noticed"));

        assert!(
            refusal.to_string().contains(fragment),
            "{label}: the refusal does not name what disagreed: {refusal}"
        );
    }
}

/// Leave the whole arrangement on disk, transfer included.
fn persist_reconciled(repository: &Repository, arrangement: &convergence::Reconciled) {
    repository
        .write_journal(&arrangement.journal)
        .expect("writable");
    repository
        .write_lineage(&arrangement.decisions)
        .expect("writable");
    repository
        .write_worlds(
            &arrangement
                .lineage
                .decided()
                .iter()
                .map(WorldRecord::of)
                .collect::<Vec<_>>(),
        )
        .expect("writable");
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
