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
//! `absorb` may be called repeatedly to fold a chain in pieces, — sound because a
//! settlement is terminal, and therefore monotonic along the chain.
//!
//! The events must continue the segment already absorbed. `previous_event` belongs to an
//! event's hashed identity, so each batch proves its own contiguity and where it attaches: an
//! accumulation refuses a batch drawn from another history, one that skips events, or one already
//! folded in.

use std::collections::BTreeMap;

use super::{
    Condition, Conflict, Dependencies, FeasibilityReport, Hypothesis, Outcome, ProjectedConditions,
    ProjectionError,
};

use crate::kernel::axiom::Knowledge;

use crate::kernel::entities::{Commitment, CommitmentId, Event, EventId, ResourceInstanceId};

use crate::kernel::value_objects::{ActionKind, Constraint, Date, Effect, ResourceKind};

#[derive(Debug, Clone)]
struct Movement {
    instance: ResourceInstanceId,
    magnitude: f64,
}

/// How many movements may share one instant on one resource before the levels their
/// arrangements can produce stop being worth enumerating.
///
/// Deciding a group means asking whether *every* admissible arrangement of it stays within bounds.
/// Order within an instant is free only where precedence does not fix it, so the levels reachable
/// are the sums of the subsets closed under the dependencies the group contains, a count bounded
/// by, its subsets. Beyond this many the group is refused rather than approximated.
const SIMULTANEOUS_LIMIT: usize = 16;

/// Every level the movements of one instant can produce from `level`, in any order their own
/// dependencies allow.
fn reachable_levels(level: f64, simultaneous: &[Simultaneous]) -> Vec<f64> {
    (1..(1u32 << simultaneous.len()))
        .filter(|landed| {
            simultaneous.iter().enumerate().all(|(slot, movement)| {
                landed & (1 << slot) == 0 || movement.requires & landed == movement.requires
            })
        })
        .map(|landed| {
            level
                + simultaneous
                    .iter()
                    .enumerate()
                    .filter(|(slot, _)| landed & (1 << slot) != 0)
                    .map(|(_, movement)| movement.magnitude)
                    .sum::<f64>()
        })
        .collect()
}

/// What to judge of the movements a single instant carries: only the level they leave behind, or
/// every level their arrangements can pass through on the way.
#[derive(Debug, Clone, Copy)]
enum Within {
    Net,
    AnyOrder,
}

/// A movement, and which of the movements sharing its instant must land before it.
struct Simultaneous {
    magnitude: f64,
    requires: u32,
}

#[derive(Debug, Clone)]
struct Settled {
    outcome: Outcome,
    occurred_at: Date,
}

/// Which head, if any, this accumulation is bound to interpret.
///
/// `Unbound` folds whatever it is handed and interprets wherever it stops, which is the
/// low-level use: a projection is valid as of the head it was computed from, and saying which
/// is enough. `Bound` is for a caller that already knows the cut being asked about, and it
/// closes the two ways a fold can answer the wrong question — absorbing past the head, or
/// interpreting before reaching it.
#[derive(Debug, Clone, Copy, Default)]
enum Recognition {
    #[default]
    Unbound,
    Bound(Option<EventId>),
}

#[derive(Debug, Clone, Default)]
pub struct Accumulation {
    selected: BTreeMap<CommitmentId, Commitment>,
    settled: BTreeMap<CommitmentId, Settled>,
    movements: BTreeMap<CommitmentId, Movement>,
    constraints: BTreeMap<ResourceInstanceId, Constraint>,
    event_head: Option<EventId>,
    recognition: Recognition,
}
/// Knowledge resolved out of one `absorb` and not yet recorded.
#[derive(Default)]
struct Resolved {
    selected: BTreeMap<CommitmentId, Commitment>,
    settled: BTreeMap<CommitmentId, Settled>,
    movements: BTreeMap<CommitmentId, Movement>,
    constraints: BTreeMap<ResourceInstanceId, Constraint>,
    event_head: Option<EventId>,
}

impl Accumulation {
    /// An accumulation that only ever answers for `head`.
    ///
    /// The chain may still arrive in batches; what is refused is an event past that head, and
    /// any interpretation taken before the head is reached.
    pub fn recognizing(head: Option<EventId>) -> Self {
        Self {
            recognition: Recognition::Bound(head),
            ..Self::default()
        }
    }

    pub fn absorb<K: Knowledge>(
        &mut self,
        knowledge: &K,
        selection: &[CommitmentId],
        events: &[Event],
    ) -> Result<(), ProjectionError> {
        let resolved = self.resolve(knowledge, selection, events)?;

        self.selected.extend(resolved.selected);
        self.settled.extend(resolved.settled);
        self.movements.extend(resolved.movements);
        self.constraints.extend(resolved.constraints);
        self.event_head = resolved.event_head;

        Ok(())
    }

    pub fn event_head(&self) -> Option<EventId> {
        self.event_head
    }

    fn resolve<K: Knowledge>(
        &self,
        knowledge: &K,
        selection: &[CommitmentId],
        events: &[Event],
    ) -> Result<Resolved, ProjectionError> {
        let mut resolved = Resolved {
            event_head: self.event_head,
            ..Resolved::default()
        };

        for id in selection {
            let commitment = knowledge
                .commitment(*id)
                .ok_or(ProjectionError::UnknownCommitment(*id))?;

            if let Some((movement, constraint)) = resolve_movement(knowledge, &commitment)? {
                resolved.constraints.insert(movement.instance, constraint);
                resolved.movements.insert(*id, movement);
            }

            resolved.selected.insert(*id, commitment);
        }

        for event in events {
            if let Recognition::Bound(recognized) = self.recognition
                && recognized == resolved.event_head
            {
                return Err(ProjectionError::EventBeyondRecognizedHead {
                    event: event.id(),
                    recognized,
                });
            }

            if *event.previous_event() != resolved.event_head {
                return Err(ProjectionError::DisjointEventChain {
                    event: event.id(),
                    absorbed: resolved.event_head,
                    carried: *event.previous_event(),
                });
            }
            resolved.event_head = Some(event.id());

            let commitment_id = *event.commitment_id();

            let commitment = resolved
                .selected
                .get(&commitment_id)
                .or_else(|| self.selected.get(&commitment_id))
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

            if self.settled.contains_key(&commitment_id)
                || resolved.settled.contains_key(&commitment_id)
            {
                return Err(ProjectionError::SettledMoreThanOnce(commitment_id));
            }

            resolved.settled.insert(
                commitment_id,
                Settled {
                    outcome,
                    occurred_at: *event.occurred_at(),
                },
            );
        }

        Ok(resolved)
    }

    /// The dependency closure is only required here, not while folding: a dependency may
    /// legitimately arrive in a later `absorb`, but a commitment whose dependency is
    /// missing cannot be told apart from one whose dependency is merely pending.
    pub fn conditions_at(&self, at: &Date) -> Result<ProjectedConditions, ProjectionError> {
        self.ensure_recognized()?;

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

        Ok(ProjectedConditions::new(
            self.event_head,
            *at,
            conditions,
        ))
    }

    /// The conflicts the selected graph carries under `hypothesis`, empty when none was
    /// found.
    ///
    /// A commitment still open behind a dependency that can never be fulfilled is reported
    /// first, and on its own. Its presence means no completion exists at all, so assuming
    /// every unsettled commitment is realized is not an assumption that can hold.
    pub fn feasibility_under(
        &self,
        hypothesis: Hypothesis,
    ) -> Result<FeasibilityReport, ProjectionError> {
        self.ensure_recognized()?;

        let conflicts = self.conflicts_under(hypothesis)?;

        Ok(FeasibilityReport::new(
            hypothesis,
            self.event_head,
            conflicts,
        ))
    }

    fn conflicts_under(&self, hypothesis: Hypothesis) -> Result<Vec<Conflict>, ProjectionError> {
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

        let within = match hypothesis {
            Hypothesis::FinalState => {
                return Ok(self.out_of_bounds(self.levels_once_every_movement_lands()));
            }
            Hypothesis::OnDueDateNet => Within::Net,
            Hypothesis::OnDueDateInAnyOrder => Within::AnyOrder,
        };

        let unrealizable_punctually = self.punctual_dependency_violations()?;
        if !unrealizable_punctually.is_empty() {
            return Ok(unrealizable_punctually);
        }

        self.breaches_along_the_punctual_sequence(within)
    }

    /// A dependency must settle before its dependent, and the sequence a punctual hypothesis
    /// derives can contradict that: a commitment due before the dependency it waits on is placed
    /// ahead of it.
    ///
    /// Only a commitment still unsettled is judged. Its punctuality is the part that remains
    /// hypothetical; one already settled is a fact, and a fact is not declared impossible however
    /// its dependencies were ordered.
    fn punctual_dependency_violations(&self) -> Result<Vec<Conflict>, ProjectionError> {
        let mut violations = Vec::new();

        for (id, commitment) in &self.selected {
            if self.outcome_of(id) != Outcome::Unsettled {
                continue;
            }

            let dependent_at = *commitment.term().due_date();

            for dependency in commitment.dependencies() {
                let required = self
                    .selected
                    .get(dependency)
                    .ok_or(ProjectionError::UnknownCommitment(*dependency))?;

                let Some(dependency_at) = self.punctual_position(dependency, required) else {
                    continue;
                };

                if !dependency_at.up_to(&dependent_at) {
                    violations.push(Conflict::PunctualDependencyViolation {
                        dependency: *dependency,
                        dependent: *id,
                    });
                }
            }
        }

        Ok(violations)
    }

    /// The instant the punctual hypothesis places a commitment at.
    fn punctual_position(&self, id: &CommitmentId, commitment: &Commitment) -> Option<Date> {
        match self.settled.get(id) {
            Some(settled) if settled.outcome == Outcome::Cancelled => None,
            Some(settled) => Some(settled.occurred_at),
            None => Some(*commitment.term().due_date()),
        }
    }

    fn breaches_along_the_punctual_sequence(
        &self,
        within: Within,
    ) -> Result<Vec<Conflict>, ProjectionError> {
        let mut conflicts = Vec::new();

        for (instance, sequence) in self.punctual_sequence() {
            let Some(constraint) = self.constraints.get(&instance) else {
                continue;
            };

            let mut level = 0.0;

            for (position, simultaneous) in sequence {
                let net = level
                    + simultaneous
                        .iter()
                        .map(|movement| movement.magnitude)
                        .sum::<f64>();

                let judged = match within {
                    Within::Net => vec![net],
                    Within::AnyOrder => {
                        if simultaneous.len() > SIMULTANEOUS_LIMIT {
                            return Err(ProjectionError::TooManySimultaneousMovements {
                                instance,
                                position,
                                count: simultaneous.len(),
                            });
                        }

                        reachable_levels(level, &simultaneous)
                    }
                };

                if let Some(breach) = judged
                    .into_iter()
                    .find(|reachable| !constraint.check(*reachable))
                {
                    conflicts.push(Conflict::OutOfBounds {
                        instance,
                        level: breach,
                    });
                    break;
                }

                level = net;
            }
        }

        Ok(conflicts)
    }

    /// Every movement that still lands, grouped by resource and then by the instant the hypothesis
    /// puts it at, each carrying the precedence it answers to within its own group.
    fn punctual_sequence(&self) -> BTreeMap<ResourceInstanceId, BTreeMap<Date, Vec<Simultaneous>>> {
        let mut grouped: BTreeMap<ResourceInstanceId, BTreeMap<Date, Vec<CommitmentId>>> =
            BTreeMap::new();

        for (id, commitment) in &self.selected {
            let Some(movement) = self.movements.get(id) else {
                continue;
            };

            let Some(position) = self.punctual_position(id, commitment) else {
                continue;
            };

            grouped
                .entry(movement.instance)
                .or_default()
                .entry(position)
                .or_default()
                .push(*id);
        }

        grouped
            .into_iter()
            .map(|(instance, positions)| {
                let sequence = positions
                    .into_iter()
                    .map(|(position, group)| (position, self.with_precedence(&group)))
                    .collect();

                (instance, sequence)
            })
            .collect()
    }

    /// Resolve each movement of one group against the others, recording which of them it waits on.
    fn with_precedence(&self, group: &[CommitmentId]) -> Vec<Simultaneous> {
        group
            .iter()
            .map(|id| {
                let dependencies = self.selected.get(id).map(Commitment::dependencies);

                let requires = group
                    .iter()
                    .enumerate()
                    .filter(|(_, other)| {
                        dependencies.is_some_and(|waiting_on| waiting_on.contains(other))
                    })
                    .fold(0, |mask, (slot, _)| mask | (1 << slot));

                Simultaneous {
                    magnitude: self.movements.get(id).map_or(0.0, |m| m.magnitude),
                    requires,
                }
            })
            .collect()
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

    /// A bound accumulation interprets nothing until the chain it recognizes has been folded.
    ///
    /// This is the half of the boundary `absorb` cannot enforce: an event past the head is
    /// visible the moment it is offered, while a chain that simply stops short looks complete
    /// from the inside. Only the question being asked reveals it.
    fn ensure_recognized(&self) -> Result<(), ProjectionError> {
        if let Recognition::Bound(recognized) = self.recognition
            && recognized != self.event_head
        {
            return Err(ProjectionError::RecognizedChainIncomplete {
                reached: self.event_head,
                recognized,
            });
        }

        Ok(())
    }

    fn outcome_of(&self, commitment: &CommitmentId) -> Outcome {
        self.settled
            .get(commitment)
            .map(|settled| settled.outcome.clone())
            .unwrap_or(Outcome::Unsettled)
    }
}

/// The level a commitment moves, together with the bounds that level answers to.
fn resolve_movement<K: Knowledge>(
    knowledge: &K,
    commitment: &Commitment,
) -> Result<Option<(Movement, Constraint)>, ProjectionError> {
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

        let (effect, magnitude) = effect;

        Ok(Some((
            Movement {
                instance: instance_id,
                magnitude: match effect {
                    Effect::Increase => magnitude,
                    Effect::Decrease => -magnitude,
                },
            },
            constraint.clone(),
        )))
    }

