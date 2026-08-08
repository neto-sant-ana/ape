//! An application built on APE, and the laboratory where the engine meets an application
//! boundary.
//!
//! The engine defines what operational knowledge *is*, and says nothing about how an
//! application names it, presents it, or carries it across a process. This crate answers
//! those questions as an ordinary consumer of the engine, so the boundary can be inspected
//! rather than assumed.
//!
//! # Library and binary
//!
//! Behavior lives in the library, because nothing can construct a type that lives inside a
//! binary — not an integration test, not the engine's conformance suites. The binary reads
//! arguments and writes output.

pub mod repository;
