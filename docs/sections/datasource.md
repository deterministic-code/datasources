# Datasource

Factory for connections, commands, and repositories. `Create` reads `INSTALLED_PROVIDERS` and `DATABASE_URL` from the process environment when defaults are unset, registers each listed factory, then creates.

## Classes

::: reference
### typescript
#### Datasource

```typescript
class Datasource
```

**Methods**

##### defaults(options)

Stores default options and registers listed factories.

```typescript
static defaults(options: DatasourceOptions): void
```

- `options` — [DatasourceOptions](#datasourceoptions); stored defaults; listed factories are registered

##### register(...factories)

Registers factories by provider name.

```typescript
static register(...factories: IDatasourceFactory[]): void
```

- `factories` — [IDatasourceFactory](./datasource-factory.md) instances keyed by `provider`

##### create(provider?, options?)

Creates a datasource from the registry.

```typescript
static create(provider?: string, options?: DatasourceOptions): IDatasource
```

- `provider` — optional; inferred from `DATABASE_URL` when omitted
- `options` — [DatasourceOptions](#datasourceoptions); omitted uses stored defaults or the environment

**Returns** [IDatasource](#idatasource)

##### reset()

Clears the registry and stored defaults. Use in tests.

```typescript
static reset(): void
```

##### registered()

Lists registered provider names.

```typescript
static registered(): string[]
```

**Returns** `string[]`

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

**Returns** [ICRUDRepository](./repositories.md#crud-repository)`<T>`

##### createReadonlyRepository(connection?)

Creates a read-only repository bound to an optional open connection.

```typescript
createReadonlyRepository<T>(connection?: IConnection): IReadonlyRepository<T>
```

- `connection` — optional open [IConnection](./connection.md); omitted constructs one

**Returns** [IReadonlyRepository](./repositories.md#readonly-repository)`<T>`

#### DatasourceOptions

```typescript
type DatasourceOptions = Record<string, DatasourceOptionValue>
```

Well-known keys for `defaults` and `create`. Other keys are passed through to the factory.

**Properties**

##### DatasourceOptionValue

Allowed value for an options key.

```typescript
type DatasourceOptionValue = string | object | undefined
```

##### INSTALLED_PROVIDERS

Comma-separated provider names to register.

```typescript
INSTALLED_PROVIDERS?: string
```

##### DATABASE_URL

Connection URL. The scheme selects the default provider.

```typescript
DATABASE_URL?: string
```

##### DATABASE_SCHEMA

Default schema.

```typescript
DATABASE_SCHEMA?: string
```

##### url

Alias of `DATABASE_URL` passed through to the factory.

```typescript
url?: string
```

##### schema

Alias of `DATABASE_SCHEMA` passed through to the factory.

```typescript
schema?: string
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

**Returns** [ICRUDRepository](./repositories.md#crud-repository)`<T>`

##### CreateReadonlyRepository(connection)

Creates a read-only repository bound to an optional open connection. Defaults to `CreateRepository`.

```csharp
IReadonlyRepository<T> CreateReadonlyRepository<T>(IConnection? connection = null)
```

- `connection` — optional open [IConnection](./connection.md); omitted constructs one

**Returns** [IReadonlyRepository](./repositories.md#readonly-repository)`<T>`

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

##### url

Alias of `database_url` passed through to the provider.

```csharp
"url"
```

##### schema

Alias of `database_schema` passed through to the provider.

```csharp
"schema"
```

### rust
#### datasources

```rust
// datasources
```

**Methods**

##### defaults(options)

Stores default options and registers listed factories.

```rust
pub fn defaults(options: HashMap<String, String>) -> Result<()>
```

- `options` — [DatasourceOptions](#datasourceoptions); stored defaults; listed factories are registered

**Returns** `Result<()>`

##### register(factory)

Registers a factory by provider name.

```rust
pub fn register(factory: Arc<dyn DatasourceFactory>)
```

- `factory` — [DatasourceFactory](./datasource-factory.md) keyed by `provider()`

##### create(provider, options)

Creates a datasource from the registry. The provider name is required.

```rust
pub fn create(provider: &str, options: Option<&HashMap<String, String>>) -> Result<Arc<dyn Datasource>>
```

- `provider` — required provider name
- `options` — [DatasourceOptions](#datasourceoptions); merged with stored defaults

**Returns** `Result<Arc<dyn `[Datasource](#datasource)`>>`

##### create_default()

Creates a datasource using the provider inferred from `DATABASE_URL`.

```rust
pub fn create_default() -> Result<Arc<dyn Datasource>>
```

**Returns** `Result<Arc<dyn `[Datasource](#datasource)`>>`

##### reset()

Clears the registry and stored defaults. Use in tests.

```rust
pub fn reset()
```

##### registered()

Lists registered provider names.

```rust
pub fn registered() -> Vec<String>
```

**Returns** `Vec<String>`

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
fn create_repository(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn CRUDRepository>
```

- `connection` — optional open [Connection](./connection.md); omitted constructs one

**Returns** `Arc<dyn `[CRUDRepository](./repositories.md#crud-repository)`>`

##### create_readonly_repository(connection)

Creates a read-only repository bound to an optional open connection. Defaults to `create_repository`.

```rust
fn create_readonly_repository(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn ReadonlyRepository>
```

- `connection` — optional open [Connection](./connection.md); omitted constructs one

**Returns** `Arc<dyn `[ReadonlyRepository](./repositories.md#readonly-repository)`>`

#### DatasourceOptions

```rust
HashMap<String, String>
```

Well-known keys for `defaults` and `create`. Other keys are passed through to the factory.

**Properties**

##### INSTALLED_PROVIDERS

Comma-separated provider names to register.

```rust
"INSTALLED_PROVIDERS"
```

##### DATABASE_URL

Connection URL. The scheme selects the default provider.

```rust
"DATABASE_URL"
```

##### DATABASE_SCHEMA

Default schema.

```rust
"DATABASE_SCHEMA"
```

##### url

Alias of `DATABASE_URL` passed through to the factory.

```rust
"url"
```

##### schema

Alias of `DATABASE_SCHEMA` passed through to the factory.

```rust
"schema"
```
:::

## Remarks

C# `DatasourceFactory.Create(null)` reads process `DATABASE_URL` and `DATABASE_SCHEMA` (as `database_url` / `database_schema`) and registers names from process `INSTALLED_PROVIDERS`. A non-null options bag is used as-is; missing `provider` and `database_url` throw.

TypeScript and Rust still read `INSTALLED_PROVIDERS`, `DATABASE_URL`, and `DATABASE_SCHEMA` when defaults are unset. Listed factories are registered through `@datasources/register` or `datasources-register` when that package is present.

TypeScript omits `provider` when `DATABASE_URL` can supply it. Rust uses `create_default()` for that path; `create` always takes an explicit provider.

`Reset` / `reset` clears the registry. Use it in tests.

See [DatasourceFactory](./datasource-factory.md) for the factory contract, and [CLI](../cli.md#command-line-cli) for `setup` and adding providers.

## Examples

### Create from environment

::: dialects
```csharp
var ds = DatasourceFactory.Create();
```
```rust
let ds = create_default()?;
```
```typescript
const ds = Datasource.create();
```
:::
