use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};

use crate::{
    domain::{
        entities::toll_transaction::TollTransaction,
        value_objects::{transaction_status::TransactionStatus, vehicle_class::VehicleClass},
    },
    error::AppError,
};

#[derive(Clone, Debug)]
pub struct TransactionRepository {
    pool: PgPool,
}

impl TransactionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_active_by_plate(
        &self,
        plate_number: &str,
    ) -> Result<Option<TollTransaction>, AppError> {
        let row = sqlx::query(include_str!("./sql/find_active_by_plate.sql"))
            .bind(&plate_number)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(r) => Ok(Some(self.map_row_to_transaction(&r)?)),
            None => Ok(None),
        }
    }

    pub async fn save_entry(&self, tx: &TollTransaction) -> Result<TollTransaction, AppError> {
        sqlx::query(include_str!("./sql/transaction_save_entry.sql"))
            .bind(tx.id)
            .bind(&tx.plate_number)
            .bind(tx.vehicle_class.as_str())
            .bind(tx.entry_toll_point_id)
            .bind(tx.entry_gate_id)
            .bind(tx.entry_time)
            .bind(tx.entry_km)
            .bind(tx.status.as_str())
            .bind(tx.created_at)
            .bind(tx.updated_at)
            .execute(&self.pool)
            .await?;

        Ok(tx.clone())
    }

    fn map_row_to_transaction(
        &self,
        r: &sqlx::postgres::PgRow,
    ) -> Result<TollTransaction, AppError> {
        let vc_str: String = r.get("vehicle_class");
        let vehicle_class: VehicleClass = VehicleClass::from_str(&vc_str)?;

        let entry_time: DateTime<Utc> = r.get("entry_time");
        let entry_km: f64 = r.get("entry_km");

        let exit_time: Option<DateTime<Utc>> = r.get("exit_time");
        let exit_km: Option<f64> = r.get("exit_km");
        let distance_km: Option<f64> = r.get("distance_km");

        let base_fare: Option<f64> = r.get("base_fare");
        let rate_per_km: Option<f64> = r.get("rate_per_km");
        let total_fee: Option<f64> = r.get("total_fee");

        let status_str: String = r.get("status");
        let status = TransactionStatus::from_str(&status_str)?;

        let created_at: DateTime<Utc> = r.get("created_at");
        let updated_at: DateTime<Utc> = r.get("updated_at");

        Ok(TollTransaction {
            id: r.get("id"),
            plate_number: r.get("plate_number"),
            vehicle_class,
            entry_toll_point_id: r.get("entry_toll_point_id"),
            entry_gate_id: r.get("entry_gate_id"),
            entry_time,
            entry_km,
            exit_toll_point_id: r.get("exit_toll_point_id"),
            exit_gate_id: r.get("exit_gate_id"),
            exit_time,
            exit_km,
            distance_km,
            base_fare,
            rate_per_km,
            total_fee,
            status,
            receipt_hash: r.get("receipt_hash"),
            created_at,
            updated_at,
        })
    }
}
