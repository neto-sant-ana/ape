//! The convergence subject: one account, and two lines of thinking about what to do with it.
//!
//! ```text
//! cash ∈ [0, 100]
//!
//! A  receive  60   recorded day  2   ──▶ what both lines start from
//! L  spend    30   recorded day  3   ──▶ the equipment line's first decision
//! R  spend    40   recorded day  3   ──▶ the inventory line's first
//! M  spend    15   recorded day  4   ──▶ its second, over knowledge already there
//! N  spend    25   recorded day 12   ──▶ knowledge that arrives after both lines exist
//! ```
//!
//! The genesis selects `A` alone. Two forks of it introduce `L` and `R`, so neither descends
//! from the other and both descend from the same world — the smallest arrangement in which a
//! transfer has anything to mean, since Synthesis measures a difference over a Base both sides
//! passed through.
//!
//! ```text
//!               ancestor { A }                      day 10
//!            ┌────────┴────────┐
//!      equipping { A, L }   stocking { A, R }       day 10
//!            │                 │
//!      advanced { A, L }    maintaining { A, R, M } day 16 / day 10
//!            │
//!      provisioning { A, L, N }                     day 16
//! ```
//!
//! # The asymmetry, and why it is not arbitrary
//!
//! `N` is recorded after both lines have already forked. Only a world whose cut reaches it may
//! select it, and a fork inherits its parent's cut — so the equipment line has to **advance
//! before it can fork**, which is the engine's own reason for keeping the two operations
//! disjoint: knowledge becoming available is not the same event as someone deciding to use it.
//!
//! The inventory line needs no advance, because `M` was already knowledge when the ancestor
//! was decided and nobody had selected it. Its second decision is intention alone.
//!
//! That leaves the two lines at different cuts, which is what gives a transfer between them two
//! different answers depending on the direction it is asked in. Reaching that by arranging when
//! knowledge arrives, rather than by declaring two cuts, is what makes it a finding.
//!
//! # Arithmetic
//!
//! Each line is within the account's bounds on its own — 5 and 5 — and a world holding both
//! would be at −50. Neither of those is arranged as a verdict: what a transfer produces is what
//! the experiment asks for rather than what it builds.
//!
//! No commitment depends on another. The protocol asks for no dependencies unless the procedure
//! demands them, and nothing so far does.
//!
//! Every quantity is an integer, for the reason [`super::reconstruction`] gives.

use ape::canon::Canon;
use ape::engine::synthesis::{ApplicabilityReport, ApplicabilityStatus, synthesize};
use ape::engine::thesis::{Thesis, ThesisId};
use ape::kernel::entities::{AgentId, CommitmentId, ResourceInstanceId, StatementId};

use crate::error::{JournalError, SubjectError};
use crate::history::ResidentHistory;
use crate::journal::{
    self, ActionKindRecord, Admission, EffectRecord, Replayed, ResourceKindRecord,
};
use crate::lineage::{self, Decision, Lineage, Taken};
use crate::transfer;

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
    /// `M` — the inventory line's second decision, over knowledge the ancestor already had.
    pub maintenance: CommitmentId,
    pub instance: ResourceInstanceId,
    pub journal: Vec<Admission>,
    /// Every entry admitted so far, accumulated through one reading rather than several.
    pub admitted: Replayed,
    /// What [`contingency`] needs to word an outflow of its own, since a commitment refers to a
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
        Admission::Commitment {
            accountable: merchant,
            executors: [merchant].into(),
            beneficiaries: [customer].into(),
            statement: outflow,
            resource: instance,
            committed_at: day(4),
            due_date: day(25),
            magnitude: Some(15.0),
            dependencies: [].into(),
            recorded_at: day(4),
        },
    ];
    journal.extend(intended);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    Ok(Constructed {
        funding: admitted.commitments[0],
        equipment: admitted.commitments[1],
        inventory: admitted.commitments[2],
        maintenance: admitted.commitments[3],
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

/// The inventory line's second decision: also maintain what it already has.
///
/// Nothing had to happen for this to be available. `M` was recorded on the fourth and the
/// ancestor's cut is the tenth, so it was knowledge nobody had selected — which is what an
/// intention is free to change without any history moving.
pub fn maintaining(extends: ThesisId, maintenance: CommitmentId) -> Decision {
    Decision::Fork {
        extends,
        omitted: [].into(),
        introduced: [maintenance].into(),
    }
}

/// `N` — an outflow recorded after both lines had already forked.
///
/// It is the arrangement's second half. A commitment recorded past a world's cut is not a
/// commitment that world declined; it is one that world cannot see, and no fork of it can
/// select what it cannot see.
pub fn contingency(subject: &Constructed) -> Admission {
    Admission::Commitment {
        accountable: subject.spending.payer,
        executors: [subject.spending.payer].into(),
        beneficiaries: [subject.spending.payee].into(),
        statement: subject.spending.statement,
        resource: subject.instance,
        committed_at: day(12),
        due_date: day(30),
        magnitude: Some(25.0),
        dependencies: [].into(),
        recorded_at: day(12),
    }
}

/// The equipment line advancing, so that it can see what arrived after it forked.
///
/// This changes no intention, and that is the point of it being its own decision. The engine
/// keeps advancing and forking disjoint precisely so that an ancestry edge says which of the
/// two happened, and a lineage that could do both at once would lose that.
pub fn advancing(extends: ThesisId) -> Decision {
    Decision::Advance {
        extends,
        known_at: day(16),
    }
}

/// The equipment line's second intention: take the contingency it can now see.
pub fn provisioning(extends: ThesisId, contingency: CommitmentId) -> Decision {
    Decision::Fork {
        extends,
        omitted: [].into(),
        introduced: [contingency].into(),
    }
}

/// The arrangement at Phase 1: one ancestor and two worlds that extend it.
pub struct Branched {
    pub canon: Canon<ResidentHistory>,
    pub subject: Constructed,
    pub decisions: Vec<Taken>,
    pub lineage: Lineage,
}

impl Branched {
    pub fn ancestor(&self) -> &Thesis {
        &self.lineage.decided()[0]
    }

    pub fn equipping(&self) -> &Thesis {
        &self.lineage.decided()[1]
    }

    pub fn stocking(&self) -> &Thesis {
        &self.lineage.decided()[2]
    }
}

/// Everything Phase 2 produces: six worlds, and the knowledge and decisions behind them.
pub struct Diverged {
    pub canon: Canon<ResidentHistory>,
    pub subject: Constructed,
    /// `N`, which exists only once the journal has grown past the first three decisions.
    pub contingency: CommitmentId,
    pub journal: Vec<Admission>,
    /// Every entry admitted by the end, so a later decision can say what it stood on.
    pub admitted: Replayed,
    pub decisions: Vec<Taken>,
    pub lineage: Lineage,
}

/// Every world by the decision that produced it.
///
/// Named rather than indexed from the end, because the two that matter are not at the end. A
/// lineage that branches has as many tips as it has branches, and "the last decision" names one
/// of them only by accident of the order the decisions were taken in.
impl Diverged {
    pub fn ancestor(&self) -> &Thesis {
        &self.lineage.decided()[0]
    }

    pub fn equipping(&self) -> &Thesis {
        &self.lineage.decided()[1]
    }

    pub fn stocking(&self) -> &Thesis {
        &self.lineage.decided()[2]
    }

    /// The tip of the inventory line.
    pub fn maintaining(&self) -> &Thesis {
        &self.lineage.decided()[3]
    }

    pub fn advanced(&self) -> &Thesis {
        &self.lineage.decided()[4]
    }

    /// The tip of the equipment line.
    pub fn provisioning(&self) -> &Thesis {
        &self.lineage.decided()[5]
    }
}

/// Admit the subject and decide the ancestor and its two forks.
pub fn branched() -> Result<Branched, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let subject = construct(&mut canon)?;

    let mut decisions = Vec::new();
    let mut lineage = Lineage::new();

    let mut take = |decision: Decision| -> Result<ThesisId, SubjectError> {
        let taken = Taken::now(decision, &subject.admitted)?;
        lineage::decide(canon.history(), &mut lineage, &taken.decision)?;
        decisions.push(taken);

        Ok(lineage.decided().last().expect("just decided").id())
    };

    let ancestor = take(genesis(subject.funding))?;
    take(equipping(ancestor, subject.equipment))?;
    take(stocking(ancestor, subject.inventory))?;

    Ok(Branched {
        canon,
        subject,
        decisions,
        lineage,
    })
}

/// Run the arrangement whole: three decisions, then knowledge, then three more.
///
/// This lives beside the subject rather than in a harness because **the order is the subject**.
/// `N` is admitted after both lines have forked, and the equipment line advances only then —
/// an experiment holding its own copy of that sequence would be observing a different
/// arrangement while believing it observed this one.
///
/// The decisions do not alternate between the branches in the order a reader might expect, and
/// that is deliberate too: an application reasoning about alternatives returns to whichever one
/// it is thinking about, and the record has to survive that.
pub fn diverged() -> Result<Diverged, SubjectError> {
    let Branched {
        mut canon,
        subject,
        mut decisions,
        mut lineage,
    } = branched()?;

    let mut journal = subject.journal.clone();
    let mut admitted = subject.admitted.clone();

    let stocking = lineage.decided()[2].id();
    decisions.push(Taken::now(
        maintaining(stocking, subject.maintenance),
        &admitted,
    )?);
    lineage::decide(canon.history(), &mut lineage, &decisions[3].decision)?;

    journal.push(contingency(&subject));
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let contingency = *admitted
        .commitments
        .last()
        .ok_or(SubjectError::NothingAdmitted)?;

    let equipping = lineage.decided()[1].id();
    decisions.push(Taken::now(advancing(equipping), &admitted)?);
    lineage::decide(canon.history(), &mut lineage, &decisions[4].decision)?;

    let advanced = lineage.decided()[4].id();
    decisions.push(Taken::now(provisioning(advanced, contingency), &admitted)?);
    lineage::decide(canon.history(), &mut lineage, &decisions[5].decision)?;

    Ok(Diverged {
        canon,
        subject,
        contingency,
        journal,
        admitted,
        decisions,
        lineage,
    })
}

/// The arrangement with the transfer carried, and the report that said it could be.
pub struct Reconciled {
    pub canon: Canon<ResidentHistory>,
    pub subject: Constructed,
    pub contingency: CommitmentId,
    pub journal: Vec<Admission>,
    pub decisions: Vec<Taken>,
    pub lineage: Lineage,
    /// What Synthesis answered before the decision was taken.
    ///
    /// Held here rather than recomputed by a phase because the decision was taken *from* it: an
    /// arrangement that asked once and applied something else would be a different arrangement.
    pub carried: ApplicabilityReport,
}

impl Reconciled {
    /// The world the transfer produced.
    pub fn reconciling(&self) -> &Thesis {
        &self.lineage.decided()[6]
    }
}

/// Carry the inventory line's intention into the equipment line, as a decision.
///
/// The direction is the one Synthesis found applicable, which is not a preference: the other
/// direction is refused, and applying a refused transfer is not something the arrangement can do
/// without the engine building a world it already said was unavailable.
///
/// Nothing new is admitted. A transfer moves intention between worlds and knowledge is untouched
/// by it, so the journal at the end of this is the journal [`diverged`] left.
pub fn reconciled() -> Result<Reconciled, SubjectError> {
    let Diverged {
        canon,
        subject,
        contingency,
        journal,
        admitted,
        mut decisions,
        mut lineage,
    } = diverged()?;

    let (base, source, target) = (
        lineage.decided()[0].id(),
        lineage.decided()[3].id(),
        lineage.decided()[5].id(),
    );

    let carried = synthesize(lineage.archive(), canon.history(), base, source, target)?;

    let ApplicabilityStatus::Applicable { transfer, .. } = carried.status() else {
        return Err(SubjectError::TransferNotApplicable);
    };

    decisions.push(Taken::now(transfer::applied(target, transfer), &admitted)?);
    lineage::decide(canon.history(), &mut lineage, &decisions[6].decision)?;

    Ok(Reconciled {
        canon,
        subject,
        contingency,
        journal,
        decisions,
        lineage,
        carried,
    })
}

fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
