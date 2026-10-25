UPDATE toll_transactions
SET
exit_toll_point_id = $1,
exit_gate_id = $2,
exit_time = $3,
exit_km = $4,
distance_km = $5,
base_fare = $6,
rate_per_km = $7,
total_fee = $8,
status = $9,
receipt_hash = $10,
updated_at = $11
WHERE id = $12;
