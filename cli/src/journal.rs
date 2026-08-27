//! The journal: admissions in the order they were made, in a form that outlives the process.
//!
//! Observation 1 established that canonical knowledge is reconstructed by replaying
//! admissions rather than by loading records. This is what gets replayed, and writing it
//! answered a question the experiment had left open: it cannot be derived from the
//! knowledge it produced.
//!
//! A `Constraint` is opaque — it offers constructors and `check`, and no way to read its
//! bounds back — so an admitted `Resource` does not yield the constraint it was built
//! with. Nothing here is therefore extracted from an entity. Every record captures the
//! input **as it was supplied**, which is also the only thing replay needs.
//!
//! The representation mirrors the inputs rather than the engine's types, because none of
//! those types can be read back from bytes. Ids are the exception: an identity round-trips
//! as hex, which is what lets a record refer to another without inventing a naming scheme.
//! Storing an id is not caching a derivation — replay re-derives every id from content,
//! and a stored one is how an edge of the graph is written down.
//!
//! Only what the subject admits is modelled. A constraint form the experiment does not use
//! is absent rather than approximated, so reaching for one fails to compile instead of
//! being quietly rewritten as something else.
//!
//! # Addressing an entry
//!
//! A journal is replayed as a whole only when nothing has to be said about *where* in it
//! something happened. Once something does — a decision taken while the journal was shorter
//! than it is now — an entry has to be nameable, and [`EntryId`] is how. Replay therefore
//! hands back one address per admission, including the two that previously produced nothing
//! a caller could hold: an eligibility and an Event.
//!
//! [`replay_through`] is what a reconstruction uses instead of [`replay`]: it admits up to a
//! named entry and stops, so that something else can happen against exactly the knowledge
//! that stood then.
//!
//! Earned by: 00-reconstruction (Confirmed), 05-coordination (Confirmed),
//! 11-veracity (Confirmed), 16-custody (Confirmed)

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use ape::canon::{Canon, EventSubmission};
use ape::kernel::entities::{
    ActionId, ActionInput, AgentId, AgentInput, CommitmentId, CommitmentInput,
    EligibilityAssignmentInput, ResourceId, ResourceInput, ResourceInstanceId,
    ResourceInstanceInput, RoleId, RoleInput, StatementId, StatementInput,
};
use ape::kernel::value_objects::{
    ActionKind, ActionValue, Assignment, Constraint, Date, Effect, Identifier, Observation,
    Participants, ResourceKind, Settlement, Term,
};

use crate::error::JournalError;
use crate::history::ResidentHistory;

/// How this application writes a count: as a decimal string.
///
/// A string and not a JSON number, for two reasons, and the second is the one that outlives Rust.
///
/// Serde cannot read a 128-bit integer out of an **internally tagged** enum — it routes one through a
/// buffer that has no case for it — and every record here is tagged, so the file says what each entry
/// is. And a JSON *number* is a double to most things that read JSON, so a count written as one comes
/// back rounded to whoever is not this program: the hazard the engine stopped having, arriving again
/// at the file.
///
/// It is attached to the fields rather than to a newtype so that nothing which merely *builds* a
/// record has to know how the record is written. And it is the application's decision, not the
/// engine's — another application may hold the same count in a column, a varint or a fixed-point
/// field. What it may not do is let the exactness stop at its boundary.
/// It is written for a count of either signedness, because the two are not interchangeable and both
/// are recorded: a magnitude is unsigned, and a bound is a level and a level can be negative.
pub(crate) mod count {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{Deserialize, Deserializer, Serializer, de::Error};

    pub fn serialize<T: Display, S: Serializer>(
        count: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&count.to_string())
    }

    pub fn deserialize<'de, T: FromStr, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<T, D::Error> {
        let written = String::deserialize(deserializer)?;

        written
            .parse()
            .map_err(|_| D::Error::custom(format!("{written:?} is not a count")))
    }

    /// The same, where a discrete action carries no magnitude at all.
    pub mod maybe {
        use std::fmt::Display;
        use std::str::FromStr;

        use serde::{Deserialize, Deserializer, Serializer, de::Error};

        pub fn serialize<T: Display, S: Serializer>(
            count: &Option<T>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            match count {
                Some(count) => super::serialize(count, serializer),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, T: FromStr, D: Deserializer<'de>>(
            deserializer: D,
        ) -> Result<Option<T>, D::Error> {
            let Some(written) = Option::<String>::deserialize(deserializer)? else {
                return Ok(None);
            };

            written
                .parse()
                .map(Some)
                .map_err(|_| D::Error::custom(format!("{written:?} is not a count")))
        }
    }
}

/// One admission, carrying its fields and the instant it was recorded at.
///
/// `recorded_at` is the one value here that is neither content nor reference: it is
/// assigned when knowledge is admitted and derived from nothing, so it exists after the
/// process only because it was written down.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "admits", rename_all = "kebab-case")]
pub enum Admission {
    Role {
        label: String,
        recorded_at: String,
    },
    Agent {
        label: String,
        recorded_at: String,
    },
    Eligibility {
        agent: AgentId,
        roles: BTreeSet<RoleId>,
        effective_from: String,
        recorded_at: String,
    },
    Resource {
        label: String,
        kind: ResourceKindRecord,
        recorded_at: String,
    },
    ResourceInstance {
        label: String,
        resource: ResourceId,
        recorded_at: String,
    },
    Action {
        verb: String,
        kind: ActionKindRecord,
        resource: ResourceId,
        recorded_at: String,
    },
    Statement {
        actors: BTreeSet<RoleId>,
        recipients: BTreeSet<RoleId>,
        action: ActionId,
        fulfills: BTreeSet<String>,
        cancels: BTreeSet<String>,
        recorded_at: String,
    },
    Commitment {
        accountable: AgentId,
        executors: BTreeSet<AgentId>,
        beneficiaries: BTreeSet<AgentId>,
        statement: StatementId,
        resource: ResourceInstanceId,
        committed_at: String,
        due_date: String,
        #[serde(with = "count::maybe")]
        magnitude: Option<u128>,
        dependencies: BTreeSet<CommitmentId>,
        recorded_at: String,
    },
    Event {
        commitment: CommitmentId,
        observation: String,
        occurred_at: String,
        recorded_at: String,
    },
}

impl Admission {
    /// When this admission says knowledge entered the system.
    ///
    /// The one field of an entry that is neither content nor reference, and therefore the one an
    /// [`EntryId`] does not cover. Two journals can hold the same entry and disagree here, which is
    /// why anything comparing journals has to reach it explicitly.
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EffectRecord {
    Increase,
    Decrease,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ActionKindRecord {
    Discrete,
    Quantifiable(EffectRecord),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResourceKindRecord {
    Discrete,
    Between {
        #[serde(with = "count")]
        lower: i128,
        #[serde(with = "count")]
        upper: i128,
    },
}

/// The address of one journal entry: the identity admitting it produced.
///
/// It is content-addressed, because every identity in the engine is, so it names an entry
/// rather than a place. A journal reordered, split across files or re-encoded still holds the
/// entry this addresses; an offset would hold none of them. Storing one is not caching a
/// derivation for the reason the module already gives — replay re-derives every identity from
/// content, and comparing the two is what makes a stored address checkable at all.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EntryId(String);

impl EntryId {
    /// Address an entry by what admitting it produced.
    ///
    /// The `AsRef` bound is what keeps this to identities. Everything else an admission
    /// carries is content, and content is not unique to the entry that carries it.
    pub fn of(id: impl AsRef<[u8; 32]> + std::fmt::Display) -> Self {
        Self(id.to_string())
    }
}

impl std::fmt::Display for EntryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// The identities replay produced, in the order they were admitted.
///
/// Replay does not carry names, so what it hands back is what it made. A caller that knows
/// which admission it wrote knows which identity to take.
///
/// `entries` is the same information without the types, one address per admission, and it
/// doubles as the cursor: its length is how much of a journal has been admitted, so a partial
/// replay can be resumed from the value that records it.
#[derive(Debug, Default, Clone)]
pub struct Replayed {
    pub roles: Vec<RoleId>,
    pub agents: Vec<AgentId>,
    pub resources: Vec<ResourceId>,
    pub instances: Vec<ResourceInstanceId>,
    pub actions: Vec<ActionId>,
    pub statements: Vec<StatementId>,
    pub commitments: Vec<CommitmentId>,
    pub entries: Vec<EntryId>,
}

/// Admit every entry in order, through the same public path that produced it.
///
/// Order is the journal's, not a sort: each admission resolves references through the
/// knowledge admitted before it, and the recording watermark refuses anything that would
/// arrive out of the sequence history already has.
pub fn replay(
    canon: &mut Canon<ResidentHistory>,
    journal: &[Admission],
) -> Result<Replayed, JournalError> {
    let mut replayed = Replayed::default();

    replay_remaining(canon, journal, &mut replayed)?;

    Ok(replayed)
}

/// What a journal comes to, as addresses, in the order admitted.
///
/// Into a canon of its own, because the question is what the journal *is* rather than what it makes
/// a caller know — admitting into the caller's would leave knowledge somewhere nobody asked for it.
///
/// It is a function of the journal and of nothing else, which is what makes it the one derived value
/// a repository can write without being handed it. See [`crate::repository`].
pub fn addresses(journal: &[Admission]) -> Result<Vec<EntryId>, JournalError> {
    let mut aside = Canon::new(ResidentHistory::new());

    Ok(replay(&mut aside, journal)?.entries)
}

/// Admit whatever `replayed` has not reached yet.
pub fn replay_remaining(
    canon: &mut Canon<ResidentHistory>,
    journal: &[Admission],
    replayed: &mut Replayed,
) -> Result<(), JournalError> {
    while replayed.entries.len() < journal.len() {
        step(canon, journal, replayed)?;
    }

    Ok(())
}

/// Admit up to and including `entry`, and no further.
///
/// What stops is the *admitting*, so the caller is left holding knowledge as it stood when
/// that entry was the most recent one. Naming the entry already admitted last admits nothing
/// further, which is what lets two things happen against one body of knowledge — planning
/// advances and then forks, and nothing is admitted in between.
///
/// Two disagreements are refused rather than absorbed, because either would answer with a
/// world nobody reasoned about. An entry the journal does not hold is not the end of it. And
/// an entry admitted *before* the last one is a caller whose order contradicts the journal's,
/// which admitting nothing would silently answer with too much knowledge.
pub fn replay_through(
    canon: &mut Canon<ResidentHistory>,
    journal: &[Admission],
    replayed: &mut Replayed,
    entry: &EntryId,
) -> Result<(), JournalError> {
    if replayed.entries.last() == Some(entry) {
        return Ok(());
    }

    if replayed.entries.contains(entry) {
        return Err(JournalError::EntryAlreadyPassed(entry.clone()));
    }

    while replayed.entries.len() < journal.len() {
        step(canon, journal, replayed)?;

        if replayed.entries.last() == Some(entry) {
            return Ok(());
        }
    }

    Err(JournalError::UnknownEntry(entry.clone()))
}

fn step(
    canon: &mut Canon<ResidentHistory>,
    journal: &[Admission],
    replayed: &mut Replayed,
) -> Result<(), JournalError> {
    let address = admit(canon, &journal[replayed.entries.len()], replayed)?;

    replayed.entries.push(address);

    Ok(())
}

fn admit(
    canon: &mut Canon<ResidentHistory>,
    entry: &Admission,
    replayed: &mut Replayed,
) -> Result<EntryId, JournalError> {
    let address = match entry {
        Admission::Role { label, recorded_at } => {
            let id = canon.admit_role(
                RoleInput {
                    label: identifier(label, "role label")?,
                },
                date(recorded_at)?,
            )?;
            replayed.roles.push(id);
            EntryId::of(id)
        }

        Admission::Agent { label, recorded_at } => {
            let id = canon.admit_agent(
                AgentInput {
                    label: identifier(label, "agent label")?,
                },
                date(recorded_at)?,
            )?;
            replayed.agents.push(id);
            EntryId::of(id)
        }

        Admission::Eligibility {
            agent,
            roles,
            effective_from,
            recorded_at,
        } => {
            let id = canon.admit_eligibility(
                EligibilityAssignmentInput {
                    agent: *agent,
                    roles: roles.clone(),
                    effective_from: date(effective_from)?,
                },
                date(recorded_at)?,
            )?;
            EntryId::of(id)
        }

        Admission::Resource {
            label,
            kind,
            recorded_at,
        } => {
            let id = canon.admit_resource(
                ResourceInput {
                    label: identifier(label, "resource label")?,
                    kind: match kind {
                        ResourceKindRecord::Discrete => ResourceKind::Discrete,
                        ResourceKindRecord::Between { lower, upper } => ResourceKind::Quantifiable(
                            Constraint::between(*lower, *upper)
                                .map_err(|e| JournalError::malformed("constraint", e))?,
                        ),
                    },
                },
                date(recorded_at)?,
            )?;
            replayed.resources.push(id);
            EntryId::of(id)
        }

        Admission::ResourceInstance {
            label,
            resource,
            recorded_at,
        } => {
            let id = canon.admit_resource_instance(
                ResourceInstanceInput {
                    label: identifier(label, "instance label")?,
                    resource: *resource,
                },
                date(recorded_at)?,
            )?;
            replayed.instances.push(id);
            EntryId::of(id)
        }

        Admission::Action {
            verb,
            kind,
            resource,
            recorded_at,
        } => {
            let id = canon.admit_action(
                ActionInput {
                    verb: identifier(verb, "action verb")?,
                    kind: match kind {
                        ActionKindRecord::Discrete => ActionKind::Discrete,
                        ActionKindRecord::Quantifiable(EffectRecord::Increase) => {
                            ActionKind::Quantifiable(Effect::Increase)
                        }
                        ActionKindRecord::Quantifiable(EffectRecord::Decrease) => {
                            ActionKind::Quantifiable(Effect::Decrease)
                        }
                    },
                    resource: *resource,
                },
                date(recorded_at)?,
            )?;
            replayed.actions.push(id);
            EntryId::of(id)
        }

        Admission::Statement {
            actors,
            recipients,
            action,
            fulfills,
            cancels,
            recorded_at,
        } => {
            let id = canon.admit_statement(
                StatementInput {
                    participants: Participants::new(actors.clone(), recipients.clone())
                        .map_err(|e| JournalError::malformed("participants", e))?,
                    action: *action,
                    settlement: Settlement::new(
                        observations(fulfills, "fulfilling observation")?,
                        observations(cancels, "cancelling observation")?,
                    )
                    .map_err(|e| JournalError::malformed("settlement", e))?,
                },
                date(recorded_at)?,
            )?;
            replayed.statements.push(id);
            EntryId::of(id)
        }

        Admission::Commitment {
            accountable,
            executors,
            beneficiaries,
            statement,
            resource,
            committed_at,
            due_date,
            magnitude,
            dependencies,
            recorded_at,
        } => {
            let id = canon.admit_commitment(
                CommitmentInput {
                    assignment: Assignment::new(
                        *accountable,
                        executors.clone(),
                        beneficiaries.clone(),
                    )
                    .map_err(|e| JournalError::malformed("assignment", e))?,
                    statement: *statement,
                    resource: *resource,
                    term: Term::new(date(committed_at)?, date(due_date)?)
                        .map_err(|e| JournalError::malformed("term", e))?,
                    action_value: match magnitude {
                        None => ActionValue::none(),
                        Some(magnitude) => ActionValue::value(*magnitude)
                            .map_err(|e| JournalError::malformed("action value", e))?,
                    },
                    dependencies: dependencies.clone(),
                },
                date(recorded_at)?,
            )?;
            replayed.commitments.push(id);
            EntryId::of(id)
        }

        Admission::Event {
            commitment,
            observation,
            occurred_at,
            recorded_at,
        } => {
            // `previous_event` is absent on purpose: the Canon reads the head it is
            // extending. Replaying in order is what makes the chain rebuild itself, and
            // writing the link down would be storing a derivation that could disagree.
            let id = canon.admit_event(
                EventSubmission {
                    commitment_id: *commitment,
                    observation: observation_of(observation, "event observation")?,
                    occurred_at: date(occurred_at)?,
                },
                date(recorded_at)?,
            )?;
            EntryId::of(id)
        }
    };

    Ok(address)
}

fn date(value: &str) -> Result<Date, JournalError> {
    Date::parse(value).map_err(|e| JournalError::malformed("date", e))
}

fn identifier(value: &str, field: &'static str) -> Result<Identifier, JournalError> {
    Identifier::new(value).map_err(|e| JournalError::malformed(field, e))
}

fn observation_of(value: &str, field: &'static str) -> Result<Observation, JournalError> {
    Observation::new(value).map_err(|e| JournalError::malformed(field, e))
}

fn observations(
    values: &BTreeSet<String>,
    field: &'static str,
) -> Result<Vec<Observation>, JournalError> {
    values
        .iter()
        .map(|value| observation_of(value, field))
        .collect()
}
