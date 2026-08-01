mod base;

mod difference;
pub use difference::IntentionalDifference;

mod transfer;
pub use transfer::ResolvedTransfer;

#[cfg(test)]
mod tests;
