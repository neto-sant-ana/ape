//! Base identity shared by every entity in the system.
//!
//! `id` is always the hash of the core fields.
//!
//! `name` must be non-blank.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Identity<Id> {
    id: Id,
    name: String,
}
impl<Id: Copy> Identity<Id> {
    pub fn new(id: Id, name: String) -> Result<Self, IdentityError> {
        if name.trim().is_empty() {
            return Err(IdentityError::EmptyName);
        }

        Ok(Self { id, name })
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

define_error! {
    pub enum IdentityError {
        EmptyName => "an entity must have a non-blank name",
        Serialization => "failed to serialize a field while computing the entity id",
    }
}

#[cfg(test)]
mod tests {
    use super::{Identity, IdentityError};

    #[test]
    fn rejects_blank_name() {
        assert!(matches!(
            Identity::new(0u8, "".into()),
            Err(IdentityError::EmptyName)
        ));

        assert!(matches!(
            Identity::new(0u8, "   ".into()),
            Err(IdentityError::EmptyName)
        ));

        assert!(Identity::new(0u8, "Role".into()).is_ok());
    }
}
