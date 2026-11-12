# Writing Providers

A provider package implements {{IDriver|IDriver|Driver}} and exports only that type. {{DatasourceFactory.Register|DatasourceFactory.register|register}} stores it by name. The driver builds the connection, command, and transaction, and supplies row cell converters when native values are not already canonical cells.

Every provider names the type `*Driver` (`JsonDriver`, `SqliteDriver`).

## Classes

::: reference
### typescript
#### IDriver

```typescript
interface IDriver
```

**Methods**

##### connection(options?)

Builds an [IConnection](./sections/connection.md) from the options bag.

```typescript
connection(options?: DatasourceOptions): IConnection
```

- `options` — [DatasourceOptions](./sections/datasource.md#datasourceoptions)

**Returns** [IConnection](./sections/connection.md)

##### command(connection)

Builds an [ICommand](./sections/command.md) for the connection.

```typescript
command(connection: IConnection): ICommand
```

- `connection` — open or closed [IConnection](./sections/connection.md)

**Returns** [ICommand](./sections/command.md)

##### transaction(connection, isolation?)

Builds an [ITransaction](./sections/transaction.md) for the connection.

```typescript
transaction(connection: IConnection, isolation?: IsolationLevel): ITransaction
```

- `connection` — open or closed [IConnection](./sections/connection.md)
- `isolation` — defaults to `readCommitted`

**Returns** [ITransaction](./sections/transaction.md)

##### rowTypeConverters

Row cell converters keyed by canonical type name (`bool`, `datetime`, …). Empty for stubs and file dialects.

```typescript
rowTypeConverters: Record<string, TypeConverter<Value, Value>>
```

**Returns** `Record<string, TypeConverter<Value, Value>>`

##### columnType(name)

SQL type name for a canonical column type (`int`, `string`, `bool`, …). The default, exported as `columnType`, returns `name`. MySQL, Postgres, and SQLite override it. An unknown name throws.

```typescript
columnType(name: string): string
```

- `name` — canonical type name

**Returns** `string`

##### typeConverter

Spec type name to native SQL column type. SQL drivers set a `DeterministicTypeConverter`. Other drivers omit it.

```typescript
typeConverter?: IMapping<string, string> | null
```

**Returns** `IMapping<string, string> | null | undefined`

##### defaultValueConverter

Spec default token (`UtcNow`, `NewId`, …) to native SQL. Required. The default is `emptyDefaultValueConverter` (`convert` returns `null`; `reverseConvert` throws). SQL drivers set a dialect converter. TypeScript assigns the exported `emptyDefaultValueConverter` unless the driver overrides it.

```typescript
defaultValueConverter: IDefaultValueConverter
```

**Returns** `IDefaultValueConverter`

##### information

Optional provider notes: descriptions, overrides, and change logs. Omitted or `null` when the driver has none.

```typescript
information?: Information | null
```

**Returns** `Information | null | undefined`

```typescript
type Information = {
  descriptions: readonly string[];
  overrides: readonly string[];
  changeLogs: readonly string[];
}
```

### csharp
#### IDriver

```csharp
public interface IDriver
```

**Methods**

##### Connection(options)

Builds an [IConnection](./sections/connection.md) from the options bag.

```csharp
IConnection Connection(IReadOnlyDictionary<string, string?>? options)
```

- `options` — [DatasourceOptions](./sections/datasource.md#datasourceoptions)

**Returns** [IConnection](./sections/connection.md)

##### Command(connection)

Builds an [ICommand](./sections/command.md) for the connection.

```csharp
ICommand Command(IConnection connection)
```

- `connection` — open or closed [IConnection](./sections/connection.md)

**Returns** [ICommand](./sections/command.md)

##### Transaction(connection, isolation)

Builds an [ITransaction](./sections/transaction.md) for the connection.

```csharp
ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.ReadCommitted)
```

- `connection` — open or closed [IConnection](./sections/connection.md)
- `isolation` — defaults to `ReadCommitted`

**Returns** [ITransaction](./sections/transaction.md)

##### RowTypeConverters

Row cell converters keyed by canonical type name (`bool`, `datetime`, …). Empty for stubs and file dialects.

```csharp
IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters { get; }
```

**Returns** `IReadOnlyDictionary<string, TypeConverter<Value, Value>>`

##### ColumnType(name)

SQL type name for a canonical column type (`int`, `string`, `bool`, …). The default returns `name`. MySQL, Postgres, and SQLite override it. An unknown name throws.

```csharp
string ColumnType(string name)
```

- `name` — canonical type name

**Returns** `string`

##### TypeConverter

Spec type name to native SQL column type. The default returns `null`. SQL drivers set a `DeterministicTypeConverter`.

```csharp
IMapping<string, string>? TypeConverter
```

**Returns** `IMapping<string, string>?`

##### DefaultValueConverter

Spec default token (`UtcNow`, `NewId`, …) to native SQL. Required. The default is `EmptyDefaultValueConverter.Instance` (`Convert` returns `null`; `ReverseConvert` throws). SQL drivers override with a dialect converter.

```csharp
IDefaultValueConverter DefaultValueConverter
```

**Returns** `IDefaultValueConverter`

##### Information

Optional provider notes: descriptions, overrides, and change logs. The default returns `null`.

```csharp
Information? Information
```

**Returns** `Information?`

```csharp
public sealed record Information(
    IReadOnlyList<string> Descriptions,
    IReadOnlyList<string> Overrides,
    IReadOnlyList<string> ChangeLogs);
```

### rust
#### Driver

```rust
pub trait Driver
```

**Methods**

##### connection(options)

Builds a [Connection](./sections/connection.md) from the options bag.

```rust
fn connection(&self, options: Option<&HashMap<String, String>>) -> Arc<dyn Connection>
```

- `options` — [DatasourceOptions](./sections/datasource.md#datasourceoptions)

**Returns** `Arc<dyn `[Connection](./sections/connection.md)`>`

##### command(connection)

Builds a [Command](./sections/command.md) for the connection.

```rust
fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command>
```

- `connection` — open or closed [Connection](./sections/connection.md)

**Returns** `Arc<dyn `[Command](./sections/command.md)`>`

##### transaction(connection, isolation)

Builds a [Transaction](./sections/transaction.md) for the connection.

```rust
fn transaction(&self, connection: Arc<dyn Connection>, isolation: IsolationLevel) -> Arc<dyn Transaction>
```

- `connection` — open or closed [Connection](./sections/connection.md)
- `isolation` — defaults to `ReadCommitted` at the call site

**Returns** `Arc<dyn `[Transaction](./sections/transaction.md)`>`

##### row_type_converters()

Row cell converters keyed by canonical type name (`bool`, `datetime`, …). Empty for stubs and file dialects.

```rust
fn row_type_converters(&self) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>
```

**Returns** `HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>`

##### column_type(name)

SQL type name for a canonical column type (`int`, `string`, `bool`, …). The default returns `name`. MySQL, Postgres, and SQLite override it. An unknown name returns an error.

```rust
fn column_type(&self, name: &str) -> Result<String>
```

- `name` — canonical type name

**Returns** `Result<String>`

##### type_converter()

Spec type name to native SQL column type. The default returns `None`. SQL drivers set a `DeterministicTypeConverter`.

```rust
fn type_converter(&self) -> Option<&dyn Mapping<String, String>>
```

**Returns** `Option<&dyn Mapping<String, String>>`

##### default_value_converter()

Spec default token (`UtcNow`, `NewId`, …) to native SQL. Required. The default is `EMPTY_DEFAULT_VALUE_CONVERTER` (`convert` returns `Ok(None)`; `reverse_convert` returns an error). SQL drivers override with a dialect converter.

```rust
fn default_value_converter(&self) -> &dyn DefaultValueConverter
```

**Returns** `&dyn DefaultValueConverter`

##### information()

Optional provider notes: descriptions, overrides, and change logs. The default returns `None`.

```rust
fn information(&self) -> Option<&Information>
```

**Returns** `Option<&Information>`

```rust
pub struct Information {
    pub descriptions: Vec<String>,
    pub overrides: Vec<String>,
    pub change_logs: Vec<String>,
}
```
:::

## Remarks

{{Connection|connection|connection}} receives the options bag. A provider parses `DATASOURCE_URL` with [DatasourceUrlParser](./sections/datasource-url.md). {{Command|command|command}} delegates to the connection's {{CreateCommand|createCommand|create_command}}. {{Transaction|transaction|transaction}} builds the dialect transaction. Isolation defaults to read committed.

{{RowTypeConverters|rowTypeConverters|row_type_converters}} maps a canonical type name to a [TypeConverter](./sections/type-converters.md). SQL dialects register `bool`, `datetime`, `uuid`, `decimal`, and `binary` when the engine cannot return those as canonical cells. Memory, document, and file providers return an empty map.

{{Information|information|information}} is optional metadata a driver may publish. Each list is free-form text: {{Descriptions|descriptions|descriptions}} for capability notes, {{Overrides|overrides|overrides}} for dialect-specific behavior, and {{ChangeLogs|changeLogs|change_logs}} for provider history. The default is unset.

Published drivers list that metadata and C# From/To conversion tables per language on [Languages and Datasource Providers](./drivers.md). Edit `docs/DriverSummary.yaml` for {{Information|information|information}}, Overrides, and ChangeLogs. Conversion tables come from C# converter tests via `scripts/generate-driver-conversions.mjs`. Run `npm run docs:drivers`.

The statement factory formats `SelectInsertedIdStatement` as a `SelectStatement`, and the repository reads that row. Commands do not sniff SQL for the inserted id. {{SelectInsertedIdStatementTextRenderer.Render|SelectInsertedIdStatementTextRenderer.render|SelectInsertedIdStatementTextRenderer::render}} throws until a provider overrides it.

{{ColumnType|columnType|column_type}} maps a canonical column type to the engine SQL type name. It does not convert cell values. The default returns the name unchanged. MySQL, Postgres, and SQLite override it and throw on an unknown name.

{{TypeConverter|typeConverter|type_converter}} is an {{IMapping|IMapping|Mapping}} from a spec type name (`integer`, `string`) to a native column type. SQL drivers set a {{DeterministicTypeConverter|DeterministicTypeConverter|DeterministicTypeConverter}}. A sized field uses the type definition (`size`), not a generator flag. Concurrency is a {{Where|where|Where}} on the update or delete. {{DefaultValueConverter|defaultValueConverter|default_value_converter}} maps a spec default token (`UtcNow`, `NewId`) to a native SQL expression. The default is the empty converter. SQL drivers override it. Other drivers inherit the empty converter.

::: prose
### typescript
Assign the exported `columnType` function.
### csharp
Inherit the default method.
### rust
Inherit the default method.
:::

SQLite, MySQL, and Postgres each keep create-table rendering on their own `CreateTableStatementTextRenderer`. The statement factory does not subclass that renderer. It maps a compiled create table to the rendered text and maps `SelectInsertedIdStatement` to `SELECT last_insert_rowid()`, `SELECT LAST_INSERT_ID()`, or `SELECT lastval()`. Statement {{Execute|executeStatement|execute_statement}} and the reader overloads call the factory before the string execute. The renderer overrides column type. MySQL and Postgres also override default and identity. A Postgres `bool` default goes through the bool type converter, so `1` becomes `TRUE`. SQLite leaves identity empty. Every other statement is left unchanged. The factory is not exported.

The package exports only the driver type.

::: prose
### csharp
That class is `public`. Connection, command, transaction, options, and converters are `internal`. `InternalsVisibleTo` opens them to the test project. `GetExportedTypes()` is exactly the driver.
### typescript
`src/index.ts` is one line, `export { SqliteDriver } from "./SqliteDriver.ts"`. `package.json` `exports` points at `dist/index.js`.
### rust
`lib.rs` declares private modules and one `pub use` of the driver. `pub` items inside those modules stay crate-private.
:::

`PublicSurfaceTests` in each C# provider test project, the `exports only the provider` test in `typescript/datasources`, and `providers_export_only_their_provider` in `datasources` reject a second export.

## Examples

### Register and Create

::: example
```csharp csharp/Providers/Memory.Tests/RegisterAndCreateTests.cs#RegisterAndCreate
```
```rust rust/providers/memory/src/lib.rs#register_and_create
```
```typescript typescript/providers/memory/src/index.test.ts#register-and-create
```
:::

### Mock driver

::: example
```csharp csharp/Datasources/Mocks/MockDriver.cs#MockDriver
```
```rust rust/datasources/src/mocks/mock_driver.rs#DriverForMockDriver
```
```typescript typescript/datasources/src/mocks/mock-driver.ts#MockDriver
```
:::
