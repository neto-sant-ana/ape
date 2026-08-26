//! The commensurability subject: two records founded apart that decided one world, and two
//! insertions that look alike and are not.
//!
//! ```text
//! cash ∈ [0, 1000]
//!
//! shared, admitted independently by both records, identical entry for entry
//!   F   receive 400   committed day 2, recorded day 2
//!   E₀  Event settling F, occurred day 3, recorded day 3
//!
//! one adds     H   spend 50   recorded day 4      and  Eₕ  Event settling H, recorded day 5
//! other adds   P   spend 70   recorded day 6
//! ```
//!
//! ```text
//!                                   entries  worlds        settled  intended
//!   base                               14       —
//!   one    { D₀, D₁ }                  16       2          400/350  400/350
//!   other  { D₀, D₂ }                  15       2          400/400  400/330
//!   union  { D₀, D₀, D₁, D₂ }          17       —          the question
//! ```
//!
//! `D₀` is **the same decision on both sides but for the party claiming it**, so the two records
//! decided one world by identity with nothing between them — which is the collision experiment's
//! twinning, and what makes a partial meeting have something to be partial about.
//!
//! # The two insertions, and why they have to look alike
//!
//! In the union, `H` and `Eₕ` both fall before `other`'s coordinate, and both are refused by the
//! witness today. Exactly one of them matters:
//!
//! ```text
//! H    a Commitment nothing in `other`'s world selects. `D₂` produces the same world
//!      by identity against the extended journal                            CANNOT have changed it
//!
//! Eₕ   an Event, which no selection names either — and a cut resolves its head from an
//!      instant, so `D₂`'s cut moves from E₀ to Eₕ, `H` enters the frozen past of a world
//!      that never selected it, and the settled level moves 400 → 350        CAN have changed it
//! ```
//!
//! They are adjacent, both belong to the other record, and neither is in anybody's selection. An
//! arrangement whose insertions were all of one kind would let a comparison admit everything and
//! report a property of the subject.
//!
//! **`D₂` is a genesis rather than a fork, and that is load-bearing.** A fork inherits its parent's
//! cut, so an Event admitted before it cannot move anything; only a decision that resolves a cut of
//! its own can be changed by one. An arrangement built out of forks could not have failed.
//!
//! Every quantity is an integer, for the reason [`super::reconstruction`] gives.

use ape::canon::Canon;
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
use ape_cli::repository::{Repository, RepositoryInput};

pub const FULFILLING: &str = "Settled";
pub const CANCELLING: &str = "Void";

/// The bounds the account answers to.
pub const FLOOR: i128 = 0;
pub const CEILING: i128 = 1000;

/// What the shared base puts in the account, and settles.
pub const FUNDED: u128 = 400;

/// What each record plans against it, in the order the records are held.
///
/// Distinct, because every other field of the two admissions is equal — two plans of one size would
/// be one commitment by identity, and the two records would not have diverged at all.
pub const SPENT: [u128; 2] = [50, 70];

/// The day the shared Event settled the fund, and the day the first record settled its own plan.
pub const SETTLED_ON: u8 = 3;
pub const OBSERVED_ON: u8 = 5;

/// The days each record's own entries are recorded on: `H`, then `Eₕ`, then `P`.
///
/// `P` is recorded **after** `Eₕ` so that the union admits in an order where both of the first
/// record's entries fall before the second record's coordinate. Recording is monotonic across
/// admission, so this is what puts the two insertions where the question is.
pub const RECORDED_ON: [u8; 3] = [4, 5, 6];

/// The instant the shared decision recognizes, and the instant each record's own decision does.
pub const SHARED_AT: u8 = 10;
pub const OWN_AT: u8 = 14;

/// The instant every reading is taken at, after every due date the subject holds.
pub const ASKED_AT: u8 = 20;

/// The entries each state holds.
pub const BASE_ENTRIES: usize = 14;
pub const SIDE_ENTRIES: [usize; 2] = [16, 15];
pub const UNION_ENTRIES: usize = 17;

/// The worlds each record decided: the shared one, and its own.
pub const SIDE_WORLDS: usize = 2;

/// What the shared world answers — settled, then intended.
pub const SHARED: (i128, i128) = (400, 400);

/// What each record's own world answers, in the order the records are held.
pub const OWN: [(i128, i128); 2] = [(350, 350), (400, 330)];

/// What the second record's own world answers when the Event is in front of its coordinate.
///
/// The whole point of the arrangement: a world nobody decided, produced by a decision nobody
/// changed, because a cut resolves its head from an instant and an Event arrived before it.
pub const CHANGED: (i128, i128) = (350, 280);

/// The literals above, weighed against each other before anything runs.
///
/// Arithmetic rather than measurement, for the reason the atomicity subject gives: asserted inside a
/// phase this would read as a result. The last three are the experiment's falsifiability condition —
/// the changed world has to differ from **every** world any record decided, or a phase reporting *a
/// decision was changed* would be reporting a coincidence.
const _: () = assert!(FLOOR < FUNDED as i128 && (FUNDED as i128) < CEILING);
const _: () = assert!(SPENT[0] != SPENT[1]);
const _: () = assert!(SETTLED_ON < RECORDED_ON[0]);
const _: () = assert!(RECORDED_ON[0] < RECORDED_ON[1] && RECORDED_ON[1] < RECORDED_ON[2]);
const _: () = assert!(OBSERVED_ON <= RECORDED_ON[1]);
// The shared cut recognizes the shared Event, which is admitted before every coordinate there is.
const _: () = assert!(SETTLED_ON <= SHARED_AT);
// And the hinge: each record's own cut is late enough to recognize the inserted Event, so whether
// it does is decided by where the Event sits rather than by the instant the decision names.
const _: () = assert!(RECORDED_ON[1] <= OWN_AT && OWN_AT < ASKED_AT);

const _: () = assert!(SHARED.0 == FUNDED as i128 && SHARED.1 == FUNDED as i128);
const _: () = assert!(OWN[0].0 == FUNDED as i128 - SPENT[0] as i128);
const _: () = assert!(OWN[0].1 == OWN[0].0);
const _: () = assert!(OWN[1].0 == FUNDED as i128 && OWN[1].1 == FUNDED as i128 - SPENT[1] as i128);
const _: () = assert!(CHANGED.0 == FUNDED as i128 - SPENT[0] as i128);
const _: () = assert!(CHANGED.1 == FUNDED as i128 - SPENT[0] as i128 - SPENT[1] as i128);
const _: () = assert!(FLOOR < CHANGED.1);

const _: () = assert!(!matching(CHANGED, SHARED));
const _: () = assert!(!matching(CHANGED, OWN[0]) && !matching(CHANGED, OWN[1]));
const _: () = assert!(!matching(OWN[0], OWN[1]) && !matching(OWN[0], SHARED));

const _: () = assert!(UNION_ENTRIES == BASE_ENTRIES + 3);
const _: () = assert!(SIDE_ENTRIES[0] == BASE_ENTRIES + 2 && SIDE_ENTRIES[1] == BASE_ENTRIES + 1);

const fn matching(one: (i128, i128), other: (i128, i128)) -> bool {
    one.0 == other.0 && one.1 == other.1
}

/// The three files a repository is made of, held as values rather than as a directory.
pub struct Files {
    pub journal: Vec<Admission>,
    pub lineage: Vec<Taken>,
    pub worlds: Vec<WorldRecord>,
}

/// One record: what it holds, who decided in it, and what its own world answers.
pub struct Side {
    pub label: &'static str,
    pub party: AgentId,
    pub files: Files,
    /// The addresses this record's journal produced, in order.
    pub entries: Vec<EntryId>,
    /// The world it decided that the other did not, from [`OWN`].
    pub answers: (i128, i128),
}

impl Side {
    /// The entries this record holds that the other one does not.
    ///
    /// Derived by position rather than listed, because the two journals share a prefix by content
    /// and an arrangement that named the tail could name one it does not have.
    pub fn beyond(&self, shared: usize) -> &[EntryId] {
        &self.entries[shared..]
    }
}

/// What the procedure refers to across phases.
pub struct Arranged {
    /// The two records, founded apart, in the order [`SPENT`] and [`OWN`] name them.
    pub sides: [Side; 2],
    /// The entries both records produced, which is the whole of what they share.
    pub shared: Vec<EntryId>,
    /// The world both of them decided, by identity.
    pub agreed: ThesisId,
    pub instance: ResourceInstanceId,
    pub fund: CommitmentId,
    /// Each record's own plan, in the order the records are held.
    pub plans: [CommitmentId; 2],
}

impl Arranged {
    /// The union of the two journals: the shared prefix, then each record's own entries in the
    /// order recording admits them.
    ///
    /// It is a journal rather than a merge: nothing here decides anything about the two lineages,
    /// and the collision experiment established that this sequence is itself admissible.
    pub fn union(&self) -> Vec<Admission> {
        let mut journal = self.sides[0].files.journal[..BASE_ENTRIES].to_vec();

        journal.extend(self.sides[0].files.journal[BASE_ENTRIES..].iter().cloned());
        journal.extend(self.sides[1].files.journal[BASE_ENTRIES..].iter().cloned());

        journal
    }

    /// The union without the Event: the insertion that cannot have changed anything, alone.
    pub fn union_without_the_event(&self) -> Vec<Admission> {
        self.union()
            .into_iter()
            .filter(|entry| {
                !matches!(entry, Admission::Event { occurred_at, .. }
                if occurred_at == &day(OBSERVED_ON))
            })
            .collect()
    }

    /// Every decision both records hold, each kept once, ordered as a journal requires.
    ///
    /// Deduplicated by the whole [`Taken`], which is what `converge` does — so the shared decision
    /// survives **twice**, because the two records claim different parties. That duplicate is the
    /// collision experiment's Request 3 and it is the arrangement rather than a defect.
    ///
    /// Ordered by where each decision was taken in `journal`, then by its own content, because a
    /// rebuild admits in step and refuses a coordinate it has already passed. The order is the one
    /// `converge` chooses and is restated here rather than reached for: this is a journal the
    /// laboratory assembles, and no merge has run.
    pub fn lineages(&self, journal: &[Admission]) -> Vec<Taken> {
        let mut merged: Vec<Taken> = Vec::new();

        for side in &self.sides {
            for taken in &side.files.lineage {
                if !merged.contains(taken) {
                    merged.push(taken.clone());
                }
            }
        }

        let entries = addresses(journal);
        let at = |entry: &EntryId| entries.iter().position(|held| held == entry);

        merged.sort_by(|one, other| (at(&one.after), one).cmp(&(at(&other.after), other)));

        merged
    }
}

/// Found two records apart, admit the same base in each, and let each decide its own world.
///
/// The two are constructed separately on purpose: what makes them twins is that they admitted the
/// same content, not that anything copied. [`Arranged::shared`] is asserted equal by the phase that
/// reads it rather than by construction here.
pub fn arranged() -> Result<Arranged, SubjectError> {
    let mut founded = Vec::new();

    for (which, label) in ["one", "other"].into_iter().enumerate() {
        let mut canon = Canon::new(ResidentHistory::new());
        let mut constructed = construct(&mut canon)?;
        let mut lineage = Lineage::new();

        let party = constructed.parties[which];

        // The shared decision, taken in each record against the base it admitted for itself.
        let agreed = Taken::claimed(
            Decision::Genesis {
                known_at: day(SHARED_AT),
                selection: [constructed.fund].into(),
            },
            party,
            &constructed.admitted,
        )?;
        lineage::decide(canon.history(), &mut lineage, &agreed.decision)?;

        for admission in constructed.own(which) {
            constructed.admit(&mut canon, admission)?;
        }

        let plan = *constructed.admitted.commitments.last().expect("admitted");

        // A genesis rather than a fork: a fork inherits its parent's cut, so nothing admitted
        // later could move it, and the arrangement could not have failed.
        let own = Taken::claimed(
            Decision::Genesis {
                known_at: day(OWN_AT),
                selection: [constructed.fund, plan].into(),
            },
            party,
            &constructed.admitted,
        )?;
        lineage::decide(canon.history(), &mut lineage, &own.decision)?;

        founded.push((
            Side {
                label,
                party,
                files: Files {
                    journal: constructed.journal.clone(),
                    lineage: vec![agreed, own],
                    worlds: worlds(&lineage),
                },
                entries: constructed.admitted.entries.clone(),
                answers: OWN[which],
            },
            plan,
            constructed.instance,
            constructed.fund,
            lineage
                .decided()
                .first()
                .ok_or(SubjectError::NothingDecided)?
                .id(),
        ));
    }

    let (other, other_plan, ..) = founded.pop().expect("two records were founded");
    let (one, one_plan, instance, fund, agreed) = founded.pop().expect("two records were founded");

    let shared = one.entries[..BASE_ENTRIES].to_vec();

    Ok(Arranged {
        sides: [one, other],
        shared,
        agreed,
        instance,
        fund,
        plans: [one_plan, other_plan],
    })
}

/// Every address a journal produces, by replaying it into a canon of its own.
pub fn addresses(journal: &[Admission]) -> Vec<EntryId> {
    let mut canon = Canon::new(ResidentHistory::new());

    journal::replay(&mut canon, journal)
        .expect("the journal admits")
        .entries
}

/// Put a whole repository there the way an application does — one call, all or nothing.
pub fn write_whole(repository: &Repository, files: &Files) -> Result<(), RepositoryError> {
    repository.write_whole(RepositoryInput {
        journal: &files.journal,
        lineage: &files.lineage,
        worlds: &files.worlds,
    })
}

/// The witnesses for every world a lineage produced.
pub fn worlds(lineage: &Lineage) -> Vec<WorldRecord> {
    lineage.decided().iter().map(WorldRecord::of).collect()
}

/// The instant every reading is taken at.
pub fn asked_at() -> Date {
    Date::parse(day(ASKED_AT)).expect("the instant every reading is taken at is a date")
}

/// What one world answers: what has settled, and what it intends.
///
/// The pair rather than either alone, because the two move for different reasons. Settled moves
/// when the recognized chain moves, which is what an inserted Event does; intended moves when the
/// selection moves, which is what a decision does.
pub fn answers(
    history: &ResidentHistory,
    thesis: &Thesis,
    instance: ResourceInstanceId,
) -> Result<(i128, i128), ReadingError> {
    let interpretation = Interpretation::of(thesis, history)?;
    let projected = interpretation.conditions_at(&asked_at())?;

    Ok((
        level::settled(history, &projected, instance)?,
        level::intended(history, &projected, instance)?,
    ))
}

/// The vocabulary, the fund, the Event that settles it, and the two parties.
pub struct Constructed {
    pub fund: CommitmentId,
    pub instance: ResourceInstanceId,
    pub parties: [AgentId; 2],
    outflow: StatementId,
    pub journal: Vec<Admission>,
    pub admitted: Replayed,
}

impl Constructed {
    /// The entries one record admits that the other does not.
    ///
    /// The first record admits a plan and an Event settling it; the second admits a plan alone. The
    /// Event is what makes one of the two insertions matter, and it belongs to the record whose
    /// entries land earliest in the union.
    pub fn own(&self, which: usize) -> Vec<Admission> {
        let plan = self.outflow_of(SPENT[which], RECORDED_ON[if which == 0 { 0 } else { 2 }]);

        if which != 0 {
            return vec![plan];
        }

        let settled = Admission::Event {
            commitment: self.planned(&plan),
            observation: FULFILLING.into(),
            occurred_at: day(OBSERVED_ON),
            recorded_at: day(RECORDED_ON[1]),
        };

        vec![plan, settled]
    }

    /// The identity a plan will have once admitted, derived by admitting it aside.
    ///
    /// An Event names the commitment it settles, so the Event cannot be built until the plan has an
    /// identity — and the plan is not admitted yet. Replaying into a canon of its own is what
    /// answers, and it leaves nothing behind.
    fn planned(&self, plan: &Admission) -> CommitmentId {
        let mut aside = Canon::new(ResidentHistory::new());
        let mut journal = self.journal.clone();

        journal.push(plan.clone());

        *journal::replay(&mut aside, &journal)
            .expect("the plan admits")
            .commitments
            .last()
            .expect("a plan is a commitment")
    }

    fn outflow_of(&self, magnitude: u128, on: u8) -> Admission {
        Admission::Commitment {
            accountable: self.parties[1],
            executors: [self.parties[1]].into(),
            beneficiaries: [self.parties[0]].into(),
            statement: self.outflow,
            resource: self.instance,
            committed_at: day(on),
            due_date: day(ASKED_AT),
            magnitude: Some(magnitude),
            dependencies: [].into(),
            recorded_at: day(on),
        }
    }

    pub fn admit(
        &mut self,
        canon: &mut Canon<ResidentHistory>,
        admission: Admission,
    ) -> Result<(), JournalError> {
        self.journal.push(admission);

        journal::replay_remaining(canon, &self.journal, &mut self.admitted)
    }
}

/// Admit the base one record holds, accumulating the journal that describes it.
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
            label: "one".into(),
            recorded_at: day(1),
        },
        Admission::Agent {
            label: "other".into(),
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
    let (first, second) = (admitted.agents[0], admitted.agents[1]);
    let cash = admitted.resources[0];

    journal.extend([
        Admission::Eligibility {
            agent: first,
            roles: [payer].into(),
            effective_from: day(1),
            recorded_at: day(1),
        },
        Admission::Eligibility {
            agent: second,
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
        accountable: first,
        executors: [first].into(),
        beneficiaries: [second].into(),
        statement: inflow,
        resource: instance,
        committed_at: day(2),
        due_date: day(ASKED_AT),
        magnitude: Some(FUNDED),
        dependencies: [].into(),
        recorded_at: day(2),
    });
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let fund = admitted.commitments[0];

    journal.push(Admission::Event {
        commitment: fund,
        observation: FULFILLING.into(),
        occurred_at: day(SETTLED_ON),
        recorded_at: day(SETTLED_ON),
    });
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    Ok(Constructed {
        fund,
        instance,
        parties: [first, second],
        outflow,
        journal,
        admitted,
    })
}

fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
