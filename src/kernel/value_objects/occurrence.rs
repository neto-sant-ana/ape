//! When a recorded fact happened and when it entered the system.
//!
//! - `Occurrence` — pairs the instant a fact `occurred_at` with the instant it was
//!   `recorded_at`. A fact cannot be recorded before it occurred, so the pair is
//!   valid only when `occurred_at <= recorded_at`.

use serde::Serialize;

use super::Date;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct Occurrence {
    occurred_at: Date,
    recorded_at: Date,
}
impl Occurrence {
    pub fn new(occurred_at: Date, recorded_at: Date) -> Result<Self, OccurrenceError> {
        if recorded_at < occurred_at {
            return Err(OccurrenceError::RecordedBeforeOccurrence);
        }

        Ok(Self {
            occurred_at,
            recorded_at,
        })
    }

    pub fn occurred_at(&self) -> Date {
        self.occurred_at
    }

    pub fn recorded_at(&self) -> Date {
        self.recorded_at
    }
}

define_error! {
    pub enum OccurrenceError {
        RecordedBeforeOccurrence => "a fact cannot be recorded before it occurred",
    }
}

#[cfg(test)]
mod tests {
    use super::{Occurrence, OccurrenceError};
    use crate::kernel::value_objects::Date;

    fn date(year: i32, month: u8, day: u8) -> Date {
        Date::from_ymd(year, month, day).unwrap()
    }

    #[test]
    fn accepts_recorded_after_occurred() {
        let o = Occurrence::new(date(2026, 7, 21), date(2026, 7, 22)).unwrap();

        assert_eq!(o.occurred_at(), date(2026, 7, 21));
        assert_eq!(o.recorded_at(), date(2026, 7, 22));
    }

    #[test]
    fn accepts_same_instant() {
        assert!(Occurrence::new(date(2026, 7, 21), date(2026, 7, 21)).is_ok());
    }

    #[test]
    fn rejects_recorded_before_occurred() {
        assert!(matches!(
            Occurrence::new(date(2026, 7, 22), date(2026, 7, 21)),
            Err(OccurrenceError::RecordedBeforeOccurrence)
        ));
    }
}
