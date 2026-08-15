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

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use ape::canon::CanonicalKnowledge;
use ape::engine::thesis::{GenesisInput, KnowledgeCut, Thesis};
use ape::kernel::entities::CommitmentId;
use ape::kernel::value_objects::Date;

use crate::error::LineageError;

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
}

/// Rebuild the lineage, oldest first.
///
/// The last Thesis is the world the repository currently reasons about; the ones before it
/// are how it got there, and Phase 2 needs the genesis to still be readable to show that a
/// cut cannot recognize what it predates.
pub fn replay<K: CanonicalKnowledge>(
    knowledge: &K,
    decisions: &[Decision],
) -> Result<Vec<Thesis>, LineageError> {
    let mut lineage: Vec<Thesis> = Vec::new();

    for decision in decisions {
        let thesis = match decision {
            Decision::Genesis {
                known_at,
                selection,
            } => Thesis::genesis(
                knowledge,
                GenesisInput {
                    cut: KnowledgeCut::at(knowledge, date(known_at)?),
                    selection: selection.clone(),
                },
            )?,

            Decision::Advance { known_at } => lineage
                .last()
                .ok_or(LineageError::AdvancedWithoutGenesis)?
                .advance(knowledge, KnowledgeCut::at(knowledge, date(known_at)?))?
                .into_thesis(),
        };

        lineage.push(thesis);
    }

    Ok(lineage)
}

fn date(value: &str) -> Result<Date, LineageError> {
    Date::parse(value).map_err(|_| LineageError::UnreadableInstant(value.to_owned()))
}
