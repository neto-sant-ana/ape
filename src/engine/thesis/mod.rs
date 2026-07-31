mod error;
pub use error::ThesisError;

mod frozen;

mod cut;
pub use cut::KnowledgeCut;

mod advancement;
pub use advancement::Advancement;

mod interpretation;
pub use interpretation::{InterpretableKnowledge, Interpretation};

mod scenario;
pub use scenario::{ForkInput, GenesisInput, Thesis, ThesisId};

#[cfg(test)]
mod tests;
