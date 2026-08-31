//! The designation subject: several decided worlds, two parties, and a plan that moves.
//!
//! ```text
//! cash ∈ [0, 100]
//!
//! B  receive 50   recorded day 2   the budget, and the whole of the shared ancestor
//! H  spend   20   recorded day 3   one plan
//! E  spend   15   recorded day 3   another
//! L  spend   35   recorded day 3   a third
//! ```
//!
//! ```text
//!                     W₀ shared { B }                        50
//!            ┌──────────────┼──────────────┐
//!   W₁ { B, H }        W₂ { B, E }      W₃ { B, L }          30 / 35 / 15
//! ```
//!
//! All three plans fit inside the budget, and a world holding all three would too — 50 − 20 − 15 −
//! 35 is under the floor, so the three of them together would not, and no arrangement here builds
//! that world. Feasibility is not what this experiment is about and nothing in it must turn on one.
//!
//! # Why three forks rather than two
//!
//! The arrangement has to hold a plan that **moves back**, and with two worlds *moving back* and
//! *moving to the other one* are the same act. Three is the smallest number at which the return is a
//! distinguishable move.
//!
//! And the return is the load-bearing part rather than a flourish. A designation of `W₁` is derived
//! from `W₁`, so designating it twice produces the **same value twice** — which means a *set* of
//! designations cannot tell `W₁ → W₂ → W₁` from `W₁ → W₂`, and neither can a set of anything else a
//! record holds. Whatever answers *what was the plan on the twelfth* has to be ordered. That is a
//! property of the arrangement, put here so that the phase which asks it is not left arguing for it.
//!
//! ```text
//! DESIGNATED   W₁      W₂      W₁          three moves over two distinct values
//! ```
//!
//! # What it extends, and what it adds
//!
//! [`super::coordination`]'s arrangement, which is the row's shape for two parties that read one
//! repository and put back: one budget, one shared ancestor, and a party that decides while being a
//! party to no commitment. Three things differ, and each is here because a phase needs it.
//!
//! **A third plan**, for the reason above.
//!
//! **Both parties admitted before anything decides.** Coordination holds them back so that a
//! decision taken before its party exists can be attributed and refused. That trap belongs to
//! attribution, which is `17-imputation`'s, and an arrangement that reproduced it here would make
//! every phase carry a refusal it is not measuring.
//!
//! **A genesis that names nobody.** Both parties then fork under it, so the record holds decisions
//! with a party and one without — which is what a repository actually looks like and what a
//! per-party reading has to survive.
//!
//! No commitment depends on another. Every quantity is an integer, for the reason
//! [`super::reconstruction`] gives.

use ape::canon::Canon;
use ape::engine::thesis::{Thesis, ThesisId};
use ape::kernel::entities::{AgentId, CommitmentId, ResourceInstanceId};

use ape_cli::error::{JournalError, SubjectError};
use ape_cli::history::ResidentHistory;
use ape_cli::journal::{
    self, ActionKindRecord, Admission, EffectRecord, Replayed, ResourceKindRecord,
};
use ape_cli::lineage::{self, Decision, Lineage, Taken};
use ape_cli::reading::{self, Corroborated, WorldRecord};
use ape_cli::repository::{Repository, RepositoryInput};

pub const FULFILLING: &str = "Settled";
pub const CANCELLING: &str = "Void";

/// The bounds the account answers to.
pub const FLOOR: i128 = 0;
pub const CEILING: i128 = 100;

/// What the budget puts in the account.
pub const BUDGET: u128 = 50;

/// The three plans, in the order they are admitted and forked.
pub const PLANS: [u128; FORKS] = [20, 15, 35];

/// How many worlds fork off the shared ancestor.
pub const FORKS: usize = 3;

/// Every world the record holds: the ancestor and its forks.
pub const WORLDS: usize = 1 + FORKS;

/// The entries the founded journal holds.
pub const ENTRIES: usize = 18;

/// Which fork the plan names, in the order it names them.
///
/// Two distinct worlds over three moves, and the third is the first again. See the module docstring
/// — the repetition is what makes an ordered answer distinguishable from an unordered one.
pub const DESIGNATED: [usize; MOVES] = [0, 1, 0];

pub const MOVES: usize = 3;

/// How many *distinct* worlds the plan ever names, written down rather than derived.
///
/// A phase that computed this from [`DESIGNATED`] would be checking that `dedup` works. What it is
/// for is the comparison against [`MOVES`]: a record holding two things where three happened is the
/// whole of what P5 asks about.
pub const DISTINCT: usize = 2;

const _: () = assert!(FLOOR < BUDGET as i128 && (BUDGET as i128) < CEILING);
const _: () = assert!(DESIGNATED[0] != DESIGNATED[1] && DESIGNATED[0] == DESIGNATED[2]);
const _: () = assert!(DISTINCT < MOVES);

/// What the procedure refers to across phases, and the journal that produced it.
pub struct Constructed {
    /// `B` — the inflow the shared ancestor selects, and nothing else does.
    pub budget: CommitmentId,
    /// `H`, `E` and `L` — one per fork, in the order of [`PLANS`].
    pub plans: [CommitmentId; FORKS],
    /// The party that forks first.
    pub planner: AgentId,
    /// The party that forks after it, so that the lineage is answerable to two.
    pub steward: AgentId,
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
        Admission::Agent {
            label: "planner".into(),
            recorded_at: day(1),
        },
        Admission::Agent {
            label: "steward".into(),
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
    let (planner, steward) = (admitted.agents[2], admitted.agents[3]);
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

    journal.push(flowing(inflow, customer, merchant, BUDGET, 2));
    journal.extend(PLANS.map(|magnitude| flowing(outflow, merchant, customer, magnitude, 3)));
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let plans = [
        admitted.commitments[1],
        admitted.commitments[2],
        admitted.commitments[3],
    ];

    Ok(Constructed {
        budget: admitted.commitments[0],
        plans,
        planner,
        steward,
        instance,
        journal,
        admitted,
    })
}

/// The shared ancestor: funded, and nothing yet decided about spending it.
///
/// It names no party, so the record holds one decision that claims nobody and three that do. See
/// the module docstring — that mixture is the realistic shape and not an omission.
pub fn shared(budget: CommitmentId) -> Decision {
    Decision::Genesis {
        known_at: day(10),
        selection: [budget].into(),
    }
}

/// A fork that withdraws nothing and asks for one more commitment.
pub fn also(extends: ThesisId, commitment: CommitmentId) -> Decision {
    Decision::Fork {
        extends,
        omitted: [].into(),
        introduced: [commitment].into(),
    }
}

/// The record every phase starts from: four worlds, decided by two parties and by nobody.
pub struct Founded {
    pub subject: Constructed,
    pub decisions: Vec<Taken>,
    pub lineage: Lineage,
    /// The three forks, in the order of [`PLANS`], which is the order [`DESIGNATED`] indexes.
    pub forks: [ThesisId; FORKS],
}

impl Founded {
    pub fn shared(&self) -> &Thesis {
        &self.lineage.decided()[0]
    }

    /// The world the plan names at one move, resolved through [`DESIGNATED`].
    pub fn designated(&self, move_of: usize) -> ThesisId {
        self.forks[DESIGNATED[move_of]]
    }

    /// The files as a whole write puts them on disk.
    pub fn files(&self) -> (&[Admission], &[Taken], Vec<WorldRecord>) {
        (
            &self.subject.journal,
            &self.decisions,
            worlds(&self.lineage),
        )
    }
}

/// Build the record before anything designates anything.
///
/// The two parties alternate rather than one taking all three, so that a per-party reading has two
/// populations to be about. Which party takes which fork is fixed here and referred to by name.
pub fn founded() -> Result<Founded, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let subject = construct(&mut canon)?;

    let mut lineage = Lineage::new();
    let genesis = Taken::now(shared(subject.budget), &subject.admitted)?;

    lineage::decide(canon.history(), &mut lineage, &genesis.decision)?;

    let ancestor = lineage
        .decided()
        .last()
        .ok_or(SubjectError::NothingDecided)?
        .id();

    let mut decisions = vec![genesis];
    let mut forks = Vec::with_capacity(FORKS);

    for (index, plan) in subject.plans.iter().enumerate() {
        let by = if index == 1 {
            subject.steward
        } else {
            subject.planner
        };
        let taken = Taken::claimed(also(ancestor, *plan), by, &subject.admitted)?;

        lineage::decide(canon.history(), &mut lineage, &taken.decision)?;
        decisions.push(taken);

        forks.push(
            lineage
                .decided()
                .last()
                .ok_or(SubjectError::NothingDecided)?
                .id(),
        );
    }

    let forks = forks
        .try_into()
        .map_err(|_| SubjectError::NothingDecided)
        .expect("one world per plan");

    Ok(Founded {
        subject,
        decisions,
        lineage,
        forks,
    })
}

/// Put the founded record on disk the way an application does — one call, all or nothing.
pub fn found(repository: &Repository, founded: &Founded) -> Result<(), SubjectError> {
    let (journal, lineage, worlds) = founded.files();

    repository.write_whole(RepositoryInput {
        journal,
        lineage,
        worlds: &worlds,
    })?;

    Ok(())
}

/// What a party reads before it does anything.
pub fn read(repository: &Repository) -> Result<Corroborated, SubjectError> {
    Ok(reading::corroborated(repository)?)
}

/// Admit knowledge against what this party read, extending its journal.
pub fn admit(working: &mut Corroborated, admission: Admission) -> Result<(), SubjectError> {
    working.journal.push(admission);

    journal::replay_remaining(&mut working.canon, &working.journal, &mut working.admitted)?;

    Ok(())
}

/// Put back everything this party holds, whole.
pub fn write(repository: &Repository, working: &Corroborated) -> Result<(), SubjectError> {
    repository.write_whole(RepositoryInput {
        journal: &working.journal,
        lineage: &working.decisions,
        worlds: &worlds(&working.lineage),
    })?;

    Ok(())
}

/// The witnesses for every world a lineage produced.
pub fn worlds(lineage: &Lineage) -> Vec<WorldRecord> {
    lineage.decided().iter().map(WorldRecord::of).collect()
}

/// A repository path no other process shares, emptied before it is used.
pub fn scratch(named: &str) -> Repository {
    let path = std::env::temp_dir()
        .join(format!("ape-designation-{}", std::process::id()))
        .join(named);
    let _ = std::fs::remove_dir_all(&path);

    Repository::open(path)
}

fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
