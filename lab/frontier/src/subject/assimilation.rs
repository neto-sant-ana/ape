//! The assimilation subject: two records founded apart on a shared base, diverged in every family.
//!
//! ```text
//! cash ∈ [0, 1000]
//!
//! base, admitted independently by both and identical by content        12 entries
//!   F   receive 400   committed day 2, recorded day 2
//!   E   Event settling F, occurred day 3, recorded day 3
//!
//! here  adds  Role auditor, Agent third, H receive 150, Eh settling H       16 entries
//! there adds  Role auditor, Role inspector, Agent third, Agent fourth,
//!             Eligibility, Resource grain, ResourceInstance silo,
//!             Action top-up, Statement, G top-up 200, P top-up 90,
//!             Eg settling G                                                 24 entries
//! ```
//!
//! ```text
//!                                       settled  intended
//!   here decides    { F }                  550      550    absorbing what its chain froze
//!   there decides   { F, G }               600      600    its own
//!   there forks     introducing P          600      690    an intention carrying no instant
//!   here retakes    the genesis            750      750    the question
//!   here retakes    the fork               750      840
//! ```
//!
//! Five worlds and no two alike, which is the falsifiability condition: a retaken decision that
//! answered what somebody already answered would let a phase report *the intention crossed* about a
//! world that was already there.
//!
//! # What the two records share after they diverged, and why it is not nothing
//!
//! Both admitted `Role auditor` and `Agent third` — the same content, so the same address, which is
//! the collision experiment's twinning happening after the split rather than before it. Ten of the
//! twelve entries `there` added are therefore new to `here`, and they cover **all nine families**:
//!
//! ```text
//! Role  Agent  Eligibility  Resource  ResourceInstance  Action  Statement  Commitment  Event
//! ```
//!
//! Which is what makes the family table closed. A subject whose divergence was all commitments would
//! report a property of itself.
//!
//! **And the overlap is what keeps the candidate from being an instruction.** If the two shared nothing
//! after the base, what `here` must learn is the whole of `there`'s tail — which is *take everything*,
//! and a phase reporting that as a candidate would be reporting the arrangement. Both degenerate
//! records are built here too, deliberately, so the phase that measures the real one can say which of
//! the three it is looking at.
//!
//! # The chains diverged, and that is what the Event row is about
//!
//! `here` has its own Event, so the two chains are `E → Eh` and `E → Eg`. An `EventId` contains its
//! predecessor, so `Eg` admitted into `here` is not the entry `there` holds — it is another assertion
//! settling the same commitment. The eight other families carry no chain and are predicted not to move,
//! and a third record without an Event of its own is built to measure the case where the chains are
//! still prefix-related.
//!
//! Every quantity is an integer, for the reason [`super::reconstruction`] gives.

use std::collections::{BTreeMap, BTreeSet};

use ape::canon::Canon;
use ape::engine::thesis::{Interpretation, Thesis, ThesisId};
use ape::kernel::entities::{
    ActionId, AgentId, CommitmentId, ResourceId, ResourceInstanceId, RoleId, StatementId,
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

/// The bounds the account answers to, and the bounds of the resource nothing uses.
pub const FLOOR: i128 = 0;
pub const CEILING: i128 = 1000;
pub const GRAIN_CEILING: i128 = 500;

/// What the shared base puts in the account, and what each record adds to it alone.
///
/// Three distinct magnitudes, because the three worlds are told apart by what they answer and two
/// equal ones would collapse a row of the table.
pub const FUNDED: u128 = 400;
pub const HERE_ADDS: u128 = 150;
pub const THERE_ADDS: u128 = 200;

/// What the record being shown also committed and did **not** settle.
///
/// It exists because an intention has two shapes and a fork is the one that carries no instant — and a
/// fork may not omit what history froze (`FrozenPastOmitted`), so the only fork this subject can hold
/// is one that **introduces**. Something open is what there is to introduce.
pub const SPARE_ADDS: u128 = 90;

/// The days the base was laid down on.
pub const VOCABULARY_ON: u8 = 1;
pub const COMMITTED_ON: u8 = 2;
pub const SETTLED_ON: u8 = 3;

/// The days each record's own tail was laid down on, which are the same days in both.
///
/// The same, on purpose: two records that diverged on different days would differ in a second way,
/// and the experiment before this one measured what that costs. Here the only difference is **what**
/// they admitted.
pub const TAIL_ON: u8 = 4;
pub const TAIL_COMMITTED_ON: u8 = 5;
pub const TAIL_SETTLED_ON: u8 = 6;

/// The instant each record's own decision recognizes.
pub const DECIDED_AT: u8 = 9;

/// The day one record is shown the other's material.
///
/// After everything either of them holds, because that is the honest instant: a record learns a
/// foreign fact when it is shown it, and what it may claim is that day and not the other's.
pub const TAKEN_ON: u8 = 15;

/// The instant every reading is taken at, after every due date the subject holds.
pub const ASKED_AT: u8 = 30;

/// The entries each record holds.
pub const BASE_ENTRIES: usize = 12;
pub const HERE_ENTRIES: usize = 16;
pub const THERE_ENTRIES: usize = 24;
pub const ALONE_ENTRIES: usize = 14;
pub const UNBRANCHED_ENTRIES: usize = 15;

/// What the two records admitted alike after they diverged, and what one lacks of the other.
pub const SHARED_AFTER: usize = 2;
pub const LACKING: usize = 10;

/// And what the two degenerate records lack, which is the whole of a tail and none of it.
///
/// Both are producible on purpose. A report whose answer is always one of these two has said *take
/// everything* or *take nothing*, and the phase that reads the real one has to be able to name which
/// of the three it is looking at.
pub const LACKING_FROM_NOTHING: usize = 12;
pub const LACKING_FROM_EVERYTHING: usize = 0;

/// The position the only operation between two records refuses at.
///
/// **Earlier than the overlap**, and that is the arrangement rather than an accident: the two records
/// share two entries after the base, and they did not admit them adjacently — `there` put a Role
/// between them. Sharing is a question about **membership** and `converge::appended` compares by
/// **position**, so a merge stops at the first position two sequences disagree at and never reaches
/// the second thing they have in common.
pub const DIVERGES_AT: usize = 13;

/// Which families keep the address they had, when they cross into a record whose chain has left the
/// base's.
///
/// Eight of nine, and the Event is the exception because an `EventId` contains its predecessor: a
/// chain is an ordering that is part of identity, so two chains that diverged cannot exchange Events
/// by identity. The other eight carry no chain.
pub const CROSSES: [(&str, bool); 9] = [
    ("role", true),
    ("agent", true),
    ("eligibility", true),
    ("resource", true),
    ("resource-instance", true),
    ("action", true),
    ("statement", true),
    ("commitment", true),
    ("event", false),
];

const _: () = assert!(CROSSES.len() == FAMILIES);

/// The kinds of entry a journal can hold, every one of which the lacking entries cover.
///
/// Ten entries and nine families, because the record being shown committed twice: what closes the
/// table is the families covered, and the phase derives that from the entries rather than from here.
pub const FAMILIES: usize = 9;

/// What each world answers on the cash account: settled, then intended.
///
/// The pair, because the fork moves only the second — it introduces an open commitment and freezes
/// nothing. Five worlds and no two alike, which is the falsifiability condition: a retaken decision
/// answering what somebody already answered would let a phase report *the intention crossed* about a
/// world that was already there.
pub const HERE_DECIDES: (i128, i128) = ((FUNDED + HERE_ADDS) as i128, (FUNDED + HERE_ADDS) as i128);
pub const THERE_DECIDES: [(i128, i128); 2] = [
    ((FUNDED + THERE_ADDS) as i128, (FUNDED + THERE_ADDS) as i128),
    (
        (FUNDED + THERE_ADDS) as i128,
        (FUNDED + THERE_ADDS + SPARE_ADDS) as i128,
    ),
];

/// The same two decisions, retaken in the record that was shown them.
///
/// Higher than either, and not because anything was added twice: a genesis absorbs whatever its cut
/// froze, so a decision taken here picks up **this** record's own settled commitment whether or not the
/// intention named it.
pub const RETAKEN: [(i128, i128); 2] = [
    (
        (FUNDED + HERE_ADDS + THERE_ADDS) as i128,
        (FUNDED + HERE_ADDS + THERE_ADDS) as i128,
    ),
    (
        (FUNDED + HERE_ADDS + THERE_ADDS) as i128,
        (FUNDED + HERE_ADDS + THERE_ADDS + SPARE_ADDS) as i128,
    ),
];

/// The literals above, weighed against each other before anything runs.
const _: () = assert!(FLOOR < FUNDED as i128 && GRAIN_CEILING < CEILING);
const _: () = assert!(HERE_ADDS != THERE_ADDS && SPARE_ADDS != HERE_ADDS);
const _: () = assert!(RETAKEN[1].1 < CEILING, "the account holds all of it");
const _: () = assert!(!alike(HERE_DECIDES, THERE_DECIDES[0]));
const _: () = assert!(!alike(THERE_DECIDES[0], THERE_DECIDES[1]));
const _: () = assert!(!alike(RETAKEN[0], RETAKEN[1]));
const _: () = assert!(!alike(RETAKEN[0], HERE_DECIDES) && !alike(RETAKEN[0], THERE_DECIDES[0]));
const _: () = assert!(!alike(RETAKEN[1], THERE_DECIDES[1]) && !alike(RETAKEN[1], HERE_DECIDES));

const fn alike(one: (i128, i128), other: (i128, i128)) -> bool {
    one.0 == other.0 && one.1 == other.1
}

const _: () = assert!(VOCABULARY_ON <= COMMITTED_ON && COMMITTED_ON <= SETTLED_ON);
const _: () = assert!(SETTLED_ON < TAIL_ON && TAIL_ON <= TAIL_COMMITTED_ON);
const _: () = assert!(TAIL_COMMITTED_ON <= TAIL_SETTLED_ON);
// Each record decides after its own tail and before it is shown anything.
const _: () = assert!(TAIL_SETTLED_ON < DECIDED_AT && DECIDED_AT < TAKEN_ON);
const _: () = assert!(TAKEN_ON < ASKED_AT);

const _: () = assert!(HERE_ENTRIES == BASE_ENTRIES + 4);
const _: () = assert!(THERE_ENTRIES == BASE_ENTRIES + 12);
const _: () = assert!(ALONE_ENTRIES == BASE_ENTRIES + 2);
const _: () = assert!(UNBRANCHED_ENTRIES == BASE_ENTRIES + 3);
// What is left after the overlap, rather than a count somebody chose.
const _: () = assert!(LACKING == THERE_ENTRIES - BASE_ENTRIES - SHARED_AFTER);
const _: () = assert!(LACKING > FAMILIES, "one family arrives twice");
// The two degenerate cases the arrangement must also be able to produce, or the phase that measures
// the real one cannot say which of the three it is looking at.
const _: () = assert!(
    SHARED_AFTER > 0,
    "an overlap, or the candidate is an instruction"
);
const _: () = assert!(LACKING_FROM_NOTHING == THERE_ENTRIES - BASE_ENTRIES);
const _: () = assert!(LACKING_FROM_NOTHING == LACKING + SHARED_AFTER);
const _: () = assert!(LACKING_FROM_EVERYTHING == 0);
const _: () = assert!(BASE_ENTRIES <= DIVERGES_AT && DIVERGES_AT < BASE_ENTRIES + SHARED_AFTER);

/// The three files a repository is made of, held as values rather than as a directory.
pub struct Files {
    pub journal: Vec<Admission>,
    pub lineage: Vec<Taken>,
    pub worlds: Vec<WorldRecord>,
}

/// What the procedure refers to across phases.
pub struct Arranged {
    /// The record the report is asked in the frame of.
    pub here: Files,
    /// The record it is shown.
    pub there: Files,
    /// A record holding everything `there` holds, which has nothing to learn.
    pub holding_everything: Files,
    /// A record sharing only the base, which would have to learn all of it.
    pub holding_nothing: Files,
    /// A record whose chain never left the base's, so `there`'s Event is still prefix-related.
    pub unbranched: Files,
    /// The addresses both records produced from the base, derived rather than listed.
    pub shared: Vec<EntryId>,
    pub account: ResourceInstanceId,
    pub fund: CommitmentId,
    /// What each record committed alone: `here`'s, then `there`'s.
    pub alone: [CommitmentId; 2],
    /// The party the record being shown claims, which is an agent only it admitted.
    pub claimed_there: AgentId,
}

impl Arranged {
    /// The world one of `there`'s decisions produced, by identity, in `there`.
    pub fn decided_there(&self, position: usize) -> &WorldRecord {
        &self.there.worlds[position]
    }
}

/// Found both records, let each decide alone, and build the three records the phases compare against.
pub fn arranged() -> Result<Arranged, SubjectError> {
    let here = founded(Tail::Here)?;
    let there = founded(Tail::There)?;
    let everything = founded(Tail::There)?;
    let nothing = founded(Tail::HereAlone)?;
    let unbranched = founded(Tail::HereUnbranched)?;

    let shared = addresses(&here.journal)?[..BASE_ENTRIES].to_vec();

    Ok(Arranged {
        here: kept(&here.journal, &[decided(&here, &[here.own])?])?,
        there: kept(&there.journal, &intended_by(&there, &there.journal)?)?,
        holding_everything: kept(&everything.journal, &[decided(&everything, &[])?])?,
        holding_nothing: kept(&nothing.journal, &[decided(&nothing, &[])?])?,
        unbranched: kept(&unbranched.journal, &[decided(&unbranched, &[])?])?,
        shared,
        account: here.account,
        fund: here.fund,
        alone: [here.own, there.own],
        claimed_there: there.party.ok_or(SubjectError::NothingDecided)?,
    })
}

/// One record's own decision: a genesis at its own instant, selecting the fund and whatever it added.
fn decided(founded: &Founded, also: &[CommitmentId]) -> Result<Taken, SubjectError> {
    let mut selection: BTreeSet<CommitmentId> = [founded.fund].into();
    selection.extend(also.iter().copied());

    let decision = Decision::Genesis {
        known_at: day(DECIDED_AT),
        selection,
    };

    Ok(match founded.party {
        Some(party) => Taken::claimed(decision, party, &founded.admitted)?,
        None => Taken::now(decision, &founded.admitted)?,
    })
}

/// The record that is shown: a genesis, then a fork of it that introduces what it left open.
///
/// Two decisions rather than one, because the question is whether an **intention** crosses and an
/// intention has two shapes. A genesis carries an instant; a fork carries none at all — its whole
/// content is *these out, those in*, under the cut it inherits. Which of the two survives being
/// retaken elsewhere is the experiment, and a lineage with only one of them could not ask.
fn intended_by(founded: &Founded, journal: &[Admission]) -> Result<Vec<Taken>, SubjectError> {
    let genesis = decided(founded, &[founded.own])?;

    let mut canon = Canon::new(ResidentHistory::new());
    let (lineage, _) = lineage::rebuild(&mut canon, journal, std::slice::from_ref(&genesis))?;

    let extends = lineage
        .decided()
        .first()
        .ok_or(SubjectError::NothingDecided)?
        .id();

    let fork = Decision::Fork {
        extends,
        omitted: [].into(),
        introduced: founded.open_own.into_iter().collect(),
    };

    let forked = match founded.party {
        Some(party) => Taken::claimed(fork, party, &founded.admitted)?,
        None => Taken::now(fork, &founded.admitted)?,
    };

    Ok(vec![genesis, forked])
}

/// One entry a record lacks, and the address it had where it came from.
///
/// The second half is what the family table needs: whether an entry crosses **by identity** is a
/// comparison between the address it had there and the address admitting it produces here, and a
/// phase that only held the entry would have nothing to compare.
pub struct Lacking {
    pub entry: Admission,
    pub there: EntryId,
}

/// Every entry one record holds that another does not, in the order the first admitted them.
///
/// By address, which is what makes the overlap derived rather than arranged: two records that admitted
/// the same content hold the same address, and the collision experiment measured that this needs no
/// operation between them.
pub fn lacking(here: &[Admission], there: &[Admission]) -> Result<Vec<Lacking>, JournalError> {
    let held: BTreeSet<EntryId> = addresses(here)?.into_iter().collect();
    let theirs = addresses(there)?;

    Ok(there
        .iter()
        .zip(theirs)
        .filter(|(_, address)| !held.contains(address))
        .map(|(entry, there)| Lacking {
            entry: entry.clone(),
            there,
        })
        .collect())
}

/// The same entries, recorded at the instant the record being shown them can claim.
///
/// The whole of what *taking* means, in one function: the content crosses and the instant does not.
/// An entry admitted with the other record's instant would be a copy, and the record would be
/// claiming a past it did not have.
pub fn taken(lacking: &[Lacking], at: u8) -> Vec<Admission> {
    lacking
        .iter()
        .map(|held| dated(&held.entry, &day(at)))
        .collect()
}

/// When a record learns another's material, and what instant a retaken decision names.
///
/// Two instants rather than one, and they are separate on purpose: the first is what a record can
/// honestly claim about its own holding, and the second is what a decision says it recognized. A phase
/// that could not tell them apart could not ask what happens when a record keeps the original's
/// instant instead of its own.
pub struct Retake {
    pub learned_at: u8,
    pub known_at: u8,
}

/// What one record would have to do to hold what another decided: learn these, then decide those.
///
/// The candidate, in the laboratory. It is an instrument and not a proposal — whether the application
/// should offer one is Part B, decided by the phases.
pub struct Retaken {
    pub learn: Vec<Admission>,
    pub decide: Vec<Taken>,
}

/// Translate another record's lineage into decisions this one takes.
///
/// Nothing is carried. Each decision is rebuilt in this record's frame: a genesis names the instant
/// **this** record can claim, and a fork names the world this record produced for the one the original
/// extended. What crosses is the intention — a selection, an omission, an introduction — and the
/// coordinate, the witness and the instant are this record's own, written by [`Taken::now`] from the
/// prefix that actually stood.
///
/// The mapping from their worlds to ours is built as it goes, because it can only be: a world's
/// identity is derived from the cut and the selection it came out with, so what their second decision
/// extends has no name here until their first has been retaken.
pub fn retaking(here: &Files, there: &Files, retake: Retake) -> Result<Retaken, SubjectError> {
    let learn = taken(&lacking(&here.journal, &there.journal)?, retake.learned_at);

    let mut journal = here.journal.clone();
    journal.extend(learn.iter().cloned());

    let mut canon = Canon::new(ResidentHistory::new());
    let (mut lineage, admitted) = lineage::rebuild(&mut canon, &journal, &here.lineage)?;

    let theirs = rebuilt(there)?;
    let mut translated: BTreeMap<ThesisId, ThesisId> = BTreeMap::new();
    let mut decide = Vec::new();

    for (position, held) in there.lineage.iter().enumerate() {
        let extended = |extends: &ThesisId| -> Result<ThesisId, SubjectError> {
            translated
                .get(extends)
                .copied()
                .ok_or(SubjectError::NothingDecided)
        };

        let decision = match &held.decision {
            Decision::Genesis { selection, .. } => Decision::Genesis {
                known_at: day(retake.known_at),
                selection: selection.clone(),
            },
            Decision::Advance { extends, known_at } => Decision::Advance {
                extends: extended(extends)?,
                known_at: known_at.clone(),
            },
            Decision::Fork {
                extends,
                omitted,
                introduced,
            } => Decision::Fork {
                extends: extended(extends)?,
                omitted: omitted.clone(),
                introduced: introduced.clone(),
            },
        };

        let retaken = Taken::now(decision, &admitted)?;
        lineage::decide(canon.history(), &mut lineage, &retaken.decision)?;

        translated.insert(
            theirs
                .lineage
                .decided()
                .get(position)
                .ok_or(SubjectError::NothingDecided)?
                .id(),
            lineage
                .decided()
                .last()
                .ok_or(SubjectError::NothingDecided)?
                .id(),
        );

        decide.push(retaken);
    }

    Ok(Retaken { learn, decide })
}

/// The whole record a receiver would keep, having taken what it was shown.
pub fn having_taken(here: &Files, retaken: &Retaken) -> Result<Files, SubjectError> {
    let mut journal = here.journal.clone();
    journal.extend(retaken.learn.iter().cloned());

    let mut decisions = here.lineage.clone();
    decisions.extend(retaken.decide.iter().cloned());

    kept(&journal, &decisions)
}

/// The same admission, recorded at another instant.
pub fn dated(admission: &Admission, at: &str) -> Admission {
    let mut moved = admission.clone();

    match &mut moved {
        Admission::Role { recorded_at, .. }
        | Admission::Agent { recorded_at, .. }
        | Admission::Eligibility { recorded_at, .. }
        | Admission::Resource { recorded_at, .. }
        | Admission::ResourceInstance { recorded_at, .. }
        | Admission::Action { recorded_at, .. }
        | Admission::Statement { recorded_at, .. }
        | Admission::Commitment { recorded_at, .. }
        | Admission::Event { recorded_at, .. } => *recorded_at = at.to_owned(),
    }

    moved
}

/// Which family an admission belongs to, so the table can be closed over all nine.
pub fn family(admission: &Admission) -> &'static str {
    match admission {
        Admission::Role { .. } => "role",
        Admission::Agent { .. } => "agent",
        Admission::Eligibility { .. } => "eligibility",
        Admission::Resource { .. } => "resource",
        Admission::ResourceInstance { .. } => "resource-instance",
        Admission::Action { .. } => "action",
        Admission::Statement { .. } => "statement",
        Admission::Commitment { .. } => "commitment",
        Admission::Event { .. } => "event",
    }
}

/// Every address a journal produces, in order.
pub fn addresses(journal: &[Admission]) -> Result<Vec<EntryId>, JournalError> {
    let mut canon = Canon::new(ResidentHistory::new());

    Ok(journal::replay(&mut canon, journal)?.entries)
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

/// What one world answers on the account: what has settled, and what it intends.
///
/// The pair, because the two move for different reasons and this subject needs both: settled moves
/// when the recognized chain moves, which is what learning another record's Event does; intended moves
/// when the selection moves, which is what a fork that introduces does.
pub fn answered(
    history: &ResidentHistory,
    thesis: &Thesis,
    account: ResourceInstanceId,
) -> Result<(i128, i128), ReadingError> {
    let interpretation = Interpretation::of(thesis, history)?;
    let projected = interpretation.conditions_at(&asked_at())?;

    Ok((
        level::settled(history, &projected, account)?,
        level::intended(history, &projected, account)?,
    ))
}

/// What each record added after the base.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tail {
    /// Two entries the other record also admitted, a commitment and an Event of its own.
    Here,
    /// One of every family, two of which the other record also admitted.
    There,
    /// The commitment and the Event alone: no overlap at all.
    HereAlone,
    /// The overlap and the commitment, and no Event — so the chain never leaves the base's.
    HereUnbranched,
}

/// A record's whole journal, and what admitting it produced.
pub struct Founded {
    pub journal: Vec<Admission>,
    pub admitted: Replayed,
    pub fund: CommitmentId,
    pub account: ResourceInstanceId,
    /// What this record committed that the other did not, and settled.
    pub own: CommitmentId,
    /// And what it committed and left open, where it has one.
    pub open_own: Option<CommitmentId>,
    /// The party this record's decisions claim, where it has one of its own to claim.
    ///
    /// The record being shown claims an agent **only it admitted**, which is what makes the
    /// prediction about provenance falsifiable: an `AgentId` is content-addressed, so once the other
    /// record has learned that agent it holds everything it would need to say the same thing. Whether
    /// it may is the question, and a decision claiming nobody could not ask it.
    pub party: Option<AgentId>,
}

/// Everything the base admitted, held so a tail can refer to it by identity.
struct Base {
    payer: RoleId,
    payee: RoleId,
    one: AgentId,
    other: AgentId,
    cash: ResourceId,
    account: ResourceInstanceId,
    inflow: StatementId,
}

/// Lay down the shared base, then whichever tail this record added.
pub fn founded(tail: Tail) -> Result<Founded, JournalError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let mut journal = Vec::new();
    let mut admitted = Replayed::default();

    let mut admit = |journal: &mut Vec<Admission>,
                     admitted: &mut Replayed,
                     entries: Vec<Admission>|
     -> Result<(), JournalError> {
        journal.extend(entries);
        journal::replay_remaining(&mut canon, journal, admitted)
    };

    admit(
        &mut journal,
        &mut admitted,
        vec![
            role("payer", VOCABULARY_ON),
            role("payee", VOCABULARY_ON),
            agent("one", VOCABULARY_ON),
            agent("other", VOCABULARY_ON),
            Admission::Resource {
                label: "cash".into(),
                kind: ResourceKindRecord::Between {
                    lower: FLOOR,
                    upper: CEILING,
                },
                recorded_at: day(VOCABULARY_ON),
            },
        ],
    )?;

    let base = Base {
        payer: admitted.roles[0],
        payee: admitted.roles[1],
        one: admitted.agents[0],
        other: admitted.agents[1],
        cash: admitted.resources[0],
        account: ResourceInstanceId::from([0; 32]),
        inflow: StatementId::from([0; 32]),
    };

    admit(
        &mut journal,
        &mut admitted,
        vec![
            eligibility(base.one, base.payer, VOCABULARY_ON),
            eligibility(base.other, base.payee, VOCABULARY_ON),
            Admission::ResourceInstance {
                label: "account".into(),
                resource: base.cash,
                recorded_at: day(VOCABULARY_ON),
            },
            action("receive", base.cash, VOCABULARY_ON),
        ],
    )?;

    let base = Base {
        account: admitted.instances[0],
        ..base
    };

    let receive = admitted.actions[0];

    admit(
        &mut journal,
        &mut admitted,
        vec![statement_at(base.payer, base.payee, receive, VOCABULARY_ON)],
    )?;

    let base = Base {
        inflow: admitted.statements[0],
        ..base
    };

    admit(
        &mut journal,
        &mut admitted,
        vec![commitment(Committed {
            accountable: base.one,
            beneficiary: base.other,
            statement: base.inflow,
            instance: base.account,
            magnitude: FUNDED,
            on: COMMITTED_ON,
        })],
    )?;

    let fund = admitted.commitments[0];

    admit(
        &mut journal,
        &mut admitted,
        vec![settling(fund, SETTLED_ON, SETTLED_ON)],
    )?;

    // The tail is admitted in groups rather than as one list, because each group refers to the one
    // before it by identity: a Statement names an Action, and an Action has no identity until it has
    // been admitted. It is inline for the same reason — a function returning the groups could not name
    // what its own earlier groups produced.
    let here = |on: u8| {
        commitment(Committed {
            accountable: base.one,
            beneficiary: base.other,
            statement: base.inflow,
            instance: base.account,
            magnitude: HERE_ADDS,
            on,
        })
    };
    let overlap = vec![role("auditor", TAIL_ON), agent("third", TAIL_ON)];

    match tail {
        Tail::HereAlone => {
            admit(&mut journal, &mut admitted, vec![here(TAIL_COMMITTED_ON)])?;
        }

        Tail::HereUnbranched => {
            admit(&mut journal, &mut admitted, overlap)?;
            admit(&mut journal, &mut admitted, vec![here(TAIL_COMMITTED_ON)])?;
        }

        Tail::Here => {
            admit(&mut journal, &mut admitted, overlap)?;
            admit(&mut journal, &mut admitted, vec![here(TAIL_COMMITTED_ON)])?;
        }

        Tail::There => {
            admit(
                &mut journal,
                &mut admitted,
                vec![
                    role("auditor", TAIL_ON),
                    role("inspector", TAIL_ON),
                    agent("third", TAIL_ON),
                    agent("fourth", TAIL_ON),
                ],
            )?;

            let (inspector, fourth) = (admitted.roles[3], admitted.agents[3]);

            admit(
                &mut journal,
                &mut admitted,
                vec![
                    eligibility(fourth, inspector, TAIL_ON),
                    Admission::Resource {
                        label: "grain".into(),
                        kind: ResourceKindRecord::Between {
                            lower: FLOOR,
                            upper: GRAIN_CEILING,
                        },
                        recorded_at: day(TAIL_ON),
                    },
                ],
            )?;

            let grain = admitted.resources[1];

            admit(
                &mut journal,
                &mut admitted,
                vec![
                    Admission::ResourceInstance {
                        label: "silo".into(),
                        resource: grain,
                        recorded_at: day(TAIL_ON),
                    },
                    action("top-up", base.cash, TAIL_ON),
                ],
            )?;

            let topping_up = admitted.actions[1];

            admit(
                &mut journal,
                &mut admitted,
                vec![statement_at(inspector, base.payee, topping_up, TAIL_ON)],
            )?;

            let topped_up = admitted.statements[1];
            let topping = |magnitude: u128| {
                commitment(Committed {
                    accountable: fourth,
                    beneficiary: base.other,
                    statement: topped_up,
                    instance: base.account,
                    magnitude,
                    on: TAIL_COMMITTED_ON,
                })
            };

            // Two commitments and only the first is settled below, which is what gives the record
            // being shown an intention in both shapes: a genesis over what history froze, and a fork
            // that introduces what it did not.
            admit(
                &mut journal,
                &mut admitted,
                vec![topping(THERE_ADDS), topping(SPARE_ADDS)],
            )?;
        }
    }

    let own = admitted.commitments[1];
    let open_own = admitted.commitments.get(2).copied();
    let party = (tail == Tail::There).then(|| admitted.agents[3]);

    // The settling Event, where this record has one. Without it the chain never leaves the base's,
    // which is the case the Event row of the family table needs on the other side.
    if tail != Tail::HereUnbranched {
        admit(
            &mut journal,
            &mut admitted,
            vec![settling(own, TAIL_SETTLED_ON, TAIL_SETTLED_ON)],
        )?;
    }

    Ok(Founded {
        journal,
        admitted,
        fund,
        account: base.account,
        own,
        open_own,
        party,
    })
}

fn role(label: &str, on: u8) -> Admission {
    Admission::Role {
        label: label.into(),
        recorded_at: day(on),
    }
}

fn agent(label: &str, on: u8) -> Admission {
    Admission::Agent {
        label: label.into(),
        recorded_at: day(on),
    }
}

fn eligibility(agent: AgentId, role: RoleId, on: u8) -> Admission {
    Admission::Eligibility {
        agent,
        roles: [role].into(),
        effective_from: day(VOCABULARY_ON),
        recorded_at: day(on),
    }
}

fn action(verb: &str, resource: ResourceId, on: u8) -> Admission {
    Admission::Action {
        verb: verb.into(),
        kind: ActionKindRecord::Quantifiable(EffectRecord::Increase),
        resource,
        recorded_at: day(on),
    }
}

fn statement_at(actor: RoleId, recipient: RoleId, action: ActionId, on: u8) -> Admission {
    Admission::Statement {
        actors: [actor].into(),
        recipients: [recipient].into(),
        action,
        fulfills: [FULFILLING.to_owned()].into(),
        cancels: [CANCELLING.to_owned()].into(),
        recorded_at: day(on),
    }
}

/// What a commitment in this subject is made of, named rather than positional.
pub struct Committed {
    pub accountable: AgentId,
    pub beneficiary: AgentId,
    pub statement: StatementId,
    pub instance: ResourceInstanceId,
    pub magnitude: u128,
    pub on: u8,
}

fn commitment(committed: Committed) -> Admission {
    Admission::Commitment {
        accountable: committed.accountable,
        executors: [committed.accountable].into(),
        beneficiaries: [committed.beneficiary].into(),
        statement: committed.statement,
        resource: committed.instance,
        committed_at: day(committed.on),
        due_date: day(ASKED_AT),
        magnitude: Some(committed.magnitude),
        dependencies: [].into(),
        recorded_at: day(committed.on),
    }
}

fn settling(commitment: CommitmentId, occurred: u8, recorded: u8) -> Admission {
    Admission::Event {
        commitment,
        observation: FULFILLING.into(),
        occurred_at: day(occurred),
        recorded_at: day(recorded),
    }
}

pub fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}

/// The world a lineage decided, by identity, at a position.
pub fn world_at(rebuilt: &Rebuilt, position: usize) -> Option<ThesisId> {
    rebuilt.lineage.decided().get(position).map(|w| w.id())
}
