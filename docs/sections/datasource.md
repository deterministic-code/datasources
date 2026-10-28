# Datasource

Use Datasource to create repositories, commands, and connections for a datasource provider. After setup has written an `.env`, {{Datasource.Default|Datasource.default|default()}} returns that provider. The connection is closed by default.

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

**Properties**

##### default

Loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`. Throws when `PROVIDER` is not set. Calls [create](#create-options) with no options.

```typescript
static get default(): IDatasource
```

**Returns** [IDatasource](#idatasource)

##### repositories

Named repository catalog for this datasource.

```typescript
readonly repositories: RepositoryFactory
```

**Returns** [RepositoryFactory](./repository-factories.md#repositoryfactory)

**Methods**

##### create(options?)

Alias of [DatasourceFactory.create](./datasource-factory.md). `undefined` options read process env; a dictionary is used as-is.

```typescript
static create(options?: DatasourceOptions): IDatasource
```

- `options` — [DatasourceOptions](#datasourceoptions); omitted loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`

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

##### repositories

Named repository catalog for this datasource.

```typescript
readonly repositories: RepositoryFactory
```

**Returns** [RepositoryFactory](./repository-factories.md#repositoryfactory)

**Methods**

##### createConnection()

Creates a connection.

```typescript
createConnection(): Promise<IConnection>
```

**Returns** `Promise<[IConnection](./connection.md)>`

##### createCommand(connection?)

Creates a command.

```typescript
createCommand(connection?: IConnection): Promise<ICommand>
```

- `connection` — optional [IConnection](./connection.md). Omitted opens the datasource connection and throws when that connection is already open. A passed connection is not opened

**Returns** `Promise<[ICommand](./command.md)>`

##### createRepository(type, connection?, options?)

Creates a CRUD repository.

```typescript
createRepository<T>(type: Constructable<T>, connection?: IConnection, options?: CRUDRepositoryOptions<T>): ICRUDRepository<T>
```

- `type` — class constructor. Its fields become columns, `id` or `Id` becomes the id, and `type.name` becomes the entity
- `connection` — optional [IConnection](./connection.md); omitted uses the datasource connection
- `options` — optional [CRUDRepositoryOptions](./crud-repository.md#crudrepositoryoptions). Entity, columns, and mappers overlay the type defaults

**Returns** [ICRUDRepository](./crud-repository.md)`<T>`

##### createReadonlyRepository(type, connection?, options?)

Creates a read-only repository.

```typescript
createReadonlyRepository<T>(type: Constructable<T>, connection?: IConnection, options?: ReadonlyRepositoryMappingOptions<T>): IReadonlyRepository<T>
```

- `type` — class constructor. Its fields become columns, `id` or `Id` becomes the id, and `type.name` becomes the entity
- `connection` — optional [IConnection](./connection.md); omitted uses the datasource connection
- `options` — optional [ReadonlyRepositoryMappingOptions](./readonly-repository.md#readonlyrepositoryoptions). Entity, columns, and mappers overlay the type defaults

**Returns** [IReadonlyRepository](./readonly-repository.md)`<T>`

##### createRepository(typeName, connection?)

Creates a registered CRUD repository by entity or plural name.

```typescript
createRepository(typeName: string, connection?: IConnection): ICRUDRepository
```

- `typeName` — entity name (`Contact`) or plural name (`contacts`)
- `connection` — optional [IConnection](./connection.md); omitted uses the datasource connection

**Returns** [ICRUDRepository](./crud-repository.md)

##### createReadonlyRepository(typeName, connection?)

Creates a registered read-only repository by entity or plural name.

```typescript
createReadonlyRepository(typeName: string, connection?: IConnection): IReadonlyRepository
```

- `typeName` — entity name (`Contact`) or plural name (`contacts`)
- `connection` — optional [IConnection](./connection.md); omitted uses the datasource connection

**Returns** [IReadonlyRepository](./readonly-repository.md)

#### RepositoryAccess

Handle returned by [RepositoryFactory.access](./repository-factories.md). `create` builds the generated CRUD repository. `createReadonly` builds the readonly repository. A plural name on [Datasource](#datasource) is the CRUD repository.

```typescript
class RepositoryAccess
```

**Methods**

##### create()

Creates the registered CRUD repository.

```typescript
create(): ICRUDRepository
```

**Returns** [ICRUDRepository](./crud-repository.md)

##### createReadonly()

Creates the registered read-only repository.

```typescript
createReadonly(): IReadonlyRepository
```

**Returns** [IReadonlyRepository](./readonly-repository.md)

#### DatasourceOptions

```typescript
type DatasourceOptions = Record<string, string>
```

Well-known keys for `DatasourceFactory.create`. Other keys are passed through. `INSTALLED_PROVIDERS` is install/register only, not a Create option.

**Properties**

##### PROVIDER

Which datasource to return.

```typescript
PROVIDER?: string
```

##### DATASOURCE_URL

Connection URL. Schema is the `schema` query (`postgres://host/db?schema=public`).

```typescript
DATASOURCE_URL?: string
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

**Properties**

##### Default

Loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`. Throws when `PROVIDER` is not set. Calls [DatasourceFactory.Create](./datasource-factory.md#create-options) with no options.

```csharp
public static IDatasource Default { get; }
```

**Returns** [IDatasource](#idatasource)

##### Repositories

Named repository catalog for this datasource.

```csharp
public RepositoryFactory Repositories { get; }
```

**Returns** [RepositoryFactory](./repository-factories.md#repositoryfactory)

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

##### Repositories

Named repository catalog for this datasource.

```csharp
RepositoryFactory Repositories { get; }
```

**Returns** [RepositoryFactory](./repository-factories.md#repositoryfactory)

**Methods**

##### CreateConnection()

Creates a connection.

```csharp
Task<IConnection> CreateConnection()
```

**Returns** `Task<[IConnection](./connection.md)>`

##### CreateCommand(connection)

Creates a command.

```csharp
Task<ICommand> CreateCommand(IConnection? connection = null)
```

- `connection` — optional [IConnection](./connection.md). Omitted opens the datasource connection and throws when that connection is already open. A passed connection is not opened

**Returns** `Task<[ICommand](./command.md)>`

##### CreateRepository(connection, options)

Creates a CRUD repository.

```csharp
ICRUDRepository<T> CreateRepository<T>(IConnection? connection = null, CRUDRepositoryOptions<T>? options = null)
```

- `connection` — optional [IConnection](./connection.md); omitted uses the datasource connection
- `options` — optional [CRUDRepositoryOptions](./crud-repository.md#crudrepositoryoptions). Entity, columns, and mappers overlay the type defaults

**Returns** [ICRUDRepository](./crud-repository.md)`<T>`

##### CreateReadonlyRepository(connection, options)

Creates a read-only repository.

```csharp
IReadonlyRepository<T> CreateReadonlyRepository<T>(IConnection? connection = null, ReadonlyRepositoryMappingOptions<T>? options = null)
```

- `connection` — optional [IConnection](./connection.md); omitted uses the datasource connection
- `options` — optional [ReadonlyRepositoryMappingOptions](./readonly-repository.md#readonlyrepositoryoptions). Entity, columns, and mappers overlay the type defaults

**Returns** [IReadonlyRepository](./readonly-repository.md)`<T>`

##### CreateRepository(typeName, connection)

Creates a registered CRUD repository by entity or plural name.

```csharp
ICRUDRepository<T> CreateRepository<T>(string typeName, IConnection? connection = null)
```

- `typeName` — entity name (`Contact`) or plural name (`Contacts`)
- `connection` — optional [IConnection](./connection.md); omitted uses the datasource connection

**Returns** [ICRUDRepository](./crud-repository.md)`<T>`

##### CreateReadonlyRepository(typeName, connection)

Creates a registered read-only repository by entity or plural name.

```csharp
IReadonlyRepository<T> CreateReadonlyRepository<T>(string typeName, IConnection? connection = null)
```

- `typeName` — entity name (`Contact`) or plural name (`Contacts`)
- `connection` — optional [IConnection](./connection.md); omitted uses the datasource connection

**Returns** [IReadonlyRepository](./readonly-repository.md)`<T>`

#### RepositoryAccess

Handle returned by [RepositoryFactory.Access](./repository-factories.md). `Create` builds the generated CRUD repository. `CreateReadonly` builds the readonly repository. A plural name on `dynamic` [Datasource](#datasource) is the CRUD repository.

```csharp
public sealed class RepositoryAccess
```

**Methods**

##### Create()

Creates the registered CRUD repository.

```csharp
public object Create()
```

**Returns** `object`

##### CreateReadonly()

Creates the registered read-only repository.

```csharp
public object CreateReadonly()
```

**Returns** `object`

#### DatasourceOptions

```csharp
IReadOnlyDictionary<string, string?>
```

Well-known keys for `DatasourceFactory.Create`. Other keys are passed through. `INSTALLED_PROVIDERS` is install/register only, not a Create option.

**Properties**

##### PROVIDER

Which datasource to return.

```csharp
"PROVIDER"
```

##### DATASOURCE_URL

Connection URL. Schema is the `schema` query (`postgres://host/db?schema=public`).

```csharp
"DATASOURCE_URL"
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

#### datasources

```rust
// datasources
```

**Methods**

##### default()

Loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`. Throws when `PROVIDER` is not set. Calls [create](./datasource-factory.md#create-options) with no options.

```rust
pub fn default() -> Result<Arc<dyn Datasource>>
```

**Returns** `Result<Arc<dyn `[Datasource](#datasource)`>>`

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

Creates a connection.

```rust
async fn create_connection(&self) -> Result<Arc<dyn Connection>>
```

**Returns** `Result<Arc<dyn `[Connection](./connection.md)`>>`

##### create_command(connection)

Creates a command.

```rust
async fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Result<Arc<dyn Command>>
```

- `connection` — optional [Connection](./connection.md). Omitted opens the datasource connection and throws when that connection is already open. A passed connection is not opened

**Returns** `Result<Arc<dyn `[Command](./command.md)`>>`

##### create_repository(connection, options)

Creates a CRUD repository.

```rust
fn create_repository<T: ConvertSchema + 'static>(&self, connection: Option<Arc<dyn Connection>>, options: Option<CRUDRepositoryOptions>) -> Arc<dyn ICRUDRepository>
```

- `connection` — optional [Connection](./connection.md); omitted uses the datasource connection
- `options` — optional [CRUDRepositoryOptions](./crud-repository.md#crudrepositoryoptions). Entity, columns, and mappers overlay the type defaults. `None` uses the type defaults

**Returns** `Arc<dyn `[CRUDRepository](./crud-repository.md)`>`

##### create_readonly_repository(connection, options)

Creates a read-only repository.

```rust
fn create_readonly_repository<T: ConvertSchema + 'static>(&self, connection: Option<Arc<dyn Connection>>, options: Option<ReadonlyRepositoryMappingOptions>) -> Arc<dyn IReadonlyRepository>
```

- `connection` — optional [Connection](./connection.md); omitted uses the datasource connection
- `options` — optional [ReadonlyRepositoryMappingOptions](./readonly-repository.md#readonlyrepositoryoptions). Entity, columns, and mappers overlay the type defaults. `None` uses the type defaults

**Returns** `Arc<dyn `[ReadonlyRepository](./readonly-repository.md)`>`

##### repositories()

Named repository catalog for this datasource.

```rust
fn repositories(&self) -> &RepositoryFactory
```

**Returns** `&`[RepositoryFactory](./repository-factories.md#repositoryfactory)

##### create_repository_by_name(name, connection)

Creates a registered CRUD repository by entity or plural name.

```rust
fn create_repository_by_name(&self, name: &str, connection: Option<Arc<dyn Connection>>) -> Result<Arc<dyn ICRUDRepository>>
```

- `name` — entity name (`Contact`) or plural name (`contacts`)
- `connection` — optional [Connection](./connection.md); omitted uses the datasource connection

**Returns** `Result<Arc<dyn `[ICRUDRepository](./crud-repository.md)`>>`

##### create_readonly_repository_by_name(name, connection)

Creates a registered read-only repository by entity or plural name.

```rust
fn create_readonly_repository_by_name(&self, name: &str, connection: Option<Arc<dyn Connection>>) -> Result<Arc<dyn IReadonlyRepository>>
```

- `name` — entity name (`Contact`) or plural name (`contacts`)
- `connection` — optional [Connection](./connection.md); omitted uses the datasource connection

**Returns** `Result<Arc<dyn `[IReadonlyRepository](./readonly-repository.md)`>>`

##### access(name, connection)

Returns a [RepositoryAccess](#repositoryaccess) for a registered name.

```rust
fn access(&self, name: &str, connection: Option<Arc<dyn Connection>>) -> RepositoryAccess
```

- `name` — entity name or plural name
- `connection` — optional [Connection](./connection.md); omitted uses the datasource connection

**Returns** [RepositoryAccess](#repositoryaccess)

#### RepositoryAccess

Handle returned by [RepositoryFactory.access](./repository-factories.md). `create` builds the generated CRUD repository. `create_readonly` builds the readonly repository. A generated plural method such as `contacts()` is the CRUD repository.

```rust
pub struct RepositoryAccess
```

**Methods**

##### create()

Creates the registered CRUD repository.

```rust
pub fn create(&self) -> Result<Arc<dyn ICRUDRepository>>
```

**Returns** `Result<Arc<dyn `[ICRUDRepository](./crud-repository.md)`>>`

##### create_readonly()

Creates the registered read-only repository.

```rust
pub fn create_readonly(&self) -> Result<Arc<dyn IReadonlyRepository>>
```

**Returns** `Result<Arc<dyn `[IReadonlyRepository](./readonly-repository.md)`>>`

#### DatasourceOptions

```rust
HashMap<String, String>
```

Well-known keys for `create`. Other keys are passed through. `INSTALLED_PROVIDERS` is install/register only, not a Create option.

**Properties**

##### PROVIDER

Which datasource to return.

```rust
"PROVIDER"
```

##### DATASOURCE_URL

Connection URL. Schema is the `schema` query (`postgres://host/db?schema=public`).

```rust
"DATASOURCE_URL"
```
:::

## Remarks

{{Datasource.Default|Datasource.default|default()}} loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`. {{DatasourceFactory.Create(null)|DatasourceFactory.create()|create(None)}} uses the same path. A non-null options bag is used as-is; missing `PROVIDER` throws.

TypeScript and C# load an installed provider with `DriverLoader` and `ProviderCatalog`. Rust registers the public driver from the cargo dependency.

::: prose
### typescript
Load the provider package.
### csharp
Load the assembly.
### rust
Register the public driver from the cargo dependency.
:::

{{Reset|reset|reset}} clears the registry. Use it in tests.

After `generate repositories`, import the generated registry so {{Datasource.Default|Datasource.default|default()}} binds each plural name. {{ReadSingle|readSingle|read_single}} returns the entity ({{Contact?|Contact \| undefined|Option<Row>}}). {{Create|create|create}} returns that same entity. `Update` and `Delete` return `Response`. {{CreateRepository|createRepository|create_repository_by_name}} takes the entity or plural name as a string.

See [DatasourceFactory](./datasource-factory.md) for the factory contract, and [CLI](../cli.md#command-line-cli) for `setup` and adding providers.

## Examples

### Create default from .env settings

::: example
```csharp csharp/Datasources.Tests/DatasourceTests.cs#CreateDefaultFromEnvSettings
```
```rust rust/datasources/src/tests/datasource.rs#create_default_from_env_settings
```
```typescript typescript/datasources/src/Datasource.test.ts#create-default-from-env-settings
```
:::

### Default using environment variables

::: example
```csharp csharp/Datasources.Tests/DatasourceFactoryTests.cs#DefaultFromEnvironment
```
```rust rust/datasources/src/tests/datasource_factory.rs#default_from_environment
```
```typescript typescript/datasources/src/DatasourceFactory.test.ts#default-from-environment
```
:::

### Create using Dynamic After Register

::: example
```csharp csharp/Providers/Sqlite.Tests/DatasourcePluralIntegrationTests.cs#CreatesUsingDynamicAfterRegister
```
```rust rust/providers/sqlite/tests/datasource_plural.rs#creates_using_dynamic_after_register
```
```typescript typescript/providers/sqlite/tests/datasource.repository.integration.test.ts#creates-using-dynamic-after-register
```
:::

### Create from a driver

::: example
```csharp csharp/Providers/Sqlite.Tests/CreateFromDriverTests.cs#CreateFromDriver
```
```rust rust/providers/sqlite/tests/create_from_driver.rs#create_from_driver
```
```typescript typescript/providers/sqlite/src/createFromDriver.test.ts#create-from-a-driver
```
:::
