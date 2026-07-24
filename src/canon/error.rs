//! Why an assertion was refused admission into canonical history.

use crate::kernel::axiom::AxiomError;

use crate::kernel::entities::EventId;

use crate::kernel::value_objects::Date;

#[derive(Debug, ::thiserror::Error)]
pub enum CanonError {
    #[error(transparent)]
    Axiom(#[from] AxiomError),

    #[error("event chain head moved: the event extends {expected:?} but the head is {found:?}")]
    UnexpectedHead {
        expected: Option<EventId>,
        found: Option<EventId>,
    },

    #[error("recorded_at {recorded_at:?} precedes the assertion's factual instant {fact:?}")]
    RecordedBeforeFact { fact: Date, recorded_at: Date },
}
