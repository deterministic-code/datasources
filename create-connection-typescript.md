# Create connection (TypeScript)

`createConnection` is async so a dialect can do async work later. Today it only constructs a closed named connection. The caller opens it.

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

```ts
import { Datasource } from "@datasources/core";
import { JsonDatasource } from "@datasources/json";

Datasource.use(JsonDatasource.factory);

const json = Datasource.create("json", { data: [] });
const connection = await json.createConnection();
await connection.open();
```

## Startup

Apps create the default datasource and connection at process start, then hold the closed connection. Call `open` on first use (`open` is a no-op if already open). Do not open during bootstrap.

```ts
const datasource = createDefaultDatasource(options);
const connection = await datasource.createConnection();

async function ensureOpen() {
  if (!connection.opened) await connection.open();
}
```

Working dialects (`json`, `memory`, `sqlite`) open against injected ports (`IFileStore`, `MemoryStore`, `ISqliteDriver`). Stub dialects return a named connection; `open` / `close` / `startTransaction` throw `ProviderNotImplementedError`.

## Mock

Depend on `IConnection`. Use `MockConnection` / `MockDatasource` from `@datasources/core`. `MockDatasource.createConnection` returns the configured `MockConnection` and does not open it.

```ts
const ds = new MockDatasource();
const connection = await ds.createConnection();
await connection.open();
expect((connection as MockConnection).opened).toBe(true);

const failing = new MockConnection();
failing.openError = new Error("open-fail");
await expect(failing.open()).rejects.toThrow("open-fail");
```

Provider tests inject fakes on options (`MemoryFileStore`, `ScriptedDriver`) and then `await open()`.
