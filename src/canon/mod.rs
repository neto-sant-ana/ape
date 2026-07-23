//! The Canon — the canonical admission layer.
//!
//! The Axiom decides whether an assertion may *exist*; the Canon decides whether
//! it may become *history*. This module owns the append side of that history: the
//! [`CanonicalHistory`] repository through which structurally valid assertions,
//! wrapped as [`Canonical`] records, are atomically admitted.

mod error;
pub use error::CanonError;

mod history;
pub use history::{AppendOutcome, CanonicalHistory};

mod record;
pub use record::Canonical;

mod orchestrator;
pub use orchestrator::{Canon, EventSubmission};

#[cfg(test)]
mod tests;
