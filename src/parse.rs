use serenity::model::event::ReadyEvent;
use thiserror::Error;
use crate::booking::{RequestError};
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

pub fn parse_time(time: &str) -> Result<NaiveTime, RequestError> {
    let s = time.trim().to_uppercase();
    for fmt in ["%H:%M", "%I:%M %p", "%I:%M%p", "%I %p", "%I%p"] { // 23:00, 12:30 AM, 12:30AM, 10 AM, 10AM
        if let Ok(t) = NaiveTime::parse_from_str(&time, fmt) {
            return Ok(t);
        }
    }
    Err(RequestError::InvalidTimeFormat { found: s })
}

pub fn parse_date(date: &str) -> Result<NaiveDate, RequestError> {
    let s = date.trim().replace('/', "-");
    if let Ok(d) = NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
        return Ok(d);
    }
    Err(RequestError::InvalidDateFormat { found: s })
}

pub(crate) fn to_local(tz: Tz, day: NaiveDate, time: NaiveTime) -> Result<DateTime<Tz>, RequestError> {
    tz.from_local_datetime(&day.and_time(time))
        .single()
        .ok_or(RequestError::AmbiguousLocalTime)
}