//! The lineage: the decisions that produced a set of worlds, and the order they were taken in.
//!
//! A Thesis is not admitted and does not reach canonical history. It is decided — which
//! commitments a world selects, and which instant it recognizes — and everything else about
//! it follows: the cut resolves its head from the instant, the selection absorbs whatever
//! that cut froze, and the identity is derived from the result.
//!
//! So a lineage is persisted as the decisions that produce it rather than as the worlds they
//! produced. What a decision may *name* is a different matter: an identity written down as a
//! reference is an edge of a graph whose vertices are content-addressed, not an answer cached
//! beside its question. The distinction holds because nothing reads such a name back as a
//! source — a `Thesis` cannot be deserialized, so a named world is one that has to be produced
//! again before it can be found.
//!
//! # A lineage is not a line
//!
//! Two worlds may extend the same parent, and neither is the other's successor. Order alone
//! cannot say which of them a later decision is about, so a decision that extends something
//! names what it extends, and the worlds already decided are held where an identity resolves
//! them — which is what a `ThesisArchive` is for.
//!
//! Nothing in this laboratory needed one until a decision had to be placed among worlds rather
//! than after them. The archive is therefore built as the decisions produce it, never opened,
//! and it refuses a child whose parent is absent: an archive with a hole in it would end an
//! ancestry walk exactly where a genesis ends one.
//!
//! # A decision is not enough to place a decision
//!
//! An instant is what a decision *names*; the knowledge that instant resolves against is
//! whatever the journal held when it was applied. Those are one body only while nothing was
//! admitted after the decision and within the instant it names — and where something was, the
//! same decision applied later builds a world nobody decided.
//!
//! So a persisted decision is a [`Taken`]: the decision, plus the entry that was the journal's
//! most recent one when it was taken. [`rebuild`] is what reads it, admitting the journal in
//! step with the lineage rather than wholly before it.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use ape::canon::{Canon, CanonicalKnowledge};
use ape::engine::synthesis::{ApplicabilityStatus, synthesize};
use ape::engine::thesis::{
    ForkInput, GenesisInput, KnowledgeCut, Thesis, ThesisArchive, ThesisId, ThesisLookup,
};
use ape::kernel::entities::CommitmentId;
use ape::kernel::value_objects::Date;

use crate::archive::ResidentArchive;
use crate::error::LineageError;
use crate::history::ResidentHistory;
use crate::journal::{self, Admission, EntryId, Replayed};

/// One decision about which world is being reasoned about.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "decides", rename_all = "kebab-case")]
pub enum Decision {
    /// The first world: what it selects, and the instant it is taken at.
    ///
    /// The selection is the *proposal*. Whatever the cut already froze is added to it by
    /// the engine, so what is written down is what was asked for rather than what resulted.
    Genesis {
        known_at: String,
        selection: BTreeSet<CommitmentId>,
    },

    /// Recognizing later history, up to an instant.
    Advance { extends: ThesisId, known_at: String },

    /// A different intention under the cut already recognized.
    ///
    /// Both halves state an outcome rather than a transition, which is the engine's own
    /// tolerance and the reason they are written down as asked for. What a fork changed is
    /// not recoverable by comparing two selections: a commitment absent from a child was
    /// either omitted by this decision or never open to begin with.
    Fork {
        extends: ThesisId,
        omitted: BTreeSet<CommitmentId>,
        introduced: BTreeSet<CommitmentId>,
    },
}

/// A decision, and the point in the sequence of admissions at which it was taken.
///
/// `after` addresses the journal entry that was the most recent one at that point. It is a
/// reference to knowledge, not a cached derivation of one: a cut, a partition and an identity
/// are all still recomputed, and what this supplies is the only thing that cannot be — *which*
/// knowledge to recompute them against.
///
/// It is not the resolved head, and could not be. The genesis of this experiment resolves an
/// empty chain, which `KnowledgeCut::within` cannot be handed; and the entry a decision
/// follows is frequently not an Event at all. What is recorded is a position in the sequence
/// that produces knowledge, and the head is derived from it like everything else.
///
/// The two halves are flattened into one object on purpose. A decision and where it was taken
/// are one record — a decision filed under a coordinate it did not have would be a lineage
/// that reads back as a different lineage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Taken {
    #[serde(flatten)]
    pub decision: Decision,
    pub after: EntryId,
    /// Every entry that had been admitted when the decision was taken.
    ///
    /// This is [`Taken::after`] written a second time, in a form that says the same thing
    /// about the whole prefix rather than about its end — and a second representation of one
    /// fact is the only thing a reader can compare. A repository holding only what produces
    /// its worlds cannot contradict itself, which is another way of saying it cannot notice
    /// anything.
    ///
    /// It is a set rather than a sequence on purpose. An entry's identity is its content, so
    /// two admissions that refer to nothing of each other's carry nothing in their order; a
    /// witness over the ordered prefix would refuse a repository whose worlds are identical.
    /// What it must disagree with is a prefix that is not the one the decision was taken
    /// against, which is a question about membership.
    pub witness: BTreeSet<EntryId>,
    /// The question a transfer answered, where the decision was chosen from one.
    ///
    /// Absent from every decision that was not, which is why it is optional rather than empty:
    /// a planner who decided for their own reasons has nothing to say here, and saying nothing is
    /// different from saying "from nowhere".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Adoption>,
}

/// Where an intention came from: the two worlds a transfer was measured between.
///
/// The Target is absent because the decision already names it — `extends` is the Target, and a
/// second copy of it would be a second place for one fact to live.
///
/// This is neither an instruction nor a witness in the corroboration experiment's sense. Nothing
/// derives from it, so a reader that ignores it loses nothing; and it is not a second
/// representation of what the decision produced, because the decision states that outright. It is
/// a **claim**, and it is checkable only to the extent that it predicts something derivable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Adoption {
    pub base: ThesisId,
    pub source: ThesisId,
}

impl Taken {
    /// A decision, witnessed by the entries that stood when it was taken.
    ///
    /// Both halves come from one reading of one journal, so they cannot disagree at the
    /// moment they are written. Corroboration is a property of the read.
    pub fn now(decision: Decision, admitted: &Replayed) -> Result<Self, LineageError> {
        Ok(Self {
            decision,
            after: admitted
                .entries
                .last()
                .cloned()
                .ok_or(LineageError::DecidedBeforeAnythingWasAdmitted)?,
            witness: admitted.entries.iter().cloned().collect(),
            from: None,
        })
    }

    /// The same, for a decision chosen from a transfer, carrying the question it answered.
    pub fn adopting(
        decision: Decision,
        admitted: &Replayed,
        from: Adoption,
    ) -> Result<Self, LineageError> {
        Ok(Self {
            from: Some(from),
            ..Self::now(decision, admitted)?
        })
    }
}

/// The worlds a lineage has produced: resolvable by identity, and in the order decided.
///
/// The two halves answer different questions and neither derives the other cheaply. A decision
/// names the world it extends, which is a question about identity; a reader compares what a
/// repository recorded against what it produced, which is a question about order. Holding one
/// and searching it for the other would be reinventing the port the engine already defines.
///
/// The archive is filled as worlds are decided, which is the only way it can be filled at all —
/// a `Thesis` does not deserialize, so an archive is never opened, only rebuilt.
#[derive(Debug, Default)]
pub struct Lineage {
    archive: ResidentArchive,
    decided: Vec<Thesis>,
}

impl Lineage {
    pub fn new() -> Self {
        Self::default()
    }

    /// Every world, in the order its decision was taken.
    pub fn decided(&self) -> &[Thesis] {
        &self.decided
    }

    /// The worlds by identity, as Synthesis reads them.
    pub fn archive(&self) -> &ResidentArchive {
        &self.archive
    }

    pub fn is_empty(&self) -> bool {
        self.decided.is_empty()
    }

    /// The world a decision names, or a refusal naming what could not be found.
    ///
    /// The refusal is the corroboration this reference buys. An identity is derived from the
    /// content of the world it addresses, so a repository whose earlier decisions no longer
    /// produce the world a later one extends cannot resolve it — and says which one.
    fn extended(&self, thesis: ThesisId) -> Result<Thesis, LineageError> {
        self.archive
            .thesis(thesis)
            .ok_or(LineageError::ExtendsUnknownWorld { thesis })
    }

    fn record(&mut self, thesis: Thesis) -> Result<(), LineageError> {
        self.archive.put_thesis(thesis.clone())?;
        self.decided.push(thesis);

        Ok(())
    }
}

/// Extend a lineage by one decision, returning what history imposed on the world it makes.
///
/// An application calls this as each decision is taken; a reconstruction calls it for every
/// decision at once, through [`replay`]. Which is to say the two run the same code and
/// differ only in *when* — and a decision resolves its cut against the knowledge standing at
/// the moment it is applied.
///
/// A genesis imposes nothing, and that is a statement about the report rather than about the
/// world: whatever its cut froze is absorbed into the selection with no one told, because
/// there is no prior intention for it to have been absent from.
pub fn decide<K: CanonicalKnowledge>(
    knowledge: &K,
    lineage: &mut Lineage,
    decision: &Decision,
) -> Result<BTreeSet<CommitmentId>, LineageError> {
    let (thesis, imposed) = match decision {
        Decision::Genesis {
            known_at,
            selection,
        } => (
            Thesis::genesis(
                knowledge,
                GenesisInput {
                    cut: KnowledgeCut::at(knowledge, date(known_at)?),
                    selection: selection.clone(),
                },
            )?,
            BTreeSet::new(),
        ),

        Decision::Advance { extends, known_at } => {
            let advancement = lineage
                .extended(*extends)?
                .advance(knowledge, KnowledgeCut::at(knowledge, date(known_at)?))?;

            let imposed = advancement.imposed().collect();

            (advancement.into_thesis(), imposed)
        }

        Decision::Fork {
            extends,
            omitted,
            introduced,
        } => (
            lineage.extended(*extends)?.fork(
                knowledge,
                ForkInput {
                    omitted: omitted.clone(),
                    introduced: introduced.clone(),
                },
            )?,
            BTreeSet::new(),
        ),
    };

    lineage.record(thesis)?;

    Ok(imposed)
}

/// Apply a whole lineage against knowledge as it stands, oldest first.
///
/// Sound only where nothing was admitted between the first decision and the last within an
/// instant one of them names. Where something was, every decision after it resolves against
/// more knowledge than it had, and what comes back is a lineage of worlds that were never
/// decided. [`rebuild`] is what a repository is read through for that reason.
pub fn replay<K: CanonicalKnowledge>(
    knowledge: &K,
    decisions: &[Decision],
) -> Result<Lineage, LineageError> {
    let mut lineage = Lineage::new();

    for decision in decisions {
        decide(knowledge, &mut lineage, decision)?;
    }

    Ok(lineage)
}

/// Rebuild a lineage from the two sequences a repository holds, oldest first.
///
/// The journal is admitted *in step with* the lineage: up to the entry each decision was
/// taken after, then the decision, then on. That is the whole of the repair, and it is a
/// statement about the order between two files rather than about either of them — replaying
/// one entirely and then the other is what resolved a cut against knowledge its decision
/// never had.
///
/// The rest of the journal is admitted at the end, so that what a caller reads the lineage
/// against is canonical history entire. A world does not learn from it: its cut is a value,
/// fixed when the decision was applied.
pub fn rebuild(
    canon: &mut Canon<ResidentHistory>,
    journal: &[Admission],
    decisions: &[Taken],
) -> Result<Lineage, LineageError> {
    let mut admitted = Replayed::default();
    let mut lineage = Lineage::new();

    for taken in decisions {
        journal::replay_through(canon, journal, &mut admitted, &taken.after)?;

        corroborate(&admitted, taken)?;
        corroborate_adoption(canon.history(), &lineage, taken)?;

        decide(canon.history(), &mut lineage, &taken.decision)?;
    }

    journal::replay_remaining(canon, journal, &mut admitted)?;

    Ok(lineage)
}

/// Weigh what the journal offered against what the decision says it was taken after.
///
/// The two are derived from the same journal by different routes — one by walking to an
/// address, the other written down when the decision was taken — so a repository whose journal
/// or whose coordinate has moved makes them disagree. What is named is the entry that
/// disagrees, because "the repository is invalid" sends a reader back to the bytes.
fn corroborate(admitted: &Replayed, taken: &Taken) -> Result<(), LineageError> {
    let offered: BTreeSet<&EntryId> = admitted.entries.iter().collect();
    let witnessed: BTreeSet<&EntryId> = taken.witness.iter().collect();

    if let Some(unexpected) = offered.difference(&witnessed).next() {
        return Err(LineageError::UnwitnessedKnowledge {
            entry: (*unexpected).clone(),
        });
    }

    if let Some(missing) = witnessed.difference(&offered).next() {
        return Err(LineageError::WitnessedKnowledgeAbsent {
            entry: (*missing).clone(),
        });
    }

    Ok(())
}

/// Weigh a claim about where an intention came from against what the decision says it asked for.
///
/// The claim names a Base and a Source; the decision names the Target and the change. So the
/// question can be asked again, and a transfer that comes back different is a claim the record
/// contradicts on its face.
///
/// What this cannot reach is a claim naming a **different Source whose transfer is the same**. Two
/// worlds that resolve to one request against this Target are indistinguishable here for exactly
/// the reason they are indistinguishable everywhere: the request is all that arrived.
fn corroborate_adoption<K: CanonicalKnowledge>(
    knowledge: &K,
    lineage: &Lineage,
    taken: &Taken,
) -> Result<(), LineageError> {
    let Some(from) = &taken.from else {
        return Ok(());
    };

    let Decision::Fork {
        extends,
        omitted,
        introduced,
    } = &taken.decision
    else {
        return Err(LineageError::AdoptedWithoutForking);
    };

    let report = synthesize(
        lineage.archive(),
        knowledge,
        from.base,
        from.source,
        *extends,
    )?;

    let ApplicabilityStatus::Applicable { transfer, .. } = report.status() else {
        return Err(LineageError::AdoptedWhatWasRefused { donor: from.source });
    };

    let asked = (
        transfer.remove().collect::<BTreeSet<_>>(),
        transfer.introduce().collect::<BTreeSet<_>>(),
    );

    if asked != (omitted.clone(), introduced.clone()) {
        return Err(LineageError::AdoptionDisagrees { donor: from.source });
    }

    Ok(())
}

fn date(value: &str) -> Result<Date, LineageError> {
    Date::parse(value).map_err(|_| LineageError::UnreadableInstant(value.to_owned()))
}
