#[warn(unused_imports)]
#[macro_use]
extern crate log;
use discord_bot::{ghub, config, sheets};
use poise::serenity_prelude as serenity;
pub struct Data {
    pub sheets: sheets::SheetsClient,
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
    let g_hub = ghub::build_hub(&conf).await.unwrap();
    let g_client = sheets::SheetsClient {
        hub: g_hub,
        config: conf,
    };
}
