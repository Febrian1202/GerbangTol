use crate::application::services::{
    toll_road_service::TollRoadService, toll_transaction_service::TollTransactionService,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub toll_road_service: TollRoadService,
    pub toll_transaction_service: TollTransactionService,
}

impl AppState {
    pub fn new(
        toll_road_service: TollRoadService,
        toll_transaction_service: TollTransactionService,
    ) -> Self {
        Self {
            toll_road_service,
            toll_transaction_service,
        }
    }
}
