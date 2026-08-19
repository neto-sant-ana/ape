//! Resource levels, folded by the application from what a projection reports.
//!
//! The engine holds no level. It reports conditions and it reports feasibility, and a
//! level is neither: it is a sum over the movements of commitments that meet some
//! criterion, and *which* criterion is the question being asked. What has landed, what
//! will have landed if nothing slips, what is at stake before a deadline — those are
//! different numbers over the same knowledge, and an engine offering one `level()` would
//! be choosing between them silently.
//!
//! So the criterion is the application's and lives here. The arithmetic is not, and does
//! not: how much a single commitment moves an instance comes from `movement_of`, because a
//! second copy of it here would be a second place for `Increase` to mean something.
//!
//! Earned by: 00-reconstruction (Confirmed), 06-exploration (Confirmed)

use ape::engine::hermeneia::{Outcome, ProjectedConditions, movement_of};
use ape::kernel::axiom::Knowledge;
use ape::kernel::entities::ResourceInstanceId;

use crate::error::LevelError;

/// The level an instance holds counting only commitments a projection reports as fulfilled.
///
/// What has landed: the criterion a reader uses to ask where an account actually stands.
pub fn settled<K: Knowledge>(
    knowledge: &K,
    conditions: &ProjectedConditions,
    instance: ResourceInstanceId,
) -> Result<f64, LevelError> {
    folded(knowledge, conditions, instance, |outcome| {
        outcome == &Outcome::Fulfilled
    })
}

/// The level an instance would hold once every movement the projection names has landed.
///
/// What will have landed if nothing slips: everything a world selects counts, whatever its
/// outcome, except a commitment already cancelled — one of those moves nothing and never will.
///
/// It is deliberately the criterion `Hypothesis::FinalState` weighs against the resource's
/// bounds, and that is not a coincidence to be tidied away. An objective that ranked worlds by a
/// level the engine never checked would prefer a candidate the engine refuses, and would be
/// comparing its own arithmetic rather than the world.
pub fn intended<K: Knowledge>(
    knowledge: &K,
    conditions: &ProjectedConditions,
    instance: ResourceInstanceId,
) -> Result<f64, LevelError> {
    folded(knowledge, conditions, instance, |outcome| {
        outcome != &Outcome::Cancelled
    })
}

/// Sum the movements on one instance over the commitments `counts` admits.
///
/// Every instance begins at zero, because nothing in the ontology gives one an opening
/// balance: a level exists only as the sum of what moved it.
///
/// A commitment the projection names but knowledge cannot resolve is an error rather than
/// a zero. Summing over a world one cannot fully read produces a number that looks like an
/// answer, and this number is about to be compared across a process boundary.
fn folded<K: Knowledge>(
    knowledge: &K,
    conditions: &ProjectedConditions,
    instance: ResourceInstanceId,
    counts: impl Fn(&Outcome) -> bool,
) -> Result<f64, LevelError> {
    let mut level = 0.0;

    for (id, condition) in conditions.conditions() {
        if !counts(condition.outcome()) {
            continue;
        }

        let commitment = knowledge
            .commitment(*id)
            .ok_or(LevelError::UnknownCommitment(*id))?;

        if let Some(movement) = movement_of(knowledge, &commitment)?
            && movement.instance() == instance
        {
            level += movement.magnitude();
        }
    }

    Ok(level)
}
