//! What the application refuses, as distinct from what the engine refuses.
//!
//! An engine error carried through unchanged says where it came from; what this adds is
//! the cases the engine has no opinion about, because they arise from the application
//! reading a world it assembled itself.
//!
//! Earned by: nothing — every refusal here belongs to the module that raises it, and is cited there.

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

    /// An arrangement that writes a repository and reads it back, which the coordination
    /// experiment is the first to need: before it, a subject was built in one process and
    /// persisted once at the end.
    #[error(transparent)]
    Repository(#[from] RepositoryError),

    #[error(transparent)]
    Reading(#[from] ReadingError),

    /// A replay that admitted nothing, asked for the entry it ended at.
    #[error("a decision was taken after a replay that admitted nothing")]
    NothingAdmitted,

    /// A decision that extends something, taken over a lineage holding no world.
    #[error("a decision extends a world, and no world has been decided")]
    NothingDecided,

    #[error(transparent)]
    Synthesis(#[from] ape::engine::synthesis::SynthesisError),

    /// An arrangement that meant to carry a transfer, over a report that refuses it. Applying it
    /// anyway would build the world Synthesis just said was unavailable.
    #[error("the transfer the arrangement carries is not applicable")]
    TransferNotApplicable,
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

    /// The journal offers an entry the record does not claim to hold.
    ///
    /// The pair below is the witness's pair one level up, and it exists for the half of a journal no
    /// witness reaches: what a record admitted after its last decision. Both name the entry, because
    /// the two send a reader to opposite places — one to a journal that grew, one to a journal that
    /// was cut.
    #[error("the journal offers entry {entry}, which the record does not claim to hold")]
    UnheldKnowledge { entry: crate::journal::EntryId },

    #[error("the record claims to hold entry {entry}, which the journal does not offer")]
    HeldKnowledgeAbsent { entry: crate::journal::EntryId },
}

/// What a transfer asked of a repository could not answer.
///
/// The two halves are kept apart because they fail for different reasons. A repository that
/// cannot be rebuilt says nothing about the transfer; a transfer refused over a repository that
/// rebuilt says nothing about the repository.
#[derive(Debug, thiserror::Error)]
pub enum TransferError {
    #[error(transparent)]
    Reading(#[from] ReadingError),

    #[error(transparent)]
    Synthesis(#[from] ape::engine::synthesis::SynthesisError),
}

/// What a party could not put back.
///
/// Every one of these leaves the repository as it was. A refusal that had written half of a merge
/// would be the tear Phase 1 measured, produced by the code meant to prevent it.
#[derive(Debug, thiserror::Error)]
pub enum ConvergeError {
    #[error(transparent)]
    Reading(#[from] ReadingError),

    #[error(transparent)]
    Lineage(#[from] LineageError),

    #[error(transparent)]
    Repository(#[from] RepositoryError),

    /// The journal a party read is not the one it is writing on top of.
    ///
    /// Shaped after the Canon's `UnexpectedHead` and refused for the same reason one layer up:
    /// knowledge is a sequence, every standing decision names the entries that stood when it was
    /// taken, and a journal whose earlier entries moved makes those decisions disagree with it.
    /// The party re-reads and admits again.
    #[error(
        "the journal diverges at entry {position}: this party holds {expected}, and {found} is there"
    )]
    Diverged {
        position: usize,
        expected: crate::journal::EntryId,
        found: crate::journal::EntryId,
    },

    /// The two journals hold the same entry and disagree about when it was recorded.
    ///
    /// Separate from [`ConvergeError::Diverged`] because the address is the *same*, and a refusal
    /// naming two identical identities sends a reader looking for a difference that is not there.
    /// A recording instant belongs to no identity, so this is the one way two journals can be a
    /// different journal and agree entry for entry.
    ///
    /// It has to be a refusal rather than a choice. The instant is what a cut resolves its Event
    /// head against, so keeping either party's would move a world the other party decided — and the
    /// record has no representation for having been told twice. The party re-reads and decides
    /// again, which is the recovery every other refusal here offers.
    #[error(
        "the journal disagrees about entry {position}, {entry}: \
         this party recorded it at {held} and {arrived} is there"
    )]
    RecordedDifferently {
        position: usize,
        entry: crate::journal::EntryId,
        held: String,
        arrived: String,
    },

    /// Decisions that cannot be put in an order, because none of what is left extends a world
    /// anything produced. A lineage whose worlds do not reach a genesis is not a lineage.
    #[error("{remaining} decisions extend worlds nothing in the merge produces")]
    NothingApplies { remaining: usize },
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

    /// The same disagreement, once its cause is known to be a readmission.
    ///
    /// Kept apart because the two send a reader to opposite places. `WitnessedKnowledgeAbsent` is
    /// about a journal that lacks something; this is about a journal that holds it twice, where the
    /// entry named by the first would be **innocent** — the replay resolved an address to its
    /// earlier occurrence and stopped before reaching what was learned in between.
    ///
    /// Both are needed. A witness can name an entry no journal ever held, and that is not this.
    #[error(
        "the decision names entry {readmitted}, which the journal admits more than once; resolving it to the first occurrence leaves witnessed entry {entry} unadmitted"
    )]
    ReadmittedEntryIsAmbiguous {
        readmitted: crate::journal::EntryId,
        entry: crate::journal::EntryId,
    },

    /// A decision attributed to a party the knowledge behind it does not hold.
    ///
    /// One refusal for two things, because at this coordinate they are one: an identity that names
    /// no agent, and an agent admitted after the decision that claims it. Both are claims about
    /// somebody the decision could not have known.
    #[error("the decision is attributed to {agent}, whom nothing had admitted when it was taken")]
    DeciderNotKnown {
        agent: ape::kernel::entities::AgentId,
    },
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

    /// Somebody else wrote into the generation this write had prepared, before it was turned.
    ///
    /// The whole of what makes a whole write a compare-and-swap rather than a compare followed by
    /// a swap. Turning would publish a repository this writer did not write — the other writer's
    /// state, or a mixture of the two — and both of those are states a *reader* cannot tell from a
    /// finished commit.
    ///
    /// It names the generation because that is where the two writers met, and because a writer
    /// that reads this has to decide whether to prepare again, which is a decision about where.
    /// What it does not name is the other writer: nothing on disk says who that was, and the
    /// contention experiment left the question of whether a repository should be able to say.
    #[error("the generation prepared at {generation} was written over before it could be turned")]
    Contended { generation: String },

    /// A journal a whole write cannot admit, met while deriving what the record claims to hold.
    ///
    /// The one refusal here that is not about the directory. A whole write derives the custody claim
    /// from the journal it is handed, so a journal that does not admit is refused at the write rather
    /// than at the next read — which is a narrowing of what `write_whole` accepts, and is stated in
    /// the module rather than discovered.
    #[error(transparent)]
    Journal(#[from] JournalError),
}

#[derive(Debug, thiserror::Error)]
pub enum LevelError {
    #[error(transparent)]
    Hermeneia(#[from] HermeneiaError),

    /// A projection named a commitment the knowledge behind it could not resolve, which
    /// means the two were not read from the same world.
    #[error("commitment {0} is projected but absent from knowledge")]
    UnknownCommitment(CommitmentId),

    /// Movements that sum past what a count can hold, folding a level.
    ///
    /// The application's own arithmetic, so the refusal is the application's. Under `f64` the sum
    /// would have gone to infinity and been compared as a level; a count refuses instead.
    #[error("the movements on resource instance {0} sum beyond what a count can hold")]
    OutOfRange(ape::kernel::entities::ResourceInstanceId),
}
