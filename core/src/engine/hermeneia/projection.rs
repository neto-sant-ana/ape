//! `ProjectedConditions` — what each selected commitment's condition is, and the context that made
//! it so.
//!
//! Interpreting knowledge yields two kind of answer, each asked for separately: this one covers the
//! conditions of individual commitments as of an effective time, and [`super::FeasibilityReport`]
//! covers whether the graph still admits a completion.
//!
//! Both are values rather than views onto anything, and both carry the coordinates that produced
//! them. What varies between the two is exactly which coordinates apply: conditions answer to an
//! effective time, feasibility does not.

use std::collections::BTreeMap;

use super::Condition;

use crate::kernel::entities::{CommitmentId, EventId};

use crate::kernel::value_objects::Date;

#[derive(Debug, Clone)]
pub struct ProjectedConditions {
    event_head: Option<EventId>,
    effective_at: Date,
    conditions: BTreeMap<CommitmentId, Condition>,
}
impl ProjectedConditions {
    pub(super) fn new(
        event_head: Option<EventId>,
        effective_at: Date,
        conditions: BTreeMap<CommitmentId, Condition>,
    ) -> Self {
        Self {
            event_head,
            effective_at,
            conditions,
        }
    }

    pub fn event_head(&self) -> Option<EventId> {
        self.event_head
    }

    pub fn effective_at(&self) -> &Date {
        &self.effective_at
    }

    pub fn condition(&self, commitment: CommitmentId) -> Option<&Condition> {
        self.conditions.get(&commitment)
    }

    pub fn conditions(&self) -> &BTreeMap<CommitmentId, Condition> {
        &self.conditions
    }
}
