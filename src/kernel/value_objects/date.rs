//!    Civil date in `YYYY-MM-DD` (ISO 8601 calendar date) form.
//!
//!    The wrapper exists to control the *serialized* form:
//!    `Serialize` always emits the canonical `YYYY-MM-DD` string,
//!    in both human-readable and binary formats, so the bytes
//!    fed into an entity's content-addressed id stay stable and independent of
//!    `time`'s internal encoding.

use serde::{Serialize, Serializer};
use time::{Date as CivilDate, Month};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Date(CivilDate);
impl Date {
    pub fn from_ymd(year: i32, month: u8, day: u8) -> Result<Self, DateError> {
        let month = Month::try_from(month).map_err(|_| DateError::Invalid)?;

        CivilDate::from_calendar_date(year, month, day)
            .map(Self)
            .map_err(|_| DateError::Invalid)
    }

    pub fn parse(value: impl AsRef<str>) -> Result<Self, DateError> {
        let mut parts = value.as_ref().split('-');

        let (Some(y), Some(m), Some(d), None) =
            (parts.next(), parts.next(), parts.next(), parts.next())
        else {
            return Err(DateError::Invalid);
        };

        Self::from_ymd(
            y.parse().map_err(|_| DateError::Invalid)?,
            m.parse().map_err(|_| DateError::Invalid)?,
            d.parse().map_err(|_| DateError::Invalid)?,
        )
    }

    pub fn to_iso(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}",
            self.0.year(),
            u8::from(self.0.month()),
            self.0.day()
        )
    }

    pub fn as_civil(&self) -> CivilDate {
        self.0
    }

    pub fn up_to(&self, to: &Date) -> bool {
        self <= to
    }

    pub fn within(&self, from: Option<&Date>, to: &Date) -> bool {
        match from {
            Some(f) => f < self && self <= to,
            None => self <= to,
        }
    }
}
impl Serialize for Date {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_iso())
    }
}

define_error! {
    pub enum DateError {
        Invalid => "date must be a valid ISO 8601 calendar date (YYYY-MM-DD)",
    }
}

#[cfg(test)]
mod tests {
    use super::Date;

    #[test]
    fn serializes_to_canonical_iso() {
        assert_eq!(Date::from_ymd(2026, 7, 21).unwrap().to_iso(), "2026-07-21");
    }

    #[test]
    fn normalizes_non_padded_input() {
        assert_eq!(Date::parse("2026-7-5").unwrap().to_iso(), "2026-07-05");
    }

    #[test]
    fn within_is_half_open() {
        let (a, b) = (
            Date::from_ymd(2026, 1, 1).unwrap(),
            Date::from_ymd(2026, 2, 1).unwrap(),
        );

        assert!(b.within(Some(&a), &b));
        assert!(!a.within(Some(&a), &b));
    }
}
