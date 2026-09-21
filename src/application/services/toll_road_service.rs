use chrono::Utc;
use uuid::Uuid;

use crate::{
    application::dto::toll_road_dto::{CreateTollRoadRequest, TollRoadResponse},
    domain::entities::{toll_point::TollPoint, toll_road::TollRoad},
    error::AppError,
    infrastructure::repositories::toll_road_repo::TollRoadRepository,
};

#[derive(Clone, Debug)]
pub struct TollRoadService {
    repo: TollRoadRepository,
}

impl TollRoadService {
    pub fn new(repo: TollRoadRepository) -> Self {
        Self { repo }
    }

    pub async fn create(&self, req: CreateTollRoadRequest) -> Result<TollRoadResponse, AppError> {
        let road_id = Uuid::new_v4();
        let now = Utc::now();

        let mut points = Vec::new();

        for (idx, p) in req.points.into_iter().enumerate() {
            let point_id = Uuid::new_v4();

            points.push(TollPoint {
                id: point_id,
                toll_road_id: road_id,
                code: p.code,
                name: p.name,
                km_position: p.km_position,
                sequence_order: p.sequence_order.unwrap_or(idx as i32),
                created_at: now,
                updated_at: now,
            });
        }

        let toll_road = TollRoad {
            id: road_id,
            code: req.code,
            name: req.name,
            description: req.description,
            points,
            created_at: now,
            updated_at: now,
        };

        let saved = self.repo.save(&toll_road).await?;
        Ok(saved.into())
    }
}
