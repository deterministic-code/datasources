# DatasourceFactory

Creates a [Datasource](./datasource.md) for a provider and `DATASOURCE_URL`. After [setup](../cli.md#setup), use `Default` / `default` / `DatasourceFactory.default` to build that datasource from `.env`.

## Classes

::: reference
### typescript
#### DatasourceFactory

```typescript
class DatasourceFactory implements IDatasourceFactory
```

**Properties**

##### default

Loads `.env` `PROVIDER` / `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. Throws when neither is set. Uses [create](#create-options).

```typescript
static get default(): IDatasource
```

**Returns** [IDatasource](./datasource.md#idatasource)

**Methods**

##### defaults(options)

Registers listed provider packages. Does not apply to a non-null `create` bag.

```typescript
static defaults(options: DatasourceOptions): void
```

- `options` — install keys such as `INSTALLED_PROVIDERS`; not [Create options](./datasource.md#datasourceoptions)

##### register(provider, driver)

Registers a provider [IDriver](#idriver) by name.

```typescript
static register(provider: string, driver: IDriver): void
```

- `provider` — registry key (`sqlite`, `mysql`, …)
- `driver` — [IDriver](#idriver) that builds connections, commands, transactions, and row converters

##### create(options?)

Creates one datasource. `undefined` options read process env; a dictionary is used as-is.

```typescript
static create(options?: DatasourceOptions): IDatasource
```

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions); omitted uses `.env` then `DATASOURCE_URL` / `DATABASE_SCHEMA`

**Returns** [IDatasource](./datasource.md#idatasource)

##### reset()

Clears the registry. Use in tests.

```typescript
static reset(): void
```

##### registered()

Lists registered provider names.

```typescript
static registered(): string[]
```

**Returns** `string[]`

#### IDriver

```typescript
interface IDriver
```

**Methods**

##### connection(options?)

Builds an [IConnection](./connection.md) from the options bag.

```typescript
connection(options?: DatasourceOptions): IConnection
```

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions)

**Returns** [IConnection](./connection.md)

##### command(connection)

Builds an [ICommand](./command.md) for the connection.

```typescript
command(connection: IConnection): ICommand
```

- `connection` — open or closed [IConnection](./connection.md)

**Returns** [ICommand](./command.md)

##### transaction(connection, isolation?)

Builds an [ITransaction](./transaction.md) for the connection.

```typescript
transaction(connection: IConnection, isolation?: IsolationLevel): ITransaction
```

- `connection` — open or closed [IConnection](./connection.md)
- `isolation` — defaults to `serializable`

**Returns** [ITransaction](./transaction.md)

##### rowTypeConverters

Row cell converters keyed by canonical type name (`bool`, `datetime`, …). Empty for stubs and file dialects.

```typescript
rowTypeConverters: Record<string, TypeConverter<Cell, Cell>>
```

**Returns** `Record<string, TypeConverter<Cell, Cell>>`

#### IDatasourceFactory

```typescript
interface IDatasourceFactory
```

**Methods**

##### create(options?)

Creates one datasource from a string dictionary.

```typescript
create(options?: DatasourceOptions): IDatasource
```

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions)

**Returns** [IDatasource](./datasource.md#idatasource)

### csharp
#### DatasourceFactory

```csharp
public sealed class DatasourceFactory : IDatasourceFactory
```

**Properties**

##### Default

Loads `.env` `PROVIDER` / `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. Throws when neither is set. Uses [Create](#create-options).

```csharp
public static IDatasource Default { get; }
```

**Returns** [IDatasource](./datasource.md#idatasource)

**Methods**

##### Defaults(options)

Registers listed provider packages. Does not apply to a non-null `Create` bag.

```csharp
public static void Defaults(IReadOnlyDictionary<string, string?> options)
```

- `options` — install keys such as `INSTALLED_PROVIDERS`; not [Create options](./datasource.md#datasourceoptions)

##### Register(provider, driver)

Registers a provider [IDriver](#idriver) by name.

```csharp
public static void Register(string provider, IDriver driver)
```

- `provider` — registry key (`sqlite`, `mysql`, …)
- `driver` — [IDriver](#idriver) that builds connections, commands, transactions, and row converters

##### Create(options)

Creates one datasource. `null` options read process `.env`; a dictionary is used as-is.

```csharp
public static IDatasource Create(IReadOnlyDictionary<string, string?>? options = null)
```

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions); omitted uses `.env` then `DATASOURCE_URL` / `DATABASE_SCHEMA`

**Returns** [IDatasource](./datasource.md#idatasource)

##### Reset()

Clears the registry. Use in tests.

```csharp
public static void Reset()
```

##### Registered()

Lists registered provider names.

```csharp
public static IReadOnlyList<string> Registered()
```

**Returns** `IReadOnlyList<string>`

#### IDriver

```csharp
public interface IDriver
```

**Methods**

##### Connection(options)

Builds an [IConnection](./connection.md) from the options bag.

```csharp
IConnection Connection(IReadOnlyDictionary<string, string?>? options)
```

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions)

**Returns** [IConnection](./connection.md)

##### Command(connection)

Builds an [ICommand](./command.md) for the connection.

```csharp
ICommand Command(IConnection connection)
```

- `connection` — open or closed [IConnection](./connection.md)

**Returns** [ICommand](./command.md)

##### Transaction(connection, isolation)

Builds an [ITransaction](./transaction.md) for the connection.

```csharp
ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable)
```

- `connection` — open or closed [IConnection](./connection.md)
- `isolation` — defaults to `Serializable`

**Returns** [ITransaction](./transaction.md)

##### RowTypeConverters

Row cell converters keyed by canonical type name (`bool`, `datetime`, …). Empty for stubs and file dialects.

```csharp
IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters { get; }
```

**Returns** `IReadOnlyDictionary<string, TypeConverter<Value, Value>>`

#### IDatasourceFactory

```csharp
public interface IDatasourceFactory
```

**Methods**

##### Create(options)

Creates one datasource from a string dictionary.

```csharp
IDatasource Create(IReadOnlyDictionary<string, string?>? options = null)
```

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions)

**Returns** [IDatasource](./datasource.md#idatasource)

### rust
#### datasources

```rust
// datasources
```

**Methods**

##### defaults(options)

Registers listed provider packages. Does not apply to a `Some` `create` bag.

```rust
pub fn defaults(options: HashMap<String, String>) -> Result<()>
```

- `options` — install keys such as `INSTALLED_PROVIDERS`; not [Create options](./datasource.md#datasourceoptions)

**Returns** `Result<()>`

##### register(provider, driver)

Registers a provider [Driver](#driver) by name.

```rust
pub fn register(provider: impl Into<String>, driver: Arc<dyn Driver>)
```

- `provider` — registry key (`sqlite`, `mysql`, …)
- `driver` — [Driver](#driver) that builds connections, commands, transactions, and row converters

##### create(options)

Creates one datasource. `None` reads process env; `Some` is used as-is.

```rust
pub fn create(options: Option<&HashMap<String, String>>) -> Result<Arc<dyn Datasource>>
```

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions); omitted uses `.env` then `DATASOURCE_URL` / `DATABASE_SCHEMA`

**Returns** `Result<Arc<dyn `[Datasource](./datasource.md#datasource)`>>`

##### default()

Loads `.env` `PROVIDER` / `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. Throws when neither is set. Uses [create](#create-options).

```rust
pub fn default() -> Result<Arc<dyn Datasource>>
```

**Returns** `Result<Arc<dyn `[Datasource](./datasource.md#datasource)`>>`

##### create_default()

`default()`.

```rust
pub fn create_default() -> Result<Arc<dyn Datasource>>
```

**Returns** `Result<Arc<dyn `[Datasource](./datasource.md#datasource)`>>`

##### reset()

Clears the registry. Use in tests.

```rust
pub fn reset()
```

##### registered()

Lists registered provider names.

```rust
pub fn registered() -> Vec<String>
```

**Returns** `Vec<String>`

#### Driver

```rust
pub trait Driver
```

**Methods**

##### connection(options)

Builds a [Connection](./connection.md) from the options bag.

```rust
fn connection(&self, options: Option<&HashMap<String, String>>) -> Arc<dyn Connection>
```

- `options` — [DatasourceOptions](./datasource.md#datasourceoptions)

**Returns** `Arc<dyn `[Connection](./connection.md)`>`

##### command(connection)

Builds a [Command](./command.md) for the connection.

```rust
fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command>
```

- `connection` — open or closed [Connection](./connection.md)

**Returns** `Arc<dyn `[Command](./command.md)`>`

##### transaction(connection, isolation)

Builds a [Transaction](./transaction.md) for the connection.

```rust
fn transaction(&self, connection: Arc<dyn Connection>, isolation: IsolationLevel) -> Arc<dyn Transaction>
```

- `connection` — open or closed [Connection](./connection.md)
- `isolation` — defaults to `Serializable` at the call site

**Returns** `Arc<dyn `[Transaction](./transaction.md)`>`

##### row_type_converters()

Row cell converters keyed by canonical type name (`bool`, `datetime`, …). Empty for stubs and file dialects.

```rust
fn row_type_converters(&self) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>
```

**Returns** `HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>`
:::

## Remarks

`Default` requires `.env` `PROVIDER` and `DATASOURCE_URL`, or process `DEFAULT_PROVIDER` and `DEFAULT_DATASOURCE_URL` to override them. Missing both throws `DatasourceException` / `DatasourceError`.

A missing factory throws `ProviderNotInstalledError` / `ProviderNotInstalledException` / `DatasourceError::ProviderNotInstalled`.

`Datasources.Hosting`, `@datasources/register`, and `datasources-register` register each name in `INSTALLED_PROVIDERS` (and the provider inferred from `DATASOURCE_URL`) when `Create` is called with null / `undefined` / `None` options. Each dialect implements [IDriver](#idriver) (`MemoryProvider`, `SqliteProvider`, …) and that is what `register` stores. Dialect engine drivers (`ISqliteDriver`, `IMysqlDriver`) stay on the connection.

## Examples

### Default from environment

::: example
```csharp csharp/Datasources.Tests/DatasourceFactoryTests.cs#L24-L32
```
```rust rust/datasources/src/tests/datasource_factory.rs#L21-L28
```
```typescript typescript/datasources/src/DatasourceFactory.test.ts#L34-L38
```
:::

### Register and Create

::: example
```csharp csharp/Providers/Memory.Tests/RegisterAndCreateTests.cs#L10-L16
```
```rust rust/providers/memory/src/lib.rs#L38-L44
```
```typescript typescript/providers/memory/src/index.test.ts#L27-L32
```
:::

### Missing provider

::: example
```csharp csharp/Datasources.Tests/DatasourceFactoryTests.cs#L34-L40
```
```rust rust/datasources/src/tests/datasource_factory.rs#L31-L37
```
```typescript typescript/datasources/src/DatasourceFactory.test.ts#L41-L45
```
:::
