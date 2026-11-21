use crate::{Context, Error};
use chrono::Utc;
use chrono_tz::Tz;
use discord_bot::booking::{Booking, RequestError, UserRequest};
use poise::serenity_prelude as serenity;

/// book command
/// Usage: /book [start-time],
#[poise::command(slash_command)]
pub async fn book(
    ctx: Context<'_>,
    #[description = "Start time. Format: "] start: String,
    #[description = "Start time. Format: "] end: String,
    #[description = "Start date. Formats:"] date: Option<String>,
    #[description = "Short description"] description: Option<String>,
) -> Result<(), Error> {
        // A Google Sheets booking looks like:
        // | Date | Start time | End time | Duration (hrs) | User name | Project/Task description | Status |
    // We construct a booking by following these steps:
    // 1. Parse the start time, duration, end date 
    // 2. Create a Booking struct with the remaining values
    // 3. Fetch the Sheet's rows and find conflicts
    // 4. Write to the Sheet
    let user_req = UserRequest {
       date: date,
       start_time: start,
       end_time: end,
       description: description , 
    };
    let u = ctx.author().name.clone();
    let booking = match Booking::from_req(user_req, &u, Tz::America__Vancouver, Utc::now()) {
        Ok(b) => b,
        Err(e) => {
            ctx.say(format!("Error: {e:?}")).await?;
            return Ok(())
        }
    };
    ctx.defer().await?;
    let _guard = ctx.data().lock.lock().await;
    ctx.data().sheets.append(&booking).await?;
    ctx.say(format!(
    "Booked: <t:{}:f> → <t:{}:f> ({:.1}h)",
    booking.start.timestamp(), booking.end.timestamp(), booking.hours()
    )).await?;
    Ok(())
}
