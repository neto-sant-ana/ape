//! The divergence subject: a world its own bounds refuse, a fact that changes whether they
//! do, and an alternative reached by choice.
//!
//! ```text
//! cash ∈ [0, 100]
//!
//! A  receive  50   recorded day 5     ─┐
//! B  spend   120   recorded day 5     ─┴▶ together, −70
//! C  spend    30   recorded day 11    ──▶ with A, 20
//! ```
//!
//! Two directions over one instance are what make a refusal reachable at all: increases
//! alone can only ever leave a level above where it started, so the previous subject had no
//! way to produce a verdict rather than an absence of one.
//!
//! What this subject adds is *when* knowledge arrives. The cancelling Event is recorded
//! within the instant the genesis names, after the genesis was decided, so the instant a
//! decision was taken at and the knowledge it was taken against stop being the same body.
//! Everything the experiment expects to find follows from that one arrangement.
//!
//! Every quantity is an integer, for the reason [`super::reconstruction`] gives.

use ape::canon::Canon;
use ape::kernel::entities::{AgentId, CommitmentId, ResourceInstanceId, StatementId};

use crate::error::JournalError;
use crate::history::ResidentHistory;
use crate::journal::{
    ActionKindRecord, Admission, AgentKindRecord, EffectRecord, ResourceKindRecord, replay,
};
use crate::lineage::Decision;

pub const FULFILLING: &str = "Settled";
pub const CANCELLING: &str = "Void";

/// What the procedure refers to across phases, and the journal that produced it.
pub struct Constructed {
    /// `A` — the inflow that makes the account solvent on its own.
    pub inflow: CommitmentId,
    /// `B` — the outflow the bounds cannot accommodate, and the one an Event later cancels.
    pub overspend: CommitmentId,
    pub instance: ResourceInstanceId,
    pub journal: Vec<Admission>,
    /// What Phase 3 needs to word an outflow of its own, since a commitment refers to a
    /// statement and to agents by identity and those exist only once admitted.
    spending: Spending,
}

/// The vocabulary an outflow is worded from.
struct Spending {
    payer: AgentId,
    payee: AgentId,
    statement: StatementId,
}

/// Admit the subject, accumulating the journal that describes it.
pub fn construct(canon: &mut Canon<ResidentHistory>) -> Result<Constructed, JournalError> {
    let mut journal = Vec::new();

    let vocabulary = vec![
        Admission::Role {
            label: "payer".into(),
            recorded_at: day(1),
        },
        Admission::Role {
            label: "payee".into(),
            recorded_at: day(1),
        },
        Admission::Agent {
            label: "customer".into(),
            kind: AgentKindRecord::Company,
            recorded_at: day(1),
        },
        Admission::Agent {
            label: "merchant".into(),
            kind: AgentKindRecord::Company,
            recorded_at: day(1),
        },
        Admission::Resource {
            label: "cash".into(),
            kind: ResourceKindRecord::Between {
                lower: 0.0,
                upper: 100.0,
            },
            recorded_at: day(1),
        },
    ];
    let named = replay(canon, &vocabulary)?;
    journal.extend(vocabulary);

    let (payer, payee) = (named.roles[0], named.roles[1]);
    let (customer, merchant) = (named.agents[0], named.agents[1]);
    let cash = named.resources[0];

    let bound = vec![
        Admission::Eligibility {
            agent: customer,
            roles: [payer].into(),
            effective_from: day(1),
            recorded_at: day(1),
        },
        Admission::Eligibility {
            agent: merchant,
            roles: [payee].into(),
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
    ];
    let placed = replay(canon, &bound)?;
    journal.extend(bound);

    let instance = placed.instances[0];
    let (receive, spend) = (placed.actions[0], placed.actions[1]);

    let proposed = vec![
        Admission::Statement {
            actors: [payer].into(),
            recipients: [payee].into(),
            action: receive,
            fulfills: [FULFILLING.to_owned()].into(),
            cancels: [CANCELLING.to_owned()].into(),
            recorded_at: day(1),
        },
        Admission::Statement {
            actors: [payee].into(),
            recipients: [payer].into(),
            action: spend,
            fulfills: [FULFILLING.to_owned()].into(),
            cancels: [CANCELLING.to_owned()].into(),
            recorded_at: day(1),
        },
    ];
    let stated = replay(canon, &proposed)?;
    journal.extend(proposed);

    let (inflow, outflow) = (stated.statements[0], stated.statements[1]);

    let intended = vec![
        Admission::Commitment {
            accountable: customer,
            executors: [customer].into(),
            beneficiaries: [merchant].into(),
            statement: inflow,
            resource: instance,
            committed_at: day(5),
            due_date: day(20),
            magnitude: Some(50.0),
            dependencies: [].into(),
            recorded_at: day(5),
        },
        Admission::Commitment {
            accountable: merchant,
            executors: [merchant].into(),
            beneficiaries: [customer].into(),
            statement: outflow,
            resource: instance,
            committed_at: day(5),
            due_date: day(20),
            magnitude: Some(120.0),
            dependencies: [].into(),
            recorded_at: day(5),
        },
    ];
    let committed = replay(canon, &intended)?;
    journal.extend(intended);

    Ok(Constructed {
        inflow: committed.commitments[0],
        overspend: committed.commitments[1],
        instance,
        journal,
        spending: Spending {
            payer: merchant,
            payee: customer,
            statement: outflow,
        },
    })
}

/// The genesis decision: the world Phase 1 reasons about, and the instant it is taken at.
///
/// The instant is one an Event has yet to reach, so the cut resolves an empty chain. It is
/// also the instant Phase 2 records that Event within, which is the whole arrangement: the
/// decision names a day the day is not finished with.
pub fn genesis(inflow: CommitmentId, overspend: CommitmentId) -> Decision {
    Decision::Genesis {
        known_at: day(10),
        selection: [inflow, overspend].into(),
    }
}

/// The Event that cancels the overspend, recorded within the instant the genesis names.
///
/// Nothing about it is irregular. Knowledge arrives when it arrives, and an application does
/// not stop deciding while a day is still in progress. What it costs is that the instant
/// `2026-01-10` no longer addresses one body of knowledge: it addresses an empty chain
/// before this admission and a chain ending here afterwards.
pub fn cancellation(overspend: CommitmentId) -> Admission {
    Admission::Event {
        commitment: overspend,
        observation: CANCELLING.into(),
        occurred_at: day(10),
        recorded_at: day(10),
    }
}

/// The advancement Phase 2 decides, at an instant later than the genesis.
pub fn advancement() -> Decision {
    Decision::Advance { known_at: day(15) }
}

/// `C` — an outflow the account can afford, admitted after the cancellation.
///
/// It is recorded within the advanced world's cut, so a fork may introduce it. Nothing
/// selects it until one does: an admission is knowledge, and knowledge is not intention.
pub fn alternative(subject: &Constructed) -> Admission {
    Admission::Commitment {
        accountable: subject.spending.payer,
        executors: [subject.spending.payer].into(),
        beneficiaries: [subject.spending.payee].into(),
        statement: subject.spending.statement,
        resource: subject.instance,
        committed_at: day(11),
        due_date: day(25),
        magnitude: Some(30.0),
        dependencies: [].into(),
        recorded_at: day(11),
    }
}

/// The fork Phase 3 decides: the same cut, spending what the account can carry.
///
/// Nothing is omitted. The overspend the parent selects is frozen by the cancellation, so
/// omitting it is not available — the alternative is reached by adding an intention rather
/// than by withdrawing one.
pub fn fork(alternative: CommitmentId) -> Decision {
    Decision::Fork {
        omitted: [].into(),
        introduced: [alternative].into(),
    }
}

fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
