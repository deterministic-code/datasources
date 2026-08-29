# Create connection (C#)

`CreateConnection` is async so a dialect can do async work later. Today it only constructs a closed named connection. The caller opens it.

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

```csharp
Datasource.Use(JsonDatasource.Instance, MemoryDatasource.Instance, SqliteDatasource.Instance);

var json = Datasource.Create("json", new JsonConnectionOptions { Data = new JsonArray() });
var connection = await json.CreateConnection();
await connection.Open();
```

## Startup

Apps create the default datasource and connection at process start, then hold the closed connection. Call `Open` on first use (`Open` is a no-op if already open). Do not open during bootstrap.

```csharp
Datasource.Defaults(options);
var datasource = Datasource.Create();
var connection = await datasource.CreateConnection();

Task EnsureOpen() => connection.Opened ? Task.CompletedTask : connection.Open();
```

Working dialects (`json`, `memory`, `sqlite`) open against injected ports (`IFileStore`, `MemoryStore`, `ISqliteDriver`). Stub dialects return a named connection; `Open` / `Close` / `StartTransaction` throw `ProviderNotImplementedException`.

## Mock

Depend on `IConnection`. Use `MockConnection` / `MockDatasource` from `Datasources`. `MockDatasource.CreateConnection` returns the configured `MockConnection` and does not open it.

```csharp
var ds = new MockDatasource();
var connection = await ds.CreateConnection();
await connection.Open();
Assert.True(((MockConnection)connection).Opened);

var failing = new MockConnection { OpenError = new InvalidOperationException("open-fail") };
ds.Connection = failing;
await Assert.ThrowsAsync<InvalidOperationException>(() => failing.Open());
```

Provider tests inject fakes on options (`MemoryFileStore`, `ScriptedDriver`) and then `await Open()`.
