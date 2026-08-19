//! The reference application: what an application must do to carry APE's meaning across a process.
//!
//! The engine defines what operational knowledge *is*, and says nothing about how an
//! application names it, presents it, or carries it across a process. This crate answers
//! those questions as an ordinary consumer of the engine, so the boundary can be inspected
//! rather than assumed.
//!
//! # Nothing here is here without an experiment that earned it
//!
//! Every module declares what earned it, and `tests/pedigree.rs` refuses a claim whose experiment
//! did not reach the verdict it cites. The laboratory that produced those obligations is
//! `ape-frontier`, which depends on this crate and is never depended on by it — so an obligation
//! discovered there has to be earned into here by a change somebody reviews.
//!
//! # Library and binary
//!
//! Behavior lives in the library, because nothing can construct a type that lives inside a
//! binary — not an integration test, not the engine's conformance suites. The binary reads
//! arguments and writes output.
//!
//! Earned by: nothing — the crate root declares modules and implements no behaviour of its own.

pub mod archive;
pub mod converge;
pub mod error;
pub mod history;
pub mod journal;
pub mod level;
pub mod lineage;
pub mod reading;
pub mod repository;
pub mod transfer;
