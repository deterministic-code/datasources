# Writing Providers

A provider package implements `IDriver` / `Driver` and exports only that type. `DatasourceFactory.Register` / `register` stores it by name. The driver builds the connection, command, and transaction, and supplies row cell converters when native values are not already canonical cells.

`mysql`, `postgres`, and `sqlite` name the type `*Driver` (`SqliteDriver`). Every other package names it `*Provider` (`MemoryProvider`).

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
- `isolation` — defaults to `serializable`

**Returns** [ITransaction](./sections/transaction.md)

##### rowTypeConverters

Row cell converters keyed by canonical type name (`bool`, `datetime`, …). Empty for stubs and file dialects.

```typescript
rowTypeConverters: Record<string, TypeConverter<Cell, Cell>>
```

**Returns** `Record<string, TypeConverter<Cell, Cell>>`

##### generatedId(id)

Canonicalizes an engine insert id before it is stored on `Response.result`. The default, exported as `generatedId`, returns `id`.

```typescript
generatedId(id: Cell): Cell
```

- `id` — insert identity from the engine

**Returns** `Cell`

##### columnType(name)

SQL type name for a canonical column type (`int`, `string`, `bool`, …). The default, exported as `columnType`, returns `name`. MySQL, Postgres, and SQLite override it. An unknown name throws.

```typescript
columnType(name: string): string
```

- `name` — canonical type name

**Returns** `string`

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
ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable)
```

- `connection` — open or closed [IConnection](./sections/connection.md)
- `isolation` — defaults to `Serializable`

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
- `isolation` — defaults to `Serializable` at the call site

**Returns** `Arc<dyn `[Transaction](./sections/transaction.md)`>`

##### row_type_converters()

Row cell converters keyed by canonical type name (`bool`, `datetime`, …). Empty for stubs and file dialects.

```rust
fn row_type_converters(&self) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>
```

**Returns** `HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>`

##### generated_id(id)

Canonicalizes an engine insert id before it is stored on `Response.result`. The default returns `id`.

```rust
fn generated_id(&self, id: Value) -> Value
```

- `id` — insert identity from the engine

**Returns** [Value](./sections/value.md)

##### column_type(name)

SQL type name for a canonical column type (`int`, `string`, `bool`, …). The default returns `name`. MySQL, Postgres, and SQLite override it. An unknown name returns an error.

```rust
fn column_type(&self, name: &str) -> Result<String>
```

- `name` — canonical type name

**Returns** `Result<String>`
:::

## Remarks

`connection` receives the options bag (`url`, `filename`, `host`, `database`, `schema`, …). `command` delegates to the connection's `createCommand` / `CreateCommand` / `create_command`. `transaction` builds the dialect transaction. Isolation defaults to serializable.

`rowTypeConverters` / `RowTypeConverters` / `row_type_converters` maps a canonical type name to a [TypeConverter](./sections/type-converters.md). SQL dialects register `bool`, `datetime`, `uuid`, `decimal`, and `binary` when the engine cannot return those as canonical cells. Memory, document, and file providers return an empty map.

`generatedId` / `generated_id` canonicalizes an insert id in Rust and TypeScript. Those commands apply it to `last_insert_rowid`, `LastInsertedId`, `RETURNING` on `INSERT`, or the memory inserted id, before that value is stored on `Response.result`. SELECT scalars stay `Value.From`. The default returns the value unchanged. TypeScript drivers assign the exported `generatedId` function; Rust inherits the default method. C# has no `GeneratedId`. The statement factory formats `InsertedIdStatement`, and the repository reads that row.

`columnType` / `ColumnType` / `column_type` maps a canonical column type to the engine SQL type name. It does not convert cell values. The default returns the name unchanged. TypeScript drivers assign the exported `columnType` function; C# and Rust inherit the default method. MySQL, Postgres, and SQLite override it and throw on an unknown name.

SQLite, MySQL, and Postgres commands own an internal statement factory that extends `CreateTableStatementRenderer`. Statement `Execute` / `executeStatement` / `execute_statement` and the reader overloads call it before the string execute. The factory overrides column type. MySQL and Postgres also override default and identity. SQLite leaves identity empty. C# `Transform` also formats `InsertedIdStatement` as `SELECT last_insert_rowid()`, `SELECT LAST_INSERT_ID()`, or `SELECT lastval()`. Every other statement is left unchanged. The factory is not exported.

The package exports only the driver type.

- C#: that class is `public`. Connection, command, transaction, options, and converters are `internal`. `InternalsVisibleTo` opens them to the test project and to an engine assembly such as `Datasources.Sqlite.Microsoft`. `GetExportedTypes()` is exactly the driver. The engine assembly exports nothing.
- TypeScript: `src/index.ts` is one line, `export { SqliteDriver } from "./SqliteDriver.ts"`. `package.json` `exports` points only at that file.
- Rust: `lib.rs` declares private modules and one `pub use` of the driver. `pub` items inside those modules stay crate-private.

`PublicSurfaceTests` in each C# provider test project, the `exports only the provider` test in `typescript/register`, and `providers_export_only_their_provider` in `datasources-register` reject a second export.

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
