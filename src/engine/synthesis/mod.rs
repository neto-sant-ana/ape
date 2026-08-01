mod base;

mod difference;
pub use difference::IntentionalDifference;

mod transfer;
pub use transfer::ResolvedTransfer;

mod candidate;
pub use candidate::CandidateSelection;

#[cfg(test)]
mod tests;
