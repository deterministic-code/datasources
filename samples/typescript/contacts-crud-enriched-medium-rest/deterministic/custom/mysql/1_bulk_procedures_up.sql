DROP PROCEDURE IF EXISTS bulk_create_contact
GO
CREATE PROCEDURE bulk_create_contact(IN payload JSON, OUT ids JSON)
BEGIN
  INSERT INTO contacts (uuid, created, updated, version, contact_source_id, first_name, last_name, email, notes)
  SELECT uuid, created, updated, FROM_BASE64(version), contact_source_id, first_name, last_name, email, notes
  FROM JSON_TABLE(payload, '$[*]' COLUMNS (
    uuid TEXT PATH '$.uuid', created DATETIME PATH '$.created', updated DATETIME PATH '$.updated',
    version TEXT PATH '$.version', contact_source_id INT PATH '$.contact_source_id', first_name TEXT PATH '$.first_name',
    last_name TEXT PATH '$.last_name', email TEXT PATH '$.email', notes TEXT PATH '$.notes'
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
GO
DROP PROCEDURE IF EXISTS bulk_update_contact
GO
CREATE PROCEDURE bulk_update_contact(IN payload JSON)
  UPDATE contacts c
  JOIN JSON_TABLE(payload, '$[*]' COLUMNS (
    id INT PATH '$.id', uuid TEXT PATH '$.uuid', created DATETIME PATH '$.created', updated DATETIME PATH '$.updated',
    version TEXT PATH '$.version', contact_source_id INT PATH '$.contact_source_id', first_name TEXT PATH '$.first_name',
    last_name TEXT PATH '$.last_name', email TEXT PATH '$.email', notes TEXT PATH '$.notes'
  )) AS payload_rows ON c.id = payload_rows.id
  SET c.uuid = payload_rows.uuid, c.created = payload_rows.created, c.updated = payload_rows.updated, c.version = FROM_BASE64(payload_rows.version),
      c.contact_source_id = payload_rows.contact_source_id, c.first_name = payload_rows.first_name, c.last_name = payload_rows.last_name,
      c.email = payload_rows.email, c.notes = payload_rows.notes
GO
DROP PROCEDURE IF EXISTS bulk_delete_contact
GO
CREATE PROCEDURE bulk_delete_contact(IN payload JSON)
  DELETE c FROM contacts c JOIN JSON_TABLE(payload, '$[*]' COLUMNS (id INT PATH '$.id')) AS payload_rows ON c.id = payload_rows.id
GO
DROP PROCEDURE IF EXISTS bulk_create_address
GO
CREATE PROCEDURE bulk_create_address(IN payload JSON, OUT ids JSON)
BEGIN
  INSERT INTO addresses (uuid, created, updated, version, contact_id, address_type_id, line1, line2, city, region, postal_code, country)
  SELECT uuid, created, updated, FROM_BASE64(version), contact_id, address_type_id, line1, line2, city, region, postal_code, country
  FROM JSON_TABLE(payload, '$[*]' COLUMNS (
    uuid TEXT PATH '$.uuid', created DATETIME PATH '$.created', updated DATETIME PATH '$.updated',
    version TEXT PATH '$.version', contact_id INT PATH '$.contact_id', address_type_id INT PATH '$.address_type_id',
    line1 TEXT PATH '$.line1', line2 TEXT PATH '$.line2', city TEXT PATH '$.city', region TEXT PATH '$.region',
    postal_code TEXT PATH '$.postal_code', country TEXT PATH '$.country'
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
GO
DROP PROCEDURE IF EXISTS bulk_update_address
GO
CREATE PROCEDURE bulk_update_address(IN payload JSON)
  UPDATE addresses a
  JOIN JSON_TABLE(payload, '$[*]' COLUMNS (
    id INT PATH '$.id', uuid TEXT PATH '$.uuid', created DATETIME PATH '$.created', updated DATETIME PATH '$.updated',
    version TEXT PATH '$.version', contact_id INT PATH '$.contact_id', address_type_id INT PATH '$.address_type_id',
    line1 TEXT PATH '$.line1', line2 TEXT PATH '$.line2', city TEXT PATH '$.city', region TEXT PATH '$.region',
    postal_code TEXT PATH '$.postal_code', country TEXT PATH '$.country'
  )) AS payload_rows ON a.id = payload_rows.id
  SET a.line1 = payload_rows.line1, a.city = payload_rows.city, a.updated = payload_rows.updated, a.version = FROM_BASE64(payload_rows.version)
GO
DROP PROCEDURE IF EXISTS bulk_delete_address
GO
CREATE PROCEDURE bulk_delete_address(IN payload JSON)
  DELETE a FROM addresses a JOIN JSON_TABLE(payload, '$[*]' COLUMNS (id INT PATH '$.id')) AS payload_rows ON a.id = payload_rows.id
GO
DROP PROCEDURE IF EXISTS bulk_create_phone
GO
CREATE PROCEDURE bulk_create_phone(IN payload JSON, OUT ids JSON)
BEGIN
  INSERT INTO phones (uuid, created, updated, version, contact_id, phone_type_id, number, label)
  SELECT uuid, created, updated, FROM_BASE64(version), contact_id, phone_type_id, number, label
  FROM JSON_TABLE(payload, '$[*]' COLUMNS (
    uuid TEXT PATH '$.uuid', created DATETIME PATH '$.created', updated DATETIME PATH '$.updated',
    version TEXT PATH '$.version', contact_id INT PATH '$.contact_id', phone_type_id INT PATH '$.phone_type_id',
    number TEXT PATH '$.number', label TEXT PATH '$.label'
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
GO
DROP PROCEDURE IF EXISTS bulk_update_phone
GO
CREATE PROCEDURE bulk_update_phone(IN payload JSON)
  UPDATE phones p
  JOIN JSON_TABLE(payload, '$[*]' COLUMNS (
    id INT PATH '$.id', number TEXT PATH '$.number', label TEXT PATH '$.label', updated DATETIME PATH '$.updated',
    version TEXT PATH '$.version'
  )) AS payload_rows ON p.id = payload_rows.id
  SET p.number = payload_rows.number, p.label = payload_rows.label, p.updated = payload_rows.updated, p.version = FROM_BASE64(payload_rows.version)
GO
DROP PROCEDURE IF EXISTS bulk_delete_phone
GO
CREATE PROCEDURE bulk_delete_phone(IN payload JSON)
  DELETE p FROM phones p JOIN JSON_TABLE(payload, '$[*]' COLUMNS (id INT PATH '$.id')) AS payload_rows ON p.id = payload_rows.id
GO
DROP PROCEDURE IF EXISTS bulk_create_setting
GO
CREATE PROCEDURE bulk_create_setting(IN payload JSON)
  INSERT INTO contact_settings (contact_id, `key`, value)
  SELECT contact_id, `key`, value
  FROM JSON_TABLE(payload, '$[*]' COLUMNS (
    contact_id INT PATH '$.contact_id', `key` TEXT PATH '$.key', value TEXT PATH '$.value'
  )) AS payload_rows
GO
DROP PROCEDURE IF EXISTS bulk_update_setting
GO
CREATE PROCEDURE bulk_update_setting(IN payload JSON)
  UPDATE contact_settings s
  JOIN JSON_TABLE(payload, '$[*]' COLUMNS (
    contact_id INT PATH '$.contact_id', `key` TEXT PATH '$.key', value TEXT PATH '$.value'
  )) AS payload_rows ON s.contact_id = payload_rows.contact_id
    AND s.`key` COLLATE utf8mb4_0900_ai_ci = payload_rows.`key` COLLATE utf8mb4_0900_ai_ci
  SET s.value = payload_rows.value
GO
DROP PROCEDURE IF EXISTS bulk_delete_setting
GO
CREATE PROCEDURE bulk_delete_setting(IN payload JSON)
  DELETE s FROM contact_settings s
  JOIN JSON_TABLE(payload, '$[*]' COLUMNS (
    contact_id INT PATH '$.contact_id', `key` TEXT PATH '$.key'
  )) AS payload_rows ON s.contact_id = payload_rows.contact_id
    AND s.`key` COLLATE utf8mb4_0900_ai_ci = payload_rows.`key` COLLATE utf8mb4_0900_ai_ci
