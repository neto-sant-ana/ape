//! The house's delivery slot: the intention it forms, and the examination of it.
//!
//! The decision the house wants is the priority slot — it is the only one that spares the
//! penalty owed to a third party, and nothing else distinguishes the two arrangements in
//! the house's favour. So that is the intention this program makes known first: a
//! Commitment to spend 120 from the account, admitted through the Canon.
//!
//! Wanting it and being able to carry it out are separate questions, and the engine keeps
//! them separate. Whether an intention can be carried out is not a property of the
//! Commitment — it is a verdict over the whole graph a world selects, which is what a
//! `Thesis` denotes and what `Interpretation::feasibility_under` judges. The account holds
//! 100 against a `cash >= 0` bound, so the examination answers `OutOfBounds` at −20: the
//! priority slot cannot be carried out at any hypothesis, and no ordering or deadline
//! saves it.
//!
//! The second act is what the house does with that answer. History is never rewritten, so
//! the intention formed is not unformed: the priority Commitment is cancelled by an Event
//! — a fact that it will not happen — the Thesis advances to recognize that fact, and a
//! fork introduces the standard slot in its place. That world examines clean, and it is
//! the one the house undertakes.
//!
//! Three things are printed for each world: what it selects, what feasibility says under
//! each of the three hypotheses, and — for the world finally undertaken — the condition of
//! every commitment in it as of today.

mod world;

use ape::canon::EventSubmission;
use ape::engine::hermeneia::{Conflict, Hypothesis, Outcome, Timeliness};
use ape::engine::thesis::{ForkInput, GenesisInput, Interpretation, KnowledgeCut, Thesis};
use ape::kernel::entities::{CommitmentId, CommitmentInput};
use ape::kernel::value_objects::{ActionValue, Assignment, Date, Term};

use world::{World, cancelling, today};

/// What each arrangement costs the house, and when the market is due to have delivered.
const STANDARD_COST: f64 = 30.0;
const STANDARD_DUE: u8 = 12;
const PRIORITY_COST: f64 = 120.0;
const PRIORITY_DUE: u8 = 8;

fn main() {
    let mut world = world::build();

    println!("The house holds an account of cash bounded by `cash >= 0`, standing at 100");
    println!(
        "after the opening {} settled. Today is {}.\n",
        short(world.opening),
        today().to_iso()
    );

    let priority = undertake(&mut world, PRIORITY_COST, january(PRIORITY_DUE));
    println!(
        "Intended: the priority slot — spend {PRIORITY_COST} due {}, {}\n",
        january(PRIORITY_DUE).to_iso(),
        short(priority),
    );

    let intended = Thesis::genesis(
        world.canon.history(),
        GenesisInput {
            cut: KnowledgeCut::at(world.canon.history(), today()),
            selection: [priority].into(),
        },
    )
    .expect("the priority slot is selectable at today's cut");

    examine("the intended world", &intended, &world);

    let standard = undertake(&mut world, STANDARD_COST, january(STANDARD_DUE));
    println!(
        "Fallen back to: the standard slot — spend {STANDARD_COST} due {}, {}\n",
        january(STANDARD_DUE).to_iso(),
        short(standard),
    );

    world
        .canon
        .admit_event(
            EventSubmission {
                commitment_id: priority,
                observation: cancelling(),
                occurred_at: today(),
            },
            today(),
        )
        .expect("an unrealizable intention may be cancelled");

    let advanced = intended
        .advance(
            world.canon.history(),
            KnowledgeCut::at(world.canon.history(), today()),
        )
        .expect("the cancellation is later knowledge under the same instant");

    let undertaken = advanced
        .thesis()
        .fork(
            world.canon.history(),
            ForkInput {
                omitted: [].into(),
                introduced: [standard].into(),
            },
        )
        .expect("the standard slot is introducible at the same cut");

    examine("the undertaken world", &undertaken, &world);
    conditions(&undertaken, &world);
}

/// Admit the house's intention to spend `amount` from the account by `due`.
///
/// The house is accountable for it and executes it; the market benefits. Both sides are
/// eligible for the roles the outbound Statement names, which is what the Axiom checks
/// before this becomes knowledge.
fn undertake(world: &mut World, amount: f64, due: Date) -> CommitmentId {
    world
        .canon
        .admit_commitment(
            CommitmentInput {
                assignment: Assignment::new(world.house, [world.house], [world.market])
                    .expect("both sides are staffed"),
                statement: world.outbound,
                resource: world.account,
                term: Term::new(today(), due).expect("committed before due"),
                action_value: ActionValue::value(amount).expect("a positive, finite magnitude"),
                dependencies: [].into(),
            },
            today(),
        )
        .expect("the house may commit to spend")
}

/// Ask one world whether it still admits a completion, under every hypothesis there is.
///
/// One fold answers all three: the hypotheses differ in what they assume about *when*
/// unsettled commitments land, not in what is known.
fn examine(label: &str, thesis: &Thesis, world: &World) {
    let interpretation = Interpretation::of(thesis, world.canon.history())
        .expect("the thesis is projectable at the head its cut recognizes");

    println!(
        "{label} — {} selected ({} frozen, {} open):",
        thesis.selection().len(),
        thesis.selection().frozen().count(),
        thesis.selection().open().count(),
    );

    for hypothesis in [
        Hypothesis::FinalState,
        Hypothesis::OnDueDateNet,
        Hypothesis::OnDueDateInAnyOrder,
    ] {
        let report = interpretation
            .feasibility_under(hypothesis)
            .expect("the recognized chain is folded");

        let findings = match report.conflicts() {
            [] => "nothing found".to_string(),
            conflicts => conflicts
                .iter()
                .map(describe)
                .collect::<Vec<_>>()
                .join("; "),
        };

        println!("  {:<22} {findings}", format!("{hypothesis:?}"));
    }

    println!();
}

/// What each selected commitment's condition is, as of today.
fn conditions(thesis: &Thesis, world: &World) {
    let interpretation = Interpretation::of(thesis, world.canon.history())
        .expect("the thesis is projectable at the head its cut recognizes");

    let projected = interpretation
        .conditions_at(&today())
        .expect("the recognized chain is folded");

    println!("Conditions as of {}:", today().to_iso());

    for (id, condition) in projected.conditions() {
        let timeliness = match condition.timeliness() {
            Some(Timeliness::WithinDeadline) => ", within deadline",
            Some(Timeliness::Breached) => ", breached",
            None => "",
        };

        println!(
            "  {}  {}{timeliness}",
            short(*id),
            outcome_of(condition.outcome()),
        );
    }
}

fn outcome_of(outcome: &Outcome) -> &'static str {
    match outcome {
        Outcome::Unsettled => "Unsettled",
        Outcome::Fulfilled => "Fulfilled",
        Outcome::Cancelled => "Cancelled",
    }
}

/// A conflict in the terms of the world it was found in.
fn describe(conflict: &Conflict) -> String {
    match conflict {
        Conflict::Unrealizable(commitment) => {
            format!("{} can never be fulfilled", short(*commitment))
        }
        Conflict::PunctualDependencyViolation {
            dependency,
            dependent,
        } => format!(
            "{} is placed before {}, which it requires",
            short(*dependent),
            short(*dependency),
        ),
        Conflict::OutOfBounds { instance, level } => {
            format!("out of bounds: {} would reach {level}", short(*instance))
        }
    }
}

fn january(day: u8) -> Date {
    Date::from_ymd(2026, 1, day).expect("a real date in January 2026")
}

/// Enough of a content-addressed id to recognize it by.
fn short(id: impl std::fmt::Display) -> String {
    id.to_string().chars().take(8).collect()
}
