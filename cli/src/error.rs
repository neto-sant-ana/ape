//! What the application refuses, as distinct from what the engine refuses.
//!
//! An engine error carried through unchanged says where it came from; what this adds is
//! the cases the engine has no opinion about, because they arise from the application
//! reading a world it assembled itself.

use ape::canon::CanonError;
use ape::engine::hermeneia::HermeneiaError;
use ape::kernel::entities::CommitmentId;

#[derive(Debug, thiserror::Error)]
pub enum JournalError {
    #[error(transparent)]
    Canon(#[from] CanonError),

    /// A journal the engine refuses to admit. The field is named because a journal is read
    /// long after it was written, and "invalid" alone sends the reader back to the bytes.
    #[error("{field} in the journal is not admissible: {cause}")]
    Malformed { field: &'static str, cause: String },

    /// An address naming an entry the journal does not hold. Something referred to this
    /// journal and meant a different one, and admitting everything instead would answer with
    /// a world that was never reasoned about.
    #[error("the journal holds no entry {0}")]
    UnknownEntry(crate::journal::EntryId),

    /// An address the replay is already past. The sequence being replayed alongside the
    /// journal disagrees with it about order, and continuing would hand out knowledge the
    /// coordinate says had not arrived.
    #[error("entry {0} was admitted before the point the journal has reached")]
    EntryAlreadyPassed(crate::journal::EntryId),
}

impl JournalError {
    pub(crate) fn malformed(field: &'static str, cause: impl std::fmt::Display) -> Self {
        Self::Malformed {
            field,
            cause: cause.to_string(),
        }
    }
}

/// A subject that could not be arranged. Fatal to any experiment using it: there is nothing
/// to observe, because what was supposed to be observed was never built.
#[derive(Debug, thiserror::Error)]
pub enum SubjectError {
    #[error(transparent)]
    Journal(#[from] JournalError),

    #[error(transparent)]
    Lineage(#[from] LineageError),

    /// A replay that admitted nothing, asked for the entry it ended at.
    #[error("a decision was taken after a replay that admitted nothing")]
    NothingAdmitted,

    /// A decision that extends something, taken over a lineage holding no world.
    #[error("a decision extends a world, and no world has been decided")]
    NothingDecided,
}

#[derive(Debug, thiserror::Error)]
pub enum ReadingError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),

    #[error(transparent)]
    Journal(#[from] JournalError),

    #[error(transparent)]
    Lineage(#[from] LineageError),

    #[error(transparent)]
    Thesis(#[from] ape::engine::thesis::ThesisError),

    #[error(transparent)]
    Hermeneia(#[from] HermeneiaError),

    #[error(transparent)]
    Level(#[from] LevelError),

    /// A repository whose lineage decides nothing. There is no world to read.
    #[error("the lineage is empty")]
    EmptyLineage,

    /// The decisions produce a different number of worlds than the repository says it reached.
    #[error("the decisions produce {derived} worlds, and {recorded} were recorded")]
    LineageLengthDisagrees { derived: usize, recorded: usize },

    /// A world the decisions produce is not the world recorded in its place.
    ///
    /// The coordinate is named because a reader told only that a world came back different
    /// has to go and find out how, and the repository already knows.
    #[error("world {position} disagrees with what was recorded, in {coordinate}")]
    WorldDisagrees {
        position: usize,
        coordinate: &'static str,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum LineageError {
    #[error(transparent)]
    Thesis(#[from] ape::engine::thesis::ThesisError),

    /// Rebuilding a lineage admits the journal in step with it, so a journal that refuses to
    /// replay is a lineage that cannot be reached.
    #[error(transparent)]
    Journal(#[from] JournalError),

    #[error("{0:?} is not readable as an instant")]
    UnreadableInstant(String),

    /// A decision naming a world the lineage did not produce.
    ///
    /// This replaces two errors that said a lineage began in the middle. Once a decision names
    /// what it extends, beginning in the middle is not a separate case: a first decision that
    /// is not a genesis names a world nothing has produced yet, and so does a decision whose
    /// ancestor came back different. The identity is named because a reader has no other way
    /// to tell those apart.
    #[error("the decision extends world {thesis}, which the lineage does not hold")]
    ExtendsUnknownWorld {
        thesis: ape::engine::thesis::ThesisId,
    },

    /// A decision written down before anything had been admitted, which no application takes:
    /// a world selects commitments, and a commitment is knowledge.
    #[error("a decision was taken before anything was admitted")]
    DecidedBeforeAnythingWasAdmitted,

    /// The journal offered knowledge the decision does not say it was taken against.
    ///
    /// Kept apart from its mirror because the two say different things about the repository:
    /// one has a journal that grew under a coordinate, the other a coordinate that outran its
    /// journal.
    #[error("entry {entry} was admitted, and the decision was not taken against it")]
    UnwitnessedKnowledge { entry: crate::journal::EntryId },

    #[error("the decision was taken against entry {entry}, which the journal does not offer")]
    WitnessedKnowledgeAbsent { entry: crate::journal::EntryId },
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("the repository could not be read or written: {0}")]
    Io(#[from] std::io::Error),

    /// A journal on disk the application cannot read. Kept distinct from an unreadable
    /// directory because the two say different things: one is the repository missing, the
    /// other is the repository present and no longer meaning what it did.
    #[error("the journal is not readable as one: {0}")]
    Encoding(#[from] serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum LevelError {
    #[error(transparent)]
    Hermeneia(#[from] HermeneiaError),

    /// A projection named a commitment the knowledge behind it could not resolve, which
    /// means the two were not read from the same world.
    #[error("commitment {0} is projected but absent from knowledge")]
    UnknownCommitment(CommitmentId),
}
