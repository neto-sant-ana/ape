//! Whether the selected graph still admits a valid completion, and under which assumption.
//!
//! Feasibility takes no effective time. Violating a deadline is a breach, not an
//! infeasibility — a commitment past its deadline remains realizable, only late — so no
//! judgment about deadlines enters here and the instant being consulted drops out. A verdict
//! changes as knowledge grows, never as the consulted time moves.
//!
//! Nothing about the future is claimed without saying under what assumption, so the
//! hypothesis is an input.
//! `FinalState`- Every unsettled commitment is eventually realized, in no particular order.
//! It asks only whether the accumulated resource levels land within their constraints once
//! every commitment has contributed.
//!
//! `OnDueDate`- Every unsettled commitment is realized exactly on the day it is due, while a
//! settled one lands when it was observed. It asks whether every level along that realization
//! stays within bounds, not only the one it ends at — which is why the two can disagree.
//!
//! What it does not establish is impossibility: `OnDueDate` tests the latest punctual realization,
//! not every realization, and a commitment fulfilled ahead of its deadline can rescue a graph this
//! hypothesis rejects. Its verdict is that the punctual plan does not hold — actionable as replanning,
//! not as impossibility.
//!
//! Findings are reported rather than a verdict named. There is no `Feasible` — an empty list
//! means nothing was found under the hypothesis asked, which for `FinalState` is not the same
//! as the graph being realizable. What is reported instead identifies *what* conflicts, so an
//! application can weigh a counterbalancing commitment instead of only learning that
//! something is wrong.
//!
//! `Unrealizable` - The commitment can never be fulfilled, so no completion can include it.
//! `OutOfBounds` - The resource's level leavs the bounds its contraint declares.

use crate::kernel::entities::{CommitmentId, ResourceInstanceId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hypothesis {
    FinalState,
    OnDueDate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Conflict {
    Unrealizable(CommitmentId),
    OutOfBounds {
        instance: ResourceInstanceId,
        level: f64,
    },
}
