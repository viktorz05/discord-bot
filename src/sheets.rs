use chrono_tz::Tz;
extern crate google_sheets4 as sheets4;
use sheets4::{Result, api::ValueRange};
use crate::booking::Booking;
use crate::config::Config;
use crate::ghub::Hub;
use serde_json::Value;
pub struct SheetsClient {
    pub hub: Hub,
    pub config: Config,
}
// look into this: https://gkit.mreshank.com/blog/google-sheets-rust-axum tomorrow
impl SheetsClient {
    pub async fn load_bookings(&self) -> Result<Vec<Booking>> {
        let rows = self.read_bookings().await?;
        let bookings  = rows
            .iter()
            .enumerate()
            .filter_map(|(i, row)| match Booking::try_from(row.as_slice()) {
                Ok(b) => Some(b),
                Err(e) => {
                    if !row.is_empty() {
                        log::warn!("Skipping row {}: {e}", i + 2)
                    }
                    None
                }
            }).collect();
        Ok(bookings)
    }

    pub async fn read_bookings(&self) -> Result<Vec<Vec<String>>> {
        let (_res, range) = self.hub
            .spreadsheets()
            .values_get(&self.config.sheet_id, &self.config.sheet_range)
            .doit()
            .await?;
        let rows = range
            .values
            .unwrap_or_default()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|col| match col {
                        Value::String(s) => s,
                        other => other.to_string()
                    })
                    .collect()
            })
            .collect();
    Ok(rows)
    }

   pub async fn append(&self, booking: &Booking) -> Result<()> {
        let row = booking.to_row(self.config.lab_tz); 
        let req = ValueRange {
            values: Some(vec![row.into_iter().map(Value::String).collect()]),
            ..Default::default()
        };
        self.hub
            .spreadsheets()
            .values_append(req, &self.config.sheet_id, &self.config.sheet_range)
            .value_input_option("RAW")
            .doit()
            .await?;
        Ok(())
    }
}