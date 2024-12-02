use anyhow::Result;
use bot::ArbitrageBot;
use std::time::Duration;

mod bot;
mod consts;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    let bot = ArbitrageBot::new()?;
    
    loop {
        if let Err(e) = bot.run().await {
            eprintln!("Error running bot: {}", e);
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}
