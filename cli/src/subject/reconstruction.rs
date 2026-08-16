//! The reconstruction subject: the smallest operational graph that exercises both intended
//! and observed reality.
//!
//! ```text
//! Agent ── Role ── Statement ──▶ Action ──▶ Resource
//!                      │
//!                  Commitment ──▶ Event
//! ```
//!
//! The domain is irrelevant and deliberately thin. What the subject must supply is a
//! complete semantic path: a quantifiable resource, so that interpreting it exposes both a
//! commitment's condition and a derived consequence on a level; one commitment, so there
//! is an intention to settle; and no dependencies, so nothing is waiting on anything but
//! the event.
//!
//! The subject is given as a **journal** rather than as a sequence of calls, because a
//! journal is what survives a process and a sequence of calls is not. Constructing it is
//! replaying it, so what Phase 1 exercises is what Phase 6 has to reproduce, and there is
//! no second description of the subject that could disagree with the first.
//!
//! Authoring it takes several passes, because a reference is an identity and an identity
//! exists only once the thing it names has been admitted. Reading it back takes one: the
//! journal that comes out is flat and complete, and that is the shape a fresh process gets.
//!
//! Every quantity is an integer. Feasibility accumulates levels in `f64`, where addition
//! is not associative, and a reconstruction that read records back in a different order
//! could differ in the last bit and flip a comparison against a constraint. Integers keep
//! the experiment measuring reconstruction rather than float determinism.

use ape::canon::Canon;
use ape::kernel::entities::{CommitmentId, ResourceInstanceId};

use crate::error::JournalError;
use crate::history::ResidentHistory;
use crate::journal::{
    ActionKindRecord, Admission, AgentKindRecord, EffectRecord, ResourceKindRecord, replay,
};
use crate::lineage::Decision;

pub const FULFILLING: &str = "Delivered";
pub const CANCELLING: &str = "Cancelled";

/// What the procedure refers to across phases, and the journal that produced it.
pub struct Constructed {
    pub commitment: CommitmentId,
    pub instance: ResourceInstanceId,
    /// Every admission made, in order. Replaying this into a fresh history reproduces the
    /// world exactly, which is what makes it the thing worth persisting.
    pub journal: Vec<Admission>,
}

/// Admit the subject, accumulating the journal that describes it.
pub fn construct(canon: &mut Canon<ResidentHistory>) -> Result<Constructed, JournalError> {
    let mut journal = Vec::new();

    let vocabulary = vec![
        Admission::Role {
            label: "supplier".into(),
            recorded_at: day(1),
        },
        Admission::Role {
            label: "buyer".into(),
            recorded_at: day(1),
        },
        Admission::Agent {
            label: "shipper".into(),
            kind: AgentKindRecord::Company,
            recorded_at: day(1),
        },
        Admission::Agent {
            label: "receiver".into(),
            kind: AgentKindRecord::Company,
            recorded_at: day(1),
        },
        Admission::Resource {
            label: "inventory".into(),
            kind: ResourceKindRecord::Between {
                lower: 0.0,
                upper: 100.0,
            },
            recorded_at: day(1),
        },
    ];
    let named = replay(canon, &vocabulary)?;
    journal.extend(vocabulary);

    let (supplier, buyer) = (named.roles[0], named.roles[1]);
    let (shipper, receiver) = (named.agents[0], named.agents[1]);
    let inventory = named.resources[0];

    let bound = vec![
        Admission::Eligibility {
            agent: shipper,
            roles: [supplier].into(),
            effective_from: day(1),
            recorded_at: day(1),
        },
        Admission::Eligibility {
            agent: receiver,
            roles: [buyer].into(),
            effective_from: day(1),
            recorded_at: day(1),
        },
        Admission::ResourceInstance {
            label: "warehouse".into(),
            resource: inventory,
            recorded_at: day(1),
        },
        Admission::Action {
            verb: "deliver".into(),
            kind: ActionKindRecord::Quantifiable(EffectRecord::Increase),
            resource: inventory,
            recorded_at: day(1),
        },
    ];
    let placed = replay(canon, &bound)?;
    journal.extend(bound);

    let instance = placed.instances[0];

    let proposed = vec![Admission::Statement {
        actors: [supplier].into(),
        recipients: [buyer].into(),
        action: placed.actions[0],
        fulfills: [FULFILLING.to_owned()].into(),
        cancels: [CANCELLING.to_owned()].into(),
        recorded_at: day(1),
    }];
    let stated = replay(canon, &proposed)?;
    journal.extend(proposed);

    let intended = vec![Admission::Commitment {
        accountable: shipper,
        executors: [shipper].into(),
        beneficiaries: [receiver].into(),
        statement: stated.statements[0],
        resource: instance,
        committed_at: day(5),
        due_date: day(20),
        magnitude: Some(10.0),
        dependencies: [].into(),
        recorded_at: day(5),
    }];
    let committed = replay(canon, &intended)?;
    journal.extend(intended);

    Ok(Constructed {
        commitment: committed.commitments[0],
        instance,
        journal,
    })
}

/// The genesis decision: which world Phase 1 reasons about, and when it is taken.
///
/// The instant sits after the commitment was recorded and before it is due, so the cut
/// resolves an empty chain — there is an intention and nothing has settled it.
pub fn genesis(commitment: CommitmentId) -> Decision {
    Decision::Genesis {
        known_at: day(10),
        selection: [commitment].into(),
    }
}

/// The advancement Phase 2 needs, because a cut cannot recognize an Event it predates.
pub fn advancement() -> Decision {
    Decision::Advance { known_at: day(15) }
}

/// The settling Event, which Phase 2 admits after the world has been interpreted without it.
pub fn settlement(commitment: CommitmentId) -> Admission {
    Admission::Event {
        commitment,
        observation: FULFILLING.into(),
        occurred_at: day(12),
        recorded_at: day(12),
    }
}

fn day(day: u8) -> String {
    format!("2026-01-{day:02}")
}
