//! Whether the selected graph still admits a valid completion, and under which assumption.
//!
//! Feasibility takes no effective time. Violating a deadline is a breach, not an
//! infeasibility — a commitment past its deadline remains realizable, only late — so no
//! judgment about deadlines enters here and the instant being consulted drops out. A verdict
//! changes as knowledge grows, never as the consulted time moves.
//!
//! Nothing about the future is claimed without saying under what assumption, so the
//! hypothesis is an input. Each states what it assumes and what it checks:
//!
//! - `FinalState` — every unsettled commitment is realized, in no particular order; checks the
//!   level once every movement has landed.
//! - `OnDueDateNet` — every unsettled commitment is realized on the day it is due, a settled one
//!   where it was observed; checks the level once each date's movements have landed.
//! - `OnDueDateInAnyOrder` — the same assumption; checks every level any arrangement *within* a
//!   date can produce, and refuses a date too crowded to enumerate.
//!
//! Findings are reported rather than a verdict named. There is no `Feasible` — an empty list
//! means nothing was found under the hypothesis asked, never that the graph is realizable. What
//! is reported identifies *what* conflicts:
//!
//! - `Unrealizable` — the commitment can never be fulfilled, so no completion can include it.
//! - `PunctualDependencyViolation` — the hypothesis places a commitment before the dependency it
//!   requires, so the realization it describes cannot happen. The commitment remains realizable;
//!   it is the punctual realization of it that does not.
//! - `OutOfBounds` — the resource's level leaves the bounds its constraint declares.

use crate::kernel::entities::{CommitmentId, EventId, ResourceInstanceId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hypothesis {
    FinalState,
    OnDueDateNet,
    OnDueDateInAnyOrder,
}

/// What was found, and what it was found under.
///
/// A report carries the hypothesis that produced it and the point on the chain it was produced from
#[derive(Debug, Clone, PartialEq)]
pub struct FeasibilityReport {
    hypothesis: Hypothesis,
    event_head: Option<EventId>,
    conflicts: Vec<Conflict>,
}
impl FeasibilityReport {
    pub(super) fn new(
        hypothesis: Hypothesis,
        event_head: Option<EventId>,
        conflicts: Vec<Conflict>,
    ) -> Self {
        Self {
            hypothesis,
            event_head,
            conflicts,
        }
    }

    pub fn hypothesis(&self) -> Hypothesis {
        self.hypothesis
    }

    pub fn event_head(&self) -> Option<EventId> {
        self.event_head
    }

    pub fn conflicts(&self) -> &[Conflict] {
        &self.conflicts
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Conflict {
    Unrealizable(CommitmentId),
    PunctualDependencyViolation {
        dependency: CommitmentId,
        dependent: CommitmentId,
    },
    /// The count that breached, in whatever unit the resource's movements are counted in — which the
    /// engine does not know and does not need to.
    OutOfBounds {
        instance: ResourceInstanceId,
        level: i128,
    },
}
