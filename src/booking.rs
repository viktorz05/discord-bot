use chrono::{DateTime, Duration, Utc};
use chrono_tz::Tz;
pub enum Status {
    Scheduled,
    Released,
    NightBatch,
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
    pub end: 
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
        ) {
        todo!()
    }
}