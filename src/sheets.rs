extern crate google_sheets4 as sheets4;
use chrono_tz::Tz;
use sheets4::{Result, Error};
use reqwest;
use crate::booking::Booking;
use crate::config::Config;
use crate::auth::Hub;
pub struct SheetsClient {
    http: reqwest::Client,
    auth: Hub,
    config: Config,
}

impl SheetsClient {
   pub fn read_bookings(&self) -> Result<Vec<Booking>> {todo!()}
   pub fn append(&self, b: &Booking, lab_tz: Tz) -> Result<()> {todo!()}
}