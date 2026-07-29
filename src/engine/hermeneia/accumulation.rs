//! `Accumulation` — the factual, resumable half of a projection.
//!
//! What accumulates over knowledge is separated from what is interpreted at the end:
//!
//! ```text
//! P = view( fold(knowledge) , T )
//! ```
//!
//! `absorb` folds; `view` interprets. Nothing here depends on the effective time, which
//! is what makes an `Accumulation` a checkpoint: it can be carried forward and
//! interpreted at whichever instant is later asked. Were a deadline evaluation folded in,
//! the checkpoint would only ever be valid for the instant it was taken at.
//!
//! It is also self-sufficient by design. `absorb` resolves each event against its
//! statement *now* and keeps the outcome, so `view` needs no lookups and a resumed
//! checkpoint is interpretable without reaching back into storage.
//!
//! `absorb` may be called repeatedly to fold a chain in pieces — sound because a
//! settlement is terminal, and therefore monotonic along the chain — with one ordering
//! requirement: an event's commitment must already be selected. Knowledge admitted
//! through the Canon satisfies this for free, since a commitment is admitted before any
//! event that settles it and recording never regresses.

use std::collections::BTreeMap;

use super::{Condition, Outcome, Projection, ProjectionError};

use crate::kernel::axiom::Knowledge;

use crate::kernel::entities::{Commitment, CommitmentId, Event};

use crate::kernel::value_objects::Date;

#[derive(Debug, Clone, Default)]
pub struct Accumulation {
    selected: BTreeMap<CommitmentId, Commitment>,
    settled: BTreeMap<CommitmentId, Outcome>,
}
impl Accumulation {
    pub fn absorb<K: Knowledge>(
        &mut self,
        knowledge: &K,
        selection: &[CommitmentId],
        events: &[Event],
    ) -> Result<(), ProjectionError> {
        for id in selection {
            let commitment = knowledge
                .commitment(*id)
                .ok_or(ProjectionError::UnknownCommitment(*id))?;

            self.selected.insert(*id, commitment);
        }

        for event in events {
            let commitment_id = *event.commitment_id();

            let commitment = self
                .selected
                .get(&commitment_id)
                .ok_or(ProjectionError::UnknownCommitment(commitment_id))?;

            let statement = knowledge.statement(*commitment.statement()).ok_or(
                ProjectionError::UnknownStatement {
                    commitment: commitment_id,
                    statement: *commitment.statement(),
                },
            )?;

            let outcome = if statement.settlement().can_settle(event.observation()) {
                Outcome::Fulfilled
            } else if statement.settlement().can_cancel(event.observation()) {
                Outcome::Cancelled
            } else {
                return Err(ProjectionError::ObservationNotSettling { event: event.id() });
            };

            if self.settled.insert(commitment_id, outcome).is_some() {
                return Err(ProjectionError::SettledMoreThanOnce(commitment_id));
            }
        }

        Ok(())
    }

    /// The dependency closure is only required here, not while folding: a dependency may
    /// legitimately arrive in a later `absorb`, but a commitment whose dependency is
    /// missing cannot be told apart from one whose dependency is merely pending.
    pub fn view(&self, at: &Date) -> Result<Projection, ProjectionError> {
        let mut conditions = BTreeMap::new();

        for (id, commitment) in &self.selected {
            let mut pending_dependencies = false;

            for dependency in commitment.dependencies() {
                if !self.selected.contains_key(dependency) {
                    return Err(ProjectionError::UnknownCommitment(*dependency));
                }

                if self.outcome_of(dependency) == Outcome::Unsettled {
                    pending_dependencies = true;
                }
            }

            conditions.insert(
                *id,
                Condition::new(
                    self.outcome_of(id),
                    pending_dependencies,
                    commitment.term(),
                    at,
                ),
            );
        }

        Ok(Projection::new(conditions))
    }

    /// Only settled commitments are recorded, so absence from `settled` is what makes a
    /// commitment `Unsettled`.
    fn outcome_of(&self, commitment: &CommitmentId) -> Outcome {
        self.settled
            .get(commitment)
            .cloned()
            .unwrap_or(Outcome::Unsettled)
    }
}
