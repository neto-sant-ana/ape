//! The atomicity subject: a repository worth losing, and one commit that does not finish.
//!
//! ```text
//! cash ∈ [0, 1000]
//!
//! P  receive 100    recorded day 2, fulfilled by an Event on day 3     the pledge
//! S₁ spend 30       recorded day 4, proposed by the second decision
//! S₂ spend 40       recorded day 5, admitted and decided about by nobody
//! S₃ spend 20       recorded day 6, admitted by the commit that is interrupted
//! ```
//!
//! ```text
//!  before        { P }  ──▸  { P, S₁ }                      two decisions, 16 entries
//!  after         { P }  ──▸  { P, S₁ }  ──▸  { P, S₁, S₂, S₃ }   three, 17 entries
//! ```
//!
//! Two whole repositories, and the experiment is about the states between them. `before` is what a
//! reader can name and what an interruption may cost; `after` is the commit the writer intended. A
//! partial state is `before` with some of `after`'s three files written over it, which is what an
//! interrupted [`write`] leaves.
//!
//! # What the arrangement has to hold, and why each part of it
//!
//! **An unwitnessed tail.** `S₂` is admitted in `before` and no decision there says it was known.
//! Without it, the journal `before` held would be exactly the prefix the last surviving decision
//! witnesses, and every question about recovering a journal would answer itself. With it, the
//! previous journal's *end* is not in the record — which is the question Phase 3 asks.
//!
//! **More than one decision, and worlds that differ.** A truncated lineage has to be a different
//! length rather than an empty one, and the world a truncation loses has to be nameable. The three
//! worlds intend different levels — [`INTENDED`] — so *what was lost* is a number rather than a
//! count.
//!
//! **A commit that both admits and decides.** `after` adds one entry and one decision. An
//! interruption that only lost knowledge, or only lost intention, would measure one file's behaviour
//! and be read as the repository's.
//!
//! # The instrument is writing, and it is indistinguishable from the other two
//!
//! Interruption is performed by writing the files an application would write and stopping — the same
//! mechanism corroboration used to tamper and exploration used to prune. Nothing here simulates a
//! signal, and no measurement needs one: a prefix of a write sequence is a value.
//!
//! Every quantity is an integer, for the reason [`super::reconstruction`] gives.

use ape::canon::Canon;
use ape::engine::hermeneia::Hypothesis;
use ape::engine::thesis::{Interpretation, Thesis, ThesisId};
use ape::kernel::entities::{AgentId, CommitmentId, ResourceInstanceId, StatementId};
use ape::kernel::value_objects::Date;

use ape_cli::error::{JournalError, ReadingError, RepositoryError, SubjectError};
use ape_cli::history::ResidentHistory;
use ape_cli::journal::{
    self, ActionKindRecord, Admission, EffectRecord, EntryId, Replayed, ResourceKindRecord,
};
use ape_cli::level;
use ape_cli::lineage::{self, Decision, Lineage, Taken};
use ape_cli::reading::WorldRecord;
use ape_cli::repository::Repository;

pub const FULFILLING: &str = "Settled";
pub const CANCELLING: &str = "Void";

/// The bounds the account answers to.
pub const FLOOR: f64 = 0.0;
pub const CEILING: f64 = 1000.0;

/// What the pledge puts in the account, and settles.
pub const PLEDGED: f64 = 100.0;

/// The three outflows, in the order they are admitted.
///
/// `S₁` is proposed by `before`'s second decision; `S₂` is admitted there and proposed by nobody;
/// `S₃` is admitted by the commit that is interrupted. Distinct magnitudes, so two of them are never
/// one commitment by identity.
pub const SPENDS: [f64; 3] = [30.0, 40.0, 20.0];

/// What each world intends the account to hold, oldest first, written before the run.
///
/// The third is what an interruption can cost, and it is a number rather than a count. Every one of
/// them is inside the bounds, so no world here is refused for a reason this experiment is not about.
pub const INTENDED: [f64; 3] = [100.0, 70.0, 10.0];

/// The entries each whole repository's journal holds.
pub const BEFORE_ENTRIES: usize = 16;
pub const AFTER_ENTRIES: usize = 17;

/// The decisions each whole repository's lineage holds, and the worlds it recorded.
pub const BEFORE_WORLDS: usize = 2;
pub const AFTER_WORLDS: usize = 3;

/// Entries in `before`'s journal that no decision there says were known.
pub const UNWITNESSED: usize = 1;

/// The assumption a level is read under, named once.
pub const HYPOTHESIS: Hypothesis = Hypothesis::FinalState;

/// The literals above, weighed against each other before anything runs.
///
/// None of it is a measurement: that the pledge fits strictly inside the bounds, and that the three
/// intended levels are what the three spends leave, is arithmetic written on one afternoon. Asserted
/// inside a phase it would have read as a result.
const _: () = assert!(FLOOR < PLEDGED && PLEDGED < CEILING);
const _: () = assert!(INTENDED[0] == PLEDGED);
const _: () = assert!(INTENDED[1] == INTENDED[0] - SPENDS[0]);
const _: () = assert!(INTENDED[2] == INTENDED[1] - SPENDS[1] - SPENDS[2]);
const _: () = assert!(FLOOR < INTENDED[2]);
const _: () = assert!(AFTER_ENTRIES == BEFORE_ENTRIES + 1);
const _: () = assert!(AFTER_WORLDS == BEFORE_WORLDS + 1);

/// The three files an application writes, held as values rather than as a directory.
///
/// They are values because the instrument needs them to be: a partial state is written by putting
/// some of one repository's files over another's, and a phase that had to reach into a live
/// application to get at them could not produce a prefix without also producing the process that
/// stopped.
pub struct Files {
    pub journal: Vec<Admission>,
    pub lineage: Vec<Taken>,
    pub worlds: Vec<WorldRecord>,
}

/// Which of the three a write had reached.
///
/// Ordered as the [`Repository`] writes them, and named rather than indexed, because Phase 4's whole
/// question is what happens under a different order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum File {
    Journal,
    Lineage,
    Worlds,
}

impl File {
    pub const ALL: [Self; 3] = [Self::Journal, Self::Lineage, Self::Worlds];

    /// Every order the three files could be written in.
    ///
    /// Six, enumerated rather than sampled, so that what a phase reports about *an* order is
    /// reported about all of them. The application's own order is [`ORDER`].
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

/// The order the application writes in, and the one every prefix before Phase 4 is taken of.
pub const ORDER: [File; 3] = [File::Journal, File::Lineage, File::Worlds];

/// Write one of the three files, and nothing else.
///
/// This is the whole instrument. It calls the same three methods the application calls, in whatever
/// order it is handed, and stopping is not doing the next call.
pub fn put(repository: &Repository, files: &Files, file: File) -> Result<(), RepositoryError> {
    match file {
        File::Journal => repository.write_journal(&files.journal),
        File::Lineage => repository.write_lineage(&files.lineage),
        File::Worlds => repository.write_worlds(&files.worlds),
    }
}

/// Write all three, in the given order.
pub fn write(
    repository: &Repository,
    files: &Files,
    order: [File; 3],
) -> Result<(), RepositoryError> {
    for file in order {
        put(repository, files, file)?;
    }

    Ok(())
}

/// A whole repository, and then a commit over it that stops after `reached` files.
///
/// `reached` is 0 for a write that never began — which the protocol excludes as a case, and which
/// this produces anyway, because a phase that cannot ask for it cannot show that it is `before`.
pub fn interrupted(
    repository: &Repository,
    arrangement: &Arranged,
    order: [File; 3],
    reached: usize,
) -> Result<(), RepositoryError> {
    write(repository, &arrangement.before, ORDER)?;

    for file in &order[..reached] {
        put(repository, &arrangement.after, *file)?;
    }

    Ok(())
}

/// What the procedure refers to across phases.
pub struct Arranged {
    /// The repository that existed before the write: what an interruption may cost.
    pub before: Files,
    /// The commit the writer intended: what an interruption may lose.
    pub after: Files,
    pub instance: ResourceInstanceId,
    /// The pledge and the three outflows, in the order they were admitted.
    pub pledge: CommitmentId,
    pub spends: [CommitmentId; 3],
}

impl Arranged {
    /// The address the interrupted commit's decision was taken after.
    ///
    /// Named because it is what a refusal is expected to name, and a phase that matched on the
    /// refusal without checking *which* entry it named would accept a repository refused for
    /// another reason.
    pub fn appended(&self) -> EntryId {
        self.after
            .lineage
            .last()
            .expect("the interrupted commit takes a decision")
            .after
            .clone()
    }

    /// The entries `before` holds that no decision there says were known.
    ///
    /// Derived from the two files rather than listed, so that the arrangement cannot claim a tail it
    /// does not have.
    pub fn unwitnessed(&self) -> Vec<EntryId> {
        let witnessed: std::collections::BTreeSet<&EntryId> = self
            .before
            .lineage
            .iter()
            .flat_map(|taken| taken.witness.iter())
            .collect();

        let mut canon = Canon::new(ResidentHistory::new());
        let replayed =
            journal::replay(&mut canon, &self.before.journal).expect("the arrangement admits");

        replayed
            .entries
            .into_iter()
            .filter(|entry| !witnessed.contains(entry))
            .collect()
    }
}

/// Admit the subject and take `before`'s decisions, then the interrupted commit's.
///
/// The two repositories come from one construction because they have to: `after` is `before`
/// extended, and an arrangement that built them separately could differ in something no phase asked
/// about.
pub fn arranged() -> Result<Arranged, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let mut constructed = construct(&mut canon)?;
    let mut lineage = Lineage::new();
    let mut decisions = Vec::new();

    let pledged = Taken::now(pledging(constructed.pledge), &constructed.admitted)?;
    lineage::decide(canon.history(), &mut lineage, &pledged.decision)?;
    decisions.push(pledged);

    let mut spends = Vec::new();

    for magnitude in SPENDS.iter().take(2) {
        constructed.admit(&mut canon, *magnitude)?;
        spends.push(*constructed.admitted.commitments.last().expect("admitted"));

        // Only the first is decided about. The second is the unwitnessed tail, and it is admitted
        // here rather than by the interrupted commit so that `before` is a repository that already
        // knows something nothing has decided about — which is the ordinary state of a journal,
        // and the state that makes recovering one a question.
        if spends.len() == 1 {
            let extended = tip(&lineage)?;
            let proposing = Taken::now(
                spending(extended, [*spends.last().expect("admitted")].into()),
                &constructed.admitted,
            )?;

            lineage::decide(canon.history(), &mut lineage, &proposing.decision)?;
            decisions.push(proposing);
        }
    }

    let before = Files {
        journal: constructed.journal.clone(),
        lineage: decisions.clone(),
        worlds: worlds(&lineage),
    };

    constructed.admit(&mut canon, SPENDS[2])?;
    spends.push(*constructed.admitted.commitments.last().expect("admitted"));

    let extended = tip(&lineage)?;
    let proposing = Taken::now(
        spending(extended, [spends[1], spends[2]].into()),
        &constructed.admitted,
    )?;

    lineage::decide(canon.history(), &mut lineage, &proposing.decision)?;
    decisions.push(proposing);

    let after = Files {
        journal: constructed.journal.clone(),
        lineage: decisions,
        worlds: worlds(&lineage),
    };

    Ok(Arranged {
        before,
        after,
        instance: constructed.instance,
        pledge: constructed.pledge,
        spends: [spends[0], spends[1], spends[2]],
    })
}

/// The world the last decision produced, which the next one extends.
fn tip(lineage: &Lineage) -> Result<ThesisId, SubjectError> {
    Ok(lineage
        .decided()
        .last()
        .ok_or(SubjectError::NothingDecided)?
        .id())
}

/// The first world: the pledge, and nothing proposed about spending it.
///
/// Its cut recognizes the Event that settled the pledge, so the pledge arrives frozen and no later
/// fork can omit it.
pub fn pledging(pledge: CommitmentId) -> Decision {
    Decision::Genesis {
        known_at: day(10),
        selection: [pledge].into(),
    }
}

/// A fork that withdraws nothing and proposes outflows.
pub fn spending(
    extends: ThesisId,
    introduced: std::collections::BTreeSet<CommitmentId>,
) -> Decision {
    Decision::Fork {
        extends,
        omitted: [].into(),
        introduced,
    }
}

/// The instant every reading is taken at, after every due date the subject holds.
pub fn asked_at() -> Date {
    Date::from_ymd(2026, 1, 20).expect("the instant every reading is taken at is a date")
}

/// What one world intends the account to hold.
///
/// The intended level rather than the settled one, because what an interruption loses is an
/// *intention*: three worlds that have settled the same pledge and proposed different outflows all
/// report the same settled level, and a phase comparing that would report the lost world as
/// identical to the one before it.
pub fn intended(
    history: &ResidentHistory,
    thesis: &Thesis,
    instance: ResourceInstanceId,
) -> Result<f64, ReadingError> {
    let interpretation = Interpretation::of(thesis, history)?;
    let projected = interpretation.conditions_at(&asked_at())?;

    Ok(level::intended(history, &projected, instance)?)
}

/// The witnesses for every world a lineage produced.
pub fn worlds(lineage: &Lineage) -> Vec<WorldRecord> {
    lineage.decided().iter().map(WorldRecord::of).collect()
}

/// The vocabulary, the pledge, and the Event that settles it.
pub struct Constructed {
    pub pledge: CommitmentId,
    pub instance: ResourceInstanceId,
    outflow: StatementId,
    merchant: AgentId,
    customer: AgentId,
    pub journal: Vec<Admission>,
    pub admitted: Replayed,
}

impl Constructed {
    /// Admit one outflow of `magnitude`, extending the journal.
    ///
    /// Every field but the magnitude is fixed, so two outflows of equal magnitude would be one
    /// commitment by identity — which is why [`SPENDS`] holds three distinct numbers.
    pub fn admit(
        &mut self,
        canon: &mut Canon<ResidentHistory>,
        magnitude: f64,
    ) -> Result<(), JournalError> {
        let recorded_on = 3 + self.admitted.commitments.len() as u8;

        self.journal.push(Admission::Commitment {
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
        });

        journal::replay_remaining(canon, &self.journal, &mut self.admitted)
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
        magnitude: Some(PLEDGED),
        dependencies: [].into(),
        recorded_at: day(2),
    });
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let pledge = admitted.commitments[0];

    journal.push(Admission::Event {
        commitment: pledge,
        observation: FULFILLING.into(),
        occurred_at: day(3),
        recorded_at: day(3),
    });
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    Ok(Constructed {
        pledge,
        instance,
        outflow,
        merchant,
        customer,
        journal,
        admitted,
    })
}

fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
