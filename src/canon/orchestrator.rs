//! The `Canon` — the canonical admission layer.
//!
//! It is the single entry point through which knowledge becomes history: it
//! delegates structural validation to the Axiom, enriches the assertion with
//! canonical metadata (`recorded_at`, via [`Canonical`]), and admits it through
//! the mechanical [`CanonicalHistory`] primitives.

use super::{CanonError, Canonical, CanonicalHistory};

use crate::kernel::axiom::Axiom;

use crate::kernel::entities::{
    ActionId, ActionInput, AgentId, AgentInput, CommitmentId, CommitmentInput,
    EligibilityAssignmentId, EligibilityAssignmentInput, EventId, EventInput, ResourceId,
    ResourceInput, ResourceInstanceId, ResourceInstanceInput, RoleId, RoleInput, StatementId,
    StatementInput,
};

use crate::kernel::value_objects::{Date, Observation};

pub struct EventSubmission {
    pub commitment_id: CommitmentId,
    pub observation: Observation,
    pub occurred_at: Date,
}

pub struct Canon<H: CanonicalHistory> {
    history: H,
}
impl<H: CanonicalHistory> Canon<H> {
    pub fn new(history: H) -> Self {
        Self { history }
    }

    pub fn history(&self) -> &H {
        &self.history
    }

    canonical_admission! {
        admit_role(RoleInput) -> RoleId { admit_role, put_role },
        admit_agent(AgentInput) -> AgentId { admit_agent, put_agent },
        admit_resource(ResourceInput) -> ResourceId { admit_resource, put_resource },
        admit_resource_instance(ResourceInstanceInput) -> ResourceInstanceId {
            admit_resource_instance, put_resource_instance
        },
        admit_action(ActionInput) -> ActionId { admit_action, put_action },
        admit_statement(StatementInput) -> StatementId { admit_statement, put_statement },
        admit_commitment(CommitmentInput) -> CommitmentId { admit_commitment, put_commitment },
    }

    pub fn admit_eligibility(
        &mut self,
        input: EligibilityAssignmentInput,
        recorded_at: Date,
    ) -> Result<EligibilityAssignmentId, CanonError> {
        if let Some(existing) = self
            .history
            .eligibilities_of(input.agent)
            .into_iter()
            .find(|e| *e.effective_from() == input.effective_from)
        {
            if existing.roles() == &input.roles {
                return Ok(existing.id());
            }
            return Err(CanonError::ConflictingEligibility {
                agent: input.agent,
                effective_from: input.effective_from,
            });
        }

        let eligibility = Axiom::new(&self.history).admit_eligibility_assignment(input)?;
        let id = eligibility.id();
        self.history
            .put_eligibility(Canonical::new(eligibility, recorded_at)?);
        Ok(id)
    }

    /// Admit an event, settling its commitment.
    ///
    /// Reads the head first, then enforces settle-once: if the commitment already
    /// has an event, an identical fact is idempotent and a different one is refused.
    /// The event is stamped with that head and appended under a compare-and-append.
    ///
    /// The order is load-bearing under concurrency. The head is monotonic — each
    /// append extends it with a fresh content-addressed id, never reused — so a
    /// successful append proves the head did not move between the stamp and the
    /// append, and thus that no event was admitted in that window. Reading the head
    /// *before* the settle-once check binds the check to that window: were the head
    /// read last, a racing settlement could land between the check and the stamp,
    /// leaving the check stale while the stamp adopts the already-settled head — the
    /// append would then extend the chain linearly and settle the commitment twice,
    /// with no [`CanonError::UnexpectedHead`] to trigger recovery.
    ///
    /// If the head does move between the stamp and the append, the append is refused
    /// with [`CanonError::UnexpectedHead`], which propagates from here. Recovery is
    /// to call `admit_event` **again** — a fresh admission that re-reads the head and
    /// re-runs the settle-once check against the new state. The caller must not
    /// rebuild the event and hand it to the history directly: only re-admission
    /// re-applies the semantic checks, so settle-once holds under concurrency.
    pub fn admit_event(
        &mut self,
        submission: EventSubmission,
        recorded_at: Date,
    ) -> Result<EventId, CanonError> {
        let previous = self.history.head();

        if let Some(settled) = self.history.event_of(submission.commitment_id) {
            if settled.observation() == &submission.observation
                && *settled.occurred_at() == submission.occurred_at
            {
                return Ok(settled.id());
            }

            return Err(CanonError::CommitmentAlreadySettled(
                submission.commitment_id,
            ));
        }

        let event = Axiom::new(&self.history).admit_event(EventInput {
            commitment_id: submission.commitment_id,
            observation: submission.observation,
            previous_event: previous,
            occurred_at: submission.occurred_at,
        })?;

        let id = event.id();

        self.history
            .append_event(Canonical::new(event, recorded_at)?)?;

        Ok(id)
    }
}
