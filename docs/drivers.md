<!-- Generated from docs/DriverSummary.yaml and docs/DriverConversions.yaml. Run npm run docs:drivers. -->

# Languages and Datasource Providers {#languages}

Pick a language, then a driver. Each driver section lists {{Information|information|information}}, Overrides, {{TypeConverter|typeConverter|type_converter}}, {{RowTypeConverters|rowTypeConverters|row_type_converters}}, and {{DefaultValueConverter|defaultValueConverter|default_value_converter}} from that language's {{IDriver|IDriver|Driver}}.

<div class="driver-header">
  <a class="driver-tile" href="#csharp">
    <img src="/languages/csharp.svg" alt="" width="48" height="48" />
    <span>C#</span>
  </a>
  <a class="driver-tile" href="#rust">
    <img src="/languages/rust.svg" alt="" width="48" height="48" />
    <span>Rust</span>
  </a>
  <a class="driver-tile" href="#typescript">
    <img src="/languages/typescript.svg" alt="" width="48" height="48" />
    <span>TypeScript</span>
  </a>
</div>

## C# {#csharp}

<a href="#languages" class="driver-back">← Languages and Datasource Providers</a>

Drivers for C#. Click a tile for Information, Overrides, TypeConverter, RowTypeConverters, and DefaultValueConverter from that `IDriver`.

<div class="driver-header">
  <a class="driver-tile" href="#csharp-sqlite">
    <img src="/drivers/sqlite.svg" alt="" width="48" height="48" />
    <span>SQLite</span>
  </a>
  <a class="driver-tile" href="#csharp-mysql">
    <img src="/drivers/mysql.svg" alt="" width="48" height="48" />
    <span>MySQL</span>
  </a>
  <a class="driver-tile" href="#csharp-postgres">
    <img src="/drivers/postgres.svg" alt="" width="48" height="48" />
    <span>PostgreSQL</span>
  </a>
</div>

### SQLite {#csharp-sqlite}

<a href="#csharp" class="driver-back">← C# drivers</a>

#### Information

- Embedded SQL database for local development and tests.
- Full connection, command, transaction, and repository support.

#### Overrides

- Overrides SelectInsertedIdStatement to SELECT last_insert_rowid() AS {column}
- Overrides CreateTableStatement type mapping (int→INTEGER, bool→BOOLEAN, uuid/string/datetime→TEXT, decimal→NUMERIC, float→REAL, binary→BLOB)

#### TypeConverter

| Type | From | To |
|---|---|---|
| `string` | `string` | `TEXT` |
| `character` | `character` | `TEXT` |
| `integer` | `integer` | `INTEGER` |
| `unsignedinteger` | `unsignedinteger` | `INTEGER` |
| `smallinteger` | `smallinteger` | `INTEGER` |
| `unsignedsmallinteger` | `unsignedsmallinteger` | `INTEGER` |
| `biginteger` | `biginteger` | `INTEGER` |
| `unsignedbiginteger` | `unsignedbiginteger` | `INTEGER` |
| `float` | `float` | `REAL` |
| `decimal` | `decimal` | `NUMERIC` |
| `boolean` | `boolean` | `BOOLEAN` |
| `datetime` | `datetime` | `TEXT` |
| `binary` | `binary` | `BLOB` |
| `uuid` | `uuid` | `TEXT` |
| `reference` | `reference` | `INTEGER` |
| `string` | `new SpecField { Type = "string" }` | `TEXT` |
| `string` | `new SpecField { Type = "string", Size = "unlimited" }` | `TEXT` |
| `string` | `new SpecField { Type = "string", Size = 64 }` | `VARCHAR(64)` |
| `nope` | `nope` | `InvalidOperationException` |
| `string` | `TEXT` | `string` |
| `integer` | `INTEGER` | `integer` |
| `nope` | `nope` | `InvalidOperationException` |

#### RowTypeConverters

| Type | From | To |
|---|---|---|
| `boolean` | `1L` | `true` |
| `boolean` | `0L` | `false` |
| `datetime` | `"2020-01-02T03:04:05.000Z"` | `new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc)` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000").ToByteArray()` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `7L` | `"7"` |
| `decimal` | `10.5d` | `"10.5"` |
| `decimal` | `"10.50"` | `"10.5"` |
| `decimal` | `0L` | `"0"` |
| `decimal` | `"0.00"` | `"0"` |
| `integer` | `"10"` | `10L` |
| `integer` | `"0"` | `0L` |
| `integer` | `""` | `FormatException` |
| `integer` | `"1.0"` | `FormatException` |
| `boolean` | `true` | `1L` |
| `boolean` | `false` | `0L` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `10.5m` | `"10.5"` |

#### DefaultValueConverter

| Column type | From | To |
|---|---|---|
| `datetime` | `Now` | `(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))` |
| `datetime` | `UtcNow` | `(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))` |
| `datetime` | `DateTime('2020-01-01T00:00:00Z')` | `'2020-01-01T00:00:00Z'` |
| `uuid` | `Empty` | `'00000000-0000-0000-0000-000000000000'` |
| `uuid` | `uuid('11111111-1111-1111-1111-111111111111')` | `'11111111-1111-1111-1111-111111111111'` |
| `binary` | `Hex('ab')` | `X'ab'` |
| `boolean` | `true` | `1` |
| `boolean` | `false` | `0` |
| `string` | `hi` | `'hi'` |
| `integer` | `7` | `7` |
| `reference` | `1` | `null` |
| `nope` | `x` | `InvalidOperationException` |
| `datetime` | `2020-01-01T00:00:00Z` | `InvalidOperationException` |
| `datetime` | `(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))` | `Now` |
| `datetime` | `(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))` | `UtcNow` |
| `datetime` | `'2020-01-01T00:00:00Z'` | `DateTime('2020-01-01T00:00:00Z')` |
| `binary` | `X'ab'` | `Hex('ab')` |
| `string` | `'hi'` | `String` |
| `string` | `oops` | `InvalidOperationException` |

#### ChangeLogs

_None._

### MySQL {#csharp-mysql}

<a href="#csharp" class="driver-back">← C# drivers</a>

#### Information

- MySQL / MariaDB server driver.
- Full connection, command, transaction, and repository support.

#### Overrides

- Overrides SelectInsertedIdStatement to SELECT LAST_INSERT_ID() AS {column}
- Overrides CreateTableStatement identity to AUTO_INCREMENT
- Overrides CreateTableStatement type mapping (int→INT, uuid→CHAR(36), string/datetime→TEXT, bool→TINYINT, decimal→DECIMAL(18,4), float→DOUBLE, binary→BLOB)

#### TypeConverter

| Type | From | To |
|---|---|---|
| `string` | `string` | `TEXT` |
| `character` | `character` | `CHAR(1)` |
| `integer` | `integer` | `INT` |
| `unsignedinteger` | `unsignedinteger` | `INT UNSIGNED` |
| `smallinteger` | `smallinteger` | `SMALLINT` |
| `unsignedsmallinteger` | `unsignedsmallinteger` | `SMALLINT UNSIGNED` |
| `biginteger` | `biginteger` | `BIGINT` |
| `unsignedbiginteger` | `unsignedbiginteger` | `BIGINT UNSIGNED` |
| `float` | `float` | `DOUBLE` |
| `decimal` | `decimal` | `DECIMAL(10, 2)` |
| `boolean` | `boolean` | `TINYINT(1)` |
| `datetime` | `datetime` | `DATETIME(3)` |
| `binary` | `binary` | `BLOB` |
| `uuid` | `uuid` | `CHAR(36)` |
| `reference` | `reference` | `INT` |
| `string` | `new SpecField { Type = "string" }` | `TEXT` |
| `string` | `new SpecField { Type = "string", Size = 64 }` | `VARCHAR(64)` |
| `string` | `new SpecField { Type = "string", Size = 20000 }` | `LONGTEXT` |
| `character` | `new SpecField { Type = "character", Size = 4 }` | `CHAR(4)` |
| `decimal` | `new SpecField { Type = "decimal", Size = new[] { 8, 3 } }` | `DECIMAL(8, 3)` |
| `binary` | `new SpecField { Type = "binary", Size = 16 }` | `VARBINARY(16)` |
| `binary` | `new SpecField { Type = "binary", Size = 70000 }` | `LONGBLOB` |
| `nope` | `nope` | `InvalidOperationException` |
| `decimal` | `new SpecField { Name = "amount", Type = "decimal", Size = 8 }` | `InvalidOperationException` |
| `integer` | `INT` | `integer` |
| `nope` | `nope` | `InvalidOperationException` |

#### RowTypeConverters

| Type | From | To |
|---|---|---|
| `boolean` | `1L` | `true` |
| `boolean` | `0L` | `false` |
| `datetime` | `"2020-01-02T03:04:05.000Z"` | `new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc)` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000").ToByteArray()` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `7L` | `"7"` |
| `decimal` | `10.5d` | `"10.5"` |
| `decimal` | `"10.50"` | `"10.5"` |
| `decimal` | `0L` | `"0"` |
| `decimal` | `"0.00"` | `"0"` |
| `binary` | `"AQIDBA=="` | `new byte[] { 1, 2, 3, 4 }` |
| `integer` | `"10"` | `10L` |
| `integer` | `"0"` | `0L` |
| `integer` | `"1.0"` | `FormatException` |
| `integer` | `""` | `FormatException` |
| `boolean` | `true` | `1L` |
| `boolean` | `false` | `0L` |
| `datetime` | `"2020-01-02T03:04:05.000Z"` | `"2020-01-02 03:04:05.000"` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `10.5m` | `"10.5"` |
| `binary` | `"AQIDBA=="` | `new byte[] { 1, 2, 3, 4 }` |

#### DefaultValueConverter

| Column type | From | To |
|---|---|---|
| `datetime` | `Now` | `(NOW(3))` |
| `datetime` | `UtcNow` | `(UTC_TIMESTAMP(3))` |
| `uuid` | `NewId` | `(UUID())` |
| `binary` | `Hex('ab')` | `(X'ab')` |
| `boolean` | `true` | `1` |
| `boolean` | `false` | `0` |
| `string` | `hi` | `('hi')` |
| `integer` | `7` | `7` |
| `reference` | `1` | `null` |
| `nope` | `x` | `InvalidOperationException` |
| `binary` | `(X'ab')` | `Hex('ab')` |
| `binary` | `X'ab'` | `Hex('ab')` |
| `string` | `('hi')` | `String` |
| `string` | `'hi'` | `String` |
| `uuid` | `(UUID())` | `NewId` |
| `binary` | `X'ab')` | `InvalidOperationException` |
| `string` | `'hi')` | `InvalidOperationException` |

#### ChangeLogs

_None._

### PostgreSQL {#csharp-postgres}

<a href="#csharp" class="driver-back">← C# drivers</a>

#### Information

- PostgreSQL server driver.
- Full connection, command, transaction, and repository support.

#### Overrides

- Overrides SelectInsertedIdStatement to SELECT lastval() AS {column}
- Overrides CreateTableStatement identity to GENERATED BY DEFAULT AS IDENTITY
- Overrides CreateTableStatement boolean defaults (0/1 → TRUE/FALSE)
- Overrides CreateTableStatement type mapping (int→INTEGER, uuid→UUID, string→TEXT, bool→BOOLEAN, decimal→NUMERIC, float→DOUBLE PRECISION, datetime→TIMESTAMPTZ, binary→BYTEA)

#### TypeConverter

| Type | From | To |
|---|---|---|
| `string` | `string` | `TEXT` |
| `character` | `character` | `CHAR(1)` |
| `integer` | `integer` | `INTEGER` |
| `unsignedinteger` | `unsignedinteger` | `BIGINT` |
| `smallinteger` | `smallinteger` | `SMALLINT` |
| `unsignedsmallinteger` | `unsignedsmallinteger` | `INTEGER` |
| `biginteger` | `biginteger` | `BIGINT` |
| `unsignedbiginteger` | `unsignedbiginteger` | `NUMERIC(20)` |
| `float` | `float` | `DOUBLE PRECISION` |
| `decimal` | `decimal` | `NUMERIC` |
| `boolean` | `boolean` | `BOOLEAN` |
| `datetime` | `datetime` | `TIMESTAMPTZ` |
| `binary` | `binary` | `BYTEA` |
| `uuid` | `uuid` | `UUID` |
| `reference` | `reference` | `INTEGER` |
| `string` | `new SpecField { Type = "string", Size = 64 }` | `VARCHAR(64)` |
| `character` | `new SpecField { Type = "character", Size = 4 }` | `CHAR(4)` |
| `decimal` | `new SpecField { Type = "decimal", Size = new[] { 10, 2 } }` | `NUMERIC(10, 2)` |
| `nope` | `nope` | `InvalidOperationException` |
| `integer` | `INTEGER` | `integer` |
| `unsignedinteger` | `BIGINT` | `unsignedinteger` |
| `nope` | `nope` | `InvalidOperationException` |

#### RowTypeConverters

| Type | From | To |
|---|---|---|
| `boolean` | `1L` | `true` |
| `boolean` | `0L` | `false` |
| `datetime` | `"2020-01-02T03:04:05.000Z"` | `new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc)` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000").ToByteArray()` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `7L` | `"7"` |
| `decimal` | `10.5d` | `"10.5"` |
| `decimal` | `"10.50"` | `"10.5"` |
| `decimal` | `0L` | `"0"` |
| `decimal` | `"0.00"` | `"0"` |
| `binary` | `"AQIDBA=="` | `new byte[] { 1, 2, 3, 4 }` |
| `integer` | `"10"` | `10L` |
| `integer` | `"0"` | `0L` |
| `integer` | `"1.0"` | `FormatException` |
| `integer` | `""` | `FormatException` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `10.5m` | `"10.5"` |
| `binary` | `"AQIDBA=="` | `new byte[] { 1, 2, 3, 4 }` |

#### DefaultValueConverter

| Column type | From | To |
|---|---|---|
| `datetime` | `Now` | `LOCALTIMESTAMP` |
| `datetime` | `UtcNow` | `(NOW() AT TIME ZONE 'UTC')` |
| `uuid` | `NewId` | `gen_random_uuid()` |
| `uuid` | `Empty` | `'00000000-0000-0000-0000-000000000000'` |
| `binary` | `Hex('ab')` | `'\xab'` |
| `boolean` | `true` | `TRUE` |
| `boolean` | `false` | `FALSE` |
| `string` | `hi` | `'hi'` |
| `integer` | `7` | `7` |
| `reference` | `1` | `null` |
| `nope` | `x` | `InvalidOperationException` |
| `datetime` | `LOCALTIMESTAMP` | `Now` |
| `datetime` | `(NOW() AT TIME ZONE 'UTC')` | `UtcNow` |
| `uuid` | `gen_random_uuid()` | `NewId` |
| `binary` | `'\xab'` | `Hex('ab')` |
| `string` | `'hi'` | `String` |
| `datetime` | `nope` | `InvalidOperationException` |

#### ChangeLogs

_None._

## Rust {#rust}

<a href="#languages" class="driver-back">← Languages and Datasource Providers</a>

Drivers for Rust. Click a tile for information, Overrides, type_converter, row_type_converters, and default_value_converter from that `Driver`.

<div class="driver-header">
  <a class="driver-tile" href="#rust-sqlite">
    <img src="/drivers/sqlite.svg" alt="" width="48" height="48" />
    <span>SQLite</span>
  </a>
  <a class="driver-tile" href="#rust-mysql">
    <img src="/drivers/mysql.svg" alt="" width="48" height="48" />
    <span>MySQL</span>
  </a>
  <a class="driver-tile" href="#rust-postgres">
    <img src="/drivers/postgres.svg" alt="" width="48" height="48" />
    <span>PostgreSQL</span>
  </a>
</div>

### SQLite {#rust-sqlite}

<a href="#rust" class="driver-back">← Rust drivers</a>

#### information

- Embedded SQL database for local development and tests.
- Full connection, command, transaction, and repository support.

#### Overrides

- Overrides SelectInsertedIdStatement to SELECT last_insert_rowid() AS {column}
- Overrides CreateTableStatement type mapping (int→INTEGER, bool→BOOLEAN, uuid/string/datetime→TEXT, decimal→NUMERIC, float→REAL, binary→BLOB)

#### type_converter

| Type | From | To |
|---|---|---|
| `string` | `string` | `TEXT` |
| `character` | `character` | `TEXT` |
| `integer` | `integer` | `INTEGER` |
| `unsignedinteger` | `unsignedinteger` | `INTEGER` |
| `smallinteger` | `smallinteger` | `INTEGER` |
| `unsignedsmallinteger` | `unsignedsmallinteger` | `INTEGER` |
| `biginteger` | `biginteger` | `INTEGER` |
| `unsignedbiginteger` | `unsignedbiginteger` | `INTEGER` |
| `float` | `float` | `REAL` |
| `decimal` | `decimal` | `NUMERIC` |
| `boolean` | `boolean` | `BOOLEAN` |
| `datetime` | `datetime` | `TEXT` |
| `binary` | `binary` | `BLOB` |
| `uuid` | `uuid` | `TEXT` |
| `reference` | `reference` | `INTEGER` |
| `string` | `new SpecField { Type = "string" }` | `TEXT` |
| `string` | `new SpecField { Type = "string", Size = "unlimited" }` | `TEXT` |
| `string` | `new SpecField { Type = "string", Size = 64 }` | `VARCHAR(64)` |
| `nope` | `nope` | `InvalidOperationException` |
| `string` | `TEXT` | `string` |
| `integer` | `INTEGER` | `integer` |
| `nope` | `nope` | `InvalidOperationException` |

#### row_type_converters

| Type | From | To |
|---|---|---|
| `boolean` | `1L` | `true` |
| `boolean` | `0L` | `false` |
| `datetime` | `"2020-01-02T03:04:05.000Z"` | `new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc)` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000").ToByteArray()` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `7L` | `"7"` |
| `decimal` | `10.5d` | `"10.5"` |
| `decimal` | `"10.50"` | `"10.5"` |
| `decimal` | `0L` | `"0"` |
| `decimal` | `"0.00"` | `"0"` |
| `integer` | `"10"` | `10L` |
| `integer` | `"0"` | `0L` |
| `integer` | `""` | `FormatException` |
| `integer` | `"1.0"` | `FormatException` |
| `boolean` | `true` | `1L` |
| `boolean` | `false` | `0L` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `10.5m` | `"10.5"` |

#### default_value_converter

| Column type | From | To |
|---|---|---|
| `datetime` | `Now` | `(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))` |
| `datetime` | `UtcNow` | `(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))` |
| `datetime` | `DateTime('2020-01-01T00:00:00Z')` | `'2020-01-01T00:00:00Z'` |
| `uuid` | `Empty` | `'00000000-0000-0000-0000-000000000000'` |
| `uuid` | `uuid('11111111-1111-1111-1111-111111111111')` | `'11111111-1111-1111-1111-111111111111'` |
| `binary` | `Hex('ab')` | `X'ab'` |
| `boolean` | `true` | `1` |
| `boolean` | `false` | `0` |
| `string` | `hi` | `'hi'` |
| `integer` | `7` | `7` |
| `reference` | `1` | `null` |
| `nope` | `x` | `InvalidOperationException` |
| `datetime` | `2020-01-01T00:00:00Z` | `InvalidOperationException` |
| `datetime` | `(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))` | `Now` |
| `datetime` | `(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))` | `UtcNow` |
| `datetime` | `'2020-01-01T00:00:00Z'` | `DateTime('2020-01-01T00:00:00Z')` |
| `binary` | `X'ab'` | `Hex('ab')` |
| `string` | `'hi'` | `String` |
| `string` | `oops` | `InvalidOperationException` |

#### change_logs

_None._

### MySQL {#rust-mysql}

<a href="#rust" class="driver-back">← Rust drivers</a>

#### information

- MySQL / MariaDB server driver.
- Full connection, command, transaction, and repository support.

#### Overrides

- Overrides SelectInsertedIdStatement to SELECT LAST_INSERT_ID() AS {column}
- Overrides CreateTableStatement identity to AUTO_INCREMENT
- Overrides CreateTableStatement type mapping (int→INT, uuid→CHAR(36), string/datetime→TEXT, bool→TINYINT, decimal→DECIMAL(18,4), float→DOUBLE, binary→BLOB)

#### type_converter

| Type | From | To |
|---|---|---|
| `string` | `string` | `TEXT` |
| `character` | `character` | `CHAR(1)` |
| `integer` | `integer` | `INT` |
| `unsignedinteger` | `unsignedinteger` | `INT UNSIGNED` |
| `smallinteger` | `smallinteger` | `SMALLINT` |
| `unsignedsmallinteger` | `unsignedsmallinteger` | `SMALLINT UNSIGNED` |
| `biginteger` | `biginteger` | `BIGINT` |
| `unsignedbiginteger` | `unsignedbiginteger` | `BIGINT UNSIGNED` |
| `float` | `float` | `DOUBLE` |
| `decimal` | `decimal` | `DECIMAL(10, 2)` |
| `boolean` | `boolean` | `TINYINT(1)` |
| `datetime` | `datetime` | `DATETIME(3)` |
| `binary` | `binary` | `BLOB` |
| `uuid` | `uuid` | `CHAR(36)` |
| `reference` | `reference` | `INT` |
| `string` | `new SpecField { Type = "string" }` | `TEXT` |
| `string` | `new SpecField { Type = "string", Size = 64 }` | `VARCHAR(64)` |
| `string` | `new SpecField { Type = "string", Size = 20000 }` | `LONGTEXT` |
| `character` | `new SpecField { Type = "character", Size = 4 }` | `CHAR(4)` |
| `decimal` | `new SpecField { Type = "decimal", Size = new[] { 8, 3 } }` | `DECIMAL(8, 3)` |
| `binary` | `new SpecField { Type = "binary", Size = 16 }` | `VARBINARY(16)` |
| `binary` | `new SpecField { Type = "binary", Size = 70000 }` | `LONGBLOB` |
| `nope` | `nope` | `InvalidOperationException` |
| `decimal` | `new SpecField { Name = "amount", Type = "decimal", Size = 8 }` | `InvalidOperationException` |
| `integer` | `INT` | `integer` |
| `nope` | `nope` | `InvalidOperationException` |

#### row_type_converters

| Type | From | To |
|---|---|---|
| `boolean` | `1L` | `true` |
| `boolean` | `0L` | `false` |
| `datetime` | `"2020-01-02T03:04:05.000Z"` | `new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc)` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000").ToByteArray()` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `7L` | `"7"` |
| `decimal` | `10.5d` | `"10.5"` |
| `decimal` | `"10.50"` | `"10.5"` |
| `decimal` | `0L` | `"0"` |
| `decimal` | `"0.00"` | `"0"` |
| `binary` | `"AQIDBA=="` | `new byte[] { 1, 2, 3, 4 }` |
| `integer` | `"10"` | `10L` |
| `integer` | `"0"` | `0L` |
| `integer` | `"1.0"` | `FormatException` |
| `integer` | `""` | `FormatException` |
| `boolean` | `true` | `1L` |
| `boolean` | `false` | `0L` |
| `datetime` | `"2020-01-02T03:04:05.000Z"` | `"2020-01-02 03:04:05.000"` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `10.5m` | `"10.5"` |
| `binary` | `"AQIDBA=="` | `new byte[] { 1, 2, 3, 4 }` |

#### default_value_converter

| Column type | From | To |
|---|---|---|
| `datetime` | `Now` | `(NOW(3))` |
| `datetime` | `UtcNow` | `(UTC_TIMESTAMP(3))` |
| `uuid` | `NewId` | `(UUID())` |
| `binary` | `Hex('ab')` | `(X'ab')` |
| `boolean` | `true` | `1` |
| `boolean` | `false` | `0` |
| `string` | `hi` | `('hi')` |
| `integer` | `7` | `7` |
| `reference` | `1` | `null` |
| `nope` | `x` | `InvalidOperationException` |
| `binary` | `(X'ab')` | `Hex('ab')` |
| `binary` | `X'ab'` | `Hex('ab')` |
| `string` | `('hi')` | `String` |
| `string` | `'hi'` | `String` |
| `uuid` | `(UUID())` | `NewId` |
| `binary` | `X'ab')` | `InvalidOperationException` |
| `string` | `'hi')` | `InvalidOperationException` |

#### change_logs

_None._

### PostgreSQL {#rust-postgres}

<a href="#rust" class="driver-back">← Rust drivers</a>

#### information

- PostgreSQL server driver.
- Full connection, command, transaction, and repository support.

#### Overrides

- Overrides SelectInsertedIdStatement to SELECT lastval() AS {column}
- Overrides CreateTableStatement identity to GENERATED BY DEFAULT AS IDENTITY
- Overrides CreateTableStatement boolean defaults (0/1 → TRUE/FALSE)
- Overrides CreateTableStatement type mapping (int→INTEGER, uuid→UUID, string→TEXT, bool→BOOLEAN, decimal→NUMERIC, float→DOUBLE PRECISION, datetime→TIMESTAMPTZ, binary→BYTEA)

#### type_converter

| Type | From | To |
|---|---|---|
| `string` | `string` | `TEXT` |
| `character` | `character` | `CHAR(1)` |
| `integer` | `integer` | `INTEGER` |
| `unsignedinteger` | `unsignedinteger` | `BIGINT` |
| `smallinteger` | `smallinteger` | `SMALLINT` |
| `unsignedsmallinteger` | `unsignedsmallinteger` | `INTEGER` |
| `biginteger` | `biginteger` | `BIGINT` |
| `unsignedbiginteger` | `unsignedbiginteger` | `NUMERIC(20)` |
| `float` | `float` | `DOUBLE PRECISION` |
| `decimal` | `decimal` | `NUMERIC` |
| `boolean` | `boolean` | `BOOLEAN` |
| `datetime` | `datetime` | `TIMESTAMPTZ` |
| `binary` | `binary` | `BYTEA` |
| `uuid` | `uuid` | `UUID` |
| `reference` | `reference` | `INTEGER` |
| `string` | `new SpecField { Type = "string", Size = 64 }` | `VARCHAR(64)` |
| `character` | `new SpecField { Type = "character", Size = 4 }` | `CHAR(4)` |
| `decimal` | `new SpecField { Type = "decimal", Size = new[] { 10, 2 } }` | `NUMERIC(10, 2)` |
| `nope` | `nope` | `InvalidOperationException` |
| `integer` | `INTEGER` | `integer` |
| `unsignedinteger` | `BIGINT` | `unsignedinteger` |
| `nope` | `nope` | `InvalidOperationException` |

#### row_type_converters

| Type | From | To |
|---|---|---|
| `boolean` | `1L` | `true` |
| `boolean` | `0L` | `false` |
| `datetime` | `"2020-01-02T03:04:05.000Z"` | `new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc)` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000").ToByteArray()` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `7L` | `"7"` |
| `decimal` | `10.5d` | `"10.5"` |
| `decimal` | `"10.50"` | `"10.5"` |
| `decimal` | `0L` | `"0"` |
| `decimal` | `"0.00"` | `"0"` |
| `binary` | `"AQIDBA=="` | `new byte[] { 1, 2, 3, 4 }` |
| `integer` | `"10"` | `10L` |
| `integer` | `"0"` | `0L` |
| `integer` | `"1.0"` | `FormatException` |
| `integer` | `""` | `FormatException` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `10.5m` | `"10.5"` |
| `binary` | `"AQIDBA=="` | `new byte[] { 1, 2, 3, 4 }` |

#### default_value_converter

| Column type | From | To |
|---|---|---|
| `datetime` | `Now` | `LOCALTIMESTAMP` |
| `datetime` | `UtcNow` | `(NOW() AT TIME ZONE 'UTC')` |
| `uuid` | `NewId` | `gen_random_uuid()` |
| `uuid` | `Empty` | `'00000000-0000-0000-0000-000000000000'` |
| `binary` | `Hex('ab')` | `'\xab'` |
| `boolean` | `true` | `TRUE` |
| `boolean` | `false` | `FALSE` |
| `string` | `hi` | `'hi'` |
| `integer` | `7` | `7` |
| `reference` | `1` | `null` |
| `nope` | `x` | `InvalidOperationException` |
| `datetime` | `LOCALTIMESTAMP` | `Now` |
| `datetime` | `(NOW() AT TIME ZONE 'UTC')` | `UtcNow` |
| `uuid` | `gen_random_uuid()` | `NewId` |
| `binary` | `'\xab'` | `Hex('ab')` |
| `string` | `'hi'` | `String` |
| `datetime` | `nope` | `InvalidOperationException` |

#### change_logs

_None._

## TypeScript {#typescript}

<a href="#languages" class="driver-back">← Languages and Datasource Providers</a>

Drivers for TypeScript. Click a tile for information, Overrides, typeConverter, rowTypeConverters, and defaultValueConverter from that `IDriver`.

<div class="driver-header">
  <a class="driver-tile" href="#typescript-sqlite">
    <img src="/drivers/sqlite.svg" alt="" width="48" height="48" />
    <span>SQLite</span>
  </a>
  <a class="driver-tile" href="#typescript-mysql">
    <img src="/drivers/mysql.svg" alt="" width="48" height="48" />
    <span>MySQL</span>
  </a>
  <a class="driver-tile" href="#typescript-postgres">
    <img src="/drivers/postgres.svg" alt="" width="48" height="48" />
    <span>PostgreSQL</span>
  </a>
</div>

### SQLite {#typescript-sqlite}

<a href="#typescript" class="driver-back">← TypeScript drivers</a>

#### information

- Embedded SQL database for local development and tests.
- Full connection, command, transaction, and repository support.

#### Overrides

- Overrides SelectInsertedIdStatement to SELECT last_insert_rowid() AS {column}
- Overrides CreateTableStatement type mapping (int→INTEGER, bool→BOOLEAN, uuid/string/datetime→TEXT, decimal→NUMERIC, float→REAL, binary→BLOB)

#### typeConverter

| Type | From | To |
|---|---|---|
| `string` | `string` | `TEXT` |
| `character` | `character` | `TEXT` |
| `integer` | `integer` | `INTEGER` |
| `unsignedinteger` | `unsignedinteger` | `INTEGER` |
| `smallinteger` | `smallinteger` | `INTEGER` |
| `unsignedsmallinteger` | `unsignedsmallinteger` | `INTEGER` |
| `biginteger` | `biginteger` | `INTEGER` |
| `unsignedbiginteger` | `unsignedbiginteger` | `INTEGER` |
| `float` | `float` | `REAL` |
| `decimal` | `decimal` | `NUMERIC` |
| `boolean` | `boolean` | `BOOLEAN` |
| `datetime` | `datetime` | `TEXT` |
| `binary` | `binary` | `BLOB` |
| `uuid` | `uuid` | `TEXT` |
| `reference` | `reference` | `INTEGER` |
| `string` | `new SpecField { Type = "string" }` | `TEXT` |
| `string` | `new SpecField { Type = "string", Size = "unlimited" }` | `TEXT` |
| `string` | `new SpecField { Type = "string", Size = 64 }` | `VARCHAR(64)` |
| `nope` | `nope` | `InvalidOperationException` |
| `string` | `TEXT` | `string` |
| `integer` | `INTEGER` | `integer` |
| `nope` | `nope` | `InvalidOperationException` |

#### rowTypeConverters

| Type | From | To |
|---|---|---|
| `boolean` | `1L` | `true` |
| `boolean` | `0L` | `false` |
| `datetime` | `"2020-01-02T03:04:05.000Z"` | `new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc)` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000").ToByteArray()` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `7L` | `"7"` |
| `decimal` | `10.5d` | `"10.5"` |
| `decimal` | `"10.50"` | `"10.5"` |
| `decimal` | `0L` | `"0"` |
| `decimal` | `"0.00"` | `"0"` |
| `integer` | `"10"` | `10L` |
| `integer` | `"0"` | `0L` |
| `integer` | `""` | `FormatException` |
| `integer` | `"1.0"` | `FormatException` |
| `boolean` | `true` | `1L` |
| `boolean` | `false` | `0L` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `10.5m` | `"10.5"` |

#### defaultValueConverter

| Column type | From | To |
|---|---|---|
| `datetime` | `Now` | `(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))` |
| `datetime` | `UtcNow` | `(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))` |
| `datetime` | `DateTime('2020-01-01T00:00:00Z')` | `'2020-01-01T00:00:00Z'` |
| `uuid` | `Empty` | `'00000000-0000-0000-0000-000000000000'` |
| `uuid` | `uuid('11111111-1111-1111-1111-111111111111')` | `'11111111-1111-1111-1111-111111111111'` |
| `binary` | `Hex('ab')` | `X'ab'` |
| `boolean` | `true` | `1` |
| `boolean` | `false` | `0` |
| `string` | `hi` | `'hi'` |
| `integer` | `7` | `7` |
| `reference` | `1` | `null` |
| `nope` | `x` | `InvalidOperationException` |
| `datetime` | `2020-01-01T00:00:00Z` | `InvalidOperationException` |
| `datetime` | `(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))` | `Now` |
| `datetime` | `(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))` | `UtcNow` |
| `datetime` | `'2020-01-01T00:00:00Z'` | `DateTime('2020-01-01T00:00:00Z')` |
| `binary` | `X'ab'` | `Hex('ab')` |
| `string` | `'hi'` | `String` |
| `string` | `oops` | `InvalidOperationException` |

#### changeLogs

_None._

### MySQL {#typescript-mysql}

<a href="#typescript" class="driver-back">← TypeScript drivers</a>

#### information

- MySQL / MariaDB server driver.
- Full connection, command, transaction, and repository support.

#### Overrides

- Overrides SelectInsertedIdStatement to SELECT LAST_INSERT_ID() AS {column}
- Overrides CreateTableStatement identity to AUTO_INCREMENT
- Overrides CreateTableStatement type mapping (int→INT, uuid→CHAR(36), string/datetime→TEXT, bool→TINYINT, decimal→DECIMAL(18,4), float→DOUBLE, binary→BLOB)

#### typeConverter

| Type | From | To |
|---|---|---|
| `string` | `string` | `TEXT` |
| `character` | `character` | `CHAR(1)` |
| `integer` | `integer` | `INT` |
| `unsignedinteger` | `unsignedinteger` | `INT UNSIGNED` |
| `smallinteger` | `smallinteger` | `SMALLINT` |
| `unsignedsmallinteger` | `unsignedsmallinteger` | `SMALLINT UNSIGNED` |
| `biginteger` | `biginteger` | `BIGINT` |
| `unsignedbiginteger` | `unsignedbiginteger` | `BIGINT UNSIGNED` |
| `float` | `float` | `DOUBLE` |
| `decimal` | `decimal` | `DECIMAL(10, 2)` |
| `boolean` | `boolean` | `TINYINT(1)` |
| `datetime` | `datetime` | `DATETIME(3)` |
| `binary` | `binary` | `BLOB` |
| `uuid` | `uuid` | `CHAR(36)` |
| `reference` | `reference` | `INT` |
| `string` | `new SpecField { Type = "string" }` | `TEXT` |
| `string` | `new SpecField { Type = "string", Size = 64 }` | `VARCHAR(64)` |
| `string` | `new SpecField { Type = "string", Size = 20000 }` | `LONGTEXT` |
| `character` | `new SpecField { Type = "character", Size = 4 }` | `CHAR(4)` |
| `decimal` | `new SpecField { Type = "decimal", Size = new[] { 8, 3 } }` | `DECIMAL(8, 3)` |
| `binary` | `new SpecField { Type = "binary", Size = 16 }` | `VARBINARY(16)` |
| `binary` | `new SpecField { Type = "binary", Size = 70000 }` | `LONGBLOB` |
| `nope` | `nope` | `InvalidOperationException` |
| `decimal` | `new SpecField { Name = "amount", Type = "decimal", Size = 8 }` | `InvalidOperationException` |
| `integer` | `INT` | `integer` |
| `nope` | `nope` | `InvalidOperationException` |

#### rowTypeConverters

| Type | From | To |
|---|---|---|
| `boolean` | `1L` | `true` |
| `boolean` | `0L` | `false` |
| `datetime` | `"2020-01-02T03:04:05.000Z"` | `new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc)` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000").ToByteArray()` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `7L` | `"7"` |
| `decimal` | `10.5d` | `"10.5"` |
| `decimal` | `"10.50"` | `"10.5"` |
| `decimal` | `0L` | `"0"` |
| `decimal` | `"0.00"` | `"0"` |
| `binary` | `"AQIDBA=="` | `new byte[] { 1, 2, 3, 4 }` |
| `integer` | `"10"` | `10L` |
| `integer` | `"0"` | `0L` |
| `integer` | `"1.0"` | `FormatException` |
| `integer` | `""` | `FormatException` |
| `boolean` | `true` | `1L` |
| `boolean` | `false` | `0L` |
| `datetime` | `"2020-01-02T03:04:05.000Z"` | `"2020-01-02 03:04:05.000"` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `10.5m` | `"10.5"` |
| `binary` | `"AQIDBA=="` | `new byte[] { 1, 2, 3, 4 }` |

#### defaultValueConverter

| Column type | From | To |
|---|---|---|
| `datetime` | `Now` | `(NOW(3))` |
| `datetime` | `UtcNow` | `(UTC_TIMESTAMP(3))` |
| `uuid` | `NewId` | `(UUID())` |
| `binary` | `Hex('ab')` | `(X'ab')` |
| `boolean` | `true` | `1` |
| `boolean` | `false` | `0` |
| `string` | `hi` | `('hi')` |
| `integer` | `7` | `7` |
| `reference` | `1` | `null` |
| `nope` | `x` | `InvalidOperationException` |
| `binary` | `(X'ab')` | `Hex('ab')` |
| `binary` | `X'ab'` | `Hex('ab')` |
| `string` | `('hi')` | `String` |
| `string` | `'hi'` | `String` |
| `uuid` | `(UUID())` | `NewId` |
| `binary` | `X'ab')` | `InvalidOperationException` |
| `string` | `'hi')` | `InvalidOperationException` |

#### changeLogs

_None._

### PostgreSQL {#typescript-postgres}

<a href="#typescript" class="driver-back">← TypeScript drivers</a>

#### information

- PostgreSQL server driver.
- Full connection, command, transaction, and repository support.

#### Overrides

- Overrides SelectInsertedIdStatement to SELECT lastval() AS {column}
- Overrides CreateTableStatement identity to GENERATED BY DEFAULT AS IDENTITY
- Overrides CreateTableStatement boolean defaults (0/1 → TRUE/FALSE)
- Overrides CreateTableStatement type mapping (int→INTEGER, uuid→UUID, string→TEXT, bool→BOOLEAN, decimal→NUMERIC, float→DOUBLE PRECISION, datetime→TIMESTAMPTZ, binary→BYTEA)

#### typeConverter

| Type | From | To |
|---|---|---|
| `string` | `string` | `TEXT` |
| `character` | `character` | `CHAR(1)` |
| `integer` | `integer` | `INTEGER` |
| `unsignedinteger` | `unsignedinteger` | `BIGINT` |
| `smallinteger` | `smallinteger` | `SMALLINT` |
| `unsignedsmallinteger` | `unsignedsmallinteger` | `INTEGER` |
| `biginteger` | `biginteger` | `BIGINT` |
| `unsignedbiginteger` | `unsignedbiginteger` | `NUMERIC(20)` |
| `float` | `float` | `DOUBLE PRECISION` |
| `decimal` | `decimal` | `NUMERIC` |
| `boolean` | `boolean` | `BOOLEAN` |
| `datetime` | `datetime` | `TIMESTAMPTZ` |
| `binary` | `binary` | `BYTEA` |
| `uuid` | `uuid` | `UUID` |
| `reference` | `reference` | `INTEGER` |
| `string` | `new SpecField { Type = "string", Size = 64 }` | `VARCHAR(64)` |
| `character` | `new SpecField { Type = "character", Size = 4 }` | `CHAR(4)` |
| `decimal` | `new SpecField { Type = "decimal", Size = new[] { 10, 2 } }` | `NUMERIC(10, 2)` |
| `nope` | `nope` | `InvalidOperationException` |
| `integer` | `INTEGER` | `integer` |
| `unsignedinteger` | `BIGINT` | `unsignedinteger` |
| `nope` | `nope` | `InvalidOperationException` |

#### rowTypeConverters

| Type | From | To |
|---|---|---|
| `boolean` | `1L` | `true` |
| `boolean` | `0L` | `false` |
| `datetime` | `"2020-01-02T03:04:05.000Z"` | `new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc)` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000").ToByteArray()` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `7L` | `"7"` |
| `decimal` | `10.5d` | `"10.5"` |
| `decimal` | `"10.50"` | `"10.5"` |
| `decimal` | `0L` | `"0"` |
| `decimal` | `"0.00"` | `"0"` |
| `binary` | `"AQIDBA=="` | `new byte[] { 1, 2, 3, 4 }` |
| `integer` | `"10"` | `10L` |
| `integer` | `"0"` | `0L` |
| `integer` | `"1.0"` | `FormatException` |
| `integer` | `""` | `FormatException` |
| `uuid` | `Guid.Parse("550e8400-e29b-41d4-a716-446655440000")` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `10.5m` | `"10.5"` |
| `binary` | `"AQIDBA=="` | `new byte[] { 1, 2, 3, 4 }` |

#### defaultValueConverter

| Column type | From | To |
|---|---|---|
| `datetime` | `Now` | `LOCALTIMESTAMP` |
| `datetime` | `UtcNow` | `(NOW() AT TIME ZONE 'UTC')` |
| `uuid` | `NewId` | `gen_random_uuid()` |
| `uuid` | `Empty` | `'00000000-0000-0000-0000-000000000000'` |
| `binary` | `Hex('ab')` | `'\xab'` |
| `boolean` | `true` | `TRUE` |
| `boolean` | `false` | `FALSE` |
| `string` | `hi` | `'hi'` |
| `integer` | `7` | `7` |
| `reference` | `1` | `null` |
| `nope` | `x` | `InvalidOperationException` |
| `datetime` | `LOCALTIMESTAMP` | `Now` |
| `datetime` | `(NOW() AT TIME ZONE 'UTC')` | `UtcNow` |
| `uuid` | `gen_random_uuid()` | `NewId` |
| `binary` | `'\xab'` | `Hex('ab')` |
| `string` | `'hi'` | `String` |
| `datetime` | `nope` | `InvalidOperationException` |

#### changeLogs

_None._
