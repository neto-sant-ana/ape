//! The individuation subject: one journal whose entries were learned on days that vary, and two
//! records that admitted the same content a day apart.
//!
//! ```text
//! cash ∈ [0, 1000]
//!
//! F   receive 400   committed day 2, recorded day 2         selected
//! G   receive 401   committed day 3, recorded day 3 or 4     selected by nobody
//! E   Event settling F, occurred day 3, recorded day 4 or 9
//!
//! D   Genesis { known_at: day 6, selection: { F } }, taken after E
//! ```
//!
//! **Two instants vary and only one of them is the coordinate's.** That is the whole of what this
//! subject adds to experiment 13's, and it is load-bearing: under an address that says *when*, a
//! coordinate whose instant is the only one that varies would determine the reference by itself, and
//! the prediction that the **witness** completes the pin would be confirmed by the arrangement rather
//! than by the change.
//!
//! ```text
//!                     entries   G learned   E learned   settled  intended
//!   early                13         3           4          400      400
//!   late                 13         3           9            0      400
//!   early-noticed-late   13         4           4          400      400
//!   reordered-early      13         3           4          400      400
//!   inserted-early       14         3           4          400      400
//! ```
//!
//! `late` differs from `early` by the coordinate's instant; `early-noticed-late` by an instant that is
//! not the coordinate's; `reordered-early` by a sequence and not by a membership;
//! `inserted-early` by an entry being there at all. The last two are the rows the change should not
//! move, and an arrangement without them would report every difference as this change's doing.
//!
//! # Two records founded apart, and one of them a day behind
//!
//! ```text
//!   here     the whole base, recorded on the days above
//!   later    the same content, every entry recorded one day later
//! ```
//!
//! Today they hold one address for every entry, because no identity carries a recording instant.
//! Their worlds are equal by identity too, and for a different reason: a `ThesisId` is derived from a
//! cut and a selection, and neither carries one either. So they are the case that says what an address
//! is **for** — it is the only place two records that never met can recognize a shared history, and a
//! world is not.
//!
//! Every quantity is an integer, for the reason [`super::reconstruction`] gives.

use std::collections::{BTreeMap, BTreeSet};

use ape::canon::Canon;
use ape::engine::thesis::{Interpretation, Thesis};
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

/// What the decision's fund puts in the account, and what the commitment nobody selects would.
///
/// Distinct, because two plans of one size would be one commitment by identity and the journal would
/// hold twelve entries rather than thirteen.
pub const FUNDED: u128 = 400;
pub const UNSELECTED: u128 = 401;

/// The days the two commitments were made, and the day the fund was observed to have settled.
pub const COMMITTED_ON: u8 = 2;
pub const UNSELECTED_ON: u8 = 3;
pub const OCCURRED_ON: u8 = 3;

/// The two days the settling Event was **recorded** on, which is the coordinate's instant.
pub const LEARNED_ON: [u8; 2] = [4, 9];

/// The two days the unselected commitment was recorded on, which is an instant that is not.
///
/// Both are before either day the Event was learned on, so every combination is admissible: recording
/// is monotonic across admission, and a candidate refused for arriving out of order would be measuring
/// the watermark instead of the address.
pub const NOTICED_ON: [u8; 2] = [3, 4];

/// The instant the decision recognizes.
pub const KNOWN_AT: u8 = 6;

/// The instant every reading is taken at, after every due date the subject holds.
pub const ASKED_AT: u8 = 20;

/// How far behind the record founded later is.
///
/// One day, and it has to leave everything still known at the cut: a record whose Event arrived after
/// the instant the decision names would answer differently for a reason that is not about addresses.
pub const LATER_BY: u8 = 1;

/// The day the fund is readmitted on, in the journal that admits one entry twice.
pub const READMITTED_ON: u8 = 5;

/// The entries each journal holds.
pub const ENTRIES: usize = 13;
pub const INSERTED_ENTRIES: usize = 14;
pub const READMITTED_ENTRIES: usize = 14;

/// The positions the reordered journal swaps, which are two Roles that refer to nothing.
pub const SWAPPED: [usize; 2] = [0, 1];

/// The places in `cli/src` an [`EntryId`] is compared, which is what *the same entry* means.
///
/// Derived rather than listed — see the phase that reads this — and the count is the guard: a new
/// site is a change to the application, and one that is not in this list is this list being wrong.
///
/// **It caught one, and the list is longer than the result that published it.** Experiment 14
/// measured five and used them to price a change it went on to refuse — an address that says when the
/// entry was recorded. Experiment 16 added `reading::held`, the sixth, when the record gained a claim
/// about what it holds. The published five stand against the commit they were taken at, and the
/// change is recorded here rather than absorbed: that change now costs six sites, not five.
pub const SITES: [&str; 6] = [
    "journal::replay_through",
    "lineage::corroborate",
    "lineage::diagnosed",
    "converge::appended",
    "converge::ordered",
    "reading::held",
];

/// What the decision answers where the Event was learned early, and where it was learned late.
pub const WHEN_EARLY: (i128, i128) = (FUNDED as i128, FUNDED as i128);
pub const WHEN_LATE: (i128, i128) = (0, FUNDED as i128);

/// The candidate journals, in the order [`Arranged::candidates`] holds them.
pub const CANDIDATES: [&str; 5] = [
    "early",
    "late",
    "early-noticed-late",
    "reordered-early",
    "inserted-early",
];

pub const CANDIDATE_ANSWERS: [(i128, i128); 5] =
    [WHEN_EARLY, WHEN_LATE, WHEN_EARLY, WHEN_EARLY, WHEN_EARLY];
pub const CANDIDATE_ENTRIES: [usize; 5] = [ENTRIES, ENTRIES, ENTRIES, ENTRIES, INSERTED_ENTRIES];

/// The pin's stages under the present shape: candidates admitted, bodies of knowledge, worlds.
///
/// Written before anything runs. A stage **determines the reference** when the middle number is one,
/// and today that is the stage which carries a recording instant for every witnessed entry.
pub const STAGES_BY_CONTENT: [(&str, usize, usize, usize); 4] = [
    ("coordinate", 5, 4, 2),
    ("witnessed", 4, 3, 2),
    ("dated", 2, 1, 1),
    ("produced", 2, 1, 1),
];

/// The same, predicted for an address that says when its entry was recorded.
///
/// The prediction is the second row: the **witness** determines, with no field added to a `Taken`. And
/// the third row buying nothing is the other half of it — a dated pin would be saying twice what the
/// addresses already say.
pub const STAGES_BY_COMPOSITE: [(&str, usize, usize, usize); 4] = [
    ("coordinate", 4, 3, 1),
    ("witnessed", 2, 1, 1),
    ("dated", 2, 1, 1),
    ("produced", 2, 1, 1),
];

/// The literals above, weighed against each other before anything runs.
const _: () = assert!(FLOOR < FUNDED as i128 && (FUNDED as i128) < CEILING);
const _: () = assert!(FUNDED != UNSELECTED && (UNSELECTED as i128) < CEILING);
const _: () = assert!(COMMITTED_ON <= UNSELECTED_ON && COMMITTED_ON <= OCCURRED_ON);
const _: () = assert!(UNSELECTED_ON <= NOTICED_ON[0] && NOTICED_ON[0] < NOTICED_ON[1]);
const _: () = assert!(OCCURRED_ON <= LEARNED_ON[0] && NOTICED_ON[1] <= LEARNED_ON[0]);
// The hinge the arrangement inherits: the instant the decision names falls between the two days the
// Event was learned on, so a cut resolves a different head in each.
const _: () = assert!(LEARNED_ON[0] <= KNOWN_AT && KNOWN_AT < LEARNED_ON[1]);
const _: () = assert!(COMMITTED_ON <= KNOWN_AT && KNOWN_AT < ASKED_AT);
// And the addition this subject makes: a second instant varies, and it is not the coordinate's. Both
// of its days sit on the same side of the cut, so what it moves is an address and never an answer.
const _: () = assert!(NOTICED_ON[1] <= KNOWN_AT);
// The record founded later is still wholly known at the cut, so the two records agree about a world.
const _: () = assert!(LATER_BY >= 1 && LEARNED_ON[0] + LATER_BY <= KNOWN_AT);
const _: () = assert!(LEARNED_ON[0] <= READMITTED_ON && READMITTED_ON <= KNOWN_AT);

const _: () = assert!(!matching(WHEN_EARLY, WHEN_LATE));
const _: () = assert!(WHEN_EARLY.1 == WHEN_LATE.1, "only the settled level moves");
const _: () = assert!(INSERTED_ENTRIES == ENTRIES + 1 && READMITTED_ENTRIES == ENTRIES + 1);
const _: () = assert!(SWAPPED[0] < SWAPPED[1] && SWAPPED[1] < ENTRIES);

const _: () = assert!(STAGES_BY_CONTENT[0].1 == CANDIDATES.len());
const _: () = assert!(
    STAGES_BY_CONTENT[1].2 > 1,
    "the witness does not determine today"
);
const _: () = assert!(STAGES_BY_CONTENT[2].2 == 1, "and the instants do");
// The prediction, stated as the difference between the two columns rather than as a row of one: the
// stage that determines moves up by one, and the stage below it stops buying anything.
const _: () = assert!(STAGES_BY_COMPOSITE[1].2 == 1);
const _: () = assert!(
    STAGES_BY_COMPOSITE[2].1 == STAGES_BY_COMPOSITE[1].1
        && STAGES_BY_COMPOSITE[2].2 == STAGES_BY_COMPOSITE[1].2
);
// And the coordinate alone must NOT determine under either shape, or the arrangement did the work.
const _: () = assert!(STAGES_BY_CONTENT[0].2 > 1 && STAGES_BY_COMPOSITE[0].2 > 1);

const fn matching(one: (i128, i128), other: (i128, i128)) -> bool {
    one.0 == other.0 && one.1 == other.1
}

/// Whether the application's address says when its entry was recorded.
///
/// Derived from an address rather than from a build flag, so the phases read the shape that is
/// actually there. A `#[cfg]` would be the suite deciding which column it is in, and this experiment
/// runs the same phases against two shapes.
pub fn composite() -> bool {
    let mut canon = Canon::new(ResidentHistory::new());
    let journal = [Admission::Role {
        label: "measured".into(),
        recorded_at: day(1),
    }];

    let address = journal::replay(&mut canon, &journal)
        .expect("one role admits")
        .entries
        .pop()
        .expect("and produces one address");

    address.to_string().len() > 64
}

/// The stage table for the shape that is actually there.
pub fn stages() -> [(&'static str, usize, usize, usize); 4] {
    match composite() {
        true => STAGES_BY_COMPOSITE,
        false => STAGES_BY_CONTENT,
    }
}

/// How much of a reference has been written down.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Coordinate,
    Witnessed,
    Dated,
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
/// Experiment 13's instrument, rebuilt here rather than shared, for the reason [`super`] gives. What
/// it measures in this experiment is different: there, whether a stage completes the pin; here,
/// whether an address that says *when* moves which stage does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pin {
    pub after: EntryId,
    pub witness: Option<BTreeSet<EntryId>>,
    pub recorded: Option<BTreeMap<EntryId, String>>,
    pub produced: Option<String>,
}

impl Pin {
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
    /// That record, whole.
    pub here: Files,
    /// The same content, admitted a day later, with its own decision taken in it.
    pub later: Files,
    /// The five journals the pin is weighed against, in the order [`CANDIDATES`] names them.
    pub candidates: Vec<Candidate>,
    /// A journal that admits the fund twice, on two days.
    pub readmitted: Vec<Admission>,
    /// The same, on one day — which is the half of the readmission an address cannot separate.
    pub readmitted_at_once: Vec<Admission>,
    pub instance: ResourceInstanceId,
    pub later_instance: ResourceInstanceId,
    pub fund: CommitmentId,
}

impl Arranged {
    pub fn candidate(&self, label: &str) -> &Candidate {
        self.candidates
            .iter()
            .find(|candidate| candidate.label == label)
            .expect("the arrangement holds every candidate it names")
    }

    pub fn pin(&self, stage: Stage) -> Result<Pin, SubjectError> {
        Pin::at(stage, &self.taken, &self.here.journal, self.instance)
    }
}

/// Found the record, take one decision in it, and build every journal that decision is put to.
pub fn arranged() -> Result<Arranged, SubjectError> {
    let here = founded(Learned {
        unselected: NOTICED_ON[0],
        settling: LEARNED_ON[0],
        later_by: 0,
    })?;
    let late = founded(Learned {
        unselected: NOTICED_ON[0],
        settling: LEARNED_ON[1],
        later_by: 0,
    })?;
    let noticed_late = founded(Learned {
        unselected: NOTICED_ON[1],
        settling: LEARNED_ON[0],
        later_by: 0,
    })?;
    let later = founded(Learned {
        unselected: NOTICED_ON[0],
        settling: LEARNED_ON[0],
        later_by: LATER_BY,
    })?;

    let decision = |founded: &Founded| Decision::Genesis {
        known_at: day(KNOWN_AT),
        selection: [founded.fund].into(),
    };

    let taken = Taken::now(decision(&here), &here.admitted)?;

    let candidates = [
        ("early", here.journal.clone()),
        ("late", late.journal.clone()),
        ("early-noticed-late", noticed_late.journal.clone()),
        ("reordered-early", reordered(&here.journal)),
        ("inserted-early", inserted(&here)?),
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
        here: kept(&here.journal, std::slice::from_ref(&taken))?,
        later: kept(
            &later.journal,
            &[Taken::now(decision(&later), &later.admitted)?],
        )?,
        readmitted: readmitted(&here),
        readmitted_at_once: readmitted_at_once(&here),
        taken,
        candidates,
        instance: here.instance,
        later_instance: later.instance,
        fund: here.fund,
    })
}

/// The same journal with two independent entries swapped.
fn reordered(journal: &[Admission]) -> Vec<Admission> {
    let mut swapped = journal.to_vec();

    swapped.swap(SWAPPED[0], SWAPPED[1]);

    swapped
}

/// The same journal with one more commitment nothing selects, admitted before the coordinate.
fn inserted(founded: &Founded) -> Result<Vec<Admission>, SubjectError> {
    let mut journal = founded.journal.clone();
    let settling = journal.pop().ok_or(SubjectError::NothingAdmitted)?;

    journal.push(
        founded
            .inflow
            .of(UNSELECTED + 1, UNSELECTED_ON, NOTICED_ON[0]),
    );
    journal.push(settling);

    Ok(journal)
}

/// The same journal, with the fund admitted a second time after the coordinate's entry.
///
/// One piece of knowledge, admitted twice on two days. The Canon says these are the same entry — it
/// answers `AlreadyPresent` and hands back the identity it already held — so whether the *record*
/// says so is the question, and this is the smallest journal that asks it.
fn readmitted(founded: &Founded) -> Vec<Admission> {
    let mut journal = founded.journal.clone();

    journal.push(founded.inflow.of(FUNDED, COMMITTED_ON, READMITTED_ON));

    journal
}

/// The same, with the two occurrences on one day.
///
/// It has to end there, and the journal is cut back to make it: a diagnosis is reached only when the
/// coordinate is the readmitted address **and** something was learned between the two occurrences, so
/// the case needs an entry between them and nothing after. Recording is monotonic across admission,
/// which means everything between two same-day occurrences was learned on that day too — and that is
/// why this readmission is the narrow one. Within a single journal it is the only kind an address that
/// says *when* cannot tell apart.
fn readmitted_at_once(founded: &Founded) -> Vec<Admission> {
    let mut journal = founded.journal.clone();

    journal.truncate(ENTRIES - 2);
    journal.push(
        founded
            .inflow
            .of(UNSELECTED + 2, COMMITTED_ON, COMMITTED_ON),
    );
    journal.push(founded.inflow.of(FUNDED, COMMITTED_ON, COMMITTED_ON));

    journal
}

/// A decision taken over the whole of a journal, witnessed by everything in it.
pub fn decided_over(journal: &[Admission]) -> Result<Taken, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let admitted = journal::replay(&mut canon, journal)?;

    let fund = *admitted
        .commitments
        .first()
        .ok_or(SubjectError::NothingAdmitted)?;

    Ok(Taken::now(
        Decision::Genesis {
            known_at: day(KNOWN_AT),
            selection: [fund].into(),
        },
        &admitted,
    )?)
}

/// What a coordinate selects in a journal: the entries admitted up to it, and when each was learned.
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

/// Put a whole repository there the way an application does — one call, all or nothing.
pub fn write_whole(repository: &Repository, files: &Files) -> Result<(), RepositoryError> {
    repository.write_whole(RepositoryInput {
        journal: &files.journal,
        lineage: &files.lineage,
        worlds: &files.worlds,
    })
}

pub fn asked_at() -> Date {
    Date::parse(day(ASKED_AT)).expect("the instant every reading is taken at is a date")
}

/// What one world answers: what has settled, and what it intends.
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

/// When a record learned the two things whose instants vary, and how far behind the whole of it is.
///
/// A parameter object rather than three positional `u8`s, because three adjacent days would swap
/// without a compiler noticing — and a candidate founded with two of its instants exchanged is a
/// different candidate that looks like this one.
pub struct Learned {
    pub unselected: u8,
    pub settling: u8,
    pub later_by: u8,
}

/// What every commitment in this subject shares, held apart from the record it belongs to.
///
/// Apart, because the journal's first commitment has to be built before there is a `Founded` to build
/// it from — and a field that could not be filled yet is how a placeholder identity gets invented.
#[derive(Clone, Copy)]
pub struct Inflow {
    parties: [AgentId; 2],
    statement: StatementId,
    instance: ResourceInstanceId,
    later_by: u8,
}

impl Inflow {
    /// One inflow, committed and recorded on the days given.
    pub fn of(&self, magnitude: u128, committed: u8, recorded: u8) -> Admission {
        Admission::Commitment {
            accountable: self.parties[0],
            executors: [self.parties[0]].into(),
            beneficiaries: [self.parties[1]].into(),
            statement: self.statement,
            resource: self.instance,
            committed_at: day(committed),
            due_date: day(ASKED_AT),
            magnitude: Some(magnitude),
            dependencies: [].into(),
            recorded_at: day(recorded + self.later_by),
        }
    }
}

/// A record's whole base, and what admitting it produced.
pub struct Founded {
    pub journal: Vec<Admission>,
    pub admitted: Replayed,
    pub fund: CommitmentId,
    pub instance: ResourceInstanceId,
    pub inflow: Inflow,
}

/// Admit a whole record's base, learning of each thing on the day given.
pub fn founded(learned: Learned) -> Result<Founded, JournalError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let mut journal = Vec::new();
    let mut admitted = Replayed::default();

    let on = |at: u8| day(at + learned.later_by);

    journal.extend([
        Admission::Role {
            label: "payer".into(),
            recorded_at: on(1),
        },
        Admission::Role {
            label: "payee".into(),
            recorded_at: on(1),
        },
        Admission::Agent {
            label: "one".into(),
            recorded_at: on(1),
        },
        Admission::Agent {
            label: "other".into(),
            recorded_at: on(1),
        },
        Admission::Resource {
            label: "cash".into(),
            kind: ResourceKindRecord::Between {
                lower: FLOOR,
                upper: CEILING,
            },
            recorded_at: on(1),
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
            recorded_at: on(1),
        },
        Admission::Eligibility {
            agent: second,
            roles: [payee].into(),
            effective_from: day(1),
            recorded_at: on(1),
        },
        Admission::ResourceInstance {
            label: "account".into(),
            resource,
            recorded_at: on(1),
        },
        Admission::Action {
            verb: "receive".into(),
            kind: ActionKindRecord::Quantifiable(EffectRecord::Increase),
            resource,
            recorded_at: on(1),
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
        recorded_at: on(1),
    });
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let inflow = Inflow {
        parties: [first, second],
        statement: admitted.statements[0],
        instance,
        later_by: learned.later_by,
    };

    journal.push(inflow.of(FUNDED, COMMITTED_ON, COMMITTED_ON));
    journal.push(inflow.of(UNSELECTED, UNSELECTED_ON, learned.unselected));
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let fund = admitted.commitments[0];

    journal.push(Admission::Event {
        commitment: fund,
        observation: FULFILLING.into(),
        occurred_at: day(OCCURRED_ON),
        recorded_at: on(learned.settling),
    });
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    Ok(Founded {
        journal,
        admitted,
        fund,
        instance,
        inflow,
    })
}

pub fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
