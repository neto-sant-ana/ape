//! The provenance subject: two plans, one of which the account refuses, and a world that could
//! have taken its tooling from either.
//!
//! ```text
//! cash ∈ [0, 100]
//!
//! A  receive 40   recorded day 2   the funding
//! X  spend   15   recorded day 3   tooling, which both plans want
//! Y  spend   30   recorded day 3   an expansion, which one plan and the receiver want
//! Z  receive 25   recorded day 3   a grant only the receiving line takes
//! ```
//!
//! ```text
//!                    ancestor { A }                          40
//!         ┌──────────────┬───────────────────┐
//!   narrow { A, X }      │           receiving { A, Y, Z }    25 / 35
//!               broad { A, X, Y }                 │           −5  ← refused
//!                                          adopting { A, X, Y, Z }   20
//! ```
//!
//! # The ambiguity, and why it is the engine being right
//!
//! `narrow` wants tooling. `broad` wants tooling **and** the expansion. Neither withdraws
//! anything, so as intentions over the ancestor they differ in what they *introduce* — which is
//! the substance of a transfer rather than an edge of it.
//!
//! The receiving line had already decided on the expansion for its own reasons. A resolved
//! transfer drops what the Target already holds, because introducing it again asks nothing — so
//! both plans ask the receiving line for exactly the same change, and the world that results
//! carries no trace of which was consulted.
//!
//! Two substantive intentions coinciding because the Target already reflects the difference is a
//! stronger arrangement than two intentions differing by a withdrawal the Target had already
//! made. The first draft of this subject did the latter; a withdrawal that asks for nothing is a
//! disagreement a reader can dismiss, and both plans here asked for something real.
//!
//! # The consequence, which is what makes the question worth asking
//!
//! `broad` is **refused by the account's own bounds**: 40 − 15 − 30 leaves −5, outside them. So
//! the two candidate Sources are not merely different records — one is a plan that works and the
//! other is a plan that cannot.
//!
//! `adopting` is feasible either way, at 20, because the grant the receiving line holds pays for
//! the expansion. So *"did this world take its intention from a line the account refuses?"* is a
//! question with a consequence outside the world's own contents, and it is the question the
//! record cannot answer.
//!
//! No commitment depends on another. Every quantity is an integer, for the reason
//! [`super::reconstruction`] gives.

use ape::canon::Canon;
use ape::engine::synthesis::{ApplicabilityReport, ApplicabilityStatus, synthesize};
use ape::engine::thesis::{Thesis, ThesisId};
use ape::kernel::entities::{CommitmentId, ResourceInstanceId};

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
    /// `A` — the inflow every world here holds.
    pub funding: CommitmentId,
    /// `X` — what both plans introduce, and the only thing that ever travels.
    pub tooling: CommitmentId,
    /// `Y` — what one plan introduces alongside it, and what the receiver already decided.
    pub expansion: CommitmentId,
    /// `Z` — an inflow only the receiving line takes, and what pays for the expansion.
    pub grant: CommitmentId,
    pub instance: ResourceInstanceId,
    pub journal: Vec<Admission>,
    pub admitted: Replayed,
}

/// Admit the subject, accumulating the journal that describes it.
pub fn construct(canon: &mut Canon<ResidentHistory>) -> Result<Constructed, JournalError> {
    let mut journal = Vec::new();
    let mut admitted = Replayed::default();

    journal.extend([
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
    ]);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let (payer, payee) = (admitted.roles[0], admitted.roles[1]);
    let (customer, merchant) = (admitted.agents[0], admitted.agents[1]);
    let cash = admitted.resources[0];

    journal.extend([
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
    ]);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let instance = admitted.instances[0];
    let (receive, spend) = (admitted.actions[0], admitted.actions[1]);

    journal.extend([
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
    ]);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let (inflow, outflow) = (admitted.statements[0], admitted.statements[1]);

    // Each statement names the roles its parties must hold, so which agent plays which side is
    // fixed by the statement rather than chosen here. An inflow is the customer's to execute; an
    // outflow is the merchant's.
    let flowing =
        |statement, executor, beneficiary, magnitude: f64, day_of: u8| Admission::Commitment {
            accountable: executor,
            executors: [executor].into(),
            beneficiaries: [beneficiary].into(),
            statement,
            resource: instance,
            committed_at: day(day_of),
            due_date: day(25),
            magnitude: Some(magnitude),
            dependencies: [].into(),
            recorded_at: day(day_of),
        };

    journal.extend([
        flowing(inflow, customer, merchant, 40.0, 2),
        flowing(outflow, merchant, customer, 15.0, 3),
        flowing(outflow, merchant, customer, 30.0, 3),
        flowing(inflow, customer, merchant, 25.0, 3),
    ]);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    Ok(Constructed {
        funding: admitted.commitments[0],
        tooling: admitted.commitments[1],
        expansion: admitted.commitments[2],
        grant: admitted.commitments[3],
        instance,
        journal,
        admitted,
    })
}

/// The common ancestor: funded, and nothing yet decided about spending it.
pub fn genesis(funding: CommitmentId) -> Decision {
    Decision::Genesis {
        known_at: day(10),
        selection: [funding].into(),
    }
}

/// The narrow plan: tooling, and nothing else.
pub fn narrow(extends: ThesisId, tooling: CommitmentId) -> Decision {
    Decision::Fork {
        extends,
        omitted: [].into(),
        introduced: [tooling].into(),
    }
}

/// The broad plan: the same tooling, and an expansion alongside it.
///
/// The extra introduction is what tells this apart from [`narrow`] as an intention. Both are
/// substantive requests — neither is a withdrawal that asks for nothing — and what makes them
/// coincide is the receiving line already holding the expansion.
///
/// It is also the plan the account refuses: 40 − 15 − 30 is outside its bounds.
pub fn broad(extends: ThesisId, tooling: CommitmentId, expansion: CommitmentId) -> Decision {
    Decision::Fork {
        extends,
        omitted: [].into(),
        introduced: [tooling, expansion].into(),
    }
}

/// The receiving line: the same expansion, decided for its own reasons, and a grant to pay it.
pub fn receiving(extends: ThesisId, expansion: CommitmentId, grant: CommitmentId) -> Decision {
    Decision::Fork {
        extends,
        omitted: [].into(),
        introduced: [expansion, grant].into(),
    }
}

/// Everything the arrangement produces before anything is transferred.
pub struct Ambiguous {
    pub canon: Canon<ResidentHistory>,
    pub subject: Constructed,
    pub journal: Vec<Admission>,
    pub admitted: Replayed,
    pub decisions: Vec<Taken>,
    pub lineage: Lineage,
}

impl Ambiguous {
    pub fn ancestor(&self) -> &Thesis {
        &self.lineage.decided()[0]
    }

    /// The plan that wanted tooling alone.
    pub fn narrow(&self) -> &Thesis {
        &self.lineage.decided()[1]
    }

    /// The plan that wanted tooling and the expansion, and that the account refuses.
    pub fn broad(&self) -> &Thesis {
        &self.lineage.decided()[2]
    }

    /// The line an intention will be carried into.
    pub fn receiving(&self) -> &Thesis {
        &self.lineage.decided()[3]
    }
}

/// Admit the subject and decide the ancestor, the two rival lines, and the receiving line.
///
/// Nothing is admitted between decisions. This experiment is not about when knowledge arrives —
/// the previous one settled what that costs — so the journal is complete before the first
/// decision and every coordinate points at its last entry.
pub fn ambiguous() -> Result<Ambiguous, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let subject = construct(&mut canon)?;

    let journal = subject.journal.clone();
    let admitted = subject.admitted.clone();

    let mut decisions = Vec::new();
    let mut lineage = Lineage::new();

    let mut take = |decision: Decision| -> Result<ThesisId, SubjectError> {
        let taken = Taken::now(decision, &admitted)?;
        lineage::decide(canon.history(), &mut lineage, &taken.decision)?;
        decisions.push(taken);

        Ok(lineage.decided().last().expect("just decided").id())
    };

    let ancestor = take(genesis(subject.funding))?;
    take(narrow(ancestor, subject.tooling))?;
    take(broad(ancestor, subject.tooling, subject.expansion))?;
    take(receiving(ancestor, subject.expansion, subject.grant))?;

    Ok(Ambiguous {
        canon,
        subject,
        journal,
        admitted,
        decisions,
        lineage,
    })
}

/// The arrangement with the tooling carried into the receiving line.
pub struct Adopted {
    pub canon: Canon<ResidentHistory>,
    pub subject: Constructed,
    pub journal: Vec<Admission>,
    pub decisions: Vec<Taken>,
    pub lineage: Lineage,
    /// Which line the intention actually came from.
    ///
    /// Held here because nothing else holds it. It is what the experiment knows and the
    /// repository does not, and every question about necessity is a question about this value.
    pub source: ThesisId,
    /// The report the decision was taken from.
    pub carried: ApplicabilityReport,
}

impl Adopted {
    pub fn ancestor(&self) -> &Thesis {
        &self.lineage.decided()[0]
    }

    pub fn narrow(&self) -> &Thesis {
        &self.lineage.decided()[1]
    }

    pub fn broad(&self) -> &Thesis {
        &self.lineage.decided()[2]
    }

    pub fn receiving(&self) -> &Thesis {
        &self.lineage.decided()[3]
    }

    /// The world the transfer produced.
    pub fn adopting(&self) -> &Thesis {
        &self.lineage.decided()[4]
    }
}

/// Carry the tooling from the **broad** plan into the receiving line.
///
/// Which of the two plans is used is the point of the arrangement, and it is chosen rather than
/// left open: the intention comes from `broad`, the plan the account refuses, and `narrow` is the
/// account the record will not be able to rule out.
pub fn adopted() -> Result<Adopted, SubjectError> {
    let Ambiguous {
        canon,
        subject,
        journal,
        admitted,
        mut decisions,
        mut lineage,
    } = ambiguous()?;

    let (base, source, target) = (
        lineage.decided()[0].id(),
        lineage.decided()[2].id(),
        lineage.decided()[3].id(),
    );

    let carried = synthesize(lineage.archive(), canon.history(), base, source, target)?;

    let ApplicabilityStatus::Applicable { transfer, .. } = carried.status() else {
        return Err(SubjectError::TransferNotApplicable);
    };

    decisions.push(Taken::now(transfer::applied(target, transfer), &admitted)?);
    lineage::decide(canon.history(), &mut lineage, &decisions[4].decision)?;

    Ok(Adopted {
        canon,
        subject,
        journal,
        decisions,
        lineage,
        source,
        carried,
    })
}

/// The Event that voids the tooling — the one commitment that ever travelled.
///
/// This is as close to *discrediting* a line of thinking as this repository admits. Nothing here
/// can be wrong: knowledge is fact, and a plan is not refuted by argument. What can happen is that
/// the world stops cooperating with it, and an Event is how that arrives.
pub fn cancellation(tooling: CommitmentId) -> Admission {
    Admission::Event {
        commitment: tooling,
        observation: CANCELLING.into(),
        occurred_at: day(12),
        recorded_at: day(12),
    }
}

/// The advancement that lets the adopted world recognize it.
pub fn recognizing(extends: ThesisId) -> Decision {
    Decision::Advance {
        extends,
        known_at: day(15),
    }
}

/// The arrangement after the travelled intention has been voided.
pub struct Discredited {
    pub canon: Canon<ResidentHistory>,
    pub subject: Constructed,
    pub journal: Vec<Admission>,
    pub decisions: Vec<Taken>,
    pub lineage: Lineage,
}

impl Discredited {
    pub fn narrow(&self) -> &Thesis {
        &self.lineage.decided()[1]
    }

    pub fn broad(&self) -> &Thesis {
        &self.lineage.decided()[2]
    }

    pub fn receiving(&self) -> &Thesis {
        &self.lineage.decided()[3]
    }

    pub fn adopting(&self) -> &Thesis {
        &self.lineage.decided()[4]
    }

    /// The adopted world advanced far enough to see that the tooling was voided.
    pub fn recognizing(&self) -> &Thesis {
        &self.lineage.decided()[5]
    }
}

/// Void the tooling, and let the world that adopted it recognize the fact.
pub fn discredited() -> Result<Discredited, SubjectError> {
    let Adopted {
        mut canon,
        subject,
        mut journal,
        mut decisions,
        mut lineage,
        ..
    } = adopted()?;

    let mut admitted = subject.admitted.clone();

    journal.push(cancellation(subject.tooling));
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let adopting = lineage.decided()[4].id();
    decisions.push(Taken::now(recognizing(adopting), &admitted)?);
    lineage::decide(canon.history(), &mut lineage, &decisions[5].decision)?;

    Ok(Discredited {
        canon,
        subject,
        journal,
        decisions,
        lineage,
    })
}

fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
