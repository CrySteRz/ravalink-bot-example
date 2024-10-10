use rusty_discord_bot::client::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let mut rusty = Client::default().await?;
    if let Err(why) = rusty.start().await {
        println!("Rusty Crash: {:?}", why);
    };
    Ok(())
}
