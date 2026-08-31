//! An experiment: what a record is worth to somebody who was not there.
//!
//! The question, the method and the predictions are recorded beside this code — the row's premise in
//! `README.md`, and each experiment's protocol and observations in its own directory. Reading them
//! first is worth more than reading this crate.
//!
//! # Why this crate depends on nothing
//!
//! The other two rows compile against `ape` and `ape-cli`, because their subject is what the engine
//! and the application do. **This row's subject is prose**, and the only thing it needs from the
//! engine is the list of names the engine has — which its guard reads out of `core/`'s sources rather
//! than importing, so that the list cannot be a second copy.
//!
//! A dependency here would be a claim that the row measures behaviour. It does not.
//!
//! # What is here
//!
//! - [`corpus`] — the eight testimonies experiment 00 reads, and where each one is.
//! - [`classification`] — what each claim in them was judged to be, as data rather than as prose,
//!   so that a reader can disagree with one judgement without re-reading 13,382 words.

pub mod classification;
pub mod corpus;
pub mod testimony;
