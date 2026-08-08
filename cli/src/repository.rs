//! A durable repository: canonical knowledge kept in a directory across process lifetime.
//!
//! Reduced to its shell by the first observation. Persisting a serialized entity and
//! reading it back is not a path the public boundary offers, so what a repository must
//! keep is whatever reproduces an *admission input*, and what opening one must do is
//! replay those admissions.
//!
//! The durable form of an input is undecided, and deliberately so — the experiment
//! introduces it when the procedure requires it, not before.

use std::path::PathBuf;

pub struct Repository {
    root: PathBuf,
}

impl Repository {
    pub fn open(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }
}
