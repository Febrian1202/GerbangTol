use sqlx::PgPool;

use crate::{domain::entities::toll_road::TollRoad, error::AppError};

#[derive(Clone, Debug)]
pub struct TollRoadRepository {
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
            .bind(&road.name)
            .bind(&road.description)
            .bind(road.created_at)
            .bind(road.updated_at)
            .execute(&mut *tx)
            .await?;

        sqlx::query(include_str!("./sql/delete_all_toll_point_by_id.sql"))
            .bind(road.id)
            .execute(&mut *tx)
            .await?;

        for point in &road.points {
            sqlx::query(include_str!("./sql/create_toll_points.sql"))
                .bind(point.id)
                .bind(road.id)
                .bind(&point.code)
                .bind(&point.name)
                .bind(point.km_position)
                .bind(point.sequence_order)
                .bind(point.created_at)
                .bind(point.updated_at)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(road.clone())
    }
}
