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
//! - `OutOfBounds` — the resource's level leaves the bounds its constraint declares.

use crate::kernel::entities::{CommitmentId, ResourceInstanceId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hypothesis {
    FinalState,
    OnDueDateNet,
    OnDueDateInAnyOrder,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Conflict {
    Unrealizable(CommitmentId),
    OutOfBounds {
        instance: ResourceInstanceId,
        level: f64,
    },
}
