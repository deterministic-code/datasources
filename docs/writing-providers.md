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
:::

## Remarks

`connection` receives the options bag (`url`, `filename`, `host`, `database`, `schema`, …). `command` delegates to the connection's `createCommand` / `CreateCommand` / `create_command`. `transaction` builds the dialect transaction. Isolation defaults to serializable.

`rowTypeConverters` / `RowTypeConverters` / `row_type_converters` maps a canonical type name to a [TypeConverter](./sections/type-converters.md). SQL dialects register `bool`, `datetime`, `uuid`, `decimal`, and `binary` when the engine cannot return those as canonical cells. Memory, document, and file providers return an empty map.

The package exports only the driver type.

- C#: that class is `public`. Connection, command, transaction, options, and converters are `internal`. `InternalsVisibleTo` opens them to the test project and to an engine assembly such as `Datasources.Sqlite.Microsoft`. `GetExportedTypes()` is exactly the driver. The engine assembly exports nothing.
- TypeScript: `src/index.ts` is one line, `export { SqliteDriver } from "./SqliteDriver.ts"`. `package.json` `exports` points only at that file.
- Rust: `lib.rs` declares private modules and one `pub use` of the driver. `pub` items inside those modules stay crate-private.

`PublicSurfaceTests` in each C# provider test project, the `exports only the provider` test in `typescript/register`, and `providers_export_only_their_provider` in `datasources-register` reject a second export.

## Examples

### Register and Create

::: example
```csharp csharp/Providers/Memory.Tests/RegisterAndCreateTests.cs#L10-L16
```
```rust rust/providers/memory/src/lib.rs#L38-L44
```
```typescript typescript/providers/memory/src/index.test.ts#L27-L32
```
:::

### Mock driver

::: example
```csharp csharp/Datasources/Mocks/MockDriver.cs#L3-L11
```
```rust rust/datasources/src/mocks/mock_driver.rs#L22-L44
```
```typescript typescript/datasources/src/mocks/mock-driver.ts#L13-L25
```
:::
