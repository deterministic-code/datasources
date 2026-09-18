<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:readme. -->

# Datasources

Datasources is a data-access and object-relational mapper library with parity across multiple languages and datasource providers.

It is designed for AI agents. The motivation is that current AI-generated data-access code is not deterministic, and quality often degrades as a project scales.

The feature stack starts with standard data-access modules for commands, connections, and object mapping. Higher up, the API is more opinionated and adds time-saving features for CRUD repositories, enriched repositories, and bulk repositories.

- [Quick Start](#quickstart)
- [Feature Matrix](#feature-matrix)

## QuickStart

### Installation

#### TypeScript

Install Core

```typescript
npm install @datasources/core
```

Install Providers

```typescript
npm install @datasources/sqlite
npm install @datasources/mysql
npm install @datasources/postgres
npm install @datasources/memory
npm install @datasources/json
```

#### C#

Install Core

```csharp
dotnet add package Datasources
```

Install Providers

```csharp
dotnet add package Datasources.Sqlite
dotnet add package Datasources.Sqlite.Microsoft
dotnet add package Datasources.Mysql
dotnet add package Datasources.Postgres
dotnet add package Datasources.Memory
dotnet add package Datasources.Json
```

#### Rust

Install Core

```rust
cargo add datasources
```

Install Providers

```rust
cargo add datasources-sqlite
cargo add datasources-mysql
cargo add datasources-postgres
cargo add datasources-memory
cargo add datasources-json
```
### Setup

Add dialect packages and write a local `.env` with `@datasources/cli`. Core does not read `.env`; the app loads it, then registers providers and defaults.

```sh
npx @datasources/cli add sqlite
```

Pass `--ecosystem ts|rust|csharp` when the directory has more than one project type. From this repo: `npm run setup` or `npm run cli -- add sqlite`.

The optional install helper (`Datasources.Hosting` / `datasources-register` / `@datasources/register`) registers each name in `providers` / `DATASOURCE_PROVIDERS`, always includes `provider` / `DATASOURCE_PROVIDER`, then sets defaults. Connections are constructed **closed**; open on first use.

#### C#

```csharp
var options = new Dictionary<string, string?>
{
    ["provider"] = Environment.GetEnvironmentVariable("DATASOURCE_PROVIDER") ?? "sqlite",
    ["providers"] = Environment.GetEnvironmentVariable("DATASOURCE_PROVIDERS"),
    ["filename"] = Environment.GetEnvironmentVariable("SQLITE_FILENAME") ?? ":memory:",
    ["url"] = Environment.GetEnvironmentVariable("DATABASE_URL")
};
DatasourceInstall.From(options);

var datasource = Datasource.Create();
var connection = await datasource.CreateConnection();
```

#### Rust

```rust
use std::collections::HashMap;
use datasources::create_default;
use datasources_register::register_from;

let options = HashMap::from([
    ("provider".into(), std::env::var("DATASOURCE_PROVIDER").unwrap_or_else(|_| "sqlite".into())),
    ("providers".into(), std::env::var("DATASOURCE_PROVIDERS").unwrap_or_default()),
    ("filename".into(), std::env::var("SQLITE_FILENAME").unwrap_or_else(|_| ":memory:".into())),
]);
register_from(options)?;

let datasource = create_default()?;
let connection = datasource.create_connection().await?;
```

#### TypeScript

```typescript
import { Datasource } from "@datasources/core";
import { install } from "@datasources/register";

await install({
  provider: process.env.DATASOURCE_PROVIDER ?? "sqlite",
  providers: process.env.DATASOURCE_PROVIDERS,
  filename: process.env.SQLITE_FILENAME ?? ":memory:",
  url: process.env.DATABASE_URL,
});

const datasource = Datasource.create();
const connection = await datasource.createConnection();
```
Do not commit URLs with credentials. Production uses the real environment or a secrets manager.

### Readonly command

Start with `createCommand` and `executeReader`. The connection must be open.

#### C#

```csharp
Datasource.Register(SqliteDatasource.Instance);

var ds = Datasource.Create("sqlite", new SqliteConnectionOptions { Filename = ":memory:" });
var connection = await ds.CreateConnection();
await connection.Open();

var command = ds.CreateCommand(connection);
var reader = await command.ExecuteReader(
    "SELECT id, name FROM item WHERE id = $id",
    [new Parameter("id", 1)]);
while (await reader.Next())
    Console.WriteLine(reader.Current);
```

#### Rust

```rust
use datasources::{create, register, Parameter};
use datasources_sqlite::SqliteDatasource;

register(SqliteDatasource::instance());

let ds = create("sqlite", None)?;
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

#### TypeScript

```typescript
import { Datasource } from "@datasources/core";
import { SqliteDatasource } from "@datasources/sqlite";

Datasource.register(SqliteDatasource.instance);

const ds = Datasource.create("sqlite", { filename: ":memory:" });
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

#### C#

```csharp
var contacts = new DeepCRUDRepository<Contact>(contactRepo);
contacts.AddEnriched("contact_type_name", "contact_type_id", types, "id", "name");
contacts.AddEnriched("addresses", "id", addresses, "contact_id", many: true,
    nest: node => node.Include(addressTypes, "address_type_name", "address_type_id", "id", "name"));

var loaded = await contacts.GetSingle(WhereBuilder.Equals("id", 10));
await contacts.Create(new Contact { first_name = "Ada", addresses = [new Address { line1 = "1 Main" }] });
```

#### Rust

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

#### TypeScript

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

#### C#

```csharp
var mapper = new TransformMapper<Nested, Flat>(
    n => new Flat { UserId = n.User.Id, UserName = n.User.Name },
    f => new Nested { User = new User { Id = f.UserId, Name = f.UserName } });
var flat = mapper.Map(nested);
var back = mapper.ReverseMap(flat);
```

#### Rust

```rust
let mapper = TransformMapper::new(
    |n: Nested| Flat { user_id: n.user.id, user_name: n.user.name },
    |f: Flat| Nested { user: User { id: f.user_id, name: f.user_name } },
);
let flat = mapper.map(nested);
let back = mapper.reverse_map(flat);
```

#### TypeScript

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

#### C#

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

#### Rust

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

#### TypeScript

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

Package names per language are on [QuickStart](#installation).

## Examples

Short recipes for the usual paths. Add samples here as Repository and related types land.

### Create a repository

TODO: register a provider, create a connection, `createRepository`, then `create` / `get` / `update` / `delete`.

### Run a command

TODO: open a connection, `createCommand`, `execute` / `executeReader`. See [Command](#command) for the current samples.

### Transaction

TODO: `startTransaction`, work, then `commit` or `abort`.

## Datasource

Factory for connections, commands, and repositories. Register providers, then create by name. The library does not auto-register providers.

### IDatasource

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IDatasource` | `Datasource` trait | `IDatasource` |
| `Provider` | `provider()` | `provider` |
| `CreateConnection` | `create_connection` | `createConnection` |
| `CreateCommand` | `create_command` | `createCommand` |
| `CreateRepository<T>` | `create_repository` | `createRepository` |
| `CreateReadonlyRepository<T>` | `create_readonly_repository` | `createReadonlyRepository` |

### Registry

| C# | Rust | TypeScript |
| --- | --- | --- |
| `Datasource` | `datasources` | `Datasource` |
| `Register` | `register` | `register` |
| `DatasourceInstall.From` (Hosting) | `register_from` (`datasources-register`) | `install` (`@datasources/register`) |
| `Create` | `create` / `create_default` | `create` |
| `Defaults` | `defaults` | `defaults` |
| `Registered` | `registered` | `registered` |
| `Reset` | `reset` | `reset` |

#### C#

```csharp
Datasource.Register(SqliteDatasource.Instance);
var ds = Datasource.Create("sqlite");
```

#### Rust

```rust
register(SqliteDatasource::instance());
let ds = create("sqlite", None)?;
```

#### TypeScript

```typescript
Datasource.register(SqliteDatasource.instance);
const ds = Datasource.create("sqlite");
```
See [QuickStart](#quickstart) for full registration and default-provider setup.

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

#### C#

```csharp
Datasource.Register(JsonDatasource.Instance, MemoryDatasource.Instance, SqliteDatasource.Instance);

var json = Datasource.Create("json", new JsonConnectionOptions { Data = new JsonArray() });
var connection = await json.CreateConnection();
await connection.Open();
```

#### Rust

```rust
use datasources::{create, register};
use datasources_json::JsonDatasource;

register(JsonDatasource::instance());

let json = create("json", None)?;
let connection = json.create_connection().await?;
connection.open().await?;
```

#### TypeScript

```typescript
import { Datasource } from "@datasources/core";
import { JsonDatasource } from "@datasources/json";

Datasource.register(JsonDatasource.instance);

const json = Datasource.create("json", { data: [] });
const connection = await json.createConnection();
await connection.open();
```
### Startup

Apps create the default datasource and connection at process start, then hold the closed connection. Call `Open` / `open` on first use (a no-op if already open). Do not open during bootstrap.

#### C#

```csharp
Datasource.Defaults(options);
var datasource = Datasource.Create();
var connection = await datasource.CreateConnection();

Task EnsureOpen() => connection.IsOpen ? Task.CompletedTask : connection.Open();
```

#### Rust

```rust
use std::sync::Arc;
use datasources::{create_default, defaults, Connection, Result};

defaults(options);
let datasource = create_default()?;
let connection = datasource.create_connection().await?;

async fn ensure_open(connection: &Arc<dyn Connection>) -> Result<()> {
    if connection.is_open() {
        return Ok(());
    }
    connection.open().await
}
```

#### TypeScript

```typescript
Datasource.defaults(options);
const datasource = Datasource.create();
const connection = await datasource.createConnection();

async function ensureOpen() {
  if (!connection.isOpen) await connection.open();
}
```
### Mock

Depend on `IConnection` / `Connection`. Use `MockConnection` / `MockDatasource`. `CreateConnection` returns the configured mock and does not open it.

#### C#

```csharp
var ds = new MockDatasource();
var connection = await ds.CreateConnection();
await connection.Open();
Assert.True(((MockConnection)connection).IsOpen);

var failing = new MockConnection { OpenError = new InvalidOperationException("open-fail") };
ds.Connection = failing;
await Assert.ThrowsAsync<InvalidOperationException>(() => failing.Open());
```

#### Rust

```rust
use datasources::{Datasource, MockConnection, MockDatasource};

let ds = MockDatasource::new("mock");
let connection = ds.create_connection().await?;
connection.open().await?;

let failing = MockConnection::new();
failing.set_open_error("open-fail");
assert!(failing.open().await.is_err());
```

#### TypeScript

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

#### C#

```csharp
Datasource.Register(JsonDatasource.Instance, MemoryDatasource.Instance, SqliteDatasource.Instance);

var json = Datasource.Create("json", new JsonConnectionOptions { Data = new JsonArray() });
var connection = await json.CreateConnection();
await connection.Open();

var command = json.CreateCommand(connection);
await command.Execute("insert", new[] { new Parameter("id", 1) });
await command.ExecuteReader("");
```

#### Rust

```rust
use datasources::{create, register};
use datasources_json::JsonDatasource;

register(JsonDatasource::instance());

let json = create("json", None)?;
let connection = json.create_connection().await?;
connection.open().await?;

let command = json.create_command(Some(connection));
command.execute("insert", None).await?;
command.execute_reader("", None).await?;
```

#### TypeScript

```typescript
import { Datasource } from "@datasources/core";
import { JsonDatasource } from "@datasources/json";

Datasource.register(JsonDatasource.instance);

const json = Datasource.create("json", { data: [] });
const connection = await json.createConnection();
await connection.open();

const command = json.createCommand(connection);
await command.execute("insert", [{ name: "id", value: 1 }]);
await command.executeReader("");
```
### Mock

Depend on `ICommand` / `Command`. Use `MockCommand` / `MockDatasource`.

#### C#

```csharp
var ds = new MockDatasource();
var command = ds.CreateCommand();
await command.Execute("insert");
Assert.Single(((MockCommand)command).ExecuteCalls);

var failing = new MockCommand(ds.Connection) { ExecuteError = new InvalidOperationException("exec-fail") };
ds.Command = failing;
await Assert.ThrowsAsync<InvalidOperationException>(() => failing.Execute(""));
```

#### Rust

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

#### TypeScript

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
