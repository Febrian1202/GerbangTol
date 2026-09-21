use sqlx::PgPool;

use crate::infrastructure::config::AppConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub pool: PgPool,
}

impl AppState {
    pub fn new(config: AppConfig, pool: PgPool) -> Self {
        Self { config, pool }
    }
}
