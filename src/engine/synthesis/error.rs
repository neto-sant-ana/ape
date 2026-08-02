//! Why a transfer could not be analysed at all.
//!
//! Nothing here says that a transfer may not be applied — that is a conflict, and a conflict
//! is a result. These are the cases where no analysis exists to return.
//!
//! `IncoherentBase` is the difference between the two made precise. Every conflict names an
//! invariant the resulting world would break, and an incoherent Base breaks none: it means
//! there is no intentional difference to speak of, because without a common ancestor an
//! absence is not a decision and a decision is not the Target's to receive. Reporting it as
//! a conflict would put a precondition of the operation among the findings of an operation
//! that never ran.
//!
//! A Thesis the archive does not hold has no lineage to establish, and a commitment canonical
//! history does not hold cannot have its dependencies read. Both are the same kind of
//! silence, which is why they live here rather than in a report.

use crate::engine::thesis::{ThesisError, ThesisId};

#[derive(Debug, ::thiserror::Error)]
pub enum SynthesisError {
    /// The fields carry the `_thesis` suffix because `thiserror` reads a field named `source`
    /// as the underlying cause of the error, and these are Theses rather than causes.
    #[error("thesis {base} is not a common ancestor of {source_thesis} and {target_thesis}")]
    IncoherentBase {
        base: ThesisId,
        source_thesis: ThesisId,
        target_thesis: ThesisId,
    },

    #[error(transparent)]
    Thesis(#[from] ThesisError),
}
