use sqlx::PgPool;

use crate::{domain::entities::toll_road::TollRoad, error::AppError};

#[derive(Clone)]
struct TollRoadRepository {
    pool: PgPool,
}

impl TollRoadRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, road: &TollRoad) -> Result<TollRoad, AppError> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(include_str!("./sql/insert_toll_road.sql"))
            .bind(road.id)
            .bind(&road.code)
            .bind(&road.description)
            .bind(road.created_at)
            .bind(road.updated_at)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(road.clone())
    }
}
