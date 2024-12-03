use anyhow::Result;
use bot::ArbitrageBot;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use std::time::Duration;

mod bot;
mod consts;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger with timestamp
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    dotenv::dotenv().ok();

    let bot = ArbitrageBot::new()?;

    loop {
        if let Err(e) = bot.run().await {
            log::error!("Error running bot: {}", e);
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}
