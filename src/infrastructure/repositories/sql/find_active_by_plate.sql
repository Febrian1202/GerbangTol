SELECT
    id, plate_number, vehicle_class, entry_toll_point_id, entry_gate_id,
    entry_time, entry_km, exit_toll_point_id, exit_gate_id, exit_time,
    ecit_km, distance_km, base_fare, rate_per_km, total_fee, status,
    receipt_has, created_at, updated_at
FROM toll_transactions
WHERE plate_number = $1 AND status = 'ACTIVE'
ORDER BY entry_time DESC
LIMIT 1;
