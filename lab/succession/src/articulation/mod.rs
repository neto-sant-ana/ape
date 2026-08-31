//! Experiment 01 — Articulation. Three carvings of one record, generated from one source.
//!
//! *Does the unit a record is carved into change what a reader who was not there can establish from
//! it — and at what cost?*
//!
//! The protocol is in `01-articulation/00-protocol.md` and is worth more than this module. What is
//! here is the instrument:
//!
//! ```text
//! record    the four files, read as a later reader has them — not through `ape-cli`
//! anchor    where a claim attaches, derived from the record's own identities and labels
//! ```
//!
//! **Written once, so no carving can be quietly favoured by a better hand**, and derived, so the
//! content cannot drift between them. That is the protocol's method and it is also its main hazard:
//! a generator is a place where a preference hides as a design decision, so every judgement made
//! while writing this is recorded where it was made rather than in the result.

pub mod anchor;
pub mod carving;
pub mod record;
