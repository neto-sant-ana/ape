//! The experimental subject: the smallest operational graph that exercises both intended
//! and observed reality.
//!
//! ```text
//! Agent ── Role ── Statement ──▶ Action ──▶ Resource
//!                      │
//!                  Commitment
//! ```
//!
//! The domain is irrelevant and deliberately thin. What the subject must supply is a
//! complete semantic path: a quantifiable resource, so that interpreting it exposes both a
//! commitment's condition and a factual consequence on a level; one commitment, so there
//! is an intention to settle; and no dependencies, so nothing here is waiting on anything
//! but the event that has not happened yet.
//!
//! Every quantity is an integer. Feasibility accumulates levels in `f64`, where addition
//! is not associative, and a reconstruction that read records back in a different order
//! could differ in the last bit and flip a comparison against a constraint. Integers keep
//! the experiment measuring reconstruction rather than float determinism.

use ape::canon::{Canon, CanonError};
use ape::kernel::entities::{
    ActionInput, AgentInput, CommitmentId, CommitmentInput, EligibilityAssignmentInput,
    ResourceInput, ResourceInstanceId, ResourceInstanceInput, RoleInput, StatementInput,
};
use ape::kernel::value_objects::{
    ActionKind, ActionValue, AgentKind, Assignment, Constraint, Date, Effect, Identifier,
    Observation, Participants, ResourceKind, Settlement, Term,
};

use crate::history::ResidentHistory;

/// What the procedure needs to carry from one phase to the next.
pub struct Subject {
    pub commitment: CommitmentId,
    pub instance: ResourceInstanceId,
    pub fulfilling: Observation,
    pub term: Term,
}

pub fn vocabulary_recorded_at() -> Date {
    day(1)
}

pub fn commitment_recorded_at() -> Date {
    day(5)
}

/// Admit the subject through the ordinary APE boundaries.
///
/// Nothing here takes a persistence-specific construction path: this is the same sequence
/// a reconstruction has to reproduce, which is what makes the two comparable at all.
pub fn construct(canon: &mut Canon<ResidentHistory>) -> Result<Subject, CanonError> {
    let vocabulary_at = vocabulary_recorded_at();

    let supplier = canon.admit_role(
        RoleInput {
            label: name("supplier"),
        },
        vocabulary_at,
    )?;

    let buyer = canon.admit_role(
        RoleInput {
            label: name("buyer"),
        },
        vocabulary_at,
    )?;

    let shipper = canon.admit_agent(
        AgentInput {
            label: name("shipper"),
            kind: AgentKind::Company,
        },
        vocabulary_at,
    )?;

    let receiver = canon.admit_agent(
        AgentInput {
            label: name("receiver"),
            kind: AgentKind::Company,
        },
        vocabulary_at,
    )?;

    canon.admit_eligibility(
        EligibilityAssignmentInput {
            agent: shipper,
            roles: [supplier].into(),
            effective_from: vocabulary_at,
        },
        vocabulary_at,
    )?;

    canon.admit_eligibility(
        EligibilityAssignmentInput {
            agent: receiver,
            roles: [buyer].into(),
            effective_from: vocabulary_at,
        },
        vocabulary_at,
    )?;

    let inventory = canon.admit_resource(
        ResourceInput {
            label: name("inventory"),
            kind: ResourceKind::Quantifiable(
                Constraint::between(0.0, 100.0).expect("an ordered, finite range"),
            ),
        },
        vocabulary_at,
    )?;

    let instance = canon.admit_resource_instance(
        ResourceInstanceInput {
            label: name("warehouse"),
            resource: inventory,
        },
        vocabulary_at,
    )?;

    let deliver = canon.admit_action(
        ActionInput {
            verb: name("deliver"),
            kind: ActionKind::Quantifiable(Effect::Increase),
            resource: inventory,
        },
        vocabulary_at,
    )?;

    let fulfilling = observation("Delivered");

    let statement = canon.admit_statement(
        StatementInput {
            participants: Participants::new([supplier], [buyer]).expect("both sides named"),
            action: deliver,
            settlement: Settlement::new([fulfilling.clone()], [observation("Cancelled")])
                .expect("both outcomes named"),
        },
        vocabulary_at,
    )?;

    let term = Term::new(day(5), day(20)).expect("committed before due");

    let commitment = canon.admit_commitment(
        CommitmentInput {
            assignment: Assignment::new(shipper, [shipper], [receiver]).expect("both sides staffed"),
            statement,
            resource: instance,
            term: term.clone(),
            action_value: ActionValue::value(10.0).expect("a positive, finite magnitude"),
            dependencies: [].into(),
        },
        commitment_recorded_at(),
    )?;

    Ok(Subject {
        commitment,
        instance,
        fulfilling,
        term,
    })
}

fn day(day: u8) -> Date {
    Date::from_ymd(2026, 1, day).expect("a real date in January 2026")
}

fn name(value: &str) -> Identifier {
    Identifier::new(value).expect("a non-blank identifier")
}

fn observation(value: &str) -> Observation {
    Observation::new(value).expect("a non-blank observation")
}
