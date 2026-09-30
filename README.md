<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:readme. -->

# Datasources

Datasources is a data-access and object-relational mapper library with parity across multiple languages and datasource providers.

It is designed for AI agents. The motivation is that current AI-generated data-access code is not deterministic, and quality often degrades as a project scales.

The feature stack starts with standard data-access modules for commands, connections, and object mapping. Higher up, the API is more opinionated and adds time-saving features for CRUD repositories, enriched repositories, and bulk repositories.

- [Quick Start](#quick-start)
- [Feature Matrix](#feature-matrix)

## Quick Start

### Installation

##### TypeScript

```typescript
npm install @datasources/core
```

##### C#

```csharp
dotnet add package Datasources
```

##### Rust

```rust
cargo add datasources
```

### Setup

Run interactive setup.

```sh
npx @datasources/cli setup
```

Add a new datasource provider.

```sh
npx @datasources/cli add sqlite
```

### Readonly command

Start with `createCommand` and `executeReader`. The connection must be open.

##### C#

```csharp
var ds = Datasource.Default;
var connection = await ds.CreateConnection();
await connection.Open();

var command = ds.CreateCommand(connection);
var reader = await command.ExecuteReader(
    "SELECT id, name FROM item WHERE id = $id",
    [new Parameter("id", 1)]);
while (await reader.Next())
    Console.WriteLine(reader.Current);
```

##### Rust

```rust
use datasources::{default, Parameter};

let ds = default()?;
let connection = ds.create_connection().await?;
connection.open().await?;

let command = ds.create_command(Some(connection));
let reader = command
    .execute_reader("SELECT id, name FROM item WHERE id = $id", Some(&[Parameter::new("id", 1)]))
    .await?;
while reader.next().await? {
    println!("{:?}", reader.current());
}
```

##### TypeScript

```typescript
import { Datasource } from "@datasources/core";

const ds = Datasource.default;
const connection = await ds.createConnection();
await connection.open();

const command = ds.createCommand(connection);
const reader = await command.executeReader("SELECT id, name FROM item WHERE id = $id", [
  { name: "id", value: 1 },
]);
while (await reader.next()) console.log(reader.current);
```

See [Command](#command) for middleware and `execute`.

### Enriched CRUD repository

A `CRUDRepository` fills related fields after `read` once enrichments are registered. When the related repository is also a CRUD repository, nested `many` children are written on create / update / delete. A readonly repository only fills.

##### C#

```csharp
var contacts = new CRUDRepository<Contact>(connection);
contacts.AddEnriched(new("contact_type_name", "contact_type_id", "id", "name"), types);
contacts.AddEnriched(new("addresses", "id", "contact_id", Many: true), addresses);

var loaded = await contacts.ReadSingle(new Where(new Operator.Equals("id", 10)));
await contacts.Create(new Contact { first_name = "Ada", addresses = [new Address { line1 = "1 Main" }] });
```

##### Rust

```rust
let mut contacts = CRUDRepository::from_type::<Contact>(connection)?;
contacts
    .add_enriched(Enrichment::new("contact_type_name", "contact_type_id", "id", Some("name".into()), false), types, None)
    .add_enriched(Enrichment::new("addresses", "id", "contact_id", None, true), addresses, None);

let loaded = contacts.read_single(Some(&Where::new(Operator::equals("id", 10))), None).await?;
contacts.create(contact).await?;
```

##### TypeScript

```typescript
const contacts = new CRUDRepository(Contact, connection);
contacts.addEnriched({ into: "contact_type_name", from: "contact_type_id", by: "id", select: "name" }, types);
contacts.addEnriched({ into: "addresses", from: "id", by: "contact_id", many: true }, addresses);

const loaded = await contacts.readSingle(new Where(new Operator.Equals("id", 10)));
await contacts.create({ first_name: "Ada", addresses: [{ line1: "1 Main" }] });
```

See [Repositories](#repositories) for readonly, CRUD, and bulk.

### Mappers

`TransformMapper` reshapes an already-converted object. Repository read is convert, then map.

##### C#

```csharp
var mapper = new TransformMapper<Nested, Flat>(
    n => new Flat { UserId = n.User.Id, UserName = n.User.Name },
    f => new Nested { User = new User { Id = f.UserId, Name = f.UserName } });
var flat = mapper.Map(nested);
var back = mapper.ReverseMap(flat);
```

##### Rust

```rust
let mapper = TransformMapper::new(
    |n: Nested| Flat { user_id: n.user.id, user_name: n.user.name },
    |f: Flat| Nested { user: User { id: f.user_id, name: f.user_name } },
);
let flat = mapper.map(nested);
let back = mapper.reverse_map(flat);
```

##### TypeScript

```typescript
const mapper = new TransformMapper<Nested, Flat>(
  (n) => ({ userId: n.user.id, userName: n.user.name }),
  (f) => ({ user: { id: f.userId, name: f.userName } }),
);
const flat = mapper.map(nested);
const back = mapper.reverseMap(flat);
```

See [Mappers](#mappers) for `TransformMapper`, `FlatToNestedMapper`, and [ReadonlyRepositoryOptions.RowsMapper](#rowsmapper).

### Type Converters

`TypeConverter` is only `Convert` / `convert` / `convert` and `ReverseConvert` / `reverseConvert` / `reverse_convert`. Attach field converters on `RowTypeConverter` when the native provider cannot map the type.

##### C#

```csharp
sealed class FlagConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => from.ToObject() is long n && n == 1;
    public override Value ReverseConvert(Value to) => to;
}

var rows = new RowTypeConverter<Item>(fields: new Dictionary<string, TypeConverter<Value, Value>>
{
    ["flag"] = new FlagConverter()
});
var item = rows.Convert(row);
```

##### Rust

```rust
struct FlagConverter;
impl TypeConverter<Value, Value> for FlagConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(match from {
            Value::Int(n) => Value::Bool(*n == 1),
            other => other.clone(),
        })
    }
    fn reverse_convert(&self, to: &Value) -> Result<Value> {
        Ok(to.clone())
    }
}

let rows = RowTypeConverter::new(vec!["flag".into()]).with_fields(HashMap::from([(
    "flag".into(),
    Arc::new(FlagConverter) as Arc<dyn TypeConverter<Value, Value>>,
)]));
let item = rows.convert(&row)?;
```

##### TypeScript

```typescript
class FlagConverter extends TypeConverter {
  convert = (value: Cell): Cell => value === 1;
  reverseConvert = (value: Cell): Cell => value;
}

const rows = new RowTypeConverter({}, { flag: new FlagConverter() });
const item = rows.convert(row);
```

See [Type Converters](#type-converters).

## Feature Matrix

Parity across C#, Rust, and TypeScript. Standard modules first; opinionated repositories above them.

### Feature stack

| Feature | C# | Rust | TypeScript |
| --- | :---: | :---: | :---: |
| [Datasource Factory](#datasource-factory) | ✅ | ✅ | ✅ |
| [Datasource](#datasource) | ✅ | ✅ | ✅ |
| [Mappers](#mappers) | ✅ | ✅ | ✅ |
| [Type Converters](#type-converters) | ✅ | ✅ | ✅ |
| [Connection](#connection) | ✅ | ✅ | ✅ |
| [Transaction](#transaction) | ✅ | ✅ | ✅ |
| [Command](#command) | ✅ | ✅ | ✅ |
| [Reader](#reader) | ✅ | ✅ | ✅ |
| [Statements](#statements) | ✅ | ✅ | ✅ |
| [Repositories](#repositories) | ✅ | ✅ | ✅ |
| [Readonly Repository](#readonly-repository) | ✅ | ✅ | ✅ |
| [CRUD Repository](#crud-repository) | ✅ | ✅ | ✅ |
| [Bulk Repository](#bulk-repository) | ✅ | ✅ | ✅ |
| [Repository Factories](#repository-factories) | ✅ | ✅ | ✅ |

### Providers

| Provider | C# | Rust | TypeScript |
| --- | :---: | :---: | :---: |
| json | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> |
| memory | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> |
| sqlite | ✅ | ✅ | ✅ |
| xml | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> |
| yaml | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> |
| mysql | ✅ | ✅ | ✅ |
| sqlserver | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> |
| postgres | ✅ | ✅ | ✅ |
| oracle | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> |
| mongodb | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> |
| dynamodb | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> |

Package names per language are on [Quick Start](#installation).

## Examples

Short recipes for the usual paths. Add samples here as Repository and related types land.

### Create a repository

TODO: register a provider, create a connection, `createRepository`, then `create` / `read` / `update` / `delete`.

### Run a command

TODO: open a connection, `createCommand`, `execute` / `executeReader`. See [Command](#command) for the current samples.

### Run a transaction

TODO: `startTransaction`, work, then `commit` or `abort`. See [Transaction](#transaction).

### Map a row

TODO: `TransformMapper` flatten, pass-through names, or rewrite a statement. See [Mappers](#mappers).

### Convert a row

TODO: `RowTypeConverter` convert / reverse-convert, or attach a field converter. See [RowTypeConverter](#row-type-converter).

## Datasource

Use Datasource to create repositories, commands, and connections for a datasource provider. After setup has written an `.env`, `Datasource.Default` / `Datasource.default` / `default()` returns that provider. The connection is closed by default.

### Classes

##### TypeScript

##### Datasource {#typescript-datasource}

```typescript
class Datasource implements IDatasource
```

**Constructor**

| Name | Description |
| --- | --- |
| [Datasource(provider, driver, options?)](#typescript-datasource-datasource-provider-driver-options) | Builds a datasource from a provider [IDriver](#typescript-idriver). |

**Properties**

| Name | Description |
| --- | --- |
| default | Loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`. Throws when `PROVIDER` is not set. Calls [create](#typescript-create-options) with no options. |

**Methods**

| Name | Description |
| --- | --- |
| [create(options?)](#typescript-datasource-create-options) | Alias of [DatasourceFactory.create](#datasource-factory). `undefined` options read process env; a dictionary is used as-is. |

###### Datasource(provider, driver, options?) {#typescript-datasource-datasource-provider-driver-options}

Builds a datasource from a provider [IDriver](#typescript-idriver).

**Definition**

```typescript
constructor(provider: string, driver: IDriver, options?: DatasourceOptions)
```

**Parameters**

| Name | Description |
| --- | --- |
| `provider` | registered provider name |
| `driver` | [IDriver](#typescript-idriver) that builds connections, commands, transactions, and row converters |
| `options` | [DatasourceOptions](#typescript-datasourceoptions) passed to `driver.connection` |

###### create(options?) {#typescript-datasource-create-options}

Alias of [DatasourceFactory.create](#datasource-factory). `undefined` options read process env; a dictionary is used as-is.

**Definition**

```typescript
static create(options?: DatasourceOptions): IDatasource
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | [DatasourceOptions](#typescript-datasourceoptions); omitted loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL` |

**Returns**

[IDatasource](#typescript-idatasource)

##### IDatasource {#typescript-idatasource}

```typescript
interface IDatasource
```

**Properties**

| Name | Description |
| --- | --- |
| provider | Registered provider name. |

**Methods**

| Name | Description |
| --- | --- |
| [createConnection()](#typescript-idatasource-createconnection) | Creates a connection. |
| [createCommand(connection?)](#typescript-idatasource-createcommand-connection) | Creates a command. |
| [createRepository(type, connection?, options?)](#typescript-idatasource-createrepository-type-connection-options) | Creates a CRUD repository. |
| [createReadonlyRepository(type, connection?, options?)](#typescript-idatasource-createreadonlyrepository-type-connection-options) | Creates a read-only repository. |

###### createConnection() {#typescript-idatasource-createconnection}

Creates a connection.

**Definition**

```typescript
createConnection(): Promise<IConnection>
```

**Returns**

`Promise<[IConnection](#connection)>`

###### createCommand(connection?) {#typescript-idatasource-createcommand-connection}

Creates a command.

**Definition**

```typescript
createCommand(connection?: IConnection): ICommand
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional [IConnection](#connection); omitted uses the datasource connection |

**Returns**

[ICommand](#command)

###### createRepository(type, connection?, options?) {#typescript-idatasource-createrepository-type-connection-options}

Creates a CRUD repository.

**Definition**

```typescript
createRepository<T>(type: Constructable<T>, connection?: IConnection, options?: CRUDRepositoryOptions<T>): ICRUDRepository<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `type` | class constructor. Its fields become columns, `id` or `Id` becomes the id, and `type.name` becomes the entity |
| `connection` | optional [IConnection](#connection); omitted uses the datasource connection |
| `options` | optional [CRUDRepositoryOptions](#typescript-crudrepositoryoptions). Entity, columns, and mappers overlay the type defaults |

**Returns**

[ICRUDRepository](#crud-repository)`<T>`

###### createReadonlyRepository(type, connection?, options?) {#typescript-idatasource-createreadonlyrepository-type-connection-options}

Creates a read-only repository.

**Definition**

```typescript
createReadonlyRepository<T>(type: Constructable<T>, connection?: IConnection, options?: ReadonlyRepositoryOptions<T>): IReadonlyRepository<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `type` | class constructor. Its fields become columns, `id` or `Id` becomes the id, and `type.name` becomes the entity |
| `connection` | optional [IConnection](#connection); omitted uses the datasource connection |
| `options` | optional [ReadonlyRepositoryOptions](#typescript-readonlyrepositoryoptions). Entity, columns, and mappers overlay the type defaults |

**Returns**

[IReadonlyRepository](#readonly-repository)`<T>`

##### DatasourceOptions {#typescript-datasourceoptions}

```typescript
type DatasourceOptions = Record<string, string>
```

**Properties**

| Name | Description |
| --- | --- |
| PROVIDER | Which datasource to return. |
| DATASOURCE_URL | Connection URL. Schema is the `schema` query (`postgres://host/db?schema=public`). |

##### C#

##### Datasource {#csharp-datasource}

```csharp
public sealed class Datasource : IDatasource
```

**Constructor**

| Name | Description |
| --- | --- |
| [Datasource(provider, driver, options)](#csharp-datasource-datasource-provider-driver-options) | Builds a datasource from a provider [IDriver](#csharp-idriver). |

**Properties**

| Name | Description |
| --- | --- |
| Default | Loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`. Throws when `PROVIDER` is not set. Calls [DatasourceFactory.Create](#csharp-create-options) with no options. |

###### Datasource(provider, driver, options) {#csharp-datasource-datasource-provider-driver-options}

Builds a datasource from a provider [IDriver](#csharp-idriver).

**Definition**

```csharp
public Datasource(string provider, IDriver driver, IReadOnlyDictionary<string, string?>? options = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `provider` | registered provider name |
| `driver` | [IDriver](#csharp-idriver) that builds connections, commands, transactions, and row converters |
| `options` | [DatasourceOptions](#csharp-datasourceoptions) passed to `driver.Connection` |

##### IDatasource {#csharp-idatasource}

```csharp
public interface IDatasource
```

**Properties**

| Name | Description |
| --- | --- |
| Provider | Registered provider name. |

**Methods**

| Name | Description |
| --- | --- |
| [CreateConnection()](#csharp-idatasource-createconnection) | Creates a connection. |
| [CreateCommand(connection)](#csharp-idatasource-createcommand-connection) | Creates a command. |
| [CreateRepository(connection, options)](#csharp-idatasource-createrepository-connection-options) | Creates a CRUD repository. |
| [CreateReadonlyRepository(connection, options)](#csharp-idatasource-createreadonlyrepository-connection-options) | Creates a read-only repository. |

###### CreateConnection() {#csharp-idatasource-createconnection}

Creates a connection.

**Definition**

```csharp
Task<IConnection> CreateConnection()
```

**Returns**

`Task<[IConnection](#connection)>`

###### CreateCommand(connection) {#csharp-idatasource-createcommand-connection}

Creates a command.

**Definition**

```csharp
ICommand CreateCommand(IConnection? connection = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional [IConnection](#connection); omitted uses the datasource connection |

**Returns**

[ICommand](#command)

###### CreateRepository(connection, options) {#csharp-idatasource-createrepository-connection-options}

Creates a CRUD repository.

**Definition**

```csharp
ICRUDRepository<T> CreateRepository<T>(IConnection? connection = null, CRUDRepositoryOptions<T>? options = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional [IConnection](#connection); omitted uses the datasource connection |
| `options` | optional [CRUDRepositoryOptions](#csharp-crudrepositoryoptions). Entity, columns, and mappers overlay the type defaults |

**Returns**

[ICRUDRepository](#crud-repository)`<T>`

###### CreateReadonlyRepository(connection, options) {#csharp-idatasource-createreadonlyrepository-connection-options}

Creates a read-only repository.

**Definition**

```csharp
IReadonlyRepository<T> CreateReadonlyRepository<T>(IConnection? connection = null, ReadonlyRepositoryOptions<T>? options = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional [IConnection](#connection); omitted uses the datasource connection |
| `options` | optional [ReadonlyRepositoryOptions](#csharp-readonlyrepositoryoptions). Entity, columns, and mappers overlay the type defaults |

**Returns**

[IReadonlyRepository](#readonly-repository)`<T>`

##### DatasourceOptions {#csharp-datasourceoptions}

```csharp
IReadOnlyDictionary<string, string?>
```

**Properties**

| Name | Description |
| --- | --- |
| PROVIDER | Which datasource to return. |
| DATASOURCE_URL | Connection URL. Schema is the `schema` query (`postgres://host/db?schema=public`). |

##### Rust

##### BuiltDatasource {#rust-builtdatasource}

```rust
pub struct BuiltDatasource
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(provider, driver, options)](#rust-builtdatasource-new-provider-driver-options) | Builds a datasource from a provider [Driver](#rust-driver). |

###### new(provider, driver, options) {#rust-builtdatasource-new-provider-driver-options}

Builds a datasource from a provider [Driver](#rust-driver).

**Definition**

```rust
pub fn new(provider: impl Into<String>, driver: Arc<dyn Driver>, options: Option<HashMap<String, String>>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `provider` | registered provider name |
| `driver` | [Driver](#rust-driver) that builds connections, commands, transactions, and row converters |
| `options` | [DatasourceOptions](#rust-datasourceoptions) passed to `driver.connection` |

##### datasources {#rust-datasources}

```rust
// datasources
```

**Methods**

| Name | Description |
| --- | --- |
| [default()](#rust-datasources-default) | Loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`. Throws when `PROVIDER` is not set. Calls [create](#rust-create-options) with no options. |

###### default() {#rust-datasources-default}

Loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`. Throws when `PROVIDER` is not set. Calls [create](#rust-create-options) with no options.

**Definition**

```rust
pub fn default() -> Result<Arc<dyn Datasource>>
```

**Returns**

`Result<Arc<dyn `[Datasource](#rust-datasource)`>>`

##### Datasource {#rust-datasource}

```rust
pub trait Datasource
```

**Methods**

| Name | Description |
| --- | --- |
| [provider()](#rust-datasource-provider) | Registered provider name. |
| [create_connection()](#rust-datasource-create_connection) | Creates a connection. |
| [create_command(connection)](#rust-datasource-create_command-connection) | Creates a command. |
| [create_repository(connection, options)](#rust-datasource-create_repository-connection-options) | Creates a CRUD repository. |
| [create_readonly_repository(connection, options)](#rust-datasource-create_readonly_repository-connection-options) | Creates a read-only repository. |

###### provider() {#rust-datasource-provider}

Registered provider name.

**Definition**

```rust
fn provider(&self) -> &str
```

**Returns**

`&str`

###### create_connection() {#rust-datasource-create_connection}

Creates a connection.

**Definition**

```rust
async fn create_connection(&self) -> Result<Arc<dyn Connection>>
```

**Returns**

`Result<Arc<dyn `[Connection](#connection)`>>`

###### create_command(connection) {#rust-datasource-create_command-connection}

Creates a command.

**Definition**

```rust
fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command>
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional [Connection](#connection); omitted uses the datasource connection |

**Returns**

`Arc<dyn `[Command](#command)`>`

###### create_repository(connection, options) {#rust-datasource-create_repository-connection-options}

Creates a CRUD repository.

**Definition**

```rust
fn create_repository<T: ConvertSchema + 'static>(&self, connection: Option<Arc<dyn Connection>>, options: Option<CRUDRepositoryOptions>) -> Arc<dyn ICRUDRepository>
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional [Connection](#connection); omitted uses the datasource connection |
| `options` | optional [CRUDRepositoryOptions](#rust-crudrepositoryoptions). Entity, columns, and mappers overlay the type defaults. `None` uses the type defaults |

**Returns**

`Arc<dyn `[CRUDRepository](#crud-repository)`>`

###### create_readonly_repository(connection, options) {#rust-datasource-create_readonly_repository-connection-options}

Creates a read-only repository.

**Definition**

```rust
fn create_readonly_repository<T: ConvertSchema + 'static>(&self, connection: Option<Arc<dyn Connection>>, options: Option<ReadonlyRepositoryOptions>) -> Arc<dyn IReadonlyRepository>
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional [Connection](#connection); omitted uses the datasource connection |
| `options` | optional [ReadonlyRepositoryOptions](#rust-readonlyrepositoryoptions). Entity, columns, and mappers overlay the type defaults. `None` uses the type defaults |

**Returns**

`Arc<dyn `[ReadonlyRepository](#readonly-repository)`>`

##### DatasourceOptions {#rust-datasourceoptions}

```rust
HashMap<String, String>
```

**Properties**

| Name | Description |
| --- | --- |
| PROVIDER | Which datasource to return. |
| DATASOURCE_URL | Connection URL. Schema is the `schema` query (`postgres://host/db?schema=public`). |

### Remarks

`Datasource.Default` / `Datasource.default` / `default()` loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`. `DatasourceFactory.Create(null)` / `DatasourceFactory.create()` / `create(None)` uses the same path. A non-null options bag is used as-is; missing `PROVIDER` throws.

Each dialect loads an installed provider with `DriverLoader` and `ProviderCatalog`.

> **TypeScript**
>
> Load the provider package.

> **C#**
>
> Load the assembly.

> **Rust**
>
> Load the provider cdylib.

`Reset` / `reset` / `reset` clears the registry. Use it in tests.

See [DatasourceFactory](#datasource-factory) for the factory contract, and [CLI](#command-line-cli) for `setup` and adding providers.

### Examples

#### Create default from .env settings

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/DatasourceTests.cs#L24-L31)

```csharp
var ds = Datasource.Default;
var command = ds.CreateCommand();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/datasource.rs#L13-L21)

```rust
let ds = default().expect("default");
let command = ds.create_command(None);
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/Datasource.test.ts#L27-L32)

```typescript
const ds = Datasource.default;
const command = ds.createCommand();
```

#### Default using environment variables

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/DatasourceFactoryTests.cs#L22-L29)

```csharp
Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", "memory");
Environment.SetEnvironmentVariable("PROVIDER", "memory");
Environment.SetEnvironmentVariable("DATASOURCE_URL", "memory://");
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/datasource_factory.rs#L15-L23)

```rust
register("sqlite", MockDriver::arc());
std::env::set_var("PROVIDER", "sqlite");
let ds = default().expect("default");
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/DatasourceFactory.test.ts#L43-L48)

```typescript
DatasourceFactory.register("sqlite", new MockDriver());
process.env.PROVIDER = "sqlite";
const ds = Datasource.default;
```

#### Create from a driver

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/CreateFromDriverTests.cs#L8-L15)

```csharp
var driver = new SqliteDriver();
var ds = new Datasource("sqlite", driver, new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
var repo = ds.CreateRepository<Item>();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/create_from_driver.rs#L15-L29)

```rust
let driver = Arc::new(SqliteDriver);
let ds: Arc<dyn Datasource> = Arc::new(BuiltDatasource::new(
    "sqlite",
    driver,
    Some(HashMap::from([(
        "DATASOURCE_URL".into(),
        "sqlite:///:memory:".into(),
    )])),
));
let repo = ds.create_repository::<Item>(None, None);
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/src/createFromDriver.test.ts#L10-L15)

```typescript
const driver = new SqliteDriver();
const ds = new Datasource("sqlite", driver, { DATASOURCE_URL: "sqlite:///:memory:" });
const repo = ds.createRepository(Item);
```

## Connection

Connection represents a connection to a datasource. You can create commands and transactions from the connection.

### Classes

##### TypeScript

##### IConnection {#typescript-iconnection}

```typescript
interface IConnection
```

**Properties**

| Name | Description |
| --- | --- |
| provider | Registered provider name. |
| isOpen | Whether the connection is open. |

**Methods**

| Name | Description |
| --- | --- |
| [createCommand()](#typescript-iconnection-createcommand) | Creates a command bound to this connection. |
| [open()](#typescript-iconnection-open) | Opens the connection. Throws if it is already open. |
| [close()](#typescript-iconnection-close) | Closes the connection. |
| [[Symbol.asyncDispose]()](#typescript-iconnection-symbol-asyncdispose) | Closes the connection. `await using` calls this. |
| [startTransaction(isolation?)](#typescript-iconnection-starttransaction-isolation) | Begins a transaction. The connection must be open. |

###### createCommand() {#typescript-iconnection-createcommand}

Creates a command bound to this connection.

**Definition**

```typescript
createCommand(): ICommand
```

**Returns**

[ICommand](#command)

###### open() {#typescript-iconnection-open}

Opens the connection. Throws if it is already open.

**Definition**

```typescript
open(): Promise<void>
```

###### close() {#typescript-iconnection-close}

Closes the connection.

**Definition**

```typescript
close(): Promise<void>
```

###### [Symbol.asyncDispose]() {#typescript-iconnection-symbol-asyncdispose}

Closes the connection. `await using` calls this.

**Definition**

```typescript
[Symbol.asyncDispose](): Promise<void>
```

###### startTransaction(isolation?) {#typescript-iconnection-starttransaction-isolation}

Begins a transaction. The connection must be open.

**Definition**

```typescript
startTransaction(isolation?: IsolationLevel): Promise<ITransaction>
```

**Parameters**

| Name | Description |
| --- | --- |
| `isolation` | [IsolationLevel](#typescript-isolationlevel); omitted uses read committed |

**Returns**

`Promise<[ITransaction](#transaction)>`

##### C#

##### IConnection {#csharp-iconnection}

```csharp
public interface IConnection : IAsyncDisposable
```

**Properties**

| Name | Description |
| --- | --- |
| Provider | Registered provider name. |
| IsOpen | Whether the connection is open. |

**Methods**

| Name | Description |
| --- | --- |
| [CreateCommand()](#csharp-iconnection-createcommand) | Creates a command bound to this connection. |
| [Open()](#csharp-iconnection-open) | Opens the connection. Throws if it is already open. |
| [Close()](#csharp-iconnection-close) | Closes the connection. |
| [DisposeAsync()](#csharp-iconnection-disposeasync) | Closes the connection. `await using` calls this. |
| [StartTransaction(isolation)](#csharp-iconnection-starttransaction-isolation) | Begins a transaction. The connection must be open. |

###### CreateCommand() {#csharp-iconnection-createcommand}

Creates a command bound to this connection.

**Definition**

```csharp
ICommand CreateCommand()
```

**Returns**

[ICommand](#command)

###### Open() {#csharp-iconnection-open}

Opens the connection. Throws if it is already open.

**Definition**

```csharp
Task Open()
```

###### Close() {#csharp-iconnection-close}

Closes the connection.

**Definition**

```csharp
Task Close()
```

###### DisposeAsync() {#csharp-iconnection-disposeasync}

Closes the connection. `await using` calls this.

**Definition**

```csharp
ValueTask DisposeAsync()
```

**Returns**

`ValueTask`

###### StartTransaction(isolation) {#csharp-iconnection-starttransaction-isolation}

Begins a transaction. The connection must be open.

**Definition**

```csharp
Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.ReadCommitted)
```

**Parameters**

| Name | Description |
| --- | --- |
| `isolation` | [IsolationLevel](#csharp-isolationlevel); default is read committed |

**Returns**

`Task<[ITransaction](#transaction)>`

##### Rust

##### Connection {#rust-connection}

```rust
pub trait Connection: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [provider()](#rust-connection-provider) | Registered provider name. |
| [is_open()](#rust-connection-is_open) | Whether the connection is open. |
| [create_command()](#rust-connection-create_command) | Creates a command bound to this connection. |
| [open()](#rust-connection-open) | Opens the connection. Throws if it is already open. |
| [close()](#rust-connection-close) | Closes the connection. |
| [start_transaction(isolation)](#rust-connection-start_transaction-isolation) | Begins a transaction. The connection must be open. |

###### provider() {#rust-connection-provider}

Registered provider name.

**Definition**

```rust
fn provider(&self) -> &str
```

**Returns**

`&str`

###### is_open() {#rust-connection-is_open}

Whether the connection is open.

**Definition**

```rust
fn is_open(&self) -> bool
```

**Returns**

`bool`

###### create_command() {#rust-connection-create_command}

Creates a command bound to this connection.

**Definition**

```rust
fn create_command(&self) -> Arc<dyn Command>
```

**Returns**

`Arc<dyn `[Command](#command)`>`

###### open() {#rust-connection-open}

Opens the connection. Throws if it is already open.

**Definition**

```rust
async fn open(&self) -> Result<()>
```

**Returns**

`Result<()>`

###### close() {#rust-connection-close}

Closes the connection.

**Definition**

```rust
async fn close(&self) -> Result<()>
```

**Returns**

`Result<()>`

###### start_transaction(isolation) {#rust-connection-start_transaction-isolation}

Begins a transaction. The connection must be open.

**Definition**

```rust
async fn start_transaction(&self, isolation: Option<IsolationLevel>) -> Result<Arc<dyn Transaction>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `isolation` | [IsolationLevel](#rust-isolationlevel); `None` uses read committed |

**Returns**

`Result<Arc<dyn `[Transaction](#transaction)`>>`

### Remarks

`CreateConnection` / `createConnection` / `create_connection` returns a closed connection. It is not open until `Open` / `open` / `open`. Calling `Open` / `open` / `open` on an open connection throws.

> **TypeScript**
>
> Close the connection at the end of an `await using` block, or call `close`.

> **C#**
>
> Close the connection at the end of an `await using` block, or call `Close`.

> **Rust**
>
> There is no `using`. Call `close().await`.

`StartTransaction` / `startTransaction` / `start_transaction` requires an open connection. Memory, JSON, and SQLite throw when the connection is closed.

Working dialects (`json`, `memory`) open against injected ports (`IFileStore` / `IFileStore` / `FileStore`, `MemoryStore`). SQLite, MySQL, and Postgres open the database client on the connection. Stub dialects throw not-implemented on open, close, and start-transaction.

### Examples

#### Create from Default

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/ConnectionTests.cs#L24-L30)

```csharp
var ds = Datasource.Default;
var connection = await ds.CreateConnection();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/connection.rs#L16-L25)

```rust
let ds = {
    let _guard = super::env_lock();
    clear_env();
    default().expect("default")
};
let connection = ds.create_connection().await.expect("connection");
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/Connection.test.ts#L27-L31)

```typescript
const ds = Datasource.default;
const connection = await ds.createConnection();
```

#### Open and close

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Memory.Tests/ConnectionLifecycleTests.cs#L8-L15)

```csharp
var connection = new MemoryConnection();
await connection.Open();
await connection.Close();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L48-L53)

```rust
let connection = MemoryConnection::from_options(None);
connection.open().await.unwrap();
connection.close().await.unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/memory/src/index.test.ts#L40-L45)

```typescript
const connection = new MemoryConnection();
await connection.open();
await connection.close();
```

#### Create and run a command

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Memory.Tests/ConnectionLifecycleTests.cs#L35-L43)

```csharp
var connection = new MemoryConnection();
await connection.Open();
var command = connection.CreateCommand();
var response = await command.Execute("insert");
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L66-L73)

```rust
let connection = MemoryConnection::from_options(None);
connection.open().await.unwrap();
let command = connection.create_command();
let response = command.execute("insert", None).await.unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/memory/src/index.test.ts#L60-L66)

```typescript
const connection = new MemoryConnection();
await connection.open();
const command = connection.createCommand();
await command.execute("insert");
```

#### Close with using

> **Rust**
>
> There is no `using`. Call `close`.

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Memory.Tests/ConnectionLifecycleTests.cs#L45-L50)

```csharp
await using var connection = new MemoryConnection();
await connection.Open();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L48-L54)

```rust
let connection = MemoryConnection::from_options(None);
connection.open().await.unwrap();
connection.close().await.unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/memory/src/index.test.ts#L68-L75)

```typescript
const connection = new MemoryConnection();
{
  await using held = connection;
  await held.open();
}
```

#### Start a transaction

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Memory.Tests/ConnectionLifecycleTests.cs#L26-L33)

```csharp
var connection = new MemoryConnection();
await connection.Open();
var txn = await connection.StartTransaction();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L75-L80)

```rust
let connection = MemoryConnection::from_options(None);
connection.open().await.unwrap();
let txn = connection.start_transaction(None).await.unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/memory/src/index.test.ts#L53-L58)

```typescript
const connection = new MemoryConnection();
await connection.open();
const txn = await connection.startTransaction();
```

## Transaction

A transaction is a single unit of work. It lets you run multiple commands and ensure they succeed together.

### Classes

##### TypeScript

##### ITransaction {#typescript-itransaction}

```typescript
interface ITransaction
```

**Properties**

| Name | Description |
| --- | --- |
| connection | The open [IConnection](#connection) that started this transaction. |
| isolation | The isolation level for this transaction. |

**Methods**

| Name | Description |
| --- | --- |
| [commit()](#typescript-itransaction-commit) | Commits the transaction. |
| [abort()](#typescript-itransaction-abort) | Rolls the transaction back. |

###### commit() {#typescript-itransaction-commit}

Commits the transaction.

**Definition**

```typescript
commit(): Promise<void>
```

###### abort() {#typescript-itransaction-abort}

Rolls the transaction back.

**Definition**

```typescript
abort(): Promise<void>
```

##### IsolationLevel {#typescript-isolationlevel}

```typescript
const IsolationLevel
```

**Properties**

| Name | Description |
| --- | --- |
| ReadUncommitted | Dirty reads are allowed. |
| ReadCommitted | Committed data only. The default for [startTransaction](#typescript-iconnection-starttransaction-isolation). |
| RepeatableRead | Repeatable reads within the transaction. |
| Serializable | Full isolation. |

##### C#

##### ITransaction {#csharp-itransaction}

```csharp
public interface ITransaction
```

**Properties**

| Name | Description |
| --- | --- |
| Connection | The open [IConnection](#connection) that started this transaction. |
| Isolation | The isolation level for this transaction. |

**Methods**

| Name | Description |
| --- | --- |
| [Commit()](#csharp-itransaction-commit) | Commits the transaction. |
| [Abort()](#csharp-itransaction-abort) | Rolls the transaction back. |

###### Commit() {#csharp-itransaction-commit}

Commits the transaction.

**Definition**

```csharp
Task Commit()
```

###### Abort() {#csharp-itransaction-abort}

Rolls the transaction back.

**Definition**

```csharp
Task Abort()
```

##### IsolationLevel {#csharp-isolationlevel}

```csharp
public enum IsolationLevel
```

**Properties**

| Name | Description |
| --- | --- |
| ReadUncommitted | Dirty reads are allowed. |
| ReadCommitted | Committed data only. The default for [StartTransaction](#csharp-iconnection-starttransaction-isolation). |
| RepeatableRead | Repeatable reads within the transaction. |
| Serializable | Full isolation. |

##### Rust

##### Transaction {#rust-transaction}

```rust
pub trait Transaction: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [isolation()](#rust-transaction-isolation) | The isolation level for this transaction. |
| [commit()](#rust-transaction-commit) | Commits the transaction. |
| [abort()](#rust-transaction-abort) | Rolls the transaction back. |

###### isolation() {#rust-transaction-isolation}

The isolation level for this transaction.

**Definition**

```rust
fn isolation(&self) -> IsolationLevel
```

**Returns**

[IsolationLevel](#rust-isolationlevel)

###### commit() {#rust-transaction-commit}

Commits the transaction.

**Definition**

```rust
async fn commit(&self) -> Result<()>
```

**Returns**

`Result<()>`

###### abort() {#rust-transaction-abort}

Rolls the transaction back.

**Definition**

```rust
async fn abort(&self) -> Result<()>
```

**Returns**

`Result<()>`

##### IsolationLevel {#rust-isolationlevel}

```rust
pub enum IsolationLevel
```

**Properties**

| Name | Description |
| --- | --- |
| ReadUncommitted | Dirty reads are allowed. |
| ReadCommitted | Committed data only. The default for [start_transaction](#rust-connection-start_transaction-isolation). |
| RepeatableRead | Repeatable reads within the transaction. |
| Serializable | Full isolation. |

### Remarks

`StartTransaction` / `startTransaction` / `start_transaction` requires an open [connection](#connection). It returns a transaction that is not committed.

`Commit` / `commit` / `commit` persists work. `Abort` / `abort` / `abort` rolls it back.

> **TypeScript**
>
> Omit the argument.

> **C#**
>
> The parameter defaults to `ReadCommitted`.

> **Rust**
>
> Pass `None`.

### Examples

#### Commit or abort

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/TransactionTests.cs#L24-L37)

```csharp
var ds = Datasource.Default;
var connection = await ds.CreateConnection();
await connection.Open();
var txn = await connection.StartTransaction();
var saved = true;
if (saved)
    await txn.Commit();
else
    await txn.Abort();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/transaction.rs#L16-L31)

```rust
let ds = default().expect("default");
let connection = ds.create_connection().await.expect("connection");
connection.open().await.expect("open");
let txn = connection.start_transaction(None).await.expect("txn");
let saved = true;
if saved {
    txn.commit().await.expect("commit");
} else {
    txn.abort().await.expect("abort");
}
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/Transaction.test.ts#L27-L36)

```typescript
const ds = Datasource.default;
const connection = await ds.createConnection();
await connection.open();
const txn = await connection.startTransaction();
const saved = true;
if (saved) await txn.commit();
else await txn.abort();
```

#### Two commands running together

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Memory.Tests/TransactionLifecycleTests.cs#L28-L43)

```csharp
var connection = new MemoryConnection();
await connection.Open();
var txn = await connection.StartTransaction();
var command = connection.CreateCommand();
var fields = new Parameter[] { new("name", "ada") };
await command.Execute(new InsertStatement("people", fields), fields);
var reader = await command.ExecuteReader(new SelectInsertedIdStatement("people", "id"));
var moved = await reader.Next();
var id = reader.Current!["id"];
await txn.Commit();
```

**Created id**

```
{
  id: 1
}
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L101-L124)

```rust
let connection = MemoryConnection::from_options(None);
connection.open().await.unwrap();
let txn = connection.start_transaction(None).await.unwrap();
let command = connection.create_command();
let fields = vec![Parameter::new("name", "ada")];
command
    .execute_statement(&InsertStatement::new("people", &fields), Some(&fields))
    .await
    .unwrap();
let reader = command
    .execute_reader_statement(&SelectInsertedIdStatement::new("people", "id"), None)
    .await
    .unwrap();
let moved = reader.next().await.unwrap();
let id = reader.current();
txn.commit().await.unwrap();
```

**Created id**

```
{
  id: 1
}
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/memory/src/index.test.ts#L95-L108)

```typescript
const connection = new MemoryConnection();
await connection.open();
const txn = await connection.startTransaction();
const command = connection.createCommand();
const fields = [{ name: "name", value: "ada" }];
await command.executeStatement(new InsertStatement("people", fields), fields);
const reader = await command.executeReaderStatement(new SelectInsertedIdStatement("people", "id"));
const moved = await reader.next();
const id = reader.current;
await txn.commit();
```

**Created id**

```
{
  id: 1
}
```

## Command

Executes a command against the datasource and returns a result or a streaming [Reader](#reader).

### Classes

##### TypeScript

##### ICommand {#typescript-icommand}

```typescript
interface ICommand
```

**Properties**

| Name | Description |
| --- | --- |
| connection | The [IConnection](#connection) this command is bound to. |

**Methods**

| Name | Description |
| --- | --- |
| [addMiddleware(middleware)](#typescript-icommand-addmiddleware-middleware) | Adds [ICommandMiddleware](#typescript-icommandmiddleware). Returns this command. |
| [executeReader(text, parameters?)](#typescript-icommand-executereader-text-parameters) | Parses `text` with [StatementParser.parse](#statements) and runs [executeReaderStatement](#typescript-icommand-executereaderstatement-statement-parameters). Blank text runs [executeReaderNative](#typescript-icommand-executereadernative-text-parameters). |
| [execute(text, parameters?)](#typescript-icommand-execute-text-parameters) | Parses `text` with [StatementParser.parse](#statements) and runs [executeStatement](#typescript-icommand-executestatement-statement-parameters). Blank text runs [executeNative](#typescript-icommand-executenative-text-parameters). |
| [executeReaderNative(text, parameters?)](#typescript-icommand-executereadernative-text-parameters) | Runs provider text without parsing it. |
| [executeNative(text, parameters?)](#typescript-icommand-executenative-text-parameters) | Runs provider text without parsing it. |
| [execute(command, statement, parameters?)](#typescript-icommand-execute-command-statement-parameters) | Runs [executeStatement](#typescript-icommand-executestatement-statement-parameters) when the command implements it, otherwise [executeNative](#typescript-icommand-executenative-text-parameters) with `statement.getText()`. |
| [executeReader(command, statement, parameters?)](#typescript-icommand-executereader-command-statement-parameters) | Runs [executeReaderStatement](#typescript-icommand-executereaderstatement-statement-parameters) when the command implements it, otherwise [executeReaderNative](#typescript-icommand-executereadernative-text-parameters) with `statement.getText()`. |

###### addMiddleware(middleware) {#typescript-icommand-addmiddleware-middleware}

Adds [ICommandMiddleware](#typescript-icommandmiddleware). Returns this command.

**Definition**

```typescript
addMiddleware(middleware: ICommandMiddleware): this
```

**Parameters**

| Name | Description |
| --- | --- |
| `middleware` | [ICommandMiddleware](#typescript-icommandmiddleware) |

**Returns**

`this`

###### executeReader(text, parameters?) {#typescript-icommand-executereader-text-parameters}

Parses `text` with [StatementParser.parse](#statements) and runs [executeReaderStatement](#typescript-icommand-executereaderstatement-statement-parameters). Blank text runs [executeReaderNative](#typescript-icommand-executereadernative-text-parameters).

**Definition**

```typescript
executeReader(text: string, parameters?: Parameter[]): Promise<IReader<Cells>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL parsed as a statement |
| `parameters` | optional [Parameter](#typescript-parameter) list |

**Returns**

`Promise<[IReader](#reader)<Cells>>`

###### execute(text, parameters?) {#typescript-icommand-execute-text-parameters}

Parses `text` with [StatementParser.parse](#statements) and runs [executeStatement](#typescript-icommand-executestatement-statement-parameters). Blank text runs [executeNative](#typescript-icommand-executenative-text-parameters).

**Definition**

```typescript
execute(text: string, parameters?: Parameter[]): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL parsed as a statement |
| `parameters` | optional [Parameter](#typescript-parameter) list |

**Returns**

`Promise<[Response](#typescript-response)>`

###### executeReaderNative(text, parameters?) {#typescript-icommand-executereadernative-text-parameters}

Runs provider text without parsing it.

**Definition**

```typescript
executeReaderNative(text: Native, parameters?: Parameter[]): Promise<IReader<Cells>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | [Native](#typescript-native) engine text |
| `parameters` | optional [Parameter](#typescript-parameter) list |

**Returns**

`Promise<[IReader](#reader)<Cells>>`

###### executeNative(text, parameters?) {#typescript-icommand-executenative-text-parameters}

Runs provider text without parsing it.

**Definition**

```typescript
executeNative(text: Native, parameters?: Parameter[]): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | [Native](#typescript-native) engine text |
| `parameters` | optional [Parameter](#typescript-parameter) list |

**Returns**

`Promise<[Response](#typescript-response)>`

###### execute(command, statement, parameters?) {#typescript-icommand-execute-command-statement-parameters}

Runs [executeStatement](#typescript-icommand-executestatement-statement-parameters) when the command implements it, otherwise [executeNative](#typescript-icommand-executenative-text-parameters) with `statement.getText()`.

**Definition**

```typescript
function execute(command: ICommand, statement: IStatement, parameters?: Parameter[]): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `command` | [ICommand](#typescript-icommand) |
| `statement` | [IStatement](#typescript-istatement) |
| `parameters` | optional [Parameter](#typescript-parameter) list |

**Returns**

`Promise<[Response](#typescript-response)>`

###### executeReader(command, statement, parameters?) {#typescript-icommand-executereader-command-statement-parameters}

Runs [executeReaderStatement](#typescript-icommand-executereaderstatement-statement-parameters) when the command implements it, otherwise [executeReaderNative](#typescript-icommand-executereadernative-text-parameters) with `statement.getText()`.

**Definition**

```typescript
function executeReader(command: ICommand, statement: IStatement, parameters?: Parameter[]): Promise<IReader<Cells>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `command` | [ICommand](#typescript-icommand) |
| `statement` | [IStatement](#typescript-istatement) |
| `parameters` | optional [Parameter](#typescript-parameter) list |

**Returns**

`Promise<[IReader](#reader)<Cells>>`

##### Native {#typescript-native}

```typescript
class Native
```

**Constructor**

| Name | Description |
| --- | --- |
| [Native(text)](#typescript-native-native-text) | Engine text. `n(text)` builds the same value. |

**Properties**

| Name | Description |
| --- | --- |
| text | Provider command text. |

###### Native(text) {#typescript-native-native-text}

Engine text. `n(text)` builds the same value.

**Definition**

```typescript
constructor(text: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | provider command text |

**Returns**

[Native](#typescript-native)

##### ICommandMiddleware {#typescript-icommandmiddleware}

```typescript
interface ICommandMiddleware
```

**Methods**

| Name | Description |
| --- | --- |
| [beforeCommand(text, params?)](#typescript-icommandmiddleware-beforecommand-text-params) | Runs before the command. A thrown error skips the command. A `{ text, params }` return rewrites it. |
| [afterCommand(text, params, results, elapsedMs, error?)](#typescript-icommandmiddleware-aftercommand-text-params-results-elapsedms-error) | Runs after the command, including when the command failed. |

###### beforeCommand(text, params?) {#typescript-icommandmiddleware-beforecommand-text-params}

Runs before the command. A thrown error skips the command. A `{ text, params }` return rewrites it.

**Definition**

```typescript
beforeCommand(text: string, params?: ReadonlyArray<Parameter>): void | { text: string; params?: ReadonlyArray<Parameter> } | Promise<void | { text: string; params?: ReadonlyArray<Parameter> }>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `params` | optional [Parameter](#typescript-parameter) list |

###### afterCommand(text, params, results, elapsedMs, error?) {#typescript-icommandmiddleware-aftercommand-text-params-results-elapsedms-error}

Runs after the command, including when the command failed.

**Definition**

```typescript
afterCommand(text: string, params: ReadonlyArray<Parameter> | undefined, results: ReadonlyArray<Row>, elapsedMs: number, error?: Error): void | Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `params` | [Parameter](#typescript-parameter) list used |
| `results` | rows produced |
| `elapsedMs` | elapsed milliseconds |
| `error` | command error when the command failed |

##### Parameter {#typescript-parameter}

```typescript
interface Parameter<T = Scalar>
```

**Properties**

| Name | Description |
| --- | --- |
| name | Parameter name. |
| value | Parameter value. Omitted for [output](#typescript-parameterdirection) / [returnValue](#typescript-parameterdirection). |
| direction | Bind direction. Omitted means [input](#typescript-parameterdirection). |

##### ParameterDirection {#typescript-parameterdirection}

```typescript
const ParameterDirection
```

##### Response {#typescript-response}

```typescript
class Response
```

**Constructor**

| Name | Description |
| --- | --- |
| [Response(result, rowsAffected, parameters?)](#typescript-response-response-result-rowsaffected-parameters) | Builds a [Response](#typescript-response). Copies a [returnValue](#typescript-parameterdirection) onto [result](#typescript-response-result) when `result` is null. |

**Properties**

| Name | Description |
| --- | --- |
| result | Scalar or row result from the command. |
| rowsAffected | Rows inserted, updated, or deleted. |
| parameters | Bound [Parameter](#typescript-parameter) list after [execute](#typescript-icommand-execute-text-parameters). Holds filled [output](#typescript-parameterdirection) / [returnValue](#typescript-parameterdirection) values. |

###### Response(result, rowsAffected, parameters?) {#typescript-response-response-result-rowsaffected-parameters}

Builds a [Response](#typescript-response). Copies a [returnValue](#typescript-parameterdirection) onto [result](#typescript-response-result) when `result` is null.

**Definition**

```typescript
constructor(result: unknown, rowsAffected: number, parameters?: Parameter[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `result` | scalar or row result |
| `rowsAffected` | rows inserted, updated, or deleted |
| `parameters` | bound [Parameter](#typescript-parameter) list |

**Returns**

[Response](#typescript-response)

##### C#

##### ICommand {#csharp-icommand}

```csharp
public interface ICommand
```

**Properties**

| Name | Description |
| --- | --- |
| Connection | The [IConnection](#connection) this command is bound to. |

**Methods**

| Name | Description |
| --- | --- |
| [AddMiddleware(middleware)](#csharp-icommand-addmiddleware-middleware) | Adds [ICommandMiddleware](#csharp-icommandmiddleware). Returns this command. |
| [ExecuteReader(text, parameters)](#csharp-icommand-executereader-text-parameters) | Parses `text` with [StatementParser.Parse](#statements) and runs [ExecuteReader(statement)](#csharp-icommand-executereader-statement-parameters). Blank text runs [ExecuteReader(Native)](#csharp-icommand-executereader-native-parameters). |
| [Execute(text, parameters)](#csharp-icommand-execute-text-parameters) | Parses `text` with [StatementParser.Parse](#statements) and runs [Execute(statement)](#csharp-icommand-execute-statement-parameters). Blank text runs [Execute(Native)](#csharp-icommand-execute-native-parameters). |
| [ExecuteReader(native, parameters)](#csharp-icommand-executereader-native-parameters) | Runs provider text without parsing it. |
| [Execute(native, parameters)](#csharp-icommand-execute-native-parameters) | Runs provider text without parsing it. |
| [ExecuteReader(statement, parameters)](#csharp-icommand-executereader-statement-parameters) | Runs the statement. The default renders [Native](#csharp-native) from `statement.GetText()`. |
| [Execute(statement, parameters)](#csharp-icommand-execute-statement-parameters) | Runs the statement. The default renders [Native](#csharp-native) from `statement.GetText()`. |

###### AddMiddleware(middleware) {#csharp-icommand-addmiddleware-middleware}

Adds [ICommandMiddleware](#csharp-icommandmiddleware). Returns this command.

**Definition**

```csharp
ICommand AddMiddleware(ICommandMiddleware middleware)
```

**Parameters**

| Name | Description |
| --- | --- |
| `middleware` | [ICommandMiddleware](#csharp-icommandmiddleware) |

**Returns**

[ICommand](#csharp-icommand)

###### ExecuteReader(text, parameters) {#csharp-icommand-executereader-text-parameters}

Parses `text` with [StatementParser.Parse](#statements) and runs [ExecuteReader(statement)](#csharp-icommand-executereader-statement-parameters). Blank text runs [ExecuteReader(Native)](#csharp-icommand-executereader-native-parameters).

**Definition**

```csharp
Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL parsed as a statement |
| `parameters` | optional [Parameter](#csharp-parameter) list |

**Returns**

`Task<[IReader](#reader)<Row>>`

###### Execute(text, parameters) {#csharp-icommand-execute-text-parameters}

Parses `text` with [StatementParser.Parse](#statements) and runs [Execute(statement)](#csharp-icommand-execute-statement-parameters). Blank text runs [Execute(Native)](#csharp-icommand-execute-native-parameters).

**Definition**

```csharp
Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL parsed as a statement |
| `parameters` | optional [Parameter](#csharp-parameter) list |

**Returns**

`Task<[Response](#csharp-response)>`

###### ExecuteReader(native, parameters) {#csharp-icommand-executereader-native-parameters}

Runs provider text without parsing it.

**Definition**

```csharp
Task<IReader<Row>> ExecuteReader(Native text, IReadOnlyList<Parameter>? parameters = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | [Native](#csharp-native) engine text |
| `parameters` | optional [Parameter](#csharp-parameter) list |

**Returns**

`Task<[IReader](#reader)<Row>>`

###### Execute(native, parameters) {#csharp-icommand-execute-native-parameters}

Runs provider text without parsing it.

**Definition**

```csharp
Task<Response> Execute(Native text, IReadOnlyList<Parameter>? parameters = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | [Native](#csharp-native) engine text |
| `parameters` | optional [Parameter](#csharp-parameter) list |

**Returns**

`Task<[Response](#csharp-response)>`

###### ExecuteReader(statement, parameters) {#csharp-icommand-executereader-statement-parameters}

Runs the statement. The default renders [Native](#csharp-native) from `statement.GetText()`.

**Definition**

```csharp
Task<IReader<Row>> ExecuteReader(IStatement statement, IReadOnlyList<Parameter>? parameters = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `statement` | [IStatement](#csharp-istatement) |
| `parameters` | optional [Parameter](#csharp-parameter) list |

**Returns**

`Task<[IReader](#reader)<Row>>`

###### Execute(statement, parameters) {#csharp-icommand-execute-statement-parameters}

Runs the statement. The default renders [Native](#csharp-native) from `statement.GetText()`.

**Definition**

```csharp
Task<Response> Execute(IStatement statement, IReadOnlyList<Parameter>? parameters = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `statement` | [IStatement](#csharp-istatement) |
| `parameters` | optional [Parameter](#csharp-parameter) list |

**Returns**

`Task<[Response](#csharp-response)>`

##### Native {#csharp-native}

```csharp
public readonly record struct Native(string Text)
```

**Constructor**

| Name | Description |
| --- | --- |
| [Native(Text)](#csharp-native-native-text) | Engine text. [N](#csharp-native-n-text) builds the same value. |

**Properties**

| Name | Description |
| --- | --- |
| Text | Provider command text. |

**Methods**

| Name | Description |
| --- | --- |
| [N(text)](#csharp-native-n-text) | Builds a [Native](#csharp-native). |

###### Native(Text) {#csharp-native-native-text}

Engine text. [N](#csharp-native-n-text) builds the same value.

**Definition**

```csharp
Native(string Text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `Text` | provider command text |

**Returns**

[Native](#csharp-native)

###### N(text) {#csharp-native-n-text}

Builds a [Native](#csharp-native).

**Definition**

```csharp
static Native N(string text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | provider command text |

**Returns**

[Native](#csharp-native)

##### ICommandMiddleware {#csharp-icommandmiddleware}

```csharp
public interface ICommandMiddleware
```

**Methods**

| Name | Description |
| --- | --- |
| [BeforeCommand(text, parameters)](#csharp-icommandmiddleware-beforecommand-text-parameters) | Runs before the command. A thrown error skips the command. A non-null return rewrites it. |
| [AfterCommand(text, parameters, results, elapsedMs, error)](#csharp-icommandmiddleware-aftercommand-text-parameters-results-elapsedms-error) | Runs after the command, including when the command failed. |

###### BeforeCommand(text, parameters) {#csharp-icommandmiddleware-beforecommand-text-parameters}

Runs before the command. A thrown error skips the command. A non-null return rewrites it.

**Definition**

```csharp
Task<(string Text, IReadOnlyList<Parameter>? Parameters)?> BeforeCommand(string text, IReadOnlyList<Parameter>? parameters)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `parameters` | optional [Parameter](#csharp-parameter) list |

**Returns**

`Task<(string Text, IReadOnlyList<Parameter>? Parameters)?>`

###### AfterCommand(text, parameters, results, elapsedMs, error) {#csharp-icommandmiddleware-aftercommand-text-parameters-results-elapsedms-error}

Runs after the command, including when the command failed.

**Definition**

```csharp
Task AfterCommand(string text, IReadOnlyList<Parameter>? parameters, IReadOnlyList<Row> results, double elapsedMs, Exception? error = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `parameters` | [Parameter](#csharp-parameter) list used |
| `results` | rows produced |
| `elapsedMs` | elapsed milliseconds |
| `error` | command error when the command failed |

##### Parameter {#csharp-parameter}

```csharp
public sealed record Parameter(string Name, Value Value, ParameterDirection Direction = ParameterDirection.Input)
```

**Properties**

| Name | Description |
| --- | --- |
| Name | Parameter name. |
| Value | Parameter value. `Value.Null` for output-only / return slots. |
| Direction | Bind direction. Default is [Input](#csharp-parameterdirection). |

**Constructor**

| Name | Description |
| --- | --- |
| [Parameter(name, direction)](#csharp-parameter-parameter-name-direction) | Output-only / return slot. [Value](#csharp-parameter-value) is `Value.Null`. |

###### Parameter(name, direction) {#csharp-parameter-parameter-name-direction}

Output-only / return slot. [Value](#csharp-parameter-value) is `Value.Null`.

**Definition**

```csharp
public Parameter(string name, ParameterDirection direction)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | parameter name |
| `direction` | [Output](#csharp-parameterdirection) / [InputOutput](#csharp-parameterdirection) / [ReturnValue](#csharp-parameterdirection) |

##### ParameterDirection {#csharp-parameterdirection}

```csharp
public enum ParameterDirection
```

##### Response {#csharp-response}

```csharp
public sealed record Response
```

**Constructor**

| Name | Description |
| --- | --- |
| [Response(result, rowsAffected, parameters?)](#csharp-response-response-result-rowsaffected-parameters) | Builds a [Response](#csharp-response). Copies a [ReturnValue](#csharp-parameterdirection) onto [Result](#csharp-response-result) when `result` is `Value.Null`. |

**Properties**

| Name | Description |
| --- | --- |
| Result | Scalar or row result from the command. |
| RowsAffected | Rows inserted, updated, or deleted. |
| Parameters | Bound [Parameter](#csharp-parameter) list after [Execute](#csharp-icommand-execute-text-parameters). Holds filled [Output](#csharp-parameterdirection) / [ReturnValue](#csharp-parameterdirection) values. |

###### Response(result, rowsAffected, parameters?) {#csharp-response-response-result-rowsaffected-parameters}

Builds a [Response](#csharp-response). Copies a [ReturnValue](#csharp-parameterdirection) onto [Result](#csharp-response-result) when `result` is `Value.Null`.

**Definition**

```csharp
public Response(Value result, int rowsAffected, IReadOnlyList<Parameter>? parameters = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `result` | scalar or row result |
| `rowsAffected` | rows inserted, updated, or deleted |
| `parameters` | bound [Parameter](#csharp-parameter) list |

**Returns**

[Response](#csharp-response)

##### Rust

##### Command {#rust-command}

```rust
pub trait Command: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [add_middleware(middleware)](#rust-command-add_middleware-middleware) | Adds [CommandMiddleware](#rust-commandmiddleware). |
| [execute_reader(text, parameters)](#rust-command-execute_reader-text-parameters) | Parses `text` with [StatementParser::parse](#statements) and runs [execute_reader_statement](#rust-command-execute_reader_statement-statement-parameters). Blank text runs [execute_reader_native](#rust-command-execute_reader_native-text-parameters). |
| [execute(text, parameters)](#rust-command-execute-text-parameters) | Parses `text` with [StatementParser::parse](#statements) and runs [execute_statement](#rust-command-execute_statement-statement-parameters). Blank text runs [execute_native](#rust-command-execute_native-text-parameters). |
| [execute_reader_native(text, parameters)](#rust-command-execute_reader_native-text-parameters) | Runs provider text without parsing it. |
| [execute_native(text, parameters)](#rust-command-execute_native-text-parameters) | Runs provider text without parsing it. |
| [execute_reader_statement(statement, parameters)](#rust-command-execute_reader_statement-statement-parameters) | Runs the statement. The default renders [Native](#rust-native) from `statement.get_text()`. |
| [execute_statement(statement, parameters)](#rust-command-execute_statement-statement-parameters) | Runs the statement. The default renders [Native](#rust-native) from `statement.get_text()`. |

###### add_middleware(middleware) {#rust-command-add_middleware-middleware}

Adds [CommandMiddleware](#rust-commandmiddleware).

**Definition**

```rust
fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `middleware` | [CommandMiddleware](#rust-commandmiddleware) |

###### execute_reader(text, parameters) {#rust-command-execute_reader-text-parameters}

Parses `text` with [StatementParser::parse](#statements) and runs [execute_reader_statement](#rust-command-execute_reader_statement-statement-parameters). Blank text runs [execute_reader_native](#rust-command-execute_reader_native-text-parameters).

**Definition**

```rust
async fn execute_reader(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Box<dyn Reader>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL parsed as a statement |
| `parameters` | optional [Parameter](#rust-parameter) slice |

**Returns**

`Result<Box<dyn `[Reader](#reader)`>>`

###### execute(text, parameters) {#rust-command-execute-text-parameters}

Parses `text` with [StatementParser::parse](#statements) and runs [execute_statement](#rust-command-execute_statement-statement-parameters). Blank text runs [execute_native](#rust-command-execute_native-text-parameters).

**Definition**

```rust
async fn execute(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL parsed as a statement |
| `parameters` | optional [Parameter](#rust-parameter) slice |

**Returns**

`Result<[Response](#rust-response)>`

###### execute_reader_native(text, parameters) {#rust-command-execute_reader_native-text-parameters}

Runs provider text without parsing it.

**Definition**

```rust
async fn execute_reader_native(&self, text: &Native, parameters: Option<&[Parameter]>) -> Result<Box<dyn Reader>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | [Native](#rust-native) engine text |
| `parameters` | optional [Parameter](#rust-parameter) slice |

**Returns**

`Result<Box<dyn `[Reader](#reader)`>>`

###### execute_native(text, parameters) {#rust-command-execute_native-text-parameters}

Runs provider text without parsing it.

**Definition**

```rust
async fn execute_native(&self, text: &Native, parameters: Option<&[Parameter]>) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | [Native](#rust-native) engine text |
| `parameters` | optional [Parameter](#rust-parameter) slice |

**Returns**

`Result<[Response](#rust-response)>`

###### execute_reader_statement(statement, parameters) {#rust-command-execute_reader_statement-statement-parameters}

Runs the statement. The default renders [Native](#rust-native) from `statement.get_text()`.

**Definition**

```rust
async fn execute_reader_statement(&self, statement: &dyn Statement, parameters: Option<&[Parameter]>) -> Result<Box<dyn Reader>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `statement` | [Statement](#rust-statement) |
| `parameters` | optional [Parameter](#rust-parameter) slice |

**Returns**

`Result<Box<dyn `[Reader](#reader)`>>`

###### execute_statement(statement, parameters) {#rust-command-execute_statement-statement-parameters}

Runs the statement. The default renders [Native](#rust-native) from `statement.get_text()`.

**Definition**

```rust
async fn execute_statement(&self, statement: &dyn Statement, parameters: Option<&[Parameter]>) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `statement` | [Statement](#rust-statement) |
| `parameters` | optional [Parameter](#rust-parameter) slice |

**Returns**

`Result<[Response](#rust-response)>`

##### Native {#rust-native}

```rust
pub struct Native(String)
```

**Methods**

| Name | Description |
| --- | --- |
| [new(text)](#rust-native-new-text) | Engine text. [n](#rust-n-text) builds the same value. |
| [text()](#rust-native-text) | Provider command text. |
| [n(text)](#rust-native-n-text) | Builds a [Native](#rust-native). |

###### new(text) {#rust-native-new-text}

Engine text. [n](#rust-n-text) builds the same value.

**Definition**

```rust
pub fn new(text: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | provider command text |

**Returns**

[Native](#rust-native)

###### text() {#rust-native-text}

Provider command text.

**Definition**

```rust
pub fn text(&self) -> &str
```

**Returns**

`&str`

###### n(text) {#rust-native-n-text}

Builds a [Native](#rust-native).

**Definition**

```rust
pub fn n(text: impl Into<String>) -> Native
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | provider command text |

**Returns**

[Native](#rust-native)

##### CommandMiddleware {#rust-commandmiddleware}

```rust
pub trait CommandMiddleware: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [before_command(text, params)](#rust-commandmiddleware-before_command-text-params) | Runs before the command. An `Err` skips the command. `Some` rewrites it. |
| [after_command(text, params, results, elapsed_ms, error)](#rust-commandmiddleware-after_command-text-params-results-elapsed_ms-error) | Runs after the command, including when the command failed. |

###### before_command(text, params) {#rust-commandmiddleware-before_command-text-params}

Runs before the command. An `Err` skips the command. `Some` rewrites it.

**Definition**

```rust
async fn before_command(&self, text: &str, params: Option<&[Parameter]>) -> Result<Option<(String, Option<Vec<Parameter>>)>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `params` | optional [Parameter](#rust-parameter) slice |

**Returns**

`Result<Option<(String, Option<Vec<Parameter>>)>>`

###### after_command(text, params, results, elapsed_ms, error) {#rust-commandmiddleware-after_command-text-params-results-elapsed_ms-error}

Runs after the command, including when the command failed.

**Definition**

```rust
async fn after_command(&self, text: &str, params: Option<&[Parameter]>, results: &[Row], elapsed_ms: f64, error: Option<&DatasourceError>) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `params` | [Parameter](#rust-parameter) slice used |
| `results` | rows produced |
| `elapsed_ms` | elapsed milliseconds |
| `error` | command error when the command failed |

**Returns**

`Result<()>`

##### Parameter {#rust-parameter}

```rust
pub struct Parameter<T = Value>
```

**Properties**

| Name | Description |
| --- | --- |
| name | Parameter name. |
| value | Parameter value. `Value::Null` for output-only / return slots. |
| direction | Bind direction. Default is [Input](#rust-parameterdirection). |

**Methods**

| Name | Description |
| --- | --- |
| [new(name, value)](#rust-parameter-new-name-value) | Input [Parameter](#rust-parameter). |
| [output(name)](#rust-parameter-output-name) | Output-only slot. [value](#rust-parameter-value) is `Value::Null`. |
| [return_value(name)](#rust-parameter-return_value-name) | Return slot. [value](#rust-parameter-value) is `Value::Null`. |
| [with_direction(name, value, direction)](#rust-parameter-with_direction-name-value-direction) | Parameter with an explicit [ParameterDirection](#rust-parameterdirection). |

###### new(name, value) {#rust-parameter-new-name-value}

Input [Parameter](#rust-parameter).

**Definition**

```rust
pub fn new(name: impl Into<String>, value: impl Into<Value>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | parameter name |
| `value` | parameter value |

**Returns**

[Parameter](#rust-parameter)

###### output(name) {#rust-parameter-output-name}

Output-only slot. [value](#rust-parameter-value) is `Value::Null`.

**Definition**

```rust
pub fn output(name: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | parameter name |

**Returns**

[Parameter](#rust-parameter)

###### return_value(name) {#rust-parameter-return_value-name}

Return slot. [value](#rust-parameter-value) is `Value::Null`.

**Definition**

```rust
pub fn return_value(name: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | parameter name |

**Returns**

[Parameter](#rust-parameter)

###### with_direction(name, value, direction) {#rust-parameter-with_direction-name-value-direction}

Parameter with an explicit [ParameterDirection](#rust-parameterdirection).

**Definition**

```rust
pub fn with_direction(name: impl Into<String>, value: impl Into<Value>, direction: ParameterDirection) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | parameter name |
| `value` | parameter value |
| `direction` | bind direction |

**Returns**

[Parameter](#rust-parameter)

##### ParameterDirection {#rust-parameterdirection}

```rust
pub enum ParameterDirection
```

##### Response {#rust-response}

```rust
pub struct Response
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(result, rows_affected, parameters)](#rust-response-new-result-rows_affected-parameters) | Builds a [Response](#rust-response). Copies a [ReturnValue](#rust-parameterdirection) onto [result](#rust-response-result) when `result` is `Value::Null`. |

**Properties**

| Name | Description |
| --- | --- |
| result | Scalar or row result from the command. |
| rows_affected | Rows inserted, updated, or deleted. |
| parameters | Bound [Parameter](#rust-parameter) list after [execute](#rust-command-execute-text-parameters). Holds filled [Output](#rust-parameterdirection) / [ReturnValue](#rust-parameterdirection) values. |

###### new(result, rows_affected, parameters) {#rust-response-new-result-rows_affected-parameters}

Builds a [Response](#rust-response). Copies a [ReturnValue](#rust-parameterdirection) onto [result](#rust-response-result) when `result` is `Value::Null`.

**Definition**

```rust
pub fn new(result: impl Into<Value>, rows_affected: i64, parameters: Option<Vec<Parameter>>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `result` | scalar or row result |
| `rows_affected` | rows inserted, updated, or deleted |
| `parameters` | bound [Parameter](#rust-parameter) list |

**Returns**

[Response](#rust-response)

### Remarks

The connection must be open before `Execute` / `execute` / `execute` or `ExecuteReader` / `executeReader` / `execute_reader`.

Use `AddMiddleware` / `addMiddleware` / `add_middleware` to add low-level diagnostics. It runs `BeforeCommand` / `beforeCommand` / `before_command`, then the command, then `AfterCommand` / `afterCommand` / `after_command`. A before error skips the command. A command error still runs after, with the error and elapsed time set.

`json` and `memory` treat `text` as insert, update, or delete. SQLite, MySQL, and Postgres treat `text` as SQL.

> **Rust**
>
> The `Command` trait has no connection getter. Stub `create_command` stores the optional connection on the named type; working dialects still construct their own typed connection.

`ExecuteReader` / `executeReader` / `execute_reader` returns a [Reader](#reader) for the first result set. Convert with [RowTypeConverter](#row-type-converter), then map with a [Mapper](#mappers).

[Parameter](#parameter) defaults to [Input](#parameterdirection). [Output](#parameterdirection), [InputOutput](#parameterdirection), and [ReturnValue](#parameterdirection) are filled on [Response.Parameters](#response-parameters) after `Execute` / `execute` / `execute`. A [ReturnValue](#parameterdirection) is copied onto [Response.Result](#response-result) when `Result` would otherwise be null.

### Examples

#### Run raw SQL

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L73-L88)

```csharp
var ds = Datasource.Default;
await using var connection = await ds.CreateConnection();
await connection.Open();
var command = connection.CreateCommand();
await command.Execute(Native.N("CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT)"));
await command.Execute("INSERT INTO items (id, name) VALUES (1, 'ada')");
await using var reader = await command.ExecuteReader("SELECT id, name FROM items");
Row? row = null;
if (await reader.Next())
    row = reader.Current;
```

**Row**

```
{
  id: 1,
  name: 'ada'
}
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L100-L127)

```rust
let ds = default().unwrap();
let connection = ds.create_connection().await.unwrap();
connection.open().await.unwrap();
let command = connection.create_command();
command
    .execute_native(
        &Native::new("CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT)"),
        None,
    )
    .await
    .unwrap();
command
    .execute("INSERT INTO items (id, name) VALUES (1, 'ada')", None)
    .await
    .unwrap();
let reader = command
    .execute_reader("SELECT id, name FROM items", None)
    .await
    .unwrap();
assert!(reader.next().await.unwrap());
let row = reader.current().unwrap();
```

**Row**

```
{
  id: 1,
  name: 'ada'
}
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L91-L103)

```typescript
const ds = Datasource.default;
await using connection = await ds.createConnection();
await connection.open();
const command = connection.createCommand();
await command.executeNative(new Native("CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT)"));
await command.execute("INSERT INTO items (id, name) VALUES (1, 'ada')");
await using reader = await command.executeReader("SELECT id, name FROM items");
let row: Cells | undefined;
if (await reader.next()) row = reader.current;
```

**Row**

```
{
  id: 1,
  name: 'ada'
}
```

#### Run ExecuteReader and return NameValue

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L17-L21) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L154-L172)

```csharp
sealed class NameValue
{
    public string name { get; set; } = "";
    public string value { get; set; } = "";
}

var ds = Datasource.Default;
await using var connection = await ds.CreateConnection();
await connection.Open();
var command = connection.CreateCommand();
await command.Execute(Native.N("CREATE TABLE settings (name TEXT, value TEXT)"));
await command.Execute(
    "INSERT INTO settings (name, value) VALUES ($name, $value)",
    [new Parameter("name", "theme"), new Parameter("value", "dark")]);
await using var reader = await command.ExecuteReader("SELECT name, value FROM settings");
var converter = new RowTypeConverter<NameValue>();
var items = new List<NameValue>();
while (await reader.Next())
    items.Add(converter.Convert(reader.Current!));
```

**NameValue**

```
{
  name: 'theme',
  value: 'dark'
}
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L9-L13) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L232-L265)

```rust
impl ConvertSchema for NameValue {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![("name", "TEXT"), ("value", "TEXT")]
    }
}

let ds = default().unwrap();
let connection = ds.create_connection().await.unwrap();
connection.open().await.unwrap();
let command = connection.create_command();
command
    .execute_native(&Native::new("CREATE TABLE settings (name TEXT, value TEXT)"), None)
    .await
    .unwrap();
let parameters = [
    Parameter::new("name", "theme"),
    Parameter::new("value", "dark"),
];
command
    .execute(
        "INSERT INTO settings (name, value) VALUES ($name, $value)",
        Some(&parameters),
    )
    .await
    .unwrap();
let reader = command
    .execute_reader("SELECT name, value FROM settings", None)
    .await
    .unwrap();
let converter = RowTypeConverter::of::<NameValue>();
let mut items = Vec::new();
while reader.next().await.unwrap() {
    items.push(converter.convert(&reader.current().unwrap()).unwrap());
}
```

**NameValue**

```
{
  name: 'theme',
  value: 'dark'
}
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L25-L28) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L156-L172)

```typescript
class NameValue {
  name = "";
  value = "";
}

const ds = Datasource.default;
await using connection = await ds.createConnection();
await connection.open();
const command = connection.createCommand();
await command.executeNative(new Native("CREATE TABLE settings (name TEXT, value TEXT)"));
await command.execute("INSERT INTO settings (name, value) VALUES ($name, $value)", [
  { name: "name", value: "theme" },
  { name: "value", value: "dark" },
]);
await using reader = await command.executeReader("SELECT name, value FROM settings");
const converter = new RowTypeConverter<NameValue>();
const items: NameValue[] = [];
while (await reader.next()) items.push(converter.convert(reader.current!, NameValue));
```

**NameValue**

```
{
  name: 'theme',
  value: 'dark'
}
```

#### Execute insert, update, and delete

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L23-L27) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L174-L209)

```csharp
sealed class Item
{
    public int id { get; set; }
    public string name { get; set; } = "";
}

var ds = Datasource.Default;
await using var connection = await ds.CreateConnection();
await connection.Open();
var command = connection.CreateCommand();
await command.Execute(Native.N("CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT)"));
await command.Execute(
    "INSERT INTO items (id, name) VALUES ($id, $name)",
    [new Parameter("id", 1), new Parameter("name", "a")]);
await command.Execute(
    "UPDATE items SET name = $name WHERE id = $id",
    [new Parameter("id", 1), new Parameter("name", "A")]);
var converter = new RowTypeConverter<Item>();
Item? updated = null;
{
    await using var reader = await command.ExecuteReader(
        "SELECT id, name FROM items WHERE id = $id",
        [new Parameter("id", 1)]);
    if (await reader.Next())
        updated = converter.Convert(reader.Current!);
}
await command.Execute("DELETE FROM items WHERE id = $id", [new Parameter("id", 1)]);
Item? gone = null;
{
    await using var reader = await command.ExecuteReader(
        "SELECT id, name FROM items WHERE id = $id",
        [new Parameter("id", 1)]);
    if (await reader.Next())
        gone = converter.Convert(reader.Current!);
}
```

**After: update**

```
{
  id: 1,
  name: 'A'
}
```

**After: delete**

```
null
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L17-L21) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L267-L337)

```rust
impl ConvertSchema for Item {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![("id", "INTEGER"), ("name", "TEXT")]
    }
}

let ds = default().unwrap();
let connection = ds.create_connection().await.unwrap();
connection.open().await.unwrap();
let command = connection.create_command();
command
    .execute_native(
        &Native::new("CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT)"),
        None,
    )
    .await
    .unwrap();
command
    .execute(
        "INSERT INTO items (id, name) VALUES ($id, $name)",
        Some(&[Parameter::new("id", 1_i64), Parameter::new("name", "a")]),
    )
    .await
    .unwrap();
command
    .execute(
        "UPDATE items SET name = $name WHERE id = $id",
        Some(&[Parameter::new("id", 1_i64), Parameter::new("name", "A")]),
    )
    .await
    .unwrap();
let converter = RowTypeConverter::of::<Item>();
let updated = {
    let reader = command
        .execute_reader(
            "SELECT id, name FROM items WHERE id = $id",
            Some(&[Parameter::new("id", 1_i64)]),
        )
        .await
        .unwrap();
    let row = if reader.next().await.unwrap() {
        Some(converter.convert(&reader.current().unwrap()).unwrap())
    } else {
        None
    };
    row
};
command
    .execute(
        "DELETE FROM items WHERE id = $id",
        Some(&[Parameter::new("id", 1_i64)]),
    )
    .await
    .unwrap();
let gone = {
    let reader = command
        .execute_reader(
            "SELECT id, name FROM items WHERE id = $id",
            Some(&[Parameter::new("id", 1_i64)]),
        )
        .await
        .unwrap();
    let row = if reader.next().await.unwrap() {
        Some(converter.convert(&reader.current().unwrap()).unwrap())
    } else {
        None
    };
    row
};
let updated = updated.unwrap();
```

**After: update**

```
{
  id: 1,
  name: 'A'
}
```

**After: delete**

```
None
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L30-L33) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L174-L207)

```typescript
class Item {
  id = 0;
  name = "";
}

const ds = Datasource.default;
await using connection = await ds.createConnection();
await connection.open();
const command = connection.createCommand();
await command.executeNative(new Native("CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT)"));
await command.execute("INSERT INTO items (id, name) VALUES ($id, $name)", [
  { name: "id", value: 1 },
  { name: "name", value: "a" },
]);
await command.execute("UPDATE items SET name = $name WHERE id = $id", [
  { name: "id", value: 1 },
  { name: "name", value: "A" },
]);
const converter = new RowTypeConverter<Item>();
let updated: Item | undefined;
{
  await using reader = await command.executeReader("SELECT id, name FROM items WHERE id = $id", [
    { name: "id", value: 1 },
  ]);
  if (await reader.next()) updated = converter.convert(reader.current!, Item);
}
await command.execute("DELETE FROM items WHERE id = $id", [{ name: "id", value: 1 }]);
let gone: Item | undefined;
{
  await using reader = await command.executeReader("SELECT id, name FROM items WHERE id = $id", [
    { name: "id", value: 1 },
  ]);
  if (await reader.next()) gone = converter.convert(reader.current!, Item);
}
```

**After: update**

```
{
  id: 1,
  name: 'A'
}
```

**After: delete**

```
undefined
```

#### Map flat contacts

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L29-L42) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L44-L54) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L211-L239)

```csharp
sealed class User
{
    public int id { get; set; }
    public string username { get; set; } = "";
    public string email { get; set; } = "";
    public string contact_name1 { get; set; } = "";
    public string contact_name2 { get; set; } = "";
    public List<Contact> contacts { get; set; } = [];

    public sealed class Contact
    {
        public string name { get; set; } = "";
    }
}

static TransformMapper<User, User> UserContactsMapper() =>
    new(
        row =>
        {
            row.contacts =
            [
                new User.Contact { name = row.contact_name1 },
                new User.Contact { name = row.contact_name2 }
            ];
            return row;
        },

var ds = Datasource.Default;
await using var connection = await ds.CreateConnection();
await connection.Open();
var command = connection.CreateCommand();
await command.Execute(Native.N("""
    CREATE TABLE users (
        id INTEGER PRIMARY KEY,
        username TEXT,
        email TEXT,
        contact_name1 TEXT,
        contact_name2 TEXT
    )
    """));
await command.Execute(
    "INSERT INTO users (id, username, email, contact_name1, contact_name2) VALUES (1, 'bobby', 'bob@asdf.com', 'bob', 'roberta')");
await using var reader = await command.ExecuteReader(
    "SELECT contact_name1, contact_name2, email, id, username FROM users");
var converter = new RowTypeConverter<User>();
var mapper = UserContactsMapper();
User? user = null;
while (await reader.Next())
    user = mapper.Map(converter.Convert(reader.Current!));
```

**User**

```
{
  id: 1,
  username: 'bobby',
  email: 'bob@asdf.com',
  contacts: [
    { name: 'bob' },
    { name: 'roberta' }
  ]
}
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L48-L98) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L339-L390)

```rust
fn text_of(row: &Row, key: &str) -> String {
    match row.get(key) {
        Some(Value::Text(text)) => text.clone(),
        _ => String::new(),
    }
}
TransformMapper::new(
    |row| {
        let contact_name1 = text_of(&row, "contact_name1");
        let contact_name2 = text_of(&row, "contact_name2");
        User {
            id: match row.get("id") {
                Some(Value::Int(id)) => *id,
                _ => 0,
            },
            username: text_of(&row, "username"),
            email: text_of(&row, "email"),
            contacts: vec![
                Contact {
                    name: contact_name1.clone(),
                },
                Contact {
                    name: contact_name2.clone(),
                },
            ],
            contact_name1,
            contact_name2,
        }
    },
    |mut user| {
        user.contact_name1 = user
            .contacts
            .first()
            .map(|contact| contact.name.clone())
            .unwrap_or_default();
        user.contact_name2 = user
            .contacts
            .get(1)
            .map(|contact| contact.name.clone())
            .unwrap_or_default();
        let mut row = Row::new();
        row.insert("id".into(), Value::Int(user.id));
        row.insert("username".into(), Value::from(user.username));
        row.insert("email".into(), Value::from(user.email));
        row.insert("contact_name1".into(), Value::from(user.contact_name1));
        row.insert("contact_name2".into(), Value::from(user.contact_name2));
        row
    },
)

let ds = default().unwrap();
let connection = ds.create_connection().await.unwrap();
connection.open().await.unwrap();
let command = connection.create_command();
command
    .execute_native(
        &Native::new("CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            username TEXT,
            email TEXT,
            contact_name1 TEXT,
            contact_name2 TEXT
        )"),
        None,
    )
    .await
    .unwrap();
command
    .execute(
        "INSERT INTO users (id, username, email, contact_name1, contact_name2)
         VALUES (1, 'bobby', 'bob@asdf.com', 'bob', 'roberta')",
        None,
    )
    .await
    .unwrap();
let reader = command
    .execute_reader(
        "SELECT contact_name1, contact_name2, email, id, username FROM users",
        None,
    )
    .await
    .unwrap();
let converter = RowTypeConverter::of::<User>();
let mapper = user_contacts_mapper();
let mut user = None;
while reader.next().await.unwrap() {
    user = Some(mapper.map(converter.convert(&reader.current().unwrap()).unwrap()));
}
let user = user.unwrap();
```

**User**

```
{
  id: 1,
  username: 'bobby',
  email: 'bob@asdf.com',
  contacts: [
    { name: 'bob' },
    { name: 'roberta' }
  ]
}
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L35-L37) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L39-L46) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L48-L63) [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L209-L234)

```typescript
class Contact {
  name = "";
}

class User {
  id = 0;
  username = "";
  email = "";
  contact_name1 = "";
  contact_name2 = "";
  contacts: Contact[] = [];
}

function userContactsMapper() {
  return new TransformMapper<User, User>(
    (row) => {
      row.contacts = [
        Object.assign(new Contact(), { name: row.contact_name1 }),
        Object.assign(new Contact(), { name: row.contact_name2 }),
      ];
      return row;
    },
    (user) => {
      user.contact_name1 = user.contacts[0]?.name ?? "";
      user.contact_name2 = user.contacts[1]?.name ?? "";
      return user;
    },
  );
}

const ds = Datasource.default;
await using connection = await ds.createConnection();
await connection.open();
const command = connection.createCommand();
await command.executeNative(new Native(`CREATE TABLE users (
  id INTEGER PRIMARY KEY,
  username TEXT,
  email TEXT,
  contact_name1 TEXT,
  contact_name2 TEXT
)`));
await command.execute(
  "INSERT INTO users (id, username, email, contact_name1, contact_name2) VALUES (1, 'bobby', 'bob@asdf.com', 'bob', 'roberta')",
);
await using reader = await command.executeReader(
  "SELECT contact_name1, contact_name2, email, id, username FROM users",
);
const converter = new RowTypeConverter<User>();
const mapper = userContactsMapper();
let user: User | undefined;
while (await reader.next()) user = mapper.map(converter.convert(reader.current!, User));
```

**User**

```
{
  id: 1,
  username: 'bobby',
  email: 'bob@asdf.com',
  contacts: [
    { name: 'bob' },
    { name: 'roberta' }
  ]
}
```

#### Fill an output parameter

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Mysql.Tests/MysqlCommandIntegrationTests.cs#L67-L79)

```csharp
await command.Execute(Native.N("""
    CREATE PROCEDURE bump(IN start_n INT, OUT result INT)
    BEGIN
      SET result = start_n + 1;
    END
    """));
var start = new Parameter("start_n", 4);
var result = new Parameter("result", ParameterDirection.Output);
var call = await command.Execute(new ExecuteStoredProcedureStatement("bump", [start, result]), [start, result]);
```

**Parameters**

```
result: 5
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/mysql/tests/integration.rs#L346-L370)

```rust
use datasources::{ExecuteStoredProcedureStatement, Parameter};
command
    .execute_native(
        &Native::new("CREATE PROCEDURE bump(IN start_n INT, OUT result INT) BEGIN SET result = start_n + 1; END"),
        None,
    )
    .await
    .unwrap();
let parameters = [
    Parameter::new("start_n", 4_i64),
    Parameter::output("result"),
];
let call = command
    .execute_statement(
        &ExecuteStoredProcedureStatement::new("bump", &parameters),
        Some(&parameters),
    )
    .await
    .unwrap();
```

**Parameters**

```
result: 5
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/mysql/tests/mysql.integration.test.ts#L25-L39)

```typescript
async function fillOutputParameter(command: MysqlCommand) {
  await command.executeNative(new Native(`
    CREATE PROCEDURE bump(IN start_n INT, OUT result INT)
    BEGIN
      SET result = start_n + 1;
    END
  `));
  const start = { name: "start_n", value: 4 };
  const result = { name: "result", direction: ParameterDirection.Output };
  const call = await command.executeStatement(
    new ExecuteStoredProcedureStatement("bump", [start, result]),
    [start, result],
  );
```

**Parameters**

```
result: 5
```

#### Read a return value

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Postgres.Tests/PostgresCommandIntegrationTests.cs#L56-L66)

```csharp
await command.Execute(Native.N("""
    CREATE FUNCTION add_one(n INT) RETURNS INT
    LANGUAGE sql
    AS $$ SELECT n + 1 $$
    """));
var n = new Parameter("n", 4);
var ret = await command.Execute(new ReturnStatement("add_one", [n]), [n]);
```

**Result**

```
5
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/postgres/tests/integration.rs#L333-L351)

```rust
use datasources::{Parameter, ReturnStatement};
command
    .execute_native(
        &Native::new("CREATE FUNCTION add_one(n INT) RETURNS INT LANGUAGE sql AS $$ SELECT n + 1 $$"),
        None,
    )
    .await
    .unwrap();
let parameters = [Parameter::new("n", 4_i64)];
let ret = command
    .execute_statement(
        &ReturnStatement::new("add_one", &parameters),
        Some(&parameters),
    )
    .await
    .unwrap();
```

**Result**

```
5
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/postgres/tests/postgres.integration.test.ts#L25-L34)

```typescript
async function readReturnValue(command: PostgresCommand) {
  await command.executeNative(new Native(`
    CREATE FUNCTION add_one(n INT) RETURNS INT
    LANGUAGE sql
    AS $$ SELECT n + 1 $$
  `));
  const n = { name: "n", value: 4 };
  const ret = await command.executeStatement(new ReturnStatement("add_one", [n]), [n]);
```

**Result**

```
5
```

## Reader

Streams rows from [Command](#command) `ExecuteReader` / `executeReader` / `execute_reader`.

### Classes

##### TypeScript

##### IReader {#typescript-ireader}

```typescript
interface IReader<T>
```

**Properties**

| Name | Description |
| --- | --- |
| current | The row from the last successful [next](#typescript-ireader-next). |

**Methods**

| Name | Description |
| --- | --- |
| [next()](#typescript-ireader-next) | Advances to the next row. |
| [nextResult()](#typescript-ireader-nextresult) | Moves to the next result set and clears [current](#typescript-ireader-current). Returns false when there is no further set. [next](#typescript-ireader-next) then reads that set. Calling only [next](#typescript-ireader-next) leaves later sets unread. |
| [dispose()](#typescript-ireader-dispose) | Releases the reader. |
| [[Symbol.asyncDispose]()](#typescript-ireader-symbol-asyncdispose) | `await using` calls [dispose](#typescript-ireader-dispose). |

###### next() {#typescript-ireader-next}

Advances to the next row.

**Definition**

```typescript
next(): Promise<boolean>
```

**Returns**

`Promise<boolean>`

###### nextResult() {#typescript-ireader-nextresult}

Moves to the next result set and clears [current](#typescript-ireader-current). Returns false when there is no further set. [next](#typescript-ireader-next) then reads that set. Calling only [next](#typescript-ireader-next) leaves later sets unread.

**Definition**

```typescript
nextResult(): Promise<boolean>
```

**Returns**

`Promise<boolean>`

###### dispose() {#typescript-ireader-dispose}

Releases the reader.

**Definition**

```typescript
dispose(): Promise<void>
```

**Returns**

`Promise<void>`

###### [Symbol.asyncDispose]() {#typescript-ireader-symbol-asyncdispose}

`await using` calls [dispose](#typescript-ireader-dispose).

**Definition**

```typescript
[Symbol.asyncDispose](): Promise<void>
```

**Returns**

`Promise<void>`

##### C#

##### IReader {#csharp-ireader}

```csharp
public interface IReader<out T>
```

**Properties**

| Name | Description |
| --- | --- |
| Current | The row from the last successful [Next](#csharp-ireader-next). |

**Methods**

| Name | Description |
| --- | --- |
| [Next()](#csharp-ireader-next) | Advances to the next row. |
| [NextResult()](#csharp-ireader-nextresult) | Moves to the next result set and clears [Current](#csharp-ireader-current). Returns false when there is no further set. [Next](#csharp-ireader-next) then reads that set. A reader with one set returns false. Calling only [Next](#csharp-ireader-next) leaves later sets unread. |
| [DisposeAsync()](#csharp-ireader-disposeasync) | Releases the reader. `await using` calls this. |

###### Next() {#csharp-ireader-next}

Advances to the next row.

**Definition**

```csharp
Task<bool> Next()
```

**Returns**

`Task<bool>`

###### NextResult() {#csharp-ireader-nextresult}

Moves to the next result set and clears [Current](#csharp-ireader-current). Returns false when there is no further set. [Next](#csharp-ireader-next) then reads that set. A reader with one set returns false. Calling only [Next](#csharp-ireader-next) leaves later sets unread.

**Definition**

```csharp
Task<bool> NextResult()
```

**Returns**

`Task<bool>`

###### DisposeAsync() {#csharp-ireader-disposeasync}

Releases the reader. `await using` calls this.

**Definition**

```csharp
ValueTask DisposeAsync()
```

**Returns**

`ValueTask`

##### Rust

##### Reader {#rust-reader}

```rust
pub trait Reader<T = Row>: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [current()](#rust-reader-current) | The row from the last successful [next](#rust-reader-next). |
| [next()](#rust-reader-next) | Advances to the next row. |
| [next_result()](#rust-reader-next_result) | Moves to the next result set and clears [current](#rust-reader-current). Returns false when there is no further set. [next](#rust-reader-next) then reads that set. The default returns false. Calling only [next](#rust-reader-next) leaves later sets unread. |

###### current() {#rust-reader-current}

The row from the last successful [next](#rust-reader-next).

**Definition**

```rust
fn current(&self) -> Option<T>
```

**Returns**

`Option<T>`

###### next() {#rust-reader-next}

Advances to the next row.

**Definition**

```rust
async fn next(&self) -> Result<bool>
```

**Returns**

`Result<bool>`

###### next_result() {#rust-reader-next_result}

Moves to the next result set and clears [current](#rust-reader-current). Returns false when there is no further set. [next](#rust-reader-next) then reads that set. The default returns false. Calling only [next](#rust-reader-next) leaves later sets unread.

**Definition**

```rust
async fn next_result(&self) -> Result<bool>
```

**Returns**

`Result<bool>`

### Remarks

`ExecuteReader` / `executeReader` / `execute_reader` returns a reader for the first result set. `Next` / `next` / `next` reads that set. `NextResult` / `nextResult` / `next_result` advances to the next set. Rows stay unread until `Next` / `next` / `next`.

Dispose the reader with `await using`. Rust drops it.

### Examples

#### Read a row

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L73-L88)

```csharp
var ds = Datasource.Default;
await using var connection = await ds.CreateConnection();
await connection.Open();
var command = connection.CreateCommand();
await command.Execute(Native.N("CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT)"));
await command.Execute("INSERT INTO items (id, name) VALUES (1, 'ada')");
await using var reader = await command.ExecuteReader("SELECT id, name FROM items");
Row? row = null;
if (await reader.Next())
    row = reader.Current;
```

**Row**

```
{
  id: 1,
  name: 'ada'
}
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L100-L127)

```rust
let ds = default().unwrap();
let connection = ds.create_connection().await.unwrap();
connection.open().await.unwrap();
let command = connection.create_command();
command
    .execute_native(
        &Native::new("CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT)"),
        None,
    )
    .await
    .unwrap();
command
    .execute("INSERT INTO items (id, name) VALUES (1, 'ada')", None)
    .await
    .unwrap();
let reader = command
    .execute_reader("SELECT id, name FROM items", None)
    .await
    .unwrap();
assert!(reader.next().await.unwrap());
let row = reader.current().unwrap();
```

**Row**

```
{
  id: 1,
  name: 'ada'
}
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L91-L103)

```typescript
const ds = Datasource.default;
await using connection = await ds.createConnection();
await connection.open();
const command = connection.createCommand();
await command.executeNative(new Native("CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT)"));
await command.execute("INSERT INTO items (id, name) VALUES (1, 'ada')");
await using reader = await command.executeReader("SELECT id, name FROM items");
let row: Cells | undefined;
if (await reader.next()) row = reader.current;
```

**Row**

```
{
  id: 1,
  name: 'ada'
}
```

#### Read with case mapping

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L90-L111)

```csharp
var ds = Datasource.Default;
await using var connection = await ds.CreateConnection();
await connection.Open();
var command = connection.CreateCommand();
await command.Execute(Native.N("CREATE TABLE contacts (user_id INTEGER PRIMARY KEY, first_name TEXT)"));
await command.Execute("INSERT INTO contacts (user_id, first_name) VALUES (1, 'Ada')");
await using var reader = await command.ExecuteReader("SELECT user_id, first_name FROM contacts");
var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
Row? mapped = null;
while (await reader.Next())
{
    var row = new Row();
    foreach (var (key, value) in reader.Current!)
        row[casing.ReverseMap(key)] = value;
    mapped = row;
}
```

**Row**

```
{
  UserId: 1,
  FirstName: 'Ada'
}
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L129-L164)

```rust
let ds = default().unwrap();
let connection = ds.create_connection().await.unwrap();
connection.open().await.unwrap();
let command = connection.create_command();
command
    .execute_native(
        &Native::new("CREATE TABLE contacts (user_id INTEGER PRIMARY KEY, first_name TEXT)"),
        None,
    )
    .await
    .unwrap();
command
    .execute(
        "INSERT INTO contacts (user_id, first_name) VALUES (1, 'Ada')",
        None,
    )
    .await
    .unwrap();
let reader = command
    .execute_reader("SELECT user_id, first_name FROM contacts", None)
    .await
    .unwrap();
let casing = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
let mut mapped = Row::new();
while reader.next().await.unwrap() {
    mapped.clear();
    for (key, value) in reader.current().unwrap() {
        mapped.insert(Mapper::reverse_map(&casing, key), value);
    }
}
```

**Row**

```
{
  UserId: 1,
  FirstName: 'Ada'
}
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L105-L123)

```typescript
const ds = Datasource.default;
await using connection = await ds.createConnection();
await connection.open();
const command = connection.createCommand();
await command.executeNative(
  new Native("CREATE TABLE contacts (user_id INTEGER PRIMARY KEY, first_name TEXT)"),
);
await command.execute("INSERT INTO contacts (user_id, first_name) VALUES (1, 'Ada')");
await using reader = await command.executeReader("SELECT user_id, first_name FROM contacts");
const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
let mapped: Cells | undefined;
while (await reader.next()) {
  const row: Cells = {};
  for (const [key, value] of Object.entries(reader.current!)) row[casing.reverseMap(key)] = value;
  mapped = row;
}
```

**Row**

```
{
  UserId: 1,
  FirstName: 'Ada'
}
```

#### Type conversion and flat to nested

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L113-L152)

```csharp
var ds = Datasource.Default;
await using var connection = await ds.CreateConnection();
await connection.Open();
var command = connection.CreateCommand();
await command.Execute(Native.N("""
    CREATE TABLE users (
        id INTEGER,
        username TEXT,
        contact_name TEXT
    )
    """));
await command.Execute("INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'bob')");
await command.Execute("INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'roberta')");
await using var reader = await command.ExecuteReader("SELECT id, username, contact_name FROM users");
var rows = new List<Row>();
while (await reader.Next())
    rows.Add(reader.Current!);
var mapper = new FlatToNestedMapper(new TypeMap
{
    Key = "id",
    Fields = new Dictionary<string, TypeMapField>
    {
        ["id"] = "id",
        ["username"] = "username",
        ["contacts"] = new TypeMapGroup
        {
            Key = "contact_name",
            Fields = new Dictionary<string, TypeMapField> { ["name"] = "contact_name" },
        },
    },
});
var nested = mapper.Map(rows);
```

**User**

```
{
  id: 1,
  username: 'bobby',
  contacts: [
    { name: 'bob' },
    { name: 'roberta' }
  ]
}
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L166-L230)

```rust
let ds = default().unwrap();
let connection = ds.create_connection().await.unwrap();
connection.open().await.unwrap();
let command = connection.create_command();
command
    .execute_native(
        &Native::new("CREATE TABLE users (id INTEGER, username TEXT, contact_name TEXT)"),
        None,
    )
    .await
    .unwrap();
command
    .execute(
        "INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'bob')",
        None,
    )
    .await
    .unwrap();
command
    .execute(
        "INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'roberta')",
        None,
    )
    .await
    .unwrap();
let reader = command
    .execute_reader("SELECT id, username, contact_name FROM users", None)
    .await
    .unwrap();
let mut rows = Vec::new();
while reader.next().await.unwrap() {
    rows.push(reader.current().unwrap());
}
let mapper = FlatToNestedMapper::new(TypeMap::new(
    "id",
    vec![
        ("id".into(), TypeMapField::column("id")),
        ("username".into(), TypeMapField::column("username")),
        (
            "contacts".into(),
            TypeMapField::Group(TypeMapGroup::new(
                "contact_name",
                vec![("name".into(), TypeMapField::column("contact_name"))],
            )),
        ),
    ],
));
let nested = Mapper::map(&mapper, rows);
```

**User**

```
{
  id: 1,
  username: 'bobby',
  contacts: [
    { name: 'bob' },
    { name: 'roberta' }
  ]
}
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L125-L154)

```typescript
const ds = Datasource.default;
await using connection = await ds.createConnection();
await connection.open();
const command = connection.createCommand();
await command.executeNative(
  new Native("CREATE TABLE users (id INTEGER, username TEXT, contact_name TEXT)"),
);
await command.execute("INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'bob')");
await command.execute("INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'roberta')");
await using reader = await command.executeReader("SELECT id, username, contact_name FROM users");
const rows: Cells[] = [];
while (await reader.next()) rows.push(reader.current!);
const mapper = new FlatToNestedMapper<{
  id: number;
  username: string;
  contacts: { name: string }[];
}>({
  key: "id",
  fields: {
    id: "id",
    username: "username",
    contacts: { key: "contact_name", fields: { name: "contact_name" } },
  },
});
const nested = mapper.map(rows);
```

**User**

```
{
  id: 1,
  username: 'bobby',
  contacts: [
    { name: 'bob' },
    { name: 'roberta' }
  ]
}
```

#### Read multiple results returned in a single command

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/SqliteCommandIntegrationTests.cs#L241-L261)

```csharp
var ds = Datasource.Default;
await using var connection = await ds.CreateConnection();
await connection.Open();
var command = connection.CreateCommand();
await using var reader = await command.ExecuteReader(Native.N("SELECT 1 AS id; SELECT 2 AS id"));
var first = new List<Row>();
while (await reader.Next())
    first.Add(reader.Current!);
var moved = await reader.NextResult();
var second = new List<Row>();
while (await reader.Next())
    second.Add(reader.Current!);
var again = await reader.NextResult();
```

**First result**

```
{
  id: 1
}
```

**Second result**

```
{
  id: 2
}
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/command.rs#L392-L417)

```rust
let ds = default().unwrap();
let connection = ds.create_connection().await.unwrap();
connection.open().await.unwrap();
let command = connection.create_command();
let reader = command
    .execute_reader_native(&Native::new("SELECT 1 AS id; SELECT 2 AS id"), None)
    .await
    .unwrap();
let mut first = Vec::new();
while reader.next().await.unwrap() {
    first.push(reader.current());
}
let moved = reader.next_result().await.unwrap();
let mut second = Vec::new();
while reader.next().await.unwrap() {
    second.push(reader.current());
}
let again = reader.next_result().await.unwrap();
assert!(moved);
assert!(!again);
```

**First result**

```
{
  id: 1
}
```

**Second result**

```
{
  id: 2
}
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/sqlite.integration.test.ts#L236-L252)

```typescript
const ds = Datasource.default;
await using connection = await ds.createConnection();
await connection.open();
const command = connection.createCommand();
await using reader = await command.executeReaderNative(new Native("SELECT 1 AS id; SELECT 2 AS id"));
const first: { id: number }[] = [];
while (await reader.next()) first.push(reader.current as { id: number });
const moved = await reader.nextResult();
const second: { id: number }[] = [];
while (await reader.next()) second.push(reader.current as { id: number });
const again = await reader.nextResult();
```

**First result**

```
{
  id: 1
}
```

**Second result**

```
{
  id: 2
}
```

> **Rust**
>
> There is no `using`. The reader is dropped at the end of the scope.

#### Call a stored procedure that returns two results

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Mysql.Tests/MysqlCommandIntegrationTests.cs#L81-L103)

```csharp
await command.Execute(Native.N("""
    CREATE PROCEDURE two_ids()
    BEGIN
      SELECT 1 AS id;
      SELECT 2 AS id;
    END
    """));
await using var reader = await command.ExecuteReader(new ExecuteStoredProcedureStatement("two_ids", []));
var first = new List<Row>();
while (await reader.Next())
    first.Add(reader.Current!);
var moved = await reader.NextResult();
var second = new List<Row>();
while (await reader.Next())
    second.Add(reader.Current!);
var again = await reader.NextResult();
```

**First result**

```
{
  id: 1
}
```

**Second result**

```
{
  id: 2
}
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/mysql/tests/integration.rs#L372-L399)

```rust
use datasources::ExecuteStoredProcedureStatement;
command
    .execute_native(
        &Native::new("CREATE PROCEDURE two_ids() BEGIN SELECT 1 AS id; SELECT 2 AS id; END"),
        None,
    )
    .await
    .unwrap();
let reader = command
    .execute_reader_statement(&ExecuteStoredProcedureStatement::new("two_ids", &[]), None)
    .await
    .unwrap();
let mut first = Vec::new();
while reader.next().await.unwrap() {
    first.push(reader.current());
}
let moved = reader.next_result().await.unwrap();
let mut second = Vec::new();
while reader.next().await.unwrap() {
    second.push(reader.current());
}
let again = reader.next_result().await.unwrap();
```

**First result**

```
{
  id: 1
}
```

**Second result**

```
{
  id: 2
}
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/mysql/tests/mysql.integration.test.ts#L41-L60)

```typescript
async function callProcedureReturnsTwoResults(command: MysqlCommand) {
  await command.executeNative(new Native(`
    CREATE PROCEDURE two_ids()
    BEGIN
      SELECT 1 AS id;
      SELECT 2 AS id;
    END
  `));
  await using reader = await command.executeReaderStatement(new ExecuteStoredProcedureStatement("two_ids", []));
  const first: { id: number }[] = [];
  while (await reader.next()) first.push(reader.current as { id: number });
  const moved = await reader.nextResult();
  const second: { id: number }[] = [];
  while (await reader.next()) second.push(reader.current as { id: number });
  const again = await reader.nextResult();
```

**First result**

```
{
  id: 1
}
```

**Second result**

```
{
  id: 2
}
```

## Repositories

Repositories map datasource commands to a domain object. They mediate reading and modifying objects in a datasource, and they can enrich those objects into view objects with extra properties. That abstraction removes boilerplate datasource code.

### Classes

##### TypeScript

##### IRepository {#typescript-irepository}

```typescript
interface IRepository<T = unknown>
```

**Properties**

| Name | Description |
| --- | --- |
| type | Item constructor the repository exposes. |

##### C#

##### IRepository {#csharp-irepository}

```csharp
public interface IRepository
```

**Properties**

| Name | Description |
| --- | --- |
| Type | Item CLR type the repository exposes. |

##### Rust

##### IRepository {#rust-irepository}

```rust
pub trait IRepository
```

**Methods**

| Name | Description |
| --- | --- |
| [entity_type()](#rust-irepository-entity_type) | Item `TypeId` the repository exposes. Defaults to `Row`. |

###### entity_type() {#rust-irepository-entity_type}

Item `TypeId` the repository exposes. Defaults to `Row`.

**Definition**

```rust
fn entity_type(&self) -> TypeId
```

**Returns**

`TypeId`

##### datasources {#rust-datasources}

```rust
pub fn as_readonly(repo: Arc<dyn ICRUDRepository>) -> Arc<dyn IReadonlyRepository>
```

**Methods**

| Name | Description |
| --- | --- |
| [as_readonly(repo)](#rust-datasources-as_readonly-repo) | Views a [ICRUDRepository](#crud-repository) as a [IReadonlyRepository](#readonly-repository). |

###### as_readonly(repo) {#rust-datasources-as_readonly-repo}

Views a [ICRUDRepository](#crud-repository) as a [IReadonlyRepository](#readonly-repository).

**Definition**

```rust
pub fn as_readonly(repo: Arc<dyn ICRUDRepository>) -> Arc<dyn IReadonlyRepository>
```

**Parameters**

| Name | Description |
| --- | --- |
| `repo` | [ICRUDRepository](#crud-repository) |

**Returns**

`Arc<dyn `[IReadonlyRepository](#readonly-repository)`>`

### Remarks

SQL provider default repositories report the row or dictionary type (`Row`, `Dictionary<string, Value>`, or `Object`), not a closed entity generic.

### Examples

#### Simple readonly

`ReadSingle` / `readSingle` / `read_single` returns the first matching user. `Read` / `read` / `read` yields every user that matches the where.

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L29-L53)

```csharp
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
var connection = await Open(ds, "minimal-users", """
    INSERT INTO users (id, username, is_enabled) VALUES
        (1, 'ada', 1),
        (2, 'grace', 0);
    """);
var repo = ds.CreateReadonlyRepository<User>(connection, new ReadonlyRepositoryOptions<User>
{
    EntityMapper = new PluralizerMapper(),
});

var byId = await repo.ReadSingle(new Where("id", eq, 1));
var disabled = new List<User>();
await foreach (var row in repo.Read(new Where("username", eq, "grace").And("is_enabled", eq, 0)))
    disabled.Add(row);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L50-L84)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
let connection = open(
    ds,
    "minimal-users",
    "
    INSERT INTO users (id, username, is_enabled) VALUES
        (1, 'ada', 1),
        (2, 'grace', 0);
    ",
)
.await;
let mut options = create_crud_repository_options::<User>();
options.entity_mapper = Box::new(PluralizerMapper::new());
let repo = ds.create_readonly_repository::<User>(Some(connection), Some(options.read));
let by_id = repo
    .read_single(Some(&Where::binary("id", eq, 1_i64)), None)
    .await
    .unwrap()
    .unwrap();
let disabled = repo
    .read(
        Some(&Where::binary("username", eq, "grace").and("is_enabled", eq, 0_i64)),
        None,
    )
    .await
    .unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L38-L57)

```typescript
const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
const connection = await open(ds, "minimal-users", `
  INSERT INTO users (id, username, is_enabled) VALUES
    (1, 'ada', 1),
    (2, 'grace', 0);
`);
const repo = ds.createReadonlyRepository(User, connection, {
  entityMapper: new PluralizerMapper(),
});
const byId = await repo.readSingle(new Where("id", eq, 1));
const disabled: User[] = [];
for await (const row of repo.read(new Where("username", eq, "grace").and("is_enabled", eq, 0)))
  disabled.push(row);
```

#### CRUD

Create omits `id`, then writes the inserted id onto the user. Update and delete use that id.

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L383-L410)

```csharp
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
var connection = await Open(ds, "user-pascal");
var usersRepo = new CRUDRepository<User>(connection);
var created = await usersRepo.Create(new User { username = "ada", is_enabled = true });
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L558-L607)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
let connection = open(ds, "user-pascal", "").await;
let users_repo = CRUDRepository::from_type::<User>(connection).unwrap();
let mut created = datasources::Row::new();
created.insert("username".into(), Value::from("ada"));
created.insert("is_enabled".into(), Value::Bool(true));
let mut created = users_repo.create(&mut created).await.unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L161-L183)

```typescript
const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
const connection = await open(ds, "user-pascal");
const usersRepo = new CRUDRepository(User, connection);
const created = await usersRepo.create(Object.assign(new User(), { username: "ada", is_enabled: true }));
```

#### Enriched CRUD

`UsersView` maps onto `user`. Create writes the user and its roles. Read fills `UserType` and `Roles`.

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L504-L576)

```csharp
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
var connection = await Open(ds, "user-view-version");
var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
var usersRepo = new CRUDRepository<UsersView>(connection, new CRUDRepositoryOptions<UsersView>
{
    Ids = ["Key"],
    ReadProperties = ["Key", "Login", "Active", "Opened", "UserTypeId", "Version"],
    EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UsersView"] = "user" }),
    ColumnMappings = new DictionaryMapper<string>(new Dictionary<string, string>
    {
        ["Key"] = "id",
        ["Login"] = "username",
        ["Active"] = "is_enabled",
        ["Opened"] = "created",
        ["UserTypeId"] = "user_type_id",
        ["Version"] = "version",
    }),
});
var userTypesRepo = new CRUDRepository<UserType>(connection, new CRUDRepositoryOptions<UserType>
{
    Ids = ["Id"],
    EntityMapper = casing,
    ColumnMappings = casing,
});
var userRolesRepo = new CRUDRepository<UserRole>(connection, new CRUDRepositoryOptions<UserRole>
{
    Ids = [],
    EntityMapper = casing,
    ColumnMappings = casing,
});
usersRepo.AddEnriched(new("UserType", "UserTypeId", "Id", "Name"), userTypesRepo);
usersRepo.AddEnriched(new("Roles", "Key", "UserId", Many: true), userRolesRepo);
var admin = await userTypesRepo.Create(new UserType { Name = "admin" });
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L609-L725)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
let connection = open(ds, "user-view-version", "").await;
let mut users_options = create_crud_repository_options::<UsersView>();
users_options.ids = vec!["Key".into()];
users_options.entity_mapper = Box::new(DictionaryMapper::new([(
    "UsersView".to_string(),
    "user".to_string(),
)]));
users_options.column_mappings = Box::new(DictionaryMapper::new([
    ("Key".to_string(), "id".to_string()),
    ("Login".to_string(), "username".to_string()),
    ("Active".to_string(), "is_enabled".to_string()),
    ("Opened".to_string(), "created".to_string()),
    ("UserTypeId".to_string(), "user_type_id".to_string()),
    ("Version".to_string(), "version".to_string()),
]));
users_options.row_converter = Some(RowTypeConverter::of::<UsersView>());
let casing = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
let mut type_options = create_crud_repository_options::<UserType>();
type_options.entity_mapper =
    Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
type_options.column_mappings = Box::new(casing);
type_options.row_converter = Some(RowTypeConverter::of::<UserType>());
let mut role_options = create_crud_repository_options::<UserRole>();
role_options.entity_mapper =
    Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
role_options.column_mappings =
    Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
role_options.row_converter = Some(RowTypeConverter::of::<UserRole>());
let mut users_repo = CRUDRepository::new(connection.clone(), users_options).unwrap();
let user_types_repo =
    Arc::new(CRUDRepository::new(connection.clone(), type_options).unwrap());
let user_roles_repo = Arc::new(CRUDRepository::new(connection, role_options).unwrap());
users_repo.add_enriched(
    datasources::Enrichment::new(
        "UserType",
        "UserTypeId",
        "Id",
        Some("Name".into()),
        false,
    ),
    user_types_repo.clone(),
    None,
);
users_repo.add_enriched(
    datasources::Enrichment::new("Roles", "Key", "UserId", None, true),
    user_roles_repo.clone(),
    None,
);
let mut admin = datasources::Row::new();
admin.insert("Name".into(), Value::from("admin"));
let admin = user_types_repo.create(&mut admin).await.unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L208-L273)

```typescript
const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
const connection = await open(ds, "user-view-version");
const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
const usersRepo = new CRUDRepository(connection, {
  ...createCRUDRepositoryOptions(UsersView),
  ids: ["Key"],
  readProperties: ["Key", "Login", "Active", "Opened", "UserTypeId", "Version"],
  entityMapper: new DictionaryMapper([["UsersView", "user"]]),
  columnMappings: new DictionaryMapper([
    ["Key", "id"],
    ["Login", "username"],
    ["Active", "is_enabled"],
    ["Opened", "created"],
    ["UserTypeId", "user_type_id"],
    ["Version", "version"],
  ]),
});
const userTypesRepo = new CRUDRepository(connection, {
  ...createCRUDRepositoryOptions(UserType),
  entityMapper: casing,
  columnMappings: casing,
});
const userRolesRepo = new CRUDRepository(connection, {
  ...createCRUDRepositoryOptions(UserRole),
  entityMapper: casing,
  columnMappings: casing,
});
usersRepo.addEnriched({ into: "UserType", from: "UserTypeId", by: "Id", select: "Name" }, userTypesRepo);
usersRepo.addEnriched({ into: "Roles", from: "Key", by: "UserId", many: true }, userRolesRepo);
const admin = await userTypesRepo.create(Object.assign(new UserType(), { Name: "admin" }));
```

#### Bulk

Each parent is created with its nested children. Responses sum `rowsAffected`.

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/BulkCRUDRepositoryTests.cs#L22-L41)

```csharp
var connection = await Setup();
var parentsRepo = new BulkCRUDRepository<Parent>(connection);
var childrenRepo = new BulkCRUDRepository<Child>(connection);
var response = await Repo(parentsRepo, childrenRepo).Create(
[
    new Parent { children = [new Child { label = "a" }] },
    new Parent { children = [new Child { label = "b" }, new Child { label = "c" }] }
]);
var parents = await parentsRepo.Read().ToListAsync();
var children = await childrenRepo.Read().ToListAsync();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1897-L1939)

```rust
let command = Arc::new(MockCommand::new());
command.reader_rows.lock().unwrap().push(record(&[
    ("id", Value::Int(1)),
    ("username", Value::from("bobby")),
    ("email", Value::from("bob@asdf.com")),
    ("contact_name1", Value::from("bob")),
    ("contact_name2", Value::from("roberta")),
]));
let connection = MockConnection::with_command_provider(command.clone(), "memory");
let mut options = CRUDRepositoryOptions::default();
options.entity = "users".into();
options.row_converter = Some(RowTypeConverter::of::<UserRow>());
options.rows_mapper = Box::new(user_contacts_mapper());
let repo = CRUDRepository::new(connection, options).unwrap();
let mut user = repo.read_single(None, None).await.unwrap().unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/BulkCRUDRepository.test.ts#L33-L50)

```typescript
const connection = await setup();
const parentsRepo = new BulkCRUDRepository(Parent, connection);
const childrenRepo = new BulkCRUDRepository(Child, connection);
const response = await relate(parentsRepo, childrenRepo).createBulk([
  Object.assign(new Parent(), { children: [Object.assign(new Child(), { label: "a" })] }),
  Object.assign(new Parent(), {
    children: [Object.assign(new Child(), { label: "b" }), Object.assign(new Child(), { label: "c" })],
  }),
]);
const parents = await collectAsync(parentsRepo.read());
const children = await collectAsync(childrenRepo.read());
expect(parents).toHaveLength(2);
```

### See also

- [Readonly repository](#readonly-repository)
- [CRUD repository](#crud-repository)
- [Bulk repository](#bulk-repository)
- [Repository factories](#repository-factories)

## Readonly repository

Runs readonly select commands and can enrich those datasource objects. Writes are [CRUD repository](#crud-repository).

### Classes

##### TypeScript

##### IReadonlyRepository {#typescript-ireadonlyrepository}

```typescript
interface IReadonlyRepository<T = unknown> extends IRepository<T>
```

**Methods**

| Name | Description |
| --- | --- |
| [readSingle(query?)](#typescript-ireadonlyrepository-readsingle-query) | Returns the first item, or `undefined` when the result is empty. |
| [read(query?)](#typescript-ireadonlyrepository-read-query) | Yields every item. |

###### readSingle(query?) {#typescript-ireadonlyrepository-readsingle-query}

Returns the first item, or `undefined` when the result is empty.

**Definition**

```typescript
readSingle(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): Promise<T | undefined>
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored procedure; omitted uses the default select |

**Returns**

`Promise<T | undefined>`

###### read(query?) {#typescript-ireadonlyrepository-read-query}

Yields every item.

**Definition**

```typescript
read(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored procedure; omitted uses the default select |

**Returns**

`AsyncIterable<T>`

##### ReadonlyRepository {#typescript-readonlyrepository}

```typescript
abstract class ReadonlyRepository<T> implements IReadonlyRepository<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [ReadonlyRepository(type)](#typescript-readonlyrepository-readonlyrepository-type) | Stores the item constructor. |
| [ReadonlyRepository(inner, strategy?)](#typescript-readonlyrepository-readonlyrepository-inner-strategy) | Wraps an inner repository. `read` reads that repository, then fills related fields. Omitted strategy uses the default. See [IEnriched](#ienriched). |

**Properties**

| Name | Description |
| --- | --- |
| type | Item constructor the repository exposes. |

**Methods**

| Name | Description |
| --- | --- |
| [read(query?)](#typescript-readonlyrepository-read-query) | Yields every item. Table repositories convert then map; subclasses may override. |
| [readSingle(query?)](#typescript-readonlyrepository-readsingle-query) | Returns the first item from [read](#typescript-readonlyrepository-read-query), or `undefined`. |

###### ReadonlyRepository(type) {#typescript-readonlyrepository-readonlyrepository-type}

Stores the item constructor.

**Definition**

```typescript
constructor(type: Constructable<T>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `type` | item constructor ([IRepository.type](#typescript-irepository-type)) |

###### ReadonlyRepository(inner, strategy?) {#typescript-readonlyrepository-readonlyrepository-inner-strategy}

Wraps an inner repository. `read` reads that repository, then fills related fields. Omitted strategy uses the default. See [IEnriched](#ienriched).

**Definition**

```typescript
constructor(inner: IReadonlyRepository<T>, strategy?: IReadonlyEnrichmentStrategy<T>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [IReadonlyRepository](#typescript-ireadonlyrepository) |
| `strategy` | optional enrichment strategy |

###### read(query?) {#typescript-readonlyrepository-read-query}

Yields every item. Table repositories convert then map; subclasses may override.

**Definition**

```typescript
abstract read(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored procedure; omitted uses the default select |

**Returns**

`AsyncIterable<T>`

###### readSingle(query?) {#typescript-readonlyrepository-readsingle-query}

Returns the first item from [read](#typescript-readonlyrepository-read-query), or `undefined`.

**Definition**

```typescript
readSingle(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): Promise<T | undefined>
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored procedure; omitted uses the default select |

**Returns**

`Promise<T | undefined>`

##### ReadonlyRepositoryOptions {#typescript-readonlyrepositoryoptions}

```typescript
type ReadonlyRepositoryOptions<T = Cells>
```

**Properties**

| Name | Description |
| --- | --- |
| entity | Table or entity name. |
| entityMapper | Maps the table name when the repository is built. Default is [IdentityMapper](#identity-mapper). |
| columnMappings | Maps property names to stored columns. Default is [IdentityMapper](#identity-mapper). The repository maps where identifiers and write parameters. Read reverse-maps each result key before the row converter. |
| select | Rewrites the finished select. Omitted leaves the statement unchanged. |
| rowsMapper | Maps each converted item. Default is [IdentityMapper](#identity-mapper). |
| rowConverter | Row [TypeConverter](#type-converters). |
| type | Item constructor. Supplies the default entity name and column list. |
| rowTypeConverters | Provider converters keyed by `bool`, `datetime`, `uuid`, `decimal`, and `binary`, merged into the row converter. The driver applies its converters when it binds a parameter. A field converter reverse-converts that property in the repository. |
| readColumns | SELECT column names. When omitted, public fields of `type` are used. |

##### IRepositoryTree {#typescript-irepositorytree}

```typescript
interface IRepositoryTree
```

**Properties**

| Name | Description |
| --- | --- |
| parent | Root [IRepositoryTreeNode](#typescript-irepositorytreenode). |

##### IRepositoryTreeNode {#typescript-irepositorytreenode}

```typescript
interface IRepositoryTreeNode
```

**Properties**

| Name | Description |
| --- | --- |
| value | Related repository. |
| propertyName | Destination property name. |
| parent | Parent node, if any. |
| items | Child nodes. |
| enrichment | [Enrichment](#enrichment) for this node. |

**Methods**

| Name | Description |
| --- | --- |
| [include(via, into, from, by, select?, many?)](#typescript-irepositorytreenode-include-via-into-from-by-select-many) | Adds a child node. |
| [enrichFrom(parents)](#typescript-irepositorytreenode-enrichfrom-parents) | Fills `parents` from this node. |

###### include(via, into, from, by, select?, many?) {#typescript-irepositorytreenode-include-via-into-from-by-select-many}

Adds a child node.

**Definition**

```typescript
include<TRelated extends object>(via: IReadonlyRepository<TRelated>, into: string, from: string, by: string, select?: string, many?: boolean): IRepositoryTreeNode
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `into` | destination property |
| `from` | parent key |
| `by` | related key |
| `select` | optional related property to copy |
| `many` | collect a list |

**Returns**

[IRepositoryTreeNode](#typescript-irepositorytreenode)

###### enrichFrom(parents) {#typescript-irepositorytreenode-enrichfrom-parents}

Fills `parents` from this node.

**Definition**

```typescript
enrichFrom(parents: object[]): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `parents` | parent objects |

##### C#

##### IReadonlyRepository {#csharp-ireadonlyrepository}

```csharp
public interface IReadonlyRepository<T> : IRepository
```

**Methods**

| Name | Description |
| --- | --- |
| [ReadSingle(query)](#csharp-ireadonlyrepository-readsingle-query) | Returns the first item, or `default` when the result is empty. |
| [Read(query)](#csharp-ireadonlyrepository-read-query) | Yields every item. |

###### ReadSingle(query) {#csharp-ireadonlyrepository-readsingle-query}

Returns the first item, or `default` when the result is empty.

**Definition**

```csharp
Task<T?> ReadSingle(IStatement? query = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored-procedure statement; omitted uses the default select |

**Returns**

`Task<T?>`

###### Read(query) {#csharp-ireadonlyrepository-read-query}

Yields every item.

**Definition**

```csharp
IAsyncEnumerable<T> Read(IStatement? query = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored-procedure statement; omitted uses the default select |

**Returns**

`IAsyncEnumerable<T>`

##### ReadonlyRepository {#csharp-readonlyrepository}

```csharp
public class ReadonlyRepository<T> : IReadonlyRepository<T>, IEnriched<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [ReadonlyRepository(inner, strategy)](#csharp-readonlyrepository-readonlyrepository-inner-strategy) | Wraps an inner repository. `Read` reads that repository, then fills related fields. Omitted strategy uses the default. See [IEnriched](#ienriched). |

**Properties**

| Name | Description |
| --- | --- |
| Type | Item CLR type. `typeof(T)`. |

**Methods**

| Name | Description |
| --- | --- |
| [Read(query)](#csharp-readonlyrepository-read-query) | Yields every item. Table repositories convert then map; subclasses may override. |
| [ReadSingle(query)](#csharp-readonlyrepository-readsingle-query) | Returns the first item from [Read](#csharp-readonlyrepository-read-query), or `default`. |

###### ReadonlyRepository(inner, strategy) {#csharp-readonlyrepository-readonlyrepository-inner-strategy}

Wraps an inner repository. `Read` reads that repository, then fills related fields. Omitted strategy uses the default. See [IEnriched](#ienriched).

**Definition**

```csharp
public ReadonlyRepository(IReadonlyRepository<T> inner, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [IReadonlyRepository](#csharp-ireadonlyrepository) |
| `strategy` | optional enrichment strategy |

###### Read(query) {#csharp-readonlyrepository-read-query}

Yields every item. Table repositories convert then map; subclasses may override.

**Definition**

```csharp
public virtual IAsyncEnumerable<T> Read(IStatement? query = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored-procedure statement; omitted uses the default select |

**Returns**

`IAsyncEnumerable<T>`

###### ReadSingle(query) {#csharp-readonlyrepository-readsingle-query}

Returns the first item from [Read](#csharp-readonlyrepository-read-query), or `default`.

**Definition**

```csharp
public virtual Task<T?> ReadSingle(IStatement? query = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored-procedure statement; omitted uses the default select |

**Returns**

`Task<T?>`

##### ReadonlyRepositoryOptions {#csharp-readonlyrepositoryoptions}

```csharp
public class ReadonlyRepositoryOptions<T> where T : new()
```

**Properties**

| Name | Description |
| --- | --- |
| Entity | Table or entity name. When omitted, `typeof(T).Name` is used. |
| EntityMapper | Maps the table name when the repository is built. Default is [IdentityMapper](#identity-mapper). |
| ColumnMappings | Maps property names to stored columns. Default is [IdentityMapper](#identity-mapper). The repository maps where identifiers and write parameters. Read reverse-maps each result key before the row converter. |
| Select | Rewrites the finished select. Omitted leaves the statement unchanged. |
| RowsMapper | Maps each converted item. Default is [IdentityMapper](#identity-mapper). |
| RowConverter | Row [TypeConverter](#type-converters). |
| RowTypeConverters | Provider converters keyed by `bool`, `datetime`, `uuid`, `decimal`, and `binary`, merged into the row converter. The driver applies its converters when it binds a parameter. A field converter reverse-converts that property in the repository. |
| ReadColumns | SELECT column names. When omitted, public properties of `T` are used. |

##### IRepositoryTree {#csharp-irepositorytree}

```csharp
public interface IRepositoryTree
```

**Properties**

| Name | Description |
| --- | --- |
| Parent | Root [IRepositoryTreeNode](#csharp-irepositorytreenode). |

##### IRepositoryTreeNode {#csharp-irepositorytreenode}

```csharp
public interface IRepositoryTreeNode
```

**Properties**

| Name | Description |
| --- | --- |
| Value | Related repository. |
| PropertyName | Destination property name. |
| Parent | Parent node, if any. |
| Items | Child nodes. |

##### Rust

##### IReadonlyRepository {#rust-ireadonlyrepository}

```rust
pub trait IReadonlyRepository: IRepository
```

**Methods**

| Name | Description |
| --- | --- |
| [read_single(query)](#rust-ireadonlyrepository-read_single-query) | Returns the first row, or `None` when the result is empty. |
| [read(query)](#rust-ireadonlyrepository-read-query) | Returns every row. |

###### read_single(query) {#rust-ireadonlyrepository-read_single-query}

Returns the first row, or `None` when the result is empty.

**Definition**

```rust
async fn read_single(&self, query: Option<&dyn ReadInput>) -> Result<Option<Row>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored procedure; omitted uses the default select |

**Returns**

`Result<Option<Row>>`

###### read(query) {#rust-ireadonlyrepository-read-query}

Returns every row.

**Definition**

```rust
async fn read(&self, query: Option<&dyn ReadInput>) -> Result<Vec<Row>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored procedure; omitted uses the default select |

**Returns**

`Result<Vec<Row>>`

##### ReadonlyRepository {#rust-readonlyrepository}

```rust
pub struct ReadonlyRepository
```

**Constructor**

| Name | Description |
| --- | --- |
| [from_repository(inner)](#rust-readonlyrepository-from_repository-inner) | Wraps an inner repository. `read` reads that repository, then fills related fields. Uses the default strategy. See [IEnriched](#ienriched). |
| [with_strategy(inner, strategy)](#rust-readonlyrepository-with_strategy-inner-strategy) | Wraps an inner repository with an enrichment strategy. |

###### from_repository(inner) {#rust-readonlyrepository-from_repository-inner}

Wraps an inner repository. `read` reads that repository, then fills related fields. Uses the default strategy. See [IEnriched](#ienriched).

**Definition**

```rust
pub fn from_repository(inner: Arc<dyn IReadonlyRepository>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [IReadonlyRepository](#rust-ireadonlyrepository) |

**Returns**

[ReadonlyRepository](#rust-readonlyrepository)

###### with_strategy(inner, strategy) {#rust-readonlyrepository-with_strategy-inner-strategy}

Wraps an inner repository with an enrichment strategy.

**Definition**

```rust
pub fn with_strategy(inner: Arc<dyn IReadonlyRepository>, strategy: Arc<dyn ReadonlyEnrichmentStrategy>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [IReadonlyRepository](#rust-ireadonlyrepository) |
| `strategy` | [ReadonlyEnrichmentStrategy](#ireadonlyenrichmentstrategy) |

**Returns**

[ReadonlyRepository](#rust-readonlyrepository)

##### ReadonlyRepositoryOptions {#rust-readonlyrepositoryoptions}

```rust
pub struct ReadonlyRepositoryOptions
```

**Properties**

| Name | Description |
| --- | --- |
| entity | Table or entity name. Default `"items"`. |
| entity_mapper | Maps the table name when the repository is built. Default is [IdentityMapper](#identity-mapper). |
| column_mappings | Maps property names to stored columns. Default is [IdentityMapper](#identity-mapper). The repository maps where identifiers and write parameters. Read reverse-maps each result key before the row converter. |
| select | Rewrites finished select SQL. `None` leaves the statement unchanged. |
| rows_mapper | Maps each converted row. Default is [IdentityMapper](#identity-mapper). |
| row_converter | Row [RowTypeConverter](#row-type-converter). |
| row_type_converters | Provider converters keyed by `bool`, `datetime`, `uuid`, `decimal`, and `binary`, merged into the row converter. The driver applies its converters when it binds a parameter. A field converter reverse-converts that property in the repository. |

##### RepositoryTree {#rust-repositorytree}

```rust
pub struct RepositoryTree
```

**Properties**

| Name | Description |
| --- | --- |
| parent | Root [RepositoryTreeNode](#rust-repositorytreenode). |

##### RepositoryTreeNode {#rust-repositorytreenode}

```rust
pub struct RepositoryTreeNode
```

**Properties**

| Name | Description |
| --- | --- |
| value | Related [ReadonlyRepository](#readonly-repository). |
| property_name | Destination field name. |
| items | Child nodes. |
| enrichment | [Enrichment](#enrichment) for this node. |

**Methods**

| Name | Description |
| --- | --- |
| [include(via, into, from, by, select, many)](#rust-repositorytreenode-include-via-into-from-by-select-many) | Adds a child node. |

###### include(via, into, from, by, select, many) {#rust-repositorytreenode-include-via-into-from-by-select-many}

Adds a child node.

**Definition**

```rust
pub fn include(&mut self, via: impl EnrichmentVia, into: impl Into<String>, from: impl Into<String>, by: impl Into<String>, select: Option<String>, many: bool) -> &mut Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#rust-ireadonlyrepository) |
| `into` | destination field |
| `from` | parent key |
| `by` | related key |
| `select` | optional related field to copy |
| `many` | collect a list |

**Returns**

`&mut Self`

### Remarks

`AddEnriched` / `addEnriched` / `add_enriched` on [IEnriched](#ienriched) registers a read fill. With no enrichments, `Read` / `read` / `read` stays streaming. With enrichments, the read is collected, filled, then yielded. A `many` list is walked. A `select` scalar is not. Depth follows each related repository's own enrichments. Writes, including `many` children and associations, are [CRUDRepository](#crud-repository).

`AddEnriched` stays fluent for siblings. Register the child repository's own fills on that repository:

```
childrenRepo.AddEnriched(new("type_name", "type_id", "id", "name"), childTypesRepo);
parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
```

Repository read is convert, then map. See [Type Converters](#type-converters) and [Mappers](#mappers).

`read` and `read_single` accept a query, a where, or an [ExecuteStoredProcedureStatement](#execute-stored-procedure-statement). Result rows still convert, then map. Procedure argument names are not rewritten as columns.

### Examples

#### Simple readonly

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L29-L53)

```csharp
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
var connection = await Open(ds, "minimal-users", """
    INSERT INTO users (id, username, is_enabled) VALUES
        (1, 'ada', 1),
        (2, 'grace', 0);
    """);
var repo = ds.CreateReadonlyRepository<User>(connection, new ReadonlyRepositoryOptions<User>
{
    EntityMapper = new PluralizerMapper(),
});

var byId = await repo.ReadSingle(new Where("id", eq, 1));
var disabled = new List<User>();
await foreach (var row in repo.Read(new Where("username", eq, "grace").And("is_enabled", eq, 0)))
    disabled.Add(row);
```

**After: by id**

```
{
  id: 1,
  username: 'ada',
  is_enabled: true
}
```

**After: disabled**

```
[
  {
    id: 2,
    username: 'grace',
    is_enabled: false
  }
]
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L50-L84)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
let connection = open(
    ds,
    "minimal-users",
    "
    INSERT INTO users (id, username, is_enabled) VALUES
        (1, 'ada', 1),
        (2, 'grace', 0);
    ",
)
.await;
let mut options = create_crud_repository_options::<User>();
options.entity_mapper = Box::new(PluralizerMapper::new());
let repo = ds.create_readonly_repository::<User>(Some(connection), Some(options.read));
let by_id = repo
    .read_single(Some(&Where::binary("id", eq, 1_i64)), None)
    .await
    .unwrap()
    .unwrap();
let disabled = repo
    .read(
        Some(&Where::binary("username", eq, "grace").and("is_enabled", eq, 0_i64)),
        None,
    )
    .await
    .unwrap();
```

**After: by id**

```
{
  id: 1,
  username: 'ada',
  is_enabled: 1
}
```

**After: disabled**

```
[
  {
    id: 2,
    username: 'grace',
    is_enabled: 0
  }
]
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L38-L57)

```typescript
const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
const connection = await open(ds, "minimal-users", `
  INSERT INTO users (id, username, is_enabled) VALUES
    (1, 'ada', 1),
    (2, 'grace', 0);
`);
const repo = ds.createReadonlyRepository(User, connection, {
  entityMapper: new PluralizerMapper(),
});
const byId = await repo.readSingle(new Where("id", eq, 1));
const disabled: User[] = [];
for await (const row of repo.read(new Where("username", eq, "grace").and("is_enabled", eq, 0)))
  disabled.push(row);
```

**After: by id**

```
{
  id: 1,
  username: 'ada',
  is_enabled: 1
}
```

**After: disabled**

```
[
  {
    id: 2,
    username: 'grace',
    is_enabled: 0
  }
]
```

#### Enriched readonly

`UsersView` maps onto `user`. `UserType` stays on the row as `user_type_id` until enrichment replaces it with the type name. The lookup repository's entity is `UserType`, and [ChangeCasingMapper](#change-casing-mapper) maps that name, plus `Id` and `Name`, to `user_type`, `id`, and `name`.

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L128-L169)

```csharp
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
var connection = await Open(ds, "user-type-lookup", $"""
    INSERT INTO user_type (id, name) VALUES
        (1, 'admin'),
        (2, 'user'),
        (3, 'service');
    INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
        ('{Ada}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
        ('{Grace}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
        ('{Lin}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
    """);
var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
var usersRepo = (ReadonlyRepository<UsersView>)ds.CreateReadonlyRepository<UsersView>(connection, new ReadonlyRepositoryOptions<UsersView>
{
    EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UsersView"] = "user" }),
    ColumnMappings = new DictionaryMapper<string>(new Dictionary<string, string>
    {
        ["Key"] = "id",
        ["Login"] = "username",
        ["Active"] = "is_enabled",
        ["Opened"] = "created",
        ["UserType"] = "user_type_id",
    }),
});
var typesRepo = ds.CreateReadonlyRepository<UserType>(connection, new ReadonlyRepositoryOptions<UserType>
{
    EntityMapper = casing,
    ColumnMappings = casing,
});
usersRepo.AddEnriched(new("UserType", "UserType", "Id", "Name"), typesRepo);
var usersView = await usersRepo.ReadSingle(new Where("Key", eq, Ada));
```

**User view**

```
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: true,
  Opened: '2020-01-01T00:00:00.000Z',
  UserType: 'admin'
}
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L202-L273)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
let connection = open(
    ds,
    "user-type-lookup",
    &format!(
        "
    INSERT INTO user_type (id, name) VALUES
        (1, 'admin'),
        (2, 'user'),
        (3, 'service');
    INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
        ('{ADA}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
        ('{GRACE}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
        ('{LIN}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
    "
    ),
)
.await;
let mut users_options = create_crud_repository_options::<UserAccount>();
users_options.entity = "user".into();
users_options.column_mappings = Box::new(DictionaryMapper::new([
    ("Key".to_string(), "id".to_string()),
    ("Login".to_string(), "username".to_string()),
    ("Active".to_string(), "is_enabled".to_string()),
    ("Opened".to_string(), "created".to_string()),
    ("UserTypeId".to_string(), "user_type_id".to_string()),
]));
let mut type_options = create_crud_repository_options::<UserType>();
type_options.entity = "UserType".into();
type_options.entity_mapper =
    Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
type_options.column_mappings =
    Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
let mut users_repo =
    ReadonlyRepository::new(connection.clone(), users_options.read).unwrap();
let types =
    ds.create_readonly_repository::<UserType>(Some(connection), Some(type_options.read));
users_repo.add_enriched(
    datasources::Enrichment::new(
        "UserType",
        "UserTypeId",
        "Id",
        Some("Name".into()),
        false,
    ),
    types,
    None,
);
let account = users_repo
    .read_single(
        Some(&Where::binary("Key", eq, Uuid::parse_str(ADA).unwrap())),
        None,
    )
    .await
    .unwrap()
    .unwrap();
let mut users_view = account.clone();
users_view.remove("UserTypeId");
```

**User view**

```
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: 1,
  Opened: '2020-01-01T00:00:00.000Z',
  UserType: 'admin'
}
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L109-L144)

```typescript
const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
const connection = await open(ds, "user-type-lookup", `
  INSERT INTO user_type (id, name) VALUES
    (1, 'admin'),
    (2, 'user'),
    (3, 'service');
  INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
    ('${ada}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
    ('${grace}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
    ('${lin}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
`);
const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
const usersRepo = ds.createReadonlyRepository(UsersView, connection, {
  entityMapper: new DictionaryMapper([["UsersView", "user"]]),
  columnMappings: new DictionaryMapper([
    ["Key", "id"],
    ["Login", "username"],
    ["Active", "is_enabled"],
    ["Opened", "created"],
    ["UserType", "user_type_id"],
  ]),
}) as ReadonlyRepository<UsersView>;
const typesRepo = ds.createReadonlyRepository(UserType, connection, {
  entityMapper: casing,
  columnMappings: casing,
});
usersRepo.addEnriched({ into: "UserType", from: "UserType", by: "Id", select: "Name" }, typesRepo);
const usersView = await usersRepo.readSingle(new Where("Key", eq, ada));
```

**User view**

```
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: 1,
  Opened: '2020-01-01T00:00:00.000Z',
  UserType: 'admin'
}
```

#### Read a user by guid, username, and created

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L55-L92)

```csharp
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
var connection = await Open(ds, "user-account", $"""
    INSERT INTO user (id, username, is_enabled, created) VALUES
        ('{Ada}', 'ada', 1, '2020-01-01T00:00:00.000Z'),
        ('{Grace}', 'grace', 1, '2024-06-01T00:00:00.000Z'),
        ('{Lin}', 'lin', 0, '2024-06-01T00:00:00.000Z');
    """);
var repo = ds.CreateReadonlyRepository<UserAccount>(connection, new ReadonlyRepositoryOptions<UserAccount>
{
    Entity = "user",
    ColumnMappings = new DictionaryMapper<string>(new Dictionary<string, string>
    {
        ["Key"] = "id",
        ["Login"] = "username",
        ["Active"] = "is_enabled",
        ["Opened"] = "created",
    }),
});

var opened = new DateTime(2024, 1, 1, 0, 0, 0, DateTimeKind.Utc);
var byId = await repo.ReadSingle(new Where("Key", eq, Ada));
var byLogin = new List<UserAccount>();
await foreach (var row in repo.Read(new Where("Login", eq, "ada").And("Active", eq, true)))
    byLogin.Add(row);
var later = new List<UserAccount>();
await foreach (var row in repo.Read(new Where("Active", eq, true).And("Opened", gt, opened)))
    later.Add(row);
```

**After: by id**

```
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: true,
  Opened: '2020-01-01T00:00:00.000Z'
}
```

**After: by login**

```
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: true,
    Opened: '2020-01-01T00:00:00.000Z'
  }
]
```

**After: later**

```
[
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: true,
    Opened: '2024-06-01T00:00:00.000Z'
  }
]
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L86-L146)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
let connection = open(
    ds,
    "user-account",
    &format!(
        "
    INSERT INTO user (id, username, is_enabled, created) VALUES
        ('{ADA}', 'ada', 1, '2020-01-01T00:00:00.000Z'),
        ('{GRACE}', 'grace', 1, '2024-06-01T00:00:00.000Z'),
        ('{LIN}', 'lin', 0, '2024-06-01T00:00:00.000Z');
    "
    ),
)
.await;
let mut options = create_crud_repository_options::<UserAccount>();
options.entity = "user".into();
options.column_mappings = Box::new(DictionaryMapper::new([
    ("Key".to_string(), "id".to_string()),
    ("Login".to_string(), "username".to_string()),
    ("Active".to_string(), "is_enabled".to_string()),
    ("Opened".to_string(), "created".to_string()),
]));
let repo = ds.create_readonly_repository::<UserAccount>(Some(connection), Some(options.read));
let opened = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
let by_id = repo
    .read_single(
        Some(&Where::binary("Key", eq, Uuid::parse_str(ADA).unwrap())),
        None,
    )
    .await
    .unwrap()
    .unwrap();
let by_login = repo
    .read(
        Some(&Where::binary("Login", eq, "ada").and("Active", eq, true)),
        None,
    )
    .await
    .unwrap();
let later = repo
    .read(
        Some(&Where::binary("Active", eq, true).and("Opened", gt, opened)),
        None,
    )
    .await
    .unwrap();
```

**After: by id**

```
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: 1,
  Opened: '2020-01-01T00:00:00.000Z'
}
```

**After: by login**

```
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: 1,
    Opened: '2020-01-01T00:00:00.000Z'
  }
]
```

**After: later**

```
[
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: 1,
    Opened: '2024-06-01T00:00:00.000Z'
  }
]
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L59-L92)

```typescript
const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
const connection = await open(ds, "user-account", `
  INSERT INTO user (id, username, is_enabled, created) VALUES
    ('${ada}', 'ada', 1, '2020-01-01T00:00:00.000Z'),
    ('${grace}', 'grace', 1, '2024-06-01T00:00:00.000Z'),
    ('${lin}', 'lin', 0, '2024-06-01T00:00:00.000Z');
`);
const repo = ds.createReadonlyRepository(UserAccount, connection, {
  entity: "user",
  columnMappings: new DictionaryMapper([
    ["Key", "id"],
    ["Login", "username"],
    ["Active", "is_enabled"],
    ["Opened", "created"],
  ]),
});
const opened = new Date("2024-01-01T00:00:00.000Z");
const byId = await repo.readSingle(new Where("Key", eq, ada));
const byLogin: UserAccount[] = [];
for await (const row of repo.read(new Where("Login", eq, "ada").and("Active", eq, true)))
  byLogin.push(row);
const later: UserAccount[] = [];
for await (const row of repo.read(new Where("Active", eq, true).and("Opened", gt, opened)))
  later.push(row);
```

**After: by id**

```
{
  Key: '11111111-1111-1111-1111-111111111111',
  Login: 'ada',
  Active: 1,
  Opened: '2020-01-01T00:00:00.000Z'
}
```

**After: by login**

```
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: 1,
    Opened: '2020-01-01T00:00:00.000Z'
  }
]
```

**After: later**

```
[
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: 1,
    Opened: '2024-06-01T00:00:00.000Z'
  }
]
```

#### Call a stored procedure

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/RepositoryTests.cs#L107-L116)

```csharp
Parameter[] parameters = [new("id", 9)];
var call = new ExecuteStoredProcedureStatement("get_user", parameters);
var rows = new List<UserRow>();
await foreach (var row in repo.Read(call, parameters))
    rows.Add(row);
```

**After: rows**

```
[
  { id: 1, name: 'a' }
]
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1438-L1454)

```rust
        &row
    ),
    no
);
```

**After: rows**

```
[
  { id: 1, name: 'a' }
]
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/repository.test.ts#L102-L109)

```typescript
command.reader = new MockReader([{ id: 1, name: "a" }]);
const parameters = [{ name: "id", value: 9 }];
const call = new ExecuteStoredProcedureStatement("get_user", parameters);
const rows: Cells[] = [];
for await (const row of repo.read(call, parameters)) rows.push(row);
```

**After: rows**

```
[
  { id: 1, name: 'a' }
]
```

#### Read and read single

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/RepositoryTests.cs#L69-L76)

```csharp
await foreach (var row in repo.Read())
    rows.Add(row);
Assert.Single(rows);
```

**After: read**

```
[
  { id: 1, name: 'a' }
]
```

**After: read single**

```
{ id: 1, name: 'a' }
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1475-L1481)

```rust

```

**After: read**

```
[
  { id: 1, name: 'a' }
]
```

**After: read single**

```
{ id: 1, name: 'a' }
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/repository.test.ts#L63-L73)

```typescript
const { repo, command } = create();
repo.addMiddleware({ beforeCommand() {}, afterCommand() {} });
command.reader = new MockReader([
  { id: 1, name: "a" },
]);
const rows: Cells[] = [];
for await (const row of repo.read()) rows.push(row);
```

**After: read**

```
[
  { id: 1, name: 'a' }
]
```

**After: read single**

```
{ id: 1, name: 'a' }
```

#### Fill a lookup

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/EnrichedReadonlyRepositoryTests.cs#L45-L47)

```csharp
var parentsRepo = new ReadonlyRepository<Parent>(connection);
var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
parentsRepo.AddEnriched(new("type_name", "type_id", "id", "name"), typesRepo);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/enriched.rs#L139-L145)

```rust
let mut parents_repo = readonly(connection, "Parent");
let types_repo = Arc::new(readonly(types_connection, "TypeRow"));
parents_repo.add_enriched(
    Enrichment::new("type_name", "type_id", "id", Some("name".into()), false),
    types_repo,
    None,
);
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/EnrichedReadonlyRepository.test.ts#L90-L92)

```typescript
const parentsRepo = new ReadonlyRepository(Parent, connection);
const typesRepo = new ReadonlyRepository(TypeRow, typesConnection);
parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo);
```

#### Fill roles from one join

The repository reads one join of `user`, `user_type`, `user_roles`, and `roles`. [FlatToNestedMapper](#flat-to-nested-mapper) groups each user's `Roles`.

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L263-L340)

```csharp
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
var connection = await Open(ds, "user-roles-read", Seed());
var mappings = new DictionaryMapper<string>(new Dictionary<string, string>
{
    ["Key"] = "id",
    ["Login"] = "username",
    ["Active"] = "is_enabled",
    ["Opened"] = "created",
    ["UserType"] = "user_type.name",
    ["UserId"] = "user_id",
    ["RoleId"] = "role_id",
    ["Name"] = "roles.name",
});
var usersRepo = new ReadonlyRepository<UsersView>(connection, new ReadonlyRepositoryOptions<UsersView>
{
    ColumnMappings = mappings,
});
usersRepo.AddEnriched(
    new("Roles", "Key", "UserId", Many: true),
    usersRepo,
    new NestedToObjectEnrichmentStrategy<UsersView>(new TypeMap
    {
        Key = "Key",
        Fields = new Dictionary<string, TypeMapField>
        {
            ["Key"] = "Key",
            ["Login"] = "Login",
            ["Active"] = "Active",
            ["Opened"] = "Opened",
            ["UserType"] = "UserType",
            ["Roles"] = new TypeMapGroup
            {
                Key = "RoleId",
                Fields = new Dictionary<string, TypeMapField>
                {
                    ["RoleId"] = "RoleId",
                    ["UserId"] = "UserId",
                    ["Name"] = "Name",
                }
            }
        }
    }));
var items = new List<UsersView>();
await foreach (var item in usersRepo.Read(new SelectStatement(new()
{
    Table = "user",
    Fields =
    [
        "user.id",
        "user.username",
        "user.is_enabled",
        "user.created",
        "user_type.name",
        "user_roles.user_id",
        "user_roles.role_id",
        "roles.name",
    ],
    Joins =
    [
        new Join(JoinKind.Left, "user_type", "user_type.id", "user.user_type_id"),
        new Join(JoinKind.Left, "user_roles", "user_roles.user_id", "user.id"),
        new Join(JoinKind.Left, "roles", "roles.id", "user_roles.role_id"),
    ],
})))
    items.Add(item);
var ada = Assert.Single(items, item => item.Login == "ada");
var grace = Assert.Single(items, item => item.Login == "grace");
var lin = Assert.Single(items, item => item.Login == "lin");
```

**Users with roles**

```
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: true,
    Opened: '2020-01-01T00:00:00.000Z',
    UserType: 'admin',
    Roles: [
      { UserId: '11111111-1111-1111-1111-111111111111', Name: 'admin' },
      { UserId: '11111111-1111-1111-1111-111111111111', Name: 'editor' }
    ]
  },
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: true,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'user',
    Roles: [
      { UserId: '22222222-2222-2222-2222-222222222222', Name: 'viewer' }
    ]
  },
  {
    Key: '33333333-3333-3333-3333-333333333333',
    Login: 'lin',
    Active: false,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'service',
    Roles: []
  }
]
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L441-L502)

```rust
            "user.is_enabled AS Active",
            "user.created AS Opened",
            "user_type.name AS UserType",
            "user.id AS UserId",
            "user_roles.role_id AS RoleId",
            "roles.name AS RoleName",
        ],
    )
}));
let options = ReadonlyRepositoryOptions::default();
let mut users_repo = ReadonlyRepository::new(connection.clone(), options).unwrap();
users_repo.add_enriched(
    datasources::Enrichment::new("Roles", "Key", "UserId", None, true),
    Arc::new(ReadonlyRepository::memory(vec![])),
    Some(Arc::new(NestedToObjectEnrichmentStrategy::new(
        TypeMap::new(
            "Key",
            vec![
                ("Key".into(), TypeMapField::column("Key")),
                ("Login".into(), TypeMapField::column("Login")),
                ("Active".into(), TypeMapField::column("Active")),
                ("Opened".into(), TypeMapField::column("Opened")),
                ("UserType".into(), TypeMapField::column("UserType")),
                (
                    "Roles".into(),
                    TypeMapField::Group(TypeMapGroup::new(
                        "RoleId",
                        vec![
                            ("RoleId".into(), TypeMapField::column("RoleId")),
                            ("UserId".into(), TypeMapField::column("UserId")),
                            ("Name".into(), TypeMapField::column("RoleName")),
                        ],
                    )),
                ),
            ],
        ),
    ))),
);
let items = users_repo.read(Some(command.as_ref()), None).await.unwrap();
let view = |login: &str| {
    items
        .iter()
        .find(|item| item.get("Login") == Some(&Value::from(login)))
        .unwrap()
        .clone()
};
let ada = view("ada");
let grace = view("grace");
let lin = view("lin");
```

**Users with roles**

```
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: 1,
    Opened: '2020-01-01T00:00:00.000Z',
    UserType: 'admin',
    Roles: 'Name=admin,UserId=11111111-1111-1111-1111-111111111111|Name=editor,UserId=11111111-1111-1111-1111-111111111111'
  },
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: 1,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'user',
    Roles: 'Name=viewer,UserId=22222222-2222-2222-2222-222222222222'
  },
  {
    Key: '33333333-3333-3333-3333-333333333333',
    Login: 'lin',
    Active: 0,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'service',
    Roles: ''
  }
]
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/users-view-roles.repository.integration.test.ts#L115-L167)

```typescript
const usersRepo = new ReadonlyRepository(connection, {
  type: UsersView,
  columnMappings: new DictionaryMapper([
    ["Key", "id"],
    ["Login", "username"],
    ["Active", "is_enabled"],
    ["Opened", "created"],
    ["UserType", "user_type.name"],
    ["UserId", "user_id"],
    ["RoleId", "role_id"],
    ["Name", "roles.name"],
  ]),
});
usersRepo.addEnriched(
  { into: "Roles", from: "Key", by: "UserId", many: true },
  usersRepo,
  new NestedToObjectEnrichmentStrategy<UsersView>({
    key: "Key",
    fields: {
      Key: "Key",
      Login: "Login",
      Active: "Active",
      Opened: "Opened",
      UserType: "UserType",
      Roles: {
        key: "RoleId",
        fields: { RoleId: "RoleId", UserId: "UserId", Name: "Name" },
      },
    },
  }),
);
const items: UsersView[] = [];
for await (const item of usersRepo.read(
  new SelectStatement({
    table: "user",
    fields: [
      "user.id",
      "user.username",
      "user.is_enabled",
      "user.created",
      "user_type.name",
      "user_roles.user_id",
      "user_roles.role_id",
      "roles.name",
    ],
    joins: [
      Join.left("user_type", "user_type.id", "user.user_type_id"),
      Join.left("user_roles", "user_roles.user_id", "user.id"),
      Join.left("roles", "roles.id", "user_roles.role_id"),
    ],
  }),
))
  items.push(item);
```

**Users with roles**

```
[
  {
    Key: '11111111-1111-1111-1111-111111111111',
    Login: 'ada',
    Active: 1,
    Opened: '2020-01-01T00:00:00.000Z',
    UserType: 'admin',
    Roles: [
      { RoleId: 1, UserId: '11111111-1111-1111-1111-111111111111', Name: 'admin' },
      { RoleId: 2, UserId: '11111111-1111-1111-1111-111111111111', Name: 'editor' }
    ]
  },
  {
    Key: '22222222-2222-2222-2222-222222222222',
    Login: 'grace',
    Active: 1,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'user',
    Roles: [
      { RoleId: 3, UserId: '22222222-2222-2222-2222-222222222222', Name: 'viewer' }
    ]
  },
  {
    Key: '33333333-3333-3333-3333-333333333333',
    Login: 'lin',
    Active: 0,
    Opened: '2024-06-01T00:00:00.000Z',
    UserType: 'service',
    Roles: []
  }
]
```

## CRUD repository

Builds on the [readonly repository](#readonly-repository) and adds create, update, and delete. The object needs a unique id in one or more fields. Custom property names use the repository mappers.

Read is convert, then map. Writes reverse-map, then reverse-convert.

### Classes

##### TypeScript

##### ICRUDRepository {#typescript-icrudrepository}

```typescript
interface ICRUDRepository<T = unknown> extends IReadonlyRepository<T>
```

**Methods**

| Name | Description |
| --- | --- |
| [create(item)](#typescript-icrudrepository-create-item) | Inserts one item. |
| [update(item, byWhere?)](#typescript-icrudrepository-update-item-bywhere) | Updates one item. Omitted `byWhere` uses id columns. |
| [delete(item, byWhere?)](#typescript-icrudrepository-delete-item-bywhere) | Deletes one item. Omitted `byWhere` uses id columns. |
| [readSingleById(id)](#typescript-icrudrepository-readsinglebyid-id) | Returns the row for one id column, or for a composite key. A value requires exactly one configured id column. A row must include every configured id column. No match returns `undefined`. |

###### create(item) {#typescript-icrudrepository-create-item}

Inserts one item.

**Definition**

```typescript
create(item: T): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to insert |

**Returns**

`Promise<Response>`

###### update(item, byWhere?) {#typescript-icrudrepository-update-item-bywhere}

Updates one item. Omitted `byWhere` uses id columns.

**Definition**

```typescript
update(item: T, byWhere?: IWhere): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to update |
| `byWhere` | optional predicate; omitted uses id values on `item` |

**Returns**

`Promise<Response>`

###### delete(item, byWhere?) {#typescript-icrudrepository-delete-item-bywhere}

Deletes one item. Omitted `byWhere` uses id columns.

**Definition**

```typescript
delete(item: T, byWhere?: IWhere): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to delete |
| `byWhere` | optional predicate; omitted uses id values on `item` |

**Returns**

`Promise<Response>`

###### readSingleById(id) {#typescript-icrudrepository-readsinglebyid-id}

Returns the row for one id column, or for a composite key. A value requires exactly one configured id column. A row must include every configured id column. No match returns `undefined`.

**Definition**

```typescript
readSingleById(id: Scalar | Cells): Promise<T | undefined>
```

**Parameters**

| Name | Description |
| --- | --- |
| `id` | value of the single id column, or a row keyed by each id column |

**Returns**

`Promise<T | undefined>`

##### CRUDRepository {#typescript-crudrepository}

```typescript
class CRUDRepository<T = Cells> extends ReadonlyRepository<T> implements ICRUDRepository<T>, IEnriched<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [CRUDRepository(type, connection)](#typescript-crudrepository-crudrepository-type-connection) | Builds a table repository from a type and [IConnection](#connection). |
| [CRUDRepository(connection, options)](#typescript-crudrepository-crudrepository-connection-options) | Builds a table repository from an [IConnection](#connection) and [CRUDRepositoryOptions](#typescript-crudrepositoryoptions). |
| [CRUDRepository(inner, strategy?)](#typescript-crudrepository-crudrepository-inner-strategy) | Wraps an inner [ICRUDRepository](#typescript-icrudrepository). Parent writes delegate to `inner`. Omitted strategy uses the default. |

**Properties**

| Name | Description |
| --- | --- |
| ids | Configured id property names. |

**Methods**

| Name | Description |
| --- | --- |
| [addMiddleware(middleware)](#typescript-crudrepository-addmiddleware-middleware) | Adds command middleware. Returns this repository. |
| [read(query?)](#typescript-crudrepository-read-query) | Reads rows. Convert the database row, then map it. |
| [create(item)](#typescript-crudrepository-create-item) | Inserts one item. |
| [update(item, byWhere?)](#typescript-crudrepository-update-item-bywhere) | Updates one item. |
| [delete(item, byWhere?)](#typescript-crudrepository-delete-item-bywhere) | Deletes one item. |
| [readSingleById(id)](#typescript-crudrepository-readsinglebyid-id) | Returns the row for one id column, or for a composite key. |
| [addEnriched(enrichment, via, strategy?)](#typescript-crudrepository-addenriched-enrichment-via-strategy) | Registers the same read fill as [ReadonlyRepository](#typescript-addenrichedenrichment-via-strategy). When `via` is a CRUD repository, `many` children are written with the parent. |
| [addLinked(enrichment, via, link, strategy?)](#typescript-crudrepository-addlinked-enrichment-via-link-strategy) | Registers an association. `via` is read. `link` is the CRUD writer. |

###### CRUDRepository(type, connection) {#typescript-crudrepository-crudrepository-type-connection}

Builds a table repository from a type and [IConnection](#connection).

**Definition**

```typescript
constructor(type: Constructable<T>, connection: IConnection)
```

**Parameters**

| Name | Description |
| --- | --- |
| `type` | item constructor |
| `connection` | open [IConnection](#connection) |

###### CRUDRepository(connection, options) {#typescript-crudrepository-crudrepository-connection-options}

Builds a table repository from an [IConnection](#connection) and [CRUDRepositoryOptions](#typescript-crudrepositoryoptions).

**Definition**

```typescript
constructor(connection: IConnection, options: CRUDRepositoryOptions<T>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | open [IConnection](#connection) |
| `options` | [CRUDRepositoryOptions](#typescript-crudrepositoryoptions) |

###### CRUDRepository(inner, strategy?) {#typescript-crudrepository-crudrepository-inner-strategy}

Wraps an inner [ICRUDRepository](#typescript-icrudrepository). Parent writes delegate to `inner`. Omitted strategy uses the default.

**Definition**

```typescript
constructor(inner: ICRUDRepository<T>, strategy?: IReadonlyEnrichmentStrategy<T>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [ICRUDRepository](#typescript-icrudrepository) |
| `strategy` | optional read enrichment strategy |

###### addMiddleware(middleware) {#typescript-crudrepository-addmiddleware-middleware}

Adds command middleware. Returns this repository.

**Definition**

```typescript
addMiddleware(middleware: ICommandMiddleware): this
```

**Parameters**

| Name | Description |
| --- | --- |
| `middleware` | command middleware |

**Returns**

`this`

###### read(query?) {#typescript-crudrepository-read-query}

Reads rows. Convert the database row, then map it.

**Definition**

```typescript
read(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored procedure; omitted uses the default select |

**Returns**

`AsyncIterable<T>`

###### create(item) {#typescript-crudrepository-create-item}

Inserts one item.

**Definition**

```typescript
create(item: T): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to insert |

**Returns**

`Promise<Response>`

###### update(item, byWhere?) {#typescript-crudrepository-update-item-bywhere}

Updates one item.

**Definition**

```typescript
update(item: T, byWhere?: IWhere): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to update |
| `byWhere` | optional predicate; omitted uses id values on `item` |

**Returns**

`Promise<Response>`

###### delete(item, byWhere?) {#typescript-crudrepository-delete-item-bywhere}

Deletes one item.

**Definition**

```typescript
delete(item: T, byWhere?: IWhere): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to delete |
| `byWhere` | optional predicate; omitted uses id values on `item` |

**Returns**

`Promise<Response>`

###### readSingleById(id) {#typescript-crudrepository-readsinglebyid-id}

Returns the row for one id column, or for a composite key.

**Definition**

```typescript
readSingleById(id: Scalar | Cells): Promise<T | undefined>
```

**Parameters**

| Name | Description |
| --- | --- |
| `id` | value of the single id column, or a row keyed by each id column |

**Returns**

`Promise<T | undefined>`

###### addEnriched(enrichment, via, strategy?) {#typescript-crudrepository-addenriched-enrichment-via-strategy}

Registers the same read fill as [ReadonlyRepository](#typescript-addenrichedenrichment-via-strategy). When `via` is a CRUD repository, `many` children are written with the parent.

**Definition**

```typescript
addEnriched<TRelated extends object>(enrichment: Enrichment, via: IReadonlyRepository<TRelated>, strategy?: IReadonlyEnrichmentStrategy<T & object>): this
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](#enrichment) |
| `via` | related repository. A CRUD repository is kept so `many` children can be written |
| `strategy` | optional read strategy for this edge. Omitted uses the default strategy |

**Returns**

`this`

###### addLinked(enrichment, via, link, strategy?) {#typescript-crudrepository-addlinked-enrichment-via-link-strategy}

Registers an association. `via` is read. `link` is the CRUD writer.

**Definition**

```typescript
addLinked<TRelated extends object, TLink extends object>(enrichment: Enrichment, via: IReadonlyRepository<TRelated>, link: ICRUDRepository<TLink>, strategy?: IReadonlyEnrichmentStrategy<T & object>): this
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](#enrichment) with `association` |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `link` | [ICRUDRepository](#typescript-icrudrepository) that stores the association rows |
| `strategy` | optional read strategy for this edge. Omitted uses the default strategy |

**Returns**

`this`

##### CRUDRepositoryOptions {#typescript-crudrepositoryoptions}

```typescript
type CRUDRepositoryOptions<T = Cells> = ReadonlyRepositoryOptions<T> & { ... }
```

**Properties**

| Name | Description |
| --- | --- |
| ids | Id column names. When omitted, `["id"]`, or `["Id"]` when `type` has that field and no `id`. |
| insert | Rewrites the finished insert. Omitted leaves the statement unchanged. |
| update | Rewrites the finished update. Omitted leaves the statement unchanged. |
| delete | Rewrites the finished delete. Omitted leaves the statement unchanged. |
| return | Rewrites a finished [ReturnStatement](#return-statement). Omitted leaves it unchanged. |

##### BuiltRepository {#typescript-builtrepository}

```typescript
type BuiltRepository<T = Cells>
```

**Properties**

| Name | Description |
| --- | --- |
| connection | Open [IConnection](#connection). |
| rowConverter | Row [TypeConverter](#type-converters). |
| rowsMapper | Read-row [Mapper](#mappers). |
| statements | SQL fragments. |
| ids | Id column names. |
| type | Item constructor. |

##### C#

##### ICRUDRepository {#csharp-icrudrepository}

```csharp
public interface ICRUDRepository<T> : IReadonlyRepository<T>
```

**Methods**

| Name | Description |
| --- | --- |
| [Create(item)](#csharp-icrudrepository-create-item) | Inserts one item. |
| [Update(item, byWhere)](#csharp-icrudrepository-update-item-bywhere) | Updates one item. Omitted `byWhere` uses id columns. |
| [Delete(item, byWhere)](#csharp-icrudrepository-delete-item-bywhere) | Deletes one item. Omitted `byWhere` uses id columns. |
| [ReadSingleById(id)](#csharp-icrudrepository-readsinglebyid-id) | Returns the row for the single configured id column. Zero id columns, or more than one, throw. |
| [ReadSingleById(ids)](#csharp-icrudrepository-readsinglebyid-ids) | Returns the row whose configured id columns match `ids`. A missing id column throws. No match returns `null`. |

###### Create(item) {#csharp-icrudrepository-create-item}

Inserts one item.

**Definition**

```csharp
Task<Response> Create(T item)
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to insert |

**Returns**

`Task<Response>`

###### Update(item, byWhere) {#csharp-icrudrepository-update-item-bywhere}

Updates one item. Omitted `byWhere` uses id columns.

**Definition**

```csharp
Task<Response> Update(T item, IWhere? byWhere = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to update |
| `byWhere` | optional predicate; omitted uses id values on `item` |

**Returns**

`Task<Response>`

###### Delete(item, byWhere) {#csharp-icrudrepository-delete-item-bywhere}

Deletes one item. Omitted `byWhere` uses id columns.

**Definition**

```csharp
Task<Response> Delete(T item, IWhere? byWhere = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to delete |
| `byWhere` | optional predicate; omitted uses id values on `item` |

**Returns**

`Task<Response>`

###### ReadSingleById(id) {#csharp-icrudrepository-readsinglebyid-id}

Returns the row for the single configured id column. Zero id columns, or more than one, throw.

**Definition**

```csharp
Task<T?> ReadSingleById(Value id)
```

**Parameters**

| Name | Description |
| --- | --- |
| `id` | value of the single id column |

**Returns**

`Task<T?>`

###### ReadSingleById(ids) {#csharp-icrudrepository-readsinglebyid-ids}

Returns the row whose configured id columns match `ids`. A missing id column throws. No match returns `null`.

**Definition**

```csharp
Task<T?> ReadSingleById(Row ids)
```

**Parameters**

| Name | Description |
| --- | --- |
| `ids` | id column names and values |

**Returns**

`Task<T?>`

##### CRUDRepository {#csharp-crudrepository}

```csharp
public class CRUDRepository<T> : ReadonlyRepository<T>, ICRUDRepository<T>, IEnriched<T> where T : new()
```

**Constructor**

| Name | Description |
| --- | --- |
| [CRUDRepository(connection)](#csharp-crudrepository-crudrepository-connection) | Builds a table repository from an [IConnection](#connection). |
| [CRUDRepository(connection, options)](#csharp-crudrepository-crudrepository-connection-options) | Builds a table repository from an [IConnection](#connection) and [CRUDRepositoryOptions](#csharp-crudrepositoryoptions). |
| [CRUDRepository(inner, strategy)](#csharp-crudrepository-crudrepository-inner-strategy) | Wraps an inner [ICRUDRepository](#csharp-icrudrepository). Parent writes delegate to `inner`. Omitted strategy uses the default. |

**Properties**

| Name | Description |
| --- | --- |
| Ids | Configured id property names. |

**Methods**

| Name | Description |
| --- | --- |
| [AddMiddleware(middleware)](#csharp-crudrepository-addmiddleware-middleware) | Adds command middleware. Returns this repository. |
| [Read(query)](#csharp-crudrepository-read-query) | Reads rows. Convert the database row, then map it. |
| [Create(item)](#csharp-crudrepository-create-item) | Inserts one item. |
| [Update(item, byWhere)](#csharp-crudrepository-update-item-bywhere) | Updates one item. |
| [Delete(item, byWhere)](#csharp-crudrepository-delete-item-bywhere) | Deletes one item. |
| [ReadSingleById(id)](#csharp-crudrepository-readsinglebyid-id) | Returns the row for the single configured id column. |
| [ReadSingleById(ids)](#csharp-crudrepository-readsinglebyid-ids) | Returns the row whose configured id columns match `ids`. |
| [AddEnriched(enrichment, via, strategy)](#csharp-crudrepository-addenriched-enrichment-via-strategy) | Registers the same read fill as [ReadonlyRepository](#csharp-addenrichedenrichment-via-strategy). When `via` is a CRUD repository, `many` children are written with the parent. |
| [AddLinked(enrichment, via, link, strategy)](#csharp-crudrepository-addlinked-enrichment-via-link-strategy) | Registers an association. `via` is read. `link` is the CRUD writer. |

###### CRUDRepository(connection) {#csharp-crudrepository-crudrepository-connection}

Builds a table repository from an [IConnection](#connection).

**Definition**

```csharp
public Repository(IConnection connection)
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | open [IConnection](#connection) |

###### CRUDRepository(connection, options) {#csharp-crudrepository-crudrepository-connection-options}

Builds a table repository from an [IConnection](#connection) and [CRUDRepositoryOptions](#csharp-crudrepositoryoptions).

**Definition**

```csharp
public CRUDRepository(IConnection connection, CRUDRepositoryOptions<T> options)
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | open [IConnection](#connection) |
| `options` | [CRUDRepositoryOptions](#csharp-crudrepositoryoptions) |

###### CRUDRepository(inner, strategy) {#csharp-crudrepository-crudrepository-inner-strategy}

Wraps an inner [ICRUDRepository](#csharp-icrudrepository). Parent writes delegate to `inner`. Omitted strategy uses the default.

**Definition**

```csharp
public CRUDRepository(ICRUDRepository<T> inner, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [ICRUDRepository](#csharp-icrudrepository) |
| `strategy` | optional read enrichment strategy |

###### AddMiddleware(middleware) {#csharp-crudrepository-addmiddleware-middleware}

Adds command middleware. Returns this repository.

**Definition**

```csharp
public Repository<T> AddMiddleware(ICommandMiddleware middleware)
```

**Parameters**

| Name | Description |
| --- | --- |
| `middleware` | command middleware |

**Returns**

[CRUDRepository](#csharp-crudrepository)`<T>`

###### Read(query) {#csharp-crudrepository-read-query}

Reads rows. Convert the database row, then map it.

**Definition**

```csharp
public override IAsyncEnumerable<T> Read(IStatement? query = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional statement or where; omitted uses the default select |

**Returns**

`IAsyncEnumerable<T>`

###### Create(item) {#csharp-crudrepository-create-item}

Inserts one item.

**Definition**

```csharp
public Task<Response> Create(T item)
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to insert |

**Returns**

`Task<Response>`

###### Update(item, byWhere) {#csharp-crudrepository-update-item-bywhere}

Updates one item.

**Definition**

```csharp
public Task<Response> Update(T item, IWhere? byWhere = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to update |
| `byWhere` | optional predicate; omitted uses id values on `item` |

**Returns**

`Task<Response>`

###### Delete(item, byWhere) {#csharp-crudrepository-delete-item-bywhere}

Deletes one item.

**Definition**

```csharp
public Task<Response> Delete(T item, IWhere? byWhere = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to delete |
| `byWhere` | optional predicate; omitted uses id values on `item` |

**Returns**

`Task<Response>`

###### ReadSingleById(id) {#csharp-crudrepository-readsinglebyid-id}

Returns the row for the single configured id column.

**Definition**

```csharp
public Task<T?> ReadSingleById(Value id)
```

**Parameters**

| Name | Description |
| --- | --- |
| `id` | value of the single id column |

**Returns**

`Task<T?>`

###### ReadSingleById(ids) {#csharp-crudrepository-readsinglebyid-ids}

Returns the row whose configured id columns match `ids`.

**Definition**

```csharp
public Task<T?> ReadSingleById(Row ids)
```

**Parameters**

| Name | Description |
| --- | --- |
| `ids` | id column names and values |

**Returns**

`Task<T?>`

###### AddEnriched(enrichment, via, strategy) {#csharp-crudrepository-addenriched-enrichment-via-strategy}

Registers the same read fill as [ReadonlyRepository](#csharp-addenrichedenrichment-via-strategy). When `via` is a CRUD repository, `many` children are written with the parent.

**Definition**

```csharp
public IEnriched<T> AddEnriched<TRelated>(Enrichment enrichment, IReadonlyRepository<TRelated> via, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](#enrichment) |
| `via` | related repository. A CRUD repository is kept so `many` children can be written |
| `strategy` | optional read strategy for this edge. Omitted uses the default strategy |

**Returns**

[IEnriched](#ienriched)`<T>`

###### AddLinked(enrichment, via, link, strategy) {#csharp-crudrepository-addlinked-enrichment-via-link-strategy}

Registers an association. `via` is read. `link` is the CRUD writer.

**Definition**

```csharp
public IEnriched<T> AddLinked<TRelated, TLink>(Enrichment enrichment, IReadonlyRepository<TRelated> via, ICRUDRepository<TLink> link, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](#enrichment) with `Association` |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `link` | [ICRUDRepository](#csharp-icrudrepository) that stores the association rows |
| `strategy` | optional read strategy for this edge. Omitted uses the default strategy |

**Returns**

[IEnriched](#ienriched)`<T>`

##### CRUDRepositoryOptions {#csharp-crudrepositoryoptions}

```csharp
public sealed class CRUDRepositoryOptions<T> : ReadonlyRepositoryOptions<T> where T : new()
```

**Properties**

| Name | Description |
| --- | --- |
| Ids | Id column names. Default `["id"]`. |
| Insert | Rewrites the finished insert. Omitted leaves the statement unchanged. |
| Update | Rewrites the finished update. Omitted leaves the statement unchanged. |
| Delete | Rewrites the finished delete. Omitted leaves the statement unchanged. |
| Return | Rewrites a finished [ReturnStatement](#return-statement). Omitted leaves it unchanged. |

##### BuiltRepository {#csharp-builtrepository}

```csharp
public sealed record BuiltRepository<T> where T : new()
```

**Properties**

| Name | Description |
| --- | --- |
| Connection | Open [IConnection](#connection). |
| RowConverter | Row [TypeConverter](#type-converters). |
| RowsMapper | Read-row [Mapper](#mappers). |
| Statements | SQL fragments. |
| Ids | Id column names. |

##### Rust

##### ICRUDRepository {#rust-icrudrepository}

```rust
pub trait ICRUDRepository: IReadonlyRepository
```

**Methods**

| Name | Description |
| --- | --- |
| [create(item)](#rust-icrudrepository-create-item) | Inserts one row. |
| [update(item, by_where)](#rust-icrudrepository-update-item-by_where) | Updates one row. `None` `by_where` uses id columns. |
| [delete(item, by_where)](#rust-icrudrepository-delete-item-by_where) | Deletes one row. `None` `by_where` uses id columns. |
| [read_single_by_id(id)](#rust-icrudrepository-read_single_by_id-id) | Returns the row for one id column, or for a composite key. A value requires exactly one configured id column. A row must include every configured id column. No match returns `None`. |

###### create(item) {#rust-icrudrepository-create-item}

Inserts one row.

**Definition**

```rust
async fn create(&self, item: &mut Row) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to insert |

**Returns**

`Result<Response>`

###### update(item, by_where) {#rust-icrudrepository-update-item-by_where}

Updates one row. `None` `by_where` uses id columns.

**Definition**

```rust
async fn update(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to update |
| `by_where` | optional predicate; omitted uses id values on `item` |

**Returns**

`Result<Response>`

###### delete(item, by_where) {#rust-icrudrepository-delete-item-by_where}

Deletes one row. `None` `by_where` uses id columns.

**Definition**

```rust
async fn delete(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to delete |
| `by_where` | optional predicate; omitted uses id values on `item` |

**Returns**

`Result<Response>`

###### read_single_by_id(id) {#rust-icrudrepository-read_single_by_id-id}

Returns the row for one id column, or for a composite key. A value requires exactly one configured id column. A row must include every configured id column. No match returns `None`.

**Definition**

```rust
async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Row>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `id` | `Value` for the single id column, or a `Row` keyed by each id column |

**Returns**

`Result<Option<Row>>`

##### CRUDRepository {#rust-crudrepository}

```rust
pub struct CRUDRepository
```

**Constructor**

| Name | Description |
| --- | --- |
| [from_type(connection)](#rust-crudrepository-from_type-connection) | Builds a table repository from `T: ConvertSchema`. |
| [new(connection, options)](#rust-crudrepository-new-connection-options) | Builds a table repository from a [Connection](#connection) and [CRUDRepositoryOptions](#rust-crudrepositoryoptions). |
| [from_repository(inner)](#rust-crudrepository-from_repository-inner) | Wraps an inner [ICRUDRepository](#rust-icrudrepository). Parent writes delegate to `inner`. |

**Methods**

| Name | Description |
| --- | --- |
| [add_enriched(enrichment, via, strategy)](#rust-crudrepository-add_enriched-enrichment-via-strategy) | Registers the same read fill as [ReadonlyRepository](#rust-add_enrichedenrichment-via-strategy). When `via` is a CRUD repository, `many` children are written with the parent. |
| [add_linked(enrichment, via, link, strategy)](#rust-crudrepository-add_linked-enrichment-via-link-strategy) | Registers an association. `via` is read. `link` is the CRUD writer. |
| [add_middleware(middleware)](#rust-crudrepository-add_middleware-middleware) | Adds command middleware. |
| [read_single_by_id(id)](#rust-crudrepository-read_single_by_id-id) | Returns the row for one id column, or for a composite key. |

###### from_type(connection) {#rust-crudrepository-from_type-connection}

Builds a table repository from `T: ConvertSchema`.

**Definition**

```rust
pub fn from_type<T: ConvertSchema + 'static>(connection: Arc<dyn Connection>) -> Result<Self>
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | [Connection](#connection) |

**Returns**

`Result<`[CRUDRepository](#rust-crudrepository)`>`

###### new(connection, options) {#rust-crudrepository-new-connection-options}

Builds a table repository from a [Connection](#connection) and [CRUDRepositoryOptions](#rust-crudrepositoryoptions).

**Definition**

```rust
pub fn new(connection: Arc<dyn Connection>, options: CRUDRepositoryOptions) -> Result<Self>
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | [Connection](#connection) |
| `options` | [CRUDRepositoryOptions](#rust-crudrepositoryoptions) |

**Returns**

`Result<`[CRUDRepository](#rust-crudrepository)`>`

###### from_repository(inner) {#rust-crudrepository-from_repository-inner}

Wraps an inner [ICRUDRepository](#rust-icrudrepository). Parent writes delegate to `inner`.

**Definition**

```rust
pub fn from_repository(inner: Arc<dyn ICRUDRepository>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [ICRUDRepository](#rust-icrudrepository) |

**Returns**

[CRUDRepository](#rust-crudrepository)

###### add_enriched(enrichment, via, strategy) {#rust-crudrepository-add_enriched-enrichment-via-strategy}

Registers the same read fill as [ReadonlyRepository](#rust-add_enrichedenrichment-via-strategy). When `via` is a CRUD repository, `many` children are written with the parent.

**Definition**

```rust
pub fn add_enriched(&mut self, enrichment: Enrichment, via: impl EnrichmentVia, strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>) -> &mut Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](#enrichment) |
| `via` | related repository. A CRUD repository is kept so `many` children can be written |
| `strategy` | optional read strategy for this edge. Omitted uses the default strategy |

**Returns**

`&mut Self`

###### add_linked(enrichment, via, link, strategy) {#rust-crudrepository-add_linked-enrichment-via-link-strategy}

Registers an association. `via` is read. `link` is the CRUD writer.

**Definition**

```rust
pub fn add_linked(&mut self, enrichment: Enrichment, via: impl EnrichmentVia, link: Arc<dyn ICRUDRepository>, strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>) -> &mut Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](#enrichment) with `association` |
| `via` | related readonly repository |
| `link` | [ICRUDRepository](#rust-icrudrepository) that stores the association rows |
| `strategy` | optional read strategy for this edge. Omitted uses the default strategy |

**Returns**

`&mut Self`

###### add_middleware(middleware) {#rust-crudrepository-add_middleware-middleware}

Adds command middleware.

**Definition**

```rust
pub fn add_middleware(&mut self, middleware: Arc<dyn CommandMiddleware>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `middleware` | command middleware |

###### read_single_by_id(id) {#rust-crudrepository-read_single_by_id-id}

Returns the row for one id column, or for a composite key.

**Definition**

```rust
pub async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Row>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `id` | `Value` for the single id column, or a `Row` keyed by each id column |

**Returns**

`Result<Option<Row>>`

##### CRUDRepositoryOptions {#rust-crudrepositoryoptions}

```rust
pub struct CRUDRepositoryOptions
```

**Properties**

| Name | Description |
| --- | --- |
| ids | Id column names. Default `["id"]`. |
| insert | Rewrites finished insert SQL. `None` leaves the statement unchanged. |
| update | Rewrites finished update SQL. `None` leaves the statement unchanged. |
| delete | Rewrites finished delete SQL. `None` leaves the statement unchanged. |
| return_ | Rewrites finished return SQL. `None` leaves it unchanged. |
| read | The embedded [ReadonlyRepositoryOptions](#rust-readonlyrepositoryoptions). Field access goes through `Deref`. |

##### BuiltRepository {#rust-builtrepository}

```rust
pub struct BuiltRepository
```

**Properties**

| Name | Description |
| --- | --- |
| connection | [Connection](#connection). |
| row_converter | [RowTypeConverter](#row-type-converter). |
| rows_mapper | Read-row [Mapper](#mappers). |
| statements | SQL fragments. |
| ids | Id column names. |

### Remarks

Update requires at least one non-id field. Missing id values throw. Insert always omits configured id columns. After a table insert with one id column, create reads `SelectInsertedId`, converts the row, maps it, and copies the id onto the entity. The driver statement factory formats that statement. A stored-procedure create reads that id from the `Return` mapper instead. A null cell is omitted on create and update, so a SQL default applies. A present value is written.

`ReadSingleById` / `readSingleById` / `read_single_by_id` takes a value when the repository has one id column, and a row keyed by each id column when the key is composite.

[AddEnriched](#ienriched) registers the same read fill as [ReadonlyRepository](#readonly-repository). Lookups are filled on read and are not written. When `via` is a CRUD repository, `many` children are created, replace-synced, and deleted with the parent. `AddLinked` reads `via` and writes the association through `link`. Eager write replaces that junction set. A null or empty list clears the association and leaves the others. Related rows stay. [LazyWriteEnrichmentStrategy](#lazywriteenrichmentstrategy) skips a null list on create and update. Delete still removes the junction rows. Passing `byWhere` to `Update` replaces the id predicate, so a version check includes both the id and the version.

[DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy) takes `useTransaction`, off by default. When it is on, `Create`, `Update`, and `Delete` commit the parent write and nested persist together. A bulk list uses one transaction for the whole list. Nested writes join when they use the same connection.

- **Create** — write the parent, write the generated identity onto the parent's id column, stamp each child's `by` from the parent `from`, then create children (and nested many).
- **Update** — [DefaultUpdateEnrichmentStrategy](#defaultupdateenrichmentstrategy) is the default. Write the parent, then replace-sync each many list (delete missing, update matches by the configured id, create the rest). [CascadeDeleteUpdateEnrichmentStrategy](#cascadedeleteupdateenrichmentstrategy) deletes every existing child, then inserts the payload as new rows. Recreated children get new ids. Links are replaced the same way. Related rows stay.
- **Delete** — [CascadeDeleteEnrichmentStrategy](#cascadedeleteenrichmentstrategy) is the default. Delete nested many rows first, then children, then the parent. Loads existing children by FK; does not trust payload lists.

See [Bulk repository](#bulk-repository) for list writes.

### Examples

#### Simple CRUD

The table, columns, and id come from `User`. Create omits `id`. The repository reads the inserted id and writes it onto `user.id`.

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L383-L410)

```csharp
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
var connection = await Open(ds, "user-pascal");
var usersRepo = new CRUDRepository<User>(connection);
var created = await usersRepo.Create(new User { username = "ada", is_enabled = true });
```

**Created**

```
id: 1
```

**Read by id**

```
{
  id: 1,
  username: 'ada',
  is_enabled: true
}
```

**Read by username and enabled**

```
ada
```

**After update**

```
ada-v2
```

**After delete**

```
no row
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L558-L607)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
let connection = open(ds, "user-pascal", "").await;
let users_repo = CRUDRepository::from_type::<User>(connection).unwrap();
let mut created = datasources::Row::new();
created.insert("username".into(), Value::from("ada"));
created.insert("is_enabled".into(), Value::Bool(true));
let mut created = users_repo.create(&mut created).await.unwrap();
```

**Created**

```
id: 1
```

**Read by id**

```
{
  id: 1,
  username: 'ada',
  is_enabled: 1
}
```

**Read by username and enabled**

```
ada
```

**After update**

```
ada-v2
```

**After delete**

```
no row
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L161-L183)

```typescript
const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
const connection = await open(ds, "user-pascal");
const usersRepo = new CRUDRepository(User, connection);
const created = await usersRepo.create(Object.assign(new User(), { username: "ada", is_enabled: true }));
```

**Created**

```
id: 1
```

**Read by id**

```
{
  id: 1,
  username: 'ada',
  is_enabled: 1
}
```

**Read by username and enabled**

```
ada
```

**After update**

```
ada-v2
```

**After delete**

```
no row
```

#### Enriched CRUD

`AddLinked` reads roles and groups. Create, update, and delete write `user_roles` and `user_groups`. The related rows stay. Update drops the admin role and adds ops. A null or empty list would clear that association.

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/UserViewCRUDRepositoryTests.cs#L120-L192)

```csharp
var graph = await GraphAsync();
var usersRepo = graph.UsersRepo;
usersRepo.AddEnriched(new("UserType", "UserTypeId", "Id"), graph.UserTypesRepo);
usersRepo.AddLinked(new("Roles", "Id", "Id", Association: new("UserId", "RoleId")), graph.RolesRepo, graph.UserRolesRepo);
usersRepo.AddLinked(new("Groups", "Id", "Id", Association: new("UserId", "GroupId")), graph.GroupsRepo, graph.UserGroupsRepo);
var created = await usersRepo.Create(new UserView
{
    Username = "ada",
    IsEnabled = true,
    UserTypeId = graph.AdminType.Id,
    Roles = [graph.AdminRole, graph.EditorRole],
    Groups = [graph.Engineering],
});
var createdRoleLinks = await graph.UserRolesRepo.Read().ToListAsync();
var createdGroupLinks = await graph.UserGroupsRepo.Read().ToListAsync();
var saved = await usersRepo.Update(new UserView
{
    Id = created.Id,
    Username = "ada",
    IsEnabled = true,
    UserTypeId = graph.AdminType.Id,
    Roles = [graph.EditorRole],
    Groups = [graph.Engineering, graph.Ops],
});
var updated = await usersRepo.ReadSingleById(created.Id);
var updatedRoleLinks = await graph.UserRolesRepo.Read().ToListAsync();
var updatedGroupLinks = await graph.UserGroupsRepo.Read().ToListAsync();
var removed = await usersRepo.Delete(new UserView { Id = created.Id });
```

**Created**

```
{
  UserType: 'admin',
  Roles: ['admin', 'editor'],
  Groups: ['engineering']
}
```

**Updated**

```
{
  Roles: ['editor'],
  Groups: ['engineering', 'ops']
}
```

**After delete**

```
no user
roles and groups remain
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/user_view.rs#L151-L306)

```rust
let (mut users, seeded) = open(None).await;
users
    .add_enriched(
        Enrichment::new("user_type", "user_type_id", "id", None, false),
        seeded.user_types.clone(),
        None,
    )
    .add_linked(
        Enrichment::new("roles", "id", "id", None, false)
            .linked(Association::new("user_id", "role_id")),
        seeded.roles.clone(),
        seeded.user_roles.clone(),
        None,
    )
    .add_linked(
        Enrichment::new("groups", "id", "id", None, false)
            .linked(Association::new("user_id", "group_id")),
        seeded.groups.clone(),
        seeded.user_groups.clone(),
        None,
    );
let created = users
    .create(&mut ada(
        &seeded,
        encode_rows(&[seeded.admin_role.clone(), seeded.editor_role.clone()]),
        encode_rows(&[seeded.engineering.clone()]),
    ))
    .await
    .unwrap();
let created_role_links = seeded.user_roles.read(None, None).await.unwrap();
let created_group_links = seeded.user_groups.read(None, None).await.unwrap();
let mut next = ada(
    &seeded,
    encode_rows(&[seeded.editor_role.clone()]),
    encode_rows(&[seeded.engineering.clone(), seeded.ops.clone()]),
);
next.insert("id".into(), created.get("id").unwrap().clone());
let saved = users.update(&next, None).await.unwrap();
let updated = users
    .read_single_by_id(created.get("id").unwrap())
    .await
    .unwrap()
    .unwrap();
let updated_role_links = seeded.user_roles.read(None, None).await.unwrap();
let updated_group_links = seeded.user_groups.read(None, None).await.unwrap();
let removed = users
    .delete(&record(&[("id", created.get("id").unwrap().clone())]), None)
    .await
    .unwrap();
```

**Created**

```
{
  UserType: 'admin',
  Roles: ['admin', 'editor'],
  Groups: ['engineering']
}
```

**Updated**

```
{
  Roles: ['editor'],
  Groups: ['engineering', 'ops']
}
```

**After delete**

```
no user
roles and groups remain
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/UserViewCRUDRepository.test.ts#L107-L172)

```typescript
const seeded = await graph();
const usersRepo = seeded.usersRepo;
usersRepo.addEnriched({ into: "user_type", from: "user_type_id", by: "id" }, seeded.userTypesRepo);
usersRepo.addLinked(
  { into: "roles", from: "id", by: "id", association: { parent: "user_id", related: "role_id" } },
  seeded.rolesRepo,
  seeded.userRolesRepo,
);
usersRepo.addLinked(
  { into: "groups", from: "id", by: "id", association: { parent: "user_id", related: "group_id" } },
  seeded.groupsRepo,
  seeded.userGroupsRepo,
);
const created = await usersRepo.create(Object.assign(new UserView(), {
  username: "ada",
  is_enabled: true,
  user_type_id: seeded.adminType.id,
  roles: [seeded.adminRole, seeded.editorRole],
  groups: [seeded.engineering],
}));
const createdRoleLinks = await collectAsync(seeded.userRolesRepo.read());
const createdGroupLinks = await collectAsync(seeded.userGroupsRepo.read());
const saved = await usersRepo.update(Object.assign(new UserView(), {
  id: created.id,
  username: "ada",
  is_enabled: true,
  user_type_id: seeded.adminType.id,
  roles: [seeded.editorRole],
  groups: [seeded.engineering, seeded.ops],
}));
const updated = await usersRepo.readSingleById(created.id);
const updatedRoleLinks = await collectAsync(seeded.userRolesRepo.read());
const updatedGroupLinks = await collectAsync(seeded.userGroupsRepo.read());
const removed = await usersRepo.delete(Object.assign(new UserView(), { id: created.id }));
```

**Created**

```
{
  UserType: 'admin',
  Roles: ['admin', 'editor'],
  Groups: ['engineering']
}
```

**Updated**

```
{
  Roles: ['editor'],
  Groups: ['engineering', 'ops']
}
```

**After delete**

```
no user
roles and groups remain
```

#### Multi-tenant

`account` is identified by `tenant_id` and `id`. The same `id` in another tenant is a different row. `ReadSingleById` / `readSingleById` / `read_single_by_id`, update, and delete take both keys. Create omits every configured id column, so this sample seeds the rows.

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L434-L463)

```csharp
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
var connection = await Open(ds, "multi-tenant", """
    INSERT INTO account (tenant_id, id, name) VALUES
        (1, 1, 'ada'),
        (2, 1, 'grace');
    """);
var accountsRepo = new CRUDRepository<Account>(connection, new CRUDRepositoryOptions<Account>
{
    Entity = "account",
    Ids = ["tenant_id", "id"],
});
var ada = await accountsRepo.ReadSingleById(new Row { ["tenant_id"] = 1, ["id"] = 1 });
ada!.name = "ada-v2";
var updated = await accountsRepo.Update(ada);
var renamed = await accountsRepo.ReadSingleById(new Row { ["tenant_id"] = 1, ["id"] = 1 });
var removed = await accountsRepo.Delete(ada);
var gone = await accountsRepo.ReadSingleById(new Row { ["tenant_id"] = 1, ["id"] = 1 });
var grace = await accountsRepo.ReadSingleById(new Row { ["tenant_id"] = 2, ["id"] = 1 });
```

**Tenant 1**

```
ada-v2
```

**Tenant 2**

```
grace
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L739-L782)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
let connection = open(
    ds,
    "multi-tenant",
    "
    INSERT INTO account (tenant_id, id, name) VALUES
        (1, 1, 'ada'),
        (2, 1, 'grace');
    ",
)
.await;
let mut options = create_crud_repository_options::<Account>();
options.entity = "account".into();
options.ids = vec!["tenant_id".into(), "id".into()];
options.row_converter = Some(RowTypeConverter::of::<Account>());
let accounts_repo = CRUDRepository::new(connection, options).unwrap();
let mut key = datasources::Row::new();
key.insert("tenant_id".into(), Value::Int(1));
key.insert("id".into(), Value::Int(1));
let mut ada = accounts_repo.read_single_by_id(&key).await.unwrap().unwrap();
ada.insert("name".into(), Value::from("ada-v2"));
let updated = accounts_repo.update(&ada, None).await.unwrap();
let renamed = accounts_repo.read_single_by_id(&key).await.unwrap().unwrap();
let removed = accounts_repo.delete(&ada, None).await.unwrap();
let gone = accounts_repo.read_single_by_id(&key).await.unwrap();
let mut other = datasources::Row::new();
other.insert("tenant_id".into(), Value::Int(2));
other.insert("id".into(), Value::Int(1));
let grace = accounts_repo
    .read_single_by_id(&other)
    .await
    .unwrap()
    .unwrap();
```

**Tenant 1**

```
ada-v2
```

**Tenant 2**

```
grace
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L283-L309)

```typescript
const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
const connection = await open(ds, "multi-tenant", `
  INSERT INTO account (tenant_id, id, name) VALUES
    (1, 1, 'ada'),
    (2, 1, 'grace');
`);
const accountsRepo = new CRUDRepository(connection, {
  ...createCRUDRepositoryOptions(Account),
  entity: "account",
  ids: ["tenant_id", "id"],
});
const ada = await accountsRepo.readSingleById({ tenant_id: 1, id: 1 });
ada!.name = "ada-v2";
const updated = await accountsRepo.update(ada!);
const renamed = await accountsRepo.readSingleById({ tenant_id: 1, id: 1 });
const removed = await accountsRepo.delete(ada!);
const gone = await accountsRepo.readSingleById({ tenant_id: 1, id: 1 });
const graceRow = await accountsRepo.readSingleById({ tenant_id: 2, id: 1 });
```

**Tenant 1**

```
ada-v2
```

**Tenant 2**

```
grace
```

#### Enriched contacts write

`AddEnriched` / `addEnriched` / `add_enriched` fills the contact type and source names, and writes the address and phone lists with the contact.

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ContactsRepositoryTests.cs#L74-L148)

```csharp
var conn = new SqliteConnection(new SqliteConnectionOptions
{
    Filename = ":memory:",
});
await conn.Open();
var command = new SqliteCommand(conn);
foreach (var statement in SchemaSpec.Tables("contacts", Quote))
    await command.Execute(statement);
foreach (var sql in StatementsOf(File.ReadAllText(SeedPath())))
    await command.Execute(new Native(sql));

var contactsRepo = new CRUDRepository<Contact>(conn, new CRUDRepositoryOptions<Contact>
{
    Entity = "contacts_base",
    ReadProperties = [
        "id", "uuid", "created", "updated", "version",
        "contact_source_id", "contact_type_id", "first_name", "last_name", "email", "notes"
    ],
});
var typesRepo = new CRUDRepository<ContactType>(conn, new CRUDRepositoryOptions<ContactType>
{
    Entity = "contact_type",
});
var sourcesRepo = new CRUDRepository<ContactSource>(conn, new CRUDRepositoryOptions<ContactSource>
{
    Entity = "contact_source",
});
var addressTypesRepo = new CRUDRepository<AddressType>(conn, new CRUDRepositoryOptions<AddressType>
{
    Entity = "address_type",
});
var phoneTypesRepo = new CRUDRepository<PhoneType>(conn, new CRUDRepositoryOptions<PhoneType>
{
    Entity = "phone_type",
});
var addressesRepo = new CRUDRepository<Address>(conn, new CRUDRepositoryOptions<Address>
{
    Entity = "address",
    ReadProperties = [
        "id", "uuid", "created", "updated", "version", "contact_id", "address_type_id",
        "line1", "line2", "city", "region", "postal_code", "country"
    ],
});
var phonesRepo = new CRUDRepository<Phone>(conn, new CRUDRepositoryOptions<Phone>
{
    Entity = "phone",
    ReadProperties = [
        "id", "uuid", "created", "updated", "version", "contact_id", "phone_type_id", "number"
    ],
});
addressesRepo.AddEnriched(new("address_type_name", "address_type_id", "id", "name"), addressTypesRepo);
phonesRepo.AddEnriched(new("phone_type_name", "phone_type_id", "id", "name"), phoneTypesRepo);
contactsRepo.AddEnriched(new("contact_type_name", "contact_type_id", "id", "name"), typesRepo);
contactsRepo.AddEnriched(new("contact_source_name", "contact_source_id", "id", "name"), sourcesRepo);
contactsRepo.AddEnriched(new("addresses", "id", "contact_id", Many: true), addressesRepo);
contactsRepo.AddEnriched(new("phones", "id", "contact_id", Many: true), phonesRepo);

var created = await contactsRepo.Create(ExtraContact.New(await ContactLookupIds.Resolve(conn)));
var loaded = await contactsRepo.ReadSingle(new Where(new Operator.Equals("id", Value.From(created.id))));
```

**Created**

```
{
  first_name: 'Extra',
  contact_type_name: 'Person',
  addresses: ['9 Extra'],
  phones: ['555-extra']
}
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/contacts.rs#L124-L329)

```rust
let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
conn.open().await.unwrap();
let command = SqliteCommand::new(conn.clone());
let sql = statements_of(&contact_schema("sqlite", render));
apply_sql(&command, &sql).await.unwrap();

let field = super::converters::field;
let named = [
    "id",
    "uuid",
    "created",
    "updated",
    "version",
    "name",
    "description",
];
let address = [
    "id",
    "uuid",
    "created",
    "updated",
    "version",
    "contact_id",
    "address_type_id",
    "line1",
    "line2",
    "city",
    "region",
    "postal_code",
    "country",
];
let phone = [
    "id",
    "uuid",
    "created",
    "updated",
    "version",
    "contact_id",
    "phone_type_id",
    "number",
];
let base = [
    "id",
    "uuid",
    "created",
    "updated",
    "version",
    "contact_source_id",
    "contact_type_id",
    "first_name",
    "last_name",
    "email",
    "notes",
];
let mut addresses_repo = contact_repo(conn.clone(), "address", &address, field);
addresses_repo.add_enriched(
    datasources::Enrichment::new(
        "address_type_name",
        "address_type_id",
        "id",
        Some("name".into()),
        false,
    ),
    as_readonly(Arc::new(contact_repo(
        conn.clone(),
        "address_type",
        &named,
        field,
    ))),
    None,
);
let mut phones_repo = contact_repo(conn.clone(), "phone", &phone, field);
phones_repo.add_enriched(
    datasources::Enrichment::new(
        "phone_type_name",
        "phone_type_id",
        "id",
        Some("name".into()),
        false,
    ),
    as_readonly(Arc::new(contact_repo(conn.clone(), "phone_type", &named, field))),
    None,
);
let mut contacts_repo = contact_repo(conn.clone(), "contacts_base", &base, field);
contacts_repo
    .add_enriched(
        datasources::Enrichment::new(
            "contact_type_name",
            "contact_type_id",
            "id",
            Some("name".into()),
            false,
        ),
        as_readonly(Arc::new(contact_repo(
            conn.clone(),
            "contact_type",
            &named,
            field,
        ))),
        None,
    )
    .add_enriched(
        datasources::Enrichment::new(
            "contact_source_name",
            "contact_source_id",
            "id",
            Some("name".into()),
            false,
        ),
        as_readonly(Arc::new(contact_repo(
            conn.clone(),
            "contact_source",
            &named,
            field,
        ))),
        None,
    )
    .add_enriched(
        datasources::Enrichment::new("addresses", "id", "contact_id", None, true),
        Arc::new(addresses_repo),
        None,
    )
    .add_enriched(
        datasources::Enrichment::new("phones", "id", "contact_id", None, true),
        Arc::new(phones_repo),
        None,
    );

let ids = contact_lookup_ids(conn).await.unwrap();
let stamp = Value::DateTime(
    chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
        .expect("stamp")
        .with_timezone(&chrono::Utc),
);
let version = Value::Binary(vec![]);
let mut created = contact_row(&[
    ("uuid", Value::Uuid(uuid::Uuid::new_v4())),
    ("created", stamp.clone()),
    ("updated", stamp.clone()),
    ("version", version.clone()),
    ("contact_source_id", Value::Int(ids.contact_source_id)),
    ("contact_type_id", Value::Int(ids.contact_type_id)),
    ("first_name", Value::from("Extra")),
    ("last_name", Value::from("Contact")),
    (
        "addresses",
        encode_rows(&[contact_row(&[
            ("uuid", Value::Uuid(uuid::Uuid::new_v4())),
            ("created", stamp.clone()),
            ("updated", stamp.clone()),
            ("version", version.clone()),
            ("address_type_id", Value::Int(ids.address_type_id)),
            ("line1", Value::from("9 Extra")),
            ("city", Value::from("Oslo")),
        ])]),
    ),
    (
        "phones",
        encode_rows(&[contact_row(&[
            ("uuid", Value::Uuid(uuid::Uuid::new_v4())),
            ("created", stamp.clone()),
            ("updated", stamp),
            ("version", version),
            ("phone_type_id", Value::Int(ids.phone_type_id)),
            ("number", Value::from("555-extra")),
        ])]),
    ),
]);
let created = contacts_repo.create(&mut created).await.unwrap();
let Value::Int(id) = created.get("id").cloned().unwrap_or(Value::Null) else {
    panic!("expected insert id");
};
let loaded = contacts_repo
    .read_single(
        Some(&Where::new(Operator::equals("id", Value::Int(id)))),
        None,
    )
    .await
    .unwrap()
    .unwrap();
```

**Created**

```
{
  first_name: 'Extra',
  contact_type_name: 'Person',
  addresses: ['9 Extra'],
  phones: ['555-extra']
}
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/contacts.repository.integration.test.ts#L96-L162)

```typescript
const connection = SqliteConnection.from({ DATASOURCE_URL: "sqlite:///:memory:" });
await connection.open();
const command = new SqliteCommand(connection);
for (const statement of tables("contacts", quote))
  await execute(command, statement);
for (const sql of statementsOf(readFileSync(seedPath, "utf8")))
  await command.executeNative(new Native(sql));

const contactsRepo = new CRUDRepository(connection, {
  type: Contact,
  entity: "contacts_base",
  readProperties: fieldNamesOf(ContactBase),
});
const typesRepo = new CRUDRepository(connection, { type: ContactType, entity: "contact_type" });
const sourcesRepo = new CRUDRepository(connection, { type: ContactSource, entity: "contact_source" });
const addressTypesRepo = new CRUDRepository(connection, { type: AddressType, entity: "address_type" });
const phoneTypesRepo = new CRUDRepository(connection, { type: PhoneType, entity: "phone_type" });
const addressesRepo = new CRUDRepository(connection, {
  type: Address,
  entity: "address",
  readProperties: [
    "id", "uuid", "created", "updated", "version", "contact_id", "address_type_id",
    "line1", "line2", "city", "region", "postal_code", "country",
  ],
});
const phonesRepo = new CRUDRepository(connection, {
  type: Phone,
  entity: "phone",
  readProperties: [
    "id", "uuid", "created", "updated", "version", "contact_id", "phone_type_id", "number",
  ],
});
addressesRepo.addEnriched(
  { into: "address_type_name", from: "address_type_id", by: "id", select: "name" },
  addressTypesRepo,
);
phonesRepo.addEnriched(
  { into: "phone_type_name", from: "phone_type_id", by: "id", select: "name" },
  phoneTypesRepo,
);
contactsRepo.addEnriched(
  { into: "contact_type_name", from: "contact_type_id", by: "id", select: "name" },
  typesRepo,
);
contactsRepo.addEnriched(
  { into: "contact_source_name", from: "contact_source_id", by: "id", select: "name" },
  sourcesRepo,
);
contactsRepo.addEnriched({ into: "addresses", from: "id", by: "contact_id", many: true }, addressesRepo);
contactsRepo.addEnriched({ into: "phones", from: "id", by: "contact_id", many: true }, phonesRepo);

const created = await contactsRepo.create(extraContact(await contactLookupIds(connection)));
const loaded = await contactsRepo.readSingle(new Where(new Operator.Equals("id", created.id)));
```

**Created**

```
{
  first_name: 'Extra',
  contact_type_name: 'Person',
  addresses: ['9 Extra'],
  phones: ['555-extra']
}
```

#### Type mapping

`UsersView` maps onto `user`. `Key` is `id`, `Login` is `username`, and `UserTypeId` is `user_type_id`. `UserType` is the enriched name and is not a column. Create writes the generated id back onto `Key`.

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L504-L576)

```csharp
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
var connection = await Open(ds, "user-view-version");
var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
var usersRepo = new CRUDRepository<UsersView>(connection, new CRUDRepositoryOptions<UsersView>
{
    Ids = ["Key"],
    ReadProperties = ["Key", "Login", "Active", "Opened", "UserTypeId", "Version"],
    EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UsersView"] = "user" }),
    ColumnMappings = new DictionaryMapper<string>(new Dictionary<string, string>
    {
        ["Key"] = "id",
        ["Login"] = "username",
        ["Active"] = "is_enabled",
        ["Opened"] = "created",
        ["UserTypeId"] = "user_type_id",
        ["Version"] = "version",
    }),
});
var userTypesRepo = new CRUDRepository<UserType>(connection, new CRUDRepositoryOptions<UserType>
{
    Ids = ["Id"],
    EntityMapper = casing,
    ColumnMappings = casing,
});
var userRolesRepo = new CRUDRepository<UserRole>(connection, new CRUDRepositoryOptions<UserRole>
{
    Ids = [],
    EntityMapper = casing,
    ColumnMappings = casing,
});
usersRepo.AddEnriched(new("UserType", "UserTypeId", "Id", "Name"), userTypesRepo);
usersRepo.AddEnriched(new("Roles", "Key", "UserId", Many: true), userRolesRepo);
var admin = await userTypesRepo.Create(new UserType { Name = "admin" });
```

**Created**

```
{
  id: 1,
  Key: 1
}
```

**Enriched Ada**

```
{
  UserType: 'admin',
  Roles: ['admin', 'editor']
}
```

**Versioned update**

```
{
  rowsAffected: 1,
  version: 2,
  Login: 'ada-v2'
}
```

**Stale update**

```
{
  rowsAffected: 0
}
```

**After delete**

```
no row
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L609-L725)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
let connection = open(ds, "user-view-version", "").await;
let mut users_options = create_crud_repository_options::<UsersView>();
users_options.ids = vec!["Key".into()];
users_options.entity_mapper = Box::new(DictionaryMapper::new([(
    "UsersView".to_string(),
    "user".to_string(),
)]));
users_options.column_mappings = Box::new(DictionaryMapper::new([
    ("Key".to_string(), "id".to_string()),
    ("Login".to_string(), "username".to_string()),
    ("Active".to_string(), "is_enabled".to_string()),
    ("Opened".to_string(), "created".to_string()),
    ("UserTypeId".to_string(), "user_type_id".to_string()),
    ("Version".to_string(), "version".to_string()),
]));
users_options.row_converter = Some(RowTypeConverter::of::<UsersView>());
let casing = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
let mut type_options = create_crud_repository_options::<UserType>();
type_options.entity_mapper =
    Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
type_options.column_mappings = Box::new(casing);
type_options.row_converter = Some(RowTypeConverter::of::<UserType>());
let mut role_options = create_crud_repository_options::<UserRole>();
role_options.entity_mapper =
    Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
role_options.column_mappings =
    Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
role_options.row_converter = Some(RowTypeConverter::of::<UserRole>());
let mut users_repo = CRUDRepository::new(connection.clone(), users_options).unwrap();
let user_types_repo =
    Arc::new(CRUDRepository::new(connection.clone(), type_options).unwrap());
let user_roles_repo = Arc::new(CRUDRepository::new(connection, role_options).unwrap());
users_repo.add_enriched(
    datasources::Enrichment::new(
        "UserType",
        "UserTypeId",
        "Id",
        Some("Name".into()),
        false,
    ),
    user_types_repo.clone(),
    None,
);
users_repo.add_enriched(
    datasources::Enrichment::new("Roles", "Key", "UserId", None, true),
    user_roles_repo.clone(),
    None,
);
let mut admin = datasources::Row::new();
admin.insert("Name".into(), Value::from("admin"));
let admin = user_types_repo.create(&mut admin).await.unwrap();
```

**Created**

```
{
  id: 1,
  Key: 1
}
```

**Enriched Ada**

```
{
  UserType: 'admin',
  Roles: ['admin', 'editor']
}
```

**Versioned update**

```
{
  rowsAffected: 1,
  version: 2,
  Login: 'ada-v2'
}
```

**Stale update**

```
{
  rowsAffected: 0
}
```

**After delete**

```
no row
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L208-L273)

```typescript
const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
const connection = await open(ds, "user-view-version");
const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
const usersRepo = new CRUDRepository(connection, {
  ...createCRUDRepositoryOptions(UsersView),
  ids: ["Key"],
  readProperties: ["Key", "Login", "Active", "Opened", "UserTypeId", "Version"],
  entityMapper: new DictionaryMapper([["UsersView", "user"]]),
  columnMappings: new DictionaryMapper([
    ["Key", "id"],
    ["Login", "username"],
    ["Active", "is_enabled"],
    ["Opened", "created"],
    ["UserTypeId", "user_type_id"],
    ["Version", "version"],
  ]),
});
const userTypesRepo = new CRUDRepository(connection, {
  ...createCRUDRepositoryOptions(UserType),
  entityMapper: casing,
  columnMappings: casing,
});
const userRolesRepo = new CRUDRepository(connection, {
  ...createCRUDRepositoryOptions(UserRole),
  entityMapper: casing,
  columnMappings: casing,
});
usersRepo.addEnriched({ into: "UserType", from: "UserTypeId", by: "Id", select: "Name" }, userTypesRepo);
usersRepo.addEnriched({ into: "Roles", from: "Key", by: "UserId", many: true }, userRolesRepo);
const admin = await userTypesRepo.create(Object.assign(new UserType(), { Name: "admin" }));
```

**Created**

```
{
  id: 1,
  Key: 1
}
```

**Enriched Ada**

```
{
  UserType: 'admin',
  Roles: ['admin', 'editor']
}
```

**Versioned update**

```
{
  rowsAffected: 1,
  version: 2,
  Login: 'ada-v2'
}
```

**Stale update**

```
{
  rowsAffected: 0
}
```

**After delete**

```
no row
```

## Bulk repository

[CRUDRepository](#crud-repository) plus list writes. Each item still runs the same deep create / update / delete. Responses sum `rowsAffected`; `result` is the last item.

### Classes

##### TypeScript

##### BulkCRUDRepository {#typescript-bulkcrudrepository}

```typescript
class BulkCRUDRepository<T extends object> extends CRUDRepository<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [BulkCRUDRepository(inner)](#typescript-bulkcrudrepository-bulkcrudrepository-inner) | Wraps an inner [ICRUDRepository](#crud-repository). |

**Methods**

| Name | Description |
| --- | --- |
| [createBulk(items)](#typescript-bulkcrudrepository-createbulk-items) | Creates each item, including nested `many` children. |
| [updateBulk(items, byWhere?)](#typescript-bulkcrudrepository-updatebulk-items-bywhere) | Updates each item, then replace-syncs nested `many` lists. |
| [deleteBulk(items, byWhere?)](#typescript-bulkcrudrepository-deletebulk-items-bywhere) | Deletes nested `many` rows for each item, then the parents. |

###### BulkCRUDRepository(inner) {#typescript-bulkcrudrepository-bulkcrudrepository-inner}

Wraps an inner [ICRUDRepository](#crud-repository).

**Definition**

```typescript
constructor(inner: ICRUDRepository<T>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [ICRUDRepository](#crud-repository) |

###### createBulk(items) {#typescript-bulkcrudrepository-createbulk-items}

Creates each item, including nested `many` children.

**Definition**

```typescript
createBulk(items: T[]): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to insert |

**Returns**

`Promise<Response>`

###### updateBulk(items, byWhere?) {#typescript-bulkcrudrepository-updatebulk-items-bywhere}

Updates each item, then replace-syncs nested `many` lists.

**Definition**

```typescript
updateBulk(items: T[], byWhere?: IWhere): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to update |
| `byWhere` | optional predicate applied to each item |

**Returns**

`Promise<Response>`

###### deleteBulk(items, byWhere?) {#typescript-bulkcrudrepository-deletebulk-items-bywhere}

Deletes nested `many` rows for each item, then the parents.

**Definition**

```typescript
deleteBulk(items: T[], byWhere?: IWhere): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to delete |
| `byWhere` | optional predicate applied to each item |

**Returns**

`Promise<Response>`

##### C#

##### BulkCRUDRepository {#csharp-bulkcrudrepository}

```csharp
public class BulkCRUDRepository<T> : CRUDRepository<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [BulkCRUDRepository(inner)](#csharp-bulkcrudrepository-bulkcrudrepository-inner) | Wraps an inner [ICRUDRepository](#crud-repository). |

**Methods**

| Name | Description |
| --- | --- |
| [Create(items)](#csharp-bulkcrudrepository-create-items) | Creates each item, including nested `many` children. |
| [Update(items, byWhere)](#csharp-bulkcrudrepository-update-items-bywhere) | Updates each item, then replace-syncs nested `many` lists. |
| [Delete(items, byWhere)](#csharp-bulkcrudrepository-delete-items-bywhere) | Deletes nested `many` rows for each item, then the parents. |
| [BulkCreate(items)](#csharp-bulkcrudrepository-bulkcreate-items) | Alias of [Create](#csharp-bulkcrudrepository-create-items). |
| [BulkUpdate(items, byWhere)](#csharp-bulkcrudrepository-bulkupdate-items-bywhere) | Alias of [Update](#csharp-bulkcrudrepository-update-items-bywhere). |
| [BulkDelete(items, byWhere)](#csharp-bulkcrudrepository-bulkdelete-items-bywhere) | Alias of [Delete](#csharp-bulkcrudrepository-delete-items-bywhere). |

###### BulkCRUDRepository(inner) {#csharp-bulkcrudrepository-bulkcrudrepository-inner}

Wraps an inner [ICRUDRepository](#crud-repository).

**Definition**

```csharp
public BulkCRUDRepository(ICRUDRepository<T> inner)
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [ICRUDRepository](#crud-repository) |

###### Create(items) {#csharp-bulkcrudrepository-create-items}

Creates each item, including nested `many` children.

**Definition**

```csharp
public Task<Response> Create(IReadOnlyList<T> items)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to insert |

**Returns**

`Task<Response>`

###### Update(items, byWhere) {#csharp-bulkcrudrepository-update-items-bywhere}

Updates each item, then replace-syncs nested `many` lists.

**Definition**

```csharp
public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to update |
| `byWhere` | optional predicate applied to each item |

**Returns**

`Task<Response>`

###### Delete(items, byWhere) {#csharp-bulkcrudrepository-delete-items-bywhere}

Deletes nested `many` rows for each item, then the parents.

**Definition**

```csharp
public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to delete |
| `byWhere` | optional predicate applied to each item |

**Returns**

`Task<Response>`

###### BulkCreate(items) {#csharp-bulkcrudrepository-bulkcreate-items}

Alias of [Create](#csharp-bulkcrudrepository-create-items).

**Definition**

```csharp
public Task<Response> BulkCreate(IReadOnlyList<T> items)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to insert |

**Returns**

`Task<Response>`

###### BulkUpdate(items, byWhere) {#csharp-bulkcrudrepository-bulkupdate-items-bywhere}

Alias of [Update](#csharp-bulkcrudrepository-update-items-bywhere).

**Definition**

```csharp
public Task<Response> BulkUpdate(IReadOnlyList<T> items, IWhere? byWhere = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to update |
| `byWhere` | optional predicate applied to each item |

**Returns**

`Task<Response>`

###### BulkDelete(items, byWhere) {#csharp-bulkcrudrepository-bulkdelete-items-bywhere}

Alias of [Delete](#csharp-bulkcrudrepository-delete-items-bywhere).

**Definition**

```csharp
public Task<Response> BulkDelete(IReadOnlyList<T> items, IWhere? byWhere = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to delete |
| `byWhere` | optional predicate applied to each item |

**Returns**

`Task<Response>`

##### Rust

##### BulkCRUDRepository {#rust-bulkcrudrepository}

```rust
pub struct BulkCRUDRepository
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(inner)](#rust-bulkcrudrepository-new-inner) | Wraps an inner [CRUDRepository](#crud-repository). |

**Methods**

| Name | Description |
| --- | --- |
| [add_enriched(enrichment, via, strategy)](#rust-bulkcrudrepository-add_enriched-enrichment-via-strategy) | Forwards to [CRUDRepository.add_enriched](#rust-add_enrichedenrichment-via-strategy). A CRUD `via` is kept so `many` children can be written. |
| [create_bulk(items)](#rust-bulkcrudrepository-create_bulk-items) | Creates each item, including nested `many` children. |
| [update_bulk(items, by_where)](#rust-bulkcrudrepository-update_bulk-items-by_where) | Updates each item, then replace-syncs nested `many` lists. |
| [delete_bulk(items, by_where)](#rust-bulkcrudrepository-delete_bulk-items-by_where) | Deletes nested `many` rows for each item, then the parents. |

###### new(inner) {#rust-bulkcrudrepository-new-inner}

Wraps an inner [CRUDRepository](#crud-repository).

**Definition**

```rust
pub fn new(inner: Arc<dyn CRUDRepository>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [CRUDRepository](#crud-repository) |

**Returns**

[BulkCRUDRepository](#rust-bulkcrudrepository)

###### add_enriched(enrichment, via, strategy) {#rust-bulkcrudrepository-add_enriched-enrichment-via-strategy}

Forwards to [CRUDRepository.add_enriched](#rust-add_enrichedenrichment-via-strategy). A CRUD `via` is kept so `many` children can be written.

**Definition**

```rust
pub fn add_enriched(&mut self, enrichment: Enrichment, via: impl EnrichmentVia, strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>) -> &mut Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](#enrichment) |
| `via` | related repository. A CRUD repository is kept so `many` children can be written |
| `strategy` | optional read strategy for this edge. Omitted uses the default strategy |

**Returns**

`&mut Self`

###### create_bulk(items) {#rust-bulkcrudrepository-create_bulk-items}

Creates each item, including nested `many` children.

**Definition**

```rust
pub async fn create_bulk(&self, items: &mut [Row]) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to insert |

**Returns**

`Result<Response>`

###### update_bulk(items, by_where) {#rust-bulkcrudrepository-update_bulk-items-by_where}

Updates each item, then replace-syncs nested `many` lists.

**Definition**

```rust
pub async fn update_bulk(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to update |
| `by_where` | optional predicate applied to each item |

**Returns**

`Result<Response>`

###### delete_bulk(items, by_where) {#rust-bulkcrudrepository-delete_bulk-items-by_where}

Deletes nested `many` rows for each item, then the parents.

**Definition**

```rust
pub async fn delete_bulk(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to delete |
| `by_where` | optional predicate applied to each item |

**Returns**

`Result<Response>`

### Remarks

> **TypeScript**
>
> Bulk writes use `createBulk` only.

> **C#**
>
> List overloads are `Create(IReadOnlyList<T>)`, with `Bulk*` aliases.

> **Rust**
>
> Bulk writes use `create_bulk` only.

Each item still runs the same deep persist as [CRUDRepository](#crud-repository). The list write goes through [IBulkWriteStrategy](#ibulkwritestrategy). The default is [SingleCommandIBulkWriteStrategy](#singlecommandibulkwritestrategy). When the write strategy's `useTransaction` is on, a list create, update, or delete is one transaction.

### Examples

#### Create parents and children

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/BulkCRUDRepositoryTests.cs#L23-L30)

```csharp
var connection = await Setup();
var parentsRepo = new BulkCRUDRepository<Parent>(connection);
var childrenRepo = new BulkCRUDRepository<Child>(connection);
var response = await Repo(parentsRepo, childrenRepo).Create(
[
    new Parent { children = [new Child { label = "a" }] },
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1897-L1911)

```rust
let command = Arc::new(MockCommand::new());
command.reader_rows.lock().unwrap().push(record(&[
    ("id", Value::Int(1)),
    ("username", Value::from("bobby")),
    ("email", Value::from("bob@asdf.com")),
    ("contact_name1", Value::from("bob")),
    ("contact_name2", Value::from("roberta")),
]));
let connection = MockConnection::with_command_provider(command.clone(), "memory");
let mut options = CRUDRepositoryOptions::default();
options.entity = "users".into();
options.row_converter = Some(RowTypeConverter::of::<UserRow>());
options.rows_mapper = Box::new(user_contacts_mapper());
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/BulkCRUDRepository.test.ts#L33-L39)

```typescript
const connection = await setup();
const parentsRepo = new BulkCRUDRepository(Parent, connection);
const childrenRepo = new BulkCRUDRepository(Child, connection);
const response = await relate(parentsRepo, childrenRepo).createBulk([
  Object.assign(new Parent(), { children: [Object.assign(new Child(), { label: "a" })] }),
  Object.assign(new Parent(), {
```

## Repository factories

Register a repository, then create by that type. Registering CRUD also registers readonly. Registering readonly-only makes CRUD `create` throw.

### Classes

##### TypeScript

##### IRepositoryFactory {#typescript-irepositoryfactory}

```typescript
interface IRepositoryFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [register(repository)](#typescript-irepositoryfactory-register-repository) | Registers a readonly repository by [type](#typescript-irepository-type). |
| [create(type)](#typescript-irepositoryfactory-create-type) | Creates a readonly repository. |

###### register(repository) {#typescript-irepositoryfactory-register-repository}

Registers a readonly repository by [type](#typescript-irepository-type).

**Definition**

```typescript
register(repository: IRepository): void
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [IRepository](#typescript-irepository) to look up by `type` |

###### create(type) {#typescript-irepositoryfactory-create-type}

Creates a readonly repository.

**Definition**

```typescript
create<T>(type: Constructable<T>): IReadonlyRepository<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `type` | item constructor |

**Returns**

[IReadonlyRepository](#readonly-repository)`<T>`

##### ICRUDRepositoryFactory {#typescript-icrudrepositoryfactory}

```typescript
interface ICRUDRepositoryFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [register(repository)](#typescript-icrudrepositoryfactory-register-repository) | Registers a CRUD repository by [type](#typescript-irepository-type). |
| [create(type)](#typescript-icrudrepositoryfactory-create-type) | Creates a CRUD repository. |

###### register(repository) {#typescript-icrudrepositoryfactory-register-repository}

Registers a CRUD repository by [type](#typescript-irepository-type).

**Definition**

```typescript
register(repository: IRepository): void
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [IRepository](#typescript-irepository) to look up by `type` |

###### create(type) {#typescript-icrudrepositoryfactory-create-type}

Creates a CRUD repository.

**Definition**

```typescript
create<T>(type: Constructable<T>): ICRUDRepository<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `type` | item constructor |

**Returns**

[ICRUDRepository](#crud-repository)`<T>`

##### RepositoryFactory {#typescript-repositoryfactory}

```typescript
class RepositoryFactory implements ICRUDRepositoryFactory
```

**Properties**

| Name | Description |
| --- | --- |
| asReadonly | Readonly factory view. `register` / `create` call [IRepositoryFactory](#typescript-irepositoryfactory). |

**Methods**

| Name | Description |
| --- | --- |
| [register(repository)](#typescript-repositoryfactory-register-repository) | Registers CRUD and readonly for the same repository. |
| [create(type)](#typescript-repositoryfactory-create-type) | Creates a CRUD repository. |

###### register(repository) {#typescript-repositoryfactory-register-repository}

Registers CRUD and readonly for the same repository.

**Definition**

```typescript
register(repository: IRepository): void
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [IRepository](#typescript-irepository) to look up by `type` |

###### create(type) {#typescript-repositoryfactory-create-type}

Creates a CRUD repository.

**Definition**

```typescript
create<T>(type: Constructable<T>): ICRUDRepository<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `type` | item constructor |

**Returns**

[ICRUDRepository](#crud-repository)`<T>`

##### RepositoryNotRegisteredError {#typescript-repositorynotregisterederror}

```typescript
class RepositoryNotRegisteredError extends Error
```

**Constructor**

| Name | Description |
| --- | --- |
| [RepositoryNotRegisteredError(entityTypeName)](#typescript-repositorynotregisterederror-repositorynotregisterederror-entitytypename) | Thrown when [create](#typescript-repositoryfactory-create-type) cannot find a repository. |

**Properties**

| Name | Description |
| --- | --- |
| entityTypeName | Missing entity type name. |

###### RepositoryNotRegisteredError(entityTypeName) {#typescript-repositorynotregisterederror-repositorynotregisterederror-entitytypename}

Thrown when [create](#typescript-repositoryfactory-create-type) cannot find a repository.

**Definition**

```typescript
constructor(entityTypeName: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `entityTypeName` | missing entity type name |

##### C#

##### IRepositoryFactory {#csharp-irepositoryfactory}

```csharp
public interface IRepositoryFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [Register(repository)](#csharp-irepositoryfactory-register-repository) | Registers a readonly repository by [Type](#csharp-irepository-type). |
| [Create()](#csharp-irepositoryfactory-create) | Creates a readonly repository by `T`. |

###### Register(repository) {#csharp-irepositoryfactory-register-repository}

Registers a readonly repository by [Type](#csharp-irepository-type).

**Definition**

```csharp
void Register(IRepository repository)
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [IRepository](#csharp-irepository) to look up by `Type` |

###### Create() {#csharp-irepositoryfactory-create}

Creates a readonly repository by `T`.

**Definition**

```csharp
IReadonlyRepository<T> Create<T>()
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional open [IConnection](#connection) |

**Returns**

[IReadonlyRepository](#readonly-repository)`<T>`

##### ICRUDRepositoryFactory {#csharp-icrudrepositoryfactory}

```csharp
public interface ICRUDRepositoryFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [Register(repository)](#csharp-icrudrepositoryfactory-register-repository) | Registers a CRUD repository by [Type](#csharp-irepository-type). |
| [Create()](#csharp-icrudrepositoryfactory-create) | Creates a CRUD repository by `T`. |

###### Register(repository) {#csharp-icrudrepositoryfactory-register-repository}

Registers a CRUD repository by [Type](#csharp-irepository-type).

**Definition**

```csharp
void Register(IRepository repository)
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [IRepository](#csharp-irepository) to look up by `Type` |

###### Create() {#csharp-icrudrepositoryfactory-create}

Creates a CRUD repository by `T`.

**Definition**

```csharp
ICRUDRepository<T> Create<T>()
```

**Returns**

[ICRUDRepository](#crud-repository)`<T>`

##### RepositoryFactory {#csharp-repositoryfactory}

```csharp
public sealed class RepositoryFactory : IRepositoryFactory, ICRUDRepositoryFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [Register(repository)](#csharp-repositoryfactory-register-repository) | Registers CRUD and readonly for the same repository. |
| [Create()](#csharp-repositoryfactory-create) | Creates a CRUD repository by `T`. |

###### Register(repository) {#csharp-repositoryfactory-register-repository}

Registers CRUD and readonly for the same repository.

**Definition**

```csharp
public void Register(IRepository repository)
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [IRepository](#csharp-irepository) to look up by `Type` |

###### Create() {#csharp-repositoryfactory-create}

Creates a CRUD repository by `T`.

**Definition**

```csharp
public ICRUDRepository<T> Create<T>()
```

**Returns**

[ICRUDRepository](#crud-repository)`<T>`

##### RepositoryNotRegisteredException {#csharp-repositorynotregisteredexception}

```csharp
public sealed class RepositoryNotRegisteredException : DatasourceException
```

**Constructor**

| Name | Description |
| --- | --- |
| [RepositoryNotRegisteredException(entityTypeName)](#csharp-repositorynotregisteredexception-repositorynotregisteredexception-entitytypename) | Thrown when [Create](#csharp-repositoryfactory-create) cannot find a repository. |

**Properties**

| Name | Description |
| --- | --- |
| EntityTypeName | Missing entity type name. |

###### RepositoryNotRegisteredException(entityTypeName) {#csharp-repositorynotregisteredexception-repositorynotregisteredexception-entitytypename}

Thrown when [Create](#csharp-repositoryfactory-create) cannot find a repository.

**Definition**

```csharp
public RepositoryNotRegisteredException(string entityTypeName)
```

**Parameters**

| Name | Description |
| --- | --- |
| `entityTypeName` | missing entity type name |

##### Rust

##### ReadonlyRepositoryFactory {#rust-readonlyrepositoryfactory}

```rust
pub trait ReadonlyRepositoryFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [register(repository)](#rust-readonlyrepositoryfactory-register-repository) | Registers a readonly repository by [entity_type](#rust-irepository-entity_type). |
| [create()](#rust-readonlyrepositoryfactory-create) | Creates a readonly repository by `T`. |

###### register(repository) {#rust-readonlyrepositoryfactory-register-repository}

Registers a readonly repository by [entity_type](#rust-irepository-entity_type).

**Definition**

```rust
fn register(&self, repository: Arc<dyn IReadonlyRepository>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [IReadonlyRepository](#readonly-repository) to look up by `entity_type` |

###### create() {#rust-readonlyrepositoryfactory-create}

Creates a readonly repository by `T`.

**Definition**

```rust
fn create<T: 'static>(&self) -> Result<Arc<dyn IReadonlyRepository>>
```

**Returns**

`Result<Arc<dyn `[IReadonlyRepository](#readonly-repository)`>>`

##### CRUDRepositoryFactory {#rust-crudrepositoryfactory}

```rust
pub trait CRUDRepositoryFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [register(repository)](#rust-crudrepositoryfactory-register-repository) | Registers a CRUD repository by [entity_type](#rust-irepository-entity_type). |
| [create()](#rust-crudrepositoryfactory-create) | Creates a CRUD repository by `T`. |

###### register(repository) {#rust-crudrepositoryfactory-register-repository}

Registers a CRUD repository by [entity_type](#rust-irepository-entity_type).

**Definition**

```rust
fn register(&self, repository: Arc<dyn ICRUDRepository>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [ICRUDRepository](#crud-repository) to look up by `entity_type` |

###### create() {#rust-crudrepositoryfactory-create}

Creates a CRUD repository by `T`.

**Definition**

```rust
fn create<T: 'static>(&self) -> Result<Arc<dyn ICRUDRepository>>
```

**Returns**

`Result<Arc<dyn `[ICRUDRepository](#crud-repository)`>>`

##### RepositoryFactory {#rust-repositoryfactory}

```rust
pub struct RepositoryFactory
```

**Constructor**

| Name | Description |
| --- | --- |
| [new()](#rust-repositoryfactory-new) | Empty registry. |

**Methods**

| Name | Description |
| --- | --- |
| [register(repository)](#rust-repositoryfactory-register-repository) | Registers CRUD and readonly for the same repository. |
| [create()](#rust-repositoryfactory-create) | Creates a CRUD repository by `T`. |

###### new() {#rust-repositoryfactory-new}

Empty registry.

**Definition**

```rust
pub fn new() -> Self
```

**Returns**

[RepositoryFactory](#rust-repositoryfactory)

###### register(repository) {#rust-repositoryfactory-register-repository}

Registers CRUD and readonly for the same repository.

**Definition**

```rust
pub fn register(&self, repository: Arc<dyn ICRUDRepository>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `repository` | [ICRUDRepository](#crud-repository) to look up by `entity_type` |

###### create() {#rust-repositoryfactory-create}

Creates a CRUD repository by `T`.

**Definition**

```rust
pub fn create<T: 'static>(&self) -> Result<Arc<dyn ICRUDRepository>>
```

**Returns**

`Result<Arc<dyn `[ICRUDRepository](#crud-repository)`>>`

### Remarks

> **TypeScript**
>
> A missing repository throws [RepositoryNotRegisteredError](#repositorynotregisterederror).

> **C#**
>
> A missing repository throws [RepositoryNotRegisteredException](#repositorynotregisteredexception).

> **Rust**
>
> A missing repository throws `DatasourceError`.

See [Datasource](#datasource) for `CreateRepository` / `createRepository` / `create_repository`.

### Examples

#### Register and create

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/RepositoryFactoryTests.cs#L12-L19)

```csharp
{
    var factory = new RepositoryFactory();
    var repo = new StubRepository<Item>("memory", new StubConnection("memory"));
    factory.Register(repo);
    var created = factory.Create<Item>();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/repository_factory.rs#L112-L116)

```rust
let factory = RepositoryFactory::new();
factory.register(Arc::new(StubRepository::typed::<Item>("memory")));
let created = factory.create::<Item>().unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/RepositoryFactory.test.ts#L18-L24)

```typescript
const factory = new RepositoryFactory();
const repo = new MockRepository<Item>(new MockConnection(), Item);
factory.register(repo);
const created = factory.create(Item);
```

### See also

- [Repositories](#repositories)
- [Readonly repository](#readonly-repository)
- [CRUD repository](#crud-repository)

## Mappers

Bidirectional `Map` / `map` / `map` and `ReverseMap` / `reverseMap` / `reverse_map`. Column names pass through unless a mapper rewrites them. Repository read is convert, then map.

### Classes

##### TypeScript

##### IMapper {#typescript-imapper}

```typescript
interface IMapper<TFrom, TTo>
```

**Methods**

| Name | Description |
| --- | --- |
| [map(value)](#typescript-imapper-map-value) | Maps `TFrom` to `TTo`. |
| [reverseMap(value)](#typescript-imapper-reversemap-value) | Maps `TTo` back to `TFrom`. |

###### map(value) {#typescript-imapper-map-value}

Maps `TFrom` to `TTo`.

**Definition**

```typescript
map(value: TFrom): TTo
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | source value |

**Returns**

`TTo`

###### reverseMap(value) {#typescript-imapper-reversemap-value}

Maps `TTo` back to `TFrom`.

**Definition**

```typescript
reverseMap(value: TTo): TFrom
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | mapped value |

**Returns**

`TFrom`

##### C#

##### IMapper {#csharp-imapper}

```csharp
public interface IMapper<TFrom, TTo>
```

**Methods**

| Name | Description |
| --- | --- |
| [Map(value)](#csharp-imapper-map-value) | Maps `TFrom` to `TTo`. |
| [ReverseMap(value)](#csharp-imapper-reversemap-value) | Maps `TTo` back to `TFrom`. |

###### Map(value) {#csharp-imapper-map-value}

Maps `TFrom` to `TTo`.

**Definition**

```csharp
TTo Map(TFrom value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | source value |

**Returns**

`TTo`

###### ReverseMap(value) {#csharp-imapper-reversemap-value}

Maps `TTo` back to `TFrom`.

**Definition**

```csharp
TFrom ReverseMap(TTo value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | mapped value |

**Returns**

`TFrom`

##### Rust

##### Mapper {#rust-mapper}

```rust
pub trait Mapper<TFrom, TTo>
```

**Methods**

| Name | Description |
| --- | --- |
| [map(value)](#rust-mapper-map-value) | Maps `TFrom` to `TTo`. |
| [reverse_map(value)](#rust-mapper-reverse_map-value) | Maps `TTo` back to `TFrom`. |

###### map(value) {#rust-mapper-map-value}

Maps `TFrom` to `TTo`.

**Definition**

```rust
fn map(&self, value: TFrom) -> TTo
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | source value |

**Returns**

`TTo`

###### reverse_map(value) {#rust-mapper-reverse_map-value}

Maps `TTo` back to `TFrom`.

**Definition**

```rust
fn reverse_map(&self, value: TTo) -> TFrom
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | mapped value |

**Returns**

`TFrom`

### Examples

#### Transform a contact

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/TransformMapperTests.cs#L41-L52)

```csharp
var contact = new Contact("Ada", "ada@x.dev");
var mapper = new TransformMapper<Contact, ContactRow>(
    map: item => new ContactRow(item.Name, item.Email),
    reverseMap: row => new Contact(row.ContactName, row.ContactEmail));
var mapped = mapper.Map(contact);
var reversed = mapper.ReverseMap(mapped);
```

**Before: contact**

```
{
  name: 'Ada',
  email: 'ada@x.dev'
}
```

**After: mapped contact**

```
{
  contactName: 'Ada',
  contactEmail: 'ada@x.dev'
}
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/mod.rs#L136-L162)

```rust
let contact = Contact {
    name: "Ada".into(),
    email: "ada@x.dev".into(),
};
let mapper = TransformMapper::new(
    |item: Contact| ContactRow {
        contact_name: item.name,
        contact_email: item.email,
    },
    |row: ContactRow| Contact {
        name: row.contact_name,
        email: row.contact_email,
    },
);
let mapped = Mapper::map(&mapper, contact.clone());
let reversed = Mapper::reverse_map(&mapper, mapped.clone());
```

**Before: contact**

```
{
  name: 'Ada',
  email: 'ada@x.dev'
}
```

**After: mapped contact**

```
{
  contactName: 'Ada',
  contactEmail: 'ada@x.dev'
}
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/transformMapper.test.ts#L55-L65)

```typescript
const contact: Contact = { name: "Ada", email: "ada@x.dev" };
const mapper = new TransformMapper<Contact, ContactRow>(
  (item) => ({ contactName: item.name, contactEmail: item.email }),
  (row) => ({ name: row.contactName, email: row.contactEmail }),
);
const mapped = mapper.map(contact);
const reversed = mapper.reverseMap(mapped);
```

**Before: contact**

```
{
  name: 'Ada',
  email: 'ada@x.dev'
}
```

**After: mapped contact**

```
{
  contactName: 'Ada',
  contactEmail: 'ada@x.dev'
}
```

#### Change casing

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/ChangeCasingMapperTests.cs#L17-L27)

```csharp
var mapper = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
var contact = mapper.Map("Contact");
var firstName = mapper.Map("FirstName");
var emailAddress = mapper.Map("EmailAddress");
```

**Map**

```
contact
first_name
email_address
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/change_casing_mapper.rs#L43-L52)

```rust
let mapper = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
let contact = mapper.map("Contact".into());
let first_name = mapper.map("FirstName".into());
let email_address = mapper.map("EmailAddress".into());
```

**Map**

```
contact
first_name
email_address
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/ChangeCasingMapper.test.ts#L13-L21)

```typescript
const mapper = new ChangeCasingMapper("pascalCase", "snakeCase");
const contact = mapper.map("Contact");
const firstName = mapper.map("FirstName");
const emailAddress = mapper.map("EmailAddress");
```

**Map**

```
contact
first_name
email_address
```

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/ChangeCasingMapperTests.cs#L29-L39)

```csharp
var mapper = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
var contact = mapper.ReverseMap("contact");
var firstName = mapper.ReverseMap("first_name");
var emailAddress = mapper.ReverseMap("email_address");
```

**Reverse map**

```
Contact
FirstName
EmailAddress
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/change_casing_mapper.rs#L54-L63)

```rust
let mapper = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
let contact = mapper.reverse_map("contact".into());
let first_name = mapper.reverse_map("first_name".into());
let email_address = mapper.reverse_map("email_address".into());
```

**Reverse map**

```
Contact
FirstName
EmailAddress
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/ChangeCasingMapper.test.ts#L23-L31)

```typescript
const mapper = new ChangeCasingMapper("pascalCase", "snakeCase");
const contact = mapper.reverseMap("contact");
const firstName = mapper.reverseMap("first_name");
const emailAddress = mapper.reverseMap("email_address");
```

**Reverse map**

```
Contact
FirstName
EmailAddress
```

#### Lowercase plural

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/PluralizerMapperTests.cs#L35-L48)

```csharp
var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
var pluralizer = new PluralizerMapper();
var contact = pluralizer.Map(casing.Map("Contact"));
var address = pluralizer.Map(casing.Map("Address"));
var contactName = casing.ReverseMap(pluralizer.ReverseMap(contact));
var addressName = casing.ReverseMap(pluralizer.ReverseMap(address));
```

**Map**

```
contacts
addresses
```

**Reverse map**

```
Contact
Address
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/pluralizer_mapper.rs#L63-L75)

```rust
let casing = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
let pluralizer = PluralizerMapper::new();
let contact = pluralizer.map(casing.map("Contact".into()));
let address = pluralizer.map(casing.map("Address".into()));
let contact_name = casing.reverse_map(pluralizer.reverse_map(contact.clone()));
let address_name = casing.reverse_map(pluralizer.reverse_map(address.clone()));
```

**Map**

```
contacts
addresses
```

**Reverse map**

```
Contact
Address
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/PluralizerMapper.test.ts#L28-L39)

```typescript
const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
const pluralizer = new PluralizerMapper();
const contact = pluralizer.map(casing.map("Contact"));
const address = pluralizer.map(casing.map("Address"));
const contactName = casing.reverseMap(pluralizer.reverseMap(contact));
const addressName = casing.reverseMap(pluralizer.reverseMap(address));
```

**Map**

```
contacts
addresses
```

**Reverse map**

```
Contact
Address
```

#### Flat to nested

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/FlatToNestedMapperTests.cs#L101-L164)

```csharp
var mapper = new FlatToNestedMapper(new TypeMap
{
    Key = "id",
    Fields = new Dictionary<string, TypeMapField>
    {
        ["id"] = "id",
        ["first_name"] = "first_name",
        ["last_name"] = "last_name",
        ["email"] = "email",
        ["contact_source_name"] = "contact_source_name",
        ["contact_type_name"] = "contact_type_name",
        ["addresses"] = new TypeMapGroup
        {
            Key = "address_id",
            Fields = new Dictionary<string, TypeMapField>
            {
                ["id"] = "address_id",
                ["line1"] = "address_line1",
                ["city"] = "address_city",
                ["address_type_name"] = "address_type_name",
            },
        },
        ["phones"] = new TypeMapGroup
        {
            Key = "phone_id",
            Fields = new Dictionary<string, TypeMapField>
            {
                ["id"] = "phone_id",
                ["number"] = "phone_number",
                ["phone_type_name"] = "phone_type_name",
            },
        },
    },
});
var adaHome = new Row(
[
    new("id", 7),
    new("first_name", "Ada"),
    new("last_name", "Lovelace"),
    new("email", "ada@ex.com"),
    new("contact_source_name", "Web"),
    new("contact_type_name", "Person"),
    new("address_id", 101),
    new("address_line1", "10 Downing St"),
    new("address_city", "London"),
    new("address_type_name", "Home"),
    new("phone_id", 201),
    new("phone_number", "+44"),
    new("phone_type_name", "Mobile"),
]);
var adaWork = new Row(adaHome)
{
    ["address_id"] = 102,
    ["address_line1"] = "1 Parliament Sq"
};
var nested = mapper.Map([adaHome, adaWork]);
var flat = mapper.ReverseMap(nested);
```

**After: nested contact**

```
{
  id: 7,
  first_name: "Ada",
  last_name: "Lovelace",
  email: "ada@ex.com",
  contact_source_name: "Web",
  contact_type_name: "Person",
  addresses: [
    { id: 101, line1: "10 Downing St", city: "London", address_type_name: "Home" },
    { id: 102, line1: "1 Parliament Sq", city: "London", address_type_name: "Home" }
  ],
  phones: [{ id: 201, number: "+44", phone_type_name: "Mobile" }]
}
```

**After: reverseMap join rows**

```
[
  {
    id: 7,
    first_name: "Ada",
    last_name: "Lovelace",
    email: "ada@ex.com",
    contact_source_name: "Web",
    contact_type_name: "Person",
    address_id: 101,
    address_line1: "10 Downing St",
    address_city: "London",
    address_type_name: "Home",
    phone_id: 201,
    phone_number: "+44",
    phone_type_name: "Mobile"
  },
  {
    id: 7,
    first_name: "Ada",
    last_name: "Lovelace",
    email: "ada@ex.com",
    contact_source_name: "Web",
    contact_type_name: "Person",
    address_id: 102,
    address_line1: "1 Parliament Sq",
    address_city: "London",
    address_type_name: "Home",
    phone_id: 201,
    phone_number: "+44",
    phone_type_name: "Mobile"
  }
]
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/flat_to_nested_mapper.rs#L385-L456)

```rust
let mapper = FlatToNestedMapper::new(TypeMap::new(
    "id",
    vec![
        ("id".into(), TypeMapField::column("id")),
        ("first_name".into(), TypeMapField::column("first_name")),
        ("last_name".into(), TypeMapField::column("last_name")),
        ("email".into(), TypeMapField::column("email")),
        (
            "contact_source_name".into(),
            TypeMapField::column("contact_source_name"),
        ),
        (
            "contact_type_name".into(),
            TypeMapField::column("contact_type_name"),
        ),
        (
            "addresses".into(),
            TypeMapField::Group(TypeMapGroup::new(
                "address_id",
                vec![
                    ("id".into(), TypeMapField::column("address_id")),
                    ("line1".into(), TypeMapField::column("address_line1")),
                    ("city".into(), TypeMapField::column("address_city")),
                    (
                        "address_type_name".into(),
                        TypeMapField::column("address_type_name"),
                    ),
                ],
            )),
        ),
        (
            "phones".into(),
            TypeMapField::Group(TypeMapGroup::new(
                "phone_id",
                vec![
                    ("id".into(), TypeMapField::column("phone_id")),
                    ("number".into(), TypeMapField::column("phone_number")),
                    (
                        "phone_type_name".into(),
                        TypeMapField::column("phone_type_name"),
                    ),
                ],
            )),
        ),
    ],
));
let ada_home = cell(&[
    ("id", Value::Int(7)),
    ("first_name", Value::from("Ada")),
    ("last_name", Value::from("Lovelace")),
    ("email", Value::from("ada@ex.com")),
    ("contact_source_name", Value::from("Web")),
    ("contact_type_name", Value::from("Person")),
    ("address_id", Value::Int(101)),
    ("address_line1", Value::from("10 Downing St")),
    ("address_city", Value::from("London")),
    ("address_type_name", Value::from("Home")),
    ("phone_id", Value::Int(201)),
    ("phone_number", Value::from("+44")),
    ("phone_type_name", Value::from("Mobile")),
]);
let mut ada_work = ada_home.clone();
ada_work.insert("address_id".into(), Value::Int(102));
ada_work.insert("address_line1".into(), Value::from("1 Parliament Sq"));
let nested = Mapper::map(&mapper, vec![ada_home.clone(), ada_work.clone()]);
let flat = Mapper::reverse_map(&mapper, nested.clone());
```

**After: nested contact**

```
{
  id: 7,
  first_name: "Ada",
  last_name: "Lovelace",
  email: "ada@ex.com",
  contact_source_name: "Web",
  contact_type_name: "Person",
  addresses: [
    { id: 101, line1: "10 Downing St", city: "London", address_type_name: "Home" },
    { id: 102, line1: "1 Parliament Sq", city: "London", address_type_name: "Home" }
  ],
  phones: [{ id: 201, number: "+44", phone_type_name: "Mobile" }]
}
```

**After: reverseMap join rows**

```
[
  {
    id: 7,
    first_name: "Ada",
    last_name: "Lovelace",
    email: "ada@ex.com",
    contact_source_name: "Web",
    contact_type_name: "Person",
    address_id: 101,
    address_line1: "10 Downing St",
    address_city: "London",
    address_type_name: "Home",
    phone_id: 201,
    phone_number: "+44",
    phone_type_name: "Mobile"
  },
  {
    id: 7,
    first_name: "Ada",
    last_name: "Lovelace",
    email: "ada@ex.com",
    contact_source_name: "Web",
    contact_type_name: "Person",
    address_id: 102,
    address_line1: "1 Parliament Sq",
    address_city: "London",
    address_type_name: "Home",
    phone_id: 201,
    phone_number: "+44",
    phone_type_name: "Mobile"
  }
]
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/FlatToNestedMapper.test.ts#L90-L155)

```typescript
const mapper = new FlatToNestedMapper<Contact>({
  key: "id",
  fields: {
    id: "id",
    first_name: "first_name",
    last_name: "last_name",
    email: "email",
    contact_source_name: "contact_source_name",
    contact_type_name: "contact_type_name",
    addresses: {
      key: "address_id",
      fields: {
        id: "address_id",
        line1: "address_line1",
        city: "address_city",
        address_type_name: "address_type_name",
      },
    },
    phones: {
      key: "phone_id",
      fields: {
        id: "phone_id",
        number: "phone_number",
        phone_type_name: "phone_type_name",
      },
    },
  },
});
const adaHome: Cells = {
  id: 7,
  first_name: "Ada",
  last_name: "Lovelace",
  email: "ada@ex.com",
  contact_source_name: "Web",
  contact_type_name: "Person",
  address_id: 101,
  address_line1: "10 Downing St",
  address_city: "London",
  address_type_name: "Home",
  phone_id: 201,
  phone_number: "+44",
  phone_type_name: "Mobile",
};
const adaWork = {
  ...adaHome,
  address_id: 102,
  address_line1: "1 Parliament Sq",
};
const nested = mapper.map([adaHome, adaWork]);
const flat = mapper.reverseMap(nested);
```

**After: nested contact**

```
{
  id: 7,
  first_name: "Ada",
  last_name: "Lovelace",
  email: "ada@ex.com",
  contact_source_name: "Web",
  contact_type_name: "Person",
  addresses: [
    { id: 101, line1: "10 Downing St", city: "London", address_type_name: "Home" },
    { id: 102, line1: "1 Parliament Sq", city: "London", address_type_name: "Home" }
  ],
  phones: [{ id: 201, number: "+44", phone_type_name: "Mobile" }]
}
```

**After: reverseMap join rows**

```
[
  {
    id: 7,
    first_name: "Ada",
    last_name: "Lovelace",
    email: "ada@ex.com",
    contact_source_name: "Web",
    contact_type_name: "Person",
    address_id: 101,
    address_line1: "10 Downing St",
    address_city: "London",
    address_type_name: "Home",
    phone_id: 201,
    phone_number: "+44",
    phone_type_name: "Mobile"
  },
  {
    id: 7,
    first_name: "Ada",
    last_name: "Lovelace",
    email: "ada@ex.com",
    contact_source_name: "Web",
    contact_type_name: "Person",
    address_id: 102,
    address_line1: "1 Parliament Sq",
    address_city: "London",
    address_type_name: "Home",
    phone_id: 201,
    phone_number: "+44",
    phone_type_name: "Mobile"
  }
]
```

### See also

- [IdentityMapper](#identity-mapper)
- [DictionaryMapper](#dictionary-mapper)
- [PluralizerMapper](#pluralizer-mapper)
- [ChangeCasingMapper](#change-casing-mapper)
- [TransformMapper](#transform-mapper)
- [FlatToNestedMapper](#flat-to-nested-mapper)
- [ReadonlyRepositoryOptions](#readonlyrepositoryoptions)

## IdentityMapper

Mapper that returns the value unchanged.

### Classes

##### TypeScript

##### IdentityMapper {#typescript-identitymapper}

```typescript
class IdentityMapper<T> implements IMapper<T, T>
```

**Methods**

| Name | Description |
| --- | --- |
| [map(value)](#typescript-identitymapper-map-value) | Returns `value`. |
| [reverseMap(value)](#typescript-identitymapper-reversemap-value) | Returns `value`. |

###### map(value) {#typescript-identitymapper-map-value}

Returns `value`.

**Definition**

```typescript
map(value: T): T
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | source value |

**Returns**

`T`

###### reverseMap(value) {#typescript-identitymapper-reversemap-value}

Returns `value`.

**Definition**

```typescript
reverseMap(value: T): T
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | mapped value |

**Returns**

`T`

##### C#

##### IdentityMapper {#csharp-identitymapper}

```csharp
public class IdentityMapper<T> : IMapper<T, T>
```

**Methods**

| Name | Description |
| --- | --- |
| [Map(value)](#csharp-identitymapper-map-value) | Returns `value`. |
| [ReverseMap(value)](#csharp-identitymapper-reversemap-value) | Returns `value`. |

###### Map(value) {#csharp-identitymapper-map-value}

Returns `value`.

**Definition**

```csharp
T Map(T value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | source value |

**Returns**

`T`

###### ReverseMap(value) {#csharp-identitymapper-reversemap-value}

Returns `value`.

**Definition**

```csharp
T ReverseMap(T value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | mapped value |

**Returns**

`T`

##### Rust

##### IdentityMapper {#rust-identitymapper}

```rust
pub struct IdentityMapper<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [new()](#rust-identitymapper-new) | Builds an identity mapper. |

**Methods**

| Name | Description |
| --- | --- |
| [map(value)](#rust-identitymapper-map-value) | Returns `value`. |
| [reverse_map(value)](#rust-identitymapper-reverse_map-value) | Returns `value`. |

###### new() {#rust-identitymapper-new}

Builds an identity mapper.

**Definition**

```rust
pub fn new() -> Self
```

**Returns**

[IdentityMapper](#rust-identitymapper)

###### map(value) {#rust-identitymapper-map-value}

Returns `value`.

**Definition**

```rust
fn map(&self, value: T) -> T
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | source value |

**Returns**

`T`

###### reverse_map(value) {#rust-identitymapper-reverse_map-value}

Returns `value`.

**Definition**

```rust
fn reverse_map(&self, value: T) -> T
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | mapped value |

**Returns**

`T`

### Examples

#### Map and reverse map

`Map` / `map` / `map` and `ReverseMap` / `reverseMap` / `reverse_map` return the value unchanged.

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/IdentityMapperTests.cs#L10-L14)

```csharp
var mapper = new IdentityMapper<string>();
var mapped = mapper.Map("contact");
var reversed = mapper.ReverseMap("contact");
```

**Map**

```
contact
```

**Reverse map**

```
contact
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/identity_mapper.rs#L39-L43)

```rust
let mapper = IdentityMapper::<String>::new();
let mapped = mapper.map("contact".into());
let reversed = mapper.reverse_map("contact".into());
```

**Map**

```
contact
```

**Reverse map**

```
contact
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/IdentityMapper.test.ts#L6-L10)

```typescript
const mapper = new IdentityMapper<string>();
const mapped = mapper.map("contact");
const reversed = mapper.reverseMap("contact");
```

**Map**

```
contact
```

**Reverse map**

```
contact
```

## DictionaryMapper

Maps keys to values with a dictionary. `ReverseMap` / `reverseMap` / `reverse_map` maps values back to keys.

### Classes

##### TypeScript

##### DictionaryMapper {#typescript-dictionarymapper}

```typescript
class DictionaryMapper<T extends string | number> implements IMapper<T, T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [DictionaryMapper(map)](#typescript-dictionarymapper-dictionarymapper-map) | Builds the forward map and its reverse. |

**Methods**

| Name | Description |
| --- | --- |
| [map(value)](#typescript-dictionarymapper-map-value) | Returns the stored name for a known key, or `value`. |
| [reverseMap(value)](#typescript-dictionarymapper-reversemap-value) | Returns the source key for a stored name, or `value`. |

###### DictionaryMapper(map) {#typescript-dictionarymapper-dictionarymapper-map}

Builds the forward map and its reverse.

**Definition**

```typescript
constructor(map: readonly (readonly [T, T])[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `map` | pairs of source key and stored name |

**Returns**

[DictionaryMapper](#typescript-dictionarymapper)

###### map(value) {#typescript-dictionarymapper-map-value}

Returns the stored name for a known key, or `value`.

**Definition**

```typescript
map(value: T): T
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | source key |

**Returns**

`T`

###### reverseMap(value) {#typescript-dictionarymapper-reversemap-value}

Returns the source key for a stored name, or `value`.

**Definition**

```typescript
reverseMap(value: T): T
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | stored name |

**Returns**

`T`

##### C#

##### DictionaryMapper {#csharp-dictionarymapper}

```csharp
public sealed class DictionaryMapper<T> : IMapper<T, T> where T : notnull
```

**Constructor**

| Name | Description |
| --- | --- |
| [DictionaryMapper(map)](#csharp-dictionarymapper-dictionarymapper-map) | Builds the forward map and its reverse. |

**Methods**

| Name | Description |
| --- | --- |
| [Map(value)](#csharp-dictionarymapper-map-value) | Returns the stored name for a known key, or `value`. |
| [ReverseMap(value)](#csharp-dictionarymapper-reversemap-value) | Returns the source key for a stored name, or `value`. |

###### DictionaryMapper(map) {#csharp-dictionarymapper-dictionarymapper-map}

Builds the forward map and its reverse.

**Definition**

```csharp
public DictionaryMapper(IReadOnlyDictionary<T, T> map)
```

**Parameters**

| Name | Description |
| --- | --- |
| `map` | pairs of source key and stored name |

###### Map(value) {#csharp-dictionarymapper-map-value}

Returns the stored name for a known key, or `value`.

**Definition**

```csharp
T Map(T value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | source key |

**Returns**

`T`

###### ReverseMap(value) {#csharp-dictionarymapper-reversemap-value}

Returns the source key for a stored name, or `value`.

**Definition**

```csharp
T ReverseMap(T value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | stored name |

**Returns**

`T`

##### Rust

##### DictionaryMapper {#rust-dictionarymapper}

```rust
pub struct DictionaryMapper<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(map)](#rust-dictionarymapper-new-map) | Builds the forward map and its reverse. `T: Clone + Eq + Hash`. |

**Methods**

| Name | Description |
| --- | --- |
| [map(value)](#rust-dictionarymapper-map-value) | Returns the stored name for a known key, or `value`. |
| [reverse_map(value)](#rust-dictionarymapper-reverse_map-value) | Returns the source key for a stored name, or `value`. |

###### new(map) {#rust-dictionarymapper-new-map}

Builds the forward map and its reverse. `T: Clone + Eq + Hash`.

**Definition**

```rust
pub fn new(map: impl IntoIterator<Item = (T, T)>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `map` | pairs of source key and stored name, in first-wins order |

**Returns**

[DictionaryMapper](#rust-dictionarymapper)

###### map(value) {#rust-dictionarymapper-map-value}

Returns the stored name for a known key, or `value`.

**Definition**

```rust
fn map(&self, value: T) -> T
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | source key |

**Returns**

`T`

###### reverse_map(value) {#rust-dictionarymapper-reverse_map-value}

Returns the source key for a stored name, or `value`.

**Definition**

```rust
fn reverse_map(&self, value: T) -> T
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | stored name |

**Returns**

`T`

### Remarks

Unknown keys pass through. When two keys share a value, the first one wins on reverse.

Use this for [ColumnMappings](#columnmappings) when property names and stored column names differ. `Map` rewrites the SELECT list, where identifiers, and write parameters. `ReverseMap` restores property names on each result key before the row converter.

### Examples

#### Map and reverse map

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/DictionaryMapperTests.cs#L14-L38)

```csharp
var mapper = new DictionaryMapper<string>(new Dictionary<string, string>
{
    ["FirstName"] = "given_name",
    ["LastName"] = "family_name",
    ["Email"] = "email_address",
});
var firstName = mapper.Map("FirstName");
var lastName = mapper.Map("LastName");
var email = mapper.Map("Email");
var phone = mapper.Map("Phone");
var fromFirst = mapper.ReverseMap("given_name");
var fromLast = mapper.ReverseMap("family_name");
var fromEmail = mapper.ReverseMap("email_address");
var fromPhone = mapper.ReverseMap("Phone");
```

**Map**

```
given_name
family_name
email_address
Phone
```

**Reverse map**

```
FirstName
LastName
Email
Phone
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/dictionary_mapper.rs#L51-L74)

```rust
let mapper = DictionaryMapper::new([
    ("FirstName".to_string(), "given_name".to_string()),
    ("LastName".to_string(), "family_name".to_string()),
    ("Email".to_string(), "email_address".to_string()),
]);
let first_name = mapper.map("FirstName".into());
let last_name = mapper.map("LastName".into());
let email = mapper.map("Email".into());
let phone = mapper.map("Phone".into());
let from_first = mapper.reverse_map("given_name".into());
let from_last = mapper.reverse_map("family_name".into());
let from_email = mapper.reverse_map("email_address".into());
let from_phone = mapper.reverse_map("Phone".into());
```

**Map**

```
given_name
family_name
email_address
Phone
```

**Reverse map**

```
FirstName
LastName
Email
Phone
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/DictionaryMapper.test.ts#L14-L36)

```typescript
const mapper = new DictionaryMapper([
  ["FirstName", "given_name"],
  ["LastName", "family_name"],
  ["Email", "email_address"],
]);
const firstName = mapper.map("FirstName");
const lastName = mapper.map("LastName");
const email = mapper.map("Email");
const phone = mapper.map("Phone");
const fromFirst = mapper.reverseMap("given_name");
const fromLast = mapper.reverseMap("family_name");
const fromEmail = mapper.reverseMap("email_address");
const fromPhone = mapper.reverseMap("Phone");
```

**Map**

```
given_name
family_name
email_address
Phone
```

**Reverse map**

```
FirstName
LastName
Email
Phone
```

#### First key wins on reverse

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/DictionaryMapperTests.cs#L40-L54)

```csharp
var mapper = new DictionaryMapper<string>(new Dictionary<string, string>
{
    ["Home"] = "address",
    ["Work"] = "address",
});
var home = mapper.Map("Home");
var work = mapper.Map("Work");
var address = mapper.ReverseMap("address");
```

**Map**

```
address
address
```

**Reverse map**

```
Home
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/dictionary_mapper.rs#L76-L88)

```rust
let mapper = DictionaryMapper::new([
    ("Home".to_string(), "address".to_string()),
    ("Work".to_string(), "address".to_string()),
]);
let home = mapper.map("Home".into());
let work = mapper.map("Work".into());
let address = mapper.reverse_map("address".into());
```

**Map**

```
address
address
```

**Reverse map**

```
Home
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/DictionaryMapper.test.ts#L38-L49)

```typescript
const mapper = new DictionaryMapper([
  ["Home", "address"],
  ["Work", "address"],
]);
const home = mapper.map("Home");
const work = mapper.map("Work");
const address = mapper.reverseMap("address");
```

**Map**

```
address
address
```

**Reverse map**

```
Home
```

## PluralizerMapper

Mapper that pluralizes a name and keeps its case. Reverse singularizes and keeps the first letter's case.

### Classes

##### TypeScript

##### PluralizerMapper {#typescript-pluralizermapper}

```typescript
class PluralizerMapper implements IMapper<string, string>
```

**Methods**

| Name | Description |
| --- | --- |
| [map(value)](#typescript-pluralizermapper-map-value) | Pluralizes `value` and keeps its case. `User` becomes `Users`. |
| [reverseMap(value)](#typescript-pluralizermapper-reversemap-value) | Singularizes `value` and keeps the first letter's case. `Users` becomes `User`. |

###### map(value) {#typescript-pluralizermapper-map-value}

Pluralizes `value` and keeps its case. `User` becomes `Users`.

**Definition**

```typescript
map(value: string): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | source name |

**Returns**

`string`

###### reverseMap(value) {#typescript-pluralizermapper-reversemap-value}

Singularizes `value` and keeps the first letter's case. `Users` becomes `User`.

**Definition**

```typescript
reverseMap(value: string): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | plural name |

**Returns**

`string`

##### C#

##### PluralizerMapper {#csharp-pluralizermapper}

```csharp
public sealed class PluralizerMapper : IMapper<string, string>
```

**Methods**

| Name | Description |
| --- | --- |
| [Map(value)](#csharp-pluralizermapper-map-value) | Pluralizes `value` and keeps its case. `User` becomes `Users`. |
| [ReverseMap(value)](#csharp-pluralizermapper-reversemap-value) | Singularizes `value` and keeps the first letter's case. `Users` becomes `User`. |

###### Map(value) {#csharp-pluralizermapper-map-value}

Pluralizes `value` and keeps its case. `User` becomes `Users`.

**Definition**

```csharp
string Map(string value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | source name |

**Returns**

`string`

###### ReverseMap(value) {#csharp-pluralizermapper-reversemap-value}

Singularizes `value` and keeps the first letter's case. `Users` becomes `User`.

**Definition**

```csharp
string ReverseMap(string value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | plural name |

**Returns**

`string`

##### Rust

##### PluralizerMapper {#rust-pluralizermapper}

```rust
pub struct PluralizerMapper
```

**Constructor**

| Name | Description |
| --- | --- |
| [new()](#rust-pluralizermapper-new) | Builds a pluralizer mapper. |

**Methods**

| Name | Description |
| --- | --- |
| [map(value)](#rust-pluralizermapper-map-value) | Pluralizes `value` and keeps its case. `User` becomes `Users`. |
| [reverse_map(value)](#rust-pluralizermapper-reverse_map-value) | Singularizes `value` and keeps the first letter's case. `Users` becomes `User`. |

###### new() {#rust-pluralizermapper-new}

Builds a pluralizer mapper.

**Definition**

```rust
pub fn new() -> Self
```

**Returns**

[PluralizerMapper](#rust-pluralizermapper)

###### map(value) {#rust-pluralizermapper-map-value}

Pluralizes `value` and keeps its case. `User` becomes `Users`.

**Definition**

```rust
fn map(&self, value: String) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | source name |

**Returns**

`String`

###### reverse_map(value) {#rust-pluralizermapper-reverse_map-value}

Singularizes `value` and keeps the first letter's case. `Users` becomes `User`.

**Definition**

```rust
fn reverse_map(&self, value: String) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | plural name |

**Returns**

`String`

### Remarks

Use this as [EntityMapper](#entitymapper) when the type name is singular and the table name is its plural. A lowercase plural table is [ChangeCasingMapper](#change-casing-mapper) then `PluralizerMapper`. Setting `Entity` / `entity` / `entity` skips the mapper.

### Examples

#### Map contacts

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/PluralizerMapperTests.cs#L15-L23)

```csharp
var mapper = new PluralizerMapper();
var contact = mapper.Map("Contact");
var address = mapper.Map("Address");
```

**Map**

```
Contacts
Addresses
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/pluralizer_mapper.rs#L45-L52)

```rust
let mapper = PluralizerMapper::new();
let contact = mapper.map("Contact".into());
let address = mapper.map("Address".into());
```

**Map**

```
Contacts
Addresses
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/PluralizerMapper.test.ts#L12-L18)

```typescript
const mapper = new PluralizerMapper();
const contact = mapper.map("Contact");
const address = mapper.map("Address");
```

**Map**

```
Contacts
Addresses
```

#### Reverse map contacts

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/PluralizerMapperTests.cs#L25-L33)

```csharp
var mapper = new PluralizerMapper();
var contact = mapper.ReverseMap("Contacts");
var address = mapper.ReverseMap("Addresses");
```

**Reverse map**

```
Contact
Address
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/pluralizer_mapper.rs#L54-L61)

```rust
let mapper = PluralizerMapper::new();
let contact = mapper.reverse_map("Contacts".into());
let address = mapper.reverse_map("Addresses".into());
```

**Reverse map**

```
Contact
Address
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/PluralizerMapper.test.ts#L20-L26)

```typescript
const mapper = new PluralizerMapper();
const contact = mapper.reverseMap("Contacts");
const address = mapper.reverseMap("Addresses");
```

**Reverse map**

```
Contact
Address
```

#### Lowercase plural

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/PluralizerMapperTests.cs#L35-L48)

```csharp
var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
var pluralizer = new PluralizerMapper();
var contact = pluralizer.Map(casing.Map("Contact"));
var address = pluralizer.Map(casing.Map("Address"));
var contactName = casing.ReverseMap(pluralizer.ReverseMap(contact));
var addressName = casing.ReverseMap(pluralizer.ReverseMap(address));
```

**Map**

```
contacts
addresses
```

**Reverse map**

```
Contact
Address
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/pluralizer_mapper.rs#L63-L75)

```rust
let casing = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
let pluralizer = PluralizerMapper::new();
let contact = pluralizer.map(casing.map("Contact".into()));
let address = pluralizer.map(casing.map("Address".into()));
let contact_name = casing.reverse_map(pluralizer.reverse_map(contact.clone()));
let address_name = casing.reverse_map(pluralizer.reverse_map(address.clone()));
```

**Map**

```
contacts
addresses
```

**Reverse map**

```
Contact
Address
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/PluralizerMapper.test.ts#L28-L39)

```typescript
const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
const pluralizer = new PluralizerMapper();
const contact = pluralizer.map(casing.map("Contact"));
const address = pluralizer.map(casing.map("Address"));
const contactName = casing.reverseMap(pluralizer.reverseMap(contact));
const addressName = casing.reverseMap(pluralizer.reverseMap(address));
```

**Map**

```
contacts
addresses
```

**Reverse map**

```
Contact
Address
```

## ChangeCasingMapper

Mapper that rewrites a name from one casing to another through `ChangeCasingConverter`. `Map` writes the stored casing. `ReverseMap` restores the application casing.

### Classes

##### TypeScript

##### ChangeCasingMapper {#typescript-changecasingmapper}

```typescript
class ChangeCasingMapper implements IMapper<string, string>
```

**Constructor**

| Name | Description |
| --- | --- |
| [ChangeCasingMapper(from, to)](#typescript-changecasingmapper-changecasingmapper-from-to) | Maps names from `from` to `to`. Reverse maps back to `from`. |

**Methods**

| Name | Description |
| --- | --- |
| [map(value)](#typescript-changecasingmapper-map-value) | Rewrites `value` into the stored casing. `UserType` with `snakeCase` becomes `user_type`. |
| [reverseMap(value)](#typescript-changecasingmapper-reversemap-value) | Rewrites `value` into the application casing. `user_type` with `pascalCase` becomes `UserType`. |

###### ChangeCasingMapper(from, to) {#typescript-changecasingmapper-changecasingmapper-from-to}

Maps names from `from` to `to`. Reverse maps back to `from`.

**Definition**

```typescript
constructor(from: Casing, to: Casing)
```

**Parameters**

| Name | Description |
| --- | --- |
| `from` | application casing, such as `pascalCase` |
| `to` | stored casing, such as `snakeCase` |

**Returns**

[ChangeCasingMapper](#typescript-changecasingmapper)

###### map(value) {#typescript-changecasingmapper-map-value}

Rewrites `value` into the stored casing. `UserType` with `snakeCase` becomes `user_type`.

**Definition**

```typescript
map(value: string): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | application name |

**Returns**

`string`

###### reverseMap(value) {#typescript-changecasingmapper-reversemap-value}

Rewrites `value` into the application casing. `user_type` with `pascalCase` becomes `UserType`.

**Definition**

```typescript
reverseMap(value: string): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | stored name |

**Returns**

`string`

##### C#

##### ChangeCasingMapper {#csharp-changecasingmapper}

```csharp
public sealed class ChangeCasingMapper : IMapper<string, string>
```

**Constructor**

| Name | Description |
| --- | --- |
| [ChangeCasingMapper(from, to)](#csharp-changecasingmapper-changecasingmapper-from-to) | Maps names from `from` to `to`. Reverse maps back to `from`. |

**Methods**

| Name | Description |
| --- | --- |
| [Map(value)](#csharp-changecasingmapper-map-value) | Rewrites `value` into the stored casing. `UserType` with `Snake` becomes `user_type`. |
| [ReverseMap(value)](#csharp-changecasingmapper-reversemap-value) | Rewrites `value` into the application casing. `user_type` with `Pascal` becomes `UserType`. |

###### ChangeCasingMapper(from, to) {#csharp-changecasingmapper-changecasingmapper-from-to}

Maps names from `from` to `to`. Reverse maps back to `from`.

**Definition**

```csharp
public ChangeCasingMapper(Casing from, Casing to)
```

**Parameters**

| Name | Description |
| --- | --- |
| `from` | application casing, such as `Pascal` |
| `to` | stored casing, such as `Snake` |

**Returns**

[ChangeCasingMapper](#csharp-changecasingmapper)

###### Map(value) {#csharp-changecasingmapper-map-value}

Rewrites `value` into the stored casing. `UserType` with `Snake` becomes `user_type`.

**Definition**

```csharp
string Map(string value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | application name |

**Returns**

`string`

###### ReverseMap(value) {#csharp-changecasingmapper-reversemap-value}

Rewrites `value` into the application casing. `user_type` with `Pascal` becomes `UserType`.

**Definition**

```csharp
string ReverseMap(string value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | stored name |

**Returns**

`string`

##### Rust

##### ChangeCasingMapper {#rust-changecasingmapper}

```rust
pub struct ChangeCasingMapper
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(from, to)](#rust-changecasingmapper-new-from-to) | Maps names from `from` to `to`. Reverse maps back to `from`. |

**Methods**

| Name | Description |
| --- | --- |
| [map(value)](#rust-changecasingmapper-map-value) | Rewrites `value` into the stored casing. `UserType` with `Snake` becomes `user_type`. |
| [reverse_map(value)](#rust-changecasingmapper-reverse_map-value) | Rewrites `value` into the application casing. `user_type` with `Pascal` becomes `UserType`. |

###### new(from, to) {#rust-changecasingmapper-new-from-to}

Maps names from `from` to `to`. Reverse maps back to `from`.

**Definition**

```rust
pub fn new(from: Casing, to: Casing) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `from` | application casing, such as `Pascal` |
| `to` | stored casing, such as `Snake` |

**Returns**

[ChangeCasingMapper](#rust-changecasingmapper)

###### map(value) {#rust-changecasingmapper-map-value}

Rewrites `value` into the stored casing. `UserType` with `Snake` becomes `user_type`.

**Definition**

```rust
fn map(&self, value: String) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | application name |

**Returns**

`String`

###### reverse_map(value) {#rust-changecasingmapper-reverse_map-value}

Rewrites `value` into the application casing. `user_type` with `Pascal` becomes `UserType`.

**Definition**

```rust
fn reverse_map(&self, value: String) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | stored name |

**Returns**

`String`

### Remarks

Use this as [EntityMapper](#entitymapper) and [ColumnMappings](#columnmappings) when application names are Pascal and stored names are snake case. `UserType` becomes `user_type`, and `Id` becomes `id`.

### Examples

#### Map contacts

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/ChangeCasingMapperTests.cs#L17-L27)

```csharp
var mapper = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
var contact = mapper.Map("Contact");
var firstName = mapper.Map("FirstName");
var emailAddress = mapper.Map("EmailAddress");
```

**Map**

```
contact
first_name
email_address
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/change_casing_mapper.rs#L43-L52)

```rust
let mapper = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
let contact = mapper.map("Contact".into());
let first_name = mapper.map("FirstName".into());
let email_address = mapper.map("EmailAddress".into());
```

**Map**

```
contact
first_name
email_address
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/ChangeCasingMapper.test.ts#L13-L21)

```typescript
const mapper = new ChangeCasingMapper("pascalCase", "snakeCase");
const contact = mapper.map("Contact");
const firstName = mapper.map("FirstName");
const emailAddress = mapper.map("EmailAddress");
```

**Map**

```
contact
first_name
email_address
```

#### Reverse map contacts

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/ChangeCasingMapperTests.cs#L29-L39)

```csharp
var mapper = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
var contact = mapper.ReverseMap("contact");
var firstName = mapper.ReverseMap("first_name");
var emailAddress = mapper.ReverseMap("email_address");
```

**Reverse map**

```
Contact
FirstName
EmailAddress
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/change_casing_mapper.rs#L54-L63)

```rust
let mapper = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
let contact = mapper.reverse_map("contact".into());
let first_name = mapper.reverse_map("first_name".into());
let email_address = mapper.reverse_map("email_address".into());
```

**Reverse map**

```
Contact
FirstName
EmailAddress
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/ChangeCasingMapper.test.ts#L23-L31)

```typescript
const mapper = new ChangeCasingMapper("pascalCase", "snakeCase");
const contact = mapper.reverseMap("contact");
const firstName = mapper.reverseMap("first_name");
const emailAddress = mapper.reverseMap("email_address");
```

**Reverse map**

```
Contact
FirstName
EmailAddress
```

## TransformMapper

Mapper built from a pair of functions.

### Classes

##### TypeScript

##### TransformMapper {#typescript-transformmapper}

```typescript
class TransformMapper<TFrom, TTo> implements IMapper<TFrom, TTo>
```

**Constructor**

| Name | Description |
| --- | --- |
| [TransformMapper(map, reverseMap)](#typescript-transformmapper-transformmapper-map-reversemap) | Builds a mapper from a pair of functions. |

**Methods**

| Name | Description |
| --- | --- |
| [map(value)](#typescript-transformmapper-map-value) | Runs the map function. |
| [reverseMap(value)](#typescript-transformmapper-reversemap-value) | Runs the reverse-map function. |

###### TransformMapper(map, reverseMap) {#typescript-transformmapper-transformmapper-map-reversemap}

Builds a mapper from a pair of functions.

**Definition**

```typescript
constructor(map: (value: TFrom) => TTo, reverseMap: (value: TTo) => TFrom)
```

**Parameters**

| Name | Description |
| --- | --- |
| `map` | `TFrom` to `TTo` |
| `reverseMap` | `TTo` to `TFrom` |

###### map(value) {#typescript-transformmapper-map-value}

Runs the map function.

**Definition**

```typescript
map(value: TFrom): TTo
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | source value |

**Returns**

`TTo`

###### reverseMap(value) {#typescript-transformmapper-reversemap-value}

Runs the reverse-map function.

**Definition**

```typescript
reverseMap(value: TTo): TFrom
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | mapped value |

**Returns**

`TFrom`

##### C#

##### TransformMapper {#csharp-transformmapper}

```csharp
public class TransformMapper<TFrom, TTo> : IMapper<TFrom, TTo>
```

**Constructor**

| Name | Description |
| --- | --- |
| [TransformMapper(map, reverseMap)](#csharp-transformmapper-transformmapper-map-reversemap) | Builds a mapper from a pair of functions. |

**Methods**

| Name | Description |
| --- | --- |
| [Map(value)](#csharp-transformmapper-map-value) | Runs the map function. |
| [ReverseMap(value)](#csharp-transformmapper-reversemap-value) | Runs the reverse-map function. |

###### TransformMapper(map, reverseMap) {#csharp-transformmapper-transformmapper-map-reversemap}

Builds a mapper from a pair of functions.

**Definition**

```csharp
public TransformMapper(Func<TFrom, TTo> map, Func<TTo, TFrom> reverseMap)
```

**Parameters**

| Name | Description |
| --- | --- |
| `map` | `TFrom` to `TTo` |
| `reverseMap` | `TTo` to `TFrom` |

###### Map(value) {#csharp-transformmapper-map-value}

Runs the map function.

**Definition**

```csharp
TTo Map(TFrom value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | source value |

**Returns**

`TTo`

###### ReverseMap(value) {#csharp-transformmapper-reversemap-value}

Runs the reverse-map function.

**Definition**

```csharp
TFrom ReverseMap(TTo value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | mapped value |

**Returns**

`TFrom`

##### Rust

##### TransformMapper {#rust-transformmapper}

```rust
pub struct TransformMapper<TFrom, TTo>
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(map, reverse_map)](#rust-transformmapper-new-map-reverse_map) | Builds a mapper from a pair of functions. |

**Methods**

| Name | Description |
| --- | --- |
| [map(value)](#rust-transformmapper-map-value) | Runs the map function. |
| [reverse_map(value)](#rust-transformmapper-reverse_map-value) | Runs the reverse-map function. |

###### new(map, reverse_map) {#rust-transformmapper-new-map-reverse_map}

Builds a mapper from a pair of functions.

**Definition**

```rust
pub fn new(map: impl Fn(TFrom) -> TTo + Send + Sync + 'static, reverse_map: impl Fn(TTo) -> TFrom + Send + Sync + 'static) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `map` | `TFrom` to `TTo` |
| `reverse_map` | `TTo` to `TFrom` |

**Returns**

[TransformMapper](#rust-transformmapper)

###### map(value) {#rust-transformmapper-map-value}

Runs the map function.

**Definition**

```rust
fn map(&self, value: TFrom) -> TTo
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | source value |

**Returns**

`TTo`

###### reverse_map(value) {#rust-transformmapper-reverse_map-value}

Runs the reverse-map function.

**Definition**

```rust
fn reverse_map(&self, value: TTo) -> TFrom
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | mapped value |

**Returns**

`TFrom`

### Remarks

Use [TransformMapper](#transformmapper) when the mapping is a pair of closures. [ReadonlyRepositoryOptions](#readonlyrepositoryoptions) accepts it for entity, column, and row mapping. The default is [IdentityMapper](#identity-mapper).

### Examples

#### Flatten with TransformMapper

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/TransformMapperTests.cs#L15-L26)

```csharp
var nested = new Nested(new User(1, "Ada"), new Contacts("ada@x.dev", "555"));
var mapper = new TransformMapper<Nested, Flat>(
    map: n => new Flat(n.User.Id, n.User.Name, n.Contacts.Email, n.Contacts.Phone),
    reverseMap: f => new Nested(new User(f.UserId, f.UserName), new Contacts(f.ContactEmail, f.ContactPhone)));
var flat = mapper.Map(nested);
```

**Before: nested value**

```
{
  user: { id: 1, name: 'Ada' },
  contacts: { email: 'ada@x.dev', phone: '555' }
}
```

**After: flat value**

```
{
  userId: 1,
  userName: 'Ada',
  contactEmail: 'ada@x.dev',
  contactPhone: '555'
}
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/mod.rs#L86-L106)

```rust
let nested = Nested {
    user: User {
        id: 1,
        name: "Ada".into(),
    },
    contacts: Contacts {
        email: "ada@x.dev".into(),
        phone: "555".into(),
    },
};
let mapper = TransformMapper::new(
    |n: Nested| Flat {
        user_id: n.user.id,
        user_name: n.user.name,
        contact_email: n.contacts.email,
        contact_phone: n.contacts.phone,
    },
    |f: Flat| Nested {
        user: User {
```

**Before: nested value**

```
{
  user: { id: 1, name: 'Ada' },
  contacts: { email: 'ada@x.dev', phone: '555' }
}
```

**After: flat value**

```
{
  userId: 1,
  userName: 'Ada',
  contactEmail: 'ada@x.dev',
  contactPhone: '555'
}
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/transformMapper.test.ts#L18-L42)

```typescript
const nested: Nested = {
  user: { id: 1, name: "Ada" },
  contacts: { email: "ada@x.dev", phone: "555" },
};
const mapper = new TransformMapper<Nested, Flat>(
  (n) => ({
    userId: n.user.id,
    userName: n.user.name,
    contactEmail: n.contacts.email,
    contactPhone: n.contacts.phone,
  }),
  (f) => ({
    user: { id: f.userId, name: f.userName },
    contacts: { email: f.contactEmail, phone: f.contactPhone },
  }),
);
const flat = mapper.map(nested);
```

**Before: nested value**

```
{
  user: { id: 1, name: 'Ada' },
  contacts: { email: 'ada@x.dev', phone: '555' }
}
```

**After: flat value**

```
{
  userId: 1,
  userName: 'Ada',
  contactEmail: 'ada@x.dev',
  contactPhone: '555'
}
```

#### Prefix a contact field

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/TransformMapperTests.cs#L29-L39)

```csharp
var mapper = new TransformMapper<string, string>(
    map: name => "contact_" + name,
    reverseMap: name => name["contact_".Length..]);
var mapped = mapper.Map("email");
var reversed = mapper.ReverseMap(mapped);
```

**Map**

```
contact_email
```

**Reverse map**

```
email
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/mod.rs#L124-L134)

```rust
let mapper = TransformMapper::new(
    |name: String| format!("contact_{name}"),
    |name: String| name["contact_".len()..].to_string(),
);
let mapped = Mapper::map(&mapper, "email".to_string());
let reversed = Mapper::reverse_map(&mapper, mapped.clone());
```

**Map**

```
contact_email
```

**Reverse map**

```
email
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/transformMapper.test.ts#L44-L53)

```typescript
const mapper = new TransformMapper<string, string>(
  (name) => `contact_${name}`,
  (name) => name.slice("contact_".length),
);
const mapped = mapper.map("email");
const reversed = mapper.reverseMap(mapped);
```

**Map**

```
contact_email
```

**Reverse map**

```
email
```

#### Map a contact name and email

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/TransformMapperTests.cs#L41-L52)

```csharp
var contact = new Contact("Ada", "ada@x.dev");
var mapper = new TransformMapper<Contact, ContactRow>(
    map: item => new ContactRow(item.Name, item.Email),
    reverseMap: row => new Contact(row.ContactName, row.ContactEmail));
var mapped = mapper.Map(contact);
var reversed = mapper.ReverseMap(mapped);
```

**Before: contact**

```
{
  name: 'Ada',
  email: 'ada@x.dev'
}
```

**After: mapped contact**

```
{
  contactName: 'Ada',
  contactEmail: 'ada@x.dev'
}
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/mod.rs#L136-L162)

```rust
let contact = Contact {
    name: "Ada".into(),
    email: "ada@x.dev".into(),
};
let mapper = TransformMapper::new(
    |item: Contact| ContactRow {
        contact_name: item.name,
        contact_email: item.email,
    },
    |row: ContactRow| Contact {
        name: row.contact_name,
        email: row.contact_email,
    },
);
let mapped = Mapper::map(&mapper, contact.clone());
let reversed = Mapper::reverse_map(&mapper, mapped.clone());
```

**Before: contact**

```
{
  name: 'Ada',
  email: 'ada@x.dev'
}
```

**After: mapped contact**

```
{
  contactName: 'Ada',
  contactEmail: 'ada@x.dev'
}
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/transformMapper.test.ts#L55-L65)

```typescript
const contact: Contact = { name: "Ada", email: "ada@x.dev" };
const mapper = new TransformMapper<Contact, ContactRow>(
  (item) => ({ contactName: item.name, contactEmail: item.email }),
  (row) => ({ name: row.contactName, email: row.contactEmail }),
);
const mapped = mapper.map(contact);
const reversed = mapper.reverseMap(mapped);
```

**Before: contact**

```
{
  name: 'Ada',
  email: 'ada@x.dev'
}
```

**After: mapped contact**

```
{
  contactName: 'Ada',
  contactEmail: 'ada@x.dev'
}
```

## FlatToNestedMapper

Maps flat objects to nested objects. `ReverseMap` / `reverseMap` / `reverse_map` maps the nested object back.

### Classes

##### TypeScript

##### IFlatToNestedMapper {#typescript-iflattonestedmapper}

```typescript
interface IFlatToNestedMapper<T> extends IMapper<Iterable<T | Cells>, T[]>
```

**Properties**

| Name | Description |
| --- | --- |
| spec | The [TypeMap](#typescript-typemap) used to collapse join rows. |

##### FlatToNestedMapper {#typescript-flattonestedmapper}

```typescript
class FlatToNestedMapper<T> implements IFlatToNestedMapper<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [FlatToNestedMapper(spec)](#typescript-flattonestedmapper-flattonestedmapper-spec) | Builds a collapse mapper from a [TypeMap](#typescript-typemap). |

**Properties**

| Name | Description |
| --- | --- |
| spec | The [TypeMap](#typescript-typemap) used to collapse join rows. |

**Methods**

| Name | Description |
| --- | --- |
| [map(rows)](#typescript-flattonestedmapper-map-rows) | Maps join rows to unique parents with unique children. Nested group fields on a collection are omitted. |
| [reverseMap(values)](#typescript-flattonestedmapper-reversemap-values) | Flattens nested parents back to denormalized join rows. |

###### FlatToNestedMapper(spec) {#typescript-flattonestedmapper-flattonestedmapper-spec}

Builds a collapse mapper from a [TypeMap](#typescript-typemap).

**Definition**

```typescript
constructor(spec: TypeMap<T>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `spec` | [TypeMap](#typescript-typemap); scalars map column to field, groups collect unique children by key |

###### map(rows) {#typescript-flattonestedmapper-map-rows}

Maps join rows to unique parents with unique children. Nested group fields on a collection are omitted.

**Definition**

```typescript
map(rows: Iterable<T | Cells>): T[]
```

**Parameters**

| Name | Description |
| --- | --- |
| `rows` | denormalized join rows, or already-mapped `T` |

**Returns**

`T[]`

###### reverseMap(values) {#typescript-flattonestedmapper-reversemap-values}

Flattens nested parents back to denormalized join rows.

**Definition**

```typescript
reverseMap(values: T[]): Cells[]
```

**Parameters**

| Name | Description |
| --- | --- |
| `values` | nested parents |

**Returns**

`Cells[]`

##### TypeMap {#typescript-typemap}

```typescript
type TypeMap<T>
```

**Properties**

| Name | Description |
| --- | --- |
| key | Parent key column. |
| fields | Column or group per destination field. |

##### TypeMapGroup {#typescript-typemapgroup}

```typescript
type TypeMapGroup<E>
```

**Properties**

| Name | Description |
| --- | --- |
| key | Child key column used to unique children. |
| fields | Column names for the child. |

##### TypeMapFields {#typescript-typemapfields}

```typescript
type TypeMapFields<T>
```

##### C#

##### IFlatToNestedMapper {#csharp-iflattonestedmapper}

```csharp
public interface IFlatToNestedMapper : IMapper<IEnumerable<Row>, IReadOnlyList<Dictionary<string, MappedValue>>>
```

**Properties**

| Name | Description |
| --- | --- |
| Spec | The [TypeMap](#csharp-typemap) used to collapse join rows. |

##### FlatToNestedMapper {#csharp-flattonestedmapper}

```csharp
public sealed class FlatToNestedMapper : IFlatToNestedMapper
```

**Constructor**

| Name | Description |
| --- | --- |
| [FlatToNestedMapper(spec)](#csharp-flattonestedmapper-flattonestedmapper-spec) | Builds a collapse mapper from a [TypeMap](#csharp-typemap). |

**Properties**

| Name | Description |
| --- | --- |
| Spec | The [TypeMap](#csharp-typemap) used to collapse join rows. |

**Methods**

| Name | Description |
| --- | --- |
| [Map(rows)](#csharp-flattonestedmapper-map-rows) | Maps join rows to unique parents with unique children. Nested group fields on a collection are omitted. |
| [ReverseMap(values)](#csharp-flattonestedmapper-reversemap-values) | Flattens nested parents back to denormalized join rows. |

###### FlatToNestedMapper(spec) {#csharp-flattonestedmapper-flattonestedmapper-spec}

Builds a collapse mapper from a [TypeMap](#csharp-typemap).

**Definition**

```csharp
public FlatToNestedMapper(TypeMap spec)
```

**Parameters**

| Name | Description |
| --- | --- |
| `spec` | [TypeMap](#csharp-typemap); scalars map column to field, groups collect unique children by key |

###### Map(rows) {#csharp-flattonestedmapper-map-rows}

Maps join rows to unique parents with unique children. Nested group fields on a collection are omitted.

**Definition**

```csharp
IReadOnlyList<Dictionary<string, MappedValue>> Map(IEnumerable<Row> rows)
```

**Parameters**

| Name | Description |
| --- | --- |
| `rows` | denormalized join rows |

**Returns**

`IReadOnlyList<Dictionary<string, `[MappedValue](#csharp-mappedvalue)`>>`

###### ReverseMap(values) {#csharp-flattonestedmapper-reversemap-values}

Flattens nested parents back to denormalized join rows.

**Definition**

```csharp
IReadOnlyList<Row> ReverseMap(IReadOnlyList<Dictionary<string, MappedValue>> values)
```

**Parameters**

| Name | Description |
| --- | --- |
| `values` | nested parents |

**Returns**

`IReadOnlyList<`[Row](#repositories)`>`

##### TypeMap {#csharp-typemap}

```csharp
public sealed class TypeMap
```

**Properties**

| Name | Description |
| --- | --- |
| Key | Parent key column. |
| Fields | Column or group per destination field. |

##### TypeMapGroup {#csharp-typemapgroup}

```csharp
public sealed class TypeMapGroup
```

**Properties**

| Name | Description |
| --- | --- |
| Key | Child key column used to unique children. |
| Fields | Column or group per child field. |

##### TypeMapField {#csharp-typemapfield}

```csharp
public abstract record TypeMapField
```

**Properties**

| Name | Description |
| --- | --- |
| Column | Maps a destination field to a source column. |
| Group | Maps a destination field to a child [TypeMapGroup](#csharp-typemapgroup). |

##### MappedValue {#csharp-mappedvalue}

```csharp
public abstract record MappedValue
```

**Properties**

| Name | Description |
| --- | --- |
| Cell | A scalar cell. |
| Many | A child collection. |

##### Rust

##### FlatToNestedMapperTrait {#rust-flattonestedmappertrait}

```rust
pub trait FlatToNestedMapper: Mapper<Vec<Row>, Vec<MappedRow>>
```

**Methods**

| Name | Description |
| --- | --- |
| [spec()](#rust-flattonestedmappertrait-spec) | The [TypeMap](#rust-typemap) used to collapse join rows. |

###### spec() {#rust-flattonestedmappertrait-spec}

The [TypeMap](#rust-typemap) used to collapse join rows.

**Definition**

```rust
fn spec(&self) -> &TypeMap
```

**Returns**

`&`[TypeMap](#rust-typemap)

##### FlatToNestedMapper {#rust-flattonestedmapper}

```rust
pub struct FlatToNestedMapper
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(spec)](#rust-flattonestedmapper-new-spec) | Builds a collapse mapper from a [TypeMap](#rust-typemap). |

**Methods**

| Name | Description |
| --- | --- |
| [map(rows)](#rust-flattonestedmapper-map-rows) | Maps join rows to unique parents with unique children. Nested group fields on a collection are omitted. |
| [reverse_map(values)](#rust-flattonestedmapper-reverse_map-values) | Flattens nested parents back to denormalized join rows. |
| [spec()](#rust-flattonestedmapper-spec) | The [TypeMap](#rust-typemap) used to collapse join rows. |

###### new(spec) {#rust-flattonestedmapper-new-spec}

Builds a collapse mapper from a [TypeMap](#rust-typemap).

**Definition**

```rust
pub fn new(spec: TypeMap) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `spec` | [TypeMap](#rust-typemap); scalars map column to field, groups collect unique children by key |

**Returns**

[FlatToNestedMapper](#rust-flattonestedmapper)

###### map(rows) {#rust-flattonestedmapper-map-rows}

Maps join rows to unique parents with unique children. Nested group fields on a collection are omitted.

**Definition**

```rust
fn map(&self, rows: Vec<Row>) -> Vec<MappedRow>
```

**Parameters**

| Name | Description |
| --- | --- |
| `rows` | denormalized join rows |

**Returns**

`Vec<`[MappedRow](#rust-mappedrow)`>`

###### reverse_map(values) {#rust-flattonestedmapper-reverse_map-values}

Flattens nested parents back to denormalized join rows.

**Definition**

```rust
fn reverse_map(&self, values: Vec<MappedRow>) -> Vec<Row>
```

**Parameters**

| Name | Description |
| --- | --- |
| `values` | nested parents |

**Returns**

`Vec<Row>`

###### spec() {#rust-flattonestedmapper-spec}

The [TypeMap](#rust-typemap) used to collapse join rows.

**Definition**

```rust
fn spec(&self) -> &TypeMap
```

**Returns**

`&`[TypeMap](#rust-typemap)

##### TypeMap {#rust-typemap}

```rust
pub struct TypeMap
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(key, fields)](#rust-typemap-new-key-fields) | Builds a collapse spec. |

**Properties**

| Name | Description |
| --- | --- |
| key | Parent key column. |
| fields | Column or group per destination field. |

###### new(key, fields) {#rust-typemap-new-key-fields}

Builds a collapse spec.

**Definition**

```rust
pub fn new(key: impl Into<String>, fields: Vec<(String, TypeMapField)>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `key` | parent key column |
| `fields` | destination name and [TypeMapField](#rust-typemapfield) pairs |

**Returns**

[TypeMap](#rust-typemap)

##### TypeMapGroup {#rust-typemapgroup}

```rust
pub struct TypeMapGroup
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(key, fields)](#rust-typemapgroup-new-key-fields) | Builds a child-group spec. |

**Properties**

| Name | Description |
| --- | --- |
| key | Child key column used to unique children. |
| fields | Column or group per child field. |

###### new(key, fields) {#rust-typemapgroup-new-key-fields}

Builds a child-group spec.

**Definition**

```rust
pub fn new(key: impl Into<String>, fields: Vec<(String, TypeMapField)>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `key` | child key column |
| `fields` | destination name and [TypeMapField](#rust-typemapfield) pairs |

**Returns**

[TypeMapGroup](#rust-typemapgroup)

##### TypeMapField {#rust-typemapfield}

```rust
pub enum TypeMapField
```

**Properties**

| Name | Description |
| --- | --- |
| Column | Maps a destination field to a source column. |
| Group | Maps a destination field to a child [TypeMapGroup](#rust-typemapgroup). |

**Methods**

| Name | Description |
| --- | --- |
| [column(source)](#rust-typemapfield-column-source) | Builds a [Column](#rust-typemapfield-column) field. |

###### column(source) {#rust-typemapfield-column-source}

Builds a [Column](#rust-typemapfield-column) field.

**Definition**

```rust
pub fn column(source: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `source` | source column name |

**Returns**

[TypeMapField](#rust-typemapfield)

##### MappedValue {#rust-mappedvalue}

```rust
pub enum MappedValue
```

**Properties**

| Name | Description |
| --- | --- |
| Cell | A scalar cell. |
| Many | A child collection. |

##### MappedRow {#rust-mappedrow}

```rust
pub type MappedRow = HashMap<String, MappedValue>
```

### Remarks

Collapses denormalized join rows into unique parents with unique children. Scalars map column to field; groups collect children by key.

> **TypeScript**
>
> Map to generic `T`.

> **C#**
>
> Map to a [MappedValue](#mappedvalue) graph.

> **Rust**
>
> Map to a [MappedValue](#mappedvalue) graph.

Nested group fields on a collection are omitted.

This is a collapse mapper, not enrichment. Related fills after read use [IEnriched](#ienriched).

### Examples

#### Map join rows to a nested contact

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/FlatToNestedMapperTests.cs#L101-L164)

```csharp
var mapper = new FlatToNestedMapper(new TypeMap
{
    Key = "id",
    Fields = new Dictionary<string, TypeMapField>
    {
        ["id"] = "id",
        ["first_name"] = "first_name",
        ["last_name"] = "last_name",
        ["email"] = "email",
        ["contact_source_name"] = "contact_source_name",
        ["contact_type_name"] = "contact_type_name",
        ["addresses"] = new TypeMapGroup
        {
            Key = "address_id",
            Fields = new Dictionary<string, TypeMapField>
            {
                ["id"] = "address_id",
                ["line1"] = "address_line1",
                ["city"] = "address_city",
                ["address_type_name"] = "address_type_name",
            },
        },
        ["phones"] = new TypeMapGroup
        {
            Key = "phone_id",
            Fields = new Dictionary<string, TypeMapField>
            {
                ["id"] = "phone_id",
                ["number"] = "phone_number",
                ["phone_type_name"] = "phone_type_name",
            },
        },
    },
});
var adaHome = new Row(
[
    new("id", 7),
    new("first_name", "Ada"),
    new("last_name", "Lovelace"),
    new("email", "ada@ex.com"),
    new("contact_source_name", "Web"),
    new("contact_type_name", "Person"),
    new("address_id", 101),
    new("address_line1", "10 Downing St"),
    new("address_city", "London"),
    new("address_type_name", "Home"),
    new("phone_id", 201),
    new("phone_number", "+44"),
    new("phone_type_name", "Mobile"),
]);
var adaWork = new Row(adaHome)
{
    ["address_id"] = 102,
    ["address_line1"] = "1 Parliament Sq"
};
var nested = mapper.Map([adaHome, adaWork]);
var flat = mapper.ReverseMap(nested);
```

**After: nested contact**

```
{
  id: 7,
  first_name: "Ada",
  last_name: "Lovelace",
  email: "ada@ex.com",
  contact_source_name: "Web",
  contact_type_name: "Person",
  addresses: [
    { id: 101, line1: "10 Downing St", city: "London", address_type_name: "Home" },
    { id: 102, line1: "1 Parliament Sq", city: "London", address_type_name: "Home" }
  ],
  phones: [{ id: 201, number: "+44", phone_type_name: "Mobile" }]
}
```

**After: reverseMap join rows**

```
[
  {
    id: 7,
    first_name: "Ada",
    last_name: "Lovelace",
    email: "ada@ex.com",
    contact_source_name: "Web",
    contact_type_name: "Person",
    address_id: 101,
    address_line1: "10 Downing St",
    address_city: "London",
    address_type_name: "Home",
    phone_id: 201,
    phone_number: "+44",
    phone_type_name: "Mobile"
  },
  {
    id: 7,
    first_name: "Ada",
    last_name: "Lovelace",
    email: "ada@ex.com",
    contact_source_name: "Web",
    contact_type_name: "Person",
    address_id: 102,
    address_line1: "1 Parliament Sq",
    address_city: "London",
    address_type_name: "Home",
    phone_id: 201,
    phone_number: "+44",
    phone_type_name: "Mobile"
  }
]
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/flat_to_nested_mapper.rs#L385-L453)

```rust
let mapper = FlatToNestedMapper::new(TypeMap::new(
    "id",
    vec![
        ("id".into(), TypeMapField::column("id")),
        ("first_name".into(), TypeMapField::column("first_name")),
        ("last_name".into(), TypeMapField::column("last_name")),
        ("email".into(), TypeMapField::column("email")),
        (
            "contact_source_name".into(),
            TypeMapField::column("contact_source_name"),
        ),
        (
            "contact_type_name".into(),
            TypeMapField::column("contact_type_name"),
        ),
        (
            "addresses".into(),
            TypeMapField::Group(TypeMapGroup::new(
                "address_id",
                vec![
                    ("id".into(), TypeMapField::column("address_id")),
                    ("line1".into(), TypeMapField::column("address_line1")),
                    ("city".into(), TypeMapField::column("address_city")),
                    (
                        "address_type_name".into(),
                        TypeMapField::column("address_type_name"),
                    ),
                ],
            )),
        ),
        (
            "phones".into(),
            TypeMapField::Group(TypeMapGroup::new(
                "phone_id",
                vec![
                    ("id".into(), TypeMapField::column("phone_id")),
                    ("number".into(), TypeMapField::column("phone_number")),
                    (
                        "phone_type_name".into(),
                        TypeMapField::column("phone_type_name"),
                    ),
                ],
            )),
        ),
    ],
));
let ada_home = cell(&[
    ("id", Value::Int(7)),
    ("first_name", Value::from("Ada")),
    ("last_name", Value::from("Lovelace")),
    ("email", Value::from("ada@ex.com")),
    ("contact_source_name", Value::from("Web")),
    ("contact_type_name", Value::from("Person")),
    ("address_id", Value::Int(101)),
    ("address_line1", Value::from("10 Downing St")),
    ("address_city", Value::from("London")),
    ("address_type_name", Value::from("Home")),
    ("phone_id", Value::Int(201)),
    ("phone_number", Value::from("+44")),
    ("phone_type_name", Value::from("Mobile")),
]);
let mut ada_work = ada_home.clone();
ada_work.insert("address_id".into(), Value::Int(102));
ada_work.insert("address_line1".into(), Value::from("1 Parliament Sq"));
let nested = Mapper::map(&mapper, vec![ada_home.clone(), ada_work.clone()]);
let flat = Mapper::reverse_map(&mapper, nested.clone());
```

**After: nested contact**

```
{
  id: 7,
  first_name: "Ada",
  last_name: "Lovelace",
  email: "ada@ex.com",
  contact_source_name: "Web",
  contact_type_name: "Person",
  addresses: [
    { id: 101, line1: "10 Downing St", city: "London", address_type_name: "Home" },
    { id: 102, line1: "1 Parliament Sq", city: "London", address_type_name: "Home" }
  ],
  phones: [{ id: 201, number: "+44", phone_type_name: "Mobile" }]
}
```

**After: reverseMap join rows**

```
[
  {
    id: 7,
    first_name: "Ada",
    last_name: "Lovelace",
    email: "ada@ex.com",
    contact_source_name: "Web",
    contact_type_name: "Person",
    address_id: 101,
    address_line1: "10 Downing St",
    address_city: "London",
    address_type_name: "Home",
    phone_id: 201,
    phone_number: "+44",
    phone_type_name: "Mobile"
  },
  {
    id: 7,
    first_name: "Ada",
    last_name: "Lovelace",
    email: "ada@ex.com",
    contact_source_name: "Web",
    contact_type_name: "Person",
    address_id: 102,
    address_line1: "1 Parliament Sq",
    address_city: "London",
    address_type_name: "Home",
    phone_id: 201,
    phone_number: "+44",
    phone_type_name: "Mobile"
  }
]
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/FlatToNestedMapper.test.ts#L90-L155)

```typescript
const mapper = new FlatToNestedMapper<Contact>({
  key: "id",
  fields: {
    id: "id",
    first_name: "first_name",
    last_name: "last_name",
    email: "email",
    contact_source_name: "contact_source_name",
    contact_type_name: "contact_type_name",
    addresses: {
      key: "address_id",
      fields: {
        id: "address_id",
        line1: "address_line1",
        city: "address_city",
        address_type_name: "address_type_name",
      },
    },
    phones: {
      key: "phone_id",
      fields: {
        id: "phone_id",
        number: "phone_number",
        phone_type_name: "phone_type_name",
      },
    },
  },
});
const adaHome: Cells = {
  id: 7,
  first_name: "Ada",
  last_name: "Lovelace",
  email: "ada@ex.com",
  contact_source_name: "Web",
  contact_type_name: "Person",
  address_id: 101,
  address_line1: "10 Downing St",
  address_city: "London",
  address_type_name: "Home",
  phone_id: 201,
  phone_number: "+44",
  phone_type_name: "Mobile",
};
const adaWork = {
  ...adaHome,
  address_id: 102,
  address_line1: "1 Parliament Sq",
};
const nested = mapper.map([adaHome, adaWork]);
const flat = mapper.reverseMap(nested);
```

**After: nested contact**

```
{
  id: 7,
  first_name: "Ada",
  last_name: "Lovelace",
  email: "ada@ex.com",
  contact_source_name: "Web",
  contact_type_name: "Person",
  addresses: [
    { id: 101, line1: "10 Downing St", city: "London", address_type_name: "Home" },
    { id: 102, line1: "1 Parliament Sq", city: "London", address_type_name: "Home" }
  ],
  phones: [{ id: 201, number: "+44", phone_type_name: "Mobile" }]
}
```

**After: reverseMap join rows**

```
[
  {
    id: 7,
    first_name: "Ada",
    last_name: "Lovelace",
    email: "ada@ex.com",
    contact_source_name: "Web",
    contact_type_name: "Person",
    address_id: 101,
    address_line1: "10 Downing St",
    address_city: "London",
    address_type_name: "Home",
    phone_id: 201,
    phone_number: "+44",
    phone_type_name: "Mobile"
  },
  {
    id: 7,
    first_name: "Ada",
    last_name: "Lovelace",
    email: "ada@ex.com",
    contact_source_name: "Web",
    contact_type_name: "Person",
    address_id: 102,
    address_line1: "1 Parliament Sq",
    address_city: "London",
    address_type_name: "Home",
    phone_id: 201,
    phone_number: "+44",
    phone_type_name: "Mobile"
  }
]
```

## Type Converters

`TypeConverter` is only `Convert` / `convert` / `convert` and `ReverseConvert` / `reverseConvert` / `reverse_convert`. A dialect adds a converter when the native provider cannot map the type. Core does not ship bool / datetime / uuid / decimal / binary converters.

### Classes

##### TypeScript

##### TypeConverter {#typescript-typeconverter}

```typescript
abstract class TypeConverter<To = Cells, From = Cells>
```

**Methods**

| Name | Description |
| --- | --- |
| [convert(from)](#typescript-typeconverter-convert-from) | Converts `From` to `To`. |
| [reverseConvert(to)](#typescript-typeconverter-reverseconvert-to) | Converts `To` back to `From`. |

###### convert(from) {#typescript-typeconverter-convert-from}

Converts `From` to `To`.

**Definition**

```typescript
abstract convert(from: From): To
```

**Parameters**

| Name | Description |
| --- | --- |
| `from` | source value |

**Returns**

`To`

###### reverseConvert(to) {#typescript-typeconverter-reverseconvert-to}

Converts `To` back to `From`.

**Definition**

```typescript
abstract reverseConvert(to: To): From
```

**Parameters**

| Name | Description |
| --- | --- |
| `to` | converted value |

**Returns**

`From`

##### C#

##### ITypeConverter {#csharp-itypeconverter}

```csharp
public interface ITypeConverter<To, From>
```

**Methods**

| Name | Description |
| --- | --- |
| [Convert(from)](#csharp-itypeconverter-convert-from) | Converts `From` to `To`. |
| [ReverseConvert(to)](#csharp-itypeconverter-reverseconvert-to) | Converts `To` back to `From`. |

###### Convert(from) {#csharp-itypeconverter-convert-from}

Converts `From` to `To`.

**Definition**

```csharp
To Convert(From from)
```

**Parameters**

| Name | Description |
| --- | --- |
| `from` | source value |

**Returns**

`To`

###### ReverseConvert(to) {#csharp-itypeconverter-reverseconvert-to}

Converts `To` back to `From`.

**Definition**

```csharp
From ReverseConvert(To to)
```

**Parameters**

| Name | Description |
| --- | --- |
| `to` | converted value |

**Returns**

`From`

##### TypeConverter {#csharp-typeconverter}

```csharp
public abstract class TypeConverter<To, From> : ITypeConverter<To, From>
```

**Methods**

| Name | Description |
| --- | --- |
| [Convert(from)](#csharp-typeconverter-convert-from) | Converts `From` to `To`. |
| [ReverseConvert(to)](#csharp-typeconverter-reverseconvert-to) | Converts `To` back to `From`. |

###### Convert(from) {#csharp-typeconverter-convert-from}

Converts `From` to `To`.

**Definition**

```csharp
public abstract To Convert(From from)
```

**Parameters**

| Name | Description |
| --- | --- |
| `from` | source value |

**Returns**

`To`

###### ReverseConvert(to) {#csharp-typeconverter-reverseconvert-to}

Converts `To` back to `From`.

**Definition**

```csharp
public abstract From ReverseConvert(To to)
```

**Parameters**

| Name | Description |
| --- | --- |
| `to` | converted value |

**Returns**

`From`

##### Rust

##### TypeConverter {#rust-typeconverter}

```rust
pub trait TypeConverter<To, From>
```

**Methods**

| Name | Description |
| --- | --- |
| [convert(from)](#rust-typeconverter-convert-from) | Converts `From` to `To`. |
| [reverse_convert(to)](#rust-typeconverter-reverse_convert-to) | Converts `To` back to `From`. |

###### convert(from) {#rust-typeconverter-convert-from}

Converts `From` to `To`.

**Definition**

```rust
fn convert(&self, from: &From) -> Result<To>
```

**Parameters**

| Name | Description |
| --- | --- |
| `from` | source value |

**Returns**

`Result<To>`

###### reverse_convert(to) {#rust-typeconverter-reverse_convert-to}

Converts `To` back to `From`.

**Definition**

```rust
fn reverse_convert(&self, to: &To) -> Result<From>
```

**Parameters**

| Name | Description |
| --- | --- |
| `to` | converted value |

**Returns**

`Result<From>`

### Remarks

Repository read is convert, then map. See [TransformMapper](#transform-mapper) and [Repositories](#repositories).

### See also

- [RowTypeConverter](#row-type-converter)

## RowTypeConverter

TypeConverter for a row. Holds `Types` / `types` / `types` and `Fields` / `fields` / `fields`. `Convert` / `convert` / `convert` uses `Fields[name]` / `fields[name]` / `fields[name]`. `Types` / `types` / `types` is the provider catalog merge and is unused at convert time.

### Classes

##### TypeScript

##### RowTypeConverter {#typescript-rowtypeconverter}

```typescript
class RowTypeConverter<T = Cells> extends TypeConverter<T, Cells>
```

**Constructor**

| Name | Description |
| --- | --- |
| [RowTypeConverter(types, fields)](#typescript-rowtypeconverter-rowtypeconverter-types-fields) | Builds a row converter. Convert uses [fields](#typescript-rowtypeconverter-fields), not [types](#typescript-rowtypeconverter-types). |

**Properties**

| Name | Description |
| --- | --- |
| types | Provider type catalog. Convert does not read this map. |
| fields | Per-column converters. Convert uses `fields[name]`. |

**Methods**

| Name | Description |
| --- | --- |
| [convert(row, type?)](#typescript-rowtypeconverter-convert-row-type) | Converts each cell through [fields](#typescript-rowtypeconverter-fields). Optional `type` constructs a destination instance. |
| [reverseConvert(row)](#typescript-rowtypeconverter-reverseconvert-row) | Reverse-converts each cell through [fields](#typescript-rowtypeconverter-fields). Skips array values. |

###### RowTypeConverter(types, fields) {#typescript-rowtypeconverter-rowtypeconverter-types-fields}

Builds a row converter. Convert uses [fields](#typescript-rowtypeconverter-fields), not [types](#typescript-rowtypeconverter-types).

**Definition**

```typescript
constructor(types?: Record<string, TypeConverter<Cell, Cell>>, fields?: Record<string, TypeConverter<Cell, Cell>>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `types` | provider catalog; unused at convert time |
| `fields` | per-column TypeConverter |

###### convert(row, type?) {#typescript-rowtypeconverter-convert-row-type}

Converts each cell through [fields](#typescript-rowtypeconverter-fields). Optional `type` constructs a destination instance.

**Definition**

```typescript
convert(row: Cells, type?: Constructable<T>): T
```

**Parameters**

| Name | Description |
| --- | --- |
| `row` | source cells |
| `type` | optional `new () => T` |

**Returns**

`T`

###### reverseConvert(row) {#typescript-rowtypeconverter-reverseconvert-row}

Reverse-converts each cell through [fields](#typescript-rowtypeconverter-fields). Skips array values.

**Definition**

```typescript
reverseConvert(row: T): Cells
```

**Parameters**

| Name | Description |
| --- | --- |
| `row` | converted object |

**Returns**

`Cells`

##### C#

##### RowTypeConverter {#csharp-rowtypeconverter}

```csharp
public sealed class RowTypeConverter<T> : TypeConverter<T, Row> where T : new()
```

**Constructor**

| Name | Description |
| --- | --- |
| [RowTypeConverter(types, fields)](#csharp-rowtypeconverter-rowtypeconverter-types-fields) | Builds a row converter. Convert uses [Fields](#csharp-rowtypeconverter-fields), not [Types](#csharp-rowtypeconverter-types). |

**Properties**

| Name | Description |
| --- | --- |
| Types | Provider type catalog. Convert does not read this map. |
| Fields | Per-column converters. Convert uses `Fields[name]`. |

**Methods**

| Name | Description |
| --- | --- |
| [Convert(from)](#csharp-rowtypeconverter-convert-from) | Converts a row to `T`. Applies [Fields](#csharp-rowtypeconverter-fields), then assigns writable scalar properties (or dictionary keys). |
| [ReverseConvert(to)](#csharp-rowtypeconverter-reverseconvert-to) | Reverse-converts `T` to a row through [Fields](#csharp-rowtypeconverter-fields). |

###### RowTypeConverter(types, fields) {#csharp-rowtypeconverter-rowtypeconverter-types-fields}

Builds a row converter. Convert uses [Fields](#csharp-rowtypeconverter-fields), not [Types](#csharp-rowtypeconverter-types).

**Definition**

```csharp
public RowTypeConverter(IReadOnlyDictionary<string, TypeConverter<Value, Value>>? types = null, IReadOnlyDictionary<string, TypeConverter<Value, Value>>? fields = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `types` | provider catalog; unused at convert time |
| `fields` | per-column TypeConverter |

###### Convert(from) {#csharp-rowtypeconverter-convert-from}

Converts a row to `T`. Applies [Fields](#csharp-rowtypeconverter-fields), then assigns writable scalar properties (or dictionary keys).

**Definition**

```csharp
public override T Convert(Row from)
```

**Parameters**

| Name | Description |
| --- | --- |
| `from` | source row |

**Returns**

`T`

###### ReverseConvert(to) {#csharp-rowtypeconverter-reverseconvert-to}

Reverse-converts `T` to a row through [Fields](#csharp-rowtypeconverter-fields).

**Definition**

```csharp
public override Row ReverseConvert(T to)
```

**Parameters**

| Name | Description |
| --- | --- |
| `to` | converted object |

**Returns**

`Row`

##### Rust

##### RowTypeConverter {#rust-rowtypeconverter}

```rust
pub struct RowTypeConverter
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(columns)](#rust-rowtypeconverter-new-columns) | Builds a row converter for the given columns. |
| [of()](#rust-rowtypeconverter-of) | Builds a converter from `T: ConvertSchema`. |

**Properties**

| Name | Description |
| --- | --- |
| types | Provider type catalog. Convert does not read this map. |
| fields | Per-column converters. Convert uses `fields[name]`. |

**Methods**

| Name | Description |
| --- | --- |
| [with_types(types)](#rust-rowtypeconverter-with_types-types) | Merges a provider catalog into [types](#rust-rowtypeconverter-types). |
| [with_fields(fields)](#rust-rowtypeconverter-with_fields-fields) | Merges per-column converters into [fields](#rust-rowtypeconverter-fields). |
| [convert(row)](#rust-rowtypeconverter-convert-row) | Converts each cell through [fields](#rust-rowtypeconverter-fields). |
| [reverse_convert(row)](#rust-rowtypeconverter-reverse_convert-row) | Reverse-converts each cell through [fields](#rust-rowtypeconverter-fields). |

###### new(columns) {#rust-rowtypeconverter-new-columns}

Builds a row converter for the given columns.

**Definition**

```rust
pub fn new(columns: Vec<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `columns` | column names |

**Returns**

[RowTypeConverter](#rust-rowtypeconverter)

###### of() {#rust-rowtypeconverter-of}

Builds a converter from `T: ConvertSchema`.

**Definition**

```rust
pub fn of<T: ConvertSchema>() -> Self
```

**Returns**

[RowTypeConverter](#rust-rowtypeconverter)

###### with_types(types) {#rust-rowtypeconverter-with_types-types}

Merges a provider catalog into [types](#rust-rowtypeconverter-types).

**Definition**

```rust
pub fn with_types(self, types: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `types` | provider catalog; unused at convert time |

**Returns**

[RowTypeConverter](#rust-rowtypeconverter)

###### with_fields(fields) {#rust-rowtypeconverter-with_fields-fields}

Merges per-column converters into [fields](#rust-rowtypeconverter-fields).

**Definition**

```rust
pub fn with_fields(self, fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `fields` | per-column TypeConverter |

**Returns**

[RowTypeConverter](#rust-rowtypeconverter)

###### convert(row) {#rust-rowtypeconverter-convert-row}

Converts each cell through [fields](#rust-rowtypeconverter-fields).

**Definition**

```rust
pub fn convert(&self, row: &Row) -> Result<Row>
```

**Parameters**

| Name | Description |
| --- | --- |
| `row` | source row |

**Returns**

`Result<Row>`

###### reverse_convert(row) {#rust-rowtypeconverter-reverse_convert-row}

Reverse-converts each cell through [fields](#rust-rowtypeconverter-fields).

**Definition**

```rust
pub fn reverse_convert(&self, row: &Row) -> Result<Row>
```

**Parameters**

| Name | Description |
| --- | --- |
| `row` | converted row |

**Returns**

`Result<Row>`

### Remarks

`Types` / `types` / `types` is the provider catalog merge. `Convert` / `convert` / `convert` and `ReverseConvert` / `reverseConvert` / `reverse_convert` only consult `Fields` / `fields` / `fields`.

Repository read is convert, then map. See [TransformMapper](#transform-mapper) and [CRUD repository](#crud-repository).

### Examples

#### Convert a row

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/RowTypeConverterTests.cs#L28-L46)

```csharp
var converter = new RowTypeConverter<User>();
var user = converter.Convert(new Row
{
    ["Id"] = 7,
    ["Username"] = "ada",
    ["IsEnabled"] = 1,
    ["Created"] = "2020-01-02T03:04:05.000Z"
});
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/converters/row_type_converter.rs#L101-L108)

```rust
let converter = RowTypeConverter::new(vec!["id".into(), "name".into()]);
let row = record(&[("id", Value::Int(7)), ("name", Value::from("ada"))]);
let dest = converter.convert(&row).unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/converters/RowTypeConverter.test.ts#L13-L18)

```typescript
const converter = new RowTypeConverter();
const item = converter.convert({ id: 7, username: "ada" });
const row = converter.reverseConvert(item);
```

#### Field converter

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/RowTypeConverterTests.cs#L91-L102)

```csharp
var converter = new RowTypeConverter<Dictionary<string, Value>>(null, new Dictionary<string, TypeConverter<Value, Value>>
{
    ["flag"] = new OnlyOne()
});
var dest = converter.Convert(new Row { ["flag"] = 1L, ["name"] = "a" });
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/converters/row_type_converter.rs#L125-L143)

```rust
let converter =
    RowTypeConverter::new(vec!["is_enabled".into()]).with_fields(HashMap::from([(
        "is_enabled".into(),
        Box::new(OnlyOne) as Box<dyn TypeConverter<Value, Value> + Send + Sync>,
    )]));
let row = record(&[("is_enabled", Value::Int(2))]);
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/converters/RowTypeConverter.test.ts#L21-L26)

```typescript
const converter = new RowTypeConverter({}, { flag: new OnlyOne() });
const dest = converter.convert({ flag: 1, name: "a" });
```

## Statements

Command text. A string passed to [Command](#command) `Execute` / `execute` / `execute` or `ExecuteReader` / `executeReader` / `execute_reader` is parsed with `StatementParser.Parse` / `StatementParser.parse` / `StatementParser::parse`. `Native` / `Native` / `Native` and `N` / `n` / `n` send that text to the provider engine. A statement overload renders with `GetText()` / `getText()` / `get_text()` after the provider transforms it, then runs `Native` / `Native` / `Native`.

### Classes

##### TypeScript

##### IText {#typescript-itext}

```typescript
interface IText
```

**Methods**

| Name | Description |
| --- | --- |
| [getText()](#typescript-itext-gettext) | Rendered text. |

###### getText() {#typescript-itext-gettext}

Rendered text.

**Definition**

```typescript
getText(): string
```

**Returns**

`string`

##### IStatement {#typescript-istatement}

```typescript
interface IStatement extends IText
```

**Methods**

| Name | Description |
| --- | --- |
| [getText()](#typescript-istatement-gettext) | Command text. |

###### getText() {#typescript-istatement-gettext}

Command text.

**Definition**

```typescript
getText(): string
```

**Returns**

`string`

##### C#

##### IText {#csharp-itext}

```csharp
public interface IText
```

**Methods**

| Name | Description |
| --- | --- |
| [GetText()](#csharp-itext-gettext) | Rendered text. |

###### GetText() {#csharp-itext-gettext}

Rendered text.

**Definition**

```csharp
string GetText()
```

**Returns**

`string`

##### IStatement {#csharp-istatement}

```csharp
public interface IStatement : IText
```

**Methods**

| Name | Description |
| --- | --- |
| [GetText()](#csharp-istatement-gettext) | Command text. |

###### GetText() {#csharp-istatement-gettext}

Command text.

**Definition**

```csharp
string GetText()
```

**Returns**

`string`

##### Rust

##### Text {#rust-text}

```rust
pub trait Text: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [get_text()](#rust-text-get_text) | Rendered text. |

###### get_text() {#rust-text-get_text}

Rendered text.

**Definition**

```rust
fn get_text(&self) -> String
```

**Returns**

`String`

##### Statement {#rust-statement}

```rust
pub trait Statement: Text
```

**Methods**

| Name | Description |
| --- | --- |
| [get_text()](#rust-statement-get_text) | Command text. |

###### get_text() {#rust-statement-get_text}

Command text.

**Definition**

```rust
fn get_text(&self) -> String
```

**Returns**

`String`

### Remarks

Bound values live on [Where](#where). [IStatement](#istatement) is not an [IExpression](#iexpression); [IText](#itext) is the shared `GetText()` / `getText()` / `get_text()` surface. [IWhere](#iwhere) is a clause ([IExpression](#iexpression) + [IText](#itext)), not an [IStatement](#istatement).

### See also

- [SelectStatement](#select-statement)
- [InsertStatement](#insert-statement)
- [UpdateStatement](#update-statement)
- [DeleteStatement](#delete-statement)
- [CreateTableStatement](#create-table-statement)
- [CreateIndexStatement](#create-index-statement)
- [ExecuteStoredProcedureStatement](#execute-stored-procedure-statement)
- [ReturnStatement](#return-statement)
- [Where](#where)
- [Expressions](#expressions)
- [StatementEvaluator](#statement-evaluator)

## Select Statement

`SELECT` text, or a compiled `SELECT` from [SelectStatementOptions](#selectstatementoptions): table, fields, optional [IWhere](#iwhere), joins, grouping, order, and limit.

### Classes

##### TypeScript

##### ISelectStatement {#typescript-iselectstatement}

```typescript
interface ISelectStatement extends IStatement
```

**Properties**

| Name | Description |
| --- | --- |
| where | Optional [IWhere](#typescript-iwhere) used to build this query. |

##### SelectStatement {#typescript-selectstatement}

```typescript
class SelectStatement implements ISelectStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [SelectStatement(text, where?)](#typescript-selectstatement-selectstatement-text-where) | Wraps query text and an optional [IWhere](#typescript-iwhere). |
| [SelectStatement(options)](#typescript-selectstatement-selectstatement-options) | Compiles a `SELECT` from [SelectStatementOptions](#typescript-selectstatementoptions). |

**Properties**

| Name | Description |
| --- | --- |
| where | Optional [IWhere](#typescript-iwhere) used to build this query. |

**Methods**

| Name | Description |
| --- | --- |
| [getText()](#typescript-selectstatement-gettext) | Command text. |

###### SelectStatement(text, where?) {#typescript-selectstatement-selectstatement-text-where}

Wraps query text and an optional [IWhere](#typescript-iwhere).

**Definition**

```typescript
constructor(text: string, where?: IWhere)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where` | optional [IWhere](#typescript-iwhere) |

###### SelectStatement(options) {#typescript-selectstatement-selectstatement-options}

Compiles a `SELECT` from [SelectStatementOptions](#typescript-selectstatementoptions).

**Definition**

```typescript
constructor(options: SelectStatementOptions)
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | table, fields, and optional where, joins, group, order, and limit |

###### getText() {#typescript-selectstatement-gettext}

Command text.

**Definition**

```typescript
getText(): string
```

**Returns**

`string`

##### SelectStatementOptions {#typescript-selectstatementoptions}

```typescript
type SelectStatementOptions = {
  table: string
  fields: string[]
  where?: IWhere
  joins?: Join[]
  groupBy?: string[]
  orderBy?: OrderBy[]
  limit?: number
}
```

**Properties**

| Name | Description |
| --- | --- |
| table | Table name. |
| fields | Selected columns. Empty compiles `SELECT *`. Aggregate fragments from [Aggregate](#typescript-aggregate) belong here. |
| where | Optional [IWhere](#typescript-iwhere). |
| joins | Optional [Join](#typescript-join) list. |
| groupBy | Optional `GROUP BY` columns. Empty or omitted leaves the clause off. |
| orderBy | Optional [OrderBy](#typescript-orderby) list. Empty or omitted leaves the clause off. |
| limit | Optional `LIMIT`. `0` is kept. A negative value throws. |

##### Aggregate {#typescript-aggregate}

```typescript
const Aggregate: {
  count: (column: string, alias?: string) => string
  sum: (column: string, alias?: string) => string
  min: (column: string, alias?: string) => string
  max: (column: string, alias?: string) => string
}
```

**Methods**

| Name | Description |
| --- | --- |
| [count(column, alias?)](#typescript-aggregate-count-column-alias) | `COUNT(column)`, or `COUNT(column) AS alias`. Pass `"*"` for `COUNT(*)`. |
| [sum(column, alias?)](#typescript-aggregate-sum-column-alias) | `SUM(column)`, or `SUM(column) AS alias`. |
| [min(column, alias?)](#typescript-aggregate-min-column-alias) | `MIN(column)`, or `MIN(column) AS alias`. |
| [max(column, alias?)](#typescript-aggregate-max-column-alias) | `MAX(column)`, or `MAX(column) AS alias`. |

###### count(column, alias?) {#typescript-aggregate-count-column-alias}

`COUNT(column)`, or `COUNT(column) AS alias`. Pass `"*"` for `COUNT(*)`.

**Definition**

```typescript
count(column: string, alias?: string): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column or `*` |
| `alias` | optional result name |

**Returns**

`string`

###### sum(column, alias?) {#typescript-aggregate-sum-column-alias}

`SUM(column)`, or `SUM(column) AS alias`.

**Definition**

```typescript
sum(column: string, alias?: string): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`string`

###### min(column, alias?) {#typescript-aggregate-min-column-alias}

`MIN(column)`, or `MIN(column) AS alias`.

**Definition**

```typescript
min(column: string, alias?: string): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`string`

###### max(column, alias?) {#typescript-aggregate-max-column-alias}

`MAX(column)`, or `MAX(column) AS alias`.

**Definition**

```typescript
max(column: string, alias?: string): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`string`

##### OrderBy {#typescript-orderby}

```typescript
type OrderBy = { column: string; direction: SortDirection }
```

**Methods**

| Name | Description |
| --- | --- |
| [asc(column)](#typescript-orderby-asc-column) | Sort ascending. |
| [desc(column)](#typescript-orderby-desc-column) | Sort descending. |

###### asc(column) {#typescript-orderby-asc-column}

Sort ascending.

**Definition**

```typescript
asc(column: string): OrderBy
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | sort column |

**Returns**

[OrderBy](#typescript-orderby)

###### desc(column) {#typescript-orderby-desc-column}

Sort descending.

**Definition**

```typescript
desc(column: string): OrderBy
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | sort column |

**Returns**

[OrderBy](#typescript-orderby)

##### SortDirection {#typescript-sortdirection}

```typescript
type SortDirection = "asc" | "desc"
```

##### Join {#typescript-join}

```typescript
type Join = { kind: JoinKind; table: string; left: string; right: string }
```

**Methods**

| Name | Description |
| --- | --- |
| [inner(table, left, right)](#typescript-join-inner-table-left-right) | Inner join. |
| [left(table, left, right)](#typescript-join-left-table-left-right) | Left join. |
| [right(table, left, right)](#typescript-join-right-table-left-right) | Right join. |

###### inner(table, left, right) {#typescript-join-inner-table-left-right}

Inner join.

**Definition**

```typescript
inner(table: string, left: string, right: string): Join
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#typescript-join)

###### left(table, left, right) {#typescript-join-left-table-left-right}

Left join.

**Definition**

```typescript
left(table: string, left: string, right: string): Join
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#typescript-join)

###### right(table, left, right) {#typescript-join-right-table-left-right}

Right join.

**Definition**

```typescript
right(table: string, left: string, right: string): Join
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#typescript-join)

##### JoinKind {#typescript-joinkind}

```typescript
type JoinKind = "inner" | "left" | "right"
```

##### C#

##### ISelectStatement {#csharp-iselectstatement}

```csharp
public interface ISelectStatement : IStatement
```

**Properties**

| Name | Description |
| --- | --- |
| Where | Optional [IWhere](#csharp-iwhere) used to build this query. |

##### SelectStatement {#csharp-selectstatement}

```csharp
public sealed record SelectStatement : ISelectStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [SelectStatement(text, where)](#csharp-selectstatement-selectstatement-text-where) | Wraps query text and an optional [IWhere](#csharp-iwhere). |
| [SelectStatement(options)](#csharp-selectstatement-selectstatement-options) | Compiles a `SELECT` from [SelectStatementOptions](#csharp-selectstatementoptions). |

**Properties**

| Name | Description |
| --- | --- |
| Where | Optional [IWhere](#csharp-iwhere) used to build this query. |

**Methods**

| Name | Description |
| --- | --- |
| [GetText()](#csharp-selectstatement-gettext) | Command text. |

###### SelectStatement(text, where) {#csharp-selectstatement-selectstatement-text-where}

Wraps query text and an optional [IWhere](#csharp-iwhere).

**Definition**

```csharp
public SelectStatement(string text, IWhere? where = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where` | optional [IWhere](#csharp-iwhere) |

###### SelectStatement(options) {#csharp-selectstatement-selectstatement-options}

Compiles a `SELECT` from [SelectStatementOptions](#csharp-selectstatementoptions).

**Definition**

```csharp
public SelectStatement(SelectStatementOptions options)
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | table, fields, and optional where, joins, group, order, and limit |

###### GetText() {#csharp-selectstatement-gettext}

Command text.

**Definition**

```csharp
public string GetText()
```

**Returns**

`string`

##### SelectStatementOptions {#csharp-selectstatementoptions}

```csharp
public sealed class SelectStatementOptions
```

**Properties**

| Name | Description |
| --- | --- |
| Table | Table name. |
| Fields | Selected columns. Empty compiles `SELECT *`. Aggregate fragments from [Aggregate](#csharp-aggregate) belong here. |
| Where | Optional [IWhere](#csharp-iwhere). |
| Joins | Optional [Join](#csharp-join) list. |
| GroupBy | Optional `GROUP BY` columns. Empty or omitted leaves the clause off. |
| OrderBy | Optional [OrderBy](#csharp-orderby) list. Empty or omitted leaves the clause off. |
| Limit | Optional `LIMIT`. `0` is kept. A negative value throws. |

##### Aggregate {#csharp-aggregate}

```csharp
public static class Aggregate
```

**Methods**

| Name | Description |
| --- | --- |
| [Count(column, alias)](#csharp-aggregate-count-column-alias) | `COUNT(column)`, or `COUNT(column) AS alias`. Pass `"*"` for `COUNT(*)`. |
| [Sum(column, alias)](#csharp-aggregate-sum-column-alias) | `SUM(column)`, or `SUM(column) AS alias`. |
| [Min(column, alias)](#csharp-aggregate-min-column-alias) | `MIN(column)`, or `MIN(column) AS alias`. |
| [Max(column, alias)](#csharp-aggregate-max-column-alias) | `MAX(column)`, or `MAX(column) AS alias`. |

###### Count(column, alias) {#csharp-aggregate-count-column-alias}

`COUNT(column)`, or `COUNT(column) AS alias`. Pass `"*"` for `COUNT(*)`.

**Definition**

```csharp
public static string Count(string column, string? alias = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column or `*` |
| `alias` | optional result name |

**Returns**

`string`

###### Sum(column, alias) {#csharp-aggregate-sum-column-alias}

`SUM(column)`, or `SUM(column) AS alias`.

**Definition**

```csharp
public static string Sum(string column, string? alias = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`string`

###### Min(column, alias) {#csharp-aggregate-min-column-alias}

`MIN(column)`, or `MIN(column) AS alias`.

**Definition**

```csharp
public static string Min(string column, string? alias = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`string`

###### Max(column, alias) {#csharp-aggregate-max-column-alias}

`MAX(column)`, or `MAX(column) AS alias`.

**Definition**

```csharp
public static string Max(string column, string? alias = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`string`

##### OrderBy {#csharp-orderby}

```csharp
public sealed record OrderBy(string Column, SortDirection Direction)
```

**Methods**

| Name | Description |
| --- | --- |
| [Asc(column)](#csharp-orderby-asc-column) | Sort ascending. |
| [Desc(column)](#csharp-orderby-desc-column) | Sort descending. |

###### Asc(column) {#csharp-orderby-asc-column}

Sort ascending.

**Definition**

```csharp
public static OrderBy Asc(string column)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | sort column |

**Returns**

[OrderBy](#csharp-orderby)

###### Desc(column) {#csharp-orderby-desc-column}

Sort descending.

**Definition**

```csharp
public static OrderBy Desc(string column)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | sort column |

**Returns**

[OrderBy](#csharp-orderby)

##### SortDirection {#csharp-sortdirection}

```csharp
public enum SortDirection
```

##### Join {#csharp-join}

```csharp
public sealed record Join(JoinKind Kind, string Table, string LeftColumn, string RightColumn)
```

**Methods**

| Name | Description |
| --- | --- |
| [Inner(table, left, right)](#csharp-join-inner-table-left-right) | Inner join. |
| [Left(table, left, right)](#csharp-join-left-table-left-right) | Left join. |
| [Right(table, left, right)](#csharp-join-right-table-left-right) | Right join. |

###### Inner(table, left, right) {#csharp-join-inner-table-left-right}

Inner join.

**Definition**

```csharp
public static Join Inner(string table, string left, string right)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#csharp-join)

###### Left(table, left, right) {#csharp-join-left-table-left-right}

Left join.

**Definition**

```csharp
public static Join Left(string table, string left, string right)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#csharp-join)

###### Right(table, left, right) {#csharp-join-right-table-left-right}

Right join.

**Definition**

```csharp
public static Join Right(string table, string left, string right)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#csharp-join)

##### JoinKind {#csharp-joinkind}

```csharp
public enum JoinKind
```

##### Rust

##### ISelectStatement {#rust-iselectstatement}

```rust
pub trait ISelectStatement: Statement
```

**Methods**

| Name | Description |
| --- | --- |
| [where_clause()](#rust-iselectstatement-where_clause) | Optional [IWhere](#rust-iwhere) used to build this query. |

###### where_clause() {#rust-iselectstatement-where_clause}

Optional [IWhere](#rust-iwhere) used to build this query.

**Definition**

```rust
fn where_clause(&self) -> Option<&dyn IWhere>
```

**Returns**

`Option<&dyn `[IWhere](#rust-iwhere)`>`

##### SelectStatement {#rust-selectstatement}

```rust
pub struct SelectStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [text(text)](#rust-selectstatement-text-text) | Wraps query text. |
| [with_where(text, where_clause)](#rust-selectstatement-with_where-text-where_clause) | Wraps query text and an [IWhere](#rust-iwhere). |
| [new(options)](#rust-selectstatement-new-options) | Compiles a `SELECT` from [SelectStatementOptions](#rust-selectstatementoptions). |

**Methods**

| Name | Description |
| --- | --- |
| [get_text()](#rust-selectstatement-get_text) | Command text. |
| [where_clause()](#rust-selectstatement-where_clause) | Optional [IWhere](#rust-iwhere) used to build this query. |

###### text(text) {#rust-selectstatement-text-text}

Wraps query text.

**Definition**

```rust
pub fn text(text: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

**Returns**

[SelectStatement](#rust-selectstatement)

###### with_where(text, where_clause) {#rust-selectstatement-with_where-text-where_clause}

Wraps query text and an [IWhere](#rust-iwhere).

**Definition**

```rust
pub fn with_where(text: impl Into<String>, where_clause: impl IWhere + 'static) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where_clause` | [IWhere](#rust-iwhere) |

**Returns**

[SelectStatement](#rust-selectstatement)

###### new(options) {#rust-selectstatement-new-options}

Compiles a `SELECT` from [SelectStatementOptions](#rust-selectstatementoptions).

**Definition**

```rust
pub fn new(options: SelectStatementOptions<'_>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | table, fields, and optional where, joins, group, order, and limit |

**Returns**

[SelectStatement](#rust-selectstatement)

###### get_text() {#rust-selectstatement-get_text}

Command text.

**Definition**

```rust
fn get_text(&self) -> String
```

**Returns**

`String`

###### where_clause() {#rust-selectstatement-where_clause}

Optional [IWhere](#rust-iwhere) used to build this query.

**Definition**

```rust
fn where_clause(&self) -> Option<&dyn IWhere>
```

**Returns**

`Option<&dyn `[IWhere](#rust-iwhere)`>`

##### SelectStatementOptions {#rust-selectstatementoptions}

```rust
pub struct SelectStatementOptions<'a>
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(table, fields)](#rust-selectstatementoptions-new-table-fields) | Table and fields, with where, joins, group, order, and limit unset. |

**Properties**

| Name | Description |
| --- | --- |
| table | Table name. |
| fields | Selected columns. Empty compiles `SELECT *`. Aggregate fragments from [Aggregate](#rust-aggregate) belong here. |
| where_clause | Optional [IWhere](#rust-iwhere). |
| joins | [Join](#rust-join) list. Empty leaves the clause off. |
| group_by | `GROUP BY` columns. Empty leaves the clause off. |
| order_by | [OrderBy](#rust-orderby) list. Empty leaves the clause off. |
| limit | Optional `LIMIT`. `Some(0)` is kept. A negative value panics. |

###### new(table, fields) {#rust-selectstatementoptions-new-table-fields}

Table and fields, with where, joins, group, order, and limit unset.

**Definition**

```rust
pub fn new(table: &'a str, fields: &'a [&'a str]) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | selected columns (`*` when empty) |

**Returns**

[SelectStatementOptions](#rust-selectstatementoptions)

##### Aggregate {#rust-aggregate}

```rust
pub struct Aggregate
```

**Methods**

| Name | Description |
| --- | --- |
| [count(column, alias)](#rust-aggregate-count-column-alias) | `COUNT(column)`, or `COUNT(column) AS alias`. Pass `"*"` for `COUNT(*)`. |
| [sum(column, alias)](#rust-aggregate-sum-column-alias) | `SUM(column)`, or `SUM(column) AS alias`. |
| [min(column, alias)](#rust-aggregate-min-column-alias) | `MIN(column)`, or `MIN(column) AS alias`. |
| [max(column, alias)](#rust-aggregate-max-column-alias) | `MAX(column)`, or `MAX(column) AS alias`. |

###### count(column, alias) {#rust-aggregate-count-column-alias}

`COUNT(column)`, or `COUNT(column) AS alias`. Pass `"*"` for `COUNT(*)`.

**Definition**

```rust
pub fn count(column: &str, alias: Option<&str>) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column or `*` |
| `alias` | optional result name |

**Returns**

`String`

###### sum(column, alias) {#rust-aggregate-sum-column-alias}

`SUM(column)`, or `SUM(column) AS alias`.

**Definition**

```rust
pub fn sum(column: &str, alias: Option<&str>) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`String`

###### min(column, alias) {#rust-aggregate-min-column-alias}

`MIN(column)`, or `MIN(column) AS alias`.

**Definition**

```rust
pub fn min(column: &str, alias: Option<&str>) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`String`

###### max(column, alias) {#rust-aggregate-max-column-alias}

`MAX(column)`, or `MAX(column) AS alias`.

**Definition**

```rust
pub fn max(column: &str, alias: Option<&str>) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column |
| `alias` | optional result name |

**Returns**

`String`

##### OrderBy {#rust-orderby}

```rust
pub struct OrderBy
```

**Methods**

| Name | Description |
| --- | --- |
| [asc(column)](#rust-orderby-asc-column) | Sort ascending. |
| [desc(column)](#rust-orderby-desc-column) | Sort descending. |

###### asc(column) {#rust-orderby-asc-column}

Sort ascending.

**Definition**

```rust
pub fn asc(column: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | sort column |

**Returns**

[OrderBy](#rust-orderby)

###### desc(column) {#rust-orderby-desc-column}

Sort descending.

**Definition**

```rust
pub fn desc(column: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | sort column |

**Returns**

[OrderBy](#rust-orderby)

##### SortDirection {#rust-sortdirection}

```rust
pub enum SortDirection
```

##### Join {#rust-join}

```rust
pub struct Join
```

**Methods**

| Name | Description |
| --- | --- |
| [inner(table, left, right)](#rust-join-inner-table-left-right) | Inner join. |
| [left(table, left, right)](#rust-join-left-table-left-right) | Left join. |
| [right(table, left, right)](#rust-join-right-table-left-right) | Right join. |

###### inner(table, left, right) {#rust-join-inner-table-left-right}

Inner join.

**Definition**

```rust
pub fn inner(table: impl Into<String>, left: impl Into<String>, right: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#rust-join)

###### left(table, left, right) {#rust-join-left-table-left-right}

Left join.

**Definition**

```rust
pub fn left(table: impl Into<String>, left: impl Into<String>, right: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#rust-join)

###### right(table, left, right) {#rust-join-right-table-left-right}

Right join.

**Definition**

```rust
pub fn right(table: impl Into<String>, left: impl Into<String>, right: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | joined table |
| `left` | left column |
| `right` | right column |

**Returns**

[Join](#rust-join)

##### JoinKind {#rust-joinkind}

```rust
pub enum JoinKind
```

### Remarks

[Command](#command) uses `GetText()` / `getText()` / `get_text()` only. Keep the [Where](#where) when the caller needs bound parameters. Empty `fields` compiles `SELECT *`. Clauses append as `GROUP BY`, `ORDER BY`, then `LIMIT`. An empty group or order list is omitted. `Aggregate` returns a select-list fragment such as `COUNT(*) AS n`.

### Examples

#### Wrap query text and an optional where

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L39-L44)

```csharp
var select = new SelectStatement("SELECT 1");
var byId = new WhereText("id = 1");
var withWhere = new SelectStatement("SELECT 1", byId);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1187-L1190)

```rust
    .not_null(NotNull::new(Some("nn_id"))),
Column::new("tenant_id", "INTEGER"),
Column::new("user_id", "INTEGER"),
Column::new("qty", "INTEGER"),
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L42-L44)

```typescript
const select = new SelectStatement("SELECT 1");
const where = new WhereText("id = 1");
const withWhere = new SelectStatement("SELECT 1", where);
```

#### Compile a SELECT

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L127-L129)

```csharp
var query = new SelectStatement(new() { Table = "users", Fields = ["id", "name"], Where = where });
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id AND age > $age
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L926-L933)

```rust
    Where::new(Operator::Equals {
        left: Box::new(Expression::Literal(Literal::new(true))),
        right: Box::new(Expression::Literal(Literal::new(false))),
    })
    .get_text(),
    "true = false"
);
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id AND age > $age
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L198-L199)

```typescript
const where = new Where(new Operator.Equals("id", 1));
const query = new SelectStatement({ table: "users", fields: ["id", "name"], where });
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id
```

#### Group, aggregate, order, and limit

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L156-L165)

```csharp
var grouped = new SelectStatement(new()
{
    Table = "orders",
    Fields = ["status", Aggregate.Count("*", "n"), Aggregate.Sum("amount", "total")],
    Where = new Where(new Operator.Equals("status", "open")),
    Joins = [Join.Inner("customers", "orders.customer_id", "customers.id")],
    GroupBy = ["status"],
    OrderBy = [OrderBy.Desc("total"), OrderBy.Asc("status")],
    Limit = 10,
});
```

**After: grouped select**

```
SELECT status, COUNT(*) AS n, SUM(amount) AS total FROM orders INNER JOIN customers ON orders.customer_id = customers.id WHERE status = $status GROUP BY status ORDER BY total DESC, status ASC LIMIT 10
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L977-L991)

```rust
.get_text()
.contains('='));
assert!(Where::new(Operator::Equals {
    left: Box::new(Expression::Literal(Literal::new(Value::DateTime(
        chrono::Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap()
    )))),
    right: Box::new(Expression::Literal(Literal::new(Value::DateTime(
        chrono::Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap()
    )))),
})
.get_text()
.contains('='));
assert!(Where::new(Operator::Equals {
    left: Box::new(Expression::Literal(Literal::new(Value::Binary(vec![1])))),
    right: Box::new(Expression::Literal(Literal::new(Value::Binary(vec![1])))),
```

**After: grouped select**

```
SELECT status, COUNT(*) AS n, SUM(amount) AS total FROM orders INNER JOIN customers ON orders.customer_id = customers.id WHERE status = $status GROUP BY status ORDER BY total DESC, status ASC LIMIT 10
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L232-L240)

```typescript
  "SELECT users.id FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN payments ON orders.id = payments.order_id RIGHT JOIN profiles ON users.id = profiles.user_id WHERE users.id = $users.id",
);
```

**After: grouped select**

```
SELECT status, COUNT(*) AS n, SUM(amount) AS total FROM orders INNER JOIN customers ON orders.customer_id = customers.id WHERE status = $status GROUP BY status ORDER BY total DESC, status ASC LIMIT 10
```

#### Group rows

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L167-L167)

```csharp
var byStatus = new SelectStatement(new() { Table = "users", Fields = ["id"], GroupBy = ["id"] });
```

**After: group by**

```
SELECT id FROM users GROUP BY id
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L996-L999)

```rust
    left: Box::new(Expression::Literal(Literal::new(1_i64))),
    right: Box::new(Expression::Literal(Literal::new(2_i64))),
})
.parameters()
```

**After: group by**

```
SELECT id FROM users GROUP BY id
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L244-L244)

```typescript
limit: 10,
```

**After: group by**

```
SELECT id FROM users GROUP BY id
```

#### Aggregate without grouping

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L169-L169)

```csharp
var bounds = new SelectStatement(new() { Table = "users", Fields = [Aggregate.Min("n"), Aggregate.Max("n")] });
```

**After: min and max**

```
SELECT MIN(n), MAX(n) FROM users
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1001-L1004)

```rust

```

**After: min and max**

```
SELECT MIN(n), MAX(n) FROM users
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L246-L246)

```typescript

```

**After: min and max**

```
SELECT MIN(n), MAX(n) FROM users
```

#### Order rows

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L171-L171)

```csharp
var ordered = new SelectStatement(new() { Table = "users", Fields = ["id"], OrderBy = [OrderBy.Asc("id"), OrderBy.Desc("name")] });
```

**After: order by**

```
SELECT id FROM users ORDER BY id ASC, name DESC
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1006-L1009)

```rust
    .get_text(),
    "(NOT (id = $id)) AND age = $age"
);
```

**After: order by**

```
SELECT id FROM users ORDER BY id ASC, name DESC
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L248-L252)

```typescript
);
const byStatus = new SelectStatement({ table: "users", fields: ["id"], groupBy: ["id"] });
```

**After: order by**

```
SELECT id FROM users ORDER BY id ASC, name DESC
```

#### Limit rows

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L173-L173)

```csharp
var limited = new SelectStatement(new() { Table = "users", Fields = ["id"], Limit = 10 });
```

**After: limit**

```
SELECT id FROM users LIMIT 10
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1014-L1017)

```rust
    .parameters()
    .is_empty());
let in_ids = Where::new(Operator::in_list("id", vec![Value::Int(1), Value::Int(2)]));
```

**After: limit**

```
SELECT id FROM users LIMIT 10
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L254-L254)

```typescript
table: "users",
```

**After: limit**

```
SELECT id FROM users LIMIT 10
```

## Insert Statement

`INSERT` text, or a compiled `INSERT` from a table and fields.

### Classes

##### TypeScript

##### IInsertStatement {#typescript-iinsertstatement}

```typescript
interface IInsertStatement extends IStatement {}
```

##### InsertStatement {#typescript-insertstatement}

```typescript
class InsertStatement implements IInsertStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [InsertStatement(text)](#typescript-insertstatement-insertstatement-text) | Wraps insert text. |
| [InsertStatement(table, fields)](#typescript-insertstatement-insertstatement-table-fields) | Compiles an `INSERT`. |

**Methods**

| Name | Description |
| --- | --- |
| [getText()](#typescript-insertstatement-gettext) | Command text. |

###### InsertStatement(text) {#typescript-insertstatement-insertstatement-text}

Wraps insert text.

**Definition**

```typescript
constructor(text: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### InsertStatement(table, fields) {#typescript-insertstatement-insertstatement-table-fields}

Compiles an `INSERT`.

**Definition**

```typescript
constructor(table: string, fields: Field[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | [Field](#typescript-field) columns and bound values |

###### getText() {#typescript-insertstatement-gettext}

Command text.

**Definition**

```typescript
getText(): string
```

**Returns**

`string`

##### Field {#typescript-field}

```typescript
interface Field
```

**Properties**

| Name | Description |
| --- | --- |
| name | Column name. |
| value | Bound [Scalar](#typescript-scalar). |

##### C#

##### IInsertStatement {#csharp-iinsertstatement}

```csharp
public interface IInsertStatement : IStatement
```

##### InsertStatement {#csharp-insertstatement}

```csharp
public sealed record InsertStatement : IInsertStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [InsertStatement(text)](#csharp-insertstatement-insertstatement-text) | Wraps insert text. |
| [InsertStatement(table, fields)](#csharp-insertstatement-insertstatement-table-fields) | Compiles an `INSERT`. |

**Methods**

| Name | Description |
| --- | --- |
| [GetText()](#csharp-insertstatement-gettext) | Command text. |

###### InsertStatement(text) {#csharp-insertstatement-insertstatement-text}

Wraps insert text.

**Definition**

```csharp
public InsertStatement(string text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### InsertStatement(table, fields) {#csharp-insertstatement-insertstatement-table-fields}

Compiles an `INSERT`.

**Definition**

```csharp
public InsertStatement(string table, IReadOnlyList<Parameter> fields)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | [Parameter](#csharp-parameter) columns and bound values |

###### GetText() {#csharp-insertstatement-gettext}

Command text.

**Definition**

```csharp
public string GetText()
```

**Returns**

`string`

##### Rust

##### InsertStatement {#rust-insertstatement}

```rust
pub struct InsertStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [text(text)](#rust-insertstatement-text-text) | Wraps insert text. |
| [new(table, fields)](#rust-insertstatement-new-table-fields) | Compiles an `INSERT`. |

**Methods**

| Name | Description |
| --- | --- |
| [get_text()](#rust-insertstatement-get_text) | Command text. |

###### text(text) {#rust-insertstatement-text-text}

Wraps insert text.

**Definition**

```rust
pub fn text(text: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

**Returns**

[InsertStatement](#rust-insertstatement)

###### new(table, fields) {#rust-insertstatement-new-table-fields}

Compiles an `INSERT`.

**Definition**

```rust
pub fn new(table: impl AsRef<str>, fields: &[Parameter]) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | [Parameter](#rust-parameter) columns and bound values |

**Returns**

[InsertStatement](#rust-insertstatement)

###### get_text() {#rust-insertstatement-get_text}

Command text.

**Definition**

```rust
fn get_text(&self) -> String
```

**Returns**

`String`

### Remarks

Compiled form is `INSERT INTO table (cols) VALUES ($cols)`. [Command](#command) uses `GetText()` / `getText()` / `get_text()` only.

### Examples

#### Compile an INSERT

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L150-L151)

```csharp
Join.Inner("orders", "users.id", "orders.user_id"),
Join.Left("payments", "orders.id", "payments.order_id"),
```

**After: insert**

```
INSERT INTO users (id, name) VALUES ($id, $name)
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L985-L989)

```rust
    )))),
})
.get_text()
.contains('='));
assert!(Where::new(Operator::Equals {
```

**After: insert**

```
INSERT INTO users (id, name) VALUES ($id, $name)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L222-L225)

```typescript
table: "users",
fields: ["users.id"],
where: new Where(new Operator.Equals("users.id", 1)),
joins: [
```

**After: insert**

```
INSERT INTO users (id, name) VALUES ($id, $name)
```

## Update Statement

`UPDATE` text, or a compiled `UPDATE` from a table, fields, and optional [IWhere](#iwhere).

### Classes

##### TypeScript

##### IUpdateStatement {#typescript-iupdatestatement}

```typescript
interface IUpdateStatement extends IStatement
```

**Properties**

| Name | Description |
| --- | --- |
| where | Optional [IWhere](#typescript-iwhere) used to build this update. |

##### UpdateStatement {#typescript-updatestatement}

```typescript
class UpdateStatement implements IUpdateStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [UpdateStatement(text, where?)](#typescript-updatestatement-updatestatement-text-where) | Wraps update text and an optional [IWhere](#typescript-iwhere). |
| [UpdateStatement(table, fields, where?)](#typescript-updatestatement-updatestatement-table-fields-where) | Compiles an `UPDATE`. |

**Properties**

| Name | Description |
| --- | --- |
| where | Optional [IWhere](#typescript-iwhere) used to build this update. |

**Methods**

| Name | Description |
| --- | --- |
| [getText()](#typescript-updatestatement-gettext) | Command text. |

###### UpdateStatement(text, where?) {#typescript-updatestatement-updatestatement-text-where}

Wraps update text and an optional [IWhere](#typescript-iwhere).

**Definition**

```typescript
constructor(text: string, where?: IWhere)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where` | optional [IWhere](#typescript-iwhere) |

###### UpdateStatement(table, fields, where?) {#typescript-updatestatement-updatestatement-table-fields-where}

Compiles an `UPDATE`.

**Definition**

```typescript
constructor(table: string, fields: Field[], where?: IWhere)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | [Field](#typescript-field) columns and bound values |
| `where` | optional [IWhere](#typescript-iwhere) |

###### getText() {#typescript-updatestatement-gettext}

Command text.

**Definition**

```typescript
getText(): string
```

**Returns**

`string`

##### C#

##### IUpdateStatement {#csharp-iupdatestatement}

```csharp
public interface IUpdateStatement : IStatement
```

**Properties**

| Name | Description |
| --- | --- |
| Where | Optional [IWhere](#csharp-iwhere) used to build this update. |

##### UpdateStatement {#csharp-updatestatement}

```csharp
public sealed record UpdateStatement : IUpdateStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [UpdateStatement(text, where)](#csharp-updatestatement-updatestatement-text-where) | Wraps update text and an optional [IWhere](#csharp-iwhere). |
| [UpdateStatement(table, fields, where)](#csharp-updatestatement-updatestatement-table-fields-where) | Compiles an `UPDATE`. |

**Properties**

| Name | Description |
| --- | --- |
| Where | Optional [IWhere](#csharp-iwhere) used to build this update. |

**Methods**

| Name | Description |
| --- | --- |
| [GetText()](#csharp-updatestatement-gettext) | Command text. |

###### UpdateStatement(text, where) {#csharp-updatestatement-updatestatement-text-where}

Wraps update text and an optional [IWhere](#csharp-iwhere).

**Definition**

```csharp
public UpdateStatement(string text, IWhere? where = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where` | optional [IWhere](#csharp-iwhere) |

###### UpdateStatement(table, fields, where) {#csharp-updatestatement-updatestatement-table-fields-where}

Compiles an `UPDATE`.

**Definition**

```csharp
public UpdateStatement(string table, IReadOnlyList<Parameter> fields, IWhere? where = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | [Parameter](#csharp-parameter) columns and bound values |
| `where` | optional [IWhere](#csharp-iwhere) |

###### GetText() {#csharp-updatestatement-gettext}

Command text.

**Definition**

```csharp
public string GetText()
```

**Returns**

`string`

##### Rust

##### UpdateStatement {#rust-updatestatement}

```rust
pub struct UpdateStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [text(text, where_clause)](#rust-updatestatement-text-text-where_clause) | Wraps update text and an optional [IWhere](#rust-iwhere). |
| [new(table, fields, where_clause)](#rust-updatestatement-new-table-fields-where_clause) | Compiles an `UPDATE`. |

**Methods**

| Name | Description |
| --- | --- |
| [get_text()](#rust-updatestatement-get_text) | Command text. |
| [where_clause()](#rust-updatestatement-where_clause) | Optional [IWhere](#rust-iwhere) used to build this update. |

###### text(text, where_clause) {#rust-updatestatement-text-text-where_clause}

Wraps update text and an optional [IWhere](#rust-iwhere).

**Definition**

```rust
pub fn text(text: impl Into<String>, where_clause: Option<Box<dyn IWhere>>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where_clause` | optional [IWhere](#rust-iwhere) |

**Returns**

[UpdateStatement](#rust-updatestatement)

###### new(table, fields, where_clause) {#rust-updatestatement-new-table-fields-where_clause}

Compiles an `UPDATE`.

**Definition**

```rust
pub fn new(table: impl AsRef<str>, fields: &[Parameter], where_clause: Option<&dyn IWhere>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | [Parameter](#rust-parameter) columns and bound values |
| `where_clause` | optional [IWhere](#rust-iwhere) |

**Returns**

[UpdateStatement](#rust-updatestatement)

###### get_text() {#rust-updatestatement-get_text}

Command text.

**Definition**

```rust
fn get_text(&self) -> String
```

**Returns**

`String`

###### where_clause() {#rust-updatestatement-where_clause}

Optional [IWhere](#rust-iwhere) used to build this update.

**Definition**

```rust
fn where_clause(&self) -> Option<&dyn IWhere>
```

**Returns**

`Option<&dyn `[IWhere](#rust-iwhere)`>`

### Remarks

Compiled form is `UPDATE table SET col = $col` plus `WHERE` when an [IWhere](#iwhere) is present. Keep the [Where](#where) when the caller needs bound parameters.

### Examples

#### Compile an UPDATE

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L152-L154)

```csharp
        Join.Right("profiles", "users.id", "profiles.user_id"),
    ],
}).GetText());
```

**After: update**

```
UPDATE users SET id = $id, name = $name WHERE id = $id AND age > $age
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L980-L985)

```rust
left: Box::new(Expression::Literal(Literal::new(Value::DateTime(
    chrono::Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap()
)))),
right: Box::new(Expression::Literal(Literal::new(Value::DateTime(
    chrono::Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap()
)))),
```

**After: update**

```
UPDATE users SET id = $id, name = $name WHERE id = $id AND age > $age
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L226-L230)

```typescript
    Join.inner("orders", "users.id", "orders.user_id"),
    Join.left("payments", "orders.id", "payments.order_id"),
    Join.right("profiles", "users.id", "profiles.user_id"),
  ],
}).getText(),
```

**After: update**

```
UPDATE users SET id = $id, name = $name WHERE id = $id
```

## Delete Statement

Compiled `DELETE` from a table and optional [IWhere](#iwhere), or raw delete text.

### Classes

##### TypeScript

##### IDeleteStatement {#typescript-ideletestatement}

```typescript
interface IDeleteStatement extends IStatement
```

**Properties**

| Name | Description |
| --- | --- |
| where | Optional [IWhere](#typescript-iwhere) used to build this delete. |

##### DeleteStatement {#typescript-deletestatement}

```typescript
class DeleteStatement implements IDeleteStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [DeleteStatement(table, where?)](#typescript-deletestatement-deletestatement-table-where) | Compiles a `DELETE`. |

**Properties**

| Name | Description |
| --- | --- |
| where | Optional [IWhere](#typescript-iwhere) used to build this delete. |

**Methods**

| Name | Description |
| --- | --- |
| [raw(text, where?)](#typescript-deletestatement-raw-text-where) | Wraps delete text and an optional [IWhere](#typescript-iwhere). |
| [getText()](#typescript-deletestatement-gettext) | Command text. |

###### DeleteStatement(table, where?) {#typescript-deletestatement-deletestatement-table-where}

Compiles a `DELETE`.

**Definition**

```typescript
constructor(table: string, where?: IWhere)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `where` | optional [IWhere](#typescript-iwhere) |

###### raw(text, where?) {#typescript-deletestatement-raw-text-where}

Wraps delete text and an optional [IWhere](#typescript-iwhere).

**Definition**

```typescript
static raw(text: string, where?: IWhere): DeleteStatement
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where` | optional [IWhere](#typescript-iwhere) |

**Returns**

[DeleteStatement](#typescript-deletestatement)

###### getText() {#typescript-deletestatement-gettext}

Command text.

**Definition**

```typescript
getText(): string
```

**Returns**

`string`

##### C#

##### IDeleteStatement {#csharp-ideletestatement}

```csharp
public interface IDeleteStatement : IStatement
```

**Properties**

| Name | Description |
| --- | --- |
| Where | Optional [IWhere](#csharp-iwhere) used to build this delete. |

##### DeleteStatement {#csharp-deletestatement}

```csharp
public sealed record DeleteStatement : IDeleteStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [DeleteStatement(table, where)](#csharp-deletestatement-deletestatement-table-where) | Compiles a `DELETE`. |

**Properties**

| Name | Description |
| --- | --- |
| Where | Optional [IWhere](#csharp-iwhere) used to build this delete. |

**Methods**

| Name | Description |
| --- | --- |
| [Raw(text, where)](#csharp-deletestatement-raw-text-where) | Wraps delete text and an optional [IWhere](#csharp-iwhere). |
| [GetText()](#csharp-deletestatement-gettext) | Command text. |

###### DeleteStatement(table, where) {#csharp-deletestatement-deletestatement-table-where}

Compiles a `DELETE`.

**Definition**

```csharp
public DeleteStatement(string table, IWhere? where = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `where` | optional [IWhere](#csharp-iwhere) |

###### Raw(text, where) {#csharp-deletestatement-raw-text-where}

Wraps delete text and an optional [IWhere](#csharp-iwhere).

**Definition**

```csharp
public static DeleteStatement Raw(string text, IWhere? where = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where` | optional [IWhere](#csharp-iwhere) |

**Returns**

[DeleteStatement](#csharp-deletestatement)

###### GetText() {#csharp-deletestatement-gettext}

Command text.

**Definition**

```csharp
public string GetText()
```

**Returns**

`string`

##### Rust

##### DeleteStatement {#rust-deletestatement}

```rust
pub struct DeleteStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(table, where_clause)](#rust-deletestatement-new-table-where_clause) | Compiles a `DELETE`. |
| [raw(text, where_clause)](#rust-deletestatement-raw-text-where_clause) | Wraps delete text and an optional [IWhere](#rust-iwhere). |

**Methods**

| Name | Description |
| --- | --- |
| [get_text()](#rust-deletestatement-get_text) | Command text. |
| [where_clause()](#rust-deletestatement-where_clause) | Optional [IWhere](#rust-iwhere) used to build this delete. |

###### new(table, where_clause) {#rust-deletestatement-new-table-where_clause}

Compiles a `DELETE`.

**Definition**

```rust
pub fn new(table: impl AsRef<str>, where_clause: Option<&dyn IWhere>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `where_clause` | optional [IWhere](#rust-iwhere) |

**Returns**

[DeleteStatement](#rust-deletestatement)

###### raw(text, where_clause) {#rust-deletestatement-raw-text-where_clause}

Wraps delete text and an optional [IWhere](#rust-iwhere).

**Definition**

```rust
pub fn raw(text: impl Into<String>, where_clause: Option<Box<dyn IWhere>>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where_clause` | optional [IWhere](#rust-iwhere) |

**Returns**

[DeleteStatement](#rust-deletestatement)

###### get_text() {#rust-deletestatement-get_text}

Command text.

**Definition**

```rust
fn get_text(&self) -> String
```

**Returns**

`String`

###### where_clause() {#rust-deletestatement-where_clause}

Optional [IWhere](#rust-iwhere) used to build this delete.

**Definition**

```rust
fn where_clause(&self) -> Option<&dyn IWhere>
```

**Returns**

`Option<&dyn `[IWhere](#rust-iwhere)`>`

### Remarks

Compiled form is `DELETE FROM table` plus `WHERE` when an [IWhere](#iwhere) is present. `Raw` / `raw` / `raw` wraps command text without compiling. Keep the [Where](#where) when the caller needs bound parameters.

### Examples

#### Compile a DELETE

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L157-L159)

```csharp
{
    Table = "orders",
    Fields = ["status", Aggregate.Count("*", "n"), Aggregate.Sum("amount", "total")],
```

**After: delete**

```
DELETE FROM users WHERE id = $id AND age > $age
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L989-L994)

```rust
assert!(Where::new(Operator::Equals {
    left: Box::new(Expression::Literal(Literal::new(Value::Binary(vec![1])))),
    right: Box::new(Expression::Literal(Literal::new(Value::Binary(vec![1])))),
})
.get_text()
.contains('='));
```

**After: delete**

```
DELETE FROM users WHERE id = $id AND age > $age
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L234-L236)

```typescript

```

**After: delete**

```
DELETE FROM users WHERE id = $id
```

## Create Table Statement

`CREATE TABLE` text, or a compiled `CREATE TABLE` from a table, columns, and optional named table constraints.

### Classes

##### TypeScript

##### ICreateTableStatement {#typescript-icreatetablestatement}

```typescript
interface ICreateTableStatement extends IStatement {}
```

##### CreateTableStatement {#typescript-createtablestatement}

```typescript
class CreateTableStatement implements ICreateTableStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [CreateTableStatement(text)](#typescript-createtablestatement-createtablestatement-text) | Wraps create-table text. |
| [CreateTableStatement(table, columns, constraints?)](#typescript-createtablestatement-createtablestatement-table-columns-constraints) | Compiles a `CREATE TABLE`. |

**Properties**

| Name | Description |
| --- | --- |
| table | Table name when compiled from columns. Absent for wrapped text. |
| columns | Columns on a compiled table. Empty for wrapped text. |
| constraints | Named table constraints on a compiled table. Empty for wrapped text. |

**Methods**

| Name | Description |
| --- | --- |
| [getText()](#typescript-createtablestatement-gettext) | Command text. |

###### CreateTableStatement(text) {#typescript-createtablestatement-createtablestatement-text}

Wraps create-table text.

**Definition**

```typescript
constructor(text: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### CreateTableStatement(table, columns, constraints?) {#typescript-createtablestatement-createtablestatement-table-columns-constraints}

Compiles a `CREATE TABLE`.

**Definition**

```typescript
constructor(table: string, columns: Column[], constraints?: ITableConstraint[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `columns` | [Column](#typescript-column) names, types, optional [ColumnDefault](#typescript-columndefault) and [NotNull](#typescript-notnull) |
| `constraints` | optional named [ITableConstraint](#typescript-itableconstraint) list |

###### getText() {#typescript-createtablestatement-gettext}

Command text.

**Definition**

```typescript
getText(): string
```

**Returns**

`string`

##### Column {#typescript-column}

```typescript
interface Column
```

**Properties**

| Name | Description |
| --- | --- |
| name | Column name. |
| type | SQL type name. |
| default | Optional [ColumnDefault](#typescript-columndefault). |
| notNull | Optional [NotNull](#typescript-notnull). |
| identity | When true, MySQL and Postgres commands render an identity clause. `getText()` leaves it off. |

##### ColumnDefault {#typescript-columndefault}

```typescript
class ColumnDefault
```

**Constructor**

| Name | Description |
| --- | --- |
| [ColumnDefault(value, name?)](#typescript-columndefault-columndefault-value-name) | Default value, optionally named. |

**Properties**

| Name | Description |
| --- | --- |
| value | Raw SQL default fragment. |
| name | Optional constraint name. |

###### ColumnDefault(value, name?) {#typescript-columndefault-columndefault-value-name}

Default value, optionally named.

**Definition**

```typescript
constructor(value: string, name?: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | raw SQL default fragment |
| `name` | optional constraint name |

##### NotNull {#typescript-notnull}

```typescript
class NotNull
```

**Constructor**

| Name | Description |
| --- | --- |
| [NotNull(name?)](#typescript-notnull-notnull-name) | `NOT NULL`, optionally named. |

**Properties**

| Name | Description |
| --- | --- |
| name | Optional constraint name. |

###### NotNull(name?) {#typescript-notnull-notnull-name}

`NOT NULL`, optionally named.

**Definition**

```typescript
constructor(name?: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | optional constraint name |

##### ITableConstraint {#typescript-itableconstraint}

```typescript
interface ITableConstraint
```

**Properties**

| Name | Description |
| --- | --- |
| kind | Constraint kind. |
| name | Constraint name. |

##### PrimaryKey {#typescript-primarykey}

```typescript
class PrimaryKey implements ITableConstraint
```

**Constructor**

| Name | Description |
| --- | --- |
| [PrimaryKey(name, columns)](#typescript-primarykey-primarykey-name-columns) | Named primary key. Multiple columns are a composite key. |

###### PrimaryKey(name, columns) {#typescript-primarykey-primarykey-name-columns}

Named primary key. Multiple columns are a composite key.

**Definition**

```typescript
constructor(name: string, columns: string[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | constraint name |
| `columns` | key columns |

##### ForeignKey {#typescript-foreignkey}

```typescript
class ForeignKey implements ITableConstraint
```

**Constructor**

| Name | Description |
| --- | --- |
| [ForeignKey(name, columns, referencesTable, referencesColumns, onDelete?, onUpdate?)](#typescript-foreignkey-foreignkey-name-columns-referencestable-referencescolumns-ondelete-onupdate) | Named foreign key. |

###### ForeignKey(name, columns, referencesTable, referencesColumns, onDelete?, onUpdate?) {#typescript-foreignkey-foreignkey-name-columns-referencestable-referencescolumns-ondelete-onupdate}

Named foreign key.

**Definition**

```typescript
constructor(name: string, columns: string[], referencesTable: string, referencesColumns: string[], onDelete?: ReferentialAction, onUpdate?: ReferentialAction)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | constraint name |
| `columns` | local columns |
| `referencesTable` | referenced table |
| `referencesColumns` | referenced columns |
| `onDelete` | optional [ReferentialAction](#typescript-referentialaction) |
| `onUpdate` | optional [ReferentialAction](#typescript-referentialaction) |

##### UniqueConstraint {#typescript-uniqueconstraint}

```typescript
class UniqueConstraint implements ITableConstraint
```

**Constructor**

| Name | Description |
| --- | --- |
| [UniqueConstraint(name, columns)](#typescript-uniqueconstraint-uniqueconstraint-name-columns) | Named unique constraint. |

###### UniqueConstraint(name, columns) {#typescript-uniqueconstraint-uniqueconstraint-name-columns}

Named unique constraint.

**Definition**

```typescript
constructor(name: string, columns: string[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | constraint name |
| `columns` | unique columns |

##### CheckConstraint {#typescript-checkconstraint}

```typescript
class CheckConstraint implements ITableConstraint
```

**Constructor**

| Name | Description |
| --- | --- |
| [CheckConstraint(name, expression)](#typescript-checkconstraint-checkconstraint-name-expression) | Named check constraint. |

###### CheckConstraint(name, expression) {#typescript-checkconstraint-checkconstraint-name-expression}

Named check constraint.

**Definition**

```typescript
constructor(name: string, expression: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | constraint name |
| `expression` | raw SQL check fragment |

##### ReferentialAction {#typescript-referentialaction}

```typescript
const ReferentialAction
```

**Properties**

| Name | Description |
| --- | --- |
| Cascade | `ON DELETE` / `ON UPDATE CASCADE`. |
| SetNull | `ON DELETE` / `ON UPDATE SET NULL`. |
| SetDefault | `ON DELETE` / `ON UPDATE SET DEFAULT`. |
| Restrict | `ON DELETE` / `ON UPDATE RESTRICT`. |
| NoAction | `ON DELETE` / `ON UPDATE NO ACTION`. |

##### C#

##### ICreateTableStatement {#csharp-icreatetablestatement}

```csharp
public interface ICreateTableStatement : IStatement
```

##### CreateTableStatement {#csharp-createtablestatement}

```csharp
public sealed record CreateTableStatement : ICreateTableStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [CreateTableStatement(text)](#csharp-createtablestatement-createtablestatement-text) | Wraps create-table text. |
| [CreateTableStatement(table, columns, constraints?)](#csharp-createtablestatement-createtablestatement-table-columns-constraints) | Compiles a `CREATE TABLE`. |

**Properties**

| Name | Description |
| --- | --- |
| Table | Table name when compiled from columns. Absent for wrapped text. |
| Columns | Columns on a compiled table. Empty for wrapped text. |
| Constraints | Named table constraints on a compiled table. Empty for wrapped text. |

**Methods**

| Name | Description |
| --- | --- |
| [GetText()](#csharp-createtablestatement-gettext) | Command text. |

###### CreateTableStatement(text) {#csharp-createtablestatement-createtablestatement-text}

Wraps create-table text.

**Definition**

```csharp
public CreateTableStatement(string text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### CreateTableStatement(table, columns, constraints?) {#csharp-createtablestatement-createtablestatement-table-columns-constraints}

Compiles a `CREATE TABLE`.

**Definition**

```csharp
public CreateTableStatement(string table, IReadOnlyList<Column> columns, IReadOnlyList<ITableConstraint>? constraints = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `columns` | [Column](#csharp-column) names, types, optional [ColumnDefault](#csharp-columndefault) and [NotNull](#csharp-notnull) |
| `constraints` | optional named [ITableConstraint](#csharp-itableconstraint) list |

###### GetText() {#csharp-createtablestatement-gettext}

Command text.

**Definition**

```csharp
public string GetText()
```

**Returns**

`string`

##### Column {#csharp-column}

```csharp
public sealed record Column(string Name, string Type, ColumnDefault? Default = null, NotNull? NotNull = null, bool Identity = false)
```

**Properties**

| Name | Description |
| --- | --- |
| Name | Column name. |
| Type | SQL type name. |
| Default | Optional [ColumnDefault](#csharp-columndefault). |
| NotNull | Optional [NotNull](#csharp-notnull). |
| Identity | When true, MySQL and Postgres commands render an identity clause. `GetText()` leaves it off. |

##### ColumnDefault {#csharp-columndefault}

```csharp
public sealed record ColumnDefault(string Value, string? Name = null)
```

**Properties**

| Name | Description |
| --- | --- |
| Value | Raw SQL default fragment. |
| Name | Optional constraint name. |

##### NotNull {#csharp-notnull}

```csharp
public sealed record NotNull(string? Name = null)
```

**Properties**

| Name | Description |
| --- | --- |
| Name | Optional constraint name. |

##### ITableConstraint {#csharp-itableconstraint}

```csharp
public interface ITableConstraint
```

##### PrimaryKey {#csharp-primarykey}

```csharp
public sealed record PrimaryKey(string Name, IReadOnlyList<string> Columns) : ITableConstraint
```

**Properties**

| Name | Description |
| --- | --- |
| Name | Constraint name. |
| Columns | Key columns. Multiple columns are a composite key. |

##### ForeignKey {#csharp-foreignkey}

```csharp
public sealed record ForeignKey(string Name, IReadOnlyList<string> Columns, string ReferencesTable, IReadOnlyList<string> ReferencesColumns, ReferentialAction? OnDelete = null, ReferentialAction? OnUpdate = null) : ITableConstraint
```

**Properties**

| Name | Description |
| --- | --- |
| Name | Constraint name. |
| Columns | Local columns. |
| ReferencesTable | Referenced table. |
| ReferencesColumns | Referenced columns. |
| OnDelete | Optional [ReferentialAction](#csharp-referentialaction). |
| OnUpdate | Optional [ReferentialAction](#csharp-referentialaction). |

##### UniqueConstraint {#csharp-uniqueconstraint}

```csharp
public sealed record UniqueConstraint(string Name, IReadOnlyList<string> Columns) : ITableConstraint
```

**Properties**

| Name | Description |
| --- | --- |
| Name | Constraint name. |
| Columns | Unique columns. |

##### CheckConstraint {#csharp-checkconstraint}

```csharp
public sealed record CheckConstraint(string Name, string Expression) : ITableConstraint
```

**Properties**

| Name | Description |
| --- | --- |
| Name | Constraint name. |
| Expression | Raw SQL check fragment. |

##### ReferentialAction {#csharp-referentialaction}

```csharp
public enum ReferentialAction
```

**Properties**

| Name | Description |
| --- | --- |
| Cascade | `ON DELETE` / `ON UPDATE CASCADE`. |
| SetNull | `ON DELETE` / `ON UPDATE SET NULL`. |
| SetDefault | `ON DELETE` / `ON UPDATE SET DEFAULT`. |
| Restrict | `ON DELETE` / `ON UPDATE RESTRICT`. |
| NoAction | `ON DELETE` / `ON UPDATE NO ACTION`. |

##### Rust

##### CreateTableStatement {#rust-createtablestatement}

```rust
pub struct CreateTableStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [text(text)](#rust-createtablestatement-text-text) | Wraps create-table text. |
| [new(table, columns, constraints)](#rust-createtablestatement-new-table-columns-constraints) | Compiles a `CREATE TABLE`. |

**Properties**

| Name | Description |
| --- | --- |
| table | Table name when compiled from columns. Absent for wrapped text. |
| columns | Columns on a compiled table. Empty for wrapped text. |
| constraints | Named table constraints on a compiled table. Empty for wrapped text. |

**Methods**

| Name | Description |
| --- | --- |
| [get_text()](#rust-createtablestatement-get_text) | Command text. |

###### text(text) {#rust-createtablestatement-text-text}

Wraps create-table text.

**Definition**

```rust
pub fn text(text: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

**Returns**

[CreateTableStatement](#rust-createtablestatement)

###### new(table, columns, constraints) {#rust-createtablestatement-new-table-columns-constraints}

Compiles a `CREATE TABLE`.

**Definition**

```rust
pub fn new(table: impl AsRef<str>, columns: &[Column], constraints: &[TableConstraint]) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `columns` | [Column](#rust-column) names, types, optional [ColumnDefault](#rust-columndefault) and [NotNull](#rust-notnull) |
| `constraints` | named [TableConstraint](#rust-tableconstraint) list (empty when omitted in C# / TypeScript) |

**Returns**

[CreateTableStatement](#rust-createtablestatement)

###### get_text() {#rust-createtablestatement-get_text}

Command text.

**Definition**

```rust
fn get_text(&self) -> String
```

**Returns**

`String`

##### Column {#rust-column}

```rust
pub struct Column
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(name, type_name)](#rust-column-new-name-type_name) | Column name and SQL type. |
| [default(self, default)](#rust-column-default-self-default) | Sets an optional [ColumnDefault](#rust-columndefault). |
| [not_null(self, not_null)](#rust-column-not_null-self-not_null) | Sets an optional [NotNull](#rust-notnull). |
| [identity(self)](#rust-column-identity-self) | Marks the column as an identity. `get_text()` leaves the clause off. MySQL and Postgres commands render it. |

###### new(name, type_name) {#rust-column-new-name-type_name}

Column name and SQL type.

**Definition**

```rust
pub fn new(name: impl Into<String>, type_name: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | column name |
| `type_name` | SQL type name |

**Returns**

[Column](#rust-column)

###### default(self, default) {#rust-column-default-self-default}

Sets an optional [ColumnDefault](#rust-columndefault).

**Definition**

```rust
pub fn default(self, default: ColumnDefault) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `default` | [ColumnDefault](#rust-columndefault) |

**Returns**

[Column](#rust-column)

###### not_null(self, not_null) {#rust-column-not_null-self-not_null}

Sets an optional [NotNull](#rust-notnull).

**Definition**

```rust
pub fn not_null(self, not_null: NotNull) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `not_null` | [NotNull](#rust-notnull) |

**Returns**

[Column](#rust-column)

###### identity(self) {#rust-column-identity-self}

Marks the column as an identity. `get_text()` leaves the clause off. MySQL and Postgres commands render it.

**Definition**

```rust
pub fn identity(self) -> Self
```

**Returns**

[Column](#rust-column)

##### ColumnDefault {#rust-columndefault}

```rust
pub struct ColumnDefault
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(value, name)](#rust-columndefault-new-value-name) | Default value, optionally named. |

###### new(value, name) {#rust-columndefault-new-value-name}

Default value, optionally named.

**Definition**

```rust
pub fn new(value: impl Into<String>, name: Option<&str>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | raw SQL default fragment |
| `name` | optional constraint name |

**Returns**

[ColumnDefault](#rust-columndefault)

##### NotNull {#rust-notnull}

```rust
pub struct NotNull
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(name)](#rust-notnull-new-name) | `NOT NULL`, optionally named. |

###### new(name) {#rust-notnull-new-name}

`NOT NULL`, optionally named.

**Definition**

```rust
pub fn new(name: Option<&str>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | optional constraint name |

**Returns**

[NotNull](#rust-notnull)

##### TableConstraint {#rust-tableconstraint}

```rust
pub enum TableConstraint
```

##### PrimaryKey {#rust-primarykey}

```rust
pub struct PrimaryKey
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(name, columns)](#rust-primarykey-new-name-columns) | Named primary key. Multiple columns are a composite key. |

###### new(name, columns) {#rust-primarykey-new-name-columns}

Named primary key. Multiple columns are a composite key.

**Definition**

```rust
pub fn new(name: impl Into<String>, columns: &[&str]) -> TableConstraint
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | constraint name |
| `columns` | key columns |

**Returns**

[TableConstraint](#rust-tableconstraint)

##### ForeignKey {#rust-foreignkey}

```rust
pub struct ForeignKey
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(name, columns, references_table, references_columns, on_delete, on_update)](#rust-foreignkey-new-name-columns-references_table-references_columns-on_delete-on_update) | Named foreign key. |

###### new(name, columns, references_table, references_columns, on_delete, on_update) {#rust-foreignkey-new-name-columns-references_table-references_columns-on_delete-on_update}

Named foreign key.

**Definition**

```rust
pub fn new(name: impl Into<String>, columns: &[&str], references_table: impl Into<String>, references_columns: &[&str], on_delete: Option<ReferentialAction>, on_update: Option<ReferentialAction>) -> TableConstraint
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | constraint name |
| `columns` | local columns |
| `references_table` | referenced table |
| `references_columns` | referenced columns |
| `on_delete` | optional [ReferentialAction](#rust-referentialaction) |
| `on_update` | optional [ReferentialAction](#rust-referentialaction) |

**Returns**

[TableConstraint](#rust-tableconstraint)

##### UniqueConstraint {#rust-uniqueconstraint}

```rust
pub struct UniqueConstraint
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(name, columns)](#rust-uniqueconstraint-new-name-columns) | Named unique constraint. |

###### new(name, columns) {#rust-uniqueconstraint-new-name-columns}

Named unique constraint.

**Definition**

```rust
pub fn new(name: impl Into<String>, columns: &[&str]) -> TableConstraint
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | constraint name |
| `columns` | unique columns |

**Returns**

[TableConstraint](#rust-tableconstraint)

##### CheckConstraint {#rust-checkconstraint}

```rust
pub struct CheckConstraint
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(name, expression)](#rust-checkconstraint-new-name-expression) | Named check constraint. |

###### new(name, expression) {#rust-checkconstraint-new-name-expression}

Named check constraint.

**Definition**

```rust
pub fn new(name: impl Into<String>, expression: impl Into<String>) -> TableConstraint
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | constraint name |
| `expression` | raw SQL check fragment |

**Returns**

[TableConstraint](#rust-tableconstraint)

##### ReferentialAction {#rust-referentialaction}

```rust
pub enum ReferentialAction
```

**Properties**

| Name | Description |
| --- | --- |
| Cascade | `ON DELETE` / `ON UPDATE CASCADE`. |
| SetNull | `ON DELETE` / `ON UPDATE SET NULL`. |
| SetDefault | `ON DELETE` / `ON UPDATE SET DEFAULT`. |
| Restrict | `ON DELETE` / `ON UPDATE RESTRICT`. |
| NoAction | `ON DELETE` / `ON UPDATE NO ACTION`. |

### Remarks

A compiled statement stores the table, columns, and constraints. `GetText()` / `getText()` / `get_text()` renders `CREATE TABLE table (name TYPE [CONSTRAINT n DEFAULT v] [CONSTRAINT n NOT NULL], …, CONSTRAINT name …)`. Table and column names are interpolated as given. Default and check values are raw SQL fragments. Wrapped text has no table. Indexes use [CreateIndexStatement](#create-index-statement). `GetText()` / `getText()` / `get_text()` uses `CreateTableStatementRenderer`. A provider subclasses that renderer and can replace `Render` / `render` / `render` or override type, default, and identity. SQLite, MySQL, and Postgres override type so canonical names become engine types. MySQL and Postgres also override default and identity. A Postgres `bool` default goes through the bool type converter, so `1` becomes `TRUE`. Identity renders `AUTO_INCREMENT` or `GENERATED BY DEFAULT AS IDENTITY`. SQLite leaves identity empty. `GetText()` / `getText()` / `get_text()` leaves those changes off. Other commands use the rendered text.

### Examples

#### Compile a CREATE TABLE

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L161-L162)

```csharp
Joins = [Join.Inner("customers", "orders.customer_id", "customers.id")],
GroupBy = ["status"],
```

**After: create table**

```
CREATE TABLE users (id INTEGER, name TEXT)
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L995-L1003)

```rust
assert!(Where::new(Operator::In {
    left: Box::new(Expression::Literal(Literal::new(1_i64))),
    right: Box::new(Expression::Literal(Literal::new(2_i64))),
})
.parameters()
.is_empty());
```

**After: create table**

```
CREATE TABLE users (id INTEGER, name TEXT)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L240-L244)

```typescript
where: new Where(new Operator.Equals("status", "open")),
joins: [Join.inner("customers", "orders.customer_id", "customers.id")],
groupBy: ["status"],
orderBy: [OrderBy.desc("total"), OrderBy.asc("status")],
limit: 10,
```

**After: create table**

```
CREATE TABLE users (id INTEGER, name TEXT)
```

#### Compile named constraints

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L163-L176)

```csharp
    OrderBy = [OrderBy.Desc("total"), OrderBy.Asc("status")],
    Limit = 10,
});
```

**After: named constraints**

```
CREATE TABLE orders (id INTEGER CONSTRAINT df_id DEFAULT 0 CONSTRAINT nn_id NOT NULL, tenant_id INTEGER, user_id INTEGER, qty INTEGER, CONSTRAINT pk_orders PRIMARY KEY (id, tenant_id), CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE, CONSTRAINT uq_orders_user UNIQUE (user_id), CONSTRAINT chk_orders_qty CHECK (qty > 0))
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1008-L1031)

```rust
);
```

**After: named constraints**

```
CREATE TABLE orders (id INTEGER CONSTRAINT df_id DEFAULT 0 CONSTRAINT nn_id NOT NULL, tenant_id INTEGER, user_id INTEGER, qty INTEGER, CONSTRAINT pk_orders PRIMARY KEY (id, tenant_id), CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE, CONSTRAINT uq_orders_user UNIQUE (user_id), CONSTRAINT chk_orders_qty CHECK (qty > 0))
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L245-L259)

```typescript
});
```

**After: named constraints**

```
CREATE TABLE orders (id INTEGER CONSTRAINT df_id DEFAULT 0 CONSTRAINT nn_id NOT NULL, tenant_id INTEGER, user_id INTEGER, qty INTEGER, CONSTRAINT pk_orders PRIMARY KEY (id, tenant_id), CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE, CONSTRAINT uq_orders_user UNIQUE (user_id), CONSTRAINT chk_orders_qty CHECK (qty > 0))
```

## Create Index Statement

`CREATE INDEX` text, or a compiled `CREATE [UNIQUE] INDEX` from a name, table, and columns.

### Classes

##### TypeScript

##### ICreateIndexStatement {#typescript-icreateindexstatement}

```typescript
interface ICreateIndexStatement extends IStatement {}
```

##### CreateIndexStatement {#typescript-createindexstatement}

```typescript
class CreateIndexStatement implements ICreateIndexStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [CreateIndexStatement(text)](#typescript-createindexstatement-createindexstatement-text) | Wraps create-index text. |
| [CreateIndexStatement(name, table, columns, unique?)](#typescript-createindexstatement-createindexstatement-name-table-columns-unique) | Compiles a `CREATE INDEX`. |

**Methods**

| Name | Description |
| --- | --- |
| [getText()](#typescript-createindexstatement-gettext) | Command text. |

###### CreateIndexStatement(text) {#typescript-createindexstatement-createindexstatement-text}

Wraps create-index text.

**Definition**

```typescript
constructor(text: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### CreateIndexStatement(name, table, columns, unique?) {#typescript-createindexstatement-createindexstatement-name-table-columns-unique}

Compiles a `CREATE INDEX`.

**Definition**

```typescript
constructor(name: string, table: string, columns: string[], unique?: boolean)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | index name |
| `table` | table name |
| `columns` | indexed columns |
| `unique` | `CREATE UNIQUE INDEX` when true |

###### getText() {#typescript-createindexstatement-gettext}

Command text.

**Definition**

```typescript
getText(): string
```

**Returns**

`string`

##### C#

##### ICreateIndexStatement {#csharp-icreateindexstatement}

```csharp
public interface ICreateIndexStatement : IStatement
```

##### CreateIndexStatement {#csharp-createindexstatement}

```csharp
public sealed record CreateIndexStatement : ICreateIndexStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [CreateIndexStatement(text)](#csharp-createindexstatement-createindexstatement-text) | Wraps create-index text. |
| [CreateIndexStatement(name, table, columns, unique?)](#csharp-createindexstatement-createindexstatement-name-table-columns-unique) | Compiles a `CREATE INDEX`. |

**Methods**

| Name | Description |
| --- | --- |
| [GetText()](#csharp-createindexstatement-gettext) | Command text. |

###### CreateIndexStatement(text) {#csharp-createindexstatement-createindexstatement-text}

Wraps create-index text.

**Definition**

```csharp
public CreateIndexStatement(string text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### CreateIndexStatement(name, table, columns, unique?) {#csharp-createindexstatement-createindexstatement-name-table-columns-unique}

Compiles a `CREATE INDEX`.

**Definition**

```csharp
public CreateIndexStatement(string name, string table, IReadOnlyList<string> columns, bool unique = false)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | index name |
| `table` | table name |
| `columns` | indexed columns |
| `unique` | `CREATE UNIQUE INDEX` when true |

###### GetText() {#csharp-createindexstatement-gettext}

Command text.

**Definition**

```csharp
public string GetText()
```

**Returns**

`string`

##### Rust

##### CreateIndexStatement {#rust-createindexstatement}

```rust
pub struct CreateIndexStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [text(text)](#rust-createindexstatement-text-text) | Wraps create-index text. |
| [new(name, table, columns, unique)](#rust-createindexstatement-new-name-table-columns-unique) | Compiles a `CREATE INDEX`. |

**Methods**

| Name | Description |
| --- | --- |
| [get_text()](#rust-createindexstatement-get_text) | Command text. |

###### text(text) {#rust-createindexstatement-text-text}

Wraps create-index text.

**Definition**

```rust
pub fn text(text: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

**Returns**

[CreateIndexStatement](#rust-createindexstatement)

###### new(name, table, columns, unique) {#rust-createindexstatement-new-name-table-columns-unique}

Compiles a `CREATE INDEX`.

**Definition**

```rust
pub fn new(name: impl AsRef<str>, table: impl AsRef<str>, columns: &[&str], unique: bool) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | index name |
| `table` | table name |
| `columns` | indexed columns |
| `unique` | `CREATE UNIQUE INDEX` when true |

**Returns**

[CreateIndexStatement](#rust-createindexstatement)

###### get_text() {#rust-createindexstatement-get_text}

Command text.

**Definition**

```rust
fn get_text(&self) -> String
```

**Returns**

`String`

### Remarks

Compiled form is `CREATE INDEX name ON table (cols)` or `CREATE UNIQUE INDEX name ON table (cols)`. Table and column names are interpolated as given. [Command](#command) uses `GetText()` / `getText()` / `get_text()` only.

### Examples

#### Compile a CREATE INDEX

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L209-L210)

```csharp
"CREATE TABLE items (id INTEGER DEFAULT 0, name TEXT NOT NULL, CONSTRAINT fk_items_user FOREIGN KEY (id) REFERENCES users (id))",
new CreateTableStatement(
```

**After: create index**

```
CREATE INDEX idx_orders_user ON orders (user_id)
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1076-L1083)

```rust
        ..SelectStatementOptions::new("users", &["users.id"])
    })
    .get_text(),
    "SELECT users.id FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN payments ON orders.id = payments.order_id RIGHT JOIN profiles ON users.id = profiles.user_id WHERE users.id = $users.id"
);
```

**After: create index**

```
CREATE INDEX idx_orders_user ON orders (user_id)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L306-L307)

```typescript
new UniqueConstraint("uq_orders_user", ["user_id"]),
new CheckConstraint("chk_orders_qty", "qty > 0"),
```

**After: create index**

```
CREATE INDEX idx_orders_user ON orders (user_id)
```

#### Compile a CREATE UNIQUE INDEX

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L211-L213)

```csharp
"items",
[new Column("id", "INTEGER", new ColumnDefault("0")), new Column("name", "TEXT", NotNull: new NotNull())],
[new ForeignKey("fk_items_user", ["id"], "users", ["id"])]).GetText());
```

**After: create unique index**

```
CREATE UNIQUE INDEX uq_orders_code ON orders (code)
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1085-L1087)

```rust
})
.get_text(),
"SELECT id FROM users"
```

**After: create unique index**

```
CREATE UNIQUE INDEX uq_orders_code ON orders (code)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L308-L310)

```typescript
  ],
);
```

**After: create unique index**

```
CREATE UNIQUE INDEX uq_orders_code ON orders (code)
```

## Execute Stored Procedure Statement

`CALL` text, or a compiled `CALL` from a name and [Parameter](#parameter) list.

### Classes

##### TypeScript

##### IExecuteStoredProcedureStatement {#typescript-iexecutestoredprocedurestatement}

```typescript
interface IExecuteStoredProcedureStatement extends IStatement {}
```

##### ExecuteStoredProcedureStatement {#typescript-executestoredprocedurestatement}

```typescript
class ExecuteStoredProcedureStatement implements IExecuteStoredProcedureStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [ExecuteStoredProcedureStatement(text)](#typescript-executestoredprocedurestatement-executestoredprocedurestatement-text) | Wraps stored-procedure call text. |
| [ExecuteStoredProcedureStatement(name, parameters)](#typescript-executestoredprocedurestatement-executestoredprocedurestatement-name-parameters) | Compiles a `CALL`. Empty [Parameter](#typescript-parameter) list is `CALL name()`. [returnValue](#typescript-parameterdirection) parameters are omitted from the argument list. |

**Methods**

| Name | Description |
| --- | --- |
| [getText()](#typescript-executestoredprocedurestatement-gettext) | Command text. |

###### ExecuteStoredProcedureStatement(text) {#typescript-executestoredprocedurestatement-executestoredprocedurestatement-text}

Wraps stored-procedure call text.

**Definition**

```typescript
constructor(text: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### ExecuteStoredProcedureStatement(name, parameters) {#typescript-executestoredprocedurestatement-executestoredprocedurestatement-name-parameters}

Compiles a `CALL`. Empty [Parameter](#typescript-parameter) list is `CALL name()`. [returnValue](#typescript-parameterdirection) parameters are omitted from the argument list.

**Definition**

```typescript
constructor(name: string, parameters: Parameter[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | procedure name |
| `parameters` | bound values (empty for `CALL name()`) |

###### getText() {#typescript-executestoredprocedurestatement-gettext}

Command text.

**Definition**

```typescript
getText(): string
```

**Returns**

`string`

##### C#

##### IExecuteStoredProcedureStatement {#csharp-iexecutestoredprocedurestatement}

```csharp
public interface IExecuteStoredProcedureStatement : IStatement
```

##### ExecuteStoredProcedureStatement {#csharp-executestoredprocedurestatement}

```csharp
public sealed record ExecuteStoredProcedureStatement : IExecuteStoredProcedureStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [ExecuteStoredProcedureStatement(text)](#csharp-executestoredprocedurestatement-executestoredprocedurestatement-text) | Wraps stored-procedure call text. |
| [ExecuteStoredProcedureStatement(name, parameters)](#csharp-executestoredprocedurestatement-executestoredprocedurestatement-name-parameters) | Compiles a `CALL`. Empty [Parameter](#csharp-parameter) list is `CALL name()`. [ReturnValue](#csharp-parameterdirection) parameters are omitted from the argument list. |

**Methods**

| Name | Description |
| --- | --- |
| [GetText()](#csharp-executestoredprocedurestatement-gettext) | Command text. |

###### ExecuteStoredProcedureStatement(text) {#csharp-executestoredprocedurestatement-executestoredprocedurestatement-text}

Wraps stored-procedure call text.

**Definition**

```csharp
public ExecuteStoredProcedureStatement(string text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### ExecuteStoredProcedureStatement(name, parameters) {#csharp-executestoredprocedurestatement-executestoredprocedurestatement-name-parameters}

Compiles a `CALL`. Empty [Parameter](#csharp-parameter) list is `CALL name()`. [ReturnValue](#csharp-parameterdirection) parameters are omitted from the argument list.

**Definition**

```csharp
public ExecuteStoredProcedureStatement(string name, IReadOnlyList<Parameter> parameters)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | procedure name |
| `parameters` | bound values (empty for `CALL name()`) |

###### GetText() {#csharp-executestoredprocedurestatement-gettext}

Command text.

**Definition**

```csharp
public string GetText()
```

**Returns**

`string`

##### Rust

##### ExecuteStoredProcedureStatement {#rust-executestoredprocedurestatement}

```rust
pub struct ExecuteStoredProcedureStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [text(text)](#rust-executestoredprocedurestatement-text-text) | Wraps stored-procedure call text. |
| [new(name, parameters)](#rust-executestoredprocedurestatement-new-name-parameters) | Compiles a `CALL`. Empty [Parameter](#rust-parameter) list is `CALL name()`. [ReturnValue](#rust-parameterdirection) parameters are omitted from the argument list. |

**Methods**

| Name | Description |
| --- | --- |
| [get_text()](#rust-executestoredprocedurestatement-get_text) | Command text. |

###### text(text) {#rust-executestoredprocedurestatement-text-text}

Wraps stored-procedure call text.

**Definition**

```rust
pub fn text(text: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

**Returns**

[ExecuteStoredProcedureStatement](#rust-executestoredprocedurestatement)

###### new(name, parameters) {#rust-executestoredprocedurestatement-new-name-parameters}

Compiles a `CALL`. Empty [Parameter](#rust-parameter) list is `CALL name()`. [ReturnValue](#rust-parameterdirection) parameters are omitted from the argument list.

**Definition**

```rust
pub fn new(name: impl AsRef<str>, parameters: &[Parameter]) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | procedure name |
| `parameters` | bound values (empty for `CALL name()`) |

**Returns**

[ExecuteStoredProcedureStatement](#rust-executestoredprocedurestatement)

###### get_text() {#rust-executestoredprocedurestatement-get_text}

Command text.

**Definition**

```rust
fn get_text(&self) -> String
```

**Returns**

`String`

### Remarks

Compiled form is `CALL name($p)` or `CALL name()`. [ReturnValue](#parameterdirection) parameters are binds only and stay out of the argument list. The name constructor keeps the parameter list on the statement so [ReadonlyRepository](#readonly-repository) can bind it.

### Examples

#### Compile a CALL

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L214-L215)

```csharp

```

**After: call**

```
CALL save($id, $name)
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1089-L1090)

```rust
let count = Aggregate::count("*", Some("n"));
let total = Aggregate::sum("amount", Some("total"));
```

**After: call**

```
CALL save($id, $name)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L311-L312)

```typescript
  "CREATE TABLE orders (id INTEGER CONSTRAINT df_id DEFAULT 0 CONSTRAINT nn_id NOT NULL, tenant_id INTEGER, user_id INTEGER, qty INTEGER, CONSTRAINT pk_orders PRIMARY KEY (id, tenant_id), CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE, CONSTRAINT uq_orders_user UNIQUE (user_id), CONSTRAINT chk_orders_qty CHECK (qty > 0))",
);
```

**After: call**

```
CALL save($id, $name)
```

## Return Statement

Function-call text, or a compiled `SELECT name($p)` from a name and [Parameter](#parameter) list.

### Classes

##### TypeScript

##### IReturnStatement {#typescript-ireturnstatement}

```typescript
interface IReturnStatement extends IStatement {}
```

##### ReturnStatement {#typescript-returnstatement}

```typescript
class ReturnStatement implements IReturnStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [ReturnStatement(text)](#typescript-returnstatement-returnstatement-text) | Wraps function-call text. |
| [ReturnStatement(name, parameters)](#typescript-returnstatement-returnstatement-name-parameters) | Compiles a `SELECT`. Empty [Parameter](#typescript-parameter) list is `SELECT name()`. [returnValue](#typescript-parameterdirection) parameters are omitted from the argument list. |

**Methods**

| Name | Description |
| --- | --- |
| [getText()](#typescript-returnstatement-gettext) | Command text. |

###### ReturnStatement(text) {#typescript-returnstatement-returnstatement-text}

Wraps function-call text.

**Definition**

```typescript
constructor(text: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### ReturnStatement(name, parameters) {#typescript-returnstatement-returnstatement-name-parameters}

Compiles a `SELECT`. Empty [Parameter](#typescript-parameter) list is `SELECT name()`. [returnValue](#typescript-parameterdirection) parameters are omitted from the argument list.

**Definition**

```typescript
constructor(name: string, parameters: Parameter[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | function name |
| `parameters` | bound values (empty for `SELECT name()`) |

###### getText() {#typescript-returnstatement-gettext}

Command text.

**Definition**

```typescript
getText(): string
```

**Returns**

`string`

##### C#

##### IReturnStatement {#csharp-ireturnstatement}

```csharp
public interface IReturnStatement : IStatement
```

##### ReturnStatement {#csharp-returnstatement}

```csharp
public sealed record ReturnStatement : IReturnStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [ReturnStatement(text)](#csharp-returnstatement-returnstatement-text) | Wraps function-call text. |
| [ReturnStatement(name, parameters)](#csharp-returnstatement-returnstatement-name-parameters) | Compiles a `SELECT`. Empty [Parameter](#csharp-parameter) list is `SELECT name()`. [ReturnValue](#csharp-parameterdirection) parameters are omitted from the argument list. |

**Methods**

| Name | Description |
| --- | --- |
| [GetText()](#csharp-returnstatement-gettext) | Command text. |

###### ReturnStatement(text) {#csharp-returnstatement-returnstatement-text}

Wraps function-call text.

**Definition**

```csharp
public ReturnStatement(string text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

###### ReturnStatement(name, parameters) {#csharp-returnstatement-returnstatement-name-parameters}

Compiles a `SELECT`. Empty [Parameter](#csharp-parameter) list is `SELECT name()`. [ReturnValue](#csharp-parameterdirection) parameters are omitted from the argument list.

**Definition**

```csharp
public ReturnStatement(string name, IReadOnlyList<Parameter> parameters)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | function name |
| `parameters` | bound values (empty for `SELECT name()`) |

###### GetText() {#csharp-returnstatement-gettext}

Command text.

**Definition**

```csharp
public string GetText()
```

**Returns**

`string`

##### Rust

##### ReturnStatement {#rust-returnstatement}

```rust
pub struct ReturnStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [text(text)](#rust-returnstatement-text-text) | Wraps function-call text. |
| [new(name, parameters)](#rust-returnstatement-new-name-parameters) | Compiles a `SELECT`. Empty [Parameter](#rust-parameter) list is `SELECT name()`. [ReturnValue](#rust-parameterdirection) parameters are omitted from the argument list. |

**Methods**

| Name | Description |
| --- | --- |
| [get_text()](#rust-returnstatement-get_text) | Command text. |

###### text(text) {#rust-returnstatement-text-text}

Wraps function-call text.

**Definition**

```rust
pub fn text(text: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |

**Returns**

[ReturnStatement](#rust-returnstatement)

###### new(name, parameters) {#rust-returnstatement-new-name-parameters}

Compiles a `SELECT`. Empty [Parameter](#rust-parameter) list is `SELECT name()`. [ReturnValue](#rust-parameterdirection) parameters are omitted from the argument list.

**Definition**

```rust
pub fn new(name: impl AsRef<str>, parameters: &[Parameter]) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | function name |
| `parameters` | bound values (empty for `SELECT name()`) |

**Returns**

[ReturnStatement](#rust-returnstatement)

###### get_text() {#rust-returnstatement-get_text}

Command text.

**Definition**

```rust
fn get_text(&self) -> String
```

**Returns**

`String`

### Remarks

Compiled form is `SELECT name($p)` or `SELECT name()`. [ReturnValue](#parameterdirection) parameters are binds only and stay out of the argument list.

### Examples

#### Compile a SELECT function call

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L217-L218)

```csharp

```

**After: return**

```
SELECT add($id, $name)
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1095-L1096)

```rust
"customers",
"orders.customer_id",
```

**After: return**

```
SELECT add($id, $name)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L316-L317)

```typescript
[
  { name: "id", type: "INTEGER", default: new ColumnDefault("0") },
```

**After: return**

```
SELECT add($id, $name)
```

## Where

[IWhere](#iwhere) over an [IExpression](#iexpression) tree. A clause is an expression, not a statement. Placeholders are `$field` and `$field_0` for `In`.

### Classes

##### TypeScript

##### Where {#typescript-where}

```typescript
class Where implements IWhere
```

**Constructor**

| Name | Description |
| --- | --- |
| [Where(condition)](#typescript-where-where-condition) | Builds a where clause from an [IExpression](#typescript-iexpression). |
| [Where(column, op, value)](#typescript-where-where-column-op-value) | Builds a comparison from a column, a [Cmp](#typescript-cmp) alias, and a literal. |

**Properties**

| Name | Description |
| --- | --- |
| condition | Predicate tree. Typically an [Operator](#typescript-operator). |
| parameters | Bound [Parameter](#typescript-parameter) list. |

**Methods**

| Name | Description |
| --- | --- |
| [getText()](#typescript-where-gettext) | Predicate text. |
| [and(column, op, value) / or(column, op, value)](#typescript-where-and-column-op-value-or-column-op-value) | Nests this clause with another comparison under [Operator.And](#typescript-operator) or `Operator.Or`. |
| [map(column, value)](#typescript-where-map-column-value) | Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals. |

###### Where(condition) {#typescript-where-where-condition}

Builds a where clause from an [IExpression](#typescript-iexpression).

**Definition**

```typescript
constructor(condition: IExpression)
```

**Parameters**

| Name | Description |
| --- | --- |
| `condition` | [IExpression](#typescript-iexpression) |

###### Where(column, op, value) {#typescript-where-where-column-op-value}

Builds a comparison from a column, a [Cmp](#typescript-cmp) alias, and a literal.

**Definition**

```typescript
constructor(column: string, op: Comparison, value: Cell)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `op` | [Cmp](#typescript-cmp) alias such as `eq` or `gt` |
| `value` | [Cell](#typescript-cell) |

###### getText() {#typescript-where-gettext}

Predicate text.

**Definition**

```typescript
getText(): string
```

**Returns**

`string`

###### and(column, op, value) / or(column, op, value) {#typescript-where-and-column-op-value-or-column-op-value}

Nests this clause with another comparison under [Operator.And](#typescript-operator) or `Operator.Or`.

**Definition**

```typescript
and(column: string, op: Comparison, value: Cell): Where
or(column: string, op: Comparison, value: Cell): Where
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `op` | [Cmp](#typescript-cmp) alias |
| `value` | [Cell](#typescript-cell) |

**Returns**

[Where](#typescript-where)

###### map(column, value) {#typescript-where-map-column-value}

Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals.

**Definition**

```typescript
map(column: (name: string) => string, value: (name: string, cell: Cell) => Cell): Where
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | property name to stored column |
| `value` | property name and domain cell to stored cell |

**Returns**

[Where](#typescript-where)

##### IWhere {#typescript-iwhere}

```typescript
interface IWhere extends IExpression, IText {}
```

##### WhereText {#typescript-wheretext}

```typescript
class WhereText implements IWhere
```

**Constructor**

| Name | Description |
| --- | --- |
| [WhereText(text)](#typescript-wheretext-wheretext-text) | Wraps predicate text. |

**Methods**

| Name | Description |
| --- | --- |
| [getText()](#typescript-wheretext-gettext) | Predicate text. |

###### WhereText(text) {#typescript-wheretext-wheretext-text}

Wraps predicate text.

**Definition**

```typescript
constructor(text: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | predicate text |

###### getText() {#typescript-wheretext-gettext}

Predicate text.

**Definition**

```typescript
getText(): string
```

**Returns**

`string`

##### Scalar {#typescript-scalar}

```typescript
type Scalar = string | number | boolean | bigint | null
```

##### C#

##### Where {#csharp-where}

```csharp
public sealed record Where(IExpression Condition) : IWhere
```

**Constructor**

| Name | Description |
| --- | --- |
| [Where(condition)](#csharp-where-where-condition) | Builds a where clause from an [IExpression](#csharp-iexpression). |
| [Where(column, op, value)](#csharp-where-where-column-op-value) | Builds a comparison from a column, a [Cmp](#csharp-cmp) alias, and a literal. |

**Properties**

| Name | Description |
| --- | --- |
| Condition | Predicate tree. Typically an [Operator](#csharp-operator). |
| Parameters | Bound [Parameter](#csharp-parameter) list. |

**Methods**

| Name | Description |
| --- | --- |
| [GetText()](#csharp-where-gettext) | Predicate text. |
| [And(column, op, value) / Or(column, op, value)](#csharp-where-and-column-op-value-or-column-op-value) | Nests this clause with another comparison under `Operator.And` or `Operator.Or`. |
| [Map(column, value)](#csharp-where-map-column-value) | Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals. |

###### Where(condition) {#csharp-where-where-condition}

Builds a where clause from an [IExpression](#csharp-iexpression).

**Definition**

```csharp
public Where(IExpression Condition)
```

**Parameters**

| Name | Description |
| --- | --- |
| `condition` | [IExpression](#csharp-iexpression) |

###### Where(column, op, value) {#csharp-where-where-column-op-value}

Builds a comparison from a column, a [Cmp](#csharp-cmp) alias, and a literal.

**Definition**

```csharp
public Where(string column, Comparison op, Value? value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `op` | [Cmp](#csharp-cmp) alias such as `eq` or `gt` |
| `value` | cell value |

###### GetText() {#csharp-where-gettext}

Predicate text.

**Definition**

```csharp
public string GetText()
```

**Returns**

`string`

###### And(column, op, value) / Or(column, op, value) {#csharp-where-and-column-op-value-or-column-op-value}

Nests this clause with another comparison under `Operator.And` or `Operator.Or`.

**Definition**

```csharp
public Where And(string column, Comparison op, Value? value)
public Where Or(string column, Comparison op, Value? value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `op` | [Cmp](#csharp-cmp) alias |
| `value` | cell value |

**Returns**

[Where](#csharp-where)

###### Map(column, value) {#csharp-where-map-column-value}

Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals.

**Definition**

```csharp
public Where Map(Func<string, string> column, Func<string, Value, Value> value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | property name to stored column |
| `value` | property name and domain cell to stored cell |

**Returns**

[Where](#csharp-where)

##### IWhere {#csharp-iwhere}

```csharp
public interface IWhere : IExpression, IText
```

##### WhereText {#csharp-wheretext}

```csharp
public sealed record WhereText : IWhere
```

**Constructor**

| Name | Description |
| --- | --- |
| [WhereText(text)](#csharp-wheretext-wheretext-text) | Wraps predicate text. |

**Methods**

| Name | Description |
| --- | --- |
| [GetText()](#csharp-wheretext-gettext) | Predicate text. |

###### WhereText(text) {#csharp-wheretext-wheretext-text}

Wraps predicate text.

**Definition**

```csharp
public WhereText(string text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | predicate text |

###### GetText() {#csharp-wheretext-gettext}

Predicate text.

**Definition**

```csharp
string GetText()
```

**Returns**

`string`

##### Rust

##### Where {#rust-where}

```rust
pub struct Where
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(condition)](#rust-where-new-condition) | Builds a where clause from an [Expression](#rust-expression). |
| [binary(column, op, value)](#rust-where-binary-column-op-value) | Builds a comparison from a column, a [Cmp](#rust-cmp) alias, and a literal. |
| [and(column, op, value) / or(column, op, value)](#rust-where-and-column-op-value-or-column-op-value) | Nests this clause with another comparison under `Operator::and` or `Operator::or`. |
| [map(column, value)](#rust-where-map-column-value) | Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals. |
| [get_text()](#rust-where-get_text) | Predicate text. |
| [parameters()](#rust-where-parameters) | Bound [Parameter](#rust-parameter) list. |

###### new(condition) {#rust-where-new-condition}

Builds a where clause from an [Expression](#rust-expression).

**Definition**

```rust
pub fn new(condition: impl Into<Expression>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `condition` | [Expression](#rust-expression) or [Operator](#rust-operator) |

**Returns**

[Where](#rust-where)

###### binary(column, op, value) {#rust-where-binary-column-op-value}

Builds a comparison from a column, a [Cmp](#rust-cmp) alias, and a literal.

**Definition**

```rust
pub fn binary(column: &str, op: fn(&str, Value) -> Operator, value: impl Into<Value>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `op` | [Cmp](#rust-cmp) alias such as `eq` or `gt` |
| `value` | [Value](#rust-value) |

**Returns**

[Where](#rust-where)

###### and(column, op, value) / or(column, op, value) {#rust-where-and-column-op-value-or-column-op-value}

Nests this clause with another comparison under `Operator::and` or `Operator::or`.

**Definition**

```rust
pub fn and(self, column: &str, op: fn(&str, Value) -> Operator, value: impl Into<Value>) -> Self
pub fn or(self, column: &str, op: fn(&str, Value) -> Operator, value: impl Into<Value>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `op` | [Cmp](#rust-cmp) alias |
| `value` | [Value](#rust-value) |

**Returns**

[Where](#rust-where)

###### map(column, value) {#rust-where-map-column-value}

Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals.

**Definition**

```rust
pub fn map<C, V>(&self, column: C, value: V) -> Result<Self>
where
    C: Fn(&str) -> String,
    V: Fn(&str, &Value) -> Result<Value>,
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | property name to stored column |
| `value` | property name and domain cell to stored cell |

**Returns**

`Result<`[Where](#rust-where)`>`

###### get_text() {#rust-where-get_text}

Predicate text.

**Definition**

```rust
fn get_text(&self) -> String
```

**Returns**

`String`

###### parameters() {#rust-where-parameters}

Bound [Parameter](#rust-parameter) list.

**Definition**

```rust
fn parameters(&self) -> Vec<Parameter>
```

**Returns**

`Vec<`[Parameter](#rust-parameter)`>`

##### IWhere {#rust-iwhere}

```rust
pub trait IWhere: Text {}
```

##### WhereText {#rust-wheretext}

```rust
pub struct WhereText
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(text)](#rust-wheretext-new-text) | Wraps predicate text. |

**Methods**

| Name | Description |
| --- | --- |
| [get_text()](#rust-wheretext-get_text) | Predicate text. |

###### new(text) {#rust-wheretext-new-text}

Wraps predicate text.

**Definition**

```rust
pub fn new(text: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | predicate text |

**Returns**

[WhereText](#rust-wheretext)

###### get_text() {#rust-wheretext-get_text}

Predicate text.

**Definition**

```rust
fn get_text(&self) -> String
```

**Returns**

`String`

### Remarks

Construct the predicate with [Operator](#operator) records, then wrap it: `new Where(new Operator.Equals("id", 9))`. A column, [Cmp](#cmp) alias, and literal is the same tree: `new Where("id", eq, 9)`. [SelectStatement](#select-statement) / [UpdateStatement](#update-statement) / [DeleteStatement](#delete-statement) embed `Where.GetText()` in the command SQL. Keep the [Where](#where) when the caller needs bound parameters. [StatementEvaluator](#statement-evaluator) interprets the same tree against a parameter map. Raw text [WhereText](#wheretext) is not evaluable.

### Examples

#### Compare, groups, not, and in

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L52-L54)

```csharp
new Operator.Equals("id", 1),
new Operator.GreaterThan("age", 18)));
```

**After: and**

```
id = $id AND age > $age
```

**After: in**

```
id IN ($id_0, $id_1)
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L650-L654)

```rust
assert!(txn.commit().await.is_err());
txn.set_abort_error("a");
assert!(txn.abort().await.is_err());

let command = MockCommand::new();
```

**After: and**

```
id = $id AND age > $age
```

**After: in**

```
id IN ($id_0, $id_1)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L57-L59)

```typescript
const where = new Where(
  new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18)),
);
```

**After: and**

```
id = $id AND age > $age
```

**After: in**

```
id IN ($id_0, $id_1)
```

#### Column, alias, literal

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/ComparisonAliasTests.cs#L11-L17)

```csharp
var equals = new Where("username", eq, "grace");
var notEquals = new Where("id", ne, 2);
var greater = new Where("n", gt, 0);
var less = new Where("n", lt, 0);
var greaterOrEqual = new Where("n", ge, 0);
var lessOrEqual = new Where("n", le, 0);
var both = equals.And("is_enabled", eq, 0);
```

**After: equals**

```
username = $username
```

**After: and**

```
username = $username AND is_enabled = $is_enabled
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L2100-L2106)

```rust
let equals = Where::binary("username", eq, "grace");
let not_equals = Where::binary("id", ne, 2_i64);
let greater = Where::binary("n", gt, 0);
let less = Where::binary("n", lt, 0);
let greater_or_equal = Where::binary("n", ge, 0);
let less_or_equal = Where::binary("n", le, 0);
let both = equals.clone().and("is_enabled", eq, 0);
```

**After: equals**

```
username = $username
```

**After: and**

```
username = $username AND is_enabled = $is_enabled
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L356-L362)

```typescript
const call = new ExecuteStoredProcedureStatement("save", [...fields]);
```

**After: equals**

```
username = $username
```

**After: and**

```
username = $username AND is_enabled = $is_enabled
```

#### Attach to a query

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L126-L129)

```csharp
var fields = new[] { new Parameter("id", 1), new Parameter("name", "a") };
var query = new SelectStatement(new() { Table = "users", Fields = ["id", "name"], Where = where });
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id AND age > $age
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L929-L935)

```rust
    })
    .get_text(),
    "true = false"
);
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id AND age > $age
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L176-L181)

```typescript
).toEqual([]);
class Ghost extends Operator {}
expect(() => evaluator.evaluate(new Ghost(), row)).toThrow("empty expression");
expect(() => new Where(new Ghost()).getText()).toThrow("empty expression");
expect(() => new Where(new Ghost()).parameters).toThrow("empty expression");
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id
```

## Expressions

AST nodes for [Where](#where). `Operator` records hold operands; `GetText()` maps them to SQL.

### Classes

##### TypeScript

##### IExpression {#typescript-iexpression}

```typescript
interface IExpression {}
```

##### Operator {#typescript-operator}

```typescript
abstract class Operator implements IExpression
```

##### Cmp {#typescript-cmp}

```typescript
function eq(column: string, value: Cell): IExpression
```

**Methods**

| Name | Description |
| --- | --- |
| [eq / ne / gt / lt / ge / le](#typescript-cmp-eq-ne-gt-lt-ge-le) | `eq` is [Operator.Equals](#typescript-operator). `ne` is not-equals. `gt` / `lt` are greater-than and less-than. `ge` / `le` are the or-equal forms. |

###### eq / ne / gt / lt / ge / le {#typescript-cmp-eq-ne-gt-lt-ge-le}

`eq` is [Operator.Equals](#typescript-operator). `ne` is not-equals. `gt` / `lt` are greater-than and less-than. `ge` / `le` are the or-equal forms.

**Definition**

```typescript
function eq(column: string, value: Cell): IExpression
function ne(column: string, value: Cell): IExpression
function gt(column: string, value: Cell): IExpression
function lt(column: string, value: Cell): IExpression
function ge(column: string, value: Cell): IExpression
function le(column: string, value: Cell): IExpression
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `value` | [Cell](#typescript-cell) |

**Returns**

[IExpression](#typescript-iexpression)

##### Identifier {#typescript-identifier}

```typescript
class Identifier implements IExpression
```

**Constructor**

| Name | Description |
| --- | --- |
| [Identifier(name)](#typescript-identifier-identifier-name) |  |

**Properties**

| Name | Description |
| --- | --- |
| name |  |

###### Identifier(name) {#typescript-identifier-identifier-name}



**Definition**

```typescript
constructor(name: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | column name |

##### Literal {#typescript-literal}

```typescript
class Literal implements IExpression
```

**Constructor**

| Name | Description |
| --- | --- |
| [Literal(value)](#typescript-literal-literal-value) |  |

**Properties**

| Name | Description |
| --- | --- |
| value |  |

###### Literal(value) {#typescript-literal-literal-value}



**Definition**

```typescript
constructor(value: Cell)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | [Cell](#typescript-cell) |

##### ArrayExpression {#typescript-arrayexpression}

```typescript
class ArrayExpression implements IExpression
```

**Constructor**

| Name | Description |
| --- | --- |
| [ArrayExpression(elements)](#typescript-arrayexpression-arrayexpression-elements) |  |

**Properties**

| Name | Description |
| --- | --- |
| elements |  |

###### ArrayExpression(elements) {#typescript-arrayexpression-arrayexpression-elements}



**Definition**

```typescript
constructor(elements: IExpression[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `elements` | [IExpression](#typescript-iexpression) list |

##### C#

##### IExpression {#csharp-iexpression}

```csharp
public interface IExpression
```

##### Operator {#csharp-operator}

```csharp
namespace Datasources.Operator
```

##### Cmp {#csharp-cmp}

```csharp
public static class Cmp
public delegate IExpression Comparison(string column, Value? value)
```

**Methods**

| Name | Description |
| --- | --- |
| [eq / ne / gt / lt / ge / le](#csharp-cmp-eq-ne-gt-lt-ge-le) | `eq` is `Operator.Equals`. `ne` is not-equals. `gt` / `lt` are greater-than and less-than. `ge` / `le` are the or-equal forms. |

###### eq / ne / gt / lt / ge / le {#csharp-cmp-eq-ne-gt-lt-ge-le}

`eq` is `Operator.Equals`. `ne` is not-equals. `gt` / `lt` are greater-than and less-than. `ge` / `le` are the or-equal forms.

**Definition**

```csharp
public static IExpression eq(string column, Value? value)
public static IExpression ne(string column, Value? value)
public static IExpression gt(string column, Value? value)
public static IExpression lt(string column, Value? value)
public static IExpression ge(string column, Value? value)
public static IExpression le(string column, Value? value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `value` | cell value |

**Returns**

[IExpression](#csharp-iexpression)

##### Identifier {#csharp-identifier}

```csharp
public sealed record Identifier(string Name) : IExpression
```

**Constructor**

| Name | Description |
| --- | --- |
| [Identifier(name)](#csharp-identifier-identifier-name) |  |

**Properties**

| Name | Description |
| --- | --- |
| Name |  |

###### Identifier(name) {#csharp-identifier-identifier-name}



**Definition**

```csharp
public Identifier(string Name)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | column name |

##### Literal {#csharp-literal}

```csharp
public sealed record Literal(Value Value) : IExpression
```

**Constructor**

| Name | Description |
| --- | --- |
| [Literal(value)](#csharp-literal-literal-value) |  |

**Properties**

| Name | Description |
| --- | --- |
| Value |  |

###### Literal(value) {#csharp-literal-literal-value}



**Definition**

```csharp
public Literal(Value Value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | cell value |

##### ArrayExpression {#csharp-arrayexpression}

```csharp
public sealed record ArrayExpression(IReadOnlyList<IExpression> Elements) : IExpression
```

**Constructor**

| Name | Description |
| --- | --- |
| [ArrayExpression(elements)](#csharp-arrayexpression-arrayexpression-elements) |  |

**Properties**

| Name | Description |
| --- | --- |
| Elements |  |

###### ArrayExpression(elements) {#csharp-arrayexpression-arrayexpression-elements}



**Definition**

```csharp
public ArrayExpression(IReadOnlyList<IExpression> Elements)
```

**Parameters**

| Name | Description |
| --- | --- |
| `elements` | [IExpression](#csharp-iexpression) list |

##### Rust

##### Expression {#rust-expression}

```rust
pub enum Expression
```

##### Operator {#rust-operator}

```rust
pub enum Operator
```

##### Cmp {#rust-cmp}

```rust
pub fn eq(column: &str, value: Value) -> Operator
pub fn ne(column: &str, value: Value) -> Operator
pub fn gt(column: &str, value: Value) -> Operator
pub fn lt(column: &str, value: Value) -> Operator
pub fn ge(column: &str, value: Value) -> Operator
pub fn le(column: &str, value: Value) -> Operator
```

**Methods**

| Name | Description |
| --- | --- |
| [eq / ne / gt / lt / ge / le](#rust-cmp-eq-ne-gt-lt-ge-le) | `eq` calls `Operator::equals`. `ne` is not-equals. `gt` / `lt` are greater-than and less-than. `ge` / `le` are the or-equal forms. |

###### eq / ne / gt / lt / ge / le {#rust-cmp-eq-ne-gt-lt-ge-le}

`eq` calls `Operator::equals`. `ne` is not-equals. `gt` / `lt` are greater-than and less-than. `ge` / `le` are the or-equal forms.

**Definition**

```rust
pub fn eq(column: &str, value: Value) -> Operator
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `value` | [Value](#rust-value) |

**Returns**

[Operator](#rust-operator)

##### Identifier {#rust-identifier}

```rust
pub struct Identifier
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(name)](#rust-identifier-new-name) |  |

###### new(name) {#rust-identifier-new-name}



**Definition**

```rust
pub fn new(name: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | column name |

**Returns**

[Identifier](#rust-identifier)

##### Literal {#rust-literal}

```rust
pub struct Literal
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(value)](#rust-literal-new-value) |  |

###### new(value) {#rust-literal-new-value}



**Definition**

```rust
pub fn new(value: impl Into<Value>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | cell value |

**Returns**

[Literal](#rust-literal)

##### ArrayExpression {#rust-arrayexpression}

```rust
pub struct ArrayExpression
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(elements)](#rust-arrayexpression-new-elements) |  |

###### new(elements) {#rust-arrayexpression-new-elements}



**Definition**

```rust
pub fn new(elements: Vec<Expression>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `elements` | [Expression](#rust-expression) list |

**Returns**

[ArrayExpression](#rust-arrayexpression)

### Remarks

A comparison is `Operator.Equals(Identifier, Literal)`. `eq`, `ne`, `gt`, `lt`, `ge`, and `le` are aliases of those constructors. `And` / `Or` nest operator records. `Not` holds one argument. `In` uses an `ArrayExpression` of literals.

### Examples

#### Compare tree

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L77-L80)

```csharp

```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L743-L758)

```rust

```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L64-L74)

```typescript
]);
```

#### Column, alias, literal

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/ComparisonAliasTests.cs#L11-L17)

```csharp
var equals = new Where("username", eq, "grace");
var notEquals = new Where("id", ne, 2);
var greater = new Where("n", gt, 0);
var less = new Where("n", lt, 0);
var greaterOrEqual = new Where("n", ge, 0);
var lessOrEqual = new Where("n", le, 0);
var both = equals.And("is_enabled", eq, 0);
```

**After: equals**

```
username = $username
```

**After: and**

```
username = $username AND is_enabled = $is_enabled
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L2100-L2106)

```rust
let equals = Where::binary("username", eq, "grace");
let not_equals = Where::binary("id", ne, 2_i64);
let greater = Where::binary("n", gt, 0);
let less = Where::binary("n", lt, 0);
let greater_or_equal = Where::binary("n", ge, 0);
let less_or_equal = Where::binary("n", le, 0);
let both = equals.clone().and("is_enabled", eq, 0);
```

**After: equals**

```
username = $username
```

**After: and**

```
username = $username AND is_enabled = $is_enabled
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L356-L362)

```typescript
const call = new ExecuteStoredProcedureStatement("save", [...fields]);
```

**After: equals**

```
username = $username
```

**After: and**

```
username = $username AND is_enabled = $is_enabled
```

## StatementEvaluator

Interprets an [IExpression](#expressions) against a name→value map. [Where.GetText](#where) materializes SQL; this type evaluates.

### Classes

##### TypeScript

##### StatementEvaluator {#typescript-statementevaluator}

```typescript
class StatementEvaluator
```

**Methods**

| Name | Description |
| --- | --- |
| [evaluate(expression, parameters?)](#typescript-statementevaluator-evaluate-expression-parameters) | Resolves identifiers from `parameters`. A where condition returns a boolean. |

###### evaluate(expression, parameters?) {#typescript-statementevaluator-evaluate-expression-parameters}

Resolves identifiers from `parameters`. A where condition returns a boolean.

**Definition**

```typescript
evaluate(expression: IExpression, parameters?: Record<string, Scalar>): Scalar
```

**Parameters**

| Name | Description |
| --- | --- |
| `expression` | [IExpression](#typescript-iexpression) |
| `parameters` | row / name→value context |

**Returns**

[Scalar](#typescript-scalar)

##### C#

##### StatementEvaluator {#csharp-statementevaluator}

```csharp
public sealed class StatementEvaluator
```

**Methods**

| Name | Description |
| --- | --- |
| [Evaluate(expression, parameters)](#csharp-statementevaluator-evaluate-expression-parameters) | Resolves identifiers from `parameters`. A where condition returns a [Value](#command) bool. |

###### Evaluate(expression, parameters) {#csharp-statementevaluator-evaluate-expression-parameters}

Resolves identifiers from `parameters`. A where condition returns a [Value](#command) bool.

**Definition**

```csharp
public Value Evaluate(IExpression expression, IReadOnlyDictionary<string, Value>? parameters)
```

**Parameters**

| Name | Description |
| --- | --- |
| `expression` | [IExpression](#csharp-iexpression) |
| `parameters` | row / name→value context |

**Returns**

`Value`

##### Rust

##### StatementEvaluator {#rust-statementevaluator}

```rust
pub struct StatementEvaluator
```

**Methods**

| Name | Description |
| --- | --- |
| [evaluate(expression, parameters)](#rust-statementevaluator-evaluate-expression-parameters) | Resolves identifiers from `parameters`. A where condition returns `Value::Bool`. |

###### evaluate(expression, parameters) {#rust-statementevaluator-evaluate-expression-parameters}

Resolves identifiers from `parameters`. A where condition returns `Value::Bool`.

**Definition**

```rust
pub fn evaluate(&self, expression: &Expression, parameters: &Row) -> Value
```

**Parameters**

| Name | Description |
| --- | --- |
| `expression` | [Expression](#rust-expression) |
| `parameters` | row / name→value context |

**Returns**

[Value](#command)

### Remarks

Parameters here are the row, not the SQL bind list on `IStatement.Parameters`. A raw text [WhereText](#wheretext) is not evaluable.

### Examples

#### Match and miss

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L244-L245)

```csharp
var ret = new ReturnStatement("add", fields);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1166-L1170)

```rust
    &[],
);
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L101-L102)

```typescript

```

## Other

Supporting types for building a datasource and reading a connection URL.

- [Datasource Factory](#datasource-factory) — `Create` / `create` / `create` registers providers and builds a datasource from options
- [Datasource URL](#datasource-url) — `DatasourceUrlParser` reads a connection URL into a `DatasourceUrl`

## DatasourceFactory

Creates a [Datasource](#datasource) for a provider and `DATASOURCE_URL`. After [setup](#setup), [Datasource.Default](#datasource) builds that datasource from `.env`.

### Classes

##### TypeScript

##### DatasourceFactory {#typescript-datasourcefactory}

```typescript
class DatasourceFactory implements IDatasourceFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [defaults(options)](#typescript-datasourcefactory-defaults-options) | Registers listed provider packages. Does not apply to a non-null `create` bag. |
| [register(provider, driver)](#typescript-datasourcefactory-register-provider-driver) | Registers a provider [IDriver](#typescript-idriver) by name. |
| [create(options?)](#typescript-datasourcefactory-create-options) | Creates one datasource. `undefined` options read process env; a dictionary is used as-is. |
| [reset()](#typescript-datasourcefactory-reset) | Clears the registry. Use in tests. |
| [registered()](#typescript-datasourcefactory-registered) | Lists registered provider names. |

###### defaults(options) {#typescript-datasourcefactory-defaults-options}

Registers listed provider packages. Does not apply to a non-null `create` bag.

**Definition**

```typescript
static defaults(options: DatasourceOptions): void
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | install keys such as `INSTALLED_PROVIDERS`; not [Create options](#typescript-datasourceoptions) |

###### register(provider, driver) {#typescript-datasourcefactory-register-provider-driver}

Registers a provider [IDriver](#typescript-idriver) by name.

**Definition**

```typescript
static register(provider: string, driver: IDriver): void
```

**Parameters**

| Name | Description |
| --- | --- |
| `provider` | registry key (`sqlite`, `mysql`, …) |
| `driver` | [IDriver](#typescript-idriver) that builds connections, commands, transactions, and row converters |

###### create(options?) {#typescript-datasourcefactory-create-options}

Creates one datasource. `undefined` options read process env; a dictionary is used as-is.

**Definition**

```typescript
static create(options?: DatasourceOptions): IDatasource
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | [DatasourceOptions](#typescript-datasourceoptions); omitted loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL` |

**Returns**

[IDatasource](#typescript-idatasource)

###### reset() {#typescript-datasourcefactory-reset}

Clears the registry. Use in tests.

**Definition**

```typescript
static reset(): void
```

###### registered() {#typescript-datasourcefactory-registered}

Lists registered provider names.

**Definition**

```typescript
static registered(): string[]
```

**Returns**

`string[]`

##### IDatasourceFactory {#typescript-idatasourcefactory}

```typescript
interface IDatasourceFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [create(options?)](#typescript-idatasourcefactory-create-options) | Creates one datasource from a string dictionary. |

###### create(options?) {#typescript-idatasourcefactory-create-options}

Creates one datasource from a string dictionary.

**Definition**

```typescript
create(options?: DatasourceOptions): IDatasource
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | [DatasourceOptions](#typescript-datasourceoptions) |

**Returns**

[IDatasource](#typescript-idatasource)

##### C#

##### DatasourceFactory {#csharp-datasourcefactory}

```csharp
public sealed class DatasourceFactory : IDatasourceFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [Defaults(options)](#csharp-datasourcefactory-defaults-options) | Registers listed provider packages. Does not apply to a non-null `Create` bag. |
| [Register(provider, driver)](#csharp-datasourcefactory-register-provider-driver) | Registers a provider [IDriver](#csharp-idriver) by name. |
| [Create(options)](#csharp-datasourcefactory-create-options) | Creates one datasource. `null` options read process `.env`; a dictionary is used as-is. |
| [Reset()](#csharp-datasourcefactory-reset) | Clears the registry. Use in tests. |
| [Registered()](#csharp-datasourcefactory-registered) | Lists registered provider names. |

###### Defaults(options) {#csharp-datasourcefactory-defaults-options}

Registers listed provider packages. Does not apply to a non-null `Create` bag.

**Definition**

```csharp
public static void Defaults(IReadOnlyDictionary<string, string?> options)
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | install keys such as `INSTALLED_PROVIDERS`; not [Create options](#csharp-datasourceoptions) |

###### Register(provider, driver) {#csharp-datasourcefactory-register-provider-driver}

Registers a provider [IDriver](#csharp-idriver) by name.

**Definition**

```csharp
public static void Register(string provider, IDriver driver)
```

**Parameters**

| Name | Description |
| --- | --- |
| `provider` | registry key (`sqlite`, `mysql`, …) |
| `driver` | [IDriver](#csharp-idriver) that builds connections, commands, transactions, and row converters |

###### Create(options) {#csharp-datasourcefactory-create-options}

Creates one datasource. `null` options read process `.env`; a dictionary is used as-is.

**Definition**

```csharp
public static IDatasource Create(IReadOnlyDictionary<string, string?>? options = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | [DatasourceOptions](#csharp-datasourceoptions); omitted loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL` |

**Returns**

[IDatasource](#csharp-idatasource)

###### Reset() {#csharp-datasourcefactory-reset}

Clears the registry. Use in tests.

**Definition**

```csharp
public static void Reset()
```

###### Registered() {#csharp-datasourcefactory-registered}

Lists registered provider names.

**Definition**

```csharp
public static IReadOnlyList<string> Registered()
```

**Returns**

`IReadOnlyList<string>`

##### IDatasourceFactory {#csharp-idatasourcefactory}

```csharp
public interface IDatasourceFactory
```

**Methods**

| Name | Description |
| --- | --- |
| [Create(options)](#csharp-idatasourcefactory-create-options) | Creates one datasource from a string dictionary. |

###### Create(options) {#csharp-idatasourcefactory-create-options}

Creates one datasource from a string dictionary.

**Definition**

```csharp
IDatasource Create(IReadOnlyDictionary<string, string?>? options = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | [DatasourceOptions](#csharp-datasourceoptions) |

**Returns**

[IDatasource](#csharp-idatasource)

##### Rust

##### datasources {#rust-datasources}

```rust
// datasources
```

**Methods**

| Name | Description |
| --- | --- |
| [defaults(options)](#rust-datasources-defaults-options) | Registers listed provider packages. Does not apply to a `Some` `create` bag. |
| [register(provider, driver)](#rust-datasources-register-provider-driver) | Registers a provider [Driver](#rust-driver) by name. |
| [create(options)](#rust-datasources-create-options) | Creates one datasource. `None` reads process env; `Some` is used as-is. |
| [reset()](#rust-datasources-reset) | Clears the registry. Use in tests. |
| [registered()](#rust-datasources-registered) | Lists registered provider names. |

###### defaults(options) {#rust-datasources-defaults-options}

Registers listed provider packages. Does not apply to a `Some` `create` bag.

**Definition**

```rust
pub fn defaults(options: HashMap<String, String>) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | install keys such as `INSTALLED_PROVIDERS`; not [Create options](#rust-datasourceoptions) |

**Returns**

`Result<()>`

###### register(provider, driver) {#rust-datasources-register-provider-driver}

Registers a provider [Driver](#rust-driver) by name.

**Definition**

```rust
pub fn register(provider: impl Into<String>, driver: Arc<dyn Driver>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `provider` | registry key (`sqlite`, `mysql`, …) |
| `driver` | [Driver](#rust-driver) that builds connections, commands, transactions, and row converters |

###### create(options) {#rust-datasources-create-options}

Creates one datasource. `None` reads process env; `Some` is used as-is.

**Definition**

```rust
pub fn create(options: Option<&HashMap<String, String>>) -> Result<Arc<dyn Datasource>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | [DatasourceOptions](#rust-datasourceoptions); omitted loads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL` |

**Returns**

`Result<Arc<dyn `[Datasource](#rust-datasource)`>>`

###### reset() {#rust-datasources-reset}

Clears the registry. Use in tests.

**Definition**

```rust
pub fn reset()
```

###### registered() {#rust-datasources-registered}

Lists registered provider names.

**Definition**

```rust
pub fn registered() -> Vec<String>
```

**Returns**

`Vec<String>`

### Remarks

`Create(null)` / `create()` / `create(None)` reads `.env`, then process `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`. Missing `PROVIDER` throws `DatasourceException` / `DatasourceError` / `DatasourceError`. [Datasource.Default](#datasource) uses that path.

A missing factory throws `ProviderNotInstalledException` / `ProviderNotInstalledError` / `DatasourceError::ProviderNotInstalled`.

When `Create` / `create` / `create` is called with `null` / `undefined` / `None` options, each name in `INSTALLED_PROVIDERS` and the provider inferred from `DATASOURCE_URL` is registered. Every dialect loads those with `DriverLoader` and `ProviderCatalog`. Each dialect implements [IDriver](#idriver) (`MemoryDriver`, `SqliteDriver`, `MysqlDriver`, `PostgresDriver`, …) and that is what `Register` / `register` / `register` stores.

### Examples

#### Register and Create

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Memory.Tests/RegisterAndCreateTests.cs#L10-L16)

```csharp
DatasourceFactory.Register("memory", new MemoryDriver());
var ds = DatasourceFactory.Create(new Dictionary<string, string?> { ["PROVIDER"] = "memory" });
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L40-L45)

```rust
register("memory", Arc::new(MemoryDriver));
let ds = create(Some(&HashMap::from([("PROVIDER".into(), "memory".into())]))).unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/memory/src/index.test.ts#L31-L36)

```typescript
DatasourceFactory.register("memory", new MemoryDriver());
const ds = DatasourceFactory.create({ PROVIDER: "memory" });
```

#### Missing provider

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/DatasourceFactoryTests.cs#L31-L37)

```csharp
DatasourceFactory.Create(new Dictionary<string, string?> { ["PROVIDER"] = "missing" });
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/datasource_factory.rs#L25-L33)

```rust
create(Some(&HashMap::from([("PROVIDER".into(), "missing".into())])));
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/DatasourceFactory.test.ts#L50-L54)

```typescript
DatasourceFactory.create({ PROVIDER: "missing" });
```

## Datasource URL

A connection URL split into provider, host, credentials, datasource, schema, and options. A provider parses `DATASOURCE_URL` with `DatasourceUrlParser`. Schema is the `schema` query.

### Classes

##### TypeScript

##### DatasourceUrlParser {#typescript-datasourceurlparser}

```typescript
class DatasourceUrlParser
```

**Methods**

| Name | Description |
| --- | --- |
| [parse(url?)](#typescript-datasourceurlparser-parse-url) | Parses a connection URL. A blank string or a string that is not a URL returns `undefined`. |

###### parse(url?) {#typescript-datasourceurlparser-parse-url}

Parses a connection URL. A blank string or a string that is not a URL returns `undefined`.

**Definition**

```typescript
static parse(url?: string): DatasourceUrl | undefined
```

**Parameters**

| Name | Description |
| --- | --- |
| `url` | connection URL, such as `postgres://ds:ds@127.0.0.1:5432/datasources?schema=public` |

**Returns**

[DatasourceUrl](#typescript-datasourceurl) or `undefined`

##### DatasourceUrl {#typescript-datasourceurl}

```typescript
class DatasourceUrl
```

**Properties**

| Name | Description |
| --- | --- |
| provider | URL scheme, lowercased and unchanged. |
| username | User from the authority. Absent when the URL has no user. |
| password | Password from the authority, percent-decoded. Absent when the URL has no password. Split from the user on the first `:`. |
| host | Authority host. Absent when the URL has no host. |
| port | Authority port. Absent when the URL omits it. |
| path | Path after the authority, including a leading `/`. An opaque URL uses the remainder after the scheme. |
| datasource | `path` with one leading `/` removed. Absent when that leaves nothing. |
| schema | The `schema` query parameter, percent-decoded. Absent when that parameter is missing. Not copied into `options`. |
| options | The other query parameters. A flag with no `=` has an empty value. A repeated key keeps the last value. |

##### C#

##### DatasourceUrlParser {#csharp-datasourceurlparser}

```csharp
public static class DatasourceUrlParser
```

**Methods**

| Name | Description |
| --- | --- |
| [Parse(url)](#csharp-datasourceurlparser-parse-url) | Parses a connection URL. A blank string or a string that is not a URL returns null. |

###### Parse(url) {#csharp-datasourceurlparser-parse-url}

Parses a connection URL. A blank string or a string that is not a URL returns null.

**Definition**

```csharp
public static DatasourceUrl? Parse(string? url)
```

**Parameters**

| Name | Description |
| --- | --- |
| `url` | connection URL, such as `postgres://ds:ds@127.0.0.1:5432/datasources?schema=public` |

**Returns**

[DatasourceUrl](#csharp-datasourceurl) or null

##### DatasourceUrl {#csharp-datasourceurl}

```csharp
public sealed record DatasourceUrl(string Provider, string? Username, string? Password, string? Host, int? Port, string Path, string? Schema, IReadOnlyDictionary<string, string> Options)
```

**Properties**

| Name | Description |
| --- | --- |
| Provider | URL scheme, lowercased and unchanged. |
| Username | User from the authority. Absent when the URL has no user. |
| Password | Password from the authority, percent-decoded. Absent when the URL has no password. Split from the user on the first `:`. |
| Host | Authority host. Absent when the URL has no host. |
| Port | Authority port. Absent when the URL omits it. |
| Path | Path after the authority, including a leading `/`. An opaque URL uses the remainder after the scheme. |
| Datasource | `Path` with one leading `/` removed. Absent when that leaves nothing. |
| Schema | The `schema` query parameter, percent-decoded. Absent when that parameter is missing. Not copied into `Options`. |
| Options | The other query parameters. A flag with no `=` has an empty value. A repeated key keeps the last value. |

##### Rust

##### DatasourceUrlParser {#rust-datasourceurlparser}

```rust
pub struct DatasourceUrlParser
```

**Methods**

| Name | Description |
| --- | --- |
| [parse(url)](#rust-datasourceurlparser-parse-url) | Parses a connection URL. A blank string or a string that is not a URL returns `None`. |

###### parse(url) {#rust-datasourceurlparser-parse-url}

Parses a connection URL. A blank string or a string that is not a URL returns `None`.

**Definition**

```rust
pub fn parse(url: &str) -> Option<DatasourceUrl>
```

**Parameters**

| Name | Description |
| --- | --- |
| `url` | connection URL, such as `postgres://ds:ds@127.0.0.1:5432/datasources?schema=public` |

**Returns**

`Option<`[DatasourceUrl](#rust-datasourceurl)`>`

##### DatasourceUrl {#rust-datasourceurl}

```rust
pub struct DatasourceUrl
```

**Properties**

| Name | Description |
| --- | --- |
| provider | URL scheme, lowercased and unchanged. |
| username | User from the authority. Absent when the URL has no user. |
| password | Password from the authority, percent-decoded. Absent when the URL has no password. Split from the user on the first `:`. |
| host | Authority host. Absent when the URL has no host. |
| port | Authority port. Absent when the URL omits it. |
| path | Path after the authority, including a leading `/`. An opaque URL uses the remainder after the scheme. |
| schema | The `schema` query parameter, percent-decoded. Absent when that parameter is missing. Not copied into `options`. |
| options | The other query parameters. A flag with no `=` has an empty value. A repeated key keeps the last value. |

**Methods**

| Name | Description |
| --- | --- |
| [datasource()](#rust-datasourceurl-datasource) | `path` with one leading `/` removed. Absent when that leaves nothing. |

###### datasource() {#rust-datasourceurl-datasource}

`path` with one leading `/` removed. Absent when that leaves nothing.

**Definition**

```rust
pub fn datasource(&self) -> Option<&str>
```

**Returns**

`Option<&str>`

### Remarks

The provider is the scheme as written, lowercased. `postgres://` stays `postgres`. The parser does not rewrite a scheme to another name.

`sqlite:////tmp/app.db` keeps the extra slash: `path` is `//tmp/app.db` and `datasource` is `/tmp/app.db`. `sqlite://:memory:` (no third slash) is a host of `:memory:`, so `datasource` is absent. SQLite still applies its own filename default.

A raw connection string such as `Host=db;Port=1` is not a URL. `Parse` returns null and the provider can keep that string.

### Examples

#### Postgres URL

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/DatasourceUrlParserTests.cs#L5-L19)

```csharp
var url = DatasourceUrlParser.Parse("postgres://ds:ds@127.0.0.1:5432/datasources?schema=public&sslmode=require");
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/datasource_url_parser.rs#L3-L18)

```rust
let url = DatasourceUrlParser::parse(
    "postgres://ds:ds@127.0.0.1:5432/datasources?schema=public&sslmode=require",
)
.unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/DatasourceUrlParser.test.ts#L5-L18)

```typescript
const url = DatasourceUrlParser.parse(
  "postgres://ds:ds@127.0.0.1:5432/datasources?schema=public&sslmode=require",
);
```

#### SQLite file

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/DatasourceUrlParserTests.cs#L91-L100)

```csharp
var url = DatasourceUrlParser.Parse("sqlite:///app.db");
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/datasource_url_parser.rs#L79-L86)

```rust
let url = DatasourceUrlParser::parse("sqlite:///app.db").unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/DatasourceUrlParser.test.ts#L71-L77)

```typescript
const url = DatasourceUrlParser.parse("sqlite:///app.db");
```

#### Not a URL

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/DatasourceUrlParserTests.cs#L221-L230)

```csharp
var blank = DatasourceUrlParser.Parse("");
var text = DatasourceUrlParser.Parse("not-a-url");
var connectionString = DatasourceUrlParser.Parse("Host=db;Port=1");
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/datasource_url_parser.rs#L181-L189)

```rust
let blank = DatasourceUrlParser::parse("");
let text = DatasourceUrlParser::parse("not-a-url");
let connection_string = DatasourceUrlParser::parse("Host=db;Port=1");
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/DatasourceUrlParser.test.ts#L165-L172)

```typescript
const blank = DatasourceUrlParser.parse("");
const text = DatasourceUrlParser.parse("not-a-url");
const connectionString = DatasourceUrlParser.parse("Host=db;Port=1");
```

## Advanced

Strategies that fill related fields on read and write nested rows or bulk lists. Repositories call these. A repository page covers the call. This group covers the strategy.

### Enrichment

- [IEnriched](#ienriched) — registers a related fill
- [Enrichment](#enrichment) — one related fill
- [IReadonlyEnrichmentStrategy](#ireadonlyenrichmentstrategy) — fills parents on read
- [DefaultReadonlyEnrichmentStrategy](#defaultreadonlyenrichmentstrategy) — reads each related key, and caches lookups
- [HashsetOptimizedReadonlyEnrichmentStrategy](#hashsetoptimizedreadonlyenrichmentstrategy) — one `IN` query for the key set
- [NestedToObjectEnrichmentStrategy](#nestedtoobjectenrichmentstrategy) — collapses join rows

### Write

- [IWriteEnrichmentStrategy](#iwriteenrichmentstrategy) — writes nested rows with the parent
- [DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy) — cascade delete and replace-sync, with an optional transaction
- [LazyWriteEnrichmentStrategy](#lazywriteenrichmentstrategy) — skips a null list on create and update
- [UpdateEnrichmentStrategy](#updateenrichmentstrategy) — how an update writes children and links
- [DefaultUpdateEnrichmentStrategy](#defaultupdateenrichmentstrategy) — replace-syncs each many list
- [CascadeDeleteUpdateEnrichmentStrategy](#cascadedeleteupdateenrichmentstrategy) — deletes children, then inserts the payload
- [DeleteEnrichmentStrategy](#deleteenrichmentstrategy) — how a delete removes children and links
- [CascadeDeleteEnrichmentStrategy](#cascadedeleteenrichmentstrategy) — deletes nested rows, then children, then the parent

### Bulk write

- [IBulkWriteStrategy](#ibulkwritestrategy) — writes a list through a [Bulk repository](#bulk-repository)
- [SingleCommandIBulkWriteStrategy](#singlecommandibulkwritestrategy) — one statement per row
- [BatchCommandIBulkWriteStrategy](#batchcommandibulkwritestrategy) — joins the statements
- [StoredProcedureBulkWriteStrategy](#storedprocedurebulkwritestrategy) — one JSON call to a stored procedure

## IEnriched

Registers a related fill on a repository. Fluent for siblings. A related repository fills its own fields. Does not write.

### Classes

##### TypeScript

##### IEnriched {#typescript-ienriched}

```typescript
interface IEnriched<T>
```

**Methods**

| Name | Description |
| --- | --- |
| [addEnriched(enrichment, via, strategy?)](#typescript-ienriched-addenriched-enrichment-via-strategy) | Adds a related fill. Fluent for siblings. A related repository fills its own fields. Does not write. |

###### addEnriched(enrichment, via, strategy?) {#typescript-ienriched-addenriched-enrichment-via-strategy}

Adds a related fill. Fluent for siblings. A related repository fills its own fields. Does not write.

**Definition**

```typescript
addEnriched<TRelated extends object>(enrichment: Enrichment, via: IReadonlyRepository<TRelated>, strategy?: IReadonlyEnrichmentStrategy<T & object>): this
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](#enrichment) |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `strategy` | optional [IReadonlyEnrichmentStrategy](#ireadonlyenrichmentstrategy) for this edge. Omitted uses [DefaultReadonlyEnrichmentStrategy](#defaultreadonlyenrichmentstrategy) |

**Returns**

`this`

##### C#

##### IEnriched {#csharp-ienriched}

```csharp
public interface IEnriched<T>
```

**Methods**

| Name | Description |
| --- | --- |
| [AddEnriched(enrichment, via, strategy)](#csharp-ienriched-addenriched-enrichment-via-strategy) | Adds a related fill. Fluent for siblings. A related repository fills its own fields. Does not write. |

###### AddEnriched(enrichment, via, strategy) {#csharp-ienriched-addenriched-enrichment-via-strategy}

Adds a related fill. Fluent for siblings. A related repository fills its own fields. Does not write.

**Definition**

```csharp
IEnriched<T> AddEnriched<TRelated>(Enrichment enrichment, IReadonlyRepository<TRelated> via, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](#enrichment) |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `strategy` | optional [IReadonlyEnrichmentStrategy](#ireadonlyenrichmentstrategy) for this edge. Omitted uses [DefaultReadonlyEnrichmentStrategy](#defaultreadonlyenrichmentstrategy) |

**Returns**

[IEnriched](#csharp-ienriched)`<T>`

##### Rust

##### ReadonlyRepository {#rust-readonlyrepository}

```rust
pub struct ReadonlyRepository
```

**Methods**

| Name | Description |
| --- | --- |
| [add_enriched(enrichment, via, strategy)](#rust-readonlyrepository-add_enriched-enrichment-via-strategy) | Adds a related fill. A related repository fills its own fields. Does not write. |

###### add_enriched(enrichment, via, strategy) {#rust-readonlyrepository-add_enriched-enrichment-via-strategy}

Adds a related fill. A related repository fills its own fields. Does not write.

**Definition**

```rust
pub fn add_enriched(&mut self, enrichment: Enrichment, via: impl EnrichmentVia, strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>) -> &mut Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](#enrichment) |
| `via` | related [IReadonlyRepository](#readonly-repository). The readonly path keeps the read repository only |
| `strategy` | optional [ReadonlyEnrichmentStrategy](#ireadonlyenrichmentstrategy) for this edge. Omitted uses [DefaultReadonlyEnrichmentStrategy](#defaultreadonlyenrichmentstrategy) |

**Returns**

`&mut Self`

### Remarks

Register the child repository's own fills on that repository:

```
childrenRepo.AddEnriched(new("type_name", "type_id", "id", "name"), childTypesRepo);
parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
```

Rust has no separate trait. `AddEnriched` / `addEnriched` / `add_enriched` is a method on [ReadonlyRepository](#readonly-repository) and [CRUDRepository](#crud-repository). With no enrichments, `Read` / `read` / `read` stays streaming. With enrichments, the read is collected, filled, then yielded. Writes, including `many` children and associations, are [CRUDRepository](#crud-repository).

### Examples

#### Register a fill

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/EnrichedReadonlyRepositoryTests.cs#L35-L51)

```csharp
var person = new Row { ["name"] = "person" };
var gone = new Row { ["name"] = "gone" };
var typesConnection = await Setup(person, gone);
await typesConnection.CreateCommand().Execute("delete", [new Parameter("id", gone["id"])]);
var connection = await Setup(
    new Row { ["type_id"] = person["id"] },
    new Row { ["type_id"] = gone["id"] });
var parentsRepo = new ReadonlyRepository<Parent>(connection);
var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
parentsRepo.AddEnriched(new("type_name", "type_id", "id", "name"), typesRepo);
var items = await parentsRepo.Read().ToListAsync();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/enriched.rs#L124-L149)

```rust
let mut seeds = [
    record(&[("name", Value::from("person"))]),
    record(&[("name", Value::from("gone"))]),
];
let types_connection = setup(&mut seeds).await;
let person = seeds[0].clone();
let gone = seeds[1].clone();
drop_row(&types_connection, &gone).await;
let mut parents = [
    record(&[("type_id", person.get("id").unwrap().clone())]),
    record(&[("type_id", gone.get("id").unwrap().clone())]),
];
let connection = setup(&mut parents).await;
let mut parents_repo = readonly(connection, "Parent");
let types_repo = Arc::new(readonly(types_connection, "TypeRow"));
parents_repo.add_enriched(
    Enrichment::new("type_name", "type_id", "id", Some("name".into()), false),
    types_repo,
    None,
);
let items = parents_repo.read(None, None).await.unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/EnrichedReadonlyRepository.test.ts#L84-L96)

```typescript
const person: Seed = { name: "person" };
const gone: Seed = { name: "gone" };
const typesConnection = await setup([person, gone]);
await remove(typesConnection, gone);
const connection = await setup([{ type_id: person.id }, { type_id: gone.id }]);
const parentsRepo = new ReadonlyRepository(Parent, connection);
const typesRepo = new ReadonlyRepository(TypeRow, typesConnection);
parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo);
const items = await collectAsync(parentsRepo.read());
```

## Enrichment

One related fill. `into` is the destination property, `from` is the parent key, and `by` is the related key. `select` copies one related property. `many` collects a list. `association` names the junction columns for a link.

### Classes

##### TypeScript

##### Enrichment {#typescript-enrichment}

```typescript
type Enrichment
```

**Properties**

| Name | Description |
| --- | --- |
| into | Destination property. |
| from | Parent key. |
| by | Related key. |
| select | Optional related property to copy. |
| many | Collect a list. |
| association | Junction columns for a link. See [Association](#typescript-association). |

##### Association {#typescript-association}

```typescript
type Association
```

**Properties**

| Name | Description |
| --- | --- |
| parent | Junction column that stores the parent key. |
| related | Junction column that stores the related key. |

##### C#

##### Enrichment {#csharp-enrichment}

```csharp
public sealed record Enrichment
```

**Constructor**

| Name | Description |
| --- | --- |
| [Enrichment(into, from, by, select, many, association)](#csharp-enrichment-enrichment-into-from-by-select-many-association) | Builds one related fill. |

**Properties**

| Name | Description |
| --- | --- |
| Into | Destination property. |
| From | Parent key. |
| By | Related key. |
| Select | Optional related property to copy. |
| Many | Collect a list. |
| Association | Junction columns for a link. See [Association](#csharp-association). |

###### Enrichment(into, from, by, select, many, association) {#csharp-enrichment-enrichment-into-from-by-select-many-association}

Builds one related fill.

**Definition**

```csharp
public Enrichment(string into, string from, string by, string? select = null, bool many = false, Association? association = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `into` | destination property |
| `from` | parent key |
| `by` | related key |
| `select` | optional related property to copy |
| `many` | collect a list |
| `association` | optional [Association](#csharp-association) |

**Returns**

[Enrichment](#csharp-enrichment)

##### Association {#csharp-association}

```csharp
public sealed record Association
```

**Constructor**

| Name | Description |
| --- | --- |
| [Association(parent, related)](#csharp-association-association-parent-related) | Builds junction column names. |

**Properties**

| Name | Description |
| --- | --- |
| Parent | Junction column that stores the parent key. |
| Related | Junction column that stores the related key. |

###### Association(parent, related) {#csharp-association-association-parent-related}

Builds junction column names.

**Definition**

```csharp
public Association(string parent, string related)
```

**Parameters**

| Name | Description |
| --- | --- |
| `parent` | junction column that stores the parent key |
| `related` | junction column that stores the related key |

**Returns**

[Association](#csharp-association)

##### Rust

##### Enrichment {#rust-enrichment}

```rust
pub struct Enrichment
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(into, from, by, select, many)](#rust-enrichment-new-into-from-by-select-many) | Builds an enrichment. `association` starts unset. |
| [linked(association)](#rust-enrichment-linked-association) | Marks the fill as a many link and stores the junction columns. |

**Properties**

| Name | Description |
| --- | --- |
| into | Destination field. |
| from | Parent key. |
| by | Related key. |
| select | Optional related field to copy. |
| many | Collect a list. |
| association | Junction columns for a link. See [Association](#rust-association). |

###### new(into, from, by, select, many) {#rust-enrichment-new-into-from-by-select-many}

Builds an enrichment. `association` starts unset.

**Definition**

```rust
pub fn new(into: impl Into<String>, from: impl Into<String>, by: impl Into<String>, select: Option<String>, many: bool) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `into` | destination field |
| `from` | parent key |
| `by` | related key |
| `select` | optional related field to copy |
| `many` | collect a list |

**Returns**

[Enrichment](#rust-enrichment)

###### linked(association) {#rust-enrichment-linked-association}

Marks the fill as a many link and stores the junction columns.

**Definition**

```rust
pub fn linked(self, association: Association) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `association` | [Association](#rust-association) |

**Returns**

[Enrichment](#rust-enrichment)

##### Association {#rust-association}

```rust
pub struct Association
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(parent, related)](#rust-association-new-parent-related) | Builds junction column names. |

**Properties**

| Name | Description |
| --- | --- |
| parent | Junction column that stores the parent key. |
| related | Junction column that stores the related key. |

###### new(parent, related) {#rust-association-new-parent-related}

Builds junction column names.

**Definition**

```rust
pub fn new(parent: impl Into<String>, related: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `parent` | junction column that stores the parent key |
| `related` | junction column that stores the related key |

**Returns**

[Association](#rust-association)

### Remarks

A lookup copies `select`, or the related row when `select` is omitted. A `many` list is walked. A `select` scalar is not. [AddLinked](#crud-repository) sets `association` and writes the junction. Pass the value to [IEnriched](#ienriched).

### Examples

#### Fill a lookup

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/EnrichedReadonlyRepositoryTests.cs#L45-L47)

```csharp
var parentsRepo = new ReadonlyRepository<Parent>(connection);
var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
parentsRepo.AddEnriched(new("type_name", "type_id", "id", "name"), typesRepo);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/enriched.rs#L139-L145)

```rust
let mut parents_repo = readonly(connection, "Parent");
let types_repo = Arc::new(readonly(types_connection, "TypeRow"));
parents_repo.add_enriched(
    Enrichment::new("type_name", "type_id", "id", Some("name".into()), false),
    types_repo,
    None,
);
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/EnrichedReadonlyRepository.test.ts#L90-L92)

```typescript
const parentsRepo = new ReadonlyRepository(Parent, connection);
const typesRepo = new ReadonlyRepository(TypeRow, typesConnection);
parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo);
```

## IReadonlyEnrichmentStrategy

Fills each parent from a related repository. [IEnriched](#ienriched) uses one strategy per edge. Omitted uses [DefaultReadonlyEnrichmentStrategy](#defaultreadonlyenrichmentstrategy).

### Classes

##### TypeScript

##### IReadonlyEnrichmentStrategy {#typescript-ireadonlyenrichmentstrategy}

```typescript
interface IReadonlyEnrichmentStrategy<T extends object>
```

**Methods**

| Name | Description |
| --- | --- |
| [apply(via, enrichment, parents, sources?)](#typescript-ireadonlyenrichmentstrategy-apply-via-enrichment-parents-sources) | Fills each parent from the related repository. |
| [applyLinked(via, link, enrichment, parents, sources?)](#typescript-ireadonlyenrichmentstrategy-applylinked-via-link-enrichment-parents-sources) | Fills each parent by reading the link table, then the related repository. |

###### apply(via, enrichment, parents, sources?) {#typescript-ireadonlyenrichmentstrategy-apply-via-enrichment-parents-sources}

Fills each parent from the related repository.

**Definition**

```typescript
apply<TRelated extends object>(via: IReadonlyRepository<TRelated>, enrichment: Enrichment, parents: object[], sources?: (Row | undefined)[]): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `enrichment` | [Enrichment](#enrichment) |
| `parents` | parents to fill |
| `sources` | optional source rows aligned with `parents` |

**Returns**

`Promise<void>`

###### applyLinked(via, link, enrichment, parents, sources?) {#typescript-ireadonlyenrichmentstrategy-applylinked-via-link-enrichment-parents-sources}

Fills each parent by reading the link table, then the related repository.

**Definition**

```typescript
applyLinked<TRelated extends object, TLink extends object>(via: IReadonlyRepository<TRelated>, link: ICRUDRepository<TLink>, enrichment: Enrichment, parents: object[], sources?: (Row | undefined)[]): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `link` | [ICRUDRepository](#crud-repository) |
| `enrichment` | [Enrichment](#enrichment) |
| `parents` | parents to fill |
| `sources` | optional source rows aligned with `parents` |

**Returns**

`Promise<void>`

##### C#

##### IReadonlyEnrichmentStrategy {#csharp-ireadonlyenrichmentstrategy}

```csharp
public interface IReadonlyEnrichmentStrategy<T>
```

**Methods**

| Name | Description |
| --- | --- |
| [Apply(via, enrichment, parents, sources)](#csharp-ireadonlyenrichmentstrategy-apply-via-enrichment-parents-sources) | Fills each parent from the related repository. |
| [ApplyLinked(via, link, enrichment, parents, sources)](#csharp-ireadonlyenrichmentstrategy-applylinked-via-link-enrichment-parents-sources) | Fills each parent by reading the link table, then the related repository. |

###### Apply(via, enrichment, parents, sources) {#csharp-ireadonlyenrichmentstrategy-apply-via-enrichment-parents-sources}

Fills each parent from the related repository.

**Definition**

```csharp
Task Apply<TRelated>(IReadonlyRepository<TRelated> via, Enrichment enrichment, IList<object> parents, IReadOnlyList<Row?>? sources = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `enrichment` | [Enrichment](#enrichment) |
| `parents` | parents to fill |
| `sources` | optional source rows aligned with `parents` |

**Returns**

`Task`

###### ApplyLinked(via, link, enrichment, parents, sources) {#csharp-ireadonlyenrichmentstrategy-applylinked-via-link-enrichment-parents-sources}

Fills each parent by reading the link table, then the related repository.

**Definition**

```csharp
Task ApplyLinked<TRelated, TLink>(IReadonlyRepository<TRelated> via, ICRUDRepository<TLink> link, Enrichment enrichment, IList<object> parents, IReadOnlyList<Row?>? sources = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `link` | [ICRUDRepository](#crud-repository) |
| `enrichment` | [Enrichment](#enrichment) |
| `parents` | parents to fill |
| `sources` | optional source rows aligned with `parents` |

**Returns**

`Task`

##### Rust

##### ReadonlyEnrichmentStrategy {#rust-readonlyenrichmentstrategy}

```rust
pub trait ReadonlyEnrichmentStrategy: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [apply(via, enrichment, parents, sources)](#rust-readonlyenrichmentstrategy-apply-via-enrichment-parents-sources) | Fills each parent from the related repository. The default dispatches to `apply_lookup` or `apply_many`. |
| [apply_lookup(via, enrichment, parents, sources)](#rust-readonlyenrichmentstrategy-apply_lookup-via-enrichment-parents-sources) | Fills a scalar related value. The default reads one related row per distinct key. |
| [apply_many(via, enrichment, parents, sources)](#rust-readonlyenrichmentstrategy-apply_many-via-enrichment-parents-sources) | Fills a list. The default reads the related rows for each parent key. |
| [apply_linked(via, link, enrichment, parents, sources)](#rust-readonlyenrichmentstrategy-apply_linked-via-link-enrichment-parents-sources) | Fills each parent by reading the link table, then the related repository. |

###### apply(via, enrichment, parents, sources) {#rust-readonlyenrichmentstrategy-apply-via-enrichment-parents-sources}

Fills each parent from the related repository. The default dispatches to `apply_lookup` or `apply_many`.

**Definition**

```rust
async fn apply(&self, via: Arc<dyn IReadonlyRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `enrichment` | [Enrichment](#enrichment) |
| `parents` | parents to fill |
| `sources` | optional source rows aligned with `parents` |

**Returns**

`Result<()>`

###### apply_lookup(via, enrichment, parents, sources) {#rust-readonlyenrichmentstrategy-apply_lookup-via-enrichment-parents-sources}

Fills a scalar related value. The default reads one related row per distinct key.

**Definition**

```rust
async fn apply_lookup(&self, via: Arc<dyn IReadonlyRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `enrichment` | [Enrichment](#enrichment) |
| `parents` | parents to fill |
| `sources` | optional source rows aligned with `parents` |

**Returns**

`Result<()>`

###### apply_many(via, enrichment, parents, sources) {#rust-readonlyenrichmentstrategy-apply_many-via-enrichment-parents-sources}

Fills a list. The default reads the related rows for each parent key.

**Definition**

```rust
async fn apply_many(&self, via: Arc<dyn IReadonlyRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `enrichment` | [Enrichment](#enrichment) |
| `parents` | parents to fill |
| `sources` | optional source rows aligned with `parents` |

**Returns**

`Result<()>`

###### apply_linked(via, link, enrichment, parents, sources) {#rust-readonlyenrichmentstrategy-apply_linked-via-link-enrichment-parents-sources}

Fills each parent by reading the link table, then the related repository.

**Definition**

```rust
async fn apply_linked(&self, via: Arc<dyn IReadonlyRepository>, link: Arc<dyn ICRUDRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `link` | [ICRUDRepository](#crud-repository) |
| `enrichment` | [Enrichment](#enrichment) |
| `parents` | parents to fill |
| `sources` | optional source rows aligned with `parents` |

**Returns**

`Result<()>`

### Remarks

Pass the strategy to [IEnriched](#ienriched) for one edge. The repository calls `Apply` / `apply` / `apply` after it reads the parents. `ApplyLinked` / `applyLinked` / `apply_linked` runs for an enrichment that has an [association](#association). A `many` list is walked. A `select` scalar is not. C# and TypeScript override the lookup and list fills on [DefaultReadonlyEnrichmentStrategy](#defaultreadonlyenrichmentstrategy). Rust overrides `apply_lookup` and `apply_many` on this trait.

### Examples

#### Fill a lookup

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/EnrichedReadonlyRepositoryTests.cs#L45-L47)

```csharp
var parentsRepo = new ReadonlyRepository<Parent>(connection);
var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
parentsRepo.AddEnriched(new("type_name", "type_id", "id", "name"), typesRepo);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/enriched.rs#L139-L145)

```rust
let mut parents_repo = readonly(connection, "Parent");
let types_repo = Arc::new(readonly(types_connection, "TypeRow"));
parents_repo.add_enriched(
    Enrichment::new("type_name", "type_id", "id", Some("name".into()), false),
    types_repo,
    None,
);
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/EnrichedReadonlyRepository.test.ts#L90-L92)

```typescript
const parentsRepo = new ReadonlyRepository(Parent, connection);
const typesRepo = new ReadonlyRepository(TypeRow, typesConnection);
parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo);
```

## DefaultReadonlyEnrichmentStrategy

Default read fill. A lookup reads one related row per distinct key and caches it. A list reads the related rows for that parent key. A link reads the junction, then each related row.

### Classes

##### TypeScript

##### DefaultReadonlyEnrichmentStrategy {#typescript-defaultreadonlyenrichmentstrategy}

```typescript
class DefaultReadonlyEnrichmentStrategy<T extends object> implements IReadonlyEnrichmentStrategy<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [new DefaultReadonlyEnrichmentStrategy()](#typescript-defaultreadonlyenrichmentstrategy-new-defaultreadonlyenrichmentstrategy) | Builds the default read strategy. |

**Methods**

| Name | Description |
| --- | --- |
| [apply(via, enrichment, parents, sources?)](#typescript-defaultreadonlyenrichmentstrategy-apply-via-enrichment-parents-sources) | Fills a lookup from a cached related row, or a list from the rows for that parent key. |
| [applyLinked(via, link, enrichment, parents, sources?)](#typescript-defaultreadonlyenrichmentstrategy-applylinked-via-link-enrichment-parents-sources) | Reads the junction for each parent key, then each related row. |

###### new DefaultReadonlyEnrichmentStrategy() {#typescript-defaultreadonlyenrichmentstrategy-new-defaultreadonlyenrichmentstrategy}

Builds the default read strategy.

**Definition**

```typescript
constructor()
```

**Returns**

[DefaultReadonlyEnrichmentStrategy](#typescript-defaultreadonlyenrichmentstrategy)

###### apply(via, enrichment, parents, sources?) {#typescript-defaultreadonlyenrichmentstrategy-apply-via-enrichment-parents-sources}

Fills a lookup from a cached related row, or a list from the rows for that parent key.

**Definition**

```typescript
apply<TRelated extends object>(via: IReadonlyRepository<TRelated>, enrichment: Enrichment, parents: object[], sources?: (Row | undefined)[]): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `enrichment` | [Enrichment](#enrichment) |
| `parents` | parents to fill |
| `sources` | optional source rows aligned with `parents` |

**Returns**

`Promise<void>`

###### applyLinked(via, link, enrichment, parents, sources?) {#typescript-defaultreadonlyenrichmentstrategy-applylinked-via-link-enrichment-parents-sources}

Reads the junction for each parent key, then each related row.

**Definition**

```typescript
applyLinked<TRelated extends object, TLink extends object>(via: IReadonlyRepository<TRelated>, link: ICRUDRepository<TLink>, enrichment: Enrichment, parents: object[], sources?: (Row | undefined)[]): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `link` | [ICRUDRepository](#crud-repository) |
| `enrichment` | [Enrichment](#enrichment) with [association](#typescript-association) |
| `parents` | parents to fill |
| `sources` | optional source rows aligned with `parents` |

**Returns**

`Promise<void>`

##### C#

##### DefaultReadonlyEnrichmentStrategy {#csharp-defaultreadonlyenrichmentstrategy}

```csharp
public class DefaultReadonlyEnrichmentStrategy<T> : IReadonlyEnrichmentStrategy<T> where T : new()
```

**Constructor**

| Name | Description |
| --- | --- |
| [DefaultReadonlyEnrichmentStrategy()](#csharp-defaultreadonlyenrichmentstrategy-defaultreadonlyenrichmentstrategy) | Builds the default read strategy. |

**Methods**

| Name | Description |
| --- | --- |
| [Apply(via, enrichment, parents, sources)](#csharp-defaultreadonlyenrichmentstrategy-apply-via-enrichment-parents-sources) | Fills a lookup from a cached related row, or a list from the rows for that parent key. |
| [ApplyLinked(via, link, enrichment, parents, sources)](#csharp-defaultreadonlyenrichmentstrategy-applylinked-via-link-enrichment-parents-sources) | Reads the junction for each parent key, then each related row. |

###### DefaultReadonlyEnrichmentStrategy() {#csharp-defaultreadonlyenrichmentstrategy-defaultreadonlyenrichmentstrategy}

Builds the default read strategy.

**Definition**

```csharp
public DefaultReadonlyEnrichmentStrategy()
```

**Returns**

[DefaultReadonlyEnrichmentStrategy](#csharp-defaultreadonlyenrichmentstrategy)`<T>`

###### Apply(via, enrichment, parents, sources) {#csharp-defaultreadonlyenrichmentstrategy-apply-via-enrichment-parents-sources}

Fills a lookup from a cached related row, or a list from the rows for that parent key.

**Definition**

```csharp
public virtual Task Apply<TRelated>(IReadonlyRepository<TRelated> via, Enrichment enrichment, IList<object> parents, IReadOnlyList<Row?>? sources = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `enrichment` | [Enrichment](#enrichment) |
| `parents` | parents to fill |
| `sources` | optional source rows aligned with `parents` |

**Returns**

`Task`

###### ApplyLinked(via, link, enrichment, parents, sources) {#csharp-defaultreadonlyenrichmentstrategy-applylinked-via-link-enrichment-parents-sources}

Reads the junction for each parent key, then each related row.

**Definition**

```csharp
public virtual Task ApplyLinked<TRelated, TLink>(IReadonlyRepository<TRelated> via, ICRUDRepository<TLink> link, Enrichment enrichment, IList<object> parents, IReadOnlyList<Row?>? sources = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `link` | [ICRUDRepository](#crud-repository) |
| `enrichment` | [Enrichment](#enrichment) with [Association](#csharp-association) |
| `parents` | parents to fill |
| `sources` | optional source rows aligned with `parents` |

**Returns**

`Task`

##### Rust

##### DefaultReadonlyEnrichmentStrategy {#rust-defaultreadonlyenrichmentstrategy}

```rust
pub struct DefaultReadonlyEnrichmentStrategy
```

**Methods**

| Name | Description |
| --- | --- |
| [apply(via, enrichment, parents, sources)](#rust-defaultreadonlyenrichmentstrategy-apply-via-enrichment-parents-sources) | Fills a lookup from a cached related row, or a list from the rows for that parent key. |
| [apply_linked(via, link, enrichment, parents, sources)](#rust-defaultreadonlyenrichmentstrategy-apply_linked-via-link-enrichment-parents-sources) | Reads the junction for each parent key, then each related row. |

###### apply(via, enrichment, parents, sources) {#rust-defaultreadonlyenrichmentstrategy-apply-via-enrichment-parents-sources}

Fills a lookup from a cached related row, or a list from the rows for that parent key.

**Definition**

```rust
async fn apply(&self, via: Arc<dyn IReadonlyRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `enrichment` | [Enrichment](#enrichment) |
| `parents` | parents to fill |
| `sources` | optional source rows aligned with `parents` |

**Returns**

`Result<()>`

###### apply_linked(via, link, enrichment, parents, sources) {#rust-defaultreadonlyenrichmentstrategy-apply_linked-via-link-enrichment-parents-sources}

Reads the junction for each parent key, then each related row.

**Definition**

```rust
async fn apply_linked(&self, via: Arc<dyn IReadonlyRepository>, link: Arc<dyn ICRUDRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `link` | [ICRUDRepository](#crud-repository) |
| `enrichment` | [Enrichment](#enrichment) with [association](#rust-association) |
| `parents` | parents to fill |
| `sources` | optional source rows aligned with `parents` |

**Returns**

`Result<()>`

### Remarks

[IEnriched](#ienriched) uses this strategy when the call omits one. A missing related row leaves the destination unset. [HashsetOptimizedReadonlyEnrichmentStrategy](#hashsetoptimizedreadonlyenrichmentstrategy) replaces the per-key reads with one `IN` query.

### Examples

#### Fill a lookup

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/EnrichedReadonlyRepositoryTests.cs#L45-L47)

```csharp
var parentsRepo = new ReadonlyRepository<Parent>(connection);
var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
parentsRepo.AddEnriched(new("type_name", "type_id", "id", "name"), typesRepo);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/enriched.rs#L139-L145)

```rust
let mut parents_repo = readonly(connection, "Parent");
let types_repo = Arc::new(readonly(types_connection, "TypeRow"));
parents_repo.add_enriched(
    Enrichment::new("type_name", "type_id", "id", Some("name".into()), false),
    types_repo,
    None,
);
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/EnrichedReadonlyRepository.test.ts#L90-L92)

```typescript
const parentsRepo = new ReadonlyRepository(Parent, connection);
const typesRepo = new ReadonlyRepository(TypeRow, typesConnection);
parentsRepo.addEnriched({ into: "type_name", from: "type_id", by: "id", select: "name" }, typesRepo);
```

## HashsetOptimizedReadonlyEnrichmentStrategy

Read fill that loads every related row for the parent keys in one `IN` query, then assigns from that set. Lookups and lists both use that query.

### Classes

##### TypeScript

##### HashsetOptimizedReadonlyEnrichmentStrategy {#typescript-hashsetoptimizedreadonlyenrichmentstrategy}

```typescript
class HashsetOptimizedReadonlyEnrichmentStrategy<T extends object> extends DefaultReadonlyEnrichmentStrategy<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [new HashsetOptimizedReadonlyEnrichmentStrategy()](#typescript-hashsetoptimizedreadonlyenrichmentstrategy-new-hashsetoptimizedreadonlyenrichmentstrategy) | Builds the batched read strategy. |

**Methods**

| Name | Description |
| --- | --- |
| [apply(via, enrichment, parents, sources?)](#typescript-hashsetoptimizedreadonlyenrichmentstrategy-apply-via-enrichment-parents-sources) | Fills lookups and lists from one `IN` read of the related repository. |

###### new HashsetOptimizedReadonlyEnrichmentStrategy() {#typescript-hashsetoptimizedreadonlyenrichmentstrategy-new-hashsetoptimizedreadonlyenrichmentstrategy}

Builds the batched read strategy.

**Definition**

```typescript
constructor()
```

**Returns**

[HashsetOptimizedReadonlyEnrichmentStrategy](#typescript-hashsetoptimizedreadonlyenrichmentstrategy)

###### apply(via, enrichment, parents, sources?) {#typescript-hashsetoptimizedreadonlyenrichmentstrategy-apply-via-enrichment-parents-sources}

Fills lookups and lists from one `IN` read of the related repository.

**Definition**

```typescript
apply<TRelated extends object>(via: IReadonlyRepository<TRelated>, enrichment: Enrichment, parents: object[], sources?: (Row | undefined)[]): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `enrichment` | [Enrichment](#enrichment) |
| `parents` | parents to fill |
| `sources` | optional source rows aligned with `parents` |

**Returns**

`Promise<void>`

##### C#

##### HashsetOptimizedReadonlyEnrichmentStrategy {#csharp-hashsetoptimizedreadonlyenrichmentstrategy}

```csharp
public sealed class HashsetOptimizedReadonlyEnrichmentStrategy<T> : DefaultReadonlyEnrichmentStrategy<T> where T : new()
```

**Constructor**

| Name | Description |
| --- | --- |
| [HashsetOptimizedReadonlyEnrichmentStrategy()](#csharp-hashsetoptimizedreadonlyenrichmentstrategy-hashsetoptimizedreadonlyenrichmentstrategy) | Builds the batched read strategy. |

**Methods**

| Name | Description |
| --- | --- |
| [Apply(via, enrichment, parents, sources)](#csharp-hashsetoptimizedreadonlyenrichmentstrategy-apply-via-enrichment-parents-sources) | Inherited from [DefaultReadonlyEnrichmentStrategy](#defaultreadonlyenrichmentstrategy). Lookups and lists each use one `IN` read of the related repository. |

###### HashsetOptimizedReadonlyEnrichmentStrategy() {#csharp-hashsetoptimizedreadonlyenrichmentstrategy-hashsetoptimizedreadonlyenrichmentstrategy}

Builds the batched read strategy.

**Definition**

```csharp
public HashsetOptimizedReadonlyEnrichmentStrategy()
```

**Returns**

[HashsetOptimizedReadonlyEnrichmentStrategy](#csharp-hashsetoptimizedreadonlyenrichmentstrategy)`<T>`

###### Apply(via, enrichment, parents, sources) {#csharp-hashsetoptimizedreadonlyenrichmentstrategy-apply-via-enrichment-parents-sources}

Inherited from [DefaultReadonlyEnrichmentStrategy](#defaultreadonlyenrichmentstrategy). Lookups and lists each use one `IN` read of the related repository.

**Definition**

```csharp
public virtual Task Apply<TRelated>(IReadonlyRepository<TRelated> via, Enrichment enrichment, IList<object> parents, IReadOnlyList<Row?>? sources = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `enrichment` | [Enrichment](#enrichment) |
| `parents` | parents to fill |
| `sources` | optional source rows aligned with `parents` |

**Returns**

`Task`

##### Rust

##### HashsetOptimizedReadonlyEnrichmentStrategy {#rust-hashsetoptimizedreadonlyenrichmentstrategy}

```rust
pub struct HashsetOptimizedReadonlyEnrichmentStrategy
```

**Methods**

| Name | Description |
| --- | --- |
| [apply(via, enrichment, parents, sources)](#rust-hashsetoptimizedreadonlyenrichmentstrategy-apply-via-enrichment-parents-sources) | Fills lookups and lists from one `IN` read of the related repository. |

###### apply(via, enrichment, parents, sources) {#rust-hashsetoptimizedreadonlyenrichmentstrategy-apply-via-enrichment-parents-sources}

Fills lookups and lists from one `IN` read of the related repository.

**Definition**

```rust
async fn apply(&self, via: Arc<dyn IReadonlyRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `enrichment` | [Enrichment](#enrichment) |
| `parents` | parents to fill |
| `sources` | optional source rows aligned with `parents` |

**Returns**

`Result<()>`

### Remarks

Pass it as the strategy on [IEnriched](#ienriched). Linked fills stay on [DefaultReadonlyEnrichmentStrategy](#defaultreadonlyenrichmentstrategy). A parent with no key is skipped. A key with no related row leaves a lookup unset and a list empty.

### Examples

#### Fill with one query

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/EnrichedReadonlyRepositoryTests.cs#L53-L72)

```csharp
var person = new Row { ["name"] = "person" };
var gone = new Row { ["name"] = "gone" };
var typesConnection = await Setup(person, gone);
await typesConnection.CreateCommand().Execute("delete", [new Parameter("id", gone["id"])]);
var connection = await Setup(
    new Row { ["type_id"] = person["id"] },
    new Row { ["type_id"] = gone["id"] });
var parentsRepo = new ReadonlyRepository<Parent>(connection);
var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
parentsRepo.AddEnriched(
    new("type_name", "type_id", "id", "name"),
    typesRepo,
    new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>());
var items = await parentsRepo.Read().ToListAsync();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L361-L423)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
let connection = open(ds, "user-roles-read", &seed()).await;
let mut users_options = create_crud_repository_options::<UsersView>();
users_options.entity_mapper = Box::new(DictionaryMapper::new([(
    "UsersView".to_string(),
    "user".to_string(),
)]));
users_options.column_mappings = Box::new(DictionaryMapper::new([
    ("Key".to_string(), "id".to_string()),
    ("Login".to_string(), "username".to_string()),
    ("Active".to_string(), "is_enabled".to_string()),
    ("Opened".to_string(), "created".to_string()),
    ("UserType".to_string(), "user_type_id".to_string()),
]));
let mut type_options = create_crud_repository_options::<UserType>();
type_options.entity_mapper =
    Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
type_options.column_mappings =
    Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
let mut role_options = create_crud_repository_options::<UserRole>();
role_options.entity = "user_roles".into();
role_options.column_mappings = Box::new(DictionaryMapper::new([
    ("UserId".to_string(), "user_id".to_string()),
    ("Name".to_string(), "name".to_string()),
]));
role_options.select = Some(Arc::new(TransformMapper::new(roles_sql, |sql| sql)));
let mut users_repo =
    ReadonlyRepository::new(connection.clone(), users_options.read).unwrap();
let types = ds.create_readonly_repository::<UserType>(
    Some(connection.clone()),
    Some(type_options.read),
);
let roles =
    ds.create_readonly_repository::<UserRole>(Some(connection), Some(role_options.read));
let strategy = Arc::new(HashsetOptimizedReadonlyEnrichmentStrategy);
users_repo.add_enriched(
    datasources::Enrichment::new("UserType", "UserType", "Id", Some("Name".into()), false),
    types,
    Some(strategy.clone()),
);
users_repo.add_enriched(
    datasources::Enrichment::new("Roles", "Key", "UserId", None, true),
    roles,
    Some(strategy),
);
let users_view = users_repo
    .read_single(Some(&Where::binary("Key", eq, ADA)), None)
    .await
    .unwrap()
    .unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/EnrichedReadonlyRepository.test.ts#L98-L114)

```typescript
const person: Seed = { name: "person" };
const gone: Seed = { name: "gone" };
const typesConnection = await setup([person, gone]);
await remove(typesConnection, gone);
const connection = await setup([{ type_id: person.id }, { type_id: gone.id }]);
const parentsRepo = new ReadonlyRepository(Parent, connection);
const typesRepo = new ReadonlyRepository(TypeRow, typesConnection);
parentsRepo.addEnriched(
  { into: "type_name", from: "type_id", by: "id", select: "name" },
  typesRepo,
  new HashsetOptimizedReadonlyEnrichmentStrategy(),
);
const items = await collectAsync(parentsRepo.read());
```

## NestedToObjectEnrichmentStrategy

Read fill that collapses join rows into objects. It ignores the related repository and maps `sources` with a [TypeMap](#typemap).

### Classes

##### TypeScript

##### NestedToObjectEnrichmentStrategy {#typescript-nestedtoobjectenrichmentstrategy}

```typescript
class NestedToObjectEnrichmentStrategy<T extends object> extends DefaultReadonlyEnrichmentStrategy<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [new NestedToObjectEnrichmentStrategy(typeMap)](#typescript-nestedtoobjectenrichmentstrategy-new-nestedtoobjectenrichmentstrategy-typemap) | Builds a collapse strategy from a [TypeMap](#typescript-typemap). |

**Methods**

| Name | Description |
| --- | --- |
| [apply(via, enrichment, parents, sources?)](#typescript-nestedtoobjectenrichmentstrategy-apply-via-enrichment-parents-sources) | Replaces `parents` with the objects collapsed from `sources`. |

###### new NestedToObjectEnrichmentStrategy(typeMap) {#typescript-nestedtoobjectenrichmentstrategy-new-nestedtoobjectenrichmentstrategy-typemap}

Builds a collapse strategy from a [TypeMap](#typescript-typemap).

**Definition**

```typescript
constructor(typeMap: TypeMap<T>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `typeMap` | [TypeMap](#typescript-typemap) used to collapse join rows |

**Returns**

[NestedToObjectEnrichmentStrategy](#typescript-nestedtoobjectenrichmentstrategy)

###### apply(via, enrichment, parents, sources?) {#typescript-nestedtoobjectenrichmentstrategy-apply-via-enrichment-parents-sources}

Replaces `parents` with the objects collapsed from `sources`.

**Definition**

```typescript
apply<TRelated extends object>(via: IReadonlyRepository<TRelated>, enrichment: Enrichment, parents: object[], sources?: (Row | undefined)[]): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | unused |
| `enrichment` | unused |
| `parents` | replaced with the collapsed objects |
| `sources` | join rows aligned with the read |

**Returns**

`Promise<void>`

##### C#

##### NestedToObjectEnrichmentStrategy {#csharp-nestedtoobjectenrichmentstrategy}

```csharp
public sealed class NestedToObjectEnrichmentStrategy<T> : DefaultReadonlyEnrichmentStrategy<T> where T : new()
```

**Constructor**

| Name | Description |
| --- | --- |
| [NestedToObjectEnrichmentStrategy(typeMap)](#csharp-nestedtoobjectenrichmentstrategy-nestedtoobjectenrichmentstrategy-typemap) | Builds a collapse strategy from a [TypeMap](#csharp-typemap). |

**Methods**

| Name | Description |
| --- | --- |
| [Apply(via, enrichment, parents, sources)](#csharp-nestedtoobjectenrichmentstrategy-apply-via-enrichment-parents-sources) | Replaces `parents` with the objects collapsed from `sources`. |

###### NestedToObjectEnrichmentStrategy(typeMap) {#csharp-nestedtoobjectenrichmentstrategy-nestedtoobjectenrichmentstrategy-typemap}

Builds a collapse strategy from a [TypeMap](#csharp-typemap).

**Definition**

```csharp
public NestedToObjectEnrichmentStrategy(TypeMap typeMap)
```

**Parameters**

| Name | Description |
| --- | --- |
| `typeMap` | [TypeMap](#csharp-typemap) used to collapse join rows |

**Returns**

[NestedToObjectEnrichmentStrategy](#csharp-nestedtoobjectenrichmentstrategy)`<T>`

###### Apply(via, enrichment, parents, sources) {#csharp-nestedtoobjectenrichmentstrategy-apply-via-enrichment-parents-sources}

Replaces `parents` with the objects collapsed from `sources`.

**Definition**

```csharp
public override Task Apply<TRelated>(IReadonlyRepository<TRelated> via, Enrichment enrichment, IList<object> parents, IReadOnlyList<Row?>? sources = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | unused |
| `enrichment` | unused |
| `parents` | replaced with the collapsed objects |
| `sources` | join rows aligned with the read |

**Returns**

`Task`

##### Rust

##### NestedToObjectEnrichmentStrategy {#rust-nestedtoobjectenrichmentstrategy}

```rust
pub struct NestedToObjectEnrichmentStrategy
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(type_map)](#rust-nestedtoobjectenrichmentstrategy-new-type_map) | Builds a collapse strategy from a [TypeMap](#rust-typemap). |

**Methods**

| Name | Description |
| --- | --- |
| [apply(via, enrichment, parents, sources)](#rust-nestedtoobjectenrichmentstrategy-apply-via-enrichment-parents-sources) | Replaces `parents` with the objects collapsed from `sources`. |

###### new(type_map) {#rust-nestedtoobjectenrichmentstrategy-new-type_map}

Builds a collapse strategy from a [TypeMap](#rust-typemap).

**Definition**

```rust
pub fn new(type_map: TypeMap) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `type_map` | [TypeMap](#rust-typemap) used to collapse join rows |

**Returns**

[NestedToObjectEnrichmentStrategy](#rust-nestedtoobjectenrichmentstrategy)

###### apply(via, enrichment, parents, sources) {#rust-nestedtoobjectenrichmentstrategy-apply-via-enrichment-parents-sources}

Replaces `parents` with the objects collapsed from `sources`.

**Definition**

```rust
async fn apply(&self, via: Arc<dyn IReadonlyRepository>, enrichment: &Enrichment, parents: &mut Vec<Row>, sources: Option<&[Option<Row>]>) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `via` | unused |
| `enrichment` | unused |
| `parents` | replaced with the collapsed objects |
| `sources` | join rows aligned with the read |

**Returns**

`Result<()>`

### Remarks

This is the read-side collapse. [FlatToNestedMapper](#flat-to-nested-mapper) is the mapper. Pass this strategy to [IEnriched](#ienriched) when the repository read is already a join. The related repository is not queried.

### Examples

#### Collapse join rows

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/EnrichedReadonlyRepositoryTests.cs#L176-L192)

```csharp
var person = new Row { ["type_name"] = "person" };
var org = new Row { ["type_name"] = "org" };
var connection = await Setup(person, org);
IList<object> parents = [];
var parentsRepo = new ReadonlyRepository<Parent>(connection);
await new NestedToObjectEnrichmentStrategy<Parent>(new TypeMap
{
    Key = "id",
    Fields = new Dictionary<string, TypeMapField> { ["id"] = "id", ["type_name"] = "type_name" }
}).Apply(parentsRepo, new("type_name", "id", "id"), parents, [person, org]);
var items = parents.Cast<Parent>();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L425-L502)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
let connection = open(ds, "user-roles-read", &seed()).await;
let command = Arc::new(SelectStatement::new(SelectStatementOptions {
    joins: &[
        Join::left("user_type", "user_type.id", "user.user_type_id"),
        Join::left("user_roles", "user_roles.user_id", "user.id"),
        Join::left("roles", "roles.id", "user_roles.role_id"),
    ],
    ..SelectStatementOptions::new(
        "user",
        &[
            "user.id AS Key",
            "user.username AS Login",
            "user.is_enabled AS Active",
            "user.created AS Opened",
            "user_type.name AS UserType",
            "user.id AS UserId",
            "user_roles.role_id AS RoleId",
            "roles.name AS RoleName",
        ],
    )
}));
let options = ReadonlyRepositoryOptions::default();
let mut users_repo = ReadonlyRepository::new(connection.clone(), options).unwrap();
users_repo.add_enriched(
    datasources::Enrichment::new("Roles", "Key", "UserId", None, true),
    Arc::new(ReadonlyRepository::memory(vec![])),
    Some(Arc::new(NestedToObjectEnrichmentStrategy::new(
        TypeMap::new(
            "Key",
            vec![
                ("Key".into(), TypeMapField::column("Key")),
                ("Login".into(), TypeMapField::column("Login")),
                ("Active".into(), TypeMapField::column("Active")),
                ("Opened".into(), TypeMapField::column("Opened")),
                ("UserType".into(), TypeMapField::column("UserType")),
                (
                    "Roles".into(),
                    TypeMapField::Group(TypeMapGroup::new(
                        "RoleId",
                        vec![
                            ("RoleId".into(), TypeMapField::column("RoleId")),
                            ("UserId".into(), TypeMapField::column("UserId")),
                            ("Name".into(), TypeMapField::column("RoleName")),
                        ],
                    )),
                ),
            ],
        ),
    ))),
);
let items = users_repo.read(Some(command.as_ref()), None).await.unwrap();
let view = |login: &str| {
    items
        .iter()
        .find(|item| item.get("Login") == Some(&Value::from(login)))
        .unwrap()
        .clone()
};
let ada = view("ada");
let grace = view("grace");
let lin = view("lin");
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/EnrichedReadonlyRepository.test.ts#L303-L316)

```typescript
const person: Seed = { type_name: "person" };
const org: Seed = { type_name: "org" };
const connection = await setup([person, org]);
const parents: object[] = [];
const parentsRepo = new ReadonlyRepository(Parent, connection);
await new NestedToObjectEnrichmentStrategy<Parent>({
  key: "id",
  fields: { id: "id", type_name: "type_name" },
}).apply(parentsRepo, { into: "type_name", from: "id", by: "id" }, parents, [person, org]);
const items = parents as Parent[];
```

## IWriteEnrichmentStrategy

Writes nested rows with a parent create, update, or delete. [CRUDRepository](#crud-repository) calls `Persist` / `persist` / `persist` after the parent write when the repository has enrichments.

### Classes

##### TypeScript

##### IWriteEnrichmentStrategy {#typescript-iwriteenrichmentstrategy}

```typescript
interface IWriteEnrichmentStrategy<T>
```

**Properties**

| Name | Description |
| --- | --- |
| useTransaction | When true, the parent write and nested persist commit together. |

**Methods**

| Name | Description |
| --- | --- |
| [persist(tree, items, mode)](#typescript-iwriteenrichmentstrategy-persist-tree-items-mode) | Writes each nested edge on `tree` for `items`. |

###### persist(tree, items, mode) {#typescript-iwriteenrichmentstrategy-persist-tree-items-mode}

Writes each nested edge on `tree` for `items`.

**Definition**

```typescript
persist(tree: IRepositoryTree, items: T[], mode: WriteMode): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `tree` | [IRepositoryTree](#typescript-irepositorytree) |
| `items` | parents to persist |
| `mode` | [WriteMode](#typescript-writemode) |

**Returns**

`Promise<void>`

##### WriteMode {#typescript-writemode}

```typescript
type WriteMode = "create" | "update" | "delete"
```

##### C#

##### IWriteEnrichmentStrategy {#csharp-iwriteenrichmentstrategy}

```csharp
public interface IWriteEnrichmentStrategy<T>
```

**Properties**

| Name | Description |
| --- | --- |
| UseTransaction | When true, the parent write and nested persist commit together. The interface default is false. |

**Methods**

| Name | Description |
| --- | --- |
| [Persist(tree, items, mode)](#csharp-iwriteenrichmentstrategy-persist-tree-items-mode) | Writes each nested edge on `tree` for `items`. |

###### Persist(tree, items, mode) {#csharp-iwriteenrichmentstrategy-persist-tree-items-mode}

Writes each nested edge on `tree` for `items`.

**Definition**

```csharp
Task Persist(IRepositoryTree tree, IList<T> items, WriteMode mode)
```

**Parameters**

| Name | Description |
| --- | --- |
| `tree` | [IRepositoryTree](#csharp-irepositorytree) |
| `items` | parents to persist |
| `mode` | [WriteMode](#csharp-writemode) |

**Returns**

`Task`

##### WriteMode {#csharp-writemode}

```csharp
public enum WriteMode
```

**Properties**

| Name | Description |
| --- | --- |
| Create | Write children and links after the parent insert. |
| Update | Replace-sync or recreate children and links after the parent update. |
| Delete | Remove children and links before the parent delete. |

##### Rust

##### WriteEnrichmentStrategy {#rust-writeenrichmentstrategy}

```rust
pub trait WriteEnrichmentStrategy: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [use_transaction()](#rust-writeenrichmentstrategy-use_transaction) | When true, the parent write and nested persist commit together. The default is false. |
| [persist(tree, items, mode)](#rust-writeenrichmentstrategy-persist-tree-items-mode) | Writes each nested edge on `tree` for `items`. |

###### use_transaction() {#rust-writeenrichmentstrategy-use_transaction}

When true, the parent write and nested persist commit together. The default is false.

**Definition**

```rust
fn use_transaction(&self) -> bool
```

**Returns**

`bool`

###### persist(tree, items, mode) {#rust-writeenrichmentstrategy-persist-tree-items-mode}

Writes each nested edge on `tree` for `items`.

**Definition**

```rust
async fn persist(&self, tree: &RepositoryTree, items: &mut [Row], mode: WriteMode) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `tree` | [RepositoryTree](#rust-repositorytree) |
| `items` | parents to persist |
| `mode` | [WriteMode](#rust-writemode) |

**Returns**

`Result<()>`

##### WriteMode {#rust-writemode}

```rust
pub enum WriteMode
```

**Properties**

| Name | Description |
| --- | --- |
| Create | Write children and links after the parent insert. |
| Update | Replace-sync or recreate children and links after the parent update. |
| Delete | Remove children and links before the parent delete. |

### Remarks

Set this as the repository `write` option. [DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy) is the default. Lookups are filled on read and are not written. A `many` child list and a link are written. A bulk list uses one transaction for the whole list when `UseTransaction` / `useTransaction` / `use_transaction` is on.

### Examples

#### Roll back a bulk update

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/BulkCRUDRepositoryTests.cs#L100-L119)

```csharp
var connection = await Setup();
var parentsRepo = new BulkCRUDRepository<Parent>(connection, new CRUDRepositoryOptions<Parent>
{
    Write = new DefaultWriteEnrichmentStrategy<Parent>(useTransaction: true),
});
var first = await parentsRepo.Create(new Parent { label = "a" });
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/enriched.rs#L574-L621)

```rust
let connection = setup(&mut []).await;
let mut options = CRUDRepositoryOptions::default();
options.entity = "Parent".into();
options.write = Some(Arc::new(DefaultWriteEnrichmentStrategy::new(
    None, None, true,
)));
let parents = BulkCRUDRepository::new(connection, options).unwrap();
let created = parents
    .create_bulk(&mut [
        record(&[("label", Value::from("a"))]),
        record(&[("label", Value::from("b"))]),
    ])
    .await
    .unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/BulkCRUDRepository.test.ts#L86-L109)

```typescript
const parentsRepo = new BulkCRUDRepository<Parent>(await setup(), {
  type: Parent,
  write: new DefaultWriteEnrichmentStrategy(undefined, undefined, true),
});
const first = await parentsRepo.create(Object.assign(new Parent(), { label: "a" }));
```

## DefaultWriteEnrichmentStrategy

Default nested write. Create writes children and links after the parent. Update uses [DefaultUpdateEnrichmentStrategy](#defaultupdateenrichmentstrategy). Delete uses [CascadeDeleteEnrichmentStrategy](#cascadedeleteenrichmentstrategy). Both can be replaced.

### Classes

##### TypeScript

##### DefaultWriteEnrichmentStrategy {#typescript-defaultwriteenrichmentstrategy}

```typescript
class DefaultWriteEnrichmentStrategy<T> implements IWriteEnrichmentStrategy<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [new DefaultWriteEnrichmentStrategy(deleteStrategy?, updateStrategy?, useTransaction?)](#typescript-defaultwriteenrichmentstrategy-new-defaultwriteenrichmentstrategy-deletestrategy-updatestrategy-usetransaction) | Builds the default write strategy. |

**Properties**

| Name | Description |
| --- | --- |
| useTransaction | When true, the parent write and nested persist commit together. |

**Methods**

| Name | Description |
| --- | --- |
| [persist(tree, items, mode)](#typescript-defaultwriteenrichmentstrategy-persist-tree-items-mode) | Walks each enrichment and writes children or links for `mode`. |

###### new DefaultWriteEnrichmentStrategy(deleteStrategy?, updateStrategy?, useTransaction?) {#typescript-defaultwriteenrichmentstrategy-new-defaultwriteenrichmentstrategy-deletestrategy-updatestrategy-usetransaction}

Builds the default write strategy.

**Definition**

```typescript
constructor(deleteStrategy?: DeleteEnrichmentStrategy<T>, updateStrategy?: UpdateEnrichmentStrategy<T>, useTransaction?: boolean)
```

**Parameters**

| Name | Description |
| --- | --- |
| `deleteStrategy` | [DeleteEnrichmentStrategy](#deleteenrichmentstrategy). Omitted uses [CascadeDeleteEnrichmentStrategy](#cascadedeleteenrichmentstrategy) |
| `updateStrategy` | [UpdateEnrichmentStrategy](#updateenrichmentstrategy). Omitted uses [DefaultUpdateEnrichmentStrategy](#defaultupdateenrichmentstrategy) |
| `useTransaction` | commit the parent write and nested persist together. Off by default |

**Returns**

[DefaultWriteEnrichmentStrategy](#typescript-defaultwriteenrichmentstrategy)

###### persist(tree, items, mode) {#typescript-defaultwriteenrichmentstrategy-persist-tree-items-mode}

Walks each enrichment and writes children or links for `mode`.

**Definition**

```typescript
persist(tree: IRepositoryTree, items: T[], mode: WriteMode): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `tree` | [IRepositoryTree](#typescript-irepositorytree) |
| `items` | parents to persist |
| `mode` | [WriteMode](#typescript-writemode) |

**Returns**

`Promise<void>`

##### C#

##### DefaultWriteEnrichmentStrategy {#csharp-defaultwriteenrichmentstrategy}

```csharp
public class DefaultWriteEnrichmentStrategy<T> : IWriteEnrichmentStrategy<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [DefaultWriteEnrichmentStrategy(delete, update, useTransaction)](#csharp-defaultwriteenrichmentstrategy-defaultwriteenrichmentstrategy-delete-update-usetransaction) | Builds the default write strategy. |

**Properties**

| Name | Description |
| --- | --- |
| UseTransaction | When true, the parent write and nested persist commit together. |

**Methods**

| Name | Description |
| --- | --- |
| [Persist(tree, items, mode)](#csharp-defaultwriteenrichmentstrategy-persist-tree-items-mode) | Walks each enrichment and writes children or links for `mode`. |

###### DefaultWriteEnrichmentStrategy(delete, update, useTransaction) {#csharp-defaultwriteenrichmentstrategy-defaultwriteenrichmentstrategy-delete-update-usetransaction}

Builds the default write strategy.

**Definition**

```csharp
public DefaultWriteEnrichmentStrategy(DeleteEnrichmentStrategy<T>? delete = null, UpdateEnrichmentStrategy<T>? update = null, bool useTransaction = false)
```

**Parameters**

| Name | Description |
| --- | --- |
| `delete` | [DeleteEnrichmentStrategy](#deleteenrichmentstrategy). Omitted uses [CascadeDeleteEnrichmentStrategy](#cascadedeleteenrichmentstrategy) |
| `update` | [UpdateEnrichmentStrategy](#updateenrichmentstrategy). Omitted uses [DefaultUpdateEnrichmentStrategy](#defaultupdateenrichmentstrategy) |
| `useTransaction` | commit the parent write and nested persist together. Off by default |

**Returns**

[DefaultWriteEnrichmentStrategy](#csharp-defaultwriteenrichmentstrategy)`<T>`

###### Persist(tree, items, mode) {#csharp-defaultwriteenrichmentstrategy-persist-tree-items-mode}

Walks each enrichment and writes children or links for `mode`.

**Definition**

```csharp
public Task Persist(IRepositoryTree tree, IList<T> items, WriteMode mode)
```

**Parameters**

| Name | Description |
| --- | --- |
| `tree` | [IRepositoryTree](#csharp-irepositorytree) |
| `items` | parents to persist |
| `mode` | [WriteMode](#csharp-writemode) |

**Returns**

`Task`

##### Rust

##### DefaultWriteEnrichmentStrategy {#rust-defaultwriteenrichmentstrategy}

```rust
pub struct DefaultWriteEnrichmentStrategy
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(delete, update, use_transaction)](#rust-defaultwriteenrichmentstrategy-new-delete-update-use_transaction) | Builds the default write strategy. |
| [lazy(delete, update, use_transaction)](#rust-defaultwriteenrichmentstrategy-lazy-delete-update-use_transaction) | Builds the same strategy and skips a null list on create and update. [LazyWriteEnrichmentStrategy](#lazywriteenrichmentstrategy) returns this. |

**Methods**

| Name | Description |
| --- | --- |
| [use_transaction()](#rust-defaultwriteenrichmentstrategy-use_transaction) | When true, the parent write and nested persist commit together. |
| [persist(tree, items, mode)](#rust-defaultwriteenrichmentstrategy-persist-tree-items-mode) | Walks each enrichment and writes children or links for `mode`. |

###### new(delete, update, use_transaction) {#rust-defaultwriteenrichmentstrategy-new-delete-update-use_transaction}

Builds the default write strategy.

**Definition**

```rust
pub fn new(delete: Option<Arc<dyn DeleteEnrichmentStrategy>>, update: Option<Arc<dyn UpdateEnrichmentStrategy>>, use_transaction: bool) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `delete` | [DeleteEnrichmentStrategy](#deleteenrichmentstrategy). Omitted uses [CascadeDeleteEnrichmentStrategy](#cascadedeleteenrichmentstrategy) |
| `update` | [UpdateEnrichmentStrategy](#updateenrichmentstrategy). Omitted uses [DefaultUpdateEnrichmentStrategy](#defaultupdateenrichmentstrategy) |
| `use_transaction` | commit the parent write and nested persist together |

**Returns**

[DefaultWriteEnrichmentStrategy](#rust-defaultwriteenrichmentstrategy)

###### lazy(delete, update, use_transaction) {#rust-defaultwriteenrichmentstrategy-lazy-delete-update-use_transaction}

Builds the same strategy and skips a null list on create and update. [LazyWriteEnrichmentStrategy](#lazywriteenrichmentstrategy) returns this.

**Definition**

```rust
pub fn lazy(delete: Option<Arc<dyn DeleteEnrichmentStrategy>>, update: Option<Arc<dyn UpdateEnrichmentStrategy>>, use_transaction: bool) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `delete` | [DeleteEnrichmentStrategy](#deleteenrichmentstrategy). Omitted uses [CascadeDeleteEnrichmentStrategy](#cascadedeleteenrichmentstrategy) |
| `update` | [UpdateEnrichmentStrategy](#updateenrichmentstrategy). Omitted uses [DefaultUpdateEnrichmentStrategy](#defaultupdateenrichmentstrategy) |
| `use_transaction` | commit the parent write and nested persist together |

**Returns**

[DefaultWriteEnrichmentStrategy](#rust-defaultwriteenrichmentstrategy)

###### use_transaction() {#rust-defaultwriteenrichmentstrategy-use_transaction}

When true, the parent write and nested persist commit together.

**Definition**

```rust
fn use_transaction(&self) -> bool
```

**Returns**

`bool`

###### persist(tree, items, mode) {#rust-defaultwriteenrichmentstrategy-persist-tree-items-mode}

Walks each enrichment and writes children or links for `mode`.

**Definition**

```rust
async fn persist(&self, tree: &RepositoryTree, items: &mut [Row], mode: WriteMode) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `tree` | [RepositoryTree](#rust-repositorytree) |
| `items` | parents to persist |
| `mode` | [WriteMode](#rust-writemode) |

**Returns**

`Result<()>`

### Remarks

Set it on the repository `write` option. Create writes the parent, stamps each child's `by` from the parent `from`, then creates children and links. Update and delete delegate to the strategies passed here. A null or empty list clears a link when this strategy is eager. [LazyWriteEnrichmentStrategy](#lazywriteenrichmentstrategy) skips that null list. Nested writes join the transaction when they use the same connection.

### Examples

#### Roll back a bulk update

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/BulkCRUDRepositoryTests.cs#L100-L119)

```csharp
var connection = await Setup();
var parentsRepo = new BulkCRUDRepository<Parent>(connection, new CRUDRepositoryOptions<Parent>
{
    Write = new DefaultWriteEnrichmentStrategy<Parent>(useTransaction: true),
});
var first = await parentsRepo.Create(new Parent { label = "a" });
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/enriched.rs#L574-L621)

```rust
let connection = setup(&mut []).await;
let mut options = CRUDRepositoryOptions::default();
options.entity = "Parent".into();
options.write = Some(Arc::new(DefaultWriteEnrichmentStrategy::new(
    None, None, true,
)));
let parents = BulkCRUDRepository::new(connection, options).unwrap();
let created = parents
    .create_bulk(&mut [
        record(&[("label", Value::from("a"))]),
        record(&[("label", Value::from("b"))]),
    ])
    .await
    .unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/BulkCRUDRepository.test.ts#L86-L109)

```typescript
const parentsRepo = new BulkCRUDRepository<Parent>(await setup(), {
  type: Parent,
  write: new DefaultWriteEnrichmentStrategy(undefined, undefined, true),
});
const first = await parentsRepo.create(Object.assign(new Parent(), { label: "a" }));
```

## LazyWriteEnrichmentStrategy

Nested write that skips a null list on create and update. Delete still removes the junction rows and owned children. The rest matches [DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy).

### Classes

##### TypeScript

##### LazyWriteEnrichmentStrategy {#typescript-lazywriteenrichmentstrategy}

```typescript
class LazyWriteEnrichmentStrategy<T> extends DefaultWriteEnrichmentStrategy<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [new LazyWriteEnrichmentStrategy(deleteStrategy?, updateStrategy?, useTransaction?)](#typescript-lazywriteenrichmentstrategy-new-lazywriteenrichmentstrategy-deletestrategy-updatestrategy-usetransaction) | Builds the lazy write strategy. |

###### new LazyWriteEnrichmentStrategy(deleteStrategy?, updateStrategy?, useTransaction?) {#typescript-lazywriteenrichmentstrategy-new-lazywriteenrichmentstrategy-deletestrategy-updatestrategy-usetransaction}

Builds the lazy write strategy.

**Definition**

```typescript
constructor(deleteStrategy?: DeleteEnrichmentStrategy<T>, updateStrategy?: UpdateEnrichmentStrategy<T>, useTransaction?: boolean)
```

**Parameters**

| Name | Description |
| --- | --- |
| `deleteStrategy` | [DeleteEnrichmentStrategy](#deleteenrichmentstrategy). Omitted uses [CascadeDeleteEnrichmentStrategy](#cascadedeleteenrichmentstrategy) |
| `updateStrategy` | [UpdateEnrichmentStrategy](#updateenrichmentstrategy). Omitted uses [DefaultUpdateEnrichmentStrategy](#defaultupdateenrichmentstrategy) |
| `useTransaction` | commit the parent write and nested persist together. Off by default |

**Returns**

[LazyWriteEnrichmentStrategy](#typescript-lazywriteenrichmentstrategy)

##### C#

##### LazyWriteEnrichmentStrategy {#csharp-lazywriteenrichmentstrategy}

```csharp
public sealed class LazyWriteEnrichmentStrategy<T> : DefaultWriteEnrichmentStrategy<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [LazyWriteEnrichmentStrategy(delete, update, useTransaction)](#csharp-lazywriteenrichmentstrategy-lazywriteenrichmentstrategy-delete-update-usetransaction) | Builds the lazy write strategy. |

###### LazyWriteEnrichmentStrategy(delete, update, useTransaction) {#csharp-lazywriteenrichmentstrategy-lazywriteenrichmentstrategy-delete-update-usetransaction}

Builds the lazy write strategy.

**Definition**

```csharp
public LazyWriteEnrichmentStrategy(DeleteEnrichmentStrategy<T>? delete = null, UpdateEnrichmentStrategy<T>? update = null, bool useTransaction = false)
```

**Parameters**

| Name | Description |
| --- | --- |
| `delete` | [DeleteEnrichmentStrategy](#deleteenrichmentstrategy). Omitted uses [CascadeDeleteEnrichmentStrategy](#cascadedeleteenrichmentstrategy) |
| `update` | [UpdateEnrichmentStrategy](#updateenrichmentstrategy). Omitted uses [DefaultUpdateEnrichmentStrategy](#defaultupdateenrichmentstrategy) |
| `useTransaction` | commit the parent write and nested persist together. Off by default |

**Returns**

[LazyWriteEnrichmentStrategy](#csharp-lazywriteenrichmentstrategy)`<T>`

##### Rust

##### LazyWriteEnrichmentStrategy {#rust-lazywriteenrichmentstrategy}

```rust
pub struct LazyWriteEnrichmentStrategy
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(delete, update, use_transaction)](#rust-lazywriteenrichmentstrategy-new-delete-update-use_transaction) | Builds a [DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy) that skips a null list on create and update. |

###### new(delete, update, use_transaction) {#rust-lazywriteenrichmentstrategy-new-delete-update-use_transaction}

Builds a [DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy) that skips a null list on create and update.

**Definition**

```rust
pub fn new(delete: Option<Arc<dyn DeleteEnrichmentStrategy>>, update: Option<Arc<dyn UpdateEnrichmentStrategy>>, use_transaction: bool) -> DefaultWriteEnrichmentStrategy
```

**Parameters**

| Name | Description |
| --- | --- |
| `delete` | [DeleteEnrichmentStrategy](#deleteenrichmentstrategy). Omitted uses [CascadeDeleteEnrichmentStrategy](#cascadedeleteenrichmentstrategy) |
| `update` | [UpdateEnrichmentStrategy](#updateenrichmentstrategy). Omitted uses [DefaultUpdateEnrichmentStrategy](#defaultupdateenrichmentstrategy) |
| `use_transaction` | commit the parent write and nested persist together |

**Returns**

[DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy)

### Remarks

A null list on create or update leaves that association as it is. An empty list still clears it. Delete still removes junction rows and owned children. Set it as the repository `write` option.

> **Rust**
>
> `new` returns [DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy).

### Examples

#### Skip unloaded lists

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/UserViewCRUDRepositoryTests.cs#L301-L366)

```csharp
var graph = await GraphAsync(new LazyWriteEnrichmentStrategy<UserView>());
var userViewRepo = Enrich(graph.UsersRepo, graph);

var created = await userViewRepo.Create(new UserView
{
    Username = "ada",
    IsEnabled = true,
    UserTypeId = graph.AdminType.Id,
    Roles = null,
    Groups = [graph.Engineering]
});
var createdRoleLinks = await graph.UserRolesRepo.Read().ToListAsync();
var createdGroupLinks = await graph.UserGroupsRepo.Read().ToListAsync();
var skipped = await userViewRepo.Update(new UserView
{
    Id = created.Id,
    Username = "ada",
    IsEnabled = false,
    UserTypeId = graph.AdminType.Id,
    Roles = null,
    Groups = null
});
var afterSkip = await userViewRepo.ReadSingleById(created.Id);
var skippedRoleLinks = await graph.UserRolesRepo.Read().ToListAsync();
var skippedGroupLinks = await graph.UserGroupsRepo.Read().ToListAsync();
var linked = await userViewRepo.Update(new UserView
{
    Id = created.Id,
    Username = "ada",
    IsEnabled = false,
    UserTypeId = graph.AdminType.Id,
    Roles = [graph.EditorRole],
    Groups = null
});
var linkedRoleLinks = await graph.UserRolesRepo.Read().ToListAsync();
var linkedGroupLinks = await graph.UserGroupsRepo.Read().ToListAsync();
var roles = await graph.RolesRepo.Read().ToListAsync();
var groups = await graph.GroupsRepo.Read().ToListAsync();
var removed = await userViewRepo.Delete(new UserView { Id = created.Id, Roles = null, Groups = null });
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/user_view.rs#L459-L560)

```rust
let (mut users, seeded) = open(Some(Arc::new(LazyWriteEnrichmentStrategy::new(
    None, None, false,
))))
.await;
link(&mut users, &seeded);
let mut creating = record(&[
    ("username", Value::from("ada")),
    ("is_enabled", Value::Bool(true)),
    ("user_type_id", seeded.admin_type.get("id").unwrap().clone()),
    ("groups", encode_rows(&[seeded.engineering.clone()])),
]);
let created = users.create(&mut creating).await.unwrap();
let created_role_links = seeded.user_roles.read(None, None).await.unwrap();
let created_group_links = seeded.user_groups.read(None, None).await.unwrap();
let next = record(&[
    ("id", created.get("id").unwrap().clone()),
    ("username", Value::from("ada")),
    ("is_enabled", Value::Bool(false)),
    ("user_type_id", seeded.admin_type.get("id").unwrap().clone()),
]);
let skipped = users.update(&next, None).await.unwrap();
let after_skip = users
    .read_single_by_id(created.get("id").unwrap())
    .await
    .unwrap()
    .unwrap();
let skipped_role_links = seeded.user_roles.read(None, None).await.unwrap();
let skipped_group_links = seeded.user_groups.read(None, None).await.unwrap();
let linked_row = record(&[
    ("id", created.get("id").unwrap().clone()),
    ("username", Value::from("ada")),
    ("is_enabled", Value::Bool(false)),
    ("user_type_id", seeded.admin_type.get("id").unwrap().clone()),
    ("roles", encode_rows(&[seeded.editor_role.clone()])),
]);
let linked = users.update(&linked_row, None).await.unwrap();
let linked_role_links = seeded.user_roles.read(None, None).await.unwrap();
let linked_group_links = seeded.user_groups.read(None, None).await.unwrap();
let removed = users
    .delete(&record(&[("id", created.get("id").unwrap().clone())]), None)
    .await
    .unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/UserViewCRUDRepository.test.ts#L248-L302)

```typescript
const seeded = await graph(new LazyWriteEnrichmentStrategy());
link(seeded.usersRepo, seeded);
const created = await seeded.usersRepo.create(Object.assign(new UserView(), {
  username: "ada",
  is_enabled: true,
  user_type_id: seeded.adminType.id,
  roles: null,
  groups: [seeded.engineering],
}));
const createdRoleLinks = await collectAsync(seeded.userRolesRepo.read());
const createdGroupLinks = await collectAsync(seeded.userGroupsRepo.read());
const skipped = await seeded.usersRepo.update(Object.assign(new UserView(), {
  id: created.id,
  username: "ada",
  is_enabled: false,
  user_type_id: seeded.adminType.id,
}));
const afterSkip = await seeded.usersRepo.readSingleById(created.id);
const skippedRoleLinks = await collectAsync(seeded.userRolesRepo.read());
const skippedGroupLinks = await collectAsync(seeded.userGroupsRepo.read());
const linked = await seeded.usersRepo.update(Object.assign(new UserView(), {
  id: created.id,
  username: "ada",
  is_enabled: false,
  user_type_id: seeded.adminType.id,
  roles: [seeded.editorRole],
}));
const linkedRoleLinks = await collectAsync(seeded.userRolesRepo.read());
const linkedGroupLinks = await collectAsync(seeded.userGroupsRepo.read());
const removed = await seeded.usersRepo.delete(Object.assign(new UserView(), { id: created.id, roles: null, groups: null }));
```

## UpdateEnrichmentStrategy

How an update writes owned children and links. [DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy) calls these methods. [DefaultUpdateEnrichmentStrategy](#defaultupdateenrichmentstrategy) replace-syncs. [CascadeDeleteUpdateEnrichmentStrategy](#cascadedeleteupdateenrichmentstrategy) deletes, then inserts.

### Classes

##### TypeScript

##### UpdateEnrichmentStrategy {#typescript-updateenrichmentstrategy}

```typescript
abstract class UpdateEnrichmentStrategy<T>
```

**Methods**

| Name | Description |
| --- | --- |
| [updateOwned(entry, enrichment, parent)](#typescript-updateenrichmentstrategy-updateowned-entry-enrichment-parent) | Writes the owned `many` list for one parent. |
| [updateLinked(entry, enrichment, parent)](#typescript-updateenrichmentstrategy-updatelinked-entry-enrichment-parent) | Writes the junction rows for one parent. |
| [createChildren(writer, enrichment, parent)](#typescript-updateenrichmentstrategy-createchildren-writer-enrichment-parent) | Stamps each child's `by` from the parent `from`, then creates the payload. |
| [createLinks(writer, enrichment, parent)](#typescript-updateenrichmentstrategy-createlinks-writer-enrichment-parent) | Inserts one junction row per related id. |

###### updateOwned(entry, enrichment, parent) {#typescript-updateenrichmentstrategy-updateowned-entry-enrichment-parent}

Writes the owned `many` list for one parent.

**Definition**

```typescript
abstract updateOwned(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#typescript-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent object |

**Returns**

`Promise<void>`

###### updateLinked(entry, enrichment, parent) {#typescript-updateenrichmentstrategy-updatelinked-entry-enrichment-parent}

Writes the junction rows for one parent.

**Definition**

```typescript
abstract updateLinked(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#typescript-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent object |

**Returns**

`Promise<void>`

###### createChildren(writer, enrichment, parent) {#typescript-updateenrichmentstrategy-createchildren-writer-enrichment-parent}

Stamps each child's `by` from the parent `from`, then creates the payload.

**Definition**

```typescript
createChildren(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `writer` | child [ICRUDRepository](#crud-repository) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent object |

**Returns**

`Promise<void>`

###### createLinks(writer, enrichment, parent) {#typescript-updateenrichmentstrategy-createlinks-writer-enrichment-parent}

Inserts one junction row per related id.

**Definition**

```typescript
createLinks(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `writer` | link [ICRUDRepository](#crud-repository) |
| `enrichment` | [Enrichment](#enrichment) with [association](#typescript-association) |
| `parent` | parent object |

**Returns**

`Promise<void>`

##### StoredEnrichment {#typescript-storedenrichment}

```typescript
class StoredEnrichment
```

**Constructor**

| Name | Description |
| --- | --- |
| [new StoredEnrichment(enrichment, apply, persist?, link?)](#typescript-storedenrichment-new-storedenrichment-enrichment-apply-persist-link) | Stores one edge. |

**Properties**

| Name | Description |
| --- | --- |
| enrichment | The registered [Enrichment](#enrichment). |
| persist | Child repository for an owned list. |
| link | Junction repository for an association. |

###### new StoredEnrichment(enrichment, apply, persist?, link?) {#typescript-storedenrichment-new-storedenrichment-enrichment-apply-persist-link}

Stores one edge.

**Definition**

```typescript
constructor(enrichment: Enrichment, apply: ApplyEnrichment, persist?: ICRUDRepository<object>, link?: ICRUDRepository<object>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `enrichment` | [Enrichment](#enrichment) |
| `apply` | read fill for this edge |
| `persist` | child [ICRUDRepository](#crud-repository) for an owned list |
| `link` | junction [ICRUDRepository](#crud-repository) |

**Returns**

[StoredEnrichment](#typescript-storedenrichment)

##### C#

##### UpdateEnrichmentStrategy {#csharp-updateenrichmentstrategy}

```csharp
public abstract class UpdateEnrichmentStrategy<T>
```

**Methods**

| Name | Description |
| --- | --- |
| [UpdateOwned(entry, enrichment, parent)](#csharp-updateenrichmentstrategy-updateowned-entry-enrichment-parent) | Writes the owned `many` list for one parent. |
| [UpdateLinked(entry, enrichment, parent)](#csharp-updateenrichmentstrategy-updatelinked-entry-enrichment-parent) | Writes the junction rows for one parent. |

###### UpdateOwned(entry, enrichment, parent) {#csharp-updateenrichmentstrategy-updateowned-entry-enrichment-parent}

Writes the owned `many` list for one parent.

**Definition**

```csharp
public abstract Task UpdateOwned(StoredEnrichment entry, Enrichment enrichment, object parent)
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#csharp-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent object |

**Returns**

`Task`

###### UpdateLinked(entry, enrichment, parent) {#csharp-updateenrichmentstrategy-updatelinked-entry-enrichment-parent}

Writes the junction rows for one parent.

**Definition**

```csharp
public abstract Task UpdateLinked(StoredEnrichment entry, Enrichment enrichment, object parent)
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#csharp-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent object |

**Returns**

`Task`

##### StoredEnrichment {#csharp-storedenrichment}

```csharp
public sealed class StoredEnrichment
```

**Properties**

| Name | Description |
| --- | --- |
| Enrichment | The registered [Enrichment](#enrichment). |

##### Rust

##### UpdateEnrichmentStrategy {#rust-updateenrichmentstrategy}

```rust
pub trait UpdateEnrichmentStrategy: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [update_owned(entry, enrichment, parent)](#rust-updateenrichmentstrategy-update_owned-entry-enrichment-parent) | Writes the owned `many` list for one parent. |
| [update_linked(entry, enrichment, parent)](#rust-updateenrichmentstrategy-update_linked-entry-enrichment-parent) | Writes the junction rows for one parent. |

###### update_owned(entry, enrichment, parent) {#rust-updateenrichmentstrategy-update_owned-entry-enrichment-parent}

Writes the owned `many` list for one parent.

**Definition**

```rust
async fn update_owned(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#rust-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent row |

**Returns**

`Result<()>`

###### update_linked(entry, enrichment, parent) {#rust-updateenrichmentstrategy-update_linked-entry-enrichment-parent}

Writes the junction rows for one parent.

**Definition**

```rust
async fn update_linked(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#rust-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent row |

**Returns**

`Result<()>`

##### StoredEnrichment {#rust-storedenrichment}

```rust
pub struct StoredEnrichment
```

**Properties**

| Name | Description |
| --- | --- |
| enrichment | The registered [Enrichment](#enrichment). |
| via | Related read repository. |
| persist | Child repository for an owned list. |
| link | Junction repository for an association. |
| strategy | Read strategy for this edge. |

### Remarks

Create uses the same child and link inserts as an update of a new parent. A subclass implements the two update methods. TypeScript also exposes `createChildren` and `createLinks`. Pass a subclass to [DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy).

### Examples

#### Replace children

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/EnrichedCRUDRepositoryTests.cs#L129-L160)

```csharp
var connection = await Setup();
var parentsRepo = new CRUDRepository<Parent>(connection, new CRUDRepositoryOptions<Parent>
{
    Write = new DefaultWriteEnrichmentStrategy<Parent>(update: new CascadeDeleteUpdateEnrichmentStrategy<Parent>()),
});
var childrenRepo = new CRUDRepository<Child>(connection);
var notesRepo = new CRUDRepository<Note>(connection);
var parent = await parentsRepo.Create(new Parent());
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/enriched.rs#L362-L441)

```rust
let connection = setup(&mut []).await;
let mut parents = crud(connection.clone(), "Parent").with_write(Arc::new(
    DefaultWriteEnrichmentStrategy::new(
        None,
        Some(Arc::new(CascadeDeleteUpdateEnrichmentStrategy)),
        false,
    ),
));
let mut children_repo = crud(connection.clone(), "Child");
let notes = Arc::new(crud(connection, "Note"));
let parent = parents
    .create(&mut record(&[("label", Value::from("p"))]))
    .await
    .unwrap();
assert!(num(&parent, "id") > 0);
let parent_id = parent.get("id").unwrap().clone();
let child = children_repo
    .create(&mut record(&[
        ("parent_id", parent_id.clone()),
        ("label", Value::from("a")),
    ]))
    .await
    .unwrap();
assert!(num(&child, "id") > 0);
let other = children_repo
    .create(&mut record(&[("label", Value::from("other"))]))
    .await
    .unwrap();
assert!(num(&other, "id") > 0);
let child_id = child.get("id").unwrap().clone();
let note = notes
    .create(&mut record(&[
        ("child_id", child_id.clone()),
        ("text", Value::from("n")),
    ]))
    .await
    .unwrap();
assert!(num(&note, "id") > 0);
children_repo.add_enriched(
    Enrichment::new("notes", "id", "child_id", None, true),
    notes.clone(),
    None,
);
let children = Arc::new(children_repo);
parents.add_enriched(
    Enrichment::new("children", "id", "parent_id", None, true),
    children.clone(),
    None,
);
let updated = parents
    .update(
        &record(&[
            ("id", parent_id.clone()),
            ("label", Value::from("p")),
            (
                "children",
                encode_rows(&[record(&[
                    ("id", child_id.clone()),
                    ("label", Value::from("fresh")),
                ])]),
            ),
        ]),
        None,
    )
    .await
    .unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/EnrichedCRUDRepository.test.ts#L158-L186)

```typescript
const connection = await setup();
const parentsRepo = new CRUDRepository<Parent>(connection, {
  type: Parent,
  write: new DefaultWriteEnrichmentStrategy(undefined, new CascadeDeleteUpdateEnrichmentStrategy()),
});
const childrenRepo = new CRUDRepository(Child, connection);
const notesRepo = new CRUDRepository(Note, connection);
const parent = await parentsRepo.create(new Parent());
```

## DefaultUpdateEnrichmentStrategy

Replace-sync for an owned list and a link. Matching children update. Missing children delete. New children insert. A link is replaced the same way: junction rows that are gone are deleted, and new related ids are inserted.

### Classes

##### TypeScript

##### DefaultUpdateEnrichmentStrategy {#typescript-defaultupdateenrichmentstrategy}

```typescript
class DefaultUpdateEnrichmentStrategy<T> extends UpdateEnrichmentStrategy<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [new DefaultUpdateEnrichmentStrategy()](#typescript-defaultupdateenrichmentstrategy-new-defaultupdateenrichmentstrategy) | Builds the replace-sync update strategy. |

**Methods**

| Name | Description |
| --- | --- |
| [updateOwned(entry, enrichment, parent)](#typescript-defaultupdateenrichmentstrategy-updateowned-entry-enrichment-parent) | Loads existing children, deletes rows missing from the payload, updates matches, and creates the rest. |
| [updateLinked(entry, enrichment, parent)](#typescript-defaultupdateenrichmentstrategy-updatelinked-entry-enrichment-parent) | Replaces the junction set for the parent key. |

###### new DefaultUpdateEnrichmentStrategy() {#typescript-defaultupdateenrichmentstrategy-new-defaultupdateenrichmentstrategy}

Builds the replace-sync update strategy.

**Definition**

```typescript
constructor()
```

**Returns**

[DefaultUpdateEnrichmentStrategy](#typescript-defaultupdateenrichmentstrategy)

###### updateOwned(entry, enrichment, parent) {#typescript-defaultupdateenrichmentstrategy-updateowned-entry-enrichment-parent}

Loads existing children, deletes rows missing from the payload, updates matches, and creates the rest.

**Definition**

```typescript
updateOwned(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#typescript-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent object |

**Returns**

`Promise<void>`

###### updateLinked(entry, enrichment, parent) {#typescript-defaultupdateenrichmentstrategy-updatelinked-entry-enrichment-parent}

Replaces the junction set for the parent key.

**Definition**

```typescript
updateLinked(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#typescript-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) with [association](#typescript-association) |
| `parent` | parent object |

**Returns**

`Promise<void>`

##### C#

##### DefaultUpdateEnrichmentStrategy {#csharp-defaultupdateenrichmentstrategy}

```csharp
public sealed class DefaultUpdateEnrichmentStrategy<T> : UpdateEnrichmentStrategy<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [DefaultUpdateEnrichmentStrategy()](#csharp-defaultupdateenrichmentstrategy-defaultupdateenrichmentstrategy) | Builds the replace-sync update strategy. |

**Methods**

| Name | Description |
| --- | --- |
| [UpdateOwned(entry, enrichment, parent)](#csharp-defaultupdateenrichmentstrategy-updateowned-entry-enrichment-parent) | Loads existing children, deletes rows missing from the payload, updates matches, and creates the rest. |
| [UpdateLinked(entry, enrichment, parent)](#csharp-defaultupdateenrichmentstrategy-updatelinked-entry-enrichment-parent) | Replaces the junction set for the parent key. |

###### DefaultUpdateEnrichmentStrategy() {#csharp-defaultupdateenrichmentstrategy-defaultupdateenrichmentstrategy}

Builds the replace-sync update strategy.

**Definition**

```csharp
public DefaultUpdateEnrichmentStrategy()
```

**Returns**

[DefaultUpdateEnrichmentStrategy](#csharp-defaultupdateenrichmentstrategy)`<T>`

###### UpdateOwned(entry, enrichment, parent) {#csharp-defaultupdateenrichmentstrategy-updateowned-entry-enrichment-parent}

Loads existing children, deletes rows missing from the payload, updates matches, and creates the rest.

**Definition**

```csharp
public override Task UpdateOwned(StoredEnrichment entry, Enrichment enrichment, object parent)
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#csharp-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent object |

**Returns**

`Task`

###### UpdateLinked(entry, enrichment, parent) {#csharp-defaultupdateenrichmentstrategy-updatelinked-entry-enrichment-parent}

Replaces the junction set for the parent key.

**Definition**

```csharp
public override Task UpdateLinked(StoredEnrichment entry, Enrichment enrichment, object parent)
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#csharp-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) with [Association](#csharp-association) |
| `parent` | parent object |

**Returns**

`Task`

##### Rust

##### DefaultUpdateEnrichmentStrategy {#rust-defaultupdateenrichmentstrategy}

```rust
pub struct DefaultUpdateEnrichmentStrategy
```

**Methods**

| Name | Description |
| --- | --- |
| [update_owned(entry, enrichment, parent)](#rust-defaultupdateenrichmentstrategy-update_owned-entry-enrichment-parent) | Loads existing children, deletes rows missing from the payload, updates matches, and creates the rest. |
| [update_linked(entry, enrichment, parent)](#rust-defaultupdateenrichmentstrategy-update_linked-entry-enrichment-parent) | Replaces the junction set for the parent key. |

###### update_owned(entry, enrichment, parent) {#rust-defaultupdateenrichmentstrategy-update_owned-entry-enrichment-parent}

Loads existing children, deletes rows missing from the payload, updates matches, and creates the rest.

**Definition**

```rust
async fn update_owned(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#rust-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent row |

**Returns**

`Result<()>`

###### update_linked(entry, enrichment, parent) {#rust-defaultupdateenrichmentstrategy-update_linked-entry-enrichment-parent}

Replaces the junction set for the parent key.

**Definition**

```rust
async fn update_linked(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#rust-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) with [association](#rust-association) |
| `parent` | parent row |

**Returns**

`Result<()>`

### Remarks

This is the update strategy [DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy) uses when `update` is omitted. Identity comes from the child repository's id columns. A child with no id is created. Related rows that are only lookups stay.

### Examples

#### Replace-sync children

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/EnrichedCRUDRepositoryTests.cs#L99-L127)

```csharp
var connection = await Setup();
var parentsRepo = new CRUDRepository<Parent>(connection);
var childrenRepo = new CRUDRepository<Child>(connection);
var parent = await parentsRepo.Create(new Parent { type_id = 0 });
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/enriched.rs#L291-L360)

```rust
let connection = setup(&mut []).await;
let mut parents = crud(connection.clone(), "Parent");
let children = Arc::new(crud(connection, "Child"));
let parent = parents
    .create(&mut record(&[("label", Value::from("p"))]))
    .await
    .unwrap();
assert!(num(&parent, "id") > 0);
let parent_id = parent.get("id").unwrap().clone();
let kept = children
    .create(&mut record(&[
        ("parent_id", parent_id.clone()),
        ("label", Value::from("a")),
    ]))
    .await
    .unwrap();
assert!(num(&kept, "id") > 0);
let dropped = children
    .create(&mut record(&[
        ("parent_id", parent_id.clone()),
        ("label", Value::from("b")),
    ]))
    .await
    .unwrap();
assert!(num(&dropped, "id") > 0);
parents.add_enriched(
    Enrichment::new("children", "id", "parent_id", None, true),
    children.clone(),
    None,
);
let updated = parents
    .update(
        &record(&[
            ("id", parent_id.clone()),
            ("label", Value::from("p")),
            (
                "children",
                encode_rows(&[
                    record(&[
                        ("id", kept.get("id").unwrap().clone()),
                        ("label", Value::from("aa")),
                    ]),
                    record(&[("label", Value::from("c"))]),
                ]),
            ),
        ]),
        None,
    )
    .await
    .unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/EnrichedCRUDRepository.test.ts#L132-L156)

```typescript
const connection = await setup();
const parentsRepo = new CRUDRepository(Parent, connection);
const childrenRepo = new CRUDRepository(Child, connection);
const parent = await parentsRepo.create(new Parent());
```

## CascadeDeleteUpdateEnrichmentStrategy

Update that deletes every existing child, then inserts the payload as new rows. Recreated children get new ids. Links are replaced the same way. Related lookup rows stay.

### Classes

##### TypeScript

##### CascadeDeleteUpdateEnrichmentStrategy {#typescript-cascadedeleteupdateenrichmentstrategy}

```typescript
class CascadeDeleteUpdateEnrichmentStrategy<T> extends UpdateEnrichmentStrategy<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [new CascadeDeleteUpdateEnrichmentStrategy()](#typescript-cascadedeleteupdateenrichmentstrategy-new-cascadedeleteupdateenrichmentstrategy) | Builds the delete-then-insert update strategy. |

**Methods**

| Name | Description |
| --- | --- |
| [updateOwned(entry, enrichment, parent)](#typescript-cascadedeleteupdateenrichmentstrategy-updateowned-entry-enrichment-parent) | Deletes every existing child for the parent key, then inserts the payload. |
| [updateLinked(entry, enrichment, parent)](#typescript-cascadedeleteupdateenrichmentstrategy-updatelinked-entry-enrichment-parent) | Deletes every junction row for the parent key, then inserts the payload ids. |

###### new CascadeDeleteUpdateEnrichmentStrategy() {#typescript-cascadedeleteupdateenrichmentstrategy-new-cascadedeleteupdateenrichmentstrategy}

Builds the delete-then-insert update strategy.

**Definition**

```typescript
constructor()
```

**Returns**

[CascadeDeleteUpdateEnrichmentStrategy](#typescript-cascadedeleteupdateenrichmentstrategy)

###### updateOwned(entry, enrichment, parent) {#typescript-cascadedeleteupdateenrichmentstrategy-updateowned-entry-enrichment-parent}

Deletes every existing child for the parent key, then inserts the payload.

**Definition**

```typescript
updateOwned(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#typescript-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent object |

**Returns**

`Promise<void>`

###### updateLinked(entry, enrichment, parent) {#typescript-cascadedeleteupdateenrichmentstrategy-updatelinked-entry-enrichment-parent}

Deletes every junction row for the parent key, then inserts the payload ids.

**Definition**

```typescript
updateLinked(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#typescript-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) with [association](#typescript-association) |
| `parent` | parent object |

**Returns**

`Promise<void>`

##### C#

##### CascadeDeleteUpdateEnrichmentStrategy {#csharp-cascadedeleteupdateenrichmentstrategy}

```csharp
public sealed class CascadeDeleteUpdateEnrichmentStrategy<T> : UpdateEnrichmentStrategy<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [CascadeDeleteUpdateEnrichmentStrategy()](#csharp-cascadedeleteupdateenrichmentstrategy-cascadedeleteupdateenrichmentstrategy) | Builds the delete-then-insert update strategy. |

**Methods**

| Name | Description |
| --- | --- |
| [UpdateOwned(entry, enrichment, parent)](#csharp-cascadedeleteupdateenrichmentstrategy-updateowned-entry-enrichment-parent) | Deletes every existing child for the parent key, then inserts the payload. |
| [UpdateLinked(entry, enrichment, parent)](#csharp-cascadedeleteupdateenrichmentstrategy-updatelinked-entry-enrichment-parent) | Deletes every junction row for the parent key, then inserts the payload ids. |

###### CascadeDeleteUpdateEnrichmentStrategy() {#csharp-cascadedeleteupdateenrichmentstrategy-cascadedeleteupdateenrichmentstrategy}

Builds the delete-then-insert update strategy.

**Definition**

```csharp
public CascadeDeleteUpdateEnrichmentStrategy()
```

**Returns**

[CascadeDeleteUpdateEnrichmentStrategy](#csharp-cascadedeleteupdateenrichmentstrategy)`<T>`

###### UpdateOwned(entry, enrichment, parent) {#csharp-cascadedeleteupdateenrichmentstrategy-updateowned-entry-enrichment-parent}

Deletes every existing child for the parent key, then inserts the payload.

**Definition**

```csharp
public override Task UpdateOwned(StoredEnrichment entry, Enrichment enrichment, object parent)
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#csharp-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent object |

**Returns**

`Task`

###### UpdateLinked(entry, enrichment, parent) {#csharp-cascadedeleteupdateenrichmentstrategy-updatelinked-entry-enrichment-parent}

Deletes every junction row for the parent key, then inserts the payload ids.

**Definition**

```csharp
public override Task UpdateLinked(StoredEnrichment entry, Enrichment enrichment, object parent)
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#csharp-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) with [Association](#csharp-association) |
| `parent` | parent object |

**Returns**

`Task`

##### Rust

##### CascadeDeleteUpdateEnrichmentStrategy {#rust-cascadedeleteupdateenrichmentstrategy}

```rust
pub struct CascadeDeleteUpdateEnrichmentStrategy
```

**Methods**

| Name | Description |
| --- | --- |
| [update_owned(entry, enrichment, parent)](#rust-cascadedeleteupdateenrichmentstrategy-update_owned-entry-enrichment-parent) | Deletes every existing child for the parent key, then inserts the payload. |
| [update_linked(entry, enrichment, parent)](#rust-cascadedeleteupdateenrichmentstrategy-update_linked-entry-enrichment-parent) | Deletes every junction row for the parent key, then inserts the payload ids. |

###### update_owned(entry, enrichment, parent) {#rust-cascadedeleteupdateenrichmentstrategy-update_owned-entry-enrichment-parent}

Deletes every existing child for the parent key, then inserts the payload.

**Definition**

```rust
async fn update_owned(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#rust-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent row |

**Returns**

`Result<()>`

###### update_linked(entry, enrichment, parent) {#rust-cascadedeleteupdateenrichmentstrategy-update_linked-entry-enrichment-parent}

Deletes every junction row for the parent key, then inserts the payload ids.

**Definition**

```rust
async fn update_linked(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &mut Row) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#rust-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) with [association](#rust-association) |
| `parent` | parent row |

**Returns**

`Result<()>`

### Remarks

Pass it as the `update` argument of [DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy). Existing child ids are not reused. [DefaultUpdateEnrichmentStrategy](#defaultupdateenrichmentstrategy) keeps matching ids.

### Examples

#### Recreate children

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/EnrichedCRUDRepositoryTests.cs#L129-L160)

```csharp
var connection = await Setup();
var parentsRepo = new CRUDRepository<Parent>(connection, new CRUDRepositoryOptions<Parent>
{
    Write = new DefaultWriteEnrichmentStrategy<Parent>(update: new CascadeDeleteUpdateEnrichmentStrategy<Parent>()),
});
var childrenRepo = new CRUDRepository<Child>(connection);
var notesRepo = new CRUDRepository<Note>(connection);
var parent = await parentsRepo.Create(new Parent());
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/enriched.rs#L362-L441)

```rust
let connection = setup(&mut []).await;
let mut parents = crud(connection.clone(), "Parent").with_write(Arc::new(
    DefaultWriteEnrichmentStrategy::new(
        None,
        Some(Arc::new(CascadeDeleteUpdateEnrichmentStrategy)),
        false,
    ),
));
let mut children_repo = crud(connection.clone(), "Child");
let notes = Arc::new(crud(connection, "Note"));
let parent = parents
    .create(&mut record(&[("label", Value::from("p"))]))
    .await
    .unwrap();
assert!(num(&parent, "id") > 0);
let parent_id = parent.get("id").unwrap().clone();
let child = children_repo
    .create(&mut record(&[
        ("parent_id", parent_id.clone()),
        ("label", Value::from("a")),
    ]))
    .await
    .unwrap();
assert!(num(&child, "id") > 0);
let other = children_repo
    .create(&mut record(&[("label", Value::from("other"))]))
    .await
    .unwrap();
assert!(num(&other, "id") > 0);
let child_id = child.get("id").unwrap().clone();
let note = notes
    .create(&mut record(&[
        ("child_id", child_id.clone()),
        ("text", Value::from("n")),
    ]))
    .await
    .unwrap();
assert!(num(&note, "id") > 0);
children_repo.add_enriched(
    Enrichment::new("notes", "id", "child_id", None, true),
    notes.clone(),
    None,
);
let children = Arc::new(children_repo);
parents.add_enriched(
    Enrichment::new("children", "id", "parent_id", None, true),
    children.clone(),
    None,
);
let updated = parents
    .update(
        &record(&[
            ("id", parent_id.clone()),
            ("label", Value::from("p")),
            (
                "children",
                encode_rows(&[record(&[
                    ("id", child_id.clone()),
                    ("label", Value::from("fresh")),
                ])]),
            ),
        ]),
        None,
    )
    .await
    .unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/EnrichedCRUDRepository.test.ts#L158-L186)

```typescript
const connection = await setup();
const parentsRepo = new CRUDRepository<Parent>(connection, {
  type: Parent,
  write: new DefaultWriteEnrichmentStrategy(undefined, new CascadeDeleteUpdateEnrichmentStrategy()),
});
const childrenRepo = new CRUDRepository(Child, connection);
const notesRepo = new CRUDRepository(Note, connection);
const parent = await parentsRepo.create(new Parent());
```

## DeleteEnrichmentStrategy

How a delete removes owned children and links. [DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy) calls these methods before the parent row is deleted. [CascadeDeleteEnrichmentStrategy](#cascadedeleteenrichmentstrategy) is the default.

### Classes

##### TypeScript

##### DeleteEnrichmentStrategy {#typescript-deleteenrichmentstrategy}

```typescript
abstract class DeleteEnrichmentStrategy<T>
```

**Methods**

| Name | Description |
| --- | --- |
| [deleteOwned(entry, enrichment, parent, deleteNested)](#typescript-deleteenrichmentstrategy-deleteowned-entry-enrichment-parent-deletenested) | Removes the owned `many` list for one parent. |
| [deleteLinked(entry, enrichment, parent)](#typescript-deleteenrichmentstrategy-deletelinked-entry-enrichment-parent) | Removes the junction rows for one parent. |
| [load(writer, enrichment, parent)](#typescript-deleteenrichmentstrategy-load-writer-enrichment-parent) | Reads the owned rows whose `by` matches the parent `from`. |
| [loadLinks(writer, column, key)](#typescript-deleteenrichmentstrategy-loadlinks-writer-column-key) | Reads junction rows whose `column` equals `key`. |

###### deleteOwned(entry, enrichment, parent, deleteNested) {#typescript-deleteenrichmentstrategy-deleteowned-entry-enrichment-parent-deletenested}

Removes the owned `many` list for one parent.

**Definition**

```typescript
abstract deleteOwned(entry: StoredEnrichment, enrichment: Enrichment, parent: object, deleteNested: (rows: object[]) => Promise<void>): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#typescript-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent object |
| `deleteNested` | deletes nested rows of the children before the children themselves |

**Returns**

`Promise<void>`

###### deleteLinked(entry, enrichment, parent) {#typescript-deleteenrichmentstrategy-deletelinked-entry-enrichment-parent}

Removes the junction rows for one parent.

**Definition**

```typescript
abstract deleteLinked(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#typescript-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent object |

**Returns**

`Promise<void>`

###### load(writer, enrichment, parent) {#typescript-deleteenrichmentstrategy-load-writer-enrichment-parent}

Reads the owned rows whose `by` matches the parent `from`.

**Definition**

```typescript
static load(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<object[]>
```

**Parameters**

| Name | Description |
| --- | --- |
| `writer` | child [ICRUDRepository](#crud-repository) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent object |

**Returns**

`Promise<object[]>`

###### loadLinks(writer, column, key) {#typescript-deleteenrichmentstrategy-loadlinks-writer-column-key}

Reads junction rows whose `column` equals `key`.

**Definition**

```typescript
static loadLinks(writer: ICRUDRepository<object>, column: string, key: Scalar): Promise<object[]>
```

**Parameters**

| Name | Description |
| --- | --- |
| `writer` | link [ICRUDRepository](#crud-repository) |
| `column` | junction column |
| `key` | parent key |

**Returns**

`Promise<object[]>`

##### C#

##### DeleteEnrichmentStrategy {#csharp-deleteenrichmentstrategy}

```csharp
public abstract class DeleteEnrichmentStrategy<T>
```

**Methods**

| Name | Description |
| --- | --- |
| [DeleteOwned(entry, enrichment, parent, deleteNested)](#csharp-deleteenrichmentstrategy-deleteowned-entry-enrichment-parent-deletenested) | Removes the owned `many` list for one parent. |
| [DeleteLinked(entry, enrichment, parent)](#csharp-deleteenrichmentstrategy-deletelinked-entry-enrichment-parent) | Removes the junction rows for one parent. |

###### DeleteOwned(entry, enrichment, parent, deleteNested) {#csharp-deleteenrichmentstrategy-deleteowned-entry-enrichment-parent-deletenested}

Removes the owned `many` list for one parent.

**Definition**

```csharp
public abstract Task DeleteOwned(StoredEnrichment entry, Enrichment enrichment, object parent, Func<IReadOnlyList<object>, Task> deleteNested)
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#csharp-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent object |
| `deleteNested` | deletes nested rows of the children before the children themselves |

**Returns**

`Task`

###### DeleteLinked(entry, enrichment, parent) {#csharp-deleteenrichmentstrategy-deletelinked-entry-enrichment-parent}

Removes the junction rows for one parent.

**Definition**

```csharp
public abstract Task DeleteLinked(StoredEnrichment entry, Enrichment enrichment, object parent)
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#csharp-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent object |

**Returns**

`Task`

##### Rust

##### DeleteEnrichmentStrategy {#rust-deleteenrichmentstrategy}

```rust
pub trait DeleteEnrichmentStrategy: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [delete_owned(entry, enrichment, parent, delete_nested)](#rust-deleteenrichmentstrategy-delete_owned-entry-enrichment-parent-delete_nested) | Removes the owned `many` list for one parent. |
| [delete_linked(entry, enrichment, parent)](#rust-deleteenrichmentstrategy-delete_linked-entry-enrichment-parent) | Removes the junction rows for one parent. |

###### delete_owned(entry, enrichment, parent, delete_nested) {#rust-deleteenrichmentstrategy-delete_owned-entry-enrichment-parent-delete_nested}

Removes the owned `many` list for one parent.

**Definition**

```rust
async fn delete_owned(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &Row, delete_nested: &dyn DeleteNested) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#rust-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent row |
| `delete_nested` | deletes nested rows of the children before the children themselves |

**Returns**

`Result<()>`

###### delete_linked(entry, enrichment, parent) {#rust-deleteenrichmentstrategy-delete_linked-entry-enrichment-parent}

Removes the junction rows for one parent.

**Definition**

```rust
async fn delete_linked(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &Row) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#rust-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent row |

**Returns**

`Result<()>`

### Remarks

Pass a subclass as the `delete` argument of [DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy). A strategy that leaves `DeleteOwned` / `deleteOwned` / `delete_owned` empty keeps the children when the parent is deleted. The default [CascadeDeleteEnrichmentStrategy](#cascadedeleteenrichmentstrategy) loads existing children by foreign key and does not trust the payload lists.

### Examples

#### Skip child deletes

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/EnrichedCRUDRepositoryTests.cs#L218-L236)

```csharp
var connection = await Setup();
var childrenRepo = new CRUDRepository<Child>(connection);
var parentsRepo = new CRUDRepository<Parent>(connection, new CRUDRepositoryOptions<Parent>
{
    Write = new DefaultWriteEnrichmentStrategy<Parent>(new SkipDelete<Parent>()),
});
var parent = await parentsRepo.Create(new Parent());
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/delete_enrichment_strategy.rs#L77-L102)

```rust
let write = Arc::new(DefaultWriteEnrichmentStrategy::new(
    Some(Arc::new(SkipDelete)),
    None,
    false,
));
let children = Arc::new(CRUDRepository::memory(vec![]));
let mut parents = CRUDRepository::memory_with(vec![], Some(write));
let mut parent = Row::new();
let created = parents.create(&mut parent).await.unwrap();
let mut child = Row::new();
child.insert("parent_id".into(), created.get("id").unwrap().clone());
child.insert("label".into(), Value::from("a"));
children.create(&mut child).await.unwrap();
parents.add_enriched(
    Enrichment::new("children", "id", "parent_id", None, true),
    children.clone(),
    None,
);
parents.delete(&created, None).await.unwrap();
let left = children.read(None, None).await.unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/EnrichedCRUDRepository.test.ts#L249-L265)

```typescript
const connection = await setup();
const childrenRepo = new CRUDRepository(Child, connection);
const parentsRepo = new CRUDRepository(connection, {
  type: Parent,
  write: new DefaultWriteEnrichmentStrategy(new SkipDelete()),
});
const parent = await parentsRepo.create(new Parent());
```

## CascadeDeleteEnrichmentStrategy

Default delete. Deletes nested `many` rows first, then children, then the parent delete continues. Loads existing children by foreign key. Does not trust payload lists. Junction rows for a link are deleted and the related rows stay.

### Classes

##### TypeScript

##### CascadeDeleteEnrichmentStrategy {#typescript-cascadedeleteenrichmentstrategy}

```typescript
class CascadeDeleteEnrichmentStrategy<T> extends DeleteEnrichmentStrategy<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [new CascadeDeleteEnrichmentStrategy()](#typescript-cascadedeleteenrichmentstrategy-new-cascadedeleteenrichmentstrategy) | Builds the cascade delete strategy. |

**Methods**

| Name | Description |
| --- | --- |
| [deleteOwned(entry, enrichment, parent, deleteNested)](#typescript-cascadedeleteenrichmentstrategy-deleteowned-entry-enrichment-parent-deletenested) | Loads children by foreign key, deletes their nested rows, then deletes the children. |
| [deleteLinked(entry, enrichment, parent)](#typescript-cascadedeleteenrichmentstrategy-deletelinked-entry-enrichment-parent) | Deletes junction rows for the parent key. Related rows stay. |

###### new CascadeDeleteEnrichmentStrategy() {#typescript-cascadedeleteenrichmentstrategy-new-cascadedeleteenrichmentstrategy}

Builds the cascade delete strategy.

**Definition**

```typescript
constructor()
```

**Returns**

[CascadeDeleteEnrichmentStrategy](#typescript-cascadedeleteenrichmentstrategy)

###### deleteOwned(entry, enrichment, parent, deleteNested) {#typescript-cascadedeleteenrichmentstrategy-deleteowned-entry-enrichment-parent-deletenested}

Loads children by foreign key, deletes their nested rows, then deletes the children.

**Definition**

```typescript
deleteOwned(entry: StoredEnrichment, enrichment: Enrichment, parent: object, deleteNested: (rows: object[]) => Promise<void>): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#typescript-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent object |
| `deleteNested` | deletes nested rows of the children before the children themselves |

**Returns**

`Promise<void>`

###### deleteLinked(entry, enrichment, parent) {#typescript-cascadedeleteenrichmentstrategy-deletelinked-entry-enrichment-parent}

Deletes junction rows for the parent key. Related rows stay.

**Definition**

```typescript
deleteLinked(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#typescript-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) with [association](#typescript-association) |
| `parent` | parent object |

**Returns**

`Promise<void>`

##### C#

##### CascadeDeleteEnrichmentStrategy {#csharp-cascadedeleteenrichmentstrategy}

```csharp
public sealed class CascadeDeleteEnrichmentStrategy<T> : DeleteEnrichmentStrategy<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [CascadeDeleteEnrichmentStrategy()](#csharp-cascadedeleteenrichmentstrategy-cascadedeleteenrichmentstrategy) | Builds the cascade delete strategy. |

**Methods**

| Name | Description |
| --- | --- |
| [DeleteOwned(entry, enrichment, parent, deleteNested)](#csharp-cascadedeleteenrichmentstrategy-deleteowned-entry-enrichment-parent-deletenested) | Loads children by foreign key, deletes their nested rows, then deletes the children. |
| [DeleteLinked(entry, enrichment, parent)](#csharp-cascadedeleteenrichmentstrategy-deletelinked-entry-enrichment-parent) | Deletes junction rows for the parent key. Related rows stay. |

###### CascadeDeleteEnrichmentStrategy() {#csharp-cascadedeleteenrichmentstrategy-cascadedeleteenrichmentstrategy}

Builds the cascade delete strategy.

**Definition**

```csharp
public CascadeDeleteEnrichmentStrategy()
```

**Returns**

[CascadeDeleteEnrichmentStrategy](#csharp-cascadedeleteenrichmentstrategy)`<T>`

###### DeleteOwned(entry, enrichment, parent, deleteNested) {#csharp-cascadedeleteenrichmentstrategy-deleteowned-entry-enrichment-parent-deletenested}

Loads children by foreign key, deletes their nested rows, then deletes the children.

**Definition**

```csharp
public override Task DeleteOwned(StoredEnrichment entry, Enrichment enrichment, object parent, Func<IReadOnlyList<object>, Task> deleteNested)
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#csharp-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent object |
| `deleteNested` | deletes nested rows of the children before the children themselves |

**Returns**

`Task`

###### DeleteLinked(entry, enrichment, parent) {#csharp-cascadedeleteenrichmentstrategy-deletelinked-entry-enrichment-parent}

Deletes junction rows for the parent key. Related rows stay.

**Definition**

```csharp
public override Task DeleteLinked(StoredEnrichment entry, Enrichment enrichment, object parent)
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#csharp-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) with [Association](#csharp-association) |
| `parent` | parent object |

**Returns**

`Task`

##### Rust

##### CascadeDeleteEnrichmentStrategy {#rust-cascadedeleteenrichmentstrategy}

```rust
pub struct CascadeDeleteEnrichmentStrategy
```

**Methods**

| Name | Description |
| --- | --- |
| [delete_owned(entry, enrichment, parent, delete_nested)](#rust-cascadedeleteenrichmentstrategy-delete_owned-entry-enrichment-parent-delete_nested) | Loads children by foreign key, deletes their nested rows, then deletes the children. |
| [delete_linked(entry, enrichment, parent)](#rust-cascadedeleteenrichmentstrategy-delete_linked-entry-enrichment-parent) | Deletes junction rows for the parent key. Related rows stay. |

###### delete_owned(entry, enrichment, parent, delete_nested) {#rust-cascadedeleteenrichmentstrategy-delete_owned-entry-enrichment-parent-delete_nested}

Loads children by foreign key, deletes their nested rows, then deletes the children.

**Definition**

```rust
async fn delete_owned(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &Row, delete_nested: &dyn DeleteNested) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#rust-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) |
| `parent` | parent row |
| `delete_nested` | deletes nested rows of the children before the children themselves |

**Returns**

`Result<()>`

###### delete_linked(entry, enrichment, parent) {#rust-cascadedeleteenrichmentstrategy-delete_linked-entry-enrichment-parent}

Deletes junction rows for the parent key. Related rows stay.

**Definition**

```rust
async fn delete_linked(&self, entry: &StoredEnrichment, enrichment: &Enrichment, parent: &Row) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `entry` | [StoredEnrichment](#rust-storedenrichment) |
| `enrichment` | [Enrichment](#enrichment) with [association](#rust-association) |
| `parent` | parent row |

**Returns**

`Result<()>`

### Remarks

[DefaultWriteEnrichmentStrategy](#defaultwriteenrichmentstrategy) uses this when `delete` is omitted. The parent row is deleted by the repository after `Persist` / `persist` / `persist` returns.

### Examples

#### Cascade delete

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/EnrichedCRUDRepositoryTests.cs#L194-L216)

```csharp
var connection = await Setup();
var parentsRepo = new CRUDRepository<Parent>(connection);
var childrenRepo = new CRUDRepository<Child>(connection);
var notesRepo = new CRUDRepository<Note>(connection);
var parent = await parentsRepo.Create(new Parent());
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/tests/enriched.rs#L623-L683)

```rust
let connection = setup(&mut []).await;
let parents = crud(connection.clone(), "Parent");
let mut children_repo = crud(connection.clone(), "Child");
let notes = Arc::new(crud(connection, "Note"));
let parent = parents
    .create(&mut record(&[("label", Value::from("p"))]))
    .await
    .unwrap();
assert!(num(&parent, "id") > 0);
let parent_id = parent.get("id").unwrap().clone();
let child = children_repo
    .create(&mut record(&[
        ("parent_id", parent_id.clone()),
        ("label", Value::from("a")),
    ]))
    .await
    .unwrap();
assert!(num(&child, "id") > 0);
let child_id = child.get("id").unwrap().clone();
let note = notes
    .create(&mut record(&[
        ("child_id", child_id),
        ("text", Value::from("n")),
    ]))
    .await
    .unwrap();
assert!(num(&note, "id") > 0);
let other = notes
    .create(&mut record(&[
        ("text", Value::from("other")),
        ("label", Value::from("keep")),
    ]))
    .await
    .unwrap();
assert!(num(&other, "id") > 0);
children_repo.add_enriched(
    Enrichment::new("notes", "id", "child_id", None, true),
    notes.clone(),
    None,
);
let children = Arc::new(children_repo);
let mut parents = parents;
parents.add_enriched(
    Enrichment::new("children", "id", "parent_id", None, true),
    children.clone(),
    None,
);
let removed = parents
    .delete(&record(&[("id", parent_id)]), None)
    .await
    .unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/EnrichedCRUDRepository.test.ts#L227-L247)

```typescript
const connection = await setup();
const parentsRepo = new CRUDRepository(Parent, connection);
const childrenRepo = new CRUDRepository(Child, connection);
const notesRepo = new CRUDRepository(Note, connection);
const parent = await parentsRepo.create(new Parent());
```

## IBulkWriteStrategy

Writes a list of rows for a [BulkCRUDRepository](#bulk-repository). Create, update, and delete each receive the items and the repository. The default is [SingleCommandIBulkWriteStrategy](#singlecommandibulkwritestrategy).

### Classes

##### TypeScript

##### IBulkWriteStrategy {#typescript-ibulkwritestrategy}

```typescript
interface IBulkWriteStrategy<T extends object>
```

**Methods**

| Name | Description |
| --- | --- |
| [create(items, repo)](#typescript-ibulkwritestrategy-create-items-repo) | Inserts `items`. |
| [update(items, byWhere, repo)](#typescript-ibulkwritestrategy-update-items-bywhere-repo) | Updates `items`. |
| [delete(items, byWhere, repo)](#typescript-ibulkwritestrategy-delete-items-bywhere-repo) | Deletes `items`. |

###### create(items, repo) {#typescript-ibulkwritestrategy-create-items-repo}

Inserts `items`.

**Definition**

```typescript
create(items: T[], repo: BulkCRUDRepository<T>): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to insert |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#typescript-response)

###### update(items, byWhere, repo) {#typescript-ibulkwritestrategy-update-items-bywhere-repo}

Updates `items`.

**Definition**

```typescript
update(items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to update |
| `byWhere` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#typescript-response)

###### delete(items, byWhere, repo) {#typescript-ibulkwritestrategy-delete-items-bywhere-repo}

Deletes `items`.

**Definition**

```typescript
delete(items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to delete |
| `byWhere` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#typescript-response)

##### C#

##### IBulkWriteStrategy {#csharp-ibulkwritestrategy}

```csharp
public interface IBulkWriteStrategy<T> where T : new()
```

**Methods**

| Name | Description |
| --- | --- |
| [Create(items, repo)](#csharp-ibulkwritestrategy-create-items-repo) | Inserts `items`. |
| [Update(items, byWhere, repo)](#csharp-ibulkwritestrategy-update-items-bywhere-repo) | Updates `items`. |
| [Delete(items, byWhere, repo)](#csharp-ibulkwritestrategy-delete-items-bywhere-repo) | Deletes `items`. |

###### Create(items, repo) {#csharp-ibulkwritestrategy-create-items-repo}

Inserts `items`.

**Definition**

```csharp
Task<Response> Create(IReadOnlyList<T> items, BulkCRUDRepository<T> repo)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to insert |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#csharp-response)

###### Update(items, byWhere, repo) {#csharp-ibulkwritestrategy-update-items-bywhere-repo}

Updates `items`.

**Definition**

```csharp
Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to update |
| `byWhere` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#csharp-response)

###### Delete(items, byWhere, repo) {#csharp-ibulkwritestrategy-delete-items-bywhere-repo}

Deletes `items`.

**Definition**

```csharp
Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to delete |
| `byWhere` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#csharp-response)

##### Rust

##### BulkWriteStrategy {#rust-bulkwritestrategy}

```rust
pub trait BulkWriteStrategy: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [create(items, repo)](#rust-bulkwritestrategy-create-items-repo) | Inserts `items`. |
| [update(items, by_where, repo)](#rust-bulkwritestrategy-update-items-by_where-repo) | Updates `items`. |
| [delete(items, by_where, repo)](#rust-bulkwritestrategy-delete-items-by_where-repo) | Deletes `items`. |

###### create(items, repo) {#rust-bulkwritestrategy-create-items-repo}

Inserts `items`.

**Definition**

```rust
async fn create(&self, items: &mut [Row], repo: &BulkCRUDRepository) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to insert |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

`Result<Response>`

###### update(items, by_where, repo) {#rust-bulkwritestrategy-update-items-by_where-repo}

Updates `items`.

**Definition**

```rust
async fn update(&self, items: &[Row], by_where: Option<&dyn IWhere>, repo: &BulkCRUDRepository) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to update |
| `by_where` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

`Result<Response>`

###### delete(items, by_where, repo) {#rust-bulkwritestrategy-delete-items-by_where-repo}

Deletes `items`.

**Definition**

```rust
async fn delete(&self, items: &[Row], by_where: Option<&dyn IWhere>, repo: &BulkCRUDRepository) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to delete |
| `by_where` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

`Result<Response>`

### Remarks

Pass the strategy to the bulk repository constructor. [SingleCommandIBulkWriteStrategy](#singlecommandibulkwritestrategy) runs one statement per row. [BatchCommandIBulkWriteStrategy](#batchcommandibulkwritestrategy) joins those statements. [StoredProcedureBulkWriteStrategy](#storedprocedurebulkwritestrategy) sends one JSON payload. An empty list does not execute. Nested persist still follows the repository write strategy.

### Examples

#### One statement per row

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/BulkWriteStrategyTests.cs#L25-L40)

```csharp
var (repo, command) = Repo(new SingleCommandIBulkWriteStrategy<Parent>());
command.Response = new Response(new Value.Int(9), 1);
await repo.Create([new Parent { label = "a" }, new Parent { label = "b" }]);
await repo.Update([new Parent { id = 1, label = "a" }, new Parent { id = 2, label = "b" }]);
await repo.Delete([new Parent { id = 1, label = "a" }, new Parent { id = 2, label = "b" }]);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/bulk_write_strategy.rs#L28-L69)

```rust
let (repo, command) = parents(Arc::new(SingleCommandIBulkWriteStrategy));
*command.response.lock().unwrap() = crate::Response::new(Value::Int(9), 1, None);
let mut created = [
    row(&[("label", Value::from("a"))]),
    row(&[("label", Value::from("b"))]),
];
repo.create_bulk(&mut created).await.unwrap();
repo.update_bulk(
    &[
        row(&[("id", Value::Int(1)), ("label", Value::from("a"))]),
        row(&[("id", Value::Int(2)), ("label", Value::from("b"))]),
    ],
    None,
)
.await
.unwrap();
repo.delete_bulk(
    &[
        row(&[("id", Value::Int(1)), ("label", Value::from("a"))]),
        row(&[("id", Value::Int(2)), ("label", Value::from("b"))]),
    ],
    None,
)
.await
.unwrap();
let logs = command.execute_logs.lock().unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/BulkWriteStrategy.test.ts#L35-L57)

```typescript
const { repo, command } = open(Parent, new SingleCommandIBulkWriteStrategy<Parent>());
command.response = { result: 9, rowsAffected: 1 };
await repo.createBulk([
  Object.assign(new Parent(), { label: "a" }),
  Object.assign(new Parent(), { label: "b" }),
]);
await repo.updateBulk([
  Object.assign(new Parent(), { id: 1, label: "a" }),
  Object.assign(new Parent(), { id: 2, label: "b" }),
]);
await repo.deleteBulk([
  Object.assign(new Parent(), { id: 1, label: "a" }),
  Object.assign(new Parent(), { id: 2, label: "b" }),
]);
expect(command.executeCalls).toHaveLength(6);
```

## SingleCommandIBulkWriteStrategy

Bulk write that runs one insert, update, or delete per row. This is the bulk repository default.

### Classes

##### TypeScript

##### SingleCommandIBulkWriteStrategy {#typescript-singlecommandibulkwritestrategy}

```typescript
class SingleCommandIBulkWriteStrategy<T extends object> implements IBulkWriteStrategy<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [new SingleCommandIBulkWriteStrategy()](#typescript-singlecommandibulkwritestrategy-new-singlecommandibulkwritestrategy) | Builds the per-row bulk strategy. |

**Methods**

| Name | Description |
| --- | --- |
| [create(items, repo)](#typescript-singlecommandibulkwritestrategy-create-items-repo) | Inserts each item with its own statement. The result is the last insert. |
| [update(items, byWhere, repo)](#typescript-singlecommandibulkwritestrategy-update-items-bywhere-repo) | Updates each item with its own statement. Rows affected are summed. |
| [delete(items, byWhere, repo)](#typescript-singlecommandibulkwritestrategy-delete-items-bywhere-repo) | Deletes each item with its own statement. Rows affected are summed. |

###### new SingleCommandIBulkWriteStrategy() {#typescript-singlecommandibulkwritestrategy-new-singlecommandibulkwritestrategy}

Builds the per-row bulk strategy.

**Definition**

```typescript
constructor()
```

**Returns**

[SingleCommandIBulkWriteStrategy](#typescript-singlecommandibulkwritestrategy)

###### create(items, repo) {#typescript-singlecommandibulkwritestrategy-create-items-repo}

Inserts each item with its own statement. The result is the last insert.

**Definition**

```typescript
create(items: T[], repo: BulkCRUDRepository<T>): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to insert |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#typescript-response)

###### update(items, byWhere, repo) {#typescript-singlecommandibulkwritestrategy-update-items-bywhere-repo}

Updates each item with its own statement. Rows affected are summed.

**Definition**

```typescript
update(items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to update |
| `byWhere` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#typescript-response)

###### delete(items, byWhere, repo) {#typescript-singlecommandibulkwritestrategy-delete-items-bywhere-repo}

Deletes each item with its own statement. Rows affected are summed.

**Definition**

```typescript
delete(items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to delete |
| `byWhere` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#typescript-response)

##### C#

##### SingleCommandIBulkWriteStrategy {#csharp-singlecommandibulkwritestrategy}

```csharp
public sealed class SingleCommandIBulkWriteStrategy<T> : IBulkWriteStrategy<T> where T : new()
```

**Constructor**

| Name | Description |
| --- | --- |
| [SingleCommandIBulkWriteStrategy()](#csharp-singlecommandibulkwritestrategy-singlecommandibulkwritestrategy) | Builds the per-row bulk strategy. |

**Methods**

| Name | Description |
| --- | --- |
| [Create(items, repo)](#csharp-singlecommandibulkwritestrategy-create-items-repo) | Inserts each item with its own statement. The result is the last insert. |
| [Update(items, byWhere, repo)](#csharp-singlecommandibulkwritestrategy-update-items-bywhere-repo) | Updates each item with its own statement. Rows affected are summed. |
| [Delete(items, byWhere, repo)](#csharp-singlecommandibulkwritestrategy-delete-items-bywhere-repo) | Deletes each item with its own statement. Rows affected are summed. |

###### SingleCommandIBulkWriteStrategy() {#csharp-singlecommandibulkwritestrategy-singlecommandibulkwritestrategy}

Builds the per-row bulk strategy.

**Definition**

```csharp
public SingleCommandIBulkWriteStrategy()
```

**Returns**

[SingleCommandIBulkWriteStrategy](#csharp-singlecommandibulkwritestrategy)`<T>`

###### Create(items, repo) {#csharp-singlecommandibulkwritestrategy-create-items-repo}

Inserts each item with its own statement. The result is the last insert.

**Definition**

```csharp
public Task<Response> Create(IReadOnlyList<T> items, BulkCRUDRepository<T> repo)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to insert |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#csharp-response)

###### Update(items, byWhere, repo) {#csharp-singlecommandibulkwritestrategy-update-items-bywhere-repo}

Updates each item with its own statement. Rows affected are summed.

**Definition**

```csharp
public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to update |
| `byWhere` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#csharp-response)

###### Delete(items, byWhere, repo) {#csharp-singlecommandibulkwritestrategy-delete-items-bywhere-repo}

Deletes each item with its own statement. Rows affected are summed.

**Definition**

```csharp
public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to delete |
| `byWhere` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#csharp-response)

##### Rust

##### SingleCommandIBulkWriteStrategy {#rust-singlecommandibulkwritestrategy}

```rust
pub struct SingleCommandIBulkWriteStrategy
```

**Methods**

| Name | Description |
| --- | --- |
| [create(items, repo)](#rust-singlecommandibulkwritestrategy-create-items-repo) | Inserts each item with its own statement. The result is the last insert. |
| [update(items, by_where, repo)](#rust-singlecommandibulkwritestrategy-update-items-by_where-repo) | Updates each item with its own statement. Rows affected are summed. |
| [delete(items, by_where, repo)](#rust-singlecommandibulkwritestrategy-delete-items-by_where-repo) | Deletes each item with its own statement. Rows affected are summed. |

###### create(items, repo) {#rust-singlecommandibulkwritestrategy-create-items-repo}

Inserts each item with its own statement. The result is the last insert.

**Definition**

```rust
async fn create(&self, items: &mut [Row], repo: &BulkCRUDRepository) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to insert |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

`Result<Response>`

###### update(items, by_where, repo) {#rust-singlecommandibulkwritestrategy-update-items-by_where-repo}

Updates each item with its own statement. Rows affected are summed.

**Definition**

```rust
async fn update(&self, items: &[Row], by_where: Option<&dyn IWhere>, repo: &BulkCRUDRepository) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to update |
| `by_where` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

`Result<Response>`

###### delete(items, by_where, repo) {#rust-singlecommandibulkwritestrategy-delete-items-by_where-repo}

Deletes each item with its own statement. Rows affected are summed.

**Definition**

```rust
async fn delete(&self, items: &[Row], by_where: Option<&dyn IWhere>, repo: &BulkCRUDRepository) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to delete |
| `by_where` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

`Result<Response>`

### Remarks

Each row still runs the repository's nested persist. An empty list returns no result and does not execute.

### Examples

#### One statement per row

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/BulkWriteStrategyTests.cs#L25-L40)

```csharp
var (repo, command) = Repo(new SingleCommandIBulkWriteStrategy<Parent>());
command.Response = new Response(new Value.Int(9), 1);
await repo.Create([new Parent { label = "a" }, new Parent { label = "b" }]);
await repo.Update([new Parent { id = 1, label = "a" }, new Parent { id = 2, label = "b" }]);
await repo.Delete([new Parent { id = 1, label = "a" }, new Parent { id = 2, label = "b" }]);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/bulk_write_strategy.rs#L28-L69)

```rust
let (repo, command) = parents(Arc::new(SingleCommandIBulkWriteStrategy));
*command.response.lock().unwrap() = crate::Response::new(Value::Int(9), 1, None);
let mut created = [
    row(&[("label", Value::from("a"))]),
    row(&[("label", Value::from("b"))]),
];
repo.create_bulk(&mut created).await.unwrap();
repo.update_bulk(
    &[
        row(&[("id", Value::Int(1)), ("label", Value::from("a"))]),
        row(&[("id", Value::Int(2)), ("label", Value::from("b"))]),
    ],
    None,
)
.await
.unwrap();
repo.delete_bulk(
    &[
        row(&[("id", Value::Int(1)), ("label", Value::from("a"))]),
        row(&[("id", Value::Int(2)), ("label", Value::from("b"))]),
    ],
    None,
)
.await
.unwrap();
let logs = command.execute_logs.lock().unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/BulkWriteStrategy.test.ts#L35-L57)

```typescript
const { repo, command } = open(Parent, new SingleCommandIBulkWriteStrategy<Parent>());
command.response = { result: 9, rowsAffected: 1 };
await repo.createBulk([
  Object.assign(new Parent(), { label: "a" }),
  Object.assign(new Parent(), { label: "b" }),
]);
await repo.updateBulk([
  Object.assign(new Parent(), { id: 1, label: "a" }),
  Object.assign(new Parent(), { id: 2, label: "b" }),
]);
await repo.deleteBulk([
  Object.assign(new Parent(), { id: 1, label: "a" }),
  Object.assign(new Parent(), { id: 2, label: "b" }),
]);
expect(command.executeCalls).toHaveLength(6);
```

## BatchCommandIBulkWriteStrategy

Bulk write that joins one statement per row into a single batch, separated by `;`. Parameters are renamed so names do not collide.

### Classes

##### TypeScript

##### BatchCommandIBulkWriteStrategy {#typescript-batchcommandibulkwritestrategy}

```typescript
class BatchCommandIBulkWriteStrategy<T extends object> implements IBulkWriteStrategy<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [new BatchCommandIBulkWriteStrategy()](#typescript-batchcommandibulkwritestrategy-new-batchcommandibulkwritestrategy) | Builds the batched statement strategy. |

**Methods**

| Name | Description |
| --- | --- |
| [create(items, repo)](#typescript-batchcommandibulkwritestrategy-create-items-repo) | Sends every insert as one batched command. |
| [update(items, byWhere, repo)](#typescript-batchcommandibulkwritestrategy-update-items-bywhere-repo) | Sends every update as one batched command. |
| [delete(items, byWhere, repo)](#typescript-batchcommandibulkwritestrategy-delete-items-bywhere-repo) | Sends every delete as one batched command. |

###### new BatchCommandIBulkWriteStrategy() {#typescript-batchcommandibulkwritestrategy-new-batchcommandibulkwritestrategy}

Builds the batched statement strategy.

**Definition**

```typescript
constructor()
```

**Returns**

[BatchCommandIBulkWriteStrategy](#typescript-batchcommandibulkwritestrategy)

###### create(items, repo) {#typescript-batchcommandibulkwritestrategy-create-items-repo}

Sends every insert as one batched command.

**Definition**

```typescript
create(items: T[], repo: BulkCRUDRepository<T>): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to insert |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#typescript-response)

###### update(items, byWhere, repo) {#typescript-batchcommandibulkwritestrategy-update-items-bywhere-repo}

Sends every update as one batched command.

**Definition**

```typescript
update(items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to update |
| `byWhere` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#typescript-response)

###### delete(items, byWhere, repo) {#typescript-batchcommandibulkwritestrategy-delete-items-bywhere-repo}

Sends every delete as one batched command.

**Definition**

```typescript
delete(items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to delete |
| `byWhere` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#typescript-response)

##### C#

##### BatchCommandIBulkWriteStrategy {#csharp-batchcommandibulkwritestrategy}

```csharp
public sealed class BatchCommandIBulkWriteStrategy<T> : IBulkWriteStrategy<T> where T : new()
```

**Constructor**

| Name | Description |
| --- | --- |
| [BatchCommandIBulkWriteStrategy()](#csharp-batchcommandibulkwritestrategy-batchcommandibulkwritestrategy) | Builds the batched statement strategy. |

**Methods**

| Name | Description |
| --- | --- |
| [Create(items, repo)](#csharp-batchcommandibulkwritestrategy-create-items-repo) | Sends every insert as one batched command. |
| [Update(items, byWhere, repo)](#csharp-batchcommandibulkwritestrategy-update-items-bywhere-repo) | Sends every update as one batched command. |
| [Delete(items, byWhere, repo)](#csharp-batchcommandibulkwritestrategy-delete-items-bywhere-repo) | Sends every delete as one batched command. |

###### BatchCommandIBulkWriteStrategy() {#csharp-batchcommandibulkwritestrategy-batchcommandibulkwritestrategy}

Builds the batched statement strategy.

**Definition**

```csharp
public BatchCommandIBulkWriteStrategy()
```

**Returns**

[BatchCommandIBulkWriteStrategy](#csharp-batchcommandibulkwritestrategy)`<T>`

###### Create(items, repo) {#csharp-batchcommandibulkwritestrategy-create-items-repo}

Sends every insert as one batched command.

**Definition**

```csharp
public Task<Response> Create(IReadOnlyList<T> items, BulkCRUDRepository<T> repo)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to insert |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#csharp-response)

###### Update(items, byWhere, repo) {#csharp-batchcommandibulkwritestrategy-update-items-bywhere-repo}

Sends every update as one batched command.

**Definition**

```csharp
public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to update |
| `byWhere` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#csharp-response)

###### Delete(items, byWhere, repo) {#csharp-batchcommandibulkwritestrategy-delete-items-bywhere-repo}

Sends every delete as one batched command.

**Definition**

```csharp
public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to delete |
| `byWhere` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#csharp-response)

##### Rust

##### BatchCommandIBulkWriteStrategy {#rust-batchcommandibulkwritestrategy}

```rust
pub struct BatchCommandIBulkWriteStrategy
```

**Methods**

| Name | Description |
| --- | --- |
| [create(items, repo)](#rust-batchcommandibulkwritestrategy-create-items-repo) | Sends every insert as one batched command. |
| [update(items, by_where, repo)](#rust-batchcommandibulkwritestrategy-update-items-by_where-repo) | Sends every update as one batched command. |
| [delete(items, by_where, repo)](#rust-batchcommandibulkwritestrategy-delete-items-by_where-repo) | Sends every delete as one batched command. |

###### create(items, repo) {#rust-batchcommandibulkwritestrategy-create-items-repo}

Sends every insert as one batched command.

**Definition**

```rust
async fn create(&self, items: &mut [Row], repo: &BulkCRUDRepository) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to insert |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

`Result<Response>`

###### update(items, by_where, repo) {#rust-batchcommandibulkwritestrategy-update-items-by_where-repo}

Sends every update as one batched command.

**Definition**

```rust
async fn update(&self, items: &[Row], by_where: Option<&dyn IWhere>, repo: &BulkCRUDRepository) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to update |
| `by_where` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

`Result<Response>`

###### delete(items, by_where, repo) {#rust-batchcommandibulkwritestrategy-delete-items-by_where-repo}

Sends every delete as one batched command.

**Definition**

```rust
async fn delete(&self, items: &[Row], by_where: Option<&dyn IWhere>, repo: &BulkCRUDRepository) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to delete |
| `by_where` | optional [IWhere](#where) applied with each item |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

`Result<Response>`

### Remarks

An empty list does not execute. The provider must accept several statements in one command. Nested persist still runs through the repository write strategy, separate from this batch.

### Examples

#### Join statements

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/BulkWriteStrategyTests.cs#L42-L69)

```csharp
var (repo, command) = Repo(new BatchCommandIBulkWriteStrategy<Parent>());
await repo.Create([new Parent { label = "a" }, new Parent { label = "b" }]);
Assert.Single(command.ExecuteCalls);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/bulk_write_strategy.rs#L71-L131)

```rust
let (repo, command) = parents(Arc::new(BatchCommandIBulkWriteStrategy));
let mut created = [
    row(&[("label", Value::from("a"))]),
    row(&[("label", Value::from("b"))]),
];
repo.create_bulk(&mut created).await.unwrap();
{
    let logs = command.execute_logs.lock().unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/BulkWriteStrategy.test.ts#L59-L94)

```typescript
const { repo, command } = open(Parent, new BatchCommandIBulkWriteStrategy<Parent>());
await repo.createBulk([
  Object.assign(new Parent(), { label: "a" }),
  Object.assign(new Parent(), { label: "b" }),
]);
expect(command.executeCalls).toHaveLength(1);
```

## StoredProcedureBulkWriteStrategy

Bulk write that sends the rows as one JSON `payload` argument to a stored procedure. Create, update, and delete each name their own procedure.

### Classes

##### TypeScript

##### StoredProcedureBulkWriteStrategy {#typescript-storedprocedurebulkwritestrategy}

```typescript
class StoredProcedureBulkWriteStrategy<T extends object> implements IBulkWriteStrategy<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [new StoredProcedureBulkWriteStrategy(createName, updateName, deleteName)](#typescript-storedprocedurebulkwritestrategy-new-storedprocedurebulkwritestrategy-createname-updatename-deletename) | Builds the procedure strategy. |

**Methods**

| Name | Description |
| --- | --- |
| [create(items, repo)](#typescript-storedprocedurebulkwritestrategy-create-items-repo) | Calls the create procedure with a JSON `payload` of the rows. |
| [update(items, byWhere, repo)](#typescript-storedprocedurebulkwritestrategy-update-items-bywhere-repo) | Calls the update procedure with a JSON `payload` of the rows. |
| [delete(items, byWhere, repo)](#typescript-storedprocedurebulkwritestrategy-delete-items-bywhere-repo) | Calls the delete procedure with a JSON `payload` of the rows. |

###### new StoredProcedureBulkWriteStrategy(createName, updateName, deleteName) {#typescript-storedprocedurebulkwritestrategy-new-storedprocedurebulkwritestrategy-createname-updatename-deletename}

Builds the procedure strategy.

**Definition**

```typescript
constructor(createName: string, updateName: string, deleteName: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `createName` | procedure for insert |
| `updateName` | procedure for update |
| `deleteName` | procedure for delete |

**Returns**

[StoredProcedureBulkWriteStrategy](#typescript-storedprocedurebulkwritestrategy)

###### create(items, repo) {#typescript-storedprocedurebulkwritestrategy-create-items-repo}

Calls the create procedure with a JSON `payload` of the rows.

**Definition**

```typescript
create(items: T[], repo: BulkCRUDRepository<T>): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to insert |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#typescript-response)

###### update(items, byWhere, repo) {#typescript-storedprocedurebulkwritestrategy-update-items-bywhere-repo}

Calls the update procedure with a JSON `payload` of the rows.

**Definition**

```typescript
update(items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to update |
| `byWhere` | unused |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#typescript-response)

###### delete(items, byWhere, repo) {#typescript-storedprocedurebulkwritestrategy-delete-items-bywhere-repo}

Calls the delete procedure with a JSON `payload` of the rows.

**Definition**

```typescript
delete(items: T[], byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to delete |
| `byWhere` | unused |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#typescript-response)

##### C#

##### StoredProcedureBulkWriteStrategy {#csharp-storedprocedurebulkwritestrategy}

```csharp
public sealed class StoredProcedureBulkWriteStrategy<T> : IBulkWriteStrategy<T> where T : new()
```

**Constructor**

| Name | Description |
| --- | --- |
| [StoredProcedureBulkWriteStrategy(create, update, delete)](#csharp-storedprocedurebulkwritestrategy-storedprocedurebulkwritestrategy-create-update-delete) | Builds the procedure strategy. |

**Methods**

| Name | Description |
| --- | --- |
| [Create(items, repo)](#csharp-storedprocedurebulkwritestrategy-create-items-repo) | Calls the create procedure with a JSON `payload` of the rows. |
| [Update(items, byWhere, repo)](#csharp-storedprocedurebulkwritestrategy-update-items-bywhere-repo) | Calls the update procedure with a JSON `payload` of the rows. |
| [Delete(items, byWhere, repo)](#csharp-storedprocedurebulkwritestrategy-delete-items-bywhere-repo) | Calls the delete procedure with a JSON `payload` of the rows. |

###### StoredProcedureBulkWriteStrategy(create, update, delete) {#csharp-storedprocedurebulkwritestrategy-storedprocedurebulkwritestrategy-create-update-delete}

Builds the procedure strategy.

**Definition**

```csharp
public StoredProcedureBulkWriteStrategy(string create, string update, string delete)
```

**Parameters**

| Name | Description |
| --- | --- |
| `create` | procedure for insert |
| `update` | procedure for update |
| `delete` | procedure for delete |

**Returns**

[StoredProcedureBulkWriteStrategy](#csharp-storedprocedurebulkwritestrategy)`<T>`

###### Create(items, repo) {#csharp-storedprocedurebulkwritestrategy-create-items-repo}

Calls the create procedure with a JSON `payload` of the rows.

**Definition**

```csharp
public Task<Response> Create(IReadOnlyList<T> items, BulkCRUDRepository<T> repo)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to insert |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#csharp-response)

###### Update(items, byWhere, repo) {#csharp-storedprocedurebulkwritestrategy-update-items-bywhere-repo}

Calls the update procedure with a JSON `payload` of the rows.

**Definition**

```csharp
public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to update |
| `byWhere` | unused |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#csharp-response)

###### Delete(items, byWhere, repo) {#csharp-storedprocedurebulkwritestrategy-delete-items-bywhere-repo}

Calls the delete procedure with a JSON `payload` of the rows.

**Definition**

```csharp
public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to delete |
| `byWhere` | unused |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

[Response](#csharp-response)

##### Rust

##### StoredProcedureBulkWriteStrategy {#rust-storedprocedurebulkwritestrategy}

```rust
pub struct StoredProcedureBulkWriteStrategy
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(create, update, delete)](#rust-storedprocedurebulkwritestrategy-new-create-update-delete) | Builds the procedure strategy. |

**Methods**

| Name | Description |
| --- | --- |
| [create(items, repo)](#rust-storedprocedurebulkwritestrategy-create-items-repo) | Calls the create procedure with a JSON `payload` of the rows. |
| [update(items, by_where, repo)](#rust-storedprocedurebulkwritestrategy-update-items-by_where-repo) | Calls the update procedure with a JSON `payload` of the rows. |
| [delete(items, by_where, repo)](#rust-storedprocedurebulkwritestrategy-delete-items-by_where-repo) | Calls the delete procedure with a JSON `payload` of the rows. |

###### new(create, update, delete) {#rust-storedprocedurebulkwritestrategy-new-create-update-delete}

Builds the procedure strategy.

**Definition**

```rust
pub fn new(create: impl Into<String>, update: impl Into<String>, delete: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `create` | procedure for insert |
| `update` | procedure for update |
| `delete` | procedure for delete |

**Returns**

[StoredProcedureBulkWriteStrategy](#rust-storedprocedurebulkwritestrategy)

###### create(items, repo) {#rust-storedprocedurebulkwritestrategy-create-items-repo}

Calls the create procedure with a JSON `payload` of the rows.

**Definition**

```rust
async fn create(&self, items: &mut [Row], repo: &BulkCRUDRepository) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to insert |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

`Result<Response>`

###### update(items, by_where, repo) {#rust-storedprocedurebulkwritestrategy-update-items-by_where-repo}

Calls the update procedure with a JSON `payload` of the rows.

**Definition**

```rust
async fn update(&self, items: &[Row], by_where: Option<&dyn IWhere>, repo: &BulkCRUDRepository) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to update |
| `by_where` | unused |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

`Result<Response>`

###### delete(items, by_where, repo) {#rust-storedprocedurebulkwritestrategy-delete-items-by_where-repo}

Calls the delete procedure with a JSON `payload` of the rows.

**Definition**

```rust
async fn delete(&self, items: &[Row], by_where: Option<&dyn IWhere>, repo: &BulkCRUDRepository) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | rows to delete |
| `by_where` | unused |
| `repo` | [BulkCRUDRepository](#bulk-repository) |

**Returns**

`Result<Response>`

### Remarks

The payload is a JSON array of objects, one object per row, with column names in order. An empty list does not execute. The procedure is an [ExecuteStoredProcedureStatement](#execute-stored-procedure-statement) with a single `payload` parameter.

### Examples

#### One JSON call

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/BulkWriteStrategyTests.cs#L71-L95)

```csharp
var (repo, command) = Repo(new StoredProcedureBulkWriteStrategy<Parent>(
    "bulk_create_parent", "bulk_update_parent", "bulk_delete_parent"));
await repo.Create([new Parent { label = "a" }, new Parent { label = "b" }]);
Assert.Single(command.ExecuteCalls);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/bulk_write_strategy.rs#L133-L181)

```rust
let (repo, command) = parents(Arc::new(StoredProcedureBulkWriteStrategy::new(
    "bulk_create_parent",
    "bulk_update_parent",
    "bulk_delete_parent",
)));
let mut created = [
    row(&[("label", Value::from("a"))]),
    row(&[("label", Value::from("b"))]),
];
repo.create_bulk(&mut created).await.unwrap();
repo.update_bulk(
    &[
        row(&[("id", Value::Int(1)), ("label", Value::from("a"))]),
        row(&[("id", Value::Int(2)), ("label", Value::from("b"))]),
    ],
    None,
)
.await
.unwrap();
repo.delete_bulk(
    &[
        row(&[("id", Value::Int(1)), ("label", Value::from(""))]),
        row(&[("id", Value::Int(2)), ("label", Value::from(""))]),
    ],
    None,
)
.await
.unwrap();
let logs = command.execute_logs.lock().unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/BulkWriteStrategy.test.ts#L96-L129)

```typescript
const { repo, command } = open(
  Parent,
  new StoredProcedureBulkWriteStrategy<Parent>("bulk_create_parent", "bulk_update_parent", "bulk_delete_parent"),
);
await repo.createBulk([
  Object.assign(new Parent(), { label: "a" }),
  Object.assign(new Parent(), { label: "b" }),
]);
expect(command.executeCalls).toHaveLength(1);
```

## Command Line (CLI)

`@datasources/cli` installs provider packages and writes a local `.env`. `Datasource.Default` / `Datasource.default` / `default()` reads `INSTALLED_PROVIDERS`, `PROVIDER`, and `DATASOURCE_URL`. Process env overrides the same keys in `.env`. Missing `PROVIDER` throws.

```sh
npx @datasources/cli setup
npx @datasources/cli add sqlite
npx @datasources/cli remove sqlite
```

Pass `--ecosystem ts|rust|csharp` when the directory has more than one project type. From this repo: `npm run setup` or `npm run cli -- add sqlite`.

### Setup

Interactive `.env` and provider install. `--yes` skips prompts and uses defaults / existing `.env`.

```sh
npx @datasources/cli setup
npx @datasources/cli setup --yes
```

| Flag | Meaning |
| --- | --- |
| `--cwd <dir>` | Consumer project directory (default: `.`) |
| `--ecosystem ts\|rust\|csharp` | Required when more than one project type exists |
| `--dry-run` | Print package-manager commands without running them |
| `--yes` | Setup without prompts |

### Add

Add provider packages. Updates `.env` `INSTALLED_PROVIDERS` when a `.env` already exists.

```sh
npx @datasources/cli add sqlite postgres
```

| Flag | Meaning |
| --- | --- |
| `--cwd <dir>` | Consumer project directory (default: `.`) |
| `--ecosystem ts\|rust\|csharp` | Required when more than one project type exists |
| `--dry-run` | Print package-manager commands without running them |

### Remove

Remove provider packages and drop them from `.env`.

```sh
npx @datasources/cli remove postgres
```

| Flag | Meaning |
| --- | --- |
| `--cwd <dir>` | Consumer project directory (default: `.`) |
| `--ecosystem ts\|rust\|csharp` | Required when more than one project type exists |
| `--dry-run` | Print package-manager commands without running them |

### .env

`setup` writes these keys. `add` / `remove` update the provider list. Other keys already in the file are kept. Process env overrides the same three keys. `Create` / `create` / `create` with omitted options registers each name in `INSTALLED_PROVIDERS` and the provider inferred from `DATASOURCE_URL`. The datasource itself is `PROVIDER`.

| Key | Default | Meaning |
| --- | --- | --- |
| `INSTALLED_PROVIDERS` | scheme of `DATASOURCE_URL` | Comma-separated providers `Create` registers. |
| `PROVIDER` | | Datasource `Create` returns. |
| `DATASOURCE_URL` | `sqlite:///:memory:` | Connection URL. Schema is the `schema` query. |

```sh
INSTALLED_PROVIDERS=sqlite,postgres
PROVIDER=postgres
DATASOURCE_URL=postgres://localhost/db?schema=public
```

SQLAlchemy-style SQLite URLs: `sqlite:///:memory:`, `sqlite:///app.db`, `sqlite:////tmp/app.db`.

## Writing Providers

A provider package implements `IDriver` / `IDriver` / `Driver` and exports only that type. `DatasourceFactory.Register` / `DatasourceFactory.register` / `register` stores it by name. The driver builds the connection, command, and transaction, and supplies row cell converters when native values are not already canonical cells.

Every provider names the type `*Driver` (`JsonDriver`, `SqliteDriver`).

### Classes

##### TypeScript

##### IDriver {#typescript-idriver}

```typescript
interface IDriver
```

**Methods**

| Name | Description |
| --- | --- |
| [connection(options?)](#typescript-idriver-connection-options) | Builds an [IConnection](#connection) from the options bag. |
| [command(connection)](#typescript-idriver-command-connection) | Builds an [ICommand](#command) for the connection. |
| [transaction(connection, isolation?)](#typescript-idriver-transaction-connection-isolation) | Builds an [ITransaction](#transaction) for the connection. |
| [rowTypeConverters](#typescript-idriver-rowtypeconverters) | Row cell converters keyed by canonical type name (`bool`, `datetime`, …). Empty for stubs and file dialects. |
| [columnType(name)](#typescript-idriver-columntype-name) | SQL type name for a canonical column type (`int`, `string`, `bool`, …). The default, exported as `columnType`, returns `name`. MySQL, Postgres, and SQLite override it. An unknown name throws. |

###### connection(options?) {#typescript-idriver-connection-options}

Builds an [IConnection](#connection) from the options bag.

**Definition**

```typescript
connection(options?: DatasourceOptions): IConnection
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | [DatasourceOptions](#typescript-datasourceoptions) |

**Returns**

[IConnection](#connection)

###### command(connection) {#typescript-idriver-command-connection}

Builds an [ICommand](#command) for the connection.

**Definition**

```typescript
command(connection: IConnection): ICommand
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | open or closed [IConnection](#connection) |

**Returns**

[ICommand](#command)

###### transaction(connection, isolation?) {#typescript-idriver-transaction-connection-isolation}

Builds an [ITransaction](#transaction) for the connection.

**Definition**

```typescript
transaction(connection: IConnection, isolation?: IsolationLevel): ITransaction
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | open or closed [IConnection](#connection) |
| `isolation` | defaults to `readCommitted` |

**Returns**

[ITransaction](#transaction)

###### rowTypeConverters {#typescript-idriver-rowtypeconverters}

Row cell converters keyed by canonical type name (`bool`, `datetime`, …). Empty for stubs and file dialects.

**Definition**

```typescript
rowTypeConverters: Record<string, TypeConverter<Cell, Cell>>
```

**Returns**

`Record<string, TypeConverter<Cell, Cell>>`

###### columnType(name) {#typescript-idriver-columntype-name}

SQL type name for a canonical column type (`int`, `string`, `bool`, …). The default, exported as `columnType`, returns `name`. MySQL, Postgres, and SQLite override it. An unknown name throws.

**Definition**

```typescript
columnType(name: string): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | canonical type name |

**Returns**

`string`

##### C#

##### IDriver {#csharp-idriver}

```csharp
public interface IDriver
```

**Methods**

| Name | Description |
| --- | --- |
| [Connection(options)](#csharp-idriver-connection-options) | Builds an [IConnection](#connection) from the options bag. |
| [Command(connection)](#csharp-idriver-command-connection) | Builds an [ICommand](#command) for the connection. |
| [Transaction(connection, isolation)](#csharp-idriver-transaction-connection-isolation) | Builds an [ITransaction](#transaction) for the connection. |
| [RowTypeConverters](#csharp-idriver-rowtypeconverters) | Row cell converters keyed by canonical type name (`bool`, `datetime`, …). Empty for stubs and file dialects. |
| [ColumnType(name)](#csharp-idriver-columntype-name) | SQL type name for a canonical column type (`int`, `string`, `bool`, …). The default returns `name`. MySQL, Postgres, and SQLite override it. An unknown name throws. |

###### Connection(options) {#csharp-idriver-connection-options}

Builds an [IConnection](#connection) from the options bag.

**Definition**

```csharp
IConnection Connection(IReadOnlyDictionary<string, string?>? options)
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | [DatasourceOptions](#csharp-datasourceoptions) |

**Returns**

[IConnection](#connection)

###### Command(connection) {#csharp-idriver-command-connection}

Builds an [ICommand](#command) for the connection.

**Definition**

```csharp
ICommand Command(IConnection connection)
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | open or closed [IConnection](#connection) |

**Returns**

[ICommand](#command)

###### Transaction(connection, isolation) {#csharp-idriver-transaction-connection-isolation}

Builds an [ITransaction](#transaction) for the connection.

**Definition**

```csharp
ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.ReadCommitted)
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | open or closed [IConnection](#connection) |
| `isolation` | defaults to `ReadCommitted` |

**Returns**

[ITransaction](#transaction)

###### RowTypeConverters {#csharp-idriver-rowtypeconverters}

Row cell converters keyed by canonical type name (`bool`, `datetime`, …). Empty for stubs and file dialects.

**Definition**

```csharp
IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters { get; }
```

**Returns**

`IReadOnlyDictionary<string, TypeConverter<Value, Value>>`

###### ColumnType(name) {#csharp-idriver-columntype-name}

SQL type name for a canonical column type (`int`, `string`, `bool`, …). The default returns `name`. MySQL, Postgres, and SQLite override it. An unknown name throws.

**Definition**

```csharp
string ColumnType(string name)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | canonical type name |

**Returns**

`string`

##### Rust

##### Driver {#rust-driver}

```rust
pub trait Driver
```

**Methods**

| Name | Description |
| --- | --- |
| [connection(options)](#rust-driver-connection-options) | Builds a [Connection](#connection) from the options bag. |
| [command(connection)](#rust-driver-command-connection) | Builds a [Command](#command) for the connection. |
| [transaction(connection, isolation)](#rust-driver-transaction-connection-isolation) | Builds a [Transaction](#transaction) for the connection. |
| [row_type_converters()](#rust-driver-row_type_converters) | Row cell converters keyed by canonical type name (`bool`, `datetime`, …). Empty for stubs and file dialects. |
| [column_type(name)](#rust-driver-column_type-name) | SQL type name for a canonical column type (`int`, `string`, `bool`, …). The default returns `name`. MySQL, Postgres, and SQLite override it. An unknown name returns an error. |

###### connection(options) {#rust-driver-connection-options}

Builds a [Connection](#connection) from the options bag.

**Definition**

```rust
fn connection(&self, options: Option<&HashMap<String, String>>) -> Arc<dyn Connection>
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | [DatasourceOptions](#rust-datasourceoptions) |

**Returns**

`Arc<dyn `[Connection](#connection)`>`

###### command(connection) {#rust-driver-command-connection}

Builds a [Command](#command) for the connection.

**Definition**

```rust
fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command>
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | open or closed [Connection](#connection) |

**Returns**

`Arc<dyn `[Command](#command)`>`

###### transaction(connection, isolation) {#rust-driver-transaction-connection-isolation}

Builds a [Transaction](#transaction) for the connection.

**Definition**

```rust
fn transaction(&self, connection: Arc<dyn Connection>, isolation: IsolationLevel) -> Arc<dyn Transaction>
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | open or closed [Connection](#connection) |
| `isolation` | defaults to `ReadCommitted` at the call site |

**Returns**

`Arc<dyn `[Transaction](#transaction)`>`

###### row_type_converters() {#rust-driver-row_type_converters}

Row cell converters keyed by canonical type name (`bool`, `datetime`, …). Empty for stubs and file dialects.

**Definition**

```rust
fn row_type_converters(&self) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>
```

**Returns**

`HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>`

###### column_type(name) {#rust-driver-column_type-name}

SQL type name for a canonical column type (`int`, `string`, `bool`, …). The default returns `name`. MySQL, Postgres, and SQLite override it. An unknown name returns an error.

**Definition**

```rust
fn column_type(&self, name: &str) -> Result<String>
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | canonical type name |

**Returns**

`Result<String>`

### Remarks

`Connection` / `connection` / `connection` receives the options bag. A provider parses `DATASOURCE_URL` with [DatasourceUrlParser](#datasource-url). `Command` / `command` / `command` delegates to the connection's `CreateCommand` / `createCommand` / `create_command`. `Transaction` / `transaction` / `transaction` builds the dialect transaction. Isolation defaults to read committed.

`RowTypeConverters` / `rowTypeConverters` / `row_type_converters` maps a canonical type name to a [TypeConverter](#type-converters). SQL dialects register `bool`, `datetime`, `uuid`, `decimal`, and `binary` when the engine cannot return those as canonical cells. Memory, document, and file providers return an empty map.

The statement factory formats `SelectInsertedIdStatement` as a `SelectStatement`, and the repository reads that row. Commands do not sniff SQL for the inserted id. The statement's own `GetText` / `getText` / `get_text` throws until a provider formats it.

`ColumnType` / `columnType` / `column_type` maps a canonical column type to the engine SQL type name. It does not convert cell values. The default returns the name unchanged. MySQL, Postgres, and SQLite override it and throw on an unknown name.

> **TypeScript**
>
> Assign the exported `columnType` function.

> **C#**
>
> Inherit the default method.

> **Rust**
>
> Inherit the default method.

SQLite, MySQL, and Postgres each keep create-table rendering on their own `CreateTableStatementRenderer`. The statement factory does not subclass that renderer. It maps a compiled create table to the rendered text and maps `SelectInsertedIdStatement` to `SELECT last_insert_rowid()`, `SELECT LAST_INSERT_ID()`, or `SELECT lastval()`. Statement `Execute` / `executeStatement` / `execute_statement` and the reader overloads call the factory before the string execute. The renderer overrides column type. MySQL and Postgres also override default and identity. A Postgres `bool` default goes through the bool type converter, so `1` becomes `TRUE`. SQLite leaves identity empty. Every other statement is left unchanged. The factory is not exported.

The package exports only the driver type.

> **TypeScript**
>
> `src/index.ts` is one line, `export { SqliteDriver } from "./SqliteDriver.ts"`. `package.json` `exports` points only at that file.

> **C#**
>
> That class is `public`. Connection, command, transaction, options, and converters are `internal`. `InternalsVisibleTo` opens them to the test project and to an engine assembly such as `Datasources.Sqlite.Microsoft`. `GetExportedTypes()` is exactly the driver. The engine assembly exports nothing.

> **Rust**
>
> `lib.rs` declares private modules and one `pub use` of the driver. `pub` items inside those modules stay crate-private.

`PublicSurfaceTests` in each C# provider test project, the `exports only the provider` test in `typescript/datasources`, and `providers_export_only_their_provider` in `datasources` reject a second export.

### Examples

#### Register and Create

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Memory.Tests/RegisterAndCreateTests.cs#L10-L16)

```csharp
DatasourceFactory.Register("memory", new MemoryDriver());
var ds = DatasourceFactory.Create(new Dictionary<string, string?> { ["PROVIDER"] = "memory" });
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L40-L45)

```rust
register("memory", Arc::new(MemoryDriver));
let ds = create(Some(&HashMap::from([("PROVIDER".into(), "memory".into())]))).unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/memory/src/index.test.ts#L31-L36)

```typescript
DatasourceFactory.register("memory", new MemoryDriver());
const ds = DatasourceFactory.create({ PROVIDER: "memory" });
```

#### Mock driver

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources/Mocks/MockDriver.cs#L3-L11)

```csharp
public sealed class MockDriver : IDriver
{
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new MockConnection();
    public ICommand Command(IConnection connection) => new MockCommand(connection);
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.ReadCommitted) =>
        new MockTransaction(connection) { Isolation = isolation };
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters { get; } =
        new Dictionary<string, TypeConverter<Value, Value>>();
}
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mocks/mock_driver.rs#L22-L44)

```rust
impl Driver for MockDriver {
    fn connection(&self, _: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        MockConnection::new()
    }

    fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command> {
        connection.create_command()
    }

    fn transaction(
        &self,
        _: Arc<dyn Connection>,
        isolation: IsolationLevel,
    ) -> Arc<dyn Transaction> {
        let txn = MockTransaction::new_unbound();
        txn.set_isolation(isolation);
        Arc::new(txn)
    }

    fn row_type_converters(
        &self,
    ) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
        HashMap::new()
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mocks/mock-driver.ts#L14-L27)

```typescript
export class MockDriver implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = {};
  columnType = identityColumnType;
  connection = (_options?: DatasourceOptions): IConnection => new MockConnection();
  command = (connection: IConnection): ICommand => new MockCommand(connection);
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "readCommitted",
  ): ITransaction => {
    const txn = new MockTransaction(connection);
    txn.isolation = isolation;
    return txn;
  };
}
```
