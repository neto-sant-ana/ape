//! The world an agent is given, and the only file it receives beyond the engine's own
//! documentation.
//!
//! ```text
//! Resource   cash, constrained to  cash >= 0
//! Instance   account
//! Known      a fulfilled Commitment that moved +100
//! ```
//!
//! What this module does is admit a vocabulary and one settled fact. What it deliberately
//! does not do is construct the options: the world offers a Statement under which cash can
//! be spent, and turning an option into an intention is the agent's job. Pre-constructing
//! either would answer the first question the experiment is asking.
//!
//! The knowledge is held in the engine's reference adapter. That adapter exists to
//! demonstrate the history contract rather than to back an application, which is exactly
//! right here: this experiment ends when the process does, and durability across process
//! death is a different experiment with a different boundary.
//!
//! Every quantity is an integer. Levels accumulate in `f64`, where addition is not
//! associative, and an experiment about what an agent can express should not be measuring
//! the last bit of a float.

use ape::canon::{Canon, EventSubmission, InMemoryHistory};
use ape::kernel::entities::{
    ActionInput, AgentId, AgentInput, CommitmentId, CommitmentInput, EligibilityAssignmentInput,
    ResourceInput, ResourceInstanceId, ResourceInstanceInput, RoleId, RoleInput, StatementId,
    StatementInput,
};
use ape::kernel::value_objects::{
    ActionKind, ActionValue, AgentKind, Assignment, Constraint, Date, Effect, Identifier,
    Observation, Participants, ResourceKind, Settlement, Term,
};

/// The world, and every handle needed to act within it.
pub struct World {
    pub canon: Canon<InMemoryHistory>,
    pub house: AgentId,
    pub market: AgentId,
    pub spender: RoleId,
    pub counterparty: RoleId,
    pub inbound: StatementId,
    pub outbound: StatementId,
    pub account: ResourceInstanceId,
    pub opening: CommitmentId,
}

/// The instant the world is current as of.
pub fn today() -> Date {
    day(6)
}

/// The observation that settles a Commitment made under either Statement.
pub fn settling() -> Observation {
    observation("Settled")
}

/// The observation that cancels one.
pub fn cancelling() -> Observation {
    observation("Cancelled")
}

/// Admit the vocabulary, then the one fact that gives the account a level.
pub fn build() -> World {
    let mut canon = Canon::new(InMemoryHistory::default());
    let at = day(1);

    let spender = canon
        .admit_role(
            RoleInput {
                label: name("spender"),
            },
            at,
        )
        .expect("a role is admissible");

    let counterparty = canon
        .admit_role(
            RoleInput {
                label: name("counterparty"),
            },
            at,
        )
        .expect("a role is admissible");

    let house = canon
        .admit_agent(
            AgentInput {
                label: name("house"),
                kind: AgentKind::Company,
            },
            at,
        )
        .expect("an agent is admissible");

    let market = canon
        .admit_agent(
            AgentInput {
                label: name("market"),
                kind: AgentKind::Company,
            },
            at,
        )
        .expect("an agent is admissible");

    canon
        .admit_eligibility(
            EligibilityAssignmentInput {
                agent: house,
                roles: [spender].into(),
                effective_from: at,
            },
            at,
        )
        .expect("the house may spend");

    canon
        .admit_eligibility(
            EligibilityAssignmentInput {
                agent: market,
                roles: [counterparty].into(),
                effective_from: at,
            },
            at,
        )
        .expect("the market may be paid");

    let cash = canon
        .admit_resource(
            ResourceInput {
                label: name("cash"),
                kind: ResourceKind::Quantifiable(
                    Constraint::greater_than_or_equal(0.0).expect("a finite bound"),
                ),
            },
            at,
        )
        .expect("a resource is admissible");

    let account = canon
        .admit_resource_instance(
            ResourceInstanceInput {
                label: name("account"),
                resource: cash,
            },
            at,
        )
        .expect("an instance is admissible");

    let receive = canon
        .admit_action(
            ActionInput {
                verb: name("receive"),
                kind: ActionKind::Quantifiable(Effect::Increase),
                resource: cash,
            },
            at,
        )
        .expect("an action is admissible");

    let spend = canon
        .admit_action(
            ActionInput {
                verb: name("spend"),
                kind: ActionKind::Quantifiable(Effect::Decrease),
                resource: cash,
            },
            at,
        )
        .expect("an action is admissible");

    let settlement =
        Settlement::new([settling()], [cancelling()]).expect("both outcomes are named");

    let inbound = canon
        .admit_statement(
            StatementInput {
                participants: Participants::new([counterparty], [spender])
                    .expect("both sides are named"),
                action: receive,
                settlement: settlement.clone(),
            },
            at,
        )
        .expect("a statement is admissible");

    let outbound = canon
        .admit_statement(
            StatementInput {
                participants: Participants::new([spender], [counterparty])
                    .expect("both sides are named"),
                action: spend,
                settlement,
            },
            at,
        )
        .expect("a statement is admissible");

    let opening = canon
        .admit_commitment(
            CommitmentInput {
                assignment: Assignment::new(market, [market], [house])
                    .expect("both sides are staffed"),
                statement: inbound,
                resource: account,
                term: Term::new(day(1), day(2)).expect("committed before due"),
                action_value: ActionValue::value(100.0).expect("a positive, finite magnitude"),
                dependencies: [].into(),
            },
            day(1),
        )
        .expect("the opening is admissible");

    canon
        .admit_event(
            EventSubmission {
                commitment_id: opening,
                observation: settling(),
                occurred_at: day(2),
            },
            day(2),
        )
        .expect("the opening settles");

    World {
        canon,
        house,
        market,
        spender,
        counterparty,
        inbound,
        outbound,
        account,
        opening,
    }
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
