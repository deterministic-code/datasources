# Create command (C#)

`CreateCommand` is sync. It binds an optional connection (or constructs one) and returns a named command. The caller executes it. The connection must be open.

| Dialect   | Provider    | Command            |
| --------- | ----------- | ------------------ |
| json      | `json`      | `JsonCommand`      |
| memory    | `memory`    | `MemoryCommand`    |
| sqlite    | `sqlite`    | `SqliteCommand`    |
| text      | `text`      | `TextCommand`      |
| xml       | `xml`       | `XmlCommand`       |
| yaml      | `yaml`      | `YamlCommand`      |
| mysql     | `mysql`     | `MysqlCommand`     |
| sqlserver | `sqlserver` | `SqlServerCommand` |
| postgres  | `postgres`  | `PostgresCommand`  |
| oracle    | `oracle`    | `OracleCommand`    |
| mongodb   | `mongodb`   | `MongoDbCommand`   |
| dynamodb  | `dynamodb`  | `DynamoDbCommand`  |

```csharp
Datasource.Use(JsonDatasource.Factory, MemoryDatasource.Factory, SqliteDatasource.Factory);

var json = Datasource.Create("json", new JsonConnectionOptions { Data = new JsonArray() });
var connection = await json.CreateConnection();
await connection.Open();

var command = json.CreateCommand(connection);
await command.Execute("insert", new[] { new Parameter("id", 1) });
await command.ExecuteReader();
```

Working dialects (`json`, `memory`) treat `text` as `insert` / `update` / `delete`. `sqlite` treats `text` as SQL against the injected `ISqliteDriver`. Stub dialects return a named command; `Execute` / `ExecuteReader` throw `ProviderNotImplementedException`.

## Mock

Depend on `ICommand`. Use `MockCommand` / `MockDatasource` from `Datasources`. `MockDatasource.CreateCommand` returns the configured `MockCommand` (or a new one bound to the given connection).

```csharp
var ds = new MockDatasource();
var command = ds.CreateCommand();
await command.Execute("insert");
Assert.Single(((MockCommand)command).ExecuteCalls);

var failing = new MockCommand(ds.Connection) { ExecuteError = new InvalidOperationException("exec-fail") };
ds.Command = failing;
await Assert.ThrowsAsync<InvalidOperationException>(() => failing.Execute());
```

Provider tests inject fakes on options (`MemoryFileStore`, `ScriptedDriver`) and then `await Execute()` / `await ExecuteReader()`.
