# Create factory (Rust)

Register a factory, then create by provider name. Install only the dialect crates you use.

`use` is a keyword, so registration is `register`.

| Dialect | Provider | Crate |
| --- | --- | --- |
| json | `json` | `datasources-json` |
| memory | `memory` | `datasources-memory` |
| sqlite | `sqlite` | `datasources-sqlite` (optional feature: `rusqlite`) |
| text | `text` | `datasources-text` |
| xml | `xml` | `datasources-xml` |
| yaml | `yaml` | `datasources-yaml` |
| mysql | `mysql` | `datasources-mysql` |
| sqlserver | `sqlserver` | `datasources-sqlserver` |
| postgres | `postgres` | `datasources-postgres` |
| oracle | `oracle` | `datasources-oracle` |
| mongodb | `mongodb` | `datasources-mongodb` |
| dynamodb | `dynamodb` | `datasources-dynamodb` |

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

register(JsonDatasource::factory());
register(MemoryDatasource::factory());
register(SqliteDatasource::factory());
register(TextDatasource::factory());
register(XmlDatasource::factory());
register(YamlDatasource::factory());
register(MysqlDatasource::factory());
register(SqlServerDatasource::factory());
register(PostgresDatasource::factory());
register(OracleDatasource::factory());
register(MongoDbDatasource::factory());
register(DynamoDbDatasource::factory());

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

Working dialects (json, memory, sqlite) accept options on `create`. Stub dialects return a stub datasource.

## Default provider in apps

The library does not load config. The consumer must call `defaults` with a `HashMap`. `create_default`, `create_connection`, `create_command`, and `create_repository` then use those values.

`datasources-register` can register several factories from the same map (enable Cargo features per dialect): `providers` / `DATASOURCE_PROVIDERS` plus `provider` / `DATASOURCE_PROVIDER`, then sets defaults.

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
let command = datasource.create_command(None);
```

