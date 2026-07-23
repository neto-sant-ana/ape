//! Why an assertion was refused admission into canonical history.

use crate::kernel::entities::EventId;

#[derive(Debug, ::thiserror::Error)]
pub enum CanonError {
    #[error("event chain head moved: the event extends {expected:?} but the head is {found:?}")]
    UnexpectedHead {
        expected: Option<EventId>,
        found: Option<EventId>,
    },
}
