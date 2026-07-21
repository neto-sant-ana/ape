//! Base identity shared by every entity in the system.
//!
//! `id` is always the hash of the core fields.
//! 
//! `name` must be non-blank.
//! 
//! `description` is free prose (not part of the id).

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Identity<Id> {
    id: Id,
    name: String,
    description: String,
}
impl<Id: Copy> Identity<Id> {
    pub fn new(id: Id, name: String, description: String) -> Result<Self, IdentityError> {
        if name.trim().is_empty() {
            return Err(IdentityError::EmptyName);
        }

        Ok(Self {
            id,
            name,
            description,
        })
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description
    }
}

define_error! {
    pub enum IdentityError {
        EmptyName => "an entity must have a non-blank name",
    }
}

#[cfg(test)]
mod tests {
    use super::{Identity, IdentityError};

    #[test]
    fn rejects_blank_name() {
        assert!(matches!(
            Identity::new(0u8, "".into(), "d".into()),
            Err(IdentityError::EmptyName)
        ));

        assert!(matches!(
            Identity::new(0u8, "   ".into(), "d".into()),
            Err(IdentityError::EmptyName)
        ));

        assert!(Identity::new(0u8, "Role".into(), "d".into()).is_ok());
    }
}
