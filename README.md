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
var ds = DatasourceFactory.Default;
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
import { DatasourceFactory } from "@datasources/core";

const ds = DatasourceFactory.default;
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
| [DatasourceFactory](#datasourcefactory) | ✅ | ✅ | ✅ |
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

### Run a transaction

TODO: `startTransaction`, work, then `commit` or `abort`. See [Transaction](#transaction).

### Map a row

TODO: `TransformMapper` flatten, pass-through names, or rewrite a statement. See [Mappers](#mappers).

### Convert a row

TODO: `RowTypeConverter` convert / reverse-convert, or attach a field converter. See [TypeConverters](#typeconverters).

## DatasourceFactory

Creates a [Datasource](#datasource) for a provider and `DATASOURCE_URL`. After [setup](#setup), use `Default` / `default` / `DatasourceFactory.default` to build that datasource from `.env`.

### Classes

##### TypeScript

##### DatasourceFactory {#typescript-datasourcefactory}

```typescript
class DatasourceFactory implements IDatasourceFactory
```

**Properties**

| Name | Description |
| --- | --- |
| [default](#typescript-datasourcefactory-default) | Loads `.env` `PROVIDER` / `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. Throws when neither is set. Uses [create](#typescript-create-options). |

**Methods**

| Name | Description |
| --- | --- |
| [defaults(options)](#typescript-datasourcefactory-defaults-options) | Registers listed provider packages. Does not apply to a non-null `create` bag. |
| [register(provider, connection, repository)](#typescript-datasourcefactory-register-provider-connection-repository) | Registers a connection factory and a repository factory by provider name. |
| [create(options?)](#typescript-datasourcefactory-create-options) | Creates one datasource. `undefined` options read process env; a dictionary is used as-is. |
| [reset()](#typescript-datasourcefactory-reset) | Clears the registry. Use in tests. |
| [registered()](#typescript-datasourcefactory-registered) | Lists registered provider names. |

###### default {#typescript-datasourcefactory-default}

Loads `.env` `PROVIDER` / `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. Throws when neither is set. Uses [create](#typescript-create-options).

**Definition**

```typescript
static get default(): IDatasource
```

**Returns**

[IDatasource](#typescript-idatasource)

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
| `options` | [DatasourceOptions](#typescript-datasourceoptions); omitted uses `.env` then `DATASOURCE_URL` / `DATABASE_SCHEMA` |

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

**Properties**

| Name | Description |
| --- | --- |
| [Default](#csharp-datasourcefactory-default) | Loads `.env` `PROVIDER` / `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. Throws when neither is set. Uses [Create](#csharp-create-options). |

**Methods**

| Name | Description |
| --- | --- |
| [Defaults(options)](#csharp-datasourcefactory-defaults-options) | Registers listed provider packages. Does not apply to a non-null `Create` bag. |
| [Register(provider, connection, repository)](#csharp-datasourcefactory-register-provider-connection-repository) | Registers a connection factory and a repository factory by provider name. |
| [Create(options)](#csharp-datasourcefactory-create-options) | Creates one datasource. `null` options read process `.env`; a dictionary is used as-is. |
| [Reset()](#csharp-datasourcefactory-reset) | Clears the registry. Use in tests. |
| [Registered()](#csharp-datasourcefactory-registered) | Lists registered provider names. |

###### Default {#csharp-datasourcefactory-default}

Loads `.env` `PROVIDER` / `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. Throws when neither is set. Uses [Create](#csharp-create-options).

**Definition**

```csharp
public static IDatasource Default { get; }
```

**Returns**

[IDatasource](#csharp-idatasource)

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
| `options` | [DatasourceOptions](#csharp-datasourceoptions); omitted uses `.env` then `DATASOURCE_URL` / `DATABASE_SCHEMA` |

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
| [default()](#rust-datasources-default) | Loads `.env` `PROVIDER` / `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. Throws when neither is set. Uses [create](#rust-create-options). |
| [create_default()](#rust-datasources-create_default) | `default()`. |
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
| `options` | [DatasourceOptions](#rust-datasourceoptions); omitted uses `.env` then `DATASOURCE_URL` / `DATABASE_SCHEMA` |

**Returns**

`Result<Arc<dyn `[Datasource](#rust-datasource)`>>`

###### default() {#rust-datasources-default}

Loads `.env` `PROVIDER` / `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. Throws when neither is set. Uses [create](#rust-create-options).

**Definition**

```rust
pub fn default() -> Result<Arc<dyn Datasource>>
```

**Returns**

`Result<Arc<dyn `[Datasource](#rust-datasource)`>>`

###### create_default() {#rust-datasources-create_default}

`default()`.

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

`Default` requires `.env` `PROVIDER` and `DATASOURCE_URL`, or process `DEFAULT_PROVIDER` and `DEFAULT_DATASOURCE_URL` to override them. Missing both throws `DatasourceException` / `DatasourceError`.

A missing factory throws `ProviderNotInstalledError` / `ProviderNotInstalledException` / `DatasourceError::ProviderNotInstalled`.

`Datasources.Hosting`, `@datasources/register`, and `datasources-register` register each name in `INSTALLED_PROVIDERS` (and the provider inferred from `DATASOURCE_URL`) when `Create` is called with null / `undefined` / `None` options.

### Examples

#### Default from environment

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/DatasourceFactoryTests.cs#L24-L32)

```csharp
DatasourceFactory.Register("sqlite", _ => new MockConnection(), c => new MockRepository<object>(c));
Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
Environment.SetEnvironmentVariable("DATASOURCE_URL", "sqlite:///:memory:");
var ds = DatasourceFactory.Default;
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/datasource_factory.rs#L21-L33)

```rust
register(
    "sqlite",
    |_| MockConnection::new(),
    |_| Arc::new(MockRepository::new()),
);
std::env::set_var("PROVIDER", "sqlite");
std::env::set_var("DATASOURCE_URL", "sqlite:///:memory:");
let ds = default().expect("default");
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/DatasourceFactory.test.ts#L33-L39)

```typescript
DatasourceFactory.register("sqlite", () => new MockConnection(), (c) => new MockRepository(c));
process.env.PROVIDER = "sqlite";
process.env.DATASOURCE_URL = "sqlite:///:memory:";
const ds = DatasourceFactory.default;
```

#### Register and Create

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Memory.Tests/RegisterAndCreateTests.cs#L10-L16)

```csharp
DatasourceFactory.Register("memory", opts => new MemoryConnection(opts), MemoryConnection.Repository);
var ds = DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = "memory" });
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L36-L46)

```rust
register(
    "memory",
    MemoryConnection::from_options,
    MemoryConnection::repository,
);
let ds = create(Some(&HashMap::from([("provider".into(), "memory".into())]))).unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/memory/src/index.test.ts#L27-L32)

```typescript
DatasourceFactory.register("memory", MemoryConnection.from, MemoryConnection.repository);
const ds = DatasourceFactory.create({ provider: "memory" });
```

#### Missing provider

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/DatasourceFactoryTests.cs#L34-L40)

```csharp
DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = "missing" });
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/datasource_factory.rs#L35-L42)

```rust
create(Some(&HashMap::from([("provider".into(), "missing".into())])));
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/DatasourceFactory.test.ts#L41-L45)

```typescript
DatasourceFactory.create({ provider: "missing" });
```

## Datasource

One configured datasource: provider name plus factories for connections, commands, and repositories. `Default` / `default` / `Create` is on [DatasourceFactory](#datasourcefactory).

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
| `options` | [DatasourceOptions](#typescript-datasourceoptions); omitted uses `.env` then `DATASOURCE_URL` / `DATABASE_SCHEMA` |

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

### Remarks

`DatasourceFactory.Default` / `DatasourceFactory.default` / `default()` loads `.env` `PROVIDER` and `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. `Create(null)` / `create()` / `create(None)` uses the same path. A non-null options bag is used as-is; missing `provider` and `database_url` throw.

Listed factories are registered through `Datasources.Hosting`, `@datasources/register`, or `datasources-register` when that package is present.

`Reset` / `reset` clears the registry. Use it in tests.

See [DatasourceFactory](#datasourcefactory) for the factory contract, and [CLI](#command-line-cli) for `setup` and adding providers.

### Examples

#### Create from environment

##### C#

```csharp
var ds = DatasourceFactory.Default;
```

##### Rust

```rust
let ds = default()?;
```

##### TypeScript

```typescript
const ds = DatasourceFactory.default;
```

## Connection

Connection represents a connection to a datasource and handles opening and closing. A new command or transaction can be created from an existing connection.

### Classes

##### TypeScript

##### IConnection {#typescript-iconnection}

```typescript
interface IConnection
```

**Properties**

| Name | Description |
| --- | --- |
| [provider](#typescript-iconnection-provider) | Registered provider name. |
| [isOpen](#typescript-iconnection-isopen) | Whether the connection is open. |

**Methods**

| Name | Description |
| --- | --- |
| [createCommand()](#typescript-iconnection-createcommand) | Creates a command bound to this connection. |
| [open()](#typescript-iconnection-open) | Opens the connection. A no-op if already open. |
| [close()](#typescript-iconnection-close) | Closes the connection. |
| [startTransaction(isolation?)](#typescript-iconnection-starttransaction-isolation) | Begins a transaction. The connection must be open. |

###### provider {#typescript-iconnection-provider}

Registered provider name.

**Definition**

```typescript
readonly provider: string
```

###### isOpen {#typescript-iconnection-isopen}

Whether the connection is open.

**Definition**

```typescript
readonly isOpen: boolean
```

###### createCommand() {#typescript-iconnection-createcommand}

Creates a command bound to this connection.

**Definition**

```typescript
createCommand(): ICommand
```

**Returns**

[ICommand](#command)

###### open() {#typescript-iconnection-open}

Opens the connection. A no-op if already open.

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

###### startTransaction(isolation?) {#typescript-iconnection-starttransaction-isolation}

Begins a transaction. The connection must be open.

**Definition**

```typescript
startTransaction(isolation?: IsolationLevel): Promise<ITransaction>
```

**Parameters**

| Name | Description |
| --- | --- |
| `isolation` | [IsolationLevel](#typescript-isolationlevel); omitted uses serializable |

**Returns**

`Promise<[ITransaction](#transaction)>`

##### C#

##### IConnection {#csharp-iconnection}

```csharp
public interface IConnection
```

**Properties**

| Name | Description |
| --- | --- |
| [Provider](#csharp-iconnection-provider) | Registered provider name. |
| [IsOpen](#csharp-iconnection-isopen) | Whether the connection is open. |

**Methods**

| Name | Description |
| --- | --- |
| [CreateCommand()](#csharp-iconnection-createcommand) | Creates a command bound to this connection. |
| [Open()](#csharp-iconnection-open) | Opens the connection. A no-op if already open. |
| [Close()](#csharp-iconnection-close) | Closes the connection. |
| [StartTransaction(isolation)](#csharp-iconnection-starttransaction-isolation) | Begins a transaction. The connection must be open. |

###### Provider {#csharp-iconnection-provider}

Registered provider name.

**Definition**

```csharp
string Provider { get; }
```

###### IsOpen {#csharp-iconnection-isopen}

Whether the connection is open.

**Definition**

```csharp
bool IsOpen { get; }
```

###### CreateCommand() {#csharp-iconnection-createcommand}

Creates a command bound to this connection.

**Definition**

```csharp
ICommand CreateCommand()
```

**Returns**

[ICommand](#command)

###### Open() {#csharp-iconnection-open}

Opens the connection. A no-op if already open.

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

###### StartTransaction(isolation) {#csharp-iconnection-starttransaction-isolation}

Begins a transaction. The connection must be open.

**Definition**

```csharp
Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.Serializable)
```

**Parameters**

| Name | Description |
| --- | --- |
| `isolation` | [IsolationLevel](#csharp-isolationlevel); default is serializable |

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
| [open()](#rust-connection-open) | Opens the connection. A no-op if already open. |
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

Opens the connection. A no-op if already open.

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
| `isolation` | [IsolationLevel](#rust-isolationlevel); `None` uses serializable |

**Returns**

`Result<Arc<dyn `[Transaction](#transaction)`>>`

### Remarks

`CreateConnection` / `createConnection` / `create_connection` is async and returns a closed connection. The caller opens it. `Open` / `open` is a no-op if already open. Do not open during process bootstrap.

`StartTransaction` requires an open connection. Memory, JSON, and SQLite throw when the connection is closed.

Working dialects (`json`, `memory`, `sqlite`) open against injected ports (`IFileStore` / `FileStore`, `MemoryStore`, `ISqliteDriver` / `SqliteDriver`). Stub dialects throw not-implemented on open, close, and start-transaction.

### Examples

#### Create from Default

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/ConnectionTests.cs#L27-L33)

```csharp
var ds = DatasourceFactory.Default;
var connection = await ds.CreateConnection();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/connection.rs#L25-L31)

```rust
let ds = default().expect("default");
let connection = ds.create_connection().await.expect("connection");
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/Connection.test.ts#L31-L35)

```typescript
const ds = DatasourceFactory.default;
const connection = await ds.createConnection();
```

#### Open and close

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Memory.Tests/ConnectionLifecycleTests.cs#L7-L14)

```csharp
var connection = new MemoryConnection();
await connection.Open();
await connection.Close();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L48-L54)

```rust
let connection = MemoryConnection::from_options(None);
connection.open().await.unwrap();
connection.close().await.unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/memory/src/index.test.ts#L36-L41)

```typescript
const connection = new MemoryConnection();
await connection.open();
await connection.close();
```

#### Start a transaction

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Memory.Tests/ConnectionLifecycleTests.cs#L16-L23)

```csharp
var connection = new MemoryConnection();
await connection.Open();
var txn = await connection.StartTransaction();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L56-L62)

```rust
let connection = MemoryConnection::from_options(None);
connection.open().await.unwrap();
let txn = connection.start_transaction(None).await.unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/memory/src/index.test.ts#L43-L48)

```typescript
const connection = new MemoryConnection();
await connection.open();
const txn = await connection.startTransaction();
```

## Transaction

Transaction represents a unit of work on an open connection and handles commit and abort. IsolationLevel selects the isolation for a new transaction.

### Classes

##### TypeScript

##### ITransaction {#typescript-itransaction}

```typescript
interface ITransaction
```

**Properties**

| Name | Description |
| --- | --- |
| [connection](#typescript-itransaction-connection) | The open [IConnection](#connection) that started this transaction. |
| [isolation](#typescript-itransaction-isolation) | The isolation level for this transaction. |

**Methods**

| Name | Description |
| --- | --- |
| [commit()](#typescript-itransaction-commit) | Commits the transaction. |
| [abort()](#typescript-itransaction-abort) | Rolls the transaction back. |

###### connection {#typescript-itransaction-connection}

The open [IConnection](#connection) that started this transaction.

**Definition**

```typescript
readonly connection: IConnection
```

###### isolation {#typescript-itransaction-isolation}

The isolation level for this transaction.

**Definition**

```typescript
readonly isolation: IsolationLevel
```

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
| [ReadUncommitted](#typescript-isolationlevel-readuncommitted) | Dirty reads are allowed. |
| [ReadCommitted](#typescript-isolationlevel-readcommitted) | Committed data only. |
| [RepeatableRead](#typescript-isolationlevel-repeatableread) | Repeatable reads within the transaction. |
| [Serializable](#typescript-isolationlevel-serializable) | Full isolation. The default for [startTransaction](#typescript-iconnection-starttransaction-isolation). |

###### ReadUncommitted {#typescript-isolationlevel-readuncommitted}

Dirty reads are allowed.

**Definition**

```typescript
ReadUncommitted: "readUncommitted"
```

###### ReadCommitted {#typescript-isolationlevel-readcommitted}

Committed data only.

**Definition**

```typescript
ReadCommitted: "readCommitted"
```

###### RepeatableRead {#typescript-isolationlevel-repeatableread}

Repeatable reads within the transaction.

**Definition**

```typescript
RepeatableRead: "repeatableRead"
```

###### Serializable {#typescript-isolationlevel-serializable}

Full isolation. The default for [startTransaction](#typescript-iconnection-starttransaction-isolation).

**Definition**

```typescript
Serializable: "serializable"
```

##### C#

##### ITransaction {#csharp-itransaction}

```csharp
public interface ITransaction
```

**Properties**

| Name | Description |
| --- | --- |
| [Connection](#csharp-itransaction-connection) | The open [IConnection](#connection) that started this transaction. |
| [Isolation](#csharp-itransaction-isolation) | The isolation level for this transaction. |

**Methods**

| Name | Description |
| --- | --- |
| [Commit()](#csharp-itransaction-commit) | Commits the transaction. |
| [Abort()](#csharp-itransaction-abort) | Rolls the transaction back. |

###### Connection {#csharp-itransaction-connection}

The open [IConnection](#connection) that started this transaction.

**Definition**

```csharp
IConnection Connection { get; }
```

###### Isolation {#csharp-itransaction-isolation}

The isolation level for this transaction.

**Definition**

```csharp
IsolationLevel Isolation { get; }
```

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
| [ReadUncommitted](#csharp-isolationlevel-readuncommitted) | Dirty reads are allowed. |
| [ReadCommitted](#csharp-isolationlevel-readcommitted) | Committed data only. |
| [RepeatableRead](#csharp-isolationlevel-repeatableread) | Repeatable reads within the transaction. |
| [Serializable](#csharp-isolationlevel-serializable) | Full isolation. The default for [StartTransaction](#csharp-iconnection-starttransaction-isolation). |

###### ReadUncommitted {#csharp-isolationlevel-readuncommitted}

Dirty reads are allowed.

**Definition**

```csharp
ReadUncommitted
```

###### ReadCommitted {#csharp-isolationlevel-readcommitted}

Committed data only.

**Definition**

```csharp
ReadCommitted
```

###### RepeatableRead {#csharp-isolationlevel-repeatableread}

Repeatable reads within the transaction.

**Definition**

```csharp
RepeatableRead
```

###### Serializable {#csharp-isolationlevel-serializable}

Full isolation. The default for [StartTransaction](#csharp-iconnection-starttransaction-isolation).

**Definition**

```csharp
Serializable
```

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
| [ReadUncommitted](#rust-isolationlevel-readuncommitted) | Dirty reads are allowed. |
| [ReadCommitted](#rust-isolationlevel-readcommitted) | Committed data only. |
| [RepeatableRead](#rust-isolationlevel-repeatableread) | Repeatable reads within the transaction. |
| [Serializable](#rust-isolationlevel-serializable) | Full isolation. The default for [start_transaction](#rust-connection-start_transaction-isolation). |

###### ReadUncommitted {#rust-isolationlevel-readuncommitted}

Dirty reads are allowed.

**Definition**

```rust
ReadUncommitted
```

###### ReadCommitted {#rust-isolationlevel-readcommitted}

Committed data only.

**Definition**

```rust
ReadCommitted
```

###### RepeatableRead {#rust-isolationlevel-repeatableread}

Repeatable reads within the transaction.

**Definition**

```rust
RepeatableRead
```

###### Serializable {#rust-isolationlevel-serializable}

Full isolation. The default for [start_transaction](#rust-connection-start_transaction-isolation).

**Definition**

```rust
Serializable
```

### Remarks

`StartTransaction` / `startTransaction` / `start_transaction` requires an open [connection](#connection). It returns a transaction that is not committed.

`Commit` / `commit` persists work. `Abort` / `abort` rolls it back.

Omitted isolation is `Serializable` (TypeScript omits the argument, C# defaults, Rust uses `None`). Memory, JSON, and SQLite throw when the connection is closed.

### Examples

#### Start a transaction

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/TransactionTests.cs#L27-L35)

```csharp
var ds = DatasourceFactory.Default;
var connection = await ds.CreateConnection();
await connection.Open();
var txn = await connection.StartTransaction();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/transaction.rs#L25-L33)

```rust
let ds = default().expect("default");
let connection = ds.create_connection().await.expect("connection");
connection.open().await.expect("open");
let txn = connection.start_transaction(None).await.expect("txn");
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/Transaction.test.ts#L31-L37)

```typescript
const ds = DatasourceFactory.default;
const connection = await ds.createConnection();
await connection.open();
const txn = await connection.startTransaction();
```

#### Commit

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Memory.Tests/TransactionLifecycleTests.cs#L7-L15)

```csharp
var connection = new MemoryConnection();
await connection.Open();
var txn = await connection.StartTransaction();
await txn.Commit();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L64-L71)

```rust
let connection = MemoryConnection::from_options(None);
connection.open().await.unwrap();
let txn = connection.start_transaction(None).await.unwrap();
txn.commit().await.unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/memory/src/index.test.ts#L52-L58)

```typescript
const connection = new MemoryConnection();
await connection.open();
const txn = await connection.startTransaction();
await txn.commit();
```

#### Abort

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Memory.Tests/TransactionLifecycleTests.cs#L17-L25)

```csharp
var connection = new MemoryConnection();
await connection.Open();
var txn = await connection.StartTransaction();
await txn.Abort();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L73-L80)

```rust
let connection = MemoryConnection::from_options(None);
connection.open().await.unwrap();
let txn = connection.start_transaction(None).await.unwrap();
txn.abort().await.unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/memory/src/index.test.ts#L60-L66)

```typescript
const connection = new MemoryConnection();
await connection.open();
const txn = await connection.startTransaction();
await txn.abort();
```

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

Bidirectional `Map` / `ReverseMap`. Column names pass through unless a mapper rewrites them. Repository get is convert, then map.

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

##### ISourceDestinationMapper {#typescript-isourcedestinationmapper}

```typescript
interface ISourceDestinationMapper
```

**Properties**

| Name | Description |
| --- | --- |
| [source](#typescript-isourcedestinationmapper-source) | Dialect or provider label for the source side. |
| [destination](#typescript-isourcedestinationmapper-destination) | Dialect or provider label for the destination side. |

###### source {#typescript-isourcedestinationmapper-source}

Dialect or provider label for the source side.

**Definition**

```typescript
readonly source: string
```

###### destination {#typescript-isourcedestinationmapper-destination}

Dialect or provider label for the destination side.

**Definition**

```typescript
readonly destination: string
```

##### NameMapper {#typescript-namemapper}

```typescript
abstract class NameMapper implements IMapper<string, string>
```

**Methods**

| Name | Description |
| --- | --- |
| [map(name)](#typescript-namemapper-map-name) | Returns the name unchanged. |
| [reverseMap(name)](#typescript-namemapper-reversemap-name) | Returns the name unchanged. |

###### map(name) {#typescript-namemapper-map-name}

Returns the name unchanged.

**Definition**

```typescript
map(name: string): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | column or entity name |

**Returns**

`string`

###### reverseMap(name) {#typescript-namemapper-reversemap-name}

Returns the name unchanged.

**Definition**

```typescript
reverseMap(name: string): string
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | column or entity name |

**Returns**

`string`

##### NameSourceDestinationMapper {#typescript-namesourcedestinationmapper}

```typescript
class NameSourceDestinationMapper extends NameMapper implements ISourceDestinationMapper
```

**Constructor**

| Name | Description |
| --- | --- |
| [NameSourceDestinationMapper(source, destination)](#typescript-namesourcedestinationmapper-namesourcedestinationmapper-source-destination) | Holds source and destination labels. Names still pass through. |

**Properties**

| Name | Description |
| --- | --- |
| [source](#typescript-namesourcedestinationmapper-source) | Dialect or provider label for the source side. |
| [destination](#typescript-namesourcedestinationmapper-destination) | Dialect or provider label for the destination side. |

###### NameSourceDestinationMapper(source, destination) {#typescript-namesourcedestinationmapper-namesourcedestinationmapper-source-destination}

Holds source and destination labels. Names still pass through.

**Definition**

```typescript
constructor(source: string, destination: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `source` | source label |
| `destination` | destination label |

###### source {#typescript-namesourcedestinationmapper-source}

Dialect or provider label for the source side.

**Definition**

```typescript
readonly source: string
```

###### destination {#typescript-namesourcedestinationmapper-destination}

Dialect or provider label for the destination side.

**Definition**

```typescript
readonly destination: string
```

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

##### IFlatToTypeMapper {#typescript-iflattotypemapper}

```typescript
interface IFlatToTypeMapper<T> extends IMapper<Cells, T>
```

**Properties**

| Name | Description |
| --- | --- |
| [spec](#typescript-iflattotypemapper-spec) | The [TypeMap](#typescript-typemap) used to collapse join rows. |

**Methods**

| Name | Description |
| --- | --- |
| [collapse(rows)](#typescript-iflattotypemapper-collapse-rows) | Collapses denormalized join rows into unique parents with unique children. |

###### spec {#typescript-iflattotypemapper-spec}

The [TypeMap](#typescript-typemap) used to collapse join rows.

**Definition**

```typescript
readonly spec: TypeMap<T>
```

###### collapse(rows) {#typescript-iflattotypemapper-collapse-rows}

Collapses denormalized join rows into unique parents with unique children.

**Definition**

```typescript
collapse(rows: Iterable<T | Cells>): T[]
```

**Parameters**

| Name | Description |
| --- | --- |
| `rows` | raw `Cells` or already-mapped `T` |

**Returns**

`T[]`

##### FlatToTypeMapper {#typescript-flattotypemapper}

```typescript
class FlatToTypeMapper<T> implements IFlatToTypeMapper<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [FlatToTypeMapper(spec)](#typescript-flattotypemapper-flattotypemapper-spec) | Builds a collapse mapper from a [TypeMap](#typescript-typemap). |

**Properties**

| Name | Description |
| --- | --- |
| [spec](#typescript-flattotypemapper-spec) | The [TypeMap](#typescript-typemap) used to collapse join rows. |

**Methods**

| Name | Description |
| --- | --- |
| [map(row)](#typescript-flattotypemapper-map-row) | Maps one denormalized row to `T`. Nested group fields on a collection are omitted. |
| [reverseMap(value)](#typescript-flattotypemapper-reversemap-value) | Flattens `T` back to one row. Groups keep the first child. |
| [collapse(rows)](#typescript-flattotypemapper-collapse-rows) | Collapses denormalized join rows into unique parents with unique children. |

###### FlatToTypeMapper(spec) {#typescript-flattotypemapper-flattotypemapper-spec}

Builds a collapse mapper from a [TypeMap](#typescript-typemap).

**Definition**

```typescript
constructor(spec: TypeMap<T>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `spec` | [TypeMap](#typescript-typemap); scalars map column to field, groups collect unique children by key |

###### spec {#typescript-flattotypemapper-spec}

The [TypeMap](#typescript-typemap) used to collapse join rows.

**Definition**

```typescript
readonly spec: TypeMap<T>
```

###### map(row) {#typescript-flattotypemapper-map-row}

Maps one denormalized row to `T`. Nested group fields on a collection are omitted.

**Definition**

```typescript
map(row: Cells): T
```

**Parameters**

| Name | Description |
| --- | --- |
| `row` | denormalized join row |

**Returns**

`T`

###### reverseMap(value) {#typescript-flattotypemapper-reversemap-value}

Flattens `T` back to one row. Groups keep the first child.

**Definition**

```typescript
reverseMap(value: T): Cells
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | mapped object |

**Returns**

`Cells`

###### collapse(rows) {#typescript-flattotypemapper-collapse-rows}

Collapses denormalized join rows into unique parents with unique children.

**Definition**

```typescript
collapse(rows: Iterable<T | Cells>): T[]
```

**Parameters**

| Name | Description |
| --- | --- |
| `rows` | raw `Cells` or already-mapped `T` |

**Returns**

`T[]`

##### TypeMap {#typescript-typemap}

```typescript
type TypeMap<T>
```

**Properties**

| Name | Description |
| --- | --- |
| [key](#typescript-typemap-key) | Parent key column. |
| [fields](#typescript-typemap-fields) | Column or group per destination field. |

###### key {#typescript-typemap-key}

Parent key column.

**Definition**

```typescript
key: string
```

###### fields {#typescript-typemap-fields}

Column or group per destination field.

**Definition**

```typescript
fields: TypeMapFields<T>
```

##### TypeMapGroup {#typescript-typemapgroup}

```typescript
type TypeMapGroup<E>
```

**Properties**

| Name | Description |
| --- | --- |
| [key](#typescript-typemapgroup-key) | Child key column used to unique children. |
| [fields](#typescript-typemapgroup-fields) | Column names for the child. |

###### key {#typescript-typemapgroup-key}

Child key column used to unique children.

**Definition**

```typescript
key: string
```

###### fields {#typescript-typemapgroup-fields}

Column names for the child.

**Definition**

```typescript
fields: TypeMapFields<E>
```

##### TypeMapFields {#typescript-typemapfields}

```typescript
type TypeMapFields<T>
```

##### StatementMapper {#typescript-statementmapper}

```typescript
class StatementMapper<T> implements IMapper<T, T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [StatementMapper(options?)](#typescript-statementmapper-statementmapper-options) | Builds a statement mapper. Omitted fields default to identity name mappers and an identity [rows](#typescript-statementmapper-rows) mapper. |

**Properties**

| Name | Description |
| --- | --- |
| [entity](#typescript-statementmapper-entity) | Maps the table or entity name when the repository is built. |
| [columns](#typescript-statementmapper-columns) | Maps column names when the repository is built. |
| [query](#typescript-statementmapper-query) | Rewrites a finished query. |
| [insert](#typescript-statementmapper-insert) | Rewrites a finished insert. |
| [update](#typescript-statementmapper-update) | Rewrites a finished update. |
| [delete](#typescript-statementmapper-delete) | Rewrites a finished delete. |
| [storedProcedure](#typescript-statementmapper-storedprocedure) | Rewrites a finished stored-procedure statement. |
| [rows](#typescript-statementmapper-rows) | Get-row [IMapper](#typescript-imapper). [map](#typescript-statementmapper-map-value) / [reverseMap](#typescript-statementmapper-reversemap-value) delegate here. |

**Methods**

| Name | Description |
| --- | --- |
| [map(value)](#typescript-statementmapper-map-value) | Maps a row through [rows](#typescript-statementmapper-rows). |
| [reverseMap(value)](#typescript-statementmapper-reversemap-value) | Reverse-maps a row through [rows](#typescript-statementmapper-rows). |

###### StatementMapper(options?) {#typescript-statementmapper-statementmapper-options}

Builds a statement mapper. Omitted fields default to identity name mappers and an identity [rows](#typescript-statementmapper-rows) mapper.

**Definition**

```typescript
constructor(options?: StatementMapperOptions<T>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `options` | [StatementMapperOptions](#typescript-statementmapperoptions) |

###### entity {#typescript-statementmapper-entity}

Maps the table or entity name when the repository is built.

**Definition**

```typescript
readonly entity: IMapper<string, string>
```

###### columns {#typescript-statementmapper-columns}

Maps column names when the repository is built.

**Definition**

```typescript
readonly columns: IMapper<string, string>
```

###### query {#typescript-statementmapper-query}

Rewrites a finished query.

**Definition**

```typescript
readonly query?: IMapper<IQuery, IQuery>
```

###### insert {#typescript-statementmapper-insert}

Rewrites a finished insert.

**Definition**

```typescript
readonly insert?: IMapper<IInsertStatement, IInsertStatement>
```

###### update {#typescript-statementmapper-update}

Rewrites a finished update.

**Definition**

```typescript
readonly update?: IMapper<IUpdateStatement, IUpdateStatement>
```

###### delete {#typescript-statementmapper-delete}

Rewrites a finished delete.

**Definition**

```typescript
readonly delete?: IMapper<IDeleteStatement, IDeleteStatement>
```

###### storedProcedure {#typescript-statementmapper-storedprocedure}

Rewrites a finished stored-procedure statement.

**Definition**

```typescript
readonly storedProcedure?: IMapper<IStoredProcedureStatement, IStoredProcedureStatement>
```

###### rows {#typescript-statementmapper-rows}

Get-row [IMapper](#typescript-imapper). [map](#typescript-statementmapper-map-value) / [reverseMap](#typescript-statementmapper-reversemap-value) delegate here.

**Definition**

```typescript
readonly rows: IMapper<T, T>
```

###### map(value) {#typescript-statementmapper-map-value}

Maps a row through [rows](#typescript-statementmapper-rows).

**Definition**

```typescript
map(value: T): T
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | converted row |

**Returns**

`T`

###### reverseMap(value) {#typescript-statementmapper-reversemap-value}

Reverse-maps a row through [rows](#typescript-statementmapper-rows).

**Definition**

```typescript
reverseMap(value: T): T
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | shaped row |

**Returns**

`T`

##### StatementMapperOptions {#typescript-statementmapperoptions}

```typescript
type StatementMapperOptions<T>
```

**Properties**

| Name | Description |
| --- | --- |
| [entity](#typescript-statementmapperoptions-entity) | Maps the table or entity name. |
| [columns](#typescript-statementmapperoptions-columns) | Maps column names. |
| [rows](#typescript-statementmapperoptions-rows) | Get-row [IMapper](#typescript-imapper). |
| [query](#typescript-statementmapperoptions-query) | Rewrites a finished query. |
| [insert](#typescript-statementmapperoptions-insert) | Rewrites a finished insert. |
| [update](#typescript-statementmapperoptions-update) | Rewrites a finished update. |
| [delete](#typescript-statementmapperoptions-delete) | Rewrites a finished delete. |
| [storedProcedure](#typescript-statementmapperoptions-storedprocedure) | Rewrites a finished stored-procedure statement. |

###### entity {#typescript-statementmapperoptions-entity}

Maps the table or entity name.

**Definition**

```typescript
entity?: IMapper<string, string>
```

###### columns {#typescript-statementmapperoptions-columns}

Maps column names.

**Definition**

```typescript
columns?: IMapper<string, string>
```

###### rows {#typescript-statementmapperoptions-rows}

Get-row [IMapper](#typescript-imapper).

**Definition**

```typescript
rows?: IMapper<T, T>
```

###### query {#typescript-statementmapperoptions-query}

Rewrites a finished query.

**Definition**

```typescript
query?: IMapper<IQuery, IQuery>
```

###### insert {#typescript-statementmapperoptions-insert}

Rewrites a finished insert.

**Definition**

```typescript
insert?: IMapper<IInsertStatement, IInsertStatement>
```

###### update {#typescript-statementmapperoptions-update}

Rewrites a finished update.

**Definition**

```typescript
update?: IMapper<IUpdateStatement, IUpdateStatement>
```

###### delete {#typescript-statementmapperoptions-delete}

Rewrites a finished delete.

**Definition**

```typescript
delete?: IMapper<IDeleteStatement, IDeleteStatement>
```

###### storedProcedure {#typescript-statementmapperoptions-storedprocedure}

Rewrites a finished stored-procedure statement.

**Definition**

```typescript
storedProcedure?: IMapper<IStoredProcedureStatement, IStoredProcedureStatement>
```

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

##### ISourceDestinationMapper {#csharp-isourcedestinationmapper}

```csharp
public interface ISourceDestinationMapper
```

**Properties**

| Name | Description |
| --- | --- |
| [Source](#csharp-isourcedestinationmapper-source) | Dialect or provider label for the source side. |
| [Destination](#csharp-isourcedestinationmapper-destination) | Dialect or provider label for the destination side. |

###### Source {#csharp-isourcedestinationmapper-source}

Dialect or provider label for the source side.

**Definition**

```csharp
string Source { get; }
```

###### Destination {#csharp-isourcedestinationmapper-destination}

Dialect or provider label for the destination side.

**Definition**

```csharp
string Destination { get; }
```

##### NameMapper {#csharp-namemapper}

```csharp
public abstract class NameMapper : IMapper<string, string>
```

**Methods**

| Name | Description |
| --- | --- |
| [Map(name)](#csharp-namemapper-map-name) | Returns the name unchanged. |
| [ReverseMap(name)](#csharp-namemapper-reversemap-name) | Returns the name unchanged. |

###### Map(name) {#csharp-namemapper-map-name}

Returns the name unchanged.

**Definition**

```csharp
string Map(string name)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | column or entity name |

**Returns**

`string`

###### ReverseMap(name) {#csharp-namemapper-reversemap-name}

Returns the name unchanged.

**Definition**

```csharp
string ReverseMap(string name)
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | column or entity name |

**Returns**

`string`

##### NameSourceDestinationMapper {#csharp-namesourcedestinationmapper}

```csharp
public sealed class NameSourceDestinationMapper : NameMapper, ISourceDestinationMapper
```

**Constructor**

| Name | Description |
| --- | --- |
| [NameSourceDestinationMapper(source, destination)](#csharp-namesourcedestinationmapper-namesourcedestinationmapper-source-destination) | Holds source and destination labels. Names still pass through. |

**Properties**

| Name | Description |
| --- | --- |
| [Source](#csharp-namesourcedestinationmapper-source) | Dialect or provider label for the source side. |
| [Destination](#csharp-namesourcedestinationmapper-destination) | Dialect or provider label for the destination side. |

###### NameSourceDestinationMapper(source, destination) {#csharp-namesourcedestinationmapper-namesourcedestinationmapper-source-destination}

Holds source and destination labels. Names still pass through.

**Definition**

```csharp
public NameSourceDestinationMapper(string source, string destination)
```

**Parameters**

| Name | Description |
| --- | --- |
| `source` | source label |
| `destination` | destination label |

###### Source {#csharp-namesourcedestinationmapper-source}

Dialect or provider label for the source side.

**Definition**

```csharp
string Source { get; }
```

###### Destination {#csharp-namesourcedestinationmapper-destination}

Dialect or provider label for the destination side.

**Definition**

```csharp
string Destination { get; }
```

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

##### IFlatToTypeMapper {#csharp-iflattotypemapper}

```csharp
public interface IFlatToTypeMapper : IMapper<Row, Dictionary<string, MappedValue>>
```

**Properties**

| Name | Description |
| --- | --- |
| [Spec](#csharp-iflattotypemapper-spec) | The [TypeMap](#csharp-typemap) used to collapse join rows. |

**Methods**

| Name | Description |
| --- | --- |
| [Collapse(rows)](#csharp-iflattotypemapper-collapse-rows) | Maps and collapses denormalized join rows. |
| [Collapse(mapped)](#csharp-iflattotypemapper-collapse-mapped) | Collapses already-mapped rows. |

###### Spec {#csharp-iflattotypemapper-spec}

The [TypeMap](#csharp-typemap) used to collapse join rows.

**Definition**

```csharp
TypeMap Spec { get; }
```

###### Collapse(rows) {#csharp-iflattotypemapper-collapse-rows}

Maps and collapses denormalized join rows.

**Definition**

```csharp
IReadOnlyList<Dictionary<string, MappedValue>> Collapse(IEnumerable<Row> rows)
```

**Parameters**

| Name | Description |
| --- | --- |
| `rows` | denormalized join rows |

**Returns**

`IReadOnlyList<Dictionary<string, `[MappedValue](#csharp-mappedvalue)`>>`

###### Collapse(mapped) {#csharp-iflattotypemapper-collapse-mapped}

Collapses already-mapped rows.

**Definition**

```csharp
IReadOnlyList<Dictionary<string, MappedValue>> Collapse(IEnumerable<Dictionary<string, MappedValue>> rows)
```

**Parameters**

| Name | Description |
| --- | --- |
| `mapped` | rows already passed through [Map](#csharp-flattotypemapper-map-value) |

**Returns**

`IReadOnlyList<Dictionary<string, `[MappedValue](#csharp-mappedvalue)`>>`

##### FlatToTypeMapper {#csharp-flattotypemapper}

```csharp
public sealed class FlatToTypeMapper : IFlatToTypeMapper
```

**Constructor**

| Name | Description |
| --- | --- |
| [FlatToTypeMapper(spec)](#csharp-flattotypemapper-flattotypemapper-spec) | Builds a collapse mapper from a [TypeMap](#csharp-typemap). |

**Properties**

| Name | Description |
| --- | --- |
| [Spec](#csharp-flattotypemapper-spec) | The [TypeMap](#csharp-typemap) used to collapse join rows. |

**Methods**

| Name | Description |
| --- | --- |
| [Map(value)](#csharp-flattotypemapper-map-value) | Maps one denormalized row. Nested group fields on a collection are omitted. |
| [ReverseMap(value)](#csharp-flattotypemapper-reversemap-value) | Flattens a mapped row back. Groups keep the first child. |
| [Collapse(rows)](#csharp-flattotypemapper-collapse-rows) | Maps and collapses denormalized join rows. |
| [Collapse(mapped)](#csharp-flattotypemapper-collapse-mapped) | Collapses already-mapped rows. |

###### FlatToTypeMapper(spec) {#csharp-flattotypemapper-flattotypemapper-spec}

Builds a collapse mapper from a [TypeMap](#csharp-typemap).

**Definition**

```csharp
public FlatToTypeMapper(TypeMap spec)
```

**Parameters**

| Name | Description |
| --- | --- |
| `spec` | [TypeMap](#csharp-typemap); scalars map column to field, groups collect unique children by key |

###### Spec {#csharp-flattotypemapper-spec}

The [TypeMap](#csharp-typemap) used to collapse join rows.

**Definition**

```csharp
TypeMap Spec { get; }
```

###### Map(value) {#csharp-flattotypemapper-map-value}

Maps one denormalized row. Nested group fields on a collection are omitted.

**Definition**

```csharp
Dictionary<string, MappedValue> Map(Row value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | denormalized join row |

**Returns**

`Dictionary<string, `[MappedValue](#csharp-mappedvalue)`>`

###### ReverseMap(value) {#csharp-flattotypemapper-reversemap-value}

Flattens a mapped row back. Groups keep the first child.

**Definition**

```csharp
Row ReverseMap(Dictionary<string, MappedValue> value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | mapped graph |

**Returns**

`Row`

###### Collapse(rows) {#csharp-flattotypemapper-collapse-rows}

Maps and collapses denormalized join rows.

**Definition**

```csharp
IReadOnlyList<Dictionary<string, MappedValue>> Collapse(IEnumerable<Row> rows)
```

**Parameters**

| Name | Description |
| --- | --- |
| `rows` | denormalized join rows |

**Returns**

`IReadOnlyList<Dictionary<string, `[MappedValue](#csharp-mappedvalue)`>>`

###### Collapse(mapped) {#csharp-flattotypemapper-collapse-mapped}

Collapses already-mapped rows.

**Definition**

```csharp
IReadOnlyList<Dictionary<string, MappedValue>> Collapse(IEnumerable<Dictionary<string, MappedValue>> rows)
```

**Parameters**

| Name | Description |
| --- | --- |
| `mapped` | rows already passed through [Map](#csharp-flattotypemapper-map-value) |

**Returns**

`IReadOnlyList<Dictionary<string, `[MappedValue](#csharp-mappedvalue)`>>`

##### TypeMap {#csharp-typemap}

```csharp
public sealed class TypeMap
```

**Properties**

| Name | Description |
| --- | --- |
| [Key](#csharp-typemap-key) | Parent key column. |
| [Fields](#csharp-typemap-fields) | Column or group per destination field. |

###### Key {#csharp-typemap-key}

Parent key column.

**Definition**

```csharp
required string Key { get; init; }
```

###### Fields {#csharp-typemap-fields}

Column or group per destination field.

**Definition**

```csharp
required IReadOnlyDictionary<string, TypeMapField> Fields { get; init; }
```

##### TypeMapGroup {#csharp-typemapgroup}

```csharp
public sealed class TypeMapGroup
```

**Properties**

| Name | Description |
| --- | --- |
| [Key](#csharp-typemapgroup-key) | Child key column used to unique children. |
| [Fields](#csharp-typemapgroup-fields) | Column or group per child field. |

###### Key {#csharp-typemapgroup-key}

Child key column used to unique children.

**Definition**

```csharp
required string Key { get; init; }
```

###### Fields {#csharp-typemapgroup-fields}

Column or group per child field.

**Definition**

```csharp
required IReadOnlyDictionary<string, TypeMapField> Fields { get; init; }
```

##### TypeMapField {#csharp-typemapfield}

```csharp
public abstract record TypeMapField
```

**Properties**

| Name | Description |
| --- | --- |
| [Column](#csharp-typemapfield-column) | Maps a destination field to a source column. |
| [Group](#csharp-typemapfield-group) | Maps a destination field to a child [TypeMapGroup](#csharp-typemapgroup). |

###### Column {#csharp-typemapfield-column}

Maps a destination field to a source column.

**Definition**

```csharp
TypeMapField.Column(string Source)
```

###### Group {#csharp-typemapfield-group}

Maps a destination field to a child [TypeMapGroup](#csharp-typemapgroup).

**Definition**

```csharp
TypeMapField.Group(TypeMapGroup Map)
```

##### MappedValue {#csharp-mappedvalue}

```csharp
public abstract record MappedValue
```

**Properties**

| Name | Description |
| --- | --- |
| [Cell](#csharp-mappedvalue-cell) | A scalar cell. |
| [Many](#csharp-mappedvalue-many) | A child collection. |

###### Cell {#csharp-mappedvalue-cell}

A scalar cell.

**Definition**

```csharp
MappedValue.Cell(Value Value)
```

###### Many {#csharp-mappedvalue-many}

A child collection.

**Definition**

```csharp
MappedValue.Many(List<Dictionary<string, MappedValue>> Items)
```

##### StatementMapper {#csharp-statementmapper}

```csharp
public sealed class StatementMapper<T> : IMapper<T, T> where T : new()
```

**Properties**

| Name | Description |
| --- | --- |
| [Entity](#csharp-statementmapper-entity) | Maps the table or entity name when the repository is built. |
| [Columns](#csharp-statementmapper-columns) | Maps column names when the repository is built. |
| [Query](#csharp-statementmapper-query) | Rewrites a finished query. |
| [Insert](#csharp-statementmapper-insert) | Rewrites a finished insert. |
| [Update](#csharp-statementmapper-update) | Rewrites a finished update. |
| [Delete](#csharp-statementmapper-delete) | Rewrites a finished delete. |
| [StoredProcedure](#csharp-statementmapper-storedprocedure) | Rewrites a finished stored-procedure statement. |
| [Rows](#csharp-statementmapper-rows) | Get-row [IMapper](#csharp-imapper). [Map](#csharp-statementmapper-map-value) / [ReverseMap](#csharp-statementmapper-reversemap-value) delegate here. |

**Methods**

| Name | Description |
| --- | --- |
| [Map(value)](#csharp-statementmapper-map-value) | Maps a row through [Rows](#csharp-statementmapper-rows). |
| [ReverseMap(value)](#csharp-statementmapper-reversemap-value) | Reverse-maps a row through [Rows](#csharp-statementmapper-rows). |

###### Entity {#csharp-statementmapper-entity}

Maps the table or entity name when the repository is built.

**Definition**

```csharp
IMapper<string, string> Entity { get; init; }
```

###### Columns {#csharp-statementmapper-columns}

Maps column names when the repository is built.

**Definition**

```csharp
IMapper<string, string> Columns { get; init; }
```

###### Query {#csharp-statementmapper-query}

Rewrites a finished query.

**Definition**

```csharp
IMapper<IQuery, IQuery>? Query { get; init; }
```

###### Insert {#csharp-statementmapper-insert}

Rewrites a finished insert.

**Definition**

```csharp
IMapper<IInsertStatement, IInsertStatement>? Insert { get; init; }
```

###### Update {#csharp-statementmapper-update}

Rewrites a finished update.

**Definition**

```csharp
IMapper<IUpdateStatement, IUpdateStatement>? Update { get; init; }
```

###### Delete {#csharp-statementmapper-delete}

Rewrites a finished delete.

**Definition**

```csharp
IMapper<IDeleteStatement, IDeleteStatement>? Delete { get; init; }
```

###### StoredProcedure {#csharp-statementmapper-storedprocedure}

Rewrites a finished stored-procedure statement.

**Definition**

```csharp
IMapper<IStoredProcedureStatement, IStoredProcedureStatement>? StoredProcedure { get; init; }
```

###### Rows {#csharp-statementmapper-rows}

Get-row [IMapper](#csharp-imapper). [Map](#csharp-statementmapper-map-value) / [ReverseMap](#csharp-statementmapper-reversemap-value) delegate here.

**Definition**

```csharp
IMapper<T, T> Rows { get; init; }
```

###### Map(value) {#csharp-statementmapper-map-value}

Maps a row through [Rows](#csharp-statementmapper-rows).

**Definition**

```csharp
T Map(T value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | converted row |

**Returns**

`T`

###### ReverseMap(value) {#csharp-statementmapper-reversemap-value}

Reverse-maps a row through [Rows](#csharp-statementmapper-rows).

**Definition**

```csharp
T ReverseMap(T value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | shaped row |

**Returns**

`T`

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

##### SourceDestinationMapper {#rust-sourcedestinationmapper}

```rust
pub trait SourceDestinationMapper
```

**Methods**

| Name | Description |
| --- | --- |
| [source()](#rust-sourcedestinationmapper-source) | Dialect or provider label for the source side. |
| [destination()](#rust-sourcedestinationmapper-destination) | Dialect or provider label for the destination side. |

###### source() {#rust-sourcedestinationmapper-source}

Dialect or provider label for the source side.

**Definition**

```rust
fn source(&self) -> &str
```

**Returns**

`&str`

###### destination() {#rust-sourcedestinationmapper-destination}

Dialect or provider label for the destination side.

**Definition**

```rust
fn destination(&self) -> &str
```

**Returns**

`&str`

##### NameMapper {#rust-namemapper}

```rust
pub trait NameMapper
```

**Methods**

| Name | Description |
| --- | --- |
| [map(name)](#rust-namemapper-map-name) | Returns the name unchanged. |
| [reverse_map(name)](#rust-namemapper-reverse_map-name) | Returns the name unchanged. |

###### map(name) {#rust-namemapper-map-name}

Returns the name unchanged.

**Definition**

```rust
fn map(&self, name: &str) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | column or entity name |

**Returns**

`String`

###### reverse_map(name) {#rust-namemapper-reverse_map-name}

Returns the name unchanged.

**Definition**

```rust
fn reverse_map(&self, name: &str) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | column or entity name |

**Returns**

`String`

##### NameSourceDestinationMapper {#rust-namesourcedestinationmapper}

```rust
pub struct NameSourceDestinationMapper
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(source, destination)](#rust-namesourcedestinationmapper-new-source-destination) | Holds source and destination labels. Names still pass through. |

**Properties**

| Name | Description |
| --- | --- |
| [source](#rust-namesourcedestinationmapper-source) | Dialect or provider label for the source side. |
| [destination](#rust-namesourcedestinationmapper-destination) | Dialect or provider label for the destination side. |

**Methods**

| Name | Description |
| --- | --- |
| [map(name)](#rust-namesourcedestinationmapper-map-name) | Returns the name unchanged. |
| [reverse_map(name)](#rust-namesourcedestinationmapper-reverse_map-name) | Returns the name unchanged. |

###### new(source, destination) {#rust-namesourcedestinationmapper-new-source-destination}

Holds source and destination labels. Names still pass through.

**Definition**

```rust
pub fn new(source: impl Into<String>, destination: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `source` | source label |
| `destination` | destination label |

**Returns**

[NameSourceDestinationMapper](#rust-namesourcedestinationmapper)

###### source {#rust-namesourcedestinationmapper-source}

Dialect or provider label for the source side.

**Definition**

```rust
pub source: String
```

###### destination {#rust-namesourcedestinationmapper-destination}

Dialect or provider label for the destination side.

**Definition**

```rust
pub destination: String
```

###### map(name) {#rust-namesourcedestinationmapper-map-name}

Returns the name unchanged.

**Definition**

```rust
pub fn map(&self, name: &str) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | column or entity name |

**Returns**

`String`

###### reverse_map(name) {#rust-namesourcedestinationmapper-reverse_map-name}

Returns the name unchanged.

**Definition**

```rust
pub fn reverse_map(&self, name: &str) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `name` | column or entity name |

**Returns**

`String`

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

##### FlatToTypeMapperTrait {#rust-flattotypemappertrait}

```rust
pub trait FlatToTypeMapper: Mapper<Row, MappedRow>
```

**Methods**

| Name | Description |
| --- | --- |
| [spec()](#rust-flattotypemappertrait-spec) | The [TypeMap](#rust-typemap) used to collapse join rows. |
| [collapse(rows)](#rust-flattotypemappertrait-collapse-rows) | Collapses already-mapped rows. |
| [collapse_rows(rows)](#rust-flattotypemappertrait-collapse_rows-rows) | Maps and collapses denormalized join rows. |

###### spec() {#rust-flattotypemappertrait-spec}

The [TypeMap](#rust-typemap) used to collapse join rows.

**Definition**

```rust
fn spec(&self) -> &TypeMap
```

**Returns**

`&`[TypeMap](#rust-typemap)

###### collapse(rows) {#rust-flattotypemappertrait-collapse-rows}

Collapses already-mapped rows.

**Definition**

```rust
fn collapse(&self, rows: Vec<MappedRow>) -> Vec<MappedRow>
```

**Parameters**

| Name | Description |
| --- | --- |
| `rows` | rows already passed through [map](#rust-flattotypemapper-map-value) |

**Returns**

`Vec<`[MappedRow](#rust-mappedrow)`>`

###### collapse_rows(rows) {#rust-flattotypemappertrait-collapse_rows-rows}

Maps and collapses denormalized join rows.

**Definition**

```rust
fn collapse_rows(&self, rows: Vec<Row>) -> Vec<MappedRow>
```

**Parameters**

| Name | Description |
| --- | --- |
| `rows` | denormalized join rows |

**Returns**

`Vec<`[MappedRow](#rust-mappedrow)`>`

##### FlatToTypeMapper {#rust-flattotypemapper}

```rust
pub struct FlatToTypeMapper
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(spec)](#rust-flattotypemapper-new-spec) | Builds a collapse mapper from a [TypeMap](#rust-typemap). |

**Methods**

| Name | Description |
| --- | --- |
| [map(row)](#rust-flattotypemapper-map-row) | Maps one denormalized row. Nested group fields on a collection are omitted. |
| [reverse_map(value)](#rust-flattotypemapper-reverse_map-value) | Flattens a mapped row back. Groups keep the first child. |
| [spec()](#rust-flattotypemapper-spec) | The [TypeMap](#rust-typemap) used to collapse join rows. |
| [collapse(rows)](#rust-flattotypemapper-collapse-rows) | Collapses already-mapped rows. |
| [collapse_rows(rows)](#rust-flattotypemapper-collapse_rows-rows) | Maps and collapses denormalized join rows. |

###### new(spec) {#rust-flattotypemapper-new-spec}

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

[FlatToTypeMapper](#rust-flattotypemapper)

###### map(row) {#rust-flattotypemapper-map-row}

Maps one denormalized row. Nested group fields on a collection are omitted.

**Definition**

```rust
fn map(&self, row: Row) -> MappedRow
```

**Parameters**

| Name | Description |
| --- | --- |
| `row` | denormalized join row |

**Returns**

[MappedRow](#rust-mappedrow)

###### reverse_map(value) {#rust-flattotypemapper-reverse_map-value}

Flattens a mapped row back. Groups keep the first child.

**Definition**

```rust
fn reverse_map(&self, value: MappedRow) -> Row
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | mapped graph |

**Returns**

`Row`

###### spec() {#rust-flattotypemapper-spec}

The [TypeMap](#rust-typemap) used to collapse join rows.

**Definition**

```rust
fn spec(&self) -> &TypeMap
```

**Returns**

`&`[TypeMap](#rust-typemap)

###### collapse(rows) {#rust-flattotypemapper-collapse-rows}

Collapses already-mapped rows.

**Definition**

```rust
fn collapse(&self, rows: Vec<MappedRow>) -> Vec<MappedRow>
```

**Parameters**

| Name | Description |
| --- | --- |
| `rows` | rows already passed through [map](#rust-flattotypemapper-map-value) |

**Returns**

`Vec<`[MappedRow](#rust-mappedrow)`>`

###### collapse_rows(rows) {#rust-flattotypemapper-collapse_rows-rows}

Maps and collapses denormalized join rows.

**Definition**

```rust
fn collapse_rows(&self, rows: Vec<Row>) -> Vec<MappedRow>
```

**Parameters**

| Name | Description |
| --- | --- |
| `rows` | denormalized join rows |

**Returns**

`Vec<`[MappedRow](#rust-mappedrow)`>`

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
| [key](#rust-typemap-key) | Parent key column. |
| [fields](#rust-typemap-fields) | Column or group per destination field. |

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

###### key {#rust-typemap-key}

Parent key column.

**Definition**

```rust
pub key: String
```

###### fields {#rust-typemap-fields}

Column or group per destination field.

**Definition**

```rust
pub fields: Vec<(String, TypeMapField)>
```

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
| [key](#rust-typemapgroup-key) | Child key column used to unique children. |
| [fields](#rust-typemapgroup-fields) | Column or group per child field. |

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

###### key {#rust-typemapgroup-key}

Child key column used to unique children.

**Definition**

```rust
pub key: String
```

###### fields {#rust-typemapgroup-fields}

Column or group per child field.

**Definition**

```rust
pub fields: Vec<(String, TypeMapField)>
```

##### TypeMapField {#rust-typemapfield}

```rust
pub enum TypeMapField
```

**Properties**

| Name | Description |
| --- | --- |
| [Column](#rust-typemapfield-column) | Maps a destination field to a source column. |
| [Group](#rust-typemapfield-group) | Maps a destination field to a child [TypeMapGroup](#rust-typemapgroup). |

**Methods**

| Name | Description |
| --- | --- |
| [column(source)](#rust-typemapfield-column-source) | Builds a [Column](#rust-typemapfield-column) field. |

###### Column {#rust-typemapfield-column}

Maps a destination field to a source column.

**Definition**

```rust
Column(String)
```

###### Group {#rust-typemapfield-group}

Maps a destination field to a child [TypeMapGroup](#rust-typemapgroup).

**Definition**

```rust
Group(TypeMapGroup)
```

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
| [Cell](#rust-mappedvalue-cell) | A scalar cell. |
| [Many](#rust-mappedvalue-many) | A child collection. |

###### Cell {#rust-mappedvalue-cell}

A scalar cell.

**Definition**

```rust
Cell(Value)
```

###### Many {#rust-mappedvalue-many}

A child collection.

**Definition**

```rust
Many(Vec<MappedRow>)
```

##### MappedRow {#rust-mappedrow}

```rust
pub type MappedRow = HashMap<String, MappedValue>
```

##### StatementMapper {#rust-statementmapper}

```rust
pub struct StatementMapper
```

**Properties**

| Name | Description |
| --- | --- |
| [entity](#rust-statementmapper-entity) | Maps the table or entity name when the repository is built. |
| [columns](#rust-statementmapper-columns) | Maps column names when the repository is built. |
| [query](#rust-statementmapper-query) | Rewrites finished query SQL. |
| [insert](#rust-statementmapper-insert) | Rewrites finished insert SQL. |
| [update](#rust-statementmapper-update) | Rewrites finished update SQL. |
| [delete](#rust-statementmapper-delete) | Rewrites finished delete SQL. |
| [stored_procedure](#rust-statementmapper-stored_procedure) | Rewrites finished stored-procedure SQL. |
| [rows](#rust-statementmapper-rows) | Get-row mapper. [map](#rust-statementmapper-map-value) / [reverse_map](#rust-statementmapper-reverse_map-value) delegate here. |

**Methods**

| Name | Description |
| --- | --- |
| [identity(source, destination)](#rust-statementmapper-identity-source-destination) | Identity statement mapper. Source and destination labels are unused. |
| [map_sql(mapper, text)](#rust-statementmapper-map_sql-mapper-text) | Applies an optional SQL-text mapper. |
| [map(value)](#rust-statementmapper-map-value) | Maps a row through [rows](#rust-statementmapper-rows). |
| [reverse_map(value)](#rust-statementmapper-reverse_map-value) | Reverse-maps a row through [rows](#rust-statementmapper-rows). |

###### entity {#rust-statementmapper-entity}

Maps the table or entity name when the repository is built.

**Definition**

```rust
pub entity: TransformMapper<String, String>
```

###### columns {#rust-statementmapper-columns}

Maps column names when the repository is built.

**Definition**

```rust
pub columns: TransformMapper<String, String>
```

###### query {#rust-statementmapper-query}

Rewrites finished query SQL.

**Definition**

```rust
pub query: Option<Arc<TransformMapper<String, String>>>
```

###### insert {#rust-statementmapper-insert}

Rewrites finished insert SQL.

**Definition**

```rust
pub insert: Option<Arc<TransformMapper<String, String>>>
```

###### update {#rust-statementmapper-update}

Rewrites finished update SQL.

**Definition**

```rust
pub update: Option<Arc<TransformMapper<String, String>>>
```

###### delete {#rust-statementmapper-delete}

Rewrites finished delete SQL.

**Definition**

```rust
pub delete: Option<Arc<TransformMapper<String, String>>>
```

###### stored_procedure {#rust-statementmapper-stored_procedure}

Rewrites finished stored-procedure SQL.

**Definition**

```rust
pub stored_procedure: Option<Arc<TransformMapper<String, String>>>
```

###### rows {#rust-statementmapper-rows}

Get-row mapper. [map](#rust-statementmapper-map-value) / [reverse_map](#rust-statementmapper-reverse_map-value) delegate here.

**Definition**

```rust
pub rows: TransformMapper<Row, Row>
```

###### identity(source, destination) {#rust-statementmapper-identity-source-destination}

Identity statement mapper. Source and destination labels are unused.

**Definition**

```rust
pub fn identity(_source: impl Into<String>, _destination: impl Into<String>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `source` | unused source label |
| `destination` | unused destination label |

**Returns**

[StatementMapper](#rust-statementmapper)

###### map_sql(mapper, text) {#rust-statementmapper-map_sql-mapper-text}

Applies an optional SQL-text mapper.

**Definition**

```rust
pub fn map_sql(mapper: &Option<Arc<TransformMapper<String, String>>>, text: String) -> String
```

**Parameters**

| Name | Description |
| --- | --- |
| `mapper` | optional SQL-text [TransformMapper](#rust-transformmapper) |
| `text` | finished SQL |

**Returns**

`String`

###### map(value) {#rust-statementmapper-map-value}

Maps a row through [rows](#rust-statementmapper-rows).

**Definition**

```rust
fn map(&self, value: Row) -> Row
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | converted row |

**Returns**

`Row`

###### reverse_map(value) {#rust-statementmapper-reverse_map-value}

Reverse-maps a row through [rows](#rust-statementmapper-rows).

**Definition**

```rust
fn reverse_map(&self, value: Row) -> Row
```

**Parameters**

| Name | Description |
| --- | --- |
| `value` | shaped row |

**Returns**

`Row`

### Remarks

Repository get is convert, then map. `StatementMapper.Rows` / `rows` is the get-row mapper. `Repository.Mapper` / `mapper` aliases it.

Apply statement mapping in either or both places:

1. **Builder** — `RepositoryBuilder` maps `Entity` / `entity` and `Columns` / `columns` into `RepositoryStatements`.
2. **Per-statement** — `Query` / `Insert` / `Update` / `Delete` rewrite the finished statement.

C# and TypeScript statement mappers rewrite statement objects. Rust `query` / `insert` / `update` / `delete` / `stored_procedure` are `TransformMapper<String, String>` over SQL text via `StatementMapper::map_sql`.

`NameMapper` passes names through as-is.

### Examples

#### Flatten with TransformMapper

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/NameMapperTests.cs#L32-L45)

```csharp
var nested = new Nested(new User(1, "Ada"), new Contacts("ada@x.dev", "555"));
var mapper = new TransformMapper<Nested, Flat>(
    map: n => new Flat(n.User.Id, n.User.Name, n.Contacts.Email, n.Contacts.Phone),
    reverseMap: f => new Nested(new User(f.UserId, f.UserName), new Contacts(f.ContactEmail, f.ContactPhone)));
var flat = mapper.Map(nested);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/mod.rs#L144-L180)

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
            id: f.user_id,
            name: f.user_name,
        },
        contacts: Contacts {
            email: f.contact_email,
            phone: f.contact_phone,
        },
    },
);
let flat = Mapper::map(&mapper, nested.clone());
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/nameMapper.test.ts#L41-L66)

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

#### Pass-through names

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/NameMapperTests.cs#L15-L25)

```csharp
var mapper = new NameSourceDestinationMapper("sql", "csharp");
ISourceDestinationMapper labels = mapper;
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/mod.rs#L107-L116)

```rust
let mapper = NameSourceDestinationMapper::new("sql", "rust");
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/nameMapper.test.ts#L19-L27)

```typescript
const mapper = new NameSourceDestinationMapper("sql", "typescript");
const labels: ISourceDestinationMapper = mapper;
const mapping: IMapper<string, string> = mapper;
```

#### Rewrite a finished statement

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/StatementMapperTests.cs#L55-L81)

```csharp
var statements = new RepositoryStatements(
    "users",
    ["id", "name"],
    new TransformMapper<IQuery, IQuery>(
        query => new Query($"{query.GetText()} LIMIT 1", query.Where),
        query => query),
    new TransformMapper<IInsertStatement, IInsertStatement>(
        insert => new InsertStatement($"{insert.GetText()} RETURNING id"),
        insert => insert),
    new TransformMapper<IUpdateStatement, IUpdateStatement>(
        update => new UpdateStatement($"{update.GetText()} RETURNING id", update.Where),
        update => update),
    new TransformMapper<IDeleteStatement, IDeleteStatement>(
        delete => new DeleteStatement($"{delete.GetText()} RETURNING id", delete.Where),
        delete => delete));
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L794-L804)

```rust
let mut mapper = StatementMapper::identity("", "");
mapper.stored_procedure = Some(Arc::new(TransformMapper::new(
    |text: String| format!("{text} /* mapped */"),
    |text| text,
)));
let built = StatementBuilder::create_stored_procedure("save", &[Parameter::new("id", 1_i64)]);
let text = StatementMapper::map_sql(&mapper.stored_procedure, built.get_text());
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/StatementMapper.test.ts#L27-L59)

```typescript
const mapper = new StatementMapper({
  entity: names,
  columns: names,
  query: new TransformMapper(
    (query) => new Query(`${query.getText()} LIMIT 1`, query.where),
    (query) => query,
  ),
  insert: new TransformMapper(
    (insert) => new InsertStatement(`${insert.getText()} RETURNING id`),
    (insert) => insert,
  ),
  update: new TransformMapper(
    (update) =>
      new UpdateStatement(`${update.getText()} RETURNING id`, update.where),
    (update) => update,
  ),
  delete: new TransformMapper(
    (remove) =>
      new DeleteStatement(`${remove.getText()} RETURNING id`, remove.where),
    (remove) => remove,
  ),
});
const statements = statementsOf("users", ["id", "name"], mapper);
```

## TypeConverters

`TypeConverter` is only `Convert` / `ReverseConvert`. `RowTypeConverter` holds `Types` and `Fields`. Convert uses `Fields[name]`.

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
| [types](#typescript-rowtypeconverter-types) | Provider type catalog. Convert does not read this map. |
| [fields](#typescript-rowtypeconverter-fields) | Per-column converters. Convert uses `fields[name]`. |

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
| `fields` | per-column [TypeConverter](#typescript-typeconverter) |

###### types {#typescript-rowtypeconverter-types}

Provider type catalog. Convert does not read this map.

**Definition**

```typescript
types: Record<string, TypeConverter<Cell, Cell>>
```

###### fields {#typescript-rowtypeconverter-fields}

Per-column converters. Convert uses `fields[name]`.

**Definition**

```typescript
fields: Record<string, TypeConverter<Cell, Cell>>
```

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
| [Types](#csharp-rowtypeconverter-types) | Provider type catalog. Convert does not read this map. |
| [Fields](#csharp-rowtypeconverter-fields) | Per-column converters. Convert uses `Fields[name]`. |

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
| `fields` | per-column [TypeConverter](#csharp-typeconverter) |

###### Types {#csharp-rowtypeconverter-types}

Provider type catalog. Convert does not read this map.

**Definition**

```csharp
Dictionary<string, TypeConverter<Value, Value>> Types { get; set; }
```

###### Fields {#csharp-rowtypeconverter-fields}

Per-column converters. Convert uses `Fields[name]`.

**Definition**

```csharp
Dictionary<string, TypeConverter<Value, Value>> Fields { get; set; }
```

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
| [types](#rust-rowtypeconverter-types) | Provider type catalog. Convert does not read this map. |
| [fields](#rust-rowtypeconverter-fields) | Per-column converters. Convert uses `fields[name]`. |

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

###### types {#rust-rowtypeconverter-types}

Provider type catalog. Convert does not read this map.

**Definition**

```rust
pub types: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>
```

###### fields {#rust-rowtypeconverter-fields}

Per-column converters. Convert uses `fields[name]`.

**Definition**

```rust
pub fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>
```

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
| `fields` | per-column [TypeConverter](#rust-typeconverter) |

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

A dialect adds a `TypeConverter` only when the native provider cannot map the type. Core does not ship bool / datetime / uuid / decimal / binary converters.

`Types` / `types` is the provider catalog merge. Convert and reverse-convert only consult `Fields` / `fields`.

Repository get is convert, then map. See [Mappers](#mappers) and [Repositories](#readonly-repository).

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

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/converters/RowTypeConverter.test.ts#L13-L19)

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

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/converters/RowTypeConverter.test.ts#L21-L27)

```typescript
const converter = new RowTypeConverter({}, { flag: new OnlyOne() });
const dest = converter.convert({ flag: 1, name: "a" });
```

## Command Line (CLI)

`@datasources/cli` installs provider packages and writes a local `.env`. `Default` / `default` / `DatasourceFactory.default` reads that file for `PROVIDER` and `DATASOURCE_URL`. `DEFAULT_PROVIDER` and `DEFAULT_DATASOURCE_URL` in the process override `.env`. If neither source sets a provider or URL, create throws.

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

`setup` writes these keys. `add` / `remove` update the provider list. Other keys already in the file are kept. `Default` uses the `DATASOURCE_URL` scheme as the default provider (`sqlite://`, `postgres://`, `mysql://`, …) unless `PROVIDER` or `DEFAULT_PROVIDER` is set.

| Key | Default | Meaning |
| --- | --- | --- |
| `INSTALLED_PROVIDERS` | scheme of `DATASOURCE_URL` | Comma-separated providers `Create` registers. |
| `DATASOURCE_URL` | `sqlite:///:memory:` | Connection URL. The scheme is the default provider. |
| `DATABASE_SCHEMA` | *(empty)* | Schema / search path (postgres). |

```sh
INSTALLED_PROVIDERS=sqlite,postgres
DATASOURCE_URL=sqlite:///:memory:
DATABASE_SCHEMA=public
```

Process overrides (not written by `setup`):

| Key | Meaning |
| --- | --- |
| `DEFAULT_PROVIDER` | Overrides `.env` `PROVIDER` / inferred scheme. |
| `DEFAULT_DATASOURCE_URL` | Overrides `.env` `DATASOURCE_URL`. |

SQLAlchemy-style SQLite URLs: `sqlite:///:memory:`, `sqlite:///app.db`, `sqlite:////tmp/app.db`.
