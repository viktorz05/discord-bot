extern crate google_sheets4 as sheets4;
use chrono_tz::Tz;
use sheets4::{Result, hyper, api::ValueRange};
use reqwest;
use crate::booking::Booking;
use crate::config::Config;
use crate::ghub::Hub;
pub struct SheetsClient {
    pub http: reqwest::Client,
    pub hub: Hub,
    pub config: Config,
}
// look into this: https://gkit.mreshank.com/blog/google-sheets-rust-axum tomorrow
impl SheetsClient {
    pub async fn read(&self) -> Result<(hyper::Response<hyper::Body>, ValueRange)>  {
        self.hub
            .spreadsheets()
            .values_get(&self.config.sheet_id, "Sheet1!A1:B2".into())
            .doit()
            .await?
            
    }
   pub async fn read_bookings(&self) -> Result<Vec<Booking>> {todo!()}
   pub async fn append(&self, b: &Booking, lab_tz: Tz) -> Result<()> {todo!()}
}