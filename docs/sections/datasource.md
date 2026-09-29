# Datasource

One configured datasource: provider name plus factories for connections, commands, and repositories. `Default` / `default` / `Create` is on [DatasourceFactory](./datasource-factory.md).

## Classes

::: reference
### typescript
#### Datasource

```typescript
class Datasource implements IDatasource
```

**Constructor**

##### Datasource(provider, driver, options?)

Builds a datasource from a provider [IDriver](../writing-providers.md#idriver).

```typescript
constructor(provider: string, driver: IDriver, options?: DatasourceOptions)
```

- `provider` — registered provider name
- `driver` — [IDriver](../writing-providers.md#idriver) that builds connections, commands, transactions, and row converters
- `options` — [DatasourceOptions](#datasourceoptions) passed to `driver.connection`

**Methods**

##### create(options?)

Alias of [DatasourceFactory.create](./datasource-factory.md). `undefined` options read process env; a dictionary is used as-is.

```typescript
static create(options?: DatasourceOptions): IDatasource
```

- `options` — [DatasourceOptions](#datasourceoptions); omitted uses `.env` then `DATASOURCE_URL` / `DATABASE_SCHEMA`

**Returns** [IDatasource](#idatasource)

#### IDatasource

```typescript
interface IDatasource
```

**Properties**

##### provider

Registered provider name.

```typescript
readonly provider: string
```

**Methods**

##### createConnection()

Creates a closed connection. The caller opens it.

```typescript
createConnection(): Promise<IConnection>
```

**Returns** `Promise<[IConnection](./connection.md)>`

##### createCommand(connection?)

Creates a command bound to an optional open connection.

```typescript
createCommand(connection?: IConnection): ICommand
```

- `connection` — optional open [IConnection](./connection.md); omitted constructs one

**Returns** [ICommand](./command.md)

##### createRepository(type, connection?, options?)

Creates a CRUD repository for a class, bound to an optional open connection. Columns, id, and entity come from that class. The driver's row type converters replace `rowTypeConverters`.

```typescript
createRepository<T>(type: Constructable<T>, connection?: IConnection, options?: CRUDRepositoryOptions<T>): ICRUDRepository<T>
```

- `type` — class constructor. Its fields become columns, `id` or `Id` becomes the id, and `type.name` becomes the entity
- `connection` — optional open [IConnection](./connection.md); omitted constructs one
- `options` — optional [CRUDRepositoryOptions](./crud-repository.md#crudrepositoryoptions). Entity, columns, and mappers overlay the type defaults

**Returns** [ICRUDRepository](./crud-repository.md)`<T>`

##### createReadonlyRepository(type, connection?, options?)

Creates a read-only repository for a class, bound to an optional open connection. `Datasource` returns a `ReadonlyRepository`. The driver's row type converters replace `rowTypeConverters`.

```typescript
createReadonlyRepository<T>(type: Constructable<T>, connection?: IConnection, options?: ReadonlyRepositoryOptions<T>): IReadonlyRepository<T>
```

- `type` — class constructor. Its fields become columns, `id` or `Id` becomes the id, and `type.name` becomes the entity
- `connection` — optional open [IConnection](./connection.md); omitted constructs one
- `options` — optional [ReadonlyRepositoryOptions](./readonly-repository.md#readonlyrepositoryoptions). Entity, columns, and mappers overlay the type defaults

**Returns** [IReadonlyRepository](./readonly-repository.md)`<T>`

#### DatasourceOptions

```typescript
type DatasourceOptions = Record<string, string>
```

Well-known keys for `DatasourceFactory.create`. Other keys are passed through. `INSTALLED_PROVIDERS` is install/register only, not a Create option.

**Properties**

##### provider

Which datasource to return.

```typescript
provider?: string
```

##### database_url

Connection URL. `provider` is required separately.

```typescript
database_url?: string
```

##### database_schema

Default schema.

```typescript
database_schema?: string
```

### csharp
#### Datasource

```csharp
public sealed class Datasource : IDatasource
```

**Constructor**

##### Datasource(provider, driver, options)

Builds a datasource from a provider [IDriver](../writing-providers.md#idriver).

```csharp
public Datasource(string provider, IDriver driver, IReadOnlyDictionary<string, string?>? options = null)
```

- `provider` — registered provider name
- `driver` — [IDriver](../writing-providers.md#idriver) that builds connections, commands, transactions, and row converters
- `options` — [DatasourceOptions](#datasourceoptions) passed to `driver.Connection`

#### IDatasource

```csharp
public interface IDatasource
```

**Properties**

##### Provider

Registered provider name.

```csharp
string Provider { get; }
```

**Methods**

##### CreateConnection()

Creates a closed connection. The caller opens it.

```csharp
Task<IConnection> CreateConnection()
```

**Returns** `Task<[IConnection](./connection.md)>`

##### CreateCommand(connection)

Creates a command bound to an optional open connection.

```csharp
ICommand CreateCommand(IConnection? connection = null)
```

- `connection` — optional open [IConnection](./connection.md); omitted constructs one

**Returns** [ICommand](./command.md)

##### CreateRepository(connection, options)

Creates a CRUD repository bound to an optional open connection. The driver's row type converters replace `RowTypeConverters`.

```csharp
ICRUDRepository<T> CreateRepository<T>(IConnection? connection = null, CRUDRepositoryOptions<T>? options = null)
```

- `connection` — optional open [IConnection](./connection.md); omitted constructs one
- `options` — optional [CRUDRepositoryOptions](./crud-repository.md#crudrepositoryoptions). Entity, columns, and mappers overlay the type defaults

**Returns** [ICRUDRepository](./crud-repository.md)`<T>`

##### CreateReadonlyRepository(connection, options)

Creates a read-only repository bound to an optional open connection. `Datasource` returns a `ReadonlyRepository`. The interface default still forwards to `CreateRepository` when a type does not override the method. The driver's row type converters replace `RowTypeConverters`.

```csharp
IReadonlyRepository<T> CreateReadonlyRepository<T>(IConnection? connection = null, ReadonlyRepositoryOptions<T>? options = null)
```

- `connection` — optional open [IConnection](./connection.md); omitted constructs one
- `options` — optional [ReadonlyRepositoryOptions](./readonly-repository.md#readonlyrepositoryoptions). Entity, columns, and mappers overlay the type defaults

**Returns** [IReadonlyRepository](./readonly-repository.md)`<T>`

#### DatasourceOptions

```csharp
IReadOnlyDictionary<string, string?>
```

Well-known keys for `DatasourceFactory.Create`. Other keys are passed through. `INSTALLED_PROVIDERS` is install/register only, not a Create option.

**Properties**

##### provider

Which datasource to return.

```csharp
"provider"
```

##### database_url

Connection URL. `provider` is required separately.

```csharp
"database_url"
```

##### database_schema

Default schema.

```csharp
"database_schema"
```

### rust
#### BuiltDatasource

```rust
pub struct BuiltDatasource
```

**Constructor**

##### new(provider, driver, options)

Builds a datasource from a provider [Driver](../writing-providers.md#driver).

```rust
pub fn new(provider: impl Into<String>, driver: Arc<dyn Driver>, options: Option<HashMap<String, String>>) -> Self
```

- `provider` — registered provider name
- `driver` — [Driver](../writing-providers.md#driver) that builds connections, commands, transactions, and row converters
- `options` — [DatasourceOptions](#datasourceoptions) passed to `driver.connection`

#### Datasource

```rust
pub trait Datasource
```

**Methods**

##### provider()

Registered provider name.

```rust
fn provider(&self) -> &str
```

**Returns** `&str`

##### create_connection()

Creates a closed connection. The caller opens it.

```rust
async fn create_connection(&self) -> Result<Arc<dyn Connection>>
```

**Returns** `Result<Arc<dyn `[Connection](./connection.md)`>>`

##### create_command(connection)

Creates a command bound to an optional open connection.

```rust
fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command>
```

- `connection` — optional open [Connection](./connection.md); omitted constructs one

**Returns** `Arc<dyn `[Command](./command.md)`>`

##### create_repository(connection, options)

Creates a CRUD repository for `T`, bound to an optional open connection. The driver's row type converters replace `row_type_converters`.

```rust
fn create_repository<T: ConvertSchema + 'static>(&self, connection: Option<Arc<dyn Connection>>, options: Option<CRUDRepositoryOptions>) -> Arc<dyn ICRUDRepository>
```

- `connection` — optional open [Connection](./connection.md); omitted constructs one
- `options` — optional [CRUDRepositoryOptions](./crud-repository.md#crudrepositoryoptions). Entity, columns, and mappers overlay the type defaults. `None` uses the type defaults

**Returns** `Arc<dyn `[CRUDRepository](./crud-repository.md)`>`

##### create_readonly_repository(connection, options)

Creates a read-only repository for `T`, bound to an optional open connection. `BuiltDatasource` returns a `ReadonlyRepository`. The driver's row type converters replace `row_type_converters`.

```rust
fn create_readonly_repository<T: ConvertSchema + 'static>(&self, connection: Option<Arc<dyn Connection>>, options: Option<ReadonlyRepositoryOptions>) -> Arc<dyn IReadonlyRepository>
```

- `connection` — optional open [Connection](./connection.md); omitted constructs one
- `options` — optional [ReadonlyRepositoryOptions](./readonly-repository.md#readonlyrepositoryoptions). Entity, columns, and mappers overlay the type defaults. `None` uses the type defaults

**Returns** `Arc<dyn `[ReadonlyRepository](./readonly-repository.md)`>`

#### DatasourceOptions

```rust
HashMap<String, String>
```

Well-known keys for `create`. Other keys are passed through. `INSTALLED_PROVIDERS` is install/register only, not a Create option.

**Properties**

##### provider

Which datasource to return.

```rust
"provider"
```

##### database_url

Connection URL. `provider` is required separately.

```rust
"database_url"
```

##### database_schema

Default schema.

```rust
"database_schema"
```
:::

## Remarks

`DatasourceFactory.Default` / `DatasourceFactory.default` / `default()` loads `.env` `PROVIDER` and `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. `Create(null)` / `create()` / `create(None)` uses the same path. A non-null options bag is used as-is; missing `provider` throws.

Listed factories are registered through `Datasources.Hosting`, `@datasources/register`, or `datasources-register` when that package is present.

`Reset` / `reset` clears the registry. Use it in tests.

See [DatasourceFactory](./datasource-factory.md) for the factory contract, and [CLI](../cli.md#command-line-cli) for `setup` and adding providers.

## Examples

### Create from environment

::: dialects
```csharp
var ds = DatasourceFactory.Default;
```
```rust
let ds = default()?;
```
```typescript
const ds = DatasourceFactory.default;
```
:::
