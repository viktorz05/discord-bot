use crate::{Context, Error};
use chrono::prelude::*;
use dateparser::{parse, parse_with};

#[poise::command(slash_command)]
pub async fn book(
    ctx: Context<'_>,
    start_time: String,
    duration: String,
    date: Option<String>,
    description: Option<String>,
) -> Result<(), Error> {
    // parse start_time and duration
    let local_van = Local::now().time().trunc_subsecs(0);
    let start_dt = parse_with(&start_time, &Local, local_van);

    // if let Some(d) = date {
    //
    // }
    // sheet stuff
    // Context stuff

    Ok(())
}
