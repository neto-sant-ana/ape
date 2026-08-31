//! An experiment: what a record is worth to somebody who was not there.
//!
//! The question, the method and the predictions are recorded beside this code — the row's premise in
//! `README.md`, and each experiment's protocol and observations in its own directory. Reading them
//! first is worth more than reading this crate.
//!
//! # Why this crate does not compile against the engine or the application
//!
//! The other two rows compile against `ape` and `ape-cli`, because their subject is what the engine
//! and the application do. **This row's subject is prose**, and the only thing it needs from the
//! engine is the list of names the engine has — which its guard reads out of `core/`'s sources rather
//! than importing, so that the list cannot be a second copy.
//!
//! A dependency on either would be a claim that the row measures behaviour. It does not.
//!
//! **It does depend on a JSON parser, since `01-articulation`.** That is not the same claim: the row
//! *reads the other rows' artefacts as data*, which `lab/README.md` names as its instrument, and a
//! record read through `ape-cli`'s own reader would make a carving a function of how the application
//! reads rather than of what a later reader has. The stance is about the subject, not about crates.
//!
//! # What is here
//!
//! - [`corpus`] — the eight testimonies experiment 00 reads, and where each one is.
//! - [`classification`] — what each claim in them was judged to be, as data rather than as prose,
//!   so that a reader can disagree with one judgement without re-reading 13,382 words.
//! - [`articulation`] — experiment 01's instrument: the record as a later reader has it, and where
//!   a claim attaches to it.

pub mod articulation;
pub mod classification;
pub mod corpus;
pub mod phase4;
pub mod testimony;
