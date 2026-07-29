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

use super::{Condition, Conflict, Dependencies, Hypothesis, Outcome, Projection, ProjectionError};

use crate::kernel::axiom::Knowledge;

use crate::kernel::entities::{Commitment, CommitmentId, Event, ResourceInstanceId};

use crate::kernel::value_objects::{ActionKind, Constraint, Date, Effect, ResourceKind};

#[derive(Debug, Clone)]
struct Movement {
    instance: ResourceInstanceId,
    magnitude: f64,
}

/// How many movements may share one instant on one resource before the levels their
/// arrangements can produce stop being worth enumerating.
/// Deciding a group means asking whether *every* arrangement of it stays within bounds, and the
/// levels reachable within a group are the sums of its subsets.
/// Beyond this many, the group is refused rather than approximated.
const SIMULTANEOUS_LIMIT: usize = 16;

/// Every level the `simultaneous` movements can produce from `level`, in any order.
fn reachable_levels(level: f64, simultaneous: &[f64]) -> Vec<f64> {
    (1..(1u32 << simultaneous.len()))
        .map(|landed| {
            level
                + simultaneous
                    .iter()
                    .enumerate()
                    .filter(|(slot, _)| landed & (1 << slot) != 0)
                    .map(|(_, magnitude)| magnitude)
                    .sum::<f64>()
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Settled {
    outcome: Outcome,
    occurred_at: Date,
}

#[derive(Debug, Clone, Default)]
pub struct Accumulation {
    selected: BTreeMap<CommitmentId, Commitment>,
    settled: BTreeMap<CommitmentId, Settled>,
    movements: BTreeMap<CommitmentId, Movement>,
    constraints: BTreeMap<ResourceInstanceId, Constraint>,
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

            if let Some(movement) = self.resolve_movement(knowledge, &commitment)? {
                self.movements.insert(*id, movement);
            }

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

            let settled = Settled {
                outcome,
                occurred_at: *event.occurred_at(),
            };

            if self.settled.insert(commitment_id, settled).is_some() {
                return Err(ProjectionError::SettledMoreThanOnce(commitment_id));
            }
        }

        Ok(())
    }

    /// The dependency closure is only required here, not while folding: a dependency may
    /// legitimately arrive in a later `absorb`, but a commitment whose dependency is
    /// missing cannot be told apart from one whose dependency is merely pending.
    pub fn view(&self, at: &Date) -> Result<Projection, ProjectionError> {
        let mut resolved = BTreeMap::new();
        let mut conditions = BTreeMap::new();

        for (id, commitment) in &self.selected {
            let mut pending = false;
            let mut unfulfillable = false;

            for dependency in commitment.dependencies() {
                pending |= self.outcome_of(dependency) == Outcome::Unsettled;
                unfulfillable |= self.unfulfillable(*dependency, &mut resolved)?;
            }

            conditions.insert(
                *id,
                Condition::new(
                    self.outcome_of(id),
                    Dependencies {
                        pending,
                        unfulfillable,
                    },
                    commitment.term(),
                    at,
                ),
            );
        }

        Ok(Projection::new(conditions))
    }

    /// The conflicts the selected graph carries under `hypothesis`, empty when none was
    /// found.
    ///
    /// A commitment still open behind a dependency that can never be fulfilled is reported
    /// first, and on its own. Its presence means no completion exists at all, so assuming
    /// every unsettled commitment is realized is not an assumption that can hold.
    pub fn conflicts(&self, hypothesis: Hypothesis) -> Result<Vec<Conflict>, ProjectionError> {
        let mut resolved = BTreeMap::new();
        let mut doomed = Vec::new();

        for id in self.selected.keys() {
            if self.outcome_of(id) == Outcome::Unsettled
                && self.unfulfillable(*id, &mut resolved)?
            {
                doomed.push(Conflict::Unrealizable(*id));
            }
        }

        if !doomed.is_empty() {
            return Ok(doomed);
        }

        match hypothesis {
            Hypothesis::FinalState => {
                Ok(self.out_of_bounds(self.levels_once_every_movement_lands()))
            }
            Hypothesis::OnDueDate => self.breaches_along_the_punctual_sequence(),
        }
    }

    fn breaches_along_the_punctual_sequence(&self) -> Result<Vec<Conflict>, ProjectionError> {
        let mut conflicts = Vec::new();

        for (instance, sequence) in self.punctual_sequence() {
            let Some(constraint) = self.constraints.get(&instance) else {
                continue;
            };

            let mut level = 0.0;

            for (position, simultaneous) in sequence {
                if simultaneous.len() > SIMULTANEOUS_LIMIT {
                    return Err(ProjectionError::TooManySimultaneousMovements {
                        instance,
                        position,
                        count: simultaneous.len(),
                    });
                }

                if let Some(breach) = reachable_levels(level, &simultaneous)
                    .into_iter()
                    .find(|reachable| !constraint.check(*reachable))
                {
                    conflicts.push(Conflict::OutOfBounds {
                        instance,
                        level: breach,
                    });
                    break;
                }

                level += simultaneous.iter().sum::<f64>();
            }
        }

        Ok(conflicts)
    }

    fn punctual_sequence(&self) -> BTreeMap<ResourceInstanceId, BTreeMap<Date, Vec<f64>>> {
        let mut sequence: BTreeMap<ResourceInstanceId, BTreeMap<Date, Vec<f64>>> = BTreeMap::new();

        for (id, commitment) in &self.selected {
            let Some(movement) = self.movements.get(id) else {
                continue;
            };

            let position = match self.settled.get(id) {
                Some(settled) if settled.outcome == Outcome::Cancelled => continue,
                Some(settled) => settled.occurred_at,
                None => *commitment.term().due_date(),
            };

            sequence
                .entry(movement.instance)
                .or_default()
                .entry(position)
                .or_default()
                .push(movement.magnitude);
        }

        sequence
    }

    fn levels_once_every_movement_lands(&self) -> BTreeMap<ResourceInstanceId, f64> {
        let mut levels = BTreeMap::new();

        for (id, movement) in &self.movements {
            if self.outcome_of(id) == Outcome::Cancelled {
                continue;
            }

            *levels.entry(movement.instance).or_insert(0.0) += movement.magnitude;
        }

        levels
    }

    fn out_of_bounds(&self, levels: BTreeMap<ResourceInstanceId, f64>) -> Vec<Conflict> {
        levels
            .into_iter()
            .filter(|(instance, level)| {
                self.constraints
                    .get(instance)
                    .is_some_and(|constraint| !constraint.check(*level))
            })
            .map(|(instance, level)| Conflict::OutOfBounds { instance, level })
            .collect()
    }

    fn resolve_movement<K: Knowledge>(
        &mut self,
        knowledge: &K,
        commitment: &Commitment,
    ) -> Result<Option<Movement>, ProjectionError> {
        let statement_id = *commitment.statement();
        let statement =
            knowledge
                .statement(statement_id)
                .ok_or(ProjectionError::UnknownStatement {
                    commitment: commitment.id(),
                    statement: statement_id,
                })?;

        let action_id = *statement.action();
        let action = knowledge
            .action(action_id)
            .ok_or(ProjectionError::UnknownAction {
                statement: statement_id,
                action: action_id,
            })?;

        let effect = match (action.kind(), commitment.action_value().as_value()) {
            (ActionKind::Discrete, None) => return Ok(None),
            (ActionKind::Quantifiable(effect), Some(magnitude)) => (effect, magnitude),
            _ => return Err(ProjectionError::ActionValueMismatch(commitment.id())),
        };

        let instance_id = *commitment.resource();
        let instance = knowledge.resource_instance(instance_id).ok_or(
            ProjectionError::UnknownResourceInstance {
                commitment: commitment.id(),
                instance: instance_id,
            },
        )?;

        let resource_id = *instance.resource();
        let resource =
            knowledge
                .resource(resource_id)
                .ok_or(ProjectionError::UnknownResource {
                    instance: instance_id,
                    resource: resource_id,
                })?;

        let ResourceKind::Quantifiable(constraint) = resource.kind() else {
            return Err(ProjectionError::ActionResourceKindMismatch(commitment.id()));
        };

        self.constraints.insert(instance_id, constraint.clone());

        let (effect, magnitude) = effect;

        Ok(Some(Movement {
            instance: instance_id,
            magnitude: match effect {
                Effect::Increase => magnitude,
                Effect::Decrease => -magnitude,
            },
        }))
    }

    fn unfulfillable(
        &self,
        id: CommitmentId,
        resolved: &mut BTreeMap<CommitmentId, bool>,
    ) -> Result<bool, ProjectionError> {
        if let Some(known) = resolved.get(&id) {
            return Ok(*known);
        }

        let commitment = self
            .selected
            .get(&id)
            .ok_or(ProjectionError::UnknownCommitment(id))?;

        let verdict = match self.outcome_of(&id) {
            Outcome::Fulfilled => false,
            Outcome::Cancelled => true,
            Outcome::Unsettled => {
                let mut behind = false;
                for dependency in commitment.dependencies() {
                    behind |= self.unfulfillable(*dependency, resolved)?;
                }
                behind
            }
        };

        resolved.insert(id, verdict);

        Ok(verdict)
    }

    /// Only settled commitments are recorded, so absence from `settled` is what makes a
    /// commitment `Unsettled`.
    fn outcome_of(&self, commitment: &CommitmentId) -> Outcome {
        self.settled
            .get(commitment)
            .map(|settled| settled.outcome.clone())
            .unwrap_or(Outcome::Unsettled)
    }
}
