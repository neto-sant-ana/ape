//! Whether a declared Base earns the reading Synthesis gives it.
//!
//! ```text
//! coherent_base(Base, Source, Target) iff
//!     Base ∈ ancestors(Source) and Base ∈ ancestors(Target)
//! ```
//!
//! Each side buys something different, and neither is decoration.
//!
//! **Ancestry to the Source** earns the right to read an absence as a decision. Without it,
//! `omitted` means only *not present*, and a difference would attribute an intention to a
//! Thesis that never chose anything.
//!
//! **Ancestry to the Target** earns the right to apply that decision. An omission is measured
//! over the Base's open future, so applying it to a Target that never passed through the Base
//! removes a commitment the Target selected on its own. Membership cannot tell a commitment
//! held by inheritance from one held by independent decision — and unlike a textual patch,
//! which carries its context and fails loudly where that context is absent, a membership
//! removal applies silently.
//!
//! Transferring commitments between unrelated lineages stays available and is a fork: the
//! Thesis layer introduces them over the Target directly. What such an operation cannot claim
//! is that its omissions carry intention, which is the whole of what Synthesis adds.
//!
//! Theses are named by identity rather than handed over, because ancestry is a property of
//! what the archive holds. A Thesis an application keeps only in memory has no lineage to
//! prove, and asking the archive is what makes the answer mean anything.
//!
//! The relation is reflexive, and that is what makes the degenerate cases informative rather
//! than incoherent: a Base equal to its Target is a fast-forward, and one equal to its Source
//! leaves an empty difference. Both are conclusions worth reaching.

use crate::engine::thesis::{ThesisError, ThesisId, ThesisLookup, descends_from};

pub(super) fn coherent_base<A: ThesisLookup>(
    archive: &A,
    base: ThesisId,
    source: ThesisId,
    target: ThesisId,
) -> Result<bool, ThesisError> {
    Ok(descends_from(archive, source, base)? && descends_from(archive, target, base)?)
}
