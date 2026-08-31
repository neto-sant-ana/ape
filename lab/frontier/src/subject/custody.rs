//! The custody subject: one record whose journal outran its last decision.
//!
//! ```text
//! cash  ∈ [0, 1000]        what every world here is about
//! paper ∈ [0,  100]        a second resource no world about cash ever selects
//!
//! ── the prefix ─────────────────────────────────────────────────────────────
//! V   twenty entries of vocabulary, recorded day 1
//!       one of which — the `observer` role — is referenced by nothing
//! F   receive 400   recorded day 2
//! E₁  Event settling F, day 3
//! D   the decision: genesis, known_at day 9, proposing nothing
//! ── the tail ───────────────────────────────────────────────────────────────
//! G   spend 40      recorded day 10
//! E₂  Event settling G, day 11
//! T₁  file 5        recorded day 12
//! T₂  file 7        recorded day 13
//! ```
//!
//! `D` is taken after `E₁`, so its witness names the twenty-two entries above it and none of the
//! four below. That is not a defect of this arrangement: it is what every record looks like between
//! one decision and the next.
//!
//! # What the arrangement has to hold, and why each part of it
//!
//! **A tail of more than one entry**, because a tail of one cannot be partially lost and the
//! experiment is about which part of it matters.
//!
//! **A tail with two halves.** An Event moves the chain a later cut resolves against and drags in
//! what it settled, so `E₂` and the `G` it names are recognized by any decision taken afterwards.
//! `T₁` and `T₂` are settled by nothing, so an advance never selects them. Both halves are in the
//! same tail, which is what lets a phase say *which* half a measurement is about instead of reporting
//! one number for the tail entire.
//!
//! **What divides them is the Event and not the resource**, and that is measured rather than assumed
//! — [`Moved::OneMoreUnsettledOnTheAccount`] is an outflow on the very resource every world here is
//! about, and a later advance reaches it exactly as much as it reaches a filing. The second resource
//! is doing a different job, below.
//!
//! **A second resource, so a gained Event can move a world without moving a number.** An Event
//! settling `T₁` changes the chain, the identity and what is frozen, and leaves the account's answer
//! where it was. Without a resource no world about cash weighs, that state would be unproducible —
//! and it is the one that keeps *what a claim covers* apart from *what an answer depends on*.
//!
//! **A control in the prefix, of the same class as the unreached half.** The `observer` role is
//! referenced by nothing, admitted by no statement, and no world is a function of it — exactly like
//! `T₁`. It sits *before* the coordinate. So the experiment's central comparison is two entries of
//! one kind, on two sides of one line, and the difference between them is not what they are.
//!
//! **Both directions.** A record may also gain entries past its last coordinate, and the two
//! extensions are chosen the same way the two halves are: one more filing, which nothing reaches, and
//! an Event settling `T₁`, which moves the chain and therefore does.
//!
//! **A later decision, because nothing else can tell two of these records apart.** An `Advance` to
//! day 20 is the smallest decision that recognizes whatever the tail left behind, and its world is
//! read on the account — where [`DECIDED_AGAIN`] differs from [`DECIDED`] only if the tail's reached
//! half survived.
//!
//! Every quantity is an integer, for the reason [`super::reconstruction`] gives.

use std::collections::BTreeSet;

use ape::canon::Canon;
use ape::engine::thesis::{Interpretation, Thesis};
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

/// The bound the second resource answers to, which no world about cash ever weighs.
pub const TALLY: i128 = 100;

/// What the fund puts in the account, and what the tail's outflow takes out of it.
pub const FUNDED: u128 = 400;
pub const SPENT: u128 = 40;

/// The tail's two filings, which move the ledger and nothing else.
pub const FILINGS: [u128; 2] = [5, 7];

/// What a third filing would be, where a record gains an entry past its last coordinate.
pub const FILED_AGAIN: u128 = 11;

/// The instants, in the order they occur.
pub const VOCABULARY_ON: u8 = 1;
pub const FUNDED_ON: u8 = 2;
pub const SETTLED_ON: u8 = 3;

/// When the record's last decision was taken, which is where the prefix ends.
pub const DECIDED_AT: u8 = 9;

/// The tail: an outflow, the Event settling it, and two filings.
pub const SPENT_ON: u8 = 10;
pub const SPENT_SETTLED_ON: u8 = 11;
pub const FILED_ON: [u8; 2] = [12, 13];

/// When a record that gained an entry gained it.
pub const EXTENDED_ON: u8 = 14;

/// When the later decision recognizes history up to.
pub const DECIDED_AGAIN_AT: u8 = 20;

/// When every world is read.
pub const ASKED_AT: u8 = 30;

/// The vocabulary, and the one entry of it nothing refers to.
pub const VOCABULARY: usize = 20;

/// Everything the last decision's witness names: the vocabulary, the fund, and the Event.
pub const PREFIX: usize = VOCABULARY + 2;

/// What the record holds past that coordinate, and how it divides.
pub const TAIL: usize = 4;
pub const REACHED: usize = 2;
pub const UNREACHED: usize = 2;

/// The whole journal.
pub const ENTRIES: usize = PREFIX + TAIL;

/// How many decisions the record holds before anything decides again.
pub const DECISIONS: usize = 1;

/// What the record's one world answers on the account: what has settled, then what it intends.
pub const DECIDED: (i128, i128) = (FUNDED as i128, FUNDED as i128);

/// And what a later decision answers, in a record whose tail still holds its reached half.
pub const DECIDED_AGAIN: (i128, i128) = ((FUNDED - SPENT) as i128, (FUNDED - SPENT) as i128);

/// The literals above, weighed against each other before anything runs.
const _: () = assert!(FLOOR < FUNDED as i128 && (FUNDED as i128) < CEILING);
const _: () = assert!(SPENT < FUNDED, "the account never leaves its bounds");
const _: () = assert!(
    !alike(DECIDED, DECIDED_AGAIN),
    "the reached half moves a number"
);
const _: () = assert!(FILINGS[0] != FILINGS[1] && FILED_AGAIN != FILINGS[0]);
const _: () = assert!(FILED_AGAIN != FILINGS[1]);
// The ledger holds every filing, so nothing here is refused for being infeasible.
const _: () = assert!(
    (FILINGS[0] + FILINGS[1] + FILED_AGAIN) as i128 <= TALLY,
    "the ledger holds every filing"
);
// The tail divides in two, and both halves are large enough to be partially lost.
const _: () = assert!(TAIL == REACHED + UNREACHED);
const _: () = assert!(REACHED > 1 && UNREACHED > 1);
const _: () = assert!(ENTRIES == PREFIX + TAIL);
// Every instant is in the order the arrangement describes.
const _: () = assert!(VOCABULARY_ON < FUNDED_ON && FUNDED_ON < SETTLED_ON);
const _: () = assert!(SETTLED_ON < DECIDED_AT && DECIDED_AT < SPENT_ON);
const _: () = assert!(SPENT_ON < SPENT_SETTLED_ON && SPENT_SETTLED_ON < FILED_ON[0]);
const _: () = assert!(FILED_ON[0] < FILED_ON[1] && FILED_ON[1] < EXTENDED_ON);
const _: () = assert!(EXTENDED_ON < DECIDED_AGAIN_AT && DECIDED_AGAIN_AT < ASKED_AT);

const fn alike(one: (i128, i128), other: (i128, i128)) -> bool {
    one.0 == other.0 && one.1 == other.1
}

/// The three files one repository is made of, as values rather than as a directory.
#[derive(Debug)]
pub struct Files {
    pub journal: Vec<Admission>,
    pub lineage: Vec<Taken>,
    pub worlds: Vec<WorldRecord>,
}

/// What has happened to a record's journal past the coordinate of its last decision.
///
/// Eight states and they are closed. Six move the tail — four by losing part of it, two by gaining
/// beside it — one moves nothing, and one moves the **prefix**, which is the control and the only
/// one anything is expected to refuse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Moved {
    /// Nothing.
    Nothing,
    /// Everything past the last coordinate, gone.
    TheWholeTail,
    /// The half of the tail no later decision reaches, gone.
    WhatNothingReaches,
    /// The half a later decision does reach, gone.
    WhatSomethingReaches,
    /// One more entry nothing would reach.
    OneMoreNothingReaches,
    /// One more entry something would reach: an Event, which moves the chain a cut resolves against.
    OneMoreSomethingReaches,
    /// One more unsettled commitment, on the very resource every world here is about.
    ///
    /// The control for the *other* half of the split. It is not there to be reached or not reached;
    /// it is there so a phase can say whether what makes the tail's unreached half unreached is the
    /// resource it moves or the absence of an Event naming it.
    OneMoreUnsettledOnTheAccount,
    /// One entry from *before* the coordinate, gone — of the same class as the unreached half.
    OneFromThePrefix,
}

/// Every state, in the order the phases report them.
pub const STATES: [Moved; 8] = [
    Moved::Nothing,
    Moved::TheWholeTail,
    Moved::WhatNothingReaches,
    Moved::WhatSomethingReaches,
    Moved::OneMoreNothingReaches,
    Moved::OneMoreSomethingReaches,
    Moved::OneMoreUnsettledOnTheAccount,
    Moved::OneFromThePrefix,
];

impl Moved {
    /// What a table calls this state.
    pub fn named(self) -> &'static str {
        match self {
            Self::Nothing => "nothing moved",
            Self::TheWholeTail => "without the whole tail",
            Self::WhatNothingReaches => "without what nothing reaches",
            Self::WhatSomethingReaches => "without what something reaches",
            Self::OneMoreNothingReaches => "with one more nothing reaches",
            Self::OneMoreSomethingReaches => "with one more something reaches",
            Self::OneMoreUnsettledOnTheAccount => "with one more unsettled, on the account",
            Self::OneFromThePrefix => "without one from the prefix",
        }
    }
}

/// The record, and everything a phase needs to ask it anything.
pub struct Arranged {
    /// The record as its writer left it: the whole journal, one decision, one world.
    pub files: Files,
    pub account: ResourceInstanceId,
    pub ledger: ResourceInstanceId,
    /// The fund, which the prefix's Event settled.
    pub fund: CommitmentId,
    /// The tail's outflow, which the tail's Event settled.
    pub outflow: CommitmentId,
    /// The tail's filings, in the order admitted.
    pub filings: [CommitmentId; 2],
    /// The prefix entry nothing refers to and no world is a function of.
    pub unreferenced: EntryId,
    /// A third filing, and an Event settling the first — the two ways a record gains an entry.
    pub one_more_unreached: Admission,
    pub one_more_reached: Admission,
    /// And one more outflow on the account, which no Event settles.
    pub one_more_unsettled: Admission,
}

/// Found the record from one statement of the subject.
pub fn arranged() -> Result<Arranged, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let mut constructed = construct(&mut canon)?;

    let founding = Taken::now(
        Decision::Genesis {
            known_at: day(DECIDED_AT),
            selection: [].into(),
        },
        &constructed.admitted,
    )?;

    let mut lineage = Lineage::new();
    lineage::decide(canon.history(), &mut lineage, &founding.decision)?;

    let unreferenced = constructed.admitted.entries[constructed.unreferenced].clone();

    let outflow = constructed.spend(&mut canon, SPENT, SPENT_ON)?;
    constructed.settle(&mut canon, outflow, SPENT_SETTLED_ON)?;

    let filings = [
        constructed.file(&mut canon, FILINGS[0], FILED_ON[0])?,
        constructed.file(&mut canon, FILINGS[1], FILED_ON[1])?,
    ];

    let one_more_unreached = constructed.filing_of(FILED_AGAIN, EXTENDED_ON);
    let one_more_unsettled = constructed.outflow_of(SPENT, EXTENDED_ON);

    Ok(Arranged {
        files: Files {
            journal: constructed.journal,
            lineage: vec![founding],
            worlds: lineage.decided().iter().map(WorldRecord::of).collect(),
        },
        account: constructed.account,
        ledger: constructed.ledger,
        fund: constructed.fund,
        outflow,
        filings,
        unreferenced,
        one_more_unreached,
        one_more_unsettled,
        one_more_reached: settling(filings[0], EXTENDED_ON),
    })
}

impl Arranged {
    /// The journal one of the seven states leaves.
    ///
    /// Positions rather than identities, because what is being modelled is a **file** that has been
    /// edited: whoever truncates a journal removes lines from it, and the addresses are what a reader
    /// derives afterwards.
    pub fn moved(&self, moved: Moved) -> Vec<Admission> {
        let journal = &self.files.journal;
        let reached = PREFIX..PREFIX + REACHED;
        let unreached = PREFIX + REACHED..ENTRIES;

        match moved {
            Moved::Nothing => journal.clone(),
            Moved::TheWholeTail => journal[..PREFIX].to_vec(),
            Moved::WhatNothingReaches => journal[..PREFIX + REACHED].to_vec(),
            Moved::WhatSomethingReaches => journal[..PREFIX]
                .iter()
                .chain(&journal[unreached])
                .cloned()
                .collect(),
            Moved::OneMoreNothingReaches => appended(journal, &self.one_more_unreached),
            Moved::OneMoreSomethingReaches => appended(journal, &self.one_more_reached),
            Moved::OneMoreUnsettledOnTheAccount => appended(journal, &self.one_more_unsettled),
            Moved::OneFromThePrefix => {
                let _ = reached;
                without(journal, self.unreferenced_at())
            }
        }
    }

    /// Where in the journal the unreferenced prefix entry sits.
    pub fn unreferenced_at(&self) -> usize {
        addresses(&self.files.journal)
            .expect("the journal admits")
            .iter()
            .position(|held| held == &self.unreferenced)
            .expect("the arrangement admitted it")
    }
}

/// The journal with one more entry at the end.
fn appended(journal: &[Admission], entry: &Admission) -> Vec<Admission> {
    let mut extended = journal.to_vec();
    extended.push(entry.clone());

    extended
}

/// The journal with the entry at `position` removed.
pub fn without(journal: &[Admission], position: usize) -> Vec<Admission> {
    let mut kept = journal.to_vec();
    kept.remove(position);

    kept
}

/// What a record holds past the coordinate of its last decision.
///
/// Derived rather than declared, and it is the derivation the whole question is about: the tail is
/// whatever the journal offers after the entry the last decision was taken at. A record that could
/// name this without replaying would be a record that says something about its own extent.
pub fn tail(files: &Files) -> Result<Vec<Admission>, SubjectError> {
    let held = addresses(&files.journal)?;

    let Some(last) = files.lineage.last() else {
        return Ok(files.journal.clone());
    };

    let at = held
        .iter()
        .position(|entry| entry == &last.after)
        .ok_or(SubjectError::NothingDecided)?;

    Ok(files.journal[at + 1..].to_vec())
}

/// The record that record becomes by deciding again over the journal it now holds.
///
/// The one thing that can tell two of these records apart, and the reason it can is that a decision
/// resolves its cut against knowledge as it stands. Everything else about the two is equal.
pub fn deciding_again(lineage: &[Taken], journal: &[Admission]) -> Result<Files, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let (built, admitted) = lineage::rebuild(&mut canon, journal, lineage)?;

    let extends = built
        .decided()
        .last()
        .ok_or(SubjectError::NothingDecided)?
        .id();

    let mut decisions = lineage.to_vec();
    decisions.push(Taken::now(
        Decision::Advance {
            extends,
            known_at: day(DECIDED_AGAIN_AT),
        },
        &admitted,
    )?);

    kept(journal, &decisions)
}

/// The record that record becomes by forking its latest world to introduce one commitment.
///
/// The decision an advance is not. An advance recognizes what history settled, so nothing the tail
/// holds *unsettled* is ever selected by one; a fork names a commitment outright, which is the only
/// way an entry no Event refers to reaches a world at all. Whether it can reach one is exactly the
/// question of whether losing that entry cost anything.
pub fn introducing(files: &Files, commitment: CommitmentId) -> Result<Files, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let (built, admitted) = lineage::rebuild(&mut canon, &files.journal, &files.lineage)?;

    let extends = built
        .decided()
        .last()
        .ok_or(SubjectError::NothingDecided)?
        .id();

    let mut decisions = files.lineage.clone();
    decisions.push(Taken::now(
        Decision::Fork {
            extends,
            omitted: BTreeSet::new(),
            introduced: [commitment].into(),
        },
        &admitted,
    )?);

    kept(&files.journal, &decisions)
}

/// The whole record a writer would keep, given a journal and the decisions taken in it.
pub fn kept(journal: &[Admission], decisions: &[Taken]) -> Result<Files, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let (built, _) = lineage::rebuild(&mut canon, journal, decisions)?;

    Ok(Files {
        journal: journal.to_vec(),
        lineage: decisions.to_vec(),
        worlds: built.decided().iter().map(WorldRecord::of).collect(),
    })
}

/// A record rebuilt, and everything a phase asks of it.
pub struct Rebuilt {
    pub canon: Canon<ResidentHistory>,
    pub lineage: Lineage,
    pub admitted: Replayed,
}

/// Rebuild a whole record from values, the way `reading::corroborated` does from a directory.
pub fn rebuilt(files: &Files) -> Result<Rebuilt, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let (lineage, admitted) = lineage::rebuild(&mut canon, &files.journal, &files.lineage)?;

    Ok(Rebuilt {
        canon,
        lineage,
        admitted,
    })
}

/// Every address a journal produces, in order.
pub fn addresses(journal: &[Admission]) -> Result<Vec<EntryId>, JournalError> {
    let mut canon = Canon::new(ResidentHistory::new());

    Ok(journal::replay(&mut canon, journal)?.entries)
}

/// Put a whole repository there the way an application does — one call, all or nothing.
pub fn write_whole(repository: &Repository, files: &Files) -> Result<(), RepositoryError> {
    repository.write_whole(RepositoryInput {
        journal: &files.journal,
        lineage: &files.lineage,
        worlds: &files.worlds,
        designations: &[],
    })
}

pub fn asked_at() -> Date {
    Date::parse(day(ASKED_AT)).expect("the instant every reading is taken at is a date")
}

/// What one world answers on an instance: what has settled, and what it intends.
pub fn answered(
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

/// The vocabulary, the fund, and whatever the record goes on to admit.
pub struct Constructed {
    pub journal: Vec<Admission>,
    pub admitted: Replayed,
    pub fund: CommitmentId,
    pub account: ResourceInstanceId,
    pub ledger: ResourceInstanceId,
    /// The position, in the journal, of the prefix entry nothing refers to.
    pub unreferenced: usize,
    inflow: StatementId,
    outflow: StatementId,
    filing: StatementId,
    customer: AgentId,
    merchant: AgentId,
    clerk: AgentId,
}

impl Constructed {
    /// Admit the outflow the tail's Event will settle.
    pub fn spend(
        &mut self,
        canon: &mut Canon<ResidentHistory>,
        magnitude: u128,
        at: u8,
    ) -> Result<CommitmentId, JournalError> {
        self.journal.push(self.outflow_of(magnitude, at));

        self.replay(canon)
    }

    /// One outflow against the account, as a value.
    pub fn outflow_of(&self, magnitude: u128, at: u8) -> Admission {
        Admission::Commitment {
            accountable: self.merchant,
            executors: [self.merchant].into(),
            beneficiaries: [self.customer].into(),
            statement: self.outflow,
            resource: self.account,
            committed_at: day(at),
            due_date: day(25),
            magnitude: Some(magnitude),
            dependencies: [].into(),
            recorded_at: day(at),
        }
    }

    /// Admit one filing against the ledger, which no world about cash selects.
    pub fn file(
        &mut self,
        canon: &mut Canon<ResidentHistory>,
        magnitude: u128,
        at: u8,
    ) -> Result<CommitmentId, JournalError> {
        self.journal.push(self.filing_of(magnitude, at));

        self.replay(canon)
    }

    /// The same filing as a value, for a record that has yet to gain it.
    pub fn filing_of(&self, magnitude: u128, at: u8) -> Admission {
        Admission::Commitment {
            accountable: self.clerk,
            executors: [self.clerk].into(),
            beneficiaries: [self.clerk].into(),
            statement: self.filing,
            resource: self.ledger,
            committed_at: day(at),
            due_date: day(25),
            magnitude: Some(magnitude),
            dependencies: [].into(),
            recorded_at: day(at),
        }
    }

    /// Admit the Event that settles `commitment`, extending the chain a cut resolves against.
    pub fn settle(
        &mut self,
        canon: &mut Canon<ResidentHistory>,
        commitment: CommitmentId,
        at: u8,
    ) -> Result<(), JournalError> {
        self.journal.push(settling(commitment, at));

        journal::replay_remaining(canon, &self.journal, &mut self.admitted)
    }

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
            committed_at: day(FUNDED_ON),
            due_date: day(25),
            magnitude: Some(magnitude),
            dependencies: [].into(),
            recorded_at: day(FUNDED_ON),
        });

        self.replay(canon)
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

/// The Event settling one commitment, as a value.
pub fn settling(commitment: CommitmentId, at: u8) -> Admission {
    Admission::Event {
        commitment,
        observation: FULFILLING.into(),
        occurred_at: day(at),
        recorded_at: day(at),
    }
}

/// Admit the vocabulary, the fund and the Event that settles it.
///
/// Two vocabularies rather than one, so the tail's unreached half is unreachable rather than merely
/// unselected — and one role, `observer`, that neither vocabulary uses. That role is the control:
/// it is the same kind of thing as a filing, and it sits on the other side of the coordinate.
pub fn construct(canon: &mut Canon<ResidentHistory>) -> Result<Constructed, JournalError> {
    let mut journal = Vec::new();
    let mut admitted = Replayed::default();
    let recorded = day(VOCABULARY_ON);

    journal.extend([
        Admission::Role {
            label: "payer".into(),
            recorded_at: recorded.clone(),
        },
        Admission::Role {
            label: "payee".into(),
            recorded_at: recorded.clone(),
        },
        Admission::Role {
            label: "auditor".into(),
            recorded_at: recorded.clone(),
        },
        Admission::Role {
            label: "observer".into(),
            recorded_at: recorded.clone(),
        },
        Admission::Agent {
            label: "customer".into(),
            recorded_at: recorded.clone(),
        },
        Admission::Agent {
            label: "merchant".into(),
            recorded_at: recorded.clone(),
        },
        Admission::Agent {
            label: "clerk".into(),
            recorded_at: recorded.clone(),
        },
        Admission::Resource {
            label: "cash".into(),
            kind: ResourceKindRecord::Between {
                lower: FLOOR,
                upper: CEILING,
            },
            recorded_at: recorded.clone(),
        },
        Admission::Resource {
            label: "paper".into(),
            kind: ResourceKindRecord::Between {
                lower: FLOOR,
                upper: TALLY,
            },
            recorded_at: recorded.clone(),
        },
    ]);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let (payer, payee, auditor) = (admitted.roles[0], admitted.roles[1], admitted.roles[2]);
    let (customer, merchant, clerk) = (admitted.agents[0], admitted.agents[1], admitted.agents[2]);
    let (cash, paper) = (admitted.resources[0], admitted.resources[1]);

    // The position of the role nothing refers to, taken here rather than searched for later: it is
    // the fourth entry, and what makes it the control is that no admission below names it.
    let unreferenced = 3;

    journal.extend([
        Admission::Eligibility {
            agent: customer,
            roles: [payer].into(),
            effective_from: recorded.clone(),
            recorded_at: recorded.clone(),
        },
        Admission::Eligibility {
            agent: merchant,
            roles: [payee].into(),
            effective_from: recorded.clone(),
            recorded_at: recorded.clone(),
        },
        Admission::Eligibility {
            agent: clerk,
            roles: [auditor].into(),
            effective_from: recorded.clone(),
            recorded_at: recorded.clone(),
        },
        Admission::ResourceInstance {
            label: "account".into(),
            resource: cash,
            recorded_at: recorded.clone(),
        },
        Admission::ResourceInstance {
            label: "ledger".into(),
            resource: paper,
            recorded_at: recorded.clone(),
        },
        Admission::Action {
            verb: "receive".into(),
            kind: ActionKindRecord::Quantifiable(EffectRecord::Increase),
            resource: cash,
            recorded_at: recorded.clone(),
        },
        Admission::Action {
            verb: "spend".into(),
            kind: ActionKindRecord::Quantifiable(EffectRecord::Decrease),
            resource: cash,
            recorded_at: recorded.clone(),
        },
        Admission::Action {
            verb: "file".into(),
            kind: ActionKindRecord::Quantifiable(EffectRecord::Increase),
            resource: paper,
            recorded_at: recorded.clone(),
        },
    ]);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let (account, ledger) = (admitted.instances[0], admitted.instances[1]);
    let (receive, spend, file) = (
        admitted.actions[0],
        admitted.actions[1],
        admitted.actions[2],
    );

    journal.extend([
        statement(payer, payee, receive),
        statement(payee, payer, spend),
        statement(auditor, auditor, file),
    ]);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    let mut constructed = Constructed {
        // Assigned by `receive` below. Nothing reads it in between, and the alternative is an
        // Option every phase would have to unwrap for a value the subject always has.
        fund: CommitmentId::from([0u8; 32]),
        account,
        ledger,
        unreferenced,
        inflow: admitted.statements[0],
        outflow: admitted.statements[1],
        filing: admitted.statements[2],
        customer,
        merchant,
        clerk,
        journal,
        admitted,
    };

    constructed.fund = constructed.receive(canon, FUNDED)?;

    let fund = constructed.fund;
    constructed.settle(canon, fund, SETTLED_ON)?;

    Ok(constructed)
}

fn statement(actor: RoleId, recipient: RoleId, action: ActionId) -> Admission {
    Admission::Statement {
        actors: [actor].into(),
        recipients: [recipient].into(),
        action,
        fulfills: [FULFILLING.to_owned()].into(),
        cancels: [CANCELLING.to_owned()].into(),
        recorded_at: day(VOCABULARY_ON),
    }
}

/// Every identity a worlds file names, whatever coordinate it names it at.
///
/// What Phase 0 reads it for is a negative: no world mentions anything the tail holds. A comparison
/// per field would have had to enumerate the fields, and the question is about the file entire.
pub fn named(worlds: &[WorldRecord]) -> BTreeSet<String> {
    worlds
        .iter()
        .flat_map(|world| {
            [Some(world.thesis.clone()), world.thesis_parent.clone()]
                .into_iter()
                .flatten()
                .chain(world.event_head.clone())
                .chain(world.frozen.iter().cloned())
                .chain(world.open.iter().cloned())
        })
        .collect()
}

pub fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
