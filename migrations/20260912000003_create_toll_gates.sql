-- Migration: 0003_create_toll_gates
CREATE TABLE IF NOT EXISTS toll_gates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    toll_point_id UUID NOT NULL REFERENCES toll_points(id) ON DELETE CASCADE,
    code VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    gate_type VARCHAR(20) NOT NULL, -- 'ENTRY', 'EXIT', 'BIDIRECTIONAL'
    assigned_vehicle_class VARCHAR(50), -- 'GOLONGAN_1', 'GOLONGAN_2', etc. (or NULL for pure exits)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_toll_point_gate_code UNIQUE(toll_point_id, code)
);

CREATE INDEX IF NOT EXISTS idx_toll_gates_toll_point_id ON toll_gates(toll_point_id);
CREATE INDEX IF NOT EXISTS idx_toll_gates_type ON toll_gates(gate_type);
