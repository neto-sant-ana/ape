//! The witness subject: one history holding far more than any of its decisions is about.
//!
//! ```text
//! cash  ∈ [0, 1000]        what every world here is about
//! paper ∈ [0,  100]        a second resource no world ever selects
//!
//! F₁  receive 400   recorded day 2      the fund
//! F₂  receive 100   recorded day 2      a second inflow, so the Event chain has two links
//! E₁  Event settling F₁, day 3          the first link
//! E₂  Event settling F₂, day 3          what every cut here resolves to
//! G   spend    40   recorded day 4      what the early world proposes
//! T…  file  5,7,11,13  recorded day 5   four filings no world selects and no cut names
//! H   spend    70   recorded day 6      what the late world proposes
//! ```
//!
//! ```text
//!  base { } → { F₁, F₂ }   500     23 entries stood, 14 of them reached
//!    ├── early { F₁, F₂, G } 460     24 entries stood, 17 of them reached
//!    └── late  { F₁, F₂, H } 430     29 entries stood, 17 of them reached
//! ```
//!
//! # What the arrangement has to hold, and why each part of it
//!
//! **A tail, and a stated one.** The protocol's own constraint is that a phase must not be satisfied
//! by an arrangement it could not have failed, and Phase 1 can only report a difference if the journal
//! holds knowledge no decision is about. So the tail is not incidental: [`TAIL_VOCABULARY`] and
//! [`FILINGS`] are the amount of it, written down before the run, and [`DEPENDED`] is the prediction
//! they make measurable. An arrangement whose journal were all dependence would report *the sets
//! coincide* and prove nothing.
//!
//! **A second resource, so the tail is unreachable rather than merely unselected.** The filings move
//! `paper` in a `ledger`, through their own role, agent, action and statement. A tail built out of the
//! same vocabulary would have been reached anyway — by the statement, the instance, the eligibility —
//! and the difference between the two sets would have collapsed to the commitments alone.
//!
//! **One decision, two positions.** [`Side::restated`] is the second decision written a second time at
//! the end of the same history: the same [`Decision`], so the same world, over a longer prefix. That is
//! W2 as a comparison rather than as an argument, and it is why the tail is admitted *between* the two
//! positions.
//!
//! **A chain under every world, not an Event.** The genesis proposes **nothing** and is decided after
//! `E₂`, so its whole selection is what the cut froze. Two Events rather than one, because an Event's
//! identity contains its predecessor: with a single link, a closure that reached only the head would be
//! indistinguishable from one that walked, and the claim that a chain cannot be pruned would be
//! asserted rather than measured.
//!
//! **Two sides, differing only in knowledge nobody selects.** [`Filings::None`] admits the same
//! vocabulary and the same three commitments about cash, and none of the filings. So the two
//! repositories hold the **same worlds by identity** and journals that are out of step — which is the
//! state the collision experiment measured as refused, and the one Phase 3 has to reproduce before it
//! re-runs it.
//!
//! Every quantity is an integer, for the reason [`super::reconstruction`] gives.

use ape::canon::Canon;
use ape::engine::thesis::{Interpretation, Thesis, ThesisId};
use ape::kernel::entities::{
    ActionId, AgentId, CommitmentId, ResourceInstanceId, RoleId, StatementId,
};
use ape::kernel::value_objects::Date;

use ape_cli::error::{JournalError, ReadingError, RepositoryError, SubjectError};
use ape_cli::history::ResidentHistory;
use ape_cli::journal::{
    self, ActionKindRecord, Admission, EffectRecord, EntryId, Replayed, ResourceKindRecord,
};
use ape_cli::level;
use ape_cli::lineage::{self, Decision, Lineage, Taken};
use ape_cli::reading::WorldRecord;
use ape_cli::repository::{Repository, RepositoryInput};

pub const FULFILLING: &str = "Settled";
pub const CANCELLING: &str = "Void";

/// The bounds the account answers to.
pub const FLOOR: i128 = 0;
pub const CEILING: i128 = 1000;

/// The bound the tail's resource answers to, which nothing here ever weighs.
pub const TALLY: i128 = 100;

/// What the two inflows put in the account, in the order admitted.
pub const FUNDED: [u128; 2] = [400, 100];

/// What each world proposes against them: the early one's, then the late one's.
pub const PLANS: [u128; 2] = [40, 70];

/// The tail: four commitments on another resource, which no world selects and no cut names.
pub const FILINGS: [u128; 4] = [5, 7, 11, 13];

/// What each world intends the account to hold: the base, then the early, then the late.
pub const INTENDED: [i128; 3] = [500, 460, 430];

/// The vocabulary every world is a function of, and the vocabulary nothing reaches.
pub const VOCABULARY: usize = 12;
pub const TAIL_VOCABULARY: usize = 7;

/// What each journal comes to: with the filings, and without them.
pub const ENTRIES: usize = 29;
pub const ENTRIES_WITHOUT_FILINGS: usize = 25;

/// The position, one-based, at which the two journals first differ.
///
/// The left admits its first filing where the right admits its late plan. Both are commitments; only
/// one of them is selected by anything.
pub const DIVERGES_AT: usize = 25;

/// The worlds each side decides: the base, the early, the late.
pub const WORLDS: usize = 3;

/// How many entries stood when each decision was taken: base, early, late.
pub const WITNESSED: [usize; 3] = [23, 24, ENTRIES];

/// How many entries each world is a function of: base, early, late.
///
/// A prediction, written before the run. The base reaches the two inflows, both Events, and the
/// vocabulary those resolve through — fourteen of the twenty-three that stood. A plan adds its own
/// statement and its own action, and nothing else: seventeen of twenty-four, and seventeen of
/// twenty-nine.
pub const DEPENDED: [usize; 3] = [14, 17, 17];

/// The literals above, weighed against each other before anything runs.
const _: () = assert!(FUNDED[0] != FUNDED[1]);
const _: () = assert!(INTENDED[0] == FUNDED[0] as i128 + FUNDED[1] as i128);
const _: () = assert!(FLOOR < INTENDED[0] && INTENDED[0] < CEILING);
const _: () = assert!(INTENDED[1] == INTENDED[0] - PLANS[0] as i128);
const _: () = assert!(INTENDED[2] == INTENDED[0] - PLANS[1] as i128);
const _: () = assert!(PLANS[0] != PLANS[1]);
// A world holding both plans would be feasible, so nothing here is lost for being infeasible.
const _: () = assert!(FLOOR < INTENDED[0] - PLANS[0] as i128 - PLANS[1] as i128);
// The filings are distinct, or two of them would be one entry.
const _: () = assert!(FILINGS[0] != FILINGS[1] && FILINGS[1] != FILINGS[2]);
const _: () = assert!(FILINGS[2] != FILINGS[3] && FILINGS[0] != FILINGS[2]);
const _: () = assert!(FILINGS[1] != FILINGS[3] && FILINGS[0] != FILINGS[3]);
// The tail is the whole of the difference between the two journals.
const _: () = assert!(ENTRIES == ENTRIES_WITHOUT_FILINGS + FILINGS.len());
// Vocabulary, two inflows, two Events, two plans, and the tail.
const _: () = assert!(ENTRIES == VOCABULARY + TAIL_VOCABULARY + 2 + 2 + 2 + FILINGS.len());
// Every dependence set is a strict subset of the prefix that stood, which is W1.
const _: () = assert!(DEPENDED[0] < WITNESSED[0]);
const _: () = assert!(DEPENDED[1] < WITNESSED[1]);
const _: () = assert!(DEPENDED[2] < WITNESSED[2]);
// The late decision is where the gap is widest, and the early one is where it is narrowest.
const _: () = assert!(WITNESSED[2] - DEPENDED[2] > WITNESSED[1] - DEPENDED[1]);

/// Whether a side admits the tail nobody selects.
///
/// The one axis this subject varies, and it varies knowledge rather than intention: both sides decide
/// the same three worlds whichever way it is set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filings {
    All,
    None,
}

/// The three files one repository is made of, as values rather than as a directory.
pub struct Files {
    pub journal: Vec<Admission>,
    pub lineage: Vec<Taken>,
    pub worlds: Vec<WorldRecord>,
}

/// One whole repository, and what a phase needs to ask it anything.
pub struct Side {
    pub files: Files,
    pub instance: ResourceInstanceId,
    /// The two inflows, in the order admitted, each of which an Event settled.
    pub funds: [CommitmentId; 2],
    /// The plans, in the order admitted: the early world's, then the late world's.
    pub plans: [CommitmentId; 2],
    /// The commitments no world selects, in the order admitted.
    pub filings: Vec<CommitmentId>,
    /// The worlds this side decided, by identity, in the order decided.
    pub worlds: Vec<ThesisId>,
    /// The second decision, restated at the end of the same history.
    ///
    /// The same [`Decision`] as `files.lineage[1]`, so the same world — and a witness holding
    /// everything admitted since. It is deliberately **not** in `files.lineage`: a repository
    /// answering for one world twice is the collision experiment's subject, not this one's.
    pub restated: Taken,
}

/// Two whole repositories, differing only in knowledge neither of them decides about.
pub struct Arranged {
    pub left: Side,
    pub right: Side,
}

/// Found both sides from one statement of the subject.
pub fn arranged() -> Result<Arranged, SubjectError> {
    Ok(Arranged {
        left: side(Filings::All)?,
        right: side(Filings::None)?,
    })
}

/// One repository: the vocabulary, the fund, the plans, whatever tail it admits, three decisions.
pub fn side(filings: Filings) -> Result<Side, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let mut constructed = construct(&mut canon)?;

    for funded in constructed.funds {
        constructed.settle(&mut canon, funded)?;
    }

    let mut lineage = Lineage::new();
    let founding = Taken::now(founding_decision(), &constructed.admitted)?;

    lineage::decide(canon.history(), &mut lineage, &founding.decision)?;

    let base = lineage
        .decided()
        .first()
        .ok_or(SubjectError::NothingDecided)?
        .id();

    let early = constructed.spend(&mut canon, PLANS[0], day(4))?;
    let planning = Taken::now(also(base, early), &constructed.admitted)?;

    lineage::decide(canon.history(), &mut lineage, &planning.decision)?;

    if filings == Filings::All {
        for magnitude in FILINGS {
            constructed.file(&mut canon, magnitude)?;
        }
    }

    let late = constructed.spend(&mut canon, PLANS[1], day(6))?;
    let replanning = Taken::now(also(base, late), &constructed.admitted)?;

    lineage::decide(canon.history(), &mut lineage, &replanning.decision)?;

    // The same decision as `planning`, over everything admitted since — which is W2, and is why the
    // tail is admitted between the two positions.
    let restated = Taken::now(planning.decision.clone(), &constructed.admitted)?;

    Ok(Side {
        files: Files {
            journal: constructed.journal,
            lineage: vec![founding, planning, replanning],
            worlds: worlds(&lineage),
        },
        instance: constructed.account,
        funds: constructed.funds,
        plans: [early, late],
        filings: constructed.filed,
        worlds: lineage.decided().iter().map(Thesis::id).collect(),
        restated,
    })
}

/// Put a repository there the way an application does — one call, all or nothing.
pub fn write(repository: &Repository, files: &Files) -> Result<(), RepositoryError> {
    repository.write_whole(input(files))
}

pub fn input(files: &Files) -> RepositoryInput<'_> {
    RepositoryInput {
        journal: &files.journal,
        lineage: &files.lineage,
        worlds: &files.worlds,
        designations: &[],
    }
}

/// The addresses one journal produces, derived rather than carried.
pub fn entries(journal: &[Admission]) -> Result<Vec<EntryId>, JournalError> {
    let mut canon = Canon::new(ResidentHistory::new());

    Ok(journal::replay(&mut canon, journal)?.entries)
}

/// The first world: nothing proposed, and everything the recognized Event froze.
///
/// A genesis with an empty proposal on purpose. What it selects is entirely what the cut imposed, so
/// no world here has a dependence set that stops short of the Event chain.
pub fn founding_decision() -> Decision {
    Decision::Genesis {
        known_at: day(10),
        selection: [].into(),
    }
}

/// A fork that withdraws nothing and proposes one outflow.
pub fn also(extends: ThesisId, commitment: CommitmentId) -> Decision {
    Decision::Fork {
        extends,
        omitted: [].into(),
        introduced: [commitment].into(),
    }
}

/// The instant every reading is taken at, after every due date the subject holds.
pub fn asked_at() -> Date {
    Date::from_ymd(2026, 1, 20).expect("the instant every reading is taken at is a date")
}

/// What one world intends the account to hold.
pub fn intended(
    history: &ResidentHistory,
    thesis: &Thesis,
    instance: ResourceInstanceId,
) -> Result<i128, ReadingError> {
    let interpretation = Interpretation::of(thesis, history)?;
    let projected = interpretation.conditions_at(&asked_at())?;

    Ok(level::intended(history, &projected, instance)?)
}

/// The witnesses for every world a lineage produced.
pub fn worlds(lineage: &Lineage) -> Vec<WorldRecord> {
    lineage.decided().iter().map(WorldRecord::of).collect()
}

/// The vocabulary, the fund, and whatever a side goes on to admit.
pub struct Constructed {
    pub funds: [CommitmentId; 2],
    pub account: ResourceInstanceId,
    pub ledger: ResourceInstanceId,
    pub journal: Vec<Admission>,
    pub admitted: Replayed,
    /// The commitments admitted against the ledger, in order.
    pub filed: Vec<CommitmentId>,
    inflow: StatementId,
    outflow: StatementId,
    filing: StatementId,
    customer: AgentId,
    merchant: AgentId,
    clerk: AgentId,
}

impl Constructed {
    /// Admit one outflow of `magnitude` against the account, returning what it produced.
    pub fn spend(
        &mut self,
        canon: &mut Canon<ResidentHistory>,
        magnitude: u128,
        at: String,
    ) -> Result<CommitmentId, JournalError> {
        self.journal.push(Admission::Commitment {
            accountable: self.merchant,
            executors: [self.merchant].into(),
            beneficiaries: [self.customer].into(),
            statement: self.outflow,
            resource: self.account,
            committed_at: at.clone(),
            due_date: day(15),
            magnitude: Some(magnitude),
            dependencies: [].into(),
            recorded_at: at,
        });

        self.replay(canon)
    }

    /// Admit one filing of `magnitude` against the ledger, which no world will select.
    pub fn file(
        &mut self,
        canon: &mut Canon<ResidentHistory>,
        magnitude: u128,
    ) -> Result<CommitmentId, JournalError> {
        self.journal.push(Admission::Commitment {
            accountable: self.clerk,
            executors: [self.clerk].into(),
            beneficiaries: [self.clerk].into(),
            statement: self.filing,
            resource: self.ledger,
            committed_at: day(5),
            due_date: day(15),
            magnitude: Some(magnitude),
            dependencies: [].into(),
            recorded_at: day(5),
        });

        let filed = self.replay(canon)?;

        self.filed.push(filed);

        Ok(filed)
    }

    /// Admit one inflow of `magnitude` into the account.
    fn receive(
        &mut self,
        canon: &mut Canon<ResidentHistory>,
        magnitude: u128,
    ) -> Result<CommitmentId, JournalError> {
        self.journal.push(Admission::Commitment {
            accountable: self.customer,
            executors: [self.customer].into(),
            beneficiaries: [self.merchant].into(),
            statement: self.inflow,
            resource: self.account,
            committed_at: day(2),
            due_date: day(15),
            magnitude: Some(magnitude),
            dependencies: [].into(),
            recorded_at: day(2),
        });

        self.replay(canon)
    }

    /// Admit the Event that settles `commitment`, extending the chain every cut here resolves to.
    pub fn settle(
        &mut self,
        canon: &mut Canon<ResidentHistory>,
        commitment: CommitmentId,
    ) -> Result<(), JournalError> {
        self.journal.push(Admission::Event {
            commitment,
            observation: FULFILLING.into(),
            occurred_at: day(3),
            recorded_at: day(3),
        });

        journal::replay_remaining(canon, &self.journal, &mut self.admitted)
    }

    fn replay(&mut self, canon: &mut Canon<ResidentHistory>) -> Result<CommitmentId, JournalError> {
        journal::replay_remaining(canon, &self.journal, &mut self.admitted)?;

        Ok(*self
            .admitted
            .commitments
            .last()
            .expect("a commitment was just admitted"))
    }
}

/// Admit the vocabulary and the fund, accumulating the journal that describes them.
///
/// Two vocabularies rather than one, and the second is the point: `paper` has a role, an agent, an
/// eligibility, an instance, an action and a statement of its own, so a closure that reaches nothing
/// about it misses **seven** entries rather than a commitment.
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

    let account = admitted.instances[0];
    let (receive, spend) = (admitted.actions[0], admitted.actions[1]);

    journal.extend([
        statement(payer, payee, receive),
        statement(payee, payer, spend),
    ]);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    journal.extend([
        Admission::Role {
            label: "auditor".into(),
            recorded_at: day(1),
        },
        Admission::Agent {
            label: "clerk".into(),
            recorded_at: day(1),
        },
        Admission::Resource {
            label: "paper".into(),
            kind: ResourceKindRecord::Between {
                lower: FLOOR,
                upper: TALLY,
            },
            recorded_at: day(1),
        },
    ]);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let auditor = admitted.roles[2];
    let clerk = admitted.agents[2];
    let paper = admitted.resources[1];

    journal.extend([
        Admission::Eligibility {
            agent: clerk,
            roles: [auditor].into(),
            effective_from: day(1),
            recorded_at: day(1),
        },
        Admission::ResourceInstance {
            label: "ledger".into(),
            resource: paper,
            recorded_at: day(1),
        },
        Admission::Action {
            verb: "file".into(),
            kind: ActionKindRecord::Quantifiable(EffectRecord::Increase),
            resource: paper,
            recorded_at: day(1),
        },
    ]);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    // One statement for the tail, naming the auditor on both sides: a filing is something the clerk
    // does to the ledger, with nobody else in it.
    journal.push(statement(auditor, auditor, admitted.actions[2]));
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let mut constructed = Constructed {
        // Assigned by `receive` below. Nothing reads them in between, and the alternative is an
        // Option every phase would have to unwrap for a value the subject always has.
        funds: [CommitmentId::from([0u8; 32]); 2],
        account,
        ledger: admitted.instances[1],
        inflow: admitted.statements[0],
        outflow: admitted.statements[1],
        filing: admitted.statements[2],
        customer,
        merchant,
        clerk,
        journal,
        admitted,
        filed: Vec::new(),
    };

    constructed.funds = [
        constructed.receive(canon, FUNDED[0])?,
        constructed.receive(canon, FUNDED[1])?,
    ];

    Ok(constructed)
}

fn statement(actor: RoleId, recipient: RoleId, action: ActionId) -> Admission {
    Admission::Statement {
        actors: [actor].into(),
        recipients: [recipient].into(),
        action,
        fulfills: [FULFILLING.to_owned()].into(),
        cancels: [CANCELLING.to_owned()].into(),
        recorded_at: day(1),
    }
}

fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
