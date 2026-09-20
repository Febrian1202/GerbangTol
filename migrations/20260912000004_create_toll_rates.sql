-- Migration: 0004_create_toll_rates
CREATE TABLE IF NOT EXISTS toll_rates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    toll_road_id UUID NOT NULL REFERENCES toll_roads(id) ON DELETE CASCADE,
    vehicle_class VARCHAR(50) NOT NULL, -- 'GOLONGAN_1', 'GOLONGAN_2', 'GOLONGAN_3', 'GOLONGAN_4', 'GOLONGAN_5'
    base_fare DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    rate_per_km DOUBLE PRECISION NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_toll_road_vehicle_class UNIQUE(toll_road_id, vehicle_class)
);

CREATE INDEX IF NOT EXISTS idx_toll_rates_toll_road_id ON toll_rates(toll_road_id);
