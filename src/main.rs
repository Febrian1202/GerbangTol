mod domain;
mod error;
mod infrastructure;
mod interface;

use infrastructure::config::AppConfig;
use tokio;
use tracing_subscriber::util::SubscriberInitExt;

use crate::{
    infrastructure::database::{create_pool, run_migrations},
    interface::{routes::create_router, state::AppState},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::registry().init();

    let config = AppConfig::from_env()?;

    let pool = match create_pool(&config).await {
        Ok(p) => {
            if let Err(e) = run_migrations(&p).await {
                tracing::error!("Gagal menjalankan migration: {}", e)
            };
            p
        }
        Err(e) => {
            return Err(e.into());
        }
    };

    let state = AppState::new(config.clone(), pool.clone());

    let router = create_router(state);
    let address = config.server_address();
    let listener = tokio::net::TcpListener::bind(&address).await?;

    axum::serve(listener, router).await?;

    Ok(())
}
