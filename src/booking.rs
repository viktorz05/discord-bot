use thiserror::Error;
use chrono::{DateTime, Duration, Utc};
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
    AmbiguousLocalTime
}

pub struct UserRequest {
    pub date: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub description: Option<String>
}
pub struct Booking {
    pub date: DateTime<Utc>,
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
        let now_local = now.with_timezone(&tz);
        let today = now_local.date_naive();
        let start_time = parse::parse_time(&req.start_time)?;
        let end_time = req.end_time;
        let mut date = match &req.date {
            Some(d) => parse::parse_date(&d)?,
            None => today, 
        };
        let mut start = parse::to_local(tz, date, start_time)?;
        if start < now_local {
            return Err(RequestError::StartInPast);
        }
        Ok(Self {
            date: date,
            start: start.with_timezone(&Utc),
            end: end,
            user: user,
            description: req.description,
            status: match start {

            }
        })
    }
}