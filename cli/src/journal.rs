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

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use ape::canon::{Canon, EventSubmission};
use ape::kernel::entities::{
    ActionId, ActionInput, AgentId, AgentInput, CommitmentId, CommitmentInput,
    EligibilityAssignmentInput, ResourceId, ResourceInput, ResourceInstanceId,
    ResourceInstanceInput, RoleId, RoleInput, StatementId, StatementInput,
};
use ape::kernel::value_objects::{
    ActionKind, ActionValue, AgentKind, Assignment, Constraint, Date, Effect, Identifier,
    Observation, Participants, ResourceKind, Settlement, Term,
};

use crate::error::JournalError;
use crate::history::ResidentHistory;

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
        kind: AgentKindRecord,
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
        magnitude: Option<f64>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AgentKindRecord {
    Company,
    Individual,
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
    Between { lower: f64, upper: f64 },
}

/// The identities replay produced, in the order they were admitted.
///
/// Replay does not carry names, so what it hands back is what it made. A caller that knows
/// which admission it wrote knows which identity to take.
#[derive(Debug, Default)]
pub struct Replayed {
    pub roles: Vec<RoleId>,
    pub agents: Vec<AgentId>,
    pub resources: Vec<ResourceId>,
    pub instances: Vec<ResourceInstanceId>,
    pub actions: Vec<ActionId>,
    pub statements: Vec<StatementId>,
    pub commitments: Vec<CommitmentId>,
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

    for entry in journal {
        admit(canon, entry, &mut replayed)?;
    }

    Ok(replayed)
}

fn admit(
    canon: &mut Canon<ResidentHistory>,
    entry: &Admission,
    replayed: &mut Replayed,
) -> Result<(), JournalError> {
    match entry {
        Admission::Role { label, recorded_at } => {
            let id = canon.admit_role(
                RoleInput {
                    label: identifier(label, "role label")?,
                },
                date(recorded_at)?,
            )?;
            replayed.roles.push(id);
        }

        Admission::Agent {
            label,
            kind,
            recorded_at,
        } => {
            let id = canon.admit_agent(
                AgentInput {
                    label: identifier(label, "agent label")?,
                    kind: match kind {
                        AgentKindRecord::Company => AgentKind::Company,
                        AgentKindRecord::Individual => AgentKind::Individual,
                    },
                },
                date(recorded_at)?,
            )?;
            replayed.agents.push(id);
        }

        Admission::Eligibility {
            agent,
            roles,
            effective_from,
            recorded_at,
        } => {
            canon.admit_eligibility(
                EligibilityAssignmentInput {
                    agent: *agent,
                    roles: roles.clone(),
                    effective_from: date(effective_from)?,
                },
                date(recorded_at)?,
            )?;
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
                        ResourceKindRecord::Between { lower, upper } => {
                            ResourceKind::Quantifiable(
                                Constraint::between(*lower, *upper)
                                    .map_err(|e| JournalError::malformed("constraint", e))?,
                            )
                        }
                    },
                },
                date(recorded_at)?,
            )?;
            replayed.resources.push(id);
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
            canon.admit_event(
                EventSubmission {
                    commitment_id: *commitment,
                    observation: observation_of(observation, "event observation")?,
                    occurred_at: date(occurred_at)?,
                },
                date(recorded_at)?,
            )?;
        }
    }

    Ok(())
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
