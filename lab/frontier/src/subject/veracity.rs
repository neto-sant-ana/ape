//! The veracity subject: one line of thinking worth being wrong about, and two parties who
//! learned the same fact on different days.
//!
//! ```text
//! cash ∈ [0, 1000]
//!
//! F  receive 300   committed day 2, recorded day 2      the fund, nothing settles it
//! C  receive 120   committed day 2, recorded day 2      the claim
//! E  Event settling C, occurred day 3                   recorded day 3 by one party,
//!                                                       day 11 by the other
//! S  spend    50   committed day 4, recorded day 4      the plan
//! L  receive  80   committed day 12, recorded day 12    admitted after the cut that selects it
//! ```
//!
//! ```text
//!  base       { }                        14 entries, no decision      what both parties read
//!  earlier    { D1 }                     15 entries, 1 world          generation one
//!  faithful   { D1, D2, D3, D4 }         17 entries, 4 worlds         generation two, the anchor
//!
//!  early      { D1 }                     15 entries, 1 world, E on day 3
//!  late       { Dl }                     15 entries, 1 world, E on day 11
//! ```
//!
//! # What the arrangement has to hold, and why each part of it
//!
//! **An Event whose recording instant decides whether a cut resolves to it.** Both parties decide at
//! [`KNOWN_AT`], which lies between the two instants in [`RECORDED_ON`]. One party's cut therefore
//! recognizes the Event and the other's does not, and the two answer different levels for it.
//!
//! **A commitment whose recording instant decides whether a cut can select it.** `L` is recorded
//! after the instant the third decision recognizes, which is why the fourth decision has to advance
//! before it can introduce it.
//!
//! **Two parties whose journals are the same journal by address.** An [`EntryId`] is derived from
//! what admitting produced, and a recording instant is not part of any identity — so the two
//! journals here are equal entry for entry and differ in a value a world is a function of. That is
//! the disagreement, and the arrangement exists to hold it.
//!
//! **A faithful record the arrangement holds and the record cannot.** Falsity is defined as
//! disagreeing with the record the same events would have produced, so every answer is written down
//! here as a literal before anything runs.
//!
//! **Two generations that answer differently.** [`earlier`](Arranged::earlier) and
//! [`faithful`](Arranged::faithful) differ in their tip's intended level, so an interrupted write is
//! measured against a state it could have contradicted.
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
    self, ActionKindRecord, Admission, EffectRecord, Replayed, ResourceKindRecord,
};
use ape_cli::level;
use ape_cli::lineage::{self, Decision, Lineage, Taken};
use ape_cli::reading::{self, Corroborated, WorldRecord};
use ape_cli::repository::{Repository, RepositoryInput};

pub const FULFILLING: &str = "Settled";
pub const CANCELLING: &str = "Void";

/// The bounds the account answers to.
pub const FLOOR: i128 = 0;
pub const CEILING: i128 = 1000;

/// The fund, which nothing settles.
pub const FUNDED: u128 = 300;
/// The claim, which the Event settles.
pub const CLAIMED: u128 = 120;
/// The outflow the second decision proposes.
pub const PLANNED: u128 = 50;
/// The inflow admitted after the instant the third decision recognizes.
pub const LATER: u128 = 80;
/// One more inflow, held unadmitted, for a phase that needs knowledge nothing has decided about.
pub const SPARE: u128 = 90;

/// The day the Event occurred, which is not the day either party recorded it.
pub const OBSERVED_ON: u8 = 3;

/// The day each party recorded the Event, in the order the parties are held.
///
/// The whole hinge of the experiment: the same Event, the same identity, two instants.
pub const RECORDED_ON: [u8; 2] = [3, 11];

/// The instant both parties' decisions recognize, and it lies between the two.
pub const KNOWN_AT: u8 = 10;

/// The instant the third decision advances to, and the day `L` is committed and recorded.
pub const ADVANCED_TO: u8 = 15;
pub const LATE_ON: u8 = 12;

/// The instant every reading is taken at, after every due date the subject holds.
pub const ASKED_AT: u8 = 20;

/// The entries each whole state holds.
pub const BASE_ENTRIES: usize = 14;
pub const SIDE_ENTRIES: usize = 15;
pub const FAITHFUL_ENTRIES: usize = 17;

/// The worlds each generation of the faithful line recorded.
pub const EARLIER_WORLDS: usize = 1;
pub const FAITHFUL_WORLDS: usize = 4;

/// What each world of the faithful line answers — settled, then intended — oldest first.
///
/// Written before the run, so that a phase reporting a level is comparing against a number rather
/// than against whatever came back.
pub const FAITHFUL: [(i128, i128); FAITHFUL_WORLDS] =
    [(120, 420), (120, 370), (120, 370), (120, 450)];

/// What each party's own record answers about its own world.
pub const ALONE: [(i128, i128); 2] = [(120, 420), (0, 120)];

/// What a merged record answers about each party's world, by which party converged last.
///
/// `MERGED[i][j]` is party `j`'s world read out of the record party `i` produced by converging
/// last. The diagonal is each party's own answer, because the journal a merge keeps is the
/// converging party's; the off-diagonal is the other party's world, re-derived.
pub const MERGED: [[(i128, i128); 2]; 2] = [[(120, 420), (120, 120)], [(0, 420), (0, 120)]];

/// The assumption a level is read under, named once.
pub const HYPOTHESIS: Hypothesis = Hypothesis::FinalState;

/// The literals above, weighed against each other before anything runs.
///
/// None of it is a measurement. That the levels are what the movements come to, and that the two
/// recording instants straddle the instant both parties decide at, is arithmetic written on one
/// afternoon; asserted inside a phase it would read as a result.
///
/// The last four are the experiment's own falsifiability condition. Each merge must move exactly
/// one party's answer, and must move it to a pair **neither** party's own record holds — otherwise
/// a phase reporting *a world neither writer produced* would be reporting a property of the
/// arithmetic.
const _: () = assert!(RECORDED_ON[0] <= KNOWN_AT && KNOWN_AT < RECORDED_ON[1]);
const _: () = assert!(OBSERVED_ON <= RECORDED_ON[0] && OBSERVED_ON <= RECORDED_ON[1]);
const _: () = assert!(KNOWN_AT < LATE_ON && LATE_ON <= ADVANCED_TO && ADVANCED_TO < ASKED_AT);

const _: () = assert!(FAITHFUL[0].0 == CLAIMED as i128);
const _: () = assert!(FAITHFUL[0].1 == FUNDED as i128 + CLAIMED as i128);
const _: () = assert!(FAITHFUL[1].1 == FAITHFUL[0].1 - PLANNED as i128);
const _: () = assert!(FAITHFUL[2].1 == FAITHFUL[1].1);
const _: () = assert!(FAITHFUL[3].1 == FAITHFUL[2].1 + LATER as i128);
const _: () = assert!(FLOOR < FAITHFUL[3].1 && FAITHFUL[3].1 < CEILING);
// The two generations must answer differently, or an interrupted write cannot be false.
const _: () = assert!(FAITHFUL[EARLIER_WORLDS - 1].1 != FAITHFUL[FAITHFUL_WORLDS - 1].1);

const _: () = assert!(ALONE[0].0 == CLAIMED as i128 && ALONE[1].0 == 0);
const _: () = assert!(!matching(ALONE[0], ALONE[1]));

const _: () = assert!(matching(MERGED[0][0], ALONE[0]) && matching(MERGED[1][1], ALONE[1]));
const _: () = assert!(!matching(MERGED[0][1], ALONE[0]) && !matching(MERGED[0][1], ALONE[1]));
const _: () = assert!(!matching(MERGED[1][0], ALONE[0]) && !matching(MERGED[1][0], ALONE[1]));

const fn matching(one: (i128, i128), other: (i128, i128)) -> bool {
    one.0 == other.0 && one.1 == other.1
}

/// The three files a repository is made of, held as values rather than as a directory.
///
/// Values because the instrument needs them to be: a mixture puts one state's file over another's,
/// and a phase that had to reach into a live application to get at them could not express one.
pub struct Files {
    pub journal: Vec<Admission>,
    pub lineage: Vec<Taken>,
    pub worlds: Vec<WorldRecord>,
}

/// One of the three, named rather than indexed.
///
/// Restated here rather than borrowed, for the reason the contention subject gives: a subject
/// belongs to the experiment that arranged it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum File {
    Journal,
    Lineage,
    Worlds,
}

impl File {
    pub const ALL: [Self; 3] = [Self::Journal, Self::Lineage, Self::Worlds];
}

/// A party: an agent the base knows, the Event as that party recorded it, and what it decided.
///
/// The observation is held **unadmitted**, so that admitting it is the party's own act and the
/// instant it carries is the party's own claim.
pub struct Side {
    pub label: &'static str,
    pub agent: AgentId,
    pub observed: Admission,
    pub decision: Decision,
    /// What this party's own record answers about its own world, from [`ALONE`].
    pub answers: (i128, i128),
}

/// What the procedure refers to across phases.
pub struct Arranged {
    /// What both parties read: the vocabulary and the two inflows, and no decision.
    pub base: Files,
    /// The first generation of the faithful line, which a whole write replaced.
    pub earlier: Files,
    /// The whole faithful line, and the anchor every later phase compares against.
    pub faithful: Files,
    /// The two writers, in the order [`RECORDED_ON`] and [`ALONE`] name them.
    pub sides: [Side; 2],
    /// Knowledge the faithful line never admitted, recorded on the last day it holds.
    ///
    /// A phase that needs an entry between two occurrences of a readmitted address needs one that
    /// is genuinely new: a readmission of something the witness already holds changes no set, which
    /// is why the ambiguous case cannot be reached with an entry the record already has.
    pub spare: Admission,
    pub instance: ResourceInstanceId,
    pub fund: CommitmentId,
    pub claim: CommitmentId,
    pub plan: CommitmentId,
    pub arrears: CommitmentId,
}

impl Arranged {
    /// The worlds the faithful line decided, by identity, oldest first.
    pub fn faithful_worlds(&self) -> Vec<String> {
        self.faithful
            .worlds
            .iter()
            .map(|world| world.thesis.clone())
            .collect()
    }
}

/// Admit the subject, take the faithful line's four decisions, and hold both parties unadmitted.
///
/// The three states come from one construction because they have to: `earlier` is a prefix of
/// `faithful`, and both extend `base`. An arrangement that built them separately could differ in
/// something no phase asked about.
pub fn arranged() -> Result<Arranged, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let mut constructed = construct(&mut canon)?;

    let base = Files {
        journal: constructed.journal.clone(),
        lineage: Vec::new(),
        worlds: Vec::new(),
    };

    let sides = [
        Side {
            label: "ledger",
            agent: constructed.ledger,
            observed: constructed.observation(RECORDED_ON[0]),
            decision: founding([constructed.fund, constructed.claim].into()),
            answers: ALONE[0],
        },
        Side {
            label: "counterparty",
            agent: constructed.counterparty,
            observed: constructed.observation(RECORDED_ON[1]),
            decision: founding([constructed.claim].into()),
            answers: ALONE[1],
        },
    ];

    constructed.admit(&mut canon, sides[0].observed.clone())?;

    let mut lineage = Lineage::new();
    let mut decisions = Vec::new();

    let founded = Taken::claimed(
        sides[0].decision.clone(),
        constructed.ledger,
        &constructed.admitted,
    )?;
    lineage::decide(canon.history(), &mut lineage, &founded.decision)?;
    decisions.push(founded);

    let earlier = Files {
        journal: constructed.journal.clone(),
        lineage: decisions.clone(),
        worlds: worlds(&lineage),
    };

    constructed.admit(&mut canon, constructed.outflow(PLANNED, 4))?;
    let plan = *constructed.admitted.commitments.last().expect("admitted");

    let proposed = proposing(tip(&lineage)?, [plan].into());
    decisions.push(extend(&mut canon, &mut lineage, &constructed, proposed)?);

    constructed.admit(&mut canon, constructed.inflow(LATER, LATE_ON))?;
    let arrears = *constructed.admitted.commitments.last().expect("admitted");

    let advanced = Decision::Advance {
        extends: tip(&lineage)?,
        known_at: day(ADVANCED_TO),
    };
    decisions.push(extend(&mut canon, &mut lineage, &constructed, advanced)?);

    let introduced = proposing(tip(&lineage)?, [arrears].into());
    decisions.push(extend(&mut canon, &mut lineage, &constructed, introduced)?);

    Ok(Arranged {
        base,
        earlier,
        faithful: Files {
            journal: constructed.journal.clone(),
            lineage: decisions,
            worlds: worlds(&lineage),
        },
        sides,
        spare: constructed.inflow(SPARE, LATE_ON),
        instance: constructed.instance,
        fund: constructed.fund,
        claim: constructed.claim,
        plan,
        arrears,
    })
}

/// Take one more decision on the faithful line, claimed by the party that wrote it.
fn extend(
    canon: &mut Canon<ResidentHistory>,
    lineage: &mut Lineage,
    constructed: &Constructed,
    decision: Decision,
) -> Result<Taken, SubjectError> {
    let taken = Taken::claimed(decision, constructed.ledger, &constructed.admitted)?;

    lineage::decide(canon.history(), lineage, &taken.decision)?;

    Ok(taken)
}

/// The world the last decision produced, which the next one extends.
fn tip(lineage: &Lineage) -> Result<ThesisId, SubjectError> {
    Ok(lineage
        .decided()
        .last()
        .ok_or(SubjectError::NothingDecided)?
        .id())
}

/// A genesis at the instant both parties recognize.
pub fn founding(selection: std::collections::BTreeSet<CommitmentId>) -> Decision {
    Decision::Genesis {
        known_at: day(KNOWN_AT),
        selection,
    }
}

/// A fork that withdraws nothing and proposes one more commitment.
pub fn proposing(
    extends: ThesisId,
    introduced: std::collections::BTreeSet<CommitmentId>,
) -> Decision {
    Decision::Fork {
        extends,
        omitted: [].into(),
        introduced,
    }
}

/// Put the base there the way an application does, so that parties have something to read.
pub fn found(repository: &Repository, arrangement: &Arranged) -> Result<(), RepositoryError> {
    write_whole(repository, &arrangement.base)
}

/// What a party holds before it decides anything.
pub fn read(repository: &Repository) -> Result<Corroborated, SubjectError> {
    Ok(reading::corroborated(repository)?)
}

/// Admit this party's observation and decide about it, against what this party read.
///
/// One call because the two are one act here: what makes a party a writer in this experiment is
/// that it records when it learned something and then decides under that instant.
pub fn observe(working: &mut Corroborated, side: &Side) -> Result<ThesisId, SubjectError> {
    working.journal.push(side.observed.clone());
    journal::replay_remaining(&mut working.canon, &working.journal, &mut working.admitted)?;

    let taken = Taken::claimed(side.decision.clone(), side.agent, &working.admitted)?;

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
        designations: &[],
    }
}

/// Put a whole repository there the way an application does — one call, all or nothing.
pub fn write_whole(repository: &Repository, files: &Files) -> Result<(), RepositoryError> {
    repository.write_whole(input(files))
}

/// Write one file into the live generation, and nothing else.
///
/// Not what an application does. It is how a mixture is reached, and how a record edited from
/// outside is produced — which this experiment uses for calibration and never as evidence.
pub fn put(repository: &Repository, files: &Files, file: File) -> Result<(), RepositoryError> {
    match file {
        File::Journal => repository.write_journal(&files.journal),
        File::Lineage => repository.write_lineage(&files.lineage),
        File::Worlds => repository.write_worlds(&files.worlds),
    }
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
/// when the recognized chain moves, which is the party disagreement; intended moves when the
/// selection moves, which is the line of decisions.
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

/// What every world of a rebuilt repository answers, oldest first.
pub fn read_answers(
    repository: &Repository,
    instance: ResourceInstanceId,
) -> Result<Vec<(i128, i128)>, SubjectError> {
    let Corroborated { canon, lineage, .. } = reading::corroborated(repository)?;

    lineage
        .decided()
        .iter()
        .map(|thesis| answers(canon.history(), thesis, instance))
        .collect::<Result<_, _>>()
        .map_err(SubjectError::from)
}

/// The vocabulary, the two inflows, and the two parties as agents the base already knows.
pub struct Constructed {
    pub fund: CommitmentId,
    pub claim: CommitmentId,
    pub instance: ResourceInstanceId,
    pub ledger: AgentId,
    pub counterparty: AgentId,
    inflow: StatementId,
    outflow: StatementId,
    pub journal: Vec<Admission>,
    pub admitted: Replayed,
}

impl Constructed {
    /// The Event settling the claim, recorded on the day the party learned of it.
    ///
    /// Every field of the identity is fixed, so the two parties' Events are one entry by address
    /// and differ only in the one value no identity contains.
    pub fn observation(&self, recorded_on: u8) -> Admission {
        Admission::Event {
            commitment: self.claim,
            observation: FULFILLING.into(),
            occurred_at: day(OBSERVED_ON),
            recorded_at: day(recorded_on),
        }
    }

    pub fn inflow(&self, magnitude: u128, on: u8) -> Admission {
        Admission::Commitment {
            accountable: self.ledger,
            executors: [self.ledger].into(),
            beneficiaries: [self.counterparty].into(),
            statement: self.inflow,
            resource: self.instance,
            committed_at: day(on),
            due_date: day(ASKED_AT),
            magnitude: Some(magnitude),
            dependencies: [].into(),
            recorded_at: day(on),
        }
    }

    pub fn outflow(&self, magnitude: u128, on: u8) -> Admission {
        Admission::Commitment {
            accountable: self.counterparty,
            executors: [self.counterparty].into(),
            beneficiaries: [self.ledger].into(),
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
            label: "ledger".into(),
            recorded_at: day(1),
        },
        Admission::Agent {
            label: "counterparty".into(),
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
    let (counterparty, ledger) = (admitted.agents[0], admitted.agents[1]);
    let cash = admitted.resources[0];

    journal.extend([
        Admission::Eligibility {
            agent: ledger,
            roles: [payer].into(),
            effective_from: day(1),
            recorded_at: day(1),
        },
        Admission::Eligibility {
            agent: counterparty,
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

    let receiving = |magnitude: u128| Admission::Commitment {
        accountable: ledger,
        executors: [ledger].into(),
        beneficiaries: [counterparty].into(),
        statement: inflow,
        resource: instance,
        committed_at: day(2),
        due_date: day(ASKED_AT),
        magnitude: Some(magnitude),
        dependencies: [].into(),
        recorded_at: day(2),
    };

    journal.extend([receiving(FUNDED), receiving(CLAIMED)]);
    journal::replay_remaining(canon, &journal, &mut admitted)?;

    Ok(Constructed {
        fund: admitted.commitments[0],
        claim: admitted.commitments[1],
        instance,
        ledger,
        counterparty,
        inflow,
        outflow,
        journal,
        admitted,
    })
}

fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
