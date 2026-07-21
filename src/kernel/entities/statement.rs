//! Statements — the building blocks of a commitment's scope.
//!
//! A statement follows the pattern *actors realize an action for recipients*:
//!
//! - `participants` — the roles that act and the roles that receive.
//!
//! - `action` — the action being realized.
//!
//! - `settlement` — the observations that may fulfill or cancel a commitment built
//!   on this statement.

use crate::kernel::entities::ActionId;

use crate::kernel::value_objects::{Participants, Settlement};

define_id!(StatementId);
define_entity! {
    pub struct Statement(StatementId) {
        participants: Participants,
        action: ActionId,
        settlement: Settlement,
    }
}
