//! The experimental subjects, one per experiment.
//!
//! A subject is what an experiment arranges in order to be able to fail, so it belongs to
//! the experiment that arranged it rather than to the laboratory. A concluded experiment
//! keeps its own unchanged, and the reason is coupling rather than reproducibility: a
//! subject two experiments share cannot move for one of them without breaking a guard that
//! is not about it. What preserves a published measurement is the commit it was taken
//! against, which is the rule in `lab/README.md`.
//!
//! They overlap in shape, and the overlap is left alone. Two arrangements that happen to
//! need a role and an agent are not two copies of one fact.

pub mod atomicity;
pub mod collision;
pub mod commensurability;
pub mod contention;
pub mod convergence;
pub mod coordination;
pub mod divergence;
pub mod exploration;
pub mod indexicality;
pub mod provenance;
pub mod reconstruction;
pub mod veracity;
pub mod witness;
