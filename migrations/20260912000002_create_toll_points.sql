-- Migration: 0002_create_toll_points
CREATE TABLE IF NOT EXISTS toll_points (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    toll_road_id UUID NOT NULL REFERENCES toll_roads(id) ON DELETE CASCADE,
    code VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    km_position DOUBLE PRECISION NOT NULL,
    sequence_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_toll_road_point_code UNIQUE(toll_road_id, code)
);

CREATE INDEX IF NOT EXISTS idx_toll_points_toll_road_id ON toll_points(toll_road_id);
CREATE INDEX IF NOT EXISTS idx_toll_points_km ON toll_points(km_position);
