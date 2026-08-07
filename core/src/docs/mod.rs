//! The architecture of APE, as prose.
//!
//! These documents are where the design is decided; the code answers to them. They ship
//! inside the crate, so rendering them here costs nothing and keeps `cargo doc` carrying
//! the reasoning rather than the signatures alone — a reader who wants to know *why* a
//! layer exists should not have to leave the documentation to find out.
//!
//! Each module below is one Markdown file under `src/docs/`, included verbatim. The
//! Markdown is the origin and the only copy: when the architecture changes it changes
//! there, and these pages follow rather than restate.
//!
//! They are ordered as they were written, each resting on the one before it:
//!
//! | Document | Layer it founds |
//! |---|---|
//! | [`philosophy`] | — |
//! | [`ontology`] | — |
//! | [`kernel`] | [`crate::kernel`] |
//! | [`axiom`] | [`crate::kernel::axiom`] |
//! | [`canon`] | [`crate::canon`] |
//! | [`hermeneia`] | [`crate::engine::hermeneia`] |
//! | [`thesis`] | [`crate::engine::thesis`] |
//! | [`synthesis`] | [`crate::engine::synthesis`] |

#[doc = include_str!("00-philosophy.md")]
pub mod philosophy {}

#[doc = include_str!("01-ontology.md")]
pub mod ontology {}

#[doc = include_str!("02-kernel.md")]
pub mod kernel {}

#[doc = include_str!("03-axiom.md")]
pub mod axiom {}

#[doc = include_str!("04-canon.md")]
pub mod canon {}

#[doc = include_str!("05-hermeneia.md")]
pub mod hermeneia {}

#[doc = include_str!("06-thesis.md")]
pub mod thesis {}

#[doc = include_str!("07-synthesis.md")]
pub mod synthesis {}
