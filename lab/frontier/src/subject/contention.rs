//! The contention subject: one base, read twice, and two commits worth telling apart.
//!
//! ```text
//! cash ∈ [0, 1000]
//!
//! F  receive 200   recorded day 2                     the fund, and the whole of the base
//! H  spend    50   recorded day 4, admitted by the recruiter
//! P  spend    70   recorded day 4, admitted by the buyer
//! ```
//!
//! ```text
//!                    base { F }                200        15 entries, 1 world
//!                ┌───────┴───────┐
//!    recruiting { F, H }   purchasing { F, P }  150 / 130  16 entries, 2 worlds each
//! ```
//!
//! The base is a repository both parties read. Each then admits one commitment and takes one
//! decision claiming itself, so each party's whole state is `base + 1 entry + 1 world` — and the
//! two are told apart by the level their world intends rather than by how many there are.
//!
//! # What the arrangement has to hold, and why each part of it
//!
//! **One base, read twice.** Both parties begin from the same reading, which is the condition the
//! whole question is about. [`found`] puts it there the way an application does, and [`read`] is what
//! a party holds afterwards.
//!
//! **Each party appends knowledge *and* decides.** A party that only decided would leave the two
//! journals identical, and the guard the coordination experiment built compares journals — so an
//! arrangement without new knowledge on both sides would measure a merge where the question is about
//! a collision. [`INTENDED`] is what tells the two lines apart, and it is a number rather than a
//! count.
//!
//! **Each legitimate alone.** Either party's state, applied to the base by itself, reconstructs and
//! corroborates: 200 − 50 and 200 − 70 are both inside the bounds, and so is 200 − 50 − 70. What is
//! lost has to be lost for being overwritten rather than for being wrong.
//!
//! **Both parties are in the base.** They are admitted as agents before either reads, so a decision
//! can claim one — and so that Phase 5's question has a sharp answer: after a loss, the record still
//! says the party *exists* and says nothing about whether it ever decided.
//!
//! # The instrument is the order of the calls
//!
//! Two handles on one directory, in one process, and the phase decides the sequence. [`write`] is
//! what an application does — one call, all or nothing. [`prepare`] and [`Prepared::turn`] are the
//! same write with its seam exposed, which is what lets an ordering be a value; and [`put`] writes
//! one file, which is the finer grain the atomicity experiment used and reported as the door left
//! open.
//!
//! Nothing here needs a thread. Every quantity is an integer, for the reason
//! [`super::reconstruction`] gives.

use ape::canon::Canon;
use ape::engine::thesis::{Interpretation, Thesis, ThesisId};
use ape::kernel::entities::{AgentId, CommitmentId, ResourceInstanceId};
use ape::kernel::value_objects::Date;

use ape_cli::error::{JournalError, ReadingError, RepositoryError, SubjectError};
use ape_cli::history::ResidentHistory;
use ape_cli::journal::{
    self, ActionKindRecord, Admission, EffectRecord, Replayed, ResourceKindRecord,
};
use ape_cli::level;
use ape_cli::lineage::{self, Decision, Lineage, Taken};
use ape_cli::reading::{self, Corroborated, WorldRecord};
use ape_cli::repository::{Prepared, Repository, RepositoryInput};

pub const FULFILLING: &str = "Settled";
pub const CANCELLING: &str = "Void";

/// The bounds the account answers to.
pub const FLOOR: i128 = 0;
pub const CEILING: i128 = 1000;

/// What the base puts in the account.
pub const FUNDED: u128 = 200;

/// What each party plans against it, in the order the parties are held.
///
/// Distinct magnitudes, because every other field of the two admissions is equal — two plans of the
/// same size would be one commitment by identity, and the two parties would be admitting the same
/// knowledge instead of colliding.
pub const PLANNED: [u128; 2] = [50, 70];

/// What each world intends the account to hold: the base, then each party's, written before the run.
///
/// This is how a phase says *whose* state survived. Three worlds and three different numbers, so an
/// outcome is named rather than counted.
pub const INTENDED: [i128; 3] = [200, 150, 130];

/// The entries and worlds each whole state holds.
///
/// `merged` is what the two parties' lines come to when neither is lost — both admissions and all
/// three worlds — and it is the value a phase compares a converged repository against.
pub const BASE_ENTRIES: usize = 15;
pub const PARTY_ENTRIES: usize = 16;
pub const MERGED_ENTRIES: usize = 17;
pub const BASE_WORLDS: usize = 1;
pub const PARTY_WORLDS: usize = 2;
pub const MERGED_WORLDS: usize = 3;

/// The literals above, weighed against each other before anything runs.
///
/// Arithmetic rather than measurement, for the reason the atomicity subject gives: asserted inside a
/// phase, this would read as a result. The casts are where the sign enters — a magnitude is unsigned
/// because the direction is the statement's, and a level is a sum of both directions.
const _: () = assert!(FLOOR < FUNDED as i128 && (FUNDED as i128) < CEILING);
const _: () = assert!(INTENDED[0] == FUNDED as i128);
const _: () = assert!(INTENDED[1] == INTENDED[0] - PLANNED[0] as i128);
const _: () = assert!(INTENDED[2] == INTENDED[0] - PLANNED[1] as i128);
const _: () = assert!(INTENDED[1] != INTENDED[2]);
const _: () = assert!(PLANNED[0] != PLANNED[1]);
// A world holding both plans would be feasible too, so nothing here is lost for being infeasible.
const _: () = assert!(FLOOR < INTENDED[0] - PLANNED[0] as i128 - PLANNED[1] as i128);
const _: () = assert!(PARTY_ENTRIES == BASE_ENTRIES + 1);
const _: () = assert!(MERGED_ENTRIES == BASE_ENTRIES + 2);
const _: () = assert!(PARTY_WORLDS == BASE_WORLDS + 1);
const _: () = assert!(MERGED_WORLDS == BASE_WORLDS + 2);

/// The three files a writer holds, as values rather than as a directory.
///
/// Values because the instrument needs them to be: a phase puts one writer's file over another's,
/// and one that had to reach into a live application to get at them could not express an ordering.
pub struct Files {
    pub journal: Vec<Admission>,
    pub lineage: Vec<Taken>,
    pub worlds: Vec<WorldRecord>,
}

/// One of the three files a repository is made of.
///
/// Restated here rather than borrowed from the atomicity subject. A subject belongs to the
/// experiment that arranged it, and what is being mixed is different: there, two states of one
/// writer; here, two writers. The lattice is the same shape and the contents are not, which is
/// exactly what Phase 4 is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum File {
    Journal,
    Lineage,
    Worlds,
}

impl File {
    pub const ALL: [Self; 3] = [Self::Journal, Self::Lineage, Self::Worlds];

    /// Every order the three could be written in — six, enumerated rather than sampled.
    pub fn orders() -> Vec<[Self; 3]> {
        let mut orders = Vec::new();

        for first in Self::ALL {
            for second in Self::ALL {
                for third in Self::ALL {
                    if first != second && second != third && first != third {
                        orders.push([first, second, third]);
                    }
                }
            }
        }

        orders
    }
}

/// The order the application writes in.
pub const ORDER: [File; 3] = [File::Journal, File::Lineage, File::Worlds];

/// A party: an agent the base knows, the knowledge it is about to admit, and what it will intend.
///
/// The plan is held **unadmitted**, so that admitting it is the party's own act and the two
/// journals diverge at the entry each party added.
pub struct Party {
    pub label: &'static str,
    pub agent: AgentId,
    pub plan: Admission,
    /// What this party's world intends the account to hold, from [`INTENDED`].
    pub intends: i128,
}

/// What the procedure refers to across phases.
pub struct Arranged {
    /// The repository both parties read, whole.
    pub base: Files,
    /// The two writers, in the order [`PLANNED`] and [`INTENDED`] name them.
    pub parties: [Party; 2],
    pub instance: ResourceInstanceId,
    pub fund: CommitmentId,
    /// The world the base decided, which both parties fork.
    ///
    /// Carried so that a phase can check the arrangement's own claim against what a reader rebuilds
    /// — a world is content-addressed, so the two must agree or the arrangement is describing a
    /// repository it did not write.
    pub world: ThesisId,
}

/// Admit the subject, decide the base's one world, and hold both parties unadmitted.
pub fn arranged() -> Result<Arranged, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let constructed = construct(&mut canon)?;

    let mut lineage = Lineage::new();
    // Claiming nobody, because nobody in particular wrote the base. Which leaves the arrangement
    // holding a repository where one decision names no party and the parties' decisions name
    // themselves — the shape Phase 5 measures rather than tidies away.
    let taken = Taken::now(founding(constructed.fund), &constructed.admitted)?;

    lineage::decide(canon.history(), &mut lineage, &taken.decision)?;

    let world = lineage
        .decided()
        .first()
        .ok_or(SubjectError::NothingDecided)?
        .id();

    Ok(Arranged {
        base: Files {
            journal: constructed.journal,
            lineage: vec![taken],
            worlds: worlds(&lineage),
        },
        parties: [
            Party {
                label: "recruiter",
                agent: constructed.recruiter,
                plan: constructed.hire,
                intends: INTENDED[1],
            },
            Party {
                label: "buyer",
                agent: constructed.buyer,
                plan: constructed.purchase,
                intends: INTENDED[2],
            },
        ],
        instance: constructed.instance,
        fund: constructed.fund,
        world,
    })
}

/// Put the base there the way an application does, so that parties have something to read.
pub fn found(repository: &Repository, arrangement: &Arranged) -> Result<(), RepositoryError> {
    write(repository, &arrangement.base)
}

/// What a party holds before it decides anything.
pub fn read(repository: &Repository) -> Result<Corroborated, SubjectError> {
    Ok(reading::corroborated(repository)?)
}

/// Admit this party's plan and decide about it, against what this party read and nobody else's.
///
/// One call because the two are one act here: what makes a party a *writer* in this experiment is
/// that it appends knowledge and intends something about it. A phase that could do one without the
/// other would be able to arrange a collision the question is not about.
pub fn plan(working: &mut Corroborated, party: &Party) -> Result<ThesisId, SubjectError> {
    let base = working
        .lineage
        .decided()
        .first()
        .ok_or(SubjectError::NothingDecided)?
        .id();

    admit(working, party.plan.clone())?;

    let introduced = *working
        .admitted
        .commitments
        .last()
        .expect("a party's plan is a commitment");

    decided(working, party.agent, also(base, introduced))
}

/// Admit knowledge against what this party read, extending its journal.
pub fn admit(working: &mut Corroborated, admission: Admission) -> Result<(), SubjectError> {
    working.journal.push(admission);

    journal::replay_remaining(&mut working.canon, &working.journal, &mut working.admitted)?;

    Ok(())
}

/// Take a decision claiming the party that took it.
pub fn decided(
    working: &mut Corroborated,
    by: AgentId,
    decision: Decision,
) -> Result<ThesisId, SubjectError> {
    let taken = Taken::claimed(decision, by, &working.admitted)?;

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

/// The three files a party holds, ready to be written.
pub fn files(working: &Corroborated) -> Files {
    Files {
        journal: working.journal.clone(),
        lineage: working.decisions.clone(),
        worlds: worlds(&working.lineage),
    }
}

/// The three files as the application's own write path takes them.
pub fn input(files: &Files) -> RepositoryInput<'_> {
    RepositoryInput {
        journal: &files.journal,
        lineage: &files.lineage,
        worlds: &files.worlds,
    }
}

/// Put a whole repository there the way an application does — one call, all or nothing.
pub fn write(repository: &Repository, files: &Files) -> Result<(), RepositoryError> {
    repository.write_whole(input(files))
}

/// The same write, with its seam exposed, so that an ordering can be a value.
pub fn prepare(repository: &Repository, files: &Files) -> Result<Prepared, RepositoryError> {
    repository.prepare(input(files))
}

/// Write one of the three files into whatever a repository reads, and nothing else.
///
/// The finer grain: not what the application does, and what five experiments need in order to
/// tamper with a repository, prune one, interrupt one — or, here, to put one writer's file over
/// another's inside a generation they both prepared.
pub fn put(repository: &Repository, files: &Files, file: File) -> Result<(), RepositoryError> {
    match file {
        File::Journal => repository.write_journal(&files.journal),
        File::Lineage => repository.write_lineage(&files.lineage),
        File::Worlds => repository.write_worlds(&files.worlds),
    }
}

/// The first world: the fund, and nothing proposed against it.
pub fn founding(fund: CommitmentId) -> Decision {
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
///
/// The intended level rather than the settled one, for the reason the atomicity subject gives: what
/// a lost writer loses is an *intention*, and every world here has settled the same nothing.
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

/// The vocabulary, the fund, the two parties, and the two plans nobody has admitted yet.
pub struct Constructed {
    pub fund: CommitmentId,
    pub instance: ResourceInstanceId,
    pub recruiter: AgentId,
    pub buyer: AgentId,
    /// `H` — held unadmitted, for the recruiter to admit on its own.
    pub hire: Admission,
    /// `P` — the same, for the buyer, so that the two admit different knowledge.
    pub purchase: Admission,
    pub journal: Vec<Admission>,
    pub admitted: Replayed,
}

/// Admit the base, accumulating the journal that describes it.
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
        // The two writers. A party is knowledge before it is a claim, so it arrives through the
        // journal like every other fact — and it arrives in the *base*, before either party reads,
        // which is what lets a decision claim one and what leaves the record able to say a party
        // exists after everything it decided is gone.
        Admission::Agent {
            label: "recruiter".into(),
            recorded_at: day(1),
        },
        Admission::Agent {
            label: "buyer".into(),
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
    let (recruiter, buyer) = (admitted.agents[2], admitted.agents[3]);
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
    // fixed by the statement rather than chosen here.
    let flowing = |statement, executor, beneficiary, magnitude: u128| Admission::Commitment {
        accountable: executor,
        executors: [executor].into(),
        beneficiaries: [beneficiary].into(),
        statement,
        resource: instance,
        committed_at: day(4),
        due_date: day(15),
        magnitude: Some(magnitude),
        dependencies: [].into(),
        recorded_at: day(4),
    };

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
        recruiter,
        buyer,
        hire: flowing(outflow, merchant, customer, PLANNED[0]),
        purchase: flowing(outflow, merchant, customer, PLANNED[1]),
        journal,
        admitted,
    })
}

fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
