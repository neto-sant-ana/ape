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

#[cfg(any(test, feature = "reference"))]
mod memory;
#[cfg(any(test, feature = "reference"))]
pub use memory::InMemoryHistory;

#[cfg(any(test, feature = "conformance"))]
pub mod conformance;

#[cfg(test)]
mod tests;
