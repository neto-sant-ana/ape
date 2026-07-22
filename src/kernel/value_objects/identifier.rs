//! - `Identifier` — a non-blank string serving as an entity's identity key.

use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Identifier(String);
impl Identifier {
    pub fn new(value: impl Into<String>) -> Result<Self, IdentifierError> {
        let value = value.into();
        let value = value.trim();

        if value.is_empty() {
            return Err(IdentifierError::Blank);
        }

        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

define_error! {
    pub enum IdentifierError {
        Blank => "an identifier must not be blank",
    }
}

#[cfg(test)]
mod tests {
    use super::{Identifier, IdentifierError};

    #[test]
    fn rejects_blank() {
        assert!(matches!(Identifier::new(""), Err(IdentifierError::Blank)));
        assert!(matches!(Identifier::new("   "), Err(IdentifierError::Blank)));
    }

    #[test]
    fn accepts_and_trims() {
        assert_eq!(
            Identifier::new("  Supplier  ").unwrap().as_str(),
            "Supplier"
        );
    }
}
