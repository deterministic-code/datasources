CREATE TABLE IF NOT EXISTS contact_sources (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  uuid TEXT NOT NULL,
  created TEXT NOT NULL,
  updated TEXT NOT NULL,
  version BLOB NOT NULL,
  name TEXT NOT NULL,
  description TEXT
);
CREATE TABLE IF NOT EXISTS address_types (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  uuid TEXT NOT NULL,
  created TEXT NOT NULL,
  updated TEXT NOT NULL,
  version BLOB NOT NULL,
  name TEXT NOT NULL,
  description TEXT
);
CREATE TABLE IF NOT EXISTS phone_types (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  uuid TEXT NOT NULL,
  created TEXT NOT NULL,
  updated TEXT NOT NULL,
  version BLOB NOT NULL,
  name TEXT NOT NULL,
  description TEXT
);
CREATE TABLE IF NOT EXISTS contacts (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  uuid TEXT NOT NULL,
  created TEXT NOT NULL,
  updated TEXT NOT NULL,
  version BLOB NOT NULL,
  contact_source_id INTEGER NOT NULL,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  email TEXT,
  notes TEXT
);
CREATE TABLE IF NOT EXISTS addresses (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  uuid TEXT NOT NULL,
  created TEXT NOT NULL,
  updated TEXT NOT NULL,
  version BLOB NOT NULL,
  contact_id INTEGER NOT NULL,
  address_type_id INTEGER NOT NULL,
  line1 TEXT NOT NULL,
  line2 TEXT,
  city TEXT NOT NULL,
  region TEXT,
  postal_code TEXT,
  country TEXT
);
CREATE TABLE IF NOT EXISTS phones (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  uuid TEXT NOT NULL,
  created TEXT NOT NULL,
  updated TEXT NOT NULL,
  version BLOB NOT NULL,
  contact_id INTEGER NOT NULL,
  phone_type_id INTEGER NOT NULL,
  number TEXT NOT NULL,
  label TEXT
);
CREATE TABLE IF NOT EXISTS contact_settings (
  contact_id INTEGER NOT NULL,
  key TEXT NOT NULL,
  value TEXT NOT NULL,
  PRIMARY KEY (contact_id, key)
);
