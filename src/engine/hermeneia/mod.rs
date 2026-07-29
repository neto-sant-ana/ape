mod error;
pub use error::ProjectionError;

mod condition;
use condition::Dependencies;
pub use condition::{Condition, Outcome, Timeliness};

mod feasibility;
pub use feasibility::{Conflict, Hypothesis};

mod projection;
pub use projection::Projection;

mod accumulation;
pub use accumulation::Accumulation;

#[cfg(test)]
mod tests;
