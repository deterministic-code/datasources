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

A `EnrichedCRUDRepository` wraps a CRUD repository, fills related fields after `read`, and writes nested `many` children on create / update / delete.

##### C#

```csharp
var contacts = new EnrichedCRUDRepository<Contact>(contactRepo);
contacts.AddEnriched("contact_type_name", "contact_type_id", types, "id", "name");
contacts.AddEnriched("addresses", "id", addresses, "contact_id", many: true,
    nest: node => node.Include(addressTypes, "address_type_name", "address_type_id", "id", "name"));

var loaded = await contacts.ReadSingle(new WhereClause(new Operator.Equals("id", 10)));
await contacts.Create(new Contact { first_name = "Ada", addresses = [new Address { line1 = "1 Main" }] });
```

##### Rust

```rust
let mut contacts = EnrichedCRUDRepository::new(contact_repo);
contacts
    .add_enriched("contact_type_name", "contact_type_id", types, "id", Some("name".into()), false, ())
    .add_enriched("addresses", "id", addresses, "contact_id", None, true, |node| {
        node.include(address_types, "address_type_name", "address_type_id", "id", Some("name".into()), false);
    });

let loaded = contacts.read_single(Some(&WhereClause::new(Operator::equals("id", 10)))).await?;
contacts.create(contact).await?;
```

##### TypeScript

```typescript
const contacts = new EnrichedCRUDRepository(contactRepo);
contacts.addEnriched("contact_type_name", "contact_type_id", types, "id", "name");
contacts.addEnriched("addresses", "id", addresses, "contact_id", undefined, true, (node) =>
  node.include(addressTypes, "address_type_name", "address_type_id", "id", "name"),
);

const loaded = await contacts.readSingle(new WhereClause(new Operator.Equals("id", 10)));
await contacts.create({ first_name: "Ada", addresses: [{ line1: "1 Main" }] });
```

See [Repository](#repository) for readonly, CRUD, and bulk.

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

See [Mapper](#mapper) for `TransformMapper`, `FlatToNestedMapper`, and [ReadonlyRepositoryOptions.RowsMapper](#rowsmapper).

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

See [TypeConverter](#type-converter).

## Feature Matrix

Parity across C#, Rust, and TypeScript. Standard modules first; opinionated repositories above them.

### Feature stack

| Feature | C# | Rust | TypeScript |
| --- | :---: | :---: | :---: |
| [Datasource Factory](#datasource-factory) | ✅ | ✅ | ✅ |
| [Datasource](#datasource) | ✅ | ✅ | ✅ |
| [Mapper](#mapper) | ✅ | ✅ | ✅ |
| [Type Converter](#type-converter) | ✅ | ✅ | ✅ |
| [Connection](#connection) | ✅ | ✅ | ✅ |
| [Transaction](#transaction) | ✅ | ✅ | ✅ |
| [Command](#command) | ✅ | ✅ | ✅ |
| [Reader](#reader) | ✅ | ✅ | ✅ |
| [Statements](#statements) | ✅ | ✅ | ✅ |
| [Repository](#repository) | ✅ | ✅ | ✅ |
| [Readonly Repository](#readonly-repository) | ✅ | ✅ | ✅ |
| [CRUD Repository](#crud-repository) | ✅ | ✅ | ✅ |
| [Enriched Repository](#enriched-repository) | ✅ | ✅ | ✅ |
| [Bulk Repository](#bulk-repository) | ✅ | ✅ | ✅ |

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

TODO: register a provider, create a connection, `createRepository`, then `create` / `read` / `update` / `delete`.

### Run a command

TODO: open a connection, `createCommand`, `execute` / `executeReader`. See [Command](#command) for the current samples.

### Run a transaction

TODO: `startTransaction`, work, then `commit` or `abort`. See [Transaction](#transaction).

### Map a row

TODO: `TransformMapper` flatten, pass-through names, or rewrite a statement. See [Mapper](#mapper).

### Convert a row

TODO: `RowTypeConverter` convert / reverse-convert, or attach a field converter. See [RowTypeConverter](#row-type-converter).

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
| default | Loads `.env` `PROVIDER` / `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. Throws when neither is set. Uses [create](#typescript-create-options). |

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
| Default | Loads `.env` `PROVIDER` / `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. Throws when neither is set. Uses [Create](#csharp-create-options). |

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
| [register(provider, driver)](#rust-datasources-register-provider-driver) | Registers a provider [Driver](#rust-driver) by name. |
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

`Datasources.Hosting`, `@datasources/register`, and `datasources-register` register each name in `INSTALLED_PROVIDERS` (and the provider inferred from `DATASOURCE_URL`) when `Create` is called with null / `undefined` / `None` options. Each dialect implements [IDriver](#idriver) (`MemoryProvider`, `SqliteDriver`, `MysqlDriver`, `PostgresDriver`, …) and that is what `register` stores.

### Examples

#### Default from environment

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/DatasourceFactoryTests.cs#L24-L32)

```csharp
DatasourceFactory.Register("sqlite", new MockDriver());
Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
Environment.SetEnvironmentVariable("DATASOURCE_URL", "sqlite:///:memory:");
var ds = DatasourceFactory.Default;
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/datasource_factory.rs#L21-L28)

```rust
register("sqlite", MockDriver::arc());
std::env::set_var("PROVIDER", "sqlite");
std::env::set_var("DATASOURCE_URL", "sqlite:///:memory:");
let ds = default().expect("default");
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/DatasourceFactory.test.ts#L34-L38)

```typescript
DatasourceFactory.register("sqlite", new MockDriver());
process.env.PROVIDER = "sqlite";
process.env.DATASOURCE_URL = "sqlite:///:memory:";
const ds = DatasourceFactory.default;
```

#### Register and Create

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Memory.Tests/RegisterAndCreateTests.cs#L10-L16)

```csharp
DatasourceFactory.Register("memory", new MemoryProvider());
var ds = DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = "memory" });
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L38-L44)

```rust
#[test]
fn register_and_create() {
    reset();
    register("memory", Arc::new(MemoryProvider));
    let ds = create(Some(&HashMap::from([("provider".into(), "memory".into())]))).unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/memory/src/index.test.ts#L27-L32)

```typescript
DatasourceFactory.register("memory", new MemoryProvider());
const ds = DatasourceFactory.create({ provider: "memory" });
```

#### Missing provider

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/DatasourceFactoryTests.cs#L34-L40)

```csharp
DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = "missing" });
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/datasource_factory.rs#L28-L35)

```rust
create(Some(&HashMap::from([("provider".into(), "missing".into())])));
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/DatasourceFactory.test.ts#L41-L45)

```typescript
DatasourceFactory.create({ provider: "missing" });
```

## Datasource

One configured datasource: provider name plus factories for connections, commands, and repositories. `Default` / `default` / `Create` is on [DatasourceFactory](#datasource-factory).

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
| provider | Registered provider name. |

**Methods**

| Name | Description |
| --- | --- |
| [createConnection()](#typescript-idatasource-createconnection) | Creates a closed connection. The caller opens it. |
| [createCommand(connection?)](#typescript-idatasource-createcommand-connection) | Creates a command bound to an optional open connection. |
| [createRepository(type, connection?, options?)](#typescript-idatasource-createrepository-type-connection-options) | Creates a CRUD repository for a class, bound to an optional open connection. Columns, id, and entity come from that class. The driver's row type converters replace `rowTypeConverters`. |
| [createReadonlyRepository(type, connection?, options?)](#typescript-idatasource-createreadonlyrepository-type-connection-options) | Creates a read-only repository for a class, bound to an optional open connection. `Datasource` returns a `ReadonlyRepository`. The driver's row type converters replace `rowTypeConverters`. |

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

###### createRepository(type, connection?, options?) {#typescript-idatasource-createrepository-type-connection-options}

Creates a CRUD repository for a class, bound to an optional open connection. Columns, id, and entity come from that class. The driver's row type converters replace `rowTypeConverters`.

**Definition**

```typescript
createRepository<T>(type: Constructable<T>, connection?: IConnection, options?: CRUDRepositoryOptions<T>): ICRUDRepository<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `type` | class constructor. Its fields become columns, `id` or `Id` becomes the id, and `type.name` becomes the entity |
| `connection` | optional open [IConnection](#connection); omitted constructs one |
| `options` | optional [CRUDRepositoryOptions](#typescript-crudrepositoryoptions). Entity, columns, and mappers overlay the type defaults |

**Returns**

[ICRUDRepository](#crud-repository)`<T>`

###### createReadonlyRepository(type, connection?, options?) {#typescript-idatasource-createreadonlyrepository-type-connection-options}

Creates a read-only repository for a class, bound to an optional open connection. `Datasource` returns a `ReadonlyRepository`. The driver's row type converters replace `rowTypeConverters`.

**Definition**

```typescript
createReadonlyRepository<T>(type: Constructable<T>, connection?: IConnection, options?: ReadonlyRepositoryOptions<T>): IReadonlyRepository<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `type` | class constructor. Its fields become columns, `id` or `Id` becomes the id, and `type.name` becomes the entity |
| `connection` | optional open [IConnection](#connection); omitted constructs one |
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
| provider | Which datasource to return. |
| database_url | Connection URL. The scheme selects the provider when `provider` is omitted. |
| database_schema | Default schema. |

##### C#

##### Datasource {#csharp-datasource}

```csharp
public sealed class Datasource : IDatasource
```

**Constructor**

| Name | Description |
| --- | --- |
| [Datasource(provider, driver, options)](#csharp-datasource-datasource-provider-driver-options) | Builds a datasource from a provider [IDriver](#csharp-idriver). |

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
| [CreateConnection()](#csharp-idatasource-createconnection) | Creates a closed connection. The caller opens it. |
| [CreateCommand(connection)](#csharp-idatasource-createcommand-connection) | Creates a command bound to an optional open connection. |
| [CreateRepository(connection, options)](#csharp-idatasource-createrepository-connection-options) | Creates a CRUD repository bound to an optional open connection. The driver's row type converters replace `RowTypeConverters`. |
| [CreateReadonlyRepository(connection, options)](#csharp-idatasource-createreadonlyrepository-connection-options) | Creates a read-only repository bound to an optional open connection. `Datasource` returns a `ReadonlyRepository`. The interface default still forwards to `CreateRepository` when a type does not override the method. The driver's row type converters replace `RowTypeConverters`. |

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

###### CreateRepository(connection, options) {#csharp-idatasource-createrepository-connection-options}

Creates a CRUD repository bound to an optional open connection. The driver's row type converters replace `RowTypeConverters`.

**Definition**

```csharp
ICRUDRepository<T> CreateRepository<T>(IConnection? connection = null, CRUDRepositoryOptions<T>? options = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional open [IConnection](#connection); omitted constructs one |
| `options` | optional [CRUDRepositoryOptions](#csharp-crudrepositoryoptions). Entity, columns, and mappers overlay the type defaults |

**Returns**

[ICRUDRepository](#crud-repository)`<T>`

###### CreateReadonlyRepository(connection, options) {#csharp-idatasource-createreadonlyrepository-connection-options}

Creates a read-only repository bound to an optional open connection. `Datasource` returns a `ReadonlyRepository`. The interface default still forwards to `CreateRepository` when a type does not override the method. The driver's row type converters replace `RowTypeConverters`.

**Definition**

```csharp
IReadonlyRepository<T> CreateReadonlyRepository<T>(IConnection? connection = null, ReadonlyRepositoryOptions<T>? options = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional open [IConnection](#connection); omitted constructs one |
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
| provider | Which datasource to return. |
| database_url | Connection URL. The scheme selects the provider when `provider` is omitted. |
| database_schema | Default schema. |

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
| [create_repository(connection, options)](#rust-datasource-create_repository-connection-options) | Creates a CRUD repository for `T`, bound to an optional open connection. The driver's row type converters replace `row_type_converters`. |
| [create_readonly_repository(connection, options)](#rust-datasource-create_readonly_repository-connection-options) | Creates a read-only repository for `T`, bound to an optional open connection. `BuiltDatasource` returns a `ReadonlyRepository`. The driver's row type converters replace `row_type_converters`. |

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

###### create_repository(connection, options) {#rust-datasource-create_repository-connection-options}

Creates a CRUD repository for `T`, bound to an optional open connection. The driver's row type converters replace `row_type_converters`.

**Definition**

```rust
fn create_repository<T: ConvertSchema + 'static>(&self, connection: Option<Arc<dyn Connection>>, options: Option<CRUDRepositoryOptions>) -> Arc<dyn ICRUDRepository>
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional open [Connection](#connection); omitted constructs one |
| `options` | optional [CRUDRepositoryOptions](#rust-crudrepositoryoptions). Entity, columns, and mappers overlay the type defaults. `None` uses the type defaults |

**Returns**

`Arc<dyn `[CRUDRepository](#crud-repository)`>`

###### create_readonly_repository(connection, options) {#rust-datasource-create_readonly_repository-connection-options}

Creates a read-only repository for `T`, bound to an optional open connection. `BuiltDatasource` returns a `ReadonlyRepository`. The driver's row type converters replace `row_type_converters`.

**Definition**

```rust
fn create_readonly_repository<T: ConvertSchema + 'static>(&self, connection: Option<Arc<dyn Connection>>, options: Option<ReadonlyRepositoryOptions>) -> Arc<dyn IReadonlyRepository>
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | optional open [Connection](#connection); omitted constructs one |
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
| provider | Which datasource to return. |
| database_url | Connection URL. The scheme selects the provider when `provider` is omitted. |
| database_schema | Default schema. |

### Remarks

`DatasourceFactory.Default` / `DatasourceFactory.default` / `default()` loads `.env` `PROVIDER` and `DATASOURCE_URL`, then `DEFAULT_PROVIDER` / `DEFAULT_DATASOURCE_URL`. `Create(null)` / `create()` / `create(None)` uses the same path. A non-null options bag is used as-is; missing `provider` and `database_url` throw.

Listed factories are registered through `Datasources.Hosting`, `@datasources/register`, or `datasources-register` when that package is present.

`Reset` / `reset` clears the registry. Use it in tests.

See [DatasourceFactory](#datasource-factory) for the factory contract, and [CLI](#command-line-cli) for `setup` and adding providers.

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
| provider | Registered provider name. |
| isOpen | Whether the connection is open. |

**Methods**

| Name | Description |
| --- | --- |
| [createCommand()](#typescript-iconnection-createcommand) | Creates a command bound to this connection. |
| [open()](#typescript-iconnection-open) | Opens the connection. A no-op if already open. |
| [close()](#typescript-iconnection-close) | Closes the connection. |
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
| Provider | Registered provider name. |
| IsOpen | Whether the connection is open. |

**Methods**

| Name | Description |
| --- | --- |
| [CreateCommand()](#csharp-iconnection-createcommand) | Creates a command bound to this connection. |
| [Open()](#csharp-iconnection-open) | Opens the connection. A no-op if already open. |
| [Close()](#csharp-iconnection-close) | Closes the connection. |
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

Working dialects (`json`, `memory`) open against injected ports (`IFileStore` / `FileStore`, `MemoryStore`). SQLite, MySQL, and Postgres open the database client on the connection. Stub dialects throw not-implemented on open, close, and start-transaction.

### Examples

#### Create from Default

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/ConnectionTests.cs#L27-L33)

```csharp
var ds = DatasourceFactory.Default;
var connection = await ds.CreateConnection();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/connection.rs#L21-L26)

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

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L47-L52)

```rust
#[tokio::test]
async fn open_and_close() {
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

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L54-L60)

```rust
#[tokio::test]
async fn start_a_transaction() {
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
| ReadCommitted | Committed data only. |
| RepeatableRead | Repeatable reads within the transaction. |
| Serializable | Full isolation. The default for [startTransaction](#typescript-iconnection-starttransaction-isolation). |

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
| ReadCommitted | Committed data only. |
| RepeatableRead | Repeatable reads within the transaction. |
| Serializable | Full isolation. The default for [StartTransaction](#csharp-iconnection-starttransaction-isolation). |

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
| ReadCommitted | Committed data only. |
| RepeatableRead | Repeatable reads within the transaction. |
| Serializable | Full isolation. The default for [start_transaction](#rust-connection-start_transaction-isolation). |

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

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/transaction.rs#L21-L28)

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

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L63-L69)

```rust
#[tokio::test]
async fn commit() {
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

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L72-L78)

```rust
#[tokio::test]
async fn abort() {
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

Named command bound to an optional open [connection](#connection). `CreateCommand` / `createCommand` / `create_command` is sync. The caller executes it.

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
| [executeReader(text, parameters?)](#typescript-icommand-executereader-text-parameters) | Runs a query and returns a reader over the rows. |
| [execute(text, parameters?)](#typescript-icommand-execute-text-parameters) | Runs a non-query command. |
| [execute(command, statement, parameters?)](#typescript-icommand-execute-command-statement-parameters) | Runs [execute](#typescript-icommand-execute-text-parameters) with `statement.getText()`. |
| [executeReader(command, statement, parameters?)](#typescript-icommand-executereader-command-statement-parameters) | Runs [executeReader](#typescript-icommand-executereader-text-parameters) with `statement.getText()`. |

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

Runs a query and returns a reader over the rows.

**Definition**

```typescript
executeReader(text: string, parameters?: Parameter[]): Promise<IReader<Cells>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL or dialect command text |
| `parameters` | optional [Parameter](#typescript-parameter) list |

**Returns**

`Promise<[IReader](#typescript-reader)<Cells>>`

###### execute(text, parameters?) {#typescript-icommand-execute-text-parameters}

Runs a non-query command.

**Definition**

```typescript
execute(text: string, parameters?: Parameter[]): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL or dialect command text |
| `parameters` | optional [Parameter](#typescript-parameter) list |

**Returns**

`Promise<[Response](#typescript-response)>`

###### execute(command, statement, parameters?) {#typescript-icommand-execute-command-statement-parameters}

Runs [execute](#typescript-icommand-execute-text-parameters) with `statement.getText()`.

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

Runs [executeReader](#typescript-icommand-executereader-text-parameters) with `statement.getText()`.

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

`Promise<[IReader](#typescript-reader)<Cells>>`

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
interface Response
```

**Properties**

| Name | Description |
| --- | --- |
| result | Scalar or row result from the command. |
| rowsAffected | Rows inserted, updated, or deleted. |
| parameters | Bound [Parameter](#typescript-parameter) list after [execute](#typescript-icommand-execute-text-parameters). Holds filled [output](#typescript-parameterdirection) / [returnValue](#typescript-parameterdirection) values. |

##### responseOf(result, rowsAffected, parameters?) {#typescript-responseof-result-rowsaffected-parameters}

```typescript
function responseOf(result: unknown, rowsAffected: number, parameters?: Parameter[]): Response
```

##### IReader {#reader} {#typescript-ireader-reader}

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
| [next()](#typescript-ireader-reader-next) | Advances to the next row. |

###### next() {#typescript-ireader-reader-next}

Advances to the next row.

**Definition**

```typescript
next(): Promise<boolean>
```

**Returns**

`Promise<boolean>`

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
| [ExecuteReader(text, parameters)](#csharp-icommand-executereader-text-parameters) | Runs a query and returns a reader over the rows. |
| [Execute(text, parameters)](#csharp-icommand-execute-text-parameters) | Runs a non-query command. |
| [ExecuteReader(statement, parameters)](#csharp-icommand-executereader-statement-parameters) | Runs [ExecuteReader](#csharp-icommand-executereader-text-parameters) with `statement.GetText()`. |
| [Execute(statement, parameters)](#csharp-icommand-execute-statement-parameters) | Runs [Execute](#csharp-icommand-execute-text-parameters) with `statement.GetText()`. |

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

Runs a query and returns a reader over the rows.

**Definition**

```csharp
Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL or dialect command text |
| `parameters` | optional [Parameter](#csharp-parameter) list |

**Returns**

`Task<[IReader](#csharp-reader)<Row>>`

###### Execute(text, parameters) {#csharp-icommand-execute-text-parameters}

Runs a non-query command.

**Definition**

```csharp
Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL or dialect command text |
| `parameters` | optional [Parameter](#csharp-parameter) list |

**Returns**

`Task<[Response](#csharp-response)>`

###### ExecuteReader(statement, parameters) {#csharp-icommand-executereader-statement-parameters}

Runs [ExecuteReader](#csharp-icommand-executereader-text-parameters) with `statement.GetText()`.

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

`Task<[IReader](#csharp-reader)<Row>>`

###### Execute(statement, parameters) {#csharp-icommand-execute-statement-parameters}

Runs [Execute](#csharp-icommand-execute-text-parameters) with `statement.GetText()`.

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
public sealed record Response(Value Result, int RowsAffected, IReadOnlyList<Parameter>? Parameters = null)
```

**Properties**

| Name | Description |
| --- | --- |
| Result | Scalar or row result from the command. |
| RowsAffected | Rows inserted, updated, or deleted. |
| Parameters | Bound [Parameter](#csharp-parameter) list after [Execute](#csharp-icommand-execute-text-parameters). Holds filled [Output](#csharp-parameterdirection) / [ReturnValue](#csharp-parameterdirection) values. |

**Methods**

| Name | Description |
| --- | --- |
| [Of(result, rowsAffected, parameters)](#csharp-response-of-result-rowsaffected-parameters) | Builds a [Response](#csharp-response). Copies a [ReturnValue](#csharp-parameterdirection) onto [Result](#csharp-response-result) when `result` is `Value.Null`. |

###### Of(result, rowsAffected, parameters) {#csharp-response-of-result-rowsaffected-parameters}

Builds a [Response](#csharp-response). Copies a [ReturnValue](#csharp-parameterdirection) onto [Result](#csharp-response-result) when `result` is `Value.Null`.

**Definition**

```csharp
public static Response Of(Value result, int rowsAffected, IReadOnlyList<Parameter>? parameters = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `result` | scalar or row result |
| `rowsAffected` | rows inserted, updated, or deleted |
| `parameters` | bound [Parameter](#csharp-parameter) list |

**Returns**

[Response](#csharp-response)

##### IReader {#reader} {#csharp-ireader-reader}

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
| [Next()](#csharp-ireader-reader-next) | Advances to the next row. |

###### Next() {#csharp-ireader-reader-next}

Advances to the next row.

**Definition**

```csharp
Task<bool> Next()
```

**Returns**

`Task<bool>`

##### Rust

##### Command {#rust-command}

```rust
pub trait Command: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [add_middleware(middleware)](#rust-command-add_middleware-middleware) | Adds [CommandMiddleware](#rust-commandmiddleware). |
| [execute_reader(text, parameters)](#rust-command-execute_reader-text-parameters) | Runs a query and returns a reader over the rows. |
| [execute(text, parameters)](#rust-command-execute-text-parameters) | Runs a non-query command. |
| [execute_reader_statement(statement, parameters)](#rust-command-execute_reader_statement-statement-parameters) | Runs [execute_reader](#rust-command-execute_reader-text-parameters) with `statement.get_text()`. |
| [execute_statement(statement, parameters)](#rust-command-execute_statement-statement-parameters) | Runs [execute](#rust-command-execute-text-parameters) with `statement.get_text()`. |

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

Runs a query and returns a reader over the rows.

**Definition**

```rust
async fn execute_reader(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Box<dyn Reader>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL or dialect command text |
| `parameters` | optional [Parameter](#rust-parameter) slice |

**Returns**

`Result<Box<dyn `[Reader](#rust-reader)`>>`

###### execute(text, parameters) {#rust-command-execute-text-parameters}

Runs a non-query command.

**Definition**

```rust
async fn execute(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | SQL or dialect command text |
| `parameters` | optional [Parameter](#rust-parameter) slice |

**Returns**

`Result<[Response](#rust-response)>`

###### execute_reader_statement(statement, parameters) {#rust-command-execute_reader_statement-statement-parameters}

Runs [execute_reader](#rust-command-execute_reader-text-parameters) with `statement.get_text()`.

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

`Result<Box<dyn `[Reader](#rust-reader)`>>`

###### execute_statement(statement, parameters) {#rust-command-execute_statement-statement-parameters}

Runs [execute](#rust-command-execute-text-parameters) with `statement.get_text()`.

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

**Properties**

| Name | Description |
| --- | --- |
| result | Scalar or row result from the command. |
| rows_affected | Rows inserted, updated, or deleted. |
| parameters | Bound [Parameter](#rust-parameter) list after [execute](#rust-command-execute-text-parameters). Holds filled [Output](#rust-parameterdirection) / [ReturnValue](#rust-parameterdirection) values. |

**Methods**

| Name | Description |
| --- | --- |
| [new(result, rows_affected)](#rust-response-new-result-rows_affected) | Builds a [Response](#rust-response) with no [parameters](#rust-response-parameters). |
| [of(result, rows_affected, parameters)](#rust-response-of-result-rows_affected-parameters) | Builds a [Response](#rust-response). Copies a [ReturnValue](#rust-parameterdirection) onto [result](#rust-response-result) when `result` is `Value::Null`. |

###### new(result, rows_affected) {#rust-response-new-result-rows_affected}

Builds a [Response](#rust-response) with no [parameters](#rust-response-parameters).

**Definition**

```rust
pub fn new(result: impl Into<Value>, rows_affected: i64) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `result` | scalar or row result |
| `rows_affected` | rows inserted, updated, or deleted |

**Returns**

[Response](#rust-response)

###### of(result, rows_affected, parameters) {#rust-response-of-result-rows_affected-parameters}

Builds a [Response](#rust-response). Copies a [ReturnValue](#rust-parameterdirection) onto [result](#rust-response-result) when `result` is `Value::Null`.

**Definition**

```rust
pub fn of(result: impl Into<Value>, rows_affected: i64, parameters: Option<Vec<Parameter>>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `result` | scalar or row result |
| `rows_affected` | rows inserted, updated, or deleted |
| `parameters` | bound [Parameter](#rust-parameter) list |

**Returns**

[Response](#rust-response)

##### Reader {#reader} {#rust-reader-reader}

```rust
pub trait Reader<T = Row>: Send + Sync
```

**Methods**

| Name | Description |
| --- | --- |
| [current()](#rust-reader-reader-current) | The row from the last successful [next](#rust-reader-next). |
| [next()](#rust-reader-reader-next) | Advances to the next row. |

###### current() {#rust-reader-reader-current}

The row from the last successful [next](#rust-reader-next).

**Definition**

```rust
fn current(&self) -> Option<T>
```

**Returns**

`Option<T>`

###### next() {#rust-reader-reader-next}

Advances to the next row.

**Definition**

```rust
async fn next(&self) -> Result<bool>
```

**Returns**

`Result<bool>`

### Remarks

The connection must be open before `Execute` / `execute` / `execute` or `ExecuteReader` / `executeReader` / `execute_reader`.

`AddMiddleware` / `addMiddleware` / `add_middleware` runs `BeforeCommand` / `beforeCommand` / `before_command` then the command, then `AfterCommand` / `afterCommand` / `after_command`. A `beforeCommand` error skips the command. A command error still runs `afterCommand` with `error` and elapsed time set. `ConsoleCommandMiddleware` logs Start/Finish lines to the console. Verbose logs use AfterCommand for the result body.

Working dialects (`json`, `memory`) treat `text` as `insert` / `update` / `delete`. `sqlite` treats `text` as SQL. Stub dialects throw or return not-implemented on execute and execute-reader.

The Rust `Command` trait has no connection getter. Stub `create_command` stores the optional connection on the named type; working dialects still construct their own typed connection today.

`ExecuteReader` returns rows. Convert with [RowTypeConverter](#row-type-converter), then map with a [Mapper](#mapper).

[Parameter](#parameter) defaults to [Input](#parameterdirection). [Output](#parameterdirection) / [InputOutput](#parameterdirection) / [ReturnValue](#parameterdirection) values are filled on [Response.Parameters](#response-parameters) after `Execute`. A [ReturnValue](#parameterdirection) copies onto [Response.Result](#response-result) when `Result` would otherwise be null. `Execute` on `SELECT` text reads the first scalar into `Result`.

### Examples

#### ExecuteReader to NameValue

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CommandTests.cs#L46-L60)

```csharp
var command = new MockCommand(new MockConnection())
{
    Reader = new MockReader<Row>([new Row { ["Name"] = "theme", ["Value"] = "dark" }])
};
var reader = await command.ExecuteReader("SELECT Name, Value FROM settings");
var converter = new RowTypeConverter<NameValue>();
var items = new List<NameValue>();
while (await reader.Next())
    items.Add(converter.Convert(reader.Current!));
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/command.rs#L60-L82)

```rust
let command = MockCommand::new();
command.reader_rows.lock().unwrap().push(row(&[
    ("name", Value::from("theme")),
    ("value", Value::from("dark")),
]));
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

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/Command.test.ts#L45-L56)

```typescript
const command = new MockCommand(new MockConnection());
command.reader = new MockReader([{ name: "theme", value: "dark" }]);
const reader = await command.executeReader("SELECT name, value FROM settings");
const converter = new RowTypeConverter<NameValue>();
const items: NameValue[] = [];
while (await reader.next()) {
  items.push(converter.convert(reader.current!, NameValue));
}
```

#### Execute insert, update, and delete

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CommandTests.cs#L62-L71)

```csharp
var command = new MockCommand(new MockConnection());
await command.Execute("insert", [new Parameter("id", 1), new Parameter("name", "a")]);
await command.Execute("update", [new Parameter("id", 1), new Parameter("name", "A")]);
await command.Execute("delete", [new Parameter("id", 1)]);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/command.rs#L84-L116)

```rust
    .execute(
        "insert",
        Some(&[Parameter::new("id", 1_i64), Parameter::new("name", "a")]),
    )
    .await
    .unwrap();
command
    .execute(
        "update",
        Some(&[Parameter::new("id", 1_i64), Parameter::new("name", "A")]),
    )
    .await
    .unwrap();
command
    .execute("delete", Some(&[Parameter::new("id", 1_i64)]))
    .await
    .unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/Command.test.ts#L58-L78)

```typescript
const command = new MockCommand(new MockConnection());
await command.execute("insert", [
  { name: "id", value: 1 },
  { name: "name", value: "a" },
]);
await command.execute("update", [
  { name: "id", value: 1 },
  { name: "name", value: "A" },
]);
await command.execute("delete", [{ name: "id", value: 1 }]);
```

#### Map flat contacts

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CommandTests.cs#L73-L97)

```csharp
var command = new MockCommand(new MockConnection())
{
    Reader = new MockReader<Row>([new Row
    {
        ["id"] = 1,
        ["username"] = "bobby",
        ["email"] = "bob@asdf.com",
        ["contact_name1"] = "bob",
        ["contact_name2"] = "roberta"
    }])
};
var reader = await command.ExecuteReader(
    "SELECT contact_name1, contact_name2, email, id, username FROM users");
var converter = new RowTypeConverter<User>();
var mapper = UserContactsMapper();
User? user = null;
while (await reader.Next())
    user = mapper.Map(converter.Convert(reader.Current!));
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/command.rs#L115-L146)

```rust
let command = MockCommand::new();
command.reader_rows.lock().unwrap().push(row(&[
    ("id", Value::Int(1)),
    ("username", Value::from("bobby")),
    ("email", Value::from("bob@asdf.com")),
    ("contact_name1", Value::from("bob")),
    ("contact_name2", Value::from("roberta")),
]));
let reader = command
    .execute_reader(
        "SELECT contact_name1, contact_name2, email, id, username FROM users",
        None,
    )
    .await
    .unwrap();
let converter = RowTypeConverter::of::<UserRow>();
let mapper = user_contacts_mapper();
let mut user = None;
while reader.next().await.unwrap() {
    user = Some(Mapper::map(
        &mapper,
        converter.convert(&reader.current().unwrap()).unwrap(),
    ));
}
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/Command.test.ts#L80-L103)

```typescript
const command = new MockCommand(new MockConnection());
command.reader = new MockReader([
  {
    id: 1,
    username: "bobby",
    email: "bob@asdf.com",
    contact_name1: "bob",
    contact_name2: "roberta",
  },
]);
const reader = await command.executeReader(
  "SELECT contact_name1, contact_name2, email, id, username FROM users",
);
const converter = new RowTypeConverter<User>();
const mapper = userContactsMapper();
let user: User | undefined;
while (await reader.next()) {
  user = mapper.map(converter.convert(reader.current!, User));
}
```

#### Parameter direction and Response.Parameters

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L226-L229)

```csharp
new ReturnStatement("add", [new Parameter("id", 1), new Parameter("ret", ParameterDirection.ReturnValue)]).GetText());
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1114-L1124)

```rust
);
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L325-L328)

```typescript
).toBe("CALL save($id)");
expect(
  new ReturnStatement("add", [
    { name: "id", value: 1 },
```

## Statements

Command text. [Command](#command) `Execute` / `ExecuteReader` overloads take any statement and use `GetText()` / `getText()` / `get_text()` only.

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

Bound values live on [WhereClause](#where-clause). [IStatement](#istatement) is not an [IExpression](#iexpression); [IText](#itext) is the shared `GetText()` / `getText()` / `get_text()` surface. [IWhere](#iwhere) is a clause ([IExpression](#iexpression) + [IText](#itext)), not an [IStatement](#istatement).

### See also

- [QueryStatement](#query-statement)
- [InsertStatement](#insert-statement)
- [UpdateStatement](#update-statement)
- [DeleteStatement](#delete-statement)
- [CreateTableStatement](#create-table-statement)
- [CreateIndexStatement](#create-index-statement)
- [ExecuteStoredProcedureStatement](#execute-stored-procedure-statement)
- [ReturnStatement](#return-statement)
- [WhereClause](#where-clause)
- [Expressions](#expressions)
- [StatementEvaluator](#statement-evaluator)

## Query Statement

`SELECT` text, or a compiled `SELECT` from a table, fields, optional [IWhere](#iwhere), and joins.

### Classes

##### TypeScript

##### IQuery {#typescript-iquery}

```typescript
interface IQuery extends IStatement
```

**Properties**

| Name | Description |
| --- | --- |
| where | Optional [IWhere](#typescript-iwhere) used to build this query. |

##### QueryStatement {#typescript-querystatement}

```typescript
class QueryStatement implements IQuery
```

**Constructor**

| Name | Description |
| --- | --- |
| [QueryStatement(text, where?)](#typescript-querystatement-querystatement-text-where) | Wraps query text and an optional [IWhere](#typescript-iwhere). |
| [QueryStatement(table, fields, where?, joins?)](#typescript-querystatement-querystatement-table-fields-where-joins) | Compiles a `SELECT` from a table, fields, optional [IWhere](#typescript-iwhere), and joins. |

**Properties**

| Name | Description |
| --- | --- |
| where | Optional [IWhere](#typescript-iwhere) used to build this query. |

**Methods**

| Name | Description |
| --- | --- |
| [getText()](#typescript-querystatement-gettext) | Command text. |

###### QueryStatement(text, where?) {#typescript-querystatement-querystatement-text-where}

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

###### QueryStatement(table, fields, where?, joins?) {#typescript-querystatement-querystatement-table-fields-where-joins}

Compiles a `SELECT` from a table, fields, optional [IWhere](#typescript-iwhere), and joins.

**Definition**

```typescript
constructor(table: string, fields: string[], where?: IWhere, joins?: Join[])
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | selected columns (`*` when empty) |
| `where` | optional [IWhere](#typescript-iwhere) |
| `joins` | optional [Join](#typescript-join) list |

###### getText() {#typescript-querystatement-gettext}

Command text.

**Definition**

```typescript
getText(): string
```

**Returns**

`string`

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

##### IQuery {#csharp-iquery}

```csharp
public interface IQuery : IStatement
```

**Properties**

| Name | Description |
| --- | --- |
| Where | Optional [IWhere](#csharp-iwhere) used to build this query. |

##### QueryStatement {#csharp-querystatement}

```csharp
public sealed record QueryStatement : IQuery
```

**Constructor**

| Name | Description |
| --- | --- |
| [QueryStatement(text, where)](#csharp-querystatement-querystatement-text-where) | Wraps query text and an optional [IWhere](#csharp-iwhere). |
| [QueryStatement(table, fields, where, joins)](#csharp-querystatement-querystatement-table-fields-where-joins) | Compiles a `SELECT` from a table, fields, optional [IWhere](#csharp-iwhere), and joins. |

**Properties**

| Name | Description |
| --- | --- |
| Where | Optional [IWhere](#csharp-iwhere) used to build this query. |

**Methods**

| Name | Description |
| --- | --- |
| [GetText()](#csharp-querystatement-gettext) | Command text. |

###### QueryStatement(text, where) {#csharp-querystatement-querystatement-text-where}

Wraps query text and an optional [IWhere](#csharp-iwhere).

**Definition**

```csharp
public QueryStatement(string text, IWhere? where = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where` | optional [IWhere](#csharp-iwhere) |

###### QueryStatement(table, fields, where, joins) {#csharp-querystatement-querystatement-table-fields-where-joins}

Compiles a `SELECT` from a table, fields, optional [IWhere](#csharp-iwhere), and joins.

**Definition**

```csharp
public QueryStatement(
    string table,
    IReadOnlyList<string> fields,
    IWhere? where = null,
    IReadOnlyList<Join>? joins = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | selected columns (`*` when empty) |
| `where` | optional [IWhere](#csharp-iwhere) |
| `joins` | optional [Join](#csharp-join) list |

###### GetText() {#csharp-querystatement-gettext}

Command text.

**Definition**

```csharp
public string GetText()
```

**Returns**

`string`

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

##### Query {#rust-query}

```rust
pub trait Query: Statement
```

**Methods**

| Name | Description |
| --- | --- |
| [where_clause()](#rust-query-where_clause) | Optional [Where](#rust-where) used to build this query. |

###### where_clause() {#rust-query-where_clause}

Optional [Where](#rust-where) used to build this query.

**Definition**

```rust
fn where_clause(&self) -> Option<&dyn Where>
```

**Returns**

`Option<&dyn `[Where](#rust-where)`>`

##### QueryStatement {#rust-querystatement}

```rust
pub struct QueryStatement
```

**Constructor**

| Name | Description |
| --- | --- |
| [text(text)](#rust-querystatement-text-text) | Wraps query text. |
| [with_where(text, where_clause)](#rust-querystatement-with_where-text-where_clause) | Wraps query text and a [Where](#rust-where). |
| [new(table, fields, where_clause, joins)](#rust-querystatement-new-table-fields-where_clause-joins) | Compiles a `SELECT` from a table, fields, optional [Where](#rust-where), and joins. |

**Methods**

| Name | Description |
| --- | --- |
| [get_text()](#rust-querystatement-get_text) | Command text. |
| [where_clause()](#rust-querystatement-where_clause) | Optional [Where](#rust-where) used to build this query. |

###### text(text) {#rust-querystatement-text-text}

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

[QueryStatement](#rust-querystatement)

###### with_where(text, where_clause) {#rust-querystatement-with_where-text-where_clause}

Wraps query text and a [Where](#rust-where).

**Definition**

```rust
pub fn with_where(text: impl Into<String>, where_clause: impl Where + 'static) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where_clause` | [Where](#rust-where) |

**Returns**

[QueryStatement](#rust-querystatement)

###### new(table, fields, where_clause, joins) {#rust-querystatement-new-table-fields-where_clause-joins}

Compiles a `SELECT` from a table, fields, optional [Where](#rust-where), and joins.

**Definition**

```rust
pub fn new(
    table: impl AsRef<str>,
    fields: &[&str],
    where_clause: Option<&dyn Where>,
    joins: &[Join],
) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | selected columns (`*` when empty) |
| `where_clause` | optional [Where](#rust-where) |
| `joins` | [Join](#rust-join) list |

**Returns**

[QueryStatement](#rust-querystatement)

###### get_text() {#rust-querystatement-get_text}

Command text.

**Definition**

```rust
fn get_text(&self) -> String
```

**Returns**

`String`

###### where_clause() {#rust-querystatement-where_clause}

Optional [Where](#rust-where) used to build this query.

**Definition**

```rust
fn where_clause(&self) -> Option<&dyn Where>
```

**Returns**

`Option<&dyn `[Where](#rust-where)`>`

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

[Command](#command) uses `GetText()` / `getText()` / `get_text()` only. Keep the [WhereClause](#where-clause) when the caller needs bound parameters. Empty `fields` compiles `SELECT *`.

### Examples

#### Wrap query text and an optional where

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L33-L38)

```csharp
var select = new QueryStatement("SELECT 1");
var byId = new Where("id = 1");
var withWhere = new QueryStatement("SELECT 1", byId);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1126-L1129)

```rust
assert!(QueryStatement::text("SELECT 1").where_clause().is_none());
let mock = MockQuery::with_where("q", MockWhere::new("w"));
assert!(mock.where_clause().is_some());
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L33-L35)

```typescript
} from "../index.ts";
import type { ITableConstraint, TableConstraintKind } from "../index.ts";
import type { Parameter } from "../index.ts";
```

#### Compile a SELECT

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L127-L130)

```csharp
var fields = new[] { new Parameter("id", 1), new Parameter("name", "a") };
var query = new QueryStatement("users", ["id", "name"], where);
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id AND age > $age
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L929-L935)

```rust
    "SELECT id, name FROM users WHERE id = $id AND age > $age"
);
assert!(query.where_clause().is_some());
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id AND age > $age
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L175-L180)

```typescript
    () =>
      new WhereClause(
        new Operator.In(new Identifier("id"), new ArrayExpression([new Identifier("x")])),
      ).parameters,
  ).toThrow("empty in");
});
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id
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

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L151-L152)

```csharp
var insert = new InsertStatement("users", fields);
```

**After: insert**

```
INSERT INTO users (id, name) VALUES ($id, $name)
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L975-L979)

```rust
);
let update = UpdateStatement::new("users", &fields, Some(&where_clause));
```

**After: insert**

```
INSERT INTO users (id, name) VALUES ($id, $name)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L221-L224)

```typescript
).toBe(
  "SELECT users.id FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN payments ON orders.id = payments.order_id RIGHT JOIN profiles ON users.id = profiles.user_id WHERE users.id = $users.id",
);
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
| [text(text, where_clause)](#rust-updatestatement-text-text-where_clause) | Wraps update text and an optional [Where](#rust-where). |
| [new(table, fields, where_clause)](#rust-updatestatement-new-table-fields-where_clause) | Compiles an `UPDATE`. |

**Methods**

| Name | Description |
| --- | --- |
| [get_text()](#rust-updatestatement-get_text) | Command text. |
| [where_clause()](#rust-updatestatement-where_clause) | Optional [Where](#rust-where) used to build this update. |

###### text(text, where_clause) {#rust-updatestatement-text-text-where_clause}

Wraps update text and an optional [Where](#rust-where).

**Definition**

```rust
pub fn text(text: impl Into<String>, where_clause: Option<Box<dyn Where>>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where_clause` | optional [Where](#rust-where) |

**Returns**

[UpdateStatement](#rust-updatestatement)

###### new(table, fields, where_clause) {#rust-updatestatement-new-table-fields-where_clause}

Compiles an `UPDATE`.

**Definition**

```rust
pub fn new(table: impl AsRef<str>, fields: &[Parameter], where_clause: Option<&dyn Where>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `fields` | [Parameter](#rust-parameter) columns and bound values |
| `where_clause` | optional [Where](#rust-where) |

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

Optional [Where](#rust-where) used to build this update.

**Definition**

```rust
fn where_clause(&self) -> Option<&dyn Where>
```

**Returns**

`Option<&dyn `[Where](#rust-where)`>`

### Remarks

Compiled form is `UPDATE table SET col = $col` plus `WHERE` when an [IWhere](#iwhere) is present. Keep the [WhereClause](#where-clause) when the caller needs bound parameters.

### Examples

#### Compile an UPDATE

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L153-L155)

```csharp
var update = new UpdateStatement("users", fields, where);
```

**After: update**

```
UPDATE users SET id = $id, name = $name WHERE id = $id AND age > $age
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L980-L985)

```rust
);
assert!(update.where_clause().is_some());
assert!(UpdateStatement::new("users", &fields, None)
    .where_clause()
    .is_none());
let delete = DeleteStatement::new("users", Some(&where_clause));
```

**After: update**

```
UPDATE users SET id = $id, name = $name WHERE id = $id AND age > $age
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L225-L229)

```typescript
  "SELECT id FROM users",
);
const insert = new InsertStatement("users", [...fields]);
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
| [raw(text, where_clause)](#rust-deletestatement-raw-text-where_clause) | Wraps delete text and an optional [Where](#rust-where). |

**Methods**

| Name | Description |
| --- | --- |
| [get_text()](#rust-deletestatement-get_text) | Command text. |
| [where_clause()](#rust-deletestatement-where_clause) | Optional [Where](#rust-where) used to build this delete. |

###### new(table, where_clause) {#rust-deletestatement-new-table-where_clause}

Compiles a `DELETE`.

**Definition**

```rust
pub fn new(table: impl AsRef<str>, where_clause: Option<&dyn Where>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `table` | table name |
| `where_clause` | optional [Where](#rust-where) |

**Returns**

[DeleteStatement](#rust-deletestatement)

###### raw(text, where_clause) {#rust-deletestatement-raw-text-where_clause}

Wraps delete text and an optional [Where](#rust-where).

**Definition**

```rust
pub fn raw(text: impl Into<String>, where_clause: Option<Box<dyn Where>>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | command text |
| `where_clause` | optional [Where](#rust-where) |

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

Optional [Where](#rust-where) used to build this delete.

**Definition**

```rust
fn where_clause(&self) -> Option<&dyn Where>
```

**Returns**

`Option<&dyn `[Where](#rust-where)`>`

### Remarks

Compiled form is `DELETE FROM table` plus `WHERE` when an [IWhere](#iwhere) is present. `Raw` / `raw` wraps command text without compiling. Keep the [WhereClause](#where-clause) when the caller needs bound parameters.

### Examples

#### Compile a DELETE

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L158-L160)

```csharp
var delete = new DeleteStatement("users", where);
```

**After: delete**

```
DELETE FROM users WHERE id = $id AND age > $age
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L989-L994)

```rust
);
assert!(delete.where_clause().is_some());
```

**After: delete**

```
DELETE FROM users WHERE id = $id AND age > $age
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L233-L235)

```typescript
  "UPDATE users SET id = $id, name = $name WHERE id = $id",
);
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
public sealed record Column(string Name, string Type, ColumnDefault? Default = null, NotNull? NotNull = null)
```

**Properties**

| Name | Description |
| --- | --- |
| Name | Column name. |
| Type | SQL type name. |
| Default | Optional [ColumnDefault](#csharp-columndefault). |
| NotNull | Optional [NotNull](#csharp-notnull). |

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

Compiled form is `CREATE TABLE table (name TYPE [CONSTRAINT n DEFAULT v] [CONSTRAINT n NOT NULL], …, CONSTRAINT name …)`. Table and column names are interpolated as given. Default and check values are raw SQL fragments. Indexes use [CreateIndexStatement](#create-index-statement). [Command](#command) uses `GetText()` / `getText()` / `get_text()` only.

### Examples

#### Compile a CREATE TABLE

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L162-L163)

```csharp
var createTable = new CreateTableStatement("users", [new Column("id", "INTEGER"), new Column("name", "TEXT")]);
```

**After: create table**

```
CREATE TABLE users (id INTEGER, name TEXT)
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L995-L1003)

```rust
let create_table = CreateTableStatement::new(
    "users",
    &[Column::new("id", "INTEGER"), Column::new("name", "TEXT")],
    &[],
);
```

**After: create table**

```
CREATE TABLE users (id INTEGER, name TEXT)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L239-L243)

```typescript
const del = new DeleteStatement("users", where);
```

**After: create table**

```
CREATE TABLE users (id INTEGER, name TEXT)
```

#### Compile named constraints

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L164-L177)

```csharp
var namedTable = new CreateTableStatement(
    "orders",
    [
        new Column("id", "INTEGER", new ColumnDefault("0", "df_id"), new NotNull("nn_id")),
        new Column("tenant_id", "INTEGER"),
        new Column("user_id", "INTEGER"),
        new Column("qty", "INTEGER")
    ],
    [
        new PrimaryKey("pk_orders", ["id", "tenant_id"]),
        new ForeignKey("fk_orders_user", ["user_id"], "users", ["id"], ReferentialAction.Cascade),
        new UniqueConstraint("uq_orders_user", ["user_id"]),
        new CheckConstraint("chk_orders_qty", "qty > 0")
    ]);
```

**After: named constraints**

```
CREATE TABLE orders (id INTEGER CONSTRAINT df_id DEFAULT 0 CONSTRAINT nn_id NOT NULL, tenant_id INTEGER, user_id INTEGER, qty INTEGER, CONSTRAINT pk_orders PRIMARY KEY (id, tenant_id), CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE, CONSTRAINT uq_orders_user UNIQUE (user_id), CONSTRAINT chk_orders_qty CHECK (qty > 0))
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1008-L1031)

```rust
            .default(ColumnDefault::new("0", Some("df_id")))
            .not_null(NotNull::new(Some("nn_id"))),
        Column::new("tenant_id", "INTEGER"),
        Column::new("user_id", "INTEGER"),
        Column::new("qty", "INTEGER"),
    ],
    &[
        PrimaryKey::new("pk_orders", &["id", "tenant_id"]),
        ForeignKey::new(
            "fk_orders_user",
            &["user_id"],
            "users",
            &["id"],
            Some(ReferentialAction::Cascade),
            None,
        ),
        UniqueConstraint::new("uq_orders_user", &["user_id"]),
        CheckConstraint::new("chk_orders_qty", "qty > 0"),
    ],
);
```

**After: named constraints**

```
CREATE TABLE orders (id INTEGER CONSTRAINT df_id DEFAULT 0 CONSTRAINT nn_id NOT NULL, tenant_id INTEGER, user_id INTEGER, qty INTEGER, CONSTRAINT pk_orders PRIMARY KEY (id, tenant_id), CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE, CONSTRAINT uq_orders_user UNIQUE (user_id), CONSTRAINT chk_orders_qty CHECK (qty > 0))
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L244-L258)

```typescript
);
const createTable = new CreateTableStatement("users", [
  { name: "id", type: "INTEGER" },
  { name: "name", type: "TEXT" },
]);
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
var createIndex = new CreateIndexStatement("idx_orders_user", "orders", ["user_id"]);
```

**After: create index**

```
CREATE INDEX idx_orders_user ON orders (user_id)
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1076-L1083)

```rust
let create_index = CreateIndexStatement::new("idx_orders_user", "orders", &["user_id"], false);
```

**After: create index**

```
CREATE INDEX idx_orders_user ON orders (user_id)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L300-L301)

```typescript
);
expect(() =>
```

**After: create index**

```
CREATE INDEX idx_orders_user ON orders (user_id)
```

#### Compile a CREATE UNIQUE INDEX

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L211-L213)

```csharp

```

**After: create unique index**

```
CREATE UNIQUE INDEX uq_orders_code ON orders (code)
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1085-L1087)

```rust
let call = ExecuteStoredProcedureStatement::new("save", &fields);
```

**After: create unique index**

```
CREATE UNIQUE INDEX uq_orders_code ON orders (code)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L302-L304)

```typescript
new CreateTableStatement("t", [{ name: "id", type: "INTEGER" }], [
  { kind: "nope" as TableConstraintKind, name: "x" } as ITableConstraint,
]).getText(),
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
var call = new ExecuteStoredProcedureStatement("save", fields);
```

**After: call**

```
CALL save($id, $name)
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1089-L1090)

```rust
ExecuteStoredProcedureStatement::new("save", &[]).get_text(),
"CALL save()"
```

**After: call**

```
CALL save($id, $name)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L305-L306)

```typescript
).toThrow("empty constraint");
const createIndex = new CreateIndexStatement("idx_orders_user", "orders", ["user_id"]);
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

```

**After: return**

```
SELECT add($id, $name)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L310-L311)

```typescript
);
const call = new ExecuteStoredProcedureStatement("save", [...fields]);
```

**After: return**

```
SELECT add($id, $name)
```

## Where Clause

[IWhere](#iwhere) over an [IExpression](#iexpression) tree. A clause is an expression, not a statement. Placeholders are `$field` and `$field_0` for `In`.

### Classes

##### TypeScript

##### WhereClause {#typescript-whereclause}

```typescript
class WhereClause implements IWhere
```

**Constructor**

| Name | Description |
| --- | --- |
| [WhereClause(condition)](#typescript-whereclause-whereclause-condition) | Builds a where clause from an [IExpression](#typescript-iexpression). |
| [WhereClause(column, op, value)](#typescript-whereclause-whereclause-column-op-value) | Builds a comparison from a column, a [Cmp](#typescript-cmp) alias, and a literal. |

**Properties**

| Name | Description |
| --- | --- |
| condition | Predicate tree. Typically an [Operator](#typescript-operator). |
| parameters | Bound [Parameter](#typescript-parameter) list. |

**Methods**

| Name | Description |
| --- | --- |
| [getText()](#typescript-whereclause-gettext) | Predicate text. |
| [and(column, op, value) / or(column, op, value)](#typescript-whereclause-and-column-op-value-or-column-op-value) | Nests this clause with another comparison under [Operator.And](#typescript-operator) or `Operator.Or`. |
| [map(column, value)](#typescript-whereclause-map-column-value) | Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals. |

###### WhereClause(condition) {#typescript-whereclause-whereclause-condition}

Builds a where clause from an [IExpression](#typescript-iexpression).

**Definition**

```typescript
constructor(condition: IExpression)
```

**Parameters**

| Name | Description |
| --- | --- |
| `condition` | [IExpression](#typescript-iexpression) |

###### WhereClause(column, op, value) {#typescript-whereclause-whereclause-column-op-value}

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

###### getText() {#typescript-whereclause-gettext}

Predicate text.

**Definition**

```typescript
getText(): string
```

**Returns**

`string`

###### and(column, op, value) / or(column, op, value) {#typescript-whereclause-and-column-op-value-or-column-op-value}

Nests this clause with another comparison under [Operator.And](#typescript-operator) or `Operator.Or`.

**Definition**

```typescript
and(column: string, op: Comparison, value: Cell): WhereClause
or(column: string, op: Comparison, value: Cell): WhereClause
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `op` | [Cmp](#typescript-cmp) alias |
| `value` | [Cell](#typescript-cell) |

**Returns**

[WhereClause](#typescript-whereclause)

###### map(column, value) {#typescript-whereclause-map-column-value}

Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals.

**Definition**

```typescript
map(column: (name: string) => string, value: (name: string, cell: Cell) => Cell): WhereClause
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | property name to stored column |
| `value` | property name and domain cell to stored cell |

**Returns**

[WhereClause](#typescript-whereclause)

##### IWhere {#typescript-iwhere}

```typescript
interface IWhere extends IExpression, IText {}
```

##### Where {#typescript-where}

```typescript
class Where implements IWhere
```

**Constructor**

| Name | Description |
| --- | --- |
| [Where(text)](#typescript-where-where-text) | Wraps predicate text. |

**Methods**

| Name | Description |
| --- | --- |
| [getText()](#typescript-where-gettext) | Predicate text. |

###### Where(text) {#typescript-where-where-text}

Wraps predicate text.

**Definition**

```typescript
constructor(text: string)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | predicate text |

###### getText() {#typescript-where-gettext}

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

##### WhereClause {#csharp-whereclause}

```csharp
public sealed record WhereClause(IExpression Condition) : IWhere
```

**Constructor**

| Name | Description |
| --- | --- |
| [WhereClause(condition)](#csharp-whereclause-whereclause-condition) | Builds a where clause from an [IExpression](#csharp-iexpression). |
| [WhereClause(column, op, value)](#csharp-whereclause-whereclause-column-op-value) | Builds a comparison from a column, a [Cmp](#csharp-cmp) alias, and a literal. |

**Properties**

| Name | Description |
| --- | --- |
| Condition | Predicate tree. Typically an [Operator](#csharp-operator). |
| Parameters | Bound [Parameter](#csharp-parameter) list. |

**Methods**

| Name | Description |
| --- | --- |
| [GetText()](#csharp-whereclause-gettext) | Predicate text. |
| [And(column, op, value) / Or(column, op, value)](#csharp-whereclause-and-column-op-value-or-column-op-value) | Nests this clause with another comparison under `Operator.And` or `Operator.Or`. |
| [Map(column, value)](#csharp-whereclause-map-column-value) | Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals. |

###### WhereClause(condition) {#csharp-whereclause-whereclause-condition}

Builds a where clause from an [IExpression](#csharp-iexpression).

**Definition**

```csharp
public WhereClause(IExpression Condition)
```

**Parameters**

| Name | Description |
| --- | --- |
| `condition` | [IExpression](#csharp-iexpression) |

###### WhereClause(column, op, value) {#csharp-whereclause-whereclause-column-op-value}

Builds a comparison from a column, a [Cmp](#csharp-cmp) alias, and a literal.

**Definition**

```csharp
public WhereClause(string column, Comparison op, Value? value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `op` | [Cmp](#csharp-cmp) alias such as `eq` or `gt` |
| `value` | cell value |

###### GetText() {#csharp-whereclause-gettext}

Predicate text.

**Definition**

```csharp
public string GetText()
```

**Returns**

`string`

###### And(column, op, value) / Or(column, op, value) {#csharp-whereclause-and-column-op-value-or-column-op-value}

Nests this clause with another comparison under `Operator.And` or `Operator.Or`.

**Definition**

```csharp
public WhereClause And(string column, Comparison op, Value? value)
public WhereClause Or(string column, Comparison op, Value? value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | column name |
| `op` | [Cmp](#csharp-cmp) alias |
| `value` | cell value |

**Returns**

[WhereClause](#csharp-whereclause)

###### Map(column, value) {#csharp-whereclause-map-column-value}

Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals.

**Definition**

```csharp
public WhereClause Map(Func<string, string> column, Func<string, Value, Value> value)
```

**Parameters**

| Name | Description |
| --- | --- |
| `column` | property name to stored column |
| `value` | property name and domain cell to stored cell |

**Returns**

[WhereClause](#csharp-whereclause)

##### IWhere {#csharp-iwhere}

```csharp
public interface IWhere : IExpression, IText
```

##### Where {#csharp-where}

```csharp
public sealed class Where : IWhere
```

**Constructor**

| Name | Description |
| --- | --- |
| [Where(text)](#csharp-where-where-text) | Wraps predicate text. |

**Methods**

| Name | Description |
| --- | --- |
| [GetText()](#csharp-where-gettext) | Predicate text. |

###### Where(text) {#csharp-where-where-text}

Wraps predicate text.

**Definition**

```csharp
public Where(string text)
```

**Parameters**

| Name | Description |
| --- | --- |
| `text` | predicate text |

###### GetText() {#csharp-where-gettext}

Predicate text.

**Definition**

```csharp
string GetText()
```

**Returns**

`string`

##### Rust

##### WhereClause {#rust-whereclause}

```rust
pub struct WhereClause
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(condition)](#rust-whereclause-new-condition) | Builds a where clause from an [Expression](#rust-expression). |
| [binary(column, op, value)](#rust-whereclause-binary-column-op-value) | Builds a comparison from a column, a [Cmp](#rust-cmp) alias, and a literal. |
| [and(column, op, value) / or(column, op, value)](#rust-whereclause-and-column-op-value-or-column-op-value) | Nests this clause with another comparison under `Operator::and` or `Operator::or`. |
| [map(column, value)](#rust-whereclause-map-column-value) | Rewrites identifiers through `column` and the literals beside them through `value`. The repository calls this so a clause uses stored column names. A field converter, when one is set, rewrites that property's literals. |
| [get_text()](#rust-whereclause-get_text) | Predicate text. |
| [parameters()](#rust-whereclause-parameters) | Bound [Parameter](#rust-parameter) list. |

###### new(condition) {#rust-whereclause-new-condition}

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

[WhereClause](#rust-whereclause)

###### binary(column, op, value) {#rust-whereclause-binary-column-op-value}

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

[WhereClause](#rust-whereclause)

###### and(column, op, value) / or(column, op, value) {#rust-whereclause-and-column-op-value-or-column-op-value}

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

[WhereClause](#rust-whereclause)

###### map(column, value) {#rust-whereclause-map-column-value}

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

`Result<`[WhereClause](#rust-whereclause)`>`

###### get_text() {#rust-whereclause-get_text}

Predicate text.

**Definition**

```rust
fn get_text(&self) -> String
```

**Returns**

`String`

###### parameters() {#rust-whereclause-parameters}

Bound [Parameter](#rust-parameter) list.

**Definition**

```rust
fn parameters(&self) -> Vec<Parameter>
```

**Returns**

`Vec<`[Parameter](#rust-parameter)`>`

##### Where {#rust-where}

```rust
pub trait Where: Text {}
```

##### TextWhere {#rust-textwhere}

```rust
pub struct TextWhere
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(text)](#rust-textwhere-new-text) | Wraps predicate text. |

**Methods**

| Name | Description |
| --- | --- |
| [get_text()](#rust-textwhere-get_text) | Predicate text. |

###### new(text) {#rust-textwhere-new-text}

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

[TextWhere](#rust-textwhere)

###### get_text() {#rust-textwhere-get_text}

Predicate text.

**Definition**

```rust
fn get_text(&self) -> String
```

**Returns**

`String`

### Remarks

Construct the predicate with [Operator](#operator) records, then wrap it: `new WhereClause(new Operator.Equals("id", 9))`. A column, [Cmp](#cmp) alias, and literal is the same tree: `new WhereClause("id", eq, 9)`. [QueryStatement](#query-statement) / [UpdateStatement](#update-statement) / [DeleteStatement](#delete-statement) embed `WhereClause.GetText()` in the command SQL. Keep the [WhereClause](#whereclause) when the caller needs bound parameters. [StatementEvaluator](#statement-evaluator) interprets the same tree against a parameter map. Raw text [Where](#where) is not evaluable.

### Examples

#### Compare, groups, not, and in

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L53-L55)

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

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L659-L663)

```rust
));
```

**After: and**

```
id = $id AND age > $age
```

**After: in**

```
id IN ($id_0, $id_1)
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L31-L33)

```typescript
  ge,
  le,
} from "../index.ts";
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
var equals = new WhereClause("username", eq, "grace");
var notEquals = new WhereClause("id", ne, 2);
var greater = new WhereClause("n", gt, 0);
var less = new WhereClause("n", lt, 0);
var greaterOrEqual = new WhereClause("n", ge, 0);
var lessOrEqual = new WhereClause("n", le, 0);
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

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L2490-L2496)

```rust
repo.update_bulk(
    &[
        record(&[
            ("id", Value::Int(1)),
            (
                "children",
                encode_rows(&[record(&[
```

**After: equals**

```
username = $username
```

**After: and**

```
username = $username AND is_enabled = $is_enabled
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L350-L356)

```typescript
describe("comparison aliases", () => {
  it("renders comparison symbols", () => {
    const equals = new WhereClause("username", eq, "grace");
    const notEquals = new WhereClause("id", ne, 2);
    const greater = new WhereClause("n", gt, 0);
    const less = new WhereClause("n", lt, 0);
    const greaterOrEqual = new WhereClause("n", ge, 0);
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

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L127-L130)

```csharp
var fields = new[] { new Parameter("id", 1), new Parameter("name", "a") };
var query = new QueryStatement("users", ["id", "name"], where);
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id AND age > $age
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L929-L935)

```rust
    "SELECT id, name FROM users WHERE id = $id AND age > $age"
);
assert!(query.where_clause().is_some());
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id AND age > $age
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L175-L180)

```typescript
    () =>
      new WhereClause(
        new Operator.In(new Identifier("id"), new ArrayExpression([new Identifier("x")])),
      ).parameters,
  ).toThrow("empty in");
});
```

**After: select**

```
SELECT id, name FROM users WHERE id = $id
```

## Expressions

AST nodes for [WhereClause](#where-clause). `Operator` records hold operands; `GetText()` maps them to SQL.

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

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L78-L81)

```csharp

```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L743-L758)

```rust
    .parameters()
    .len(),
    3
);
let not_where = WhereClause::new(Operator::not(WhereClause::new(Operator::equals(
    "id", 1_i64,
))));
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L63-L73)

```typescript

```

#### Column, alias, literal

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/ComparisonAliasTests.cs#L11-L17)

```csharp
var equals = new WhereClause("username", eq, "grace");
var notEquals = new WhereClause("id", ne, 2);
var greater = new WhereClause("n", gt, 0);
var less = new WhereClause("n", lt, 0);
var greaterOrEqual = new WhereClause("n", ge, 0);
var lessOrEqual = new WhereClause("n", le, 0);
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

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L2490-L2496)

```rust
repo.update_bulk(
    &[
        record(&[
            ("id", Value::Int(1)),
            (
                "children",
                encode_rows(&[record(&[
```

**After: equals**

```
username = $username
```

**After: and**

```
username = $username AND is_enabled = $is_enabled
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L350-L356)

```typescript
describe("comparison aliases", () => {
  it("renders comparison symbols", () => {
    const equals = new WhereClause("username", eq, "grace");
    const notEquals = new WhereClause("id", ne, 2);
    const greater = new WhereClause("n", gt, 0);
    const less = new WhereClause("n", lt, 0);
    const greaterOrEqual = new WhereClause("n", ge, 0);
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

Interprets an [IExpression](#expressions) against a name→value map. [WhereClause.GetText](#where-clause) materializes SQL; this type evaluates.

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

Parameters here are the row, not the SQL bind list on `IStatement.Parameters`. A raw text [Where](#where) is not evaluable.

### Examples

#### Match and miss

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/CoreTests.cs#L244-L245)

```csharp
var evaluator = new StatementEvaluator();
IReadOnlyDictionary<string, Value> row = new Dictionary<string, Value> { ["id"] = 1, ["age"] = 21, ["name"] = "a" };
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1166-L1170)

```rust
        &row
    ),
    yes
);
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/builders/builders.test.ts#L100-L101)

```typescript
});
```

## Repository

Typed item marker and instance registry. `Type` / `type` / `entity_type` is the item CLR type, constructor, or `TypeId` the repository exposes, not the table `entity` name. Register a repository, then create by that type.

Registering CRUD also registers readonly. Registering readonly-only makes CRUD `create` throw.

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

##### IRepository {#csharp-irepository}

```csharp
public interface IRepository
```

**Properties**

| Name | Description |
| --- | --- |
| Type | Item CLR type the repository exposes. |

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

SQL provider default repositories report the row or dictionary type (`Row`, `Dictionary<string, Value>`, or `Object`), not a closed entity generic.

A missing repository throws [RepositoryNotRegisteredException](#repositorynotregisteredexception) / [RepositoryNotRegisteredError](#repositorynotregisterederror) / `DatasourceError`.

See [Datasource](#datasource) for `CreateRepository` / `createRepository`.

### Examples

#### Register and create

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/RepositoryFactoryTests.cs#L12-L19)

```csharp
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

- [Readonly repository](#readonly-repository)
- [CRUD repository](#crud-repository)
- [Enriched repository](#enriched-repository)
- [Bulk repository](#bulk-repository)

## Readonly repository

Read-only typed read over a connection. `ReadSingle` / `readSingle` / `read_single` returns the first row or none. `Read` / `read` yields every row.

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
readSingle(query?: IQuery | IWhere | IExecuteStoredProcedureStatement): Promise<T | undefined>
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
read(query?: IQuery | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>
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

###### read(query?) {#typescript-readonlyrepository-read-query}

Yields every item. Table repositories convert then map; subclasses may override.

**Definition**

```typescript
abstract read(query?: IQuery | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>
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
readSingle(query?: IQuery | IWhere | IExecuteStoredProcedureStatement): Promise<T | undefined>
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
| query | Rewrites the finished select. Omitted leaves the statement unchanged. |
| rowsMapper | Maps each converted item. `Repository.mapper` is this mapper. Default is [IdentityMapper](#identity-mapper). |
| rowConverter | Row [TypeConverter](#type-converter). |
| type | Item constructor. Supplies the default entity name and column list. |
| rowTypeConverters | Provider converters keyed by `bool`, `datetime`, `uuid`, `decimal`, and `binary`, merged into the row converter. The driver applies its converters when it binds a parameter. A field converter reverse-converts that property in the repository. |
| readColumns | SELECT column names. When omitted, public fields of `type` are used. |

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
public class ReadonlyRepository<T> : IReadonlyRepository<T>
```

**Properties**

| Name | Description |
| --- | --- |
| Type | Item CLR type. `typeof(T)`. |

**Methods**

| Name | Description |
| --- | --- |
| [Read(query)](#csharp-readonlyrepository-read-query) | Yields every item. Table repositories convert then map; subclasses may override. |
| [ReadSingle(query)](#csharp-readonlyrepository-readsingle-query) | Returns the first item from [Read](#csharp-readonlyrepository-read-query), or `default`. |

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
| Query | Rewrites the finished select. Omitted leaves the statement unchanged. |
| RowsMapper | Maps each converted item. `Repository.Mapper` is this mapper. Default is [IdentityMapper](#identity-mapper). |
| RowConverter | Row [TypeConverter](#type-converter). |
| RowTypeConverters | Provider converters keyed by `bool`, `datetime`, `uuid`, `decimal`, and `binary`, merged into the row converter. The driver applies its converters when it binds a parameter. A field converter reverse-converts that property in the repository. |
| ReadColumns | SELECT column names. When omitted, public properties of `T` are used. |

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
| query | Rewrites finished select SQL. `None` leaves the statement unchanged. |
| rows_mapper | Maps each converted row. Default is [IdentityMapper](#identity-mapper). |
| row_converter | Row [RowTypeConverter](#row-type-converter). |
| row_type_converters | Provider converters keyed by `bool`, `datetime`, `uuid`, `decimal`, and `binary`, merged into the row converter. The driver applies its converters when it binds a parameter. A field converter reverse-converts that property in the repository. |

### Remarks

Repository read is convert, then map. See [TypeConverter](#type-converter) and [Mapper](#mapper).

`read` and `read_single` accept a query, a where, or an [ExecuteStoredProcedureStatement](#execute-stored-procedure-statement). Result rows still convert, then map. Procedure argument names are not rewritten as columns.

### Examples

#### Read users by id and by username

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L28-L58)

```csharp
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["filename"] = ":memory:" });
var connection = await Open(ds, """
    CREATE TABLE users (
        id INTEGER PRIMARY KEY,
        username TEXT NOT NULL,
        is_enabled INTEGER NOT NULL
    );
    INSERT INTO users (id, username, is_enabled) VALUES
        (1, 'ada', 1),
        (2, 'grace', 0);
    """);
var repo = ds.CreateReadonlyRepository<User>(connection, new ReadonlyRepositoryOptions<User>
{
    EntityMapper = new PluralizerMapper(),
});

var byId = await repo.ReadSingle(new WhereClause("id", eq, 1));
var disabled = new List<User>();
await foreach (var row in repo.Read(new WhereClause("username", eq, "grace").And("is_enabled", eq, 0)))
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

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L47-L86)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
let connection = open(
    ds,
    "
    CREATE TABLE users (
        id INTEGER PRIMARY KEY,
        username TEXT NOT NULL,
        is_enabled INTEGER NOT NULL
    );
    INSERT INTO users (id, username, is_enabled) VALUES
        (1, 'ada', 1),
        (2, 'grace', 0);
    ",
)
.await;
let mut options = create_repository_options::<User>();
options.entity_mapper = Box::new(PluralizerMapper::new());
let repo = ds.create_readonly_repository::<User>(Some(connection), Some(options.read));
let by_id = repo
    .read_single(Some(&WhereClause::binary("id", eq, 1_i64)))
    .await
    .unwrap()
    .unwrap();
let disabled = repo
    .read(Some(&WhereClause::binary("username", eq, "grace").and(
        "is_enabled",
        eq,
        0_i64,
    )))
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

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L32-L56)

```typescript
const ds = new Datasource("sqlite", new SqliteDriver(), { filename: ":memory:" });
const connection = await open(ds, `
  CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL,
    is_enabled INTEGER NOT NULL
  );
  INSERT INTO users (id, username, is_enabled) VALUES
    (1, 'ada', 1),
    (2, 'grace', 0);
`);
const repo = ds.createReadonlyRepository(User, connection, {
  entityMapper: new PluralizerMapper(),
});
const byId = await repo.readSingle(new WhereClause("id", eq, 1));
const disabled: User[] = [];
for await (const row of repo.read(new WhereClause("username", eq, "grace").and("is_enabled", eq, 0)))
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

#### Read a user by guid, username, and created

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L59-L102)

```csharp
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["filename"] = ":memory:" });
var connection = await Open(ds, $"""
    CREATE TABLE user (
        id TEXT PRIMARY KEY,
        username TEXT NOT NULL,
        is_enabled INTEGER NOT NULL,
        created TEXT NOT NULL
    );
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
var byId = await repo.ReadSingle(new WhereClause("Key", eq, Ada));
var byLogin = new List<UserAccount>();
await foreach (var row in repo.Read(new WhereClause("Login", eq, "ada").And("Active", eq, true)))
    byLogin.Add(row);
var later = new List<UserAccount>();
await foreach (var row in repo.Read(new WhereClause("Active", eq, true).And("Opened", gt, opened)))
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

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L88-L152)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
let connection = open(
    ds,
    &format!(
        "
    CREATE TABLE user (
        id TEXT PRIMARY KEY,
        username TEXT NOT NULL,
        is_enabled INTEGER NOT NULL,
        created TEXT NOT NULL
    );
    INSERT INTO user (id, username, is_enabled, created) VALUES
        ('{ADA}', 'ada', 1, '2020-01-01T00:00:00.000Z'),
        ('{GRACE}', 'grace', 1, '2024-06-01T00:00:00.000Z'),
        ('{LIN}', 'lin', 0, '2024-06-01T00:00:00.000Z');
    "
    ),
)
.await;
let mut options = create_repository_options::<UserAccount>();
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
    .read_single(Some(&WhereClause::binary(
        "Key",
        eq,
        Uuid::parse_str(ADA).unwrap(),
    )))
    .await
    .unwrap()
    .unwrap();
let by_login = repo
    .read(Some(
        &WhereClause::binary("Login", eq, "ada").and("Active", eq, true),
    ))
    .await
    .unwrap();
let later = repo
    .read(Some(
        &WhereClause::binary("Active", eq, true).and("Opened", gt, opened),
    ))
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

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L58-L97)

```typescript
const ds = new Datasource("sqlite", new SqliteDriver(), { filename: ":memory:" });
const connection = await open(ds, `
  CREATE TABLE user (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL,
    is_enabled INTEGER NOT NULL,
    created TEXT NOT NULL
  );
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
const byId = await repo.readSingle(new WhereClause("Key", eq, ada));
const byLogin: UserAccount[] = [];
for await (const row of repo.read(new WhereClause("Login", eq, "ada").and("Active", eq, true)))
  byLogin.push(row);
const later: UserAccount[] = [];
for await (const row of repo.read(new WhereClause("Active", eq, true).and("Opened", gt, opened)))
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

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/RepositoryTests.cs#L99-L108)

```csharp
var call = new ExecuteStoredProcedureStatement("get_user", [new Parameter("id", 9)]);
var rows = new List<UserRow>();
await foreach (var row in repo.Read(call))
    rows.Add(row);
```

**After: rows**

```
[
  { id: 1, name: 'a' }
]
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1428-L1444)

```rust
let call = ExecuteStoredProcedureStatement::new("get_user", &[Parameter::new("id", 9_i64)]);
let rows = repo.read(Some(&call)).await.unwrap();
```

**After: rows**

```
[
  { id: 1, name: 'a' }
]
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/repository.test.ts#L102-L109)

```typescript
const call = new ExecuteStoredProcedureStatement("get_user", [{ name: "id", value: 9 }]);
const rows: Cells[] = [];
for await (const row of repo.read(call)) rows.push(row);
```

**After: rows**

```
[
  { id: 1, name: 'a' }
]
```

#### Read and read single

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/RepositoryTests.cs#L61-L71)

```csharp
["id"] = 1, ["name"] = "a" 
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

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1465-L1471)

```rust
let rows = repo.read(None).await.unwrap();
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

## CRUD repository

[Readonly repository](#readonly-repository) plus `create`, `update`, and `delete`. SQL providers typically build a table `CRUDRepository` from `RepositoryBuilder` and a [RowTypeConverter](#row-type-converter).

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

##### CRUDRepository {#typescript-crudrepository}

```typescript
class CRUDRepository<T = Cells> extends ReadonlyRepository<T> implements ICRUDRepository<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [CRUDRepository(type, connection)](#typescript-crudrepository-crudrepository-type-connection) | Builds a table repository from a type and [IConnection](#connection). |
| [CRUDRepository(connection, options)](#typescript-crudrepository-crudrepository-connection-options) | Builds a table repository from an [IConnection](#connection) and [CRUDRepositoryOptions](#typescript-crudrepositoryoptions). |
| [CRUDRepository(built)](#typescript-crudrepository-crudrepository-built) | Builds a table repository from [BuiltRepository](#typescript-builtrepository). |

**Properties**

| Name | Description |
| --- | --- |
| rowConverter | Row [TypeConverter](#type-converter). Convert uses this, then [mapper](#typescript-repository-mapper). |
| mapper | Read-row [Mapper](#mapper). This is [ReadonlyRepositoryOptions.rowsMapper](#typescript-rowsmapper). |
| statements | SQL fragments for query / insert / update / delete. |

**Methods**

| Name | Description |
| --- | --- |
| [addMiddleware(middleware)](#typescript-crudrepository-addmiddleware-middleware) | Adds command middleware. Returns this repository. |
| [read(query?)](#typescript-crudrepository-read-query) | Reads rows through [rowConverter](#typescript-repository-rowconverter) then [mapper](#typescript-repository-mapper). |
| [create(item)](#typescript-crudrepository-create-item) | Inserts one item. |
| [update(item, byWhere?)](#typescript-crudrepository-update-item-bywhere) | Updates one item. |
| [delete(item, byWhere?)](#typescript-crudrepository-delete-item-bywhere) | Deletes one item. |

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

###### CRUDRepository(built) {#typescript-crudrepository-crudrepository-built}

Builds a table repository from [BuiltRepository](#typescript-builtrepository).

**Definition**

```typescript
constructor(built: BuiltRepository<T>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `built` | [BuiltRepository](#typescript-builtrepository) |

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

Reads rows through [rowConverter](#typescript-repository-rowconverter) then [mapper](#typescript-repository-mapper).

**Definition**

```typescript
read(query?: IQuery | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>
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
| create | Maps `Response.result` onto the created item. Default writes the value to `id`. |

##### BuiltRepository {#typescript-builtrepository}

```typescript
type BuiltRepository<T = Cells>
```

**Properties**

| Name | Description |
| --- | --- |
| connection | Open [IConnection](#connection). |
| rowConverter | Row [TypeConverter](#type-converter). |
| rowsMapper | Read-row [Mapper](#mapper). |
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

##### CRUDRepository {#csharp-crudrepository}

```csharp
public sealed class CRUDRepository<T> : ReadonlyRepository<T>, ICRUDRepository<T> where T : new()
```

**Constructor**

| Name | Description |
| --- | --- |
| [CRUDRepository(connection)](#csharp-crudrepository-crudrepository-connection) | Builds a table repository from an [IConnection](#connection). |
| [CRUDRepository(connection, options)](#csharp-crudrepository-crudrepository-connection-options) | Builds a table repository from an [IConnection](#connection) and [CRUDRepositoryOptions](#csharp-crudrepositoryoptions). |
| [CRUDRepository(built)](#csharp-crudrepository-crudrepository-built) | Builds a table repository from [BuiltRepository](#csharp-builtrepository). |

**Properties**

| Name | Description |
| --- | --- |
| Mapper | Read-row [Mapper](#mapper). This is [ReadonlyRepositoryOptions.RowsMapper](#csharp-rowsmapper). |

**Methods**

| Name | Description |
| --- | --- |
| [AddMiddleware(middleware)](#csharp-crudrepository-addmiddleware-middleware) | Adds command middleware. Returns this repository. |
| [Read(query)](#csharp-crudrepository-read-query) | Reads rows through the row converter then [Mapper](#csharp-repository-mapper). |
| [Create(item)](#csharp-crudrepository-create-item) | Inserts one item. |
| [Update(item, byWhere)](#csharp-crudrepository-update-item-bywhere) | Updates one item. |
| [Delete(item, byWhere)](#csharp-crudrepository-delete-item-bywhere) | Deletes one item. |

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

###### CRUDRepository(built) {#csharp-crudrepository-crudrepository-built}

Builds a table repository from [BuiltRepository](#csharp-builtrepository).

**Definition**

```csharp
public Repository(BuiltRepository<T> built)
```

**Parameters**

| Name | Description |
| --- | --- |
| `built` | [BuiltRepository](#csharp-builtrepository) |

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

Reads rows through the row converter then [Mapper](#csharp-repository-mapper).

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
| Create | Maps `Response.Result` onto the created item. Default writes the value to `id` or `Id`. |

##### BuiltRepository {#csharp-builtrepository}

```csharp
public sealed record BuiltRepository<T> where T : new()
```

**Properties**

| Name | Description |
| --- | --- |
| Connection | Open [IConnection](#connection). |
| RowConverter | Row [TypeConverter](#type-converter). |
| RowsMapper | Read-row [Mapper](#mapper). |
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
async fn update(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response>
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
async fn delete(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | row to delete |
| `by_where` | optional predicate; omitted uses id values on `item` |

**Returns**

`Result<Response>`

##### CRUDRepository {#rust-crudrepository}

```rust
pub struct CRUDRepository
```

**Constructor**

| Name | Description |
| --- | --- |
| [from_type(connection)](#rust-crudrepository-from_type-connection) | Builds a table repository from `T: ConvertSchema`. |
| [new(connection, options)](#rust-crudrepository-new-connection-options) | Builds a table repository from a [Connection](#connection) and [CRUDRepositoryOptions](#rust-crudrepositoryoptions). |
| [from_built(built)](#rust-crudrepository-from_built-built) | Builds a table repository from [BuiltRepository](#rust-builtrepository). |

**Methods**

| Name | Description |
| --- | --- |
| [add_middleware(middleware)](#rust-crudrepository-add_middleware-middleware) | Adds command middleware. |

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

###### from_built(built) {#rust-crudrepository-from_built-built}

Builds a table repository from [BuiltRepository](#rust-builtrepository).

**Definition**

```rust
pub fn from_built(built: BuiltRepository) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `built` | [BuiltRepository](#rust-builtrepository) |

**Returns**

[CRUDRepository](#rust-crudrepository)

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
| create | Maps `Response.result` onto the created row. Default writes the value to `id`. |
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
| rows_mapper | Read-row [Mapper](#mapper). |
| statements | SQL fragments. |
| ids | Id column names. |

### Remarks

Update requires at least one non-id field. Missing id values throw. Insert always omits configured id columns. After execute, `Create` / `create` maps `Response.Result` onto the created item.

`Mapper` / `mapper` is [ReadonlyRepositoryOptions.RowsMapper](#rowsmapper).

### Examples

#### Create, update, and delete

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/RepositoryTests.cs#L100-L108)

```csharp
var rows = new List<UserRow>();
await foreach (var row in repo.Read(call))
    rows.Add(row);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1468-L1482)

```rust
    command.reader_logs.lock().unwrap()[0].0,
    "SELECT id, name FROM users"
);
assert!(repo.read_single(None).await.unwrap().is_some());

let where_clause = WhereClause::new(Operator::equals("id", 9_i64));
let query = QueryStatement::new("users", &["id"], Some(&where_clause), &[]);
command.reader_rows.lock().unwrap().clear();
assert!(repo.read_single(Some(&query)).await.unwrap().is_none());
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/repository.test.ts#L103-L113)

```typescript
id: 1, name: "a" 
```

## Enriched repository

A read-only wrapper fills properties already on `T` from related repositories. This is a post-Read step (after convert-then-map). It does not join or collapse rows.

Lookups use `ReadSingle` / `read_single` (cached per distinct key). Lists use `Read` / `read`. Missing lookups leave the destination default.

`EnrichedCRUDRepository` is the write side of the same tree. `Read` still enriches. `select` lookups and 1:1 embeds are not written. Each `many` node must be a CRUD repository.

- **Create** — write the parent, apply generated identity to `id`, stamp each child's `by` from the parent `from`, then create children (and nested many).
- **Update** — write the parent, then replace-sync each many list (delete missing, update matches by `id`, create the rest).
- **Delete** — delete nested many rows first, then children, then the parent. Loads existing children by FK; does not trust payload lists.

See [Bulk repository](#bulk-repository) for list writes.

### Classes

##### TypeScript

##### IEnriched {#typescript-ienriched}

```typescript
interface IEnriched<T>
```

**Methods**

| Name | Description |
| --- | --- |
| [addEnriched(into, from, via, by, select?, many?, nest?)](#typescript-ienriched-addenriched-into-from-via-by-select-many-nest) | Adds a related fill. Fluent for siblings; `nest` receives the child node. |
| [enrich(items)](#typescript-ienriched-enrich-items) | Fills `items` in place from the tree. |

###### addEnriched(into, from, via, by, select?, many?, nest?) {#typescript-ienriched-addenriched-into-from-via-by-select-many-nest}

Adds a related fill. Fluent for siblings; `nest` receives the child node.

**Definition**

```typescript
addEnriched<TRelated extends object>(into: keyof T & string, from: keyof T & string, via: IReadonlyRepository<TRelated>, by: keyof TRelated & string, select?: keyof TRelated & string, many?: boolean, nest?: (node: IRepositoryTreeNode) => void): this
```

**Parameters**

| Name | Description |
| --- | --- |
| `into` | destination property on `T` |
| `from` | key on the parent |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `by` | key on the related item |
| `select` | optional related property to copy; omitted copies the whole row |
| `many` | collect a list; default `false` |
| `nest` | optional child-node callback |

**Returns**

`this`

###### enrich(items) {#typescript-ienriched-enrich-items}

Fills `items` in place from the tree.

**Definition**

```typescript
enrich(items: T[]): Promise<T[]>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to fill |

**Returns**

`Promise<T[]>`

##### EnrichedReadonlyRepository {#typescript-enrichedreadonlyrepository}

```typescript
class EnrichedReadonlyRepository<T extends object> extends ReadonlyRepository<T> implements IEnriched<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [EnrichedReadonlyRepository(inner, strategy?)](#typescript-enrichedreadonlyrepository-enrichedreadonlyrepository-inner-strategy) | Wraps an inner [IReadonlyRepository](#readonly-repository). Omitted strategy uses the default. |

**Properties**

| Name | Description |
| --- | --- |
| tree | Enrichment [IRepositoryTree](#typescript-irepositorytree). |

**Methods**

| Name | Description |
| --- | --- |
| [addEnriched(into, from, via, by, select?, many?, nest?)](#typescript-enrichedreadonlyrepository-addenriched-into-from-via-by-select-many-nest) | Adds a related fill. |
| [enrich(items)](#typescript-enrichedreadonlyrepository-enrich-items) | Fills `items` in place from the tree. |
| [read(query?)](#typescript-enrichedreadonlyrepository-read-query) | Reads through the inner repository, then enriches. |

###### EnrichedReadonlyRepository(inner, strategy?) {#typescript-enrichedreadonlyrepository-enrichedreadonlyrepository-inner-strategy}

Wraps an inner [IReadonlyRepository](#readonly-repository). Omitted strategy uses the default.

**Definition**

```typescript
constructor(inner: IReadonlyRepository<T>, strategy?: IReadonlyEnrichmentStrategy<T>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [IReadonlyRepository](#readonly-repository) |
| `strategy` | optional [IReadonlyEnrichmentStrategy](#typescript-ireadonlyenrichmentstrategy) |

###### addEnriched(into, from, via, by, select?, many?, nest?) {#typescript-enrichedreadonlyrepository-addenriched-into-from-via-by-select-many-nest}

Adds a related fill.

**Definition**

```typescript
addEnriched<TRelated extends object>(into: keyof T & string, from: keyof T & string, via: IReadonlyRepository<TRelated>, by: keyof TRelated & string, select?: keyof TRelated & string, many?: boolean, nest?: (node: IRepositoryTreeNode) => void): this
```

**Parameters**

| Name | Description |
| --- | --- |
| `into` | destination property on `T` |
| `from` | key on the parent |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `by` | key on the related item |
| `select` | optional related property to copy |
| `many` | collect a list |
| `nest` | optional child-node callback |

**Returns**

`this`

###### enrich(items) {#typescript-enrichedreadonlyrepository-enrich-items}

Fills `items` in place from the tree.

**Definition**

```typescript
enrich(items: T[]): Promise<T[]>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to fill |

**Returns**

`Promise<T[]>`

###### read(query?) {#typescript-enrichedreadonlyrepository-read-query}

Reads through the inner repository, then enriches.

**Definition**

```typescript
read(query?: IQuery | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional query, where, or stored procedure |

**Returns**

`AsyncIterable<T>`

##### EnrichedCRUDRepository {#typescript-enrichedcrudrepository}

```typescript
class EnrichedCRUDRepository<T extends object> extends EnrichedReadonlyRepository<T> implements ICRUDRepository<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [EnrichedCRUDRepository(inner, strategy?)](#typescript-enrichedcrudrepository-enrichedcrudrepository-inner-strategy) | Wraps an inner [ICRUDRepository](#crud-repository). Omitted strategy uses the default. |

**Methods**

| Name | Description |
| --- | --- |
| [create(item)](#typescript-enrichedcrudrepository-create-item) | Writes the parent, then nested `many` children. |
| [update(item, byWhere?)](#typescript-enrichedcrudrepository-update-item-bywhere) | Writes the parent, then replace-syncs nested `many` lists. |
| [delete(item, byWhere?)](#typescript-enrichedcrudrepository-delete-item-bywhere) | Deletes nested `many` rows, then the parent. |

###### EnrichedCRUDRepository(inner, strategy?) {#typescript-enrichedcrudrepository-enrichedcrudrepository-inner-strategy}

Wraps an inner [ICRUDRepository](#crud-repository). Omitted strategy uses the default.

**Definition**

```typescript
constructor(inner: ICRUDRepository<T>, strategy?: IReadonlyEnrichmentStrategy<T>)
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [ICRUDRepository](#crud-repository) |
| `strategy` | optional [IReadonlyEnrichmentStrategy](#typescript-ireadonlyenrichmentstrategy) |

###### create(item) {#typescript-enrichedcrudrepository-create-item}

Writes the parent, then nested `many` children.

**Definition**

```typescript
create(item: T): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | parent to insert |

**Returns**

`Promise<Response>`

###### update(item, byWhere?) {#typescript-enrichedcrudrepository-update-item-bywhere}

Writes the parent, then replace-syncs nested `many` lists.

**Definition**

```typescript
update(item: T, byWhere?: IWhere): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | parent to update |
| `byWhere` | optional predicate |

**Returns**

`Promise<Response>`

###### delete(item, byWhere?) {#typescript-enrichedcrudrepository-delete-item-bywhere}

Deletes nested `many` rows, then the parent.

**Definition**

```typescript
delete(item: T, byWhere?: IWhere): Promise<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | parent to delete |
| `byWhere` | optional predicate |

**Returns**

`Promise<Response>`

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
| enrichment | [Enrichment](#typescript-enrichment) for this node. |

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

##### IReadonlyEnrichmentStrategy {#typescript-ireadonlyenrichmentstrategy}

```typescript
interface IReadonlyEnrichmentStrategy<T extends object> extends IReadonlyEnrichmentBinder
```

**Methods**

| Name | Description |
| --- | --- |
| [read(inner, tree, query?)](#typescript-ireadonlyenrichmentstrategy-read-inner-tree-query) | Reads through `inner`, then enriches. |
| [enrich(tree, items)](#typescript-ireadonlyenrichmentstrategy-enrich-tree-items) | Fills `items` from the tree. |

###### read(inner, tree, query?) {#typescript-ireadonlyenrichmentstrategy-read-inner-tree-query}

Reads through `inner`, then enriches.

**Definition**

```typescript
read(inner: IReadonlyRepository<T>, tree: IRepositoryTree, query?: IQuery | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [IReadonlyRepository](#readonly-repository) |
| `tree` | [IRepositoryTree](#typescript-irepositorytree) |
| `query` | optional query, where, or stored procedure |

**Returns**

`AsyncIterable<T>`

###### enrich(tree, items) {#typescript-ireadonlyenrichmentstrategy-enrich-tree-items}

Fills `items` from the tree.

**Definition**

```typescript
enrich(tree: IRepositoryTree, items: T[]): Promise<void>
```

**Parameters**

| Name | Description |
| --- | --- |
| `tree` | [IRepositoryTree](#typescript-irepositorytree) |
| `items` | parents to fill |

##### C#

##### IEnriched {#csharp-ienriched}

```csharp
public interface IEnriched<T>
```

**Methods**

| Name | Description |
| --- | --- |
| [AddEnriched(into, from, via, by, select, many, nest)](#csharp-ienriched-addenriched-into-from-via-by-select-many-nest) | Adds a related fill. Fluent for siblings; `nest` receives the child node. |
| [Enrich(items)](#csharp-ienriched-enrich-items) | Fills `items` in place from the tree. |

###### AddEnriched(into, from, via, by, select, many, nest) {#csharp-ienriched-addenriched-into-from-via-by-select-many-nest}

Adds a related fill. Fluent for siblings; `nest` receives the child node.

**Definition**

```csharp
IEnriched<T> AddEnriched<TRelated>(string into, string from, IReadonlyRepository<TRelated> via, string by, string? select = null, bool many = false, Action<IRepositoryTreeNode>? nest = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `into` | destination property on `T` |
| `from` | key on the parent |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `by` | key on the related item |
| `select` | optional related property to copy; omitted copies the whole row |
| `many` | collect a list |
| `nest` | optional child-node callback |

**Returns**

[IEnriched](#csharp-ienriched)`<T>`

###### Enrich(items) {#csharp-ienriched-enrich-items}

Fills `items` in place from the tree.

**Definition**

```csharp
Task Enrich(IList<T> items)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to fill |

##### EnrichedReadonlyRepository {#csharp-enrichedreadonlyrepository}

```csharp
public class EnrichedReadonlyRepository<T> : ReadonlyRepository<T>, IEnriched<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [EnrichedReadonlyRepository(inner, strategy)](#csharp-enrichedreadonlyrepository-enrichedreadonlyrepository-inner-strategy) | Wraps an inner [IReadonlyRepository](#readonly-repository). Omitted strategy uses the default. |

**Properties**

| Name | Description |
| --- | --- |
| Tree | Enrichment [IRepositoryTree](#csharp-irepositorytree). |

**Methods**

| Name | Description |
| --- | --- |
| [AddEnriched(into, from, via, by, select, many, nest)](#csharp-enrichedreadonlyrepository-addenriched-into-from-via-by-select-many-nest) | Adds a related fill. |
| [Enrich(items)](#csharp-enrichedreadonlyrepository-enrich-items) | Fills `items` in place from the tree. |
| [Read(query)](#csharp-enrichedreadonlyrepository-read-query) | Reads through the inner repository, then enriches. |

###### EnrichedReadonlyRepository(inner, strategy) {#csharp-enrichedreadonlyrepository-enrichedreadonlyrepository-inner-strategy}

Wraps an inner [IReadonlyRepository](#readonly-repository). Omitted strategy uses the default.

**Definition**

```csharp
public EnrichedReadonlyRepository(IReadonlyRepository<T> inner, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [IReadonlyRepository](#readonly-repository) |
| `strategy` | optional [IReadonlyEnrichmentStrategy](#csharp-ireadonlyenrichmentstrategy) |

###### AddEnriched(into, from, via, by, select, many, nest) {#csharp-enrichedreadonlyrepository-addenriched-into-from-via-by-select-many-nest}

Adds a related fill.

**Definition**

```csharp
public IEnriched<T> AddEnriched<TRelated>(string into, string from, IReadonlyRepository<TRelated> via, string by, string? select = null, bool many = false, Action<IRepositoryTreeNode>? nest = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `into` | destination property on `T` |
| `from` | key on the parent |
| `via` | related [IReadonlyRepository](#readonly-repository) |
| `by` | key on the related item |
| `select` | optional related property to copy |
| `many` | collect a list |
| `nest` | optional child-node callback |

**Returns**

[IEnriched](#csharp-ienriched)`<T>`

###### Enrich(items) {#csharp-enrichedreadonlyrepository-enrich-items}

Fills `items` in place from the tree.

**Definition**

```csharp
public Task Enrich(IList<T> items)
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to fill |

###### Read(query) {#csharp-enrichedreadonlyrepository-read-query}

Reads through the inner repository, then enriches.

**Definition**

```csharp
public override IAsyncEnumerable<T> Read(IStatement? query = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `query` | optional statement or where |

**Returns**

`IAsyncEnumerable<T>`

##### EnrichedCRUDRepository {#csharp-enrichedcrudrepository}

```csharp
public class EnrichedCRUDRepository<T> : EnrichedReadonlyRepository<T>, ICRUDRepository<T>
```

**Constructor**

| Name | Description |
| --- | --- |
| [EnrichedCRUDRepository(inner, strategy)](#csharp-enrichedcrudrepository-enrichedcrudrepository-inner-strategy) | Wraps an inner [ICRUDRepository](#crud-repository). Omitted strategy uses the default. |

**Methods**

| Name | Description |
| --- | --- |
| [Create(item)](#csharp-enrichedcrudrepository-create-item) | Writes the parent, then nested `many` children. |
| [Update(item, byWhere)](#csharp-enrichedcrudrepository-update-item-bywhere) | Writes the parent, then replace-syncs nested `many` lists. |
| [Delete(item, byWhere)](#csharp-enrichedcrudrepository-delete-item-bywhere) | Deletes nested `many` rows, then the parent. |

###### EnrichedCRUDRepository(inner, strategy) {#csharp-enrichedcrudrepository-enrichedcrudrepository-inner-strategy}

Wraps an inner [ICRUDRepository](#crud-repository). Omitted strategy uses the default.

**Definition**

```csharp
public EnrichedCRUDRepository(ICRUDRepository<T> inner, IReadonlyEnrichmentStrategy<T>? strategy = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [ICRUDRepository](#crud-repository) |
| `strategy` | optional [IReadonlyEnrichmentStrategy](#csharp-ireadonlyenrichmentstrategy) |

###### Create(item) {#csharp-enrichedcrudrepository-create-item}

Writes the parent, then nested `many` children.

**Definition**

```csharp
public Task<Response> Create(T item)
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | parent to insert |

**Returns**

`Task<Response>`

###### Update(item, byWhere) {#csharp-enrichedcrudrepository-update-item-bywhere}

Writes the parent, then replace-syncs nested `many` lists.

**Definition**

```csharp
public Task<Response> Update(T item, IWhere? byWhere = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | parent to update |
| `byWhere` | optional predicate |

**Returns**

`Task<Response>`

###### Delete(item, byWhere) {#csharp-enrichedcrudrepository-delete-item-bywhere}

Deletes nested `many` rows, then the parent.

**Definition**

```csharp
public Task<Response> Delete(T item, IWhere? byWhere = null)
```

**Parameters**

| Name | Description |
| --- | --- |
| `item` | parent to delete |
| `byWhere` | optional predicate |

**Returns**

`Task<Response>`

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
| Enrichment | [Enrichment](#csharp-enrichment) for this node. |

**Methods**

| Name | Description |
| --- | --- |
| [Include(via, into, from, by, select, many)](#csharp-irepositorytreenode-include-via-into-from-by-select-many) | Adds a child node. |
| [EnrichFrom(parents)](#csharp-irepositorytreenode-enrichfrom-parents) | Fills `parents` from this node. |

###### Include(via, into, from, by, select, many) {#csharp-irepositorytreenode-include-via-into-from-by-select-many}

Adds a child node.

**Definition**

```csharp
IRepositoryTreeNode Include<TRelated>(IReadonlyRepository<TRelated> via, string into, string from, string by, string? select = null, bool many = false)
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

[IRepositoryTreeNode](#csharp-irepositorytreenode)

###### EnrichFrom(parents) {#csharp-irepositorytreenode-enrichfrom-parents}

Fills `parents` from this node.

**Definition**

```csharp
Task EnrichFrom(IReadOnlyList<object> parents)
```

**Parameters**

| Name | Description |
| --- | --- |
| `parents` | parent objects |

##### Enrichment {#csharp-enrichment}

```csharp
public sealed record Enrichment
```

**Properties**

| Name | Description |
| --- | --- |
| Into | Destination property. |
| From | Parent key. |
| By | Related key. |
| Select | Optional related property to copy. |
| Many | Collect a list. |

##### IReadonlyEnrichmentStrategy {#csharp-ireadonlyenrichmentstrategy}

```csharp
public interface IReadonlyEnrichmentStrategy<T> : IReadonlyEnrichmentBinder
```

**Methods**

| Name | Description |
| --- | --- |
| [Read(inner, tree, query)](#csharp-ireadonlyenrichmentstrategy-read-inner-tree-query) | Reads through `inner`, then enriches. |
| [Enrich(tree, items)](#csharp-ireadonlyenrichmentstrategy-enrich-tree-items) | Fills `items` from the tree. |

###### Read(inner, tree, query) {#csharp-ireadonlyenrichmentstrategy-read-inner-tree-query}

Reads through `inner`, then enriches.

**Definition**

```csharp
IAsyncEnumerable<T> Read(IReadonlyRepository<T> inner, IRepositoryTree tree, IStatement? query)
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [IReadonlyRepository](#readonly-repository) |
| `tree` | [IRepositoryTree](#csharp-irepositorytree) |
| `query` | optional statement or where |

**Returns**

`IAsyncEnumerable<T>`

###### Enrich(tree, items) {#csharp-ireadonlyenrichmentstrategy-enrich-tree-items}

Fills `items` from the tree.

**Definition**

```csharp
Task Enrich(IRepositoryTree tree, IList<T> items)
```

**Parameters**

| Name | Description |
| --- | --- |
| `tree` | [IRepositoryTree](#csharp-irepositorytree) |
| `items` | parents to fill |

##### Rust

##### Enriched {#rust-enriched}

```rust
pub trait Enriched
```

**Methods**

| Name | Description |
| --- | --- |
| [enrich(items)](#rust-enriched-enrich-items) | Fills `items` in place from the tree. |

###### enrich(items) {#rust-enriched-enrich-items}

Fills `items` in place from the tree.

**Definition**

```rust
async fn enrich(&self, items: &mut [Row]) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to fill |

**Returns**

`Result<()>`

##### EnrichedReadonlyRepository {#rust-enrichedreadonlyrepository}

```rust
pub struct EnrichedReadonlyRepository
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(inner)](#rust-enrichedreadonlyrepository-new-inner) | Wraps an inner [ReadonlyRepository](#readonly-repository). Uses the default strategy. |
| [with_strategy(inner, strategy)](#rust-enrichedreadonlyrepository-with_strategy-inner-strategy) | Wraps an inner repository with a [ReadonlyEnrichmentStrategy](#rust-readonlyenrichmentstrategy). |

**Properties**

| Name | Description |
| --- | --- |
| tree | Enrichment [RepositoryTree](#rust-repositorytree). |

**Methods**

| Name | Description |
| --- | --- |
| [add_enriched(into, from, via, by, select, many, nest)](#rust-enrichedreadonlyrepository-add_enriched-into-from-via-by-select-many-nest) | Adds a related fill. Pass `()` for `nest` when the child has no further fill. |

###### new(inner) {#rust-enrichedreadonlyrepository-new-inner}

Wraps an inner [ReadonlyRepository](#readonly-repository). Uses the default strategy.

**Definition**

```rust
pub fn new(inner: Arc<dyn ReadonlyRepository>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [ReadonlyRepository](#readonly-repository) |

**Returns**

[EnrichedReadonlyRepository](#rust-enrichedreadonlyrepository)

###### with_strategy(inner, strategy) {#rust-enrichedreadonlyrepository-with_strategy-inner-strategy}

Wraps an inner repository with a [ReadonlyEnrichmentStrategy](#rust-readonlyenrichmentstrategy).

**Definition**

```rust
pub fn with_strategy(inner: Arc<dyn ReadonlyRepository>, strategy: Arc<dyn ReadonlyEnrichmentStrategy>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [ReadonlyRepository](#readonly-repository) |
| `strategy` | [ReadonlyEnrichmentStrategy](#rust-readonlyenrichmentstrategy) |

**Returns**

[EnrichedReadonlyRepository](#rust-enrichedreadonlyrepository)

###### add_enriched(into, from, via, by, select, many, nest) {#rust-enrichedreadonlyrepository-add_enriched-into-from-via-by-select-many-nest}

Adds a related fill. Pass `()` for `nest` when the child has no further fill.

**Definition**

```rust
pub fn add_enriched(&mut self, into: impl Into<String>, from: impl Into<String>, via: impl EnrichmentVia, by: impl Into<String>, select: Option<String>, many: bool, nest: impl Nest) -> &mut Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `into` | destination field |
| `from` | key on the parent |
| `via` | related readonly or [CRUDRepository](#crud-repository). A CRUD repository is kept for deep writes |
| `by` | key on the related row |
| `select` | optional related field to copy |
| `many` | collect a list |
| `nest` | child-node callback, or `()` |

**Returns**

`&mut Self`

##### EnrichedCRUDRepository {#rust-enrichedcrudrepository}

```rust
pub struct EnrichedCRUDRepository
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(inner)](#rust-enrichedcrudrepository-new-inner) | Wraps an inner [CRUDRepository](#crud-repository). Uses the default strategy. |
| [with_strategy(inner, strategy)](#rust-enrichedcrudrepository-with_strategy-inner-strategy) | Wraps an inner repository with a [ReadonlyEnrichmentStrategy](#rust-readonlyenrichmentstrategy). |

**Methods**

| Name | Description |
| --- | --- |
| [add_enriched(into, from, via, by, select, many, nest)](#rust-enrichedcrudrepository-add_enriched-into-from-via-by-select-many-nest) | Forwards to the inner [EnrichedReadonlyRepository](#rust-enrichedreadonlyrepository). A CRUD `via` is kept so `many` children can be written. |

###### new(inner) {#rust-enrichedcrudrepository-new-inner}

Wraps an inner [CRUDRepository](#crud-repository). Uses the default strategy.

**Definition**

```rust
pub fn new(inner: Arc<dyn CRUDRepository>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [CRUDRepository](#crud-repository) |

**Returns**

[EnrichedCRUDRepository](#rust-deepcrudrepository)

###### with_strategy(inner, strategy) {#rust-enrichedcrudrepository-with_strategy-inner-strategy}

Wraps an inner repository with a [ReadonlyEnrichmentStrategy](#rust-readonlyenrichmentstrategy).

**Definition**

```rust
pub fn with_strategy(inner: Arc<dyn CRUDRepository>, strategy: Arc<dyn ReadonlyEnrichmentStrategy>) -> Self
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [CRUDRepository](#crud-repository) |
| `strategy` | [ReadonlyEnrichmentStrategy](#rust-readonlyenrichmentstrategy) |

**Returns**

[EnrichedCRUDRepository](#rust-deepcrudrepository)

###### add_enriched(into, from, via, by, select, many, nest) {#rust-enrichedcrudrepository-add_enriched-into-from-via-by-select-many-nest}

Forwards to the inner [EnrichedReadonlyRepository](#rust-enrichedreadonlyrepository). A CRUD `via` is kept so `many` children can be written.

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
| enrichment | [Enrichment](#rust-enrichment) for this node. |

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
| `via` | related readonly or [CRUDRepository](#crud-repository). A CRUD repository is kept for deep writes |
| `into` | destination field |
| `from` | parent key |
| `by` | related key |
| `select` | optional related field to copy |
| `many` | collect a list |

**Returns**

`&mut Self`

##### Enrichment {#rust-enrichment}

```rust
pub struct Enrichment
```

**Constructor**

| Name | Description |
| --- | --- |
| [new(into, from, by, select, many)](#rust-enrichment-new-into-from-by-select-many) | Builds an enrichment. |

**Properties**

| Name | Description |
| --- | --- |
| into | Destination field. |
| from | Parent key. |
| by | Related key. |
| select | Optional related field to copy. |
| many | Collect a list. |

###### new(into, from, by, select, many) {#rust-enrichment-new-into-from-by-select-many}

Builds an enrichment.

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

##### ReadonlyEnrichmentStrategy {#rust-readonlyenrichmentstrategy}

```rust
pub trait ReadonlyEnrichmentStrategy
```

**Methods**

| Name | Description |
| --- | --- |
| [read(inner, tree, query)](#rust-readonlyenrichmentstrategy-read-inner-tree-query) | Reads through `inner`, then enriches. |
| [enrich(tree, items)](#rust-readonlyenrichmentstrategy-enrich-tree-items) | Fills `items` from the tree. |

###### read(inner, tree, query) {#rust-readonlyenrichmentstrategy-read-inner-tree-query}

Reads through `inner`, then enriches.

**Definition**

```rust
async fn read(&self, inner: &dyn ReadonlyRepository, tree: &RepositoryTree, query: Option<&dyn ReadInput>) -> Result<Vec<Row>>
```

**Parameters**

| Name | Description |
| --- | --- |
| `inner` | [ReadonlyRepository](#readonly-repository) |
| `tree` | [RepositoryTree](#rust-repositorytree) |
| `query` | optional query, where, or stored procedure |

**Returns**

`Result<Vec<Row>>`

###### enrich(tree, items) {#rust-readonlyenrichmentstrategy-enrich-tree-items}

Fills `items` from the tree.

**Definition**

```rust
async fn enrich(&self, tree: &RepositoryTree, items: &mut [Row]) -> Result<()>
```

**Parameters**

| Name | Description |
| --- | --- |
| `tree` | [RepositoryTree](#rust-repositorytree) |
| `items` | parents to fill |

**Returns**

`Result<()>`

### Remarks

Nest only on walkable objects (`many: true` or a whole related row). A `select` scalar is not walked. Depth is unbounded; each node applies, then children run on the related items.

`AddEnriched` stays fluent for siblings:

```
AddEnriched("children", "id", children, "parent_id", many: true,
    nest: node => node.Include(childTypes, "type_name", "type_id", "id", "name"));
```

### Examples

#### Fill a lookup

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/EnrichedReadonlyRepositoryTests.cs#L43-L47)

```csharp
var repo = new EnrichedReadonlyRepository<Parent>(parents, strategy);
repo.AddEnriched("type_name", "type_id", types, "id", "name");
var items = new List<Parent>();
await foreach (var item in repo.Read())
    items.Add(item);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1845-L1854)

```rust
let mut repo = EnrichedReadonlyRepository::new(parents);
repo.add_enriched(
    "type_name",
    "type_id",
    types,
    "id",
    Some("name".into()),
    false,
    (),
);
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/EnrichedReadonlyRepository.test.ts#L39-L51)

```typescript
it("fills lookup select and leaves missing default", async () => {
  const repo = new EnrichedReadonlyRepository(
    new ListReadonlyRepository([parent(1, 10), parent(2, 99)]),
  );
  repo.addEnriched(
    "type_name",
    "type_id",
    new ListReadonlyRepository([
      Object.assign(new TypeRow(), { id: 10, name: "person" }),
    ]),
    "id",
    "name",
  );
```

#### Fill user type on a user view

The user row keeps `UserTypeId` so the lookup can run. The lookup repository's entity is `UserType`, and [ChangeCasingMapper](#change-casing-mapper) maps that Pascal name, plus `Id` and `Name`, to `user_type`, `id`, and `name`. `UserView` drops `UserTypeId` and keeps `UserType`.

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Sqlite.Tests/ReadonlyRepositoryIntegrationTests.cs#L146-L210)

```csharp
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["filename"] = ":memory:" });
var connection = await Open(ds, $"""
    CREATE TABLE user_type (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
    );
    INSERT INTO user_type (id, name) VALUES
        (1, 'admin'),
        (2, 'user'),
        (3, 'service');
    CREATE TABLE user (
        id TEXT PRIMARY KEY,
        username TEXT NOT NULL,
        is_enabled INTEGER NOT NULL,
        created TEXT NOT NULL,
        user_type_id INTEGER NOT NULL
    );
    INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
        ('{Ada}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
        ('{Grace}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
        ('{Lin}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
    """);
var users = ds.CreateReadonlyRepository<UserAccount>(connection, new ReadonlyRepositoryOptions<UserAccount>
{
    Entity = "user",
    ReadColumns = ["Key", "Login", "Active", "Opened", "UserTypeId"],
    ColumnMappings = new DictionaryMapper<string>(new Dictionary<string, string>
    {
        ["Key"] = "id",
        ["Login"] = "username",
        ["Active"] = "is_enabled",
        ["Opened"] = "created",
        ["UserTypeId"] = "user_type_id",
    }),
});
var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
var types = ds.CreateReadonlyRepository<UserType>(connection, new ReadonlyRepositoryOptions<UserType>
{
    Entity = "UserType",
    EntityMapper = casing,
    ColumnMappings = casing,
});
var enriched = new EnrichedReadonlyRepository<UserAccount>(users);
enriched.AddEnriched("UserType", "UserTypeId", types, "Id", "Name");
var account = await enriched.ReadSingle(new WhereClause("Key", eq, Ada));
var usersView = new UserView
{
    Key = account!.Key,
    Login = account.Login,
    Active = account.Active,
    Opened = account.Opened,
    UserType = account.UserType,
};
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

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/sqlite/tests/readonly_repository.rs#L198-L281)

```rust
let built = datasource();
let ds: &dyn Datasource = &*built;
let connection = open(
    ds,
    &format!(
        "
    CREATE TABLE user_type (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
    );
    INSERT INTO user_type (id, name) VALUES
        (1, 'admin'),
        (2, 'user'),
        (3, 'service');
    CREATE TABLE user (
        id TEXT PRIMARY KEY,
        username TEXT NOT NULL,
        is_enabled INTEGER NOT NULL,
        created TEXT NOT NULL,
        user_type_id INTEGER NOT NULL
    );
    INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
        ('{ADA}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
        ('{GRACE}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
        ('{LIN}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
    "
    ),
)
.await;
let mut users_options = create_repository_options::<UserAccount>();
users_options.entity = "user".into();
users_options.column_mappings = Box::new(DictionaryMapper::new([
    ("Key".to_string(), "id".to_string()),
    ("Login".to_string(), "username".to_string()),
    ("Active".to_string(), "is_enabled".to_string()),
    ("Opened".to_string(), "created".to_string()),
    ("UserTypeId".to_string(), "user_type_id".to_string()),
]));
let mut type_options = create_repository_options::<UserType>();
type_options.entity = "UserType".into();
type_options.entity_mapper =
    Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
type_options.column_mappings =
    Box::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
let users = ds.create_readonly_repository::<UserAccount>(
    Some(connection.clone()),
    Some(users_options.read),
);
let types =
    ds.create_readonly_repository::<UserType>(Some(connection), Some(type_options.read));
let mut enriched = EnrichedReadonlyRepository::new(users);
enriched.add_enriched(
    "UserType",
    "UserTypeId",
    types,
    "Id",
    Some("Name".into()),
    false,
    (),
);
let account = enriched
    .read_single(Some(&WhereClause::binary(
        "Key",
        eq,
        Uuid::parse_str(ADA).unwrap(),
    )))
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

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/sqlite/tests/readonly.repository.integration.test.ts#L123-L179)

```typescript
const ds = new Datasource("sqlite", new SqliteDriver(), { filename: ":memory:" });
const connection = await open(ds, `
  CREATE TABLE user_type (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
  );
  INSERT INTO user_type (id, name) VALUES
    (1, 'admin'),
    (2, 'user'),
    (3, 'service');
  CREATE TABLE user (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL,
    is_enabled INTEGER NOT NULL,
    created TEXT NOT NULL,
    user_type_id INTEGER NOT NULL
  );
  INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
    ('${ada}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
    ('${grace}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
    ('${lin}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
`);
const users = ds.createReadonlyRepository(UserAccount, connection, {
  entity: "user",
  readColumns: ["Key", "Login", "Active", "Opened", "UserTypeId"],
  columnMappings: new DictionaryMapper([
    ["Key", "id"],
    ["Login", "username"],
    ["Active", "is_enabled"],
    ["Opened", "created"],
    ["UserTypeId", "user_type_id"],
  ]),
});
const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
const types = ds.createReadonlyRepository(UserType, connection, {
  entity: "UserType",
  entityMapper: casing,
  columnMappings: casing,
});
const enriched = new EnrichedReadonlyRepository(users);
enriched.addEnriched("UserType", "UserTypeId", types, "Id", "Name");
const account = await enriched.readSingle(new WhereClause("Key", eq, ada));
const usersView = new UserView();
usersView.Key = account?.Key ?? "";
usersView.Login = account?.Login ?? "";
usersView.Active = account?.Active ?? 0;
usersView.Opened = account?.Opened ?? "";
usersView.UserType = account?.UserType ?? "";
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

## Bulk repository

[EnrichedCRUDRepository](#deepcrudrepository) plus list writes. Each item still runs the same deep create / update / delete. Responses sum `rowsAffected`; `result` is the last item.

### Classes

##### TypeScript

##### BulkCRUDRepository {#typescript-bulkcrudrepository}

```typescript
class BulkCRUDRepository<T extends object> extends EnrichedCRUDRepository<T>
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
public class BulkCRUDRepository<T> : EnrichedCRUDRepository<T>
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
| [add_enriched(into, from, via, by, select, many, nest)](#rust-bulkcrudrepository-add_enriched-into-from-via-by-select-many-nest) | Forwards to the inner [EnrichedCRUDRepository](#rust-deepcrudrepository). A CRUD `via` is kept so `many` children can be written. |
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

###### add_enriched(into, from, via, by, select, many, nest) {#rust-bulkcrudrepository-add_enriched-into-from-via-by-select-many-nest}

Forwards to the inner [EnrichedCRUDRepository](#rust-deepcrudrepository). A CRUD `via` is kept so `many` children can be written.

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
pub async fn update_bulk(&self, items: &[Row], by_where: Option<&dyn Where>) -> Result<Response>
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
pub async fn delete_bulk(&self, items: &[Row], by_where: Option<&dyn Where>) -> Result<Response>
```

**Parameters**

| Name | Description |
| --- | --- |
| `items` | parents to delete |
| `by_where` | optional predicate applied to each item |

**Returns**

`Result<Response>`

### Remarks

C# exposes list overloads (`Create(IReadOnlyList<T>)`) and `Bulk*` aliases. TypeScript and Rust use `createBulk` / `create_bulk` only.

Each item still runs the same deep persist as [EnrichedCRUDRepository](#deepcrudrepository).

### Examples

#### Create parents and children

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/BulkCRUDRepositoryTests.cs#L23-L30)

```csharp
var parents = new ListCRUDRepository<Parent>();
var children = new ListCRUDRepository<Child>();
var repo = Repo(parents, children);
var response = await repo.Create(
[
    new Parent { children = [new Child { label = "a" }] },
    new Parent { children = [new Child { label = "b" }, new Child { label = "c" }] }
]);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/tests/mod.rs#L1862-L1879)

```rust
let parents = Arc::new(items_repository::ItemsRepository::new(vec![record(&[
    ("id", Value::Int(1)),
    ("type_id", Value::Int(10)),
])]));
let types = Arc::new(items_repository::ItemsRepository::new(vec![record(&[
    ("id", Value::Int(10)),
    ("name", Value::from("org")),
])]));
let children = Arc::new(items_repository::ItemsRepository::new(vec![
    record(&[("parent_id", Value::Int(1)), ("label", Value::from("a"))]),
    record(&[("parent_id", Value::Int(1)), ("label", Value::from("b"))]),
    record(&[("parent_id", Value::Int(2)), ("label", Value::from("x"))]),
]));
let mut repo = EnrichedReadonlyRepository::new(parents);
repo.add_enriched(
    "type_name",
    "type_id",
    types,
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/BulkCRUDRepository.test.ts#L29-L37)

```typescript
it("creates parents and children", async () => {
  const parents = new ListCRUDRepository<Parent>();
  const children = new ListCRUDRepository<Child>();
  const response = await repoOf(parents, children).createBulk([
    assign(Parent, { children: [assign(Child, { label: "a" })] }),
    assign(Parent, {
      children: [assign(Child, { label: "b" }), assign(Child, { label: "c" })],
    }),
  ]);
```

## Mapper

Bidirectional `Map` / `ReverseMap`. Column names pass through unless a mapper rewrites them. Repository read is convert, then map.

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

### Remarks

Repository read is convert, then map. [ReadonlyRepositoryOptions.RowsMapper](#rowsmapper) is the read-row mapper. `Repository.Mapper` / `mapper` is that mapper.

### See also

- [IdentityMapper](#identity-mapper)
- [DictionaryMapper](#dictionary-mapper)
- [PluralizerMapper](#pluralizer-mapper)
- [ChangeCasingMapper](#change-casing-mapper)
- [TransformMapper](#transform-mapper)
- [FlatToNestedMapper](#flat-to-nested-mapper)
- [ReadonlyRepositoryOptions](#readonlyrepositoryoptions)

## IdentityMapper

Mapper that returns the value unchanged, in both directions. This is the default entity, column, and row mapper on [ReadonlyRepositoryOptions](#readonlyrepositoryoptions).

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

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/IdentityMapperTests.cs#L10-L12)

```csharp
var mapper = new IdentityMapper<string>();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/identity_mapper.rs#L39-L41)

```rust
let mapper = IdentityMapper::<String>::new();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/IdentityMapper.test.ts#L6-L8)

```typescript
const mapper = new IdentityMapper<string>();
```

## DictionaryMapper

Mapper that rewrites known keys and passes the rest through. Reverse lookup uses the mapped value. When two keys share a value, the first one wins on reverse.

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

Use this for [ColumnMappings](#columnmappings) when property names and stored column names differ. `Map` rewrites the SELECT list, where identifiers, and write parameters. `ReverseMap` restores property names on each result key before the row converter.

### Examples

#### Map and reverse map

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/DictionaryMapperTests.cs#L14-L25)

```csharp
var mapper = new DictionaryMapper<string>(new Dictionary<string, string>
{
    ["Key"] = "id",
    ["Login"] = "username",
});
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/dictionary_mapper.rs#L51-L61)

```rust
let mapper = DictionaryMapper::new([
    ("Key".to_string(), "id".to_string()),
    ("Login".to_string(), "username".to_string()),
]);
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/DictionaryMapper.test.ts#L14-L23)

```typescript
const mapper = new DictionaryMapper([
  ["Key", "id"],
  ["Login", "username"],
]);
```

## PluralizerMapper

Mapper that pluralizes a name and lowercases it. Reverse singularizes and keeps the first letter's case.

### Classes

##### TypeScript

##### PluralizerMapper {#typescript-pluralizermapper}

```typescript
class PluralizerMapper implements IMapper<string, string>
```

**Methods**

| Name | Description |
| --- | --- |
| [map(value)](#typescript-pluralizermapper-map-value) | Pluralizes `value` and lowercases it. `User` becomes `users`. |
| [reverseMap(value)](#typescript-pluralizermapper-reversemap-value) | Singularizes `value`. `users` becomes `user`. |

###### map(value) {#typescript-pluralizermapper-map-value}

Pluralizes `value` and lowercases it. `User` becomes `users`.

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

Singularizes `value`. `users` becomes `user`.

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
| [Map(value)](#csharp-pluralizermapper-map-value) | Pluralizes `value` and lowercases it. `User` becomes `users`. |
| [ReverseMap(value)](#csharp-pluralizermapper-reversemap-value) | Singularizes `value`. `users` becomes `user`. |

###### Map(value) {#csharp-pluralizermapper-map-value}

Pluralizes `value` and lowercases it. `User` becomes `users`.

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

Singularizes `value`. `users` becomes `user`.

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
| [map(value)](#rust-pluralizermapper-map-value) | Pluralizes `value` and lowercases it. `User` becomes `users`. |
| [reverse_map(value)](#rust-pluralizermapper-reverse_map-value) | Singularizes `value`. `users` becomes `user`. |

###### new() {#rust-pluralizermapper-new}

Builds a pluralizer mapper.

**Definition**

```rust
pub fn new() -> Self
```

**Returns**

[PluralizerMapper](#rust-pluralizermapper)

###### map(value) {#rust-pluralizermapper-map-value}

Pluralizes `value` and lowercases it. `User` becomes `users`.

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

Singularizes `value`. `users` becomes `user`.

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

Use this as [EntityMapper](#entitymapper) when the type name is singular and the table name is a lowercase plural. Setting `Entity` / `entity` skips the mapper.

### Examples

#### Pluralize and lowercase

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/PluralizerMapperTests.cs#L8-L14)

```csharp
var mapper = new PluralizerMapper();
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/pluralizer_mapper.rs#L37-L42)

```rust
let mapper = PluralizerMapper::new();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/PluralizerMapper.test.ts#L4-L9)

```typescript
describe("PluralizerMapper", () => {
  it("pluralizes and lowercases the table name", () => {
    const mapper = new PluralizerMapper();
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

#### Map Pascal to snake case

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/ChangeCasingMapperTests.cs#L7-L15)

```csharp
var mapper = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/change_casing_mapper.rs#L34-L42)

```rust
let mapper = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/ChangeCasingMapper.test.ts#L4-L11)

```typescript
describe("ChangeCasingMapper", () => {
  it("maps pascal to snake and back", () => {
    const mapper = new ChangeCasingMapper("pascalCase", "snakeCase");
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

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources.Tests/TransformMapperTests.cs#L13-L24)

```csharp
public void FlattensAndUnflattensUserAndContacts()
{
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

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/mod.rs#L59-L94)

```rust
user: User,
contacts: Contacts,
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

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mappers/transformMapper.test.ts#L16-L40)

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

## FlatToNestedMapper

Collapses denormalized join rows into unique parents with unique children. Scalars map column to field; groups collect children by key.

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

`IReadOnlyList<`[Row](#repository)`>`

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

C# and Rust map to a [MappedValue](#mappedvalue) graph. TypeScript maps to generic `T`. Nested group fields on a collection are omitted.

This is a collapse mapper, not enrichment. Related fills after read use [Enriched repository](#enriched-repository).

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

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/datasources/src/mappers/flat_to_nested_mapper.rs#L383-L453)

```rust
}

#[test]
fn maps_join_rows_to_a_nested_contact() {
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

## TypeConverter

`TypeConverter` is only `Convert` / `ReverseConvert`. A dialect adds a converter when the native provider cannot map the type. Core does not ship bool / datetime / uuid / decimal / binary converters.

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

Repository read is convert, then map. See [TransformMapper](#transform-mapper) and [Repository](#repository).

### See also

- [RowTypeConverter](#row-type-converter)

## RowTypeConverter

TypeConverter for a row. Holds `Types` / `types` and `Fields` / `fields`. Convert uses `Fields[name]`. `Types` is the provider catalog merge and is unused at convert time.

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

`Types` / `types` is the provider catalog merge. Convert and reverse-convert only consult `Fields` / `fields`.

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

## Writing Providers

A provider package implements `IDriver` / `Driver` and exports only that type. `DatasourceFactory.Register` / `register` stores it by name. The driver builds the connection, command, and transaction, and supplies row cell converters when native values are not already canonical cells.

`mysql`, `postgres`, and `sqlite` name the type `*Driver` (`SqliteDriver`). Every other package names it `*Provider` (`MemoryProvider`).

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
| `isolation` | defaults to `serializable` |

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
ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable)
```

**Parameters**

| Name | Description |
| --- | --- |
| `connection` | open or closed [IConnection](#connection) |
| `isolation` | defaults to `Serializable` |

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
| `isolation` | defaults to `Serializable` at the call site |

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

### Remarks

`connection` receives the options bag (`url`, `filename`, `host`, `database`, `schema`, …). `command` delegates to the connection's `createCommand` / `CreateCommand` / `create_command`. `transaction` builds the dialect transaction. Isolation defaults to serializable.

`rowTypeConverters` / `RowTypeConverters` / `row_type_converters` maps a canonical type name to a [TypeConverter](#type-converter). SQL dialects register `bool`, `datetime`, `uuid`, `decimal`, and `binary` when the engine cannot return those as canonical cells. Memory, document, and file providers return an empty map.

The package exports only the driver type.

- C#: that class is `public`. Connection, command, transaction, options, and converters are `internal`. `InternalsVisibleTo` opens them to the test project and to an engine assembly such as `Datasources.Sqlite.Microsoft`. `GetExportedTypes()` is exactly the driver. The engine assembly exports nothing.
- TypeScript: `src/index.ts` is one line, `export { SqliteDriver } from "./SqliteDriver.ts"`. `package.json` `exports` points only at that file.
- Rust: `lib.rs` declares private modules and one `pub use` of the driver. `pub` items inside those modules stay crate-private.

`PublicSurfaceTests` in each C# provider test project, the `exports only the provider` test in `typescript/register`, and `providers_export_only_their_provider` in `datasources-register` reject a second export.

### Examples

#### Register and Create

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Providers/Memory.Tests/RegisterAndCreateTests.cs#L10-L16)

```csharp
DatasourceFactory.Register("memory", new MemoryProvider());
var ds = DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = "memory" });
```

##### Rust [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/rust/providers/memory/src/lib.rs#L38-L44)

```rust
#[test]
fn register_and_create() {
    reset();
    register("memory", Arc::new(MemoryProvider));
    let ds = create(Some(&HashMap::from([("provider".into(), "memory".into())]))).unwrap();
```

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/providers/memory/src/index.test.ts#L27-L32)

```typescript
DatasourceFactory.register("memory", new MemoryProvider());
const ds = DatasourceFactory.create({ provider: "memory" });
```

#### Mock driver

##### C# [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/csharp/Datasources/Mocks/MockDriver.cs#L3-L11)

```csharp
public sealed class MockDriver : IDriver
{
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new MockConnection();
    public ICommand Command(IConnection connection) => new MockCommand(connection);
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable) =>
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

##### TypeScript [⟨/⟩](https://github.com/deterministic-code/datasources/blob/main/typescript/datasources/src/mocks/mock-driver.ts#L13-L25)

```typescript
export class MockDriver implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = {};
  connection = (_options?: DatasourceOptions): IConnection => new MockConnection();
  command = (connection: IConnection): ICommand => new MockCommand(connection);
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "serializable",
  ): ITransaction => {
    const txn = new MockTransaction(connection);
    txn.isolation = isolation;
    return txn;
  };
}
```
