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

<div class="driver-conversions">

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
| `string` | `{ type: string }` | `TEXT` |
| `string` | `{ type: string, size: "unlimited" }` | `TEXT` |
| `string` | `{ type: string, size: 64 }` | `VARCHAR(64)` |
| `nope` | `nope` | `InvalidOperationException` |
| `string` | `TEXT` | `string` |
| `integer` | `INTEGER` | `integer` |
| `nope` | `nope` | `InvalidOperationException` |

</div>

#### RowTypeConverters

<div class="driver-conversions driver-conversions-cells">

| Type | From Type | From | To Type | To |
|---|---|---|---|---|
| `boolean` | `long` | `1` | `bool` | `true` |
| `boolean` | `long` | `0` | `bool` | `false` |
| `datetime` | `string` | `"2020-01-02T03:04:05.000Z"` | `DateTime` | `2020-01-02T03:04:05.000Z` |
| `datetime` | `string` | `"2020-01-02T03:04:05.000+00:00"` | `DateTime` | `2020-01-02T03:04:05.000Z` |
| `uuid` | `Guid` | `550e8400-e29b-41d4-a716-446655440000` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `byte[]` | `[0,132,14,85,155,226,212,65,167,22,68,102,85,68,0,0]` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `long` | `7` | `string` | `"7"` |
| `decimal` | `double` | `10.5` | `string` | `"10.5"` |
| `decimal` | `string` | `"10.50"` | `string` | `"10.5"` |
| `decimal` | `string` | `"10.50000000000000"` | `string` | `"10.5"` |
| `decimal` | `long` | `0` | `string` | `"0"` |
| `decimal` | `string` | `"0.00"` | `string` | `"0"` |
| `integer` | `string` | `"10"` | `long` | `10` |
| `integer` | `string` | `"0"` | `long` | `0` |
| `integer` | `double` | `10` | `long` | `10` |
| `integer` | `string` | `""` | `FormatException` | `FormatException` |
| `integer` | `string` | `"1.0"` | `long` | `1` |
| `integer` | `string` | `"10.0"` | `long` | `10` |
| `integer` | `string` | `"10.00000000000000"` | `long` | `10` |
| `integer` | `string` | `"10.50000000000000"` | `FormatException` | `FormatException` |
| `boolean` | `bool` | `true` | `long` | `1` |
| `boolean` | `bool` | `false` | `long` | `0` |
| `uuid` | `Guid` | `550e8400-e29b-41d4-a716-446655440000` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `decimal` | `10.5` | `string` | `"10.5"` |

</div>

#### DefaultValueConverter

<div class="driver-conversions">

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

</div>

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

<div class="driver-conversions">

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
| `string` | `{ type: string }` | `TEXT` |
| `string` | `{ type: string, size: 64 }` | `VARCHAR(64)` |
| `string` | `{ type: string, size: 20000 }` | `LONGTEXT` |
| `character` | `{ type: character, size: 4 }` | `CHAR(4)` |
| `decimal` | `{ type: decimal, size: [8,3] }` | `DECIMAL(8, 3)` |
| `binary` | `{ type: binary, size: 16 }` | `VARBINARY(16)` |
| `binary` | `{ type: binary, size: 70000 }` | `LONGBLOB` |
| `nope` | `nope` | `InvalidOperationException` |
| `decimal` | `{ type: decimal, name: amount, size: 8 }` | `InvalidOperationException` |
| `integer` | `INT` | `integer` |
| `nope` | `nope` | `InvalidOperationException` |

</div>

#### RowTypeConverters

<div class="driver-conversions driver-conversions-cells">

| Type | From Type | From | To Type | To |
|---|---|---|---|---|
| `boolean` | `long` | `1` | `bool` | `true` |
| `boolean` | `long` | `0` | `bool` | `false` |
| `datetime` | `string` | `"2020-01-02T03:04:05.000Z"` | `DateTime` | `2020-01-02T03:04:05.000Z` |
| `datetime` | `string` | `"2020-01-02T03:04:05.000+00:00"` | `DateTime` | `2020-01-02T03:04:05.000Z` |
| `uuid` | `Guid` | `550e8400-e29b-41d4-a716-446655440000` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `byte[]` | `[0,132,14,85,155,226,212,65,167,22,68,102,85,68,0,0]` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `long` | `7` | `string` | `"7"` |
| `decimal` | `double` | `10.5` | `string` | `"10.5"` |
| `decimal` | `string` | `"10.50"` | `string` | `"10.5"` |
| `decimal` | `string` | `"10.50000000000000"` | `string` | `"10.5"` |
| `decimal` | `long` | `0` | `string` | `"0"` |
| `decimal` | `string` | `"0.00"` | `string` | `"0"` |
| `integer` | `string` | `"10"` | `long` | `10` |
| `integer` | `string` | `"0"` | `long` | `0` |
| `integer` | `double` | `10` | `long` | `10` |
| `integer` | `string` | `""` | `FormatException` | `FormatException` |
| `integer` | `string` | `"1.0"` | `long` | `1` |
| `integer` | `string` | `"10.0"` | `long` | `10` |
| `integer` | `string` | `"10.00000000000000"` | `long` | `10` |
| `integer` | `string` | `"10.50000000000000"` | `FormatException` | `FormatException` |
| `boolean` | `bool` | `true` | `long` | `1` |
| `boolean` | `bool` | `false` | `long` | `0` |
| `uuid` | `Guid` | `550e8400-e29b-41d4-a716-446655440000` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `decimal` | `10.5` | `string` | `"10.5"` |
| `binary` | `string` | `AQIDBA==` | `byte[]` | `[1,2,3,4]` |
| `binary` | `string` | `latin1(1, 2, 3, 4)` | `byte[]` | `[1,2,3,4]` |
| `datetime` | `string` | `"2020-01-02T03:04:05.000Z"` | `string` | `"2020-01-02 03:04:05.000"` |
| `binary` | `string` | `AQIDBA==` | `byte[]` | `[1,2,3,4]` |

</div>

#### DefaultValueConverter

<div class="driver-conversions">

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

</div>

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

<div class="driver-conversions">

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
| `string` | `{ type: string, size: 64 }` | `VARCHAR(64)` |
| `character` | `{ type: character, size: 4 }` | `CHAR(4)` |
| `decimal` | `{ type: decimal, size: [10,2] }` | `NUMERIC(10, 2)` |
| `nope` | `nope` | `InvalidOperationException` |
| `integer` | `INTEGER` | `integer` |
| `unsignedinteger` | `BIGINT` | `unsignedinteger` |
| `nope` | `nope` | `InvalidOperationException` |

</div>

#### RowTypeConverters

<div class="driver-conversions driver-conversions-cells">

| Type | From Type | From | To Type | To |
|---|---|---|---|---|
| `boolean` | `long` | `1` | `bool` | `true` |
| `boolean` | `long` | `0` | `bool` | `false` |
| `datetime` | `string` | `"2020-01-02T03:04:05.000Z"` | `DateTime` | `2020-01-02T03:04:05.000Z` |
| `datetime` | `string` | `"2020-01-02T03:04:05.000+00:00"` | `DateTime` | `2020-01-02T03:04:05.000Z` |
| `uuid` | `Guid` | `550e8400-e29b-41d4-a716-446655440000` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `byte[]` | `[0,132,14,85,155,226,212,65,167,22,68,102,85,68,0,0]` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `long` | `7` | `string` | `"7"` |
| `decimal` | `double` | `10.5` | `string` | `"10.5"` |
| `decimal` | `string` | `"10.50"` | `string` | `"10.5"` |
| `decimal` | `string` | `"10.50000000000000"` | `string` | `"10.5"` |
| `decimal` | `long` | `0` | `string` | `"0"` |
| `decimal` | `string` | `"0.00"` | `string` | `"0"` |
| `integer` | `string` | `"10"` | `long` | `10` |
| `integer` | `string` | `"0"` | `long` | `0` |
| `integer` | `double` | `10` | `long` | `10` |
| `integer` | `string` | `""` | `FormatException` | `FormatException` |
| `integer` | `string` | `"1.0"` | `long` | `1` |
| `integer` | `string` | `"10.0"` | `long` | `10` |
| `integer` | `string` | `"10.00000000000000"` | `long` | `10` |
| `integer` | `string` | `"10.50000000000000"` | `FormatException` | `FormatException` |
| `boolean` | `bool` | `true` | `long` | `1` |
| `boolean` | `bool` | `false` | `long` | `0` |
| `uuid` | `Guid` | `550e8400-e29b-41d4-a716-446655440000` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `decimal` | `10.5` | `string` | `"10.5"` |
| `binary` | `string` | `AQIDBA==` | `byte[]` | `[1,2,3,4]` |
| `binary` | `string` | `AQIDBA==` | `byte[]` | `[1,2,3,4]` |

</div>

#### DefaultValueConverter

<div class="driver-conversions">

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

</div>

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

<div class="driver-conversions">

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
| `string` | `{ type: string }` | `TEXT` |
| `string` | `{ type: string, size: "unlimited" }` | `TEXT` |
| `string` | `{ type: string, size: 64 }` | `VARCHAR(64)` |
| `nope` | `nope` | `error` |
| `float` | `REAL` | `float` |
| `nope` | `nope` | `error` |

</div>

#### row_type_converters

<div class="driver-conversions driver-conversions-cells">

| Type | From Type | From | To Type | To |
|---|---|---|---|---|
| `boolean` | `i64` | `1` | `bool` | `true` |
| `boolean` | `i64` | `0` | `bool` | `false` |
| `datetime` | `String` | `"2020-01-02T03:04:05.000Z"` | `DateTime<Utc>` | `2020-01-02T03:04:05.000Z` |
| `datetime` | `String` | `"2020-01-02T03:04:05.000+00:00"` | `DateTime<Utc>` | `2020-01-02T03:04:05.000Z` |
| `uuid` | `Uuid` | `550e8400-e29b-41d4-a716-446655440000` | `String` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `Vec<u8>` | `[85,14,132,0,226,155,65,212,167,22,68,102,85,68,0,0]` | `String` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `i64` | `7` | `String` | `"7"` |
| `decimal` | `f64` | `10.5` | `String` | `"10.5"` |
| `decimal` | `String` | `"10.50"` | `String` | `"10.5"` |
| `decimal` | `String` | `"10.50000000000000"` | `String` | `"10.5"` |
| `decimal` | `i64` | `0` | `String` | `"0"` |
| `decimal` | `String` | `"0.00"` | `String` | `"0"` |
| `integer` | `String` | `"10"` | `i64` | `10` |
| `integer` | `String` | `"0"` | `i64` | `0` |
| `integer` | `f64` | `10` | `i64` | `10` |
| `integer` | `String` | `""` | `error` | `error` |
| `integer` | `String` | `"1.0"` | `i64` | `1` |
| `integer` | `String` | `"10.0"` | `i64` | `10` |
| `integer` | `String` | `"10.00000000000000"` | `i64` | `10` |
| `integer` | `String` | `"10.50000000000000"` | `error` | `error` |
| `boolean` | `bool` | `true` | `i64` | `1` |
| `boolean` | `bool` | `false` | `i64` | `0` |
| `uuid` | `Uuid` | `550e8400-e29b-41d4-a716-446655440000` | `String` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `Decimal` | `10.5` | `String` | `"10.5"` |

</div>

#### default_value_converter

<div class="driver-conversions">

| Column type | From | To |
|---|---|---|
| `datetime` | `Now` | `(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))` |
| `datetime` | `UtcNow` | `(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))` |
| `binary` | `Hex('ab')` | `X'ab'` |
| `boolean` | `true` | `1` |
| `boolean` | `false` | `0` |
| `string` | `hi` | `'hi'` |
| `nope` | `x` | `error` |
| `datetime` | `(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))` | `Now` |
| `datetime` | `(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))` | `UtcNow` |
| `binary` | `X'ab'` | `Hex('ab')` |
| `string` | `'hi'` | `hi` |
| `string` | `oops` | `error` |

</div>

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

<div class="driver-conversions">

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
| `string` | `{ type: string }` | `TEXT` |
| `string` | `{ type: string, size: 64 }` | `VARCHAR(64)` |
| `string` | `{ type: string, size: 20000 }` | `LONGTEXT` |
| `character` | `{ type: character, size: 4 }` | `CHAR(4)` |
| `decimal` | `{ type: decimal, size: [8,3] }` | `DECIMAL(8, 3)` |
| `binary` | `{ type: binary, size: 16 }` | `VARBINARY(16)` |
| `binary` | `{ type: binary, size: 70000 }` | `LONGBLOB` |
| `nope` | `nope` | `error` |
| `decimal` | `{ type: decimal, name: amount, size: 8 }` | `error` |
| `integer` | `INT` | `integer` |
| `nope` | `nope` | `error` |

</div>

#### row_type_converters

<div class="driver-conversions driver-conversions-cells">

| Type | From Type | From | To Type | To |
|---|---|---|---|---|
| `boolean` | `i64` | `1` | `bool` | `true` |
| `boolean` | `i64` | `0` | `bool` | `false` |
| `datetime` | `String` | `"2020-01-02T03:04:05.000Z"` | `DateTime<Utc>` | `2020-01-02T03:04:05.000Z` |
| `datetime` | `String` | `"2020-01-02T03:04:05.000+00:00"` | `DateTime<Utc>` | `2020-01-02T03:04:05.000Z` |
| `uuid` | `Uuid` | `550e8400-e29b-41d4-a716-446655440000` | `String` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `Vec<u8>` | `[85,14,132,0,226,155,65,212,167,22,68,102,85,68,0,0]` | `String` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `i64` | `7` | `String` | `"7"` |
| `decimal` | `f64` | `10.5` | `String` | `"10.5"` |
| `decimal` | `String` | `"10.50"` | `String` | `"10.5"` |
| `decimal` | `String` | `"10.50000000000000"` | `String` | `"10.5"` |
| `decimal` | `i64` | `0` | `String` | `"0"` |
| `decimal` | `String` | `"0.00"` | `String` | `"0"` |
| `integer` | `String` | `"10"` | `i64` | `10` |
| `integer` | `String` | `"0"` | `i64` | `0` |
| `integer` | `f64` | `10` | `i64` | `10` |
| `integer` | `String` | `""` | `error` | `error` |
| `integer` | `String` | `"1.0"` | `i64` | `1` |
| `integer` | `String` | `"10.0"` | `i64` | `10` |
| `integer` | `String` | `"10.00000000000000"` | `i64` | `10` |
| `integer` | `String` | `"10.50000000000000"` | `error` | `error` |
| `boolean` | `bool` | `true` | `i64` | `1` |
| `boolean` | `bool` | `false` | `i64` | `0` |
| `uuid` | `Uuid` | `550e8400-e29b-41d4-a716-446655440000` | `String` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `Decimal` | `10.5` | `String` | `"10.5"` |
| `binary` | `String` | `AQIDBA==` | `Vec<u8>` | `[1,2,3,4]` |
| `binary` | `String` | `latin1(1, 2, 3, 4)` | `Vec<u8>` | `[1,2,3,4]` |
| `datetime` | `String` | `"2020-01-02T03:04:05.000Z"` | `String` | `"2020-01-02 03:04:05.000"` |
| `binary` | `String` | `AQIDBA==` | `Vec<u8>` | `[1,2,3,4]` |

</div>

#### default_value_converter

<div class="driver-conversions">

| Column type | From | To |
|---|---|---|
| `datetime` | `Now` | `(NOW(3))` |
| `datetime` | `UtcNow` | `(UTC_TIMESTAMP(3))` |
| `uuid` | `NewId` | `(UUID())` |
| `binary` | `Hex('ab')` | `(X'ab')` |
| `boolean` | `true` | `1` |
| `boolean` | `false` | `0` |
| `string` | `hi` | `('hi')` |
| `nope` | `x` | `error` |
| `binary` | `(X'ab')` | `Hex('ab')` |
| `binary` | `X'ab'` | `Hex('ab')` |
| `string` | `('hi')` | `hi` |
| `string` | `'hi'` | `hi` |
| `uuid` | `(UUID())` | `NewId` |
| `binary` | `X'ab')` | `error` |
| `string` | `'hi')` | `error` |

</div>

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

<div class="driver-conversions">

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
| `string` | `{ type: string, size: 64 }` | `VARCHAR(64)` |
| `character` | `{ type: character, size: 4 }` | `CHAR(4)` |
| `decimal` | `{ type: decimal, size: [10,2] }` | `NUMERIC(10, 2)` |
| `nope` | `nope` | `error` |
| `datetime` | `TIMESTAMPTZ` | `datetime` |
| `uuid` | `UUID` | `uuid` |
| `nope` | `nope` | `error` |

</div>

#### row_type_converters

<div class="driver-conversions driver-conversions-cells">

| Type | From Type | From | To Type | To |
|---|---|---|---|---|
| `boolean` | `i64` | `1` | `bool` | `true` |
| `boolean` | `i64` | `0` | `bool` | `false` |
| `datetime` | `String` | `"2020-01-02T03:04:05.000Z"` | `DateTime<Utc>` | `2020-01-02T03:04:05.000Z` |
| `datetime` | `String` | `"2020-01-02T03:04:05.000+00:00"` | `DateTime<Utc>` | `2020-01-02T03:04:05.000Z` |
| `uuid` | `Uuid` | `550e8400-e29b-41d4-a716-446655440000` | `String` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `Vec<u8>` | `[85,14,132,0,226,155,65,212,167,22,68,102,85,68,0,0]` | `String` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `i64` | `7` | `String` | `"7"` |
| `decimal` | `f64` | `10.5` | `String` | `"10.5"` |
| `decimal` | `String` | `"10.50"` | `String` | `"10.5"` |
| `decimal` | `String` | `"10.50000000000000"` | `String` | `"10.5"` |
| `decimal` | `i64` | `0` | `String` | `"0"` |
| `decimal` | `String` | `"0.00"` | `String` | `"0"` |
| `integer` | `String` | `"10"` | `i64` | `10` |
| `integer` | `String` | `"0"` | `i64` | `0` |
| `integer` | `f64` | `10` | `i64` | `10` |
| `integer` | `String` | `""` | `error` | `error` |
| `integer` | `String` | `"1.0"` | `i64` | `1` |
| `integer` | `String` | `"10.0"` | `i64` | `10` |
| `integer` | `String` | `"10.00000000000000"` | `i64` | `10` |
| `integer` | `String` | `"10.50000000000000"` | `error` | `error` |
| `boolean` | `bool` | `true` | `i64` | `1` |
| `boolean` | `bool` | `false` | `i64` | `0` |
| `uuid` | `Uuid` | `550e8400-e29b-41d4-a716-446655440000` | `String` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `Decimal` | `10.5` | `String` | `"10.5"` |
| `binary` | `String` | `AQIDBA==` | `Vec<u8>` | `[1,2,3,4]` |
| `binary` | `String` | `AQIDBA==` | `Vec<u8>` | `[1,2,3,4]` |

</div>

#### default_value_converter

<div class="driver-conversions">

| Column type | From | To |
|---|---|---|
| `datetime` | `Now` | `LOCALTIMESTAMP` |
| `datetime` | `UtcNow` | `(NOW() AT TIME ZONE 'UTC')` |
| `uuid` | `NewId` | `gen_random_uuid()` |
| `binary` | `Hex('ab')` | `'\xab'` |
| `boolean` | `true` | `TRUE` |
| `boolean` | `false` | `FALSE` |
| `string` | `hi` | `'hi'` |
| `nope` | `x` | `error` |
| `datetime` | `LOCALTIMESTAMP` | `Now` |
| `datetime` | `(NOW() AT TIME ZONE 'UTC')` | `UtcNow` |
| `uuid` | `gen_random_uuid()` | `NewId` |
| `binary` | `'\xab'` | `Hex('ab')` |
| `string` | `'hi'` | `hi` |
| `datetime` | `nope` | `error` |

</div>

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

<div class="driver-conversions">

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
| `string` | `{ type: string }` | `TEXT` |
| `string` | `{ type: string, size: "unlimited" }` | `TEXT` |
| `string` | `{ type: string, size: 64 }` | `VARCHAR(64)` |
| `nope` | `nope` | `Error` |
| `string` | `TEXT` | `string` |
| `integer` | `INTEGER` | `integer` |
| `nope` | `nope` | `Error` |

</div>

#### rowTypeConverters

<div class="driver-conversions driver-conversions-cells">

| Type | From Type | From | To Type | To |
|---|---|---|---|---|
| `boolean` | `number` | `1` | `boolean` | `true` |
| `boolean` | `number` | `0` | `boolean` | `false` |
| `datetime` | `string` | `"2020-01-02T03:04:05.000Z"` | `Date` | `2020-01-02T03:04:05.000Z` |
| `datetime` | `string` | `"2020-01-02T03:04:05.000+00:00"` | `Date` | `2020-01-02T03:04:05.000Z` |
| `uuid` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `Uint8Array` | `[85,14,132,0,226,155,65,212,167,22,68,102,85,68,0,0]` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `number` | `7` | `string` | `"7"` |
| `decimal` | `number` | `10.5` | `string` | `"10.5"` |
| `decimal` | `string` | `"10.50"` | `string` | `"10.5"` |
| `decimal` | `string` | `"10.50000000000000"` | `string` | `"10.5"` |
| `decimal` | `number` | `0` | `string` | `"0"` |
| `decimal` | `string` | `"0.00"` | `string` | `"0"` |
| `integer` | `string` | `"10"` | `number` | `10` |
| `integer` | `string` | `"0"` | `number` | `0` |
| `integer` | `number` | `10` | `number` | `10` |
| `integer` | `string` | `""` | `DatasourceError` | `DatasourceError` |
| `integer` | `string` | `"1.0"` | `number` | `1` |
| `integer` | `string` | `"10.0"` | `number` | `10` |
| `integer` | `string` | `"10.00000000000000"` | `number` | `10` |
| `integer` | `string` | `"10.50000000000000"` | `DatasourceError` | `DatasourceError` |
| `boolean` | `boolean` | `true` | `number` | `1` |
| `boolean` | `boolean` | `false` | `number` | `0` |
| `uuid` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `string` | `10.5` | `string` | `"10.5"` |

</div>

#### defaultValueConverter

<div class="driver-conversions">

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
| `nope` | `x` | `Error` |
| `datetime` | `2020-01-01T00:00:00Z` | `Error` |
| `datetime` | `(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))` | `Now` |
| `datetime` | `(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))` | `UtcNow` |
| `datetime` | `'2020-01-01T00:00:00Z'` | `DateTime('2020-01-01T00:00:00Z')` |
| `binary` | `X'ab'` | `Hex('ab')` |
| `string` | `'hi'` | `hi` |
| `string` | `oops` | `Error` |

</div>

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

<div class="driver-conversions">

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
| `string` | `{ type: string }` | `TEXT` |
| `string` | `{ type: string, size: 64 }` | `VARCHAR(64)` |
| `string` | `{ type: string, size: 20000 }` | `LONGTEXT` |
| `character` | `{ type: character, size: 4 }` | `CHAR(4)` |
| `decimal` | `{ type: decimal, size: [8,3] }` | `DECIMAL(8, 3)` |
| `binary` | `{ type: binary, size: 16 }` | `VARBINARY(16)` |
| `binary` | `{ type: binary, size: 70000 }` | `LONGBLOB` |
| `nope` | `nope` | `Error` |
| `decimal` | `{ type: decimal, name: amount, size: 8 }` | `Error` |
| `integer` | `INT` | `integer` |
| `nope` | `nope` | `Error` |

</div>

#### rowTypeConverters

<div class="driver-conversions driver-conversions-cells">

| Type | From Type | From | To Type | To |
|---|---|---|---|---|
| `boolean` | `number` | `1` | `boolean` | `true` |
| `boolean` | `number` | `0` | `boolean` | `false` |
| `datetime` | `string` | `"2020-01-02T03:04:05.000Z"` | `Date` | `2020-01-02T03:04:05.000Z` |
| `datetime` | `string` | `"2020-01-02T03:04:05.000+00:00"` | `Date` | `2020-01-02T03:04:05.000Z` |
| `uuid` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `Uint8Array` | `[85,14,132,0,226,155,65,212,167,22,68,102,85,68,0,0]` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `number` | `7` | `string` | `"7"` |
| `decimal` | `number` | `10.5` | `string` | `"10.5"` |
| `decimal` | `string` | `"10.50"` | `string` | `"10.5"` |
| `decimal` | `string` | `"10.50000000000000"` | `string` | `"10.5"` |
| `decimal` | `number` | `0` | `string` | `"0"` |
| `decimal` | `string` | `"0.00"` | `string` | `"0"` |
| `integer` | `string` | `"10"` | `number` | `10` |
| `integer` | `string` | `"0"` | `number` | `0` |
| `integer` | `number` | `10` | `number` | `10` |
| `integer` | `string` | `""` | `DatasourceError` | `DatasourceError` |
| `integer` | `string` | `"1.0"` | `number` | `1` |
| `integer` | `string` | `"10.0"` | `number` | `10` |
| `integer` | `string` | `"10.00000000000000"` | `number` | `10` |
| `integer` | `string` | `"10.50000000000000"` | `DatasourceError` | `DatasourceError` |
| `boolean` | `boolean` | `true` | `number` | `1` |
| `boolean` | `boolean` | `false` | `number` | `0` |
| `uuid` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `string` | `10.5` | `string` | `"10.5"` |
| `binary` | `string` | `AQIDBA==` | `Uint8Array` | `[1,2,3,4]` |
| `binary` | `string` | `latin1(1, 2, 3, 4)` | `Uint8Array` | `[1,2,3,4]` |
| `datetime` | `string` | `"2020-01-02T03:04:05.000Z"` | `string` | `"2020-01-02 03:04:05.000"` |
| `binary` | `string` | `AQIDBA==` | `Uint8Array` | `[1,2,3,4]` |

</div>

#### defaultValueConverter

<div class="driver-conversions">

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
| `nope` | `x` | `Error` |
| `binary` | `(X'ab')` | `Hex('ab')` |
| `binary` | `X'ab'` | `Hex('ab')` |
| `string` | `('hi')` | `hi` |
| `string` | `'hi'` | `hi` |
| `uuid` | `(UUID())` | `NewId` |
| `binary` | `X'ab')` | `Error` |
| `string` | `'hi')` | `Error` |

</div>

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

<div class="driver-conversions">

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
| `string` | `{ type: string, size: 64 }` | `VARCHAR(64)` |
| `character` | `{ type: character, size: 4 }` | `CHAR(4)` |
| `decimal` | `{ type: decimal, size: [10,2] }` | `NUMERIC(10, 2)` |
| `nope` | `nope` | `Error` |
| `integer` | `INTEGER` | `integer` |
| `unsignedinteger` | `BIGINT` | `unsignedinteger` |
| `nope` | `nope` | `Error` |

</div>

#### rowTypeConverters

<div class="driver-conversions driver-conversions-cells">

| Type | From Type | From | To Type | To |
|---|---|---|---|---|
| `boolean` | `number` | `1` | `boolean` | `true` |
| `boolean` | `number` | `0` | `boolean` | `false` |
| `datetime` | `string` | `"2020-01-02T03:04:05.000Z"` | `Date` | `2020-01-02T03:04:05.000Z` |
| `datetime` | `string` | `"2020-01-02T03:04:05.000+00:00"` | `Date` | `2020-01-02T03:04:05.000Z` |
| `uuid` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `uuid` | `Uint8Array` | `[85,14,132,0,226,155,65,212,167,22,68,102,85,68,0,0]` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `number` | `7` | `string` | `"7"` |
| `decimal` | `number` | `10.5` | `string` | `"10.5"` |
| `decimal` | `string` | `"10.50"` | `string` | `"10.5"` |
| `decimal` | `string` | `"10.50000000000000"` | `string` | `"10.5"` |
| `decimal` | `number` | `0` | `string` | `"0"` |
| `decimal` | `string` | `"0.00"` | `string` | `"0"` |
| `integer` | `string` | `"10"` | `number` | `10` |
| `integer` | `string` | `"0"` | `number` | `0` |
| `integer` | `number` | `10` | `number` | `10` |
| `integer` | `string` | `""` | `DatasourceError` | `DatasourceError` |
| `integer` | `string` | `"1.0"` | `number` | `1` |
| `integer` | `string` | `"10.0"` | `number` | `10` |
| `integer` | `string` | `"10.00000000000000"` | `number` | `10` |
| `integer` | `string` | `"10.50000000000000"` | `DatasourceError` | `DatasourceError` |
| `boolean` | `boolean` | `true` | `number` | `1` |
| `boolean` | `boolean` | `false` | `number` | `0` |
| `uuid` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` | `string` | `"550e8400-e29b-41d4-a716-446655440000"` |
| `decimal` | `string` | `10.5` | `string` | `"10.5"` |
| `binary` | `string` | `AQIDBA==` | `Uint8Array` | `[1,2,3,4]` |
| `binary` | `string` | `AQIDBA==` | `Uint8Array` | `[1,2,3,4]` |

</div>

#### defaultValueConverter

<div class="driver-conversions">

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
| `nope` | `x` | `Error` |
| `datetime` | `LOCALTIMESTAMP` | `Now` |
| `datetime` | `(NOW() AT TIME ZONE 'UTC')` | `UtcNow` |
| `uuid` | `gen_random_uuid()` | `NewId` |
| `binary` | `'\xab'` | `Hex('ab')` |
| `string` | `'hi'` | `hi` |
| `datetime` | `nope` | `Error` |

</div>

#### changeLogs

_None._
