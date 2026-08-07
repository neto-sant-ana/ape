//! `synthesize` — the one operation, and the order its steps must keep.
//!
//! ```text
//! resolve the three Theses
//! establish the Base
//!     → derive the difference       (Base ↔ Source)
//!     → resolve it against the Target
//!     → derive the candidate
//!     → collect the conflicts
//! ```
//!
//! Theses are named by identity because ancestry is a property of what the archive holds. A
//! Thesis an application keeps only in memory has no lineage to prove, and asking the archive
//! is what makes the answer mean anything.
//!
//! The Base is established before anything is derived, and an incoherent one ends the
//! operation rather than filling a report. A difference measured over a Base neither side
//! descends from is arithmetic without meaning, and everything downstream would inherit that:
//! the conflicts found in such a candidate would be attributed to a transfer nobody could
//! have intended.
//!
//! Nothing is constructed here. The report is evidence, and turning it into a Thesis is the
//! application's decision, taken through the Thesis layer.

use super::base::coherent_base;
use super::conflict::conflicts;
use super::{
    ApplicabilityReport, ApplicabilityStatus, CandidateSelection, IntentionalDifference,
    ResolvedTransfer, SynthesisError,
};

use crate::canon::CanonicalKnowledge;

use crate::engine::thesis::{Thesis, ThesisError, ThesisId, ThesisLookup};

pub fn synthesize<A: ThesisLookup, K: CanonicalKnowledge>(
    archive: &A,
    knowledge: &K,
    base: ThesisId,
    source: ThesisId,
    target: ThesisId,
) -> Result<ApplicabilityReport, SynthesisError> {
    let base_thesis = read(archive, base)?;
    let source_thesis = read(archive, source)?;
    let target_thesis = read(archive, target)?;

    if !coherent_base(archive, base, source, target)? {
        return Err(SynthesisError::IncoherentBase {
            base,
            source_thesis: source,
            target_thesis: target,
        });
    }

    let difference = IntentionalDifference::between(&base_thesis, &source_thesis);
    let transfer = ResolvedTransfer::resolving(&difference, &target_thesis);

    let status = if transfer.is_empty() {
        ApplicabilityStatus::AlreadyApplied
    } else {
        let found = conflicts(knowledge, &transfer, &target_thesis)?;

        if found.is_empty() {
            let candidate = CandidateSelection::deriving(&transfer, &target_thesis);

            ApplicabilityStatus::Applicable {
                transfer,
                candidate,
            }
        } else {
            ApplicabilityStatus::Conflicted {
                attempted: transfer,
                conflicts: found,
            }
        }
    };

    Ok(ApplicabilityReport::new(
        base, source, target, difference, status,
    ))
}

fn read<A: ThesisLookup>(archive: &A, id: ThesisId) -> Result<Thesis, SynthesisError> {
    archive
        .thesis(id)
        .ok_or(ThesisError::UnknownThesis(id))
        .map_err(SynthesisError::from)
}
