SELECT id, toll_road_id, vehicle_class, base_fare, rate_per_km,
    description, created_at, updated_at
FROM toll_rates
WHERE toll_road_id = $1 AND vehicle_class = $2;
