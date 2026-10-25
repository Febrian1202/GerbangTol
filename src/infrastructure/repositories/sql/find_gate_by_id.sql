SELECT
    g.id AS gate_id, g.code AS gate_code, g.name AS gate_name, g.gate_type,
    g.assigned_vehicle_class, g.created_at AS gate_created_at, g.updated_at AS gate_updated_at,
    p.id as point_id,p.code as point_code, p.name as point_name, p.km_position, p.sequence_order, p.created_at as point_created_at, p.updated_at as point_updated_at,
    r.id as road_id, r.code as road_code, r.name as road_name, r.description as road_description, r.created_at as road_created_at, r.updated_at as road_updated_at
FROM toll_gates g
INNER JOIN toll_points p ON g.toll_point_id = p.id
INNER JOIN toll_roads r ON p.toll_road_id = r.id
WHERE g.id = $1
