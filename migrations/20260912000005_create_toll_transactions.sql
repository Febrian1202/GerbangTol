-- Migration: 0005_create_toll_transactions
CREATE TABLE IF NOT EXISTS toll_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    plate_number VARCHAR(20) NOT NULL,
    vehicle_class VARCHAR(50) NOT NULL,
    entry_toll_point_id UUID NOT NULL REFERENCES toll_points(id),
    entry_gate_id UUID NOT NULL REFERENCES toll_gates(id),
    entry_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    entry_km DOUBLE PRECISION NOT NULL,
    exit_toll_point_id UUID REFERENCES toll_points(id),
    exit_gate_id UUID REFERENCES toll_gates(id),
    exit_time TIMESTAMPTZ,
    exit_km DOUBLE PRECISION,
    distance_km DOUBLE PRECISION,
    base_fare DOUBLE PRECISION,
    rate_per_km DOUBLE PRECISION,
    total_fee DOUBLE PRECISION,
    payment_status VARCHAR(20) NOT NULL DEFAULT 'UNPAID', -- 'UNPAID', 'PAID', 'FAILED'
    status VARCHAR(20) NOT NULL DEFAULT 'ACTIVE', -- 'ACTIVE', 'COMPLETED', 'CANCELLED'
    receipt_hash VARCHAR(64),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_toll_tx_plate_status ON toll_transactions(plate_number, status);
CREATE INDEX IF NOT EXISTS idx_toll_tx_entry_time ON toll_transactions(entry_time DESC);
