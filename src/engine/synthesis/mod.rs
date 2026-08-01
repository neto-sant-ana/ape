mod base;

mod difference;
pub use difference::IntentionalDifference;

mod transfer;
pub use transfer::ResolvedTransfer;

mod candidate;
pub use candidate::CandidateSelection;

mod conflict;
pub use conflict::ApplicabilityConflict;

#[cfg(test)]
mod tests;
