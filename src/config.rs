use chrono_tz::Tz;

pub struct Config {
    pub priv_key: String,
    pub sheet_id: String,
    pub sheet_range: String,
    pub lab_tz: Tz
}

impl Config {
    pub fn new() -> Self {
        Config {
            priv_key: String::from("credentials.json"),
            sheet_id: String::from("sheet_id"),
            sheet_range: String::from("Ziva Workstation Booking!A13"),
            lab_tz: Tz::America__Vancouver
        }
    }
}