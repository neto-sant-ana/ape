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
//! and then turns the pointer. Turning replaces one file by a `rename`, so a write is all-or-nothing
//! against a process that stops: interrupted before the turn, the repository is what it was, byte for
//! byte. Interrupted *by the operating system* mid-`rename` is a different question, and this makes no
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
//! # And atomic against a process that stops is not atomic against another writer
//!
//! The atomicity experiment said so in its own result, and the contention experiment asked it. Two
//! writers that prepare before either turns choose the **same** generation, because `prepare` picks
//! its target by reading the pointer — a question about where a *reader* is looking, not about who
//! else is writing. Measured over all six interleavings the two operations admit: what a reader ends
//! up reading is the state of whoever prepared **last**, the turn does not appear in the rule, and
//! nothing refused any of it.
//!
//! So a turn compares before it swaps. [`Prepared`] keeps what it wrote and [`Prepared::turn`] reads
//! it back, refusing to publish a generation this write did not write. The whole reasoning, and the
//! three states it puts out of reach, are there.
//!
//! What that does *not* touch is a writer holding a stale reading: it overwrites nobody, prepares a
//! legitimate whole state of its own, and loses the other line in silence. Comparing knowledge is the
//! journal's business and lives in [`crate::converge`] — which, measured, refuses every ordering of
//! two parties that both put back through it. Two comparisons, and neither substitutes for the other.
//!
//! # Writing one file is still possible, and that is not an oversight
//!
//! [`Repository::write_journal`] and its two neighbours write into the live generation, one file,
//! visibly. Nothing in this application calls them: they are what a *record edited from outside*
//! looks like, and five experiments need exactly that in order to tamper with a repository, prune
//! one, or interrupt one. The record's defence against them is corroboration, which is measured
//! rather than promised.
//!
//! They are also how a second writer reaches a **mixture**: aimed at a generation somebody else
//! prepared, they put one writer's file over another's, and the contention experiment measured that
//! the mixtures which reconstruct silently are exactly the ones whose two journals stand in extension.
//! A turn that would publish one is now refused, which is the same shape as before — the states are
//! out of reach through the write, and not impossible.
//!
//! # What the record says about itself
//!
//! The three files above are sequences: a journal is one entry per admission, a lineage one record
//! per decision, a worlds file one record per world. So every claim in them is about **one item**,
//! and each file's length is a property of that file — which means a file that lost an entry agrees
//! with itself about how many it has.
//!
//! The one claim whose subject is the record's history is a decision's witness, and a witness is
//! written by a decision out of the prefix that stood when it was taken. What a record admits *after*
//! its last decision is therefore in no witness, and the custody experiment measured what that costs:
//! a journal truncated past the last coordinate reads, answers what it answered, and a decision taken
//! afterwards stands on a prefix that is short — with nothing having said so.
//!
//! ```text
//! journal.json    [ ......... the prefix ......... ][ ... the tail ... ]
//! lineage.json      each entry named by a witness              nothing
//! custody.json      every address the journal comes to, whole
//! ```
//!
//! So a whole write records a fourth thing: **the addresses the journal produces**, compared against
//! the replay on every read. It is the one derived value here the writer does not supply, and the
//! reason is that it is a function of the journal and of nothing else — unlike a world, which needs
//! the lineage and the engine, and unlike a witness, which needs to know *when* the decision was
//! taken. Deriving it beside the journal at write time is `Taken::now`'s argument one level up: both
//! halves come from one reading, so they cannot disagree when they are written, and corroboration is
//! a property of the read.
//!
//! **Membership rather than order**, for the reason [`Taken::witness`] gives: an entry's identity is
//! its content, so a journal reordered holds the same entries and the record's own reader answers the
//! same way. What it must disagree with is a journal that is not the one the record wrote.
//!
//! **A repository with no `custody.json` is read as one that makes no such claim**, which is the same
//! tolerance the pointer has and for a sharper reason: four repositories under
//! `lab/agents/04-multiagent` were written by parties nobody can re-run, and a required fourth file
//! would strand them.
//!
//! **What it costs.** A whole write now replays the journal it is handed, and every read compares two
//! sets the size of it. That is the second observable price in this module, and it is paid per write
//! rather than per entry the way the turn's comparison is.
//!
//! # And a fifth, which the writer does supply
//!
//! ```text
//! designations.json   which of its worlds this record means, in order
//! ```
//!
//! Custody is the one derived thing here; this is the opposite, and the difference decides its shape.
//! Nothing produces a plan, so a whole write cannot compute one and a caller has to hand it over —
//! which is why it is a field of [`RepositoryInput`] rather than something [`Repository::prepare`]
//! works out. [`crate::designation`] holds the whole of why it is a sequence and not a pointer.
//!
//! **It has to be an input to the write**, and that was measured rather than assumed. A fifth file
//! written beside the live generation is left behind by the next turn — the pointer moves to the
//! other generation, which never had it — and the record then reads as one that never claimed
//! anything, which is the same silence as a record that genuinely never did.
//!
//! **The same tolerance as custody, and a sharper `[]`.** A repository with no `designations.json`
//! makes no claim. One whose file holds `[]` says its plan never moved, which is a sentence a write
//! can honestly make — where an empty custody would be a record claiming to hold no entries.
//!
//! Earned by: 00-reconstruction (Confirmed), 02-corroboration (Confirmed), 07-atomicity (Confirmed),
//! 08-contention (Confirmed), 16-custody (Confirmed), 18-designation (Confirmed)

use std::fs;
use std::path::{Path, PathBuf};

use crate::designation::Designated;
use crate::error::RepositoryError;
use crate::journal::{self, Admission, EntryId};
use crate::lineage::Taken;
use crate::reading::WorldRecord;

const JOURNAL: &str = "journal.json";
const LINEAGE: &str = "lineage.json";
const WORLDS: &str = "worlds.json";

/// What the record claims to hold, as distinct from what any decision stood on.
const CUSTODY: &str = "custody.json";

/// Which of its worlds the record means, in order, and whose meaning each is.
const DESIGNATIONS: &str = "designations.json";

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
    /// Which of its worlds the record means, in order.
    ///
    /// Supplied rather than derived, which is what makes it a field here instead of something
    /// [`Repository::prepare`] computes. Nothing produces a plan, so a whole write that did not
    /// take one could only carry the previous plan forward — and a write that preserves what it was
    /// not given is the kind of implicitness this repository is built to avoid.
    ///
    /// The cost of that choice is stated rather than hidden: a caller passing `&[]` publishes a
    /// record whose plan never moved, so a caller that *had* a plan and forgot it here erases it.
    /// Experiment 18 measured the alternative — a fifth file written beside the live generation is
    /// left behind by the next turn, and the record then says it never had a plan at all.
    pub designations: &'a [Designated],
}

/// Three files written where nothing reads them, and the pointer not yet turned.
///
/// It carries no reference to the [`Repository`] on purpose: a prepared generation is a fact on
/// disk, and dropping this value abandons it rather than undoing it. The next write overwrites it,
/// and nothing ever read it.
///
/// It also carries what it wrote, which is what makes [`Prepared::turn`] a comparison. See there.
pub struct Prepared {
    root: PathBuf,
    generation: &'static str,
    written: [(&'static str, String); 5],
}

impl Prepared {
    /// Make the prepared generation the one a reader reads, if it is still the one this write made.
    ///
    /// The comparison is the whole of what the contention experiment earned. [`Repository::prepare`]
    /// chooses its target by reading the pointer, so two writers that both prepare before either
    /// turns choose the **same** generation — and before this, both turns published it. Whoever
    /// prepared last decided what a reader would read, both writers were told they had succeeded,
    /// and the pointer's turn was not the commit it looked like.
    ///
    /// So a turn reads back the three files it prepared and refuses to publish a generation it did
    /// not write. That makes a whole write a compare-and-swap in the one sense this application can
    /// support: it compares against what *it* put there, so it refuses a generation another writer
    /// overwrote, one another writer partially overwrote, and one that has since been superseded and
    /// would be published backwards.
    ///
    /// **What it costs, and it is this sequence's first observable price.** A turn used to be one
    /// `rename` over a pointer, with everything expensive already done; it now reads three files
    /// first, and a [`Prepared`] holds an encoded copy of them until it is turned. That is paid by
    /// every write, in exchange for a refusal that only two writers can provoke.
    ///
    /// **What it does not do**, and neither half is a detail. It does not serialize anybody: nothing
    /// waits, nothing is held, and a refused writer's recovery is the one the coordination
    /// experiment already established — read again, decide again, converge. And it says nothing
    /// about a writer holding a **stale reading**: a party that never re-read and never converged
    /// prepares a legitimate whole state of its own, overwrites nothing, and loses the other line in
    /// silence. That comparison is the journal's, and it lives in [`crate::converge`].
    ///
    /// The honest limit: between the last comparison and the `rename` there is a window, and closing
    /// it would take an atomic filesystem primitive rather than two calls. What this refuses is an
    /// interleaving of *calls*, which is what the experiment measured and all it claims.
    pub fn turn(self) -> Result<(), RepositoryError> {
        let into = self.root.join(self.generation);

        for (name, encoded) in &self.written {
            if fs::read_to_string(into.join(name))? != *encoded {
                return Err(RepositoryError::Contended {
                    generation: self.generation.to_owned(),
                });
            }
        }

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
    ///
    /// What it encoded is kept rather than dropped, because the turn compares against it. Encoding
    /// again there would compare a repository against a second rendering of itself, which answers a
    /// question about `serde_json` instead of one about who wrote the files.
    pub fn prepare(&self, input: RepositoryInput<'_>) -> Result<Prepared, RepositoryError> {
        let generation = self.next();
        let into = self.root.join(generation);

        fs::create_dir_all(&into)?;

        let written = [
            (JOURNAL, serde_json::to_string_pretty(input.journal)?),
            (LINEAGE, serde_json::to_string_pretty(input.lineage)?),
            (WORLDS, serde_json::to_string_pretty(input.worlds)?),
            (
                CUSTODY,
                serde_json::to_string_pretty(&journal::addresses(input.journal)?)?,
            ),
            (
                DESIGNATIONS,
                serde_json::to_string_pretty(input.designations)?,
            ),
        ];

        for (name, encoded) in &written {
            fs::write(into.join(name), encoded)?;
        }

        Ok(Prepared {
            root: self.root.clone(),
            generation,
            written,
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

    pub fn custody_path(&self) -> PathBuf {
        self.live().join(CUSTODY)
    }

    /// Write what the record claims to hold, into the live generation, one file, visibly.
    ///
    /// Not what an application does — a whole write derives this one. It is here for the same reason
    /// its three neighbours are: a claim nothing can edit from outside is a claim nothing can be
    /// measured against.
    pub fn write_custody(&self, held: &[EntryId]) -> Result<(), RepositoryError> {
        fs::create_dir_all(self.live())?;

        let encoded = serde_json::to_string_pretty(held)?;

        fs::write(self.custody_path(), encoded)?;

        Ok(())
    }

    /// What the record claims to hold, or nothing where it claims nothing.
    ///
    /// Absent is not empty, and the difference decides what a reader may conclude: a repository
    /// written before this claim existed says nothing about its own extent, and one whose file holds
    /// `[]` says it holds no entries at all. The first is read as before; the second is refused by
    /// the first entry the journal offers.
    pub fn read_custody(&self) -> Result<Option<Vec<EntryId>>, RepositoryError> {
        let path = self.custody_path();

        if !path.exists() {
            return Ok(None);
        }

        let encoded = fs::read_to_string(path)?;

        Ok(Some(serde_json::from_str(&encoded)?))
    }

    pub fn designations_path(&self) -> PathBuf {
        self.live().join(DESIGNATIONS)
    }

    /// Write which of its worlds the record means, into the live generation, one file, visibly.
    ///
    /// Not what an application does — a whole write takes this one as an input. It is here for the
    /// same reason its four neighbours are: a claim nothing can edit from outside is a claim nothing
    /// can be measured against.
    pub fn write_designations(&self, held: &[Designated]) -> Result<(), RepositoryError> {
        fs::create_dir_all(self.live())?;

        let encoded = serde_json::to_string_pretty(held)?;

        fs::write(self.designations_path(), encoded)?;

        Ok(())
    }

    /// Which of its worlds the record means, or nothing where it means to say nothing.
    ///
    /// Absent is not empty, and here the two are further apart than they are for custody. A
    /// repository written before this claim existed says nothing about which world it means; one
    /// whose file holds `[]` says its plan never moved, which is a sentence a record can honestly
    /// make and an empty custody could not.
    pub fn read_designations(&self) -> Result<Option<Vec<Designated>>, RepositoryError> {
        let path = self.designations_path();

        if !path.exists() {
            return Ok(None);
        }

        let encoded = fs::read_to_string(path)?;

        Ok(Some(serde_json::from_str(&encoded)?))
    }
}
