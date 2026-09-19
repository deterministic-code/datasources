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

Install Core

```typescript
npm install @datasources/core
```

Install Providers

```typescript
npm install @datasources/register
npm install @datasources/sqlite
npm install @datasources/mysql
npm install @datasources/postgres
npm install @datasources/memory
npm install @datasources/json
```

##### C#

Install Core

```csharp
dotnet add package Datasources
```

Install Providers

```csharp
dotnet add package Datasources.Hosting
dotnet add package Datasources.Sqlite
dotnet add package Datasources.Sqlite.Microsoft
dotnet add package Datasources.Mysql
dotnet add package Datasources.Postgres
dotnet add package Datasources.Memory
dotnet add package Datasources.Json
```

##### Rust

Install Core

```rust
cargo add datasources
```

Install Providers

```rust
cargo add datasources-register --features sqlite
cargo add datasources-sqlite
cargo add datasources-mysql
cargo add datasources-postgres
cargo add datasources-memory
cargo add datasources-json
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
var ds = DatasourceFactory.Create();
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
use datasources::{create_default, Parameter};

let ds = create_default()?;
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

const ds = Datasource.create();
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

A `DeepCRUDRepository` wraps a CRUD repository, fills related fields after `get`, and writes nested `many` children on create / update / delete.

##### C#

```csharp
var contacts = new DeepCRUDRepository<Contact>(contactRepo);
contacts.AddEnriched("contact_type_name", "contact_type_id", types, "id", "name");
contacts.AddEnriched("addresses", "id", addresses, "contact_id", many: true,
    nest: node => node.Include(addressTypes, "address_type_name", "address_type_id", "id", "name"));

var loaded = await contacts.GetSingle(WhereBuilder.Equals("id", 10));
await contacts.Create(new Contact { first_name = "Ada", addresses = [new Address { line1 = "1 Main" }] });
```

##### Rust

```rust
let mut contacts = DeepCRUDRepository::new(contact_repo);
contacts
    .add_enriched("contact_type_name", "contact_type_id", types, "id", Some("name".into()), false)
    .add_enriched_repo_with("addresses", "id", addresses, "contact_id", None, true, |node| {
        node.include(address_types, "address_type_name", "address_type_id", "id", Some("name".into()), false);
    });

let loaded = contacts.get_single_where(&WhereBuilder::equals("id", 10)).await?;
contacts.create(contact).await?;
```

##### TypeScript

```typescript
const contacts = new DeepCRUDRepository(contactRepo);
contacts.addEnriched("contact_type_name", "contact_type_id", types, "id", "name");
contacts.addEnriched("addresses", "id", addresses, "contact_id", undefined, true, (node) =>
  node.include(addressTypes, "address_type_name", "address_type_id", "id", "name"),
);

const loaded = await contacts.getSingle(WhereBuilder.equals("id", 10));
await contacts.create({ first_name: "Ada", addresses: [{ line1: "1 Main" }] });
```

See [Repositories](#readonly-repository) for readonly, CRUD, and bulk.

### Mappers

`TransformMapper` reshapes an already-converted object. Repository get is convert, then map.

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

See [Mappers](#mappers) for `NameMapper`, `FlatToTypeMapper`, and `StatementMapper`.

### Type Converter

`TypeConverter` is only `Convert` / `ReverseConvert`. Attach field converters on `RowTypeConverter` when the native provider cannot map the type.

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

See [TypeConverters](#type-converters).

## Feature Matrix

Parity across C#, Rust, and TypeScript. Standard modules first; opinionated repositories above them.

### Feature stack

| Feature | C# | Rust | TypeScript |
| --- | :---: | :---: | :---: |
| [Datasource](#datasource) | ✅ | ✅ | ✅ |
| [Mappers](#mappers) | ✅ | ✅ | ✅ |
| [Type Converters](#typeconverters) | ✅ | ✅ | ✅ |
| [Connection](#connection) | ✅ | ✅ | ✅ |
| [Transaction](#transaction) | ✅ | ✅ | ✅ |
| [Command](#command) | ✅ | ✅ | ✅ |
| [Reader](#reader) | ✅ | ✅ | ✅ |
| [Statements](#statements) | ✅ | ✅ | ✅ |
| [Readonly repository](#readonly-repository) | ✅ | ✅ | ✅ |
| [CRUD repository](#crud-repository) | ✅ | ✅ | ✅ |
| [Enriched repository](#enriched-repository) | ✅ | ✅ | ✅ |
| [Bulk repository](#bulk-repository) | ✅ | ✅ | ✅ |

### Providers

| Provider | C# | Rust | TypeScript |
| --- | :---: | :---: | :---: |
| json | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> |
| memory | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> |
| sqlite | ✅ | ✅ | ✅ |
| text | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> | <span style="color:#9ca3af">✕</span> |
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

TODO: register a provider, create a connection, `createRepository`, then `create` / `get` / `update` / `delete`.

### Run a command

TODO: open a connection, `createCommand`, `execute` / `executeReader`. See [Command](#command) for the current samples.

### Transaction

TODO: `startTransaction`, work, then `commit` or `abort`.

## Datasource

One configured datasource: provider name plus factories for connections, commands, and repositories. `Create` / `create` / `create_default` is on [DatasourceFactory](#datasourcefactory).

### Classes

##### TypeScript

##### Datasource {#typescript-datasource}

```typescript
class Datasource implements IDatasource
```

**Constructor**

| Name | Description |
| --- | --- |
| [Datasource(provider, newConnection, newRepository)](#typescript-datasource-datasource-provider-newconnection-newrepository) | Builds a datasource from a connection factory and a repository factory. |

**Methods**

| Name | Description |
| --- | --- |
| [create(options?)](#typescript-datasource-create-options) | Alias of [DatasourceFactory.create](#datasourcefactory). `undefined` options read process env; a dictionary is used as-is. |

###### Datasource(provider, newConnection, newRepository) {#typescript-datasource-datasource-provider-newconnection-newrepository}

Builds a datasource from a connection factory and a repository factory.

**Definition**

```typescript
constructor(provider: string, newConnection: () => IConnection, newRepository: (connection: IConnection) => object)
```

**Parameters**

| Name | Description |
| --- | --- |
| `provider` | registered provider name |
| `newConnection` | constructs the default [IConnection](#connection) |
| `newRepository` | constructs a repository from an [IConnection](#connection) |

###### create(options?) {#typescript-datasource-create-options}

Alias of [DatasourceFactory.create](#datasourcefactory). `undefined` options read process env; a dictionary is used as-is.

**Definition**

```typescript
static create(options?: DatasourceOptions): IDatasource
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | [DatasourceOptions](#typescript-datasourceoptions); omitted uses `DATABASE_URL` / `DATABASE_SCHEMA` |

**Returns**

[IDatasource](#typescript-idatasource)

##### IDatasource {#typescript-idatasource}

```typescript
interface IDatasource
```

**Properties**

| Name | Description |
| --- | --- |
| [provider](#typescript-idatasource-provider) | Registered provider name. |

**Methods**

| Name | Description |
| --- | --- |
| [createConnection()](#typescript-idatasource-createconnection) | Creates a closed connection. The caller opens it. |
| [createCommand(connection?)](#typescript-idatasource-createcommand-connection) | Creates a command bound to an optional open connection. |
| [createRepository(connection?)](#typescript-idatasource-createrepository-connection) | Creates a CRUD repository bound to an optional open connection. |
| [createReadonlyRepository(connection?)](#typescript-idatasource-createreadonlyrepository-connection) | Creates a read-only repository bound to an optional open connection. |

###### provider {#typescript-idatasource-provider}

Registered provider name.

**Definition**

```typescript
readonly provider: string
```

###### createConnection() {#typescript-idatasource-createconnection}

Creates a closed connection. The caller opens it.

**Definition**

```typescript
createConnection(): Promise<IConnection>
```

**Returns**

`Promise<[IConnection](#connection)>`

###### createCommand(connection?) {#typescript-idatasource-createcommand-connection}

Creates a command bound to an optional open connection.

**Definition**

```typescript
createCommand(connection?: IConnection): ICommand
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional open [IConnection](#connection); omitted constructs one |

**Returns**

[ICommand](#command)

###### createRepository(connection?) {#typescript-idatasource-createrepository-connection}

Creates a CRUD repository bound to an optional open connection.

**Definition**

```typescript
createRepository<T>(connection?: IConnection): ICRUDRepository<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional open [IConnection](#connection); omitted constructs one |

**Returns**

[ICRUDRepository](#typescript-crud-repository)`<T>`

###### createReadonlyRepository(connection?) {#typescript-idatasource-createreadonlyrepository-connection}

Creates a read-only repository bound to an optional open connection.

**Definition**

```typescript
createReadonlyRepository<T>(connection?: IConnection): IReadonlyRepository<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional open [IConnection](#connection); omitted constructs one |

**Returns**

[IReadonlyRepository](#typescript-readonly-repository)`<T>`

##### DatasourceOptions {#typescript-datasourceoptions}

```typescript
type DatasourceOptions = Record<string, string>
```

**Properties**

| Name | Description |
| --- | --- |
| [provider](#typescript-datasourceoptions-provider) | Which datasource to return. |
| [database_url](#typescript-datasourceoptions-database_url) | Connection URL. The scheme selects the provider when `provider` is omitted. |
| [database_schema](#typescript-datasourceoptions-database_schema) | Default schema. |
| [url](#typescript-datasourceoptions-url) | Alias of `database_url` passed through to the provider. |
| [schema](#typescript-datasourceoptions-schema) | Alias of `database_schema` passed through to the provider. |

###### provider {#typescript-datasourceoptions-provider}

Which datasource to return.

**Definition**

```typescript
provider?: string
```

###### database_url {#typescript-datasourceoptions-database_url}

Connection URL. The scheme selects the provider when `provider` is omitted.

**Definition**

```typescript
database_url?: string
```

###### database_schema {#typescript-datasourceoptions-database_schema}

Default schema.

**Definition**

```typescript
database_schema?: string
```

###### url {#typescript-datasourceoptions-url}

Alias of `database_url` passed through to the provider.

**Definition**

```typescript
url?: string
```

###### schema {#typescript-datasourceoptions-schema}

Alias of `database_schema` passed through to the provider.

**Definition**

```typescript
schema?: string
```

##### C#

##### Datasource {#csharp-datasource}

```csharp
public sealed class Datasource : IDatasource
```

**Constructor**

| Name | Description |
| --- | --- |
| [Datasource(provider, newConnection, newRepository)](#csharp-datasource-datasource-provider-newconnection-newrepository) | Builds a datasource from a connection factory and a repository factory. |

###### Datasource(provider, newConnection, newRepository) {#csharp-datasource-datasource-provider-newconnection-newrepository}

Builds a datasource from a connection factory and a repository factory.

**Definition**

```csharp
public Datasource(string provider, Func<IConnection> newConnection, Func<IConnection, object> newRepository)
```

**Parameters**

| Name | Description |
| --- | --- |
| `provider` | registered provider name |
| `newConnection` | constructs the default [IConnection](#connection) |
| `newRepository` | constructs a repository from an [IConnection](#connection) |

##### IDatasource {#csharp-idatasource}

```csharp
public interface IDatasource
```

**Properties**

| Name | Description |
| --- | --- |
| [Provider](#csharp-idatasource-provider) | Registered provider name. |

**Methods**

| Name | Description |
| --- | --- |
| [CreateConnection()](#csharp-idatasource-createconnection) | Creates a closed connection. The caller opens it. |
| [CreateCommand(connection)](#csharp-idatasource-createcommand-connection) | Creates a command bound to an optional open connection. |
| [CreateRepository(connection)](#csharp-idatasource-createrepository-connection) | Creates a CRUD repository bound to an optional open connection. |
| [CreateReadonlyRepository(connection)](#csharp-idatasource-createreadonlyrepository-connection) | Creates a read-only repository bound to an optional open connection. Defaults to `CreateRepository`. |

###### Provider {#csharp-idatasource-provider}

Registered provider name.

**Definition**

```csharp
string Provider { get; }
```

###### CreateConnection() {#csharp-idatasource-createconnection}

Creates a closed connection. The caller opens it.

**Definition**

```csharp
Task<IConnection> CreateConnection()
```

**Returns**

`Task<[IConnection](#connection)>`

###### CreateCommand(connection) {#csharp-idatasource-createcommand-connection}

Creates a command bound to an optional open connection.

**Definition**

```csharp
ICommand CreateCommand(IConnection? connection = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional open [IConnection](#connection); omitted constructs one |

**Returns**

[ICommand](#command)

###### CreateRepository(connection) {#csharp-idatasource-createrepository-connection}

Creates a CRUD repository bound to an optional open connection.

**Definition**

```csharp
ICRUDRepository<T> CreateRepository<T>(IConnection? connection = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional open [IConnection](#connection); omitted constructs one |

**Returns**

[ICRUDRepository](#csharp-crud-repository)`<T>`

###### CreateReadonlyRepository(connection) {#csharp-idatasource-createreadonlyrepository-connection}

Creates a read-only repository bound to an optional open connection. Defaults to `CreateRepository`.

**Definition**

```csharp
IReadonlyRepository<T> CreateReadonlyRepository<T>(IConnection? connection = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional open [IConnection](#connection); omitted constructs one |

**Returns**

[IReadonlyRepository](#csharp-readonly-repository)`<T>`

##### DatasourceOptions {#csharp-datasourceoptions}

```csharp
IReadOnlyDictionary<string, string?>
```

**Properties**

| Name | Description |
| --- | --- |
| [provider](#csharp-datasourceoptions-provider) | Which datasource to return. |
| [database_url](#csharp-datasourceoptions-database_url) | Connection URL. The scheme selects the provider when `provider` is omitted. |
| [database_schema](#csharp-datasourceoptions-database_schema) | Default schema. |
| [url](#csharp-datasourceoptions-url) | Alias of `database_url` passed through to the provider. |
| [schema](#csharp-datasourceoptions-schema) | Alias of `database_schema` passed through to the provider. |

###### provider {#csharp-datasourceoptions-provider}

Which datasource to return.

**Definition**

```csharp
"provider"
```

###### database_url {#csharp-datasourceoptions-database_url}

Connection URL. The scheme selects the provider when `provider` is omitted.

**Definition**

```csharp
"database_url"
```

###### database_schema {#csharp-datasourceoptions-database_schema}

Default schema.

**Definition**

```csharp
"database_schema"
```

###### url {#csharp-datasourceoptions-url}

Alias of `database_url` passed through to the provider.

**Definition**

```csharp
"url"
```

###### schema {#csharp-datasourceoptions-schema}

Alias of `database_schema` passed through to the provider.

**Definition**

```csharp
"schema"
```

##### Rust

##### BuiltDatasource {#rust-builtdatasource}

```rust
pub struct BuiltDatasource
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(provider, new_connection, new_repository)](#rust-builtdatasource-new-provider-new_connection-new_repository) | Builds a datasource from a connection factory and a repository factory. |

###### new(provider, new_connection, new_repository) {#rust-builtdatasource-new-provider-new_connection-new_repository}

Builds a datasource from a connection factory and a repository factory.

**Definition**

```rust
pub fn new(provider: impl Into<String>, new_connection: impl Fn() -> Arc<dyn Connection> + Send + Sync + 'static, new_repository: impl Fn(Arc<dyn Connection>) -> Arc<dyn CRUDRepository> + Send + Sync + 'static) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `provider` | registered provider name |
| `new_connection` | constructs the default [Connection](#connection) |
| `new_repository` | constructs a repository from a [Connection](#connection) |

##### Datasource {#rust-datasource}

```rust
pub trait Datasource
```

**Methods**

| Name | Description |
| --- | --- |
| [provider()](#rust-datasource-provider) | Registered provider name. |
| [create_connection()](#rust-datasource-create_connection) | Creates a closed connection. The caller opens it. |
| [create_command(connection)](#rust-datasource-create_command-connection) | Creates a command bound to an optional open connection. |
| [create_repository(connection)](#rust-datasource-create_repository-connection) | Creates a CRUD repository bound to an optional open connection. |
| [create_readonly_repository(connection)](#rust-datasource-create_readonly_repository-connection) | Creates a read-only repository bound to an optional open connection. Defaults to `create_repository`. |

###### provider() {#rust-datasource-provider}

Registered provider name.

**Definition**

```rust
fn provider(&self) -> &str
```

**Returns**

`&str`

###### create_connection() {#rust-datasource-create_connection}

Creates a closed connection. The caller opens it.

**Definition**

```rust
async fn create_connection(&self) -> Result<Arc<dyn Connection>>
```

**Returns**

`Result<Arc<dyn `[Connection](#connection)`>>`

###### create_command(connection) {#rust-datasource-create_command-connection}

Creates a command bound to an optional open connection.

**Definition**

```rust
fn create_command(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn Command>
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional open [Connection](#connection); omitted constructs one |

**Returns**

`Arc<dyn `[Command](#command)`>`

###### create_repository(connection) {#rust-datasource-create_repository-connection}

Creates a CRUD repository bound to an optional open connection.

**Definition**

```rust
fn create_repository(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn CRUDRepository>
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional open [Connection](#connection); omitted constructs one |

**Returns**

`Arc<dyn `[CRUDRepository](#rust-crud-repository)`>`

###### create_readonly_repository(connection) {#rust-datasource-create_readonly_repository-connection}

Creates a read-only repository bound to an optional open connection. Defaults to `create_repository`.

**Definition**

```rust
fn create_readonly_repository(&self, connection: Option<Arc<dyn Connection>>) -> Arc<dyn ReadonlyRepository>
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional open [Connection](#connection); omitted constructs one |

**Returns**

`Arc<dyn `[ReadonlyRepository](#rust-readonly-repository)`>`

##### DatasourceOptions {#rust-datasourceoptions}

```rust
HashMap<String, String>
```

**Properties**

| Name | Description |
| --- | --- |
| [provider](#rust-datasourceoptions-provider) | Which datasource to return. |
| [database_url](#rust-datasourceoptions-database_url) | Connection URL. The scheme selects the provider when `provider` is omitted. |
| [database_schema](#rust-datasourceoptions-database_schema) | Default schema. |
| [url](#rust-datasourceoptions-url) | Alias of `database_url` passed through to the provider. |
| [schema](#rust-datasourceoptions-schema) | Alias of `database_schema` passed through to the provider. |

###### provider {#rust-datasourceoptions-provider}

Which datasource to return.

**Definition**

```rust
"provider"
```

###### database_url {#rust-datasourceoptions-database_url}

Connection URL. The scheme selects the provider when `provider` is omitted.

**Definition**

```rust
"database_url"
```

###### database_schema {#rust-datasourceoptions-database_schema}

Default schema.

**Definition**

```rust
"database_schema"
```

###### url {#rust-datasourceoptions-url}

Alias of `database_url` passed through to the provider.

**Definition**

```rust
"url"
```

###### schema {#rust-datasourceoptions-schema}

Alias of `database_schema` passed through to the provider.

**Definition**

```rust
"schema"
```

### Remarks

`DatasourceFactory.Create(null)` / `Datasource.create()` / `create(None)` reads process `DATABASE_URL` and `DATABASE_SCHEMA` (as `database_url` / `database_schema`) and registers names from process `INSTALLED_PROVIDERS`. A non-null options bag is used as-is; missing `provider` and `database_url` throw.

Listed factories are registered through `Datasources.Hosting`, `@datasources/register`, or `datasources-register` when that package is present.

`Reset` / `reset` clears the registry. Use it in tests.

See [DatasourceFactory](#datasourcefactory) for the factory contract, and [CLI](#command-line-cli) for `setup` and adding providers.

### Examples

#### Create from environment

##### C#

```csharp
var ds = DatasourceFactory.Create();
```

##### Rust

```rust
let ds = create_default()?;
```

##### TypeScript

```typescript
const ds = Datasource.create();
```

## DatasourceFactory

Resolves a provider from options and returns one configured datasource.

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
| [register(provider, connection, repository)](#typescript-datasourcefactory-register-provider-connection-repository) | Registers a connection factory and a repository factory by provider name. |
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

###### register(provider, connection, repository) {#typescript-datasourcefactory-register-provider-connection-repository}

Registers a connection factory and a repository factory by provider name.

**Definition**

```typescript
static register(provider: string, connection: (options?: DatasourceOptions) => IConnection, repository: (connection: IConnection) => object): void
```

**Parameters**

| Name | Description |
| --- | --- |
| `provider` | registry key (`sqlite`, `mysql`, …) |
| `connection` | builds an [IConnection](#connection) from the options bag |
| `repository` | builds a repository from an [IConnection](#connection) |

###### create(options?) {#typescript-datasourcefactory-create-options}

Creates one datasource. `undefined` options read process env; a dictionary is used as-is.

**Definition**

```typescript
static create(options?: DatasourceOptions): IDatasource
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | [DatasourceOptions](#typescript-datasourceoptions); omitted uses `DATABASE_URL` / `DATABASE_SCHEMA` |

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
| [Register(provider, connection, repository)](#csharp-datasourcefactory-register-provider-connection-repository) | Registers a connection factory and a repository factory by provider name. |
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

###### Register(provider, connection, repository) {#csharp-datasourcefactory-register-provider-connection-repository}

Registers a connection factory and a repository factory by provider name.

**Definition**

```csharp
public static void Register(string provider, Func<IReadOnlyDictionary<string, string?>?, IConnection> connection, Func<IConnection, object> repository)
```

**Parameters**

| Name | Description |
| --- | --- |
| `provider` | registry key (`sqlite`, `mysql`, …) |
| `connection` | builds an [IConnection](#connection) from the options bag |
| `repository` | builds a repository from an [IConnection](#connection) |

###### Create(options) {#csharp-datasourcefactory-create-options}

Creates one datasource. `null` options read process `.env`; a dictionary is used as-is.

**Definition**

```csharp
public static IDatasource Create(IReadOnlyDictionary<string, string?>? options = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | [DatasourceOptions](#csharp-datasourceoptions); omitted uses `DATABASE_URL` / `DATABASE_SCHEMA` |

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
| [register(provider, connection, repository)](#rust-datasources-register-provider-connection-repository) | Registers a connection factory and a repository factory by provider name. |
| [create(options)](#rust-datasources-create-options) | Creates one datasource. `None` reads process env; `Some` is used as-is. |
| [create_default()](#rust-datasources-create_default) | `create(None)`. |
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

###### register(provider, connection, repository) {#rust-datasources-register-provider-connection-repository}

Registers a connection factory and a repository factory by provider name.

**Definition**

```rust
pub fn register(provider: impl Into<String>, connection: impl Fn(Option<&HashMap<String, String>>) -> Arc<dyn Connection> + Send + Sync + 'static, repository: impl Fn(Arc<dyn Connection>) -> Arc<dyn CRUDRepository> + Send + Sync + 'static)
```

**Parameters**

| Name | Description |
| --- | --- |
| `provider` | registry key (`sqlite`, `mysql`, …) |
| `connection` | builds a [Connection](#connection) from the options bag |
| `repository` | builds a repository from a [Connection](#connection) |

###### create(options) {#rust-datasources-create-options}

Creates one datasource. `None` reads process env; `Some` is used as-is.

**Definition**

```rust
pub fn create(options: Option<&HashMap<String, String>>) -> Result<Arc<dyn Datasource>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | [DatasourceOptions](#rust-datasourceoptions); omitted uses `DATABASE_URL` / `DATABASE_SCHEMA` |

**Returns**

`Result<Arc<dyn `[Datasource](#rust-datasource)`>>`

###### create_default() {#rust-datasources-create_default}

`create(None)`.

**Definition**

```rust
pub fn create_default() -> Result<Arc<dyn Datasource>>
```

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

Each language registers a connection factory and a repository factory, then `Create` / `create` / `create_default` returns one [Datasource](#datasource). See [Datasource](#datasource) for the instance contract.

A missing factory throws `ProviderNotInstalledError` / `ProviderNotInstalledException` / `DatasourceError::ProviderNotInstalled`.

`Datasources.Hosting`, `@datasources/register`, and `datasources-register` register each name in `INSTALLED_PROVIDERS` (and the provider inferred from `DATABASE_URL`) when `Create` is called with null / `undefined` / `None` options.

### Examples

#### Register a factory

##### C#

```csharp
DatasourceFactory.Register("memory", opts => new MemoryConnection(opts), MemoryConnection.Repository);
var ds = DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = "memory" });
```

##### Rust

```rust
register("memory", MemoryConnection::from_options, MemoryConnection::repository);
let ds = create(Some(&HashMap::from([("provider".into(), "memory".into())])))?;
```

##### TypeScript

```typescript
DatasourceFactory.register("memory", opts => new MemoryConnection(opts), MemoryConnection.repository);
const ds = Datasource.create({ provider: "memory" });
```

## Connection

`CreateConnection` / `createConnection` / `create_connection` is async so a dialect can do async work later. Today it only constructs a closed named connection. The caller opens it.

### IConnection

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IConnection` | `Connection` | `IConnection` |
| `Provider` | `provider()` | `provider` |
| `IsOpen` | `is_open()` | `isOpen` |
| `CreateCommand` | `create_command` | `createCommand` |
| `Open` | `open` | `open` |
| `Close` | `close` | `close` |
| `StartTransaction` | `start_transaction` | `startTransaction` |

Working dialects (`json`, `memory`, `sqlite`) open against injected ports (`IFileStore` / `FileStore`, `MemoryStore`, `ISqliteDriver` / `SqliteDriver`). Stub dialects return a named connection; open / close / start-transaction throw or return not-implemented.

### Transaction

`StartTransaction` / `startTransaction` / `start_transaction` begins a transaction on an open connection.

### Create and open

##### C#

```csharp
DatasourceFactory.Register("json", opts => new JsonConnection(opts), JsonConnection.Repository);

var json = new Datasource("json", () => new JsonConnection(new JsonConnectionOptions { Data = new JsonArray() }), JsonConnection.Repository);
var connection = await json.CreateConnection();
await connection.Open();
```

##### Rust

```rust
use datasources::{create, register};
use datasources_json::JsonConnection;
use std::collections::HashMap;

register("json", JsonConnection::from_options, JsonConnection::repository);

let json = create(Some(&HashMap::from([("provider".into(), "json".into())])))?;
let connection = json.create_connection().await?;
connection.open().await?;
```

##### TypeScript

```typescript
import { Datasource, DatasourceFactory } from "@datasources/core";
import { JsonConnection } from "@datasources/json";

DatasourceFactory.register("json", opts => new JsonConnection(opts), JsonConnection.repository);

const json = Datasource.create({ provider: "json" });
const connection = await json.createConnection();
await connection.open();
```

### Startup

Apps create the default datasource and connection at process start, then hold the closed connection. Call `Open` / `open` on first use (a no-op if already open). Do not open during bootstrap.

##### C#

```csharp
var datasource = DatasourceFactory.Create();
var connection = await datasource.CreateConnection();

Task EnsureOpen() => connection.IsOpen ? Task.CompletedTask : connection.Open();
```

##### Rust

```rust
use std::sync::Arc;
use datasources::{create_default, Connection, Result};

let datasource = create_default()?;
let connection = datasource.create_connection().await?;

async fn ensure_open(connection: &Arc<dyn Connection>) -> Result<()> {
    if connection.is_open() {
        return Ok(());
    }
    connection.open().await
}
```

##### TypeScript

```typescript
const datasource = Datasource.create();
const connection = await datasource.createConnection();

async function ensureOpen() {
  if (!connection.isOpen) await connection.open();
}
```

### Mock

Depend on `IConnection` / `Connection`. Use `MockConnection` / `MockDatasource`. `CreateConnection` returns the configured mock and does not open it.

##### C#

```csharp
var ds = new MockDatasource();
var connection = await ds.CreateConnection();
await connection.Open();
Assert.True(((MockConnection)connection).IsOpen);

var failing = new MockConnection { OpenError = new InvalidOperationException("open-fail") };
ds.Connection = failing;
await Assert.ThrowsAsync<InvalidOperationException>(() => failing.Open());
```

##### Rust

```rust
use datasources::{Datasource, MockConnection, MockDatasource};

let ds = MockDatasource::new("mock");
let connection = ds.create_connection().await?;
connection.open().await?;

let failing = MockConnection::new();
failing.set_open_error("open-fail");
assert!(failing.open().await.is_err());
```

##### TypeScript

```typescript
const ds = new MockDatasource();
const connection = await ds.createConnection();
await connection.open();
expect((connection as MockConnection).isOpen).toBe(true);

const failing = new MockConnection();
failing.openError = new Error("open-fail");
await expect(failing.open()).rejects.toThrow("open-fail");
```

Provider tests inject fakes on options (`MemoryFileStore`, `ScriptedDriver`) and then open.

## Command

`CreateCommand` / `createCommand` / `create_command` is sync. It binds an optional connection (or constructs one) and returns a named command. The caller executes it. The connection must be open.

Working dialects (`json`, `memory`) treat `text` as `insert` / `update` / `delete`. `sqlite` treats `text` as SQL against the injected driver. Stub dialects return a named command; execute / execute-reader throw or return not-implemented.

The Rust `Command` trait has no connection getter. Stub `create_command` stores the optional connection on the named type; working dialects still construct their own typed connection today.

### ICommand

| C# | Rust | TypeScript |
| --- | --- | --- |
| `ICommand` | `Command` | `ICommand` |
| `AddMiddleware` | `add_middleware` | `addMiddleware` |
| `ExecuteReader` | `execute_reader` | `executeReader` |
| `Execute` | `execute` | `execute` |

`ICommand` supports `addMiddleware` / `AddMiddleware` / `add_middleware` with `beforeQuery` / `afterQuery`. A `beforeQuery` error skips the query. A query error still runs `afterQuery` with `error` and elapsed time set. `ConsoleCommandMiddleware` logs Start/Finish lines to the console.

### Reader

`ExecuteReader` / `executeReader` / `execute_reader` returns a reader over query results.

### Create and execute

##### C#

```csharp
DatasourceFactory.Register("json", opts => new JsonConnection(opts), JsonConnection.Repository);

var json = new Datasource("json", () => new JsonConnection(new JsonConnectionOptions { Data = new JsonArray() }), JsonConnection.Repository);
var connection = await json.CreateConnection();
await connection.Open();

var command = json.CreateCommand(connection);
await command.Execute("insert", new[] { new Parameter("id", 1) });
await command.ExecuteReader("");
```

##### Rust

```rust
use datasources::{create, register};
use datasources_json::JsonConnection;
use std::collections::HashMap;

register("json", JsonConnection::from_options, JsonConnection::repository);

let json = create(Some(&HashMap::from([("provider".into(), "json".into())])))?;
let connection = json.create_connection().await?;
connection.open().await?;

let command = json.create_command(Some(connection));
command.execute("insert", None).await?;
command.execute_reader("", None).await?;
```

##### TypeScript

```typescript
import { Datasource, DatasourceFactory } from "@datasources/core";
import { JsonConnection } from "@datasources/json";

DatasourceFactory.register("json", opts => new JsonConnection(opts), JsonConnection.repository);

const json = Datasource.create({ provider: "json" });
const connection = await json.createConnection();
await connection.open();

const command = json.createCommand(connection);
await command.execute("insert", [{ name: "id", value: 1 }]);
await command.executeReader("");
```

### Mock

Depend on `ICommand` / `Command`. Use `MockCommand` / `MockDatasource`.

##### C#

```csharp
var ds = new MockDatasource();
var command = ds.CreateCommand();
await command.Execute("insert");
Assert.Single(((MockCommand)command).ExecuteCalls);

var failing = new MockCommand(ds.Connection) { ExecuteError = new InvalidOperationException("exec-fail") };
ds.Command = failing;
await Assert.ThrowsAsync<InvalidOperationException>(() => failing.Execute(""));
```

##### Rust

```rust
use datasources::{Datasource, MockCommand, MockDatasource};

let ds = MockDatasource::new("mock");
let command = ds.create_command(None);
command.execute("", None).await?;
command.execute_reader("", None).await?;

let failing = MockCommand::new();
failing.set_execute_error("exec-fail");
assert!(failing.execute("", None).await.is_err());
```

##### TypeScript

```typescript
const ds = new MockDatasource();
const command = ds.createCommand();
await command.execute("insert");
expect((command as MockCommand).executeCalls).toHaveLength(1);

const failing = new MockCommand(ds.connection);
failing.executeError = new Error("exec-fail");
await expect(failing.execute("")).rejects.toThrow("exec-fail");
```

Provider tests inject fakes on options (`MemoryFileStore`, `ScriptedDriver`) and then execute / execute-reader.

## Statements

Shared statement construction used by SQL providers. `StatementBuilder` builds query / insert / update / delete text. `WhereBuilder` composes predicates. Dialect packages may wrap `StatementBuilder` for quoting and native column types, including `CreateStoredProcedure` the same way as `CreateInsert`.

### StatementBuilder

| C# | Rust | TypeScript |
| --- | --- | --- |
| `StatementBuilder` | `StatementBuilder` | `StatementBuilder` |
| `CreateQuery` | `create_query` | `createQuery` |
| `CreateInsert` | `create_insert` | `createInsert` |
| `CreateUpdate` | `create_update` | `createUpdate` |
| `CreateDelete` | `create_delete` | `createDelete` |
| `CreateTable` | `create_table` | `createTable` |
| `CreateStoredProcedure` | `create_stored_procedure` | `createStoredProcedure` |

### WhereBuilder

| C# | Rust | TypeScript |
| --- | --- | --- |
| `WhereBuilder` | `WhereBuilder` | `WhereBuilder` |
| `Equals` | `equals` | `equals` |
| `And` / `Or` / `Not` | `and` / `or` / `not` | `and` / `or` / `not` |

### Examples

TODO: query / insert / where samples as the builder API is documented.

## Repositories

Typed read and write over a connection. `IReadonlyRepository<T>` is get-only; `ICRUDRepository<T>` (C# / TypeScript) / `CRUDRepository` (Rust) adds `create`, `update`, and `delete`. SQL providers typically build a table repository from `RepositoryBuilder` and a `TypeConverter` mapping.

`Type` / `type` / `entity_type` is the item CLR type, constructor, or `TypeId` the repository exposes, not the table `entity` name. SQL provider generic casts may report the row or dictionary type.

### Readonly repository

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IReadonlyRepository<T>` | `ReadonlyRepository` | `IReadonlyRepository<T>` |
| `Type` | `entity_type` | `type` |
| `GetSingle` | `get_single` | `getSingle` |
| `Get` | `get` | `get` |

### CRUD repository

| C# | Rust | TypeScript |
| --- | --- | --- |
| `ICRUDRepository<T>` | `CRUDRepository` | `ICRUDRepository<T>` |
| `Create` | `create` | `create` |
| `Update` | `update` | `update` |
| `Delete` | `delete` | `delete` |

### Factory

Instance registry of factory callbacks. Register a builder, then create by entity type name or by type. The callback receives an optional connection.

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IRepositoryFactory` | `ReadonlyRepositoryFactory` | `IRepositoryFactory` |
| `ICRUDRepositoryFactory` | `CRUDRepositoryFactory` | `ICRUDRepositoryFactory` |
| `RepositoryFactory` | `RepositoryFactory` | `RepositoryFactory` |
| `Register` / `Create` | `register` / `create` | `register` / `create` |
| `Create<T>` / `Create(typeof(T))` | `register_type` / `create_type` | `register(type)` / `create(type)` |
| `RepositoryNotRegisteredException` | `DatasourceError` | `RepositoryNotRegisteredError` |

Registering CRUD also registers readonly. Registering readonly-only makes CRUD `create` throw.

```csharp
var factory = new RepositoryFactory();
factory.Register<Contact>(connection => new Repository<Contact>(connection!));
var repo = factory.Create<Contact>();
var byName = factory.Create("Contact");
```

### Enriched repository

A read-only service wraps an inner repository, then fills properties already on `T` from related repositories. This is a post-Get step (after convert-then-map). It does not join or collapse rows.

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IEnriched<T>` | `Enriched` | `IEnriched<T>` |
| `IRepositoryTree` | `RepositoryTree` | `IRepositoryTree` |
| `IRepositoryTreeNode` | `RepositoryTreeNode` | `IRepositoryTreeNode` |
| `EnrichedReadonlyRepository<T>` | `EnrichedReadonlyRepository` | `EnrichedReadonlyRepository<T>` |
| `DeepCRUDRepository<T>` | `DeepCRUDRepository` | `DeepCRUDRepository<T>` |
| `BulkCRUDRepository<T>` | `BulkCRUDRepository` | `BulkCRUDRepository<T>` |
| `AddEnriched` | `add_enriched` / `add_enriched_repo` | `addEnriched` |
| `Enrich` | `enrich` | `enrich` |

Lookups use `GetSingle` / `get_single_where` (cached per distinct key). Lists use `Get` / `get_where`. Missing lookups leave the destination default.

Contacts as the model — fill existing fields only (`contact_type_name`, `contact_source_name`, `addresses`, `phones`):

```csharp
AddEnriched("contact_type_name", "contact_type_id", types, "id", "name");
AddEnriched("contact_source_name", "contact_source_id", sources, "id", "name");
AddEnriched("addresses", "id", addresses, "contact_id", many: true);
AddEnriched("phones", "id", phones, "contact_id", many: true);
```

Nested rules run on the related objects after that node fills. `AddEnriched` stays fluent for siblings; `nest` receives the child node:

```csharp
AddEnriched("children", "id", children, "parent_id", many: true,
    nest: node => node.Include(childTypes, "type_name", "type_id", "id", "name"));
```

Nest only on walkable objects (`many: true` or a whole related row). A `select` scalar is not walked. Depth is unbounded; each node applies, then children run on the related items.

`DeepCRUDRepository` is the write side of the same tree. `Get` still enriches. `select` lookups and 1:1 embeds are not written. Each `many` node must be a CRUD repository (Rust: `add_enriched_repo`).

- **Create** — write the parent, apply generated identity to `id`, stamp each child's `by` from the parent `from`, then create children (and nested many).
- **Update** — write the parent, then replace-sync each many list (delete missing, update matches by `id`, create the rest).
- **Delete** — delete nested many rows first, then children, then the parent. Loads existing children by FK; does not trust payload lists.

### Bulk repository

`BulkCRUDRepository` inherits `DeepCRUDRepository` and adds list writes. Each item still runs the same deep create / update / delete. Responses sum `rowsAffected`; `result` is the last item.

| C# | TypeScript | Rust |
| --- | --- | --- |
| `Create(IReadOnlyList<T>)` / `BulkCreate` | `createBulk` | `create_bulk` |
| `Update(IReadOnlyList<T>)` / `BulkUpdate` | `updateBulk` | `update_bulk` |
| `Delete(IReadOnlyList<T>)` / `BulkDelete` | `deleteBulk` | `delete_bulk` |

### Examples

TODO: fill as the generic `Repository<T>` surface settles.

## Mappers

Holds source/destination labels. Column names pass through as-is.

### IMapper

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IMapper<TFrom, TTo>` | `Mapper<TFrom, TTo>` | `IMapper<TFrom, TTo>` |
| `Map` | `map` | `map` |
| `ReverseMap` | `reverse_map` | `reverseMap` |

### ISourceDestinationMapper

| C# | Rust | TypeScript |
| --- | --- | --- |
| `ISourceDestinationMapper` | `SourceDestinationMapper` | `ISourceDestinationMapper` |
| `Source` | `source` | `source` |
| `Destination` | `destination` | `destination` |

### NameMapper

| C# | Rust | TypeScript |
| --- | --- | --- |
| `NameMapper` | `NameMapper` | `NameMapper` |
| `Map` | `map` | `map` |
| `ReverseMap` | `reverse_map` | `reverseMap` |

### NameSourceDestinationMapper

| C# | Rust | TypeScript |
| --- | --- | --- |
| `NameSourceDestinationMapper` | `NameSourceDestinationMapper` | `NameSourceDestinationMapper` |
| constructor | `new` | constructor |
| `Source` | `source` | `source` |
| `Destination` | `destination` | `destination` |

### TransformMapper

| C# | Rust | TypeScript |
| --- | --- | --- |
| `TransformMapper<TFrom, TTo>` | `TransformMapper<TFrom, TTo>` | `TransformMapper<TFrom, TTo>` |
| constructor(`map`, `reverseMap`) | `new(map, reverse_map)` | constructor(`map`, `reverseMap`) |
| `Map` | `map` | `map` |
| `ReverseMap` | `reverse_map` | `reverseMap` |

### FlatToTypeMapper

Collapses denormalized join rows into a typed graph. Scalars map column → field; groups collect unique children by key. Nested group fields on a collection are omitted.

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IFlatToTypeMapper` | `FlatToTypeMapperTrait` | `IFlatToTypeMapper<T>` |
| `FlatToTypeMapper` | `FlatToTypeMapper` | `FlatToTypeMapper<T>` |
| `TypeMap` | `TypeMap` | `TypeMap<T>` |
| `TypeMapGroup` | `TypeMapGroup` | `TypeMapGroup<E>` |
| `TypeMapField` | `TypeMapField` | string or group |
| `MappedValue` | `MappedValue` | `T` fields |
| `Map` / `ReverseMap` | `map` / `reverse_map` | `map` / `reverseMap` |
| `Collapse` | `collapse` / `collapse_rows` | `collapse` |

### StatementMapper

Maps entity and column names when the repository is built, and can map each finished statement. `Rows` is the get-row `IMapper<T, T>`. `Repository` only convert-then-maps rows.

Apply in either or both places:

1. **Builder** — `RepositoryBuilder` maps `Entity` and `Columns` into `RepositoryStatements`.
2. **Per-statement** — `Query` / `Insert` / `Update` / `Delete` mappers rewrite the finished statement.

| C# | Rust | TypeScript |
| --- | --- | --- |
| `StatementMapper<T>` | `StatementMapper` | `StatementMapper<T>` |
| `Entity` | `entity` | `entity` |
| `Columns` | `columns` | `columns` |
| `Query` | `query` | `query` |
| `Insert` | `insert` | `insert` |
| `Update` | `update` | `update` |
| `Delete` | `delete` | `delete` |
| `Rows` | `rows` | `rows` |
| `Map` / `ReverseMap` | `map` / `reverse_map` | `map` / `reverseMap` |

## TypeConverters

`TypeConverter` is only `Convert` / `ReverseConvert`. `RowTypeConverter` has `Types` and `Fields`. Convert uses `Fields[name]`.

A dialect adds a `TypeConverter` only when the native provider cannot map the type. Core does not ship bool / datetime / uuid / decimal / binary converters.

### TypeConverter

| C# | Rust | TypeScript |
| --- | --- | --- |
| `TypeConverter<To, From>` | `TypeConverter<To, From>` | `TypeConverter` |
| `Convert` | `convert` | `convert` |
| `ReverseConvert` | `reverse_convert` | `reverseConvert` |

### RowTypeConverter

| C# | Rust | TypeScript |
| --- | --- | --- |
| `RowTypeConverter<T>` | `RowTypeConverter` | `RowTypeConverter` |
| `Convert` / `ReverseConvert` | `convert` / `reverse_convert` | `convert` / `reverseConvert` |
| `Types` / `Fields` | `types` / `fields` | `types` / `fields` |

## Command Line (CLI)

`@datasources/cli` installs provider packages and writes a local `.env`. Core does not parse a `.env` file; the app loads it into the process. `Create` / `create` / `create_default` then reads `INSTALLED_PROVIDERS` and `DATABASE_URL`, registers each listed factory, and creates.

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

`setup` writes these keys. `add` / `remove` update the provider list. Other keys already in the file are kept. `create` / `create_default` uses the `DATABASE_URL` scheme as the default provider (`sqlite://`, `postgres://`, `mysql://`, …).

| Key | Default | Meaning |
| --- | --- | --- |
| `INSTALLED_PROVIDERS` | scheme of `DATABASE_URL` | Comma-separated providers `Create` registers. |
| `DATABASE_URL` | `sqlite:///:memory:` | Connection URL. The scheme is the default provider. |
| `DATABASE_SCHEMA` | *(empty)* | Schema / search path (postgres). |

```sh
INSTALLED_PROVIDERS=sqlite,postgres
DATABASE_URL=sqlite:///:memory:
DATABASE_SCHEMA=public
```

SQLAlchemy-style SQLite URLs: `sqlite:///:memory:`, `sqlite:///app.db`, `sqlite:////tmp/app.db`.
