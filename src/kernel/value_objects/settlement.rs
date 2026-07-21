//! Value objects describing the events that settle a commitment.
//!
//! - `Observation` — a named event condition (`InvoiceIssued`, `ContractSigned`).
//!
//! - `Settlement` — the observations that `fulfills` a commitment and the ones
//!   that `cancels` it.

use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Observation(String);
impl Observation {
    pub fn new(name: impl Into<String>) -> Result<Self, ObservationError> {
        let name = name.into();
        let name = name.trim();

        if name.is_empty() {
            return Err(ObservationError::EmptyName);
        }

        Ok(Self(name.to_owned()))
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}

define_error! {
    pub enum ObservationError {
        EmptyName => "an observation must have a non-empty name",
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Settlement {
    fulfills: BTreeSet<Observation>,
    cancels: BTreeSet<Observation>,
}
impl Settlement {
    pub fn new(
        fulfills: impl IntoIterator<Item = Observation>,
        cancels: impl IntoIterator<Item = Observation>,
    ) -> Result<Self, SettlementError> {
        let fulfills: BTreeSet<Observation> = fulfills.into_iter().collect();
        let cancels: BTreeSet<Observation> = cancels.into_iter().collect();

        if fulfills.is_empty() {
            return Err(SettlementError::NoFulfills);
        }
        if cancels.is_empty() {
            return Err(SettlementError::NoCancels);
        }

        Ok(Self { fulfills, cancels })
    }

    pub fn can_settle(&self, observation: &Observation) -> bool {
        self.fulfills.contains(observation)
    }

    pub fn can_cancel(&self, observation: &Observation) -> bool {
        self.cancels.contains(observation)
    }

    pub fn fulfills(&self) -> &BTreeSet<Observation> {
        &self.fulfills
    }

    pub fn cancels(&self) -> &BTreeSet<Observation> {
        &self.cancels
    }
}

define_error! {
    pub enum SettlementError {
        NoFulfills => "a settlement must have at least one fulfilling observation",
        NoCancels => "a settlement must have at least one cancelling observation",
    }
}

#[cfg(test)]
mod tests {
    use super::{Observation, ObservationError, Settlement};

    fn obs(name: &str) -> Observation {
        Observation::new(name).unwrap()
    }

    #[test]
    fn observation_requires_non_blank_name() {
        assert!(matches!(
            Observation::new(""),
            Err(ObservationError::EmptyName)
        ));

        assert!(matches!(
            Observation::new("   "),
            Err(ObservationError::EmptyName)
        ));

        assert_eq!(
            Observation::new("  InvoiceIssued  ").unwrap().name(),
            "InvoiceIssued"
        );
    }

    #[test]
    fn settlement_requires_both_sides_non_empty() {
        assert!(Settlement::new([], [obs("Canceled")]).is_err());
        assert!(Settlement::new([obs("InvoiceIssued")], []).is_err());
        assert!(Settlement::new([obs("InvoiceIssued")], [obs("Canceled")]).is_ok());
    }

    #[test]
    fn settlement_membership_is_set_based() {
        let s = Settlement::new(
            [
                obs("InvoiceIssued"),
                obs("InvoiceIssued"),
            obs("ContractSigned"),
            ],
            [obs("Canceled")],
        )
        .unwrap();

        assert_eq!(s.fulfills().len(), 2);
        assert!(s.can_settle(&obs("InvoiceIssued")));
        assert!(!s.can_settle(&obs("Canceled")));
        assert!(s.can_cancel(&obs("Canceled")));
    }
}
