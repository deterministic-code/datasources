DROP PROCEDURE IF EXISTS bulk_create_user(jsonb, jsonb)
---
CREATE PROCEDURE bulk_create_user(payload jsonb, INOUT ids jsonb DEFAULT NULL)
LANGUAGE plpgsql
AS $$
BEGIN
  WITH inserted AS (
    INSERT INTO "users" (username, password, is_enabled, created, updated)
    SELECT username, password, is_enabled, created, decode(updated, 'base64')
    FROM jsonb_to_recordset(payload) AS x(
      username text, password text, is_enabled boolean, created timestamptz, updated text
    )
    RETURNING id
  )
  SELECT COALESCE(jsonb_agg(id), '[]'::jsonb) INTO ids FROM inserted;
END;
$$
---
DROP PROCEDURE IF EXISTS bulk_update_user(jsonb)
---
CREATE PROCEDURE bulk_update_user(payload jsonb)
LANGUAGE plpgsql
AS $$
BEGIN
  UPDATE "users" u SET
    username = x.username,
    password = x.password,
    is_enabled = x.is_enabled,
    created = x.created,
    updated = decode(x.updated, 'base64')
  FROM jsonb_to_recordset(payload) AS x(
    id int, username text, password text, is_enabled boolean, created timestamptz, updated text
  )
  WHERE u.id = x.id;
END;
$$
---
DROP PROCEDURE IF EXISTS bulk_delete_user(jsonb)
---
CREATE PROCEDURE bulk_delete_user(payload jsonb)
LANGUAGE plpgsql
AS $$
BEGIN
  DELETE FROM "users" WHERE id IN (SELECT id FROM jsonb_to_recordset(payload) AS x(id int));
END;
$$
