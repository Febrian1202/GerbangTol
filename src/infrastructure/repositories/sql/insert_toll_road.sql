INSERT INTO toll_roads (id, code, name, description, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6)
ON CONFLICT (ID) DO UPDATE SET
name = EXCLUDED.name,
description = EXCLUDED.description,
updated_at = EXCLUDED.updated_at
