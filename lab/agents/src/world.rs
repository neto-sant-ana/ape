//! The world an agent is given, accumulated as the journal that describes it.
//!
//! ```text
//! cash ∈ [0, 1000]
//! account          the instance every intention moves
//! opening  +100    received and settled, and the only money the account has held
//! ```
//!
//! What this admits is a vocabulary and one settled fact. What it deliberately does not admit
//! is either option: the world offers a Statement under which cash can be spent, and turning an
//! option into an intention is the agent's job. Pre-constructing one would answer the first
//! question the experiment asks.
//!
//! # Two things about this world are the substrate's doing
//!
//! **The floor is written as a range.** The world it describes is *cash may not go below zero*,
//! with no ceiling. A journal records `Between { lower, upper }` and nothing else, following the
//! rule that an unused form is absent rather than approximated — so the floor is restated as a
//! range whose upper bound nothing in the scenario reaches, and the ceiling takes no part in any
//! verdict. The need for a floor-only form is a finding handed to the experiment that owns the
//! journal, not a change made here.
//!
//! The floor became a parameter when an experiment needed to change *the world* while leaving the
//! agents and their objectives alone, and a reserve the account must keep is the smallest such
//! change available. [`construct`] is the arrangement the first three experiments ran against and
//! is left saying so; [`with_cash`] is the same construction with the constraint named by the
//! caller.
//!
//! **The house is a party, and the agent is not.** `by` on a decision names an admitted Agent,
//! and the agent acting here is not one — it decides on the house's behalf and has no
//! representation of its own. So the party recorded is the house. That is the whole of what a
//! record can hold about who decided: a name that resolves against knowledge, and nothing about
//! who operated it.
//!
//! Every quantity is an integer. Levels accumulate in `f64`, where addition is not associative,
//! and an experiment about what an agent can express should not be measuring the last bit of a
//! float.

use std::collections::BTreeSet;

use ape::canon::Canon;
use ape::kernel::entities::{AgentId, CommitmentId, ResourceInstanceId, RoleId, StatementId};
use ape::kernel::value_objects::Date;

use ape_cli::error::JournalError;
use ape_cli::history::ResidentHistory;
use ape_cli::journal::{
    self, ActionKindRecord, Admission, EffectRecord, Replayed, ResourceKindRecord,
};

pub const FULFILLING: &str = "Settled";
pub const CANCELLING: &str = "Cancelled";

/// The world, and every handle needed to act within it.
pub struct Constructed {
    pub house: AgentId,
    pub market: AgentId,
    pub spender: RoleId,
    pub counterparty: RoleId,
    pub inbound: StatementId,
    pub outbound: StatementId,
    pub account: ResourceInstanceId,
    pub opening: CommitmentId,
    pub journal: Vec<Admission>,
    pub admitted: Replayed,
}

/// A day of January 2026, as a journal records one.
pub fn day(day: u8) -> String {
    on(day).to_iso()
}

/// The same instant, as the engine takes one.
pub fn on(day: u8) -> Date {
    Date::from_ymd(2026, 1, day).expect("a real date in January 2026")
}

/// The instant the world is current as of.
pub fn today() -> Date {
    on(6)
}

/// Cash as the first three experiments constrained it: a floor at zero, and no reachable ceiling.
pub fn cash() -> ResourceKindRecord {
    ResourceKindRecord::Between {
        lower: 0.0,
        upper: 1000.0,
    }
}

/// Admit the vocabulary, then the one fact that gives the account a level.
pub fn construct(canon: &mut Canon<ResidentHistory>) -> Result<Constructed, JournalError> {
    with_cash(canon, cash())
}

/// The same world, under a constraint the caller names.
pub fn with_cash(
    canon: &mut Canon<ResidentHistory>,
    cash: ResourceKindRecord,
) -> Result<Constructed, JournalError> {
    let mut journal = Vec::new();
    let mut admitted = Replayed::default();

    journal.extend([
        Admission::Role {
            label: "spender".into(),
            recorded_at: day(1),
        },
        Admission::Role {
            label: "counterparty".into(),
            recorded_at: day(1),
        },
        Admission::Agent {
            label: "house".into(),
            recorded_at: day(1),
        },
        Admission::Agent {
            label: "market".into(),
            recorded_at: day(1),
        },
        Admission::Resource {
            label: "cash".into(),
            kind: cash,
            recorded_at: day(1),
        },
    ]);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let (spender, counterparty) = (admitted.roles[0], admitted.roles[1]);
    let (house, market) = (admitted.agents[0], admitted.agents[1]);
    let cash = admitted.resources[0];

    journal.extend([
        Admission::Eligibility {
            agent: house,
            roles: [spender].into(),
            effective_from: day(1),
            recorded_at: day(1),
        },
        Admission::Eligibility {
            agent: market,
            roles: [counterparty].into(),
            effective_from: day(1),
            recorded_at: day(1),
        },
        Admission::ResourceInstance {
            label: "account".into(),
            resource: cash,
            recorded_at: day(1),
        },
        Admission::Action {
            verb: "receive".into(),
            kind: ActionKindRecord::Quantifiable(EffectRecord::Increase),
            resource: cash,
            recorded_at: day(1),
        },
        Admission::Action {
            verb: "spend".into(),
            kind: ActionKindRecord::Quantifiable(EffectRecord::Decrease),
            resource: cash,
            recorded_at: day(1),
        },
    ]);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let account = admitted.instances[0];
    let (receive, spend) = (admitted.actions[0], admitted.actions[1]);

    journal.extend([
        Admission::Statement {
            actors: [counterparty].into(),
            recipients: [spender].into(),
            action: receive,
            fulfills: [FULFILLING.into()].into(),
            cancels: [CANCELLING.into()].into(),
            recorded_at: day(1),
        },
        Admission::Statement {
            actors: [spender].into(),
            recipients: [counterparty].into(),
            action: spend,
            fulfills: [FULFILLING.into()].into(),
            cancels: [CANCELLING.into()].into(),
            recorded_at: day(1),
        },
    ]);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let (inbound, outbound) = (admitted.statements[0], admitted.statements[1]);

    journal.push(Admission::Commitment {
        accountable: market,
        executors: [market].into(),
        beneficiaries: [house].into(),
        statement: inbound,
        resource: account,
        committed_at: day(1),
        due_date: day(2),
        magnitude: Some(100.0),
        dependencies: [].into(),
        recorded_at: day(1),
    });
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let opening = admitted.commitments[0];

    journal.push(Admission::Event {
        commitment: opening,
        observation: FULFILLING.into(),
        occurred_at: day(2),
        recorded_at: day(2),
    });
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    Ok(Constructed {
        house,
        market,
        spender,
        counterparty,
        inbound,
        outbound,
        account,
        opening,
        journal,
        admitted,
    })
}

/// What an intention says, filled in by name.
///
/// Named rather than positional because two of these fields are days and a third is a boolean:
/// three arguments in a row that the compiler cannot tell apart, in a call whose meaning depends
/// entirely on their order.
pub struct Intention {
    pub magnitude: f64,
    pub incoming: bool,
    pub due: u8,
    pub recorded_at: u8,
    /// What this intention cannot stand without.
    ///
    /// Empty in the first three experiments, where nothing hung off anything else. A world in which
    /// something does is what makes a removal structurally refusable rather than merely unwise.
    pub dependencies: BTreeSet<CommitmentId>,
}

/// An intention of the house to move on the account, as a journal records one. Not admitted here —
/// a caller admits it when its sequence says to.
pub fn intention(world: &Constructed, input: Intention) -> Admission {
    let (accountable, beneficiary, statement) = if input.incoming {
        (world.market, world.house, world.inbound)
    } else {
        (world.house, world.market, world.outbound)
    };

    Admission::Commitment {
        accountable,
        executors: [accountable].into(),
        beneficiaries: [beneficiary].into(),
        statement,
        resource: world.account,
        committed_at: day(input.recorded_at),
        due_date: day(input.due),
        magnitude: Some(input.magnitude),
        dependencies: input.dependencies,
        recorded_at: day(input.recorded_at),
    }
}
