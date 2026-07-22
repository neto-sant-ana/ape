//! Admission for the resource family (see `entities/resource.rs`).

use super::{Axiom, AxiomError, Knowledge};

use crate::kernel::entities::{Resource, ResourceInput, ResourceInstance, ResourceInstanceInput};

impl<'k, K: Knowledge> Axiom<'k, K> {
    pub fn admit_resource(&self, input: ResourceInput) -> Result<Resource, AxiomError> {
        Ok(Resource::create(input)?)
    }

    pub fn admit_resource_instance(
        &self,
        input: ResourceInstanceInput,
    ) -> Result<ResourceInstance, AxiomError> {
        if self.knowledge.resource(input.resource).is_none() {
            return Err(AxiomError::UnknownResource(input.resource));
        }

        Ok(ResourceInstance::create(input)?)
    }
}
