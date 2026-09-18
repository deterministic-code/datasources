<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:readme. -->

# Datasources

Datasources is a data-access and object-relational mapper library with parity across multiple languages and datasource providers.

It is designed for AI agents. The motivation is that current AI-generated data-access code is not deterministic, and quality often degrades as a project scales.

The feature stack starts with standard data-access modules for commands, connections, and object mapping. Higher up, the API is more opinionated and adds time-saving features for CRUD repositories, enriched reading, and bulk commands.

- [Quick Start](#quickstart)
- [Feature Matrix](#feature-matrix)

## QuickStart

Register a factory, then create by provider name. Install only the dialect packages you use. In Rust, `use` is a keyword, so registration is `register`.

Working dialects (json, memory, sqlite, mysql, postgres) accept options on create. Stub dialects return a stub datasource.

### Packages

| Dialect | Provider | C# | Rust | TypeScript |
| --- | --- | --- | --- | --- |
| json | `json` | `Datasources.Json` | `datasources-json` | `@datasources/json` |
| memory | `memory` | `Datasources.Memory` | `datasources-memory` | `@datasources/memory` |
| sqlite | `sqlite` | `Datasources.Sqlite` (optional engine: `Datasources.Sqlite.Microsoft`) | `datasources-sqlite` (optional feature: `rusqlite`) | `@datasources/sqlite` (optional peer: `better-sqlite3`) |
| text | `text` | `Datasources.Text` | `datasources-text` | `@datasources/text` |
| xml | `xml` | `Datasources.Xml` | `datasources-xml` | `@datasources/xml` |
| yaml | `yaml` | `Datasources.Yaml` | `datasources-yaml` | `@datasources/yaml` |
| mysql | `mysql` | `Datasources.Mysql` | `datasources-mysql` | `@datasources/mysql` |
| sqlserver | `sqlserver` | `Datasources.SqlServer` | `datasources-sqlserver` | `@datasources/sqlserver` |
| postgres | `postgres` | `Datasources.Postgres` | `datasources-postgres` | `@datasources/postgres` |
| oracle | `oracle` | `Datasources.Oracle` | `datasources-oracle` | `@datasources/oracle` |
| mongodb | `mongodb` | `Datasources.MongoDb` | `datasources-mongodb` | `@datasources/mongodb` |
| dynamodb | `dynamodb` | `Datasources.DynamoDb` | `datasources-dynamodb` | `@datasources/dynamodb` |

Optional name-based register: `Datasources.Hosting`, `datasources-register` (Cargo features per dialect), `@datasources/register`.

### Register and create

#### C#

```csharp
Datasource.Use(
    JsonDatasource.Instance,
    MemoryDatasource.Instance,
    SqliteDatasource.Instance,
    TextDatasource.Instance,
    XmlDatasource.Instance,
    YamlDatasource.Instance,
    MysqlDatasource.Instance,
    SqlServerDatasource.Instance,
    PostgresDatasource.Instance,
    OracleDatasource.Instance,
    MongoDbDatasource.Instance,
    DynamoDbDatasource.Instance);

var json = Datasource.Create("json");
var memory = Datasource.Create("memory");
var sqlite = Datasource.Create("sqlite");
var text = Datasource.Create("text");
var xml = Datasource.Create("xml");
var yaml = Datasource.Create("yaml");
var mysql = Datasource.Create("mysql");
var sqlserver = Datasource.Create("sqlserver");
var postgres = Datasource.Create("postgres");
var oracle = Datasource.Create("oracle");
var mongodb = Datasource.Create("mongodb");
var dynamodb = Datasource.Create("dynamodb");
```

#### Rust

```rust
use datasources::{create, register};
use datasources_json::JsonDatasource;
use datasources_memory::MemoryDatasource;
use datasources_sqlite::SqliteDatasource;
use datasources_text::TextDatasource;
use datasources_xml::XmlDatasource;
use datasources_yaml::YamlDatasource;
use datasources_mysql::MysqlDatasource;
use datasources_sqlserver::SqlServerDatasource;
use datasources_postgres::PostgresDatasource;
use datasources_oracle::OracleDatasource;
use datasources_mongodb::MongoDbDatasource;
use datasources_dynamodb::DynamoDbDatasource;

register(JsonDatasource::instance());
register(MemoryDatasource::instance());
register(SqliteDatasource::instance());
register(TextDatasource::instance());
register(XmlDatasource::instance());
register(YamlDatasource::instance());
register(MysqlDatasource::instance());
register(SqlServerDatasource::instance());
register(PostgresDatasource::instance());
register(OracleDatasource::instance());
register(MongoDbDatasource::instance());
register(DynamoDbDatasource::instance());

let json = create("json", None)?;
let memory = create("memory", None)?;
let sqlite = create("sqlite", None)?;
let text = create("text", None)?;
let xml = create("xml", None)?;
let yaml = create("yaml", None)?;
let mysql = create("mysql", None)?;
let sqlserver = create("sqlserver", None)?;
let postgres = create("postgres", None)?;
let oracle = create("oracle", None)?;
let mongodb = create("mongodb", None)?;
let dynamodb = create("dynamodb", None)?;
```

#### TypeScript

```typescript
import { Datasource } from "@datasources/core";
import { JsonDatasource } from "@datasources/json";
import { MemoryDatasource } from "@datasources/memory";
import { SqliteDatasource } from "@datasources/sqlite";
import { TextDatasource } from "@datasources/text";
import { XmlDatasource } from "@datasources/xml";
import { YamlDatasource } from "@datasources/yaml";
import { MysqlDatasource } from "@datasources/mysql";
import { SqlServerDatasource } from "@datasources/sqlserver";
import { PostgresDatasource } from "@datasources/postgres";
import { OracleDatasource } from "@datasources/oracle";
import { MongoDbDatasource } from "@datasources/mongodb";
import { DynamoDbDatasource } from "@datasources/dynamodb";

Datasource.use(
  JsonDatasource.instance,
  MemoryDatasource.instance,
  SqliteDatasource.instance,
  TextDatasource.instance,
  XmlDatasource.instance,
  YamlDatasource.instance,
  MysqlDatasource.instance,
  SqlServerDatasource.instance,
  PostgresDatasource.instance,
  OracleDatasource.instance,
  MongoDbDatasource.instance,
  DynamoDbDatasource.instance,
);

const json = Datasource.create("json");
const memory = Datasource.create("memory");
const sqlite = Datasource.create("sqlite");
const text = Datasource.create("text");
const xml = Datasource.create("xml");
const yaml = Datasource.create("yaml");
const mysql = Datasource.create("mysql");
const sqlserver = Datasource.create("sqlserver");
const postgres = Datasource.create("postgres");
const oracle = Datasource.create("oracle");
const mongodb = Datasource.create("mongodb");
const dynamodb = Datasource.create("dynamodb");
```
### Default provider in apps

Datasources does not read config files or `.env`. The app owns defaults, then calls `create` / `createConnection`. Connections are constructed **closed**; open on first use.

| Value | Where |
| --- | --- |
| Provider (`postgres`, `sqlite`, `memory`) | Committed config or `DATASOURCE_PROVIDER` |
| Non-secrets (sqlite `filename`, json path, `schema`) | Committed config |
| Connection URL | Environment; locally a gitignored `.env` |
| Drivers / file stores | Memory (constructed in code) |

Do not commit URLs with credentials. Do not parse `.env` or `appsettings` inside `@datasources/core` / `Datasources` / `datasources`.

`.env` is for local and CI secrets only. Commit `.env.example`. Production uses the real environment or a secrets manager. This repo’s e2e tests already use `MYSQL_URL` and `POSTGRES_URL`.

Call `Datasource.defaults` / `Datasource.Defaults` / `defaults` with one dictionary. The library does not set this itself. `create()`, `createConnection`, `createCommand`, and `createRepository` use those values and reuse one closed connection.

Use `@datasources/cli` to write a local `.env` and add or remove dialect packages (and optional native engines) for npm, Cargo, or .NET. From this repo: `npm run setup` or `npm run cli -- add sqlite`. In a consumer app: `npx @datasources/cli setup`, `npx @datasources/cli add sqlite`, `npx @datasources/cli remove mysql`. Pass `--ecosystem ts|rust|csharp` when the directory has more than one project type. Core still does not read `.env`; load it in the app, then call the install helper.

To register several providers without listing factories, pass the same dictionary to the optional install helper (`Datasources.Hosting` / `datasources-register` / `@datasources/register`). It calls `Use` / `register` / `use` once per name in `providers` / `DATASOURCE_PROVIDERS`, always includes `provider` / `DATASOURCE_PROVIDER`, then sets defaults.

#### C#

```csharp
var options = new Dictionary<string, string?>
{
    ["provider"] = Environment.GetEnvironmentVariable("DATASOURCE_PROVIDER") ?? "sqlite",
    ["providers"] = Environment.GetEnvironmentVariable("DATASOURCE_PROVIDERS"),
    ["filename"] = Environment.GetEnvironmentVariable("SQLITE_FILENAME") ?? ":memory:",
    ["url"] = Environment.GetEnvironmentVariable("DATABASE_URL"),
    ["schema"] = Environment.GetEnvironmentVariable("DATABASE_SCHEMA")
};
DatasourceInstall.From(options);

var datasource = Datasource.Create();
var connection = await datasource.CreateConnection();
var command = datasource.CreateCommand();
```

#### Rust

```rust
use std::collections::HashMap;
use datasources::{create_default};
use datasources_register::register_from;

let options = HashMap::from([
    ("provider".into(), std::env::var("DATASOURCE_PROVIDER").unwrap_or_else(|_| "sqlite".into())),
    ("providers".into(), std::env::var("DATASOURCE_PROVIDERS").unwrap_or_default()),
    ("filename".into(), std::env::var("SQLITE_FILENAME").unwrap_or_else(|_| ":memory:".into())),
]);
register_from(options)?;

let datasource = create_default()?;
let connection = datasource.create_connection().await?;
let command = datasource.create_command(None);
```

#### TypeScript

```typescript
import { install } from "@datasources/register";

const options = {
  provider: process.env.DATASOURCE_PROVIDER ?? config.provider,
  providers: process.env.DATASOURCE_PROVIDERS,
  filename: config.sqliteFilename ?? ":memory:",
  url: process.env.DATABASE_URL,
  schema: config.schema,
};
await install(options);

const datasource = Datasource.create();
const connection = await datasource.createConnection();
const command = datasource.createCommand();
```
Register factories (explicitly or via install), create the default datasource, then `createConnection`. Do not `open` during bootstrap. Hold both in DI or a module binding. On first command or repository call, open if needed. See [Connection](#connection).

## Feature Matrix

Parity across C#, Rust, and TypeScript. Standard modules first; opinionated repositories above them.

### Feature stack

| Feature | C# | Rust | TypeScript | Role |
| --- | --- | --- | --- | --- |
| Datasource | `IDatasource` | `Datasource` | `IDatasource` | Factory: `createConnection` (async, construct only), `createCommand`, `createRepository`, `createReadonlyRepository` |
| Mapping | `IMapper<TFrom, TTo>`, `TypeConverter<To, From>` | `Mapper<TFrom, TTo>`, `TypeConverter<To, From>` | `IMapper<TFrom, TTo>`, `TypeConverter` | column → object convert, then map |
| Connection | `IConnection` | `Connection` | `IConnection` | `open`, `close`, `startTransaction(isolation)` |
| Transaction | `ITransaction` | `Transaction` | `ITransaction` | `isolation`, `commit`, `abort` |
| Command | `ICommand` | `Command` | `ICommand` | `executeReader`, `execute` |
| Reader | `IReader` | `Reader` | `IReader` | `current`, `next` |
| Query / Where | `IQuery` / `IWhere` | `Query` / `Where` | `IQuery` / `IWhere` | `getText()` — provider-specific |
| Parameter / Response | `Parameter` / `Response` | `Parameter` / `Response` | `Parameter` / `Response` | `{ name, value }` / `{ result, rowsAffected }` |
| Readonly repository | `IReadonlyRepository<T>` | `ReadonlyRepository` | `IReadonlyRepository<T>` | `connection`, `getSingle`, `get` |
| CRUD repository | `ICRUDRepository<T>` | `CRUDRepository` | `ICRUDRepository<T>` | extends `IReadonlyRepository<T>` with `create`, `update`, `delete` |
| Enriched reading | `EnrichedReadonlyRepository<T>` | `EnrichedReadonlyRepository` | `EnrichedReadonlyRepository<T>` | fill related properties after get |
| Bulk commands | `BulkCRUDRepository<T>` | `BulkCRUDRepository` | `BulkCRUDRepository<T>` | list create / update / delete |

`ICommand` supports `addMiddleware` / `AddMiddleware` / `add_middleware` with `beforeQuery` / `afterQuery`. A `beforeQuery` error skips the query. A query error still runs `afterQuery` with `error` and elapsed time set. `ConsoleCommandMiddleware` logs Start/Finish lines to the console.

### Providers

Working: **json**, **memory**, **sqlite**, **mysql**, **postgres** (inject `IFileStore` / `ISqliteDriver` / `IMysqlDriver` / `IPostgresDriver`). Native engines are opt-in: `better-sqlite3` / `mysql2` / `pg` (optional npm peers), `rusqlite` / `mysql_async` / `tokio-postgres`, or `Datasources.Sqlite.Microsoft` / MySqlConnector / Npgsql.

Stubs (installable, throw “not implemented”): text, xml, yaml, sqlserver, oracle, mongodb, dynamodb.

Package names per language are on [QuickStart](#packages).

## Common Examples

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
| `Datasource` | `register` / `create` / `defaults` | `Datasource` |
| `Use` | `register` | `use` |
| `DatasourceInstall.From` (Hosting) | `register_from` (`datasources-register`) | `install` (`@datasources/register`) |
| `Create` | `create` / `create_default` | `create` |
| `Defaults` | `defaults` | `defaults` |
| `Registered` | `registered` | `registered` |
| `Reset` | `reset` | `reset` |

#### C#

```csharp
Datasource.Use(SqliteDatasource.Instance);
var ds = Datasource.Create("sqlite");
```

#### Rust

```rust
register(SqliteDatasource::instance());
let ds = create("sqlite", None)?;
```

#### TypeScript

```typescript
Datasource.use(SqliteDatasource.instance);
const ds = Datasource.create("sqlite");
```
See [QuickStart](#quickstart) for full registration and default-provider setup.

## Repositories

Typed read and write over a connection. `IReadonlyRepository<T>` is get-only; `ICRUDRepository<T>` (C# / TypeScript) / `CRUDRepository` (Rust) adds `create`, `update`, and `delete`. SQL providers typically build a table repository from `RepositoryBuilder` and a `TypeConverter` mapping.

### IReadonlyRepository / ICRUDRepository

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IReadonlyRepository<T>` | `ReadonlyRepository` | `IReadonlyRepository<T>` |
| `ICRUDRepository<T>` | `CRUDRepository` | `ICRUDRepository<T>` |
| `GetSingle` | `get_single` | `getSingle` |
| `Get` | `get` | `get` |
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

### Enrichment

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

`BulkCRUDRepository` inherits `DeepCRUDRepository` and adds list writes. Each item still runs the same deep create / update / delete. Responses sum `rowsAffected`; `result` is the last item.

| C# | TypeScript | Rust |
| --- | --- | --- |
| `Create(IReadOnlyList<T>)` / `BulkCreate` | `createBulk` | `create_bulk` |
| `Update(IReadOnlyList<T>)` / `BulkUpdate` | `updateBulk` | `update_bulk` |
| `Delete(IReadOnlyList<T>)` / `BulkDelete` | `deleteBulk` | `delete_bulk` |

### Examples

TODO: fill as the generic `Repository<T>` surface settles.

## Connection

`CreateConnection` / `createConnection` / `create_connection` is async so a dialect can do async work later. Today it only constructs a closed named connection. The caller opens it.

### IConnection

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IConnection` | `Connection` | `IConnection` |
| `Provider` | `provider()` | `provider` |
| `Opened` | `opened()` | `opened` |
| `CreateCommand` | `create_command` | `createCommand` |
| `Open` | `open` | `open` |
| `Close` | `close` | `close` |
| `StartTransaction` | `start_transaction` | `startTransaction` |

Working dialects (`json`, `memory`, `sqlite`) open against injected ports (`IFileStore` / `FileStore`, `MemoryStore`, `ISqliteDriver` / `SqliteDriver`). Stub dialects return a named connection; open / close / start-transaction throw or return not-implemented.

### Provider types

| Dialect | Provider | Connection |
| --- | --- | --- |
| json | `json` | `JsonConnection` |
| memory | `memory` | `MemoryConnection` |
| sqlite | `sqlite` | `SqliteConnection` |
| text | `text` | `TextConnection` |
| xml | `xml` | `XmlConnection` |
| yaml | `yaml` | `YamlConnection` |
| mysql | `mysql` | `MysqlConnection` |
| sqlserver | `sqlserver` | `SqlServerConnection` |
| postgres | `postgres` | `PostgresConnection` |
| oracle | `oracle` | `OracleConnection` |
| mongodb | `mongodb` | `MongoDbConnection` |
| dynamodb | `dynamodb` | `DynamoDbConnection` |

### Create and open

#### C#

```csharp
Datasource.Use(JsonDatasource.Instance, MemoryDatasource.Instance, SqliteDatasource.Instance);

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

Datasource.use(JsonDatasource.instance);

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

Task EnsureOpen() => connection.Opened ? Task.CompletedTask : connection.Open();
```

#### Rust

```rust
use std::sync::Arc;
use datasources::{create_default, defaults, Connection, Result};

defaults(options);
let datasource = create_default()?;
let connection = datasource.create_connection().await?;

async fn ensure_open(connection: &Arc<dyn Connection>) -> Result<()> {
    if connection.opened() {
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
  if (!connection.opened) await connection.open();
}
```
### Mock

Depend on `IConnection` / `Connection`. Use `MockConnection` / `MockDatasource`. `CreateConnection` returns the configured mock and does not open it.

#### C#

```csharp
var ds = new MockDatasource();
var connection = await ds.CreateConnection();
await connection.Open();
Assert.True(((MockConnection)connection).Opened);

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
expect((connection as MockConnection).opened).toBe(true);

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

### Provider types

| Dialect | Provider | Command |
| --- | --- | --- |
| json | `json` | `JsonCommand` |
| memory | `memory` | `MemoryCommand` |
| sqlite | `sqlite` | `SqliteCommand` |
| text | `text` | `TextCommand` |
| xml | `xml` | `XmlCommand` |
| yaml | `yaml` | `YamlCommand` |
| mysql | `mysql` | `MysqlCommand` |
| sqlserver | `sqlserver` | `SqlServerCommand` |
| postgres | `postgres` | `PostgresCommand` |
| oracle | `oracle` | `OracleCommand` |
| mongodb | `mongodb` | `MongoDbCommand` |
| dynamodb | `dynamodb` | `DynamoDbCommand` |

### Create and execute

#### C#

```csharp
Datasource.Use(JsonDatasource.Instance, MemoryDatasource.Instance, SqliteDatasource.Instance);

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

Datasource.use(JsonDatasource.instance);

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

## Builders

Shared statement construction used by SQL providers. Dialect packages may wrap `StatementBuilder` for quoting and native column types, including `CreateStoredProcedure` the same way as `CreateInsert`.

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
