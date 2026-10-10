DROP PROCEDURE IF EXISTS bulk_create_contact(jsonb)
---
CREATE PROCEDURE bulk_create_contact(payload jsonb)
LANGUAGE plpgsql AS $$
BEGIN
  INSERT INTO contacts (id, uuid, created, updated, version, contact_source_id, first_name, last_name, email, notes)
  SELECT id, uuid, created, updated, decode(version, 'base64'), contact_source_id, first_name, last_name, email, notes
  FROM jsonb_to_recordset(payload) AS x(
    id int, uuid text, created timestamptz, updated timestamptz, version text, contact_source_id int,
    first_name text, last_name text, email text, notes text
  );
END;
$$
---
DROP PROCEDURE IF EXISTS bulk_update_contact(jsonb)
---
CREATE PROCEDURE bulk_update_contact(payload jsonb)
LANGUAGE plpgsql AS $$
BEGIN
  UPDATE contacts c SET
    uuid = x.uuid, created = x.created, updated = x.updated, version = decode(x.version, 'base64'),
    contact_source_id = x.contact_source_id, first_name = x.first_name, last_name = x.last_name,
    email = x.email, notes = x.notes
  FROM jsonb_to_recordset(payload) AS x(
    id int, uuid text, created timestamptz, updated timestamptz, version text, contact_source_id int,
    first_name text, last_name text, email text, notes text
  )
  WHERE c.id = x.id;
END;
$$
---
DROP PROCEDURE IF EXISTS bulk_delete_contact(jsonb)
---
CREATE PROCEDURE bulk_delete_contact(payload jsonb)
LANGUAGE plpgsql AS $$
BEGIN
  DELETE FROM contacts WHERE id IN (SELECT id FROM jsonb_to_recordset(payload) AS x(id int));
END;
$$
---
DROP PROCEDURE IF EXISTS bulk_create_address(jsonb)
---
CREATE PROCEDURE bulk_create_address(payload jsonb)
LANGUAGE plpgsql AS $$
BEGIN
  INSERT INTO addresses (id, uuid, created, updated, version, contact_id, address_type_id, line1, line2, city, region, postal_code, country)
  SELECT id, uuid, created, updated, decode(version, 'base64'), contact_id, address_type_id, line1, line2, city, region, postal_code, country
  FROM jsonb_to_recordset(payload) AS x(
    id int, uuid text, created timestamptz, updated timestamptz, version text, contact_id int, address_type_id int,
    line1 text, line2 text, city text, region text, postal_code text, country text
  );
END;
$$
---
DROP PROCEDURE IF EXISTS bulk_update_address(jsonb)
---
CREATE PROCEDURE bulk_update_address(payload jsonb)
LANGUAGE plpgsql AS $$
BEGIN
  UPDATE addresses a SET line1 = x.line1, city = x.city, updated = x.updated, version = decode(x.version, 'base64')
  FROM jsonb_to_recordset(payload) AS x(id int, line1 text, city text, updated timestamptz, version text)
  WHERE a.id = x.id;
END;
$$
---
DROP PROCEDURE IF EXISTS bulk_delete_address(jsonb)
---
CREATE PROCEDURE bulk_delete_address(payload jsonb)
LANGUAGE plpgsql AS $$
BEGIN
  DELETE FROM addresses WHERE id IN (SELECT id FROM jsonb_to_recordset(payload) AS x(id int));
END;
$$
---
DROP PROCEDURE IF EXISTS bulk_create_phone(jsonb)
---
CREATE PROCEDURE bulk_create_phone(payload jsonb)
LANGUAGE plpgsql AS $$
BEGIN
  INSERT INTO phones (id, uuid, created, updated, version, contact_id, phone_type_id, number, label)
  SELECT id, uuid, created, updated, decode(version, 'base64'), contact_id, phone_type_id, number, label
  FROM jsonb_to_recordset(payload) AS x(
    id int, uuid text, created timestamptz, updated timestamptz, version text, contact_id int, phone_type_id int,
    number text, label text
  );
END;
$$
---
DROP PROCEDURE IF EXISTS bulk_update_phone(jsonb)
---
CREATE PROCEDURE bulk_update_phone(payload jsonb)
LANGUAGE plpgsql AS $$
BEGIN
  UPDATE phones p SET number = x.number, label = x.label, updated = x.updated, version = decode(x.version, 'base64')
  FROM jsonb_to_recordset(payload) AS x(id int, number text, label text, updated timestamptz, version text)
  WHERE p.id = x.id;
END;
$$
---
DROP PROCEDURE IF EXISTS bulk_delete_phone(jsonb)
---
CREATE PROCEDURE bulk_delete_phone(payload jsonb)
LANGUAGE plpgsql AS $$
BEGIN
  DELETE FROM phones WHERE id IN (SELECT id FROM jsonb_to_recordset(payload) AS x(id int));
END;
$$
---
DROP PROCEDURE IF EXISTS bulk_create_setting(jsonb)
---
CREATE PROCEDURE bulk_create_setting(payload jsonb)
LANGUAGE plpgsql AS $$
BEGIN
  INSERT INTO contact_settings (contact_id, key, value)
  SELECT contact_id, key, value
  FROM jsonb_to_recordset(payload) AS x(contact_id int, key text, value text);
END;
$$
---
DROP PROCEDURE IF EXISTS bulk_update_setting(jsonb)
---
CREATE PROCEDURE bulk_update_setting(payload jsonb)
LANGUAGE plpgsql AS $$
BEGIN
  UPDATE contact_settings s SET value = x.value
  FROM jsonb_to_recordset(payload) AS x(contact_id int, key text, value text)
  WHERE s.contact_id = x.contact_id AND s.key = x.key;
END;
$$
---
DROP PROCEDURE IF EXISTS bulk_delete_setting(jsonb)
---
CREATE PROCEDURE bulk_delete_setting(payload jsonb)
LANGUAGE plpgsql AS $$
BEGIN
  DELETE FROM contact_settings s
  USING jsonb_to_recordset(payload) AS x(contact_id int, key text)
  WHERE s.contact_id = x.contact_id AND s.key = x.key;
END;
$$
