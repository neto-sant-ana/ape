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
}

#[derive(Debug, Clone, PartialEq)]
pub enum Conflict {
    Unrealizable(CommitmentId),
    OutOfBounds {
        instance: ResourceInstanceId,
        level: f64,
    },
}
