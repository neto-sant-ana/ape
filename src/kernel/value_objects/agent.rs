//! Value objects describing agents, the roles they may take, and how they are
//! assigned to a statement.
//!
//! - `AgentKind` — whether an agent is a company or an individual.
//! 
//! - `Eligibility` — the roles an agent is allowed to assume.
//! 
//! - `Participants` — the roles required to execute (`actors`) and to benefit from
//!   (`recipients`) a statement.
//! 
//! - `Assignment` — the agents taking those roles: one `accountable` plus the
//!   `executors` and `beneficiaries`.

use serde::Serialize;
use std::collections::BTreeSet;

use crate::kernel::entities::{AgentId, RoleId};

define_value_object! {
    pub enum AgentKind {
        Company,
        Individual,
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Eligibility(BTreeSet<RoleId>);
impl Eligibility {
    pub fn new(roles: impl IntoIterator<Item = RoleId>) -> Result<Self, EligibilityError> {
        let roles: BTreeSet<RoleId> = roles.into_iter().collect();

        if roles.is_empty() {
            return Err(EligibilityError::Empty);
        }

        Ok(Self(roles))
    }

    pub fn can_assume_role(&self, role_id: &RoleId) -> bool {
        self.0.contains(role_id)
    }

    pub fn can_assume_any<'a>(&self, roles: impl IntoIterator<Item = &'a RoleId>) -> bool {
        roles.into_iter().any(|r| self.can_assume_role(r))
    }

    pub fn roles(&self) -> &BTreeSet<RoleId> {
        &self.0
    }
}

define_error! {
    pub enum EligibilityError {
        Empty => "an eligibility must grant at least one role",
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Participants {
    actors: BTreeSet<RoleId>,
    recipients: BTreeSet<RoleId>,
}
impl Participants {
    pub fn new(
        actors: impl IntoIterator<Item = RoleId>,
        recipients: impl IntoIterator<Item = RoleId>,
    ) -> Result<Self, ParticipantsError> {
        let actors: BTreeSet<RoleId> = actors.into_iter().collect();
        let recipients: BTreeSet<RoleId> = recipients.into_iter().collect();

        if actors.is_empty() {
            return Err(ParticipantsError::NoActors);
        }
        if recipients.is_empty() {
            return Err(ParticipantsError::NoRecipients);
        }

        Ok(Self { actors, recipients })
    }

    pub fn actors(&self) -> &BTreeSet<RoleId> {
        &self.actors
    }

    pub fn recipients(&self) -> &BTreeSet<RoleId> {
        &self.recipients
    }
}

define_error! {
    pub enum ParticipantsError {
        NoActors => "participants must have at least one actor",
        NoRecipients => "participants must have at least one recipient",
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Assignment {
    accountable: AgentId,
    executors: BTreeSet<AgentId>,
    beneficiaries: BTreeSet<AgentId>,
}
impl Assignment {
    pub fn new(
        accountable: AgentId,
        executors: impl IntoIterator<Item = AgentId>,
        beneficiaries: impl IntoIterator<Item = AgentId>,
    ) -> Result<Self, AssignmentError> {
        let executors: BTreeSet<AgentId> = executors.into_iter().collect();
        let beneficiaries: BTreeSet<AgentId> = beneficiaries.into_iter().collect();

        if executors.is_empty() {
            return Err(AssignmentError::NoExecutors);
        }
        if beneficiaries.is_empty() {
            return Err(AssignmentError::NoBeneficiaries);
        }

        Ok(Self {
            accountable,
            executors,
            beneficiaries,
        })
    }

    pub fn accountable(&self) -> AgentId {
        self.accountable
    }

    pub fn executors(&self) -> &BTreeSet<AgentId> {
        &self.executors
    }

    pub fn beneficiaries(&self) -> &BTreeSet<AgentId> {
        &self.beneficiaries
    }
}

define_error! {
    pub enum AssignmentError {
        NoExecutors => "an assignment must have at least one executor",
        NoBeneficiaries => "an assignment must have at least one beneficiary",
    }
}

#[cfg(test)]
mod tests {
    use super::{Assignment, Eligibility, Participants};
    use crate::kernel::entities::{AgentId, RoleId};

    fn role(byte: u8) -> RoleId {
        RoleId::from([byte; 32])
    }

    fn agent(byte: u8) -> AgentId {
        AgentId::from([byte; 32])
    }

    #[test]
    fn eligibility_rejects_empty() {
        assert!(Eligibility::new([]).is_err());
    }

    #[test]
    fn eligibility_checks_membership() {
        let e = Eligibility::new([role(1), role(2)]).unwrap();

        assert!(e.can_assume_role(&role(1)));
        assert!(!e.can_assume_role(&role(9)));
        assert!(e.can_assume_any([&role(9), &role(2)]));
    }

    #[test]
    fn participants_require_both_sides_non_empty() {
        assert!(Participants::new([], [role(1)]).is_err());
        assert!(Participants::new([role(1)], []).is_err());
        assert!(Participants::new([role(1)], [role(2)]).is_ok());
    }

    #[test]
    fn set_semantics_are_order_and_duplicate_independent() {
        let a = Participants::new([role(1), role(2), role(1)], [role(3)]).unwrap();
        let b = Participants::new([role(2), role(1)], [role(3)]).unwrap();

        assert_eq!(a, b);
        assert_eq!(a.actors().len(), 2);
    }

    #[test]
    fn assignment_requires_executors_and_beneficiaries() {
        assert!(Assignment::new(agent(1), [], [agent(2)]).is_err());
        assert!(Assignment::new(agent(1), [agent(2)], []).is_err());
        assert!(Assignment::new(agent(1), [agent(2)], [agent(3)]).is_ok());
    }
}
