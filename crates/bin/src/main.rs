use core::config::AppConfig;
use std::sync::Arc;
use clap::Parser;
use infra::connection_pool::NexusConnectionManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config = Arc::new(AppConfig::parse());

    println!("config: {:#?}", config);

    //info!("environment loaded and configuration parsed, initializing Postgres connection and running migrations...");
    let pg_pool = NexusConnectionManager::new_pool(&config.database_url, config.run_migrations)
        .await
        .expect("could not initialize the database connection pool");
    
    println!("Nexus backend is running...");

    Ok(())
}
