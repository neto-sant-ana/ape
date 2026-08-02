mod error;
pub use error::SynthesisError;

mod base;

mod difference;
pub use difference::IntentionalDifference;

mod transfer;
pub use transfer::ResolvedTransfer;

mod candidate;
pub use candidate::CandidateSelection;

mod conflict;
pub use conflict::ApplicabilityConflict;

mod report;
pub use report::{ApplicabilityReport, ApplicabilityStatus};

mod merge;
pub use merge::synthesize;

#[cfg(test)]
mod tests;
