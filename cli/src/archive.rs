//! A resident `ThesisArchive`: the worlds an application decided, held by identity.
//!
//! It is the second port this laboratory implements, and the first one whose records are
//! *derived*. Everything the canonical history holds was supplied by someone; everything here
//! was computed from something else, which is why the previous experiments had no reason to
//! store any of it.
//!
//! The reason it exists now is corroboration rather than lookup. A world resolved from here
//! and a world derived from the decisions that produced it are two representations of one
//! fact, and two representations are the only thing a reader can compare.
//!
//! # What this cannot do
//!
//! It cannot survive the process. A `Thesis` derives `Serialize` and not `Deserialize` —
//! identity in this engine is re-derived, never read — so an archive can be written out and
//! never read back in. Whatever crosses a process boundary is therefore a comparison, not a
//! source, and the boundary makes that a fact rather than a discipline.
//!
//! Earned by: 02-corroboration (Confirmed), 03-convergence (Confirmed)

use std::collections::BTreeMap;

use ape::engine::thesis::{
    ArchiveOutcome, Thesis, ThesisArchive, ThesisError, ThesisId, ThesisLookup,
};

/// Every world stored, keyed by the identity derived from its content.
#[derive(Debug, Default)]
pub struct ResidentArchive {
    worlds: BTreeMap<ThesisId, Thesis>,
}

impl ResidentArchive {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.worlds.len()
    }

    pub fn is_empty(&self) -> bool {
        self.worlds.is_empty()
    }
}

impl ThesisLookup for ResidentArchive {
    fn thesis(&self, id: ThesisId) -> Option<Thesis> {
        self.worlds.get(&id).cloned()
    }
}

impl ThesisArchive for ResidentArchive {
    /// Store a world, refusing one whose parent is absent.
    ///
    /// The refusal is what keeps ancestry walkable. A child stored over a hole ends the walk
    /// exactly where a genesis ends it, and nothing downstream could tell a lineage that
    /// finished from one that is missing a record.
    fn put_thesis(&mut self, thesis: Thesis) -> Result<ArchiveOutcome, ThesisError> {
        if self.worlds.contains_key(&thesis.id()) {
            return Ok(ArchiveOutcome::AlreadyPresent);
        }

        if let Some(parent) = thesis.parent()
            && !self.worlds.contains_key(parent)
        {
            return Err(ThesisError::ParentNotArchived {
                thesis: thesis.id(),
                parent: *parent,
            });
        }

        self.worlds.insert(thesis.id(), thesis);

        Ok(ArchiveOutcome::Stored)
    }
}
