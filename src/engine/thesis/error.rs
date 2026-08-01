//! Why a Thesis could not be derived, or interpreted.
//!
//! Every variant reports a request that could not produce an interpretable world, never an
//! operational judgment. A continuation that contradicts what history imposed is not
//! refused here.
//!
//! `Hermeneia` carries what it refused while folding a recognized chain. Deriving and
//! interpreting share one error because one operation does both: an interpretation resolves
//! the chain its cut recognizes and then folds it, and either half may refuse.
//!
//! The refusals separate by who got it wrong:
//! `FrozenPastOmitted` and `DanglingDependency`
//! Answer the caller: one names a commitment history made unavoidable, the other a
//! selection that names a dependency it does not select.
//!
//! `CutNotLater`, `HeadDoesNotDescend` and `HeadWithdrawn`
//! Answer an advancement that is not one: one that recognizes no later knowledge, one
//! reaching a head of another reach of history, one giving a head back.
//!
//! `CommitmentNotKnownAtCut`, `EventNotKnownAtCut`, `NoEventGroupAtCut` and `HeadPrecedesCut`
//! Answer a cut that does not describe one moment. The first two are anachronism, an
//! assertion recorded after the instant that claims to be recognized at it. The last two are
//! its mirror, retraction wearing the shape of a finer selection, and they refuse it one step
//! apart: a finer cut refines the group of *its own* instant, so `NoEventGroupAtCut` answers an
//! instant that has no group to refine, and `HeadPrecedesCut` answers a head belonging to a
//! group earlier than the one its instant addresses. Either way the cut would set aside Events
//! it already knew.
//!
//! `UnknownCommitment` and `UnknownEvent`
//! Report a reference canonical history does not hold, and each is a single refusal
//! whether the assertion was never admitted or the reference is simply wrong.
//!
//! `ParentNotArchived` and `UnknownThesis`
//! Answer a lineage that cannot be walked. Ancestry is resolved `parent` by `parent`, so a
//! stored child whose parent is absent would end that walk indistinguishably from a
//! genesis; the first refuses to create that hole, the second refuses to walk one — a
//! Thesis that was never stored, or an adapter that let a child in without its parent.
//!
//! `OmittedAndIntroduced` and `SelectionUnchanged`
//! Answer a request that states nothing: one both drops and selects the same commitment, the
//! other leaves the world exactly as its parent had it. A Thesis is produced by a change in
//! what is selected or in what is recognized, and an edge that carries neither would record a
//! decision the model cannot read back.

use crate::engine::hermeneia::HermeneiaError;

use super::ThesisId;

use crate::kernel::entities::{CommitmentId, EventId, IdentityError};

use crate::kernel::value_objects::Date;

#[derive(Debug, ::thiserror::Error)]
pub enum ThesisError {
    #[error("commitment {0} is selected but absent from canonical history")]
    UnknownCommitment(CommitmentId),

    #[error("event {0} is named as a head but absent from canonical history")]
    UnknownEvent(EventId),

    #[error("thesis {thesis} names parent {parent}, which the archive does not hold")]
    ParentNotArchived { thesis: ThesisId, parent: ThesisId },

    #[error("thesis {0} is absent from the archive")]
    UnknownThesis(ThesisId),

    #[error("commitment {0} belongs to the frozen causal past and may not be omitted")]
    FrozenPastOmitted(CommitmentId),

    #[error("commitment {dependent} depends on {dependency}, which the selection omits")]
    DanglingDependency {
        dependent: CommitmentId,
        dependency: CommitmentId,
    },

    #[error("commitment {0} is both omitted and introduced")]
    OmittedAndIntroduced(CommitmentId),

    #[error("the fork selects exactly what its parent selects; nothing would distinguish them")]
    SelectionUnchanged,

    #[error("event {target} does not descend from the recognized head {parent:?}")]
    HeadDoesNotDescend {
        parent: Option<EventId>,
        target: EventId,
    },

    #[error(
        "no event was recorded at {known_at:?}, so it addresses no group to refine; it resolves to the group of {addressed_at:?}, ending at {addressed:?}"
    )]
    NoEventGroupAtCut {
        known_at: Date,
        addressed: Option<EventId>,
        addressed_at: Option<Date>,
    },

    #[error("head {named} precedes the cut {addressed:?}, which its instant addresses")]
    HeadPrecedesCut {
        named: EventId,
        addressed: Option<EventId>,
    },

    #[error(
        "head {named} shares the instant of {addressed} without lying on the chain ending there"
    )]
    HeadDoesNotBelongToCut { named: EventId, addressed: EventId },

    /// Unreachable while a cut is resolved from its instant: a later instant cannot address an
    /// earlier chain, so no target can recognize nothing where its parent recognized something.
    /// Kept because the code has to branch somewhere, and a named refusal beats a Thesis holding a
    /// frozen past with no chain to have frozen it.
    #[error("the recognized head {parent} may not be given back by an advancement")]
    HeadWithdrawn { parent: EventId },

    #[error(
        "the cut ({target:?}, {target_head:?}) is not later than ({parent:?}, {parent_head:?}); advancement requires later knowledge"
    )]
    CutNotLater {
        parent: Date,
        parent_head: Option<EventId>,
        target: Date,
        target_head: Option<EventId>,
    },

    #[error(
        "commitment {commitment} was not knowledge at {known_at:?}, having been recorded at {recorded_at:?}"
    )]
    CommitmentNotKnownAtCut {
        commitment: CommitmentId,
        recorded_at: Date,
        known_at: Date,
    },

    #[error(
        "event {event} was not knowledge at {known_at:?}, having been recorded at {recorded_at:?}"
    )]
    EventNotKnownAtCut {
        event: EventId,
        recorded_at: Date,
        known_at: Date,
    },

    #[error(transparent)]
    Identity(#[from] IdentityError),

    #[error(transparent)]
    Hermeneia(#[from] HermeneiaError),
}
