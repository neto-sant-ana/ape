mod gateway;
pub use gateway::{Axiom, Knowledge};

mod error;
pub use error::AxiomError;

mod action;
mod agent;
mod assertion;
mod resource;
mod statement;

#[cfg(test)]
mod tests;
