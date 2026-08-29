# Create command (Rust)

`create_command` is sync. It accepts an optional connection and returns a named command. The caller executes it. The connection must be open.

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

Working dialects (`json`, `memory`) treat `text` as `insert` / `update` / `delete`. `sqlite` treats `text` as SQL against the injected `SqliteDriver`. Stub dialects return a named command; `execute` / `execute_reader` return not-implemented.

The `Command` trait has no connection getter. Stub `create_command` stores the optional connection on the named type; working dialects still construct their own typed connection today.

## Mock

Depend on `Command`. Use `MockCommand` / `MockDatasource` from `datasources`. `MockDatasource::create_command` returns the configured command.

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

Provider tests inject fakes (`MemoryFileStore`, `ScriptedDriver`) and then `execute().await` / `execute_reader().await`.
