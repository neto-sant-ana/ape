//! A durable repository: a directory holding what a later process needs to rebuild a world.
//!
//! What it keeps is the [`Admission`] journal, because Observation 1 established that
//! canonical knowledge is reconstructed by replaying admissions and Observation 3's
//! neighbour — that a `Constraint` cannot be read back off an admitted `Resource` — settled
//! that the journal has to be captured as it is supplied rather than derived afterwards.
//!
//! Derived values are written **only where something compares them**. A level, a condition
//! and a projection are recomputed and never kept; what a decision was taken against, and the
//! world it produced, are kept precisely so a reader has two representations to weigh. The
//! objection to storing an answer beside its question was that the two can disagree, and that
//! is the whole of what makes disagreement visible — a repository holding only its inputs
//! cannot contradict itself, and cannot notice anything either.
//!
//! The rule that replaces it is narrower and stricter: a derived value that is written and not
//! compared on every read is the liability the old one feared.
//!
//! The format is JSON, and it is a decision about *this experiment* rather than about APE.
//! A laboratory whose repository cannot be read by eye hides half of what it is for; the
//! cost is a format wordier than the engine's own postcard encoding, which nothing here
//! measures and nothing here depends on.
//!
//! # A whole write, and the pointer that makes it one
//!
//! Three files are three writes, and a process that stops between two of them leaves a repository
//! nobody wrote. The atomicity experiment measured what that costs: of the six states an
//! interruption can leave, five are refused on being read and one **reconstructs** — byte-identical
//! to a repository whose writer admitted knowledge and decided nothing about it. There is no fact in
//! the record that separates them, so no amount of checking on the reader's side reaches it.
//!
//! So the repository holds two **generations** and a pointer at the live one:
//!
//! ```text
//! current       a               one file, replaced by a rename
//! a/            journal.json  lineage.json  worlds.json      ← what a reader reads
//! b/            journal.json  lineage.json  worlds.json      ← what the previous write left
//! ```
//!
//! A whole write puts three files in the generation that is *not* live, where nothing reads them,
//! and then turns the pointer. Turning is one `rename`, so a write is all-or-nothing against a
//! process that stops: interrupted before the turn, the repository is what it was, byte for byte.
//! Interrupted *by the operating system* mid-`rename` is a different question, and this makes no
//! promise about it.
//!
//! Turning a reference is the verb the convergence experiment already used for moving one, and the
//! seam between preparing and turning is public for the reason exploration's seam between weighing
//! and keeping is: an interruption is a **prefix** of a write, and a laboratory cannot produce one
//! against a call that does both.
//!
//! **A repository with no pointer is its own live generation.** That is what every repository written
//! before generations existed looks like, and they are read unchanged — including two that were
//! written by parties nobody can re-run.
//!
//! # Writing one file is still possible, and that is not an oversight
//!
//! [`Repository::write_journal`] and its two neighbours write into the live generation, one file,
//! visibly. Nothing in this application calls them: they are what a *record edited from outside*
//! looks like, and five experiments need exactly that in order to tamper with a repository, prune
//! one, or interrupt one. The record's defence against them is corroboration, which is measured
//! rather than promised.
//!
//! Earned by: 00-reconstruction (Confirmed), 02-corroboration (Confirmed), 07-atomicity (Confirmed)

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::RepositoryError;
use crate::journal::Admission;
use crate::lineage::Taken;
use crate::reading::WorldRecord;

const JOURNAL: &str = "journal.json";
const LINEAGE: &str = "lineage.json";
const WORLDS: &str = "worlds.json";

/// The file naming the generation a reader reads.
const CURRENT: &str = "current";

/// The name it is written under before it replaces itself.
const TURNING: &str = "current.turning";

/// The two generations a repository alternates between.
///
/// Two rather than a series, because what the atomicity experiment's criterion asks for is *one*
/// previous state: the repository before an interrupted write. A series would keep more and would
/// make pruning a question nothing here has measured a need to answer.
const GENERATIONS: [&str; 2] = ["a", "b"];

/// Everything a whole write puts on disk, by name.
///
/// A parameter object rather than three arguments, because the three are what a repository *is* and
/// a caller that could supply two of them would be back to the state this exists to remove.
pub struct RepositoryInput<'a> {
    pub journal: &'a [Admission],
    pub lineage: &'a [Taken],
    pub worlds: &'a [WorldRecord],
}

/// Three files written where nothing reads them, and the pointer not yet turned.
///
/// It carries no reference to the [`Repository`] on purpose: a prepared generation is a fact on
/// disk, and dropping this value abandons it rather than undoing it. The next write overwrites it,
/// and nothing ever read it.
pub struct Prepared {
    root: PathBuf,
    generation: &'static str,
}

impl Prepared {
    /// Make the prepared generation the one a reader reads.
    ///
    /// One `rename`, over a pointer written beside itself. Everything expensive already happened.
    pub fn turn(self) -> Result<(), RepositoryError> {
        let turning = self.root.join(TURNING);

        fs::write(&turning, self.generation)?;
        fs::rename(turning, self.root.join(CURRENT))?;

        Ok(())
    }

    /// Where the files were put, for a phase that needs to see that nothing reads them.
    pub fn generation(&self) -> PathBuf {
        self.root.join(self.generation)
    }
}

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

    /// The generation a reader reads: whichever the pointer names, or the root itself.
    ///
    /// The pointer's contents are not checked against a list of names. A pointer naming something
    /// that is not there resolves to a directory that is not there, and the refusal says which path
    /// — where a whitelist would have fallen back to the root and answered with a repository that is
    /// not the one being pointed at.
    fn live(&self) -> PathBuf {
        match self.pointer() {
            Some(generation) => self.root.join(generation),
            None => self.root.clone(),
        }
    }

    fn pointer(&self) -> Option<String> {
        let named = fs::read_to_string(self.root.join(CURRENT)).ok()?;
        let named = named.trim().to_owned();

        (!named.is_empty()).then_some(named)
    }

    /// The generation a whole write puts its files in: never the live one.
    ///
    /// Safe for any pointer, including one naming neither generation — the only name that maps to
    /// the second slot is the first slot's own.
    fn next(&self) -> &'static str {
        match self.pointer().as_deref() {
            Some(live) if live == GENERATIONS[0] => GENERATIONS[1],
            _ => GENERATIONS[0],
        }
    }

    /// Write all three files where nothing reads them, and turn the pointer.
    ///
    /// This is what an application uses. Whatever it replaces stays on disk in the other generation.
    pub fn write_whole(&self, input: RepositoryInput<'_>) -> Result<(), RepositoryError> {
        self.prepare(input)?.turn()
    }

    /// The first half of a whole write, kept separate so an interruption can be a value.
    pub fn prepare(&self, input: RepositoryInput<'_>) -> Result<Prepared, RepositoryError> {
        let generation = self.next();
        let into = self.root.join(generation);

        fs::create_dir_all(&into)?;

        for (name, encoded) in [
            (JOURNAL, serde_json::to_string_pretty(input.journal)?),
            (LINEAGE, serde_json::to_string_pretty(input.lineage)?),
            (WORLDS, serde_json::to_string_pretty(input.worlds)?),
        ] {
            fs::write(into.join(name), encoded)?;
        }

        Ok(Prepared {
            root: self.root.clone(),
            generation,
        })
    }

    pub fn journal_path(&self) -> PathBuf {
        self.live().join(JOURNAL)
    }

    /// Write the admissions a world was built from into the live generation, one file, visibly.
    ///
    /// Not what an application does. See the note at the top of this module about why it exists.
    pub fn write_journal(&self, journal: &[Admission]) -> Result<(), RepositoryError> {
        fs::create_dir_all(self.live())?;

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
        self.live().join(LINEAGE)
    }

    /// Write the decisions a Thesis was reached by, each with the entry it was taken after.
    ///
    /// Kept apart from the journal because the two answer to different authorities: what
    /// became known is not revisable, and which world is being reasoned about is a choice
    /// that may be made again. Nothing holds the relation between the two files, so each
    /// decision carries its own: the sequence that may be decided again is the one that says
    /// where in the other it belongs.
    /// Written into the live generation, one file, visibly — like its two neighbours.
    pub fn write_lineage(&self, lineage: &[Taken]) -> Result<(), RepositoryError> {
        fs::create_dir_all(self.live())?;

        let encoded = serde_json::to_string_pretty(lineage)?;

        fs::write(self.lineage_path(), encoded)?;

        Ok(())
    }

    pub fn read_lineage(&self) -> Result<Vec<Taken>, RepositoryError> {
        let encoded = fs::read_to_string(self.lineage_path())?;

        Ok(serde_json::from_str(&encoded)?)
    }

    pub fn worlds_path(&self) -> PathBuf {
        self.live().join(WORLDS)
    }

    /// Write the worlds the decisions produced.
    ///
    /// Derived, all of them, and kept for exactly that reason. What is written is the
    /// application's record of a world rather than the engine's serialized `Thesis`, because
    /// a `Thesis` cannot be read back from bytes — so this is a witness and never a source,
    /// and the boundary is what makes that true rather than a promise.
    /// Written into the live generation, one file, visibly — like its two neighbours.
    pub fn write_worlds(&self, worlds: &[WorldRecord]) -> Result<(), RepositoryError> {
        fs::create_dir_all(self.live())?;

        let encoded = serde_json::to_string_pretty(worlds)?;

        fs::write(self.worlds_path(), encoded)?;

        Ok(())
    }

    pub fn read_worlds(&self) -> Result<Vec<WorldRecord>, RepositoryError> {
        let encoded = fs::read_to_string(self.worlds_path())?;

        Ok(serde_json::from_str(&encoded)?)
    }
}
