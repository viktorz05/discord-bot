#[warn(unused_imports)]
#[macro_use]
extern crate log;
use chrono::prelude::*;
use poise::serenity_prelude as serenity;
pub struct Data {
    pub tz: chrono_tz::Tz,
    pub lock: tokio::sync::Mutex<()>
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
mod commands;

#[tokio::main]
async fn main() {
    colog::init();
    let token = dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    println!("My token: {}", token);
    let now = Local::now();
    println!("Today is: {}", now);
    let now_utc = Utc::now();
    println!("Today is: {} ", now_utc);
}
