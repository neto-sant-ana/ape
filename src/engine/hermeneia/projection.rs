//! `Projection` — the interpreted result: what each selected commitment's condition is,
//! as of the knowledge and the effective time it was derived from.
//!
//! It is a value, not a view onto anything. Nothing it holds can change after it is
//! produced, so it stays attributable to the context that produced it: a later
//! projection is a new result rather than an update to this one.

use std::collections::BTreeMap;

use super::Condition;

use crate::kernel::entities::CommitmentId;

#[derive(Debug, Clone)]
pub struct Projection {
    conditions: BTreeMap<CommitmentId, Condition>,
}
impl Projection {
    pub(super) fn new(conditions: BTreeMap<CommitmentId, Condition>) -> Self {
        Self { conditions }
    }

    pub fn condition(&self, commitment: CommitmentId) -> Option<&Condition> {
        self.conditions.get(&commitment)
    }

    pub fn conditions(&self) -> &BTreeMap<CommitmentId, Condition> {
        &self.conditions
    }
}
