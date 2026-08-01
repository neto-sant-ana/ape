//! `InMemoryArchive` — the reference [`ThesisArchive`], and the shape a real one takes.
//!
//! It keeps Theses in a map by identity and nothing else: no ordering, no lineage index,
//! no persistence. Ancestry is not stored as a structure because it is already carried by
//! each record — a Thesis commits its parent inside its own content-addressed id — so the
//! walk is a sequence of lookups rather than a traversal of anything the archive maintains.

use std::collections::BTreeMap;

use super::{ArchiveOutcome, Thesis, ThesisArchive, ThesisError, ThesisId, ThesisLookup};

#[derive(Debug, Clone, Default)]
pub struct InMemoryArchive {
    theses: BTreeMap<ThesisId, Thesis>,
}
impl ThesisLookup for InMemoryArchive {
    fn thesis(&self, id: ThesisId) -> Option<Thesis> {
        self.theses.get(&id).cloned()
    }
}
impl ThesisArchive for InMemoryArchive {
    fn put_thesis(&mut self, thesis: Thesis) -> Result<ArchiveOutcome, ThesisError> {
        let id = thesis.id();

        if self.theses.contains_key(&id) {
            return Ok(ArchiveOutcome::AlreadyPresent);
        }

        if let Some(parent) = *thesis.parent()
            && !self.theses.contains_key(&parent)
        {
            return Err(ThesisError::ParentNotArchived { thesis: id, parent });
        }

        self.theses.insert(id, thesis);

        Ok(ArchiveOutcome::Stored)
    }
}
