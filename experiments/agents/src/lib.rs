//! An experiment: APE with an autonomous agent where the planner used to be.
//!
//! Every existing exercise of the engine has a caller that knew beforehand what would be
//! committed. This crate replaces that caller with an LLM agent and asks whether the
//! ontology carries the decision it makes — without gaining a primitive to do so.
//!
//! The question, the method and the predictions are recorded beside this code, in
//! `00-question/`, and each experiment keeps its protocol and observations in its own
//! directory. Reading them first is worth more than reading this crate.
//!
//! # What is here
//!
//! - [`world`] — the world the agent acts in, and the only file it is given beyond the
//!   engine's public documentation.
//! - [`policy`] — whether an intention may proceed, decided outside the engine, by a rule
//!   fixed before the agent runs.
//!
//! What the agent produces is not here. It is recorded as it was written, in the directory
//! of the experiment that produced it, because a run is evidence rather than source.

pub mod policy;
pub mod world;
