use crate::booking::{RequestError};
use chrono_tz::Tz;
use chrono::{
    DateTime,
    NaiveDate,
    NaiveTime,
    TimeZone,
};

pub(crate) fn to_local(tz: Tz, day: NaiveDate, time: NaiveTime) -> Result<DateTime<Tz>, RequestError> {
    tz.from_local_datetime(&day.and_time(time))
        .single()
        .ok_or(RequestError::AmbiguousLocalTime)
}