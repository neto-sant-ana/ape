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
use ape::engine::thesis::ThesisId;
use ape::kernel::entities::{AgentId, CommitmentId, ResourceInstanceId, StatementId};

use crate::error::{JournalError, SubjectError};
use crate::history::ResidentHistory;
use crate::journal::{
    self, ActionKindRecord, Admission, EffectRecord, Replayed, ResourceKindRecord,
};
use crate::lineage::{self, Decision, Lineage, Taken};

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
    /// Every entry admitted so far, accumulated through one reading rather than several.
    ///
    /// A decision written down carries the knowledge it was taken against, and that has to
    /// come from the same reading that produced the coordinate — two readings could disagree
    /// about the prefix while the writer believed it had recorded one.
    pub admitted: Replayed,
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
            recorded_at: day(1),
        },
        Admission::Agent {
            label: "merchant".into(),
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
    journal.extend(intended);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    Ok(Constructed {
        inflow: admitted.commitments[0],
        overspend: admitted.commitments[1],
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
///
/// The world it extends is named rather than implied. In this arrangement that is always the
/// world decided last, and saying so costs nothing — the arrangement is a line, and a line is
/// a tree that never branched.
pub fn advancement(extends: ThesisId) -> Decision {
    Decision::Advance {
        extends,
        known_at: day(15),
    }
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
pub fn fork(extends: ThesisId, alternative: CommitmentId) -> Decision {
    Decision::Fork {
        extends,
        omitted: [].into(),
        introduced: [alternative].into(),
    }
}

/// Everything the arrangement produces: three worlds, and the knowledge and decisions that
/// produced them.
pub struct Reasoned {
    pub canon: Canon<ResidentHistory>,
    pub subject: Constructed,
    pub alternative: CommitmentId,
    pub journal: Vec<Admission>,
    pub decisions: Vec<Taken>,
    pub lineage: Lineage,
}

/// The arrangement at its first decision: the subject admitted, and the genesis decided.
pub struct Begun {
    pub canon: Canon<ResidentHistory>,
    pub subject: Constructed,
    pub decisions: Vec<Taken>,
    pub lineage: Lineage,
}

/// Admit the subject and decide the genesis over it.
///
/// The decision is written down with the entry it was taken after, which an application knows
/// because it is the one admitting. Here that is `B`, the last commitment the subject admits —
/// the genesis is decided while the journal ends there.
pub fn begun() -> Result<Begun, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let subject = construct(&mut canon)?;

    let decisions = vec![Taken::now(
        genesis(subject.inflow, subject.overspend),
        &subject.admitted,
    )?];

    let mut lineage = Lineage::new();
    lineage::decide(canon.history(), &mut lineage, &decisions[0].decision)?;

    Ok(Begun {
        canon,
        subject,
        decisions,
        lineage,
    })
}

/// Run the arrangement whole: admit, decide, and interleave the two as prescribed.
///
/// This lives beside the subject rather than in a harness because **the order is the
/// subject** — the genesis is decided before the cancellation is recorded, and the
/// cancellation shares the instant the genesis named. An experiment holding its own copy of
/// that sequence would be observing a different arrangement while believing it observed this
/// one.
pub fn reasoned() -> Result<Reasoned, SubjectError> {
    let Begun {
        mut canon,
        subject,
        mut decisions,
        mut lineage,
    } = begun()?;

    let mut journal = subject.journal.clone();
    let mut admitted = subject.admitted.clone();

    journal.push(cancellation(subject.overspend));
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    decisions.push(Taken::now(advancement(tip(&lineage)?), &admitted)?);
    lineage::decide(canon.history(), &mut lineage, &decisions[1].decision)?;

    journal.push(alternative(&subject));
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let alternative = *admitted
        .commitments
        .last()
        .ok_or(SubjectError::NothingAdmitted)?;

    decisions.push(Taken::now(fork(tip(&lineage)?, alternative), &admitted)?);
    lineage::decide(canon.history(), &mut lineage, &decisions[2].decision)?;

    Ok(Reasoned {
        canon,
        subject,
        alternative,
        journal,
        decisions,
        lineage,
    })
}

/// The world decided last, which in a line is the only one a decision could extend.
fn tip(lineage: &Lineage) -> Result<ThesisId, SubjectError> {
    Ok(lineage
        .decided()
        .last()
        .ok_or(SubjectError::NothingDecided)?
        .id())
}

fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
