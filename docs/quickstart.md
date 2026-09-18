# QuickStart

Register a factory, then create by provider name. Install only the dialect packages you use.

Working dialects (json, memory, sqlite, mysql, postgres) accept options on create. Stub dialects return a stub datasource.

## Packages

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

## Register and create

::: dialects
```csharp
Datasource.Register(
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

Datasource.register(
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
:::

## Default provider in apps

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

To register several providers without listing factories, pass the same dictionary to the optional install helper (`Datasources.Hosting` / `datasources-register` / `@datasources/register`). It calls `Register` / `register` / `register` once per name in `providers` / `DATASOURCE_PROVIDERS`, always includes `provider` / `DATASOURCE_PROVIDER`, then sets defaults.

::: dialects
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
:::

Register factories (explicitly or via install), create the default datasource, then `createConnection`. Do not `open` during bootstrap. Hold both in DI or a module binding. On first command or repository call, open if needed. See [Connection](./sections/connection.md).
