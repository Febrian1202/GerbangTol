use crate::infrastructure::config::AppConfig;
use sqlx::{PgPool, migrate::MigrateError, postgres::PgPoolOptions};

pub async fn create_pool(config: &AppConfig) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .connect(config.db.connection_string().as_str())
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
