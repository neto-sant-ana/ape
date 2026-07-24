#[macro_use]
mod macros;

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
