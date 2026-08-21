//! The collision subject: two whole repositories, and nothing between them.
//!
//! ```text
//! cash ∈ [0, 1000]
//!
//! F  receive 300   recorded day 2               the fund, and the whole of the shared base
//! S  Event settling F, day 3                    admitted on the left only, where a phase needs
//!                                               the two cuts to differ
//! G  spend    40   recorded day 4               what the left repository admits and decides about
//! H  spend    70   recorded day 4               what the right repository admits and decides about
//! U  receive  25   recorded day 4               admitted on the left only, selected by no world
//!                                               and named by no cut
//! ```
//!
//! ```text
//!  base { F }                    300     13 entries, 1 world — decided by BOTH, independently
//!    ├── left  { F, G }          260     14 entries, 2 worlds
//!    └── right { F, H }          230
//! ```
//!
//! # What the arrangement has to hold, and why each part of it
//!
//! **Two foundings from one construction.** Both repositories are built by [`construct`], so where
//! they share a base they share it **because the admissions were the same** and not because anything
//! copied one into the other. That is [`Relation::Shared`], and it is the whole of what C2 measures —
//! an arrangement that cloned would have answered the prediction by construction.
//!
//! **A mark, so that disjoint is really disjoint.** [`Relation::Disjoint`] founds the two under
//! different labels, beginning with the *first* admission — so the two journals differ at position 0
//! and the refusal has nowhere earlier to point.
//!
//! **One decision, taken twice.** Both sides take the same genesis over the same instant, so C3 has
//! two worlds that ought to be one identity. Nothing coordinates that: a `Thesis` is identified by
//! its parent, its cut and its selection, and the arrangement's job is only to make those equal.
//!
//! **Two ways to make them stop being equal, and they are not the same way.**
//!
//! ```text
//! Settling::LeftOnly    an Event on one side only. A cut is (known_at, event_head) and the head
//!                       is RESOLVED against the Event chain that stood — so the same instant
//!                       addresses a different cut, and the world's identity moves
//!
//! Unselected::LeftOnly  a Commitment on one side only, which no world selects and no cut can
//!                       name. Nothing about either world's identity moves, and the journals
//!                       differ anyway — which is C4
//! ```
//!
//! The two are held as separate enums rather than as two booleans, because a pair of adjacent
//! booleans is exactly the argument two callers swap without the compiler noticing.
//!
//! # The instrument is which repository is read as a working copy
//!
//! There is no operation whose subject is two repositories. What there is is `converge`, which takes
//! a repository and a working copy — and a repository read back **is** a working copy. So a meeting
//! is expressed by handing one repository's reading to the other, and the asymmetry that produces is
//! not an accident of the arrangement: it is what the application can say.
//!
//! Every quantity is an integer, for the reason [`super::reconstruction`] gives.

use ape::canon::Canon;
use ape::engine::thesis::{Interpretation, Thesis, ThesisId};
use ape::kernel::entities::{CommitmentId, ResourceInstanceId};
use ape::kernel::value_objects::Date;

use ape_cli::error::{JournalError, ReadingError, RepositoryError, SubjectError};
use ape_cli::history::ResidentHistory;
use ape_cli::journal::{
    self, ActionKindRecord, Admission, EffectRecord, EntryId, Replayed, ResourceKindRecord,
};
use ape_cli::level;
use ape_cli::lineage::{self, Decision, Lineage, Taken};
use ape_cli::reading::{Corroborated, WorldRecord};
use ape_cli::repository::{Repository, RepositoryInput};

pub const FULFILLING: &str = "Settled";
pub const CANCELLING: &str = "Void";

/// The bounds the account answers to.
pub const FLOOR: i128 = 0;
pub const CEILING: i128 = 1000;

/// What the fund puts in the account, and the whole of the shared base.
pub const FUNDED: u128 = 300;

/// What each side plans against it: the left's, then the right's.
pub const PLANS: [u128; 2] = [40, 70];

/// An inflow admitted on one side, which no world selects and no cut can name.
pub const UNSELECTED: u128 = 25;

/// What each world intends the account to hold: the base, then the left's, then the right's.
pub const INTENDED: [i128; 3] = [300, 260, 230];

/// The entries the shared base holds, and what each side's journal comes to.
pub const BASE_ENTRIES: usize = 13;
pub const SIDE_ENTRIES: usize = 14;

/// The worlds each side decides: the base, and its own.
pub const SIDE_WORLDS: usize = 2;

/// The literals above, weighed against each other before anything runs.
///
/// Arithmetic rather than measurement. The casts are where the sign enters — a magnitude is unsigned
/// because the direction is the statement's, and a level is a sum of both directions.
const _: () = assert!(FLOOR < FUNDED as i128 && (FUNDED as i128) < CEILING);
const _: () = assert!(INTENDED[0] == FUNDED as i128);
const _: () = assert!(INTENDED[1] == INTENDED[0] - PLANS[0] as i128);
const _: () = assert!(INTENDED[2] == INTENDED[0] - PLANS[1] as i128);
const _: () = assert!(INTENDED[1] != INTENDED[2]);
const _: () = assert!(PLANS[0] != PLANS[1] && PLANS[0] != UNSELECTED && PLANS[1] != UNSELECTED);
// A world holding both plans would be feasible, so nothing here is lost for being infeasible.
const _: () = assert!(FLOOR < INTENDED[0] - PLANS[0] as i128 - PLANS[1] as i128);
const _: () = assert!(SIDE_ENTRIES == BASE_ENTRIES + 1);

/// How the two journals stand to one another.
///
/// Three, because a meeting has three shapes and the protocol claims something about all of them. The
/// names are about the *journals* rather than about the lineages: what `converge` compares is
/// knowledge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
    /// Two independent foundings. Nothing in either journal is in the other, from the first entry on.
    Disjoint,
    /// One base, admitted identically by both, and one plan each. Neither extends the other.
    Shared,
    /// The right journal is the left journal, extended by the right's own plan.
    Extending,
}

/// Where the Event that settles the fund is admitted.
///
/// The thing a cut resolves against, held as its own type so it cannot be confused with
/// [`Unselected`] at a call site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Settling {
    Neither,
    LeftOnly,
}

/// Where a Commitment no world selects is admitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unselected {
    Neither,
    LeftOnly,
}

/// What to found, by name.
///
/// A parameter object because the three fields are what an arrangement *is*, and because two of them
/// would otherwise be adjacent booleans — the call that swaps them would compile.
pub struct Founding {
    pub relation: Relation,
    pub settling: Settling,
    pub unselected: Unselected,
}

impl Founding {
    /// The plain case: one base, one plan each, nothing else.
    pub fn shared() -> Self {
        Self {
            relation: Relation::Shared,
            settling: Settling::Neither,
            unselected: Unselected::Neither,
        }
    }

    pub fn of(relation: Relation) -> Self {
        Self {
            relation,
            ..Self::shared()
        }
    }
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
    pub fund: CommitmentId,
    /// The worlds this side decided, by identity, in the order decided.
    ///
    /// Carried because C3 and C4 are comparisons **between** identities, and a phase that compared
    /// readings instead would report two worlds that are the same as two worlds that agree.
    pub worlds: Vec<ThesisId>,
}

/// Two whole repositories, and nothing between them.
pub struct Arranged {
    pub left: Side,
    pub right: Side,
}

/// Found both repositories, independently, from one statement of the subject.
pub fn arranged(founding: Founding) -> Result<Arranged, SubjectError> {
    let (left_mark, right_mark) = match founding.relation {
        // Different vocabularies, beginning at the first admission, so the journals differ at 0.
        Relation::Disjoint => ("north", "south"),
        Relation::Shared | Relation::Extending => ("north", "north"),
    };

    let left = side(
        left_mark,
        &[],
        PLANS[0],
        founding.settling,
        founding.unselected,
    )?;

    // Extending is the one relation where the right side's journal is not its own: it holds the
    // left's plan first, so that the left journal is a prefix of the right's.
    let carried: Vec<u128> = match founding.relation {
        Relation::Extending => vec![PLANS[0]],
        Relation::Disjoint | Relation::Shared => vec![],
    };

    let right = side(
        right_mark,
        &carried,
        PLANS[1],
        Settling::Neither,
        Unselected::Neither,
    )?;

    Ok(Arranged { left, right })
}

/// One repository: the base, whatever it carries, its own plan, and the two decisions it takes.
///
/// `carried` is knowledge this side admits and decides nothing about — which is how [`Relation::Extending`]
/// is built, and is the ordinary state of a journal besides.
fn side(
    mark: &str,
    carried: &[u128],
    plan: u128,
    settling: Settling,
    unselected: Unselected,
) -> Result<Side, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let mut constructed = construct(&mut canon, mark)?;

    if settling == Settling::LeftOnly {
        constructed.settle(&mut canon)?;
    }

    if unselected == Unselected::LeftOnly {
        constructed.admit(&mut canon, UNSELECTED, EffectRecord::Increase)?;
    }

    for magnitude in carried {
        constructed.admit(&mut canon, *magnitude, EffectRecord::Decrease)?;
    }

    let mut lineage = Lineage::new();
    let founding = Taken::now(founding_decision(constructed.fund), &constructed.admitted)?;

    lineage::decide(canon.history(), &mut lineage, &founding.decision)?;

    constructed.admit(&mut canon, plan, EffectRecord::Decrease)?;

    let introduced = *constructed
        .admitted
        .commitments
        .last()
        .expect("a side's plan is a commitment");
    let base = lineage
        .decided()
        .first()
        .ok_or(SubjectError::NothingDecided)?
        .id();
    let planning = Taken::now(also(base, introduced), &constructed.admitted)?;

    lineage::decide(canon.history(), &mut lineage, &planning.decision)?;

    Ok(Side {
        files: Files {
            journal: constructed.journal,
            lineage: vec![founding, planning],
            worlds: worlds(&lineage),
        },
        instance: constructed.instance,
        fund: constructed.fund,
        worlds: lineage.decided().iter().map(Thesis::id).collect(),
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
    }
}

/// The addresses one journal produces, derived rather than carried.
///
/// This is what C2 is measured on: an [`EntryId`] comes from what admitting produced, so two journals
/// that said the same thing hold the same addresses whether or not anything ever copied one.
pub fn entries(files: &Files) -> Result<Vec<EntryId>, JournalError> {
    let mut canon = Canon::new(ResidentHistory::new());

    Ok(journal::replay(&mut canon, &files.journal)?.entries)
}

/// The first world: the fund, and nothing proposed against it.
///
/// Both sides take this one, at the same instant, with no coordination — which is the whole of what
/// C3 needs, because a genesis is identified by its cut and its selection and nothing else.
pub fn founding_decision(fund: CommitmentId) -> Decision {
    Decision::Genesis {
        known_at: day(10),
        selection: [fund].into(),
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

/// A repository read back, which is exactly a working copy.
///
/// Named here rather than called inline because it is the experiment's central claim about what the
/// application can say: there is no operation whose subject is two repositories, and this is the
/// substitute — one of them stops being a repository and becomes a party.
pub fn as_party(repository: &Repository) -> Result<Corroborated, SubjectError> {
    Ok(ape_cli::reading::corroborated(repository)?)
}

/// The vocabulary, the fund, and whatever a side goes on to admit.
pub struct Constructed {
    pub fund: CommitmentId,
    pub instance: ResourceInstanceId,
    inflow: ape::kernel::entities::StatementId,
    outflow: ape::kernel::entities::StatementId,
    customer: ape::kernel::entities::AgentId,
    merchant: ape::kernel::entities::AgentId,
    pub journal: Vec<Admission>,
    pub admitted: Replayed,
}

impl Constructed {
    /// Admit one flow of `magnitude`, extending the journal.
    pub fn admit(
        &mut self,
        canon: &mut Canon<ResidentHistory>,
        magnitude: u128,
        effect: EffectRecord,
    ) -> Result<(), JournalError> {
        let (statement, executor, beneficiary) = match effect {
            EffectRecord::Increase => (self.inflow, self.customer, self.merchant),
            EffectRecord::Decrease => (self.outflow, self.merchant, self.customer),
        };

        self.journal.push(Admission::Commitment {
            accountable: executor,
            executors: [executor].into(),
            beneficiaries: [beneficiary].into(),
            statement,
            resource: self.instance,
            committed_at: day(4),
            due_date: day(15),
            magnitude: Some(magnitude),
            dependencies: [].into(),
            recorded_at: day(4),
        });

        journal::replay_remaining(canon, &self.journal, &mut self.admitted)
    }

    /// Admit the Event that settles the fund, which is what a cut resolves against.
    pub fn settle(&mut self, canon: &mut Canon<ResidentHistory>) -> Result<(), JournalError> {
        self.journal.push(Admission::Event {
            commitment: self.fund,
            observation: FULFILLING.into(),
            occurred_at: day(3),
            recorded_at: day(3),
        });

        journal::replay_remaining(canon, &self.journal, &mut self.admitted)
    }
}

/// Admit the base, accumulating the journal that describes it.
///
/// `mark` goes into the **first** label and every one after it, so two constructions under different
/// marks produce journals that differ at position 0 and share nothing at all.
pub fn construct(
    canon: &mut Canon<ResidentHistory>,
    mark: &str,
) -> Result<Constructed, JournalError> {
    let mut journal = Vec::new();
    let mut admitted = Replayed::default();
    let named = |what: &str| format!("{what}-{mark}");

    journal.extend([
        Admission::Role {
            label: named("payer"),
            recorded_at: day(1),
        },
        Admission::Role {
            label: named("payee"),
            recorded_at: day(1),
        },
        Admission::Agent {
            label: named("customer"),
            recorded_at: day(1),
        },
        Admission::Agent {
            label: named("merchant"),
            recorded_at: day(1),
        },
        Admission::Resource {
            label: named("cash"),
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
            label: named("account"),
            resource: cash,
            recorded_at: day(1),
        },
        Admission::Action {
            verb: named("receive"),
            kind: ActionKindRecord::Quantifiable(EffectRecord::Increase),
            resource: cash,
            recorded_at: day(1),
        },
        Admission::Action {
            verb: named("spend"),
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
        due_date: day(15),
        magnitude: Some(FUNDED),
        dependencies: [].into(),
        recorded_at: day(2),
    });
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    Ok(Constructed {
        fund: admitted.commitments[0],
        instance,
        inflow,
        outflow,
        customer,
        merchant,
        journal,
        admitted,
    })
}

fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
