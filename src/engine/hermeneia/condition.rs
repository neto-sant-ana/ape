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
//! A dependency is a requirement, and it answers two questions differently. A dependency
//! settled either way, fulfilled *or* cancelled, stops the waiting; but cancellation is
//! not fulfilment, so only one of the two leaves its dependent realizable.
//!
//! `unfulfillable` is the realizability half. A dependency that can never be fulfilled —
//! cancelled, or unsettled behind one that was,  makes its dependent unrealizable, and the
//! consequence travels the dependency path.
//!
//! Propagation stops at a fact. A commitment that *was* fulfilled is fulfilled whatever
//! its own dependencies did, so nothing downstream of it inherits unfulfillability. That
//! such a commitment has an unfulfillable dependency is not a contradiction to resolve
//! here — it is the record of reality having outrun the plan.

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
pub(super) struct Dependencies {
    pub pending: bool,
    pub unfulfillable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    outcome: Outcome,
    dependencies: Dependencies,
    timeliness: Option<Timeliness>,
}
impl Condition {
    pub(super) fn new(
        outcome: Outcome,
        dependencies: Dependencies,
        term: &Term,
        at: &Date,
    ) -> Self {
        let timeliness = match outcome {
            Outcome::Unsettled => Some(Timeliness::of(term, at)),
            _ => None,
        };

        Self {
            outcome,
            dependencies,
            timeliness,
        }
    }

    pub fn outcome(&self) -> &Outcome {
        &self.outcome
    }

    pub fn has_pending_dependencies(&self) -> bool {
        self.dependencies.pending
    }

    pub fn has_unfulfillable_dependencies(&self) -> bool {
        self.dependencies.unfulfillable
    }

    pub fn timeliness(&self) -> Option<&Timeliness> {
        self.timeliness.as_ref()
    }
}
