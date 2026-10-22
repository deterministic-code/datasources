DROP PROCEDURE IF EXISTS bulk_create_user
CREATE PROCEDURE bulk_create_user(IN payload JSON, OUT ids JSON)
BEGIN
  INSERT INTO `users` (username, password, is_enabled, created, updated)
  SELECT username, password, is_enabled, created, FROM_BASE64(updated)
  FROM JSON_TABLE(payload, '$[*]' COLUMNS (
    username TEXT PATH '$.username',
    password TEXT PATH '$.password',
    is_enabled TINYINT PATH '$.is_enabled',
    created DATETIME PATH '$.created',
    updated TEXT PATH '$.updated'
  )) AS payload_rows;
  GET DIAGNOSTICS @n = ROW_COUNT;
  SET @first = LAST_INSERT_ID();
  SET ids = JSON_ARRAY();
  SET @i = 0;
  WHILE @i < @n DO
    SET ids = JSON_ARRAY_APPEND(ids, '$', @first + @i);
    SET @i = @i + 1;
  END WHILE;
END
DROP PROCEDURE IF EXISTS bulk_update_user
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
DROP PROCEDURE IF EXISTS bulk_delete_user
CREATE PROCEDURE bulk_delete_user(IN payload JSON)
  DELETE u FROM `users` u
  JOIN JSON_TABLE(payload, '$[*]' COLUMNS (id INT PATH '$.id')) AS payload_rows ON u.id = payload_rows.id
