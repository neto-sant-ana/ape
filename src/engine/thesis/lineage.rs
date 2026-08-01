//! Ancestry, answered by walking the archive.
//!
//! A Thesis commits its parent inside its own content-addressed id, so a lineage is already
//! a hash-linked chain and needs no index to be trusted: resolving `parent` repeatedly is
//! the whole of it. The walk terminates because a genesis has none, and the archive refuses
//! to hold a child before its parent, so it cannot end anywhere else.
//!
//! [`descends_from`] is **reflexive**: a Thesis descends from itself. That is not a
//! convenience — it is what makes the degenerate cases of a transfer answer informatively.
//! Where a Base equals its Target the transfer is a fast-forward, and where it equals its
//! Source the difference is empty; both are conclusions worth reaching, and a strict
//! relation would report them as an incoherent Base instead.
//!
//! Only the starting Thesis must be resolvable. An ancestor absent from the archive is not
//! an error but a `false`: the archive holds no child before its parent, so a Thesis it does
//! not hold is an ancestor of nothing it does.

use super::{Thesis, ThesisError, ThesisId, ThesisLookup};

/// Whether `thesis` descends from `ancestor`, itself included.
pub fn descends_from<A: ThesisLookup>(
    archive: &A,
    thesis: ThesisId,
    ancestor: ThesisId,
) -> Result<bool, ThesisError> {
    let mut cursor = read_thesis(archive, thesis)?;

    loop {
        if cursor.id() == ancestor {
            return Ok(true);
        }

        match *cursor.parent() {
            Some(parent) => cursor = read_thesis(archive, parent)?,
            None => return Ok(false),
        }
    }
}

/// A parent the archive does not hold is a broken lineage rather than a genesis.
///
/// The port refuses to store a child before its parent, so reaching one is either a Thesis
/// that was never stored or an adapter that does not honor the contract. The code has to
/// branch somewhere, and a named refusal beats reporting an ancestry that was never walked.
fn read_thesis<A: ThesisLookup>(archive: &A, id: ThesisId) -> Result<Thesis, ThesisError> {
    archive.thesis(id).ok_or(ThesisError::UnknownThesis(id))
}
