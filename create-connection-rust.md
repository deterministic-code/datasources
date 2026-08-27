# Create connection (Rust)

`create_connection` is async so a dialect can do async work later. Today it only constructs a closed named connection. The caller opens it.

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

```rust
use datasources::{create, register};
use datasources_json::JsonDatasource;

register(JsonDatasource::factory());

let json = create("json", None)?;
let connection = json.create_connection().await?;
connection.open().await?;
```

Working dialects (`json`, `memory`, `sqlite`) open against injected ports (`FileStore`, `MemoryStore`, `SqliteDriver`). Stub dialects return a named connection; `open` / `close` / `start_transaction` return not-implemented.

## Mock

Depend on `Connection`. Use `MockConnection` / `MockDatasource` from `datasources`. `MockDatasource::create_connection` returns the configured connection and does not open it.

```rust
use datasources::{Datasource, MockConnection, MockDatasource};

let ds = MockDatasource::new("mock");
let connection = ds.create_connection().await?;
connection.open().await?;

let failing = MockConnection::new();
failing.set_open_error("open-fail");
assert!(failing.open().await.is_err());
```

Provider tests inject fakes (`MemoryFileStore`, `ScriptedDriver`) and then `open().await`.
