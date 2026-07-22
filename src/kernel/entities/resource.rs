//! Resources and their concrete instances.
//!
//! - `Resource` — a kind of resource, classified by its `ResourceKind` (see the
//!   value object for the Discrete/Quantifiable taxonomy and how each is
//!   constrained).
//! 
//! - `ResourceInstance` — a concrete occurrence of a `Resource`, referenced by id.

use crate::kernel::value_objects::{Name, ResourceKind};

define_id!(ResourceId);
define_entity! {
    pub struct Resource(ResourceId) via ResourceInput {
        name: Name,
        kind: ResourceKind,
    }
}

define_id!(ResourceInstanceId);
define_entity! {
    pub struct ResourceInstance(ResourceInstanceId) via ResourceInstanceInput {
        name: Name,
        resource: ResourceId,
    }
}
