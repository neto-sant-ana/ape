//! The `Canon` — the canonical admission layer.
//!
//! It is the single entry point through which knowledge becomes history: it
//! delegates structural validation to the Axiom, enriches the assertion with
//! canonical metadata (`recorded_at`, via [`Canonical`]), and admits it through
//! the mechanical [`CanonicalHistory`] primitives. Applications hold a `Canon`,
//! never the history directly, so an assertion cannot reach storage without having
//! passed the Axiom — the reason the admission rules live here and not in an
//! adapter.

use crate::kernel::axiom::Axiom;
use crate::kernel::entities::{CommitmentId, CommitmentInput, EligibilityAssignmentInput, EventInput};
use crate::kernel::value_objects::{Date, Observation};

use super::{AppendOutcome, CanonError, Canonical, CanonicalHistory};

/// An event as submitted to the Canon: everything an `Event` carries except its
/// place in the chain. The caller neither knows nor sets `previous_event`; the
/// Canon stamps it with the current head so the link cannot be forged.
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

    pub fn admit_commitment(
        &mut self,
        input: CommitmentInput,
        recorded_at: Date,
    ) -> Result<AppendOutcome, CanonError> {
        let commitment = Axiom::new(&self.history).admit_commitment(input)?;
        let record = Canonical::new(commitment, recorded_at)?;

        Ok(self.history.put_commitment(record))
    }

    pub fn admit_eligibility(
        &mut self,
        input: EligibilityAssignmentInput,
        recorded_at: Date,
    ) -> Result<AppendOutcome, CanonError> {
        let eligibility = Axiom::new(&self.history).admit_eligibility_assignment(input)?;
        let record = Canonical::new(eligibility, recorded_at)?;

        Ok(self.history.put_eligibility(record))
    }

    pub fn admit_event(
        &mut self,
        submission: EventSubmission,
        recorded_at: Date,
    ) -> Result<AppendOutcome, CanonError> {
        // The Canon stamps the link with the head it reads; the same head is what
        // `advance_head` checks against, so a concurrent extension is caught by the
        // compare-and-swap rather than by this read.
        let previous = self.history.head();

        let event = Axiom::new(&self.history).admit_event(EventInput {
            commitment_id: submission.commitment_id,
            observation: submission.observation,
            previous_event: previous,
            occurred_at: submission.occurred_at,
        })?;

        let id = event.id();
        let record = Canonical::new(event, recorded_at)?;

        self.history.put_event(record);
        self.history.advance_head(previous, id)?;

        Ok(AppendOutcome::Admitted)
    }
}
