//! The indexicality subject: one decision, two journals that satisfy its witness, and a record
//! founded apart that holds neither.
//!
//! ```text
//! cash ∈ [0, 1000]
//!
//! F   receive 400   committed day 2, recorded day 2
//! E   Event settling F, occurred day 3, recorded day 4 in one journal and day 9 in the other
//!
//! D   Genesis { known_at: day 6, selection: { F } }, taken after E
//! ```
//!
//! The two journals hold the same entries by address — no identity carries a recording instant —
//! so one witness is satisfied by both. A cut resolves its Event head from **recorded** instants,
//! and `day 6` falls between the two, so the same decision selects a settled fund in one journal
//! and an open one in the other.
//!
//! ```text
//!                           entries  event head at day 6   settled  intended
//!   early                      12    E                        400      400
//!   late                       12    none                       0      400
//!   reordered-early            12    E                        400      400
//!   reordered-late             12    none                       0      400
//!   inserted-early             13    E                        400      400
//! ```
//!
//! # The five candidates are the space a pin is measured against
//!
//! A pin determines the reference when **every** journal satisfying it offers the same knowledge.
//! That is a claim about a family, so the family is arranged rather than argued, and each member
//! differs from `early` in exactly one nameable way:
//!
//! ```text
//! late              one recording instant, and no address moves           (11's finding)
//! reordered-*       two independent entries swapped: the same addresses
//!                   in a different sequence, which no witness can hold
//! inserted-early    a Commitment nothing selects, admitted before the
//!                   coordinate — different knowledge, identical world     (12's finding)
//! ```
//!
//! The last is what keeps the final stage honest. A pin that names the world a decision produced
//! is satisfied by a journal whose knowledge is not the one the decision was taken against, so
//! *produces the same world* cannot stand in for *resolves correctly* — which is the distinction
//! the protocol settled before any of this ran.
//!
//! # The foreign record is founded apart in content, not merely in place
//!
//! Two records that admit the same content hold the same addresses, whoever wrote them — the
//! collision experiment measured it. So a receiver that shares no address has to share no
//! *content*: its own vocabulary, its own parties, its own fund. That it holds neither journal is
//! then a property of what it admitted rather than of where it sits, and the phase that uses it
//! verifies the intersection is empty instead of assuming it.
//!
//! Every quantity is an integer, for the reason [`super::reconstruction`] gives.

use std::collections::{BTreeMap, BTreeSet};

use ape::canon::Canon;
use ape::engine::thesis::{Interpretation, Thesis};
use ape::kernel::entities::{AgentId, CommitmentId, ResourceInstanceId};
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

/// What the fund puts in the account, and what the foreign record's own fund puts in its own.
///
/// Different, so that the two records are not twins by content in the one place they could be.
pub const FUNDED: u128 = 400;
pub const FOREIGN_FUNDED: u128 = 250;

/// The day the fund was committed, and the day it was observed to have settled.
pub const COMMITTED_ON: u8 = 2;
pub const OCCURRED_ON: u8 = 3;

/// The two days the settling Event was **recorded** on, one per journal.
///
/// The whole arrangement is this pair and the instant between them. Nothing else about the two
/// journals differs, and no address moves.
pub const LEARNED_ON: [u8; 2] = [4, 9];

/// The day the harmless Commitment is recorded on, which is before either journal learned of `E`.
pub const INSERTED_ON: u8 = 3;

/// A third day the Event could have been learned on, on the same side of the cut as the first.
///
/// The instant moves and no world does, which is the state the record has nothing to say about:
/// what a record weighs a recording instant against is a world it derived from one, so an instant
/// that moves without moving a world is weighed against nothing at all.
pub const RELEARNED_ON: u8 = 5;

/// The instant the decision recognizes.
pub const KNOWN_AT: u8 = 6;

/// The instant every reading is taken at, after every due date the subject holds.
pub const ASKED_AT: u8 = 20;

/// Where a guard moves a recording instant to when it is mutating one.
///
/// Later than everything, so that moving the last entry of any prefix stays admissible: recording
/// is monotonic across admission, and a mutation refused for arriving out of order would be
/// measuring the watermark rather than the address.
pub const MOVED_TO: u8 = 28;

/// The entries each journal holds.
pub const ENTRIES: usize = 12;
pub const INSERTED_ENTRIES: usize = 13;
pub const FOREIGN_ENTRIES: usize = 12;

/// The kinds of entry a journal can hold, every one of which the base journal contains.
pub const KINDS: usize = 9;

/// The positions the two reordered journals swap, which are two Roles that refer to nothing.
pub const SWAPPED: [usize; 2] = [0, 1];

/// What the decision answers where the Event was learned early, and where it was learned late.
///
/// Settled, then intended. The fund is the same commitment in both and the same decision selects
/// it; what moves is whether the cut recognizes the Event that settled it.
pub const WHEN_EARLY: (i128, i128) = (FUNDED as i128, FUNDED as i128);
pub const WHEN_LATE: (i128, i128) = (0, FUNDED as i128);

/// The candidate journals, in the order [`Arranged::candidates`] holds them.
pub const CANDIDATES: [&str; 5] = [
    "early",
    "late",
    "reordered-early",
    "reordered-late",
    "inserted-early",
];

/// What each candidate answers, and how many entries it holds.
pub const CANDIDATE_ANSWERS: [(i128, i128); 5] =
    [WHEN_EARLY, WHEN_LATE, WHEN_EARLY, WHEN_LATE, WHEN_EARLY];
pub const CANDIDATE_ENTRIES: [usize; 5] = [ENTRIES, ENTRIES, ENTRIES, ENTRIES, INSERTED_ENTRIES];

/// The pin, completed one stage at a time, and what each stage is expected to leave undecided.
///
/// Written before the run: how many of the five candidates satisfy the pin at that stage, how many
/// distinct bodies of knowledge those candidates offer, and how many distinct worlds they produce.
///
/// A stage **determines the reference** when the middle number is one. A stage that leaves it above
/// one while the last number is one is a stage that fixes the answer and not the question, which is
/// the distinction this experiment exists to keep.
pub const STAGES: [(&str, usize, usize, usize); 4] = [
    ("coordinate", 5, 3, 2),
    ("witnessed", 4, 2, 2),
    ("dated", 2, 1, 1),
    ("produced", 2, 1, 1),
];

/// The literals above, weighed against each other before anything runs.
///
/// Arithmetic rather than measurement, for the reason the atomicity subject gives: asserted inside
/// a phase this would read as a result.
const _: () = assert!(FLOOR < FUNDED as i128 && (FUNDED as i128) < CEILING);
const _: () = assert!(FLOOR < FOREIGN_FUNDED as i128 && FOREIGN_FUNDED != FUNDED);
const _: () = assert!(COMMITTED_ON <= OCCURRED_ON);
const _: () = assert!(OCCURRED_ON <= LEARNED_ON[0] && LEARNED_ON[0] < LEARNED_ON[1]);
// The hinge, and the whole of the arrangement: the instant the decision names falls between the
// two days the Event was learned on, so a cut resolves a different head in each journal.
const _: () = assert!(LEARNED_ON[0] <= KNOWN_AT && KNOWN_AT < LEARNED_ON[1]);
// And the fund is selectable at that cut in both, so what moves is the chain and never the
// selection: a decision refused for naming a commitment it could not know would measure nothing.
const _: () = assert!(COMMITTED_ON <= KNOWN_AT && KNOWN_AT < ASKED_AT);
const _: () = assert!(COMMITTED_ON <= INSERTED_ON && INSERTED_ON <= LEARNED_ON[0]);
const _: () = assert!(LEARNED_ON[0] < RELEARNED_ON && RELEARNED_ON <= KNOWN_AT);
const _: () = assert!(LEARNED_ON[1] < MOVED_TO);

const _: () = assert!(!matching(WHEN_EARLY, WHEN_LATE));
const _: () = assert!(WHEN_EARLY.1 == WHEN_LATE.1, "only the settled level moves");
const _: () = assert!(INSERTED_ENTRIES == ENTRIES + 1);
const _: () = assert!(SWAPPED[0] < SWAPPED[1] && SWAPPED[1] < ENTRIES);

const _: () = assert!(STAGES[0].1 == CANDIDATES.len());
const _: () = assert!(STAGES[0].1 >= STAGES[1].1 && STAGES[1].1 >= STAGES[2].1);
const _: () = assert!(STAGES[2].1 == STAGES[3].1);
// The dated stage is the one that determines, and the stage after it buys nothing.
const _: () = assert!(STAGES[1].2 > 1 && STAGES[2].2 == 1);
const _: () = assert!(STAGES[3].2 == STAGES[2].2 && STAGES[3].3 == STAGES[2].3);

const fn matching(one: (i128, i128), other: (i128, i128)) -> bool {
    one.0 == other.0 && one.1 == other.1
}

/// How much of a reference has been written down.
///
/// Cumulative, because the question is what each addition buys on top of what the record already
/// holds. The first two are what a [`Taken`] carries today; the last two are the laboratory's, and
/// are an instrument rather than a proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    /// The address of the entry the decision was taken after.
    Coordinate,
    /// And every address that stood with it.
    Witnessed,
    /// And the instant each of those was recorded at.
    Dated,
    /// And the world the decision produced there.
    Produced,
}

impl Stage {
    pub const ALL: [Self; 4] = [
        Self::Coordinate,
        Self::Witnessed,
        Self::Dated,
        Self::Produced,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Self::Coordinate => "coordinate",
            Self::Witnessed => "witnessed",
            Self::Dated => "dated",
            Self::Produced => "produced",
        }
    }
}

/// A reference to knowledge, carried apart from the journal it refers to.
///
/// Every field is a value a reader could be handed: an address, a set of addresses, a map from
/// address to instant, an identity. None of them is content, which is why a pin is a *description*
/// of a journal rather than a copy of one — and why a receiver has to already be able to write the
/// journal the description names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pin {
    pub after: EntryId,
    pub witness: Option<BTreeSet<EntryId>>,
    pub recorded: Option<BTreeMap<EntryId, String>>,
    pub produced: Option<String>,
}

impl Pin {
    /// The pin a decision carries at one stage, read off the journal it was taken in.
    pub fn at(
        stage: Stage,
        taken: &Taken,
        journal: &[Admission],
        instance: ResourceInstanceId,
    ) -> Result<Self, SubjectError> {
        let selected = selects(journal, &taken.after).ok_or(SubjectError::NothingAdmitted)?;

        Ok(Self {
            after: taken.after.clone(),
            witness: (stage != Stage::Coordinate).then(|| taken.witness.clone()),
            recorded: matches!(stage, Stage::Dated | Stage::Produced)
                .then(|| selected.into_iter().collect()),
            produced: match stage {
                Stage::Produced => Some(
                    resolve(journal, &taken.after, &taken.decision, instance)?
                        .thesis
                        .id()
                        .to_string(),
                ),
                _ => None,
            },
        })
    }

    /// Whether a journal is one of those this pin describes.
    ///
    /// Every part is weighed against something the journal produces, so a candidate is admitted or
    /// refused by derivation rather than by resemblance.
    pub fn satisfied_by(
        &self,
        journal: &[Admission],
        decision: &Decision,
        instance: ResourceInstanceId,
    ) -> bool {
        let Some(selected) = selects(journal, &self.after) else {
            return false;
        };

        if let Some(witness) = &self.witness {
            let offered: BTreeSet<EntryId> =
                selected.iter().map(|(entry, _)| entry.clone()).collect();

            if &offered != witness {
                return false;
            }
        }

        if let Some(recorded) = &self.recorded {
            let dated: BTreeMap<EntryId, String> = selected.iter().cloned().collect();

            if &dated != recorded {
                return false;
            }
        }

        if let Some(produced) = &self.produced {
            let Ok(resolved) = resolve(journal, &self.after, decision, instance) else {
                return false;
            };

            if &resolved.thesis.id().to_string() != produced {
                return false;
            }
        }

        true
    }

    /// The one thing about a pin that is answerable without holding anything: whether it
    /// contradicts itself.
    ///
    /// A coordinate outside its own witness describes no journal at all — the prefix ending at an
    /// entry contains that entry. It is the distinction experiment 01 named, arriving where a
    /// receiver can reach it: **self-contradictory** is not **false**, and this separates the first
    /// from everything.
    pub fn contradicts_itself(&self) -> bool {
        self.witness
            .as_ref()
            .is_some_and(|witness| !witness.contains(&self.after))
    }

    /// The journal this pin describes, written out of entries a receiver already holds.
    ///
    /// The receiver supplies the content and the pin supplies the instants, which is the whole of
    /// what it adds. Nothing here is checked: an instant is derived from nothing, so a receiver
    /// that rewrites its own journal to match a pin has believed it.
    pub fn as_described(&self, journal: &[Admission]) -> Option<Vec<Admission>> {
        let recorded = self.recorded.as_ref()?;
        let entries = addresses(journal).ok()?;

        Some(
            journal
                .iter()
                .zip(&entries)
                .map(|(admission, entry)| match recorded.get(entry) {
                    Some(instant) => dated(admission, instant),
                    None => admission.clone(),
                })
                .collect(),
        )
    }
}

/// The three files a repository is made of, held as values rather than as a directory.
pub struct Files {
    pub journal: Vec<Admission>,
    pub lineage: Vec<Taken>,
    pub worlds: Vec<WorldRecord>,
}

/// One journal the pin is measured against, and what the decision answers in it.
pub struct Candidate {
    pub label: &'static str,
    pub journal: Vec<Admission>,
    pub answers: (i128, i128),
}

/// What the procedure refers to across phases.
pub struct Arranged {
    /// The decision, as the record it was taken in wrote it.
    pub taken: Taken,
    /// That record, whole — the `early` journal, its lineage and its worlds.
    pub own: Files,
    /// A record holding the `late` journal, which satisfies the same witness.
    pub sibling: Files,
    /// A record founded apart, sharing no content and therefore no address.
    pub foreign: Files,
    /// The five journals the pin is weighed against, in the order [`CANDIDATES`] names them.
    pub candidates: Vec<Candidate>,
    pub instance: ResourceInstanceId,
    pub foreign_instance: ResourceInstanceId,
    pub fund: CommitmentId,
    pub parties: [AgentId; 2],
    pub foreign_parties: [AgentId; 2],
}

impl Arranged {
    /// The candidate by the label [`CANDIDATES`] gives it.
    pub fn candidate(&self, label: &str) -> &Candidate {
        self.candidates
            .iter()
            .find(|candidate| candidate.label == label)
            .expect("the arrangement holds every candidate it names")
    }

    /// The pin at one stage, read off the record the decision was taken in.
    pub fn pin(&self, stage: Stage) -> Result<Pin, SubjectError> {
        Pin::at(stage, &self.taken, &self.own.journal, self.instance)
    }
}

/// The labels the record under study is founded on, and the ones the record founded apart is.
///
/// Disjoint in every field, because an identity is derived from content: two records that admit one
/// label hold one address, whoever wrote them. Sharing nothing is what makes the second a stranger,
/// and the phase that uses it verifies the intersection rather than trusting this.
pub const OWN_NAMES: Names = Names {
    payer: "payer",
    payee: "payee",
    one: "one",
    other: "other",
    resource: "cash",
    instance: "account",
    verb: "receive",
};

pub const FOREIGN_NAMES: Names = Names {
    payer: "sender",
    payee: "recipient",
    one: "third",
    other: "fourth",
    resource: "grain",
    instance: "silo",
    verb: "deliver",
};

/// Found the record, take one decision in it, and build every journal that decision is put to.
pub fn arranged() -> Result<Arranged, SubjectError> {
    let early = founded(&OWN_NAMES, FUNDED, LEARNED_ON[0])?;
    let late = founded(&OWN_NAMES, FUNDED, LEARNED_ON[1])?;
    let foreign = founded(&FOREIGN_NAMES, FOREIGN_FUNDED, LEARNED_ON[0])?;

    let decision = Decision::Genesis {
        known_at: day(KNOWN_AT),
        selection: [early.fund].into(),
    };
    let taken = Taken::claimed(decision.clone(), early.parties[0], &early.admitted)?;

    let candidates = [
        ("early", early.journal.clone()),
        ("late", late.journal.clone()),
        ("reordered-early", reordered(&early.journal)),
        ("reordered-late", reordered(&late.journal)),
        ("inserted-early", inserted(&early)?),
    ]
    .into_iter()
    .zip(CANDIDATE_ANSWERS)
    .map(|((label, journal), answers)| Candidate {
        label,
        journal,
        answers,
    })
    .collect();

    Ok(Arranged {
        own: recorded_files(&early, &taken)?,
        sibling: recorded_files(&late, &taken)?,
        foreign: decided_alone(&foreign)?,
        taken,
        candidates,
        instance: early.instance,
        foreign_instance: foreign.instance,
        fund: early.fund,
        parties: early.parties,
        foreign_parties: foreign.parties,
    })
}

/// A whole record: the journal, the one decision, and the world it produced.
fn recorded_files(founded: &Founded, taken: &Taken) -> Result<Files, SubjectError> {
    let resolved = resolve(
        &founded.journal,
        &taken.after,
        &taken.decision,
        founded.instance,
    )?;

    Ok(Files {
        journal: founded.journal.clone(),
        lineage: vec![taken.clone()],
        worlds: vec![WorldRecord::of(&resolved.thesis)],
    })
}

/// The whole record a writer would keep, given a journal and the decisions taken in it.
///
/// The worlds are derived by rebuilding rather than copied from anywhere, which is what a writer
/// does and is the reason a rewritten `worlds.json` stops being able to disagree with anything.
pub fn kept(journal: &[Admission], decisions: &[Taken]) -> Result<Files, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let (built, _) = lineage::rebuild(&mut canon, journal, decisions)?;

    Ok(Files {
        journal: journal.to_vec(),
        lineage: decisions.to_vec(),
        worlds: built.decided().iter().map(WorldRecord::of).collect(),
    })
}

/// The same, for a record that took its own decision rather than being handed one.
fn decided_alone(founded: &Founded) -> Result<Files, SubjectError> {
    let taken = Taken::claimed(
        Decision::Genesis {
            known_at: day(KNOWN_AT),
            selection: [founded.fund].into(),
        },
        founded.parties[0],
        &founded.admitted,
    )?;

    recorded_files(founded, &taken)
}

/// The same journal with two independent entries swapped.
///
/// Two Roles, which refer to nothing and are referred to by identity, so the sequence moves and no
/// address does. This is what a witness cannot hold: a set has no order to disagree about.
fn reordered(journal: &[Admission]) -> Vec<Admission> {
    let mut swapped = journal.to_vec();

    swapped.swap(SWAPPED[0], SWAPPED[1]);

    swapped
}

/// The same journal with a Commitment nothing selects admitted before the coordinate.
///
/// Different knowledge, and the decision produces the identical world: a genesis absorbs what its
/// cut froze, and nothing froze this. It is experiment 12's harmless insertion, kept here as the
/// instrument that stops *the same world* from standing in for *the same knowledge*.
fn inserted(founded: &Founded) -> Result<Vec<Admission>, SubjectError> {
    let mut journal = founded.journal.clone();
    let settling = journal.pop().ok_or(SubjectError::NothingAdmitted)?;

    journal.push(Admission::Commitment {
        accountable: founded.parties[0],
        executors: [founded.parties[0]].into(),
        beneficiaries: [founded.parties[1]].into(),
        statement: founded.statement,
        resource: founded.instance,
        committed_at: day(INSERTED_ON),
        due_date: day(ASKED_AT),
        magnitude: Some(FUNDED + 1),
        dependencies: [].into(),
        recorded_at: day(INSERTED_ON),
    });
    journal.push(settling);

    Ok(journal)
}

/// What a coordinate selects in a journal: the entries admitted up to it, and when each was
/// learned.
///
/// This is the operational form of the word the protocol settled. *Resolves correctly* is this
/// value being equal to what the same coordinate selected in the record the decision was taken in,
/// and it is deliberately not the world produced — experiment 12 measured the two coming apart.
///
/// The instants are in it because knowledge is not only which entries stood: a cut resolves its
/// head from recorded instants, so two prefixes with one membership and different instants are two
/// bodies of knowledge.
pub fn selects(journal: &[Admission], after: &EntryId) -> Option<Vec<(EntryId, String)>> {
    let mut canon = Canon::new(ResidentHistory::new());
    let mut admitted = Replayed::default();

    journal::replay_through(&mut canon, journal, &mut admitted, after).ok()?;

    Some(
        admitted
            .entries
            .iter()
            .enumerate()
            .map(|(position, entry)| (entry.clone(), journal[position].recorded_at().to_owned()))
            .collect(),
    )
}

/// The same, as the set two prefixes are compared by.
///
/// A set rather than a sequence, because order is what the arrangement measures to be immaterial:
/// two admissible orders of one membership admit the same entities and resolve the same chain.
pub fn knowledge(journal: &[Admission], after: &EntryId) -> Option<BTreeSet<(EntryId, String)>> {
    Some(selects(journal, after)?.into_iter().collect())
}

/// A decision applied at its coordinate in a journal, and everything reading it needs.
pub struct Resolved {
    pub canon: Canon<ResidentHistory>,
    pub thesis: Thesis,
    pub admitted: Replayed,
    pub answers: (i128, i128),
}

/// Admit a journal up to a coordinate, take the decision there, and read what it says.
pub fn resolve(
    journal: &[Admission],
    after: &EntryId,
    decision: &Decision,
    instance: ResourceInstanceId,
) -> Result<Resolved, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let mut admitted = Replayed::default();

    journal::replay_through(&mut canon, journal, &mut admitted, after)?;

    let (thesis, _) = lineage::produced(canon.history(), &Lineage::new(), decision)?;
    let answers = answers(canon.history(), &thesis, instance)?;

    Ok(Resolved {
        canon,
        thesis,
        admitted,
        answers,
    })
}

/// Every address a journal produces, in order.
pub fn addresses(journal: &[Admission]) -> Result<Vec<EntryId>, JournalError> {
    let mut canon = Canon::new(ResidentHistory::new());

    Ok(journal::replay(&mut canon, journal)?.entries)
}

/// The same admission, recorded at another instant.
///
/// Every variant carries the field, so one binding reaches all nine — which is also the shape of
/// the claim being measured: a recording instant is the one thing every kind of entry has and no
/// kind of identity contains.
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

/// One kind of entry, where it sits in the journal, and the same entry with one other field moved.
///
/// The pair is what makes the measurement two-sided. Moving the recording instant must leave the
/// address where it was; moving anything else must not — otherwise *the instant escapes* would be
/// a statement about an address that never moves at all.
pub struct Kind {
    pub label: &'static str,
    pub at: usize,
    pub altered: Admission,
}

/// Every kind of entry the base journal holds, each with a field of it that is not the instant.
///
/// Derived from the journal rather than listed beside it: the altered entry is built by taking the
/// one at that position apart, so a journal whose shape moved makes this fail to build instead of
/// silently measuring a different entry.
pub fn kinds(journal: &[Admission]) -> Result<Vec<Kind>, SubjectError> {
    let altered = |at: usize| -> Result<Admission, SubjectError> {
        let mut entry = journal
            .get(at)
            .ok_or(SubjectError::NothingAdmitted)?
            .clone();

        match &mut entry {
            Admission::Role { label, .. } => *label = format!("{label}-else"),
            Admission::Agent { label, .. } => *label = format!("{label}-else"),
            Admission::Eligibility { effective_from, .. } => *effective_from = day(COMMITTED_ON),
            Admission::Resource { kind, .. } => {
                *kind = ResourceKindRecord::Between {
                    lower: FLOOR,
                    upper: CEILING - 1,
                }
            }
            Admission::ResourceInstance { label, .. } => *label = format!("{label}-else"),
            Admission::Action { verb, .. } => *verb = format!("{verb}-else"),
            Admission::Statement { fulfills, .. } => *fulfills = ["Done".to_owned()].into(),
            Admission::Commitment { magnitude, .. } => *magnitude = Some(FUNDED + 1),
            Admission::Event { occurred_at, .. } => *occurred_at = day(COMMITTED_ON),
        }

        Ok(entry)
    };

    [
        ("role", 0),
        ("agent", 2),
        ("resource", 4),
        ("eligibility", 5),
        ("resource-instance", 7),
        ("action", 8),
        ("statement", 9),
        ("commitment", 10),
        ("event", 11),
    ]
    .into_iter()
    .map(|(label, at)| {
        Ok(Kind {
            label,
            at,
            altered: altered(at)?,
        })
    })
    .collect()
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

/// The instant every reading is taken at.
pub fn asked_at() -> Date {
    Date::parse(day(ASKED_AT)).expect("the instant every reading is taken at is a date")
}

/// What one world answers: what has settled, and what it intends.
///
/// The pair rather than either alone, because the two move for different reasons — and here only
/// the first one does, which is what makes the arrangement about the chain rather than about the
/// intention.
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

/// What a record is founded on: the labels that make its content its own.
///
/// A parameter object rather than a positional list, because every field is a string and two
/// adjacent ones would swap without a compiler noticing — and a record founded with its parties
/// exchanged is a different record that looks like this one.
pub struct Names {
    pub payer: &'static str,
    pub payee: &'static str,
    pub one: &'static str,
    pub other: &'static str,
    pub resource: &'static str,
    pub instance: &'static str,
    pub verb: &'static str,
}

/// A record's whole base: the journal that founds it, and what admitting it produced.
pub struct Founded {
    pub journal: Vec<Admission>,
    pub admitted: Replayed,
    pub fund: CommitmentId,
    pub instance: ResourceInstanceId,
    pub statement: ape::kernel::entities::StatementId,
    pub parties: [AgentId; 2],
}

/// Admit a whole record's base, learning of the settling Event on the day given.
pub fn founded(names: &Names, funded: u128, learned_on: u8) -> Result<Founded, JournalError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let mut journal = Vec::new();
    let mut admitted = Replayed::default();

    journal.extend([
        Admission::Role {
            label: names.payer.into(),
            recorded_at: day(1),
        },
        Admission::Role {
            label: names.payee.into(),
            recorded_at: day(1),
        },
        Admission::Agent {
            label: names.one.into(),
            recorded_at: day(1),
        },
        Admission::Agent {
            label: names.other.into(),
            recorded_at: day(1),
        },
        Admission::Resource {
            label: names.resource.into(),
            kind: ResourceKindRecord::Between {
                lower: FLOOR,
                upper: CEILING,
            },
            recorded_at: day(1),
        },
    ]);
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let (payer, payee) = (admitted.roles[0], admitted.roles[1]);
    let (first, second) = (admitted.agents[0], admitted.agents[1]);
    let resource = admitted.resources[0];

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
            label: names.instance.into(),
            resource,
            recorded_at: day(1),
        },
        Admission::Action {
            verb: names.verb.into(),
            kind: ActionKindRecord::Quantifiable(EffectRecord::Increase),
            resource,
            recorded_at: day(1),
        },
    ]);
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let instance = admitted.instances[0];

    journal.push(Admission::Statement {
        actors: [payer].into(),
        recipients: [payee].into(),
        action: admitted.actions[0],
        fulfills: [FULFILLING.to_owned()].into(),
        cancels: [CANCELLING.to_owned()].into(),
        recorded_at: day(1),
    });
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let statement = admitted.statements[0];

    journal.push(Admission::Commitment {
        accountable: first,
        executors: [first].into(),
        beneficiaries: [second].into(),
        statement,
        resource: instance,
        committed_at: day(COMMITTED_ON),
        due_date: day(ASKED_AT),
        magnitude: Some(funded),
        dependencies: [].into(),
        recorded_at: day(COMMITTED_ON),
    });
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let fund = admitted.commitments[0];

    journal.push(Admission::Event {
        commitment: fund,
        observation: FULFILLING.into(),
        occurred_at: day(OCCURRED_ON),
        recorded_at: day(learned_on),
    });
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    Ok(Founded {
        journal,
        admitted,
        fund,
        instance,
        statement,
        parties: [first, second],
    })
}

pub fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
