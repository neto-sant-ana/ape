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

    /// An advancement with nothing to advance. The lineage on disk begins in the middle,
    /// which no sequence of decisions could have produced.
    #[error("the lineage advances before it begins")]
    AdvancedWithoutGenesis,

    /// The same, for a fork. Kept distinct because the two say which decision was orphaned,
    /// and a lineage read long after it was written is read by someone who was not there.
    #[error("the lineage forks before it begins")]
    ForkedWithoutParent,
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
