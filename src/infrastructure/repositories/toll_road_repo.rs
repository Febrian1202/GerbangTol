use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::{
    domain::{
        entities::{
            toll_gate::TollGate, toll_point::TollPoint, toll_rate::TollRate, toll_road::TollRoad,
        },
        value_objects::{gate_type::GateType, vehicle_class::VehicleClass},
    },
    error::AppError,
};

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

        sqlx::query(include_str!("./sql/delete_all_toll_rate_by_road_id.sql"))
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

            for gate in &point.gates {
                sqlx::query(include_str!("./sql/create_toll_gate.sql"))
                    .bind(gate.id)
                    .bind(point.id)
                    .bind(&gate.code)
                    .bind(&gate.name)
                    .bind(gate.gate_type.as_str())
                    .bind(
                        gate.assigned_vehicle_class
                            .map(|vc| vc.as_str().to_string()),
                    )
                    .bind(gate.created_at)
                    .bind(gate.updated_at)
                    .execute(&mut *tx)
                    .await?;
            }
        }

        for rate in &road.rates {
            sqlx::query(include_str!("./sql/create_toll_rates.sql"))
                .bind(rate.id)
                .bind(road.id)
                .bind(rate.vehicle_class.as_str())
                .bind(rate.base_fare)
                .bind(rate.rate_per_km)
                .bind(&rate.description)
                .bind(rate.created_at)
                .bind(rate.updated_at)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(road.clone())
    }
    pub async fn find_gate_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<(TollGate, TollPoint, TollRoad)>, AppError> {
        let row = sqlx::query(include_str!("./sql/find_gate_by_id.sql"))
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        let Some(r) = row else { return Ok(None) };

        let gate_type_str: String = r.get("gate_type");
        let gate_type = GateType::from_str(&gate_type_str)?;

        let assigned_vc_str: Option<String> = r.get("assigned_vehicle_class");
        let assigned_vc = match assigned_vc_str {
            Some(s) => Some(VehicleClass::from_str(&s)?),
            None => None,
        };

        let gate_created_at: DateTime<Utc> = r.get("gate_created_at");
        let gate_updated_at: DateTime<Utc> = r.get("gate_updated_at");

        let gate = TollGate {
            id: r.get("gate_id"),
            toll_point_id: r.get("point_id"),
            code: r.get("gate_code"),
            name: r.get("gate_name"),
            gate_type,
            assigned_vehicle_class: assigned_vc,
            created_at: gate_created_at,
            updated_at: gate_updated_at,
        };

        let km_position: f64 = r.get("km_position");
        let sequence_order: i32 = r.get("sequence_order");

        let point_created_at: DateTime<Utc> = r.get("point_created_at");
        let point_updated_at: DateTime<Utc> = r.get("point_updated_at");

        let point = TollPoint {
            id: r.get("point_id"),
            toll_road_id: r.get("road_id"),
            code: r.get("point_code"),
            name: r.get("point_name"),
            km_position,
            sequence_order,
            gates: vec![],
            created_at: point_created_at,
            updated_at: point_updated_at,
        };

        let road_created_at: DateTime<Utc> = r.get("road_created_at");
        let road_updated_at: DateTime<Utc> = r.get("road_updated_at");

        let road = TollRoad {
            id: r.get("road_id"),
            code: r.get("road_code"),
            name: r.get("road_name"),
            description: r.get("road_description"),
            points: vec![],
            rates: vec![],
            created_at: road_created_at,
            updated_at: road_updated_at,
        };

        Ok(Some((gate, point, road)))
    }

    pub async fn find_rate_for_road(
        &self,
        road_id: Uuid,
        vc: VehicleClass,
    ) -> Result<Option<TollRate>, AppError> {
        let row = sqlx::query(include_str!("./sql/find_rate_for_road.sql"))
            .bind(road_id)
            .bind(vc.as_str())
            .fetch_optional(&self.pool)
            .await?;

        let Some(r) = row else {
            return Ok(None);
        };

        Ok(Some(TollRate {
            id: r.get("id"),
            toll_road_id: r.get("toll_road_id"),
            vehicle_class: vc,
            base_fare: r.get("base_fare"),
            rate_per_km: r.get("rate_per_km"),
            description: r.get("description"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }
}
