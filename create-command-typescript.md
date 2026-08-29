# Create command (TypeScript)

`createCommand` is sync. It binds an optional connection (or constructs one) and returns a named command. The caller executes it. The connection must be open.

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

```ts
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

Working dialects (`json`, `memory`) treat `text` as `insert` / `update` / `delete`. `sqlite` treats `text` as SQL against the injected `ISqliteDriver`. Stub dialects return a named command; `execute` / `executeReader` throw `ProviderNotImplementedError`.

## Mock

Depend on `ICommand`. Use `MockCommand` / `MockDatasource` from `@datasources/core`. `MockDatasource.createCommand` returns the configured `MockCommand` (or a new one bound to the given connection).

```ts
const ds = new MockDatasource();
const command = ds.createCommand();
await command.execute("insert");
expect((command as MockCommand).executeCalls).toHaveLength(1);

const failing = new MockCommand(ds.connection);
failing.executeError = new Error("exec-fail");
await expect(failing.execute()).rejects.toThrow("exec-fail");
```

Provider tests inject fakes on options (`MemoryFileStore`, `ScriptedDriver`) and then `await execute()` / `await executeReader()`.
