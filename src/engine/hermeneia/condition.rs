//! The operational conditions derived for a single commitment.
//!
//! Independent axes rather than one state: a commitment may be unsettled, waiting on a
//! dependency and breached at once, and collapsing them into a single enum would force an
//! order of precedence the domain does not have.
//!
//! `Outcome` names the settlement axis; the kernel already spends `Settlement` on the
//! value object declaring which observations settle a statement.
//!
//! Timeliness is absent once an outcome is known: a deadline stops applying pressure the
//! moment the commitment is settled, so reporting it would answer a question that no
//! longer exists.
//!
//! Dependencies are reported as `has_pending_dependencies` and nothing more, deliberately.
//! What can be computed from a commitment's dependencies alone is the narrow, negative
//! fact that none of them is still pending — a dependency settled either way, fulfilled
//! *or* cancelled, stops the waiting. Naming that as a quality of the commitment
//! (`Available`, `Ready`) would claim something wider than the computation supports,
//! because a commitment can be waiting on nothing and still be impossible: a consumer
//! reading `Available` would schedule work this layer already knows cannot be done.
//! Whether a terminal dependency leaves its dependent realizable is feasibility's
//! question — a different dimension, not a harder computation.

use crate::kernel::value_objects::{Date, Term};

#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    Unsettled,
    Fulfilled,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Timeliness {
    WithinDeadline,
    Breached,
}
impl Timeliness {
    pub(super) fn of(term: &Term, at: &Date) -> Self {
        if at.up_to(term.due_date()) {
            Self::WithinDeadline
        } else {
            Self::Breached
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    outcome: Outcome,
    pending_dependencies: bool,
    timeliness: Option<Timeliness>,
}
impl Condition {
    pub(super) fn new(
        outcome: Outcome,
        pending_dependencies: bool,
        term: &Term,
        at: &Date,
    ) -> Self {
        let timeliness = match outcome {
            Outcome::Unsettled => Some(Timeliness::of(term, at)),
            _ => None,
        };

        Self {
            outcome,
            pending_dependencies,
            timeliness,
        }
    }

    pub fn outcome(&self) -> &Outcome {
        &self.outcome
    }

    pub fn has_pending_dependencies(&self) -> bool {
        self.pending_dependencies
    }

    pub fn timeliness(&self) -> Option<&Timeliness> {
        self.timeliness.as_ref()
    }
}
