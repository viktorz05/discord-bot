pub enum Status {
    Scheduled,
    Released,
    NightBatch,
}

pub struct BookingRequest {
    pub start_time: String,
    pub duration: String,
    pub date: Option<String>,
    pub description: Option<String>
}
pub struct Booking {
    pub date: NaiveDate,
    pub start: NaiveTime,
    pub duration: NaiveTime,
    pub user: String,
    pub description: Option<String>,
    pub Status: Status
}
