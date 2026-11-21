use std::time::Duration;

#[warn(unused_imports)]
#[macro_use]
extern crate log;
use chrono::prelude::*;
use discord_bot::{ghub::{self, Hub}, config, sheets};
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
    let conf = config::Config::new();
    println!("My token: {}", token);
    let http_client = reqwest::Client::builder()
    .connect_timeout(Duration::from_millis(200))
    .https_only(true)
    .build()
    .unwrap();
    let g_hub = ghub::build_hub(&conf).await.unwrap();
    let g_client = sheets::SheetsClient {
        http: http_client,
        auth: g_hub,
        config: conf,
    };
}
