//! The convergence subject: one account, and two lines of thinking about what to do with it.
//!
//! ```text
//! cash ∈ [0, 100]
//!
//! A  receive  60   recorded day 2    ──▶ what both lines start from
//! L  spend    30   recorded day 3    ──▶ what one line decides
//! R  spend    40   recorded day 3    ──▶ what the other decides
//! ```
//!
//! The genesis selects `A` alone. Two forks of it introduce `L` and `R` respectively, so
//! neither descends from the other and both descend from the same world. That is the whole of
//! the arrangement, and it is the smallest one in which a transfer has anything to mean:
//! Synthesis measures a difference over a Base both sides passed through.
//!
//! Both lines are within the account's bounds on their own — 30 and 20 — which is what makes
//! the question about intention rather than about arithmetic. What a world holding both would
//! be is not decided here; it is what a transfer would produce, and the experiment asks for it
//! rather than arranging it.
//!
//! No commitment depends on another. The protocol asks for no dependencies unless the
//! procedure demands them, and nothing so far does.
//!
//! Every quantity is an integer, for the reason [`super::reconstruction`] gives.

use ape::canon::Canon;
use ape::engine::thesis::ThesisId;
use ape::kernel::entities::{AgentId, CommitmentId, ResourceInstanceId, StatementId};

use crate::error::JournalError;
use crate::history::ResidentHistory;
use crate::journal::{
    self, ActionKindRecord, Admission, AgentKindRecord, EffectRecord, Replayed, ResourceKindRecord,
};
use crate::lineage::Decision;

pub const FULFILLING: &str = "Settled";
pub const CANCELLING: &str = "Void";

/// What the procedure refers to across phases, and the journal that produced it.
pub struct Constructed {
    /// `A` — the inflow the common ancestor selects, and the only intention it holds.
    pub funding: CommitmentId,
    /// `L` — what one sibling introduces.
    pub equipment: CommitmentId,
    /// `R` — what the other introduces.
    pub inventory: CommitmentId,
    pub instance: ResourceInstanceId,
    pub journal: Vec<Admission>,
    /// Every entry admitted so far, accumulated through one reading rather than several.
    pub admitted: Replayed,
    /// What a later phase needs to word an outflow of its own, since a commitment refers to a
    /// statement and to agents by identity and those exist only once admitted.
    #[allow(dead_code)]
    spending: Spending,
}

/// The vocabulary an outflow is worded from.
struct Spending {
    #[allow(dead_code)]
    payer: AgentId,
    #[allow(dead_code)]
    payee: AgentId,
    #[allow(dead_code)]
    statement: StatementId,
}

/// Admit the subject, accumulating the journal that describes it.
pub fn construct(canon: &mut Canon<ResidentHistory>) -> Result<Constructed, JournalError> {
    let mut journal = Vec::new();
    let mut admitted = Replayed::default();

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
    journal.extend(vocabulary);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let (payer, payee) = (admitted.roles[0], admitted.roles[1]);
    let (customer, merchant) = (admitted.agents[0], admitted.agents[1]);
    let cash = admitted.resources[0];

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
    journal.extend(bound);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let instance = admitted.instances[0];
    let (receive, spend) = (admitted.actions[0], admitted.actions[1]);

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
    journal.extend(proposed);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let (inflow, outflow) = (admitted.statements[0], admitted.statements[1]);

    let intended = vec![
        Admission::Commitment {
            accountable: customer,
            executors: [customer].into(),
            beneficiaries: [merchant].into(),
            statement: inflow,
            resource: instance,
            committed_at: day(2),
            due_date: day(20),
            magnitude: Some(60.0),
            dependencies: [].into(),
            recorded_at: day(2),
        },
        Admission::Commitment {
            accountable: merchant,
            executors: [merchant].into(),
            beneficiaries: [customer].into(),
            statement: outflow,
            resource: instance,
            committed_at: day(3),
            due_date: day(20),
            magnitude: Some(30.0),
            dependencies: [].into(),
            recorded_at: day(3),
        },
        Admission::Commitment {
            accountable: merchant,
            executors: [merchant].into(),
            beneficiaries: [customer].into(),
            statement: outflow,
            resource: instance,
            committed_at: day(3),
            due_date: day(25),
            magnitude: Some(40.0),
            dependencies: [].into(),
            recorded_at: day(3),
        },
    ];
    journal.extend(intended);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    Ok(Constructed {
        funding: admitted.commitments[0],
        equipment: admitted.commitments[1],
        inventory: admitted.commitments[2],
        instance,
        journal,
        admitted,
        spending: Spending {
            payer: merchant,
            payee: customer,
            statement: outflow,
        },
    })
}

/// The common ancestor: the account funded, and nothing yet decided about spending it.
///
/// The instant is past everything admitted and no Event exists, so the cut freezes nothing and
/// the whole selection is open. Both siblings inherit that cut, which is what makes them
/// siblings rather than two worlds that merely resemble each other.
pub fn genesis(funding: CommitmentId) -> Decision {
    Decision::Genesis {
        known_at: day(10),
        selection: [funding].into(),
    }
}

/// One line of thinking: spend the funding on equipment.
pub fn equipping(extends: ThesisId, equipment: CommitmentId) -> Decision {
    Decision::Fork {
        extends,
        omitted: [].into(),
        introduced: [equipment].into(),
    }
}

/// The other: spend it on inventory.
///
/// Both decisions extend the same world, and nothing but `extends` says so. Written down
/// without it, this one would have been read as extending [`equipping`]'s result — which is
/// not a world anyone decided, and is what Phase 1 measured before naming was available.
pub fn stocking(extends: ThesisId, inventory: CommitmentId) -> Decision {
    Decision::Fork {
        extends,
        omitted: [].into(),
        introduced: [inventory].into(),
    }
}

fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
