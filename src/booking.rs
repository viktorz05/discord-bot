use thiserror::Error;
use chrono::{DateTime, Duration, NaiveDate, Timelike, Utc};
use chrono_tz::Tz;
use crate::parse;
pub enum Status {
    Scheduled,
    Released,
    NightBatch,
}
#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Could not read the time: {found:?}. Try '2 pm', '14:00' or '2:00 PM'")]
    InvalidTimeFormat { found: String },

    #[error("Could not read the date: {found:?}. Try '11-15', or '2026--11-15'")]
    InvalidDateFormat { found: String },

    #[error("That start time is in the past.")]
    StartInPast,

    #[error("Bookings must not exceed {max_hours:?} hours.")]
    TooLong { max_hours: i64 },

    #[error("The time you entered is ambiguous.")]
    AmbiguousLocalTime,

    #[error("The date you entered is out of range.")]
    DateOutOfRange
}

pub struct UserRequest {
    pub date: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub description: Option<String>
}
pub struct Booking {
    pub date: NaiveDate, 
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub user: String,
    pub description: Option<String>,
    pub status: Status
}

impl Booking {
    pub fn from_req(
        req: UserRequest,
        user: String,
        tz: Tz,
        now: DateTime<Utc>
    ) -> Result<Self, RequestError> {
        // Local time
        let now_local = now.with_timezone(&tz);
        let today = now_local.date_naive();

        let start_req = parse::parse_time(&req.start_time)?;
        let end_req = parse::parse_time(&req.end_time)?;

        let mut date = match &req.date {
            Some(d) => parse::parse_date(&d)?,
            None => today, 
        };
        
        let mut start: DateTime<Tz> = parse::to_local(tz, date, start_req)?;
        if req.date.is_none() && start < now_local {
            date = date.succ_opt().ok_or(RequestError::DateOutOfRange)?;
            start = parse::to_local(tz, date, start_req)?;
        }
        if start < now_local {
            return Err(RequestError::StartInPast);
        }
        let end = parse::to_local(tz, date, end_req)?;
        Ok(Self {
            date: date,
            start: start.with_timezone(&Utc),
            end: end.with_timezone(&Utc),
            user: user,
            description: req.description,
            status: Status::Scheduled
        })
    }
}