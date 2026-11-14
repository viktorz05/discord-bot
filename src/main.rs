use chrono::prelude::*;
use poise::serenity_prelude as serenity;
mod commands;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {}

#[tokio::main]
async fn main() {
    let token = dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    println!("My token: {}", token);
    let now = Local::now();
    println!("Today is: {}", now);
}
