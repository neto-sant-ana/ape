//! The reference application's laboratory: what each experiment arranged in order to be able to fail.
//!
//! This crate holds the **subjects**. The phases that measure them are the integration suites beside
//! it, one per experiment, and the record of what they found is in `cli/docs/`.
//!
//! # Why this is a crate and not a module
//!
//! It depends on [`ape_cli`], and the application never depends back. The direction is structural
//! rather than a matter of discipline, and it buys two things that a module boundary does not:
//!
//! An obligation discovered here has to be **earned into** the application by a change somebody
//! reviews — it cannot be reached for in the same file. And a change to the application that moves an
//! experiment's result reads as a **consumer breaking**, because the two cannot move in one commit
//! without saying so.
//!
//! It also buys the smaller thing that made the split urgent: the application can be handed to
//! somebody, or extracted, without the laboratory coming with it. Before the split one unit test in
//! the application built its world from a concluded experiment's subject, so removing the laboratory
//! broke the application's own suite — and the subject could not move without breaking a guard that
//! was not about it.
//!
//! What a subject is, and why they are not shared, is in [`subject`].

pub mod subject;

/// The application's binary, for the phases that terminate into a process of their own.
///
/// Cargo sets `CARGO_BIN_EXE_<name>` for integration tests of the crate that **owns** the binary, and
/// the laboratory does not own it. So the path is derived from where the test binary itself is: a test
/// runs from `target/<profile>/deps/`, and the application sits one directory up.
///
/// It lives here rather than beside each suite because it is not a subject. A subject belongs to the
/// experiment that arranged it and must not move; this is workspace layout, and if it moves, every
/// suite moves with it — which is exactly right, since none of them is about where a binary sits.
///
/// The phases that use it were measuring a fresh process finding the application from outside. They
/// now locate it the way any consumer would, which is closer to what they were always claiming.
pub fn binary() -> std::path::PathBuf {
    let mut path = std::env::current_exe().expect("a test binary knows where it is");

    path.pop();
    path.pop();
    path.push(format!("ape-cli{}", std::env::consts::EXE_SUFFIX));

    path
}
