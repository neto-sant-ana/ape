//! The human-facing name of an entity.
//!
//! - `Name` — a non-blank label.

use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Name(String);
impl Name {
    pub fn new(value: impl Into<String>) -> Result<Self, NameError> {
        let value = value.into();
        let value = value.trim();

        if value.is_empty() {
            return Err(NameError::Blank);
        }

        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

define_error! {
    pub enum NameError {
        Blank => "an entity name must not be blank",
    }
}

#[cfg(test)]
mod tests {
    use super::{Name, NameError};

    #[test]
    fn rejects_blank() {
        assert!(matches!(Name::new(""), Err(NameError::Blank)));
        assert!(matches!(Name::new("   "), Err(NameError::Blank)));
    }

    #[test]
    fn accepts_and_trims() {
        assert_eq!(Name::new("  Supplier  ").unwrap().as_str(), "Supplier");
    }
}
