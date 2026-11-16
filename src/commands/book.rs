use crate::{Context, Error};
use booking::UserRequest;
use chrono::prelude::*;

#[poise::command(slash_command)]
pub async fn book(
    ctx: Context<'_>,
    start_time: String,
    duration: String,
    date: Option<String>,
    description: Option<String>,
) -> Result<(), Error> {
    /// Usage: /book [start-time],
    /// A Google Sheets booking looks like:
    /// | Date | Start time | End time | Duration (hrs) | User name | Project/Task description | Status |
    /// We construct a booking by following these steps:
    /// 1. Parse the start time, duration, end date 
    /// 2. Create a Booking struct with the remaining values
    /// 3. Fetch the Sheet's rows and find conflicts
    /// 4. Write to the Sheet
    let user_req = UserRequest {
        
    };
    Ok(())
}
