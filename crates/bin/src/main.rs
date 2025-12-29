use core::config::AppConfig;
use std::sync::Arc;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config = Arc::new(AppConfig::parse());
    
    println!("Nexus backend is running...");

    Ok(())
}
