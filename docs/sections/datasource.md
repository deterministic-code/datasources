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

##### Datasource(provider, newConnection, newRepository)

Builds a datasource from a connection factory and a repository factory.

```typescript
constructor(provider: string, newConnection: () => IConnection, newRepository: (connection: IConnection) => object)
```

- `provider` — registered provider name
- `newConnection` — constructs the default [IConnection](./connection.md)
- `newRepository` — constructs a repository from an [IConnection](./connection.md)

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

##### createRepository(connection?)

Creates a CRUD repository bound to an optional open connection.

```typescript
createRepository<T>(connection?: IConnection): ICRUDRepository<T>
```

- `connection` — optional open [IConnection](./connection.md); omitted constructs one

**Returns** [ICRUDRepository](./crud-repository.md)`<T>`

##### createReadonlyRepository(connection?)

Creates a read-only repository bound to an optional open connection.

```typescript
createReadonlyRepository<T>(connection?: IConnection): IReadonlyRepository<T>
```

- `connection` — optional open [IConnection](./connection.md); omitted constructs one

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

Connection URL. The scheme selects the provider when `provider` is omitted.

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

##### Datasource(provider, newConnection, newRepository)

Builds a datasource from a connection factory and a repository factory.

```csharp
public Datasource(string provider, Func<IConnection> newConnection, Func<IConnection, object> newRepository)
```

- `provider` — registered provider name
- `newConnection` — constructs the default [IConnection](./connection.md)
- `newRepository` — constructs a repository from an [IConnection](./connection.md)

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

##### CreateRepository(connection)

Creates a CRUD repository bound to an optional open connection.

```csharp
ICRUDRepository<T> CreateRepository<T>(IConnection? connection = null)
```

- `connection` — optional open [IConnection](./connection.md); omitted constructs one

**Returns** [ICRUDRepository](./crud-repository.md)`<T>`

##### CreateReadonlyRepository(connection)

Creates a read-only repository bound to an optional open connection. Defaults to `CreateRepository`.

```csharp
IReadonlyRepository<T> CreateReadonlyRepository<T>(IConnection? connection = null)
```

- `connection` — optional open [IConnection](./connection.md); omitted constructs one

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

Connection URL. The scheme selects the provider when `provider` is omitted.

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

##### new(provider, new_connection, new_repository)

Builds a datasource from a connection factory and a repository factory.

```rust
pub fn new(provider: impl Into<String>, new_connection: impl Fn() -> Arc<dyn Connection> + Send + Sync + 'static, new_repository: impl Fn(Arc<dyn Connection>) -> Arc<dyn ICRUDRepository> + Send + Sync + 'static) -> Self
```

- `provider` — registered provider name
- `new_connection` — constructs the default [Connection](./connection.md)
- `new_repository` — constructs a repository from a [Connection](./connection.md)

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

##### create_repository(connection)

Creates a CRUD repository bound to an optional open connection.

```rust
fn create_repository(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn ICRUDRepository>
```

- `connection` — optional open [Connection](./connection.md); omitted constructs one

**Returns** `Arc<dyn `[CRUDRepository](./crud-repository.md)`>`

##### create_readonly_repository(connection)

Creates a read-only repository bound to an optional open connection. Defaults to `create_repository`.

```rust
fn create_readonly_repository(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn IReadonlyRepository>
```

- `connection` — optional open [Connection](./connection.md); omitted constructs one

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

Connection URL. The scheme selects the provider when `provider` is omitted.

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

`DatasourceFactory.Default` / `DatasourceFactory.default` / `default()` loads `.env` `PROVIDER` and `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. `Create(null)` / `create()` / `create(None)` uses the same path. A non-null options bag is used as-is; missing `provider` and `database_url` throw.

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
