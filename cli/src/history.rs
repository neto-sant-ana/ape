//! `ResidentHistory` — the canonical history an application holds after replaying it.
//!
//! The first observation established that a durable adapter cannot answer a canonical read
//! by loading one stored record: it receives records through `put_*` and must retain what
//! later reads are answered from. This is that retention, and it is deliberately the whole
//! history rather than a cache — a cache implies something to fall back to.
//!
//! It is not the engine's `InMemoryHistory`. That one is the reference implementation, and
//! an application leaning on it would be exercising the engine's storage instead of its
//! own; the comparison this experiment makes requires the same adapter on both sides of
//! process death.
//!
//! Single-process by construction, because the experiment excludes concurrency. Nothing
//! here is shared between threads, and `verify_thread_safe` is not claimed.
//!
//! Earned by: 00-reconstruction (Confirmed)

use std::collections::BTreeMap;

use ape::canon::{AppendOutcome, CanonError, Canonical, CanonicalHistory, CanonicalKnowledge};
use ape::kernel::axiom::Knowledge;
use ape::kernel::entities::{
    Action, ActionId, Agent, AgentId, Commitment, CommitmentId, EligibilityAssignment,
    EligibilityAssignmentId, Event, EventId, Resource, ResourceId, ResourceInstance,
    ResourceInstanceId, Role, RoleId, Statement, StatementId,
};
use ape::kernel::value_objects::Date;

#[derive(Default)]
pub struct ResidentHistory {
    roles: BTreeMap<RoleId, Canonical<Role>>,
    agents: BTreeMap<AgentId, Canonical<Agent>>,
    resources: BTreeMap<ResourceId, Canonical<Resource>>,
    instances: BTreeMap<ResourceInstanceId, Canonical<ResourceInstance>>,
    actions: BTreeMap<ActionId, Canonical<Action>>,
    statements: BTreeMap<StatementId, Canonical<Statement>>,
    commitments: BTreeMap<CommitmentId, Canonical<Commitment>>,
    eligibility: BTreeMap<EligibilityAssignmentId, Canonical<EligibilityAssignment>>,
    events: BTreeMap<EventId, Canonical<Event>>,
    events_by_commitment: BTreeMap<CommitmentId, EventId>,
    head: Option<EventId>,
    recorded_through: Option<Date>,
}

impl ResidentHistory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Refuse a record that would be back-dated into knowledge already admitted.
    ///
    /// Checking is kept apart from advancing because a refusal must leave no trace, and an
    /// append is refused for two reasons in sequence: a watermark accepted before a stale
    /// head is discovered would move for a record that never entered.
    fn refuse_back_dated(&self, recorded_at: &Date) -> Result<(), CanonError> {
        if let Some(through) = self.recorded_through
            && !through.up_to(recorded_at)
        {
            return Err(CanonError::RecordedOutOfOrder {
                recorded_at: *recorded_at,
                recorded_through: through,
            });
        }

        Ok(())
    }

    /// The watermark moves on every family, so recording is one order across all of them
    /// rather than one per kind.
    fn advance_recording(&mut self, recorded_at: &Date) {
        self.recorded_through = Some(*recorded_at);
    }

    /// The Events of the chain ending at `head`, oldest first.
    fn chain(&self) -> Vec<&Canonical<Event>> {
        let mut walked = Vec::new();
        let mut cursor = self.head;

        while let Some(id) = cursor {
            let Some(record) = self.events.get(&id) else {
                break;
            };

            walked.push(record);
            cursor = *record.assertion().previous_event();
        }

        walked.reverse();
        walked
    }
}

/// `put_*` is put-if-absent: the id is derived from the content, so a re-put is the same
/// record arriving twice and the watermark must not move for it.
macro_rules! resident_put {
    ($($method:ident($type:ty) -> $field:ident),+ $(,)?) => {
        $(
            fn $method(&mut self, record: Canonical<$type>) -> Result<AppendOutcome, CanonError> {
                let id = record.assertion().id();

                if self.$field.contains_key(&id) {
                    return Ok(AppendOutcome::AlreadyPresent);
                }

                self.refuse_back_dated(record.recorded_at())?;

                self.advance_recording(record.recorded_at());
                self.$field.insert(id, record);

                Ok(AppendOutcome::Admitted)
            }
        )+
    };
}

impl Knowledge for ResidentHistory {
    fn role(&self, id: RoleId) -> Option<Role> {
        self.roles.get(&id).map(|r| r.assertion().clone())
    }

    fn agent(&self, id: AgentId) -> Option<Agent> {
        self.agents.get(&id).map(|r| r.assertion().clone())
    }

    fn resource(&self, id: ResourceId) -> Option<Resource> {
        self.resources.get(&id).map(|r| r.assertion().clone())
    }

    fn resource_instance(&self, id: ResourceInstanceId) -> Option<ResourceInstance> {
        self.instances.get(&id).map(|r| r.assertion().clone())
    }

    fn action(&self, id: ActionId) -> Option<Action> {
        self.actions.get(&id).map(|r| r.assertion().clone())
    }

    fn statement(&self, id: StatementId) -> Option<Statement> {
        self.statements.get(&id).map(|r| r.assertion().clone())
    }

    fn commitment(&self, id: CommitmentId) -> Option<Commitment> {
        self.commitments.get(&id).map(|r| r.assertion().clone())
    }

    fn event(&self, id: EventId) -> Option<Event> {
        self.events.get(&id).map(|r| r.assertion().clone())
    }

    fn eligibilities_of(&self, agent: AgentId) -> Vec<EligibilityAssignment> {
        self.eligibility
            .values()
            .map(|r| r.assertion())
            .filter(|e| e.agent() == &agent)
            .cloned()
            .collect()
    }
}

impl CanonicalKnowledge for ResidentHistory {
    fn canonical_commitment(&self, id: CommitmentId) -> Option<Canonical<Commitment>> {
        self.commitments.get(&id).cloned()
    }

    fn canonical_event(&self, id: EventId) -> Option<Canonical<Event>> {
        self.events.get(&id).cloned()
    }

    /// The last Event recorded no later than `at`, in chain order.
    ///
    /// Walking the chain rather than scanning the map is what makes "the last of several
    /// sharing an instant" mean the last one *in the chain*, which is the answer the
    /// contract names.
    fn head_as_of(&self, at: &Date) -> Option<EventId> {
        self.chain()
            .into_iter()
            .take_while(|record| record.recorded_at().up_to(at))
            .last()
            .map(|record| record.assertion().id())
    }
}

impl CanonicalHistory for ResidentHistory {
    fn head(&self) -> Option<EventId> {
        self.head
    }

    fn recorded_through(&self) -> Option<Date> {
        self.recorded_through
    }

    fn event_of(&self, commitment: CommitmentId) -> Option<Event> {
        self.events_by_commitment
            .get(&commitment)
            .and_then(|id| self.event(*id))
    }

    resident_put! {
        put_role(Role) -> roles,
        put_agent(Agent) -> agents,
        put_resource(Resource) -> resources,
        put_resource_instance(ResourceInstance) -> instances,
        put_action(Action) -> actions,
        put_statement(Statement) -> statements,
        put_commitment(Commitment) -> commitments,
        put_eligibility(EligibilityAssignment) -> eligibility,
    }

    /// The watermark is checked before the head, because the two refusals call for opposite
    /// responses: a stale head invites a fresh admission, a back-dated instant would have
    /// that admission fail identically forever.
    fn append_event(&mut self, event: Canonical<Event>) -> Result<AppendOutcome, CanonError> {
        let id = event.assertion().id();

        if self.events.contains_key(&id) {
            return Ok(AppendOutcome::AlreadyPresent);
        }

        self.refuse_back_dated(event.recorded_at())?;

        let previous = *event.assertion().previous_event();
        if previous != self.head {
            return Err(CanonError::UnexpectedHead {
                expected: previous,
                found: self.head,
            });
        }

        let commitment = *event.assertion().commitment_id();

        self.advance_recording(event.recorded_at());
        self.events.insert(id, event);
        self.events_by_commitment.insert(commitment, id);
        self.head = Some(id);

        Ok(AppendOutcome::Admitted)
    }
}
