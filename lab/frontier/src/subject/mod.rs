//! The experimental subjects, one per experiment.
//!
//! A subject is what an experiment arranges in order to be able to fail, so it belongs to
//! the experiment that arranged it rather than to the laboratory. A concluded experiment
//! keeps its own unchanged: a published result whose subject moved underneath it is a
//! result nobody can run again.
//!
//! They overlap in shape, and the overlap is left alone. Two arrangements that happen to
//! need a role and an agent are not two copies of one fact.

pub mod convergence;
pub mod coordination;
pub mod divergence;
pub mod exploration;
pub mod provenance;
pub mod reconstruction;
