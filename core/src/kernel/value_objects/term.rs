//! `Term` — the temporal span of a commitment: when it was `committed_at`
//! (the factual moment the commitment was made) and when it is `due`.

use serde::Serialize;

use crate::kernel::value_objects::Date;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Term {
    committed_at: Date,
    due_date: Date,
}
impl Term {
    pub fn new(committed_at: Date, due_date: Date) -> Result<Self, TermError> {
        if !committed_at.up_to(&due_date) {
            return Err(TermError::CommittedAfterDue);
        }

        Ok(Self {
            committed_at,
            due_date,
        })
    }

    pub fn committed_at(&self) -> &Date {
        &self.committed_at
    }

    pub fn due_date(&self) -> &Date {
        &self.due_date
    }
}

define_error! {
    pub enum TermError {
        CommittedAfterDue => "a commitment cannot be committed after its due date",
    }
}

#[cfg(test)]
mod tests {
    use super::{Term, TermError};
    use crate::kernel::value_objects::Date;

    fn date(y: i32, m: u8, d: u8) -> Date {
        Date::from_ymd(y, m, d).unwrap()
    }

    #[test]
    fn accepts_committed_before_or_on_due() {
        assert!(Term::new(date(2026, 1, 1), date(2026, 12, 31)).is_ok());
        assert!(Term::new(date(2026, 6, 30), date(2026, 6, 30)).is_ok());
    }

    #[test]
    fn rejects_committed_after_due() {
        assert!(matches!(
            Term::new(date(2026, 12, 31), date(2026, 1, 1)),
            Err(TermError::CommittedAfterDue)
        ));
    }
}
