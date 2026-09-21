use crate::infrastructure::repositories::toll_road_repo::TollRoadRepository;

#[derive(Clone, Debug)]
pub struct TollRoadService {
    repo: TollRoadRepository,
}

impl TollRoadService {
    pub fn new(repo: TollRoadRepository) -> Self {
        Self { repo }
    }
}
