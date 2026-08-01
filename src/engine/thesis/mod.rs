mod error;
pub use error::ThesisError;

mod frozen;

mod cut;
pub use cut::KnowledgeCut;

mod selection;
pub use selection::Selection;

mod advancement;
pub use advancement::Advancement;

mod interpretation;
pub use interpretation::{InterpretableKnowledge, Interpretation};

mod scenario;
pub use scenario::{ForkInput, GenesisInput, Thesis, ThesisId};

mod archive;
pub use archive::{ArchiveOutcome, ThesisArchive, ThesisLookup};

#[cfg(any(test, feature = "reference"))]
mod memory;
#[cfg(any(test, feature = "reference"))]
pub use memory::InMemoryArchive;

#[cfg(any(test, feature = "conformance"))]
pub mod conformance;

#[cfg(test)]
mod tests;
