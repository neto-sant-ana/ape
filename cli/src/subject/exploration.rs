//! The exploration subject: a settled opening, and many candidate ways to spend it.
//!
//! ```text
//! cash ∈ [0, 1000]
//!
//! O  receive 100    recorded day 2, fulfilled by an Event on day 3   the opening
//! C  spend 10..120  recorded day 4, twelve of them                   the candidates
//! ```
//!
//! ```text
//!               opening { O }              O frozen, and nothing open
//!          ┌─────────┴─────────┐
//!  { O, C₁ }       …      { O, C₁₂ }       one world per candidate weighed
//! ```
//!
//! # The opening is frozen, and that is the arrangement
//!
//! Every previous subject decided its worlds over a selection that was entirely open. Here an Event
//! settles the opening before anything explores, so every world carries it frozen and no fork can
//! omit it. That is not a guard added for safety: an explorer able to withdraw the money it is
//! deciding how to spend would be exploring a different question, and this is the first arrangement
//! where a cut partitions a selection for a reason other than convention.
//!
//! # The objective is not the engine's, and neither is the hypothesis
//!
//! The engine contributes signed movements and findings under a named assumption. *Which* level
//! matters and what *better* means are this module's, fixed here before anything ran:
//!
//! > spend as much as the account admits, and never break the floor.
//!
//! The floor is not enforced here either. It is the resource's lower bound, so a candidate that
//! breaks it is refused by the engine reporting `OutOfBounds` — and the objective is left unable to
//! prefer a world the engine would not admit. What it does is rank: among worlds nothing was found
//! against, the one that leaves the account lowest.
//!
//! Which candidates exist is decided by [`CANDIDATES`] and by nothing else. An objective that chose
//! what to enumerate next would make every measurement here a measurement of the objective.
//!
//! No commitment depends on another. Every quantity is an integer, for the reason
//! [`super::reconstruction`] gives.

use ape::canon::Canon;
use ape::engine::hermeneia::{Conflict, Hypothesis};
use ape::engine::thesis::{Interpretation, Thesis, ThesisId};
use ape::kernel::entities::{AgentId, CommitmentId, ResourceInstanceId, StatementId};
use ape::kernel::value_objects::Date;

use crate::error::{JournalError, ReadingError, SubjectError};
use crate::history::ResidentHistory;
use crate::journal::{
    self, ActionKindRecord, Admission, EffectRecord, Replayed, ResourceKindRecord,
};
use crate::level;
use crate::lineage::{self, Decision, Lineage, Taken};
use crate::reading::{self, Corroborated, WorldRecord};
use crate::repository::Repository;

pub const FULFILLING: &str = "Settled";
pub const CANCELLING: &str = "Void";

/// The bounds the account answers to. The lower one is the floor the objective must not break.
pub const FLOOR: f64 = 0.0;
pub const CEILING: f64 = 1000.0;

/// What the opening puts in the account, and settles.
pub const OPENING: f64 = 100.0;

/// The candidates, enumerated in advance and in this order.
///
/// Twelve, spanning the floor: ten of them fit inside the opening and two do not, so an arrangement
/// that weighed only admissible candidates would be measuring a subject that cannot refuse anything.
pub const CANDIDATES: [f64; BUDGET] = [
    10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0, 110.0, 120.0,
];

/// How many candidates every arrangement explores.
///
/// One number, read by all three, fixed before anything ran. An arrangement that chose its own
/// budget would produce three measurements nobody can put side by side.
pub const BUDGET: usize = 12;

/// How the twelve divide, written down before the run rather than derived from [`OPENING`].
///
/// A prediction computed from the same arithmetic it predicts cannot be wrong, and cannot be a
/// measurement either. So these are literals: ten candidates leave the account at or above the
/// floor, two leave it below, and the best a candidate can do is spend the opening exactly.
pub const ADMISSIBLE: usize = 10;
pub const REFUSED: usize = 2;
pub const BEST: f64 = 0.0;

/// The literals above, weighed against each other at compile time.
///
/// Here rather than in a phase, because none of it is a measurement: that the opening fits strictly
/// inside the bounds and that the two halves of the prediction cover the budget are relations among
/// numbers written on one afternoon. Asserted inside a test they would have looked like results.
///
/// What the engine makes of these candidates is Phase 1's to report, and nothing here anticipates it.
const _: () = assert!(FLOOR < OPENING && OPENING < CEILING);
const _: () = assert!(ADMISSIBLE + REFUSED == BUDGET);
const _: () = assert!(BEST == OPENING - CANDIDATES[ADMISSIBLE - 1]);

/// The assumption every candidate is weighed under.
///
/// Named once, because two candidates weighed under different assumptions are not comparable and
/// nothing in a score says which one produced it.
pub const HYPOTHESIS: Hypothesis = Hypothesis::FinalState;

/// The entries the journal holds once the subject is admitted, before anything explores.
pub const OPENED: usize = 14;

/// What the procedure refers to across phases, and the journal that produced it.
pub struct Constructed {
    /// `O` — the inflow the opening world selects, settled before anything explores.
    pub opening: CommitmentId,
    pub instance: ResourceInstanceId,
    /// What a candidate is made of, so that one can be minted after the subject is admitted.
    ///
    /// A candidate is knowledge the exploring admits rather than knowledge the subject holds, which
    /// is the whole point of the arrangement: the journal grows because something explored.
    outflow: StatementId,
    merchant: AgentId,
    customer: AgentId,
    pub journal: Vec<Admission>,
    pub admitted: Replayed,
}

impl Constructed {
    /// The candidate that spends `magnitude`, as an admission nobody has admitted yet.
    ///
    /// Every field but the magnitude is fixed, so two candidates of equal magnitude are one
    /// commitment by identity — which is what makes repetition measurable rather than arranged.
    pub fn candidate(&self, magnitude: f64) -> Admission {
        self.outflowing(magnitude, 4)
    }

    /// The same admission, recorded one day earlier: something the arrangement means, not weighs.
    ///
    /// It exists so that a phase can have knowledge in place *before* the recording instant that
    /// exploration will advance to. Without the gap, everything derived before exploration is
    /// derived at the same watermark as everything derived after, and a phase asking whether the
    /// watermark moves any derived answer would be asking about a watermark that never moved.
    ///
    /// Nothing else distinguishes the two, which is Observation 2's measurement rather than an
    /// oversight here: an intention and a candidate are one shape, and only their values differ.
    pub fn intention(&self, magnitude: f64) -> Admission {
        self.outflowing(magnitude, 3)
    }

    fn outflowing(&self, magnitude: f64, recorded_on: u8) -> Admission {
        Admission::Commitment {
            accountable: self.merchant,
            executors: [self.merchant].into(),
            beneficiaries: [self.customer].into(),
            statement: self.outflow,
            resource: self.instance,
            committed_at: day(recorded_on),
            due_date: day(20),
            magnitude: Some(magnitude),
            dependencies: [].into(),
            recorded_at: day(recorded_on),
        }
    }
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
                lower: FLOOR,
                upper: CEILING,
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

    journal.push(Admission::Commitment {
        accountable: customer,
        executors: [customer].into(),
        beneficiaries: [merchant].into(),
        statement: inflow,
        resource: instance,
        committed_at: day(2),
        due_date: day(20),
        magnitude: Some(OPENING),
        dependencies: [].into(),
        recorded_at: day(2),
    });
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let opening = admitted.commitments[0];

    journal.push(Admission::Event {
        commitment: opening,
        observation: FULFILLING.into(),
        occurred_at: day(3),
        recorded_at: day(3),
    });
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    Ok(Constructed {
        opening,
        instance,
        outflow,
        merchant,
        customer,
        journal,
        admitted,
    })
}

/// The world every arrangement starts from: the opening, and nothing proposed about spending it.
///
/// Its cut recognizes the Event, so the opening arrives frozen and the selection has an empty open
/// half. Naming the opening in the selection anyway is how the decision says what it was *for* —
/// the engine would have frozen it either way.
pub fn opening(opening: CommitmentId) -> Decision {
    Decision::Genesis {
        known_at: day(10),
        selection: [opening].into(),
    }
}

/// A fork that withdraws nothing and proposes one candidate.
pub fn spending(extends: ThesisId, candidate: CommitmentId) -> Decision {
    Decision::Fork {
        extends,
        omitted: [].into(),
        introduced: [candidate].into(),
    }
}

/// The instant the objective asks at.
///
/// After every due date the subject holds, so nothing the objective reads is waiting on a term to
/// begin. It is the objective's and not a caller's: an instant chosen per candidate would rank two
/// worlds by when they were asked about.
pub fn asked_at() -> Date {
    Date::from_ymd(2026, 1, 20).expect("the instant the objective asks at is a date")
}

/// The repository as an explorer finds it: the subject admitted, and one world decided.
pub struct Founded {
    pub subject: Constructed,
    pub decisions: Vec<Taken>,
    pub lineage: Lineage,
}

impl Founded {
    pub fn opening(&self) -> &Thesis {
        &self.lineage.decided()[0]
    }
}

/// Build the starting point, before anything explores.
pub fn founded() -> Result<Founded, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let subject = construct(&mut canon)?;

    let mut lineage = Lineage::new();
    let taken = Taken::now(opening(subject.opening), &subject.admitted)?;

    lineage::decide(canon.history(), &mut lineage, &taken.decision)?;

    Ok(Founded {
        subject,
        decisions: vec![taken],
        lineage,
    })
}

/// Write the starting repository, so that an arrangement has something to explore from.
pub fn found(repository: &Repository, founded: &Founded) -> Result<(), SubjectError> {
    repository.write_journal(&founded.subject.journal)?;
    repository.write_lineage(&founded.decisions)?;
    repository.write_worlds(&worlds(&founded.lineage))?;

    Ok(())
}

/// What an arrangement reads before it explores anything.
pub fn read(repository: &Repository) -> Result<Corroborated, SubjectError> {
    Ok(reading::corroborated(repository)?)
}

/// Admit a candidate against what this arrangement holds, extending its journal.
///
/// Every arrangement does this, and it is the half none of them can undo: comparison requires
/// construction, construction is admission, and the journal is where an admission lands.
pub fn admit(working: &mut Corroborated, admission: Admission) -> Result<(), SubjectError> {
    working.journal.push(admission);

    journal::replay_remaining(&mut working.canon, &working.journal, &mut working.admitted)?;

    Ok(())
}

/// Weigh a candidate without recording that it was weighed — arrangement A.
///
/// The world comes back and the lineage never hears of it, which is what an application does when it
/// interprets the cheap way. It is produced from the very [`Decision`] arrangement B would write
/// down, so the two arrangements cannot come to weigh different worlds; what differs between them is
/// only whether the world is kept.
pub fn considered(working: &Corroborated, decision: &Decision) -> Result<Thesis, SubjectError> {
    let (thesis, _) = lineage::produced(working.canon.history(), &working.lineage, decision)?;

    Ok(thesis)
}

/// Record a decision against what this arrangement holds — arrangement B's verb.
///
/// One call apart from [`considered`], and that call is what the experiment is about: this one keeps
/// the world, and keeps a record of having decided it.
///
/// Phase 1 had no use for it, and its absence was what kept arrangement A from recording anything by
/// accident. Phase 2 is what needed it: an applicability report is derived from three worlds, and
/// three worlds have to be in a repository before anything can ask.
pub fn decide(working: &mut Corroborated, decision: Decision) -> Result<ThesisId, SubjectError> {
    let taken = Taken::now(decision, &working.admitted)?;

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

/// Put back everything this arrangement holds, whole.
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

/// What the objective makes of one candidate world.
///
/// `Refused` carries the findings rather than a boolean, because *what* the engine found is the
/// only thing separating a candidate that breaks the floor from one that could never be realized
/// at all — and an objective is not permitted to work either out for itself.
#[derive(Debug, Clone, PartialEq)]
pub enum Judged {
    Refused(Vec<Conflict>),
    /// Nothing was found under [`HYPOTHESIS`], and this is where the account would be left.
    Admissible {
        level: f64,
    },
}

impl Judged {
    pub fn level(&self) -> Option<f64> {
        match self {
            Self::Refused(_) => None,
            Self::Admissible { level } => Some(*level),
        }
    }
}

/// Weigh one world by the objective, using only what the engine derives.
///
/// Two questions of the same interpretation, and they are asked in this order for a reason: a world
/// the engine found something against has no score, so scoring it first would produce a number the
/// objective then has to remember not to use.
pub fn judge(
    history: &ResidentHistory,
    thesis: &Thesis,
    instance: ResourceInstanceId,
) -> Result<Judged, ReadingError> {
    let interpretation = Interpretation::of(thesis, history)?;

    let conflicts = interpretation.feasibility_under(HYPOTHESIS)?;
    if !conflicts.conflicts().is_empty() {
        return Ok(Judged::Refused(conflicts.conflicts().to_vec()));
    }

    let projected = interpretation.conditions_at(&asked_at())?;

    Ok(Judged::Admissible {
        level: level::intended(history, &projected, instance)?,
    })
}

/// The best of what has been weighed: the admissible world leaving the account lowest.
///
/// A tie keeps the earlier one, which is the enumeration's order and not a preference — nothing in
/// the objective distinguishes two worlds that spend the same.
pub fn best<T: Copy>(weighed: &[(T, Judged)]) -> Option<(T, f64)> {
    weighed
        .iter()
        .filter_map(|(what, judged)| judged.level().map(|level| (*what, level)))
        .reduce(|best, next| if next.1 < best.1 { next } else { best })
}

fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
