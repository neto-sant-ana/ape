//! Phase 5 — the same choices, in a world that has changed and in which they now break something.
//!
//! Two arrangements. In the first, the storage the house pays is **contingent** on the courier slot,
//! so standing the slot down strands it. In the second, the account must keep a reserve, so the two
//! parties' intentions together describe a world that cannot happen.
//!
//! Nothing about either party changes: same objective, same magnitude, same deadline, same omission,
//! same recording instant. What changes is the world.
//!
//! # Why these are re-enacted rather than replayed
//!
//! The recorded runs cannot be reused here, and the reason is mechanical rather than a choice. An
//! identity is its content: a resource's constraint is part of the resource, the instance names the
//! resource, and every commitment names the instance — so changing the reserve changes every identity
//! downstream of it. A commitment that carries a dependency is likewise not the commitment that does
//! not carry one.
//!
//! So the choices are re-enacted **by content** — the numbers, the dates, the omission — and the
//! suite asserts that the identities did move, so that "the same choice" is not read as "the same
//! objects". The first arrangement also makes one field a party never chose: in that world the fee
//! *is* contingent, which is what changing the world rather than the party means here.
//!
//! # None of the three verdicts is a finding
//!
//! Applicability is structural and never evaluates a level; a transfer may be applicable while
//! producing an infeasible candidate; and the two dependency conflicts are told apart by the origin
//! of the absence. All three are the engine's own documentation. What they are here for is to put
//! criterion 7 in an arrangement where the answer matters.

use std::collections::BTreeSet;

use ape::engine::hermeneia::{Conflict, Hypothesis};
use ape::engine::synthesis::{ApplicabilityStatus, synthesize};
use ape::engine::thesis::{Interpretation, ThesisId, ThesisLookup};
use ape::kernel::entities::CommitmentId;

use ape_cli::journal::ResourceKindRecord;

use ape_agents::coordination::{self, Line, STANDING_DUE, Shared};
use ape_agents::policy::{self, Verdict};
use ape_agents::world::{self, Intention};

/// What each party's objective asked for, unchanged between the arrangements.
const PURCHASE: f64 = 60.0;
const PURCHASE_DUE: u8 = 20;
const STORAGE: f64 = 30.0;
const STORAGE_DUE: u8 = 14;

/// The instants the parties chose, and each stated why.
const FINANCE_RECORDED: u8 = 6;
const OPERATIONS_RECORDED: u8 = 7;

/// Both lines re-enacted in one world.
struct Enacted {
    shared: Shared,
    operations: ThesisId,
    finance: ThesisId,
    storage: CommitmentId,
}

/// Re-enact the two parties' choices in a world the caller describes.
///
/// Finance's knowledge is admitted first because it is dated first, which is the order the merge of
/// the recorded runs also had to take.
fn enact(cash: ResourceKindRecord, contingent: bool) -> Enacted {
    let mut shared = coordination::under(cash);
    let base = shared.base;

    let dependencies: BTreeSet<CommitmentId> = if contingent {
        [shared.standing].into()
    } else {
        [].into()
    };

    let storage = coordination::intend(
        &mut shared,
        Intention {
            magnitude: STORAGE,
            incoming: false,
            due: STORAGE_DUE,
            recorded_at: FINANCE_RECORDED,
            dependencies,
        },
    );

    let purchase = coordination::intend(
        &mut shared,
        Intention {
            magnitude: PURCHASE,
            incoming: false,
            due: PURCHASE_DUE,
            recorded_at: OPERATIONS_RECORDED,
            dependencies: [].into(),
        },
    );

    let party = shared.finance;
    let finance = coordination::decide(
        &mut shared,
        &Line {
            omitted: [].into(),
            introduced: [storage].into(),
        },
        base,
        party,
    );

    let party = shared.operations;
    let standing = shared.standing;
    let carried = coordination::carry(&mut shared, base, OPERATIONS_RECORDED, party);
    let operations = coordination::decide(
        &mut shared,
        &Line {
            omitted: [standing].into(),
            introduced: [purchase].into(),
        },
        carried,
        party,
    );

    Enacted {
        shared,
        operations,
        finance,
        storage,
    }
}

fn report(
    enacted: &Enacted,
    source: ThesisId,
    target: ThesisId,
) -> ape::engine::synthesis::ApplicabilityReport {
    synthesize(
        enacted.shared.lineage.archive(),
        enacted.shared.canon.history(),
        enacted.shared.base,
        source,
        target,
    )
    .expect("a report is derivable")
}

/// Changing the world moves every identity under it, so the same choice is not the same object.
#[test]
fn the_same_choice_in_a_changed_world_is_a_different_commitment() {
    let plain = enact(world::cash(), false);
    let contingent = enact(world::cash(), true);
    let reserved = enact(reserve(30.0), false);

    assert_ne!(
        plain.storage, contingent.storage,
        "a commitment that carries a dependency is not the one that does not"
    );
    assert_ne!(
        plain.shared.world.account, reserved.shared.world.account,
        "the instance names the resource, and the resource carries the constraint"
    );
    assert_ne!(
        plain.storage, reserved.storage,
        "so the commitment that names the instance moves with it"
    );
    assert_ne!(
        plain.shared.base, reserved.shared.base,
        "and so does the world they were all decided against"
    );
}

/// Phase B1 — a removal strands what the changed world made contingent.
#[test]
fn standing_the_slot_down_strands_the_fee_that_hangs_off_it() {
    let enacted = enact(world::cash(), true);

    let into_finance = report(&enacted, enacted.operations, enacted.finance);

    let ApplicabilityStatus::Conflicted { conflicts, .. } = into_finance.status() else {
        panic!("a transfer that strands a dependent is refused: {into_finance:?}");
    };

    assert!(
        conflicts.iter().any(|conflict| matches!(
            conflict,
            ape::engine::synthesis::ApplicabilityConflict::DependencyBreakage {
                missing_dependency,
                ..
            } if *missing_dependency == enacted.shared.standing
        )),
        "and it names the slot the fee hangs off: {conflicts:?}"
    );
}

/// Phase B1 — the other direction is refused too, and the engine tells the two absences apart.
///
/// Documented, not discovered: a dependency absent from the candidate and present in the Target is a
/// breakage, and one that was never present is missing. Asserted because the arrangement produces
/// both, and a suite that showed only one would leave the classification looking incidental.
#[test]
fn carrying_the_fee_where_the_slot_was_dropped_is_missing_rather_than_broken() {
    let enacted = enact(world::cash(), true);

    let into_operations = report(&enacted, enacted.finance, enacted.operations);

    let ApplicabilityStatus::Conflicted { conflicts, .. } = into_operations.status() else {
        panic!("the fee cannot be carried where its dependency was dropped: {into_operations:?}");
    };

    assert!(
        conflicts.iter().any(|conflict| matches!(
            conflict,
            ape::engine::synthesis::ApplicabilityConflict::MissingDependency { dependency, .. }
                if *dependency == enacted.shared.standing
        )),
        "the slot was never in this Target, so the absence is missing rather than broken: {conflicts:?}"
    );
}

/// Phase B2 — applicable, and the world it produces cannot happen.
///
/// Each line on its own clears the reserve. Only the two of them together do not, and the report says
/// nothing about it — which is the division of labour the layer states and the reason criterion 7 is
/// worth asking.
#[test]
fn the_reserve_makes_the_merged_world_unrealizable_and_the_report_applicable() {
    let mut enacted = enact(reserve(30.0), false);

    for line in [enacted.operations, enacted.finance] {
        assert_eq!(
            ruling(&enacted, line, Hypothesis::FinalState),
            Verdict::MayProceed,
            "each party's own line clears the reserve"
        );
    }

    let into_operations = report(&enacted, enacted.finance, enacted.operations);

    let ApplicabilityStatus::Applicable { transfer, .. } = into_operations.status() else {
        panic!("nothing structural is broken here: {into_operations:?}");
    };

    let transfer = transfer.clone();
    let target = enacted.operations;
    let by = enacted.shared.operations;

    let adopted = adopt(&mut enacted, target, &transfer, by);

    assert_eq!(
        ruling(&enacted, adopted, Hypothesis::FinalState),
        Verdict::Refused(vec![Conflict::OutOfBounds {
            instance: enacted.shared.world.account,
            level: 10.0,
        }]),
        "100 received, 60 and 30 intended out, and a reserve of 30 the remainder does not reach"
    );
}

/// The reserve arrangement: the same cash, with a floor nothing else in the world moves.
fn reserve(floor: f64) -> ResourceKindRecord {
    ResourceKindRecord::Between {
        lower: floor,
        upper: 1000.0,
    }
}

fn ruling(enacted: &Enacted, world: ThesisId, hypothesis: Hypothesis) -> Verdict {
    let thesis = enacted
        .shared
        .lineage
        .archive()
        .thesis(world)
        .expect("the archive holds every world decided");

    let interpretation =
        Interpretation::of(&thesis, enacted.shared.canon.history()).expect("interpretable");

    policy::rule(&interpretation, hypothesis).expect("feasibility is derivable")
}

fn adopt(
    enacted: &mut Enacted,
    target: ThesisId,
    transfer: &ape::engine::synthesis::ResolvedTransfer,
    by: ape::kernel::entities::AgentId,
) -> ThesisId {
    let line = Line {
        omitted: transfer.remove().collect(),
        introduced: transfer.introduce().collect(),
    };

    coordination::decide(&mut enacted.shared, &line, target, by)
}

/// Kept so that a change to the world's standing arrangement is noticed here too.
#[test]
fn the_standing_arrangement_is_still_the_one_the_objectives_assume() {
    assert_eq!(STANDING_DUE, 10, "the slot the objectives name comes due");
}
