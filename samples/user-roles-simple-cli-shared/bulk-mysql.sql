DROP PROCEDURE IF EXISTS bulk_create_user
---
CREATE PROCEDURE bulk_create_user(IN payload JSON)
  INSERT INTO `users` (id, username, password, is_enabled, created, updated)
  SELECT id, username, password, is_enabled, created, FROM_BASE64(updated)
  FROM JSON_TABLE(payload, '$[*]' COLUMNS (
    id INT PATH '$.id',
    username TEXT PATH '$.username',
    password TEXT PATH '$.password',
    is_enabled TINYINT PATH '$.is_enabled',
    created DATETIME PATH '$.created',
    updated TEXT PATH '$.updated'
  )) AS payload_rows
---
DROP PROCEDURE IF EXISTS bulk_update_user
---
CREATE PROCEDURE bulk_update_user(IN payload JSON)
  UPDATE `users` u
  JOIN JSON_TABLE(payload, '$[*]' COLUMNS (
    id INT PATH '$.id',
    username TEXT PATH '$.username',
    password TEXT PATH '$.password',
    is_enabled TINYINT PATH '$.is_enabled',
    created DATETIME PATH '$.created',
    updated TEXT PATH '$.updated'
  )) AS payload_rows ON u.id = payload_rows.id
  SET u.username = payload_rows.username, u.password = payload_rows.password, u.is_enabled = payload_rows.is_enabled,
      u.created = payload_rows.created, u.updated = FROM_BASE64(payload_rows.updated)
---
DROP PROCEDURE IF EXISTS bulk_delete_user
---
CREATE PROCEDURE bulk_delete_user(IN payload JSON)
  DELETE u FROM `users` u
  JOIN JSON_TABLE(payload, '$[*]' COLUMNS (id INT PATH '$.id')) AS payload_rows ON u.id = payload_rows.id
