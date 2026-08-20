//! The coordination subject: one budget, and two parties who each plan against it.
//!
//! ```text
//! cash ∈ [0, 100]
//!
//! B  receive 50   recorded day 2   the budget, and the whole of the shared ancestor
//! H  spend   20   recorded day 3   what one party plans
//! E  spend   15   recorded day 3   what the other plans
//! G  receive 30   recorded day 4   knowledge one party admits on its own
//! R  receive 10   recorded day 4   knowledge the other admits on its own
//! ```
//!
//! ```text
//!                shared { B }                50
//!            ┌────────┴────────┐
//!  staffing { B, H }    equipping { B, E }    30 / 35
//! ```
//!
//! Both lines are feasible, and so is a world holding both — 50 − 20 − 15 leaves 15. That is
//! deliberate: this experiment is about what a record does with two writers, and an arrangement
//! whose parties cannot coexist would answer a question about feasibility instead.
//!
//! # What is new here, and it is not the arrangement
//!
//! Every previous subject built a lineage in memory and wrote it down once at the end. This one
//! needs a party to **read a repository somebody else wrote and extend it**, which is a path the
//! laboratory has never had — four experiments' worth of one writer, and the exclusion was doing
//! more work than it looked like.
//!
//! So the apparatus is here: [`decide`] and [`admit`] extend what a party read, and [`write`]
//! puts it back the only way this repository writes, which is whole. Nothing coordinates. That
//! is the measurement.
//!
//! No commitment depends on another. Every quantity is an integer, for the reason
//! [`super::reconstruction`] gives.

use ape::canon::Canon;
use ape::engine::synthesis::{ApplicabilityStatus, synthesize};
use ape::engine::thesis::{Thesis, ThesisId};
use ape::kernel::entities::{AgentId, CommitmentId, ResourceInstanceId};

use ape_cli::error::{JournalError, SubjectError};
use ape_cli::history::ResidentHistory;
use ape_cli::journal::{
    self, ActionKindRecord, Admission, EffectRecord, Replayed, ResourceKindRecord,
};
use ape_cli::lineage::{self, Decision, Lineage, Taken};
use ape_cli::reading::{self, Corroborated, WorldRecord};
use ape_cli::repository::Repository;
use ape_cli::transfer;

pub const FULFILLING: &str = "Settled";
pub const CANCELLING: &str = "Void";

/// What the procedure refers to across phases, and the journal that produced it.
pub struct Constructed {
    /// `B` — the inflow the shared ancestor selects, and nothing else does.
    pub budget: CommitmentId,
    /// `H` — what one party plans.
    pub hiring: CommitmentId,
    /// `E` — what the other party plans.
    pub equipment: CommitmentId,
    /// `G` — an inflow nobody has admitted, held for one party to admit on its own.
    pub grant: Admission,
    /// `R` — the same, for the other party, so that the two admit different knowledge.
    pub rebate: Admission,
    /// The party that plans the hiring, held unadmitted.
    ///
    /// A party is knowledge before it is a claim. It arrives through the journal like every other
    /// fact, and the append-only rule applies to it — which is what makes *that this party exists*
    /// checkable and leaves *that it decided anything* not.
    pub planner: Admission,
    /// The party that plans the equipment, held the same way.
    pub steward: Admission,
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
                lower: 0,
                upper: 100,
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
        |statement, executor, beneficiary, magnitude: u128, day_of: u8| Admission::Commitment {
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
        flowing(inflow, customer, merchant, 50, 2),
        flowing(outflow, merchant, customer, 20, 3),
        flowing(outflow, merchant, customer, 15, 3),
    ]);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    Ok(Constructed {
        budget: admitted.commitments[0],
        hiring: admitted.commitments[1],
        equipment: admitted.commitments[2],
        grant: flowing(inflow, customer, merchant, 30, 4),
        rebate: flowing(inflow, customer, merchant, 10, 4),
        planner: deciding("planner"),
        steward: deciding("steward"),
        instance,
        journal,
        admitted,
    })
}

/// The shared ancestor: funded, and nothing yet decided about spending it.
pub fn shared(budget: CommitmentId) -> Decision {
    Decision::Genesis {
        known_at: day(10),
        selection: [budget].into(),
    }
}

/// A fork that withdraws nothing and asks for one more commitment.
///
/// Every intention in this arrangement has that shape, so there is one of these rather than
/// several: what tells the two parties' lines apart is which commitment, and which world.
pub fn also(extends: ThesisId, commitment: CommitmentId) -> Decision {
    Decision::Fork {
        extends,
        omitted: [].into(),
        introduced: [commitment].into(),
    }
}

/// The repository as both parties find it: the subject admitted, and one world decided.
pub struct Founded {
    pub subject: Constructed,
    pub decisions: Vec<Taken>,
    pub lineage: Lineage,
}

impl Founded {
    pub fn shared(&self) -> &Thesis {
        &self.lineage.decided()[0]
    }
}

/// Build the shared starting point, in one process, as the previous experiments did.
///
/// Nothing about this is contended. It is the state a repository is in *before* two parties look
/// at it, and it exists so that what follows is about extending a repository rather than about
/// creating one.
pub fn founded() -> Result<Founded, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let subject = construct(&mut canon)?;

    let mut lineage = Lineage::new();
    // Unattributed, as every experiment before this one wrote it. Which leaves the arrangement
    // holding a repository where some decisions name a party and some do not — the realistic shape
    // of an optional field, and one Phase 5 has to measure rather than tidy away.
    let taken = Taken::now(shared(subject.budget), &subject.admitted)?;

    lineage::decide(canon.history(), &mut lineage, &taken.decision)?;

    Ok(Founded {
        subject,
        decisions: vec![taken],
        lineage,
    })
}

/// Write a founded repository, so that parties have something to read.
pub fn found(repository: &Repository, founded: &Founded) -> Result<(), SubjectError> {
    repository.write_journal(&founded.subject.journal)?;
    repository.write_lineage(&founded.decisions)?;
    repository.write_worlds(&worlds(&founded.lineage))?;

    Ok(())
}

/// What a party reads before it decides anything.
///
/// A [`Corroborated`] is exactly a working copy: knowledge, the worlds already decided, the
/// coordinate the replay reached, and the two sequences as they were on disk. What makes it a
/// party rather than a reader is that it goes on to [`decide`] and [`write`].
pub fn read(repository: &Repository) -> Result<Corroborated, SubjectError> {
    Ok(reading::corroborated(repository)?)
}

/// Take a decision against what this party read, and nobody else's.
///
/// The cut resolves against the knowledge in hand and the parent resolves out of the archive in
/// hand, so a party that read an older repository decides in an older world — which is the
/// arrangement rather than a defect of it.
pub fn decide(working: &mut Corroborated, decision: Decision) -> Result<ThesisId, SubjectError> {
    taken(working, Taken::now(decision, &working.admitted)?)
}

/// The same, with the party that took it written down.
///
/// A second entry point rather than a parameter on the first, and that is the same constraint that
/// made the field optional: Part A's measurements were published against decisions that name
/// nobody, and a phase whose calls all changed would be a phase nobody can compare to what it
/// reported.
pub fn decided(
    working: &mut Corroborated,
    by: AgentId,
    decision: Decision,
) -> Result<ThesisId, SubjectError> {
    taken(working, Taken::claimed(decision, by, &working.admitted)?)
}

fn taken(working: &mut Corroborated, taken: Taken) -> Result<ThesisId, SubjectError> {
    lineage::decide(
        working.canon.history(),
        &mut working.lineage,
        &taken.decision,
    )?;
    working.decisions.push(taken);

    Ok(working
        .lineage
        .decided()
        .last()
        .ok_or(SubjectError::NothingDecided)?
        .id())
}

/// Admit knowledge against what this party read, extending its journal.
pub fn admit(working: &mut Corroborated, admission: Admission) -> Result<(), SubjectError> {
    working.journal.push(admission);

    journal::replay_remaining(&mut working.canon, &working.journal, &mut working.admitted)?;

    Ok(())
}

/// Take up an intention out of another party's line, against this party's own.
///
/// The Source is the other party's world, and the Target is this one's. Both have to be in the
/// archive this party holds, which is a precondition rather than a formality: a party that has not
/// read the other's line cannot name it, and a party that read before the other converged does not
/// have it. Reaching each other is downstream of converging.
///
/// Nothing about this is a party-to-party operation. The report names three worlds and no party, and
/// what comes out of it is an ordinary fork of the Target.
pub fn adopt(
    working: &mut Corroborated,
    base: ThesisId,
    source: ThesisId,
    target: ThesisId,
) -> Result<ThesisId, SubjectError> {
    let carried = resolved(working, base, source, target)?;

    decide(working, carried)
}

/// The same, with the party that took it up written down.
pub fn adopted(
    working: &mut Corroborated,
    by: AgentId,
    base: ThesisId,
    source: ThesisId,
    target: ThesisId,
) -> Result<ThesisId, SubjectError> {
    let carried = resolved(working, base, source, target)?;

    decided(working, by, carried)
}

fn resolved(
    working: &Corroborated,
    base: ThesisId,
    source: ThesisId,
    target: ThesisId,
) -> Result<Decision, SubjectError> {
    let report = synthesize(
        working.lineage.archive(),
        working.canon.history(),
        base,
        source,
        target,
    )?;

    let ApplicabilityStatus::Applicable { transfer, .. } = report.status() else {
        return Err(SubjectError::TransferNotApplicable);
    };

    Ok(transfer::applied(target, transfer))
}

/// Put back everything this party holds, whole.
///
/// Whole is not a choice made here. It is what [`Repository`] does, and it has never mattered
/// because nothing else was ever writing.
pub fn write(repository: &Repository, working: &Corroborated) -> Result<(), SubjectError> {
    repository.write_journal(&working.journal)?;
    repository.write_lineage(&working.decisions)?;
    repository.write_worlds(&worlds(&working.lineage))?;

    Ok(())
}

/// The witnesses for every world a lineage produced.
pub fn worlds(lineage: &Lineage) -> Vec<WorldRecord> {
    lineage.decided().iter().map(WorldRecord::of).collect()
}

/// A party that decides, and is a party to no commitment.
///
/// Which is the arrangement rather than an oversight: whoever selects a set of commitments need not
/// appear in any of them, and Phase 4 measured that the two populations here are disjoint. Recorded
/// on day 5, after everything the founded journal holds, so that a decision taken before it can be
/// attributed to it and refused.
fn deciding(label: &str) -> Admission {
    Admission::Agent {
        label: label.to_owned(),
        recorded_at: day(5),
    }
}

fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
