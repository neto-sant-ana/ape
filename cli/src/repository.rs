//! A durable repository: a directory holding what a later process needs to rebuild a world.
//!
//! What it keeps is the [`Admission`] journal, because Observation 1 established that
//! canonical knowledge is reconstructed by replaying admissions and Observation 3's
//! neighbour — that a `Constraint` cannot be read back off an admitted `Resource` — settled
//! that the journal has to be captured as it is supplied rather than derived afterwards.
//!
//! Nothing derived is written. A level, a condition, a projection and a Thesis identity are
//! all recomputed from the journal, and persisting any of them would be storing an answer
//! next to the question it comes from, where the two can disagree.
//!
//! The format is JSON, and it is a decision about *this experiment* rather than about APE.
//! A laboratory whose repository cannot be read by eye hides half of what it is for; the
//! cost is a format wordier than the engine's own postcard encoding, which nothing here
//! measures and nothing here depends on.

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::RepositoryError;
use crate::journal::Admission;
use crate::lineage::Taken;

const JOURNAL: &str = "journal.json";
const LINEAGE: &str = "lineage.json";

pub struct Repository {
    root: PathBuf,
}

impl Repository {
    pub fn open(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn journal_path(&self) -> PathBuf {
        self.root.join(JOURNAL)
    }

    /// Write the admissions a world was built from, replacing whatever was there.
    pub fn write_journal(&self, journal: &[Admission]) -> Result<(), RepositoryError> {
        fs::create_dir_all(&self.root)?;

        let encoded = serde_json::to_string_pretty(journal)?;

        fs::write(self.journal_path(), encoded)?;

        Ok(())
    }

    /// Read the admissions back, in the order they were made.
    pub fn read_journal(&self) -> Result<Vec<Admission>, RepositoryError> {
        let encoded = fs::read_to_string(self.journal_path())?;

        Ok(serde_json::from_str(&encoded)?)
    }

    pub fn lineage_path(&self) -> PathBuf {
        self.root.join(LINEAGE)
    }

    /// Write the decisions a Thesis was reached by, each with the entry it was taken after.
    ///
    /// Kept apart from the journal because the two answer to different authorities: what
    /// became known is not revisable, and which world is being reasoned about is a choice
    /// that may be made again. Nothing holds the relation between the two files, so each
    /// decision carries its own: the sequence that may be decided again is the one that says
    /// where in the other it belongs.
    pub fn write_lineage(&self, lineage: &[Taken]) -> Result<(), RepositoryError> {
        fs::create_dir_all(&self.root)?;

        let encoded = serde_json::to_string_pretty(lineage)?;

        fs::write(self.lineage_path(), encoded)?;

        Ok(())
    }

    pub fn read_lineage(&self) -> Result<Vec<Taken>, RepositoryError> {
        let encoded = fs::read_to_string(self.lineage_path())?;

        Ok(serde_json::from_str(&encoded)?)
    }
}
