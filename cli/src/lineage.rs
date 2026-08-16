//! The lineage: the decisions that produced a Thesis, in the order they were taken.
//!
//! A Thesis is not admitted and does not reach canonical history. It is decided — which
//! commitments a world selects, and which instant it recognizes — and everything else about
//! it follows: the cut resolves its head from the instant, the selection absorbs whatever
//! that cut froze, and the identity is derived from the result.
//!
//! So a lineage is persisted the way knowledge is: as the sequence that produces it, never
//! as what it produced. The same discipline for the same reason — a stored `ThesisId` would
//! be an answer kept beside the question, free to disagree with it after any change to how
//! identity is derived.
//!
//! Nothing here needs a `ThesisArchive`. An archive holds Theses so they can be resolved by
//! identity, and a repository that keeps the decisions resolves them by replaying instead.
//! That is a consequence of this experiment's boundary rather than a claim about the port:
//! ancestry walked across processes may well want one, and nothing here has needed it.
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
use ape::engine::thesis::{ForkInput, GenesisInput, KnowledgeCut, Thesis};
use ape::kernel::entities::CommitmentId;
use ape::kernel::value_objects::Date;

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
    Advance { known_at: String },

    /// A different intention under the cut already recognized.
    ///
    /// Both halves state an outcome rather than a transition, which is the engine's own
    /// tolerance and the reason they are written down as asked for. What a fork changed is
    /// not recoverable by comparing two selections: a commitment absent from a child was
    /// either omitted by this decision or never open to begin with.
    Fork {
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
        })
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
    lineage: &mut Vec<Thesis>,
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

        Decision::Advance { known_at } => {
            let advancement = lineage
                .last()
                .ok_or(LineageError::AdvancedWithoutGenesis)?
                .advance(knowledge, KnowledgeCut::at(knowledge, date(known_at)?))?;

            let imposed = advancement.imposed().collect();

            (advancement.into_thesis(), imposed)
        }

        Decision::Fork {
            omitted,
            introduced,
        } => (
            lineage
                .last()
                .ok_or(LineageError::ForkedWithoutParent)?
                .fork(
                    knowledge,
                    ForkInput {
                        omitted: omitted.clone(),
                        introduced: introduced.clone(),
                    },
                )?,
            BTreeSet::new(),
        ),
    };

    lineage.push(thesis);

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
) -> Result<Vec<Thesis>, LineageError> {
    let mut lineage: Vec<Thesis> = Vec::new();

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
) -> Result<Vec<Thesis>, LineageError> {
    let mut admitted = Replayed::default();
    let mut lineage: Vec<Thesis> = Vec::new();

    for taken in decisions {
        journal::replay_through(canon, journal, &mut admitted, &taken.after)?;

        corroborate(&admitted, taken)?;

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

fn date(value: &str) -> Result<Date, LineageError> {
    Date::parse(value).map_err(|_| LineageError::UnreadableInstant(value.to_owned()))
}
