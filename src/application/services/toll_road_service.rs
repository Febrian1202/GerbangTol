use chrono::Utc;
use uuid::Uuid;

use crate::{
    application::dto::toll_road_dto::{CreateTollRoadRequest, TollRoadResponse},
    domain::entities::toll_road::TollRoad,
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

        let toll_road = TollRoad {
            id: road_id,
            code: req.code,
            name: req.name,
            description: req.description,
            created_at: now,
            updated_at: now,
        };

        let saved = self.repo.save(&toll_road).await?;
        Ok(saved.into())
    }
}
