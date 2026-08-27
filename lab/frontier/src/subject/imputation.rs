//! The imputation subject: a record holding another's intention, and no way to say whose it was.
//!
//! ```text
//! cash ∈ [0, 1000]
//!
//! ── the base, admitted identically by every record here ────────────────────
//! V   twelve entries of vocabulary, day 1
//! F   receive 400, day 2      E₀  Event settling F, day 3
//!
//! ── four entries, and every record that holds them holds the same four ─────
//! A   agent `analyst`         S   spend 60        E₁  Event settling S
//! P   spend 20, left open
//!
//!   origin    the base, the four on day 4, and two decisions on day 6
//!   there     the base, the four on day 5, and two decisions on day 7
//!   relayed   the base, then SHOWN origin's four — learned on day 5 —
//!             and origin's two decisions retaken on day 7
//!   here      the base, a spend of 30 and its Event, one decision on day 7
//! ```
//!
//! # What the arrangement has to hold, and why each part of it
//!
//! **A record that actually took another's material.** `here` is shown `there` on day 9 and retakes
//! both of its decisions on day 10, which is experiment 15's third move. A source that was never a
//! source is not the case being asked about.
//!
//! **A source party the receiver holds only because it learned it.** `analyst` is admitted by nobody
//! in the base. `here` cannot resolve it before the crossing and can after, which is what makes P1
//! about **learning** rather than about the vocabulary both records always shared. It is an agent that
//! does nothing but decide — no eligibility, no commitment — because a party is what a decision claims
//! and not a participant in one.
//!
//! **A source world the receiver never produces.** Experiment 15 measured that a retaken world is
//! never a world the other record decided, so `there`'s first world is a name `here` has no way to
//! reach. That is what P2 has to fail against.
//!
//! **A decision the receiver took alone**, so that a lineage holding one of each can be read for
//! whether the two are distinguishable.
//!
//! **And `relayed`, which is the whole of Phase 4.** It holds exactly what `there` holds and authored
//! none of it: the same four entries at the same instant, the same two intentions at the same instant.
//! Whether the two records come out the same is the measurement, and it is the only way to ask what it
//! would take for a writer to be **wrong** rather than to ask whether this one is honest — which the
//! arrangement decides by construction and therefore cannot answer.
//!
//! The four entries are admitted on **one day** by every record that authors them, because a relay
//! learns everything at the instant it was shown. An arrangement that spread them over three days
//! would have made `relayed` distinguishable from `there` by an accident of the subject rather than by
//! anything about relaying.
//!
//! Every quantity is an integer, for the reason [`super::reconstruction`] gives.

use std::collections::{BTreeMap, BTreeSet};

use ape::canon::Canon;
use ape::engine::thesis::{Interpretation, Thesis, ThesisId, ThesisLookup};
use ape::kernel::axiom::Knowledge;
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

pub const FLOOR: i128 = 0;
pub const CEILING: i128 = 1000;

/// What the base puts in the account, and what each record proposes against it.
pub const FUNDED: u128 = 400;
pub const HERE_SPENDS: u128 = 30;
pub const THERE_SPENDS: u128 = 60;

/// What the source record committed and deliberately left open, so a fork has something to introduce.
///
/// A fork cannot omit what history froze — experiment 15 met `FrozenPastOmitted` — so an intention
/// that crosses at all is one that introduces.
pub const SPARE: u128 = 20;

/// The instants, in the order they occur.
pub const VOCABULARY_ON: u8 = 1;
pub const FUNDED_ON: u8 = 2;
pub const SETTLED_ON: u8 = 3;

/// When the four entries say they happened, in every record that holds them.
///
/// Pinned rather than varied with the recording instant, and it is the difference between a
/// comparison and an accident. A relay learns content at its own instant — [`dated`] moves
/// `recorded_at` and nothing else — so an author whose *content* instants differed from the origin's
/// would produce different identities, and `relayed` would be distinguishable from `authored` by
/// arithmetic rather than by anything about relaying.
pub const COMMITTED_ON: u8 = 4;

/// When each record records them: the origin on day 4, everyone downstream on day 5.
pub const ORIGIN_RECORDS_ON: u8 = 4;
pub const RECORDS_ON: u8 = 5;

/// When the origin decides, and when everything downstream of it does.
pub const ORIGIN_DECIDES_AT: u8 = 6;
pub const DECIDES_AT: u8 = 7;

/// When the receiving record is shown the other's material, and when it retakes.
pub const SHOWN_ON: u8 = 9;
pub const RETAKES_AT: u8 = 10;

pub const ASKED_AT: u8 = 20;

/// The vocabulary, the fund and its Event: what every record here begins from.
pub const BASE_ENTRIES: usize = 14;

/// The four entries a source record holds and a receiver does not.
pub const CROSSING: usize = 4;

/// What each record's journal comes to.
pub const ORIGIN_ENTRIES: usize = BASE_ENTRIES + CROSSING;
pub const THERE_ENTRIES: usize = BASE_ENTRIES + CROSSING;
pub const HERE_ENTRIES: usize = BASE_ENTRIES + 2;
pub const TAKEN_ENTRIES: usize = HERE_ENTRIES + CROSSING;

/// What the receiving record lacks, which is the whole of the crossing.
pub const LACKING: usize = CROSSING;

/// How many decisions each record holds.
pub const THERE_DECISIONS: usize = 2;
pub const HERE_DECISIONS: usize = 1;
pub const TAKEN_DECISIONS: usize = HERE_DECISIONS + THERE_DECISIONS;

/// What each world answers on the account: what has settled, then what it intends.
pub const HERE_DECIDES: (i128, i128) = (
    (FUNDED - HERE_SPENDS) as i128,
    (FUNDED - HERE_SPENDS) as i128,
);

pub const THERE_DECIDES: [(i128, i128); 2] = [
    (
        (FUNDED - THERE_SPENDS) as i128,
        (FUNDED - THERE_SPENDS) as i128,
    ),
    (
        (FUNDED - THERE_SPENDS) as i128,
        (FUNDED - THERE_SPENDS - SPARE) as i128,
    ),
];

pub const RETAKEN: [(i128, i128); 2] = [
    (
        (FUNDED - HERE_SPENDS - THERE_SPENDS) as i128,
        (FUNDED - HERE_SPENDS - THERE_SPENDS) as i128,
    ),
    (
        (FUNDED - HERE_SPENDS - THERE_SPENDS) as i128,
        (FUNDED - HERE_SPENDS - THERE_SPENDS - SPARE) as i128,
    ),
];

/// The literals above, weighed against each other before anything runs.
const _: () = assert!(FLOOR < FUNDED as i128 && (FUNDED as i128) < CEILING);
const _: () = assert!(HERE_SPENDS != THERE_SPENDS && SPARE != HERE_SPENDS);
const _: () = assert!(SPARE != THERE_SPENDS);
const _: () = assert!(
    FLOOR < (FUNDED - HERE_SPENDS - THERE_SPENDS - SPARE) as i128,
    "no world here is refused for leaving the account's bounds"
);
// Five answers, and no two of them coincide — so a phase reporting one has said which world.
const _: () = assert!(!alike(HERE_DECIDES, THERE_DECIDES[0]));
const _: () = assert!(!alike(THERE_DECIDES[0], THERE_DECIDES[1]));
const _: () = assert!(!alike(RETAKEN[0], RETAKEN[1]));
const _: () = assert!(!alike(RETAKEN[0], HERE_DECIDES) && !alike(RETAKEN[0], THERE_DECIDES[0]));
const _: () = assert!(!alike(RETAKEN[1], THERE_DECIDES[1]) && !alike(RETAKEN[1], HERE_DECIDES));
// Every instant is in the order the arrangement describes.
const _: () = assert!(VOCABULARY_ON < FUNDED_ON && FUNDED_ON < SETTLED_ON);
const _: () = assert!(SETTLED_ON < COMMITTED_ON && COMMITTED_ON <= ORIGIN_RECORDS_ON);
const _: () = assert!(ORIGIN_RECORDS_ON < RECORDS_ON);
const _: () = assert!(RECORDS_ON < ORIGIN_DECIDES_AT && ORIGIN_DECIDES_AT < DECIDES_AT);
const _: () = assert!(DECIDES_AT < SHOWN_ON && SHOWN_ON < RETAKES_AT);
const _: () = assert!(RETAKES_AT < ASKED_AT);
// The receiver lacks exactly the crossing, and holds two entries of its own the source does not.
const _: () = assert!(TAKEN_ENTRIES == HERE_ENTRIES + LACKING);
const _: () = assert!(HERE_ENTRIES + 2 == THERE_ENTRIES);

const fn alike(one: (i128, i128), other: (i128, i128)) -> bool {
    one.0 == other.0 && one.1 == other.1
}

/// The three files one repository is made of, as values rather than as a directory.
#[derive(Debug, Clone)]
pub struct Files {
    pub journal: Vec<Admission>,
    pub lineage: Vec<Taken>,
    pub worlds: Vec<WorldRecord>,
}

/// What a decision would say about where its intention came from, expressed two ways.
///
/// In the laboratory and nowhere else. Neither is proposed: the question is what a receiving record
/// can *do* with each, and adding a field would answer it by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Imputed {
    /// The party whose record the intention came from.
    Party(AgentId),
    /// The world the intention was taken from.
    World(ThesisId),
}

/// Every record the experiment needs, and what a phase asks any of them.
pub struct Arranged {
    /// Where the intention was authored, and which nothing downstream of `there` ever sees.
    pub origin: Files,
    /// A record that authored the four entries and both intentions itself.
    pub authored: Files,
    /// A record that was shown them and claims nobody, which is what a candidate writes.
    pub relayed_honestly: Files,
    /// The same, writing the source's party into `by` — the sentence 15 measured is accepted.
    pub relayed_claiming: Files,
    /// The receiving record, before it is shown anything.
    pub here: Files,
    /// And after it has taken `authored`'s material and retaken both of its intentions.
    pub taken: Files,

    pub account: ResourceInstanceId,
    /// The party the source's decisions claim, admitted by no base and learned in the crossing.
    pub analyst: AgentId,
    /// A party of the base, which every record has always held.
    pub merchant: AgentId,
    /// The first world the source decided, which the receiver never produces.
    pub source_world: ThesisId,
}

/// Found every record from one statement of the subject.
pub fn arranged() -> Result<Arranged, SubjectError> {
    let origin = authoring(ORIGIN_RECORDS_ON, ORIGIN_DECIDES_AT)?;
    let authored = authoring(RECORDS_ON, DECIDES_AT)?;

    let base = base()?;
    let relayed_honestly = shown(&base.files, &origin, Shown::claiming(None))?;

    let claimed = analyst(&authored)?;
    let relayed_claiming = shown(&base.files, &origin, Shown::claiming(Some(claimed)))?;

    let here = deciding_alone()?;
    let taken = shown(
        &here,
        &authored,
        Shown {
            learned_at: SHOWN_ON,
            known_at: RETAKES_AT,
            by: None,
        },
    )?;

    let source_world = rebuilt(&authored)?
        .lineage
        .decided()
        .first()
        .ok_or(SubjectError::NothingDecided)?
        .id();

    Ok(Arranged {
        account: base.account,
        analyst: claimed,
        merchant: base.merchant,
        source_world,
        origin,
        authored,
        relayed_honestly,
        relayed_claiming,
        here,
        taken,
    })
}

/// The agent a record's decisions claim, read back out of the record rather than carried.
fn analyst(files: &Files) -> Result<AgentId, SubjectError> {
    files
        .lineage
        .first()
        .and_then(|taken| taken.by)
        .ok_or(SubjectError::NothingDecided)
}

/// How a record that was shown another's material writes what it takes.
///
/// A parameter object rather than three arguments, because two of the three are instants and adjacent
/// instants of the same type are the footgun a named field removes.
pub struct Shown {
    /// The instant the receiving record can honestly claim about holding what it was shown.
    pub learned_at: u8,
    /// And the instant a retaken decision names.
    pub known_at: u8,
    /// What the retaken decisions claim, where the record claims anything.
    pub by: Option<AgentId>,
}

impl Shown {
    /// A relay of the source's material, at the instants the arrangement pins for one.
    fn claiming(by: Option<AgentId>) -> Self {
        Self {
            learned_at: RECORDS_ON,
            known_at: DECIDES_AT,
            by,
        }
    }
}

/// One entry a record does not hold, and the address it had where it does.
pub struct Lacking {
    pub entry: Admission,
    pub there: EntryId,
}

/// What one record's journal holds that another's does not, by address.
pub fn lacking(here: &[Admission], there: &[Admission]) -> Result<Vec<Lacking>, JournalError> {
    let held: BTreeSet<EntryId> = addresses(here)?.into_iter().collect();

    Ok(there
        .iter()
        .zip(addresses(there)?)
        .filter(|(_, address)| !held.contains(address))
        .map(|(entry, there)| Lacking {
            entry: entry.clone(),
            there,
        })
        .collect())
}

/// A record that was shown another's whole record, and took what it lacked.
///
/// Experiment 15's third move, rebuilt here rather than borrowed: the content crosses and the instant
/// does not, each decision is retaken in this record's frame, and what a fork extends is mapped
/// through the world this record produced for the one the original extended.
pub fn shown(into: &Files, from: &Files, shown: Shown) -> Result<Files, SubjectError> {
    let learned: Vec<Admission> = lacking(&into.journal, &from.journal)?
        .iter()
        .map(|held| dated(&held.entry, &day(shown.learned_at)))
        .collect();

    let mut journal = into.journal.clone();
    journal.extend(learned);

    let mut canon = Canon::new(ResidentHistory::new());
    let (mut lineage, mut admitted) = lineage::rebuild(&mut canon, &journal, &into.lineage)?;

    let theirs = rebuilt(from)?;
    let mut translated: BTreeMap<ThesisId, ThesisId> = BTreeMap::new();
    let mut decisions = into.lineage.clone();

    for (position, held) in from.lineage.iter().enumerate() {
        let decision = match &held.decision {
            Decision::Genesis { selection, .. } => Decision::Genesis {
                known_at: day(shown.known_at),
                selection: selection.clone(),
            },
            Decision::Advance { extends, known_at } => Decision::Advance {
                extends: mapped(&translated, extends)?,
                known_at: known_at.clone(),
            },
            Decision::Fork {
                extends,
                omitted,
                introduced,
            } => Decision::Fork {
                extends: mapped(&translated, extends)?,
                omitted: omitted.clone(),
                introduced: introduced.clone(),
            },
        };

        let retaken = match shown.by {
            Some(party) => Taken::claimed(decision, party, &admitted)?,
            None => Taken::now(decision, &admitted)?,
        };

        lineage::decide(canon.history(), &mut lineage, &retaken.decision)?;
        journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

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

        decisions.push(retaken);
    }

    kept(&journal, &decisions)
}

fn mapped(
    translated: &BTreeMap<ThesisId, ThesisId>,
    extends: &ThesisId,
) -> Result<ThesisId, SubjectError> {
    translated
        .get(extends)
        .copied()
        .ok_or(SubjectError::NothingDecided)
}

/// A record shown another's whole record that learned everything and took no intention at all.
///
/// The control Phase 1 needs. If a party's name resolves here too, then resolving is a fact about the
/// journal and says nothing about an intention having crossed — which is the distinction between
/// *resolves* and *checks*, reached from the side nobody would think to look at.
pub fn learning(into: &Files, from: &Files, at: u8) -> Result<Files, SubjectError> {
    let learned: Vec<Admission> = lacking(&into.journal, &from.journal)?
        .iter()
        .map(|held| dated(&held.entry, &day(at)))
        .collect();

    let mut journal = into.journal.clone();
    journal.extend(learned);

    kept(&journal, &into.lineage)
}

/// A record extending a world by name, which is the record's one way of resolving one.
///
/// What Phase 2 puts a foreign identity to. `lineage::decide` looks the name up in the archive it has
/// built, so a world this record never produced has nothing to resolve against.
pub fn extending(files: &Files, thesis: ThesisId) -> Result<Files, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let (_, admitted) = lineage::rebuild(&mut canon, &files.journal, &files.lineage)?;

    let mut decisions = files.lineage.clone();
    decisions.push(Taken::now(
        Decision::Advance {
            extends: thesis,
            known_at: day(ASKED_AT),
        },
        &admitted,
    )?);

    kept(&files.journal, &decisions)
}

/// The same decisions, with the ones from `first` onward claiming a party.
///
/// A position rather than the whole lineage, because `attributed` is **coordinate-relative**: it
/// weighs the party against the knowledge that stood when the decision was taken, not against the
/// record entire. A record's own decision, taken before it was shown anything, cannot claim a party it
/// had not met — which is measured rather than worked around.
pub fn claiming(lineage: &[Taken], by: AgentId, first: usize) -> Vec<Taken> {
    lineage
        .iter()
        .enumerate()
        .map(|(position, taken)| match position >= first {
            true => Taken {
                by: Some(by),
                ..taken.clone()
            },
            false => taken.clone(),
        })
        .collect()
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

pub fn rebuilt(files: &Files) -> Result<Rebuilt, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
    let (lineage, admitted) = lineage::rebuild(&mut canon, &files.journal, &files.lineage)?;

    Ok(Rebuilt {
        canon,
        lineage,
        admitted,
    })
}

/// Whether the receiving record can find what a relation names.
///
/// **Resolving is not checking**, and keeping the two apart is most of this experiment. This answers
/// whether the record holds something by that identity, through the same two lookups the record itself
/// uses — `Knowledge::agent` for a party, and the archive for a world. It says nothing whatever about
/// the sentence a relation naming it would make.
pub fn resolves(rebuilt: &Rebuilt, imputed: Imputed) -> bool {
    match imputed {
        Imputed::Party(agent) => rebuilt.canon.history().agent(agent).is_some(),
        Imputed::World(thesis) => rebuilt.lineage.archive().thesis(thesis).is_some(),
    }
}

pub fn addresses(journal: &[Admission]) -> Result<Vec<EntryId>, JournalError> {
    let mut canon = Canon::new(ResidentHistory::new());

    Ok(journal::replay(&mut canon, journal)?.entries)
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

/// What one world answers on the account: what has settled, and what it intends.
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

/// Every world a record decided, and what each answers.
pub fn answers(
    files: &Files,
    account: ResourceInstanceId,
) -> Result<Vec<(i128, i128)>, SubjectError> {
    let rebuilt = rebuilt(files)?;

    rebuilt
        .lineage
        .decided()
        .iter()
        .map(|world| Ok(answered(rebuilt.canon.history(), world, account)?))
        .collect()
}

/// The base every record here begins from: the vocabulary, the fund, and the Event settling it.
pub struct Base {
    pub files: Files,
    pub account: ResourceInstanceId,
    pub merchant: AgentId,
    customer: AgentId,
    outflow: StatementId,
}

pub fn base() -> Result<Base, SubjectError> {
    let mut canon = Canon::new(ResidentHistory::new());
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
        Admission::Agent {
            label: "customer".into(),
            recorded_at: recorded.clone(),
        },
        Admission::Agent {
            label: "merchant".into(),
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
    ]);
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let (payer, payee) = (admitted.roles[0], admitted.roles[1]);
    let (customer, merchant) = (admitted.agents[0], admitted.agents[1]);
    let cash = admitted.resources[0];

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
        Admission::ResourceInstance {
            label: "account".into(),
            resource: cash,
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
    ]);
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let account = admitted.instances[0];
    let (receive, spend) = (admitted.actions[0], admitted.actions[1]);

    journal.extend([
        statement(payer, payee, receive),
        statement(payee, payer, spend),
    ]);
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let (inflow, outflow) = (admitted.statements[0], admitted.statements[1]);

    journal.push(Admission::Commitment {
        accountable: customer,
        executors: [customer].into(),
        beneficiaries: [merchant].into(),
        statement: inflow,
        resource: account,
        committed_at: day(FUNDED_ON),
        due_date: day(15),
        magnitude: Some(FUNDED),
        dependencies: [].into(),
        recorded_at: day(FUNDED_ON),
    });
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    journal.push(settling(
        *admitted
            .commitments
            .last()
            .expect("the fund was just admitted"),
        SETTLED_ON,
        SETTLED_ON,
    ));
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    Ok(Base {
        files: Files {
            journal,
            lineage: Vec::new(),
            worlds: Vec::new(),
        },
        account,
        merchant,
        customer,
        outflow,
    })
}

impl Base {
    /// One outflow against the account, committed on the day the arrangement pins and recorded on
    /// whichever day the record holding it learned of it.
    fn spending(&self, magnitude: u128, recorded_on: u8) -> Admission {
        Admission::Commitment {
            accountable: self.merchant,
            executors: [self.merchant].into(),
            beneficiaries: [self.customer].into(),
            statement: self.outflow,
            resource: self.account,
            committed_at: day(COMMITTED_ON),
            due_date: day(15),
            magnitude: Some(magnitude),
            dependencies: [].into(),
            recorded_at: day(recorded_on),
        }
    }
}

/// A record that authored the four entries and both intentions itself.
///
/// The four go in on **one day**, because a relay learns everything at the instant it was shown and the
/// two must be comparable. See the note at the top of the module.
pub fn authoring(records_on: u8, decides_at: u8) -> Result<Files, SubjectError> {
    let base = base()?;
    let mut canon = Canon::new(ResidentHistory::new());
    let mut journal = base.files.journal.clone();
    let mut admitted = journal::replay(&mut canon, &journal)?;

    journal.push(Admission::Agent {
        label: "analyst".into(),
        recorded_at: day(records_on),
    });
    journal.push(base.spending(THERE_SPENDS, records_on));
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let analyst = *admitted.agents.last().expect("the party was just admitted");
    let spent = *admitted
        .commitments
        .last()
        .expect("the outflow was just admitted");

    journal.push(settling(spent, COMMITTED_ON, records_on));
    journal.push(base.spending(SPARE, records_on));
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let spare = *admitted
        .commitments
        .last()
        .expect("the open outflow was just admitted");

    let mut lineage = Lineage::new();
    let founding = Taken::claimed(
        Decision::Genesis {
            known_at: day(decides_at),
            selection: [].into(),
        },
        analyst,
        &admitted,
    )?;

    lineage::decide(canon.history(), &mut lineage, &founding.decision)?;

    let forking = Taken::claimed(
        Decision::Fork {
            extends: lineage
                .decided()
                .last()
                .ok_or(SubjectError::NothingDecided)?
                .id(),
            omitted: BTreeSet::new(),
            introduced: [spare].into(),
        },
        analyst,
        &admitted,
    )?;

    lineage::decide(canon.history(), &mut lineage, &forking.decision)?;

    kept(&journal, &[founding, forking])
}

/// A record that reached the same five worlds having never met anybody.
///
/// Experiment 09's question asked of intentions rather than of knowledge, and it is what decides
/// whether a reader of a record that took is **misled** or merely uninformed. It admits the same four
/// entries at the same instant and takes the same three decisions — by admitting and deciding
/// outright, with no crossing and no translation anywhere in the path.
///
/// Built along a different route from [`shown`] on purpose. A comparison between two records the same
/// function produced is a comparison of that function with itself.
pub fn reasoning_alone() -> Result<Files, SubjectError> {
    let base = base()?;
    let here = deciding_alone()?;

    let mut canon = Canon::new(ResidentHistory::new());
    let mut journal = here.journal.clone();
    let mut admitted = journal::replay(&mut canon, &journal)?;

    journal.push(Admission::Agent {
        label: "analyst".into(),
        recorded_at: day(SHOWN_ON),
    });
    journal.push(base.spending(THERE_SPENDS, SHOWN_ON));
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let spent = *admitted
        .commitments
        .last()
        .expect("the outflow was just admitted");

    journal.push(settling(spent, COMMITTED_ON, SHOWN_ON));
    journal.push(base.spending(SPARE, SHOWN_ON));
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let spare = *admitted
        .commitments
        .last()
        .expect("the open outflow was just admitted");

    let mut lineage = Lineage::new();
    let mut decisions = here.lineage.clone();

    // Applied before the fork is written, because a world is derived: what the fork extends has no
    // identity until the genesis has produced one.
    let founding = Decision::Genesis {
        known_at: day(RETAKES_AT),
        selection: BTreeSet::new(),
    };
    lineage::decide(canon.history(), &mut lineage, &founding)?;
    decisions.push(Taken::now(founding, &admitted)?);

    let forking = Decision::Fork {
        extends: lineage
            .decided()
            .last()
            .ok_or(SubjectError::NothingDecided)?
            .id(),
        omitted: BTreeSet::new(),
        introduced: [spare].into(),
    };
    lineage::decide(canon.history(), &mut lineage, &forking)?;
    decisions.push(Taken::now(forking, &admitted)?);

    kept(&journal, &decisions)
}

/// The receiving record before it is shown anything: its own outflow, its own Event, one decision.
pub fn deciding_alone() -> Result<Files, SubjectError> {
    let base = base()?;
    let mut canon = Canon::new(ResidentHistory::new());
    let mut journal = base.files.journal.clone();
    let mut admitted = journal::replay(&mut canon, &journal)?;

    journal.push(base.spending(HERE_SPENDS, ORIGIN_RECORDS_ON));
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let spent = *admitted
        .commitments
        .last()
        .expect("the outflow was just admitted");

    journal.push(settling(spent, COMMITTED_ON, RECORDS_ON));
    journal::replay_remaining(&mut canon, &journal, &mut admitted)?;

    let founding = Taken::now(
        Decision::Genesis {
            known_at: day(DECIDES_AT),
            selection: [].into(),
        },
        &admitted,
    )?;

    kept(&journal, &[founding])
}

/// The Event settling one commitment: when it happened, and when the record learned of it.
pub fn settling(commitment: CommitmentId, occurred_on: u8, recorded_on: u8) -> Admission {
    Admission::Event {
        commitment,
        observation: FULFILLING.into(),
        occurred_at: day(occurred_on),
        recorded_at: day(recorded_on),
    }
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

pub fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
