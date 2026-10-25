use chrono::Utc;
use uuid::Uuid;

use crate::{
    application::dto::toll_transaction_dto::TollEntryRequest,
    domain::{
        entities::toll_transaction::TollTransaction,
        value_objects::{transaction_status::TransactionStatus, vehicle_class::VehicleClass},
    },
    error::AppError,
    infrastructure::repositories::{
        toll_road_repo::{self, TollRoadRepository},
        transaction_repo::TransactionRepository,
    },
};

#[derive(Clone, Debug)]
pub struct TollTransactionService {
    toll_road_repo: TollRoadRepository,
    tx_repo: TransactionRepository,
}

impl TollTransactionService {
    pub fn new(toll_road_repo: TollRoadRepository, tx_repo: TransactionRepository) -> Self {
        Self {
            toll_road_repo,
            tx_repo,
        }
    }

    pub async fn process_entry(&self, req: TollEntryRequest) -> Result<(), AppError> {
        let plate = req.plate_number.trim().to_uppercase();

        if let Some(_) = self.tx_repo.find_active_by_plate(&plate).await? {
            return Err(AppError::Conflict(format!(
                "Kendaraan dengan plat {} masih berada dalam jalan tol",
                plate
            )));
        }

        // Cari gate & point
        let (gate, point, road) = self
            .toll_road_repo
            .find_gate_by_id(req.gate_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Gate ID {} not found", req.gate_id)))?;

        if !gate.gate_type.allow_entry() {
            return Err(AppError::BadRequest(format!(
                "Gate toll {}, bukan untuk masuk",
                req.gate_id
            )));
        }

        let vehicle_class = gate
            .assigned_vehicle_class
            .unwrap_or(VehicleClass::Golongan1);

        let now = Utc::now();
        let transaction = TollTransaction {
            id: Uuid::new_v4(),
            plate_number: plate.clone(),
            vehicle_class,
            entry_toll_point_id: point.id,
            entry_gate_id: gate.id,
            entry_time: now,
            entry_km: point.km_position,
            exit_toll_point_id: None,
            exit_gate_id: None,
            exit_time: None,
            exit_km: None,
            distance_km: None,
            base_fare: None,
            rate_per_km: None,
            total_fee: None,
            status: TransactionStatus::Active,
            receipt_hash: None,
            created_at: now,
            updated_at: now,
        };

        let saved = self.tx_repo.save_entry(&transaction).await?;

        Ok(())
    }
}
