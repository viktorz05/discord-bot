use thiserror::Error;
use chrono::{DateTime, Duration, NaiveDate, Timelike, Utc};
use chrono_tz::Tz;
use dateparser;
use crate::parse;
use log::{debug, info};

#[derive(Debug, PartialEq)]
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

/// night: anytime after 7 pm Local Vancouver TZ
/// morning: anytime before 10 am Local Vancouver TZ
pub fn is_night_batch(tz: Tz, start: DateTime<Utc>, end: DateTime<Utc>) -> bool {
    let local_start = parse::to_local(tz, start.date_naive(), start.time()).unwrap();
    let local_end = parse::to_local(tz, end.date_naive(), end.time()).unwrap();
    return local_start.hour() > 20 && local_end.hour() < 10
}

/// something
impl Booking {
    pub fn from_req(
        req: UserRequest,
        user: &String,
        tz: Tz,
        now: DateTime<Utc>
    ) -> Result<Self, RequestError> {
        // Local time
        let now_local = now.with_timezone(&tz);
        let today = now_local.date_naive();
        debug!("Now local: {}", now_local);
        // parse start and end
        let parsed_start = dateparser::parse_with_timezone(&req.start_time, &Utc)
        .map_err(|_| RequestError::InvalidTimeFormat { found: req.start_time.clone() })?;
        debug!{"Parsed start time: {}", parsed_start};
        let parsed_end = dateparser::parse_with_timezone(&req.end_time, &Utc)
        .map_err(|_| RequestError::InvalidTimeFormat { found: req.end_time.clone() })?;
        debug!{"Parsed end time: {}", parsed_end};

        let total_hours = (parsed_end - parsed_start).num_hours();
        if total_hours > 4 {
            return Err(RequestError::TooLong { max_hours: 4 })
        }
        let status = match is_night_batch(tz, parsed_start, parsed_end) {
            true => Status::NightBatch,
            false => Status::Scheduled
        };
        // get date
        let mut date = match &req.date {
            Some(d) => dateparser::parse_with_timezone(d, &Utc)
            .map_err(|_| RequestError::InvalidDateFormat { found: d.to_string()})?
            .date_naive(),
            None => today, 
        };
        debug!{"Date supplied: {}", date};
        
        let mut local_start_dt: DateTime<Tz> = parse::to_local(tz, date, parsed_start.time())?;
        if req.date.is_none() && local_start_dt < now_local {
            date = date.succ_opt().ok_or(RequestError::DateOutOfRange)?;
            local_start_dt = parse::to_local(tz, date, parsed_start.time())?;
        }
        
        debug!{"Local start datetime supplied: {}", local_start_dt};
        if local_start_dt < now_local {
            return Err(RequestError::StartInPast);
        }
        let end_date = if parsed_end.time() <= parsed_start.time() {
            date.succ_opt().ok_or(RequestError::DateOutOfRange)?
        } else {
            date
        };
        let end2 = parse::to_local(tz, end_date, parsed_end.time())?;
        debug!{"End date supplied: {}", end2};
        info!("Yay we made it to the end");
        Ok(Self {
            date: date,
            start: local_start_dt.with_timezone(&Utc),
            end: end2.with_timezone(&Utc),
            user: user.to_string(),
            description: req.description,
            status: status
        })
    }
    pub fn duration(&self) -> Duration {
        self.end - self.start
    }
    /// Returns number of hours (fractional) between start and end
    pub fn hours(&self) -> f64 {
        self.duration().num_minutes() as f64 / 60.0
    }
    pub fn overlaps(&self, other: &Booking) -> bool {
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::{assert_eq, println};
    use std::error::Error;
    use chrono::prelude::*;
    use dateparser::parse_with_timezone;


    #[test]
    pub fn parse_date() {
        let start: &str = "Nov 12 at 5:40 PM";
        let parsed = parse_with_timezone(start, &Utc).unwrap();
        assert_eq!(parsed, Utc.ymd(2026, 11, 12).and_hms(17, 40, 0));
    }

    #[test]
    pub fn parse_equest() -> Result<(), Box<dyn Error>> {
        let req = UserRequest {
            date: None,
            start_time: "4:00pm".to_string(),
            end_time: "8:00pm".to_string(),
            description: Some("Proj 1".to_string()),
        };
        let today8pm = Utc::now()
                                        .date_naive()
                                        .and_hms(20, 0, 0);
        let parsed_end = parse_with_timezone(&req.end_time, &Utc)?.naive_utc();
        assert_eq!(parsed_end, today8pm);
        Ok(())
    }    
    #[test]
    pub fn parse_request_overnight() -> Result<(), Box<dyn Error>> {
        let today = Utc::now().date_naive();
        let tmr = today.succ_opt().ok_or(RequestError::DateOutOfRange)?;
        let req = UserRequest {
            date: None,
            start_time: "23:00".to_string(),
            end_time: "8:00 am" .to_string(),
            description: Some("Proj 1".to_string()),
        };
        let parsed_start = parse_with_timezone(&req.start_time, &Utc)?;
        debug!("start time: {}", parsed_start);
        let parsed_end = parse_with_timezone(&req.end_time, &Utc)?;
        debug!("end time: {}", parsed_end);
        let end_date = if parsed_end.time() <= parsed_start.time() {
            today + Duration::days(1)
        } else {
            today
        };
        let end = end_date.and_time(parsed_end.time());
        info!("end booking: {}", end);
        assert_eq!(end, tmr.and_hms(8, 0, 0));
        Ok(())
        }
    #[test]
    pub fn booking_from_request() -> Result<(), Box<dyn Error>> {
        let van = Tz::America__Vancouver;
        // 12:00Z on Nov 15 = 04:00 PST on Nov 15
        let now = Utc.with_ymd_and_hms(2026, 11, 15, 12, 0, 0).unwrap();
        let user = String::from("bek");

        let req = UserRequest {
            date: None,
            start_time: "20:00".into(),
            end_time: "8:00 AM".into(),
            description: Some("Test".into()),
        };
        let book = Booking::from_req(req, &user, van, now)?;

        assert_eq!(book.status, Status::Scheduled);
        assert_eq!(book.date, NaiveDate::from_ymd_opt(2026, 11, 15).unwrap());
        // 20:00 PST Nov 15 = 04:00Z Nov 16
        assert_eq!(book.start, Utc.with_ymd_and_hms(2026, 11, 16, 4, 0, 0).unwrap());
        // 08:00 PST Nov 16 = 16:00Z Nov 16
        assert_eq!(book.end, Utc.with_ymd_and_hms(2026, 11, 16, 16, 0, 0).unwrap());
        assert_eq!(book.user, user);
        assert_eq!(book.description, Some("Test".to_string()));
        Ok(()) 
    }
}