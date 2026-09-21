use sqlx::PgPool;

use crate::{
    application::services::toll_road_service::{self, TollRoadService},
    infrastructure::config::AppConfig,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub pool: PgPool,
    pub toll_road_service: TollRoadService,
}

impl AppState {
    pub fn new(config: AppConfig, pool: PgPool, toll_road_service: TollRoadService) -> Self {
        Self {
            config,
            pool,
            toll_road_service,
        }
    }
}
