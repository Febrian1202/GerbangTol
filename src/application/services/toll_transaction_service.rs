use chrono::Utc;
use sha2::{Digest, Sha256};
use tokio::time::Instant;
use uuid::Uuid;

use crate::{
    application::dto::toll_transaction_dto::{
        CalculationResult, TollEntryRequest, TollExitRequest,
    },
    domain::{
        entities::toll_transaction::TollTransaction,
        value_objects::{transaction_status::TransactionStatus, vehicle_class::VehicleClass},
    },
    error::AppError,
    infrastructure::repositories::{
        toll_road_repo::TollRoadRepository, transaction_repo::TransactionRepository,
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

    pub async fn process_entry(&self, req: TollEntryRequest) -> Result<TollTransaction, AppError> {
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

        Ok(saved)
    }

    pub async fn process_exit(&self, req: TollExitRequest) -> Result<TollTransaction, AppError> {
        let plate = req.plate_number.trim().to_uppercase();

        let mut transaction = self
            .tx_repo
            .find_active_by_plate(&plate)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("Transaction untuk plat {} tidak ditemukan", plate))
            })?;

        let (exit_gate, exit_point, toll_road) = self
            .toll_road_repo
            .find_gate_by_id(req.gate_id)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("Gate ID {} tidak ditemukan", req.gate_id))
            })?;

        if !exit_gate.gate_type.allow_exit() {
            return Err(AppError::BadRequest(format!(
                "Gate ID {} tidak untuk exit",
                req.gate_id
            )));
        }

        let rate = self
            .toll_road_repo
            .find_rate_for_road(toll_road.id, transaction.vehicle_class)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "Biaya di toll toad {} tidak ditemukan",
                    toll_road.id
                ))
            })?;

        let calc_result = self
            .calculate_toll(
                plate.clone(),
                transaction.vehicle_class,
                transaction.entry_km,
                exit_point.km_position,
                rate.base_fare,
                rate.rate_per_km,
            )
            .await?;

        let now = Utc::now();

        transaction.exit_toll_point_id = Some(exit_point.id);
        transaction.exit_gate_id = Some(exit_gate.id);
        transaction.exit_time = Some(now);
        transaction.exit_km = Some(exit_point.km_position);
        transaction.distance_km = Some(calc_result.distance_km);
        transaction.base_fare = Some(calc_result.base_fare);
        transaction.total_fee = Some(calc_result.total_fee);
        transaction.rate_per_km = Some(calc_result.rate_per_km);
        transaction.status = TransactionStatus::Completed;
        transaction.receipt_hash = Some(calc_result.receipt_hash);
        transaction.updated_at = now;

        let updated_transaction = self.tx_repo.update_exit(&transaction).await?;
        Ok(updated_transaction)
    }

    async fn calculate_toll(
        &self,
        plate_number: String,
        vehicle_class: VehicleClass,
        entry_km: f64,
        exit_km: f64,
        base_fare: f64,
        rate_per_km: f64,
    ) -> Result<CalculationResult, AppError> {
        tokio::task::spawn_blocking(move || {
            let start = Instant::now();

            let distance = (exit_km - entry_km).abs();

            let total_fee = base_fare + (distance * rate_per_km);

            let mut hasher = Sha256::new();
            hasher.update(plate_number.as_bytes());
            hasher.update(vehicle_class.as_str().as_bytes());
            hasher.update(entry_km.to_string().as_bytes());
            hasher.update(exit_km.to_string().as_bytes());
            hasher.update(total_fee.to_string().as_bytes());

            let mut hash_bytes = hasher.finalize();
            for i in 0..10000u32 {
                let mut round = Sha256::new();
                round.update(&hash_bytes);
                round.update(i.to_be_bytes());
                hash_bytes = round.finalize();
            }

            let receipt_hash = format!("{:x}", hash_bytes);
            let elapsed_nanos = start.elapsed().as_nanos();

            Ok(CalculationResult {
                distance_km: distance,
                base_fare,
                rate_per_km,
                total_fee,
                receipt_hash,
                elapsed_nanos,
            })
        })
        .await
        .map_err(|e| AppError::Internal(format!("Komputasi blocking task gaga: {}", e)))?
    }
}
