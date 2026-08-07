//! A coordination engine for operational evolution, built on immutable assertions.
//!
//! APE models what is *expected to become true* rather than how work should flow. Nothing
//! here executes business logic, stores operational state or orchestrates a process: the
//! crate defines a small ontology, admits knowledge into an append-only history, and
//! derives every operational condition from that history rather than maintaining it.
//!
//! # Layers
//!
//! Each layer rests on the one before it and never reaches back:
//!
//! - [`kernel`] — the ontology. Entities and value objects that cannot be constructed in
//!   an invalid state, and [`kernel::axiom`], the single gateway through which knowledge
//!   is admitted once its cross-entity invariants hold.
//! - [`canon`] — the canonical history. What became known, in the order it became known,
//!   appended under compare-and-append.
//! - [`engine`] — interpretation of that history:
//!   Conditions and feasibility ([`engine::hermeneia`]);
//!   Scenarios and their lineage ([`engine::thesis`]);
//!   The transfer of intent between them ([`engine::synthesis`]).
//!
//! # Adapters
//!
//! Persistence lives outside the engine. An adapter implements
//! [`canon::CanonicalHistory`] and/or [`engine::thesis::ThesisArchive`] over whatever medium
//! it likes, then proves it honors the contract by running the suites behind the
//! `conformance` feature against a fresh instance. The `reference` feature exposes the
//! in-memory implementations used by this crate's own tests; they demonstrate the
//! contract, and are not meant to back an application.
//!
//! # Reasoning
//!
//! The design documents the implementation answers to are rendered in [`docs`].

#[macro_use]
mod macros;

pub mod docs;

pub mod kernel;

pub mod canon;

pub mod engine;
