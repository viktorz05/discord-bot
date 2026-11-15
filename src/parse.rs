use thiserror::Error;
use crate::booking::{UserRequest, Booking};
use chrono_tz::Tz;
use chrono::{
    DateTime,
    Duration,
    NaiveDate,
    NaiveTime,
    TimeZone,
    Utc,
    Datelike
};
#[derive(Error, Debug)]
enum RequestError {
    #[error("Duration must not exceed 24 hours. Entered {found:?}")]
    DurationTooLong { found: String },
    #[error("Could not read duration {found:?}. \n Accepted formats: 2h, 1.5hr, 90m")]
    InvalidDurationFormat { found: String },
    #[error("Could not read the time: {found:?}. Try '2 pm', '14:00' or '2:00 PM'")]
    InvalidTimeFormat { found: String },
    #[error("That start time is in the past.")]
    PastDate,
}

pub fn parse_time(time: &str) -> Result<NaiveTime, RequestError> {
    Ok(())
}

pub fn parse_date(date: &str) -> Result<NaiveDate, RequestError> {
    todo!()
}

pub fn parse_duration(duration: &str) -> Result<Duration, RequestError> {
    todo!()
}

pub fn build_booking(req: UserRequest) -> Result<Booking, RequestError> {
    todo!()
}