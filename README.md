<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:readme. -->

# Datasources

Async interfaces for reading, writing, updating, and deleting resources. Providers implement the same surface for files, SQL, and (later) document stores. Install only the providers you use.

This site and the generated root `README.md` are built from the markdown in `docs/`. Edit those files and run `npm run docs:readme` (or `npm run docs` for the site).

## Layout

```
typescript/   npm workspaces (datasources + providers/{json,memory,sqlite,mysql,postgres,stubs})
rust/         cargo workspace (datasources + providers/{json,memory,sqlite,mysql,postgres,stubs})
csharp/       .NET 10 solution (Datasources + Providers/{Json,Memory,Sqlite,Mysql,Postgres,stubs})
```

Optional helpers register factories from a provider list (`DATASOURCE_PROVIDERS`) without calling `Use` per dialect: `Datasources.Hosting`, `datasources-register`, `@datasources/register`. See [QuickStart](#quickstart).

## Interfaces

| Type | Role |
| --- | --- |
| `IDatasource` | Factory: `createConnection` (async, construct only), `createCommand`, `createRepository`, `createReadonlyRepository` |
| `IConnection` | `open`, `close`, `startTransaction(isolation)` |
| `ITransaction` | `isolation`, `commit`, `abort` |
| `IsolationLevel` | `readUncommitted`, `readCommitted`, `repeatableRead`, `serializable` |
| `ICommand` | `executeReader`, `execute` |
| `IReadonlyRepository<T>` | `connection`, `getSingle`, `get` |
| `IRepository<T>` | extends `IReadonlyRepository<T>` with `add`, `update`, `delete` |
| `IReader` | `current`, `next` |
| `IQuery` / `Where` | `getText()` — provider-specific |
| `Parameter` / `IResponse` | `{ name, value }` / `{ result, rowsAffected }` |

`ICommand` supports `use(ICommandMiddleware)` with `beforeQuery` / `afterQuery`. A `beforeQuery` error skips the query. A query error still runs `afterQuery` with `error` and elapsed time set. `ConsoleCommandMiddleware` logs Start/Finish lines to the console.

## Providers

Working: **json**, **memory**, **sqlite**, **mysql**, **postgres** (inject `IFileStore` / `ISqliteDriver` / `IMysqlDriver` / `IPostgresDriver`). Native engines are opt-in: `better-sqlite3` / `mysql2` / `pg` (optional npm peers), `rusqlite` / `mysql_async` / `tokio-postgres`, or `Datasources.Sqlite.Microsoft` / MySqlConnector / Npgsql.

Stubs (installable, throw “not implemented”): text, xml, yaml, sqlserver, oracle, mongodb, dynamodb.

Shared SQL scenarios live in `specs/*.yaml`. Engine-specific DDL and isolation expects are in `specs/dialects/{sqlite,mysql,postgres}.yaml` (fragment names such as `createUserTable` and `dirtyReadRows`).

## Tests

`npm test` is offline: mocks, injectable ports, and in-process SQLite command specs. `npm run test:e2e` runs repository specs against in-process SQLite plus MySQL/Postgres Docker command specs (starts the matching Compose services).

```
npm test
npm run test:e2e
npm run test:e2e:verbose
```

`test:e2e:verbose` (or `npm run test:e2e -- --verbose` / `VERBOSE=1`) prints each `ConsoleCommandMiddleware` Start/Finish line while specs run.

`test:e2e` defaults to `MYSQL_URL=mysql://ds:ds@127.0.0.1:3306/datasources` and `POSTGRES_URL=postgres://ds:ds@127.0.0.1:5432/datasources`. Override those env vars if the servers are elsewhere.

All languages against Docker (Rust/C# expect Compose already up):

```
npm run test:e2e
npm run test:e2e:rust
npm run test:e2e:csharp
```

- TypeScript: `npm test` / `npm run test:e2e` (Vitest; unit coverage 100% line/branch/function/statement)
- Rust: `cd rust && cargo test --workspace` (optional: `./coverage.sh` with cargo-llvm-cov)
- C#: `cd csharp && dotnet test Datasources.slnx`

CI (`.github/workflows/ci.yml`) runs `npm test` plus Rust/C# unit tests, then a second job that starts Compose and runs `test:e2e:ci` (skips starting Docker) plus Rust/C# e2e.

`Datasources` publishes test doubles (`MockConnection`, `MockCommand`, `MockRepository`, …) for consumers.

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

## Common Examples

Short recipes for the usual paths. Add samples here as Repository and related types land.

### Create a repository

TODO: register a provider, create a connection, `createRepository`, then `add` / `get` / `update` / `delete`.

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

Typed read and write over a connection. `IReadonlyRepository<T>` is get-only; `IRepository<T>` adds `add`, `update`, and `delete`. SQL providers typically build a table repository from `RepositoryBuilder` and a `TypeConverter` mapping.

### IReadonlyRepository / IRepository

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IReadonlyRepository<T>` | `ReadonlyRepository` | `IReadonlyRepository<T>` |
| `IRepository<T>` | `Repository` | `IRepository<T>` |
| `GetSingle` | `get_single` | `getSingle` |
| `Get` | `get` | `get` |
| `Add` | `add` | `add` |
| `Update` | `update` | `update` |
| `Delete` | `delete` | `delete` |

### Examples

TODO: fill as the generic `Repository<T>` / `TableRepository` surface settles.

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
| `Use` | `use_middleware` | `use` |
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
await command.ExecuteReader();
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
command.execute(Some("insert"), None).await?;
command.execute_reader(None, None).await?;
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
await command.executeReader();
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
await Assert.ThrowsAsync<InvalidOperationException>(() => failing.Execute());
```

#### Rust

```rust
use datasources::{Datasource, MockCommand, MockDatasource};

let ds = MockDatasource::new("mock");
let command = ds.create_command(None);
command.execute(None, None).await?;
command.execute_reader(None, None).await?;

let failing = MockCommand::new();
failing.set_execute_error("exec-fail");
assert!(failing.execute(None, None).await.is_err());
```

#### TypeScript

```typescript
const ds = new MockDatasource();
const command = ds.createCommand();
await command.execute("insert");
expect((command as MockCommand).executeCalls).toHaveLength(1);

const failing = new MockCommand(ds.connection);
failing.executeError = new Error("exec-fail");
await expect(failing.execute()).rejects.toThrow("exec-fail");
```
Provider tests inject fakes on options (`MemoryFileStore`, `ScriptedDriver`) and then execute / execute-reader.

## Builders

Shared statement construction used by SQL providers. Dialect packages may wrap `StatementBuilder` for quoting and native column types.

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

### ISourceDestinationMapper

| C# | Rust | TypeScript |
| --- | --- | --- |
| `ISourceDestinationMapper` | `ISourceDestinationMapper` | `ISourceDestinationMapper` |
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

## TypeConverters

Registry of provider type tables and per-field converters. Register a provider’s types, then `Mapping` / `mapping` to project rows to a host type. The host destination is `"csharp"`, `"rust"`, or `"typescript"`.

### TypeConverter

| C# | Rust | TypeScript |
| --- | --- | --- |
| `TypeConverter` | `use_provider` / `mapping` | `TypeConverter` |
| `Use` | `use_provider` | `use` |
| `Mapping<T>` | `mapping` / `mapping_with` | `mapping` |
| `Reset` | `reset` | `reset` |
| `SqlProviderTypes` | `sql_provider_types` | `sqlProviderTypes` |
| `IdentityProviderTypes` | `identity_provider_types` | `identityProviderTypes` |

### FieldConverter

| C# | Rust | TypeScript |
| --- | --- | --- |
| `IFieldConverter` / `FieldConverter` | `FieldConverter` | `FieldConverter` |
| `FieldConverters` (Identity, Boolean, DateTime, Decimal, Binary, Uuid) | `identity_field_converter`, `boolean_field_converter`, … | `identityFieldConverter`, … |

### Examples

TODO: register a provider table and map a row type as converters are documented.
