//! Actions performed over resources.
//!
//! - `Action` — what is done (`kind`) to a resource (`resource`). A discrete
//!   action targets a discrete resource; a quantifiable one targets a quantifiable
//!   resource and declares its `Effect` on the level.

use crate::kernel::entities::ResourceId;

use crate::kernel::value_objects::ActionKind;

define_id!(ActionId);
define_entity! {
    pub struct Action(ActionId) {
        kind: ActionKind,
        resource: ResourceId,
    }
}
