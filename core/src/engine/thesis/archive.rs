//! The archive of Theses: the port through which a lineage is stored and resolved.
//!
//! A Thesis is content-addressed and immutable, so storing one is a put-if-absent and
//! resolving one is a lookup by identity. What the port adds to a map is a single rule,
//! and it is here for the same reason the Canon keeps its recording watermark in the
//! adapter: only the thing holding the records knows what it already holds.
//!
//! **A Thesis may not be stored before its parent.** Ancestry is walked by resolving
//! `parent` repeatedly, and a stored child whose parent is absent ends that walk in a
//! hole rather than at a genesis. Nothing downstream could tell the two apart — a
//! lineage that ends because it began, and one that ends because a record is missing —
//! and Synthesis decides whether a Base is a common ancestor by exactly that walk.
//!
//! Reading is separated from writing because the readers never write. Synthesis
//! resolves Theses to establish ancestry and has no business storing them; only an
//! application deciding to keep a Thesis does.
//!
//! ```text
//! ThesisLookup   → resolve by identity
//! ThesisArchive  → resolve, and store what has a parent to hang from
//! ```
//!
//! What the port does not do is decide *which* Theses are worth keeping, when to
//! discard one, or what any mutable reference such as `main` points at. It stores what
//! it is handed and answers what it holds.

use super::{Thesis, ThesisError, ThesisId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveOutcome {
    Stored,
    AlreadyPresent,
}

/// Resolving a Thesis by identity, which is all a reader of ancestry needs.
pub trait ThesisLookup {
    fn thesis(&self, id: ThesisId) -> Option<Thesis>;
}

pub trait ThesisArchive: ThesisLookup {
    /// Store `thesis` if absent, refusing one whose parent the archive does not hold.
    ///
    /// Idempotent by construction: the id is the hash of the content, so a re-put is a
    /// no-op rather than an overwrite, and [`ArchiveOutcome`] says which of the two
    /// happened for a caller that cares.
    fn put_thesis(&mut self, thesis: Thesis) -> Result<ArchiveOutcome, ThesisError>;
}
