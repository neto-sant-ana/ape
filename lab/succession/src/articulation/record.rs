//! The record `01-articulation` carves, read as a later reader has it: four files of JSON.
//!
//! **Not through `ape-cli`**, and that is the experiment rather than an omission. The application's
//! reader rebuilds a world, corroborates it and hands back a `Corroborated` — a reading. What this
//! row asks is what somebody who was not there can establish from *the artefact*, so the artefact is
//! what it opens. Reading it through the application would make every carving a function of how the
//! application reads.
//!
//! It follows that these types are **structurally** what the files hold and are not the application's
//! types renamed. Where a field is unused by any carving it is still read, so that a file this cannot
//! parse fails loudly rather than being carved with a hole in it.
//!
//! # The source, and it is a four-file record
//!
//! `lab/agents/05-reconciliation/run-a/mine`, live generation — twenty-one journal entries, five
//! decisions, five worlds. Written by parties nobody can re-run, which `lab/README.md` names as the
//! one real veto, so it holds no `designations.json` and none of the carvings gets a plan to carve.
//! Recorded in the protocol before the run.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Where the record is, from the repository root.
pub const SOURCE: &str = "lab/agents/05-reconciliation/run-a/mine";

/// One admitted entry, by the tag the file carries and the fields that tag brings.
///
/// `#[serde(other)]` is deliberately absent: an unknown `admits` is a parse failure, because a
/// carving that silently skipped an entry would report a record that is not the one on disk.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "admits", rename_all = "kebab-case")]
pub enum Entry {
    Role {
        label: String,
        recorded_at: String,
    },
    Agent {
        label: String,
        recorded_at: String,
    },
    Eligibility {
        agent: String,
        roles: Vec<String>,
        effective_from: String,
        recorded_at: String,
    },
    Resource {
        label: String,
        kind: serde_json::Value,
        recorded_at: String,
    },
    ResourceInstance {
        label: String,
        resource: String,
        recorded_at: String,
    },
    Action {
        verb: String,
        kind: serde_json::Value,
        resource: String,
        recorded_at: String,
    },
    Statement {
        actors: Vec<String>,
        recipients: Vec<String>,
        action: String,
        fulfills: Vec<String>,
        cancels: Vec<String>,
        recorded_at: String,
    },
    Commitment {
        accountable: String,
        executors: Vec<String>,
        beneficiaries: Vec<String>,
        statement: String,
        resource: String,
        committed_at: String,
        due_date: String,
        magnitude: Option<String>,
        dependencies: Vec<String>,
        recorded_at: String,
    },
    Event {
        commitment: String,
        observation: String,
        occurred_at: String,
        recorded_at: String,
    },
}

impl Entry {
    /// The kind, as the file spells it — used to name a page and to group the vocabulary.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Role { .. } => "role",
            Self::Agent { .. } => "agent",
            Self::Eligibility { .. } => "eligibility",
            Self::Resource { .. } => "resource",
            Self::ResourceInstance { .. } => "resource-instance",
            Self::Action { .. } => "action",
            Self::Statement { .. } => "statement",
            Self::Commitment { .. } => "commitment",
            Self::Event { .. } => "event",
        }
    }

    /// The human name this entry introduces, where it introduces one.
    ///
    /// The record's own words, and the reason [`super::anchor`] can reach a claim that names no hex:
    /// a testimony says *finance*, and *finance* is a label this journal admits.
    pub fn label(&self) -> Option<&str> {
        match self {
            Self::Role { label, .. }
            | Self::Agent { label, .. }
            | Self::Resource { label, .. }
            | Self::ResourceInstance { label, .. } => Some(label),
            Self::Action { verb, .. } => Some(verb),
            _ => None,
        }
    }

    pub fn recorded_at(&self) -> &str {
        match self {
            Self::Role { recorded_at, .. }
            | Self::Agent { recorded_at, .. }
            | Self::Eligibility { recorded_at, .. }
            | Self::Resource { recorded_at, .. }
            | Self::ResourceInstance { recorded_at, .. }
            | Self::Action { recorded_at, .. }
            | Self::Statement { recorded_at, .. }
            | Self::Commitment { recorded_at, .. }
            | Self::Event { recorded_at, .. } => recorded_at,
        }
    }
}

/// One decision, flattened the way `lineage.json` holds it.
///
/// **`decides` is the kind and not an identity**, which cost this experiment a dead branch before it
/// was noticed: the file tags the variant, so the value is `genesis`, `advance` or `fork`. A
/// decision's world is nowhere in `lineage.json` — it is derived, and the record holds it in
/// `worlds.json` at the same position. See [`Record::decided`].
#[derive(Debug, Clone, Deserialize)]
pub struct Taken {
    pub decides: String,
    pub after: String,
    pub witness: Vec<String>,
    #[serde(default)]
    pub by: Option<String>,
    #[serde(default)]
    pub known_at: Option<String>,
    #[serde(default)]
    pub extends: Option<String>,
    #[serde(default)]
    pub selection: Vec<String>,
    #[serde(default)]
    pub omitted: Vec<String>,
    #[serde(default)]
    pub introduced: Vec<String>,
}

impl Taken {
    /// Every commitment identity this decision mentions, whichever half it mentions it in.
    ///
    /// A fork states outcomes rather than a transition, so what a decision *is about* is the union
    /// of the three sets and not any one of them.
    pub fn commitments(&self) -> Vec<&str> {
        self.selection
            .iter()
            .chain(&self.omitted)
            .chain(&self.introduced)
            .map(String::as_str)
            .collect()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct World {
    pub thesis: String,
    pub thesis_parent: Option<String>,
    pub known_at: String,
    pub event_head: Option<String>,
    pub frozen: Vec<String>,
    pub open: Vec<String>,
}

/// The four files, and the addresses the journal came to.
///
/// The journal is a sequence and `custody` is its addresses **in the same order**, which is what
/// lets an entry be named by its identity without this row replaying anything. Pairing them is the
/// one derivation here, and it is checked rather than assumed: see [`Record::open`].
#[derive(Debug, Clone)]
pub struct Record {
    pub journal: Vec<Entry>,
    pub custody: Vec<String>,
    pub lineage: Vec<Taken>,
    pub worlds: Vec<World>,
}

#[derive(Debug)]
pub enum RecordError {
    Unreadable { file: PathBuf, why: String },
    NoPointer { root: PathBuf },
    LengthsDisagree { journal: usize, custody: usize },
    DecisionsAndWorldsDisagree { lineage: usize, worlds: usize },
}

impl std::fmt::Display for RecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unreadable { file, why } => {
                write!(f, "{} could not be read: {why}", file.display())
            }
            Self::NoPointer { root } => write!(
                f,
                "{} has no `current`, so which generation a reader reads is unknown",
                root.display()
            ),
            Self::LengthsDisagree { journal, custody } => write!(
                f,
                "the journal offers {journal} entries and custody claims {custody} addresses, so \
                 pairing them by position would name the wrong entry"
            ),
            Self::DecisionsAndWorldsDisagree { lineage, worlds } => write!(
                f,
                "the record holds {lineage} decisions and {worlds} worlds, so pairing them by \
                 position would give a decision the wrong world"
            ),
        }
    }
}

impl Record {
    /// Open the record the way a reader with a directory does: follow `current`, read four files.
    ///
    /// The pointer is followed rather than guessed. `run-a/mine` holds two generations and only one
    /// of them is live; carving the other would be carving a record nobody reads.
    pub fn open(root: &Path) -> Result<Self, RecordError> {
        let pointer = std::fs::read_to_string(root.join("current"))
            .map_err(|_| RecordError::NoPointer {
                root: root.to_path_buf(),
            })?
            .trim()
            .to_owned();

        let live = root.join(pointer);
        let read = |name: &str| -> Result<String, RecordError> {
            let file = live.join(name);
            std::fs::read_to_string(&file).map_err(|why| RecordError::Unreadable {
                file,
                why: why.to_string(),
            })
        };
        fn parse<T: serde::de::DeserializeOwned>(
            live: &Path,
            name: &str,
            text: &str,
        ) -> Result<T, RecordError> {
            serde_json::from_str(text).map_err(|why| RecordError::Unreadable {
                file: live.join(name),
                why: why.to_string(),
            })
        }

        let journal: Vec<Entry> = parse(&live, "journal.json", &read("journal.json")?)?;
        let custody: Vec<String> = parse(&live, "custody.json", &read("custody.json")?)?;
        let lineage: Vec<Taken> = parse(&live, "lineage.json", &read("lineage.json")?)?;
        let worlds: Vec<World> = parse(&live, "worlds.json", &read("worlds.json")?)?;

        if journal.len() != custody.len() {
            return Err(RecordError::LengthsDisagree {
                journal: journal.len(),
                custody: custody.len(),
            });
        }

        if lineage.len() != worlds.len() {
            return Err(RecordError::DecisionsAndWorldsDisagree {
                lineage: lineage.len(),
                worlds: worlds.len(),
            });
        }

        Ok(Self {
            journal,
            custody,
            lineage,
            worlds,
        })
    }

    /// Each decision with the world it produced, paired by position.
    ///
    /// Position is the only pairing the record offers — `lineage.json` names no world and
    /// `worlds.json` names no decision — and it is the application's own rule: `reading` refuses a
    /// record whose decisions produce a different number of worlds than it recorded. Checked at
    /// [`Record::open`] by the same comparison, so a mispaired record never reaches here.
    pub fn decided(&self) -> Vec<(&Taken, &World)> {
        self.lineage.iter().zip(&self.worlds).collect()
    }

    /// Each entry with the address it produced.
    pub fn addressed(&self) -> Vec<(&str, &Entry)> {
        self.custody
            .iter()
            .map(String::as_str)
            .zip(&self.journal)
            .collect()
    }

    /// The labels the record introduces, by the identity that introduced each.
    ///
    /// Lowercased, because a testimony writes *Finance* at the start of a sentence and *finance*
    /// inside one, and the record's own casing is not a fact about the claim.
    pub fn labelled(&self) -> BTreeMap<String, &str> {
        self.addressed()
            .into_iter()
            .filter_map(|(id, entry)| entry.label().map(|label| (label.to_lowercase(), id)))
            .collect()
    }

    /// Every identity the record holds, from any of its four files.
    pub fn identities(&self) -> Vec<&str> {
        let mut every: Vec<&str> = self.custody.iter().map(String::as_str).collect();

        for world in &self.worlds {
            every.push(&world.thesis);
            every.extend(world.thesis_parent.as_deref());
            every.extend(world.frozen.iter().map(String::as_str));
            every.extend(world.open.iter().map(String::as_str));
            every.extend(world.event_head.as_deref());
        }

        every.sort_unstable();
        every.dedup();
        every
    }
}
